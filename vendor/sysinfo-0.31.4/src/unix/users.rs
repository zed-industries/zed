// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    common::{Gid, Uid},
    Group,
};

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use crate::User;

use libc::{getgrgid_r, getgrouplist};

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
            if last_errno == libc::ERANGE as _ {
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

pub(crate) unsafe fn get_user_groups(
    name: *const libc::c_char,
    group_id: libc::gid_t,
) -> Vec<Group> {
    let mut buffer = Vec::with_capacity(2048);
    let mut groups = Vec::with_capacity(256);

    loop {
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

// Not used by mac.
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub(crate) fn get_users(users: &mut Vec<User>) {
    use std::fs::File;
    use std::io::Read;

    #[inline]
    fn parse_id(id: &str) -> Option<u32> {
        id.parse::<u32>().ok()
    }

    users.clear();

    let mut s = String::new();

    let _ = File::open("/etc/passwd").and_then(|mut f| f.read_to_string(&mut s));
    for line in s.lines() {
        let mut parts = line.split(':');
        if let Some(username) = parts.next() {
            let mut parts = parts.skip(1);
            // Skip the user if the uid cannot be parsed correctly
            if let Some(uid) = parts.next().and_then(parse_id) {
                if let Some(group_id) = parts.next().and_then(parse_id) {
                    users.push(User {
                        inner: UserInner::new(Uid(uid), Gid(group_id), username.to_owned()),
                    });
                }
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) use crate::unix::apple::users::get_users;
