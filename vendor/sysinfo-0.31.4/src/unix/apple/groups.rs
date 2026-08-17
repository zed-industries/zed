// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Gid, Group, GroupInner};

use libc::{endgrent, getgrent, setgrent};
use std::collections::HashMap;

pub(crate) fn get_groups(groups: &mut Vec<Group>) {
    groups.clear();

    let mut groups_map = HashMap::with_capacity(10);

    unsafe {
        setgrent();
        loop {
            let gr = getgrent();
            if gr.is_null() {
                // The call was interrupted by a signal, retrying.
                if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                break;
            }

            if let Some(name) = crate::unix::utils::cstr_to_rust((*gr).gr_name) {
                if groups_map.contains_key(&name) {
                    continue;
                }

                let gid = (*gr).gr_gid;
                groups_map.insert(name, Gid(gid));
            }
        }
        endgrent();
    }
    for (name, gid) in groups_map {
        groups.push(Group {
            inner: GroupInner::new(gid, name),
        });
    }
}
