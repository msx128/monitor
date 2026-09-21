use crate::refresh::*;
use reqwest::Client;
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tokio::sync::Mutex;

pub async fn run() {
    println!("Starting...");
    let state = init();

    CpuUsageInfo::spawn(&state, "cpu_info", None);
    NetworkInfo::spawn(&state, "network_info", None);
    MemoryUsageInfo::spawn(&state, "mem_info", None);
    DisksInfo::spawn(&state, "disk_info", None);

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
