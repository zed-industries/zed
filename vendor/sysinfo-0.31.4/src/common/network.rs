// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;

use crate::{NetworkDataInner, NetworksInner};

/// Interacting with network interfaces.
///
/// ```no_run
/// use sysinfo::Networks;
///
/// let networks = Networks::new_with_refreshed_list();
/// for (interface_name, network) in &networks {
///     println!("[{interface_name}]: {network:?}");
/// }
/// ```
pub struct Networks {
    pub(crate) inner: NetworksInner,
}

impl<'a> IntoIterator for &'a Networks {
    type Item = (&'a String, &'a NetworkData);
    type IntoIter = std::collections::hash_map::Iter<'a, String, NetworkData>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Default for Networks {
    fn default() -> Self {
        Networks::new()
    }
}

impl Networks {
    /// Creates a new empty [`Networks`][crate::Networks] type.
    ///
    /// If you want it to be filled directly, take a look at [`Networks::new_with_refreshed_list`].
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let mut networks = Networks::new();
    /// networks.refresh_list();
    /// for (interface_name, network) in &networks {
    ///     println!("[{interface_name}]: {network:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            inner: NetworksInner::new(),
        }
    }

    /// Creates a new [`Networks`][crate::Networks] type with the network interfaces
    /// list loaded. It is a combination of [`Networks::new`] and
    /// [`Networks::refresh_list`].
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for network in &networks {
    ///     println!("{network:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list() -> Self {
        let mut networks = Self::new();
        networks.refresh_list();
        networks
    }

    /// Returns the network interfaces map.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for network in networks.list() {
    ///     println!("{network:?}");
    /// }
    /// ```
    pub fn list(&self) -> &HashMap<String, NetworkData> {
        self.inner.list()
    }

    /// Refreshes the network interfaces list.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let mut networks = Networks::new();
    /// networks.refresh_list();
    /// ```
    pub fn refresh_list(&mut self) {
        self.inner.refresh_list()
    }

    /// Refreshes the network interfaces' content. If you didn't run [`Networks::refresh_list`]
    /// before, calling this method won't do anything as no interfaces are present.
    ///
    /// ⚠️ If a network interface is added or removed, this method won't take it into account. Use
    /// [`Networks::refresh_list`] instead.
    ///
    /// ⚠️ If you didn't call [`Networks::refresh_list`] beforehand, this method will do nothing
    /// as the network list will be empty.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Wait some time...? Then refresh the data of each network.
    /// networks.refresh();
    /// ```
    pub fn refresh(&mut self) {
        self.inner.refresh()
    }
}

impl std::ops::Deref for Networks {
    type Target = HashMap<String, NetworkData>;

    fn deref(&self) -> &Self::Target {
        self.list()
    }
}

/// Getting volume of received and transmitted data.
///
/// ```no_run
/// use sysinfo::Networks;
///
/// let networks = Networks::new_with_refreshed_list();
/// for (interface_name, network) in &networks {
///     println!("[{interface_name}] {network:?}");
/// }
/// ```
pub struct NetworkData {
    pub(crate) inner: NetworkDataInner,
}

impl NetworkData {
    /// Returns the number of received bytes since the last refresh.
    ///
    /// If you want the total number of bytes received, take a look at the
    /// [`total_received`](NetworkData::total_received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("in: {} B", network.received());
    /// }
    /// ```
    pub fn received(&self) -> u64 {
        self.inner.received()
    }

    /// Returns the total number of received bytes.
    ///
    /// If you want the amount of received bytes since the last refresh, take a look at the
    /// [`received`](NetworkData::received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("in: {} B", network.total_received());
    /// }
    /// ```
    pub fn total_received(&self) -> u64 {
        self.inner.total_received()
    }

    /// Returns the number of transmitted bytes since the last refresh.
    ///
    /// If you want the total number of bytes transmitted, take a look at the
    /// [`total_transmitted`](NetworkData::total_transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("out: {} B", network.transmitted());
    /// }
    /// ```
    pub fn transmitted(&self) -> u64 {
        self.inner.transmitted()
    }

    /// Returns the total number of transmitted bytes.
    ///
    /// If you want the amount of transmitted bytes since the last refresh, take a look at the
    /// [`transmitted`](NetworkData::transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("out: {} B", network.total_transmitted());
    /// }
    /// ```
    pub fn total_transmitted(&self) -> u64 {
        self.inner.total_transmitted()
    }

    /// Returns the number of incoming packets since the last refresh.
    ///
    /// If you want the total number of packets received, take a look at the
    /// [`total_packets_received`](NetworkData::total_packets_received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("in: {}", network.packets_received());
    /// }
    /// ```
    pub fn packets_received(&self) -> u64 {
        self.inner.packets_received()
    }

    /// Returns the total number of incoming packets.
    ///
    /// If you want the amount of received packets since the last refresh, take a look at the
    /// [`packets_received`](NetworkData::packets_received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("in: {}", network.total_packets_received());
    /// }
    /// ```
    pub fn total_packets_received(&self) -> u64 {
        self.inner.total_packets_received()
    }

    /// Returns the number of outcoming packets since the last refresh.
    ///
    /// If you want the total number of packets transmitted, take a look at the
    /// [`total_packets_transmitted`](NetworkData::total_packets_transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("out: {}", network.packets_transmitted());
    /// }
    /// ```
    pub fn packets_transmitted(&self) -> u64 {
        self.inner.packets_transmitted()
    }

    /// Returns the total number of outcoming packets.
    ///
    /// If you want the amount of transmitted packets since the last refresh, take a look at the
    /// [`packets_transmitted`](NetworkData::packets_transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("out: {}", network.total_packets_transmitted());
    /// }
    /// ```
    pub fn total_packets_transmitted(&self) -> u64 {
        self.inner.total_packets_transmitted()
    }

    /// Returns the number of incoming errors since the last refresh.
    ///
    /// If you want the total number of errors on received packets, take a look at the
    /// [`total_errors_on_received`](NetworkData::total_errors_on_received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("in: {}", network.errors_on_received());
    /// }
    /// ```
    pub fn errors_on_received(&self) -> u64 {
        self.inner.errors_on_received()
    }

    /// Returns the total number of incoming errors.
    ///
    /// If you want the amount of errors on received packets since the last refresh, take a look at
    /// the [`errors_on_received`](NetworkData::errors_on_received) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("in: {}", network.total_errors_on_received());
    /// }
    /// ```
    pub fn total_errors_on_received(&self) -> u64 {
        self.inner.total_errors_on_received()
    }

    /// Returns the number of outcoming errors since the last refresh.
    ///
    /// If you want the total number of errors on transmitted packets, take a look at the
    /// [`total_errors_on_transmitted`](NetworkData::total_errors_on_transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    /// use std::{thread, time};
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// // Waiting a bit to get data from network...
    /// thread::sleep(time::Duration::from_millis(10));
    /// // Refreshing again to generate diff.
    /// networks.refresh();
    ///
    /// for (interface_name, network) in &networks {
    ///     println!("out: {}", network.errors_on_transmitted());
    /// }
    /// ```
    pub fn errors_on_transmitted(&self) -> u64 {
        self.inner.errors_on_transmitted()
    }

    /// Returns the total number of outcoming errors.
    ///
    /// If you want the amount of errors on transmitted packets since the last refresh, take a look at
    /// the [`errors_on_transmitted`](NetworkData::errors_on_transmitted) method.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("out: {}", network.total_errors_on_transmitted());
    /// }
    /// ```
    pub fn total_errors_on_transmitted(&self) -> u64 {
        self.inner.total_errors_on_transmitted()
    }

    /// Returns the MAC address associated to current interface.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("MAC address: {}", network.mac_address());
    /// }
    /// ```
    pub fn mac_address(&self) -> MacAddr {
        self.inner.mac_address()
    }

    /// Returns the Ip Networks associated to current interface.
    ///
    /// ```no_run
    /// use sysinfo::Networks;
    ///
    /// let mut networks = Networks::new_with_refreshed_list();
    /// for (interface_name, network) in &networks {
    ///     println!("Ip Networks: {:?}", network.ip_networks());
    /// }
    /// ```
    pub fn ip_networks(&self) -> &[IpNetwork] {
        self.inner.ip_networks()
    }
}

/// MAC address for network interface.
///
/// It is returned by [`NetworkData::mac_address`][crate::NetworkData::mac_address].
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    /// A `MacAddr` with all bytes set to `0`.
    pub const UNSPECIFIED: Self = MacAddr([0; 6]);

    /// Checks if this `MacAddr` has all bytes equal to `0`.
    pub fn is_unspecified(&self) -> bool {
        self == &MacAddr::UNSPECIFIED
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            data[0], data[1], data[2], data[3], data[4], data[5],
        )
    }
}

/// Ip networks address for network interface.
///
/// It is returned by [`NetworkData::ip_networks`][crate::NetworkData::ip_networks].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpNetwork {
    /// The ip of the network interface
    pub addr: IpAddr,
    /// The netmask, prefix of the ipaddress
    pub prefix: u8,
}

impl fmt::Display for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // Ensure that the `Display` and `Debug` traits are implemented on the `MacAddr` struct
    #[test]
    fn check_display_impl_mac_address() {
        println!(
            "{} {:?}",
            MacAddr([0x1, 0x2, 0x3, 0x4, 0x5, 0x6]),
            MacAddr([0xa, 0xb, 0xc, 0xd, 0xe, 0xf])
        );
    }

    #[test]
    fn check_mac_address_is_unspecified_true() {
        assert!(MacAddr::UNSPECIFIED.is_unspecified());
        assert!(MacAddr([0; 6]).is_unspecified());
    }

    #[test]
    fn check_mac_address_is_unspecified_false() {
        assert!(!MacAddr([1, 2, 3, 4, 5, 6]).is_unspecified());
    }

    // Ensure that the `Display` and `Debug` traits are implemented on the `IpNetwork` struct
    #[test]
    fn check_display_impl_ip_network_ipv4() {
        println!(
            "{} {:?}",
            IpNetwork {
                addr: IpAddr::from(Ipv4Addr::new(1, 2, 3, 4)),
                prefix: 3
            },
            IpNetwork {
                addr: IpAddr::from(Ipv4Addr::new(255, 255, 255, 0)),
                prefix: 21
            }
        );
    }

    #[test]
    fn check_display_impl_ip_network_ipv6() {
        println!(
            "{} {:?}",
            IpNetwork {
                addr: IpAddr::from(Ipv6Addr::new(0xffff, 0xaabb, 00, 0, 0, 0x000c, 11, 21)),
                prefix: 127
            },
            IpNetwork {
                addr: IpAddr::from(Ipv6Addr::new(0xffcc, 0, 0, 0xffcc, 0, 0xffff, 0, 0xccaa)),
                prefix: 120
            }
        )
    }

    #[test]
    fn check_ip_networks() {
        if !IS_SUPPORTED_SYSTEM {
            return;
        }
        let networks = Networks::new_with_refreshed_list();
        if networks.iter().any(|(_, n)| !n.ip_networks().is_empty()) {
            return;
        }
        panic!("Networks should have at least one IP network ");
    }
}
