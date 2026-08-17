//! Functions and structs related to process information
//!
//! The primary source of data for functions in this module is the files in a `/proc/<pid>/`
//! directory.

use super::*;
use crate::from_iter;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

mod limit;
pub use limit::*;

mod stat;
pub use stat::*;

mod mount;
pub use mount::*;

mod namespaces;
pub use namespaces::*;

mod status;
pub use status::*;

mod schedstat;
pub use schedstat::*;

mod smaps_rollup;
pub use smaps_rollup::*;

mod pagemap;
pub use pagemap::*;

mod clear_refs;
pub use clear_refs::*;

bitflags! {
    /// Kernel flags for a process
    ///
    /// See also the [Stat::flags()] method.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct StatFlags: u32 {
        /// I am an IDLE thread
        const PF_IDLE = 0x0000_0002;
        /// Getting shut down
        const PF_EXITING = 0x0000_0004;
        /// PI exit done on shut down
        const PF_EXITPIDONE = 0x0000_0008;
        /// I'm a virtual CPU
        const PF_VCPU = 0x0000_0010;
        /// I'm a workqueue worker
        const PF_WQ_WORKER = 0x0000_0020;
        /// Forked but didn't exec
        const PF_FORKNOEXEC = 0x0000_0040;
        /// Process policy on mce errors;
        const PF_MCE_PROCESS = 0x0000_0080;
        /// Used super-user privileges
        const PF_SUPERPRIV = 0x0000_0100;
        /// Dumped core
        const PF_DUMPCORE = 0x0000_0200;
        /// Killed by a signal
        const PF_SIGNALED = 0x0000_0400;
        ///Allocating memory
        const PF_MEMALLOC = 0x0000_0800;
        /// set_user() noticed that RLIMIT_NPROC was exceeded
        const PF_NPROC_EXCEEDED = 0x0000_1000;
        /// If unset the fpu must be initialized before use
        const PF_USED_MATH = 0x0000_2000;
         /// Used async_schedule*(), used by module init
        const PF_USED_ASYNC = 0x0000_4000;
        ///  This thread should not be frozen
        const PF_NOFREEZE = 0x0000_8000;
        /// Frozen for system suspend
        const PF_FROZEN = 0x0001_0000;
        /// I am kswapd
        const PF_KSWAPD = 0x0002_0000;
        /// All allocation requests will inherit GFP_NOFS
        const PF_MEMALLOC_NOFS = 0x0004_0000;
        /// All allocation requests will inherit GFP_NOIO
        const PF_MEMALLOC_NOIO = 0x0008_0000;
        /// Throttle me less: I clean memory
        const PF_LESS_THROTTLE = 0x0010_0000;
        /// I am a kernel thread
        const PF_KTHREAD = 0x0020_0000;
        /// Randomize virtual address space
        const PF_RANDOMIZE = 0x0040_0000;
        /// Allowed to write to swap
        const PF_SWAPWRITE = 0x0080_0000;
        /// Stalled due to lack of memory
        const PF_MEMSTALL = 0x0100_0000;
        /// I'm an Usermodehelper process
        const PF_UMH = 0x0200_0000;
        /// Userland is not allowed to meddle with cpus_allowed
        const PF_NO_SETAFFINITY = 0x0400_0000;
        /// Early kill for mce process policy
        const PF_MCE_EARLY = 0x0800_0000;
        /// All allocation request will have _GFP_MOVABLE cleared
        const PF_MEMALLOC_NOCMA = 0x1000_0000;
        /// Thread belongs to the rt mutex tester
        const PF_MUTEX_TESTER = 0x2000_0000;
        /// Freezer should not count it as freezable
        const PF_FREEZER_SKIP = 0x4000_0000;
        /// This thread called freeze_processes() and should not be frozen
        const PF_SUSPEND_TASK = 0x8000_0000;

    }
}

bitflags! {
    /// See the [coredump_filter()](struct.Process.html#method.coredump_filter) method.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct CoredumpFlags: u32 {
        const ANONYMOUS_PRIVATE_MAPPINGS = 0x01;
        const ANONYMOUS_SHARED_MAPPINGS = 0x02;
        const FILEBACKED_PRIVATE_MAPPINGS = 0x04;
        const FILEBACKED_SHARED_MAPPINGS = 0x08;
        const ELF_HEADERS = 0x10;
        const PROVATE_HUGEPAGES = 0x20;
        const SHARED_HUGEPAGES = 0x40;
        const PRIVATE_DAX_PAGES = 0x80;
        const SHARED_DAX_PAGES = 0x100;
    }
}

bitflags! {
    /// The permissions a process has on memory map entries.
    ///
    /// Note that the `SHARED` and `PRIVATE` are mutually exclusive, so while you can
    /// use `MMPermissions::all()` to construct an instance that has all bits set,
    /// this particular value would never been seen in procfs.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Default)]
    pub struct MMPermissions: u8 {
        /// No permissions
        const NONE = 0;
        /// Read permission
        const READ = 1 << 0;
        /// Write permission
        const WRITE = 1 << 1;
        /// Execute permission
        const EXECUTE = 1 << 2;
        /// Memory is shared with another process.
        ///
        /// Mutually exclusive with PRIVATE.
        const SHARED = 1 << 3;
        /// Memory is private (and copy-on-write)
        ///
        /// Mutually exclusive with SHARED.
        const PRIVATE = 1 << 4;
    }
}

impl MMPermissions {
    fn from_ascii_char(b: u8) -> Self {
        match b {
            b'r' => Self::READ,
            b'w' => Self::WRITE,
            b'x' => Self::EXECUTE,
            b's' => Self::SHARED,
            b'p' => Self::PRIVATE,
            _ => Self::NONE,
        }
    }
    /// Returns this permission map as a 4-character string, similar to what you
    /// might see in `/proc/\<pid\>/maps`.
    ///
    /// Note that the SHARED and PRIVATE bits are mutually exclusive, so this
    /// string is 4 characters long, not 5.
    pub fn as_str(&self) -> String {
        let mut s = String::with_capacity(4);
        s.push(if self.contains(Self::READ) { 'r' } else { '-' });
        s.push(if self.contains(Self::WRITE) { 'w' } else { '-' });
        s.push(if self.contains(Self::EXECUTE) { 'x' } else { '-' });
        s.push(if self.contains(Self::SHARED) {
            's'
        } else if self.contains(Self::PRIVATE) {
            'p'
        } else {
            '-'
        });

        s
    }
}

impl FromStr for MMPermissions {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on ASCII (byte) values
        Ok(s.bytes()
            .map(Self::from_ascii_char)
            .fold(Self::default(), std::ops::BitOr::bitor))
    }
}

bitflags! {
    /// Represents the kernel flags associated with the virtual memory area.
    /// The names of these flags are just those you'll find in the man page, but in upper case.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Default)]
    pub struct VmFlags: u32 {
        /// No flags
        const NONE = 0;
        /// Readable
        const RD = 1 << 0;
        /// Writable
        const WR = 1 << 1;
        /// Executable
        const EX = 1 << 2;
        /// Shared
        const SH = 1 << 3;
        /// May read
        const MR = 1 << 4;
        /// May write
        const MW = 1 << 5;
        /// May execute
        const ME = 1 << 6;
        /// May share
        const MS = 1 << 7;
        /// Stack segment grows down
        const GD = 1 << 8;
        /// Pure PFN range
        const PF = 1 << 9;
        /// Disable write to the mapped file
        const DW = 1 << 10;
        /// Pages are locked in memory
        const LO = 1 << 11;
        /// Memory mapped I/O area
        const IO = 1 << 12;
        /// Sequential read advise provided
        const SR = 1 << 13;
        /// Random read provided
        const RR = 1 << 14;
        /// Do not copy area on fork
        const DC = 1 << 15;
        /// Do not expand area on remapping
        const DE = 1 << 16;
        /// Area is accountable
        const AC = 1 << 17;
        /// Swap space is not reserved for the area
        const NR = 1 << 18;
        /// Area uses huge TLB pages
        const HT = 1 << 19;
        /// Perform synchronous page faults (since Linux 4.15)
        const SF = 1 << 20;
        /// Non-linear mapping (removed in Linux 4.0)
        const NL = 1 << 21;
        /// Architecture specific flag
        const AR = 1 << 22;
        /// Wipe on fork (since Linux 4.14)
        const WF = 1 << 23;
        /// Do not include area into core dump
        const DD = 1 << 24;
        /// Soft-dirty flag (since Linux 3.13)
        const SD = 1 << 25;
        /// Mixed map area
        const MM = 1 << 26;
        /// Huge page advise flag
        const HG = 1 << 27;
        /// No-huge page advise flag
        const NH = 1 << 28;
        /// Mergeable advise flag
        const MG = 1 << 29;
        /// Userfaultfd missing pages tracking (since Linux 4.3)
        const UM = 1 << 30;
        /// Userfaultfd wprotect pages tracking (since Linux 4.3)
        const UW = 1 << 31;
    }
}

impl VmFlags {
    fn from_str(flag: &str) -> Self {
        if flag.len() != 2 {
            return VmFlags::NONE;
        }

        match flag {
            "rd" => VmFlags::RD,
            "wr" => VmFlags::WR,
            "ex" => VmFlags::EX,
            "sh" => VmFlags::SH,
            "mr" => VmFlags::MR,
            "mw" => VmFlags::MW,
            "me" => VmFlags::ME,
            "ms" => VmFlags::MS,
            "gd" => VmFlags::GD,
            "pf" => VmFlags::PF,
            "dw" => VmFlags::DW,
            "lo" => VmFlags::LO,
            "io" => VmFlags::IO,
            "sr" => VmFlags::SR,
            "rr" => VmFlags::RR,
            "dc" => VmFlags::DC,
            "de" => VmFlags::DE,
            "ac" => VmFlags::AC,
            "nr" => VmFlags::NR,
            "ht" => VmFlags::HT,
            "sf" => VmFlags::SF,
            "nl" => VmFlags::NL,
            "ar" => VmFlags::AR,
            "wf" => VmFlags::WF,
            "dd" => VmFlags::DD,
            "sd" => VmFlags::SD,
            "mm" => VmFlags::MM,
            "hg" => VmFlags::HG,
            "nh" => VmFlags::NH,
            "mg" => VmFlags::MG,
            "um" => VmFlags::UM,
            "uw" => VmFlags::UW,
            _ => VmFlags::NONE,
        }
    }
}

/// Represents the state of a process.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProcState {
    /// Running (R)
    Running,
    /// Sleeping in an interruptible wait (S)
    Sleeping,
    /// Waiting in uninterruptible disk sleep (D)
    Waiting,
    /// Zombie (Z)
    Zombie,
    /// Stopped (on a signal) (T)
    ///
    /// Or before Linux 2.6.33, trace stopped
    Stopped,
    /// Tracing stop (t) (Linux 2.6.33 onward)
    Tracing,
    /// Dead (X)
    Dead,
    /// Wakekill (K) (Linux 2.6.33 to 3.13 only)
    Wakekill,
    /// Waking (W) (Linux 2.6.33 to 3.13 only)
    Waking,
    /// Parked (P) (Linux 3.9 to 3.13 only)
    Parked,
    /// Idle (I)
    Idle,
}

impl ProcState {
    pub fn from_char(c: char) -> Option<ProcState> {
        match c {
            'R' => Some(ProcState::Running),
            'S' => Some(ProcState::Sleeping),
            'D' => Some(ProcState::Waiting),
            'Z' => Some(ProcState::Zombie),
            'T' => Some(ProcState::Stopped),
            't' => Some(ProcState::Tracing),
            'X' | 'x' => Some(ProcState::Dead),
            'K' => Some(ProcState::Wakekill),
            'W' => Some(ProcState::Waking),
            'P' => Some(ProcState::Parked),
            'I' => Some(ProcState::Idle),
            _ => None,
        }
    }
}

impl FromStr for ProcState {
    type Err = ProcError;
    fn from_str(s: &str) -> Result<ProcState, ProcError> {
        ProcState::from_char(expect!(s.chars().next(), "empty string"))
            .ok_or_else(|| build_internal_error!("failed to convert"))
    }
}

/// This struct contains I/O statistics for the process, built from `/proc/<pid>/io`
///
/// #  Note
///
/// In the current implementation, things are a bit racy on 32-bit systems: if process A
/// reads process B's `/proc/<pid>/io` while process  B is updating one of these 64-bit
/// counters, process A could see an intermediate result.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Io {
    /// Characters read
    ///
    /// The number of bytes which this task has caused to be read from storage.  This is simply the
    /// sum of bytes which this process passed to read(2)  and  similar system calls.  It includes
    /// things such as terminal I/O and is unaffected by whether or not actual physical disk I/O
    /// was required (the read might have been satisfied from pagecache).
    pub rchar: u64,

    /// characters written
    ///
    /// The number of bytes which this task has caused, or shall cause to be written to disk.
    /// Similar caveats apply here as with rchar.
    pub wchar: u64,
    /// read syscalls
    ///
    /// Attempt to count the number of write I/O operations—that is, system calls such as write(2)
    /// and pwrite(2).
    pub syscr: u64,
    /// write syscalls
    ///
    /// Attempt to count the number of write I/O operations—that is, system calls such as write(2)
    /// and pwrite(2).
    pub syscw: u64,
    /// bytes read
    ///
    /// Attempt to count the number of bytes which this process really did cause to be fetched from
    /// the storage layer.  This is accurate  for block-backed filesystems.
    pub read_bytes: u64,
    /// bytes written
    ///
    /// Attempt to count the number of bytes which this process caused to be sent to the storage layer.
    pub write_bytes: u64,
    /// Cancelled write bytes.
    ///
    /// The  big inaccuracy here is truncate.  If a process writes 1MB to a file and then deletes
    /// the file, it will in fact perform no write‐ out.  But it will have been accounted as having
    /// caused 1MB of write.  In other words: this field represents the number of bytes which this
    /// process caused to not happen, by truncating pagecache.  A task can cause "negative" I/O too.
    /// If this task truncates some dirty pagecache, some I/O which another task has been accounted
    /// for (in its write_bytes) will not be happening.
    pub cancelled_write_bytes: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum MMapPath {
    /// The file that is backing the mapping.
    Path(PathBuf),
    /// The process's heap.
    Heap,
    /// The initial process's (also known as the main thread's) stack.
    Stack,
    /// A thread's stack (where the `<tid>` is a thread ID).  It corresponds to the
    /// `/proc/<pid>/task/<tid>/` path.
    ///
    /// (since Linux 3.4)
    TStack(u32),
    /// The virtual dynamically linked shared object.
    Vdso,
    /// Shared kernel variables
    Vvar,
    /// obsolete virtual syscalls, succeeded by vdso
    Vsyscall,
    /// rollup memory mappings, from `/proc/<pid>/smaps_rollup`
    Rollup,
    /// An anonymous mapping as obtained via mmap(2).
    Anonymous,
    /// Shared memory segment. The i32 value corresponds to [Shm.key](Shm::key), while [MemoryMap.inode](MemoryMap::inode) corresponds to [Shm.shmid](Shm::shmid)
    Vsys(i32),
    /// Some other pseudo-path
    Other(String),
}

impl MMapPath {
    pub fn from(path: &str) -> ProcResult<MMapPath> {
        Ok(match path.trim() {
            "" => MMapPath::Anonymous,
            "[heap]" => MMapPath::Heap,
            "[stack]" => MMapPath::Stack,
            "[vdso]" => MMapPath::Vdso,
            "[vvar]" => MMapPath::Vvar,
            "[vsyscall]" => MMapPath::Vsyscall,
            "[rollup]" => MMapPath::Rollup,
            x if x.starts_with("[stack:") => {
                let mut s = x[1..x.len() - 1].split(':');
                let tid = from_str!(u32, expect!(s.nth(1)));
                MMapPath::TStack(tid)
            }
            x if x.starts_with('[') && x.ends_with(']') => MMapPath::Other(x[1..x.len() - 1].to_string()),
            x if x.starts_with("/SYSV") => MMapPath::Vsys(u32::from_str_radix(&x[5..13], 16)? as i32), // 32bits signed hex. /SYSVaabbccdd (deleted)
            x => MMapPath::Path(PathBuf::from(x)),
        })
    }
}

/// Represents all entries in a `/proc/<pid>/maps` or `/proc/<pid>/smaps` file.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct MemoryMaps(pub Vec<MemoryMap>);

impl crate::FromBufRead for MemoryMaps {
    /// The data should be formatted according to procfs /proc/pid/{maps,smaps,smaps_rollup}.
    fn from_buf_read<R: BufRead>(reader: R) -> ProcResult<Self> {
        let mut memory_maps = Vec::new();

        let mut line_iter = reader.lines().map(|r| r.map_err(|_| ProcError::Incomplete(None)));
        let mut current_memory_map: Option<MemoryMap> = None;
        while let Some(line) = line_iter.next().transpose()? {
            // Assumes all extension fields (in `/proc/<pid>/smaps`) start with a capital letter,
            // which seems to be the case.
            if line.starts_with(|c: char| c.is_ascii_uppercase()) {
                match current_memory_map.as_mut() {
                    None => return Err(ProcError::Incomplete(None)),
                    Some(mm) => {
                        // This is probably an attribute
                        if line.starts_with("VmFlags") {
                            let flags = line.split_ascii_whitespace();
                            let flags = flags.skip(1); // Skips the `VmFlags:` part since we don't need it.

                            let flags = flags
                                .map(VmFlags::from_str)
                                // FUTURE: use `Iterator::reduce`
                                .fold(VmFlags::NONE, std::ops::BitOr::bitor);

                            mm.extension.vm_flags = flags;
                        } else {
                            let mut parts = line.split_ascii_whitespace();

                            let key = parts.next();
                            let value = parts.next();

                            if let (Some(k), Some(v)) = (key, value) {
                                // While most entries do have one, not all of them do.
                                let size_suffix = parts.next();

                                // Limited poking at /proc/<pid>/smaps and then checking if "MB", "GB", and "TB" appear in the C file that is
                                // supposedly responsible for creating smaps, has lead me to believe that the only size suffixes we'll ever encounter
                                // "kB", which is most likely kibibytes. Actually checking if the size suffix is any of the above is a way to
                                // future-proof the code, but I am not sure it is worth doing so.
                                let size_multiplier = if size_suffix.is_some() { 1024 } else { 1 };

                                let v = v.parse::<u64>().map_err(|_| {
                                    ProcError::Other("Value in `Key: Value` pair was not actually a number".into())
                                })?;

                                // This ignores the case when our Key: Value pairs are really Key Value pairs. Is this a good idea?
                                let k = k.trim_end_matches(':');

                                mm.extension.map.insert(k.into(), v * size_multiplier);
                            }
                        }
                    }
                }
            } else {
                if let Some(mm) = current_memory_map.take() {
                    memory_maps.push(mm);
                }
                current_memory_map = Some(MemoryMap::from_line(&line)?);
            }
        }
        if let Some(mm) = current_memory_map.take() {
            memory_maps.push(mm);
        }

        Ok(MemoryMaps(memory_maps))
    }
}

impl MemoryMaps {
    /// Return an iterator over [MemoryMap].
    pub fn iter(&self) -> std::slice::Iter<MemoryMap> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a MemoryMaps {
    type IntoIter = std::slice::Iter<'a, MemoryMap>;
    type Item = &'a MemoryMap;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for MemoryMaps {
    type IntoIter = std::vec::IntoIter<MemoryMap>;
    type Item = MemoryMap;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Represents an entry in a `/proc/<pid>/maps` or `/proc/<pid>/smaps` file.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct MemoryMap {
    /// The address space in the process that the mapping occupies.
    pub address: (u64, u64),
    pub perms: MMPermissions,
    /// The offset into the file/whatever
    pub offset: u64,
    /// The device (major, minor)
    pub dev: (i32, i32),
    /// The inode on that device
    ///
    /// 0 indicates that no inode is associated with the memory region, as would be the case with
    /// BSS (uninitialized data).
    pub inode: u64,
    pub pathname: MMapPath,
    /// Memory mapping extension information, populated when parsing `/proc/<pid>/smaps`.
    ///
    /// The members will be `Default::default()` (empty/none) when the information isn't available.
    pub extension: MMapExtension,
}

impl MemoryMap {
    fn from_line(line: &str) -> ProcResult<MemoryMap> {
        let mut s = line.splitn(6, ' ');
        let address = expect!(s.next());
        let perms = expect!(s.next());
        let offset = expect!(s.next());
        let dev = expect!(s.next());
        let inode = expect!(s.next());
        let path = expect!(s.next());

        Ok(MemoryMap {
            address: split_into_num(address, '-', 16)?,
            perms: perms.parse()?,
            offset: from_str!(u64, offset, 16),
            dev: split_into_num(dev, ':', 16)?,
            inode: from_str!(u64, inode),
            pathname: MMapPath::from(path)?,
            extension: Default::default(),
        })
    }
}

/// Represents the information about a specific mapping as presented in /proc/\<pid\>/smaps
#[derive(Default, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct MMapExtension {
    /// Key-value pairs that may represent statistics about memory usage, or other interesting things,
    /// such a "ProtectionKey" (if you're on X86 and that kernel config option was specified).
    ///
    /// Note that should a key-value pair represent a memory usage statistic, it will be in bytes.
    ///
    /// Check your manpage for more information
    pub map: HashMap<String, u64>,
    /// Kernel flags associated with the virtual memory area
    ///
    /// (since Linux 3.8)
    pub vm_flags: VmFlags,
}

impl MMapExtension {
    /// Return whether the extension information is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty() && self.vm_flags == VmFlags::NONE
    }
}

impl crate::FromBufRead for Io {
    fn from_buf_read<R: BufRead>(reader: R) -> ProcResult<Self> {
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() || !line.contains(' ') {
                continue;
            }
            let mut s = line.split_whitespace();
            let field = expect!(s.next());
            let value = expect!(s.next());

            let value = from_str!(u64, value);

            map.insert(field[..field.len() - 1].to_string(), value);
        }
        let io = Io {
            rchar: expect!(map.remove("rchar")),
            wchar: expect!(map.remove("wchar")),
            syscr: expect!(map.remove("syscr")),
            syscw: expect!(map.remove("syscw")),
            read_bytes: expect!(map.remove("read_bytes")),
            write_bytes: expect!(map.remove("write_bytes")),
            cancelled_write_bytes: expect!(map.remove("cancelled_write_bytes")),
        };

        assert!(!cfg!(test) || map.is_empty(), "io map is not empty: {:#?}", map);

        Ok(io)
    }
}

/// Describes a file descriptor opened by a process.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum FDTarget {
    /// A file or device
    Path(PathBuf),
    /// A socket type, with an inode
    Socket(u64),
    Net(u64),
    Pipe(u64),
    /// A file descriptor that have no corresponding inode.
    AnonInode(String),
    /// A memfd file descriptor with a name.
    MemFD(String),
    /// Some other file descriptor type, with an inode.
    Other(String, u64),
}

impl FromStr for FDTarget {
    type Err = ProcError;
    fn from_str(s: &str) -> Result<FDTarget, ProcError> {
        // helper function that removes the first and last character
        fn strip_first_last(s: &str) -> ProcResult<&str> {
            if s.len() > 2 {
                let mut c = s.chars();
                // remove the first and last characters
                let _ = c.next();
                let _ = c.next_back();
                Ok(c.as_str())
            } else {
                Err(ProcError::Incomplete(None))
            }
        }

        if !s.starts_with('/') && s.contains(':') {
            let mut s = s.split(':');
            let fd_type = expect!(s.next());
            match fd_type {
                "socket" => {
                    let inode = expect!(s.next(), "socket inode");
                    let inode = expect!(u64::from_str_radix(strip_first_last(inode)?, 10));
                    Ok(FDTarget::Socket(inode))
                }
                "net" => {
                    let inode = expect!(s.next(), "net inode");
                    let inode = expect!(u64::from_str_radix(strip_first_last(inode)?, 10));
                    Ok(FDTarget::Net(inode))
                }
                "pipe" => {
                    let inode = expect!(s.next(), "pipe inode");
                    let inode = expect!(u64::from_str_radix(strip_first_last(inode)?, 10));
                    Ok(FDTarget::Pipe(inode))
                }
                "anon_inode" => Ok(FDTarget::AnonInode(expect!(s.next(), "anon inode").to_string())),
                "" => Err(ProcError::Incomplete(None)),
                x => {
                    let inode = expect!(s.next(), "other inode");
                    let inode = expect!(u64::from_str_radix(strip_first_last(inode)?, 10));
                    Ok(FDTarget::Other(x.to_string(), inode))
                }
            }
        } else if let Some(s) = s.strip_prefix("/memfd:") {
            Ok(FDTarget::MemFD(s.to_string()))
        } else {
            Ok(FDTarget::Path(PathBuf::from(s)))
        }
    }
}

/// Provides information about memory usage, measured in pages.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct StatM {
    /// Total program size, measured in pages
    ///
    /// (same as VmSize in /proc/\<pid\>/status)
    pub size: u64,
    /// Resident set size, measured in pages
    ///
    /// (same as VmRSS in /proc/\<pid\>/status)
    pub resident: u64,
    /// number of resident shared pages (i.e., backed by a file)
    ///
    /// (same as RssFile+RssShmem in /proc/\<pid\>/status)
    pub shared: u64,
    /// Text (code)
    pub text: u64,
    /// library (unused since Linux 2.6; always 0)
    pub lib: u64,
    /// data + stack
    pub data: u64,
    /// dirty pages (unused since Linux 2.6; always 0)
    pub dt: u64,
}

impl crate::FromRead for StatM {
    fn from_read<R: Read>(mut r: R) -> ProcResult<Self> {
        let mut line = String::new();
        r.read_to_string(&mut line)?;
        let mut s = line.split_whitespace();

        let size = expect!(from_iter(&mut s));
        let resident = expect!(from_iter(&mut s));
        let shared = expect!(from_iter(&mut s));
        let text = expect!(from_iter(&mut s));
        let lib = expect!(from_iter(&mut s));
        let data = expect!(from_iter(&mut s));
        let dt = expect!(from_iter(&mut s));

        if cfg!(test) {
            assert!(s.next().is_none());
        }

        Ok(StatM {
            size,
            resident,
            shared,
            text,
            lib,
            data,
            dt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_memory_map_permissions() {
        use MMPermissions as P;
        assert_eq!("rw-p".parse(), Ok(P::READ | P::WRITE | P::PRIVATE));
        assert_eq!("r-xs".parse(), Ok(P::READ | P::EXECUTE | P::SHARED));
        assert_eq!("----".parse(), Ok(P::NONE));

        assert_eq!((P::READ | P::WRITE | P::PRIVATE).as_str(), "rw-p");
        assert_eq!((P::READ | P::EXECUTE | P::SHARED).as_str(), "r-xs");
        assert_eq!(P::NONE.as_str(), "----");
    }
}
