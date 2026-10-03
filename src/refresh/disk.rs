use crate::refresh::Metric;
use crate::refresh::inner::State;
use crate::refresh::inner::Update;
use crate::refresh::inner::get_timestamp;
use serde::Serialize;
use std::sync::Arc;
use sysinfo::DiskKind;
use sysinfo::Disks;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

fn serialize_disk_kind<S>(kind: &DiskKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let kind_str = match kind {
        DiskKind::HDD => "HDD",
        DiskKind::SSD => "SSD",
        DiskKind::Unknown(_) => unreachable!("unknown kinds are skipped"),
    };
    kind_str.serialize(serializer)
}

#[derive(Serialize, Debug, Clone)]
pub struct DisksInfo {
    disks: Vec<DiskInfo>,
}

// change this later
impl DisksInfo {
    pub fn get_avail(&self) -> u64 {
        self.disks.iter().next().unwrap().available
    }

    pub fn get_total(&self) -> u64 {
        self.disks.iter().next().unwrap().total
    }
}

#[derive(Serialize, Debug, Clone)]
struct DiskInfo {
    timestamp: u64,
    #[serde(serialize_with = "serialize_disk_kind")]
    kind: DiskKind,
    name: String,
    total: u64,
    available: u64,
    used: u64,
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
