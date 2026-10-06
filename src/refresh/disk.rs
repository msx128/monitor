use crate::refresh::Metric;
use sysinfo::DiskKind;
use sysinfo::Disks;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct DisksInfo {
    disks: Vec<DiskInfo>,
}

// change this later
impl DisksInfo {
    pub fn get_avail(&self) -> u64 {
        match self.disks.first() {
            None => 0,
            Some(d) => d.available,
        }
    }

    pub fn get_total(&self) -> u64 {
        match self.disks.first() {
            None => 0,
            Some(d) => d.total,
        }
    }
}

#[derive(Debug, Clone)]
struct DiskInfo {
    _kind: DiskKind,
    _name: String,
    total: u64,
    available: u64,
}

impl DisksInfo {
    pub fn spawn(mut disks: Disks, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        let mut inter = interval(inter.unwrap_or(Duration::from_mins(30)));

        tokio::spawn(async move {
            let mut first = true;
            loop {
                if !first {
                    inter.tick().await;
                }

                let metric = get_disks_metric(&mut disks).await;

                // println!("{:?}", metric);
                tx.send(Metric::Disk(metric.clone()))
                    .await
                    .expect("Change this later");

                first = false;
            }
        });
    }
}

async fn get_disks_metric(disks: &mut Disks) -> DisksInfo {
    disks.refresh(true);

    let mut batch: Vec<DiskInfo> = Vec::with_capacity(1);

    for disk in disks.iter() {
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
