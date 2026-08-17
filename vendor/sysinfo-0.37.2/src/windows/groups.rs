// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::to_utf8_str;
use crate::{Gid, Group, GroupInner};

use std::ptr::null_mut;
use windows::Win32::Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS};
use windows::Win32::NetworkManagement::NetManagement::{
    LOCALGROUP_INFO_0, MAX_PREFERRED_LENGTH, NetApiBufferFree, NetLocalGroupEnum,
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

struct NetApiBuffer(*mut LOCALGROUP_INFO_0);

impl Drop for NetApiBuffer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { NetApiBufferFree(Some(self.0.cast())) };
        }
    }
}

impl Default for NetApiBuffer {
    fn default() -> Self {
        Self(null_mut())
    }
}

impl NetApiBuffer {
    pub fn inner_mut(&mut self) -> *mut *mut LOCALGROUP_INFO_0 {
        &mut self.0 as *mut _
    }
}

pub(crate) fn get_groups(groups: &mut Vec<Group>) {
    groups.clear();

    unsafe {
        let mut nb_entries = 0;
        let mut total_entries_hint = 0;
        let mut handle = 0;
        loop {
            let mut buff = NetApiBuffer::default();
            let res = NetLocalGroupEnum(
                None,
                0, // Level. Here, just get the group names.
                buff.inner_mut() as *mut _,
                MAX_PREFERRED_LENGTH,
                &mut nb_entries,
                &mut total_entries_hint,
                Some(&mut handle),
            );
            if res != ERROR_SUCCESS.0 && res != ERROR_MORE_DATA.0 {
                sysinfo_debug!("NetLocalGroupEnum failed: {res:?}");
                break;
            }
            let entries = std::slice::from_raw_parts(buff.0, nb_entries as usize);
            for entry in entries {
                let name = to_utf8_str(entry.lgrpi0_name);
                groups.push(Group {
                    inner: GroupInner::new(Gid(0), name),
                });
            }
            if res != ERROR_MORE_DATA.0 {
                break;
            }
        }
    }
}
