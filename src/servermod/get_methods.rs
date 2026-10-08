use crate::refresh::{CpuUsageInfo, DisksInfo, MemoryUsageInfo, NetworkInfo};

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub cpu: Option<CpuUsageInfo>,
    pub mem: Option<MemoryUsageInfo>,
    pub disk: Option<DisksInfo>,
    pub net: Option<NetworkInfo>,
}

pub fn get_cpu_percentage(snap: &Snapshot) -> f64 {
    let Some(cpu) = snap.cpu.as_ref() else {
        return 0.0;
    };
    cpu.get_cpu_average()
}

pub fn get_total_mem(snap: &Snapshot) -> i64 {
    let Some(mem) = snap.mem.as_ref() else {
        return 0;
    };
    mem.get_total() as i64
}

pub fn get_available_mem(snap: &Snapshot) -> i64 {
    let Some(mem) = snap.mem.as_ref() else {
        return 0;
    };
    mem.get_available() as i64
}

pub fn get_disk_size(snap: &Snapshot) -> i64 {
    let Some(disk) = snap.disk.as_ref() else {
        return 0;
    };
    disk.get_total() as i64
}

pub fn get_disk_available(snap: &Snapshot) -> i64 {
    let Some(disk) = snap.disk.as_ref() else {
        return 0;
    };
    disk.get_avail() as i64
}

pub fn get_net_transmited(snap: &Snapshot) -> i64 {
    let Some(net) = snap.net.as_ref() else {
        return 0;
    };
    net.get_transmited() as i64
}

pub fn get_net_received(snap: &Snapshot) -> i64 {
    let Some(net) = snap.net.as_ref() else {
        return 0;
    };
    net.get_received() as i64
}
