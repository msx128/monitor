use crate::refresh::Metric;
use crate::refresh::inner::State;
use crate::refresh::inner::Update;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use sysinfo::Networks;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    _name: String,
    received: u64,
    transmited: u64,
}

impl NetworkInfo {
    pub fn get_transmited(&self) -> u64 {
        self.transmited
    }

    pub fn get_received(&self) -> u64 {
        self.received
    }
}

impl Update for NetworkInfo {
    fn spawn(state: &State, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        let state_ark = state.inner.clone();
        let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));
        let networks = state_ark.networks.clone();

        tokio::spawn(async move {
            loop {
                inter.tick().await;

                let metric = {
                    let mut networks_lock = networks.lock().await;
                    networks_lock.refresh(true);
                    let network_name = find_main_network(&networks_lock);
                    get_network_metric(&networks_lock, &network_name)
                };

                let metric = match metric {
                    None => {
                        eprintln!("Failed to get metric");
                        continue;
                    }
                    Some(v) => v,
                };

                // println!("{:?}", metric);

                tx.send(Metric::Network(metric.clone()))
                    .await
                    .expect("Change this later");
            }
        });
    }
}

fn get_network_metric(
    networks_lock: &Networks,
    network_name: &Option<&String>,
    // wtf is this,       ^^
) -> Option<NetworkInfo> {
    let network_name = network_name.as_ref()?;
    // &&String is implement to_string but this is wow
    let network = networks_lock.get(&network_name.to_string())?;
    Some(NetworkInfo {
        _name: network_name.to_string(),
        received: network.received(),
        transmited: network.transmitted(),
    })
}

fn find_main_network(networks: &Networks) -> Option<&String> {
    match networks.iter().find(|(_k, v)| {
        matches!(
            v.operational_state(),
            sysinfo::InterfaceOperationalState::Up
                | sysinfo::InterfaceOperationalState::Dormant
                | sysinfo::InterfaceOperationalState::Unknown
        ) && {
            let addr = v.ip_networks()[0].addr;
            const LOCALHOST_V4: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            const LOCALHOST_V6: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
            !matches!(addr, LOCALHOST_V4 | LOCALHOST_V6)
        }
    }) {
        Some(res) => Some(res.0),
        None => {
            eprintln!("Failed to find network");
            None
        }
    }
}
