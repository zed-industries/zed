mod atty;
mod buffer;

use self::atty::{is_stderr, is_stdout};
use self::buffer::BufferWriter;
use std::{fmt, io, mem, sync::Mutex};

pub(super) mod glob {
    pub use super::*;
}

pub(super) use self::buffer::Buffer;

/// Log target, either `stdout`, `stderr` or a custom pipe.
#[non_exhaustive]
pub enum Target {
    /// Logs will be sent to standard output.
    Stdout,
    /// Logs will be sent to standard error.
    Stderr,
    /// Logs will be sent to a custom pipe.
    Pipe(Box<dyn io::Write + Send + 'static>),
}

impl Default for Target {
    fn default() -> Self {
        Target::Stderr
    }
}

impl fmt::Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stdout => "stdout",
                Self::Stderr => "stderr",
                Self::Pipe(_) => "pipe",
            }
        )
    }
}

/// Log target, either `stdout`, `stderr` or a custom pipe.
///
/// Same as `Target`, except the pipe is wrapped in a mutex for interior mutability.
pub(super) enum WritableTarget {
    /// Logs will be written to standard output.
    #[allow(dead_code)]
    WriteStdout,
    /// Logs will be printed to standard output.
    PrintStdout,
    /// Logs will be written to standard error.
    #[allow(dead_code)]
    WriteStderr,
    /// Logs will be printed to standard error.
    PrintStderr,
    /// Logs will be sent to a custom pipe.
    Pipe(Box<Mutex<dyn io::Write + Send + 'static>>),
}

impl WritableTarget {
    fn print(&self, buf: &Buffer) -> io::Result<()> {
        use std::io::Write as _;

        let buf = buf.as_bytes();
        match self {
            WritableTarget::WriteStdout => {
                let stream = std::io::stdout();
                let mut stream = stream.lock();
                stream.write_all(buf)?;
                stream.flush()?;
            }
            WritableTarget::PrintStdout => print!("{}", String::from_utf8_lossy(buf)),
            WritableTarget::WriteStderr => {
                let stream = std::io::stderr();
                let mut stream = stream.lock();
                stream.write_all(buf)?;
                stream.flush()?;
            }
            WritableTarget::PrintStderr => eprint!("{}", String::from_utf8_lossy(buf)),
            // Safety: If the target type is `Pipe`, `target_pipe` will always be non-empty.
            WritableTarget::Pipe(pipe) => {
                let mut stream = pipe.lock().unwrap();
                stream.write_all(buf)?;
                stream.flush()?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for WritableTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WriteStdout => "stdout",
                Self::PrintStdout => "stdout",
                Self::WriteStderr => "stderr",
                Self::PrintStderr => "stderr",
                Self::Pipe(_) => "pipe",
            }
        )
    }
}
/// Whether or not to print styles to the target.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WriteStyle {
    /// Try to print styles, but don't force the issue.
    Auto,
    /// Try very hard to print styles.
    Always,
    /// Never print styles.
    Never,
}

impl Default for WriteStyle {
    fn default() -> Self {
        WriteStyle::Auto
    }
}

#[cfg(feature = "color")]
impl WriteStyle {
    fn into_color_choice(self) -> ::termcolor::ColorChoice {
        match self {
            WriteStyle::Always => ::termcolor::ColorChoice::Always,
            WriteStyle::Auto => ::termcolor::ColorChoice::Auto,
            WriteStyle::Never => ::termcolor::ColorChoice::Never,
        }
    }
}

/// A terminal target with color awareness.
pub(crate) struct Writer {
    inner: BufferWriter,
}

impl Writer {
    pub fn write_style(&self) -> WriteStyle {
        self.inner.write_style()
    }

    pub(super) fn buffer(&self) -> Buffer {
        self.inner.buffer()
    }

    pub(super) fn print(&self, buf: &Buffer) -> io::Result<()> {
        self.inner.print(buf)
    }
}

impl fmt::Debug for Writer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer").finish()
    }
}

/// A builder for a terminal writer.
///
/// The target and style choice can be configured before building.
#[derive(Debug)]
pub(crate) struct Builder {
    target: Target,
    write_style: WriteStyle,
    is_test: bool,
    built: bool,
}

impl Builder {
    /// Initialize the writer builder with defaults.
    pub(crate) fn new() -> Self {
        Builder {
            target: Default::default(),
            write_style: Default::default(),
            is_test: false,
            built: false,
        }
    }

    /// Set the target to write to.
    pub(crate) fn target(&mut self, target: Target) -> &mut Self {
        self.target = target;
        self
    }

    /// Parses a style choice string.
    ///
    /// See the [Disabling colors] section for more details.
    ///
    /// [Disabling colors]: ../index.html#disabling-colors
    pub(crate) fn parse_write_style(&mut self, write_style: &str) -> &mut Self {
        self.write_style(parse_write_style(write_style))
    }

    /// Whether or not to print style characters when writing.
    pub(crate) fn write_style(&mut self, write_style: WriteStyle) -> &mut Self {
        self.write_style = write_style;
        self
    }

    /// Whether or not to capture logs for `cargo test`.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn is_test(&mut self, is_test: bool) -> &mut Self {
        self.is_test = is_test;
        self
    }

    /// Build a terminal writer.
    pub(crate) fn build(&mut self) -> Writer {
        assert!(!self.built, "attempt to re-use consumed builder");
        self.built = true;

        let color_choice = match self.write_style {
            WriteStyle::Auto => {
                if match &self.target {
                    Target::Stderr => is_stderr(),
                    Target::Stdout => is_stdout(),
                    Target::Pipe(_) => false,
                } {
                    WriteStyle::Auto
                } else {
                    WriteStyle::Never
                }
            }
            color_choice => color_choice,
        };
        let color_choice = if self.is_test {
            WriteStyle::Never
        } else {
            color_choice
        };

        let writer = match mem::take(&mut self.target) {
            Target::Stderr => BufferWriter::stderr(self.is_test, color_choice),
            Target::Stdout => BufferWriter::stdout(self.is_test, color_choice),
            Target::Pipe(pipe) => BufferWriter::pipe(Box::new(Mutex::new(pipe))),
        };

        Writer { inner: writer }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

fn parse_write_style(spec: &str) -> WriteStyle {
    match spec {
        "auto" => WriteStyle::Auto,
        "always" => WriteStyle::Always,
        "never" => WriteStyle::Never,
        _ => Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_write_style_valid() {
        let inputs = vec![
            ("auto", WriteStyle::Auto),
            ("always", WriteStyle::Always),
            ("never", WriteStyle::Never),
        ];

        for (input, expected) in inputs {
            assert_eq!(expected, parse_write_style(input));
        }
    }

    #[test]
    fn parse_write_style_invalid() {
        let inputs = vec!["", "true", "false", "NEVER!!"];

        for input in inputs {
            assert_eq!(WriteStyle::Auto, parse_write_style(input));
        }
    }
}
