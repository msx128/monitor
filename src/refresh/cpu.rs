use crate::refresh::Update;
use crate::refresh::inner::State;
use crate::refresh::inner::{create_system_metric_update_thread, get_timestamp};
use serde::Serialize;
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, System};
use tokio::time::Duration;

#[derive(Serialize, Debug)]
pub struct CpuUsageInfo {
    timestamp: u64,
    cpu_usage: Vec<f32>,
}

impl Update for CpuUsageInfo {
    fn spawn(state: &State, endpoint: &str, inter: Option<Duration>) {
        create_system_metric_update_thread(state, endpoint, inter, create_cpu_metric);
    }
}

fn create_cpu_metric(sys: &mut System) -> CpuUsageInfo {
    sys.refresh_cpu_usage();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    CpuUsageInfo {
        timestamp: get_timestamp(),
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    }
}
