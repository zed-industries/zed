use core::{
    mem,
    ops::{Deref, DerefMut},
    slice,
};

use bitflags::bitflags;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Sqe {
    pub opcode: u8,
    pub sqe_flags: SqeFlags,
    pub _rsvd: u16, // TODO: priority
    pub tag: u32,
    pub args: [u64; 6],
    pub caller: u64,
}
impl Deref for Sqe {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Sqe as *const u8, mem::size_of::<Sqe>()) }
    }
}

impl DerefMut for Sqe {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Sqe as *mut u8, mem::size_of::<Sqe>()) }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct SqeFlags: u8 {
        // If zero, the message is bidirectional, and the scheme is expected to pass the Ksmsg's
        // tag field to the Skmsg. Some opcodes require this flag to be set.
        const ONEWAY = 1;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Cqe {
    pub flags: u8, // bits 2:0 are CqeOpcode
    pub extra_raw: [u8; 3],
    pub tag: u32,
    pub result: u64,
}
impl Deref for Cqe {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Cqe as *const u8, mem::size_of::<Cqe>()) }
    }
}

impl DerefMut for Cqe {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Cqe as *mut u8, mem::size_of::<Cqe>()) }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct NewFdFlags: u8 {
        const POSITIONED = 1;
    }
}

impl Cqe {
    pub fn extra(&self) -> u32 {
        u32::from_ne_bytes([self.extra_raw[0], self.extra_raw[1], self.extra_raw[2], 0])
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CqeOpcode {
    RespondRegular,
    RespondWithFd,
    SendFevent, // no tag
    ObtainFd,
    RespondWithMultipleFds,
    // TODO: ProvideMmap
}
impl CqeOpcode {
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::RespondRegular,
            1 => Self::RespondWithFd,
            2 => Self::SendFevent,
            3 => Self::ObtainFd,
            4 => Self::RespondWithMultipleFds,
            _ => return None,
        })
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Open = 0,    // path_ptr, path_len (utf8), flags
    Rmdir = 1,   // path_ptr, path_len (utf8)
    Unlink = 2,  // path_ptr, path_len (utf8)
    Close = 3,   // fd
    Dup = 4,     // old fd, buf_ptr, buf_len
    Read = 5,    // fd, buf_ptr, buf_len, TODO offset, TODO flags, _
    Write = 6,   // fd, buf_ptr, buf_len, TODO offset, TODO flags)
    Fsize = 7,   // fd
    Fchmod = 8,  // fd, new mode
    Fchown = 9,  // fd, new uid, new gid
    Fcntl = 10,  // fd, cmd, arg
    Fevent = 11, // fd, requested mask
    Sendfd = 12,
    Fpath = 13, // fd, buf_ptr, buf_len
    Frename = 14,
    Fstat = 15,     // fd, buf_ptr, buf_len
    Fstatvfs = 16,  // fd, buf_ptr, buf_len
    Fsync = 17,     // fd
    Ftruncate = 18, // fd, new len
    Futimens = 19,  // fd, times_buf, times_len

    MmapPrep = 20,
    RequestMmap = 21,
    Mremap = 22,
    Munmap = 23,
    Msync = 24, // TODO

    Cancel = 25, // @tag

    Getdents = 26,
    CloseMsg = 27,
    Call = 28,

    OpenAt = 29, // fd, buf_ptr, buf_len, flags
    Flink = 30,

    Recvfd = 31,
}

impl Opcode {
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        use Opcode::*;

        // TODO: Use a library where this match can be automated.
        Some(match raw {
            0 => Open,
            1 => Rmdir,
            2 => Unlink,
            3 => Close,
            4 => Dup,
            5 => Read,
            6 => Write,
            7 => Fsize,
            8 => Fchmod,
            9 => Fchown,
            10 => Fcntl,
            11 => Fevent,
            12 => Sendfd,
            13 => Fpath,
            14 => Frename,
            15 => Fstat,
            16 => Fstatvfs,
            17 => Fsync,
            18 => Ftruncate,
            19 => Futimens,

            20 => MmapPrep,
            21 => RequestMmap,
            22 => Mremap,
            23 => Munmap,
            24 => Msync,

            25 => Cancel,
            26 => Getdents,
            27 => CloseMsg,
            28 => Call,

            29 => OpenAt,
            30 => Flink,

            31 => Recvfd,

            _ => return None,
        })
    }
}
