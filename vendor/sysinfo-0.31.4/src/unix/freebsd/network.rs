// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::{hash_map, HashMap};
use std::mem::MaybeUninit;

use super::utils;
use crate::network::refresh_networks_addresses;
use crate::{IpNetwork, MacAddr, NetworkData};

macro_rules! old_and_new {
    ($ty_:expr, $name:ident, $old:ident, $data:expr) => {{
        $ty_.$old = $ty_.$name;
        $ty_.$name = $data.$name;
    }};
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

    pub(crate) fn refresh_list(&mut self) {
        unsafe {
            self.refresh_interfaces(true);
        }
        // Remove interfaces which are gone.
        self.interfaces.retain(|_, n| n.inner.updated);
        refresh_networks_addresses(&mut self.interfaces);
    }

    pub(crate) fn refresh(&mut self) {
        unsafe {
            self.refresh_interfaces(false);
        }
    }

    unsafe fn refresh_interfaces(&mut self, refresh_all: bool) {
        let mut nb_interfaces: libc::c_int = 0;
        if !utils::get_sys_value(
            &[
                libc::CTL_NET,
                libc::PF_LINK,
                libc::NETLINK_GENERIC,
                libc::IFMIB_SYSTEM,
                libc::IFMIB_IFCOUNT,
            ],
            &mut nb_interfaces,
        ) {
            return;
        }
        if refresh_all {
            // We don't need to update this value if we're not updating all interfaces.
            for interface in self.interfaces.values_mut() {
                interface.inner.updated = false;
            }
        }
        let mut data: libc::ifmibdata = MaybeUninit::zeroed().assume_init();
        for row in 1..=nb_interfaces {
            let mib = [
                libc::CTL_NET,
                libc::PF_LINK,
                libc::NETLINK_GENERIC,
                libc::IFMIB_IFDATA,
                row,
                libc::IFDATA_GENERAL,
            ];

            if !utils::get_sys_value(&mib, &mut data) {
                continue;
            }
            if let Some(name) = utils::c_buf_to_utf8_string(&data.ifmd_name) {
                let data = &data.ifmd_data;
                match self.interfaces.entry(name) {
                    hash_map::Entry::Occupied(mut e) => {
                        let interface = e.get_mut();
                        let interface = &mut interface.inner;

                        old_and_new!(interface, ifi_ibytes, old_ifi_ibytes, data);
                        old_and_new!(interface, ifi_obytes, old_ifi_obytes, data);
                        old_and_new!(interface, ifi_ipackets, old_ifi_ipackets, data);
                        old_and_new!(interface, ifi_opackets, old_ifi_opackets, data);
                        old_and_new!(interface, ifi_ierrors, old_ifi_ierrors, data);
                        old_and_new!(interface, ifi_oerrors, old_ifi_oerrors, data);
                        interface.updated = true;
                    }
                    hash_map::Entry::Vacant(e) => {
                        if !refresh_all {
                            // This is simply a refresh, we don't want to add new interfaces!
                            continue;
                        }
                        e.insert(NetworkData {
                            inner: NetworkDataInner {
                                ifi_ibytes: data.ifi_ibytes,
                                old_ifi_ibytes: 0,
                                ifi_obytes: data.ifi_obytes,
                                old_ifi_obytes: 0,
                                ifi_ipackets: data.ifi_ipackets,
                                old_ifi_ipackets: 0,
                                ifi_opackets: data.ifi_opackets,
                                old_ifi_opackets: 0,
                                ifi_ierrors: data.ifi_ierrors,
                                old_ifi_ierrors: 0,
                                ifi_oerrors: data.ifi_oerrors,
                                old_ifi_oerrors: 0,
                                updated: true,
                                mac_addr: MacAddr::UNSPECIFIED,
                                ip_networks: vec![],
                            },
                        });
                    }
                }
            }
        }
    }
}

pub(crate) struct NetworkDataInner {
    /// Total number of bytes received over interface.
    ifi_ibytes: u64,
    old_ifi_ibytes: u64,
    /// Total number of bytes transmitted over interface.
    ifi_obytes: u64,
    old_ifi_obytes: u64,
    /// Total number of packets received.
    ifi_ipackets: u64,
    old_ifi_ipackets: u64,
    /// Total number of packets transmitted.
    ifi_opackets: u64,
    old_ifi_opackets: u64,
    /// Shows the total number of packets received with error. This includes
    /// too-long-frames errors, ring-buffer overflow errors, CRC errors,
    /// frame alignment errors, fifo overruns, and missed packets.
    ifi_ierrors: u64,
    old_ifi_ierrors: u64,
    /// similar to `ifi_ierrors`
    ifi_oerrors: u64,
    old_ifi_oerrors: u64,
    /// Whether or not the above data has been updated during refresh
    updated: bool,
    /// MAC address
    pub(crate) mac_addr: MacAddr,
    /// IP networks
    pub(crate) ip_networks: Vec<IpNetwork>,
}

impl NetworkDataInner {
    pub(crate) fn received(&self) -> u64 {
        self.ifi_ibytes.saturating_sub(self.old_ifi_ibytes)
    }

    pub(crate) fn total_received(&self) -> u64 {
        self.ifi_ibytes
    }

    pub(crate) fn transmitted(&self) -> u64 {
        self.ifi_obytes.saturating_sub(self.old_ifi_obytes)
    }

    pub(crate) fn total_transmitted(&self) -> u64 {
        self.ifi_obytes
    }

    pub(crate) fn packets_received(&self) -> u64 {
        self.ifi_ipackets.saturating_sub(self.old_ifi_ipackets)
    }

    pub(crate) fn total_packets_received(&self) -> u64 {
        self.ifi_ipackets
    }

    pub(crate) fn packets_transmitted(&self) -> u64 {
        self.ifi_opackets.saturating_sub(self.old_ifi_opackets)
    }

    pub(crate) fn total_packets_transmitted(&self) -> u64 {
        self.ifi_opackets
    }

    pub(crate) fn errors_on_received(&self) -> u64 {
        self.ifi_ierrors.saturating_sub(self.old_ifi_ierrors)
    }

    pub(crate) fn total_errors_on_received(&self) -> u64 {
        self.ifi_ierrors
    }

    pub(crate) fn errors_on_transmitted(&self) -> u64 {
        self.ifi_oerrors.saturating_sub(self.old_ifi_oerrors)
    }

    pub(crate) fn total_errors_on_transmitted(&self) -> u64 {
        self.ifi_oerrors
    }

    pub(crate) fn mac_address(&self) -> MacAddr {
        self.mac_addr
    }

    pub(crate) fn ip_networks(&self) -> &[IpNetwork] {
        &self.ip_networks
    }
}
