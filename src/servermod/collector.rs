use crate::refresh::Metric;
use crate::servermod::get_methods::Snapshot;
use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, interval};

pub async fn collector(
    mut m_rx: mpsc::Receiver<Metric>,
    w_tx: watch::Sender<Snapshot>,
    is_debug: bool,
) {
    while let Some(v) = m_rx.recv().await {
        w_tx.send_modify(|s| match v {
            Metric::Cpu(c) => s.cpu = Some(c),
            Metric::Mem(m) => s.mem = Some(m),
            Metric::Disk(d) => s.disk = Some(d),
            Metric::Network(n) => s.net = Some(n),
        });
        if is_debug {
            println!("fetched");
        }
    }
}

pub async fn display(w_rx: watch::Receiver<Snapshot>) {
    let mut tic = interval(Duration::from_secs(2));
    loop {
        tic.tick().await;
        println!("Data: {:?}", *w_rx.borrow());
    }
}
