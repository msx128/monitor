use crate::refresh::Metric;
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

pub fn create_system_metric_update_thread<F>(
    system: Arc<Mutex<System>>,
    inter: Option<Duration>,
    mut metric_fn: F,
    tx: mpsc::Sender<Metric>,
) where
    F: FnMut(&mut sysinfo::System) -> Metric + Send + 'static,
{
    let sys = system.clone();
    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));

    tokio::spawn(async move {
        loop {
            inter.tick().await;

            let metric = {
                let mut sys_guard = sys.lock().await;
                metric_fn(&mut sys_guard)
            };

            // println!("{:?}", metric);
            tx.send(metric.clone()).await.expect("Change this later");
        }
    });
}

pub trait Update {
    fn spawn(_system: Arc<Mutex<System>>, _inter: Option<Duration>, _tx: mpsc::Sender<Metric>) {
        unimplemented!();
    }
}
