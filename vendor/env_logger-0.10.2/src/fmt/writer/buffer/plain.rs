use std::{io, sync::Mutex};

use crate::fmt::{WritableTarget, WriteStyle};

pub(in crate::fmt::writer) struct BufferWriter {
    target: WritableTarget,
}

impl BufferWriter {
    pub(in crate::fmt::writer) fn stderr(is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: if is_test {
                WritableTarget::PrintStderr
            } else {
                WritableTarget::WriteStderr
            },
        }
    }

    pub(in crate::fmt::writer) fn stdout(is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: if is_test {
                WritableTarget::PrintStdout
            } else {
                WritableTarget::WriteStdout
            },
        }
    }

    pub(in crate::fmt::writer) fn pipe(pipe: Box<Mutex<dyn io::Write + Send + 'static>>) -> Self {
        BufferWriter {
            target: WritableTarget::Pipe(pipe),
        }
    }

    pub(in crate::fmt::writer) fn write_style(&self) -> WriteStyle {
        WriteStyle::Never
    }

    pub(in crate::fmt::writer) fn buffer(&self) -> Buffer {
        Buffer(Vec::new())
    }

    pub(in crate::fmt::writer) fn print(&self, buf: &Buffer) -> io::Result<()> {
        self.target.print(buf)
    }
}

pub(in crate::fmt) struct Buffer(Vec<u8>);

impl Buffer {
    pub(in crate::fmt) fn clear(&mut self) {
        self.0.clear();
    }

    pub(in crate::fmt) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    pub(in crate::fmt) fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub(in crate::fmt) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
