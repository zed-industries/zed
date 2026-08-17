#[cfg(all(
    not(mio_unsupported_force_poll_poll),
    not(all(
        not(mio_unsupported_force_waker_pipe),
        any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )),
    not(any(target_os = "solaris", target_os = "vita")),
))]
mod fdbased {
    #[cfg(all(
        not(mio_unsupported_force_waker_pipe),
        any(target_os = "linux", target_os = "android"),
    ))]
    use crate::sys::unix::waker::eventfd::WakerInternal;
    #[cfg(any(
        mio_unsupported_force_waker_pipe,
        target_os = "aix",
        target_os = "dragonfly",
        target_os = "illumos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
    ))]
    use crate::sys::unix::waker::pipe::WakerInternal;
    use crate::sys::Selector;
    use crate::{Interest, Token};
    use std::io;
    use std::os::unix::io::AsRawFd;

    #[derive(Debug)]
    pub struct Waker {
        waker: WakerInternal,
    }

    impl Waker {
        pub fn new(selector: &Selector, token: Token) -> io::Result<Waker> {
            let waker = WakerInternal::new()?;
            selector.register(waker.as_raw_fd(), token, Interest::READABLE)?;
            Ok(Waker { waker })
        }

        pub fn wake(&self) -> io::Result<()> {
            self.waker.wake()
        }
    }
}

#[cfg(all(
    not(mio_unsupported_force_poll_poll),
    not(all(
        not(mio_unsupported_force_waker_pipe),
        any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )),
    not(any(target_os = "solaris", target_os = "vita")),
))]
pub use self::fdbased::Waker;

#[cfg(all(
    not(mio_unsupported_force_waker_pipe),
    any(target_os = "linux", target_os = "android", target_os = "espidf")
))]
mod eventfd {
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

    /// Waker backed by `eventfd`.
    ///
    /// `eventfd` is effectively an 64 bit counter. All writes must be of 8
    /// bytes (64 bits) and are converted (native endian) into an 64 bit
    /// unsigned integer and added to the count. Reads must also be 8 bytes and
    /// reset the count to 0, returning the count.
    #[derive(Debug)]
    pub struct WakerInternal {
        fd: File,
    }

    impl WakerInternal {
        pub fn new() -> io::Result<WakerInternal> {
            #[cfg(not(target_os = "espidf"))]
            let flags = libc::EFD_CLOEXEC | libc::EFD_NONBLOCK;
            // ESP-IDF is EFD_NONBLOCK by default and errors if you try to pass this flag.
            #[cfg(target_os = "espidf")]
            let flags = 0;
            let fd = syscall!(eventfd(0, flags))?;

            let file = unsafe { File::from_raw_fd(fd) };
            Ok(WakerInternal { fd: file })
        }

        pub fn wake(&self) -> io::Result<()> {
            let buf: [u8; 8] = 1u64.to_ne_bytes();
            match (&self.fd).write(&buf) {
                Ok(_) => Ok(()),
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    // Writing only blocks if the counter is going to overflow.
                    // So we'll reset the counter to 0 and wake it again.
                    self.reset()?;
                    self.wake()
                }
                Err(err) => Err(err),
            }
        }

        #[cfg(mio_unsupported_force_poll_poll)]
        pub fn ack_and_reset(&self) {
            let _ = self.reset();
        }

        /// Reset the eventfd object, only need to call this if `wake` fails.
        fn reset(&self) -> io::Result<()> {
            let mut buf: [u8; 8] = 0u64.to_ne_bytes();
            match (&self.fd).read(&mut buf) {
                Ok(_) => Ok(()),
                // If the `Waker` hasn't been awoken yet this will return a
                // `WouldBlock` error which we can safely ignore.
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => Ok(()),
                Err(err) => Err(err),
            }
        }
    }

    impl AsRawFd for WakerInternal {
        fn as_raw_fd(&self) -> RawFd {
            self.fd.as_raw_fd()
        }
    }
}

#[cfg(all(
    mio_unsupported_force_poll_poll,
    not(mio_unsupported_force_waker_pipe),
    any(target_os = "linux", target_os = "android", target_os = "espidf")
))]
pub(crate) use self::eventfd::WakerInternal;

#[cfg(all(
    not(mio_unsupported_force_waker_pipe),
    any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos",
    )
))]
mod kqueue {
    use crate::sys::Selector;
    use crate::Token;

    use std::io;

    /// Waker backed by kqueue user space notifications (`EVFILT_USER`).
    ///
    /// The implementation is fairly simple, first the kqueue must be setup to
    /// receive waker events this done by calling `Selector.setup_waker`. Next
    /// we need access to kqueue, thus we need to duplicate the file descriptor.
    /// Now waking is as simple as adding an event to the kqueue.
    #[derive(Debug)]
    pub struct Waker {
        selector: Selector,
        token: Token,
    }

    impl Waker {
        pub fn new(selector: &Selector, token: Token) -> io::Result<Waker> {
            let selector = selector.try_clone()?;
            selector.setup_waker(token)?;
            Ok(Waker { selector, token })
        }

        pub fn wake(&self) -> io::Result<()> {
            self.selector.wake(self.token)
        }
    }
}

#[cfg(all(
    not(mio_unsupported_force_waker_pipe),
    any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos",
    )
))]
pub use self::kqueue::Waker;

#[cfg(any(
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
mod pipe {
    use crate::sys::unix::pipe;
    use std::fs::File;
    use std::io::{self, Read, Write};
    use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

    /// Waker backed by a unix pipe.
    ///
    /// Waker controls both the sending and receiving ends and empties the pipe
    /// if writing to it (waking) fails.
    #[derive(Debug)]
    pub struct WakerInternal {
        sender: File,
        receiver: File,
    }

    impl WakerInternal {
        pub fn new() -> io::Result<WakerInternal> {
            let [receiver, sender] = pipe::new_raw()?;
            let sender = unsafe { File::from_raw_fd(sender) };
            let receiver = unsafe { File::from_raw_fd(receiver) };
            Ok(WakerInternal { sender, receiver })
        }

        pub fn wake(&self) -> io::Result<()> {
            // The epoll emulation on some illumos systems currently requires
            // the pipe buffer to be completely empty for an edge-triggered
            // wakeup on the pipe read side.
            #[cfg(target_os = "illumos")]
            self.empty();

            match (&self.sender).write(&[1]) {
                Ok(_) => Ok(()),
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    // The reading end is full so we'll empty the buffer and try
                    // again.
                    self.empty();
                    self.wake()
                }
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => self.wake(),
                Err(err) => Err(err),
            }
        }

        #[cfg(any(
            mio_unsupported_force_poll_poll,
            target_os = "solaris",
            target_os = "vita"
        ))]
        pub fn ack_and_reset(&self) {
            self.empty();
        }

        /// Empty the pipe's buffer, only need to call this if `wake` fails.
        /// This ignores any errors.
        fn empty(&self) {
            let mut buf = [0; 4096];
            loop {
                match (&self.receiver).read(&mut buf) {
                    Ok(n) if n > 0 => continue,
                    _ => return,
                }
            }
        }
    }

    impl AsRawFd for WakerInternal {
        fn as_raw_fd(&self) -> RawFd {
            self.receiver.as_raw_fd()
        }
    }
}

#[cfg(any(
    all(
        mio_unsupported_force_poll_poll,
        any(
            mio_unsupported_force_waker_pipe,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "illumos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )
    ),
    target_os = "solaris",
    target_os = "vita",
))]
pub(crate) use self::pipe::WakerInternal;

#[cfg(any(
    mio_unsupported_force_poll_poll,
    target_os = "solaris",
    target_os = "vita"
))]
mod poll {
    use crate::sys::Selector;
    use crate::Token;
    use std::io;

    #[derive(Debug)]
    pub struct Waker {
        selector: Selector,
        token: Token,
    }

    impl Waker {
        pub fn new(selector: &Selector, token: Token) -> io::Result<Waker> {
            Ok(Waker {
                selector: selector.try_clone()?,
                token,
            })
        }

        pub fn wake(&self) -> io::Result<()> {
            self.selector.wake(self.token)
        }
    }
}

#[cfg(any(
    mio_unsupported_force_poll_poll,
    target_os = "solaris",
    target_os = "vita"
))]
pub use self::poll::Waker;
