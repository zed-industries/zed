// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, Process, ProcessesToUpdate, ProcessRefreshKind};

use std::collections::HashMap;
use std::time::Duration;

declare_signals! {
    (),
    _ => None,
}

#[doc = include_str!("../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(0);

pub(crate) struct SystemInner {
    processes_list: HashMap<Pid, Process>,
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        Self {
            processes_list: Default::default(),
        }
    }

    pub(crate) fn refresh_memory_specifics(&mut self, _refresh_kind: MemoryRefreshKind) {}

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        None
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, _refresh_kind: CpuRefreshKind) {}

    pub(crate) fn refresh_cpu_list(&mut self, _refresh_kind: CpuRefreshKind) {}

    pub(crate) fn refresh_processes_specifics(
        &mut self,
        _processes_to_update: ProcessesToUpdate<'_>,
        _refresh_kind: ProcessRefreshKind,
    ) -> usize {
        0
    }

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    pub(crate) fn processes(&self) -> &HashMap<Pid, Process> {
        &self.processes_list
    }

    pub(crate) fn process(&self, _pid: Pid) -> Option<&Process> {
        None
    }

    pub(crate) fn global_cpu_usage(&self) -> f32 {
        0.
    }

    pub(crate) fn cpus(&self) -> &[Cpu] {
        &[]
    }

    pub(crate) fn physical_core_count(&self) -> Option<usize> {
        None
    }

    pub(crate) fn total_memory(&self) -> u64 {
        0
    }

    pub(crate) fn free_memory(&self) -> u64 {
        0
    }

    pub(crate) fn available_memory(&self) -> u64 {
        0
    }

    pub(crate) fn used_memory(&self) -> u64 {
        0
    }

    pub(crate) fn total_swap(&self) -> u64 {
        0
    }

    pub(crate) fn free_swap(&self) -> u64 {
        0
    }

    pub(crate) fn used_swap(&self) -> u64 {
        0
    }

    pub(crate) fn uptime() -> u64 {
        0
    }

    pub(crate) fn boot_time() -> u64 {
        0
    }

    pub(crate) fn load_average() -> LoadAvg {
        LoadAvg {
            one: 0.,
            five: 0.,
            fifteen: 0.,
        }
    }

    pub(crate) fn name() -> Option<String> {
        None
    }

    pub(crate) fn long_os_version() -> Option<String> {
        None
    }

    pub(crate) fn kernel_version() -> Option<String> {
        None
    }

    pub(crate) fn os_version() -> Option<String> {
        None
    }

    pub(crate) fn distribution_id() -> String {
        std::env::consts::OS.to_owned()
    }

    pub(crate) fn host_name() -> Option<String> {
        None
    }
    pub(crate) fn cpu_arch() -> Option<String> {
        None
    }
}
