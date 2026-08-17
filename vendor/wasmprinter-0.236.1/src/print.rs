use std::fmt;
use std::io;
use termcolor::{Color, ColorSpec};

/// Trait used to print WebAssembly modules in this crate.
///
/// Instances of this trait are passed to
/// [`Config::print`](super::Config::print). Instances of this trait are where
/// the output of a WebAssembly binary goes.
///
/// Note that this trait has built-in adapters in the `wasmprinter` crate:
///
/// * For users of [`std::io::Write`] use [`PrintIoWrite`].
/// * For users of [`std::fmt::Write`] use [`PrintFmtWrite`].
pub trait Print {
    /// Writes the given string `s` in its entirety.
    ///
    /// Returns an error for any I/O error.
    fn write_str(&mut self, s: &str) -> io::Result<()>;

    /// Indicates that a newline is being printed.
    ///
    /// This can be overridden to hook into the offset at which lines are
    /// printed.
    fn newline(&mut self) -> io::Result<()> {
        self.write_str("\n")
    }

    /// Indicates that a new line in the output is starting at the
    /// `binary_offset` provided.
    ///
    /// Not all new lines have a binary offset associated with them but this
    /// method should be called for new lines in the output. This enables
    /// correlating binary offsets to lines in the output.
    fn start_line(&mut self, binary_offset: Option<usize>) {
        let _ = binary_offset;
    }

    /// Enables usage of `write!` with this trait.
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        struct Adapter<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: io::Result<()>,
        }

        impl<T: Print + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, args) {
            Ok(()) => Ok(()),
            Err(..) => output.error,
        }
    }

    /// Helper to print a custom section, if needed.
    ///
    /// If this output has a custom means of printing a custom section then this
    /// can be used to override the default. If `Ok(true)` is returned then the
    /// section will be considered to be printed. Otherwise an `Ok(false)`
    /// return value indicates that the default printing should happen.
    fn print_custom_section(
        &mut self,
        name: &str,
        binary_offset: usize,
        data: &[u8],
    ) -> io::Result<bool> {
        let _ = (name, binary_offset, data);
        Ok(false)
    }

    /// Sets the colors settings for literals (`"foo"`) to be printed.
    fn start_literal(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Sets the colors settings for a name (`$foo`) to be printed.
    fn start_name(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Sets the colors settings for a keyword (`(module ...)`) to be printed.
    fn start_keyword(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Sets the colors settings for a type (`(param i32)`) to be printed.
    fn start_type(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Sets the colors settings for a comment (`;; ...`) to be printed.
    fn start_comment(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Resets colors settings to the default.
    fn reset_color(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Returns `true` if the device uses colors without interacting synchronously with a terminal (e.g. ANSI)
    fn supports_async_color(&self) -> bool {
        false
    }
}

/// An adapter between the [`std::io::Write`] trait and [`Print`].
pub struct PrintIoWrite<T>(pub T);

impl<T> Print for PrintIoWrite<T>
where
    T: io::Write,
{
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.0.write_all(s.as_bytes())
    }
}

/// An adapter between the [`std::fmt::Write`] trait and [`Print`].
pub struct PrintFmtWrite<T>(pub T);

impl<T> Print for PrintFmtWrite<T>
where
    T: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        match self.0.write_str(s) {
            Ok(()) => Ok(()),
            Err(fmt::Error) => Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to write string",
            )),
        }
    }
}

/// An adapter between the [`termcolor::WriteColor`] trait and [`Print`].
pub struct PrintTermcolor<T>(pub T);

impl<T> Print for PrintTermcolor<T>
where
    T: termcolor::WriteColor,
{
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.0.write_all(s.as_bytes())
    }

    fn start_name(&mut self) -> io::Result<()> {
        self.0
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
    }

    fn start_literal(&mut self) -> io::Result<()> {
        self.0.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
    }

    fn start_keyword(&mut self) -> io::Result<()> {
        self.0.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Yellow))
                .set_bold(true)
                .set_intense(true),
        )
    }

    fn start_type(&mut self) -> io::Result<()> {
        self.0.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Green))
                .set_bold(true)
                .set_intense(true),
        )
    }

    fn start_comment(&mut self) -> io::Result<()> {
        self.0.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
    }

    fn reset_color(&mut self) -> io::Result<()> {
        self.0.reset()
    }

    fn supports_async_color(&self) -> bool {
        self.0.supports_color() && !self.0.is_synchronous()
    }
}
