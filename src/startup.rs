use crate::refresh::*;
use reqwest::Client;
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};
use tokio::sync::Mutex;

pub async fn run() {
    println!("Starting...");
    let state = init();

    create_cpu_update_thread(&state, "cpu_info", None);
    create_memory_update_thread(&state, "mem_info", None);
    create_disk_update_thread(&state, "disk_info", None);
    create_network_update_thread(&state, "network_info", None);

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

pub struct InnerState {
    pub client: Arc<Client>,
    pub sys: Arc<Mutex<System>>,
    pub disks: Arc<Mutex<Disks>>,
    pub networks: Arc<Mutex<Networks>>,
}

pub struct State {
    pub inner: Arc<InnerState>,
}
