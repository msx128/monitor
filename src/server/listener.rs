use crate::server::get_methods::*;
use crate::server::labels::{Labels, default_lable};
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::watch;

pub async fn listener(w_rx: watch::Receiver<Snapshot>) -> Result<(), std::io::Error> {
    let mut registry = <Registry>::default();
    let def_lable = default_lable(); // instead of creating new one every time

    let http_requests = Family::<Labels, Counter>::default();
    registry.register(
        "http_requests",
        "Number of HTTP requests received",
        http_requests.clone(),
    );

    let cpu_usage = Family::<Labels, Gauge>::default();
    registry.register("cpu_usage_ratio", "Percent of cpu usage", cpu_usage.clone());

    let mem_total = Family::<Labels, Gauge>::default();
    registry.register(
        "mem_total_bytes",
        "Total of memory bytes",
        mem_total.clone(),
    );
    let mem_available = Family::<Labels, Gauge>::default();
    registry.register(
        "mem_available_bytes",
        "Available memory bytes",
        mem_available.clone(),
    );

    let disk_size = Family::<Labels, Gauge>::default();
    registry.register("disk_size", "Total disk size", disk_size.clone());
    let disk_available = Family::<Labels, Gauge>::default();
    registry.register(
        "disk_available",
        "Available disk space",
        disk_available.clone(),
    );

    let net_transmited = Family::<Labels, Gauge>::default();
    registry.register("net_transmited", "Transmited bytes", net_transmited.clone());
    let net_received = Family::<Labels, Gauge>::default();
    registry.register("net_received", "Received bytes", net_received.clone());

    let listener = TcpListener::bind("127.0.0.1:9090").await?;
    println!("Listening!");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Connected: {:?}!", addr);

        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        let _n = reader.read_line(&mut request_line).await?;

        let is_metric_get_req = request_line.starts_with("GET /metrics ");
        if is_metric_get_req {
            http_requests.get_or_create(&def_lable).inc();

            shared_watch_borrow(
                &w_rx,
                &cpu_usage,
                &mem_total,
                &mem_available,
                &disk_size,
                &disk_available,
                &net_transmited,
                &net_received,
                &def_lable,
            );
            // idk how all of them &self but ok

            let mut body = String::new();
            encode(&mut body, &registry).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "encoding to openmetrics format error",
                )
            })?;

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

            reader.get_mut().write_all(response.as_bytes()).await?;
        } else {
            reader
                .get_mut()
                .write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .await?;
        }
    }
}

pub fn shared_watch_borrow(
    w_rx: &watch::Receiver<Snapshot>,
    cpu_usage: &Family<Labels, Gauge>,
    mem_total: &Family<Labels, Gauge>,
    mem_available: &Family<Labels, Gauge>,
    disk_size: &Family<Labels, Gauge>,
    disk_available: &Family<Labels, Gauge>,
    net_transmited: &Family<Labels, Gauge>,
    net_received: &Family<Labels, Gauge>,
    def_lable: &Labels,
) {
    let borrow = w_rx.borrow().clone();

    cpu_usage
        .get_or_create(def_lable)
        .set(get_cpu_percentage(&borrow));

    mem_total
        .get_or_create(def_lable)
        .set(get_total_mem(&borrow));
    mem_available
        .get_or_create(def_lable)
        .set(get_available_mem(&borrow));

    disk_size
        .get_or_create(def_lable)
        .set(get_disk_size(&borrow));
    disk_available
        .get_or_create(def_lable)
        .set(get_disk_available(&borrow));

    net_transmited
        .get_or_create(def_lable)
        .set(get_net_transmited(&borrow));
    net_received
        .get_or_create(def_lable)
        .set(get_net_received(&borrow));
}
