use crate::refresh::inner::State;
use crate::refresh::inner::create_system_metric_update_thread;
use crate::refresh::{Metric, Update};
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, System};
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct CpuUsageInfo {
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
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    };

    Metric::Cpu(cpu)
}
