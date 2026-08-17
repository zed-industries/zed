use crate::{ProcError, ProcResult};

use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Process limits
///
/// For more details about each of these limits, see the `getrlimit` man page.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Limits {
    /// Max Cpu Time
    ///
    /// This is a limit, in seconds, on the amount of CPU time that the process can consume.
    pub max_cpu_time: Limit,

    /// Max file size
    ///
    /// This is the maximum size in bytes of files that the process may create.
    pub max_file_size: Limit,

    /// Max data size
    ///
    /// This is the maximum size of the process's data segment (initialized data, uninitialized
    /// data, and heap).
    pub max_data_size: Limit,

    /// Max stack size
    ///
    /// This is the maximum size of the process stack, in bytes.
    pub max_stack_size: Limit,

    /// Max core file size
    ///
    /// This is the maximum size of a *core* file in bytes that the process may dump.
    pub max_core_file_size: Limit,

    /// Max resident set
    ///
    /// This is a limit (in bytes) on the process's resident set (the number of virtual pages
    /// resident in RAM).
    pub max_resident_set: Limit,

    /// Max processes
    ///
    /// This is a limit on the number of extant process (or, more precisely on Linux, threads) for
    /// the real user rID of the calling process.
    pub max_processes: Limit,

    /// Max open files
    ///
    /// This specifies a value one greater than the maximum file descriptor number that can be
    /// opened by this process.
    pub max_open_files: Limit,

    /// Max locked memory
    ///
    /// This is the maximum number of bytes of memory that may be locked into RAM.
    pub max_locked_memory: Limit,

    /// Max address space
    ///
    /// This is the maximum size of the process's virtual memory (address space).
    pub max_address_space: Limit,

    /// Max file locks
    ///
    /// This is a limit on the combined number of flock locks and fcntl leases that this process
    /// may establish.
    pub max_file_locks: Limit,

    /// Max pending signals
    ///
    /// This is a limit on the number of signals that may be queued for the real user rID of the
    /// calling process.
    pub max_pending_signals: Limit,

    /// Max msgqueue size
    ///
    /// This is a limit on the number of bytes that can be allocated for POSIX message queues for
    /// the real user rID of the calling process.
    pub max_msgqueue_size: Limit,

    /// Max nice priority
    ///
    /// This specifies a ceiling to which the process's nice value can be raised using
    /// `setpriority` or `nice`.
    pub max_nice_priority: Limit,

    /// Max realtime priority
    ///
    /// This specifies a ceiling on the real-time priority that may be set for this process using
    /// `sched_setscheduler` and `sched_setparam`.
    pub max_realtime_priority: Limit,

    /// Max realtime timeout
    ///
    /// This is a limit (in microseconds) on the amount of CPU time that a process scheduled under
    /// a real-time scheduling policy may consume without making a blocking system call.
    pub max_realtime_timeout: Limit,
}

impl crate::FromBufRead for Limits {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut lines = r.lines();

        let mut map = HashMap::new();

        while let Some(Ok(line)) = lines.next() {
            let line = line.trim();
            if line.starts_with("Limit") {
                continue;
            }
            let s: Vec<_> = line.split_whitespace().collect();
            let l = s.len();

            let (hard_limit, soft_limit, name) =
                if line.starts_with("Max nice priority") || line.starts_with("Max realtime priority") {
                    // these two limits don't have units, and so need different offsets:
                    let hard_limit = expect!(s.get(l - 1)).to_owned();
                    let soft_limit = expect!(s.get(l - 2)).to_owned();
                    let name = s[0..l - 2].join(" ");
                    (hard_limit, soft_limit, name)
                } else {
                    let hard_limit = expect!(s.get(l - 2)).to_owned();
                    let soft_limit = expect!(s.get(l - 3)).to_owned();
                    let name = s[0..l - 3].join(" ");
                    (hard_limit, soft_limit, name)
                };
            let _units = expect!(s.get(l - 1));

            map.insert(name.to_owned(), (soft_limit.to_owned(), hard_limit.to_owned()));
        }

        let limits = Limits {
            max_cpu_time: Limit::from_pair(expect!(map.remove("Max cpu time")))?,
            max_file_size: Limit::from_pair(expect!(map.remove("Max file size")))?,
            max_data_size: Limit::from_pair(expect!(map.remove("Max data size")))?,
            max_stack_size: Limit::from_pair(expect!(map.remove("Max stack size")))?,
            max_core_file_size: Limit::from_pair(expect!(map.remove("Max core file size")))?,
            max_resident_set: Limit::from_pair(expect!(map.remove("Max resident set")))?,
            max_processes: Limit::from_pair(expect!(map.remove("Max processes")))?,
            max_open_files: Limit::from_pair(expect!(map.remove("Max open files")))?,
            max_locked_memory: Limit::from_pair(expect!(map.remove("Max locked memory")))?,
            max_address_space: Limit::from_pair(expect!(map.remove("Max address space")))?,
            max_file_locks: Limit::from_pair(expect!(map.remove("Max file locks")))?,
            max_pending_signals: Limit::from_pair(expect!(map.remove("Max pending signals")))?,
            max_msgqueue_size: Limit::from_pair(expect!(map.remove("Max msgqueue size")))?,
            max_nice_priority: Limit::from_pair(expect!(map.remove("Max nice priority")))?,
            max_realtime_priority: Limit::from_pair(expect!(map.remove("Max realtime priority")))?,
            max_realtime_timeout: Limit::from_pair(expect!(map.remove("Max realtime timeout")))?,
        };
        if cfg!(test) {
            assert!(map.is_empty(), "Map isn't empty: {:?}", map);
        }
        Ok(limits)
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Limit {
    pub soft_limit: LimitValue,
    pub hard_limit: LimitValue,
}

impl Limit {
    fn from_pair(l: (String, String)) -> ProcResult<Limit> {
        let (soft, hard) = l;
        Ok(Limit {
            soft_limit: LimitValue::from_str(&soft)?,
            hard_limit: LimitValue::from_str(&hard)?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum LimitValue {
    Unlimited,
    Value(u64),
}

impl FromStr for LimitValue {
    type Err = ProcError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "unlimited" {
            Ok(LimitValue::Unlimited)
        } else {
            Ok(LimitValue::Value(from_str!(u64, s)))
        }
    }
}
