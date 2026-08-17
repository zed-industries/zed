// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::to_utf8_str;
use crate::windows::sid::Sid;
use crate::{Gid, Group, GroupInner};

use std::ptr::null_mut;
use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_MORE_DATA;
use windows::Win32::NetworkManagement::NetManagement::{
    NERR_Success, NetApiBufferFree, NetGroupEnum, NetGroupGetInfo, GROUP_INFO_0, GROUP_INFO_3,
    MAX_PREFERRED_LENGTH,
};

impl GroupInner {
    pub(crate) fn new(id: Gid, name: String) -> Self {
        Self { id, name }
    }

    pub(crate) fn id(&self) -> &Gid {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

struct NetApiBuffer<T>(*mut T);

impl<T> Drop for NetApiBuffer<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { NetApiBufferFree(Some(self.0.cast())) };
        }
    }
}

impl<T> Default for NetApiBuffer<T> {
    fn default() -> Self {
        Self(null_mut())
    }
}

impl<T> NetApiBuffer<T> {
    pub fn inner_mut(&mut self) -> &mut *mut T {
        assert!(self.0.is_null());
        &mut self.0
    }

    pub unsafe fn inner_mut_as_bytes(&mut self) -> &mut *mut u8 {
        // https://doc.rust-lang.org/std/mem/fn.transmute.html
        // Turning an &mut T into an &mut U:
        &mut *(self.inner_mut() as *mut *mut T as *mut *mut u8)
    }
}

pub(crate) fn get_groups(groups: &mut Vec<Group>) {
    groups.clear();

    let mut resume_handle: usize = 0;
    unsafe {
        loop {
            let mut buffer: NetApiBuffer<GROUP_INFO_0> = Default::default();
            let mut nb_read = 0;
            let mut total = 0;
            let status = NetGroupEnum(
                PCWSTR::null(),
                0,
                buffer.inner_mut_as_bytes(),
                MAX_PREFERRED_LENGTH,
                &mut nb_read,
                &mut total,
                Some(&mut resume_handle),
            );
            if status == NERR_Success || status == ERROR_MORE_DATA.0 {
                let entries = std::slice::from_raw_parts(buffer.0, nb_read as _);
                for entry in entries {
                    if entry.grpi0_name.is_null() {
                        continue;
                    }

                    let mut group: NetApiBuffer<GROUP_INFO_3> = Default::default();
                    if NetGroupGetInfo(
                        PCWSTR::null(),
                        PCWSTR::from_raw(entry.grpi0_name.as_ptr()),
                        3,
                        group.inner_mut_as_bytes(),
                    ) == NERR_Success
                    {
                        if let Some(_sid) = Sid::from_psid((*group.0).grpi3_group_sid) {
                            // Get the account name from the SID (because it's usually
                            // a better name), but fall back to the name we were given
                            // if this fails.
                            let name = to_utf8_str(entry.grpi0_name);
                            groups.push(Group {
                                inner: GroupInner::new(Gid(0), name),
                            });
                        }
                    }
                }
            } else {
                sysinfo_debug!(
                    "NetGroupEnum error: {}",
                    if status == windows::Win32::Foundation::ERROR_ACCESS_DENIED.0 {
                        "access denied"
                    } else if status == windows::Win32::Foundation::ERROR_INVALID_LEVEL.0 {
                        "invalid level"
                    } else {
                        "unknown error"
                    }
                );
            }
            if status != ERROR_MORE_DATA.0 {
                break;
            }
        }
    }
}
