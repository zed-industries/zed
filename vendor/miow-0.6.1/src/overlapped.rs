use std::fmt;
use std::io;
use std::mem;
use std::ptr;

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Threading::CreateEventW;
use windows_sys::Win32::System::IO::OVERLAPPED;

/// A wrapper around `OVERLAPPED` to provide "rustic" accessors and
/// initializers.
pub struct Overlapped(OVERLAPPED);

impl fmt::Debug for Overlapped {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OVERLAPPED")
    }
}

unsafe impl Send for Overlapped {}
unsafe impl Sync for Overlapped {}

impl Overlapped {
    /// Creates a new zeroed out instance of an overlapped I/O tracking state.
    ///
    /// This is suitable for passing to methods which will then later get
    /// notified via an I/O Completion Port.
    pub fn zero() -> Overlapped {
        Overlapped(unsafe { mem::zeroed() })
    }

    /// Creates a new `Overlapped` with an initialized non-null `hEvent`.  The caller is
    /// responsible for calling `CloseHandle` on the `hEvent` field of the returned
    /// `Overlapped`.  The event is created with `bManualReset` set to `FALSE`, meaning after a
    /// single thread waits on the event, it will be reset.
    pub fn initialize_with_autoreset_event() -> io::Result<Overlapped> {
        let event = unsafe { CreateEventW(ptr::null_mut(), 0i32, 0i32, ptr::null_mut()) };
        if event.is_null() {
            return Err(io::Error::last_os_error());
        }
        let mut overlapped = Self::zero();
        overlapped.set_event(event);
        Ok(overlapped)
    }

    /// Creates a new `Overlapped` function pointer from the underlying
    /// `OVERLAPPED`, wrapping in the "rusty" wrapper for working with
    /// accessors.
    ///
    /// # Safety
    ///
    /// This function doesn't validate `ptr` nor the lifetime of the returned
    /// pointer at all, it's recommended to use this method with extreme
    /// caution.
    pub unsafe fn from_raw<'a>(ptr: *mut OVERLAPPED) -> &'a mut Overlapped {
        &mut *(ptr as *mut Overlapped)
    }

    /// Gain access to the raw underlying data
    pub fn raw(&self) -> *mut OVERLAPPED {
        &self.0 as *const _ as *mut _
    }

    /// Sets the offset inside this overlapped structure.
    ///
    /// Note that for I/O operations in general this only has meaning for I/O
    /// handles that are on a seeking device that supports the concept of an
    /// offset.
    pub fn set_offset(&mut self, offset: u64) {
        self.0.Anonymous.Anonymous.Offset = offset as u32;
        self.0.Anonymous.Anonymous.OffsetHigh = (offset >> 32) as u32;
    }

    /// Reads the offset inside this overlapped structure.
    pub fn offset(&self) -> u64 {
        unsafe {
            (self.0.Anonymous.Anonymous.Offset as u64)
                | ((self.0.Anonymous.Anonymous.OffsetHigh as u64) << 32)
        }
    }

    /// Sets the `hEvent` field of this structure.
    ///
    /// The event specified can be null.
    pub fn set_event(&mut self, event: HANDLE) {
        self.0.hEvent = event;
    }

    /// Reads the `hEvent` field of this structure, may return null.
    pub fn event(&self) -> HANDLE {
        self.0.hEvent
    }
}
