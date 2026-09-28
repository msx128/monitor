use crate::{refresh::*, server::start_server};
use reqwest::Client;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tokio::sync::Mutex;

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

    start_server();

    CpuUsageInfo::spawn(&state, "cpu_info", get_env_var("CPU_INTERVAL"));
    NetworkInfo::spawn(&state, "network_info", get_env_var("NETWORK_INTERVAL"));
    MemoryUsageInfo::spawn(&state, "mem_info", get_env_var("MEM_INTERVAL"));
    DisksInfo::spawn(&state, "disk_info", get_env_var("DISK_INTERVAL"));

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

    let client = Client::new();

    let state = InnerState {
        client: Arc::new(client),
        sys: Arc::new(Mutex::new(sys)),
        disks: Arc::new(Mutex::new(Disks::new_with_refreshed_list())),
        networks: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
    };

    State {
        inner: Arc::new(state),
    }
}
