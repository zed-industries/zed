#[cfg(target_os = "macos")]
use core::ptr;
use core::{fmt::Display, mem, str};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::os::fd::{AsRawFd, RawFd};

#[cfg(not(target_os = "macos"))]
use std::sync::OnceLock;

use crate::kb::Key;
use crate::term::Term;

pub(crate) use crate::common_term::*;

pub(crate) const DEFAULT_WIDTH: u16 = 80;

#[inline]
pub(crate) fn is_a_terminal(out: &impl AsRawFd) -> bool {
    unsafe { libc::isatty(out.as_raw_fd()) != 0 }
}

pub(crate) fn is_a_color_terminal(out: &Term) -> bool {
    if !is_a_terminal(out) {
        return false;
    }

    if env::var("NO_COLOR").is_ok() {
        return false;
    }

    match env::var("TERM") {
        Ok(term) => term != "dumb",
        Err(_) => false,
    }
}

pub(crate) fn is_a_true_color_terminal(out: &Term) -> bool {
    if !is_a_color_terminal(out) {
        return false;
    }
    env::var("COLORTERM").is_ok_and(|term| term == "truecolor" || term == "24bit")
}

fn c_result<F: FnOnce() -> libc::c_int>(f: F) -> io::Result<()> {
    let res = f();
    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub(crate) fn terminal_size(out: &Term) -> Option<(u16, u16)> {
    if !is_a_terminal(out) {
        return None;
    }
    let winsize = unsafe {
        let mut winsize: libc::winsize = mem::zeroed();

        // FIXME: ".into()" used as a temporary fix for a libc bug
        // https://github.com/rust-lang/libc/pull/704
        #[allow(clippy::useless_conversion)]
        libc::ioctl(out.as_raw_fd(), libc::TIOCGWINSZ.into(), &mut winsize);
        winsize
    };
    if winsize.ws_row > 0 && winsize.ws_col > 0 {
        Some((winsize.ws_row, winsize.ws_col))
    } else {
        None
    }
}

enum Input<T> {
    Stdin(io::Stdin),
    File(T),
}

impl Input<BufReader<fs::File>> {
    fn buffered() -> io::Result<Self> {
        Ok(match Input::unbuffered()? {
            Input::Stdin(s) => Input::Stdin(s),
            Input::File(f) => Input::File(BufReader::new(f)),
        })
    }
}

impl Input<fs::File> {
    fn unbuffered() -> io::Result<Self> {
        let stdin = io::stdin();
        if is_a_terminal(&stdin) {
            Ok(Input::Stdin(stdin))
        } else {
            let f = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?;
            Ok(Input::File(f))
        }
    }
}

// NB: this is not a full BufRead implementation because io::Stdin does not implement BufRead.
impl<T: BufRead> Input<T> {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        match self {
            Self::Stdin(s) => s.read_line(buf),
            Self::File(f) => f.read_line(buf),
        }
    }
}

impl AsRawFd for Input<fs::File> {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Self::Stdin(s) => s.as_raw_fd(),
            Self::File(f) => f.as_raw_fd(),
        }
    }
}

impl AsRawFd for Input<BufReader<fs::File>> {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Self::Stdin(s) => s.as_raw_fd(),
            Self::File(f) => f.get_ref().as_raw_fd(),
        }
    }
}

pub(crate) fn read_secure() -> io::Result<String> {
    let mut input = Input::buffered()?;

    let mut termios = mem::MaybeUninit::uninit();
    c_result(|| unsafe { libc::tcgetattr(input.as_raw_fd(), termios.as_mut_ptr()) })?;
    let mut termios = unsafe { termios.assume_init() };
    let original = termios;
    termios.c_lflag &= !libc::ECHO;
    c_result(|| unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSAFLUSH, &termios) })?;
    let mut rv = String::new();

    let read_rv = input.read_line(&mut rv);

    c_result(|| unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSAFLUSH, &original) })?;

    read_rv.map(|_| {
        let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        rv
    })
}

fn poll_fd(fd: RawFd, timeout: i32) -> io::Result<bool> {
    let mut pollfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    let ret = unsafe { libc::poll(&mut pollfd as *mut _, 1, timeout) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(pollfd.revents & libc::POLLIN != 0)
    }
}

#[cfg(target_os = "macos")]
fn select_fd(fd: RawFd, timeout: i32) -> io::Result<bool> {
    unsafe {
        let mut read_fd_set: libc::fd_set = mem::zeroed();

        let mut timeout_val;
        let timeout = if timeout < 0 {
            ptr::null_mut()
        } else {
            timeout_val = libc::timeval {
                tv_sec: (timeout / 1000) as _,
                tv_usec: (timeout * 1000) as _,
            };
            &mut timeout_val
        };

        libc::FD_ZERO(&mut read_fd_set);
        libc::FD_SET(fd, &mut read_fd_set);
        let ret = libc::select(
            fd + 1,
            &mut read_fd_set,
            ptr::null_mut(),
            ptr::null_mut(),
            timeout,
        );
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(libc::FD_ISSET(fd, &read_fd_set))
        }
    }
}

fn select_or_poll_term_fd(fd: RawFd, timeout: i32) -> io::Result<bool> {
    // There is a bug on macos that ttys cannot be polled, only select()
    // works.  However given how problematic select is in general, we
    // normally want to use poll there too.
    #[cfg(target_os = "macos")]
    {
        if unsafe { libc::isatty(fd) == 1 } {
            return select_fd(fd, timeout);
        }
    }
    poll_fd(fd, timeout)
}

fn read_single_char(fd: RawFd) -> io::Result<Option<char>> {
    // timeout of zero means that it will not block
    let is_ready = select_or_poll_term_fd(fd, 0)?;

    if is_ready {
        // if there is something to be read, take 1 byte from it
        let mut buf: [u8; 1] = [0];

        read_bytes(fd, &mut buf, 1)?;
        Ok(Some(buf[0] as char))
    } else {
        //there is nothing to be read
        Ok(None)
    }
}

// Similar to libc::read. Read count bytes into slice buf from descriptor fd.
// If successful, return the number of bytes read.
// Will return an error if nothing was read, i.e when called at end of file.
fn read_bytes(fd: RawFd, buf: &mut [u8], count: u8) -> io::Result<u8> {
    let read = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, count as usize) };
    if read < 0 {
        Err(io::Error::last_os_error())
    } else if read == 0 {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Reached end of file",
        ))
    } else if buf[0] == b'\x03' {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "read interrupted",
        ))
    } else {
        Ok(read as u8)
    }
}

fn read_single_key_impl(fd: RawFd) -> Result<Key, io::Error> {
    loop {
        match read_single_char(fd)? {
            Some('\x1b') => {
                // Escape was read, keep reading in case we find a familiar key
                break if let Some(c1) = read_single_char(fd)? {
                    if c1 == '[' {
                        if let Some(c2) = read_single_char(fd)? {
                            match c2 {
                                'A' => Ok(Key::ArrowUp),
                                'B' => Ok(Key::ArrowDown),
                                'C' => Ok(Key::ArrowRight),
                                'D' => Ok(Key::ArrowLeft),
                                'H' => Ok(Key::Home),
                                'F' => Ok(Key::End),
                                'Z' => Ok(Key::BackTab),
                                _ => {
                                    let c3 = read_single_char(fd)?;
                                    if let Some(c3) = c3 {
                                        if c3 == '~' {
                                            match c2 {
                                                '1' => Ok(Key::Home), // tmux
                                                '2' => Ok(Key::Insert),
                                                '3' => Ok(Key::Del),
                                                '4' => Ok(Key::End), // tmux
                                                '5' => Ok(Key::PageUp),
                                                '6' => Ok(Key::PageDown),
                                                '7' => Ok(Key::Home), // xrvt
                                                '8' => Ok(Key::End),  // xrvt
                                                _ => Ok(Key::UnknownEscSeq(vec![c1, c2, c3])),
                                            }
                                        } else {
                                            Ok(Key::UnknownEscSeq(vec![c1, c2, c3]))
                                        }
                                    } else {
                                        // \x1b[ and 1 more char
                                        Ok(Key::UnknownEscSeq(vec![c1, c2]))
                                    }
                                }
                            }
                        } else {
                            // \x1b[ and no more input
                            Ok(Key::UnknownEscSeq(vec![c1]))
                        }
                    } else {
                        // char after escape is not [
                        Ok(Key::UnknownEscSeq(vec![c1]))
                    }
                } else {
                    //nothing after escape
                    Ok(Key::Escape)
                };
            }
            Some(c) => {
                let byte = c as u8;
                let mut buf: [u8; 4] = [byte, 0, 0, 0];

                break if byte & 224u8 == 192u8 {
                    // a two byte unicode character
                    read_bytes(fd, &mut buf[1..], 1)?;
                    Ok(key_from_utf8(&buf[..2]))
                } else if byte & 240u8 == 224u8 {
                    // a three byte unicode character
                    read_bytes(fd, &mut buf[1..], 2)?;
                    Ok(key_from_utf8(&buf[..3]))
                } else if byte & 248u8 == 240u8 {
                    // a four byte unicode character
                    read_bytes(fd, &mut buf[1..], 3)?;
                    Ok(key_from_utf8(&buf[..4]))
                } else {
                    Ok(match c {
                        '\n' | '\r' => Key::Enter,
                        '\x7f' => Key::Backspace,
                        '\t' => Key::Tab,
                        '\x01' => Key::Home,      // Control-A (home)
                        '\x05' => Key::End,       // Control-E (end)
                        '\x08' => Key::Backspace, // Control-H (8) (Identical to '\b')
                        _ => Key::Char(c),
                    })
                };
            }
            None => {
                // there is no subsequent byte ready to be read, block and wait for input
                // negative timeout means that it will block indefinitely
                match select_or_poll_term_fd(fd, -1) {
                    Ok(_) => continue,
                    Err(_) => break Err(io::Error::last_os_error()),
                }
            }
        }
    }
}

pub(crate) fn read_single_key(ctrlc_key: bool) -> io::Result<Key> {
    let input = Input::unbuffered()?;

    let mut termios = core::mem::MaybeUninit::uninit();
    c_result(|| unsafe { libc::tcgetattr(input.as_raw_fd(), termios.as_mut_ptr()) })?;
    let mut termios = unsafe { termios.assume_init() };
    let original = termios;
    make_raw(&mut termios);
    termios.c_oflag = original.c_oflag;
    c_result(|| unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSADRAIN, &termios) })?;
    let rv = read_single_key_impl(input.as_raw_fd());
    c_result(|| unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSADRAIN, &original) })?;

    // if the user hit ^C we want to signal SIGINT to ourselves.
    if let Err(ref err) = rv {
        if err.kind() == io::ErrorKind::Interrupted {
            if !ctrlc_key {
                unsafe {
                    libc::raise(libc::SIGINT);
                }
            } else {
                return Ok(Key::CtrlC);
            }
        }
    }

    rv
}

fn key_from_utf8(buf: &[u8]) -> Key {
    if let Ok(s) = str::from_utf8(buf) {
        if let Some(c) = s.chars().next() {
            return Key::Char(c);
        }
    }
    Key::Unknown
}

#[cfg(target_os = "macos")]
pub(crate) fn wants_emoji() -> bool {
    true
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn wants_emoji() -> bool {
    static IS_LANG_UTF8: OnceLock<bool> = OnceLock::new();

    *IS_LANG_UTF8.get_or_init(|| match std::env::var("LANG") {
        Ok(lang) => lang.to_uppercase().ends_with("UTF-8"),
        _ => false,
    })
}

pub(crate) fn set_title<T: Display>(title: T) {
    print!("\x1b]0;{title}\x07");
}

#[cfg(target_os = "nto")]
fn make_raw(termios: &mut libc::termios) {
    // This is the manual implementation for QNX, which does not have cfmakeraw.
    termios.c_iflag &= !(libc::IGNBRK
        | libc::BRKINT
        | libc::PARMRK
        | libc::ISTRIP
        | libc::INLCR
        | libc::IGNCR
        | libc::ICRNL
        | libc::IXON);
    termios.c_oflag &= !libc::OPOST;
    termios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
    termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
    termios.c_cflag |= libc::CS8;
}

// This version will be compiled for all targets that are NOT QNX.
#[cfg(not(target_os = "nto"))]
fn make_raw(termios: &mut libc::termios) {
    // For other systems (Linux, macOS, BSD, etc.), use the standard libc call.
    unsafe { libc::cfmakeraw(termios) };
}
