use io_lifetimes::{AsFilelike, AsSocketlike};
use std::fmt::Arguments;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::slice;

/// Extension trait for I/O handles that are exterior-mutable readable
/// and writeable.
pub trait IoExt {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// This is similar to [`std::io::Read::read`], except it takes `self` by
    /// immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Read::read`]: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
    fn read(&self, buf: &mut [u8]) -> io::Result<usize>;

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This is similar to [`std::io::Read::read_exact`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read_exact
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()>;

    /// Like `read`, except that it reads into a slice of buffers.
    ///
    /// This is similar to [`std::io::Read::read_vectored`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Read::read_vectored`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_vectored
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize>;

    /// Is to `read_vectored` what `read_exact` is to `read`.
    fn read_exact_vectored(&self, mut bufs: &mut [IoSliceMut]) -> io::Result<()> {
        bufs = skip_leading_empties(bufs);
        while !bufs.is_empty() {
            match self.read_vectored(bufs) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "failed to fill whole buffer",
                    ))
                }
                Ok(nread) => bufs = advance_mut(bufs, nread),
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => (),
                Err(e) => return Err(e),
            }
            bufs = skip_leading_empties(bufs);
        }
        Ok(())
    }

    /// Read all bytes until EOF in this source, placing them into `buf`.
    ///
    /// This is similar to [`std::io::Read::read_to_end`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Read::read_to_end`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_end
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize>;

    /// Read all bytes until EOF in this source, appending them to `buf`.
    ///
    /// This is similar to [`std::io::Read::read_to_string`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Read::read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize>;

    /// Read bytes from the current position without advancing the current
    /// position.
    ///
    /// This is similar to [`crate::io::Peek::peek`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize>;

    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// This is similar to [`std::io::Write::write`], except it takes `self` by
    /// immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Write::write`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write
    fn write(&self, buf: &[u8]) -> io::Result<usize>;

    /// Attempts to write an entire buffer into this writer.
    ///
    /// This is similar to [`std::io::Write::write_all`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write_all
    fn write_all(&self, buf: &[u8]) -> io::Result<()>;

    /// Like `write`, except that it writes from a slice of buffers.
    ///
    /// This is similar to [`std::io::Write::write_vectored`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Write::write_vectored`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_vectored
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize>;

    /// Is to `write_vectored` what `write_all` is to `write`.
    fn write_all_vectored(&self, mut bufs: &mut [IoSlice]) -> io::Result<()> {
        // TODO: Use [rust-lang/rust#70436] once it stabilizes.
        // [rust-lang/rust#70436]: https://github.com/rust-lang/rust/issues/70436
        while !bufs.is_empty() {
            match self.write_vectored(bufs) {
                Ok(nwritten) => bufs = advance(bufs, nwritten),
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Writes a formatted string into this writer, returning any error
    /// encountered.
    ///
    /// This is similar to [`std::io::Write::write_fmt`], except it takes
    /// `self` by immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Write::write_fmt`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write_fmt
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    ///
    /// This is similar to [`std::io::Write::flush`], except it takes `self` by
    /// immutable reference since the entire side effect is I/O.
    ///
    /// [`std::io::Write::flush`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
    fn flush(&self) -> io::Result<()>;
}

/// Skip any leading elements in `bufs` which are empty buffers.
fn skip_leading_empties<'a, 'b>(mut bufs: &'b mut [IoSliceMut<'a>]) -> &'b mut [IoSliceMut<'a>] {
    while !bufs.is_empty() {
        if !bufs[0].is_empty() {
            break;
        }
        bufs = &mut bufs[1..];
    }
    bufs
}

/// This will be obviated by [rust-lang/rust#62726].
///
/// [rust-lang/rust#62726]: https://github.com/rust-lang/rust/issues/62726.
fn advance<'a, 'b>(bufs: &'b mut [IoSlice<'a>], n: usize) -> &'b mut [IoSlice<'a>] {
    // Number of buffers to remove.
    let mut remove = 0;
    // Total length of all the to be removed buffers.
    let mut accumulated_len = 0;
    for buf in bufs.iter() {
        if accumulated_len + buf.len() > n {
            break;
        } else {
            accumulated_len += buf.len();
            remove += 1;
        }
    }

    #[allow(clippy::indexing_slicing)]
    let bufs = &mut bufs[remove..];
    if let Some(first) = bufs.first_mut() {
        let advance_by = n - accumulated_len;
        let mut ptr = first.as_ptr();
        let mut len = first.len();
        unsafe {
            ptr = ptr.add(advance_by);
            len -= advance_by;
            *first = IoSlice::<'a>::new(slice::from_raw_parts::<'a>(ptr, len));
        }
    }
    bufs
}

/// This will be obviated by [rust-lang/rust#62726].
///
/// [rust-lang/rust#62726]: https://github.com/rust-lang/rust/issues/62726.
fn advance_mut<'a, 'b>(bufs: &'b mut [IoSliceMut<'a>], n: usize) -> &'b mut [IoSliceMut<'a>] {
    // Number of buffers to remove.
    let mut remove = 0;
    // Total length of all the to be removed buffers.
    let mut accumulated_len = 0;
    for buf in bufs.iter() {
        if accumulated_len + buf.len() > n {
            break;
        } else {
            accumulated_len += buf.len();
            remove += 1;
        }
    }

    #[allow(clippy::indexing_slicing)]
    let bufs = &mut bufs[remove..];
    if let Some(first) = bufs.first_mut() {
        let advance_by = n - accumulated_len;
        let mut ptr = first.as_mut_ptr();
        let mut len = first.len();
        unsafe {
            ptr = ptr.add(advance_by);
            len -= advance_by;
            *first = IoSliceMut::<'a>::new(slice::from_raw_parts_mut::<'a>(ptr, len));
        }
    }
    bufs
}

/// Implement `IoExt` for any type which implements `AsRawFd`.
#[cfg(not(windows))]
impl<T: AsFilelike + AsSocketlike> IoExt for T {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        Read::read(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        Read::read_exact(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        Read::read_vectored(&mut &*self.as_filelike_view::<std::fs::File>(), bufs)
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        Read::read_to_end(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        Read::read_to_string(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        match self.as_socketlike_view::<std::net::TcpStream>().peek(buf) {
            Err(err) if err.raw_os_error() == Some(rustix::io::Errno::NOTSOCK.raw_os_error()) => {
                match self.as_filelike_view::<std::fs::File>().peek(buf) {
                    Err(err)
                        if err.raw_os_error() == Some(rustix::io::Errno::SPIPE.raw_os_error()) =>
                    {
                        Ok(0)
                    }
                    otherwise => otherwise,
                }
            }
            otherwise => otherwise,
        }
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        Write::write_all(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        Write::write_vectored(&mut &*self.as_filelike_view::<std::fs::File>(), bufs)
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        Write::flush(&mut &*self.as_filelike_view::<std::fs::File>())
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        Write::write_fmt(&mut &*self.as_filelike_view::<std::fs::File>(), fmt)
    }
}

#[cfg(windows)]
impl IoExt for std::fs::File {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        Read::read(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        Read::read_exact(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        Read::read_vectored(&mut &*self.as_filelike_view::<std::fs::File>(), bufs)
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        Read::read_to_end(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        Read::read_to_string(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        use std::os::windows::io::AsRawHandle;

        let mut bytes_read = std::mem::MaybeUninit::<u32>::uninit();
        let len = std::cmp::min(buf.len(), u32::MAX as usize) as u32;
        let res = unsafe {
            windows_sys::Win32::System::Pipes::PeekNamedPipe(
                self.as_filelike().as_raw_handle() as _,
                buf.as_mut_ptr() as *mut std::ffi::c_void,
                len,
                bytes_read.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if res == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { bytes_read.assume_init() } as usize)
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        Write::write_all(&mut &*self.as_filelike_view::<std::fs::File>(), buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        Write::write_vectored(&mut &*self.as_filelike_view::<std::fs::File>(), bufs)
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        Write::flush(&mut &*self.as_filelike_view::<std::fs::File>())
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        Write::write_fmt(&mut &*self.as_filelike_view::<std::fs::File>(), fmt)
    }
}

#[cfg(windows)]
#[cfg(feature = "cap_std_impls")]
impl IoExt for cap_std::fs::File {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read(buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().read_exact(buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_to_string(buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().peek(buf)
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().write(buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().write_all(buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>()
            .write_vectored(bufs)
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().flush()
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().write_fmt(fmt)
    }
}

#[cfg(windows)]
#[cfg(feature = "cap_std_impls_fs_utf8")]
impl IoExt for cap_std::fs_utf8::File {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read(buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().read_exact(buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().read_to_string(buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().peek(buf)
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>().write(buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().write_all(buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.as_filelike_view::<std::fs::File>()
            .write_vectored(bufs)
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().flush()
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        self.as_filelike_view::<std::fs::File>().write_fmt(fmt)
    }
}

#[cfg(windows)]
impl IoExt for std::net::TcpStream {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        Read::read(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        Read::read_exact(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        Read::read_vectored(
            &mut &*self.as_socketlike_view::<std::net::TcpStream>(),
            bufs,
        )
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        Read::read_to_end(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        Read::read_to_string(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>().peek(buf)
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        Write::write_all(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        Write::write_vectored(
            &mut &*self.as_socketlike_view::<std::net::TcpStream>(),
            bufs,
        )
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        Write::flush(&mut &*self.as_socketlike_view::<std::net::TcpStream>())
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        Write::write_fmt(&mut &*self.as_socketlike_view::<std::net::TcpStream>(), fmt)
    }
}

#[cfg(windows)]
#[cfg(feature = "cap_std_impls")]
impl IoExt for cap_std::net::TcpStream {
    #[inline]
    fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>().read(buf)
    }

    #[inline]
    fn read_exact(&self, buf: &mut [u8]) -> io::Result<()> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .read_exact(buf)
    }

    #[inline]
    fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&self, buf: &mut String) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .read_to_string(buf)
    }

    #[inline]
    fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>().peek(buf)
    }

    #[inline]
    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>().write(buf)
    }

    #[inline]
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .write_all(buf)
    }

    #[inline]
    fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .write_vectored(bufs)
    }

    #[inline]
    fn flush(&self) -> io::Result<()> {
        self.as_socketlike_view::<std::net::TcpStream>().flush()
    }

    #[inline]
    fn write_fmt(&self, fmt: Arguments) -> io::Result<()> {
        self.as_socketlike_view::<std::net::TcpStream>()
            .write_fmt(fmt)
    }
}

fn _io_ext_can_be_trait_object(_: &dyn IoExt) {}
