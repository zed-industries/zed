pub(crate) mod datagram {
    use crate::net::SocketAddr;
    use std::io;
    use std::os::unix::net;
    use std::path::Path;

    pub(crate) fn bind(_: &Path) -> io::Result<net::UnixDatagram> {
        os_required!()
    }

    pub(crate) fn unbound() -> io::Result<net::UnixDatagram> {
        os_required!()
    }

    pub(crate) fn pair() -> io::Result<(net::UnixDatagram, net::UnixDatagram)> {
        os_required!()
    }

    pub(crate) fn local_addr(_: &net::UnixDatagram) -> io::Result<SocketAddr> {
        os_required!()
    }

    pub(crate) fn peer_addr(_: &net::UnixDatagram) -> io::Result<SocketAddr> {
        os_required!()
    }

    pub(crate) fn recv_from(
        _: &net::UnixDatagram,
        _: &mut [u8],
    ) -> io::Result<(usize, SocketAddr)> {
        os_required!()
    }
}

pub(crate) mod listener {
    use crate::net::{SocketAddr, UnixStream};
    use std::io;
    use std::os::unix::net;
    use std::path::Path;

    pub(crate) fn bind(_: &Path) -> io::Result<net::UnixListener> {
        os_required!()
    }

    pub(crate) fn bind_addr(_: &SocketAddr) -> io::Result<net::UnixListener> {
        os_required!()
    }

    pub(crate) fn accept(_: &net::UnixListener) -> io::Result<(UnixStream, SocketAddr)> {
        os_required!()
    }

    pub(crate) fn local_addr(_: &net::UnixListener) -> io::Result<SocketAddr> {
        os_required!()
    }
}

pub(crate) mod stream {
    use crate::net::SocketAddr;
    use std::io;
    use std::os::unix::net;
    use std::path::Path;

    pub(crate) fn connect(_: &Path) -> io::Result<net::UnixStream> {
        os_required!()
    }

    pub(crate) fn connect_addr(_: &SocketAddr) -> io::Result<net::UnixStream> {
        os_required!()
    }

    pub(crate) fn pair() -> io::Result<(net::UnixStream, net::UnixStream)> {
        os_required!()
    }

    pub(crate) fn local_addr(_: &net::UnixStream) -> io::Result<SocketAddr> {
        os_required!()
    }

    pub(crate) fn peer_addr(_: &net::UnixStream) -> io::Result<SocketAddr> {
        os_required!()
    }
}
