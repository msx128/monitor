mod cpu;
mod disk;
mod inner;
mod mem;
mod network;

pub use cpu::create_cpu_update_thread;
pub use disk::create_disk_update_thread;
pub use mem::create_memory_update_thread;
pub use network::create_network_update_thread;
