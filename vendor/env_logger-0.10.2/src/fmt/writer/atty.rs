/*
This internal module contains the terminal detection implementation.

If the `auto-color` feature is enabled then we detect whether we're attached to a particular TTY.
Otherwise, assume we're not attached to anything. This effectively prevents styles from being
printed.
*/

#[cfg(feature = "auto-color")]
mod imp {
    use is_terminal::IsTerminal;

    pub(in crate::fmt) fn is_stdout() -> bool {
        std::io::stdout().is_terminal()
    }

    pub(in crate::fmt) fn is_stderr() -> bool {
        std::io::stderr().is_terminal()
    }
}

#[cfg(not(feature = "auto-color"))]
mod imp {
    pub(in crate::fmt) fn is_stdout() -> bool {
        false
    }

    pub(in crate::fmt) fn is_stderr() -> bool {
        false
    }
}

pub(in crate::fmt) use self::imp::*;
