use crate::io::IsReadWrite;
use io_lifetimes::raw::{AsRawFilelike, FromRawFilelike};
#[cfg(not(any(windows, target_os = "redox")))]
use rustix::io::ioctl_fionread;
use std::io::{self, Seek, SeekFrom, Stdin, StdinLock};
#[cfg(not(target_os = "redox"))]
use std::net;
use std::process::{ChildStderr, ChildStdout};
#[cfg(windows)]
use {
    std::{mem::MaybeUninit, os::windows::io::AsRawSocket},
    windows_sys::Win32::Networking::WinSock::{ioctlsocket, FIONREAD, SOCKET},
};

/// Extension for readable streams that can indicate the number of bytes
/// ready to be read immediately.
pub trait ReadReady {
    /// Return the number of bytes which are ready to be read immediately.
    ///
    /// The returned number may be greater than the number of bytes actually
    /// readable if the end of the stream is known to be reachable without
    /// blocking.
    fn num_ready_bytes(&self) -> io::Result<u64>;
}

/// Implement `ReadReady` for `Stdin`.
#[cfg(not(any(windows, target_os = "redox")))]
impl ReadReady for Stdin {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

/// Implement `ReadReady` for `Stdin`.
#[cfg(any(windows, target_os = "redox"))]
impl ReadReady for Stdin {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

/// Implement `ReadReady` for `StdinLock`.
#[cfg(not(any(windows, target_os = "redox")))]
impl<'a> ReadReady for StdinLock<'a> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

/// Implement `ReadReady` for `StdinLock`.
#[cfg(any(windows, target_os = "redox"))]
impl<'a> ReadReady for StdinLock<'a> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

impl crate::io::ReadReady for std::fs::File {
    #[inline]
    fn num_ready_bytes(&self) -> std::io::Result<u64> {
        let (read, _write) = self.is_read_write()?;
        if read {
            // If it's a file, we can query how many bytes are left.
            let metadata = self.metadata()?;
            if metadata.is_file() {
                // When `File::tell` is stable use that, but for now, create a
                // temporary `File` so that we can get a `&mut File` and call
                // `seek` on it.
                let mut tmp = unsafe { std::fs::File::from_raw_filelike(self.as_raw_filelike()) };
                let current = tmp.seek(SeekFrom::Current(0));
                std::mem::forget(tmp);
                return Ok(metadata.len() - current?);
            }

            // Otherwise, try our luck with `FIONREAD`.
            #[cfg(unix)]
            if let Ok(n) = ioctl_fionread(self) {
                return Ok(n);
            }

            // It's something without a specific length that we can't query,
            // so return the conservatively correct answer.
            return Ok(0);
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "stream is not readable",
        ))
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(not(any(windows, target_os = "redox")))]
impl ReadReady for net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(not(any(windows, target_os = "redox")))]
#[cfg(feature = "cap_std_impls")]
impl ReadReady for cap_std::net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<net::TcpStream>()
            .num_ready_bytes()
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(target_os = "redox")]
impl ReadReady for net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(target_os = "redox")]
impl ReadReady for cap_std::net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<net::TcpStream>()
            .num_ready_bytes()
    }
}

/// Implement `ReadReady` for `std::os::unix::net::UnixStream`.
#[cfg(unix)]
impl ReadReady for std::os::unix::net::UnixStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

/// Implement `ReadReady` for `&[u8]`.
impl ReadReady for &[u8] {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(self.len() as u64)
    }
}

/// Implement `ReadReady` for `std::io::Empty`.
impl ReadReady for std::io::Empty {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Empty has zero bytes, but say 1 so that consumers know that they
        // can immediately attempt to read 1 byte and it won't block.
        Ok(1)
    }
}

/// Implement `ReadReady` for `std::io::Repeat`.
impl ReadReady for std::io::Repeat {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Unlimited bytes available immediately.
        Ok(u64::MAX)
    }
}

/// Implement `ReadReady` for `std::collections::VecDeque<T>`.
impl<T> ReadReady for std::collections::VecDeque<T> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(self.len() as u64)
    }
}

/// Implement `ReadReady` for `Box`.
impl<R: ReadReady> ReadReady for Box<R> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        self.as_ref().num_ready_bytes()
    }
}

/// Implement `ReadReady` for `std::io::BufReader<R>`.
impl<R: std::io::Read + ReadReady> ReadReady for std::io::BufReader<R> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        let buffer = self.buffer();
        match self.get_ref().num_ready_bytes() {
            Ok(n) => Ok(buffer.len() as u64 + n),
            Err(_) if !buffer.is_empty() => Ok(buffer.len() as u64),
            Err(e) => Err(e),
        }
    }
}

/// Implement `ReadReady` for `std::io::Cursor<T>`.
impl<T: AsRef<[u8]>> ReadReady for std::io::Cursor<T> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(self.get_ref().as_ref().len() as u64 - self.position())
    }
}

/// Implement `ReadReady` for `std::io::Take<T>`.
impl<T: ReadReady> ReadReady for std::io::Take<T> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(std::cmp::min(
            self.limit(),
            self.get_ref().num_ready_bytes()?,
        ))
    }
}

/// Implement `ReadReady` for `std::io::Chain<T, U>`.
impl<T: ReadReady, U> ReadReady for std::io::Chain<T, U> {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Just return the ready bytes for the first source, since we don't
        // know if it'll block before we exhaust it and move to the second.
        self.get_ref().0.num_ready_bytes()
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(windows)]
impl ReadReady for net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        let mut arg = MaybeUninit::<std::os::raw::c_ulong>::uninit();
        if unsafe { ioctlsocket(self.as_raw_socket() as SOCKET, FIONREAD, arg.as_mut_ptr()) } == 0 {
            Ok(unsafe { arg.assume_init() }.into())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(windows)]
#[cfg(feature = "cap_std_impls")]
impl ReadReady for cap_std::net::TcpStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<net::TcpStream>()
            .num_ready_bytes()
    }
}

/// Implement `ReadReady` for `std::net::TcpStream`.
#[cfg(feature = "socket2")]
impl ReadReady for socket2::Socket {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<std::net::TcpStream>()
            .num_ready_bytes()
    }
}

#[cfg(all(not(windows), feature = "os_pipe"))]
impl ReadReady for os_pipe::PipeReader {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

#[cfg(all(windows, feature = "os_pipe"))]
impl ReadReady for os_pipe::PipeReader {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

#[cfg(not(windows))]
impl ReadReady for ChildStdout {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

#[cfg(windows)]
impl ReadReady for ChildStdout {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

#[cfg(not(windows))]
impl ReadReady for ChildStderr {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(ioctl_fionread(self)?)
    }
}

#[cfg(windows)]
impl ReadReady for ChildStderr {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        // Return the conservatively correct result.
        Ok(0)
    }
}

#[cfg(feature = "socketpair")]
impl ReadReady for socketpair::SocketpairStream {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        socketpair::SocketpairStream::num_ready_bytes(self)
    }
}

#[cfg(feature = "ssh2")]
impl ReadReady for ssh2::Channel {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        Ok(u64::from(self.read_window().available))
    }
}

#[cfg(feature = "char-device")]
impl ReadReady for char_device::CharDevice {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        char_device::CharDevice::num_ready_bytes(self)
    }
}

/// Implement `ReadReady` for `cap_std::fs::File`.
#[cfg(feature = "cap_std_impls")]
impl ReadReady for cap_std::fs::File {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsFilelike;
        self.as_filelike_view::<std::fs::File>().num_ready_bytes()
    }
}

/// Implement `ReadReady` for `cap_std::fs_utf8::File`.
#[cfg(feature = "cap_std_impls_fs_utf8")]
impl ReadReady for cap_std::fs_utf8::File {
    #[inline]
    fn num_ready_bytes(&self) -> io::Result<u64> {
        use io_lifetimes::AsFilelike;
        self.as_filelike_view::<std::fs::File>().num_ready_bytes()
    }
}
