use crate::refresh::Update;
use crate::refresh::inner::State;
use crate::refresh::inner::{create_metric_update_thread, get_timestamp};
use serde::Serialize;
use sysinfo::System;
use tokio::time::Duration;

#[derive(Serialize, Debug)]
pub struct MemoryUsageInfo {
    timestamp: u64,
    used: u64,
    total: u64,
}

impl Update for MemoryUsageInfo {
    fn spawn(state: &State, endpoint: &str, inter: Option<Duration>) {
        create_metric_update_thread(state, endpoint, inter, create_memory_metric);
    }
}

fn create_memory_metric(sys: &mut System) -> MemoryUsageInfo {
    sys.refresh_memory();

    MemoryUsageInfo {
        timestamp: get_timestamp(),
        used: sys.used_memory(),
        total: sys.total_memory(),
    }
}
