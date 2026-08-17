use core_foundation::{
    base::{Boolean, CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};

use crate::{
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
    GLenum, GLuint,
};

pub type CVOpenGLESTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVOpenGLESTextureGetTypeID() -> CFTypeID;
    pub fn CVOpenGLESTextureGetTarget(image: CVOpenGLESTextureRef) -> GLenum;
    pub fn CVOpenGLESTextureGetName(image: CVOpenGLESTextureRef) -> GLuint;
    pub fn CVOpenGLESTextureIsFlipped(image: CVOpenGLESTextureRef) -> Boolean;
    pub fn CVOpenGLESTextureGetCleanTexCoords(
        image: CVOpenGLESTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );
}

declare_TCFType!(CVOpenGLESTexture, CVOpenGLESTextureRef);
impl_TCFType!(CVOpenGLESTexture, CVOpenGLESTextureRef, CVOpenGLESTextureGetTypeID);
impl_CFTypeDescription!(CVOpenGLESTexture);

impl TCVBuffer for CVOpenGLESTexture {}
impl TCVImageBuffer for CVOpenGLESTexture {}

impl CVOpenGLESTexture {
    #[inline]
    pub fn get_target(&self) -> GLenum {
        unsafe { CVOpenGLESTextureGetTarget(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_name(&self) -> GLuint {
        unsafe { CVOpenGLESTextureGetName(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVOpenGLESTextureIsFlipped(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_clean_tex_coords(&self) -> (f32, f32, f32, f32) {
        let mut lower_left = 0.0;
        let mut lower_right = 0.0;
        let mut upper_right = 0.0;
        let mut upper_left = 0.0;
        unsafe {
            CVOpenGLESTextureGetCleanTexCoords(self.as_concrete_TypeRef(), &mut lower_left, &mut lower_right, &mut upper_right, &mut upper_left);
        }
        (lower_left, lower_right, upper_right, upper_left)
    }
}
