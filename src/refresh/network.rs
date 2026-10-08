use crate::refresh::Metric;
use sysinfo::Networks;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    name: String,
    received: u64,
    transmited: u64,
}

impl NetworkInfo {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_transmited(&self) -> u64 {
        self.transmited
    }

    pub fn get_received(&self) -> u64 {
        self.received
    }
}

impl NetworkInfo {
    pub fn spawn(mut networks: Networks, inter: Option<Duration>, tx: mpsc::Sender<Metric>) {
        let mut inter = interval(inter.unwrap_or(Duration::from_secs(1)));

        tokio::spawn(async move {
            loop {
                inter.tick().await;

                let metric = {
                    networks.refresh(true);
                    let network_name = find_main_network(&networks);
                    get_network_metric(&networks, &network_name)
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
        name: network_name.to_string(),
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
        ) && v.ip_networks().iter().any(|n| !n.addr.is_loopback())
    }) {
        Some(res) => Some(res.0),
        None => {
            eprintln!("Failed to find network");
            None
        }
    }
}
