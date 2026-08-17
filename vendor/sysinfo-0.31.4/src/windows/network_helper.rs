// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::ptr::{null_mut, NonNull};

use windows::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};
use windows::Win32::NetworkManagement::IpHelper::{
    GetAdaptersAddresses, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST,
    IP_ADAPTER_ADDRESSES_LH, IP_ADAPTER_UNICAST_ADDRESS_LH,
};
use windows::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, AF_UNSPEC, SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6,
};

use crate::{IpNetwork, MacAddr};

/// this iterator yields an interface name and address
pub(crate) struct InterfaceAddressIterator {
    /// The first item in the linked list
    buf: *mut IP_ADAPTER_ADDRESSES_LH,
    /// The current adapter
    adapter: *mut IP_ADAPTER_ADDRESSES_LH,
}

impl InterfaceAddressIterator {
    fn new() -> Self {
        Self {
            buf: null_mut(),
            adapter: null_mut(),
        }
    }
    unsafe fn realloc(mut self, size: libc::size_t) -> Result<Self, String> {
        let new_buf = libc::realloc(self.buf as _, size) as *mut IP_ADAPTER_ADDRESSES_LH;
        if new_buf.is_null() {
            // insufficient memory available
            // https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/malloc?view=msvc-170#return-value
            // malloc is not documented to set the last-error code
            Err("failed to allocate memory for IP_ADAPTER_ADDRESSES".to_string())
        } else {
            self.buf = new_buf;
            self.adapter = new_buf;
            Ok(self)
        }
    }
}

impl Iterator for InterfaceAddressIterator {
    type Item = (String, MacAddr);

    fn next(&mut self) -> Option<Self::Item> {
        if self.adapter.is_null() {
            return None;
        }
        unsafe {
            let adapter = self.adapter;
            // Move to the next adapter
            self.adapter = (*adapter).Next;
            if let Ok(interface_name) = (*adapter).FriendlyName.to_string() {
                // take the first 6 bytes and return the MAC address instead
                let [mac @ .., _, _] = (*adapter).PhysicalAddress;
                Some((interface_name, MacAddr(mac)))
            } else {
                // Not sure whether error can occur when parsing adapter name.
                self.next()
            }
        }
    }
}

impl InterfaceAddressIterator {
    pub fn generate_ip_networks(&mut self) -> HashMap<String, HashSet<IpNetwork>> {
        let mut results = HashMap::new();
        while !self.adapter.is_null() {
            unsafe {
                let adapter = self.adapter;
                // Move to the next adapter
                self.adapter = (*adapter).Next;
                if let Ok(interface_name) = (*adapter).FriendlyName.to_string() {
                    let ip_networks = get_ip_networks((*adapter).FirstUnicastAddress);
                    results.insert(interface_name, ip_networks);
                }
            }
        }
        results
    }
}

pub(crate) unsafe fn get_interface_ip_networks() -> HashMap<String, HashSet<IpNetwork>> {
    match get_interface_address() {
        Ok(mut interface_iter) => interface_iter.generate_ip_networks(),
        _ => HashMap::new(),
    }
}

impl Drop for InterfaceAddressIterator {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.buf as _);
        }
    }
}

pub(crate) unsafe fn get_interface_address() -> Result<InterfaceAddressIterator, String> {
    // https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getadaptersaddresses#remarks
    // A 15k buffer is recommended
    let mut size: u32 = 15 * 1024;
    let mut ret = ERROR_SUCCESS.0;
    let mut iterator = InterfaceAddressIterator::new();

    // https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getadaptersaddresses#examples
    // Try to retrieve adapter information up to 3 times
    for _ in 0..3 {
        iterator = iterator.realloc(size as _)?;
        ret = GetAdaptersAddresses(
            AF_UNSPEC.0.into(),
            GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_DNS_SERVER,
            None,
            Some(iterator.buf),
            &mut size,
        );
        if ret == ERROR_SUCCESS.0 {
            return Ok(iterator);
        } else if ret != ERROR_BUFFER_OVERFLOW.0 {
            break;
        }
        // if the given memory size is too small to hold the adapter information,
        // the SizePointer returned will point to the required size of the buffer,
        // and we should continue.
        // Otherwise, break the loop and check the return code again
    }

    Err(format!("GetAdaptersAddresses() failed with code {ret}"))
}

fn get_ip_networks(mut prefixes_ptr: *mut IP_ADAPTER_UNICAST_ADDRESS_LH) -> HashSet<IpNetwork> {
    let mut ip_networks = HashSet::new();
    while !prefixes_ptr.is_null() {
        let prefix = unsafe { prefixes_ptr.read_unaligned() };
        if let Some(socket_address) = NonNull::new(prefix.Address.lpSockaddr) {
            if let Some(ipaddr) = get_ip_address_from_socket_address(socket_address) {
                ip_networks.insert(IpNetwork {
                    addr: ipaddr,
                    prefix: prefix.OnLinkPrefixLength,
                });
            }
        }
        prefixes_ptr = prefix.Next;
    }
    ip_networks
}

/// Converts a Windows socket address to an ip address.
fn get_ip_address_from_socket_address(socket_address: NonNull<SOCKADDR>) -> Option<IpAddr> {
    let socket_address_family = unsafe { socket_address.as_ref().sa_family };
    match socket_address_family {
        AF_INET => {
            let socket_address = unsafe { socket_address.cast::<SOCKADDR_IN>().as_ref() };
            let address = unsafe { socket_address.sin_addr.S_un.S_addr };
            let ipv4_address = IpAddr::from(address.to_ne_bytes());
            Some(ipv4_address)
        }
        AF_INET6 => {
            let socket_address = unsafe { socket_address.cast::<SOCKADDR_IN6>().as_ref() };
            let address = unsafe { socket_address.sin6_addr.u.Byte };
            let ipv6_address = IpAddr::from(address);
            Some(ipv6_address)
        }
        _ => None,
    }
}
