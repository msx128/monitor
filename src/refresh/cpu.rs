use crate::refresh::inner::push_message;
use crate::startup::State;
use serde::Serialize;
use tokio::time::{Duration, interval};

#[derive(Serialize, Debug)]
struct CpuUsageInfo {
    timestamp: u64,
    cpu_usage: Vec<f32>,
}

pub fn create_cpu_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    // initialization
    let state_arc = state.inner.clone();

    let client = state_arc.client.clone();
    let sys = state_arc.sys.clone();
    let endpoint = endpoint.to_string();

    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));
    tokio::spawn(async move {
        println!("started cpu");
        loop {
            inter.tick().await;
            let (timestamp, cpu_usage) = {
                let mut sys_lock = sys.lock().await;
                sys_lock.refresh_cpu_usage();
                let times = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u64;
                let cpu_usage: Vec<f32> =
                    sys_lock.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
                (times, cpu_usage)
            };

            let cpu_info = CpuUsageInfo {
                timestamp,
                cpu_usage,
            };

            println!("cpu usage");

            // boiler plate
            // or something
            println!("pushing cpu_info!");
            push_message(&client, cpu_info, &endpoint).await;
        }
    });
}
