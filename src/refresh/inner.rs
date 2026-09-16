use serde::Serialize;
use std::env;
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
