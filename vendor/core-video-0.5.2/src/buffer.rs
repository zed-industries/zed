use std::mem;

use core_foundation::{
    base::{Boolean, CFGetTypeID, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription,
    string::{CFString, CFStringRef},
};
use libc::c_void;
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

#[repr(C)]
pub struct __CVBuffer(c_void);

pub type CVBufferRef = *mut __CVBuffer;

pub type CVAttachmentMode = u32;
pub const kCVAttachmentMode_ShouldNotPropagate: CVAttachmentMode = 0;
pub const kCVAttachmentMode_ShouldPropagate: CVAttachmentMode = 1;

extern "C" {
    pub static kCVBufferPropagatedAttachmentsKey: CFStringRef;
    pub static kCVBufferNonPropagatedAttachmentsKey: CFStringRef;

    pub static kCVBufferMovieTimeKey: CFStringRef;
    pub static kCVBufferTimeValueKey: CFStringRef;
    pub static kCVBufferTimeScaleKey: CFStringRef;

    pub fn CVBufferRetain(buffer: CVBufferRef) -> CVBufferRef;
    pub fn CVBufferRelease(buffer: CVBufferRef);
    pub fn CVBufferSetAttachment(buffer: CVBufferRef, key: CFStringRef, value: CFTypeRef, attachmentMode: CVAttachmentMode);
    pub fn CVBufferGetAttachment(buffer: CVBufferRef, key: CFStringRef, attachmentMode: *mut CVAttachmentMode) -> CFTypeRef;
    pub fn CVBufferRemoveAttachment(buffer: CVBufferRef, key: CFStringRef);
    pub fn CVBufferRemoveAllAttachments(buffer: CVBufferRef);
    pub fn CVBufferGetAttachments(buffer: CVBufferRef, attachmentMode: CVAttachmentMode) -> CFDictionaryRef;
    pub fn CVBufferSetAttachments(buffer: CVBufferRef, theAttachments: CFDictionaryRef, attachmentMode: CVAttachmentMode);
    pub fn CVBufferPropagateAttachments(sourceBuffer: CVBufferRef, destinationBuffer: CVBufferRef);
    pub fn CVBufferCopyAttachments(sourceBuffer: CVBufferRef, attachmentMode: CVAttachmentMode) -> CFDictionaryRef;
    pub fn CVBufferCopyAttachment(buffer: CVBufferRef, key: CFStringRef, attachmentMode: *mut CVAttachmentMode) -> CFTypeRef;
    pub fn CVBufferHasAttachment(buffer: CVBufferRef, key: CFStringRef) -> Boolean;
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CVBuffer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("__CVBuffer", &[]));
}

pub enum CVBufferAttachmentsKeys {
    Propagated,
    NonPropagated,
}

impl From<CVBufferAttachmentsKeys> for CFStringRef {
    fn from(key: CVBufferAttachmentsKeys) -> CFStringRef {
        unsafe {
            match key {
                CVBufferAttachmentsKeys::Propagated => kCVBufferPropagatedAttachmentsKey,
                CVBufferAttachmentsKeys::NonPropagated => kCVBufferNonPropagatedAttachmentsKey,
            }
        }
    }
}

impl From<CVBufferAttachmentsKeys> for CFString {
    fn from(key: CVBufferAttachmentsKeys) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub enum CVBufferKeys {
    MovieTime,
    TimeValue,
    TimeScale,
}

impl From<CVBufferKeys> for CFStringRef {
    fn from(key: CVBufferKeys) -> CFStringRef {
        unsafe {
            match key {
                CVBufferKeys::MovieTime => kCVBufferMovieTimeKey,
                CVBufferKeys::TimeValue => kCVBufferTimeValueKey,
                CVBufferKeys::TimeScale => kCVBufferTimeScaleKey,
            }
        }
    }
}

impl From<CVBufferKeys> for CFString {
    fn from(key: CVBufferKeys) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub struct CVBuffer(CVBufferRef);

impl Drop for CVBuffer {
    fn drop(&mut self) {
        unsafe { CVBufferRelease(self.0) }
    }
}

impl CVBuffer {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CVBufferRef {
        self.0
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    pub fn into_CFType(self) -> CFType {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { CFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(reference: CVBufferRef) -> CVBuffer {
        CVBuffer(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CVBufferRef) -> CVBuffer {
        CVBuffer(CVBufferRetain(reference))
    }

    #[inline]
    pub fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn instance_of<T: TCFType>(&self) -> bool {
        self.type_of() == T::type_id()
    }
}

impl Clone for CVBuffer {
    #[inline]
    fn clone(&self) -> CVBuffer {
        unsafe { CVBuffer::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for CVBuffer {
    #[inline]
    fn eq(&self, other: &CVBuffer) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CVBuffer {}

impl_CFTypeDescription!(CVBuffer);

pub trait TCVBuffer: TCFType {
    #[inline]
    fn as_buffer(&self) -> CVBuffer {
        unsafe { CVBuffer::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr() as CVBufferRef) }
    }

    #[inline]
    fn into_buffer(self) -> CVBuffer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr() as CVBufferRef;
        mem::forget(self);
        unsafe { CVBuffer::wrap_under_create_rule(reference) }
    }
}

impl CVBuffer {
    #[inline]
    pub fn downcast<T: TCVBuffer>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe { Some(T::wrap_under_get_rule(T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void))) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_into<T: TCVBuffer>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void);
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

impl CVBuffer {
    #[inline]
    pub fn set_attachment(&self, key: &CFString, value: &CFType, attachment_mode: CVAttachmentMode) {
        unsafe {
            CVBufferSetAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef(), value.as_concrete_TypeRef(), attachment_mode);
        }
    }

    #[inline]
    pub fn get_attachment(&self, key: &CFString, attachment_mode: Option<&mut CVAttachmentMode>) -> Option<CFType> {
        unsafe {
            let attachment_mode = match attachment_mode {
                Some(attachment_mode) => attachment_mode as *mut CVAttachmentMode,
                None => std::ptr::null_mut(),
            };
            let value = CVBufferGetAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef(), attachment_mode);
            if value.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(value))
            }
        }
    }

    #[inline]
    pub fn remove_attachment(&self, key: &CFString) {
        unsafe {
            CVBufferRemoveAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef());
        }
    }

    #[inline]
    pub fn remove_all_attachments(&self) {
        unsafe {
            CVBufferRemoveAllAttachments(self.as_concrete_TypeRef());
        }
    }

    #[inline]
    pub fn get_attachments(&self, attachment_mode: CVAttachmentMode) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attachments = CVBufferGetAttachments(self.as_concrete_TypeRef(), attachment_mode);
            if attachments.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(attachments))
            }
        }
    }

    #[inline]
    pub fn set_attachments(&self, the_attachments: &CFDictionary<CFString, CFType>, attachment_mode: CVAttachmentMode) {
        unsafe {
            CVBufferSetAttachments(self.as_concrete_TypeRef(), the_attachments.as_concrete_TypeRef(), attachment_mode);
        }
    }

    #[inline]
    pub fn propagate_attachments(&self, destination_buffer: &CVBuffer) {
        unsafe {
            CVBufferPropagateAttachments(self.as_concrete_TypeRef(), destination_buffer.as_concrete_TypeRef());
        }
    }

    #[inline]
    pub fn copy_attachments(&self, attachment_mode: CVAttachmentMode) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attachments = CVBufferCopyAttachments(self.as_concrete_TypeRef(), attachment_mode);
            if attachments.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attachments))
            }
        }
    }

    #[inline]
    pub fn copy_attachment(&self, key: &CFString, attachment_mode: Option<&mut CVAttachmentMode>) -> Option<CFType> {
        unsafe {
            let attachment_mode = match attachment_mode {
                Some(attachment_mode) => attachment_mode as *mut CVAttachmentMode,
                None => std::ptr::null_mut(),
            };
            let attachment = CVBufferCopyAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef(), attachment_mode);
            if attachment.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attachment))
            }
        }
    }

    #[inline]
    pub fn has_attachment(&self, key: &CFString) -> bool {
        unsafe { CVBufferHasAttachment(self.as_concrete_TypeRef(), key.as_concrete_TypeRef()) != 0 }
    }
}
