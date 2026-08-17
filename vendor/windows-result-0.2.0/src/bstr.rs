use super::*;

#[repr(transparent)]
pub struct BasicString(*const u16);

impl BasicString {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        if self.0.is_null() {
            0
        } else {
            unsafe { SysStringLen(self.0) as usize }
        }
    }

    pub fn as_wide(&self) -> &[u16] {
        let len = self.len();
        if len != 0 {
            unsafe { core::slice::from_raw_parts(self.as_ptr(), len) }
        } else {
            &[]
        }
    }

    pub fn as_ptr(&self) -> *const u16 {
        if !self.is_empty() {
            self.0
        } else {
            const EMPTY: [u16; 1] = [0];
            EMPTY.as_ptr()
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
