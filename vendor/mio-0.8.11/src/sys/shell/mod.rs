macro_rules! os_required {
    () => {
        panic!("mio must be compiled with `os-poll` to run.")
    };
}

mod selector;
pub(crate) use self::selector::{event, Event, Events, Selector};

#[cfg(not(target_os = "wasi"))]
mod waker;
#[cfg(not(target_os = "wasi"))]
pub(crate) use self::waker::Waker;

cfg_net! {
    pub(crate) mod tcp;
    pub(crate) mod udp;
    #[cfg(unix)]
    pub(crate) mod uds;
}

cfg_io_source! {
    use std::io;
    #[cfg(windows)]
    use std::os::windows::io::RawSocket;
    #[cfg(unix)]
    use std::os::unix::io::RawFd;

    #[cfg(any(windows, unix))]
    use crate::{Registry, Token, Interest};

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
    }

    #[cfg(unix)]
    impl IoSourceState {
        pub fn register(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
            _: RawFd,
        ) -> io::Result<()> {
            os_required!()
        }

        pub fn reregister(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
            _: RawFd,
        ) -> io::Result<()> {
           os_required!()
        }

        pub fn deregister(&mut self, _: &Registry, _: RawFd) -> io::Result<()> {
            os_required!()
        }
    }

    #[cfg(windows)]
    impl IoSourceState {
         pub fn register(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
            _: RawSocket,
        ) -> io::Result<()> {
            os_required!()
        }

        pub fn reregister(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
        ) -> io::Result<()> {
           os_required!()
        }

        pub fn deregister(&mut self) -> io::Result<()> {
            os_required!()
        }
    }
}
