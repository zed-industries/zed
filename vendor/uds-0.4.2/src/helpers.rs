/* See each function for copyright holders */

/// Functions to handle OS differences.
/// Several adapted from std.

use std::convert::TryInto;
use std::os::unix::io::{RawFd, AsRawFd, IntoRawFd};
use std::io::{self, ErrorKind};
use std::mem;
use std::time::Duration;

use libc::{c_int, sockaddr, socklen_t, AF_UNIX};
use libc::{bind, connect, getsockname, getpeername};
use libc::{socket, accept, close, listen, socketpair};
use libc::{ioctl, FIONBIO};
#[cfg(not(target_os = "haiku"))]
use libc::{FIOCLEX,FIONCLEX};
use libc::{fcntl, F_DUPFD_CLOEXEC, EINVAL, dup};
use libc::{getsockopt, SOL_SOCKET, SO_ERROR, c_void};
#[cfg_attr(target_env="musl", allow(deprecated))]
use libc::{setsockopt, SO_RCVTIMEO, SO_SNDTIMEO, timeval, time_t};
#[cfg(any(target_os="illumos", target_os="solaris"))]
use libc::{F_GETFD, F_SETFD, FD_CLOEXEC};
#[cfg(not(any(target_vendor="apple", target_os = "haiku")))]
use libc::{SOCK_CLOEXEC, SOCK_NONBLOCK};
#[cfg(not(any(target_vendor="apple", target_os = "haiku", all(target_os="android", target_arch="x86"))))]
use libc::{accept4, ENOSYS};
#[cfg(target_vendor="apple")]
use libc::SO_NOSIGPIPE;

use crate::addr::*;



const LISTEN_BACKLOG: c_int = 10; // what std uses, I think



#[cfg(not(target_vendor="apple"))]
pub use libc::MSG_NOSIGNAL;
#[cfg(target_vendor="apple")]
pub const MSG_NOSIGNAL: c_int = 0; // SO_NOSIGPIPE is set instead



/// Enables / disables CLOEXEC, for when SOCK_CLOEXEC can't be used.
#[cfg(not(target_os = "haiku"))]
pub fn set_cloexec(fd: RawFd,  close_on_exec: bool) -> Result<(), io::Error> {
    let op = if close_on_exec {FIOCLEX} else {FIONCLEX};
    match cvt!(unsafe { ioctl(fd, op) }) {
        Ok(_) => Ok(()),
        #[cfg(any(target_os="illumos", target_os="solaris"))]
        Err(ref e) if e.kind() == ErrorKind::InvalidInput => {
            unsafe {
                let prev = cvt!(fcntl(fd, F_GETFD))?;
                let change_to = if close_on_exec {prev | FD_CLOEXEC} else {prev & !FD_CLOEXEC};
                if change_to != prev {
                    cvt!(fcntl(fd, F_SETFD, change_to))?;
                }
                Ok(())
            }
        },
        Err(e) => Err(e),
    }
}

#[cfg(target_os = "haiku")]
pub fn set_cloexec(_: RawFd,  _: bool) -> Result<(), io::Error> {
    Ok(())
}

/// Enables / disables FIONBIO. Used if SOCK_NONBLOCK can't be used.
pub fn set_nonblocking(fd: RawFd,  nonblocking: bool) -> Result<(), io::Error> {
    cvt!(unsafe { ioctl(fd, FIONBIO, &mut (nonblocking as c_int)) })?;
    Ok(())
}



pub struct SetAddr(unsafe extern "C" fn(RawFd, *const sockaddr, socklen_t) -> c_int);
impl SetAddr {
    pub const LOCAL: Self = SetAddr(bind);
    pub const PEER: Self = SetAddr(connect);
}
/// Safe wrapper around `bind()` or `connect()`, that retries on EINTR.
pub fn set_unix_addr(socket: RawFd,  set_side: SetAddr,  addr: &UnixSocketAddr)
-> Result<(), io::Error> {
    unsafe {
        let (addr, len) = addr.as_raw_general();
        // check for EINTR just in case. If the file system is slow or somethhing.
        loop {
            if (set_side.0)(socket, addr, len) != -1 {
                break Ok(());
            }
            let err = io::Error::last_os_error();
            if err.kind() != ErrorKind::Interrupted {
                break Err(err);
            }
        }
    }
}

pub struct GetAddr(unsafe extern "C" fn(RawFd, *mut sockaddr, *mut socklen_t) -> c_int);
impl GetAddr {
    pub const LOCAL: Self = GetAddr(getsockname);
    pub const PEER: Self = GetAddr(getpeername);
}
/// Safe wrapper around `getsockname()` or `getpeername()`.
pub fn get_unix_addr(socket: RawFd,  get_side: GetAddr)
-> Result<UnixSocketAddr, io::Error> {
    unsafe {
        UnixSocketAddr::new_from_ffi(|addr_ptr, addr_len| {
            match (get_side.0)(socket, addr_ptr, addr_len) {
                -1 => Err(io::Error::last_os_error()),
                _ => Ok(()),
            }
        }).map(|((), addr)| addr )
    }
}

/// Safe wrapper around `getsockopt(SO_ERROR)`.
pub fn take_error(socket: RawFd) -> Result<Option<io::Error>, io::Error> {
    unsafe {
        let mut stored_errno: c_int = 0;
        let mut optlen = mem::size_of::<c_int>() as socklen_t;
        let dst_ptr = &mut stored_errno as *mut c_int as *mut c_void;
        if getsockopt(socket, SOL_SOCKET, SO_ERROR, dst_ptr, &mut optlen) == -1 {
            Err(io::Error::last_os_error())
        } else if optlen != mem::size_of::<c_int>() as socklen_t {
            // std panics here
            Err(io::Error::new(
                ErrorKind::InvalidData,
                "got unexpected length from getsockopt(SO_ERROR)"
            ))
        } else if stored_errno == 0 {
            Ok(None)
        } else {
            Ok(Some(io::Error::from_raw_os_error(stored_errno)))
        }
    }
}

#[repr(C)]
pub struct TimeoutDirection(c_int);
impl TimeoutDirection {
    pub const READ: Self = TimeoutDirection(SO_RCVTIMEO);
    pub const WRITE: Self = TimeoutDirection(SO_SNDTIMEO);
}
/// Safe wrapper around `setsockopt(SO_RCVTIMEO)` or `setsockopt(SO_SNDTIMEO)`.
pub fn set_timeout(socket: RawFd,  direction: TimeoutDirection,  timeout: Option<Duration>)
-> Result<(), io::Error> {
    let mut time = unsafe { mem::zeroed::<timeval>() };
    if let Some(duration) = timeout {
        time.tv_sec = match duration.as_secs().try_into() {
            Ok(seconds) => seconds,
            // Setting it to the max value is what std does.
            // tv_sec is time_t on all unices supported by libc.
            // (there is no polymorphic way to get the max value of a signed type.)
            // TODO change to ::MAX after MSRV is bumped to 1.43.
            #[cfg_attr(target_env="musl", allow(deprecated))]
            Err(_) => time_t::max_value() as _,
        };
        time.tv_usec = duration.subsec_micros() as _;

        if time.tv_sec == 0  &&  time.tv_usec == 0 {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "cannot set a 0 duration timeout"
            ));
        }
    }

    unsafe {
        let time_ptr = &time as *const timeval as *const c_void;
        let time_size = mem::size_of::<timeval>() as socklen_t;
        let option = direction.0;
        cvt!(setsockopt(socket, SOL_SOCKET, option, time_ptr, time_size))?;
    }
    Ok(())
}
/// Safe wrapper around `getsockopt(SO_RCVTIMEO)` or `getsockopt(SO_SNDTIMEO)`.
pub fn get_timeout(socket: RawFd,  direction: TimeoutDirection)
-> Result<Option<Duration>, io::Error> {
    let timeout = unsafe {
        let option = direction.0;
        let mut time = mem::zeroed::<timeval>();
        let time_ptr = &mut time as *mut timeval as *mut c_void;
        let mut time_size = mem::size_of::<timeval>() as socklen_t;
        cvt!(getsockopt(socket, SOL_SOCKET, option, time_ptr, &mut time_size))?;
        if time_size as usize != mem::size_of::<timeval>() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "timeout has unexpected size"
            ));
        }
        time
    };

    if timeout.tv_sec < 0 {
        Err(io::Error::new(ErrorKind::InvalidData, "timeout is negative"))
    } else if timeout.tv_usec < 0  ||  timeout.tv_usec >= 1_000_000 {
        Err(io::Error::new(ErrorKind::InvalidData, "timeout has invalid microsecond part"))
    } else if timeout.tv_sec == 0  &&  timeout.tv_usec == 0 {
        Ok(None)
    } else {
        Ok(Some(Duration::new(timeout.tv_sec as u64, timeout.tv_usec as u32 * 1000)))
    }
}



/// Used in setup of sockets to ensure the file descriptor is always closed
/// if later parts of the setup fails.
pub struct Socket(RawFd);

impl Drop for Socket {
    fn drop(&mut self) {
        // ignore errors - unlikely and there is nowhere to return them
        unsafe { close(self.0) };
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.0;
        mem::forget(self);
        fd
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl Socket {
    /// Enable / disable Apple-only SO_NOSIGPIPE
    fn set_nosigpipe(&self,  nosigpipe: bool) -> Result<(), io::Error> {
        #![allow(unused_variables)]
        #[cfg(target_vendor="apple")] {
            unsafe {
                let nosigpipe = &(nosigpipe as c_int) as *const c_int as *const c_void;
                let int_size = mem::size_of::<c_int>() as socklen_t;
                cvt!(setsockopt(self.0, SOL_SOCKET, SO_NOSIGPIPE, nosigpipe, int_size))?;
            }
        }
        Ok(())
    }

    pub fn new(socket_type: c_int,  nonblocking: bool) -> Result<Self, io::Error> {
        // Set close-on-exec atomically with SOCK_CLOEXEC if possible.
        // Falls through to the portable but race-prone way for compatibility
        // with Linux < 2.6.27 becaue Rust std still supports 2.6.18.
        // (EINVAL is what std checks for, and EPROTONOTSUPPORT is for
        // known-but-not-supported protcol or protocol families).
        #[cfg(not(any(target_vendor="apple", target_os = "haiku")))] {
            let type_flags = socket_type | SOCK_CLOEXEC | if nonblocking {SOCK_NONBLOCK} else {0};
            match cvt!(unsafe { socket(AF_UNIX, type_flags, 0) }) {
                Ok(fd) => return Ok(Socket(fd)),
                Err(ref e) if e.raw_os_error() == Some(EINVAL) => {/*try without*/}
                Err(e) => return Err(e),
            }
        }

        // portable but race-prone
        let fd = cvt!(unsafe { socket(AF_UNIX, socket_type, 0) })?;
        let socket = Socket(fd);
        set_cloexec(socket.0, true)?;
        socket.set_nosigpipe(true)?;
        if nonblocking {
            set_nonblocking(socket.0, true)?;
        }
        Ok(socket)
    }

    pub fn accept_from(fd: RawFd,  nonblocking: bool)
    -> Result<(Self, UnixSocketAddr), io::Error> {
        unsafe { UnixSocketAddr::new_from_ffi(|addr_ptr, len_ptr| {
            // Use accept4() to set close-on-exec atomically if possible.
            // (macOS and iOS doesn't have accept4(),
            //  and the Android emulator prohibits it: https://github.com/tokio-rs/mio/issues/1445)
            // ENOSYS is handled for compatibility with Linux < 2.6.28,
            // because Rust std still supports Linux 2.6.18.
            // (used by RHEL 5 which didn't reach EOL until November 2020).
            #[cfg(not(any(
                target_vendor="apple",
                target_os = "haiku",
                all(target_os="android", target_arch="x86")
            )))] {
                let flags = SOCK_CLOEXEC | if nonblocking {SOCK_NONBLOCK} else {0};
                match cvt_r!(accept4(fd, addr_ptr, len_ptr, flags)) {
                    Ok(fd) => return Ok(Socket(fd)),
                    Err(ref e) if e.raw_os_error() == Some(ENOSYS) => {/*try normal accept()*/},
                    Err(e) => return Err(e),
                }
            }

            // Portable but not as efficient:
            let fd = cvt_r!(accept(fd, addr_ptr, len_ptr))?;
            let socket = Socket(fd);
            set_cloexec(socket.0, true)?;
            socket.set_nosigpipe(true)?;
            if nonblocking {
                set_nonblocking(socket.0, true)?;
            }
            Ok(socket)
        }) }
    }

    pub fn start_listening(&self) -> Result<(), io::Error> {
        cvt!(unsafe { listen(self.0, LISTEN_BACKLOG) }).map(|_| () )
    }

    pub fn try_clone_from(fd: RawFd) -> Result<Self, io::Error> {
        // nonblocking-ness is shared, so doesn't need to potentially be set.
        // FIXME is SO_NOSIGPIPE shared?
        // If so setting it again doesn't hurt in most cases,
        // but might be unwanted if somebody has for some reason cleared it.

        // use fcntl(F_DUPFD_CLOEXEC) to set close-on-exec atomically
        // if possible, but fall through to dup()-and-ioctl(FIOCLEX)
        // for compatibility with Linux < 2.6.24
        match cvt!(unsafe { fcntl(fd, F_DUPFD_CLOEXEC, 0) }) {
            Ok(cloned) => {
                let socket = Socket(cloned);
                socket.set_nosigpipe(true)?;
                return Ok(socket);
            },
            Err(ref e) if e.raw_os_error() == Some(EINVAL) => {/*try dup() instead*/}
            Err(e) => return Err(e),
        }

        let cloned = cvt!(unsafe { dup(fd) })?;
        let socket = Socket(cloned);
        set_cloexec(socket.0, true)?;
        socket.set_nosigpipe(true)?;
        Ok(socket)
    }

    pub fn pair(socket_type: c_int,  nonblocking: bool) -> Result<(Self, Self), io::Error> {
        let mut fd_buf = [-1; 2];
        // Set close-on-exec atomically with SOCK_CLOEXEC if possible.
        // Falls through for compatibility with Linux < 2.6.27
        #[cfg(not(any(target_vendor="apple", target_os = "haiku")))] {
            let type_flags = socket_type | SOCK_CLOEXEC | if nonblocking {SOCK_NONBLOCK} else {0};
            match cvt!(unsafe { socketpair(AF_UNIX, type_flags, 0, fd_buf[..].as_mut_ptr()) }) {
                Ok(_) => return Ok((Socket(fd_buf[0]), Socket(fd_buf[1]))),
                Err(ref e) if e.raw_os_error() == Some(EINVAL) => {/*try without*/}
                Err(e) => return Err(e),
            }
        }

        cvt!(unsafe { socketpair(AF_UNIX, socket_type, 0, fd_buf[..].as_mut_ptr()) })?;
        let a = Socket(fd_buf[0]);
        let b = Socket(fd_buf[1]);
        set_cloexec(a.0, true)?;
        set_cloexec(b.0, true)?;
        a.set_nosigpipe(true)?;
        b.set_nosigpipe(true)?;
        if nonblocking {
            set_nonblocking(a.0, true)?;
            set_nonblocking(b.0, true)?;
        }
        Ok((a, b))
    }
}
