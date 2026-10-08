use std::collections::HashMap;

use crate::refresh::{CpuUsageInfo, DisksInfo, MemoryUsageInfo, NetworkInfo};
// I really don't like this repetitions

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub cpu: Option<CpuUsageInfo>,
    pub mem: Option<MemoryUsageInfo>,
    pub disk: Option<DisksInfo>,
    pub net: Option<NetworkInfo>,
}

pub fn get_cpu_percentage(snap: &Snapshot) -> f64 {
    let Some(cpu) = snap.cpu.as_ref() else {
        return -1.0;
    };
    cpu.get_cpu_average()
}

pub fn get_total_mem(snap: &Snapshot) -> i64 {
    let Some(mem) = snap.mem.as_ref() else {
        return -1;
    };
    mem.get_total() as i64
}

pub fn get_available_mem(snap: &Snapshot) -> i64 {
    let Some(mem) = snap.mem.as_ref() else {
        return -1;
    };
    mem.get_available() as i64
}

pub fn get_disks_map_total(snap: &Snapshot) -> HashMap<String, i64> {
    let Some(disk) = snap.disk.as_ref() else {
        let mut m = HashMap::<String, i64>::new();
        m.insert("none".to_string(), -1);
        return m;
    };
    disk.get_disks_total_map_i64()
}

pub fn get_disks_map_available(snap: &Snapshot) -> HashMap<String, i64> {
    let Some(disk) = snap.disk.as_ref() else {
        let mut m = HashMap::<String, i64>::new();
        m.insert("none".to_string(), -1);
        return m;
    };
    disk.get_disks_available_map_i64()
}

pub fn get_net_transmited(snap: &Snapshot) -> i64 {
    let Some(net) = snap.net.as_ref() else {
        return -1;
    };
    net.get_transmited() as i64
}

pub fn get_net_name(snap: &Snapshot) -> String {
    let Some(net) = snap.net.as_ref() else {
        return "None".to_string();
    };
    net.get_name()
}

pub fn get_net_received(snap: &Snapshot) -> i64 {
    let Some(net) = snap.net.as_ref() else {
        return -1;
    };
    net.get_received() as i64
}
