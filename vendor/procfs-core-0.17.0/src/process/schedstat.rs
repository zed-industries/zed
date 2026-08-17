use crate::from_iter;
use crate::ProcResult;
use std::io::Read;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Provides scheduler statistics of the process, based on the `/proc/<pid>/schedstat` file.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Schedstat {
    /// Time spent on the cpu.
    ///
    /// Measured in nanoseconds.
    pub sum_exec_runtime: u64,
    /// Time spent waiting on a runqueue.
    ///
    /// Measured in nanoseconds.
    pub run_delay: u64,
    /// \# of timeslices run on this cpu.
    pub pcount: u64,
}

impl crate::FromRead for Schedstat {
    fn from_read<R: Read>(mut r: R) -> ProcResult<Self> {
        let mut line = String::new();
        r.read_to_string(&mut line)?;
        let mut s = line.split_whitespace();

        let schedstat = Schedstat {
            sum_exec_runtime: expect!(from_iter(&mut s)),
            run_delay: expect!(from_iter(&mut s)),
            pcount: expect!(from_iter(&mut s)),
        };

        if cfg!(test) {
            assert!(s.next().is_none());
        }

        Ok(schedstat)
    }
}
