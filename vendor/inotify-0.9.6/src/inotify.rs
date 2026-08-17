use std::{
    ffi::CString,
    io,
    os::unix::ffi::OsStrExt,
    os::unix::io::{
        AsRawFd,
        FromRawFd,
        IntoRawFd,
        RawFd,
    },
    path::Path,
    sync::{
        atomic::AtomicBool,
        Arc,
    }
};

use inotify_sys as ffi;
use libc::{
    F_GETFL,
    F_SETFD,
    F_SETFL,
    FD_CLOEXEC,
    O_NONBLOCK,
    fcntl,
};

use crate::events::Events;
use crate::fd_guard::FdGuard;
use crate::util::read_into_buffer;
use crate::watches::{
    WatchDescriptor,
    WatchMask,
};


#[cfg(feature = "stream")]
use crate::stream::EventStream;


/// Idiomatic Rust wrapper around Linux's inotify API
///
/// `Inotify` is a wrapper around an inotify instance. It generally tries to
/// adhere to the underlying inotify API closely, while making access to it
/// safe and convenient.
///
/// Please refer to the [top-level documentation] for further details and a
/// usage example.
///
/// [top-level documentation]: index.html
#[derive(Debug)]
pub struct Inotify {
    fd: Arc<FdGuard>,
}

impl Inotify {
    /// Creates an [`Inotify`] instance
    ///
    /// Initializes an inotify instance by calling [`inotify_init1`].
    ///
    /// This method passes both flags accepted by [`inotify_init1`], not giving
    /// the user any choice in the matter, as not passing the flags would be
    /// inappropriate in the context of this wrapper:
    ///
    /// - [`IN_CLOEXEC`] prevents leaking file descriptors to other processes.
    /// - [`IN_NONBLOCK`] controls the blocking behavior of the inotify API,
    ///   which is entirely managed by this wrapper.
    ///
    /// # Errors
    ///
    /// Directly returns the error from the call to [`inotify_init1`], without
    /// adding any error conditions of its own.
    ///
    /// # Examples
    ///
    /// ```
    /// use inotify::Inotify;
    ///
    /// let inotify = Inotify::init()
    ///     .expect("Failed to initialize an inotify instance");
    /// ```
    ///
    /// [`Inotify`]: struct.Inotify.html
    /// [`inotify_init1`]: ../inotify_sys/fn.inotify_init1.html
    /// [`IN_CLOEXEC`]: ../inotify_sys/constant.IN_CLOEXEC.html
    /// [`IN_NONBLOCK`]: ../inotify_sys/constant.IN_NONBLOCK.html
    pub fn init() -> io::Result<Inotify> {
        // Initialize inotify and set CLOEXEC and NONBLOCK flags.
        //
        // NONBLOCK is needed, because `Inotify` manages blocking behavior for
        // the API consumer, and the way we do that is to make everything non-
        // blocking by default and later override that as required.
        //
        // CLOEXEC prevents leaking file descriptors to processes executed by
        // this process and seems to be a best practice. I don't grasp this
        // issue completely and failed to find any authoritative sources on the
        // topic. There's some discussion in the open(2) and fcntl(2) man pages,
        // but I didn't find that helpful in understanding the issue of leaked
        // file descriptors. For what it's worth, there's a Rust issue about
        // this:
        // https://github.com/rust-lang/rust/issues/12148
        let fd = unsafe {
            let fd = ffi::inotify_init();
            if fd == -1 {
                return Err(io::Error::last_os_error());
            }
            if fcntl(fd, F_SETFD, FD_CLOEXEC) == -1 {
                return Err(io::Error::last_os_error());
            }
            if fcntl(fd, F_SETFL, O_NONBLOCK) == -1 {
                return Err(io::Error::last_os_error());
            }
            fd
        };

        Ok(Inotify {
            fd: Arc::new(FdGuard {
                fd,
                close_on_drop: AtomicBool::new(true),
            }),
        })
    }

    /// Adds or updates a watch for the given path
    ///
    /// Adds a new watch or updates an existing one for the file referred to by
    /// `path`. Returns a watch descriptor that can be used to refer to this
    /// watch later.
    ///
    /// The `mask` argument defines what kind of changes the file should be
    /// watched for, and how to do that. See the documentation of [`WatchMask`]
    /// for details.
    ///
    /// If this method is used to add a new watch, a new [`WatchDescriptor`] is
    /// returned. If it is used to update an existing watch, a
    /// [`WatchDescriptor`] that equals the previously returned
    /// [`WatchDescriptor`] for that watch is returned instead.
    ///
    /// Under the hood, this method just calls [`inotify_add_watch`] and does
    /// some trivial translation between the types on the Rust side and the C
    /// side.
    ///
    /// # Attention: Updating watches and hardlinks
    ///
    /// As mentioned above, this method can be used to update an existing watch.
    /// This is usually done by calling this method with the same `path`
    /// argument that it has been called with before. But less obviously, it can
    /// also happen if the method is called with a different path that happens
    /// to link to the same inode.
    ///
    /// You can detect this by keeping track of [`WatchDescriptor`]s and the
    /// paths they have been returned for. If the same [`WatchDescriptor`] is
    /// returned for a different path (and you haven't freed the
    /// [`WatchDescriptor`] by removing the watch), you know you have two paths
    /// pointing to the same inode, being watched by the same watch.
    ///
    /// # Errors
    ///
    /// Directly returns the error from the call to
    /// [`inotify_add_watch`][`inotify_add_watch`] (translated into an
    /// `io::Error`), without adding any error conditions of
    /// its own.
    ///
    /// # Examples
    ///
    /// ```
    /// use inotify::{
    ///     Inotify,
    ///     WatchMask,
    /// };
    ///
    /// let mut inotify = Inotify::init()
    ///     .expect("Failed to initialize an inotify instance");
    ///
    /// # // Create a temporary file, so `add_watch` won't return an error.
    /// # use std::fs::File;
    /// # File::create("/tmp/inotify-rs-test-file")
    /// #     .expect("Failed to create test file");
    /// #
    /// inotify.add_watch("/tmp/inotify-rs-test-file", WatchMask::MODIFY)
    ///     .expect("Failed to add file watch");
    ///
    /// // Handle events for the file here
    /// ```
    ///
    /// [`inotify_add_watch`]: ../inotify_sys/fn.inotify_add_watch.html
    /// [`WatchMask`]: struct.WatchMask.html
    /// [`WatchDescriptor`]: struct.WatchDescriptor.html
    pub fn add_watch<P>(&mut self, path: P, mask: WatchMask)
        -> io::Result<WatchDescriptor>
        where P: AsRef<Path>
    {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;

        let wd = unsafe {
            ffi::inotify_add_watch(
                **self.fd,
                path.as_ptr() as *const _,
                mask.bits(),
            )
        };

        match wd {
            -1 => Err(io::Error::last_os_error()),
            _  => Ok(WatchDescriptor{ id: wd, fd: Arc::downgrade(&self.fd) }),
        }
    }

    /// Stops watching a file
    ///
    /// Removes the watch represented by the provided [`WatchDescriptor`] by
    /// calling [`inotify_rm_watch`]. [`WatchDescriptor`]s can be obtained via
    /// [`Inotify::add_watch`], or from the `wd` field of [`Event`].
    ///
    /// # Errors
    ///
    /// Directly returns the error from the call to [`inotify_rm_watch`].
    /// Returns an [`io::Error`] with [`ErrorKind`]`::InvalidInput`, if the given
    /// [`WatchDescriptor`] did not originate from this [`Inotify`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use inotify::Inotify;
    ///
    /// let mut inotify = Inotify::init()
    ///     .expect("Failed to initialize an inotify instance");
    ///
    /// # // Create a temporary file, so `add_watch` won't return an error.
    /// # use std::fs::File;
    /// # let mut test_file = File::create("/tmp/inotify-rs-test-file")
    /// #     .expect("Failed to create test file");
    /// #
    /// # // Add a watch and modify the file, so the code below doesn't block
    /// # // forever.
    /// # use inotify::WatchMask;
    /// # inotify.add_watch("/tmp/inotify-rs-test-file", WatchMask::MODIFY)
    /// #     .expect("Failed to add file watch");
    /// # use std::io::Write;
    /// # write!(&mut test_file, "something\n")
    /// #     .expect("Failed to write something to test file");
    /// #
    /// let mut buffer = [0; 1024];
    /// let events = inotify
    ///     .read_events_blocking(&mut buffer)
    ///     .expect("Error while waiting for events");
    ///
    /// for event in events {
    ///     inotify.rm_watch(event.wd);
    /// }
    /// ```
    ///
    /// [`WatchDescriptor`]: struct.WatchDescriptor.html
    /// [`inotify_rm_watch`]: ../inotify_sys/fn.inotify_rm_watch.html
    /// [`Inotify::add_watch`]: struct.Inotify.html#method.add_watch
    /// [`Event`]: struct.Event.html
    /// [`Inotify`]: struct.Inotify.html
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    /// [`ErrorKind`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html
    pub fn rm_watch(&mut self, wd: WatchDescriptor) -> io::Result<()> {
        if wd.fd.upgrade().as_ref() != Some(&self.fd) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid WatchDescriptor",
            ));
        }

        let result = unsafe { ffi::inotify_rm_watch(**self.fd, wd.id) };
        match result {
            0  => Ok(()),
            -1 => Err(io::Error::last_os_error()),
            _  => panic!(
                "unexpected return code from inotify_rm_watch ({})", result)
        }
    }

    /// Waits until events are available, then returns them
    ///
    /// Blocks the current thread until at least one event is available. If this
    /// is not desirable, please consider [`Inotify::read_events`].
    ///
    /// This method calls [`Inotify::read_events`] internally and behaves
    /// essentially the same, apart from the blocking behavior. Please refer to
    /// the documentation of [`Inotify::read_events`] for more information.
    ///
    /// [`Inotify::read_events`]: struct.Inotify.html#method.read_events
    /// [`read`]: ../libc/fn.read.html
    pub fn read_events_blocking<'a>(&mut self, buffer: &'a mut [u8])
        -> io::Result<Events<'a>>
    {
        unsafe {
            let res = fcntl(**self.fd, F_GETFL);
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
            if fcntl(**self.fd, F_SETFL, res & !O_NONBLOCK) == -1 {
                return Err(io::Error::last_os_error());
            }
        };
        let result = self.read_events(buffer);
        unsafe {
            let res = fcntl(**self.fd, F_GETFL);
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
            if fcntl(**self.fd, F_SETFL, res | O_NONBLOCK) == -1 {
                return Err(io::Error::last_os_error());
            }
        };

        result
    }

    /// Returns one buffer's worth of available events
    ///
    /// Reads as many events as possible into `buffer`, and returns an iterator
    /// over them. If no events are available, an iterator is still returned. If
    /// you need a method that will block until at least one event is available,
    /// please consider [`read_events_blocking`].
    ///
    /// Please note that inotify will merge identical successive unread events 
    /// into a single event. This means this method can not be used to count the 
    /// number of file system events.
    ///
    /// The `buffer` argument, as the name indicates, is used as a buffer for
    /// the inotify events. Its contents may be overwritten.
    ///
    /// # Errors
    ///
    /// This function directly returns all errors from the call to [`read`]
    /// (except EGAIN/EWOULDBLOCK, which result in an empty iterator). In
    /// addition, [`ErrorKind::UnexpectedEof`] is returned, if the call to
    /// [`read`] returns `0`, signaling end-of-file.
    ///
    /// If `buffer` is too small, this will result in an error with
    /// [`ErrorKind::InvalidInput`]. On very old Linux kernels,
    /// [`ErrorKind::UnexpectedEof`] will be returned instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use inotify::Inotify;
    ///
    /// let mut inotify = Inotify::init()
    ///     .expect("Failed to initialize an inotify instance");
    ///
    /// let mut buffer = [0; 1024];
    /// let events = inotify.read_events(&mut buffer)
    ///     .expect("Error while reading events");
    ///
    /// for event in events {
    ///     // Handle event
    /// }
    /// ```
    ///
    /// [`read_events_blocking`]: struct.Inotify.html#method.read_events_blocking
    /// [`read`]: ../libc/fn.read.html
    /// [`ErrorKind::UnexpectedEof`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.UnexpectedEof
    /// [`ErrorKind::InvalidInput`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidInput
    pub fn read_events<'a>(&mut self, buffer: &'a mut [u8])
        -> io::Result<Events<'a>>
    {
        let num_bytes = read_into_buffer(**self.fd, buffer);

        let num_bytes = match num_bytes {
            0 => {
                return Err(
                    io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "`read` return `0`, signaling end-of-file"
                    )
                );
            }
            -1 => {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::WouldBlock {
                    return Ok(Events::new(Arc::downgrade(&self.fd), buffer, 0));
                }
                else {
                    return Err(error);
                }
            },
            _ if num_bytes < 0 => {
                panic!("{} {} {} {} {} {}",
                    "Unexpected return value from `read`. Received a negative",
                    "value that was not `-1`. According to the `read` man page",
                    "this shouldn't happen, as either `-1` is returned on",
                    "error, `0` on end-of-file, or a positive value for the",
                    "number of bytes read. Returned value:",
                    num_bytes,
                );
            }
            _ => {
                // The value returned by `read` should be `isize`. Let's quickly
                // verify this with the following assignment, so we can be sure
                // our cast below is valid.
                let num_bytes: isize = num_bytes;

                // The type returned by `read` is `isize`, and we've ruled out
                // all negative values with the match arms above. This means we
                // can safely cast to `usize`.
                debug_assert!(num_bytes > 0);
                num_bytes as usize
            }
        };

        Ok(Events::new(Arc::downgrade(&self.fd), buffer, num_bytes))
    }

    /// Create a stream which collects events
    ///
    /// Returns a `Stream` over all events that are available. This stream is an
    /// infinite source of events.
    ///
    /// An internal buffer which can hold the largest possible event is used.
    #[cfg(feature = "stream")]
    pub fn event_stream<T>(&mut self, buffer: T)
        -> io::Result<EventStream<T>>
    where
        T: AsMut<[u8]> + AsRef<[u8]>,
    {
        EventStream::new(self.fd.clone(), buffer)
    }

    /// Closes the inotify instance
    ///
    /// Closes the file descriptor referring to the inotify instance. The user
    /// usually doesn't have to call this function, as the underlying inotify
    /// instance is closed automatically, when [`Inotify`] is dropped.
    ///
    /// # Errors
    ///
    /// Directly returns the error from the call to [`close`], without adding any
    /// error conditions of its own.
    ///
    /// # Examples
    ///
    /// ```
    /// use inotify::Inotify;
    ///
    /// let mut inotify = Inotify::init()
    ///     .expect("Failed to initialize an inotify instance");
    ///
    /// inotify.close()
    ///     .expect("Failed to close inotify instance");
    /// ```
    ///
    /// [`Inotify`]: struct.Inotify.html
    /// [`close`]: ../libc/fn.close.html
    pub fn close(self) -> io::Result<()> {
        // `self` will be dropped when this method returns. If this is the only
        // owner of `fd`, the `Arc` will also be dropped. The `Drop`
        // implementation for `FdGuard` will attempt to close the file descriptor
        // again, unless this flag here is cleared.
        self.fd.should_not_close();

        match unsafe { ffi::close(**self.fd) } {
            0 => Ok(()),
            _ => Err(io::Error::last_os_error()),
        }
    }
}

impl AsRawFd for Inotify {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl FromRawFd for Inotify {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Inotify {
            fd: Arc::new(FdGuard::from_raw_fd(fd))
        }
    }
}

impl IntoRawFd for Inotify {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.fd.should_not_close();
        self.fd.fd
    }
}
