#![cfg(feature = "std")]

use std::fmt;
use std::io;

use crate::{Format, ToFormattedString};

/// <b><u>A key trait</u></b>. Gives types in the standard library that implement [`io::Write`]
/// or [`fmt::Write`], such as `&mut [u8]` and `&mut String`, a [`write_formatted`] method for writing
/// formatted numbers.
///
/// [`fmt::Write`]: https://doc.rust-lang.org/stable/std/fmt/trait.Write.html
/// [`io::Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
/// [`write_formatted`]: trait.WriteFormatted.html#method.write_formatted
pub trait WriteFormatted {
    /// Formats the provided number according to the provided format and then writes the resulting
    /// bytes to the object. Meant to be analagous to [`io::Write`]'s [`write_all`] method or
    /// [`fmt::Write`]'s [`write_str`] method. On success, returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] under the same conditions as [`io::Write`]'s [`write_all`] method.
    ///
    /// [`fmt::Write`]: https://doc.rust-lang.org/stable/std/fmt/trait.Write.html
    /// [`io::Error`]: https://doc.rust-lang.org/stable/std/io/struct.Error.html
    /// [`io::Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
    /// [`write_all`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html#method.write_all
    /// [`write_str`]: https://doc.rust-lang.org/stable/std/fmt/trait.Write.html#tymethod.write_str
    fn write_formatted<F, N>(&mut self, n: &N, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        N: ToFormattedString;
}

macro_rules! impl_for_fmt_write {
    () => {
        #[inline(always)]
        fn write_formatted<F, N>(&mut self, n: &N, format: &F) -> Result<usize, io::Error>
        where
            F: Format,
            N: ToFormattedString,
        {
            n.read_to_fmt_writer(self, format)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }
    };
}

macro_rules! impl_for_io_write {
    () => {
        #[inline(always)]
        fn write_formatted<F, N>(&mut self, n: &N, format: &F) -> Result<usize, io::Error>
        where
            F: Format,
            N: ToFormattedString,
        {
            n.read_to_io_writer(self, format)
        }
    };
}

#[rustfmt::skip]
mod any {
    use std::fs;
    use std::net;
    use std::process;

    use super::*;

    impl<W: io::Write + ?Sized> WriteFormatted for Box<W> { impl_for_io_write!(); }
    impl<W: io::Write> WriteFormatted for io::BufWriter<W> { impl_for_io_write!(); }
    impl WriteFormatted for process::ChildStdin { impl_for_io_write!(); }
    impl WriteFormatted for io::Cursor<Box<[u8]>> { impl_for_io_write!(); }
    impl WriteFormatted for io::Cursor<Vec<u8>> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for io::Cursor<&'a mut [u8]> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for io::Cursor<&'a mut Vec<u8>> { impl_for_io_write!(); }
    impl WriteFormatted for fs::File { impl_for_io_write!(); }
    impl<W: io::Write> WriteFormatted for io::LineWriter<W> { impl_for_io_write!(); }
    impl WriteFormatted for io::Sink { impl_for_io_write!(); }
    impl WriteFormatted for io::Stderr { impl_for_io_write!(); }
    impl WriteFormatted for io::Stdout { impl_for_io_write!(); }
    impl WriteFormatted for String { impl_for_fmt_write!(); }
    impl WriteFormatted for net::TcpStream { impl_for_io_write!(); }
    impl WriteFormatted for Vec<u8> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for io::StderrLock<'a> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for io::StdoutLock<'a> { impl_for_io_write!(); }

    impl<'a> WriteFormatted for &'a mut [u8] { impl_for_io_write!(); }
    impl<'a, W: io::Write + ?Sized> WriteFormatted for &'a mut Box<W> { impl_for_io_write!(); }
    impl<'a, W: io::Write> WriteFormatted for &'a mut io::BufWriter<W> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut process::ChildStdin { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut io::Cursor<Box<[u8]>> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut io::Cursor<Vec<u8>> { impl_for_io_write!(); }
    impl<'a, 'b> WriteFormatted for &'a mut io::Cursor<&'b mut [u8]> { impl_for_io_write!(); }
    impl<'a, 'b> WriteFormatted for &'a mut io::Cursor<&'b mut Vec<u8>> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a fs::File { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut fs::File { impl_for_io_write!(); }
    impl<'a, 'b> WriteFormatted for &'a mut fmt::Formatter<'b> { impl_for_fmt_write!(); }
    impl<'a, W: io::Write> WriteFormatted for &'a mut io::LineWriter<W> { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut io::Sink { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut io::Stderr { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut io::Stdout { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut String { impl_for_fmt_write!(); }
    impl<'a> WriteFormatted for &'a net::TcpStream { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut net::TcpStream { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut Vec<u8> { impl_for_io_write!(); }
    impl<'a, 'b> WriteFormatted for &'a mut io::StderrLock<'b> { impl_for_io_write!(); }
    impl<'a, 'b> WriteFormatted for &'a mut io::StdoutLock<'b> { impl_for_io_write!(); }
}

#[cfg(unix)]
#[rustfmt::skip]
mod unix {
    use std::os::unix::net::UnixStream;

    use super::*;

    impl WriteFormatted for UnixStream { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a UnixStream { impl_for_io_write!(); }
    impl<'a> WriteFormatted for &'a mut UnixStream { impl_for_io_write!(); }
}
