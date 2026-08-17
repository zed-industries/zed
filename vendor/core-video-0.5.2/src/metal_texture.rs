use core_foundation::{
    base::{Boolean, CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use metal::{foreign_types::ForeignType, MTLTexture, Texture};

use crate::{
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
};

pub type CVMetalTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVMetalTextureGetTypeID() -> CFTypeID;
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> *mut MTLTexture;
    pub fn CVMetalTextureIsFlipped(image: CVMetalTextureRef) -> Boolean;
    pub fn CVMetalTextureGetCleanTexCoords(
        image: CVMetalTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );

    pub static kCVMetalTextureUsage: CFStringRef;
    pub static kCVMetalTextureStorageMode: CFStringRef;
}

pub enum CVMetalTextureKeys {
    Usage,
    StorageMode,
}

impl From<CVMetalTextureKeys> for CFStringRef {
    fn from(key: CVMetalTextureKeys) -> Self {
        unsafe {
            match key {
                CVMetalTextureKeys::Usage => kCVMetalTextureUsage,
                CVMetalTextureKeys::StorageMode => kCVMetalTextureStorageMode,
            }
        }
    }
}

impl From<CVMetalTextureKeys> for CFString {
    fn from(key: CVMetalTextureKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

declare_TCFType!(CVMetalTexture, CVMetalTextureRef);
impl_TCFType!(CVMetalTexture, CVMetalTextureRef, CVMetalTextureGetTypeID);
impl_CFTypeDescription!(CVMetalTexture);

impl TCVBuffer for CVMetalTexture {}
impl TCVImageBuffer for CVMetalTexture {}

impl CVMetalTexture {
    #[inline]
    pub fn get_texture(&self) -> Option<Texture> {
        unsafe {
            let texture = CVMetalTextureGetTexture(self.as_concrete_TypeRef());
            if texture.is_null() {
                None
            } else {
                Some(Texture::from_ptr(texture))
            }
        }
    }

    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVMetalTextureIsFlipped(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_clean_tex_coords(&self) -> (f32, f32, f32, f32) {
        let mut lower_left = 0.0;
        let mut lower_right = 0.0;
        let mut upper_right = 0.0;
        let mut upper_left = 0.0;
        unsafe {
            CVMetalTextureGetCleanTexCoords(self.as_concrete_TypeRef(), &mut lower_left, &mut lower_right, &mut upper_right, &mut upper_left);
        }
        (lower_left, lower_right, upper_right, upper_left)
    }
}
