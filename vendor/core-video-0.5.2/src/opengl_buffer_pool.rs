use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::{
    opengl_buffer::{CVOpenGLBuffer, CVOpenGLBufferRef},
    r#return::{kCVReturnSuccess, CVReturn},
};

#[repr(C)]
pub struct __CVOpenGLBufferPool(c_void);

pub type CVOpenGLBufferPoolRef = *mut __CVOpenGLBufferPool;

extern "C" {
    pub static kCVOpenGLBufferPoolMinimumBufferCountKey: CFStringRef;
    pub static kCVOpenGLBufferPoolMaximumBufferAgeKey: CFStringRef;

    pub fn CVOpenGLBufferPoolGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferPoolRetain(openGLBufferPool: CVOpenGLBufferPoolRef) -> CVOpenGLBufferPoolRef;
    pub fn CVOpenGLBufferPoolRelease(openGLBufferPool: CVOpenGLBufferPoolRef);
    pub fn CVOpenGLBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        openGLBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVOpenGLBufferPoolRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferPoolGetAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferPoolGetOpenGLBufferAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
        allocator: CFAllocatorRef,
        openGLBufferPool: CVOpenGLBufferPoolRef,
        openGLBufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
}

pub enum CVOpenGLBufferPoolKeys {
    MinimumBufferCount,
    MaximumBufferAge,
}

impl From<CVOpenGLBufferPoolKeys> for CFStringRef {
    fn from(key: CVOpenGLBufferPoolKeys) -> Self {
        unsafe {
            match key {
                CVOpenGLBufferPoolKeys::MinimumBufferCount => kCVOpenGLBufferPoolMinimumBufferCountKey,
                CVOpenGLBufferPoolKeys::MaximumBufferAge => kCVOpenGLBufferPoolMaximumBufferAgeKey,
            }
        }
    }
}

impl From<CVOpenGLBufferPoolKeys> for CFString {
    fn from(key: CVOpenGLBufferPoolKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub struct CVOpenGLBufferPool(CVOpenGLBufferPoolRef);

impl Drop for CVOpenGLBufferPool {
    fn drop(&mut self) {
        unsafe { CVOpenGLBufferPoolRelease(self.0) }
    }
}

impl_TCFType!(CVOpenGLBufferPool, CVOpenGLBufferPoolRef, CVOpenGLBufferPoolGetTypeID);
impl_CFTypeDescription!(CVOpenGLBufferPool);

impl CVOpenGLBufferPool {
    #[inline]
    pub fn new(
        pool_attributes: Option<&CFDictionary<CFString, CFType>>,
        opengl_buffer_attributes: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVOpenGLBufferPool, CVReturn> {
        let mut pool: CVOpenGLBufferPoolRef = null_mut();
        let status = unsafe {
            CVOpenGLBufferPoolCreate(
                kCFAllocatorDefault,
                pool_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                opengl_buffer_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                &mut pool,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pool) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferPoolGetAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    #[inline]
    pub fn get_opengl_buffer_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferPoolGetOpenGLBufferAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    #[inline]
    pub fn create_open_gl_buffer(&self) -> Result<CVOpenGLBuffer, CVReturn> {
        let mut buffer: CVOpenGLBufferRef = null_mut();
        let status = unsafe { CVOpenGLBufferPoolCreateOpenGLBuffer(kCFAllocatorDefault, self.as_concrete_TypeRef(), &mut buffer) };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(buffer) })
        } else {
            Err(status)
        }
    }
}
