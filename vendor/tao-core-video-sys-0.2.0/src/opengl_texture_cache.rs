use core_foundation_sys::{
    base::{CFAllocatorRef, CFTypeID},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};

use crate::{
    base::CVOptionFlags,
    image_buffer::CVImageBufferRef,
    opengl_buffer::{CGLContextObj, CGLPixelFormatObj},
    opengl_texture::CVOpenGLTextureRef,
    return_::CVReturn,
};

pub type CVOpenGLTextureCacheRef = CVImageBufferRef;

extern "C" {
    pub static kCVOpenGLTextureCacheChromaSamplingModeKey: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeAutomatic: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeHighestQuality: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeBestPerformance: CFStringRef;

    pub fn CVOpenGLTextureCacheGetTypeID() -> CFTypeID;
    pub fn CVOpenGLTextureCacheRetain(
        textureCache: CVOpenGLTextureCacheRef,
    ) -> CVOpenGLTextureCacheRef;
    pub fn CVOpenGLTextureCacheRelease(textureCache: CVOpenGLTextureCacheRef);
    pub fn CVOpenGLTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        cglContext: CGLContextObj,
        cglPixelFormat: CGLPixelFormatObj,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVOpenGLTextureCacheRef,
    ) -> CVReturn;
    pub fn CVOpenGLTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVOpenGLTextureCacheRef,
        sourceImage: CVImageBufferRef,
        attributes: CFDictionaryRef,
        textureOut: *mut CVOpenGLTextureRef,
    ) -> CVReturn;
    pub fn CVOpenGLTextureCacheFlush(textureCache: CVOpenGLTextureCacheRef, options: CVOptionFlags);
}
