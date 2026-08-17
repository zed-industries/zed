use super::*;

#[doc(hidden)]
pub struct Waiter(isize);
pub struct WaiterSignaler(isize);

impl Waiter {
    pub fn new() -> crate::Result<(Waiter, WaiterSignaler)> {
        unsafe {
            let handle = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
            if handle == 0 {
                Err(crate::Error::from_win32())
            } else {
                Ok((Waiter(handle), WaiterSignaler(handle)))
            }
        }
    }
}

impl WaiterSignaler {
    /// # Safety
    /// Signals the `Waiter`. This is unsafe because the lifetime of `WaiterSignaler` is not tied
    /// to the lifetime of the `Waiter`. This is not possible in this case because the `Waiter`
    /// is used to signal a WinRT async completion and the compiler doesn't know that the lifetime
    /// of the delegate is bounded by the calling function.
    pub unsafe fn signal(&self) {
        // https://github.com/microsoft/windows-rs/pull/374#discussion_r535313344
        SetEvent(self.0);
    }
}

impl Drop for Waiter {
    fn drop(&mut self) {
        unsafe {
            WaitForSingleObject(self.0, 0xFFFFFFFF);
            CloseHandle(self.0);
        }
    }
}
