use crate::refresh::inner::State;
use crate::refresh::inner::Update;
use crate::refresh::inner::{get_timestamp, push_message};
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use sysinfo::Networks;
use tokio::time::{Duration, interval};

#[derive(Serialize, Debug)]
pub struct NetworkInfo {
    timestamp: u64,
    name: String,
    received: u64,
    transmited: u64,
}

impl Update for NetworkInfo {
    fn spawn(state: &State, endpoint: &str, inter: Option<Duration>) {
        let state_ark = state.inner.clone();
        let client = state_ark.client.clone();
        let endpoint = endpoint.to_string();
        let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));
        let networks = state_ark.networks.clone();

        tokio::spawn(async move {
            loop {
                inter.tick().await;

                let metric = {
                    let mut networks_lock = networks.lock().await;
                    networks_lock.refresh(true);
                    let network_name = find_main_network(&networks_lock);
                    get_network_metric(&*networks_lock, &network_name)
                };

                let metric = match metric {
                    None => {
                        eprintln!("Failed to get metric");
                        continue;
                    }
                    Some(v) => v,
                };

                println!("pushing {}!", &endpoint);
                let _ = match push_message(&client, metric, &endpoint).await {
                    Err(e) => eprintln!("Failed to push from network: {e}"),
                    Ok(_) => (),
                };
            }
        });
    }
}

fn get_network_metric(
    networks_lock: &Networks,
    network_name: &Option<&String>,
) -> Option<NetworkInfo> {
    let network_name = match network_name {
        None => return None,
        Some(v) => v,
    };
    let network = match networks_lock.get(&network_name.to_string()) {
        None => return None,
        Some(v) => v,
    };
    Some(NetworkInfo {
        timestamp: get_timestamp(),
        name: network_name.to_string(),
        received: network.received(),
        transmited: network.transmitted(),
    })
}

fn find_main_network(networks: &Networks) -> Option<&String> {
    match networks
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
        }).next() {
            Some(res) => Some(res.0),
            None => {
                eprintln!("Failed to find network");
                None
            },
    }
}
