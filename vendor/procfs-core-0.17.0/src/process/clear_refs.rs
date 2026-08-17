use std::fmt;

/// Clearing the PG_Referenced and ACCESSED/YOUNG bits
/// provides a method to measure approximately how much memory
/// a process is using.  One first inspects the values in the
/// "Referenced" fields for the VMAs shown in
/// `/proc/[pid]/smaps` to get an idea of the memory footprint
/// of the process.  One then clears the PG_Referenced and
/// ACCESSED/YOUNG bits and, after some measured time
/// interval, once again inspects the values in the
/// "Referenced" fields to get an idea of the change in memory
/// footprint of the process during the measured interval.  If
/// one is interested only in inspecting the selected mapping
/// types, then the value 2 or 3 can be used instead of 1.
///
/// The `/proc/[pid]/clear_refs` file is present only if the
/// CONFIG_PROC_PAGE_MONITOR kernel configuration option is
/// enabled.
///
/// Only writable by the owner of the process
///
/// See `procfs::Process::clear_refs()` and `procfs::Process::pagemap()`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClearRefs {
    /// (since Linux 2.6.22)
    ///
    ///  Reset the PG_Referenced and ACCESSED/YOUNG bits for
    ///  all the pages associated with the process.  (Before
    ///  kernel 2.6.32, writing any nonzero value to this
    ///  file had this effect.)
    PGReferencedAll = 1,
    /// (since Linux 2.6.32)
    ///
    /// Reset the PG_Referenced and ACCESSED/YOUNG bits for
    /// all anonymous pages associated with the process.
    PGReferencedAnonymous = 2,
    /// (since Linux 2.6.32)
    ///
    /// Reset the PG_Referenced and ACCESSED/YOUNG bits for
    /// all file-mapped pages associated with the process.
    PGReferencedFile = 3,
    /// (since Linux 3.11)
    ///
    /// Clear the soft-dirty bit for all the pages
    /// associated with the process.  This is used (in
    /// conjunction with `/proc/[pid]/pagemap`) by the check-
    /// point restore system to discover which pages of a
    /// process have been dirtied since the file
    /// `/proc/[pid]/clear_refs` was written to.
    SoftDirty = 4,
    /// (since Linux 4.0)
    ///
    /// Reset the peak resident set size ("high water
    /// mark") to the process's current resident set size
    /// value.
    PeakRSS = 5,
}

impl fmt::Display for ClearRefs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClearRefs::PGReferencedAll => 1,
                ClearRefs::PGReferencedAnonymous => 2,
                ClearRefs::PGReferencedFile => 3,
                ClearRefs::SoftDirty => 4,
                ClearRefs::PeakRSS => 5,
            }
        )
    }
}

impl std::str::FromStr for ClearRefs {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map_err(|_| "Fail to parse clear refs value")
            .and_then(|n| match n {
                1 => Ok(ClearRefs::PGReferencedAll),
                2 => Ok(ClearRefs::PGReferencedAnonymous),
                3 => Ok(ClearRefs::PGReferencedFile),
                4 => Ok(ClearRefs::SoftDirty),
                5 => Ok(ClearRefs::PeakRSS),
                _ => Err("Unknown clear refs value"),
            })
    }
}
