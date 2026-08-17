use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::{
    base::CVOptionFlags,
    pixel_buffer::{CVPixelBuffer, CVPixelBufferRef},
    r#return::{kCVReturnSuccess, CVReturn},
};

#[repr(C)]
pub struct __CVPixelBufferPool(c_void);

pub type CVPixelBufferPoolRef = *mut __CVPixelBufferPool;

pub type CVPixelBufferPoolFlushFlags = CVOptionFlags;

pub const kCVPixelBufferPoolFlushExcessBuffers: CVPixelBufferPoolFlushFlags = 1;

extern "C" {
    pub static kCVPixelBufferPoolMinimumBufferCountKey: CFStringRef;
    pub static kCVPixelBufferPoolMaximumBufferAgeKey: CFStringRef;

    pub static kCVPixelBufferPoolAllocationThresholdKey: CFStringRef;
    pub static kCVPixelBufferPoolFreeBufferNotification: CFStringRef;

    pub fn CVPixelBufferPoolGetTypeID() -> CFTypeID;
    pub fn CVPixelBufferPoolRetain(pixelBufferPool: CVPixelBufferPoolRef) -> CVPixelBufferPoolRef;
    pub fn CVPixelBufferPoolRelease(pixelBufferPool: CVPixelBufferPoolRef);
    pub fn CVPixelBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        pixelBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVPixelBufferPoolRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolGetAttributes(pool: CVPixelBufferPoolRef) -> CFDictionaryRef;
    pub fn CVPixelBufferPoolGetPixelBufferAttributes(pool: CVPixelBufferPoolRef) -> CFDictionaryRef;
    pub fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        auxAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolFlush(pool: CVPixelBufferPoolRef, options: CVPixelBufferPoolFlushFlags);
}

pub enum CVPixelBufferPoolKeys {
    MinimumBufferCount,
    MaximumBufferAge,
    AllocationThreshold,
    FreeBufferNotification,
}

impl From<CVPixelBufferPoolKeys> for CFStringRef {
    fn from(key: CVPixelBufferPoolKeys) -> CFStringRef {
        match key {
            CVPixelBufferPoolKeys::MinimumBufferCount => unsafe { kCVPixelBufferPoolMinimumBufferCountKey },
            CVPixelBufferPoolKeys::MaximumBufferAge => unsafe { kCVPixelBufferPoolMaximumBufferAgeKey },
            CVPixelBufferPoolKeys::AllocationThreshold => unsafe { kCVPixelBufferPoolAllocationThresholdKey },
            CVPixelBufferPoolKeys::FreeBufferNotification => unsafe { kCVPixelBufferPoolFreeBufferNotification },
        }
    }
}

impl From<CVPixelBufferPoolKeys> for CFString {
    fn from(key: CVPixelBufferPoolKeys) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub struct CVPixelBufferPool(CVPixelBufferPoolRef);

impl Drop for CVPixelBufferPool {
    fn drop(&mut self) {
        unsafe { CVPixelBufferPoolRelease(self.0) }
    }
}

impl_TCFType!(CVPixelBufferPool, CVPixelBufferPoolRef, CVPixelBufferPoolGetTypeID);
impl_CFTypeDescription!(CVPixelBufferPool);

impl CVPixelBufferPool {
    #[inline]
    pub fn new(
        pool_attributes: Option<&CFDictionary<CFString, CFType>>,
        pixel_buffer_attributes: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVPixelBufferPool, CVReturn> {
        let mut pool: CVPixelBufferPoolRef = null_mut();
        let status = unsafe {
            CVPixelBufferPoolCreate(
                kCFAllocatorDefault,
                pool_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                pixel_buffer_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
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
            let attributes = CVPixelBufferPoolGetAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    #[inline]
    pub fn get_pixel_buffer_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVPixelBufferPoolGetPixelBufferAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    #[inline]
    pub fn create_pixel_buffer(&self) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe { CVPixelBufferPoolCreatePixelBuffer(kCFAllocatorDefault, self.as_concrete_TypeRef(), &mut pixel_buffer) };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn create_pixel_buffer_with_aux_attributes(&self, auxAttributes: Option<&CFDictionary<CFString, CFType>>) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe {
            CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                auxAttributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                &mut pixel_buffer,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }
}
