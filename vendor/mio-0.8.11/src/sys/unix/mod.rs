/// Helper macro to execute a system call that returns an `io::Result`.
//
// Macro must be defined before any modules that uses them.
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

cfg_os_poll! {
    mod selector;
    pub(crate) use self::selector::{event, Event, Events, Selector};

    mod sourcefd;
    #[cfg(feature = "os-ext")]
    pub use self::sourcefd::SourceFd;

    mod waker;
    pub(crate) use self::waker::Waker;

    cfg_net! {
        mod net;

        pub(crate) mod tcp;
        pub(crate) mod udp;
        pub(crate) mod uds;
        pub use self::uds::SocketAddr;
    }

    cfg_io_source! {
        // Both `kqueue` and `epoll` don't need to hold any user space state.
        #[cfg(not(any(mio_unsupported_force_poll_poll, target_os = "solaris", target_os = "vita")))]
        mod stateless_io_source {
            use std::io;
            use std::os::unix::io::RawFd;
            use crate::Registry;
            use crate::Token;
            use crate::Interest;

            pub(crate) struct IoSourceState;

            impl IoSourceState {
                pub fn new() -> IoSourceState {
                    IoSourceState
                }

                pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
                where
                    F: FnOnce(&T) -> io::Result<R>,
                {
                    // We don't hold state, so we can just call the function and
                    // return.
                    f(io)
                }

                pub fn register(
                    &mut self,
                    registry: &Registry,
                    token: Token,
                    interests: Interest,
                    fd: RawFd,
                ) -> io::Result<()> {
                    // Pass through, we don't have any state
                    registry.selector().register(fd, token, interests)
                }

                pub fn reregister(
                    &mut self,
                    registry: &Registry,
                    token: Token,
                    interests: Interest,
                    fd: RawFd,
                ) -> io::Result<()> {
                    // Pass through, we don't have any state
                    registry.selector().reregister(fd, token, interests)
                }

                pub fn deregister(&mut self, registry: &Registry, fd: RawFd) -> io::Result<()> {
                    // Pass through, we don't have any state
                    registry.selector().deregister(fd)
                }
            }
        }

        #[cfg(not(any(mio_unsupported_force_poll_poll, target_os = "solaris",target_os = "vita")))]
        pub(crate) use self::stateless_io_source::IoSourceState;

        #[cfg(any(mio_unsupported_force_poll_poll, target_os = "solaris", target_os = "vita"))]
        pub(crate) use self::selector::IoSourceState;
    }

    #[cfg(any(
        // For the public `pipe` module, must match `cfg_os_ext` macro.
        feature = "os-ext",
        // For the `Waker` type based on a pipe.
        mio_unsupported_force_waker_pipe,
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "illumos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris",
        target_os = "vita",
    ))]
    pub(crate) mod pipe;
}

cfg_not_os_poll! {
    cfg_net! {
        mod uds;
        pub use self::uds::SocketAddr;
    }

    cfg_any_os_ext! {
        mod sourcefd;
        #[cfg(feature = "os-ext")]
        pub use self::sourcefd::SourceFd;
    }
}
