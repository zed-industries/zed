use std::fmt::Display;
use std::io;

use crate::kb::Key;
use crate::term::Term;

pub(crate) use crate::common_term::*;

pub(crate) const DEFAULT_WIDTH: u16 = 80;

#[inline]
pub(crate) fn is_a_terminal(_out: &Term) -> bool {
    #[cfg(all(target_os = "wasi", target_env = "p1"))]
    {
        use std::os::fd::AsRawFd;
        unsafe { libc::isatty(_out.as_raw_fd()) != 0 }
    }
    #[cfg(not(all(target_os = "wasi", target_env = "p1")))]
    {
        false
    }
}

#[inline]
pub(crate) fn is_a_color_terminal(_out: &Term) -> bool {
    // We currently never report color terminals.  For discussion see
    // the issue in the WASI repo: https://github.com/WebAssembly/WASI/issues/162
    false
}

#[inline]
pub(crate) fn is_a_true_color_terminal(_out: &Term) -> bool {
    false
}

#[inline]
pub(crate) fn terminal_size(_out: &Term) -> Option<(u16, u16)> {
    None
}

pub(crate) fn read_secure() -> io::Result<String> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "unsupported operation",
    ))
}

pub(crate) fn read_single_key(_ctrlc_key: bool) -> io::Result<Key> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "unsupported operation",
    ))
}

#[inline]
pub(crate) fn wants_emoji() -> bool {
    false
}

pub(crate) fn set_title<T: Display>(_title: T) {}
