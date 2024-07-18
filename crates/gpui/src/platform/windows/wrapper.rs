use std::ops::Deref;

use windows::Win32::Foundation::HANDLE;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SafeHandle {
    raw: HANDLE,
}

unsafe impl Send for SafeHandle {}
unsafe impl Sync for SafeHandle {}

impl From<HANDLE> for SafeHandle {
    fn from(value: HANDLE) -> Self {
        SafeHandle { raw: value }
    }
}

impl Deref for SafeHandle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
