// Take a look at the license at the top of the repository in the LICENSE file.

use libc::{
    self, CTL_NET, IFNAMSIZ, NET_RT_IFLIST2, PF_ROUTE, RTM_IFINFO2, c_char, c_int, c_uint,
    if_data64, if_msghdr2, sysctl,
};

use std::collections::{HashMap, hash_map};
use std::mem::{MaybeUninit, size_of};
use std::ptr::null_mut;

use crate::network::refresh_networks_addresses;
use crate::{IpNetwork, MacAddr, NetworkData};

// FIXME: To be removed once https://github.com/rust-lang/libc/pull/4022 is merged and released.
#[repr(C)]
struct ifmibdata {
    ifmd_name: [c_char; IFNAMSIZ],
    ifmd_pcount: c_uint,
    ifmd_flags: c_uint,
    ifmd_snd_len: c_uint,
    ifmd_snd_maxlen: c_uint,
    ifmd_snd_drops: c_uint,
    ifmd_filler: [c_uint; 4],
    ifmd_data: if_data64,
}
// FIXME: To be removed once https://github.com/rust-lang/libc/pull/4022 is merged and released.
pub const IFDATA_GENERAL: c_int = 1;
// FIXME: To be removed once https://github.com/rust-lang/libc/pull/4022 is merged and released.
pub const IFMIB_IFDATA: c_int = 2;
// FIXME: To be removed once https://github.com/rust-lang/libc/pull/4022 is merged and released.
pub const NETLINK_GENERIC: c_int = 0;

#[inline]
fn update_field(old_field: &mut u64, new_field: &mut u64, value: u64) {
    *old_field = *new_field;
    *new_field = value;
}

fn update_network_data(inner: &mut NetworkDataInner, data: &if_data64) {
    update_field(&mut inner.old_out, &mut inner.current_out, data.ifi_obytes);
    update_field(&mut inner.old_in, &mut inner.current_in, data.ifi_ibytes);

    update_field(
        &mut inner.old_packets_out,
        &mut inner.packets_out,
        data.ifi_opackets,
    );
    update_field(
        &mut inner.old_packets_in,
        &mut inner.packets_in,
        data.ifi_ipackets,
    );

    update_field(
        &mut inner.old_errors_in,
        &mut inner.errors_in,
        data.ifi_ierrors,
    );
    update_field(
        &mut inner.old_errors_out,
        &mut inner.errors_out,
        data.ifi_oerrors,
    );
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

    pub(crate) fn refresh(&mut self, remove_not_listed_interfaces: bool) {
        self.update_networks();
        if remove_not_listed_interfaces {
            self.interfaces.retain(|_, i| {
                if !i.inner.updated {
                    return false;
                }
                i.inner.updated = false;
                true
            });
        }
        refresh_networks_addresses(&mut self.interfaces);
    }

    #[allow(clippy::cast_ptr_alignment)]
    #[allow(clippy::uninit_vec)]
    fn update_networks(&mut self) {
        let mib = &mut [CTL_NET, PF_ROUTE, 0, 0, NET_RT_IFLIST2, 0];
        let mib2 = &mut [
            CTL_NET,
            libc::PF_LINK,
            NETLINK_GENERIC,
            IFMIB_IFDATA,
            0,
            IFDATA_GENERAL,
        ];

        let mut len = 0;
        unsafe {
            if sysctl(
                mib.as_mut_ptr(),
                mib.len() as _,
                null_mut(),
                &mut len,
                null_mut(),
                0,
            ) < 0
            {
                // TODO: might be nice to put an error in here...
                return;
            }
            let mut buf = Vec::with_capacity(len);
            buf.set_len(len);
            if sysctl(
                mib.as_mut_ptr(),
                mib.len() as _,
                buf.as_mut_ptr(),
                &mut len,
                null_mut(),
                0,
            ) < 0
            {
                // TODO: might be nice to put an error in here...
                return;
            }
            let buf = buf.as_ptr() as *const c_char;
            let lim = buf.add(len);
            let mut next = buf;

            while next < lim {
                let ifm = next as *const libc::if_msghdr;
                next = next.offset((*ifm).ifm_msglen as isize);
                if (*ifm).ifm_type == RTM_IFINFO2 as u8 {
                    // The interface (line description) name stored at ifname will be returned in
                    // the default coded character set identifier (CCSID) currently in effect for
                    // the job. If this is not a single byte CCSID, then storage greater than
                    // IFNAMSIZ (16) bytes may be needed. 22 bytes is large enough for all CCSIDs.
                    let mut name = vec![0u8; libc::IFNAMSIZ + 6];

                    let if2m: *const if_msghdr2 = ifm as *const if_msghdr2;
                    let pname =
                        libc::if_indextoname((*if2m).ifm_index as _, name.as_mut_ptr() as _);
                    if pname.is_null() {
                        continue;
                    }
                    name.set_len(libc::strlen(pname));
                    let name = String::from_utf8_unchecked(name);
                    let mtu = (*if2m).ifm_data.ifi_mtu as u64;

                    // Because data size is capped at 32 bits with the previous sysctl call for some
                    // reasons, we need to make another sysctl call to get the actual values
                    // we originally got into `ifm.ifm_data`...
                    //
                    // Issue: https://github.com/GuillaumeGomez/sysinfo/issues/1378
                    let mut mib_data: MaybeUninit<ifmibdata> = MaybeUninit::uninit();

                    mib2[4] = (*if2m).ifm_index as _;
                    let ret = sysctl(
                        mib2.as_mut_ptr(),
                        mib2.len() as _,
                        mib_data.as_mut_ptr() as *mut _,
                        &mut size_of::<ifmibdata>(),
                        null_mut(),
                        0,
                    );

                    match self.interfaces.entry(name) {
                        hash_map::Entry::Occupied(mut e) => {
                            let interface = e.get_mut();
                            let interface = &mut interface.inner;

                            if ret < 0 {
                                sysinfo_debug!(
                                    "Cannot get network interface data usage: sysctl failed: {ret}"
                                );
                            } else {
                                let data = mib_data.assume_init();
                                update_network_data(interface, &data.ifmd_data);
                            }
                            if interface.mtu != mtu {
                                interface.mtu = mtu
                            }
                            interface.updated = true;
                        }
                        hash_map::Entry::Vacant(e) => {
                            let current_in;
                            let current_out;
                            let packets_in;
                            let packets_out;
                            let errors_in;
                            let errors_out;

                            if ret < 0 {
                                sysinfo_debug!(
                                    "Cannot get network interface data usage: sysctl failed: {ret}"
                                );

                                current_in = 0;
                                current_out = 0;
                                packets_in = 0;
                                packets_out = 0;
                                errors_in = 0;
                                errors_out = 0;
                            } else {
                                let data = mib_data.assume_init();
                                let data = data.ifmd_data;

                                current_in = data.ifi_ibytes;
                                current_out = data.ifi_obytes;
                                packets_in = data.ifi_ipackets;
                                packets_out = data.ifi_opackets;
                                errors_in = data.ifi_ierrors;
                                errors_out = data.ifi_oerrors;
                            }

                            e.insert(NetworkData {
                                inner: NetworkDataInner {
                                    current_in,
                                    old_in: current_in,
                                    current_out,
                                    old_out: current_out,
                                    packets_in,
                                    old_packets_in: packets_in,
                                    packets_out,
                                    old_packets_out: packets_out,
                                    errors_in,
                                    old_errors_in: errors_in,
                                    errors_out,
                                    old_errors_out: errors_out,
                                    updated: true,
                                    mac_addr: MacAddr::UNSPECIFIED,
                                    ip_networks: vec![],
                                    mtu,
                                },
                            });
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct NetworkDataInner {
    current_in: u64,
    old_in: u64,
    current_out: u64,
    old_out: u64,
    packets_in: u64,
    old_packets_in: u64,
    packets_out: u64,
    old_packets_out: u64,
    errors_in: u64,
    old_errors_in: u64,
    errors_out: u64,
    old_errors_out: u64,
    updated: bool,
    /// MAC address
    pub(crate) mac_addr: MacAddr,
    /// IP networks
    pub(crate) ip_networks: Vec<IpNetwork>,
    /// Interface Maximum Transfer Unit (MTU)
    mtu: u64,
}

impl NetworkDataInner {
    pub(crate) fn received(&self) -> u64 {
        self.current_in.saturating_sub(self.old_in)
    }

    pub(crate) fn total_received(&self) -> u64 {
        self.current_in
    }

    pub(crate) fn transmitted(&self) -> u64 {
        self.current_out.saturating_sub(self.old_out)
    }

    pub(crate) fn total_transmitted(&self) -> u64 {
        self.current_out
    }

    pub(crate) fn packets_received(&self) -> u64 {
        self.packets_in.saturating_sub(self.old_packets_in)
    }

    pub(crate) fn total_packets_received(&self) -> u64 {
        self.packets_in
    }

    pub(crate) fn packets_transmitted(&self) -> u64 {
        self.packets_out.saturating_sub(self.old_packets_out)
    }

    pub(crate) fn total_packets_transmitted(&self) -> u64 {
        self.packets_out
    }

    pub(crate) fn errors_on_received(&self) -> u64 {
        self.errors_in.saturating_sub(self.old_errors_in)
    }

    pub(crate) fn total_errors_on_received(&self) -> u64 {
        self.errors_in
    }

    pub(crate) fn errors_on_transmitted(&self) -> u64 {
        self.errors_out.saturating_sub(self.old_errors_out)
    }

    pub(crate) fn total_errors_on_transmitted(&self) -> u64 {
        self.errors_out
    }

    pub(crate) fn mac_address(&self) -> MacAddr {
        self.mac_addr
    }

    pub(crate) fn ip_networks(&self) -> &[IpNetwork] {
        &self.ip_networks
    }

    pub(crate) fn mtu(&self) -> u64 {
        self.mtu
    }
}
