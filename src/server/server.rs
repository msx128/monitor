use crate::refresh::Metric;
use crate::server::collector::{collector, display};
use crate::server::get_methods::Snapshot;
use crate::server::listener::listener;
use tokio::sync::mpsc;
use tokio::sync::watch;

// it is actually pretty convinient have separate big function, I don't need to declare
// anything in run and just do all things internally
pub async fn server(
    rx: mpsc::Receiver<Metric>,
    is_debug: bool,
    is_localhost: bool,
    port: String,
) -> Result<(), std::io::Error> {
    // change localhost to 0.0.0.0
    let (w_tx, w_rx) = watch::channel(Snapshot::default());
    tokio::spawn(collector(rx, w_tx));

    if is_debug {
        tokio::spawn(display(w_rx.clone()));
    }

    listener(w_rx, is_localhost, &port).await?;

    Ok(())
}
