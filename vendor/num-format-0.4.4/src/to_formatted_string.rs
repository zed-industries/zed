#![cfg(feature = "std")]

use std::fmt;
use std::io;

use crate::constants::MAX_BUF_LEN;
use crate::sealed::Sealed;
use crate::{Buffer, Format, ToFormattedStr};

/// <b><u>A key trait</u></b>. Gives numbers the [`to_formatted_string`] method.
///
/// This trait is sealed; so you may not implement it on your own types.
///
/// [`to_formatted_string`]: trait.ToFormattedString.html#method.to_formatted_string
pub trait ToFormattedString: Sealed + Sized {
    #[doc(hidden)]
    fn read_to_fmt_writer<F, W>(&self, w: W, format: &F) -> Result<usize, fmt::Error>
    where
        F: Format,
        W: fmt::Write;

    #[doc(hidden)]
    fn read_to_io_writer<F, W>(&self, w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: io::Write;

    /// Returns a string representation of the number formatted according to the provided format.
    fn to_formatted_string<F>(&self, format: &F) -> String
    where
        F: Format,
    {
        let mut s = String::with_capacity(MAX_BUF_LEN);
        let _ = self.read_to_fmt_writer(&mut s, format).unwrap();
        s
    }
}

impl<T> ToFormattedString for T
where
    T: ToFormattedStr,
{
    #[inline(always)]
    fn read_to_fmt_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, fmt::Error>
    where
        F: Format,
        W: fmt::Write,
    {
        let mut buf = Buffer::default();
        let c = self.read_to_buffer(&mut buf, format);
        w.write_str(buf.as_str())?;
        Ok(c)
    }

    #[inline(always)]
    fn read_to_io_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: io::Write,
    {
        let mut buf = Buffer::default();
        let c = self.read_to_buffer(&mut buf, format);
        w.write_all(buf.as_bytes())?;
        Ok(c)
    }
}
