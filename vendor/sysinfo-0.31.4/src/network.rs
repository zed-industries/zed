// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::HashMap;

use crate::network_helper::{get_interface_address, get_interface_ip_networks};
use crate::NetworkData;

/// Interface addresses are OS-independent
pub(crate) fn refresh_networks_addresses(interfaces: &mut HashMap<String, NetworkData>) {
    let interface_networks = unsafe { get_interface_ip_networks() };
    for (interface_name, ip_networks) in interface_networks {
        if let Some(interface) = interfaces.get_mut(&interface_name) {
            interface.inner.ip_networks = ip_networks.into_iter().collect::<Vec<_>>();
        }
    }
    match unsafe { get_interface_address() } {
        Ok(ifa_iterator) => {
            for (name, ifa) in ifa_iterator {
                if let Some(interface) = interfaces.get_mut(&name) {
                    interface.inner.mac_addr = ifa;
                }
            }
        }
        Err(_e) => {
            sysinfo_debug!("refresh_networks_addresses failed: {:?}", _e);
        }
    }
}
