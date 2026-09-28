use crate::refresh::inner::State;
use crate::refresh::inner::{create_system_metric_update_thread, get_timestamp};
use crate::refresh::{Metric, Update};
use serde::Serialize;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Serialize, Debug, Clone)]
pub struct MemoryUsageInfo {
    timestamp: u64,
    used: u64,
    total: u64,
}

impl Update for MemoryUsageInfo {
    fn spawn(state: &State, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        create_system_metric_update_thread(state, inter, create_memory_metric, tx);
    }
}

fn create_memory_metric(sys: &mut System) -> Metric {
    sys.refresh_memory();

    let mem = MemoryUsageInfo {
        timestamp: get_timestamp(),
        used: sys.used_memory(),
        total: sys.total_memory(),
    };

    Metric::Mem(mem)
}
