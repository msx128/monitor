use crate::refresh::inner::push_message;
use crate::startup::State;
use serde::Serialize;
use tokio::time::{Duration, interval};

#[derive(Serialize, Debug)]
struct MemoryUsageInfo {
    timestamp: u64,
    used: u64,
    total: u64,
}

pub fn create_memory_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    // initialization
    let state_arc = state.inner.clone();

    let client = state_arc.client.clone();
    let sys = state_arc.sys.clone();
    let endpoint = endpoint.to_string();

    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));
    tokio::spawn(async move {
        println!("started memory");
        loop {
            inter.tick().await;
            // actual code
            let (timestamp, used, total) = {
                let mut sys_lock = sys.lock().await;
                sys_lock.refresh_memory();
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u64;
                let used = sys_lock.used_memory();
                let total = sys_lock.total_memory();
                (timestamp, used, total)
            };

            let memory_info = MemoryUsageInfo {
                timestamp,
                used,
                total,
            };

            // boiler plate
            // or something
            println!("pushing memory_info!");
            push_message(&client, memory_info, &endpoint).await;
        }
    });
}
