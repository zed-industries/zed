use std::fs::File;
use std::io::{
    self, BufRead, BufReader, Chain, Cursor, Empty, Read, Repeat, Seek, SeekFrom, StdinLock, Take,
};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

/// A trait providing the `peek` function for reading without consuming.
///
/// Many common `Read` implementations have `Peek` implementations, however
/// [`Stdin`], [`ChildStderr`], [`ChildStdout`], [`PipeReader`], and
/// [`CharDevice`] do not, since they are unbuffered and pipes and character
/// devices don't support any form of peeking.
///
/// [`Stdin`]: std::io::Stdin
/// [`ChildStdout`]: std::process::ChildStdout
/// [`ChildStderr`]: std::process::ChildStderr
/// [`PipeReader`]: https://docs.rs/os_pipe/latest/os_pipe/struct.PipeReader.html
/// [`CharDevice`]: https://docs.rs/char-device/latest/char_device/struct.CharDevice.html
pub trait Peek {
    /// Reads data from a stream without consuming it; subsequent reads will
    /// re-read the data. May return fewer bytes than requested; `Ok(0)`
    /// indicates that seeking is not possible (but reading may still be).
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

impl Peek for File {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = self.seek(SeekFrom::Current(0))?;
        crate::fs::FileIoExt::read_at(self, buf, pos)
    }
}

#[cfg(feature = "cap_std_impls")]
impl Peek for cap_std::fs::File {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = self.seek(SeekFrom::Current(0))?;
        crate::fs::FileIoExt::read_at(self, buf, pos)
    }
}

#[cfg(feature = "cap_std_impls_fs_utf8")]
impl Peek for cap_std::fs_utf8::File {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = self.seek(SeekFrom::Current(0))?;
        crate::fs::FileIoExt::read_at(self, buf, pos)
    }
}

impl Peek for TcpStream {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        TcpStream::peek(self, buf)
    }
}

#[cfg(unix)]
impl Peek for UnixStream {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        #[cfg(unix_socket_peek)]
        {
            UnixStream::peek(self, buf)
        }

        #[cfg(not(unix_socket_peek))]
        {
            // Return the conservatively correct value.
            let _ = buf;
            Ok(0)
        }
    }
}

impl Peek for Repeat {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Repeat just reads the same byte repeatedly, so we can just read
        // from it.
        self.read(buf)
    }
}

#[cfg(feature = "socketpair")]
impl Peek for socketpair::SocketpairStream {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        socketpair::SocketpairStream::peek(self, buf)
    }
}

impl Peek for Empty {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<'a> Peek for &'a [u8] {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<'a> Peek for StdinLock<'a> {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<B: BufRead + ?Sized> Peek for Box<B> {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<'a, B: BufRead + ?Sized> Peek for &'a mut B {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<R: Read> Peek for BufReader<R> {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<T> Peek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<T: BufRead> Peek for Take<T> {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

impl<T: BufRead, U: BufRead> Peek for Chain<T, U> {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        peek_from_bufread(self, buf)
    }
}

/// Implement `peek` for types that implement `BufRead`.
#[inline]
pub fn peek_from_bufread<BR: BufRead>(buf_read: &mut BR, buf: &mut [u8]) -> io::Result<usize> {
    // Call `fill_buf` to read the bytes, but don't call `consume`.
    Read::read(&mut buf_read.fill_buf()?, buf)
}

#[cfg(feature = "socket2")]
impl Peek for socket2::Socket {
    #[inline]
    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use io_lifetimes::AsSocketlike;
        self.as_socketlike_view::<std::net::TcpStream>().peek(buf)
    }
}
