use crate::refresh::{CpuUsageInfo, DisksInfo, MemoryUsageInfo, NetworkInfo};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Metric {
    Cpu(CpuUsageInfo),
    Disk(DisksInfo),
    Mem(MemoryUsageInfo),
    Network(NetworkInfo),
}
