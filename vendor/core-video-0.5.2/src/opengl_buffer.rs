use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::size_t;

use crate::{
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
    r#return::{kCVReturnSuccess, CVReturn},
    CGLContextObj, GLenum, GLint,
};

pub type CVOpenGLBufferRef = CVImageBufferRef;

extern "C" {
    pub static kCVOpenGLBufferWidth: CFStringRef;
    pub static kCVOpenGLBufferHeight: CFStringRef;
    pub static kCVOpenGLBufferTarget: CFStringRef;
    pub static kCVOpenGLBufferInternalFormat: CFStringRef;
    pub static kCVOpenGLBufferMaximumMipmapLevel: CFStringRef;

    pub fn CVOpenGLBufferGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferRetain(buffer: CVOpenGLBufferRef) -> CVOpenGLBufferRef;
    pub fn CVOpenGLBufferRelease(buffer: CVOpenGLBufferRef);
    pub fn CVOpenGLBufferCreate(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        attributes: CFDictionaryRef,
        bufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferGetAttributes(openGLBuffer: CVOpenGLBufferRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferAttach(openGLBuffer: CVOpenGLBufferRef, cglContext: CGLContextObj, face: GLenum, level: GLint, screen: GLint) -> CVReturn;
}

pub enum CVOpenGLBufferKeys {
    Width,
    Height,
    Target,
    InternalFormat,
    MaximumMipmapLevel,
}

impl From<CVOpenGLBufferKeys> for CFStringRef {
    fn from(key: CVOpenGLBufferKeys) -> Self {
        unsafe {
            match key {
                CVOpenGLBufferKeys::Width => kCVOpenGLBufferWidth,
                CVOpenGLBufferKeys::Height => kCVOpenGLBufferHeight,
                CVOpenGLBufferKeys::Target => kCVOpenGLBufferTarget,
                CVOpenGLBufferKeys::InternalFormat => kCVOpenGLBufferInternalFormat,
                CVOpenGLBufferKeys::MaximumMipmapLevel => kCVOpenGLBufferMaximumMipmapLevel,
            }
        }
    }
}

impl From<CVOpenGLBufferKeys> for CFString {
    fn from(key: CVOpenGLBufferKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

declare_TCFType!(CVOpenGLBuffer, CVOpenGLBufferRef);
impl_TCFType!(CVOpenGLBuffer, CVOpenGLBufferRef, CVOpenGLBufferGetTypeID);
impl_CFTypeDescription!(CVOpenGLBuffer);

impl TCVBuffer for CVOpenGLBuffer {}
impl TCVImageBuffer for CVOpenGLBuffer {}

impl CVOpenGLBuffer {
    #[inline]
    pub fn new(width: size_t, height: size_t, attributes: Option<&CFDictionary<CFString, CFType>>) -> Result<CVOpenGLBuffer, CVReturn> {
        let mut buffer: CVOpenGLBufferRef = null_mut();
        let status = unsafe {
            CVOpenGLBufferCreate(kCFAllocatorDefault, width, height, attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()), &mut buffer)
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVOpenGLBufferGetAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }

    #[inline]
    pub unsafe fn attach(&self, cgl_context: CGLContextObj, face: GLenum, level: GLint, screen: GLint) -> Result<(), CVReturn> {
        let status = unsafe { CVOpenGLBufferAttach(self.as_concrete_TypeRef(), cgl_context, face, level, screen) };
        if status == kCVReturnSuccess {
            Ok(())
        } else {
            Err(status)
        }
    }
}
