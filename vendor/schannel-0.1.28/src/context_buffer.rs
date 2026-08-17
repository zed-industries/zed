use std::ops::Deref;
use std::slice;

use windows_sys::Win32::Security::Authentication::Identity;

pub struct ContextBuffer(pub Identity::SecBuffer);

impl Drop for ContextBuffer {
    fn drop(&mut self) {
        unsafe {
            Identity::FreeContextBuffer(self.0.pvBuffer);
        }
    }
}

impl Deref for ContextBuffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        if self.0.cbBuffer == 0 {
            return &[];
        }
        
        unsafe { slice::from_raw_parts(self.0.pvBuffer as *const _, self.0.cbBuffer as usize) }
    }
}
