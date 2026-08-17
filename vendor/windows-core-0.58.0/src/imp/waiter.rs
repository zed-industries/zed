use super::*;

pub struct Waiter(HANDLE);
pub struct WaiterSignaler(HANDLE);

unsafe impl Send for WaiterSignaler {}

impl Waiter {
    pub fn new() -> crate::Result<(Waiter, WaiterSignaler)> {
        unsafe {
            let handle = CreateEventW(core::ptr::null(), 1, 0, core::ptr::null());
            if handle.is_null() {
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
