use core_foundation_sys::{
    base::{Boolean, CFTypeID},
    string::CFStringRef,
};
use metal::Texture;

use crate::image_buffer::CVImageBufferRef;

pub type CVMetalTextureRef = CVImageBufferRef;

extern "C" {
    pub static kCVMetalTextureUsage: CFStringRef;

    pub fn CVMetalTextureGetTypeID() -> CFTypeID;
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> Texture;
    pub fn CVMetalTextureIsFlipped(image: CVMetalTextureRef) -> Boolean;
    // CV_EXPORT void CVMetalTextureGetCleanTexCoords( CVMetalTextureRef CV_NONNULL image,
    //                                                float lowerLeft[2],
    //                                                float lowerRight[2],
    //                                                float upperRight[2],
    //                                                float upperLeft[2] ) API_AVAILABLE(macosx(10.11), ios(8.0), tvos(9.0)) __WATCHOS_PROHIBITED;

}
