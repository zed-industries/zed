use std::io::{self, Write};
use std::sync::Mutex;

use termcolor::{self, ColorSpec, WriteColor};

use crate::fmt::{WritableTarget, WriteStyle};

pub(in crate::fmt::writer) struct BufferWriter {
    inner: termcolor::BufferWriter,
    uncolored_target: Option<WritableTarget>,
    write_style: WriteStyle,
}

impl BufferWriter {
    pub(in crate::fmt::writer) fn stderr(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stderr(write_style.into_color_choice()),
            uncolored_target: if is_test {
                Some(WritableTarget::PrintStderr)
            } else {
                None
            },
            write_style,
        }
    }

    pub(in crate::fmt::writer) fn stdout(is_test: bool, write_style: WriteStyle) -> Self {
        BufferWriter {
            inner: termcolor::BufferWriter::stdout(write_style.into_color_choice()),
            uncolored_target: if is_test {
                Some(WritableTarget::PrintStdout)
            } else {
                None
            },
            write_style,
        }
    }

    pub(in crate::fmt::writer) fn pipe(pipe: Box<Mutex<dyn io::Write + Send + 'static>>) -> Self {
        let write_style = WriteStyle::Never;
        BufferWriter {
            // The inner Buffer is never printed from, but it is still needed to handle coloring and other formatting
            inner: termcolor::BufferWriter::stderr(write_style.into_color_choice()),
            uncolored_target: Some(WritableTarget::Pipe(pipe)),
            write_style,
        }
    }

    pub(in crate::fmt::writer) fn write_style(&self) -> WriteStyle {
        self.write_style
    }

    pub(in crate::fmt::writer) fn buffer(&self) -> Buffer {
        Buffer {
            inner: self.inner.buffer(),
            has_uncolored_target: self.uncolored_target.is_some(),
        }
    }

    pub(in crate::fmt::writer) fn print(&self, buf: &Buffer) -> io::Result<()> {
        if let Some(target) = &self.uncolored_target {
            target.print(buf)
        } else {
            self.inner.print(&buf.inner)
        }
    }
}

pub(in crate::fmt) struct Buffer {
    inner: termcolor::Buffer,
    has_uncolored_target: bool,
}

impl Buffer {
    pub(in crate::fmt) fn clear(&mut self) {
        self.inner.clear()
    }

    pub(in crate::fmt) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    pub(in crate::fmt) fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    pub(in crate::fmt) fn as_bytes(&self) -> &[u8] {
        self.inner.as_slice()
    }

    pub(in crate::fmt) fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        // Ignore styles for test captured logs because they can't be printed
        if !self.has_uncolored_target {
            self.inner.set_color(spec)
        } else {
            Ok(())
        }
    }

    pub(in crate::fmt) fn reset(&mut self) -> io::Result<()> {
        // Ignore styles for test captured logs because they can't be printed
        if !self.has_uncolored_target {
            self.inner.reset()
        } else {
            Ok(())
        }
    }
}
