use crate::startup::State;
use serde::Serialize;
use std::env;
use tokio::time::{Duration, interval};
// maybe make it udp
pub async fn push_message(client: &reqwest::Client, message: impl Serialize, endpoint: &str) {
    let _response = client
        .post(&format!("{}{}", get_base_url(), endpoint))
        .json(&message)
        .send()
        .await
        .expect("Failed to send POST request");
}

fn get_base_url() -> String {
    match env::var("BASE_URL") {
        Err(_) => "http://localhost:8000/api/v1/".to_string(),
        Ok(s) => s,
    }
}

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
}

pub fn create_metric_update_thread<T, F>(
    state: &State,
    endpoint: &str,
    inter: Option<Duration>,
    mut metric_fn: F,
) where
    F: FnMut(&mut sysinfo::System) -> T + Send + 'static,
    T: Serialize + Send + 'static,
{
    let state_arc = state.inner.clone();

    let client = state_arc.client.clone();
    let sys = state_arc.sys.clone();
    let endpoint = endpoint.to_string();

    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));

    tokio::spawn(async move {
        loop {
            inter.tick().await;

            let metric = {
                let mut sys_guard = sys.lock().await;
                metric_fn(&mut sys_guard)
            };

            println!("pushing {}!", &endpoint);
            push_message(&client, metric, &endpoint).await;
        }
    });
}
