// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::network::refresh_networks_addresses;
use crate::{IpNetwork, MacAddr, NetworkData};

macro_rules! old_and_new {
    ($ty_:expr, $name:ident, $old:ident) => {{
        $ty_.$old = $ty_.$name;
        $ty_.$name = $name;
    }};
    ($ty_:expr, $name:ident, $old:ident, $path:expr) => {{
        let _tmp = $path;
        $ty_.$old = $ty_.$name;
        $ty_.$name = _tmp;
    }};
}

#[allow(clippy::ptr_arg)]
fn read<P: AsRef<Path>>(parent: P, path: &str, data: &mut Vec<u8>) -> u64 {
    if let Ok(mut f) = File::open(parent.as_ref().join(path)) {
        if let Ok(size) = f.read(data) {
            let mut i = 0;
            let mut ret = 0;

            while i < size && i < data.len() && data[i] >= b'0' && data[i] <= b'9' {
                ret *= 10;
                ret += (data[i] - b'0') as u64;
                i += 1;
            }
            return ret;
        }
    }
    0
}

fn refresh_networks_list_from_sysfs(
    interfaces: &mut HashMap<String, NetworkData>,
    sysfs_net: &Path,
) {
    if let Ok(dir) = std::fs::read_dir(sysfs_net) {
        let mut data = vec![0; 30];

        for stats in interfaces.values_mut() {
            stats.inner.updated = false;
        }

        for entry in dir.flatten() {
            let parent = &entry.path().join("statistics");
            let entry = match entry.file_name().into_string() {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let rx_bytes = read(parent, "rx_bytes", &mut data);
            let tx_bytes = read(parent, "tx_bytes", &mut data);
            let rx_packets = read(parent, "rx_packets", &mut data);
            let tx_packets = read(parent, "tx_packets", &mut data);
            let rx_errors = read(parent, "rx_errors", &mut data);
            let tx_errors = read(parent, "tx_errors", &mut data);
            // let rx_compressed = read(parent, "rx_compressed", &mut data);
            // let tx_compressed = read(parent, "tx_compressed", &mut data);
            match interfaces.entry(entry) {
                hash_map::Entry::Occupied(mut e) => {
                    let interface = e.get_mut();
                    let interface = &mut interface.inner;

                    old_and_new!(interface, rx_bytes, old_rx_bytes);
                    old_and_new!(interface, tx_bytes, old_tx_bytes);
                    old_and_new!(interface, rx_packets, old_rx_packets);
                    old_and_new!(interface, tx_packets, old_tx_packets);
                    old_and_new!(interface, rx_errors, old_rx_errors);
                    old_and_new!(interface, tx_errors, old_tx_errors);
                    // old_and_new!(e, rx_compressed, old_rx_compressed);
                    // old_and_new!(e, tx_compressed, old_tx_compressed);
                    interface.updated = true;
                }
                hash_map::Entry::Vacant(e) => {
                    e.insert(NetworkData {
                        inner: NetworkDataInner {
                            rx_bytes,
                            old_rx_bytes: rx_bytes,
                            tx_bytes,
                            old_tx_bytes: tx_bytes,
                            rx_packets,
                            old_rx_packets: rx_packets,
                            tx_packets,
                            old_tx_packets: tx_packets,
                            rx_errors,
                            old_rx_errors: rx_errors,
                            tx_errors,
                            old_tx_errors: tx_errors,
                            mac_addr: MacAddr::UNSPECIFIED,
                            ip_networks: vec![],
                            // rx_compressed,
                            // old_rx_compressed: rx_compressed,
                            // tx_compressed,
                            // old_tx_compressed: tx_compressed,
                            updated: true,
                        },
                    });
                }
            };
        }

        // Remove interfaces which are gone.
        interfaces.retain(|_, d| d.inner.updated);
    }
}

pub(crate) struct NetworksInner {
    pub(crate) interfaces: HashMap<String, NetworkData>,
}

impl NetworksInner {
    pub(crate) fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
        }
    }

    pub(crate) fn list(&self) -> &HashMap<String, NetworkData> {
        &self.interfaces
    }

    pub(crate) fn refresh(&mut self) {
        let mut v = vec![0; 30];

        for (interface_name, data) in self.interfaces.iter_mut() {
            data.inner.update(interface_name, &mut v);
        }
    }

    pub(crate) fn refresh_list(&mut self) {
        refresh_networks_list_from_sysfs(&mut self.interfaces, Path::new("/sys/class/net/"));
        refresh_networks_addresses(&mut self.interfaces);
    }
}

pub(crate) struct NetworkDataInner {
    /// Total number of bytes received over interface.
    rx_bytes: u64,
    old_rx_bytes: u64,
    /// Total number of bytes transmitted over interface.
    tx_bytes: u64,
    old_tx_bytes: u64,
    /// Total number of packets received.
    rx_packets: u64,
    old_rx_packets: u64,
    /// Total number of packets transmitted.
    tx_packets: u64,
    old_tx_packets: u64,
    /// Shows the total number of packets received with error. This includes
    /// too-long-frames errors, ring-buffer overflow errors, CRC errors,
    /// frame alignment errors, fifo overruns, and missed packets.
    rx_errors: u64,
    old_rx_errors: u64,
    /// similar to `rx_errors`
    tx_errors: u64,
    old_tx_errors: u64,
    /// MAC address
    pub(crate) mac_addr: MacAddr,
    pub(crate) ip_networks: Vec<IpNetwork>,
    // /// Indicates the number of compressed packets received by this
    // /// network device. This value might only be relevant for interfaces
    // /// that support packet compression (e.g: PPP).
    // rx_compressed: usize,
    // old_rx_compressed: usize,
    // /// Indicates the number of transmitted compressed packets. Note
    // /// this might only be relevant for devices that support
    // /// compression (e.g: PPP).
    // tx_compressed: usize,
    // old_tx_compressed: usize,
    /// Whether or not the above data has been updated during refresh
    updated: bool,
}

impl NetworkDataInner {
    fn update(&mut self, path: &str, data: &mut Vec<u8>) {
        let path = &Path::new("/sys/class/net/").join(path).join("statistics");
        old_and_new!(self, rx_bytes, old_rx_bytes, read(path, "rx_bytes", data));
        old_and_new!(self, tx_bytes, old_tx_bytes, read(path, "tx_bytes", data));
        old_and_new!(
            self,
            rx_packets,
            old_rx_packets,
            read(path, "rx_packets", data)
        );
        old_and_new!(
            self,
            tx_packets,
            old_tx_packets,
            read(path, "tx_packets", data)
        );
        old_and_new!(
            self,
            rx_errors,
            old_rx_errors,
            read(path, "rx_errors", data)
        );
        old_and_new!(
            self,
            tx_errors,
            old_tx_errors,
            read(path, "tx_errors", data)
        );
        // old_and_new!(
        //     self,
        //     rx_compressed,
        //     old_rx_compressed,
        //     read(path, "rx_compressed", data)
        // );
        // old_and_new!(
        //     self,
        //     tx_compressed,
        //     old_tx_compressed,
        //     read(path, "tx_compressed", data)
        // );
    }

    pub(crate) fn received(&self) -> u64 {
        self.rx_bytes.saturating_sub(self.old_rx_bytes)
    }

    pub(crate) fn total_received(&self) -> u64 {
        self.rx_bytes
    }

    pub(crate) fn transmitted(&self) -> u64 {
        self.tx_bytes.saturating_sub(self.old_tx_bytes)
    }

    pub(crate) fn total_transmitted(&self) -> u64 {
        self.tx_bytes
    }

    pub(crate) fn packets_received(&self) -> u64 {
        self.rx_packets.saturating_sub(self.old_rx_packets)
    }

    pub(crate) fn total_packets_received(&self) -> u64 {
        self.rx_packets
    }

    pub(crate) fn packets_transmitted(&self) -> u64 {
        self.tx_packets.saturating_sub(self.old_tx_packets)
    }

    pub(crate) fn total_packets_transmitted(&self) -> u64 {
        self.tx_packets
    }

    pub(crate) fn errors_on_received(&self) -> u64 {
        self.rx_errors.saturating_sub(self.old_rx_errors)
    }

    pub(crate) fn total_errors_on_received(&self) -> u64 {
        self.rx_errors
    }

    pub(crate) fn errors_on_transmitted(&self) -> u64 {
        self.tx_errors.saturating_sub(self.old_tx_errors)
    }

    pub(crate) fn total_errors_on_transmitted(&self) -> u64 {
        self.tx_errors
    }

    pub(crate) fn mac_address(&self) -> MacAddr {
        self.mac_addr
    }

    pub(crate) fn ip_networks(&self) -> &[IpNetwork] {
        &self.ip_networks
    }
}

#[cfg(test)]
mod test {
    use super::refresh_networks_list_from_sysfs;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn refresh_networks_list_add_interface() {
        let sys_net_dir = tempfile::tempdir().expect("failed to create temporary directory");

        fs::create_dir(sys_net_dir.path().join("itf1")).expect("failed to create subdirectory");

        let mut interfaces = HashMap::new();

        refresh_networks_list_from_sysfs(&mut interfaces, sys_net_dir.path());
        assert_eq!(interfaces.keys().collect::<Vec<_>>(), ["itf1"]);

        fs::create_dir(sys_net_dir.path().join("itf2")).expect("failed to create subdirectory");

        refresh_networks_list_from_sysfs(&mut interfaces, sys_net_dir.path());
        let mut itf_names: Vec<String> = interfaces.keys().map(|n| n.to_owned()).collect();
        itf_names.sort();
        assert_eq!(itf_names, ["itf1", "itf2"]);
    }

    #[test]
    fn refresh_networks_list_remove_interface() {
        let sys_net_dir = tempfile::tempdir().expect("failed to create temporary directory");

        let itf1_dir = sys_net_dir.path().join("itf1");
        let itf2_dir = sys_net_dir.path().join("itf2");
        fs::create_dir(&itf1_dir).expect("failed to create subdirectory");
        fs::create_dir(itf2_dir).expect("failed to create subdirectory");

        let mut interfaces = HashMap::new();

        refresh_networks_list_from_sysfs(&mut interfaces, sys_net_dir.path());
        let mut itf_names: Vec<String> = interfaces.keys().map(|n| n.to_owned()).collect();
        itf_names.sort();
        assert_eq!(itf_names, ["itf1", "itf2"]);

        fs::remove_dir(&itf1_dir).expect("failed to remove subdirectory");

        refresh_networks_list_from_sysfs(&mut interfaces, sys_net_dir.path());
        assert_eq!(interfaces.keys().collect::<Vec<_>>(), ["itf2"]);
    }
}
