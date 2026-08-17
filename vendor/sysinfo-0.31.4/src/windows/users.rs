// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::to_utf8_str;
use crate::{windows::sid::Sid, Gid, Group, GroupInner, Uid, User};

use std::ptr::null_mut;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{ERROR_MORE_DATA, LUID};
use windows::Win32::NetworkManagement::NetManagement::{
    NERR_Success, NetApiBufferFree, NetUserEnum, NetUserGetInfo, NetUserGetLocalGroups,
    FILTER_NORMAL_ACCOUNT, LG_INCLUDE_INDIRECT, LOCALGROUP_USERS_INFO_0, MAX_PREFERRED_LENGTH,
    USER_INFO_0, USER_INFO_23,
};
use windows::Win32::Security::Authentication::Identity::{
    LsaEnumerateLogonSessions, LsaFreeReturnBuffer, LsaGetLogonSessionData,
    SECURITY_LOGON_SESSION_DATA, SECURITY_LOGON_TYPE,
};

pub(crate) struct UserInner {
    pub(crate) uid: Uid,
    pub(crate) gid: Gid,
    pub(crate) name: String,
    c_user_name: Option<Vec<u16>>,
    is_local: bool,
}

impl UserInner {
    fn new(uid: Uid, name: String, c_name: PCWSTR, is_local: bool) -> Self {
        let c_user_name = if c_name.is_null() {
            None
        } else {
            Some(unsafe { c_name.as_wide() }.into())
        };
        Self {
            uid,
            gid: Gid(0),
            name,
            c_user_name,
            is_local,
        }
    }

    pub(crate) fn id(&self) -> &Uid {
        &self.uid
    }

    pub(crate) fn group_id(&self) -> Gid {
        self.gid
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn groups(&self) -> Vec<Group> {
        if let (Some(c_user_name), true) = (&self.c_user_name, self.is_local) {
            unsafe { get_groups_for_user(PCWSTR(c_user_name.as_ptr())) }
        } else {
            Vec::new()
        }
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

struct LsaBuffer<T>(*mut T);

impl<T> Drop for LsaBuffer<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _r = unsafe { LsaFreeReturnBuffer(self.0 as *mut _) };
        }
    }
}

impl<T> Default for LsaBuffer<T> {
    fn default() -> Self {
        Self(null_mut())
    }
}

impl<T> LsaBuffer<T> {
    pub fn inner_mut(&mut self) -> &mut *mut T {
        assert!(self.0.is_null());
        &mut self.0
    }
}

unsafe fn get_groups_for_user(username: PCWSTR) -> Vec<Group> {
    let mut buf: NetApiBuffer<LOCALGROUP_USERS_INFO_0> = Default::default();
    let mut nb_entries = 0;
    let mut total_entries = 0;
    let mut groups;

    let status = NetUserGetLocalGroups(
        w!(""),
        username,
        0,
        LG_INCLUDE_INDIRECT,
        buf.inner_mut_as_bytes(),
        MAX_PREFERRED_LENGTH,
        &mut nb_entries,
        &mut total_entries,
    );

    if status == NERR_Success {
        groups = Vec::with_capacity(nb_entries as _);
        if !buf.0.is_null() {
            let entries = std::slice::from_raw_parts(buf.0, nb_entries as _);
            groups.extend(entries.iter().map(|entry| Group {
                inner: GroupInner::new(Gid(0), to_utf8_str(entry.lgrui0_name)),
            }));
        }
    } else {
        groups = Vec::new();
        sysinfo_debug!("NetUserGetLocalGroups failed with ret code {}", status);
    }

    groups
}

pub(crate) fn get_users(users: &mut Vec<User>) {
    users.clear();

    let mut resume_handle: u32 = 0;
    unsafe {
        loop {
            let mut buffer: NetApiBuffer<USER_INFO_0> = Default::default();
            let mut nb_read = 0;
            let mut total = 0;
            let status = NetUserEnum(
                PCWSTR::null(),
                0,
                FILTER_NORMAL_ACCOUNT,
                buffer.inner_mut_as_bytes(),
                MAX_PREFERRED_LENGTH,
                &mut nb_read,
                &mut total,
                Some(&mut resume_handle),
            );
            if status == NERR_Success || status == ERROR_MORE_DATA.0 {
                let entries = std::slice::from_raw_parts(buffer.0, nb_read as _);
                for entry in entries {
                    if entry.usri0_name.is_null() {
                        continue;
                    }

                    let mut user: NetApiBuffer<USER_INFO_23> = Default::default();
                    if NetUserGetInfo(
                        PCWSTR::null(),
                        PCWSTR::from_raw(entry.usri0_name.as_ptr()),
                        23,
                        user.inner_mut_as_bytes(),
                    ) == NERR_Success
                    {
                        if let Some(sid) = Sid::from_psid((*user.0).usri23_user_sid) {
                            // Get the account name from the SID (because it's usually
                            // a better name), but fall back to the name we were given
                            // if this fails.
                            let name = sid
                                .account_name()
                                .unwrap_or_else(|| to_utf8_str(entry.usri0_name));
                            users.push(User {
                                inner: UserInner::new(
                                    Uid(sid),
                                    name,
                                    PCWSTR(entry.usri0_name.0 as *const _),
                                    true,
                                ),
                            });
                        }
                    }
                }
            } else {
                sysinfo_debug!(
                    "NetUserEnum error: {}",
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

        // First part done. Second part now!
        let mut nb_sessions = 0;
        let mut uids: LsaBuffer<LUID> = Default::default();
        if LsaEnumerateLogonSessions(&mut nb_sessions, uids.inner_mut()).is_err() {
            sysinfo_debug!("LsaEnumerateLogonSessions failed");
        } else {
            let entries = std::slice::from_raw_parts_mut(uids.0, nb_sessions as _);
            for entry in entries {
                let mut data: LsaBuffer<SECURITY_LOGON_SESSION_DATA> = Default::default();
                if LsaGetLogonSessionData(entry, data.inner_mut()).is_ok() && !data.0.is_null() {
                    let data = *data.0;
                    if data.LogonType == SECURITY_LOGON_TYPE::Network.0 as u32 {
                        continue;
                    }

                    let sid = match Sid::from_psid(data.Sid) {
                        Some(sid) => sid,
                        None => continue,
                    };

                    if users.iter().any(|u| u.inner.uid.0 == sid) {
                        continue;
                    }

                    // Get the account name from the SID (because it's usually
                    // a better name), but fall back to the name we were given
                    // if this fails.
                    let name = sid.account_name().unwrap_or_else(|| {
                        String::from_utf16(std::slice::from_raw_parts(
                            data.UserName.Buffer.as_ptr(),
                            data.UserName.Length as usize / std::mem::size_of::<u16>(),
                        ))
                        .unwrap_or_else(|_err| {
                            sysinfo_debug!("Failed to convert from UTF-16 string: {}", _err);
                            String::new()
                        })
                    });

                    users.push(User {
                        inner: UserInner::new(Uid(sid), name, PCWSTR::null(), false),
                    });
                }
            }
        }
    }
}
