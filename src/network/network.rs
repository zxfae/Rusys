use std::time::Instant;
use sysinfo::{Networks, IpNetwork};

///Network information interface snapshot
pub struct NetworkData
{
    //Networks interface name
    pub interface: String,
    //Physical address of interface
    pub mac_address: String,
    //IPv4
    pub ip_network: String,
    //Bytes
    pub total_received: u64,
    pub total_transmitted: u64,
    //Received per seconds
    pub rx_rate: f64,
    //Transmitted per secondes
    pub tx_rate: f64,
}

//Network statistics
pub struct NetworkMonitor
{
    //https://docs.rs/sysinfo/latest/sysinfo/struct.Networks.html
    networks: Networks,
    //Last update; collect metrics
    last_update: Instant,
    //-> String == Interface, u64 == bytes
    last_received: std::collections::HashMap<String, u64>,
    //-> String == Interface, u64 == bytes
    last_transmitted: std::collections::HashMap<String, u64>,
    //History -> String == interface, speed_rx and speed_tx
    history: std::collections::HashMap<String, Vec<(f64, f64)>>,
}

impl NetworkMonitor
{
    //New NetworkMonitor Instance for tracking
    pub fn new() -> Self
    {
        NetworkMonitor
        {
            //Each value start with n 0
            networks: Networks::new(),
            last_update: Instant::now(),
            last_received: std::collections::HashMap::new(),
            last_transmitted: std::collections::HashMap::new(),
            history: std::collections::HashMap::new(),
        }
    }
    //Get IPv4. Don't show IPv6 (U can print IPva4 && IPv6)
    //-> &[IpNetwork] reference to the IP address array
    //See sysinfo::IpNetwork
    fn get_ipv4(ip_networks:&[IpNetwork]) -> String
    {
        ip_networks.iter()
        //Search string contains .
        //Example IPv6 -> .find(|network| network.to_string().contains(':'))
        .find(|network| network.to_string().contains('.'))
        //map_or_else ?
        //map_or method's better;
        //|ip == element|
        .map_or("No IPv4".to_string(), |ip| ip.to_string())
    }

    //Return collections of stats for each interface
    pub fn get_network_info(&mut self) -> Vec<NetworkData>
    {
        let mut network_data = Vec::new();
        //See sysinfo::Networks
        self.networks.refresh_list();

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();

        //Updating values
        if elapsed < 0.1
        {
            //Empty Vec
            return network_data;
        }

        for (interface_name, network) in self.networks.iter()
        {
            //Create new variables with rrx && ttx
            let current_rx = network.total_received();
            let current_tx = network.total_transmitted();

            //Calculate tx
            let (rx_rate, tx_rate) = match (self.last_received.get(interface_name.as_str()),
                                            self.last_transmitted.get(interface_name.as_str()))
            {
                (Some(&last_rx), Some(&last_tx)) =>
                {
                    let rx_diff = current_rx.saturating_sub(last_rx);
                    let tx_diff = current_tx.saturating_sub(last_tx);

                    let rx = rx_diff as f64 / elapsed;
                    let tx = tx_diff as f64 / elapsed;
                    (rx, tx)
                },
                _ => (0.0, 0.0)
            };

                                            let history = self.history.entry(interface_name.clone())
                                            .or_insert_with(Vec::new);
                                            //Create history with 3 last values
                                            //Average with 3 lasts values
                                            //3 are optimise way?
                                            history.push((rx_rate, tx_rate));
                                            if history.len() > 3
                                            {
                                                history.remove(0);
                                            }

                                            let avg_rx = history.iter().map(|(rx, _)| rx).sum::<f64>() / history.len() as f64;
                                            let avg_tx = history.iter().map(|(_, tx)| tx).sum::<f64>() / history.len() as f64;

                                            //Update datas
                                            self.last_received.insert(interface_name.to_string(), current_rx);
                                            self.last_transmitted.insert(interface_name.to_string(), current_tx);

                                            //Call get_ipv4 function
                                            let ipv4 = Self::get_ipv4(network.ip_networks());
                                            //Push values to Vec
                                            //Please refeere to the NetworkData struct;
                                            //If u can add more datas u need pass here
                                            network_data.push(NetworkData
                                            {
                                                              interface: interface_name.to_string(),
                                                              mac_address: network.mac_address().to_string(),
                                                              ip_network: ipv4,
                                                              total_received: current_rx,
                                                              total_transmitted: current_tx,
                                                              rx_rate: avg_rx,
                                                              tx_rate: avg_tx,
                                            });
        }
        //Updated
        self.last_update = now;
        network_data
    }
}
