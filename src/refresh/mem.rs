use crate::refresh::inner::{create_metric_update_thread, get_timestamp};
use crate::startup::State;
use serde::Serialize;
use sysinfo::System;
use tokio::time::Duration;

#[derive(Serialize, Debug)]
struct MemoryUsageInfo {
    timestamp: u64,
    used: u64,
    total: u64,
}
pub fn create_memory_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    create_metric_update_thread(state, endpoint, inter, create_memory_metric);
}

fn create_memory_metric(sys: &mut System) -> MemoryUsageInfo {
    sys.refresh_memory();

    MemoryUsageInfo {
        timestamp: get_timestamp(),
        used: sys.used_memory(),
        total: sys.total_memory(),
    }
}
