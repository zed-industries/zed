use core_foundation_sys::{base::CFTypeRef, dictionary::CFDictionaryRef, string::CFStringRef};

#[derive(Debug, Copy, Clone)]
pub enum __CVBuffer {}
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
    pub fn CVBufferSetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        value: CFTypeRef,
        attachmentMode: CVAttachmentMode,
    );
    pub fn CVBufferGetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachmentMode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
    pub fn CVBufferRemoveAttachment(buffer: CVBufferRef, key: CFStringRef);
    pub fn CVBufferRemoveAllAttachments(buffer: CVBufferRef);
    pub fn CVBufferGetAttachments(
        buffer: CVBufferRef,
        attachmentMode: CVAttachmentMode,
    ) -> CFDictionaryRef;
    pub fn CVBufferSetAttachments(
        buffer: CVBufferRef,
        theAttachments: CFDictionaryRef,
        attachmentMode: CVAttachmentMode,
    );
    pub fn CVBufferPropagateAttachments(sourceBuffer: CVBufferRef, destinationBuffer: CVBufferRef);

}
