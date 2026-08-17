use std::fmt;

/// Alternative implementation of `fmt::Debug` for byte slice.
///
/// Standard `Debug` implementation for `[u8]` is comma separated
/// list of numbers. Since large amount of byte strings are in fact
/// ASCII strings or contain a lot of ASCII strings (e. g. HTTP),
/// it is convenient to print strings as ASCII when possible.
///
/// This struct wraps `&[u8]` just to override `fmt::Debug`.
///
/// `BsDebug` is not a part of public API of bytes crate.
pub struct BsDebug<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for BsDebug<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(fmt, "b\""));
        for &c in self.0 {
            // https://doc.rust-lang.org/reference.html#byte-escapes
            if c == b'\n' {
                try!(write!(fmt, "\\n"));
            } else if c == b'\r' {
                try!(write!(fmt, "\\r"));
            } else if c == b'\t' {
                try!(write!(fmt, "\\t"));
            } else if c == b'\\' || c == b'"' {
                try!(write!(fmt, "\\{}", c as char));
            } else if c == b'\0' {
                try!(write!(fmt, "\\0"));
            // ASCII printable
            } else if c >= 0x20 && c < 0x7f {
                try!(write!(fmt, "{}", c as char));
            } else {
                try!(write!(fmt, "\\x{:02x}", c));
            }
        }
        try!(write!(fmt, "\""));
        Ok(())
    }
}
