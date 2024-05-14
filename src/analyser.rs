use sysinfo::System;

pub struct SystemInfo {
    pub system: System,
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            system: System::new_all(),
        }
    }

    pub fn get_total_memory(&self) -> u64 {
        self.system.total_memory() as u64
    }

    pub fn get_free_memory(&self) -> u64 {
        self.system.free_memory() as u64
    }

    pub fn get_cpu_load(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage() as f32
    }
}
