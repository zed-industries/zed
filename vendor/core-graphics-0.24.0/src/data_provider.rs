// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, TCFType};
use core_foundation::data::{CFData, CFDataRef};

use libc::{off_t, size_t};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::Arc;

use foreign_types::{foreign_type, ForeignType, ForeignTypeRef};

pub type CGDataProviderGetBytesCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t) -> size_t>;
pub type CGDataProviderReleaseInfoCallback = Option<unsafe extern "C" fn(*mut c_void)>;
pub type CGDataProviderRewindCallback = Option<unsafe extern "C" fn(*mut c_void)>;
pub type CGDataProviderSkipBytesCallback = Option<unsafe extern "C" fn(*mut c_void, size_t)>;
pub type CGDataProviderSkipForwardCallback =
    Option<unsafe extern "C" fn(*mut c_void, off_t) -> off_t>;

pub type CGDataProviderGetBytePointerCallback =
    Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
pub type CGDataProviderGetBytesAtOffsetCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t, size_t)>;
pub type CGDataProviderReleaseBytePointerCallback =
    Option<unsafe extern "C" fn(*mut c_void, *const c_void)>;
pub type CGDataProviderReleaseDataCallback =
    Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t)>;
pub type CGDataProviderGetBytesAtPositionCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, off_t, size_t)>;

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGDataProvider {
        type CType = crate::sys::CGDataProvider;
        fn drop = |cs| CFRelease(cs as *mut _);
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGDataProvider {
    pub fn type_id() -> CFTypeID {
        unsafe { CGDataProviderGetTypeID() }
    }

    /// Creates a data provider from the given reference-counted buffer.
    ///
    /// The `CGDataProvider` object takes ownership of the reference. Once the data provider
    /// is destroyed, the reference count of the buffer is automatically decremented.
    pub fn from_buffer<T: AsRef<[u8]> + Sync + Send>(buffer: Arc<T>) -> Self {
        unsafe {
            let ptr = (*buffer).as_ref().as_ptr() as *const c_void;
            let len = (*buffer).as_ref().len() as size_t;
            let info = Arc::into_raw(buffer) as *mut c_void;
            let result = CGDataProviderCreateWithData(info, ptr, len, Some(release::<T>));
            return CGDataProvider::from_ptr(result);
        }

        unsafe extern "C" fn release<T>(info: *mut c_void, _: *const c_void, _: size_t) {
            drop(Arc::from_raw(info as *mut T))
        }
    }

    /// Creates a data prvider from a given slice. The data provider does not own the slice in this
    /// case, so it's up to the user to ensure the memory safety here.
    pub unsafe fn from_slice(buffer: &[u8]) -> Self {
        let ptr = buffer.as_ptr() as *const c_void;
        let len = buffer.len() as size_t;
        let result = CGDataProviderCreateWithData(ptr::null_mut(), ptr, len, None);
        CGDataProvider::from_ptr(result)
    }

    /// Creates a data provider from the given raw pointer, length, and destructor function.
    ///
    /// This is double-boxed because the Core Text API requires that the userdata be a single
    /// pointer.
    pub unsafe fn from_custom_data(custom_data: Box<Box<dyn CustomData>>) -> Self {
        let (ptr, len) = (custom_data.ptr() as *const c_void, custom_data.len());
        let userdata = mem::transmute::<Box<Box<dyn CustomData>>, &mut c_void>(custom_data);
        let data_provider = CGDataProviderCreateWithData(userdata, ptr, len, Some(release));
        return CGDataProvider::from_ptr(data_provider);

        unsafe extern "C" fn release(info: *mut c_void, _: *const c_void, _: size_t) {
            drop(mem::transmute::<*mut c_void, Box<Box<dyn CustomData>>>(
                info,
            ))
        }
    }
}

impl CGDataProviderRef {
    /// Creates a copy of the data from the underlying `CFDataProviderRef`.
    pub fn copy_data(&self) -> CFData {
        unsafe { CFData::wrap_under_create_rule(CGDataProviderCopyData(self.as_ptr())) }
    }
}

/// Encapsulates custom data that can be wrapped.
pub trait CustomData {
    /// Returns a pointer to the start of the custom data. This pointer *must not change* during
    /// the lifespan of this `CustomData`.
    unsafe fn ptr(&self) -> *const u8;
    /// Returns the length of this custom data. This value must not change during the lifespan of
    /// this `CustomData`.
    unsafe fn len(&self) -> usize;
}

#[test]
fn test_data_provider() {
    let l = vec![5];
    CGDataProvider::from_buffer(Arc::new(l));

    let l = vec![5];
    CGDataProvider::from_buffer(Arc::new(l.into_boxed_slice()));

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
    let dp = CGDataProvider::from_buffer(l);
    drop(m);
    assert!(!dropped.load(SeqCst));
    drop(dp);
    assert!(dropped.load(SeqCst))
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    fn CGDataProviderCopyData(provider: crate::sys::CGDataProviderRef) -> CFDataRef;
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *const c_void,
        size: size_t,
        releaseData: CGDataProviderReleaseDataCallback,
    ) -> crate::sys::CGDataProviderRef;
    //fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    //fn CGDataProviderRelease(provider: CGDataProviderRef);
    //fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
