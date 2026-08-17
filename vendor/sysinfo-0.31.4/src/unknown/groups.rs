// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Gid, Group, GroupInner};

impl GroupInner {
    pub(crate) fn id(&self) -> &Gid {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn get_groups(_: &mut Vec<Group>) {}
