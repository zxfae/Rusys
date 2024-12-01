use sysinfo::System;
use crate::monitoring::cpu::CpuMonitor;
use crate::network::{NetworkData, NetworkMonitor};

pub struct App {
    pub sys: System,
    pub cpu_monitor: CpuMonitor,
    pub network_monitor: NetworkMonitor,
    pub network_data: Vec<NetworkData>,
}

impl App {
    pub fn new() -> Self {
        App {
            sys: System::new_all(),
            cpu_monitor: CpuMonitor::new(),
            network_monitor: NetworkMonitor::new(),
            network_data: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.sys.refresh_all();
        self.network_data = self.network_monitor.get_network_info();
    }
}
