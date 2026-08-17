use super::*;
use core::ops::Deref;

#[repr(transparent)]
pub struct BasicString(*const u16);

impl Deref for BasicString {
    type Target = [u16];

    fn deref(&self) -> &[u16] {
        let len = if self.0.is_null() {
            0
        } else {
            unsafe { SysStringLen(self.0) as usize }
        };

        if len > 0 {
            unsafe { core::slice::from_raw_parts(self.0, len) }
        } else {
            // This ensures that if `as_ptr` is called on the slice that the resulting pointer
            // will still refer to a null-terminated string.
            const EMPTY: [u16; 1] = [0];
            &EMPTY[..0]
        }
    }
}

impl Default for BasicString {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

impl Drop for BasicString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { SysFreeString(self.0) }
        }
    }
}
