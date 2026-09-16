use crate::refresh::inner::{create_metric_update_thread, get_timestamp};
use crate::startup::State;
use serde::Serialize;
use sysinfo::System;
use tokio::time::Duration;

#[derive(Serialize, Debug)]
struct CpuUsageInfo {
    timestamp: u64,
    cpu_usage: Vec<f32>,
}

pub fn create_cpu_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    create_metric_update_thread(state, endpoint, inter, create_cpu_metric);
}

fn create_cpu_metric(sys: &mut System) -> CpuUsageInfo {
    sys.refresh_cpu_usage();

    CpuUsageInfo {
        timestamp: get_timestamp(),
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    }
}
