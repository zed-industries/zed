use core::{
    mem,
    ops::{Deref, DerefMut},
    slice,
};

use crate::flag::{EventFlags, MapFlags, PtraceFlags};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Event {
    pub id: usize,
    pub flags: EventFlags,
    pub data: usize,
}

impl Deref for Event {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Event as *const u8, mem::size_of::<Event>()) }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Event as *mut u8, mem::size_of::<Event>()) }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ITimerSpec {
    pub it_interval: TimeSpec,
    pub it_value: TimeSpec,
}

impl Deref for ITimerSpec {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const ITimerSpec as *const u8,
                mem::size_of::<ITimerSpec>(),
            )
        }
    }
}

impl DerefMut for ITimerSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut ITimerSpec as *mut u8,
                mem::size_of::<ITimerSpec>(),
            )
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct OldMap {
    pub offset: usize,
    pub size: usize,
    pub flags: MapFlags,
}

impl Deref for OldMap {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const OldMap as *const u8, mem::size_of::<OldMap>())
        }
    }
}

impl DerefMut for OldMap {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut OldMap as *mut u8, mem::size_of::<OldMap>())
        }
    }
}
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Map {
    /// The offset inside the file that is being mapped.
    pub offset: usize,

    /// The size of the memory map.
    pub size: usize,

    /// Contains both prot and map flags.
    pub flags: MapFlags,

    /// Functions as a hint to where in the virtual address space of the running process, to place
    /// the memory map. If [`MapFlags::MAP_FIXED`] is set, then this address must be the address to
    /// map to.
    pub address: usize,
}

impl Deref for Map {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Map as *const u8, mem::size_of::<Map>()) }
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Map as *mut u8, mem::size_of::<Map>()) }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Packet {
    pub id: u64,
    pub pid: usize,
    pub uid: u32,
    pub gid: u32,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
}

impl Deref for Packet {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Packet as *const u8, mem::size_of::<Packet>())
        }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Packet as *mut u8, mem::size_of::<Packet>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u16,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_size: u64,
    pub st_blksize: u32,
    pub st_blocks: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u32,
    pub st_atime: u64,
    pub st_atime_nsec: u32,
    pub st_ctime: u64,
    pub st_ctime_nsec: u32,
}

impl Deref for Stat {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Stat as *const u8, mem::size_of::<Stat>()) }
    }
}

impl DerefMut for Stat {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Stat as *mut u8, mem::size_of::<Stat>()) }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct StatVfs {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
}

impl Deref for StatVfs {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const StatVfs as *const u8,
                mem::size_of::<StatVfs>(),
            )
        }
    }
}

impl DerefMut for StatVfs {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut StatVfs as *mut u8, mem::size_of::<StatVfs>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

impl Deref for TimeSpec {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const TimeSpec as *const u8,
                mem::size_of::<TimeSpec>(),
            )
        }
    }
}

impl DerefMut for TimeSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut TimeSpec as *mut u8, mem::size_of::<TimeSpec>())
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PtraceEvent {
    pub cause: PtraceFlags,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub e: usize,
    pub f: usize,
}

impl Deref for PtraceEvent {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const PtraceEvent as *const u8,
                mem::size_of::<PtraceEvent>(),
            )
        }
    }
}

impl DerefMut for PtraceEvent {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut PtraceEvent as *mut u8,
                mem::size_of::<PtraceEvent>(),
            )
        }
    }
}

#[macro_export]
macro_rules! ptrace_event {
    ($cause:expr $(, $a:expr $(, $b:expr $(, $c:expr)?)?)?) => {
        $crate::data::PtraceEvent {
            cause: $cause,
            $(a: $a,
              $(b: $b,
                $(c: $c,)?
              )?
            )?
            ..Default::default()
        }
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct GrantFlags: usize {
        const GRANT_READ = 0x0000_0001;
        const GRANT_WRITE = 0x0000_0002;
        const GRANT_EXEC = 0x0000_0004;

        const GRANT_SHARED = 0x0000_0008;
        const GRANT_LAZY = 0x0000_0010;
        const GRANT_SCHEME = 0x0000_0020;
        const GRANT_PHYS = 0x0000_0040;
        const GRANT_PINNED = 0x0000_0080;
        const GRANT_PHYS_CONTIGUOUS = 0x0000_0100;
    }
}

impl GrantFlags {
    #[deprecated = "use the safe `from_bits_retain` method instead"]
    pub unsafe fn from_bits_unchecked(bits: usize) -> Self {
        Self::from_bits_retain(bits)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GrantDesc {
    pub base: usize,
    pub size: usize,
    pub flags: GrantFlags,
    pub offset: u64,
}

impl Deref for GrantDesc {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const GrantDesc as *const u8,
                mem::size_of::<GrantDesc>(),
            )
        }
    }
}

impl DerefMut for GrantDesc {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut GrantDesc as *mut u8,
                mem::size_of::<GrantDesc>(),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SetSighandlerData {
    pub user_handler: usize,
    pub excp_handler: usize,
    pub thread_control_addr: usize,
    pub proc_control_addr: usize,
}

impl Deref for SetSighandlerData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}

impl DerefMut for SetSighandlerData {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>()) }
    }
}
pub use crate::sigabi::*;

/// UNSTABLE
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ProcSchemeAttrs {
    pub pid: u32,
    pub euid: u32,
    pub egid: u32,
    pub ens: u32,
    pub debug_name: [u8; 32],
}
impl Deref for ProcSchemeAttrs {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}
impl DerefMut for ProcSchemeAttrs {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut ProcSchemeAttrs as *mut u8,
                mem::size_of::<ProcSchemeAttrs>(),
            )
        }
    }
}
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct CtxtStsBuf {
    pub status: usize,
    pub excp: crate::Exception,
}
impl Deref for CtxtStsBuf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}
impl DerefMut for CtxtStsBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut CtxtStsBuf as *mut u8,
                mem::size_of::<CtxtStsBuf>(),
            )
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalSchemes {
    Debug = 1,
    Event = 2,
    Memory = 3,
    Pipe = 4,
    Serio = 5,
    Irq = 6,
    Time = 7,
    Sys = 8,
    Proc = 9,
    Acpi = 10,
    Dtb = 11,
}
impl GlobalSchemes {
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::Debug),
            2 => Some(Self::Event),
            3 => Some(Self::Memory),
            4 => Some(Self::Pipe),
            5 => Some(Self::Serio),
            6 => Some(Self::Irq),
            7 => Some(Self::Time),
            8 => Some(Self::Sys),
            9 => Some(Self::Proc),
            10 => Some(Self::Acpi),
            11 => Some(Self::Dtb),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Event => "event",
            Self::Memory => "memory",
            Self::Pipe => "pipe",
            Self::Serio => "serio",
            Self::Irq => "irq",
            Self::Time => "time",
            Self::Sys => "sys",
            Self::Proc => "kernel.proc",
            Self::Acpi => "kernel.acpi",
            Self::Dtb => "kernel.dtb",
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct KernelSchemeInfo {
    pub scheme_id: u8,
    pub fd: usize,
}
