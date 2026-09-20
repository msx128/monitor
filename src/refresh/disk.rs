use crate::refresh::inner::{get_timestamp, push_message};
use crate::startup::State;
use serde::Serialize;
use std::sync::Arc;
use sysinfo::DiskKind;
use sysinfo::Disks;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

#[derive(Serialize, Debug)]
struct DisksInfo {
    disks: Vec<DiskInfo>,
}

#[derive(Serialize, Debug)]
struct DiskInfo {
    timestamp: u64,
    kind: DiskKind,
    name: String,
    total: u64,
    available: u64,
    used: u64,
}

pub fn create_disk_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    let state_ark = state.inner.clone();

    let client = state_ark.client.clone();
    let disks = state_ark.disks.clone();
    let endpoint = endpoint.to_string();

    let mut inter = interval(inter.unwrap_or(Duration::from_mins(30)));

    tokio::spawn(async move {
        let mut first = true;
        loop {
            if !first {
                inter.tick().await;
            }

            let metric = get_disks_metric(&disks).await;

            println!("pushing {}!", &endpoint);
            push_message(&client, metric, &endpoint).await;
            first = false;
        }
    });
}

async fn get_disks_metric(disks: &Arc<Mutex<Disks>>) -> DisksInfo {
    let mut disks_lock = disks.lock().await;
    disks_lock.refresh(true);

    let mut batch: Vec<DiskInfo> = Vec::with_capacity(1);

    for disk in disks_lock.iter() {
        let total = disk.total_space();
        let available = disk.available_space();

        let info = DiskInfo {
            timestamp: get_timestamp(),
            kind: disk.kind(),
            name: disk.name().to_string_lossy().into_owned(),
            total: total,
            available: available,
            used: total - available,
        };

        batch.push(info);
    }

    DisksInfo { disks: batch }
}
