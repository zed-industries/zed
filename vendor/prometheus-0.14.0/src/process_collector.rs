// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

//! Monitor a process.
//!
//! This module only supports **Linux** platform.

use lazy_static::lazy_static;

use crate::counter::IntCounter;
use crate::desc::Desc;
use crate::gauge::IntGauge;
use crate::metrics::{Collector, Opts};
use crate::proto;

/// The `pid_t` data type represents process IDs.
pub use libc::pid_t;

/// Seven metrics per ProcessCollector.
const METRICS_NUMBER: usize = 7;

/// A collector which exports the current state of process metrics including
/// CPU, memory and file descriptor usage, thread count, as well as the process
/// start time for the given process id.
#[derive(Debug)]
pub struct ProcessCollector {
    pid: pid_t,
    descs: Vec<Desc>,
    cpu_total: IntCounter,
    open_fds: IntGauge,
    max_fds: IntGauge,
    vsize: IntGauge,
    rss: IntGauge,
    start_time: IntGauge,
    threads: IntGauge,
}

impl ProcessCollector {
    /// Create a `ProcessCollector` with the given process id and namespace.
    pub fn new<S: Into<String>>(pid: pid_t, namespace: S) -> ProcessCollector {
        let namespace = namespace.into();
        let mut descs = Vec::new();

        let cpu_total = IntCounter::with_opts(
            Opts::new(
                "process_cpu_seconds_total",
                "Total user and system CPU time spent in \
                 seconds.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(cpu_total.desc().into_iter().cloned());

        let open_fds = IntGauge::with_opts(
            Opts::new("process_open_fds", "Number of open file descriptors.")
                .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(open_fds.desc().into_iter().cloned());

        let max_fds = IntGauge::with_opts(
            Opts::new(
                "process_max_fds",
                "Maximum number of open file descriptors.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(max_fds.desc().into_iter().cloned());

        let vsize = IntGauge::with_opts(
            Opts::new(
                "process_virtual_memory_bytes",
                "Virtual memory size in bytes.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(vsize.desc().into_iter().cloned());

        let rss = IntGauge::with_opts(
            Opts::new(
                "process_resident_memory_bytes",
                "Resident memory size in bytes.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(rss.desc().into_iter().cloned());

        let start_time = IntGauge::with_opts(
            Opts::new(
                "process_start_time_seconds",
                "Start time of the process since unix epoch \
                 in seconds.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        // proc_start_time init once because it is immutable
        if let Ok(boot_time) = procfs::boot_time_secs() {
            if let Ok(stat) = procfs::process::Process::myself().and_then(|p| p.stat()) {
                start_time.set(stat.starttime as i64 / *CLK_TCK + boot_time as i64);
            }
        }
        descs.extend(start_time.desc().into_iter().cloned());

        let threads = IntGauge::with_opts(
            Opts::new("process_threads", "Number of OS threads in the process.")
                .namespace(namespace),
        )
        .unwrap();
        descs.extend(threads.desc().into_iter().cloned());

        ProcessCollector {
            pid,
            descs,
            cpu_total,
            open_fds,
            max_fds,
            vsize,
            rss,
            start_time,
            threads,
        }
    }

    /// Return a `ProcessCollector` of the calling process.
    pub fn for_self() -> ProcessCollector {
        let pid = unsafe { libc::getpid() };
        ProcessCollector::new(pid, "")
    }
}

impl Collector for ProcessCollector {
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        let p = match procfs::process::Process::new(self.pid) {
            Ok(p) => p,
            Err(..) => {
                // we can't construct a Process object, so there's no stats to gather
                return Vec::new();
            }
        };

        // file descriptors
        if let Ok(fd_count) = p.fd_count() {
            self.open_fds.set(fd_count as i64);
        }
        if let Ok(limits) = p.limits() {
            if let procfs::process::LimitValue::Value(max) = limits.max_open_files.soft_limit {
                self.max_fds.set(max as i64)
            }
        }

        let mut cpu_total_mfs = None;
        if let Ok(stat) = p.stat() {
            // memory
            self.vsize.set(stat.vsize as i64);
            self.rss.set((stat.rss as i64) * *PAGESIZE);

            // cpu
            let total = (stat.utime + stat.stime) / *CLK_TCK as u64;
            let past = self.cpu_total.get();
            // If two threads are collecting metrics at the same time,
            // the cpu_total counter may have already been updated,
            // and the subtraction may underflow.
            self.cpu_total.inc_by(total.saturating_sub(past));
            cpu_total_mfs = Some(self.cpu_total.collect());

            // threads
            self.threads.set(stat.num_threads);
        }

        // collect MetricFamilys.
        let mut mfs = Vec::with_capacity(METRICS_NUMBER);
        if let Some(cpu) = cpu_total_mfs {
            mfs.extend(cpu);
        }
        mfs.extend(self.open_fds.collect());
        mfs.extend(self.max_fds.collect());
        mfs.extend(self.vsize.collect());
        mfs.extend(self.rss.collect());
        mfs.extend(self.start_time.collect());
        mfs.extend(self.threads.collect());
        mfs
    }
}

lazy_static! {
    // getconf CLK_TCK
    static ref CLK_TCK: i64 = {
        unsafe {
            libc::sysconf(libc::_SC_CLK_TCK)
        }.into()
    };

    // getconf PAGESIZE
    static ref PAGESIZE: i64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE)
        }.into()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::Collector;
    use crate::registry;

    #[test]
    fn test_process_collector() {
        let pc = ProcessCollector::for_self();
        {
            // Seven metrics per process collector.
            let descs = pc.desc();
            assert_eq!(descs.len(), super::METRICS_NUMBER);
            let mfs = pc.collect();
            assert_eq!(mfs.len(), super::METRICS_NUMBER);
        }

        let r = registry::Registry::new();
        let res = r.register(Box::new(pc));
        assert!(res.is_ok());
    }
}
