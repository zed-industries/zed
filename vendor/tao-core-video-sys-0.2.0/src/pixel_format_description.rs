use core_foundation_sys::{
    array::CFArrayRef,
    base::{Boolean, CFAllocatorRef, CFIndex},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use libc::c_void;

use crate::{pixel_buffer::CVPixelBufferRef, OSType};

pub type CVFillExtendedPixelsCallBack =
    extern "C" fn(pixelBuffer: CVPixelBufferRef, refCon: *mut c_void) -> Boolean;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVFillExtendedPixelsCallBackData {
    pub version: CFIndex,
    pub fillCallBack: CVFillExtendedPixelsCallBack,
    pub refCon: *mut c_void,
}

extern "C" {
    pub static kCVPixelFormatName: CFStringRef;

    /// QuickTime/QuickDraw Pixel Format Type constant (OSType)
    pub static kCVPixelFormatConstant: CFStringRef;

    /// This is the codec type constant, i.e. '2vuy' or k422YpCbCr8CodecType
    pub static kCVPixelFormatCodecType: CFStringRef;

    /// This is the equivalent Microsoft FourCC code for this pixel format
    pub static kCVPixelFormatFourCC: CFStringRef;

    /// kCFBooleanTrue indicates that the format contains alpha and some images may be considered transparent
    /// kCFBooleanFalse indicates that there is no alpha and images are always opaque.
    pub static kCVPixelFormatContainsAlpha: CFStringRef;

    /// kCFBooleanTrue indicates that the format contains YCbCr data
    pub static kCVPixelFormatContainsYCbCr: CFStringRef;

    /// kCFBooleanTrue indicates that the format contains RGB data
    pub static kCVPixelFormatContainsRGB: CFStringRef;

    pub static kCVPixelFormatComponentRange: CFStringRef;

    pub static kCVPixelFormatComponentRange_VideoRange: CFStringRef;

    pub static kCVPixelFormatComponentRange_FullRange: CFStringRef;

    pub static kCVPixelFormatComponentRange_WideRange: CFStringRef;

    /// All buffers have one or more image planes.
    /// Each plane may contain a single or an interleaved set of components
    /// For simplicity sake,
    /// pixel formats that are not planar may place the required format keys at the top level dictionary.
    pub static kCVPixelFormatPlanes: CFStringRef;

    pub static kCVPixelFormatBlockWidth: CFStringRef;
    pub static kCVPixelFormatBlockHeight: CFStringRef;

    /// This value must be present.
    /// For simple pixel formats this will be equivalent to the traditional bitsPerPixel value.
    pub static kCVPixelFormatBitsPerBlock: CFStringRef;

    /// Used to state requirements on block multiples.  v210 would be '8' here for the horizontal case,
    /// to match the standard v210 row alignment value of 48.
    /// These may be assumed as 1 if not present.
    pub static kCVPixelFormatBlockHorizontalAlignment: CFStringRef;
    pub static kCVPixelFormatBlockVerticalAlignment: CFStringRef;

    /// CFData containing the bit pattern for a block of black pixels.  If absent, black is assumed to be all zeros.
    /// If present, this should be bitsPerPixel bits long -- if bitsPerPixel is less than a byte, repeat the bit pattern
    /// for the full byte.
    pub static kCVPixelFormatBlackBlock: CFStringRef;

    /// Subsampling information for this plane.  Assumed to be '1' if not present.
    pub static kCVPixelFormatHorizontalSubsampling: CFStringRef;
    pub static kCVPixelFormatVerticalSubsampling: CFStringRef;

    /// If present, these two keys describe the OpenGL format and type enums you would use to describe this
    /// image plane to OpenGL
    pub static kCVPixelFormatOpenGLFormat: CFStringRef;
    pub static kCVPixelFormatOpenGLType: CFStringRef;
    pub static kCVPixelFormatOpenGLInternalFormat: CFStringRef;

    /// CGBitmapInfo value, if required
    pub static kCVPixelFormatCGBitmapInfo: CFStringRef;

    /// Pixel format compatibility flags
    pub static kCVPixelFormatQDCompatibility: CFStringRef;
    pub static kCVPixelFormatCGBitmapContextCompatibility: CFStringRef;
    pub static kCVPixelFormatCGImageCompatibility: CFStringRef;
    pub static kCVPixelFormatOpenGLCompatibility: CFStringRef;
    pub static kCVPixelFormatOpenGLESCompatibility: CFStringRef;

    pub static kCVPixelFormatFillExtendedPixelsCallback: CFStringRef;

    pub fn CVPixelFormatDescriptionCreateWithPixelFormatType(
        allocator: CFAllocatorRef,
        pixelFormat: OSType,
    ) -> CFDictionaryRef;
    pub fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(
        allocator: CFAllocatorRef,
    ) -> CFArrayRef;
    pub fn CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType(
        description: CFDictionaryRef,
        pixelFormat: OSType,
    );
}

#[cfg(feature = "direct3d")]
extern "C" {
    pub static kCVPixelFormatDirect3DFormat: CFStringRef;
    pub static kCVPixelFormatDirect3DType: CFStringRef;
    pub static kCVPixelFormatDirect3DInternalFormat: CFStringRef;
    pub static kCVPixelFormatDirect3DCompatibility: CFStringRef;
}
