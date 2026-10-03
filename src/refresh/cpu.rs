use crate::refresh::inner::State;
use crate::refresh::inner::{create_system_metric_update_thread, get_timestamp};
use crate::refresh::{Metric, Update};
use serde::Serialize;
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, System};
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Serialize, Debug, Clone)]
pub struct CpuUsageInfo {
    timestamp: u64,
    cpu_usage: Vec<f32>,
}

impl CpuUsageInfo {
    pub fn get_cpu_usage_as_i64(&self) -> i64 {
        let s: f32 = self.cpu_usage.iter().sum();
        let len = self.cpu_usage.len() as f32;
        let res = s / len;
        res as i64
    }
}

impl Update for CpuUsageInfo {
    fn spawn(state: &State, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        create_system_metric_update_thread(state, inter, create_cpu_metric, tx);
    }
}

fn create_cpu_metric(sys: &mut System) -> Metric {
    sys.refresh_cpu_usage();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    let cpu = CpuUsageInfo {
        timestamp: get_timestamp(),
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    };

    Metric::Cpu(cpu)
}
