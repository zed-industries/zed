use std::ptr::{null, null_mut};

use core_foundation::{
    array::CFArrayRef,
    base::{kCFAllocatorDefault, Boolean, CFAllocatorRef, CFType, CFTypeID, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};

use crate::{
    base::CVOptionFlags,
    buffer::TCVBuffer,
    image_buffer::{CVImageBufferRef, TCVImageBuffer},
    r#return::{kCVReturnInvalidArgument, kCVReturnSuccess, CVReturn},
    OSType,
};

pub type CVPixelBufferRef = CVImageBufferRef;

#[inline]
const fn fourcc(code: &[u8; 4]) -> u32 {
    ((code[0] as u32) << 24) | ((code[1] as u32) << 16) | ((code[2] as u32) << 8) | ((code[3] as u32) << 0)
}

pub type CVPixelBufferLockFlags = u64;

pub const kCVPixelFormatType_1Monochrome: OSType = 0x00000001; // 1 bit indexed
pub const kCVPixelFormatType_2Indexed: OSType = 0x00000002; // 2 bit indexed
pub const kCVPixelFormatType_4Indexed: OSType = 0x00000004; // 4 bit indexed
pub const kCVPixelFormatType_8Indexed: OSType = 0x00000008; // 8 bit indexed
pub const kCVPixelFormatType_1IndexedGray_WhiteIsZero: OSType = 0x00000021; // 1 bit indexed gray, white is zero
pub const kCVPixelFormatType_2IndexedGray_WhiteIsZero: OSType = 0x00000022; // 2 bit indexed gray, white is zero
pub const kCVPixelFormatType_4IndexedGray_WhiteIsZero: OSType = 0x00000024; // 4 bit indexed gray, white is zero
pub const kCVPixelFormatType_8IndexedGray_WhiteIsZero: OSType = 0x00000028; // 8 bit indexed gray, white is zero
pub const kCVPixelFormatType_16BE555: OSType = 0x00000010; // 16 bit BE RGB 555
pub const kCVPixelFormatType_16LE555: OSType = fourcc(b"L555"); // 16 bit LE RGB 555
pub const kCVPixelFormatType_16LE5551: OSType = fourcc(b"5551"); // 16 bit LE RGB 5551
pub const kCVPixelFormatType_16BE565: OSType = fourcc(b"B565"); // 16 bit BE RGB 565
pub const kCVPixelFormatType_16LE565: OSType = fourcc(b"L565"); // 16 bit LE RGB 565
pub const kCVPixelFormatType_24RGB: OSType = 0x00000018; // 24 bit RGB
pub const kCVPixelFormatType_24BGR: OSType = fourcc(b"24BG"); // 24 bit BGR
pub const kCVPixelFormatType_32ARGB: OSType = 0x00000020; // 32 bit ARGB
pub const kCVPixelFormatType_32BGRA: OSType = fourcc(b"BGRA"); // 32 bit BGRA
pub const kCVPixelFormatType_32ABGR: OSType = fourcc(b"ABGR"); // 32 bit ABGR
pub const kCVPixelFormatType_32RGBA: OSType = fourcc(b"RGBA"); // 32 bit RGBA
pub const kCVPixelFormatType_64ARGB: OSType = fourcc(b"b64a"); // 64 bit ARGB, 16-bit big-endian samples
pub const kCVPixelFormatType_48RGB: OSType = fourcc(b"b48r"); // 48 bit RGB, 16-bit big-endian samples
pub const kCVPixelFormatType_32AlphaGray: OSType = fourcc(b"b32a"); // 32 bit AlphaGray, 16-bit big-endian samples, black is zero
pub const kCVPixelFormatType_16Gray: OSType = fourcc(b"b16g"); // 16 bit Grayscale, 16-bit big-endian samples, black is zero
pub const kCVPixelFormatType_30RGB: OSType = fourcc(b"R10k"); // 30 bit RGB, 10-bit big-endian samples, 2 unused padding bits (at least
                                                              // significant end).
pub const kCVPixelFormatType_422YpCbCr8: OSType = fourcc(b"2vuy"); // Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1
pub const kCVPixelFormatType_4444YpCbCrA8: OSType = fourcc(b"v408"); // Component Y'CbCrA 8-bit 4:4:4:4, ordered Cb Y' Cr A
pub const kCVPixelFormatType_4444YpCbCrA8R: OSType = fourcc(b"r408"); // Component Y'CbCrA 8-bit 4:4:4:4, rendering format. full range alpha, zero
                                                                      // biased YUV, ordered A Y' Cb Cr
pub const kCVPixelFormatType_4444AYpCbCr8: OSType = fourcc(b"y408"); // Component Y'CbCrA 8-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video
                                                                     // range Y'CbCr.
pub const kCVPixelFormatType_4444AYpCbCr16: OSType = fourcc(b"y416"); // Component Y'CbCrA 16-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video
                                                                      // range Y'CbCr, 16-bit little-endian samples.
pub const kCVPixelFormatType_4444AYpCbCrFloat: OSType = fourcc(b"r4fl"); // Component AY'CbCr single precision floating-point 4:4:4:4
pub const kCVPixelFormatType_444YpCbCr8: OSType = fourcc(b"v308"); // Component Y'CbCr 8-bit 4:4:4
pub const kCVPixelFormatType_422YpCbCr16: OSType = fourcc(b"v216"); // Component Y'CbCr 10,12,14,16-bit 4:2:2
pub const kCVPixelFormatType_422YpCbCr10: OSType = fourcc(b"v210"); // Component Y'CbCr 10-bit 4:2:2
pub const kCVPixelFormatType_444YpCbCr10: OSType = fourcc(b"v410"); // Component Y'CbCr 10-bit 4:4:4
pub const kCVPixelFormatType_420YpCbCr8Planar: OSType = fourcc(b"y420"); // Planar Component Y'CbCr 8-bit 4:2:0.  baseAddr points to a big-endian
                                                                         // CVPlanarPixelBufferInfo_YCbCrPlanar struct
pub const kCVPixelFormatType_420YpCbCr8PlanarFullRange: OSType = fourcc(b"f420"); // Planar Component Y'CbCr 8-bit 4:2:0, full range.  baseAddr points to a
                                                                                  // big-endian CVPlanarPixelBufferInfo_YCbCrPlanar struct
pub const kCVPixelFormatType_422YpCbCr_4A_8BiPlanar: OSType = fourcc(b"a2vy"); // First plane: Video-range Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1;
                                                                               // second plane: alpha 8-bit 0-255
pub const kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange: OSType = fourcc(b"420v"); // Bi-Planar Component Y'CbCr 8-bit 4:2:0, video-range (luma=[16,235]
                                                                                     // chroma=[16,240]).  baseAddr points to a big-endian
                                                                                     // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_420YpCbCr8BiPlanarFullRange: OSType = fourcc(b"420f"); // Bi-Planar Component Y'CbCr 8-bit 4:2:0, full-range (luma=[0,255]
                                                                                    // chroma=[1,255]).  baseAddr points to a big-endian
                                                                                    // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_422YpCbCr8BiPlanarVideoRange: OSType = fourcc(b"422v"); // Bi-Planar Component Y'CbCr 8-bit 4:2:2, video-range (luma=[16,235]
                                                                                     // chroma=[16,240]).  baseAddr points to a big-endian
                                                                                     // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_422YpCbCr8BiPlanarFullRange: OSType = fourcc(b"422f"); // Bi-Planar Component Y'CbCr 8-bit 4:2:2, full-range (luma=[0,255]
                                                                                    // chroma=[1,255]).  baseAddr points to a big-endian
                                                                                    // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_444YpCbCr8BiPlanarVideoRange: OSType = fourcc(b"444v"); // Bi-Planar Component Y'CbCr 8-bit 4:4:4, video-range (luma=[16,235]
                                                                                     // chroma=[16,240]).  baseAddr points to a big-endian
                                                                                     // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_444YpCbCr8BiPlanarFullRange: OSType = fourcc(b"444f"); // Bi-Planar Component Y'CbCr 8-bit 4:4:4, full-range (luma=[0,255]
                                                                                    // chroma=[1,255]).  baseAddr points to a big-endian
                                                                                    // CVPlanarPixelBufferInfo_YCbCrBiPlanar struct
pub const kCVPixelFormatType_422YpCbCr8_yuvs: OSType = fourcc(b"yuvs"); // Component Y'CbCr 8-bit 4:2:2, ordered Y'0 Cb Y'1 Cr
pub const kCVPixelFormatType_422YpCbCr8FullRange: OSType = fourcc(b"yuvf"); // Component Y'CbCr 8-bit 4:2:2, full range, ordered Y'0 Cb Y'1 Cr
pub const kCVPixelFormatType_OneComponent8: OSType = fourcc(b"L008"); // 8 bit one component, black is zero
pub const kCVPixelFormatType_TwoComponent8: OSType = fourcc(b"2C08"); // 8 bit two component, black is zero
pub const kCVPixelFormatType_30RGBLEPackedWideGamut: OSType = fourcc(b"w30r"); // little-endian RGB101010, 2 MSB are zero, wide-gamut (384-895)
pub const kCVPixelFormatType_ARGB2101010LEPacked: OSType = fourcc(b"l10r"); // little-endian ARGB2101010 full-range ARGB
pub const kCVPixelFormatType_40ARGBLEWideGamut: OSType = fourcc(b"w40a"); // little-endian ARGB10101010, each 10 bits in the MSBs of 16bits, wide-gamut
                                                                          // (384-895, including alpha)
pub const kCVPixelFormatType_40ARGBLEWideGamutPremultiplied: OSType = fourcc(b"w40m"); // little-endian ARGB10101010, each 10 bits in the MSBs of 16bits, wide-gamut
                                                                                       // (384-895, including alpha). Alpha premultiplied
pub const kCVPixelFormatType_OneComponent10: OSType = fourcc(b"L010"); // 10 bit little-endian one component, stored as 10 MSBs of 16 bits, black is
                                                                       // zero
pub const kCVPixelFormatType_OneComponent12: OSType = fourcc(b"L012"); // 12 bit little-endian one component, stored as 12 MSBs of 16 bits, black is
                                                                       // zero
pub const kCVPixelFormatType_OneComponent16: OSType = fourcc(b"L016"); // 16 bit little-endian one component, black is zero
pub const kCVPixelFormatType_TwoComponent16: OSType = fourcc(b"2C16"); // 16 bit little-endian two component, black is zero
pub const kCVPixelFormatType_OneComponent16Half: OSType = fourcc(b"L00h"); // 16 bit one component IEEE half-precision float, 16-bit little-endian samples
pub const kCVPixelFormatType_OneComponent32Float: OSType = fourcc(b"L00f"); // 32 bit one component IEEE float, 32-bit little-endian samples
pub const kCVPixelFormatType_TwoComponent16Half: OSType = fourcc(b"2C0h"); // 16 bit two component IEEE half-precision float, 16-bit little-endian samples
pub const kCVPixelFormatType_TwoComponent32Float: OSType = fourcc(b"2C0f"); // 32 bit two component IEEE float, 32-bit little-endian samples
pub const kCVPixelFormatType_64RGBAHalf: OSType = fourcc(b"RGhA"); // 64 bit RGBA IEEE half-precision float, 16-bit little-endian samples
pub const kCVPixelFormatType_128RGBAFloat: OSType = fourcc(b"RGfA"); // 128 bit RGBA IEEE float, 32-bit little-endian samples
pub const kCVPixelFormatType_14Bayer_GRBG: OSType = fourcc(b"grb4"); // Bayer 14-bit Little-Endian, packed in 16-bits, ordered G R G R... alternating
                                                                     // with B G B G...
pub const kCVPixelFormatType_14Bayer_RGGB: OSType = fourcc(b"rgg4"); // Bayer 14-bit Little-Endian, packed in 16-bits, ordered R G R G... alternating
                                                                     // with G B G B...
pub const kCVPixelFormatType_14Bayer_BGGR: OSType = fourcc(b"bgg4"); // Bayer 14-bit Little-Endian, packed in 16-bits, ordered B G B G... alternating
                                                                     // with G R G R...
pub const kCVPixelFormatType_14Bayer_GBRG: OSType = fourcc(b"gbr4"); // Bayer 14-bit Little-Endian, packed in 16-bits, ordered G B G B... alternating
                                                                     // with R G R G...
pub const kCVPixelFormatType_DisparityFloat16: OSType = fourcc(b"hdis"); // IEEE754-2008 binary16 (half float), describing the normalized shift when
                                                                         // comparing two images. Units are 1/meters: ( pixelShift / (pixelFocalLength *
                                                                         // baselineInMeters) )
pub const kCVPixelFormatType_DisparityFloat32: OSType = fourcc(b"fdis"); // IEEE754-2008 binary32 float, describing the normalized shift when comparing
                                                                         // two images. Units are 1/meters: ( pixelShift / (pixelFocalLength *
                                                                         // baselineInMeters) )
pub const kCVPixelFormatType_DepthFloat16: OSType = fourcc(b"hdep"); // IEEE754-2008 binary16 (half float), describing the depth (distance to an
                                                                     // object) in meters
pub const kCVPixelFormatType_DepthFloat32: OSType = fourcc(b"fdep"); // IEEE754-2008 binary32 float, describing the depth (distance to an object) in
                                                                     // meters
pub const kCVPixelFormatType_420YpCbCr10BiPlanarVideoRange: OSType = fourcc(b"x420"); // 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, video-range
                                                                                      // (luma=[64,940] chroma=[64,960])
pub const kCVPixelFormatType_422YpCbCr10BiPlanarVideoRange: OSType = fourcc(b"x422"); // 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, video-range
                                                                                      // (luma=[64,940] chroma=[64,960])
pub const kCVPixelFormatType_444YpCbCr10BiPlanarVideoRange: OSType = fourcc(b"x444"); // 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, video-range
                                                                                      // (luma=[64,940] chroma=[64,960])
pub const kCVPixelFormatType_420YpCbCr10BiPlanarFullRange: OSType = fourcc(b"xf20"); // 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, full-range (Y
                                                                                     // range 0-1023)
pub const kCVPixelFormatType_422YpCbCr10BiPlanarFullRange: OSType = fourcc(b"xf22"); // 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, full-range (Y
                                                                                     // range 0-1023)
pub const kCVPixelFormatType_444YpCbCr10BiPlanarFullRange: OSType = fourcc(b"xf44"); // 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, full-range (Y
                                                                                     // range 0-1023)
pub const kCVPixelFormatType_420YpCbCr8VideoRange_8A_TriPlanar: OSType = fourcc(b"v0a8"); // first and second planes as per 420YpCbCr8BiPlanarVideoRange (420v), alpha 8
                                                                                          // bits in third plane full-range.  No CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_16VersatileBayer: OSType = fourcc(b"bp16"); // Single plane Bayer 16-bit little-endian sensor element ("sensel") samples
                                                                         // from full-size decoding of ProRes RAW images; Bayer pattern (sensel ordering)
                                                                         // and other raw conversion information is described via buffer attachments
pub const kCVPixelFormatType_64RGBA_DownscaledProResRAW: OSType = fourcc(b"bp64"); // Single plane 64-bit RGBA (16-bit little-endian samples) from downscaled
                                                                                   // decoding of ProRes RAW images; components--which may not be co-sited with one
                                                                                   // another--are sensel values and require raw conversion, information for which
                                                                                   // is described via buffer attachments
pub const kCVPixelFormatType_422YpCbCr16BiPlanarVideoRange: OSType = fourcc(b"sv22"); // 2 plane YCbCr16 4:2:2, video-range (luma=[4096,60160] chroma=[4096,61440])
pub const kCVPixelFormatType_444YpCbCr16BiPlanarVideoRange: OSType = fourcc(b"sv44"); // 2 plane YCbCr16 4:4:4, video-range (luma=[4096,60160] chroma=[4096,61440])
pub const kCVPixelFormatType_444YpCbCr16VideoRange_16A_TriPlanar: OSType = fourcc(b"s4as"); // 3 plane video-range YCbCr16 4:4:4 with 16-bit full-range alpha
                                                                                            // (luma=[4096,60160] chroma=[4096,61440] alpha=[0,65535]).  No
                                                                                            // CVPlanarPixelBufferInfo struct.

pub const kCVPixelFormatType_Lossless_32BGRA: OSType = fourcc(b"&BGA"); // Lossless-compressed form of kCVPixelFormatType_32BGRA.

// Lossless-compressed Bi-planar YCbCr pixel format types
pub const kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarVideoRange: OSType = fourcc(b"&8v0"); // Lossless-compressed form of kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange.
                                                                                              // No CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarFullRange: OSType = fourcc(b"&8f0"); // Lossless-compressed form of kCVPixelFormatType_420YpCbCr8BiPlanarFullRange.
                                                                                             // No CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_Lossless_420YpCbCr10PackedBiPlanarVideoRange: OSType = fourcc(b"&xv0"); // Lossless-compressed-packed form of
                                                                                                     // kCVPixelFormatType_420YpCbCr10BiPlanarVideoRange.  No CVPlanarPixelBufferInfo
                                                                                                     // struct. Format is compressed-packed with no padding bits between pixels.
pub const kCVPixelFormatType_Lossless_422YpCbCr10PackedBiPlanarVideoRange: OSType = fourcc(b"&xv2"); // Lossless-compressed form of kCVPixelFormatType_422YpCbCr10BiPlanarVideoRange.
                                                                                                     // No CVPlanarPixelBufferInfo struct. Format is compressed-packed with no
                                                                                                     // padding bits between pixels.

pub const kCVPixelFormatType_Lossy_32BGRA: OSType = fourcc(b"-BGA"); // Lossy-compressed form of kCVPixelFormatType_32BGRA. No
                                                                     // CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarVideoRange: OSType = fourcc(b"-8v0"); // Lossy-compressed form of kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange.  No
                                                                                           // CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarFullRange: OSType = fourcc(b"-8f0"); // Lossy-compressed form of kCVPixelFormatType_420YpCbCr8BiPlanarFullRange.  No
                                                                                          // CVPlanarPixelBufferInfo struct.
pub const kCVPixelFormatType_Lossy_420YpCbCr10PackedBiPlanarVideoRange: OSType = fourcc(b"-xv0"); // Lossy-compressed form of kCVPixelFormatType_420YpCbCr10BiPlanarVideoRange.
                                                                                                  // No CVPlanarPixelBufferInfo struct. Format is compressed-packed with no
                                                                                                  // padding bits between pixels.
pub const kCVPixelFormatType_Lossy_422YpCbCr10PackedBiPlanarVideoRange: OSType = fourcc(b"-xv2"); // Lossy-compressed form of kCVPixelFormatType_422YpCbCr10BiPlanarVideoRange.
                                                                                                  // No CVPlanarPixelBufferInfo struct. Format is compressed-packed with no
                                                                                                  // padding bits between pixels.

pub const kCVPixelBufferLock_ReadOnly: CVPixelBufferLockFlags = 0x00000001;

#[repr(C)]
pub struct CVPlanarComponentInfo {
    pub offset: i32,
    pub rowBytes: u32,
}

#[repr(C)]
pub struct CVPlanarPixelBufferInfo {
    pub componentInfo: [CVPlanarComponentInfo; 1],
}

#[repr(C)]
pub struct CVPlanarPixelBufferInfo_YCbCrPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCb: CVPlanarComponentInfo,
    pub componentInfoCr: CVPlanarComponentInfo,
}

#[repr(C)]
pub struct CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCbCr: CVPlanarComponentInfo,
}

pub type CVPixelBufferReleaseBytesCallback = extern "C" fn(releaseRefCon: *mut c_void, baseAddress: *const *const c_void);
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
    #[cfg(target_os = "ios")]
    pub static kCVPixelBufferOpenGLESCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferMetalCompatibilityKey: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kCVPixelBufferOpenGLTextureCacheCompatibilityKey: CFStringRef;
    #[cfg(target_os = "ios")]
    pub static kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferVersatileBayerKey_BayerPattern: CFStringRef;
}

pub const kCVVersatileBayer_BayerPattern_RGGB: u32 = 0;
pub const kCVVersatileBayer_BayerPattern_GRBG: u32 = 1;
pub const kCVVersatileBayer_BayerPattern_GBRG: u32 = 2;
pub const kCVVersatileBayer_BayerPattern_BGGR: u32 = 3;

extern "C" {
    pub static kCVPixelBufferProResRAWKey_SenselSitingOffsets: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_BlackLevel: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_WhiteLevel: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_WhiteBalanceCCT: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_ColorMatrix: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_GainFactor: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_RecommendedCrop: CFStringRef;
    pub static kCVPixelBufferProResRAWKey_MetadataExtension: CFStringRef;
}

extern "C" {
    pub fn CVPixelBufferGetTypeID() -> CFTypeID;
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
    pub fn CVPixelBufferCreateWithBytes(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        pixelFormatType: OSType,
        baseAddress: *mut c_void,
        bytesPerRow: size_t,
        releaseCallback: CVPixelBufferReleaseBytesCallback,
        releaseRefCon: *mut c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferCreateWithPlanarBytes(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        pixelFormatType: OSType,
        dataPtr: *mut c_void,
        dataSize: size_t,
        numberOfPlanes: size_t,
        planeBaseAddress: *const *mut c_void,
        planeWidth: *const size_t,
        planeHeight: *const size_t,
        planeBytesPerRow: *const size_t,
        releaseCallback: CVPixelBufferReleasePlanarBytesCallback,
        releaseRefCon: *mut c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferLockBaseAddress(pixelBuffer: CVPixelBufferRef, lockFlags: CVOptionFlags) -> CVReturn;
    pub fn CVPixelBufferUnlockBaseAddress(pixelBuffer: CVPixelBufferRef, unlockFlags: CVOptionFlags) -> CVReturn;
    pub fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;

    pub fn CVPixelBufferGetBaseAddress(pixelBuffer: CVPixelBufferRef) -> *mut c_void;
    pub fn CVPixelBufferGetBytesPerRow(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetDataSize(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetWidthOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t) -> size_t;
    pub fn CVPixelBufferGetHeightOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t) -> size_t;
    pub fn CVPixelBufferGetBaseAddressOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t) -> *mut c_void;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t) -> size_t;
    pub fn CVPixelBufferGetExtendedPixels(
        pixelBuffer: CVPixelBufferRef,
        extraColumnsOnLeft: *const size_t,
        extraColumnsOnRight: *const size_t,
        extraRowsOnTop: *const size_t,
        extraRowsOnBottom: *const size_t,
    );
    pub fn CVPixelBufferFillExtendedPixels(pixelBuffer: CVPixelBufferRef) -> CVReturn;
    pub fn CVPixelBufferCopyCreationAttributes(pixelBuffer: CVPixelBufferRef) -> CFDictionaryRef;
}

pub enum CVPixelBufferKeys {
    PixelFormatType,
    MemoryAllocator,
    Width,
    Height,
    ExtendedPixelsLeft,
    ExtendedPixelsTop,
    ExtendedPixelsRight,
    ExtendedPixelsBottom,
    BytesPerRowAlignment,
    CGBitmapContextCompatibility,
    CGImageCompatibility,
    OpenGLCompatibility,
    PlaneAlignment,
    IOSurfaceProperties,
    #[cfg(target_os = "ios")]
    OpenGLESCompatibility,
    MetalCompatibility,
    #[cfg(target_os = "macos")]
    OpenGLTextureCacheCompatibility,
    #[cfg(target_os = "ios")]
    OpenGLESTextureCacheCompatibility,
    VersatileBayerKey_BayerPattern,
}

impl From<CVPixelBufferKeys> for CFStringRef {
    fn from(key: CVPixelBufferKeys) -> Self {
        unsafe {
            match key {
                CVPixelBufferKeys::PixelFormatType => kCVPixelBufferPixelFormatTypeKey,
                CVPixelBufferKeys::MemoryAllocator => kCVPixelBufferMemoryAllocatorKey,
                CVPixelBufferKeys::Width => kCVPixelBufferWidthKey,
                CVPixelBufferKeys::Height => kCVPixelBufferHeightKey,
                CVPixelBufferKeys::ExtendedPixelsLeft => kCVPixelBufferExtendedPixelsLeftKey,
                CVPixelBufferKeys::ExtendedPixelsTop => kCVPixelBufferExtendedPixelsTopKey,
                CVPixelBufferKeys::ExtendedPixelsRight => kCVPixelBufferExtendedPixelsRightKey,
                CVPixelBufferKeys::ExtendedPixelsBottom => kCVPixelBufferExtendedPixelsBottomKey,
                CVPixelBufferKeys::BytesPerRowAlignment => kCVPixelBufferBytesPerRowAlignmentKey,
                CVPixelBufferKeys::CGBitmapContextCompatibility => kCVPixelBufferCGBitmapContextCompatibilityKey,
                CVPixelBufferKeys::CGImageCompatibility => kCVPixelBufferCGImageCompatibilityKey,
                CVPixelBufferKeys::OpenGLCompatibility => kCVPixelBufferOpenGLCompatibilityKey,
                CVPixelBufferKeys::PlaneAlignment => kCVPixelBufferPlaneAlignmentKey,
                CVPixelBufferKeys::IOSurfaceProperties => kCVPixelBufferIOSurfacePropertiesKey,
                #[cfg(target_os = "ios")]
                CVPixelBufferKeys::OpenGLESCompatibility => kCVPixelBufferOpenGLESCompatibilityKey,
                CVPixelBufferKeys::MetalCompatibility => kCVPixelBufferMetalCompatibilityKey,
                #[cfg(target_os = "macos")]
                CVPixelBufferKeys::OpenGLTextureCacheCompatibility => kCVPixelBufferOpenGLTextureCacheCompatibilityKey,
                #[cfg(target_os = "ios")]
                CVPixelBufferKeys::OpenGLESTextureCacheCompatibility => kCVPixelBufferOpenGLESTextureCacheCompatibilityKey,
                CVPixelBufferKeys::VersatileBayerKey_BayerPattern => kCVPixelBufferVersatileBayerKey_BayerPattern,
            }
        }
    }
}

impl From<CVPixelBufferKeys> for CFString {
    fn from(key: CVPixelBufferKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

declare_TCFType!(CVPixelBuffer, CVPixelBufferRef);
impl_TCFType!(CVPixelBuffer, CVPixelBufferRef, CVPixelBufferGetTypeID);
impl_CFTypeDescription!(CVPixelBuffer);

impl TCVBuffer for CVPixelBuffer {}
impl TCVImageBuffer for CVPixelBuffer {}

impl CVPixelBuffer {
    #[inline]
    pub fn new(
        pixel_format: OSType,
        width: usize,
        height: usize,
        options: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe {
            CVPixelBufferCreate(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format,
                options.map_or(null(), |options| options.as_concrete_TypeRef()),
                &mut pixel_buffer,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub unsafe fn new_with_bytes(
        pixel_format: OSType,
        width: usize,
        height: usize,
        base_address: *mut c_void,
        bytes_per_row: usize,
        release_callback: CVPixelBufferReleaseBytesCallback,
        release_ref_con: *mut c_void,
        options: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe {
            CVPixelBufferCreateWithBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format,
                base_address,
                bytes_per_row,
                release_callback,
                release_ref_con,
                options.map_or(null(), |options| options.as_concrete_TypeRef()),
                &mut pixel_buffer,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub unsafe fn new_with_planar_bytes(
        pixel_format: OSType,
        width: usize,
        height: usize,
        data_ptr: *mut c_void,
        data_size: usize,
        number_of_planes: usize,
        plane_base_address: Vec<*mut c_void>,
        plane_width: Vec<usize>,
        plane_height: Vec<usize>,
        plane_bytes_per_row: Vec<usize>,
        release_callback: CVPixelBufferReleasePlanarBytesCallback,
        release_ref_con: *mut c_void,
        options: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVPixelBuffer, CVReturn> {
        if plane_base_address.len() != number_of_planes ||
            plane_width.len() != number_of_planes ||
            plane_height.len() != number_of_planes ||
            plane_bytes_per_row.len() != number_of_planes
        {
            return Err(kCVReturnInvalidArgument);
        }
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe {
            CVPixelBufferCreateWithPlanarBytes(
                kCFAllocatorDefault,
                width,
                height,
                pixel_format,
                data_ptr,
                data_size,
                number_of_planes,
                plane_base_address.as_ptr(),
                plane_width.as_ptr(),
                plane_height.as_ptr(),
                plane_bytes_per_row.as_ptr(),
                release_callback,
                release_ref_con,
                options.map_or(null(), |options| options.as_concrete_TypeRef()),
                &mut pixel_buffer,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn lock_base_address(&self, options: CVPixelBufferLockFlags) -> CVReturn {
        unsafe { CVPixelBufferLockBaseAddress(self.as_concrete_TypeRef(), options) }
    }

    #[inline]
    pub fn unlock_base_address(&self, options: CVPixelBufferLockFlags) -> CVReturn {
        unsafe { CVPixelBufferUnlockBaseAddress(self.as_concrete_TypeRef(), options) }
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_pixel_format(&self) -> OSType {
        unsafe { CVPixelBufferGetPixelFormatType(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub unsafe fn get_base_address(&self) -> *mut c_void {
        unsafe { CVPixelBufferGetBaseAddress(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_bytes_per_row(&self) -> usize {
        unsafe { CVPixelBufferGetBytesPerRow(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_data_size(&self) -> usize {
        unsafe { CVPixelBufferGetDataSize(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { CVPixelBufferIsPlanar(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_plane_count(&self) -> usize {
        unsafe { CVPixelBufferGetPlaneCount(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_width_of_plane(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetWidthOfPlane(self.as_concrete_TypeRef(), plane_index) }
    }

    #[inline]
    pub fn get_height_of_plane(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetHeightOfPlane(self.as_concrete_TypeRef(), plane_index) }
    }

    #[inline]
    pub unsafe fn get_base_address_of_plane(&self, plane_index: usize) -> *mut c_void {
        unsafe { CVPixelBufferGetBaseAddressOfPlane(self.as_concrete_TypeRef(), plane_index) }
    }

    #[inline]
    pub fn get_bytes_per_row_of_plane(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetBytesPerRowOfPlane(self.as_concrete_TypeRef(), plane_index) }
    }

    #[inline]
    pub fn get_extended_pixels(&self) -> (usize, usize, usize, usize) {
        unsafe {
            let mut left = 0;
            let mut right = 0;
            let mut top = 0;
            let mut bottom = 0;
            CVPixelBufferGetExtendedPixels(self.as_concrete_TypeRef(), &mut left, &mut right, &mut top, &mut bottom);
            (left, right, top, bottom)
        }
    }

    #[inline]
    pub fn fill_extended_pixels(&self) -> CVReturn {
        unsafe { CVPixelBufferFillExtendedPixels(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn copy_creation_attributes(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let attributes = CVPixelBufferCopyCreationAttributes(self.as_concrete_TypeRef());
            if attributes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(attributes))
            }
        }
    }
}
