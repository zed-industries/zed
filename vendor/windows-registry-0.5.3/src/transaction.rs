use super::*;

/// A transaction object.
#[repr(transparent)]
#[derive(Debug)]
pub struct Transaction(pub(crate) HANDLE);

impl Transaction {
    /// Creates a new transaction.
    pub fn new() -> Result<Self> {
        let handle = unsafe { CreateTransaction(null_mut(), null_mut(), 0, 0, 0, 0, null()) };

        if core::ptr::eq(handle, INVALID_HANDLE_VALUE) {
            Err(Error::from_win32())
        } else {
            Ok(Self(handle))
        }
    }

    /// Commits the transaction.
    ///
    /// The transaction rolls back if it is dropped before `commit` is called.
    pub fn commit(self) -> Result<()> {
        let result = unsafe { CommitTransaction(self.0) };

        if result == 0 {
            Err(Error::from_win32())
        } else {
            Ok(())
        }
    }

    /// Constructs a transaction object from an existing handle.
    ///
    /// # Safety
    ///
    /// This function takes ownership of the handle.
    /// The handle must be owned by the caller and safe to free with `CloseHandle`.
    pub unsafe fn from_raw(handle: *mut core::ffi::c_void) -> Self {
        Self(handle)
    }

    /// Returns the underlying transaction handle.
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.0
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
