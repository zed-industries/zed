//! Bindings to IOCP, I/O Completion Ports

use crate::FALSE;
use std::cmp;
use std::fmt;
use std::io;
use std::mem;
use std::os::windows::io::{AsRawHandle, AsRawSocket, FromRawHandle, IntoRawHandle, RawHandle};
use std::time::Duration;

use crate::handle::Handle;
use crate::Overlapped;
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::IO::{
    CreateIoCompletionPort, GetQueuedCompletionStatus, GetQueuedCompletionStatusEx,
    PostQueuedCompletionStatus, OVERLAPPED, OVERLAPPED_ENTRY,
};

/// A handle to an Windows I/O Completion Port.
#[derive(Debug)]
pub struct CompletionPort {
    handle: Handle,
}

/// A status message received from an I/O completion port.
///
/// These statuses can be created via the `new` or `empty` constructors and then
/// provided to a completion port, or they are read out of a completion port.
/// The fields of each status are read through its accessor methods.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CompletionStatus(OVERLAPPED_ENTRY);

impl fmt::Debug for CompletionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionStatus(OVERLAPPED_ENTRY)")
    }
}

unsafe impl Send for CompletionStatus {}
unsafe impl Sync for CompletionStatus {}

impl CompletionPort {
    /// Creates a new I/O completion port with the specified concurrency value.
    ///
    /// The number of threads given corresponds to the level of concurrency
    /// allowed for threads associated with this port. Consult the Windows
    /// documentation for more information about this value.
    pub fn new(threads: u32) -> io::Result<CompletionPort> {
        let ret = unsafe {
            CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, threads)
        };
        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(CompletionPort {
                handle: Handle::new(ret),
            })
        }
    }

    /// Associates a new `HANDLE` to this I/O completion port.
    ///
    /// This function will associate the given handle to this port with the
    /// given `token` to be returned in status messages whenever it receives a
    /// notification.
    ///
    /// Any object which is convertible to a `HANDLE` via the `AsRawHandle`
    /// trait can be provided to this function, such as `std::fs::File` and
    /// friends.
    pub fn add_handle<T: AsRawHandle + ?Sized>(&self, token: usize, t: &T) -> io::Result<()> {
        self._add(token, t.as_raw_handle() as HANDLE)
    }

    /// Associates a new `SOCKET` to this I/O completion port.
    ///
    /// This function will associate the given socket to this port with the
    /// given `token` to be returned in status messages whenever it receives a
    /// notification.
    ///
    /// Any object which is convertible to a `SOCKET` via the `AsRawSocket`
    /// trait can be provided to this function, such as `std::net::TcpStream`
    /// and friends.
    pub fn add_socket<T: AsRawSocket + ?Sized>(&self, token: usize, t: &T) -> io::Result<()> {
        self._add(token, t.as_raw_socket() as HANDLE)
    }

    fn _add(&self, token: usize, handle: HANDLE) -> io::Result<()> {
        assert_eq!(mem::size_of_val(&token), mem::size_of::<usize>());
        let ret = unsafe { CreateIoCompletionPort(handle, self.handle.raw(), token, 0) };
        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            debug_assert_eq!(ret, self.handle.raw());
            Ok(())
        }
    }

    /// Dequeue a completion status from this I/O completion port.
    ///
    /// This function will associate the calling thread with this completion
    /// port and then wait for a status message to become available. The precise
    /// semantics on when this function returns depends on the concurrency value
    /// specified when the port was created.
    ///
    /// A timeout can optionally be specified to this function. If `None` is
    /// provided this function will not time out, and otherwise it will time out
    /// after the specified duration has passed.
    ///
    /// On success this will return the status message which was dequeued from
    /// this completion port.
    pub fn get(&self, timeout: Option<Duration>) -> io::Result<CompletionStatus> {
        let mut bytes = 0;
        let mut token = 0;
        let mut overlapped = std::ptr::null_mut();
        let timeout = crate::dur2ms(timeout);
        let ret = unsafe {
            GetQueuedCompletionStatus(
                self.handle.raw(),
                &mut bytes,
                &mut token,
                &mut overlapped,
                timeout,
            )
        };
        crate::cvt(ret).map(|_| {
            CompletionStatus(OVERLAPPED_ENTRY {
                dwNumberOfBytesTransferred: bytes,
                lpCompletionKey: token,
                lpOverlapped: overlapped,
                Internal: 0,
            })
        })
    }

    /// Dequeues a number of completion statuses from this I/O completion port.
    ///
    /// This function is the same as `get` except that it may return more than
    /// one status. A buffer of "zero" statuses is provided (the contents are
    /// not read) and then on success this function will return a sub-slice of
    /// statuses which represent those which were dequeued from this port. This
    /// function does not wait to fill up the entire list of statuses provided.
    ///
    /// Like with `get`, a timeout may be specified for this operation.
    pub fn get_many<'a>(
        &self,
        list: &'a mut [CompletionStatus],
        timeout: Option<Duration>,
    ) -> io::Result<&'a mut [CompletionStatus]> {
        debug_assert_eq!(
            mem::size_of::<CompletionStatus>(),
            mem::size_of::<OVERLAPPED_ENTRY>()
        );
        let mut removed = 0;
        let timeout = crate::dur2ms(timeout);
        let len = cmp::min(list.len(), u32::MAX as usize) as u32;
        let ret = unsafe {
            GetQueuedCompletionStatusEx(
                self.handle.raw(),
                list.as_ptr() as *mut _,
                len,
                &mut removed,
                timeout,
                FALSE,
            )
        };
        match crate::cvt(ret) {
            Ok(_) => Ok(&mut list[..removed as usize]),
            Err(e) => Err(e),
        }
    }

    /// Posts a new completion status onto this I/O completion port.
    ///
    /// This function will post the given status, with custom parameters, to the
    /// port. Threads blocked in `get` or `get_many` will eventually receive
    /// this status.
    pub fn post(&self, status: CompletionStatus) -> io::Result<()> {
        let ret = unsafe {
            PostQueuedCompletionStatus(
                self.handle.raw(),
                status.0.dwNumberOfBytesTransferred,
                status.0.lpCompletionKey,
                status.0.lpOverlapped,
            )
        };
        crate::cvt(ret).map(|_| ())
    }
}

impl AsRawHandle for CompletionPort {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle.raw() as RawHandle
    }
}

impl FromRawHandle for CompletionPort {
    unsafe fn from_raw_handle(handle: RawHandle) -> CompletionPort {
        CompletionPort {
            handle: Handle::new(handle as HANDLE),
        }
    }
}

impl IntoRawHandle for CompletionPort {
    fn into_raw_handle(self) -> RawHandle {
        self.handle.into_raw() as RawHandle
    }
}

impl CompletionStatus {
    /// Creates a new completion status with the provided parameters.
    ///
    /// This function is useful when creating a status to send to a port with
    /// the `post` method. The parameters are opaquely passed through and not
    /// interpreted by the system at all.
    pub fn new(bytes: u32, token: usize, overlapped: *mut Overlapped) -> CompletionStatus {
        assert_eq!(mem::size_of_val(&token), mem::size_of::<usize>());
        CompletionStatus(OVERLAPPED_ENTRY {
            dwNumberOfBytesTransferred: bytes,
            lpCompletionKey: token,
            lpOverlapped: overlapped as *mut _,
            Internal: 0,
        })
    }

    /// Creates a new borrowed completion status from the borrowed
    /// `OVERLAPPED_ENTRY` argument provided.
    ///
    /// This method will wrap the `OVERLAPPED_ENTRY` in a `CompletionStatus`,
    /// returning the wrapped structure.
    pub fn from_entry(entry: &OVERLAPPED_ENTRY) -> &CompletionStatus {
        // Safety: CompletionStatus is repr(transparent) w/ OVERLAPPED_ENTRY, so
        // a reference to one is guaranteed to be layout compatible with the
        // reference to another.
        unsafe { &*(entry as *const _ as *const _) }
    }

    /// Creates a new "zero" completion status.
    ///
    /// This function is useful when creating a stack buffer or vector of
    /// completion statuses to be passed to the `get_many` function.
    pub fn zero() -> CompletionStatus {
        CompletionStatus::new(0, 0, std::ptr::null_mut())
    }

    /// Returns the number of bytes that were transferred for the I/O operation
    /// associated with this completion status.
    pub fn bytes_transferred(&self) -> u32 {
        self.0.dwNumberOfBytesTransferred
    }

    /// Returns the completion key value associated with the file handle whose
    /// I/O operation has completed.
    ///
    /// A completion key is a per-handle key that is specified when it is added
    /// to an I/O completion port via `add_handle` or `add_socket`.
    pub fn token(&self) -> usize {
        self.0.lpCompletionKey
    }

    /// Returns a pointer to the `Overlapped` structure that was specified when
    /// the I/O operation was started.
    pub fn overlapped(&self) -> *mut OVERLAPPED {
        self.0.lpOverlapped
    }

    /// Returns a pointer to the internal `OVERLAPPED_ENTRY` object.
    pub fn entry(&self) -> &OVERLAPPED_ENTRY {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::iocp::{CompletionPort, CompletionStatus};
    use std::mem;
    use std::time::Duration;
    use windows_sys::Win32::Foundation::*;

    #[test]
    fn is_send_sync() {
        fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<CompletionPort>();
    }

    #[test]
    fn token_right_size() {
        assert_eq!(mem::size_of::<usize>(), mem::size_of::<usize>());
    }

    #[test]
    fn timeout() {
        let c = CompletionPort::new(1).unwrap();
        let err = c.get(Some(Duration::from_millis(1))).unwrap_err();
        assert_eq!(err.raw_os_error(), Some(WAIT_TIMEOUT as i32));
    }

    #[test]
    fn get() {
        let c = CompletionPort::new(1).unwrap();
        c.post(CompletionStatus::new(
            1,
            2,
            std::ptr::NonNull::dangling().as_ptr(),
        ))
        .unwrap();
        let s = c.get(None).unwrap();
        assert_eq!(s.bytes_transferred(), 1);
        assert_eq!(s.token(), 2);
        assert_eq!(s.overlapped(), std::ptr::NonNull::dangling().as_ptr());
    }

    #[test]
    fn get_many() {
        let c = CompletionPort::new(1).unwrap();

        c.post(CompletionStatus::new(
            1,
            2,
            std::ptr::NonNull::dangling().as_ptr(),
        ))
        .unwrap();
        c.post(CompletionStatus::new(
            4,
            5,
            std::ptr::NonNull::dangling().as_ptr(),
        ))
        .unwrap();

        let mut s = vec![CompletionStatus::zero(); 4];
        {
            let s = c.get_many(&mut s, None).unwrap();
            assert_eq!(s.len(), 2);
            assert_eq!(s[0].bytes_transferred(), 1);
            assert_eq!(s[0].token(), 2);
            assert_eq!(s[0].overlapped(), std::ptr::NonNull::dangling().as_ptr());
            assert_eq!(s[1].bytes_transferred(), 4);
            assert_eq!(s[1].token(), 5);
            assert_eq!(s[1].overlapped(), std::ptr::NonNull::dangling().as_ptr());
        }
        assert_eq!(s[2].bytes_transferred(), 0);
        assert_eq!(s[2].token(), 0);
        assert_eq!(s[2].overlapped(), std::ptr::null_mut());
    }
}
