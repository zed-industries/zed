use std::io::{self, Write};
use term::color::Color;
use term::{self, Attr, Terminal};

/// A `Terminal` that just ignores all attempts at formatting. Used
/// to report errors when no ANSI terminfo is available.
pub struct FakeTerminal<W: Write> {
    write: W,
}

impl<W: Write> FakeTerminal<W> {
    pub fn new(write: W) -> FakeTerminal<W> {
        FakeTerminal { write }
    }
}

impl<W: Write> Write for FakeTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write.flush()
    }
}

impl<W: Write> Terminal for FakeTerminal<W> {
    type Output = W;

    fn fg(&mut self, _color: Color) -> term::Result<()> {
        Ok(())
    }

    fn bg(&mut self, _color: Color) -> term::Result<()> {
        Ok(())
    }

    fn attr(&mut self, _attr: Attr) -> term::Result<()> {
        Ok(())
    }

    fn supports_attr(&self, _attr: Attr) -> bool {
        false
    }

    fn reset(&mut self) -> term::Result<()> {
        Ok(())
    }

    fn supports_reset(&self) -> bool {
        false
    }

    fn supports_color(&self) -> bool {
        false
    }

    fn cursor_up(&mut self) -> term::Result<()> {
        Ok(())
    }

    fn delete_line(&mut self) -> term::Result<()> {
        Ok(())
    }

    fn carriage_return(&mut self) -> term::Result<()> {
        Ok(())
    }

    fn get_ref(&self) -> &Self::Output {
        &self.write
    }

    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.write
    }

    fn into_inner(self) -> Self::Output
    where
        Self: Sized,
    {
        self.write
    }
}
