use super::socket_addr;
use crate::net::{SocketAddr, UnixStream};
use crate::sys::unix::net::new_socket;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net;
use std::path::Path;
use std::{io, mem};

pub(crate) fn bind(path: &Path) -> io::Result<net::UnixListener> {
    let socket_address = {
        let (sockaddr, socklen) = socket_addr(path.as_os_str().as_bytes())?;
        SocketAddr::from_parts(sockaddr, socklen)
    };

    bind_addr(&socket_address)
}

pub(crate) fn bind_addr(address: &SocketAddr) -> io::Result<net::UnixListener> {
    let fd = new_socket(libc::AF_UNIX, libc::SOCK_STREAM)?;
    let socket = unsafe { net::UnixListener::from_raw_fd(fd) };
    let sockaddr = address.raw_sockaddr() as *const libc::sockaddr_un as *const libc::sockaddr;

    syscall!(bind(fd, sockaddr, *address.raw_socklen()))?;
    syscall!(listen(fd, 1024))?;

    Ok(socket)
}

pub(crate) fn accept(listener: &net::UnixListener) -> io::Result<(UnixStream, SocketAddr)> {
    let sockaddr = mem::MaybeUninit::<libc::sockaddr_un>::zeroed();

    // This is safe to assume because a `libc::sockaddr_un` filled with `0`
    // bytes is properly initialized.
    //
    // `0` is a valid value for `sockaddr_un::sun_family`; it is
    // `libc::AF_UNSPEC`.
    //
    // `[0; 108]` is a valid value for `sockaddr_un::sun_path`; it begins an
    // abstract path.
    let mut sockaddr = unsafe { sockaddr.assume_init() };

    sockaddr.sun_family = libc::AF_UNIX as libc::sa_family_t;
    let mut socklen = mem::size_of_val(&sockaddr) as libc::socklen_t;

    #[cfg(not(any(
        target_os = "aix",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "redox",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "espidf",
        target_os = "vita",
        // Android x86's seccomp profile forbids calls to `accept4(2)`
        // See https://github.com/tokio-rs/mio/issues/1445 for details
        all(target_arch = "x86", target_os = "android"),
    )))]
    let socket = {
        let flags = libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC;
        syscall!(accept4(
            listener.as_raw_fd(),
            &mut sockaddr as *mut libc::sockaddr_un as *mut libc::sockaddr,
            &mut socklen,
            flags
        ))
        .map(|socket| unsafe { net::UnixStream::from_raw_fd(socket) })
    };

    #[cfg(any(
        target_os = "aix",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "redox",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "espidf",
        target_os = "vita",
        all(target_arch = "x86", target_os = "android")
    ))]
    let socket = syscall!(accept(
        listener.as_raw_fd(),
        &mut sockaddr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        &mut socklen,
    ))
    .and_then(|socket| {
        // Ensure the socket is closed if either of the `fcntl` calls
        // error below.
        let s = unsafe { net::UnixStream::from_raw_fd(socket) };
        #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
        syscall!(fcntl(socket, libc::F_SETFD, libc::FD_CLOEXEC))?;

        // See https://github.com/tokio-rs/mio/issues/1450
        #[cfg(any(
            all(target_arch = "x86", target_os = "android"),
            target_os = "espidf",
            target_os = "vita",
        ))]
        syscall!(fcntl(socket, libc::F_SETFL, libc::O_NONBLOCK))?;

        Ok(s)
    });

    socket
        .map(UnixStream::from_std)
        .map(|stream| (stream, SocketAddr::from_parts(sockaddr, socklen)))
}

pub(crate) fn local_addr(listener: &net::UnixListener) -> io::Result<SocketAddr> {
    super::local_addr(listener.as_raw_fd())
}
