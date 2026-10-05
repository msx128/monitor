use crate::refresh::inner::State;
use crate::refresh::inner::create_system_metric_update_thread;
use crate::refresh::{Metric, Update};
use sysinfo::System;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct MemoryUsageInfo {
    available: u64,
    total: u64,
}

impl MemoryUsageInfo {
    pub fn get_total(&self) -> u64 {
        self.total
    }

    pub fn get_available(&self) -> u64 {
        self.available
    }
}

impl Update for MemoryUsageInfo {
    fn spawn(state: &State, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        create_system_metric_update_thread(state, inter, create_memory_metric, tx);
    }
}

fn create_memory_metric(sys: &mut System) -> Metric {
    sys.refresh_memory();

    let mem = MemoryUsageInfo {
        available: sys.available_memory(),
        total: sys.total_memory(),
    };

    Metric::Mem(mem)
}
