use super::{socket_addr, SocketAddr};
use crate::sys::unix::net::new_socket;

use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net;
use std::path::Path;

pub(crate) fn connect(path: &Path) -> io::Result<net::UnixStream> {
    let socket_address = {
        let (sockaddr, socklen) = socket_addr(path.as_os_str().as_bytes())?;
        SocketAddr::from_parts(sockaddr, socklen)
    };

    connect_addr(&socket_address)
}

pub(crate) fn connect_addr(address: &SocketAddr) -> io::Result<net::UnixStream> {
    let fd = new_socket(libc::AF_UNIX, libc::SOCK_STREAM)?;
    let socket = unsafe { net::UnixStream::from_raw_fd(fd) };
    let sockaddr = address.raw_sockaddr() as *const libc::sockaddr_un as *const libc::sockaddr;

    match syscall!(connect(fd, sockaddr, *address.raw_socklen())) {
        Ok(_) => {}
        Err(ref err) if err.raw_os_error() == Some(libc::EINPROGRESS) => {}
        Err(e) => return Err(e),
    }

    Ok(socket)
}

pub(crate) fn pair() -> io::Result<(net::UnixStream, net::UnixStream)> {
    super::pair(libc::SOCK_STREAM)
}

pub(crate) fn local_addr(socket: &net::UnixStream) -> io::Result<SocketAddr> {
    super::local_addr(socket.as_raw_fd())
}

pub(crate) fn peer_addr(socket: &net::UnixStream) -> io::Result<SocketAddr> {
    super::peer_addr(socket.as_raw_fd())
}
