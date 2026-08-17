// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Gid, Group, Uid, User};

pub(crate) struct UserInner;

impl UserInner {
    pub(crate) fn id(&self) -> &Uid {
        &Uid(0)
    }

    pub(crate) fn group_id(&self) -> Gid {
        Gid(0)
    }

    pub(crate) fn name(&self) -> &str {
        ""
    }

    pub(crate) fn groups(&self) -> Vec<Group> {
        Vec::new()
    }
}

pub(crate) fn get_users(_: &mut Vec<User>) {}
