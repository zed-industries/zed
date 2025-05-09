use std::ops::Deref;

use util::ResultExt;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::Memory::{GlobalLock, GlobalSize, GlobalUnlock},
    UI::WindowsAndMessaging::HCURSOR,
};

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

#[derive(Debug, Clone, Copy)]
pub(crate) struct SafeCursor {
    raw: HCURSOR,
}

unsafe impl Send for SafeCursor {}
unsafe impl Sync for SafeCursor {}

impl From<HCURSOR> for SafeCursor {
    fn from(value: HCURSOR) -> Self {
        SafeCursor { raw: value }
    }
}

impl Deref for SafeCursor {
    type Target = HCURSOR;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
