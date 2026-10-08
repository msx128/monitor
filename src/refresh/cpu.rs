use crate::refresh::inner::create_system_metric_update_thread;
use crate::refresh::{Metric, Update};
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct CpuUsageInfo {
    cpu_usage: Vec<f32>,
}

impl CpuUsageInfo {
    pub fn get_cpu_average(&self) -> f64 {
        let s: f64 = self.cpu_usage.iter().sum::<f32>() as f64;
        let len = self.cpu_usage.len() as f64;
        s / len / 100.0
    }
}

impl Update for CpuUsageInfo {
    fn spawn(system: Arc<Mutex<System>>, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        create_system_metric_update_thread(system, inter, create_cpu_metric, tx);
    }
}

fn create_cpu_metric(sys: &mut System) -> Metric {
    sys.refresh_cpu_usage();

    let cpu = CpuUsageInfo {
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    };

    Metric::Cpu(cpu)
}
