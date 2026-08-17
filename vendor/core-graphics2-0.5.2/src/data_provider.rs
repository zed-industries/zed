use std::{ffi::CString, ptr::null_mut, sync::Arc};

use core_foundation::{
    base::{CFTypeID, TCFType},
    data::{CFData, CFDataRef},
    impl_CFTypeDescription, impl_TCFType,
    url::{CFURLRef, CFURL},
};
use libc::{c_char, c_void, off_t, size_t};

#[repr(C)]
pub struct __CGDataProvider(c_void);

pub type CGDataProviderRef = *const __CGDataProvider;

pub type CGDataProviderGetBytesCallback = extern "C" fn(*mut c_void, *mut c_void, size_t) -> size_t;
pub type CGDataProviderSkipForwardCallback = extern "C" fn(*mut c_void, off_t) -> off_t;
pub type CGDataProviderRewindCallback = extern "C" fn(*mut c_void);
pub type CGDataProviderReleaseInfoCallback = extern "C" fn(*mut c_void);

#[repr(C)]
pub struct CGDataProviderSequentialCallbacks {
    pub version: u32,
    pub getBytes: CGDataProviderGetBytesCallback,
    pub skipForward: CGDataProviderSkipForwardCallback,
    pub rewind: CGDataProviderRewindCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}

pub type CGDataProviderGetBytePointerCallback = extern "C" fn(*mut c_void) -> *mut c_void;
pub type CGDataProviderReleaseBytePointerCallback = extern "C" fn(*mut c_void, *const c_void);
pub type CGDataProviderGetBytesAtPositionCallback = extern "C" fn(*mut c_void, *mut c_void, off_t, size_t);

#[repr(C)]
pub struct CGDataProviderDirectCallbacks {
    pub version: u32,
    pub getBytePointer: CGDataProviderGetBytePointerCallback,
    pub releaseBytePointer: CGDataProviderReleaseBytePointerCallback,
    pub getBytesAtPosition: CGDataProviderGetBytesAtPositionCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}

pub type CGDataProviderReleaseDataCallback = extern "C" fn(*mut c_void, *const c_void, size_t);

extern "C" {
    pub fn CGDataProviderGetTypeID() -> CFTypeID;
    pub fn CGDataProviderCreateSequential(info: *mut c_void, callbacks: *const CGDataProviderSequentialCallbacks) -> CGDataProviderRef;
    pub fn CGDataProviderCreateDirect(info: *mut c_void, size: off_t, callbacks: *const CGDataProviderDirectCallbacks) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *const c_void,
        size: size_t,
        releaseData: Option<CGDataProviderReleaseDataCallback>,
    ) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithURL(url: CFURLRef) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithFilename(filename: *const c_char) -> CGDataProviderRef;
    pub fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
    pub fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
    pub fn CGDataProviderGetInfo(provider: CGDataProviderRef) -> *mut c_void;
}

pub struct CGDataProvider(CGDataProviderRef);

impl Drop for CGDataProvider {
    fn drop(&mut self) {
        unsafe { CGDataProviderRelease(self.0) }
    }
}

impl_TCFType!(CGDataProvider, CGDataProviderRef, CGDataProviderGetTypeID);
impl_CFTypeDescription!(CGDataProvider);

impl CGDataProvider {
    pub unsafe fn new_sequential(info: *mut c_void, callbacks: *const CGDataProviderSequentialCallbacks) -> Option<Self> {
        let provider = CGDataProviderCreateSequential(info, callbacks);
        if provider.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(provider))
        }
    }

    pub unsafe fn new_direct(info: *mut c_void, size: off_t, callbacks: *const CGDataProviderDirectCallbacks) -> Option<Self> {
        let provider = CGDataProviderCreateDirect(info, size, callbacks);
        if provider.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(provider))
        }
    }

    pub fn from_buffer<T>(buffer: Arc<T>) -> Option<Self>
    where
        T: AsRef<[u8]> + Sync + Send,
    {
        unsafe {
            let ptr = (*buffer).as_ref().as_ptr() as *const c_void;
            let len = (*buffer).as_ref().len() as size_t;
            let info = Arc::into_raw(buffer) as *mut c_void;
            let data_provider = CGDataProviderCreateWithData(info, ptr, len, Some(release::<T>));
            if data_provider.is_null() {
                drop(Arc::from_raw(info));
                return None;
            } else {
                return Some(TCFType::wrap_under_create_rule(data_provider));
            }
        }

        extern "C" fn release<T>(info: *mut c_void, _: *const c_void, _: size_t) {
            unsafe { drop(Arc::from_raw(info as *mut T)) }
        }
    }

    pub unsafe fn from_slice(buffer: &[u8]) -> Option<Self> {
        let ptr = buffer.as_ptr() as *const c_void;
        let len = buffer.len() as size_t;
        let data_provider = CGDataProviderCreateWithData(null_mut(), ptr, len, None);
        if data_provider.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(data_provider))
        }
    }

    pub fn from_data(data: &CFData) -> Option<Self> {
        unsafe {
            let data_provider = CGDataProviderCreateWithCFData(data.as_concrete_TypeRef());
            if data_provider.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data_provider))
            }
        }
    }

    pub fn from_url(url: CFURL) -> Option<Self> {
        unsafe {
            let data_provider = CGDataProviderCreateWithURL(url.as_concrete_TypeRef());
            if data_provider.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data_provider))
            }
        }
    }

    pub fn from_filename(filename: &str) -> Option<Self> {
        let c_str = CString::new(filename).ok()?;
        unsafe {
            let data_provider = CGDataProviderCreateWithFilename(c_str.as_ptr() as *const c_char);
            if data_provider.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data_provider))
            }
        }
    }

    pub fn copy_data(&self) -> Option<CFData> {
        unsafe {
            let data = CGDataProviderCopyData(self.as_concrete_TypeRef());
            if data.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data))
            }
        }
    }

    pub unsafe fn get_info(&self) -> *mut c_void {
        unsafe { CGDataProviderGetInfo(self.as_concrete_TypeRef()) }
    }
}
