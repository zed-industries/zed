//! Formatting for log records.
//!
//! This module contains a [`Formatter`] that can be used to format log records
//! into without needing temporary allocations. Usually you won't need to worry
//! about the contents of this module and can use the `Formatter` like an ordinary
//! [`Write`].
//!
//! # Formatting log records
//!
//! The format used to print log records can be customised using the [`Builder::format`]
//! method.
//! Custom formats can apply different color and weight to printed values using
//! [`Style`] builders.
//!
//! ```
//! use std::io::Write;
//!
//! let mut builder = env_logger::Builder::new();
//!
//! builder.format(|buf, record| {
//!     writeln!(buf, "{}: {}",
//!         record.level(),
//!         record.args())
//! });
//! ```
//!
//! [`Formatter`]: struct.Formatter.html
//! [`Style`]: struct.Style.html
//! [`Builder::format`]: ../struct.Builder.html#method.format
//! [`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html

use std::cell::RefCell;
use std::fmt::Display;
use std::io::prelude::*;
use std::rc::Rc;
use std::{fmt, io, mem};

#[cfg(feature = "color")]
use log::Level;
use log::Record;

#[cfg(feature = "humantime")]
mod humantime;
pub(crate) mod writer;

#[cfg(feature = "color")]
mod style;
#[cfg(feature = "color")]
pub use style::{Color, Style, StyledValue};

#[cfg(feature = "humantime")]
pub use self::humantime::Timestamp;
pub use self::writer::glob::*;

use self::writer::{Buffer, Writer};

pub(crate) mod glob {
    pub use super::{Target, TimestampPrecision, WriteStyle};
}

/// Formatting precision of timestamps.
///
/// Seconds give precision of full seconds, milliseconds give thousands of a
/// second (3 decimal digits), microseconds are millionth of a second (6 decimal
/// digits) and nanoseconds are billionth of a second (9 decimal digits).
#[derive(Copy, Clone, Debug)]
pub enum TimestampPrecision {
    /// Full second precision (0 decimal digits)
    Seconds,
    /// Millisecond precision (3 decimal digits)
    Millis,
    /// Microsecond precision (6 decimal digits)
    Micros,
    /// Nanosecond precision (9 decimal digits)
    Nanos,
}

/// The default timestamp precision is seconds.
impl Default for TimestampPrecision {
    fn default() -> Self {
        TimestampPrecision::Seconds
    }
}

/// A formatter to write logs into.
///
/// `Formatter` implements the standard [`Write`] trait for writing log records.
/// It also supports terminal colors, through the [`style`] method.
///
/// # Examples
///
/// Use the [`writeln`] macro to format a log record.
/// An instance of a `Formatter` is passed to an `env_logger` format as `buf`:
///
/// ```
/// use std::io::Write;
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));
/// ```
///
/// [`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
/// [`writeln`]: https://doc.rust-lang.org/stable/std/macro.writeln.html
/// [`style`]: #method.style
pub struct Formatter {
    buf: Rc<RefCell<Buffer>>,
    write_style: WriteStyle,
}

impl Formatter {
    pub(crate) fn new(writer: &Writer) -> Self {
        Formatter {
            buf: Rc::new(RefCell::new(writer.buffer())),
            write_style: writer.write_style(),
        }
    }

    pub(crate) fn write_style(&self) -> WriteStyle {
        self.write_style
    }

    pub(crate) fn print(&self, writer: &Writer) -> io::Result<()> {
        writer.print(&self.buf.borrow())
    }

    pub(crate) fn clear(&mut self) {
        self.buf.borrow_mut().clear()
    }
}

#[cfg(feature = "color")]
impl Formatter {
    /// Begin a new [`Style`].
    ///
    /// # Examples
    ///
    /// Create a bold, red colored style and use it to print the log level:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut level_style = buf.style();
    ///
    ///     level_style.set_color(Color::Red).set_bold(true);
    ///
    ///     writeln!(buf, "{}: {}",
    ///         level_style.value(record.level()),
    ///         record.args())
    /// });
    /// ```
    ///
    /// [`Style`]: struct.Style.html
    pub fn style(&self) -> Style {
        Style {
            buf: self.buf.clone(),
            spec: termcolor::ColorSpec::new(),
        }
    }

    /// Get the default [`Style`] for the given level.
    ///
    /// The style can be used to print other values besides the level.
    pub fn default_level_style(&self, level: Level) -> Style {
        let mut level_style = self.style();
        match level {
            Level::Trace => level_style.set_color(Color::Cyan),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Info => level_style.set_color(Color::Green),
            Level::Warn => level_style.set_color(Color::Yellow),
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
        };
        level_style
    }

    /// Get a printable [`Style`] for the given level.
    ///
    /// The style can only be used to print the level.
    pub fn default_styled_level(&self, level: Level) -> StyledValue<'static, Level> {
        self.default_level_style(level).into_value(level)
    }
}

impl Write for Formatter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.borrow_mut().flush()
    }
}

impl fmt::Debug for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Formatter").finish()
    }
}

pub(crate) type FormatFn = Box<dyn Fn(&mut Formatter, &Record) -> io::Result<()> + Sync + Send>;

pub(crate) struct Builder {
    pub format_timestamp: Option<TimestampPrecision>,
    pub format_module_path: bool,
    pub format_target: bool,
    pub format_level: bool,
    pub format_indent: Option<usize>,
    pub custom_format: Option<FormatFn>,
    pub format_suffix: &'static str,
    built: bool,
}

impl Builder {
    /// Convert the format into a callable function.
    ///
    /// If the `custom_format` is `Some`, then any `default_format` switches are ignored.
    /// If the `custom_format` is `None`, then a default format is returned.
    /// Any `default_format` switches set to `false` won't be written by the format.
    pub fn build(&mut self) -> FormatFn {
        assert!(!self.built, "attempt to re-use consumed builder");

        let built = mem::replace(
            self,
            Builder {
                built: true,
                ..Default::default()
            },
        );

        if let Some(fmt) = built.custom_format {
            fmt
        } else {
            Box::new(move |buf, record| {
                let fmt = DefaultFormat {
                    timestamp: built.format_timestamp,
                    module_path: built.format_module_path,
                    target: built.format_target,
                    level: built.format_level,
                    written_header_value: false,
                    indent: built.format_indent,
                    suffix: built.format_suffix,
                    buf,
                };

                fmt.write(record)
            })
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            format_timestamp: Some(Default::default()),
            format_module_path: false,
            format_target: true,
            format_level: true,
            format_indent: Some(4),
            custom_format: None,
            format_suffix: "\n",
            built: false,
        }
    }
}

#[cfg(feature = "color")]
type SubtleStyle = StyledValue<'static, &'static str>;
#[cfg(not(feature = "color"))]
type SubtleStyle = &'static str;

/// The default format.
///
/// This format needs to work with any combination of crate features.
struct DefaultFormat<'a> {
    timestamp: Option<TimestampPrecision>,
    module_path: bool,
    target: bool,
    level: bool,
    written_header_value: bool,
    indent: Option<usize>,
    buf: &'a mut Formatter,
    suffix: &'a str,
}

impl<'a> DefaultFormat<'a> {
    fn write(mut self, record: &Record) -> io::Result<()> {
        self.write_timestamp()?;
        self.write_level(record)?;
        self.write_module_path(record)?;
        self.write_target(record)?;
        self.finish_header()?;

        self.write_args(record)
    }

    fn subtle_style(&self, text: &'static str) -> SubtleStyle {
        #[cfg(feature = "color")]
        {
            self.buf
                .style()
                .set_color(Color::Black)
                .set_intense(true)
                .clone()
                .into_value(text)
        }
        #[cfg(not(feature = "color"))]
        {
            text
        }
    }

    fn write_header_value<T>(&mut self, value: T) -> io::Result<()>
    where
        T: Display,
    {
        if !self.written_header_value {
            self.written_header_value = true;

            let open_brace = self.subtle_style("[");
            write!(self.buf, "{}{}", open_brace, value)
        } else {
            write!(self.buf, " {}", value)
        }
    }

    fn write_level(&mut self, record: &Record) -> io::Result<()> {
        if !self.level {
            return Ok(());
        }

        let level = {
            #[cfg(feature = "color")]
            {
                self.buf.default_styled_level(record.level())
            }
            #[cfg(not(feature = "color"))]
            {
                record.level()
            }
        };

        self.write_header_value(format_args!("{:<5}", level))
    }

    fn write_timestamp(&mut self) -> io::Result<()> {
        #[cfg(feature = "humantime")]
        {
            use self::TimestampPrecision::*;
            let ts = match self.timestamp {
                None => return Ok(()),
                Some(Seconds) => self.buf.timestamp_seconds(),
                Some(Millis) => self.buf.timestamp_millis(),
                Some(Micros) => self.buf.timestamp_micros(),
                Some(Nanos) => self.buf.timestamp_nanos(),
            };

            self.write_header_value(ts)
        }
        #[cfg(not(feature = "humantime"))]
        {
            // Trick the compiler to think we have used self.timestamp
            // Workaround for "field is never used: `timestamp`" compiler nag.
            let _ = self.timestamp;
            Ok(())
        }
    }

    fn write_module_path(&mut self, record: &Record) -> io::Result<()> {
        if !self.module_path {
            return Ok(());
        }

        if let Some(module_path) = record.module_path() {
            self.write_header_value(module_path)
        } else {
            Ok(())
        }
    }

    fn write_target(&mut self, record: &Record) -> io::Result<()> {
        if !self.target {
            return Ok(());
        }

        match record.target() {
            "" => Ok(()),
            target => self.write_header_value(target),
        }
    }

    fn finish_header(&mut self) -> io::Result<()> {
        if self.written_header_value {
            let close_brace = self.subtle_style("]");
            write!(self.buf, "{} ", close_brace)
        } else {
            Ok(())
        }
    }

    fn write_args(&mut self, record: &Record) -> io::Result<()> {
        match self.indent {
            // Fast path for no indentation
            None => write!(self.buf, "{}{}", record.args(), self.suffix),

            Some(indent_count) => {
                // Create a wrapper around the buffer only if we have to actually indent the message

                struct IndentWrapper<'a, 'b: 'a> {
                    fmt: &'a mut DefaultFormat<'b>,
                    indent_count: usize,
                }

                impl<'a, 'b> Write for IndentWrapper<'a, 'b> {
                    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                        let mut first = true;
                        for chunk in buf.split(|&x| x == b'\n') {
                            if !first {
                                write!(
                                    self.fmt.buf,
                                    "{}{:width$}",
                                    self.fmt.suffix,
                                    "",
                                    width = self.indent_count
                                )?;
                            }
                            self.fmt.buf.write_all(chunk)?;
                            first = false;
                        }

                        Ok(buf.len())
                    }

                    fn flush(&mut self) -> io::Result<()> {
                        self.fmt.buf.flush()
                    }
                }

                // The explicit scope here is just to make older versions of Rust happy
                {
                    let mut wrapper = IndentWrapper {
                        fmt: self,
                        indent_count,
                    };
                    write!(wrapper, "{}", record.args())?;
                }

                write!(self.buf, "{}", self.suffix)?;

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::{Level, Record};

    fn write_record(record: Record, fmt: DefaultFormat) -> String {
        let buf = fmt.buf.buf.clone();

        fmt.write(&record).expect("failed to write record");

        let buf = buf.borrow();
        String::from_utf8(buf.as_bytes().to_vec()).expect("failed to read record")
    }

    fn write_target(target: &str, fmt: DefaultFormat) -> String {
        write_record(
            Record::builder()
                .args(format_args!("log\nmessage"))
                .level(Level::Info)
                .file(Some("test.rs"))
                .line(Some(144))
                .module_path(Some("test::path"))
                .target(target)
                .build(),
            fmt,
        )
    }

    fn write(fmt: DefaultFormat) -> String {
        write_target("", fmt)
    }

    #[test]
    fn format_with_header() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: true,
            target: false,
            level: true,
            written_header_value: false,
            indent: None,
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("[INFO  test::path] log\nmessage\n", written);
    }

    #[test]
    fn format_no_header() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: false,
            target: false,
            level: false,
            written_header_value: false,
            indent: None,
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("log\nmessage\n", written);
    }

    #[test]
    fn format_indent_spaces() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: true,
            target: false,
            level: true,
            written_header_value: false,
            indent: Some(4),
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("[INFO  test::path] log\n    message\n", written);
    }

    #[test]
    fn format_indent_zero_spaces() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: true,
            target: false,
            level: true,
            written_header_value: false,
            indent: Some(0),
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("[INFO  test::path] log\nmessage\n", written);
    }

    #[test]
    fn format_indent_spaces_no_header() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: false,
            target: false,
            level: false,
            written_header_value: false,
            indent: Some(4),
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("log\n    message\n", written);
    }

    #[test]
    fn format_suffix() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: false,
            target: false,
            level: false,
            written_header_value: false,
            indent: None,
            suffix: "\n\n",
            buf: &mut f,
        });

        assert_eq!("log\nmessage\n\n", written);
    }

    #[test]
    fn format_suffix_with_indent() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: false,
            target: false,
            level: false,
            written_header_value: false,
            indent: Some(4),
            suffix: "\n\n",
            buf: &mut f,
        });

        assert_eq!("log\n\n    message\n\n", written);
    }

    #[test]
    fn format_target() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write_target(
            "target",
            DefaultFormat {
                timestamp: None,
                module_path: true,
                target: true,
                level: true,
                written_header_value: false,
                indent: None,
                suffix: "\n",
                buf: &mut f,
            },
        );

        assert_eq!("[INFO  test::path target] log\nmessage\n", written);
    }

    #[test]
    fn format_empty_target() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write(DefaultFormat {
            timestamp: None,
            module_path: true,
            target: true,
            level: true,
            written_header_value: false,
            indent: None,
            suffix: "\n",
            buf: &mut f,
        });

        assert_eq!("[INFO  test::path] log\nmessage\n", written);
    }

    #[test]
    fn format_no_target() {
        let writer = writer::Builder::new()
            .write_style(WriteStyle::Never)
            .build();

        let mut f = Formatter::new(&writer);

        let written = write_target(
            "target",
            DefaultFormat {
                timestamp: None,
                module_path: true,
                target: false,
                level: true,
                written_header_value: false,
                indent: None,
                suffix: "\n",
                buf: &mut f,
            },
        );

        assert_eq!("[INFO  test::path] log\nmessage\n", written);
    }
}
