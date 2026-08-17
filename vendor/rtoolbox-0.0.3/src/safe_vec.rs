use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;
use std::{ptr, sync::atomic};

/// Vec that is zeroed when dropped
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafeVec {
    pub inner: Vec<u8>,
}

impl SafeVec {
    pub fn new(inner: Vec<u8>) -> SafeVec {
        SafeVec { inner: inner }
    }

    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl Drop for SafeVec {
    fn drop(&mut self) {
        let default = u8::default();

        for c in self.inner.as_mut_slice() {
            unsafe { ptr::write_volatile(c, default) };
        }

        atomic::fence(atomic::Ordering::SeqCst);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

impl Deref for SafeVec {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.inner.deref()
    }
}

impl DerefMut for SafeVec {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.inner.deref_mut()
    }
}
