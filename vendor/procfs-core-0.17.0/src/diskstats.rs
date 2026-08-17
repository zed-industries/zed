use crate::{expect, from_str, ProcResult};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::io::BufRead;

/// Disk IO stat information
///
/// To fully understand these fields, please see the [iostats.txt](https://www.kernel.org/doc/Documentation/iostats.txt)
/// kernel documentation.
///
/// For an example, see the [diskstats.rs](https://github.com/eminence/procfs/tree/master/examples)
/// example in the source repo.
// Doc reference: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
// Doc reference: https://www.kernel.org/doc/Documentation/iostats.txt
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct DiskStat {
    /// The device major number
    pub major: i32,

    /// The device minor number
    pub minor: i32,

    /// Device name
    pub name: String,

    /// Reads completed successfully
    ///
    /// This is the total number of reads completed successfully
    pub reads: u64,

    /// Reads merged
    ///
    /// The number of adjacent reads that have been merged for efficiency.
    pub merged: u64,

    /// Sectors read successfully
    ///
    /// This is the total number of sectors read successfully.
    pub sectors_read: u64,

    /// Time spent reading (ms)
    pub time_reading: u64,

    /// writes completed
    pub writes: u64,

    /// writes merged
    ///
    /// The number of adjacent writes that have been merged for efficiency.
    pub writes_merged: u64,

    /// Sectors written successfully
    pub sectors_written: u64,

    /// Time spent writing (ms)
    pub time_writing: u64,

    /// I/Os currently in progress
    pub in_progress: u64,

    /// Time spent doing I/Os (ms)
    pub time_in_progress: u64,

    /// Weighted time spent doing I/Os (ms)
    pub weighted_time_in_progress: u64,

    /// Discards completed successfully
    ///
    /// (since kernel 4.18)
    pub discards: Option<u64>,

    /// Discards merged
    pub discards_merged: Option<u64>,

    /// Sectors discarded
    pub sectors_discarded: Option<u64>,

    /// Time spent discarding
    pub time_discarding: Option<u64>,

    /// Flush requests completed successfully
    ///
    /// (since kernel 5.5)
    pub flushes: Option<u64>,

    /// Time spent flushing
    pub time_flushing: Option<u64>,
}

/// A list of disk stats.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct DiskStats(pub Vec<DiskStat>);

impl crate::FromBufRead for DiskStats {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut v = Vec::new();

        for line in r.lines() {
            let line = line?;
            v.push(DiskStat::from_line(&line)?);
        }
        Ok(DiskStats(v))
    }
}

impl DiskStat {
    pub fn from_line(line: &str) -> ProcResult<DiskStat> {
        let mut s = line.split_whitespace();

        let major = from_str!(i32, expect!(s.next()));
        let minor = from_str!(i32, expect!(s.next()));
        let name = expect!(s.next()).to_string();
        let reads = from_str!(u64, expect!(s.next()));
        let merged = from_str!(u64, expect!(s.next()));
        let sectors_read = from_str!(u64, expect!(s.next()));
        let time_reading = from_str!(u64, expect!(s.next()));
        let writes = from_str!(u64, expect!(s.next()));
        let writes_merged = from_str!(u64, expect!(s.next()));
        let sectors_written = from_str!(u64, expect!(s.next()));
        let time_writing = from_str!(u64, expect!(s.next()));
        let in_progress = from_str!(u64, expect!(s.next()));
        let time_in_progress = from_str!(u64, expect!(s.next()));
        let weighted_time_in_progress = from_str!(u64, expect!(s.next()));
        let discards = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());
        let discards_merged = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());
        let sectors_discarded = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());
        let time_discarding = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());
        let flushes = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());
        let time_flushing = s.next().and_then(|s| u64::from_str_radix(s, 10).ok());

        Ok(DiskStat {
            major,
            minor,
            name,
            reads,
            merged,
            sectors_read,
            time_reading,
            writes,
            writes_merged,
            sectors_written,
            time_writing,
            in_progress,
            time_in_progress,
            weighted_time_in_progress,
            discards,
            discards_merged,
            sectors_discarded,
            time_discarding,
            flushes,
            time_flushing,
        })
    }
}
