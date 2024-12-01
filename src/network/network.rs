
use std::time::Instant;
use sysinfo::Networks;

/// U can add others ctx to monitor networkDatas
pub struct NetworkData {
    pub interface: String,
    pub mac_address: String,
    //Ipv4 && Ipv6
    pub ip_network: String,
    pub received: u64,
    pub transmitted: u64,
    pub total_received: u64,
    pub total_transmitted: u64,
    pub rx_rate: f64,
    pub tx_rate: f64,
}


pub struct NetworkMonitor {
    networks: Networks,
    last_update: Instant,
    last_received: std::collections::HashMap<String, u64>,
    last_transmitted: std::collections::HashMap<String, u64>,
    samples: std::collections::HashMap<String, Vec<(f64, f64)>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        NetworkMonitor {
            networks: Networks::new(),
            last_update: Instant::now(),
            last_received: std::collections::HashMap::new(),
            last_transmitted: std::collections::HashMap::new(),
            samples: std::collections::HashMap::new(),
        }
    }

    pub fn get_network_info(&mut self) -> Vec<NetworkData> {
        let mut network_data = Vec::new();
        self.networks.refresh_list();

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();

        if elapsed < 0.1 {
            return network_data;
        }

        for (interface_name, network) in self.networks.iter() {
            let current_rx = network.total_received();
            let current_tx = network.total_transmitted();

            let (rx_rate, tx_rate) = match (self.last_received.get(interface_name.as_str()),
                                            self.last_transmitted.get(interface_name.as_str())) {
                (Some(&last_rx), Some(&last_tx)) => {
                    let rx_diff = current_rx.saturating_sub(last_rx);
                    let tx_diff = current_tx.saturating_sub(last_tx);

                    let rx = rx_diff as f64 / elapsed;
                    let tx = tx_diff as f64 / elapsed;
                    (rx, tx)
                },
                _ => (0.0, 0.0)
                                            };

                                            let samples = self.samples.entry(interface_name.clone())
                                            .or_insert_with(Vec::new);

                                            samples.push((rx_rate, tx_rate));
                                            if samples.len() > 3 {
                                                samples.remove(0);
                                            }

                                            let avg_rx = samples.iter().map(|(rx, _)| rx).sum::<f64>() / samples.len() as f64;
                                            let avg_tx = samples.iter().map(|(_, tx)| tx).sum::<f64>() / samples.len() as f64;

                                            self.last_received.insert(interface_name.to_string(), current_rx);
                                            self.last_transmitted.insert(interface_name.to_string(), current_tx);

                                            network_data.push(NetworkData {
                                                              interface: interface_name.to_string(),
                                                              mac_address: network.mac_address().to_string(),
                                                              // -> &[IpNetwork]; monitoring Ipv4 && Ipv6 ?
                                                              ip_network: network.ip_networks()[0].to_string(),
                                                              received: network.received(),
                                                              transmitted: network.transmitted(),
                                                              total_received: current_rx,
                                                              total_transmitted: current_tx,
                                                              rx_rate: avg_rx,
                                                              tx_rate: avg_tx,
                                            });
        }
        self.last_update = now;
        network_data
    }
}
