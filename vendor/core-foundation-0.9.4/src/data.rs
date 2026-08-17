// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation byte buffers.

use core_foundation_sys::base::kCFAllocatorDefault;
use core_foundation_sys::base::CFIndex;
pub use core_foundation_sys::data::*;
use std::ops::Deref;
use std::slice;
use std::sync::Arc;

use crate::base::{CFIndexConvertible, TCFType};

declare_TCFType! {
    /// A byte buffer.
    CFData, CFDataRef
}
impl_TCFType!(CFData, CFDataRef, CFDataGetTypeID);
impl_CFTypeDescription!(CFData);

impl CFData {
    /// Creates a [`CFData`] around a copy `buffer`
    pub fn from_buffer(buffer: &[u8]) -> CFData {
        unsafe {
            let data_ref = CFDataCreate(
                kCFAllocatorDefault,
                buffer.as_ptr(),
                buffer.len().to_CFIndex(),
            );
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Creates a [`CFData`] referencing `buffer` without creating a copy
    pub fn from_arc<T: AsRef<[u8]> + Sync + Send>(buffer: Arc<T>) -> Self {
        use crate::base::{CFAllocator, CFAllocatorContext};
        use std::os::raw::c_void;

        unsafe {
            let ptr = (*buffer).as_ref().as_ptr() as *const _;
            let len = (*buffer).as_ref().len().to_CFIndex();
            let info = Arc::into_raw(buffer) as *mut c_void;

            extern "C" fn deallocate<T>(_: *mut c_void, info: *mut c_void) {
                unsafe {
                    drop(Arc::from_raw(info as *mut T));
                }
            }

            // Use a separate allocator for each allocation because
            // we need `info` to do the deallocation vs. `ptr`
            let allocator = CFAllocator::new(CFAllocatorContext {
                info,
                version: 0,
                retain: None,
                reallocate: None,
                release: None,
                copyDescription: None,
                allocate: None,
                deallocate: Some(deallocate::<T>),
                preferredSize: None,
            });
            let data_ref = CFDataCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                ptr,
                len,
                allocator.as_CFTypeRef(),
            );
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns a pointer to the underlying bytes in this data. Note that this byte buffer is
    /// read-only.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize) }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe { CFDataGetLength(self.0) }
    }

    /// Returns `true` if this byte buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Deref for CFData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

#[cfg(test)]
mod test {
    use super::CFData;
    use std::sync::Arc;

    #[test]
    fn test_data_provider() {
        let l = vec![5];
        CFData::from_arc(Arc::new(l));

        let l = vec![5];
        CFData::from_arc(Arc::new(l.into_boxed_slice()));

        // Make sure the buffer is actually dropped
        use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
        struct VecWrapper {
            inner: Vec<u8>,
            dropped: Arc<AtomicBool>,
        }

        impl Drop for VecWrapper {
            fn drop(&mut self) {
                self.dropped.store(true, SeqCst)
            }
        }

        impl std::convert::AsRef<[u8]> for VecWrapper {
            fn as_ref(&self) -> &[u8] {
                &self.inner
            }
        }

        let dropped = Arc::new(AtomicBool::default());
        let l = Arc::new(VecWrapper {
            inner: vec![5],
            dropped: dropped.clone(),
        });
        let m = l.clone();
        let dp = CFData::from_arc(l);
        drop(m);
        assert!(!dropped.load(SeqCst));
        drop(dp);
        assert!(dropped.load(SeqCst))
    }
}
