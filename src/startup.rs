use crate::{refresh::*, servermod::server};
use dotenvy;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Disks, Networks, System};
use tokio::sync::{Mutex, mpsc};

// env funcitions used only once so I don't think that possiblity
// of repeated loading is a problem(because there is no any)
fn get_env_duration_var(key: &str) -> Option<Duration> {
    match env::var(key) {
        Ok(v) => {
            let time: u64 = match v.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "Error: while passing env: wrong value format it interval variable: {}",
                        e
                    );
                    return None;
                }
            };
            Some(Duration::from_secs(time))
        }
        Err(e) => {
            eprintln!("Error: while passing env: {}", e);
            None
        }
    }
}

fn is_bool(s: &str) -> bool {
    match env::var(s) {
        Ok(v) => {
            let vref = &*v.to_lowercase(); // same as as_str() 
            !(vref == "f" || vref == "false")
        }
        Err(e) => {
            eprintln!(
                "Error: while passing env: wrong value format it interval variable: {}",
                e
            );
            true
        }
    }
}

fn get_port() -> String {
    let var = env::var("PORT");
    let port = match var {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Reading port env: {e}");
            "9090".to_string()
        }
    };
    let port_num: i32 = match port.parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Port must be int: {e}");
            eprintln!("Setting to default 9090");
            return "9090".to_string();
        }
    };
    if !(1024..=49151).contains(&port_num) {
        eprintln!("Port must be between 1024 and 49151 including");
        eprintln!("Setting to default 9090");
        "9090".to_string()
    } else {
        port
    }
}

pub async fn run() {
    println!("Starting...");
    if let Err(e) = dotenvy::dotenv() {
        eprintln!(
            "Error: while passing env: wrong value format it interval variable: {}",
            e
        );
    };
    let is_debug = is_bool("DEBUG");
    let is_localhost = is_bool("LOCALHOST");
    let port = get_port();

    let sys = Arc::new(Mutex::new(System::new_all()));

    let (tx, rx) = mpsc::channel(4096);
    let disks = Disks::new_with_refreshed_list();
    let mountpoints: Vec<String> = disks
        .iter()
        .map(|d| d.name().to_string_lossy().to_string())
        .collect();
    tokio::spawn(server(rx, is_debug, is_localhost, port, mountpoints));
    CpuUsageInfo::spawn(
        sys.clone(),
        get_env_duration_var("CPU_INTERVAL"),
        tx.clone(),
    );
    MemoryUsageInfo::spawn(
        sys.clone(),
        get_env_duration_var("MEM_INTERVAL"),
        tx.clone(),
    );
    NetworkInfo::spawn(
        Networks::new_with_refreshed_list(),
        get_env_duration_var("NETWORK_INTERVAL"),
        tx.clone(),
    );
    DisksInfo::spawn(disks, get_env_duration_var("DISK_INTERVAL"), tx);

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}
