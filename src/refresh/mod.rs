mod cpu;
mod inner;
mod mem;

pub use cpu::create_cpu_update_thread;
pub use mem::create_memory_update_thread;
