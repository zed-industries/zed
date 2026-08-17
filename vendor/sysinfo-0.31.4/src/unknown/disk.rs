// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Disk, DiskKind};

use std::{ffi::OsStr, path::Path};

pub(crate) struct DiskInner;

impl DiskInner {
    pub(crate) fn kind(&self) -> DiskKind {
        unreachable!()
    }

    pub(crate) fn name(&self) -> &OsStr {
        unreachable!()
    }

    pub(crate) fn file_system(&self) -> &OsStr {
        Default::default()
    }

    pub(crate) fn mount_point(&self) -> &Path {
        Path::new("")
    }

    pub(crate) fn total_space(&self) -> u64 {
        0
    }

    pub(crate) fn available_space(&self) -> u64 {
        0
    }

    pub(crate) fn is_removable(&self) -> bool {
        false
    }

    pub(crate) fn refresh(&mut self) -> bool {
        true
    }
}

pub(crate) struct DisksInner {
    pub(crate) disks: Vec<Disk>,
}

impl DisksInner {
    pub(crate) fn new() -> Self {
        Self { disks: Vec::new() }
    }

    pub(crate) fn from_vec(disks: Vec<Disk>) -> Self {
        Self { disks }
    }

    pub(crate) fn into_vec(self) -> Vec<Disk> {
        self.disks
    }

    pub(crate) fn refresh_list(&mut self) {
        // Does nothing.
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}
