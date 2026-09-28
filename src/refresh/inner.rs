use crate::refresh::Metric;
use std::sync::Arc;
use sysinfo::Disks;
use sysinfo::Networks;
use sysinfo::System;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
}

pub fn create_system_metric_update_thread<F>(
    state: &State,
    inter: Option<Duration>,
    mut metric_fn: F,
    tx: mpsc::Sender<Metric>,
) where
    F: FnMut(&mut sysinfo::System) -> Metric + Send + 'static,
{
    let state_arc = state.inner.clone();
    let sys = state_arc.sys.clone();
    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));

    tokio::spawn(async move {
        loop {
            inter.tick().await;

            let metric = {
                let mut sys_guard = sys.lock().await;
                metric_fn(&mut sys_guard)
            };

            println!("{:?}", metric);
            tx.send(metric.clone()).await.expect("Change this later");
        }
    });
}

pub struct InnerState {
    pub sys: Arc<Mutex<System>>,
    pub disks: Arc<Mutex<Disks>>,
    pub networks: Arc<Mutex<Networks>>,
}

pub struct State {
    pub inner: Arc<InnerState>,
}

pub trait Update {
    fn spawn(_state: &State, _inter: Option<Duration>, _tx: mpsc::Sender<Metric>) {}
}
