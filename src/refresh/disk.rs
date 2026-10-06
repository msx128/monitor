use crate::refresh::Metric;
use crate::refresh::inner::State;
use crate::refresh::inner::Update;
use std::sync::Arc;
use sysinfo::DiskKind;
use sysinfo::Disks;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct DisksInfo {
    disks: Vec<DiskInfo>,
}

// change this later
impl DisksInfo {
    pub fn get_avail(&self) -> u64 {
        self.disks.first().unwrap().available
    }

    pub fn get_total(&self) -> u64 {
        self.disks.first().unwrap().total
    }
}

#[derive(Debug, Clone)]
struct DiskInfo {
    _kind: DiskKind,
    _name: String,
    total: u64,
    available: u64,
}

impl Update for DisksInfo {
    fn spawn(state: &State, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        let state_ark = state.inner.clone();
        let disks = state_ark.disks.clone();
        let mut inter = interval(inter.unwrap_or(Duration::from_mins(30)));

        tokio::spawn(async move {
            let mut first = true;
            loop {
                if !first {
                    inter.tick().await;
                }

                let metric = get_disks_metric(&disks).await;

                // println!("{:?}", metric);
                tx.send(Metric::Disk(metric.clone()))
                    .await
                    .expect("Change this later");

                first = false;
            }
        });
    }
}

async fn get_disks_metric(disks: &Arc<Mutex<Disks>>) -> DisksInfo {
    let mut disks_lock = disks.lock().await;
    disks_lock.refresh(true);

    let mut batch: Vec<DiskInfo> = Vec::with_capacity(1);

    for disk in disks_lock.iter() {
        if matches!(disk.kind(), DiskKind::Unknown(_)) {
            continue;
        }
        let total = disk.total_space();
        let available = disk.available_space();

        let info = DiskInfo {
            _kind: disk.kind(),
            _name: disk.name().to_string_lossy().into_owned(),
            total,
            available,
        };

        batch.push(info);
    }

    DisksInfo { disks: batch }
}
