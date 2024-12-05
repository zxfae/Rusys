use sysinfo::System;

pub struct SystemInfo {
    pub host_name: String,
    pub os_name: String,
    pub cpu_architecture: String,
    pub kernel_version: String,
    pub total_memory: u64,
}

pub fn get_system_info() -> SystemInfo {
    //Create new instance
    let sys = System::new_all();
    SystemInfo {
        host_name: System::host_name().unwrap_or_default(),
        os_name: System::name().unwrap_or_default(),
        cpu_architecture: System::cpu_arch(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        //u64
        total_memory: sys.total_memory(),
    }
}
