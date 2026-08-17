// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    Group, User,
    common::{Gid, Uid},
};
use libc::{getgrgid_r, getgrouplist};

#[cfg(not(target_os = "android"))]
use libc::{endpwent, getpwent, setpwent};

// See `https://github.com/rust-lang/libc/issues/3014`.
#[cfg(target_os = "android")]
unsafe extern "C" {
    fn getpwent() -> *mut libc::passwd;
    fn setpwent();
    fn endpwent();
}

pub(crate) struct UserInner {
    pub(crate) uid: Uid,
    pub(crate) gid: Gid,
    pub(crate) name: String,
    c_user: Vec<u8>,
}

impl UserInner {
    pub(crate) fn new(uid: Uid, gid: Gid, name: String) -> Self {
        let mut c_user = name.as_bytes().to_vec();
        c_user.push(0);
        Self {
            uid,
            gid,
            name,
            c_user,
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
        unsafe { get_user_groups(self.c_user.as_ptr() as *const _, self.gid.0 as _) }
    }
}

pub(crate) unsafe fn get_group_name(
    id: libc::gid_t,
    buffer: &mut Vec<libc::c_char>,
) -> Option<String> {
    let mut g = std::mem::MaybeUninit::<libc::group>::uninit();
    let mut tmp_ptr = std::ptr::null_mut();
    let mut last_errno = 0;

    unsafe {
        loop {
            if retry_eintr!(set_to_0 => last_errno => getgrgid_r(
                id as _,
                g.as_mut_ptr() as _,
                buffer.as_mut_ptr(),
                buffer.capacity() as _,
                &mut tmp_ptr as _
            )) != 0
            {
                // If there was not enough memory, we give it more.
                if last_errno == libc::ERANGE as libc::c_int {
                    // Needs to be updated for `Vec::reserve` to actually add additional capacity.
                    // In here it's "fine" since we never read from `buffer`.
                    buffer.set_len(buffer.capacity());
                    buffer.reserve(2048);
                    continue;
                }
                return None;
            }
            break;
        }
        let g = g.assume_init();
        super::utils::cstr_to_rust(g.gr_name)
    }
}

pub(crate) unsafe fn get_user_groups(
    name: *const libc::c_char,
    group_id: libc::gid_t,
) -> Vec<Group> {
    let mut buffer = Vec::with_capacity(2048);
    let mut groups = Vec::with_capacity(256);

    loop {
        unsafe {
            let mut nb_groups = groups.capacity();
            if getgrouplist(
                name,
                group_id as _,
                groups.as_mut_ptr(),
                &mut nb_groups as *mut _ as *mut _,
            ) == -1
            {
                // Ensure the length matches the number of returned groups.
                // Needs to be updated for `Vec::reserve` to actually add additional capacity.
                groups.set_len(nb_groups as _);
                groups.reserve(256);
                continue;
            }
            groups.set_len(nb_groups as _);
            return groups
                .iter()
                .filter_map(|group_id| {
                    let name = get_group_name(*group_id as _, &mut buffer)?;
                    Some(Group {
                        inner: crate::GroupInner::new(Gid(*group_id as _), name),
                    })
                })
                .collect();
        }
    }
}

pub(crate) fn get_users(users: &mut Vec<User>) {
    fn filter(shell: *const std::ffi::c_char, uid: u32) -> bool {
        !endswith(shell, b"/false") && !endswith(shell, b"/uucico") && uid < 65536
    }

    users.clear();

    let mut users_map = std::collections::HashMap::with_capacity(10);

    unsafe {
        setpwent();
        loop {
            let pw = getpwent();
            if pw.is_null() {
                // The call was interrupted by a signal, retrying.
                if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                break;
            }

            if !filter((*pw).pw_shell, (*pw).pw_uid) {
                // This is not a "real" or "local" user.
                continue;
            }
            if let Some(name) = crate::unix::utils::cstr_to_rust((*pw).pw_name) {
                if users_map.contains_key(&name) {
                    continue;
                }

                let uid = (*pw).pw_uid;
                let gid = (*pw).pw_gid;
                users_map.insert(name, (Uid(uid), Gid(gid)));
            }
        }
        endpwent();
    }
    for (name, (uid, gid)) in users_map {
        users.push(User {
            inner: UserInner::new(uid, gid, name),
        });
    }
}

fn endswith(s1: *const std::ffi::c_char, s2: &[u8]) -> bool {
    if s1.is_null() {
        return false;
    }
    unsafe {
        let mut len = libc::strlen(s1) as isize - 1;
        let mut i = s2.len() as isize - 1;
        while len >= 0 && i >= 0 && *s1.offset(len) == s2[i as usize] as std::ffi::c_char {
            i -= 1;
            len -= 1;
        }
        i == -1
    }
}
