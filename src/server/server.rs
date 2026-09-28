use anyhow::Result;
use tokio::net::TcpListener;

async fn server() -> Result<()> {
    // change localhost to 0.0.0.0
    let listener = TcpListener::bind("127.0.0.1:9090").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Connected! {:?}", socket);
    }
}

pub fn start_server() {
    tokio::spawn(server());
}
