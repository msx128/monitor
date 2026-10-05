use crate::refresh::{CpuUsageInfo, DisksInfo, MemoryUsageInfo, NetworkInfo};

#[derive(Debug, Clone)]
pub enum Metric {
    Cpu(CpuUsageInfo),
    Disk(DisksInfo),
    Mem(MemoryUsageInfo),
    Network(NetworkInfo),
}
