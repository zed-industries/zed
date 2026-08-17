use super::*;

pub struct HeapString(pub *mut u16);

impl Default for HeapString {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

impl Drop for HeapString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                HeapFree(GetProcessHeap(), 0, self.0 as _);
            }
        }
    }
}

pub fn wide_trim_end(mut wide: &[u16]) -> &[u16] {
    while let Some(last) = wide.last() {
        match last {
            32 | 9..=13 => wide = &wide[..wide.len() - 1],
            _ => break,
        }
    }
    wide
}
