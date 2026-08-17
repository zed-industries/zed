// Copyright 2024 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use windows::Win32::Foundation::HWND;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowHandle(pub HWND);

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}

impl From<HWND> for WindowHandle {
    fn from(value: HWND) -> Self {
        Self(value)
    }
}

impl From<WindowHandle> for HWND {
    fn from(value: WindowHandle) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(WindowHandle: Send, Sync);
}
