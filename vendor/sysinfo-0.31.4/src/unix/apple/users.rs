// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    common::{Gid, Uid},
    User, UserInner,
};

use libc::{c_char, endpwent, getpwent, setpwent, strlen};
use std::collections::HashMap;

fn endswith(s1: *const c_char, s2: &[u8]) -> bool {
    if s1.is_null() {
        return false;
    }
    unsafe {
        let mut len = strlen(s1) as isize - 1;
        let mut i = s2.len() as isize - 1;
        while len >= 0 && i >= 0 && *s1.offset(len) == s2[i as usize] as _ {
            i -= 1;
            len -= 1;
        }
        i == -1
    }
}

pub(crate) fn get_users(users: &mut Vec<User>) {
    fn filter(shell: *const c_char, uid: u32) -> bool {
        !endswith(shell, b"/false") && !endswith(shell, b"/uucico") && uid < 65536
    }

    users.clear();

    let mut users_map = HashMap::with_capacity(10);

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

// This was the OSX-based solution. It provides enough information, but what a mess!
// pub fn get_users_list() -> Vec<User> {
//     let mut users = Vec::new();
//     let node_name = b"/Local/Default\0";

//     unsafe {
//         let node_name = ffi::CFStringCreateWithCStringNoCopy(
//             std::ptr::null_mut(),
//             node_name.as_ptr() as *const c_char,
//             ffi::kCFStringEncodingMacRoman,
//             ffi::kCFAllocatorNull as *mut c_void,
//         );
//         let node_ref = ffi::ODNodeCreateWithName(
//             ffi::kCFAllocatorDefault,
//             ffi::kODSessionDefault,
//             node_name,
//             std::ptr::null_mut(),
//         );
//         let query = ffi::ODQueryCreateWithNode(
//             ffi::kCFAllocatorDefault,
//             node_ref,
//             ffi::kODRecordTypeUsers as _, // kODRecordTypeGroups
//             std::ptr::null(),
//             0,
//             std::ptr::null(),
//             std::ptr::null(),
//             0,
//             std::ptr::null_mut(),
//         );
//         if query.is_null() {
//             return users;
//         }
//         let results = ffi::ODQueryCopyResults(
//             query,
//             false as _,
//             std::ptr::null_mut(),
//         );
//         let len = ffi::CFArrayGetCount(results);
//         for i in 0..len {
//             let name = match get_user_name(ffi::CFArrayGetValueAtIndex(results, i)) {
//                 Some(n) => n,
//                 None => continue,
//             };
//             users.push(User { name });
//         }

//         ffi::CFRelease(results as *const c_void);
//         ffi::CFRelease(query as *const c_void);
//         ffi::CFRelease(node_ref as *const c_void);
//         ffi::CFRelease(node_name as *const c_void);
//     }
//     users.sort_unstable_by(|x, y| x.name.partial_cmp(&y.name).unwrap());
//     return users;
// }

// fn get_user_name(result: *const c_void) -> Option<String> {
//     let user_name = ffi::ODRecordGetRecordName(result as _);
//     let ptr = ffi::CFStringGetCharactersPtr(user_name);
//     String::from_utf16(&if ptr.is_null() {
//         let len = ffi::CFStringGetLength(user_name); // It returns the len in UTF-16 code pairs.
//         if len == 0 {
//             continue;
//         }
//         let mut v = Vec::with_capacity(len as _);
//         for x in 0..len {
//             v.push(ffi::CFStringGetCharacterAtIndex(user_name, x));
//         }
//         v
//     } else {
//         let mut v: Vec<u16> = Vec::new();
//         let mut x = 0;
//         loop {
//             let letter = *ptr.offset(x);
//             if letter == 0 {
//                 break;
//             }
//             v.push(letter);
//             x += 1;
//         }
//         v
//     }.ok()
// }
