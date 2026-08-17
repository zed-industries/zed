use std::io::{Error as IOError, Write};
use std::string::FromUtf8Error;

/// The Output API.
///
/// Handlebars uses this trait to define rendered output.
pub trait Output {
    fn write(&mut self, seg: &str) -> Result<(), IOError>;

    /// Designed to be used with `write!` macro.
    /// for backward compatibility and to avoid breakage the default implementation
    /// uses `format!` this may be not what you want.
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), IOError> {
        // Check if there is nothing to format to avoid allocation on case like
        // write!(out, "hey")?;
        if let Some(content) = args.as_str() {
            self.write(content)
        } else {
            self.write(&std::fmt::format(args))
        }
    }
}

pub struct WriteOutput<W: Write> {
    write: W,
}

impl<W: Write> Output for WriteOutput<W> {
    fn write(&mut self, seg: &str) -> Result<(), IOError> {
        self.write.write_all(seg.as_bytes())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), IOError> {
        self.write.write_fmt(args)
    }
}

impl<W: Write> WriteOutput<W> {
    pub fn new(write: W) -> WriteOutput<W> {
        WriteOutput { write }
    }
}

pub struct StringOutput {
    buf: Vec<u8>,
}

impl Output for StringOutput {
    fn write(&mut self, seg: &str) -> Result<(), IOError> {
        self.buf.extend_from_slice(seg.as_bytes());
        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), IOError> {
        self.buf.write_fmt(args)
    }
}

impl StringOutput {
    pub fn new() -> StringOutput {
        StringOutput {
            buf: Vec::with_capacity(8 * 1024),
        }
    }

    pub fn into_string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.buf)
    }
}

impl Default for StringOutput {
    fn default() -> Self {
        StringOutput::new()
    }
}
