use sysinfo::{System, RefreshKind, CpuRefreshKind};
//All informations required
pub struct CpuInfo {
    pub index: usize,
    pub vendor_id: String,
    pub name: String,
    pub usage: f32,
    //Why using frequency ?
    pub frequency: u64,
}

pub struct CpuMonitor {
    system: System,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything())
            )
        }
    }

    pub fn get_cpu_info(&mut self) -> Vec<CpuInfo> {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu_all();

        self.system.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuInfo {
                index: i,
                vendor_id: cpu.vendor_id().to_string(),
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect()
    }
}