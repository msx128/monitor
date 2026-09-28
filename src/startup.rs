use crate::{refresh::*, server::server};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tokio::sync::{Mutex, mpsc};

fn get_env_var(key: &str) -> Option<Duration> {
    match env::var(key) {
        Ok(v) => {
            let time: u64 = match v.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "Error: while passing env: wrong value format it interval variable: {}",
                        e
                    );
                    return None;
                }
            };
            Some(Duration::from_secs(time))
        }
        Err(e) => {
            eprintln!("Error: while passing env: {}", e);
            None
        }
    }
}

pub async fn run() {
    println!("Starting...");
    let state = init();

    let (tx, rx) = mpsc::channel(4096);
    tokio::spawn(server(rx));
    CpuUsageInfo::spawn(&state, get_env_var("CPU_INTERVAL"), tx.clone());
    NetworkInfo::spawn(&state, get_env_var("NETWORK_INTERVAL"), tx.clone());
    MemoryUsageInfo::spawn(&state, get_env_var("MEM_INTERVAL"), tx.clone());
    DisksInfo::spawn(&state, get_env_var("DISK_INTERVAL"), tx);

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}

fn init() -> State {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let state = InnerState {
        sys: Arc::new(Mutex::new(sys)),
        disks: Arc::new(Mutex::new(Disks::new_with_refreshed_list())),
        networks: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
    };

    State {
        inner: Arc::new(state),
    }
}
