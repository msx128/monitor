use crate::refresh::inner::{get_timestamp, push_message};
use crate::startup::State;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use sysinfo::{NetworkData, Networks};
use tokio::time::{Duration, interval};

#[derive(Serialize, Debug)]
struct NetworkInfo {
    timestamp: u64,
    name: String,
    recieved: u64,
    transmited: u64,
}

pub fn create_network_update_thread(state: &State, endpoint: &str, inter: Option<Duration>) {
    let state_ark = state.inner.clone();
    let client = state_ark.client.clone();
    let endpoint = endpoint.to_string();
    let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));
    let networks = state_ark.networks.clone();

    tokio::spawn(async move {
        let network_name = {
            let binding = networks.lock().await;
            find_main_network(&binding).0.clone()
        };
        loop {
            inter.tick().await;

            let metric = {
                let mut networks_lock = networks.lock().await;
                networks_lock.refresh(true);
                get_network_metric(&*networks_lock, &network_name)
            };

            println!("pushing {}!", &endpoint);
            push_message(&client, metric, &endpoint).await;
        }
    });
}

fn get_network_metric(networks_lock: &Networks, network_name: &String) -> NetworkInfo {
    let network = networks_lock.get(network_name).expect("Network gone :(");
    NetworkInfo {
        timestamp: get_timestamp(),
        name: network_name.to_string(),
        recieved: network.received(),
        transmited: network.transmitted(),
    }
}

fn find_main_network(networks: &Networks) -> (&String, &NetworkData) {
    networks
        .iter()
        .filter(|(_k, v)| match v.operational_state() {
            sysinfo::InterfaceOperationalState::Up => true,
            sysinfo::InterfaceOperationalState::Dormant => true,
            sysinfo::InterfaceOperationalState::Unknown => true,
            _ => false,
        } && {
            let addr = v.ip_networks()[0].addr;
            const LOCALHOST_V4: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            const LOCALHOST_V6: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
            match addr {
                LOCALHOST_V4 => false,
                LOCALHOST_V6 => false,
                _ => true
            }
        }).next().expect("Failed to find network")
}
