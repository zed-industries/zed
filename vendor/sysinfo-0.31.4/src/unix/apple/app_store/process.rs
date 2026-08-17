// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::{OsStr, OsString};
use std::path::Path;

use crate::{DiskUsage, Gid, Pid, ProcessStatus, Signal, Uid};

pub(crate) struct ProcessInner;

impl ProcessInner {
    pub(crate) fn kill_with(&self, _signal: Signal) -> Option<bool> {
        None
    }

    pub(crate) fn name(&self) -> &OsStr {
        OsStr::new("")
    }

    pub(crate) fn cmd(&self) -> &[OsString] {
        &[]
    }

    pub(crate) fn exe(&self) -> Option<&Path> {
        None
    }

    pub(crate) fn pid(&self) -> Pid {
        Pid(0)
    }

    pub(crate) fn environ(&self) -> &[OsString] {
        &[]
    }

    pub(crate) fn cwd(&self) -> Option<&Path> {
        None
    }

    pub(crate) fn root(&self) -> Option<&Path> {
        None
    }

    pub(crate) fn memory(&self) -> u64 {
        0
    }

    pub(crate) fn virtual_memory(&self) -> u64 {
        0
    }

    pub(crate) fn parent(&self) -> Option<Pid> {
        None
    }

    pub(crate) fn status(&self) -> ProcessStatus {
        ProcessStatus::Unknown(0)
    }

    pub(crate) fn start_time(&self) -> u64 {
        0
    }

    pub(crate) fn run_time(&self) -> u64 {
        0
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        0.0
    }

    pub(crate) fn disk_usage(&self) -> DiskUsage {
        DiskUsage::default()
    }

    pub(crate) fn user_id(&self) -> Option<&Uid> {
        None
    }

    pub(crate) fn effective_user_id(&self) -> Option<&Uid> {
        None
    }

    pub(crate) fn group_id(&self) -> Option<Gid> {
        None
    }

    pub(crate) fn effective_group_id(&self) -> Option<Gid> {
        None
    }

    pub(crate) fn wait(&self) {}

    pub(crate) fn session_id(&self) -> Option<Pid> {
        None
    }
}
