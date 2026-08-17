use bitflags::bitflags as inner_bitflags;
use core::{mem, ops::Deref, slice};

macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        pub struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        // First, use the inner bitflags
        inner_bitflags! {
            #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
            $(#[$outer])*
            pub struct $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )+
            }
        }

        impl $BitFlags {
            #[deprecated = "use the safe `from_bits_retain` method instead"]
            pub unsafe fn from_bits_unchecked(bits: $T) -> Self {
                Self::from_bits_retain(bits)
            }
        }

        // Secondly, re-export all inner constants
        // (`pub use self::Struct::*` doesn't work)
        $(
            $(#[$inner $($args)*])*
            pub const $Flag: $BitFlags = $BitFlags::$Flag;
        )+
    }
}

pub const CLOCK_REALTIME: usize = 1;
pub const CLOCK_MONOTONIC: usize = 4;

bitflags! {
    pub struct EventFlags: usize {
        const EVENT_NONE = 0;
        const EVENT_READ = 1;
        const EVENT_WRITE = 2;
    }
}

pub const F_DUPFD: usize = 0;
pub const F_GETFD: usize = 1;
pub const F_SETFD: usize = 2;
pub const F_GETFL: usize = 3;
pub const F_SETFL: usize = 4;

pub const FUTEX_WAIT: usize = 0;
pub const FUTEX_WAKE: usize = 1;
pub const FUTEX_REQUEUE: usize = 2;
pub const FUTEX_WAIT64: usize = 3;

// packet.c = fd
pub const SKMSG_FRETURNFD: usize = 0;

// packet.uid:packet.gid = offset, packet.c = base address, packet.d = page count
pub const SKMSG_PROVIDE_MMAP: usize = 1;

// packet.id provides state, packet.c = dest fd or pointer to dest fd, packet.d = flags
pub const SKMSG_FOBTAINFD: usize = 2;

// TODO: Split SendFdFlags into caller flags and flags that the scheme receives?
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct SendFdFlags: usize {
        /// If set, the kernel will enforce that the file descriptors are exclusively owned.
        ///
        /// That is, there will no longer exist any other reference to those FDs when removed from
        /// the file table (sendfd always removes the FDs from the file table, but without this
        /// flag, it can be retained by SYS_DUPing them first).
        const EXCLUSIVE = 1;

        /// If set, the file descriptors will be cloned and *not* removed from the sender's file table.
        /// By default, `SYS_SENDFD` moves the file descriptors, removing them from the sender.
        const CLONE = 2;
    }
}
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct FobtainFdFlags: usize {
        /// If set, the SYS_CALL payload specifies the destination file descriptor slots, otherwise the lowest
        /// available slots will be selected, and placed in the usize pointed to by SYS_CALL
        /// payload.
        const MANUAL_FD = 1;

        /// If set, the file descriptors received are guaranteed to be exclusively owned (by the file
        /// table the obtainer is running in).
        const EXCLUSIVE = 2;

        /// If set, the file descriptors received will be placed into the *upper* file table.
        const UPPER_TBL = 4;

        // No, cloexec won't be stored in the kernel in the future, when the stable ABI is moved to
        // relibc, so no flag for that!
    }
}
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct RecvFdFlags: usize {
        /// If set, the SYS_CALL payload specifies the destination file descriptor slots, otherwise the lowest
        /// available slots will be selected, and placed in the usize pointed to by SYS_CALL
        /// payload.
        const MANUAL_FD = 1;

        /// If set, the file descriptors received will be placed into the *upper* file table.
        const UPPER_TBL = 2;
    }
}
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct FmoveFdFlags: usize {
        /// If set, the kernel will enforce that the file descriptors are exclusively owned.
        ///
        /// That is, there will no longer exist any other reference to those FDs when removed from
        /// the file table (SYS_CALL always removes the FDs from the file table, but without this
        /// flag, it can be retained by SYS_DUPing them first).
        const EXCLUSIVE = 1;

        /// If set, the file descriptors will be cloned and *not* removed from the sender's file table.
        /// By default, sendfd moves the file descriptors, removing them from the sender.
        const CLONE = 2;
    }
}

bitflags! {
    pub struct MapFlags: usize {
        // TODO: Downgrade PROT_NONE to global constant? (bitflags specifically states zero flags
        // can cause buggy behavior).
        const PROT_NONE = 0x0000_0000;

        const PROT_EXEC = 0x0001_0000;
        const PROT_WRITE = 0x0002_0000;
        const PROT_READ = 0x0004_0000;

        const MAP_SHARED = 0x0001;
        const MAP_PRIVATE = 0x0002;

        const MAP_FIXED = 0x0004;
        const MAP_FIXED_NOREPLACE = 0x000C;

        /// For *userspace-backed mmaps*, return from the mmap call before all pages have been
        /// provided by the scheme. This requires the scheme to be trusted, as the current context
        /// can block indefinitely, if the scheme does not respond to the page fault handler's
        /// request, as it tries to map the page by requesting it from the scheme.
        ///
        /// In some cases however, such as the program loader, the data needs to be trusted as much
        /// with or without MAP_LAZY, and if so, mapping lazily will not cause insecureness by
        /// itself.
        ///
        /// For kernel-backed mmaps, this flag has no effect at all. It is unspecified whether
        /// kernel mmaps are lazy or not.
        const MAP_LAZY = 0x0010;
    }
}
bitflags! {
    pub struct MunmapFlags: usize {
        /// Indicates whether the funmap call must implicitly do an msync, for the changes to
        /// become visible later.
        ///
        /// This flag will currently be set if and only if MAP_SHARED | PROT_WRITE are set.
        const NEEDS_SYNC = 1;
    }
}

pub const MODE_TYPE: u16 = 0xF000;
pub const MODE_DIR: u16 = 0x4000;
pub const MODE_FILE: u16 = 0x8000;
pub const MODE_SYMLINK: u16 = 0xA000;
pub const MODE_FIFO: u16 = 0x1000;
pub const MODE_CHR: u16 = 0x2000;
pub const MODE_SOCK: u16 = 0xC000;

pub const MODE_PERM: u16 = 0x0FFF;
pub const MODE_SETUID: u16 = 0o4000;
pub const MODE_SETGID: u16 = 0o2000;

pub const O_RDONLY: usize = 0x0001_0000;
pub const O_WRONLY: usize = 0x0002_0000;
pub const O_RDWR: usize = 0x0003_0000;
pub const O_NONBLOCK: usize = 0x0004_0000;
pub const O_APPEND: usize = 0x0008_0000;
pub const O_SHLOCK: usize = 0x0010_0000;
pub const O_EXLOCK: usize = 0x0020_0000;
pub const O_ASYNC: usize = 0x0040_0000;
pub const O_FSYNC: usize = 0x0080_0000;
pub const O_CLOEXEC: usize = 0x0100_0000;
pub const O_CREAT: usize = 0x0200_0000;
pub const O_TRUNC: usize = 0x0400_0000;
pub const O_EXCL: usize = 0x0800_0000;
pub const O_DIRECTORY: usize = 0x1000_0000;
pub const O_STAT: usize = 0x2000_0000;
pub const O_SYMLINK: usize = 0x4000_0000;
pub const O_NOFOLLOW: usize = 0x8000_0000;
pub const O_ACCMODE: usize = O_RDONLY | O_WRONLY | O_RDWR;

// The top 48 bits of PTRACE_* are reserved, for now

// NOT ABI STABLE!
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(usize)]
pub enum ContextStatus {
    Runnable,
    Blocked,
    NotYetStarted,
    Dead,
    ForceKilled,
    Stopped,
    UnhandledExcp,
    #[default]
    Other, // reserved
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum ContextVerb {
    Stop = 1,
    Unstop = 2,
    Interrupt = 3,
    ForceKill = usize::MAX,
}
impl ContextVerb {
    pub fn try_from_raw(raw: usize) -> Option<Self> {
        Some(match raw {
            1 => Self::Stop,
            2 => Self::Unstop,
            3 => Self::Interrupt,
            usize::MAX => Self::ForceKill,
            _ => return None,
        })
    }
}

// NOT ABI STABLE!
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ProcSchemeVerb {
    Iopl = 255,
}
impl ProcSchemeVerb {
    pub fn try_from_raw(verb: u8) -> Option<Self> {
        Some(match verb {
            255 => Self::Iopl,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum SchemeSocketCall {
    ObtainFd = 0,
    MoveFd = 1,
}
impl SchemeSocketCall {
    pub fn try_from_raw(raw: usize) -> Option<Self> {
        Some(match raw {
            0 => Self::ObtainFd,
            1 => Self::MoveFd,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
#[non_exhaustive]
pub enum FsCall {
    Connect = 0,
}
impl FsCall {
    pub fn try_from_raw(raw: usize) -> Option<Self> {
        Some(match raw {
            0 => Self::Connect,
            _ => return None,
        })
    }
}

bitflags! {
    pub struct PtraceFlags: u64 {
        /// Stop before a syscall is handled. Send PTRACE_FLAG_IGNORE to not
        /// handle the syscall.
        const PTRACE_STOP_PRE_SYSCALL = 0x0000_0000_0000_0001;
        /// Stop after a syscall is handled.
        const PTRACE_STOP_POST_SYSCALL = 0x0000_0000_0000_0002;
        /// Stop after exactly one instruction. TODO: This may not handle
        /// fexec/signal boundaries. Should it?
        const PTRACE_STOP_SINGLESTEP = 0x0000_0000_0000_0004;
        /// Stop before a signal is handled. Send PTRACE_FLAG_IGNORE to not
        /// handle signal.
        const PTRACE_STOP_SIGNAL = 0x0000_0000_0000_0008;
        /// Stop on a software breakpoint, such as the int3 instruction for
        /// x86_64.
        const PTRACE_STOP_BREAKPOINT = 0x0000_0000_0000_0010;
        /// Stop just before exiting for good.
        const PTRACE_STOP_EXIT = 0x0000_0000_0000_0020;

        const PTRACE_STOP_MASK = 0x0000_0000_0000_00FF;


        /// Sent when a child is cloned, giving you the opportunity to trace it.
        /// If you don't catch this, the child is started as normal.
        const PTRACE_EVENT_CLONE = 0x0000_0000_0000_0100;

        /// Sent when current-addrspace is changed, allowing the tracer to reopen the memory file.
        const PTRACE_EVENT_ADDRSPACE_SWITCH = 0x0000_0000_0000_0200;

        const PTRACE_EVENT_MASK = 0x0000_0000_0000_0F00;

        /// Special meaning, depending on the event. Usually, when fired before
        /// an action, it will skip performing that action.
        const PTRACE_FLAG_IGNORE = 0x0000_0000_0000_1000;

        const PTRACE_FLAG_MASK = 0x0000_0000_0000_F000;
    }
}
impl Deref for PtraceFlags {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // Same as to_ne_bytes but in-place
        unsafe {
            slice::from_raw_parts(&self.bits() as *const _ as *const u8, mem::size_of::<u64>())
        }
    }
}

pub const SEEK_SET: usize = 0;
pub const SEEK_CUR: usize = 1;
pub const SEEK_END: usize = 2;

pub const SIGCHLD: usize = 17;
pub const SIGTSTP: usize = 20;
pub const SIGTTIN: usize = 21;
pub const SIGTTOU: usize = 22;

pub const ADDRSPACE_OP_MMAP: usize = 0;
pub const ADDRSPACE_OP_MUNMAP: usize = 1;
pub const ADDRSPACE_OP_MPROTECT: usize = 2;
pub const ADDRSPACE_OP_TRANSFER: usize = 3;

bitflags! {
    pub struct MremapFlags: usize {
        const FIXED = 1;
        const FIXED_REPLACE = 3;
        /// Alias's memory region at `old_address` to `new_address` such that both regions share
        /// the same frames.
        const KEEP_OLD = 1 << 2;
        // TODO: MAYMOVE, DONTUNMAP
    }
}
bitflags! {
    pub struct RwFlags: u32 {
        const NONBLOCK = 1;
        const APPEND = 2;
        // TODO: sync/dsync
        // TODO: O_DIRECT?
    }
}
bitflags! {
    pub struct SigcontrolFlags: usize {
        /// Prevents the kernel from jumping the context to the signal trampoline, but otherwise
        /// has absolutely no effect on which signals are blocked etc. Meant to be used for
        /// short-lived critical sections inside libc.
        const INHIBIT_DELIVERY = 1;
    }
}
bitflags! {
    pub struct CallFlags: usize {
        // reserved
        const RSVD0 = 1 << 0;
        const RSVD1 = 1 << 1;
        const RSVD2 = 1 << 2;
        const RSVD3 = 1 << 3;
        const RSVD4 = 1 << 4;
        const RSVD5 = 1 << 5;
        const RSVD6 = 1 << 6;
        const RSVD7 = 1 << 7;

        /// Remove the fd from the caller's file table before sending the message.
        const CONSUME = 1 << 8;

        const WRITE = 1 << 9;
        const READ = 1 << 10;

        /// Indicates the request is a bulk fd passing request.
        const FD = 1 << 11;
        /// Flags for the fd passing request.
        const FD_EXCLUSIVE = 1 << 12;
        const FD_CLONE = 1 << 13;
        const FD_UPPER = 1 << 14;
    }
}

/// The tag for the fd number in the upper file descriptor table.
pub const UPPER_FDTBL_TAG: usize = 1 << (usize::BITS - 2);
