//! Asynchronous signal handling.
//!
//! This crate provides the [`Signals`] type, which can be used to listen for POSIX signals asynchronously.
//! It can be seen as an asynchronous version of [`signal_hook::iterator::Signals`].
//!
//! [`signal_hook::iterator::Signals`]: https://docs.rs/signal-hook/latest/signal_hook/iterator/struct.Signals.html
//!
//! # Implementation
//!
//! This crate uses the [`signal_hook_registry`] crate to register a listener for each signal. That
//! listener will then send a message through a Unix socket to the [`Signals`] type, which will
//! receive it and notify the user. Asynchronous notification is done through the [`async-io`] crate.
//!
//! Note that the internal pipe has a limited capacity. Once it has reached capacity, additional
//! signals will be dropped.
//!
//! On Windows, a different implementation that only supports `SIGINT` is used. This implementation
//! uses a channel to notify the user.
//!
//! [`signal_hook_registry`]: https://crates.io/crates/signal-hook-registry
//! [`async-io`]: https://crates.io/crates/async-io
//!
//! # Examples
//!
//! ```no_run
//! use async_signal::{Signal, Signals};
//! use futures_lite::prelude::*;
//! use signal_hook::low_level;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # async_io::block_on(async {
//! // Register the signals we want to receive.
//! let mut signals = Signals::new(&[
//!     Signal::Term,
//!     Signal::Quit,
//!     Signal::Int,
//! ])?;
//!
//! // Wait for a signal to be received.
//! while let Some(signal) = signals.next().await {
//!     // Print the signal.
//!     eprintln!("Received signal {:?}", signal);
//!
//!     // After printing it, do whatever the signal was supposed to do in the first place.
//!     low_level::emulate_default_handler(signal.unwrap() as i32).unwrap();
//! }
//! # Ok(())
//! # })
//! # }
//! ```

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod channel;
        use channel as sys;
    } else {
        mod pipe;
        use pipe as sys;
    }
}

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        use signal_hook_registry as registry;
    } else if #[cfg(windows)] {
        mod windows_registry;
        use windows_registry as registry;
    }
}

use futures_core::ready;
use futures_core::stream::Stream;
use registry::SigId;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, RawFd};

mod signum {
    pub(crate) use std::os::raw::c_int;

    macro_rules! sig {
        ($rustix_name:ident, $raw_value:literal) => {{
            #[cfg(unix)]
            {
                rustix::process::Signal::$rustix_name.as_raw()
            }

            #[cfg(windows)]
            {
                $raw_value
            }
        }};
    }

    // Define these ourselves.
    pub const SIGHUP: c_int = sig!(HUP, 1);
    pub const SIGINT: c_int = sig!(INT, 2);
    pub const SIGQUIT: c_int = sig!(QUIT, 3);
    pub const SIGILL: c_int = sig!(ILL, 4);
    pub const SIGTRAP: c_int = sig!(TRAP, 5);
    pub const SIGABRT: c_int = sig!(ABORT, 6);
    pub const SIGFPE: c_int = sig!(FPE, 8);
    pub const SIGKILL: c_int = sig!(KILL, 9);
    pub const SIGSEGV: c_int = sig!(SEGV, 11);
    pub const SIGPIPE: c_int = sig!(PIPE, 13);
    pub const SIGALRM: c_int = sig!(ALARM, 14);
    pub const SIGTERM: c_int = sig!(TERM, 15);
    pub const SIGTTIN: c_int = sig!(TTIN, 21);
    pub const SIGTTOU: c_int = sig!(TTOU, 22);
    pub const SIGXCPU: c_int = sig!(XCPU, 24);
    pub const SIGXFSZ: c_int = sig!(XFSZ, 25);
    pub const SIGVTALRM: c_int = sig!(VTALARM, 26);
    pub const SIGPROF: c_int = sig!(PROF, 27);
    pub const SIGWINCH: c_int = sig!(WINCH, 28);
    pub const SIGCHLD: c_int = sig!(CHILD, 17);
    pub const SIGBUS: c_int = sig!(BUS, 7);
    pub const SIGUSR1: c_int = sig!(USR1, 10);
    pub const SIGUSR2: c_int = sig!(USR2, 12);
    pub const SIGCONT: c_int = sig!(CONT, 18);
    pub const SIGSTOP: c_int = sig!(STOP, 19);
    pub const SIGTSTP: c_int = sig!(TSTP, 20);
    pub const SIGURG: c_int = sig!(URG, 23);
    pub const SIGIO: c_int = sig!(IO, 29);
    pub const SIGSYS: c_int = sig!(SYS, 31);
}

macro_rules! define_signal_enum {
    (
        $(#[$outer:meta])*
        pub enum Signal {
            $(
                $(#[$inner:meta])*
                $name:ident = $value:ident,
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
        #[repr(i32)]
        pub enum Signal {
            $(
                $(#[$inner])*
                $name = signum::$value,
            )*
        }

        impl Signal {
            /// Returns the signal number.
            fn number(self) -> std::os::raw::c_int {
                match self {
                    $(
                        Signal::$name => signum::$value,
                    )*
                }
            }

            /// Parse a signal from its number.
            #[cfg(unix)]
            fn from_number(number: std::os::raw::c_int) -> Option<Self> {
                match number {
                    $(
                        signum::$value => Some(Signal::$name),
                    )*
                    _ => None,
                }
            }
        }
    }
}

define_signal_enum! {
    // Copied from https://github.com/bytecodealliance/rustix/blob/main/src/backend/linux_raw/process/types.rs#L81-L161

    /// The signal types that we are able to listen for.
    pub enum Signal {
        /// `SIGHUP`
        Hup = SIGHUP,
        /// `SIGINT`
        Int = SIGINT,
        /// `SIGQUIT`
        Quit = SIGQUIT,
        /// `SIGILL`
        Ill = SIGILL,
        /// `SIGTRAP`
        Trap = SIGTRAP,
        /// `SIGABRT`, aka `SIGIOT`
        #[doc(alias = "Iot")]
        #[doc(alias = "Abrt")]
        Abort = SIGABRT,
        /// `SIGBUS`
        Bus = SIGBUS,
        /// `SIGFPE`
        Fpe = SIGFPE,
        /// `SIGKILL`
        Kill = SIGKILL,
        /// `SIGUSR1`
        Usr1 = SIGUSR1,
        /// `SIGSEGV`
        Segv = SIGSEGV,
        /// `SIGUSR2`
        Usr2 = SIGUSR2,
        /// `SIGPIPE`
        Pipe = SIGPIPE,
        /// `SIGALRM`
        #[doc(alias = "Alrm")]
        Alarm = SIGALRM,
        /// `SIGTERM`
        Term = SIGTERM,
        /// `SIGCHLD`
        #[doc(alias = "Chld")]
        Child = SIGCHLD,
        /// `SIGCONT`
        Cont = SIGCONT,
        /// `SIGSTOP`
        Stop = SIGSTOP,
        /// `SIGTSTP`
        Tstp = SIGTSTP,
        /// `SIGTTIN`
        Ttin = SIGTTIN,
        /// `SIGTTOU`
        Ttou = SIGTTOU,
        /// `SIGURG`
        Urg = SIGURG,
        /// `SIGXCPU`
        Xcpu = SIGXCPU,
        /// `SIGXFSZ`
        Xfsz = SIGXFSZ,
        /// `SIGVTALRM`
        #[doc(alias = "Vtalrm")]
        Vtalarm = SIGVTALRM,
        /// `SIGPROF`
        Prof = SIGPROF,
        /// `SIGWINCH`
        Winch = SIGWINCH,
        /// `SIGIO`, aka `SIGPOLL`
        #[doc(alias = "Poll")]
        Io = SIGIO,
        /// `SIGSYS`, aka `SIGUNUSED`
        #[doc(alias = "Unused")]
        Sys = SIGSYS,
    }
}

/// Wait for a specific set of signals.
///
/// See the [module-level documentation](index.html) for more details.
pub struct Signals {
    /// The strategy used to read the signals.
    notifier: sys::Notifier,

    /// The map between signal numbers and signal IDs.
    signal_ids: HashMap<Signal, SigId>,
}

impl Drop for Signals {
    fn drop(&mut self) {
        for signal in self.signal_ids.values() {
            registry::unregister(*signal);
        }
    }
}

impl fmt::Debug for Signals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct RegisteredSignals<'a>(&'a HashMap<Signal, SigId>);

        impl fmt::Debug for RegisteredSignals<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_set().entries(self.0.keys()).finish()
            }
        }

        f.debug_struct("Signals")
            .field("notifier", &self.notifier)
            .field("signal_ids", &RegisteredSignals(&self.signal_ids))
            .finish()
    }
}

impl Signals {
    /// Create a new `Signals` instance with a set of signals.
    pub fn new<B>(signals: impl IntoIterator<Item = B>) -> io::Result<Self>
    where
        B: Borrow<Signal>,
    {
        let mut this = Self {
            notifier: sys::Notifier::new()?,
            signal_ids: HashMap::new(),
        };

        // Add the signals to the set of signals to wait for.
        this.add_signals(signals)?;

        Ok(this)
    }

    /// Add signals to the set of signals to wait for.
    ///
    /// One signal cannot be added twice. If a signal that has already been added is passed to this
    /// method, it will be ignored.
    ///
    /// Registering a signal prevents the default behavior of that signal from occurring. For
    /// example, if you register `SIGINT`, pressing `Ctrl+C` will no longer terminate the process.
    /// To run the default signal handler, use [`signal_hook::low_level::emulate_default_handler`]
    /// instead.
    ///
    /// [`signal_hook::low_level::emulate_default_handler`]: https://docs.rs/signal-hook/latest/signal_hook/low_level/fn.emulate_default_handler.html
    pub fn add_signals<B>(&mut self, signals: impl IntoIterator<Item = B>) -> io::Result<()>
    where
        B: Borrow<Signal>,
    {
        for signal in signals {
            let signal = signal.borrow();

            // If we've already registered this signal, skip it.
            if self.signal_ids.contains_key(signal) {
                continue;
            }

            // Get the closure to call when the signal is received.
            let closure = self.notifier.add_signal(*signal)?;

            let id = unsafe {
                // SAFETY: Closure is guaranteed to be signal-safe.
                registry::register(signal.number(), closure)?
            };

            // Add the signal ID to the map.
            self.signal_ids.insert(*signal, id);
        }

        Ok(())
    }

    /// Remove signals from the set of signals to wait for.
    ///
    /// This function can be used to opt out of listening to signals previously registered via
    /// [`add_signals`](Self::add_signals) or [`new`](Self::new). If a signal that has not been
    /// registered is passed to this method, it will be ignored.
    pub fn remove_signals<B>(&mut self, signals: impl IntoIterator<Item = B>) -> io::Result<()>
    where
        B: Borrow<Signal>,
    {
        for signal in signals {
            let signal = signal.borrow();

            // If we haven't registered this signal, skip it.
            let id = match self.signal_ids.remove(signal) {
                Some(id) => id,
                None => continue,
            };

            // Remove the signal from the notifier.
            self.notifier.remove_signal(*signal)?;

            // Use `signal-hook-registry` to unregister the signal.
            registry::unregister(id);
        }

        Ok(())
    }
}

#[cfg(unix)]
impl AsRawFd for Signals {
    fn as_raw_fd(&self) -> RawFd {
        self.notifier.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for Signals {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.notifier.as_fd()
    }
}

impl Unpin for Signals {}

impl Stream for Signals {
    type Item = io::Result<Signal>;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut &*self).poll_next(cx)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // This stream is expected to never end.
        (usize::MAX, None)
    }
}

impl Stream for &Signals {
    type Item = io::Result<Signal>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let signal = ready!(self.notifier.poll_next(cx))?;
        Poll::Ready(Some(Ok(signal)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // This stream is expected to never end.
        (usize::MAX, None)
    }
}
