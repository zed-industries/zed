use core_foundation_sys::base::{Boolean, CFTypeID};

use crate::{image_buffer::CVImageBufferRef, GLenum, GLuint};

pub type CVOpenGLTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVOpenGLTextureGetTypeID() -> CFTypeID;
    pub fn CVOpenGLTextureRetain(texture: CVOpenGLTextureRef) -> CVOpenGLTextureRef;
    pub fn CVOpenGLTextureRelease(texture: CVOpenGLTextureRef);
    pub fn CVOpenGLTextureGetTarget(image: CVOpenGLTextureRef) -> GLenum;
    pub fn CVOpenGLTextureGetName(image: CVOpenGLTextureRef) -> GLuint;
    pub fn CVOpenGLTextureIsFlipped(image: CVOpenGLTextureRef) -> Boolean;
    // CV_EXPORT void CVOpenGLTextureGetCleanTexCoords( CVOpenGLTextureRef CV_NONNULL image,
    //                      GLfloat lowerLeft[2],
    //                      GLfloat lowerRight[2],
    //                      GLfloat upperRight[2],
    //                      GLfloat upperLeft[2] ) AVAILABLE_MAC_OS_X_VERSION_10_4_AND_LATER;
}
