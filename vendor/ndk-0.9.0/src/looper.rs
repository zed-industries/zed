//! Bindings for [`ALooper`]
//!
//! In Android, [`ALooper`]s are inherently thread-local.  Due to this, there are two different
//! [`ALooper`] interfaces exposed in this module:
//!
//! * [`ThreadLooper`], which has methods for the operations performable with a looper in one's own
//!   thread; and
//! * [`ForeignLooper`], which has methods for the operations performable with any thread's looper.
//!
//! [`ALooper`]: https://developer.android.com/ndk/reference/group/looper#alooper

use std::mem::ManuallyDrop;
use std::os::{
    fd::{AsRawFd, BorrowedFd, RawFd},
    raw::c_void,
};
use std::ptr;
use std::time::Duration;
use thiserror::Error;

use crate::utils::abort_on_panic;

/// A thread-local native [`ALooper *`].  This promises that there is a looper associated with the
/// current thread.
///
/// [`ALooper *`]: https://developer.android.com/ndk/reference/group/looper#alooper
#[derive(Debug)]
pub struct ThreadLooper {
    _marker: std::marker::PhantomData<*mut ()>, // Not send or sync
    foreign: ForeignLooper,
}

bitflags::bitflags! {
    /// Flags for file descriptor events that a looper can monitor.
    ///
    /// These flag bits can be combined to monitor multiple events at once.
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FdEvent : u32 {
        /// The file descriptor is available for read operations.
        #[doc(alias = "ALOOPER_EVENT_INPUT")]
        const INPUT = ffi::ALOOPER_EVENT_INPUT;
        /// The file descriptor is available for write operations.
        #[doc(alias = "ALOOPER_EVENT_OUTPUT")]
        const OUTPUT = ffi::ALOOPER_EVENT_OUTPUT;
        /// The file descriptor has encountered an error condition.
        ///
        /// The looper always sends notifications about errors; it is not necessary to specify this
        /// event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_ERROR")]
        const ERROR = ffi::ALOOPER_EVENT_ERROR;
        /// The file descriptor was hung up.
        ///
        /// For example, indicates that the remote end of a pipe or socket was closed.
        ///
        /// The looper always sends notifications about hangups; it is not necessary to specify this
        /// event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_HANGUP")]
        const HANGUP = ffi::ALOOPER_EVENT_HANGUP;
        /// The file descriptor is invalid.
        ///
        /// For example, the file descriptor was closed prematurely.
        ///
        /// The looper always sends notifications about invalid file descriptors; it is not
        /// necessary to specify this event flag in the requested event set.
        #[doc(alias = "ALOOPER_EVENT_INVALID")]
        const INVALID = ffi::ALOOPER_EVENT_INVALID;

        // https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
        const _ = !0;
    }
}

/// The poll result from a [`ThreadLooper`].
#[derive(Debug)]
pub enum Poll<'fd> {
    /// This looper was woken using [`ForeignLooper::wake()`]
    Wake,
    /// For [`ThreadLooper::poll_once*()`][ThreadLooper::poll_once()], an event was received and processed using a callback.
    Callback,
    /// For [`ThreadLooper::poll_*_timeout()`][ThreadLooper::poll_once_timeout()], the requested timeout was reached before any events.
    Timeout,
    /// An event was received
    Event {
        ident: i32,
        /// # Safety
        /// The caller should guarantee that this file descriptor remains open after it was added
        /// via [`ForeignLooper::add_fd()`] or [`ForeignLooper::add_fd_with_callback()`].
        fd: BorrowedFd<'fd>,
        events: FdEvent,
        data: *mut c_void,
    },
}

#[derive(Debug, Copy, Clone, Error)]
#[error("Android Looper error")]
pub struct LooperError;

impl ThreadLooper {
    /// Prepares a looper for the current thread and returns it
    pub fn prepare() -> Self {
        unsafe {
            let ptr = ffi::ALooper_prepare(ffi::ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as _);
            let foreign = ForeignLooper::from_ptr(ptr::NonNull::new(ptr).expect("looper non null"));
            Self {
                _marker: std::marker::PhantomData,
                foreign,
            }
        }
    }

    /// Returns the looper associated with the current thread, if any.
    pub fn for_thread() -> Option<Self> {
        Some(Self {
            _marker: std::marker::PhantomData,
            foreign: ForeignLooper::for_thread()?,
        })
    }

    /// Polls the looper, blocking on processing an event, but with a timeout in milliseconds.
    /// Give a timeout of `0` to make this non-blocking.
    fn poll_once_ms(&self, ms: i32) -> Result<Poll<'_>, LooperError> {
        let mut fd = -1;
        let mut events = -1;
        let mut data: *mut c_void = ptr::null_mut();
        match unsafe { ffi::ALooper_pollOnce(ms, &mut fd, &mut events, &mut data) } {
            ffi::ALOOPER_POLL_WAKE => Ok(Poll::Wake),
            ffi::ALOOPER_POLL_CALLBACK => Ok(Poll::Callback),
            ffi::ALOOPER_POLL_TIMEOUT => Ok(Poll::Timeout),
            ffi::ALOOPER_POLL_ERROR => Err(LooperError),
            ident if ident >= 0 => Ok(Poll::Event {
                ident,
                // SAFETY: Even though this FD at least shouldn't outlive self, a user could have
                // closed it after calling add_fd or add_fd_with_callback.
                fd: unsafe { BorrowedFd::borrow_raw(fd) },
                events: FdEvent::from_bits(events as u32)
                    .expect("poll event contains unknown bits"),
                data,
            }),
            _ => unreachable!(),
        }
    }

    /// Polls the looper, blocking on processing an event.
    #[inline]
    pub fn poll_once(&self) -> Result<Poll<'_>, LooperError> {
        self.poll_once_ms(-1)
    }

    /// Polls the looper, blocking on processing an event, but with a timeout.  Give a timeout of
    /// [`Duration::ZERO`] to make this non-blocking.
    ///
    /// It panics if the timeout is larger than expressible as an [`i32`] of milliseconds (roughly 25
    /// days).
    #[inline]
    pub fn poll_once_timeout(&self, timeout: Duration) -> Result<Poll<'_>, LooperError> {
        self.poll_once_ms(
            timeout
                .as_millis()
                .try_into()
                .expect("Supplied timeout is too large"),
        )
    }

    /// Repeatedly polls the looper, blocking on processing an event, but with a timeout in
    /// milliseconds.  Give a timeout of `0` to make this non-blocking.
    ///
    /// This function will never return [`Poll::Callback`].
    fn poll_all_ms(&self, ms: i32) -> Result<Poll<'_>, LooperError> {
        let mut fd = -1;
        let mut events = -1;
        let mut data: *mut c_void = ptr::null_mut();
        match unsafe { ffi::ALooper_pollAll(ms, &mut fd, &mut events, &mut data) } {
            ffi::ALOOPER_POLL_WAKE => Ok(Poll::Wake),
            ffi::ALOOPER_POLL_TIMEOUT => Ok(Poll::Timeout),
            ffi::ALOOPER_POLL_ERROR => Err(LooperError),
            ident if ident >= 0 => Ok(Poll::Event {
                ident,
                // SAFETY: Even though this FD at least shouldn't outlive self, a user could have
                // closed it after calling add_fd or add_fd_with_callback.
                fd: unsafe { BorrowedFd::borrow_raw(fd) },
                events: FdEvent::from_bits(events as u32)
                    .expect("poll event contains unknown bits"),
                data,
            }),
            _ => unreachable!(),
        }
    }

    /// Repeatedly polls the looper, blocking on processing an event.
    ///
    /// This function will never return [`Poll::Callback`].
    #[inline]
    pub fn poll_all(&self) -> Result<Poll<'_>, LooperError> {
        self.poll_all_ms(-1)
    }

    /// Repeatedly polls the looper, blocking on processing an event, but with a timeout.  Give a
    /// timeout of [`Duration::ZERO`] to make this non-blocking.
    ///
    /// This function will never return [`Poll::Callback`].
    ///
    /// It panics if the timeout is larger than expressible as an [`i32`] of milliseconds (roughly 25
    /// days).
    #[inline]
    pub fn poll_all_timeout(&self, timeout: Duration) -> Result<Poll<'_>, LooperError> {
        self.poll_all_ms(
            timeout
                .as_millis()
                .try_into()
                .expect("Supplied timeout is too large"),
        )
    }

    /// Adds a file descriptor to be polled, with a callback that is invoked when any of the
    /// [`FdEvent`]s described in `events` is triggered.
    ///
    /// The callback receives the file descriptor it is associated with and a bitmask of the poll
    /// events that were triggered (typically [`FdEvent::INPUT`]).  It should return [`true`] to
    /// continue receiving callbacks, or [`false`] to have the callback unregistered.
    ///
    /// See also [the NDK
    /// docs](https://developer.android.com/ndk/reference/group/looper.html#alooper_addfd).
    ///
    /// Note that this will leak a [`Box`] unless the callback returns [`false`] to unregister
    /// itself.
    ///
    /// # Threading
    /// This function will be called on the current thread when this [`ThreadLooper`] is
    /// polled. A callback can also be registered from other threads via the equivalent
    /// [`ForeignLooper::add_fd_with_callback()`] function, which requires a [`Send`] bound.
    ///
    /// # Safety
    /// The caller should guarantee that this file descriptor stays open until it is removed via
    /// [`remove_fd()`][ForeignLooper::remove_fd()] or by returning [`false`] from the callback,
    /// and for however long the caller wishes to use this file descriptor inside and after the
    /// callback.
    #[doc(alias = "ALooper_addFd")]
    pub fn add_fd_with_callback<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
        &self,
        fd: BorrowedFd<'_>,
        events: FdEvent,
        callback: F,
    ) -> Result<(), LooperError> {
        unsafe {
            self.foreign
                .add_fd_with_callback_assume_send(fd, events, callback)
        }
    }

    /// Returns a reference to the [`ForeignLooper`] that is associated with the current thread.
    pub fn as_foreign(&self) -> &ForeignLooper {
        &self.foreign
    }

    pub fn into_foreign(self) -> ForeignLooper {
        self.foreign
    }
}

/// A native [`ALooper *`], not necessarily allocated with the current thread.
///
/// [`ALooper *`]: https://developer.android.com/ndk/reference/group/looper#alooper
#[derive(Debug)]
pub struct ForeignLooper {
    ptr: ptr::NonNull<ffi::ALooper>,
}

unsafe impl Send for ForeignLooper {}
unsafe impl Sync for ForeignLooper {}

impl Drop for ForeignLooper {
    fn drop(&mut self) {
        unsafe { ffi::ALooper_release(self.ptr.as_ptr()) }
    }
}

impl Clone for ForeignLooper {
    fn clone(&self) -> Self {
        unsafe {
            ffi::ALooper_acquire(self.ptr.as_ptr());
            Self { ptr: self.ptr }
        }
    }
}

impl ForeignLooper {
    /// Returns the looper associated with the current thread, if any.
    #[inline]
    pub fn for_thread() -> Option<Self> {
        ptr::NonNull::new(unsafe { ffi::ALooper_forThread() })
            .map(|ptr| unsafe { Self::from_ptr(ptr) })
    }

    /// Construct a [`ForeignLooper`] object from the given pointer.
    ///
    /// # Safety
    /// By calling this function, you guarantee that the pointer is a valid, non-null pointer to an
    /// NDK [`ffi::ALooper`].
    #[inline]
    pub unsafe fn from_ptr(ptr: ptr::NonNull<ffi::ALooper>) -> Self {
        ffi::ALooper_acquire(ptr.as_ptr());
        Self { ptr }
    }

    /// Returns a pointer to the NDK `ALooper` object.
    #[inline]
    pub fn ptr(&self) -> ptr::NonNull<ffi::ALooper> {
        self.ptr
    }

    /// Wakes the looper.  An event of [`Poll::Wake`] will be sent.
    pub fn wake(&self) {
        unsafe { ffi::ALooper_wake(self.ptr.as_ptr()) }
    }

    /// Adds a file descriptor to be polled, without a callback.
    ///
    /// See also [the NDK
    /// docs](https://developer.android.com/ndk/reference/group/looper.html#alooper_addfd).
    ///
    /// # Safety
    /// The caller should guarantee that this file descriptor stays open until it is removed via
    /// [`remove_fd()`][Self::remove_fd()], and for however long the caller wishes to use this file
    /// descriptor when it is returned in [`Poll::Event::fd`].

    // `ALooper_addFd` won't dereference `data`; it will only pass it on to the event.
    // Optionally dereferencing it there already enforces `unsafe` context.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn add_fd(
        &self,
        fd: BorrowedFd<'_>,
        ident: i32,
        events: FdEvent,
        data: *mut c_void,
    ) -> Result<(), LooperError> {
        match unsafe {
            ffi::ALooper_addFd(
                self.ptr.as_ptr(),
                fd.as_raw_fd(),
                ident,
                events.bits() as i32,
                None,
                data,
            )
        } {
            1 => Ok(()),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }

    /// Adds a file descriptor to be polled, with a callback that is invoked when any of the
    /// [`FdEvent`]s described in `events` is triggered.
    ///
    /// The callback receives the file descriptor it is associated with and a bitmask of the poll
    /// events that were triggered (typically [`FdEvent::INPUT`]).  It should return [`true`] to
    /// continue receiving callbacks, or [`false`] to have the callback unregistered.
    ///
    /// See also [the NDK
    /// docs](https://developer.android.com/ndk/reference/group/looper.html#alooper_addfd).
    ///
    /// Note that this will leak a [`Box`] unless the callback returns [`false`] to unregister
    /// itself.
    ///
    /// # Threading
    /// This function will be called on the looper thread where and when it is polled.
    /// For registering callbacks without [`Send`] requirement, call the equivalent
    /// [`ThreadLooper::add_fd_with_callback()`] function on the Looper thread.
    ///
    /// # Safety
    /// The caller should guarantee that this file descriptor stays open until it is removed via
    /// [`remove_fd()`][Self::remove_fd()] or by returning [`false`] from the callback, and for
    /// however long the caller wishes to use this file descriptor inside and after the callback.
    #[doc(alias = "ALooper_addFd")]
    pub fn add_fd_with_callback<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool + Send>(
        &self,
        fd: BorrowedFd<'_>,
        events: FdEvent,
        callback: F,
    ) -> Result<(), LooperError> {
        unsafe { self.add_fd_with_callback_assume_send(fd, events, callback) }
    }

    /// Private helper to deduplicate/commonize the implementation behind
    /// [`ForeignLooper::add_fd_with_callback()`] and [`ThreadLooper::add_fd_with_callback()`],
    /// as both have their own way of guaranteeing thread-safety.  The former, [`ForeignLooper`],
    /// requires the closure to be [`Send`]. The latter, [`ThreadLooper`], can only exist on the
    /// thread where polling happens and where the closure will end up being invoked, and does not
    /// require [`Send`].
    ///
    /// # Safety
    /// The caller must guarantee that `F` is [`Send`] or that `F` will only run on the current
    /// thread.  See the explanation above about why this function exists.
    unsafe fn add_fd_with_callback_assume_send<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
        &self,
        fd: BorrowedFd<'_>,
        events: FdEvent,
        callback: F,
    ) -> Result<(), LooperError> {
        extern "C" fn cb_handler<F: FnMut(BorrowedFd<'_>, FdEvent) -> bool>(
            fd: RawFd,
            events: i32,
            data: *mut c_void,
        ) -> i32 {
            abort_on_panic(|| unsafe {
                let mut cb = ManuallyDrop::new(Box::<F>::from_raw(data as *mut _));
                let events = FdEvent::from_bits_retain(
                    events.try_into().expect("Unexpected sign bit in `events`"),
                );
                let keep_registered = cb(BorrowedFd::borrow_raw(fd), events);
                if !keep_registered {
                    ManuallyDrop::into_inner(cb);
                }
                keep_registered as i32
            })
        }
        let data = Box::into_raw(Box::new(callback)) as *mut _;
        match unsafe {
            ffi::ALooper_addFd(
                self.ptr.as_ptr(),
                fd.as_raw_fd(),
                ffi::ALOOPER_POLL_CALLBACK,
                events.bits() as i32,
                Some(cb_handler::<F>),
                data,
            )
        } {
            1 => Ok(()),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }

    /// Removes a previously added file descriptor from the looper.
    ///
    /// Returns [`true`] if the file descriptor was removed, [`false`] if it was not previously
    /// registered.
    ///
    /// # Safety
    /// When this method returns, it is safe to close the file descriptor since the looper will no
    /// longer have a reference to it. However, it is possible for the callback to already be
    /// running or for it to run one last time if the file descriptor was already signalled.
    /// Calling code is responsible for ensuring that this case is safely handled. For example, if
    /// the callback takes care of removing itself during its own execution either by returning `0`
    /// or by calling this method, then it can be guaranteed to not be invoked again at any later
    /// time unless registered anew.
    ///
    /// Note that unregistering a file descriptor with callback will leak a [`Box`] created in
    /// [`add_fd_with_callback()`][Self::add_fd_with_callback()]. Consider returning [`false`]
    /// from the callback instead to drop it.
    pub fn remove_fd(&self, fd: BorrowedFd<'_>) -> Result<bool, LooperError> {
        match unsafe { ffi::ALooper_removeFd(self.ptr.as_ptr(), fd.as_raw_fd()) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(LooperError),
            _ => unreachable!(),
        }
    }
}
