use crate::servermod::get_methods::*;
use crate::servermod::labels::*;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;

pub async fn listener(
    w_rx: watch::Receiver<Snapshot>,
    is_localhost: bool,
    port: &str,
    mountpoints: Vec<String>,
) -> Result<(), std::io::Error> {
    let mut registry = <Registry>::default();

    let chunky = register_and_get_metrics(&mut registry, &mountpoints);

    let address = if is_localhost {
        format!("127.0.0.1:{}", port)
    } else {
        format!("0.0.0.0:{}", port)
    };
    let listener = TcpListener::bind(address).await?;
    println!("Listening!");

    let load_arc = Arc::new(Load {
        chunky,
        w_rx,
        registry,
        mountpoints,
    });

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not get client: {e}");
                continue;
            }
        };
        println!("Connected: {:?}!", addr);

        let load = load_arc.clone();
        // move tokio spawn closure to function
        tokio::spawn(handler(stream, load));
    }
}

pub struct Chuncky {
    cpu_usage: Gauge<f64, AtomicU64>,
    mem_total: Gauge,
    mem_available: Gauge,
    disks_size: HashMap<String, Family<DiskLabel, Gauge>>,
    disks_available: HashMap<String, Family<DiskLabel, Gauge>>,
    net_transmited: Family<NetLabel, Counter>,
    net_received: Family<NetLabel, Counter>,
    http_requests: Family<HttpLabel, Counter>,
}

pub fn shared_watch_borrow(
    w_rx: &watch::Receiver<Snapshot>,
    chunky: &Chuncky,
    mountpoints: &Vec<String>,
) {
    let borrow = w_rx.borrow().clone();

    chunky.cpu_usage.set(get_cpu_percentage(&borrow));

    chunky.mem_total.set(get_total_mem(&borrow));
    chunky.mem_available.set(get_available_mem(&borrow));

    for mountpoint in mountpoints {
        chunky
            .disks_size
            .get(mountpoint)
            .expect("no mountpoint in family map, this is definitely a bug")
            .get_or_create(&DiskLabel {
                mountpoint: mountpoint.to_string(),
            })
            .set(
                get_disks_map_total(&borrow)
                    .get(mountpoint)
                    .expect("no mountpoint in get map, this is definitely a bug")
                    .to_owned(),
            );
        chunky
            .disks_available
            .get(mountpoint)
            .expect("no mountpoint in map, this is definitely a bug")
            .get_or_create(&DiskLabel {
                mountpoint: mountpoint.to_string(),
            })
            .set(
                get_disks_map_available(&borrow)
                    .get(mountpoint)
                    .expect("no mountpoint in get map, this is definitely a bug")
                    .to_owned(),
            );
    }

    let net_name = get_net_name(&borrow);
    chunky
        .net_transmited
        .get_or_create(&NetLabel {
            interface: net_name.clone(),
        })
        .inc_by(get_net_transmited(&borrow) as u64);
    chunky
        .net_received
        .get_or_create(&NetLabel {
            interface: net_name,
        })
        .inc_by(get_net_received(&borrow) as u64);
}

struct Load {
    chunky: Chuncky,
    w_rx: watch::Receiver<Snapshot>,
    registry: Registry,
    mountpoints: Vec<String>,
}

fn register_and_get_metrics(registry: &mut Registry, mountpoints: &Vec<String>) -> Chuncky {
    let http_requests = Family::<HttpLabel, Counter>::default();
    registry.register(
        "http_requests",
        "Number of HTTP requests received",
        http_requests.clone(),
    );

    let cpu_usage = Gauge::<f64, AtomicU64>::default();
    registry.register(
        "monitor_cpu_usage_ratio",
        "Average cpu usage across all cores (0 to 1)",
        cpu_usage.clone(),
    );

    let mem_total = Gauge::default();
    registry.register(
        "monitor_memory_total_bytes",
        "Total of memory bytes",
        mem_total.clone(),
    );
    let mem_available = Gauge::default();
    registry.register(
        "monitor_memory_available_bytes",
        "Available memory bytes",
        mem_available.clone(),
    );

    let disks_len = mountpoints.len();
    let mut disks_size: HashMap<String, Family<DiskLabel, Gauge>> =
        HashMap::with_capacity(disks_len);
    let mut disks_available: HashMap<String, Family<DiskLabel, Gauge>> =
        HashMap::with_capacity(disks_len);

    for mountpoint in mountpoints {
        let disk_size = Family::<DiskLabel, Gauge>::default();
        registry.register(
            "monitor_disk_total_bytes",
            "Total disk size",
            disk_size.clone(),
        );
        let disk_available = Family::<DiskLabel, Gauge>::default();
        registry.register(
            "monitor_disk_available_bytes",
            "Available disk space_bytes",
            disk_available.clone(),
        );
        disks_size.insert(mountpoint.clone(), disk_size);
        disks_available.insert(mountpoint.clone(), disk_available);
    }

    let net_transmited = Family::<NetLabel, Counter>::default();
    registry.register(
        "monitor_network_transmited",
        "Transmited bytes since boot",
        net_transmited.clone(),
    );
    let net_received = Family::<NetLabel, Counter>::default();
    registry.register(
        "monitor_network_received",
        "Received bytes since boot",
        net_received.clone(),
    );

    Chuncky {
        cpu_usage,
        mem_total,
        mem_available,
        disks_size,
        disks_available,
        net_transmited,
        net_received,
        http_requests,
    }
}

async fn handler(stream: TcpStream, load: Arc<Load>) {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    if let Err(e) = reader.read_line(&mut request_line).await {
        eprintln!("Failed to read request line: {e}");
    }

    let is_metric_get_req = request_line.starts_with("GET /metrics ");
    if is_metric_get_req {
        load.chunky
            .http_requests
            .get_or_create(&HttpLabel {
                method: Methods::GET,
                path: "/metrics".to_string(),
            })
            .inc();

        shared_watch_borrow(&load.w_rx, &load.chunky, &load.mountpoints);
        // idk how all of them &self but ok

        let mut body = String::new();
        if let Err(e) = encode(&mut body, &load.registry) {
            eprintln!("encoding to openmetrics format error: {e}");
        }

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: application/openmetrics-text; version=1.0.0; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Connection: Close\r\n\
        \r\n\
        {}",
            body.len(),
            body,
        );

        if let Err(e) = reader.get_mut().write_all(response.as_bytes()).await {
            eprintln!("Failed to write to client: {e}");
        };
    } else {
        if let Err(e) = reader
            .get_mut()
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
        {
            eprintln!("Failed to write to client: {e}");
        };
    }
}
