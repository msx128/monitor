use crate::refresh::Metric;
use anyhow::Result;
use tokio::{net::TcpListener, sync::mpsc};

pub async fn server(mut rx: mpsc::Receiver<Metric>) -> Result<()> {
    // change localhost to 0.0.0.0

    while let Some(v) = rx.recv().await {
        println!("{:?}", v);
    }

    // let listener = TcpListener::bind("127.0.0.1:9090").await?;

    // loop {
    //     let (socket, _) = listener.accept().await?;
    //     println!("Connected! {:?}", socket);
    // }
    Ok(())
}
