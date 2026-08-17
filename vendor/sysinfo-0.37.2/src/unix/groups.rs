// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Gid, Group, GroupInner};

impl GroupInner {
    pub(crate) fn new(id: crate::Gid, name: String) -> Self {
        Self { id, name }
    }

    pub(crate) fn id(&self) -> &crate::Gid {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn get_groups(groups: &mut Vec<Group>) {
    groups.clear();

    let mut groups_map = std::collections::HashMap::with_capacity(10);

    unsafe {
        libc::setgrent();
        loop {
            let gr = libc::getgrent();
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
        libc::endgrent();
    }
    for (name, gid) in groups_map {
        groups.push(Group {
            inner: GroupInner::new(gid, name),
        });
    }
}
