use core_foundation_sys::{
    array::CFArrayRef,
    base::{Boolean, CFAllocatorRef, CFTypeID},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use libc::{c_void, size_t};

use crate::{base::CVOptionFlags, image_buffer::CVImageBufferRef, return_::CVReturn, OSType};

const fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

pub type CVPixelBufferLockFlags = u64;
pub type CVPixelBufferRef = CVImageBufferRef;

pub const kCVPixelFormatType_1Monochrome: OSType = 0x00000001; /* 1 bit indexed */
pub const kCVPixelFormatType_2Indexed: OSType = 0x00000002; /* 2 bit indexed */
pub const kCVPixelFormatType_4Indexed: OSType = 0x00000004; /* 4 bit indexed */
pub const kCVPixelFormatType_8Indexed: OSType = 0x00000008; /* 8 bit indexed */
pub const kCVPixelFormatType_1IndexedGray_WhiteIsZero: OSType = 0x00000021; /* 1 bit indexed gray, white is zero */
pub const kCVPixelFormatType_2IndexedGray_WhiteIsZero: OSType = 0x00000022; /* 2 bit indexed gray, white is zero */
pub const kCVPixelFormatType_4IndexedGray_WhiteIsZero: OSType = 0x00000024; /* 4 bit indexed gray, white is zero */
pub const kCVPixelFormatType_8IndexedGray_WhiteIsZero: OSType = 0x00000028; /* 8 bit indexed gray, white is zero */
pub const kCVPixelFormatType_16BE555: OSType = 0x00000010; /* 16 bit BE RGB 555 */
pub const kCVPixelFormatType_16LE555: OSType = as_u32_be(b"L555"); /* 16 bit LE RGB 555 */
pub const kCVPixelFormatType_16LE5551: OSType = as_u32_be(b"5551"); /* 16 bit LE RGB 5551 */
pub const kCVPixelFormatType_16BE565: OSType = as_u32_be(b"B565"); /* 16 bit BE RGB 565 */
pub const kCVPixelFormatType_16LE565: OSType = as_u32_be(b"L565"); /* 16 bit LE RGB 565 */
pub const kCVPixelFormatType_24RGB: OSType = 0x00000018; /* 24 bit RGB */
pub const kCVPixelFormatType_24BGR: OSType = as_u32_be(b"24BG"); /* 24 bit BGR */
pub const kCVPixelFormatType_32ARGB: OSType = 0x00000020; /* 32 bit ARGB */
pub const kCVPixelFormatType_32BGRA: OSType = as_u32_be(b"BGRA"); /* 32 bit BGRA */
pub const kCVPixelFormatType_32ABGR: OSType = as_u32_be(b"ABGR"); /* 32 bit ABGR */
pub const kCVPixelFormatType_32RGBA: OSType = as_u32_be(b"RGBA"); /* 32 bit RGBA */
pub const kCVPixelFormatType_64ARGB: OSType = as_u32_be(b"b64a"); /* 64 bit ARGB, 16-bit big-endian samples */
pub const kCVPixelFormatType_48RGB: OSType = as_u32_be(b"b48r"); /* 48 bit RGB, 16-bit big-endian samples */
pub const kCVPixelFormatType_32AlphaGray: OSType = as_u32_be(b"b32a"); /* 32 bit AlphaGray, 16-bit big-endian samples, black is zero */
pub const kCVPixelFormatType_16Gray: OSType = as_u32_be(b"b16g"); /* 16 bit Grayscale, 16-bit big-endian samples, black is zero */
pub const kCVPixelFormatType_30RGB: OSType = as_u32_be(b"R10k"); /* 30 bit RGB, 10-bit big-endian samples, 2 unused padding bits (at least significant end). */
pub const kCVPixelFormatType_422YpCbCr8: OSType = as_u32_be(b"2vuy"); /* Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1 */
pub const kCVPixelFormatType_4444YpCbCrA8: OSType = as_u32_be(b"v408"); /* Component Y'CbCrA 8-bit 4:4:4:4, ordered Cb Y' Cr A */
pub const kCVPixelFormatType_4444YpCbCrA8R: OSType = as_u32_be(b"r408"); /* Component Y'CbCrA 8-bit 4:4:4:4, rendering format. full range alpha, zero biased YUV, ordered A Y' Cb Cr */
pub const kCVPixelFormatType_4444AYpCbCr8: OSType = as_u32_be(b"y408"); /* Component Y'CbCrA 8-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video range Y'CbCr. */
pub const kCVPixelFormatType_4444AYpCbCr16: OSType = as_u32_be(b"y416"); /* Component Y'CbCrA 16-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video range Y'CbCr, 16-bit little-endian samples. */
pub const kCVPixelFormatType_444YpCbCr8: OSType = as_u32_be(b"v308"); /* Component Y'CbCr 8-bit 4:4:4 */
pub const kCVPixelFormatType_422YpCbCr16: OSType = as_u32_be(b"v216"); /* Component Y'CbCr 10,12,14,16-bit 4:2:2 */
pub const kCVPixelFormatType_422YpCbCr10: OSType = as_u32_be(b"v210"); /* Component Y'CbCr 10-bit 4:2:2 */
pub const kCVPixelFormatType_444YpCbCr10: OSType = as_u32_be(b"v410"); /* Component Y'CbCr 10-bit 4:4:4 */
pub const kCVPixelFormatType_420YpCbCr8Planar: OSType = as_u32_be(b"y420"); /* Planar Component Y'CbCr 8-bit 4:2:0.  baseAddr points to a big-endian CVPlanarPixelBufferInfo_YCbCrPlanar struct */
pub const kCVPixelFormatType_420YpCbCr8PlanarFullRange: OSType = as_u32_be(b"f420"); /* Planar Component Y'CbCr 8-bit 4:2:0, full range.  baseAddr points to a big-endian CVPlanarPixelBufferInfo_YCbCrPlanar struct */
pub const kCVPixelFormatType_422YpCbCr_4A_8BiPlanar: OSType = as_u32_be(b"a2vy"); /* First plane: Video-range Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1; second plane: alpha 8-bit 0-255 */
pub const kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange: OSType = as_u32_be(b"420v"); /* Bi-Planar Component Y'CbCr 8-bit 4:2:0, video-range (luma=[16,235] chroma=[16,240]).  baseAddr points to a big-endian CVPlanarPixelBufferInfo_YCbCrBiPlanar struct */
pub const kCVPixelFormatType_420YpCbCr8BiPlanarFullRange: OSType = as_u32_be(b"420f"); /* Bi-Planar Component Y'CbCr 8-bit 4:2:0, full-range (luma=[0,255] chroma=[1,255]).  baseAddr points to a big-endian CVPlanarPixelBufferInfo_YCbCrBiPlanar struct */
pub const kCVPixelFormatType_422YpCbCr8_yuvs: OSType = as_u32_be(b"yuvs"); /* Component Y'CbCr 8-bit 4:2:2, ordered Y'0 Cb Y'1 Cr */
pub const kCVPixelFormatType_422YpCbCr8FullRange: OSType = as_u32_be(b"yuvf"); /* Component Y'CbCr 8-bit 4:2:2, full range, ordered Y'0 Cb Y'1 Cr */
pub const kCVPixelFormatType_OneComponent8: OSType = as_u32_be(b"L008"); /* 8 bit one component, black is zero */
pub const kCVPixelFormatType_TwoComponent8: OSType = as_u32_be(b"2C08"); /* 8 bit two component, black is zero */
pub const kCVPixelFormatType_30RGBLEPackedWideGamut: OSType = as_u32_be(b"w30r"); /* little-endian RGB101010, 2 MSB are zero, wide-gamut (384-895) */
pub const kCVPixelFormatType_ARGB2101010LEPacked: OSType = as_u32_be(b"l10r"); /* little-endian ARGB2101010 full-range ARGB */
pub const kCVPixelFormatType_OneComponent16Half: OSType = as_u32_be(b"L00h"); /* 16 bit one component IEEE half-precision float, 16-bit little-endian samples */
pub const kCVPixelFormatType_OneComponent32Float: OSType = as_u32_be(b"L00f"); /* 32 bit one component IEEE float, 32-bit little-endian samples */
pub const kCVPixelFormatType_TwoComponent16Half: OSType = as_u32_be(b"2C0h"); /* 16 bit two component IEEE half-precision float, 16-bit little-endian samples */
pub const kCVPixelFormatType_TwoComponent32Float: OSType = as_u32_be(b"2C0f"); /* 32 bit two component IEEE float, 32-bit little-endian samples */
pub const kCVPixelFormatType_64RGBAHalf: OSType = as_u32_be(b"RGhA"); /* 64 bit RGBA IEEE half-precision float, 16-bit little-endian samples */
pub const kCVPixelFormatType_128RGBAFloat: OSType = as_u32_be(b"RGfA"); /* 128 bit RGBA IEEE float, 32-bit little-endian samples */
pub const kCVPixelFormatType_14Bayer_GRBG: OSType = as_u32_be(b"grb4"); /* Bayer 14-bit Little-Endian, packed in 16-bits, ordered G R G R... alternating with B G B G... */
pub const kCVPixelFormatType_14Bayer_RGGB: OSType = as_u32_be(b"rgg4"); /* Bayer 14-bit Little-Endian, packed in 16-bits, ordered R G R G... alternating with G B G B... */
pub const kCVPixelFormatType_14Bayer_BGGR: OSType = as_u32_be(b"bgg4"); /* Bayer 14-bit Little-Endian, packed in 16-bits, ordered B G B G... alternating with G R G R... */
pub const kCVPixelFormatType_14Bayer_GBRG: OSType = as_u32_be(b"gbr4"); /* Bayer 14-bit Little-Endian, packed in 16-bits, ordered G B G B... alternating with R G R G... */
pub const kCVPixelFormatType_DisparityFloat16: OSType = as_u32_be(b"hdis"); /* IEEE754-2008 binary16 (half float), describing the normalized shift when comparing two images. Units are 1/meters: ( pixelShift / (pixelFocalLength * baselineInMeters) ) */
pub const kCVPixelFormatType_DisparityFloat32: OSType = as_u32_be(b"fdis"); /* IEEE754-2008 binary32 float, describing the normalized shift when comparing two images. Units are 1/meters: ( pixelShift / (pixelFocalLength * baselineInMeters) ) */
pub const kCVPixelFormatType_DepthFloat16: OSType = as_u32_be(b"hdep"); /* IEEE754-2008 binary16 (half float), describing the depth (distance to an object) in meters */
pub const kCVPixelFormatType_DepthFloat32: OSType = as_u32_be(b"fdep"); /* IEEE754-2008 binary32 float, describing the depth (distance to an object) in meters */
pub const kCVPixelFormatType_420YpCbCr10BiPlanarVideoRange: OSType = as_u32_be(b"x420"); /* 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, video-range (luma=[64,940] chroma=[64,960]) */
pub const kCVPixelFormatType_422YpCbCr10BiPlanarVideoRange: OSType = as_u32_be(b"x422"); /* 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, video-range (luma=[64,940] chroma=[64,960]) */
pub const kCVPixelFormatType_444YpCbCr10BiPlanarVideoRange: OSType = as_u32_be(b"x444"); /* 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, video-range (luma=[64,940] chroma=[64,960]) */
pub const kCVPixelFormatType_420YpCbCr10BiPlanarFullRange: OSType = as_u32_be(b"xf20"); /* 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023) */
pub const kCVPixelFormatType_422YpCbCr10BiPlanarFullRange: OSType = as_u32_be(b"xf22"); /* 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023) */
pub const kCVPixelFormatType_444YpCbCr10BiPlanarFullRange: OSType = as_u32_be(b"xf44"); /* 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023) */

pub const kCVPixelBufferLock_ReadOnly: CVPixelBufferLockFlags = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVPlanarComponentInfo {
    /// offset from main base address to base address of this plane, big-endian
    pub offset: i32,
    /// bytes per row of this plane, big-endian
    pub rowBytes: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVPlanarPixelBufferInfo {
    pub componentInfo: [CVPlanarComponentInfo; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVPlanarPixelBufferInfo_YCbCrPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCb: CVPlanarComponentInfo,
    pub componentInfoCr: CVPlanarComponentInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCbCr: CVPlanarComponentInfo,
}

pub type CVPixelBufferReleaseBytesCallback =
    extern "C" fn(releaseRefCon: *mut c_void, baseAddress: *const *const c_void);
pub type CVPixelBufferReleasePlanarBytesCallback = extern "C" fn(
    releaseRefCon: *mut c_void,
    dataPtr: *const *const c_void,
    dataSize: size_t,
    numberOfPlanes: size_t,
    planeAddresses: *const *const c_void,
);

extern "C" {
    pub static kCVPixelBufferPixelFormatTypeKey: CFStringRef;
    pub static kCVPixelBufferMemoryAllocatorKey: CFStringRef;
    pub static kCVPixelBufferWidthKey: CFStringRef;
    pub static kCVPixelBufferHeightKey: CFStringRef;
    pub static kCVPixelBufferExtendedPixelsLeftKey: CFStringRef;
    pub static kCVPixelBufferExtendedPixelsTopKey: CFStringRef;
    pub static kCVPixelBufferExtendedPixelsRightKey: CFStringRef;
    pub static kCVPixelBufferExtendedPixelsBottomKey: CFStringRef;
    pub static kCVPixelBufferBytesPerRowAlignmentKey: CFStringRef;
    pub static kCVPixelBufferCGBitmapContextCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferCGImageCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferOpenGLCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferPlaneAlignmentKey: CFStringRef;
    pub static kCVPixelBufferIOSurfacePropertiesKey: CFStringRef;
    pub static kCVPixelBufferOpenGLESCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferMetalCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferOpenGLTextureCacheCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFStringRef;

    pub fn CVBufferGetTypeID() -> CFTypeID;
    pub fn CVPixelBufferRetain(texture: CVPixelBufferRef) -> CVPixelBufferRef;
    pub fn CVPixelBufferRelease(texture: CVPixelBufferRef);
    pub fn CVPixelBufferCreateResolvedAttributesDictionary(
        allocator: CFAllocatorRef,
        attributes: CFArrayRef,
        resolvedDictionaryOut: *mut CFDictionaryRef,
    ) -> CVReturn;
    pub fn CVPixelBufferCreate(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        pixelFormatType: OSType,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    // pub fn CVPixelBufferCreateWithBytes(allocator: CFAllocatorRef,
    //                                     width: size_t,
    //                                     height: size_t,
    //                                     pixelFormatType: OSType,
    //                                     baseAddress: *const c_void,
    //                                     bytesPerRow: size_t,
    //                                     releaseCallback: CVPixelBufferReleaseBytesCallback,
    //                                     releaseRefCon: *const c_void,
    //                                     pixelBufferAttributes: CFDictionaryRef,
    //                                     pixelBufferOut: *mut CVPixelBufferRef) -> CVReturn;
    // pub fn CVPixelBufferCreateWithPlanarBytes(allocator: CFAllocatorRef,
    //                                           width: size_t,
    //                                           height: size_t,
    //                                           pixelFormatType: OSType,
    //                                           dataPtr: *const c_void,
    //                                           dataSize: size_t,
    //                                           numberOfPlanes: size_t,
    //                                           planeBaseAddress: *const *const c_void,
    //                                           ) -> CVReturn;

    pub fn CVPixelBufferLockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        lockFlags: CVOptionFlags,
    ) -> CVReturn;
    pub fn CVPixelBufferUnlockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        unlockFlags: CVOptionFlags,
    ) -> CVReturn;
    pub fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;

    pub fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut c_void;
    pub fn CVPixelBufferGetBytesPerRow(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetWidthOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;
    pub fn CVPixelBufferGetHeightOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;
    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> *mut c_void;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;

    pub fn CVPixelBufferGetExtendedPixels(
        pixelBuffer: CVPixelBufferRef,
        extraColumnsOnLeft: *const size_t,
        extraColumnsOnRight: *const size_t,
        extraRowsOnTop: *const size_t,
        extraRowsOnBottom: *const size_t,
    );
    pub fn CVPixelBufferFillExtendedPixels(pixelBuffer: CVPixelBufferRef) -> CVReturn;
}
