use crate::rt::{execution, thread, MAX_THREADS};

#[cfg(feature = "checkpoint")]
use serde::{Deserialize, Serialize};
use std::cmp;
use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "checkpoint", derive(Serialize, Deserialize))]
pub(crate) struct VersionVec {
    versions: [u16; MAX_THREADS],
}

impl VersionVec {
    pub(crate) fn new() -> VersionVec {
        VersionVec {
            versions: [0; MAX_THREADS],
        }
    }

    pub(crate) fn versions(
        &self,
        execution_id: execution::Id,
    ) -> impl Iterator<Item = (thread::Id, u16)> + '_ {
        self.versions
            .iter()
            .enumerate()
            .map(move |(thread_id, &version)| (thread::Id::new(execution_id, thread_id), version))
    }

    pub(crate) fn inc(&mut self, id: thread::Id) {
        self.versions[id.as_usize()] += 1;
    }

    pub(crate) fn join(&mut self, other: &VersionVec) {
        for (i, &version) in other.versions.iter().enumerate() {
            self.versions[i] = cmp::max(self.versions[i], version);
        }
    }

    /// Returns the thread ID, if any, that is ahead of the current version.
    pub(crate) fn ahead(&self, other: &VersionVec) -> Option<usize> {
        for (i, &version) in other.versions.iter().enumerate() {
            if self.versions[i] < version {
                return Some(i);
            }
        }

        None
    }
}

impl cmp::PartialOrd for VersionVec {
    fn partial_cmp(&self, other: &VersionVec) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;

        let mut ret = Equal;

        for i in 0..MAX_THREADS {
            let a = self.versions[i];
            let b = other.versions[i];
            match a.cmp(&b) {
                Equal => {}
                Less if ret == Greater => return None,
                Greater if ret == Less => return None,
                ordering => ret = ordering,
            }
        }

        Some(ret)
    }
}

impl ops::Index<thread::Id> for VersionVec {
    type Output = u16;

    fn index(&self, index: thread::Id) -> &u16 {
        self.versions.index(index.as_usize())
    }
}

impl ops::IndexMut<thread::Id> for VersionVec {
    fn index_mut(&mut self, index: thread::Id) -> &mut u16 {
        self.versions.index_mut(index.as_usize())
    }
}
