use std::io;

use crate::term::Term;

pub(crate) fn move_cursor_down(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{n}B"))
    } else {
        Ok(())
    }
}

pub(crate) fn move_cursor_up(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{n}A"))
    } else {
        Ok(())
    }
}
pub(crate) fn move_cursor_left(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{n}D"))
    } else {
        Ok(())
    }
}

pub(crate) fn move_cursor_right(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{n}C"))
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn move_cursor_to(out: &Term, x: usize, y: usize) -> io::Result<()> {
    out.write_str(&format!("\x1B[{};{}H", y + 1, x + 1))
}

pub(crate) fn clear_chars(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{n}D\x1b[0K"))
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn clear_line(out: &Term) -> io::Result<()> {
    out.write_str("\r\x1b[2K")
}

#[inline]
pub(crate) fn clear_screen(out: &Term) -> io::Result<()> {
    out.write_str("\r\x1b[2J\r\x1b[H")
}

#[inline]
pub(crate) fn clear_to_end_of_screen(out: &Term) -> io::Result<()> {
    out.write_str("\r\x1b[0J")
}

#[inline]
pub(crate) fn show_cursor(out: &Term) -> io::Result<()> {
    out.write_str("\x1b[?25h")
}

#[inline]
pub(crate) fn hide_cursor(out: &Term) -> io::Result<()> {
    out.write_str("\x1b[?25l")
}
