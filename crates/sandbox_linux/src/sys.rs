//! Thin wrappers over the raw syscalls the sandbox needs that `libc` / `nix`
//! don't wrap (or that we want with exact semantics).

use std::ffi::{CStr, CString};
use std::io;
use std::os::fd::{AsRawFd as _, BorrowedFd, FromRawFd as _, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};

pub(crate) const MOUNT_ATTR_RDONLY: u64 = 0x1;

// Syscalls added after the per-arch tables were unified share one number.
const SYS_PIDFD_OPEN: libc::c_long = 434;
const SYS_PIDFD_GETFD: libc::c_long = 438;
const SYS_MOUNT_SETATTR: libc::c_long = 442;
const AT_RECURSIVE: libc::c_uint = 0x8000;

pub(crate) fn errno(code: i32) -> io::Error {
    io::Error::from_raw_os_error(code)
}

pub(crate) fn cvt(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub(crate) fn cstring(bytes: &[u8]) -> io::Result<CString> {
    CString::new(bytes).map_err(|_| errno(libc::EINVAL))
}

pub(crate) fn path_cstring(path: &Path) -> io::Result<CString> {
    cstring(path.as_os_str().as_bytes())
}

/// `openat(2)`. `O_CLOEXEC` is always added.
pub(crate) fn openat(dirfd: RawFd, path: &CStr, flags: i32, mode: u32) -> io::Result<OwnedFd> {
    // SAFETY: valid NUL-terminated path.
    let fd = cvt(unsafe { libc::openat(dirfd, path.as_ptr(), flags | libc::O_CLOEXEC, mode) })?;
    // SAFETY: a freshly returned descriptor that nothing else owns.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

pub(crate) fn open(path: &Path, flags: i32) -> io::Result<OwnedFd> {
    openat(libc::AT_FDCWD, &path_cstring(path)?, flags, 0)
}

/// `/proc/self/fd/N`: opening or operating on this path reaches exactly the
/// object `fd` refers to, through the mount `fd` was opened on.
pub(crate) fn proc_self_fd(fd: BorrowedFd<'_>) -> PathBuf {
    PathBuf::from(format!("/proc/self/fd/{}", fd.as_raw_fd()))
}

/// The path of the object `fd` refers to, as rendered by the kernel.
///
/// For objects in another mount namespace (e.g. a sandbox's), the kernel
/// renders the path as seen from that namespace's root.
pub(crate) fn fd_path(fd: BorrowedFd<'_>) -> io::Result<PathBuf> {
    std::fs::read_link(proc_self_fd(fd))
}

pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<libc::stat> {
    // SAFETY: `stat` is plain old data, filled in by the kernel.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: valid fd and pointer to a `stat`.
    cvt(unsafe { libc::fstat(fd.as_raw_fd(), &mut stat) })?;
    Ok(stat)
}

pub(crate) fn mount(
    source: Option<&Path>,
    target: &Path,
    filesystem: Option<&str>,
    flags: libc::c_ulong,
    data: Option<&str>,
) -> io::Result<()> {
    let source = source.map(path_cstring).transpose()?;
    let target = path_cstring(target)?;
    let filesystem = filesystem
        .map(|name| cstring(name.as_bytes()))
        .transpose()?;
    let data = data.map(|data| cstring(data.as_bytes())).transpose()?;
    let pointer = |value: &Option<CString>| {
        value
            .as_ref()
            .map_or(std::ptr::null(), |value| value.as_ptr())
    };
    // SAFETY: every pointer is null or a valid NUL-terminated string.
    cvt(unsafe {
        libc::mount(
            pointer(&source),
            target.as_ptr(),
            pointer(&filesystem),
            flags,
            pointer(&data).cast(),
        )
    })?;
    Ok(())
}

#[repr(C)]
struct MountAttr {
    attr_set: u64,
    attr_clr: u64,
    propagation: u64,
    userns_fd: u64,
}

/// `mount_setattr(2)`: add `attributes` to the mount at `target` (and, with
/// `recursive`, every mount beneath it).
pub(crate) fn mount_setattr(target: &Path, attributes: u64, recursive: bool) -> io::Result<()> {
    let target = path_cstring(target)?;
    let attr = MountAttr {
        attr_set: attributes,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let flags = if recursive { AT_RECURSIVE } else { 0 };
    // SAFETY: valid path and a correctly sized `mount_attr`.
    let result = unsafe {
        libc::syscall(
            SYS_MOUNT_SETATTR,
            libc::AT_FDCWD,
            target.as_ptr(),
            flags,
            &attr as *const MountAttr,
            size_of::<MountAttr>(),
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Duplicate `fd` to the lowest free descriptor `>= minimum`, close-on-exec.
pub(crate) fn duplicate_above(fd: BorrowedFd<'_>, minimum: RawFd) -> io::Result<OwnedFd> {
    // SAFETY: fcntl on a valid descriptor.
    let duplicate = cvt(unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_DUPFD_CLOEXEC, minimum) })?;
    // SAFETY: a freshly returned descriptor that nothing else owns.
    Ok(unsafe { OwnedFd::from_raw_fd(duplicate) })
}

fn own_syscall_fd(result: libc::c_long) -> io::Result<OwnedFd> {
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: a freshly returned descriptor that nothing else owns.
    Ok(unsafe { OwnedFd::from_raw_fd(result as RawFd) })
}

/// A duplicate of descriptor `fd` of process `pid` (a thread group leader).
pub(crate) fn pidfd_getfd(pid: i32, fd: RawFd) -> io::Result<OwnedFd> {
    // SAFETY: plain syscalls; each returns a new descriptor or fails.
    let pidfd = own_syscall_fd(unsafe { libc::syscall(SYS_PIDFD_OPEN, pid, 0) })?;
    // SAFETY: as above.
    own_syscall_fd(unsafe { libc::syscall(SYS_PIDFD_GETFD, pidfd.as_raw_fd(), fd, 0) })
}

/// `socket(2)`. `SOCK_CLOEXEC` is always added.
pub(crate) fn socket(domain: i32, kind: i32, protocol: i32) -> io::Result<OwnedFd> {
    // SAFETY: plain socket call.
    let fd = cvt(unsafe { libc::socket(domain, kind | libc::SOCK_CLOEXEC, protocol) })?;
    // SAFETY: a freshly returned descriptor that nothing else owns.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

pub(crate) fn socket_option(fd: BorrowedFd<'_>, level: i32, name: i32) -> io::Result<i32> {
    let mut value: libc::c_int = 0;
    let mut length = size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: an int-sized buffer with its length.
    cvt(unsafe {
        libc::getsockopt(
            fd.as_raw_fd(),
            level,
            name,
            (&mut value as *mut libc::c_int).cast(),
            &mut length,
        )
    })?;
    Ok(value)
}

pub(crate) fn set_socket_option(
    fd: BorrowedFd<'_>,
    level: i32,
    name: i32,
    value: i32,
) -> io::Result<()> {
    // SAFETY: an int-sized value with its length.
    cvt(unsafe {
        libc::setsockopt(
            fd.as_raw_fd(),
            level,
            name,
            (&value as *const libc::c_int).cast(),
            size_of::<libc::c_int>() as libc::socklen_t,
        )
    })?;
    Ok(())
}

/// Wait until `fd` has one of `events`, giving up after `timeout` or as
/// soon as `keep_waiting` returns false.
pub(crate) fn wait_for(
    fd: BorrowedFd<'_>,
    events: libc::c_short,
    timeout: std::time::Duration,
    keep_waiting: &dyn Fn() -> bool,
) -> io::Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let mut poll_fd = libc::pollfd {
            fd: fd.as_raw_fd(),
            events,
            revents: 0,
        };
        // SAFETY: one valid pollfd.
        let ready = unsafe { libc::poll(&mut poll_fd, 1, 250) };
        if ready > 0 {
            return Ok(());
        }
        if ready < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::EINTR) {
                return Err(error);
            }
        }
        if !keep_waiting() {
            return Err(errno(libc::ECONNABORTED));
        }
        if std::time::Instant::now() >= deadline {
            return Err(errno(libc::ETIMEDOUT));
        }
    }
}

pub(crate) fn set_parent_death_signal() -> io::Result<()> {
    // SAFETY: plain prctl.
    cvt(unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL, 0, 0, 0) })?;
    Ok(())
}

/// Take ownership of a descriptor this process inherited at a fixed number.
///
/// # Safety
/// `fd` must be open and not owned by anything else in this process.
pub(crate) unsafe fn inherited_fd(fd: RawFd) -> io::Result<OwnedFd> {
    // SAFETY: fcntl on a possibly-closed descriptor only reports EBADF.
    let flags = cvt(unsafe { libc::fcntl(fd, libc::F_GETFD) })?;
    // SAFETY: as above.
    cvt(unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) })?;
    // SAFETY: guaranteed by the caller.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}
