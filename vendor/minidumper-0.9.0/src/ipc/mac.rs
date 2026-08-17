//! Implements support for Unix domain sockets for Macos. We don't use std since
//! it `peek` is nightly only and it only implements the [`std::io::Read`] and
//! [`std::io::Write`] traits, which doesn't fit with the model we started with
//! with `uds`, which is that the sockets only do vectored reads/writes so
//! exclusive access is not desired

#![allow(clippy::mem_forget, unsafe_code)]

use std::{
    io,
    os::{
        fd::AsFd,
        unix::{
            ffi::OsStrExt,
            io::{AsRawFd, IntoRawFd, RawFd},
            prelude::BorrowedFd,
        },
    },
};

#[inline]
fn sun_path_offset(addr: &libc::sockaddr_un) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = addr as *const _ as usize;
    let path = &addr.sun_path as *const _ as usize;
    path - base
}

pub struct UnixSocketAddr {
    pub(super) addr: libc::sockaddr_un,
    pub(super) len: libc::socklen_t,
}

impl UnixSocketAddr {
    pub(super) fn new(path: &std::path::Path) -> io::Result<Self> {
        // SAFETY: All zeros is a valid representation for `sockaddr_un`.
        let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
        addr.sun_family = libc::AF_UNIX as _;

        let bytes = path.as_os_str().as_bytes();

        if bytes.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "paths must not contain interior null bytes",
            ));
        }

        if bytes.len() >= addr.sun_path.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path must be shorter than SUN_LEN",
            ));
        }

        // SAFETY: `bytes` and `addr.sun_path` are not overlapping and
        // both point to valid memory.
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                addr.sun_path.as_mut_ptr().cast(),
                bytes.len(),
            );
        }

        let mut len = sun_path_offset(&addr) + bytes.len();
        match bytes.first() {
            Some(&0) | None => {}
            Some(_) => len += 1, // + null terminator
        }

        Ok(Self {
            addr,
            len: len as libc::socklen_t,
        })
    }

    pub(super) fn from_parts(
        addr: libc::sockaddr_un,
        mut len: libc::socklen_t,
    ) -> io::Result<Self> {
        if len == 0 {
            // When there is a datagram from unnamed unix socket
            // linux returns zero bytes of address
            len = sun_path_offset(&addr) as libc::socklen_t; // i.e., zero-length address
        } else if addr.sun_family != libc::AF_UNIX as libc::sa_family_t {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file descriptor did not correspond to a Unix socket",
            ));
        }

        Ok(Self { addr, len })
    }
}

struct Uds(RawFd);

impl Uds {
    pub fn new() -> io::Result<Self> {
        // SAFETY: syscalls
        unsafe {
            let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);

            if fd == -1 {
                return Err(io::Error::last_os_error());
            }

            let s = Self(fd);

            if libc::ioctl(s.0, libc::FIOCLEX) != 0 {
                return Err(io::Error::last_os_error());
            }

            if libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_NOSIGPIPE,
                (&1 as *const i32).cast(),
                std::mem::size_of::<u32>() as _,
            ) != 0
            {
                return Err(io::Error::last_os_error());
            }

            Ok(s)
        }
    }

    fn accept(&self, storage: *mut libc::sockaddr, len: &mut libc::socklen_t) -> io::Result<Self> {
        // SAFETY: syscalls
        unsafe {
            let fd = libc::accept(self.0, storage, len);
            if fd == -1 {
                return Err(io::Error::last_os_error());
            }

            let s = Self(fd);

            if libc::ioctl(s.0, libc::FIOCLEX) != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(s)
        }
    }

    fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let mut nonblocking = nonblocking as i32;
        // SAFETY: syscall
        if unsafe { libc::ioctl(self.0, libc::FIONBIO, &mut nonblocking) } != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: i32) -> io::Result<usize> {
        // SAFETY: syscall
        let read = unsafe { libc::recv(self.0, buf.as_mut_ptr().cast(), buf.len(), flags) };

        if read == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(read as usize)
        }
    }

    fn recv_vectored(&self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        // SAFETY: syscall
        let read = unsafe {
            libc::readv(
                self.0,
                bufs.as_ptr().cast(),
                std::cmp::min(bufs.len(), libc::IOV_MAX as usize) as _,
            )
        };

        if read == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(read as usize)
        }
    }

    fn send_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        // SAFETY: syscall
        let sent = unsafe {
            libc::writev(
                self.0,
                bufs.as_ptr().cast(),
                std::cmp::min(bufs.len(), libc::IOV_MAX as usize) as _,
            )
        };

        if sent == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(sent as usize)
        }
    }
}

impl AsRawFd for Uds {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl AsFd for Uds {
    fn as_fd(&self) -> BorrowedFd<'_> {
        #[allow(unsafe_code)]
        unsafe {
            BorrowedFd::borrow_raw(self.as_raw_fd())
        }
    }
}

impl Drop for Uds {
    fn drop(&mut self) {
        // SAFETY: syscall
        let _ = unsafe { libc::close(self.0) };
    }
}

/// A Unix domain socket server
pub(crate) struct UnixListener(Uds);

impl UnixListener {
    pub(crate) fn bind(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let listener = std::os::unix::net::UnixListener::bind(path)?;
        Ok(Self(Uds(listener.into_raw_fd())))
    }

    pub(crate) fn accept_unix_addr(&self) -> io::Result<(UnixStream, UnixSocketAddr)> {
        let mut sock_addr = std::mem::MaybeUninit::<libc::sockaddr_un>::uninit();
        let mut len = std::mem::size_of::<libc::sockaddr_un>() as _;

        let sock = self.0.accept(sock_addr.as_mut_ptr().cast(), &mut len)?;
        // SAFETY: should have been initialized if accept succeeded
        let addr = UnixSocketAddr::from_parts(unsafe { sock_addr.assume_init() }, len)?;

        Ok((UnixStream(sock), addr))
    }

    #[inline]
    pub(crate) fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.0.set_nonblocking(nonblocking)
    }
}

impl AsRawFd for UnixListener {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl AsFd for UnixListener {
    fn as_fd(&self) -> BorrowedFd<'_> {
        #[allow(unsafe_code)]
        unsafe {
            BorrowedFd::borrow_raw(self.as_raw_fd())
        }
    }
}

/// A Unix doman socket stream
pub(crate) struct UnixStream(Uds);

impl UnixStream {
    pub(crate) fn connect(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        // SAFETY: syscalls
        unsafe {
            let inner = Uds::new()?;
            let addr = UnixSocketAddr::new(path.as_ref())?;

            if libc::connect(
                inner.0,
                (&addr.addr as *const libc::sockaddr_un).cast(),
                addr.len,
            ) != 0
            {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(Self(inner))
            }
        }
    }

    #[inline]
    pub(crate) fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv_with_flags(buf, libc::MSG_PEEK)
    }

    #[inline]
    pub(crate) fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_vectored(&mut [io::IoSliceMut::new(buf)])
    }

    #[inline]
    pub(crate) fn recv_vectored(&self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.recv_vectored(bufs)
    }

    #[inline]
    pub(crate) fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.send_vectored(&[io::IoSlice::new(buf)])
    }

    #[inline]
    pub(crate) fn send_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.0.send_vectored(bufs)
    }
}

impl AsRawFd for UnixStream {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl AsFd for UnixStream {
    fn as_fd(&self) -> BorrowedFd<'_> {
        #[allow(unsafe_code)]
        unsafe {
            BorrowedFd::borrow_raw(self.as_raw_fd())
        }
    }
}
