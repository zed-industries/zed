//! Redox-specific system library.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "call")]
use self::error::{Error, Result};

pub mod error {
    use super::*;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    // TODO: Define as enum?
    pub struct Error {
        // TODO: NonZeroU16? "success" is not an error.
        errno: u16,
    }

    impl Error {
        pub fn new(errno: i32) -> Self {
            Self {
                errno: errno.try_into().unwrap_or(u16::MAX),
            }
        }
        pub fn errno(self) -> i32 {
            self.errno.into()
        }
        pub fn is_wouldblock(self) -> bool {
            matches!(self.errno(), errno::EAGAIN | errno::EWOULDBLOCK)
        }
        pub fn is_interrupt(self) -> bool {
            self.errno() == errno::EINTR
        }
        pub fn mux(res: Result<usize, Self>) -> usize {
            match res {
                // TODO: Ensure success cannot overlap with error values.
                Ok(success) => success,
                Err(error) => usize::wrapping_neg(usize::from(error.errno)),
            }
        }
        pub fn demux(res: usize) -> Result<usize, Self> {
            if res > usize::wrapping_neg(4096) {
                Err(Self {
                    errno: res.wrapping_neg().try_into().expect("2^BITS - res < 4096"),
                })
            } else {
                Ok(res)
            }
        }
        #[cfg(feature = "call")]
        pub fn description(self, buf: &mut [u8]) -> &str {
            call::strerror(self.errno, buf).map_or("unknown error", |(desc, _)| desc)
        }
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            #[cfg(feature = "call")]
            {
                write!(f, "{}", self.description(&mut [0; 256]))
            }
            #[cfg(not(feature = "call"))]
            {
                write!(f, "Error {} (strerror unavailable)", self.errno)
            }
        }
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            #[cfg(feature = "call")]
            {
                write!(
                    f,
                    "Error `{}` {}",
                    self.description(&mut [0; 256]),
                    self.errno
                )
            }
            #[cfg(not(feature = "call"))]
            {
                write!(f, "Error {} (strerror unavailable)", self.errno)
            }
        }
    }
    #[cfg(feature = "redox_syscall")]
    impl From<syscall::Error> for Error {
        fn from(value: syscall::Error) -> Self {
            Self {
                errno: value.errno.try_into().unwrap_or(u16::MAX),
            }
        }
    }
    #[cfg(feature = "redox_syscall")]
    impl From<Error> for syscall::Error {
        fn from(value: Error) -> Self {
            Self::new(value.errno())
        }
    }
    #[cfg(feature = "std")]
    impl From<Error> for std::io::Error {
        fn from(value: Error) -> Self {
            Self::from_raw_os_error(value.errno.into())
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for Error {}

    pub type Result<T, E = Error> = core::result::Result<T, E>;
}

pub mod flag {
    pub use libc::{
        O_ACCMODE, O_APPEND, O_ASYNC, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL, O_FSYNC, O_NOFOLLOW,
        O_NONBLOCK, O_PATH, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
    };

    pub use libc::{CLOCK_MONOTONIC, CLOCK_REALTIME};

    pub use libc::{SIG_BLOCK, SIG_SETMASK, SIG_UNBLOCK};

    pub use libc::{
        SIGABRT,
        SIGALRM,
        SIGBUS,
        SIGCHLD,
        SIGCONT,
        SIGFPE,
        // TODO: Const rather than use, to convert to u32
        SIGHUP,
        SIGILL,
        SIGINT,
        SIGIO,
        SIGKILL,
        SIGPIPE,
        SIGPROF,
        SIGPWR,
        SIGQUIT,
        SIGSEGV,
        SIGSTKFLT,
        SIGSYS,
        SIGTERM,
        SIGTRAP,
        SIGTSTP,
        SIGTTIN,
        SIGTTOU,
        SIGURG,
        SIGUSR1,
        SIGUSR2,
        SIGVTALRM,
        SIGWINCH,
        SIGXFSZ,
    };

    #[cfg(target_os = "redox")]
    pub use libc::{O_EXLOCK, O_SHLOCK, O_SYMLINK};

    pub const MAP_SHARED: u32 = libc::MAP_SHARED as u32;
    pub const MAP_PRIVATE: u32 = libc::MAP_PRIVATE as u32;

    pub const PROT_NONE: u32 = libc::PROT_NONE as u32;
    pub const PROT_READ: u32 = libc::PROT_READ as u32;
    pub const PROT_WRITE: u32 = libc::PROT_WRITE as u32;
    pub const PROT_EXEC: u32 = libc::PROT_EXEC as u32;

    pub const WNOHANG: u32 = libc::WNOHANG as u32;
    pub const WUNTRACED: u32 = libc::WUNTRACED as u32;
    pub const WCONTINUED: u32 = libc::WCONTINUED as u32;
}

pub mod errno {
    pub use libc::{
        E2BIG, EACCES, EADDRINUSE, EADDRNOTAVAIL, EADV, EAFNOSUPPORT, EAGAIN, EALREADY, EBADE,
        EBADF, EBADFD, EBADMSG, EBADR, EBADRQC, EBADSLT, EBFONT, EBUSY, ECANCELED, ECHILD, ECHRNG,
        ECOMM, ECONNABORTED, ECONNREFUSED, ECONNRESET, EDEADLK, EDEADLOCK, EDESTADDRREQ, EDOM,
        EDOTDOT, EDQUOT, EEXIST, EFAULT, EFBIG, EHOSTDOWN, EHOSTUNREACH, EIDRM, EILSEQ,
        EINPROGRESS, EINTR, EINVAL, EIO, EISCONN, EISDIR, EISNAM, EKEYEXPIRED, EKEYREJECTED,
        EKEYREVOKED, EL2HLT, EL2NSYNC, EL3HLT, EL3RST, ELIBACC, ELIBBAD, ELIBEXEC, ELIBMAX,
        ELIBSCN, ELNRNG, ELOOP, EMEDIUMTYPE, EMFILE, EMLINK, EMSGSIZE, EMULTIHOP, ENAMETOOLONG,
        ENAVAIL, ENETDOWN, ENETRESET, ENETUNREACH, ENFILE, ENOANO, ENOBUFS, ENOCSI, ENODEV, ENOENT,
        ENOEXEC, ENOKEY, ENOLCK, ENOMEDIUM, ENOMEM, ENOMSG, ENONET, ENOPKG, ENOPROTOOPT, ENOSPC,
        ENOSR, ENOSTR, ENOSYS, ENOTBLK, ENOTCONN, ENOTDIR, ENOTEMPTY, ENOTNAM, ENOTRECOVERABLE,
        ENOTSOCK, ENOTTY, ENOTUNIQ, ENXIO, EOPNOTSUPP, EOVERFLOW, EOWNERDEAD, EPERM, EPFNOSUPPORT,
        EPIPE, EPROTO, EPROTONOSUPPORT, EPROTOTYPE, ERANGE, EREMCHG, EREMOTE, EREMOTEIO, ERESTART,
        EROFS, ESHUTDOWN, ESOCKTNOSUPPORT, ESPIPE, ESRCH, ESRMNT, ESTALE, ESTRPIPE, ETIME,
        ETIMEDOUT, ETOOMANYREFS, ETXTBSY, EUCLEAN, EUNATCH, EUSERS, EWOULDBLOCK, EXDEV, EXFULL,
    };
}
pub mod data {
    pub use libc::iovec as IoVec;
    pub use libc::sigaction as SigAction;
    pub use libc::stat as Stat;
    pub use libc::statvfs as StatVfs;
    pub use libc::timespec as TimeSpec;

    // TODO: Remove
    pub fn timespec_from_mut_bytes(bytes: &mut [u8]) -> &mut TimeSpec {
        assert!(bytes.len() >= core::mem::size_of::<TimeSpec>());
        assert_eq!(
            bytes.as_ptr() as usize % core::mem::align_of::<TimeSpec>(),
            0
        );
        unsafe { &mut *bytes.as_mut_ptr().cast() }
    }
    // TODO: Remove
    pub fn timespec_from_bytes(bytes: &[u8]) -> &TimeSpec {
        assert!(bytes.len() >= core::mem::size_of::<TimeSpec>());
        assert_eq!(
            bytes.as_ptr() as usize % core::mem::align_of::<TimeSpec>(),
            0
        );
        unsafe { &*bytes.as_ptr().cast() }
    }

    #[cfg(target_os = "redox")]
    pub use libc::sigset_t as SigSet;

    // TODO: Should libredox compile on non-Redox platforms?
    #[cfg(not(target_os = "redox"))]
    pub type SigSet = u64;
}

#[cfg(feature = "call")]
type RawResult = usize;

#[cfg(feature = "call")]
#[allow(dead_code)]
extern "C" {
    // NOTE: Although there are version suffixes, there'd have to be strong reasons for adding new
    // version.
    fn redox_open_v1(path_base: *const u8, path_len: usize, flags: u32, mode: u16) -> RawResult;
    fn redox_openat_v1(
        fd: usize,
        buf: *const u8,
        path_len: usize,
        flags: u32,
        fcntl_flags: u32,
    ) -> RawResult;
    fn redox_dup_v1(fd: usize, buf: *const u8, len: usize) -> RawResult;
    fn redox_dup2_v1(old_fd: usize, new_fd: usize, buf: *const u8, len: usize) -> RawResult;
    fn redox_read_v1(fd: usize, dst_base: *mut u8, dst_len: usize) -> RawResult;
    fn redox_write_v1(fd: usize, src_base: *const u8, src_len: usize) -> RawResult;
    fn redox_fsync_v1(fd: usize) -> RawResult;
    fn redox_fdatasync_v1(fd: usize) -> RawResult;
    fn redox_fchmod_v1(fd: usize, new_mode: u16) -> RawResult;
    fn redox_fchown_v1(fd: usize, new_uid: u32, new_gid: u32) -> RawResult;
    fn redox_fpath_v1(fd: usize, dst_base: *mut u8, dst_len: usize) -> RawResult;
    fn redox_fstat_v1(fd: usize, dst: *mut data::Stat) -> RawResult;
    fn redox_fstatvfs_v1(fd: usize, dst: *mut data::StatVfs) -> RawResult;
    fn redox_futimens_v1(fd: usize, times: *const data::TimeSpec) -> RawResult;
    fn redox_close_v1(fd: usize) -> RawResult;

    // NOTE: While the Redox kernel currently doesn't distinguish between threads and processes,
    // the return value of this function is expected to be treated as a process ID and not a thread
    // ID.
    fn redox_get_pid_v1() -> RawResult;

    fn redox_get_euid_v1() -> RawResult;
    fn redox_get_ruid_v1() -> RawResult;
    fn redox_get_egid_v1() -> RawResult;
    fn redox_get_rgid_v1() -> RawResult;

    fn redox_get_ens_v0() -> RawResult;

    // This function is used to get the credentials, pid, euid, egid etc. of the process with the target pid.
    fn redox_get_proc_credentials_v1(cap_fd: usize, target_pid: usize, buf: &mut [u8])
        -> RawResult;

    fn redox_setrens_v1(rns: usize, ens: usize) -> RawResult;
    fn redox_mkns_v1(names: *const data::IoVec, num_names: usize, _flags: u32) -> RawResult;

    fn redox_kill_v1(pid: usize, signal: u32) -> RawResult;
    fn redox_waitpid_v1(pid: usize, status: *mut i32, options: u32) -> RawResult;

    fn redox_sigprocmask_v1(how: u32, new: *const u64, old: *mut u64) -> RawResult;
    fn redox_sigaction_v1(
        signal: u32,
        new: *const data::SigAction,
        old: *mut data::SigAction,
    ) -> RawResult;

    fn redox_clock_gettime_v1(clock: usize, ts: *mut data::TimeSpec) -> RawResult;

    fn redox_mmap_v1(
        addr: *mut (),
        unaligned_len: usize,
        prot: u32,
        flags: u32,
        fd: usize,
        offset: u64,
    ) -> RawResult;
    fn redox_munmap_v1(addr: *mut (), unaligned_len: usize) -> RawResult;

    fn redox_strerror_v1(dst: *mut u8, dst_len: *mut usize, error: u32) -> RawResult;

    fn redox_sys_call_v0(
        fd: usize,
        payload: *mut u8,
        payload_len: usize,
        flags: usize,
        metadata: *const u64,
        metadata_len: usize,
    ) -> RawResult;

    fn redox_get_socket_token_v0(fd: usize, payload: *mut u8, payload_len: usize) -> RawResult;
}

#[cfg(feature = "call")]
pub struct Fd(usize);

#[cfg(feature = "call")]
impl Fd {
    #[inline]
    pub fn open(path: &str, flags: i32, mode: u16) -> Result<Self> {
        Ok(Self(call::open(path, flags, mode)?))
    }
    pub fn openat(&self, path: &str, flags: i32, fcntl_flags: u32) -> Result<Self> {
        Ok(Self(call::openat(self.raw(), path, flags, fcntl_flags)?))
    }
    #[inline]
    pub fn dup(&self, buf: &[u8]) -> Result<usize> {
        call::dup(self.raw(), buf)
    }
    #[inline]
    pub fn dup2(&self, new_fd: usize, buf: &[u8]) -> Result<usize> {
        call::dup2(self.raw(), new_fd, buf)
    }

    #[inline]
    pub const fn raw(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn into_raw(self) -> usize {
        let raw = self.raw();
        core::mem::forget(self);
        raw
    }

    #[inline]
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        call::read(self.raw(), buf)
    }
    #[inline]
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        call::write(self.raw(), buf)
    }
    #[inline]
    pub fn fpath(&self, path: &mut [u8]) -> Result<usize> {
        call::fpath(self.raw(), path)
    }
    #[inline]
    pub fn fsync(&self) -> Result<()> {
        call::fsync(self.raw())
    }
    #[inline]
    pub fn fdatasync(&self) -> Result<()> {
        call::fdatasync(self.raw())
    }

    #[inline]
    pub fn chmod(&self, new_mode: u16) -> Result<()> {
        call::fchmod(self.raw(), new_mode)
    }
    #[inline]
    pub fn chown(&self, new_uid: u32, new_gid: u32) -> Result<()> {
        call::fchown(self.raw(), new_uid, new_gid)
    }
    pub fn stat(&self) -> Result<data::Stat> {
        call::fstat(self.raw())
    }
    pub fn statvfs(&self) -> Result<data::StatVfs> {
        call::fstatvfs(self.raw())
    }
    // TODO: futimens
    #[inline]
    pub fn close(self) -> Result<()> {
        call::close(self.into_raw())
    }

    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_ro(
        &self,
        payload: &mut [u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        call::call_ro(self.raw(), payload, flags, metadata)
    }
    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_wo(
        &self,
        payload: &[u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        call::call_wo(self.raw(), payload, flags, metadata)
    }
    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_rw(
        &self,
        payload: &mut [u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        call::call_rw(self.raw(), payload, flags, metadata)
    }
}
#[cfg(feature = "call")]
impl Drop for Fd {
    fn drop(&mut self) {
        let _ = unsafe { redox_close_v1(self.0) };
    }
}

#[cfg(feature = "call")]
pub mod call {
    use core::mem::MaybeUninit;

    use super::*;

    /// flags and mode are binary compatible with libc
    #[inline]
    pub fn open(path: impl AsRef<str>, flags: i32, mode: u16) -> Result<usize> {
        let path = path.as_ref();
        Ok(Error::demux(unsafe {
            redox_open_v1(path.as_ptr(), path.len(), flags as u32, mode)
        })?)
    }
    #[inline]
    pub fn openat(
        fd: usize,
        path: impl AsRef<[u8]>,
        flags: i32,
        fcntl_flags: u32,
    ) -> Result<usize> {
        let path = path.as_ref();
        Ok(Error::demux(unsafe {
            redox_openat_v1(fd, path.as_ptr(), path.len(), flags as u32, fcntl_flags)
        })?)
    }
    #[inline]
    pub fn dup(fd: usize, buf: impl AsRef<[u8]>) -> Result<usize> {
        let buf = buf.as_ref();
        Ok(Error::demux(unsafe {
            redox_dup_v1(fd, buf.as_ptr(), buf.len())
        })?)
    }
    #[inline]
    pub fn dup2(old_fd: usize, new_fd: usize, buf: impl AsRef<[u8]>) -> Result<usize> {
        let buf = buf.as_ref();
        Ok(Error::demux(unsafe {
            redox_dup2_v1(old_fd, new_fd, buf.as_ptr(), buf.len())
        })?)
    }
    #[inline]
    pub fn read(raw_fd: usize, buf: &mut [u8]) -> Result<usize> {
        Ok(Error::demux(unsafe {
            redox_read_v1(raw_fd, buf.as_mut_ptr(), buf.len())
        })?)
    }
    #[inline]
    pub fn write(raw_fd: usize, buf: &[u8]) -> Result<usize> {
        Error::demux(unsafe { redox_write_v1(raw_fd, buf.as_ptr(), buf.len()) })
    }
    #[inline]
    pub fn fsync(raw_fd: usize) -> Result<()> {
        Error::demux(unsafe { redox_fsync_v1(raw_fd) }).map(|_| ())
    }
    #[inline]
    pub fn fdatasync(raw_fd: usize) -> Result<()> {
        Error::demux(unsafe { redox_fdatasync_v1(raw_fd) }).map(|_| ())
    }
    #[inline]
    pub fn fchmod(raw_fd: usize, new_mode: u16) -> Result<()> {
        Error::demux(unsafe { redox_fchmod_v1(raw_fd, new_mode) })?;
        Ok(())
    }
    #[inline]
    pub fn fchown(raw_fd: usize, new_uid: u32, new_gid: u32) -> Result<()> {
        Error::demux(unsafe { redox_fchown_v1(raw_fd, new_uid, new_gid) })?;
        Ok(())
    }
    #[inline]
    pub fn fpath(raw_fd: usize, buf: &mut [u8]) -> Result<usize> {
        Error::demux(unsafe { redox_fpath_v1(raw_fd, buf.as_mut_ptr(), buf.len()) })
    }
    #[inline]
    pub fn fstat(raw_fd: usize) -> Result<data::Stat> {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            Error::demux(redox_fstat_v1(raw_fd, ret.as_mut_ptr()))?;
            Ok(ret.assume_init())
        }
    }
    #[inline]
    pub fn fstatvfs(raw_fd: usize) -> Result<data::StatVfs> {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            Error::demux(redox_fstatvfs_v1(raw_fd, ret.as_mut_ptr()))?;
            Ok(ret.assume_init())
        }
    }
    #[inline]
    pub fn futimens(raw_fd: usize, times: &[data::TimeSpec; 2]) -> Result<()> {
        Error::demux(unsafe { redox_futimens_v1(raw_fd, times.as_ptr()) })?;
        Ok(())
    }
    #[inline]
    pub fn close(raw_fd: usize) -> Result<()> {
        Error::demux(unsafe { redox_close_v1(raw_fd) })?;
        Ok(())
    }
    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_ro(
        fd: usize,
        payload: &mut [u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        Ok(Error::demux(unsafe {
            redox_sys_call_v0(
                fd,
                payload.as_mut_ptr(),
                payload.len(),
                (flags | syscall::CallFlags::READ).bits(),
                metadata.as_ptr(),
                metadata.len(),
            )
        })?)
    }
    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_wo(
        fd: usize,
        payload: &[u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        Ok(Error::demux(unsafe {
            redox_sys_call_v0(
                fd,
                payload.as_ptr() as *mut u8,
                payload.len(),
                (flags | syscall::CallFlags::WRITE).bits(),
                metadata.as_ptr(),
                metadata.len(),
            )
        })?)
    }
    #[cfg(feature = "redox_syscall")]
    #[inline]
    pub fn call_rw(
        fd: usize,
        payload: &mut [u8],
        flags: syscall::CallFlags,
        metadata: &[u64],
    ) -> Result<usize> {
        Ok(Error::demux(unsafe {
            redox_sys_call_v0(
                fd,
                payload.as_mut_ptr(),
                payload.len(),
                (flags | syscall::CallFlags::READ | syscall::CallFlags::WRITE).bits(),
                metadata.as_ptr(),
                metadata.len(),
            )
        })?)
    }

    #[inline]
    pub fn geteuid() -> Result<usize> {
        Error::demux(unsafe { redox_get_euid_v1() })
    }

    #[inline]
    pub fn getruid() -> Result<usize> {
        Error::demux(unsafe { redox_get_ruid_v1() })
    }

    #[inline]
    pub fn getegid() -> Result<usize> {
        Error::demux(unsafe { redox_get_egid_v1() })
    }
    #[inline]
    pub fn getrgid() -> Result<usize> {
        Error::demux(unsafe { redox_get_rgid_v1() })
    }
    #[inline]
    pub fn getpid() -> Result<usize> {
        Error::demux(unsafe { redox_get_pid_v1() })
    }
    #[inline]
    pub fn getens() -> Result<usize> {
        Error::demux(unsafe { redox_get_ens_v0() })
    }

    #[inline]
    // [u8; size_of::<crate::protocol::ProcMeta>()]
    pub fn get_proc_credentials(cap_fd: usize, target_pid: usize, buf: &mut [u8]) -> Result<usize> {
        Error::demux(unsafe { redox_get_proc_credentials_v1(cap_fd, target_pid, buf) })
    }

    #[inline]
    pub fn setrens(rns: usize, ens: usize) -> Result<usize> {
        Error::demux(unsafe { redox_setrens_v1(rns, ens) })
    }
    #[inline]
    pub fn waitpid(pid: usize, status: &mut i32, options: i32) -> Result<usize> {
        Error::demux(unsafe { redox_waitpid_v1(pid, status as *mut i32, options as u32) })
    }
    #[inline]
    pub fn kill(pid: usize, signal: u32) -> Result<()> {
        Error::demux(unsafe { redox_kill_v1(pid, signal) }).map(|_| ())
    }
    #[inline]
    pub fn clock_gettime(clock: i32) -> Result<data::TimeSpec> {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            Error::demux(redox_clock_gettime_v1(clock as usize, ret.as_mut_ptr()))?;
            Ok(ret.assume_init())
        }
    }
    #[inline]
    pub fn sigprocmask(
        how: i32,
        newmask: Option<&data::SigSet>,
        oldmask: Option<&mut data::SigSet>,
    ) -> Result<()> {
        Error::demux(unsafe {
            redox_sigprocmask_v1(
                how as u32,
                newmask.map_or(core::ptr::null(), |m| m),
                oldmask.map_or(core::ptr::null_mut(), |m| m),
            )
        })
        .map(|_| ())
    }
    #[inline]
    pub fn sigaction(
        signal: i32,
        newact: Option<&data::SigAction>,
        oldact: Option<&mut data::SigAction>,
    ) -> Result<()> {
        Error::demux(unsafe {
            redox_sigaction_v1(
                signal as u32,
                newact.map_or(core::ptr::null(), |m| m),
                oldact.map_or(core::ptr::null_mut(), |m| m),
            )
        })
        .map(|_| ())
    }

    #[derive(Clone, Copy, Debug)]
    pub struct MmapArgs {
        pub addr: *mut (),
        pub length: usize,
        pub prot: u32,
        pub flags: u32,
        pub fd: usize,
        pub offset: u64,
    }
    #[inline]
    pub unsafe fn mmap(args: MmapArgs) -> Result<*mut ()> {
        Error::demux(redox_mmap_v1(
            args.addr,
            args.length,
            args.prot,
            args.flags,
            args.fd,
            args.offset,
        ))
        .map(|addr| addr as *mut ())
    }
    #[inline]
    pub unsafe fn munmap(addr: *mut (), length: usize) -> Result<()> {
        Error::demux(redox_munmap_v1(addr, length)).map(|_| ())
    }

    #[inline]
    pub fn strerror(error: u16, desc: &mut [u8]) -> Option<(&str, usize)> {
        unsafe {
            let mut len_inout = desc.len();
            let copied_len = Error::demux(redox_strerror_v1(
                desc.as_mut_ptr(),
                &mut len_inout,
                error.into(),
            ))
            .ok()?;
            Some((
                core::str::from_utf8_unchecked(&desc[..copied_len]),
                len_inout,
            ))
        }
    }

    #[inline]
    #[cfg(feature = "mkns")]
    pub fn mkns(names: &[ioslice::IoSlice]) -> Result<usize> {
        // no-op
        let iovecs = ioslice::IoSlice::cast_to_raw_iovecs(names);

        unsafe { Error::demux(redox_mkns_v1(iovecs.as_ptr(), iovecs.len(), 0)) }
    }

    #[inline]
    pub fn get_socket_token(fd: usize, buf: &mut [u8]) -> Result<usize> {
        Error::demux(unsafe { redox_get_socket_token_v0(fd, buf.as_mut_ptr(), buf.len()) })
    }
}
