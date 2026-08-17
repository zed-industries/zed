use std::io;
#[cfg(not(windows))]
use {io_lifetimes::AsFilelike, rustix::io::is_read_write};
#[cfg(windows)]
use {
    std::{
        os::windows::io::{AsRawSocket, RawSocket},
        ptr,
    },
    windows_sys::Win32::Networking::WinSock::{
        recv, send, MSG_PEEK, SOCKET, SOCKET_ERROR, WSAEFAULT, WSAESHUTDOWN, WSAEWOULDBLOCK,
    },
};

/// A trait for the `is_read_write` function.
pub trait IsReadWrite {
    /// Test whether the handle is readable and/or writable.
    fn is_read_write(&self) -> io::Result<(bool, bool)>;
}

#[cfg(not(windows))]
impl<T: AsFilelike> IsReadWrite for T {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        Ok(is_read_write(self)?)
    }
}

#[cfg(windows)]
impl IsReadWrite for std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        file_is_read_write(self)
    }
}

#[cfg(all(windows, feature = "cap_std_impls"))]
impl IsReadWrite for cap_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsFilelike;
        file_is_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(windows, feature = "cap_std_impls_fs_utf8"))]
impl IsReadWrite for cap_std::fs_utf8::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsFilelike;
        file_is_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(windows, feature = "async-std"))]
impl IsReadWrite for async_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsFilelike;
        file_is_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(windows, feature = "async-std"))]
impl IsReadWrite for cap_async_std::fs::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsFilelike;
        file_is_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(all(
    windows,
    feature = "async-std",
    feature = "cap_async_std_impls_fs_utf8"
))]
impl IsReadWrite for cap_async_std::fs_utf8::File {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsFilelike;
        file_is_read_write(&self.as_filelike_view::<std::fs::File>())
    }
}

#[cfg(windows)]
impl IsReadWrite for std::net::TcpStream {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        raw_socket_is_read_write(self.as_raw_socket())
    }
}

#[cfg(all(windows, feature = "cap_std_impls"))]
impl IsReadWrite for cap_std::net::TcpStream {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        raw_socket_is_read_write(self.as_raw_socket())
    }
}

#[cfg(all(windows, feature = "cap_async_std_impls"))]
impl IsReadWrite for async_std::net::TcpStream {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        raw_socket_is_read_write(self.as_raw_socket())
    }
}

#[cfg(all(windows, feature = "cap_async_std_impls"))]
impl IsReadWrite for cap_async_std::net::TcpStream {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        raw_socket_is_read_write(self.as_raw_socket())
    }
}

#[cfg(windows)]
#[inline]
fn file_is_read_write(file: &std::fs::File) -> std::io::Result<(bool, bool)> {
    cap_fs_ext::IsFileReadWrite::is_file_read_write(file)
}

#[cfg(windows)]
fn raw_socket_is_read_write(raw_socket: RawSocket) -> io::Result<(bool, bool)> {
    let (mut read, mut write) = (true, true);

    // Detect write shutdown. A zero-length `send` doesn't block but does
    // provide a helpful error message.
    let socket = raw_socket as SOCKET;
    let write_result = unsafe { send(socket, ptr::null_mut(), 0, 0) };
    if write_result == SOCKET_ERROR {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(WSAESHUTDOWN) => write = false,
            Some(WSAEWOULDBLOCK) => (),
            _ => return Err(err),
        }
    }

    // Detect read shutdown. A normal zero-length `recv` does block, so
    // use deliberately invalid pointer, as we get different error codes in
    // the case of a shut-down stream.
    let read_result = unsafe { recv(socket, usize::MAX as *mut _, 1, MSG_PEEK) };
    if read_result == SOCKET_ERROR {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(WSAEFAULT) => (),
            Some(WSAESHUTDOWN) => read = false,
            _ => return Err(err),
        }
    }

    Ok((read, write))
}

#[cfg(all(windows, feature = "socket2"))]
impl IsReadWrite for socket2::Socket {
    #[inline]
    fn is_read_write(&self) -> io::Result<(bool, bool)> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<std::net::TcpStream>()
            .is_read_write()
    }
}
