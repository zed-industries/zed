use std::ptr::{null, null_mut};

use bitflags::bitflags;
use cfg_if::cfg_if;
use core_foundation::{
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    base::CGFloat,
    color_space::{CGColorRenderingIntent, CGColorSpace, CGColorSpaceRef},
    data_provider::{CGDataProvider, CGDataProviderRef},
    geometry::CGRect,
};

#[repr(C)]
pub struct __CGImage(c_void);

pub type CGImageRef = *mut __CGImage;

pub const kCGImageAlphaNone: u32 = 0;
pub const kCGImageAlphaPremultipliedLast: u32 = 1;
pub const kCGImageAlphaPremultipliedFirst: u32 = 2;
pub const kCGImageAlphaLast: u32 = 3;
pub const kCGImageAlphaFirst: u32 = 4;
pub const kCGImageAlphaNoneSkipLast: u32 = 5;
pub const kCGImageAlphaNoneSkipFirst: u32 = 6;
pub const kCGImageAlphaOnly: u32 = 7;

pub const kCGImageByteOrderMask: u32 = 0x7000;
pub const kCGImageByteOrderDefault: u32 = 0 << 12;
pub const kCGImageByteOrder16Little: u32 = 1 << 12;
pub const kCGImageByteOrder32Little: u32 = 2 << 12;
pub const kCGImageByteOrder16Big: u32 = 3 << 12;
pub const kCGImageByteOrder32Big: u32 = 4 << 12;

pub const kCGImagePixelFormatMask: u32 = 0xF000;
pub const kCGImagePixelFormatPacked: u32 = 0 << 16;
pub const kCGImagePixelFormatRGB555: u32 = 1 << 16;
pub const kCGImagePixelFormatRGB565: u32 = 2 << 16;
pub const kCGImagePixelFormatRGB101010: u32 = 3 << 16;
pub const kCGImagePixelFormatRGBCIF10: u32 = 4 << 16;

cfg_if! {
    if #[cfg(target_endian = "big")] {
        pub const kCGImageByteOrder16Host: u32 = kCGImageByteOrder16Big;
        pub const kCGImageByteOrder32Host: u32 = kCGImageByteOrder32Big;
    } else {
        pub const kCGImageByteOrder16Host: u32 = kCGImageByteOrder16Little;
        pub const kCGImageByteOrder32Host: u32 = kCGImageByteOrder32Little;
    }
}

extern "C" {
    pub fn CGImageGetTypeID() -> CFTypeID;
    pub fn CGImageCreate(
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bitsPerPixel: size_t,
        bytesPerRow: size_t,
        space: CGColorSpaceRef,
        bitmapInfo: u32,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageMaskCreate(
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bitsPerPixel: size_t,
        bytesPerRow: size_t,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
    ) -> CGImageRef;
    pub fn CGImageCreateCopy(image: CGImageRef) -> CGImageRef;
    pub fn CGImageCreateWithJPEGDataProvider(
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageCreateWithPNGDataProvider(
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageCreateWithImageInRect(image: CGImageRef, rect: CGRect) -> CGImageRef;
    pub fn CGImageCreateWithMask(image: CGImageRef, mask: CGImageRef) -> CGImageRef;
    pub fn CGImageCreateWithMaskingColors(image: CGImageRef, components: *const CGFloat) -> CGImageRef;
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef) -> CGImageRef;
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);
    pub fn CGImageIsMask(image: CGImageRef) -> bool;
    pub fn CGImageGetWidth(image: CGImageRef) -> size_t;
    pub fn CGImageGetHeight(image: CGImageRef) -> size_t;
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> size_t;
    pub fn CGImageGetBitsPerPixel(image: CGImageRef) -> size_t;
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> size_t;
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> CGImageAlphaInfo;
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
    pub fn CGImageGetDecode(image: CGImageRef) -> *const CGFloat;
    pub fn CGImageGetShouldInterpolate(image: CGImageRef) -> bool;
    pub fn CGImageGetRenderingIntent(image: CGImageRef) -> CGColorRenderingIntent;
    pub fn CGImageGetBitmapInfo(image: CGImageRef) -> CGBitmapInfo;
    pub fn CGImageGetByteOrderInfo(image: CGImageRef) -> CGImageByteOrderInfo;
    pub fn CGImageGetPixelFormatInfo(image: CGImageRef) -> CGImagePixelFormatInfo;
    pub fn CGImageGetUTType(image: CGImageRef) -> CFStringRef;
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImageAlphaInfo {
    #[doc(alias = "kCGImageAlphaNone")]
    AlphaNone               = kCGImageAlphaNone,
    #[doc(alias = "kCGImageAlphaPremultipliedLast")]
    AlphaPremultipliedLast  = kCGImageAlphaPremultipliedLast,
    #[doc(alias = "kCGImageAlphaPremultipliedFirst")]
    AlphaPremultipliedFirst = kCGImageAlphaPremultipliedFirst,
    #[doc(alias = "kCGImageAlphaLast")]
    AlphaLast               = kCGImageAlphaLast,
    #[doc(alias = "kCGImageAlphaFirst")]
    AlphaFirst              = kCGImageAlphaFirst,
    #[doc(alias = "kCGImageAlphaNoneSkipLast")]
    AlphaNoneSkipLast       = kCGImageAlphaNoneSkipLast,
    #[doc(alias = "kCGImageAlphaNoneSkipFirst")]
    AlphaNoneSkipFirst      = kCGImageAlphaNoneSkipFirst,
    #[doc(alias = "kCGImageAlphaOnly")]
    AlphaOnly               = kCGImageAlphaOnly,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImageByteOrderInfo {
    #[doc(alias = "kCGImageByteOrderMask")]
    ByteOrderMask     = kCGImageByteOrderMask,
    #[doc(alias = "kCGImageByteOrderDefault")]
    ByteOrderDefault  = kCGImageByteOrderDefault,
    #[doc(alias = "kCGImageByteOrder16Little")]
    ByteOrder16Little = kCGImageByteOrder16Little,
    #[doc(alias = "kCGImageByteOrder32Little")]
    ByteOrder32Little = kCGImageByteOrder32Little,
    #[doc(alias = "kCGImageByteOrder16Big")]
    ByteOrder16Big    = kCGImageByteOrder16Big,
    #[doc(alias = "kCGImageByteOrder32Big")]
    ByteOrder32Big    = kCGImageByteOrder32Big,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImagePixelFormatInfo {
    #[doc(alias = "kCGImagePixelFormatMask")]
    PixelFormatMask      = kCGImagePixelFormatMask,
    #[doc(alias = "kCGImagePixelFormatPacked")]
    PixelFormatPacked    = kCGImagePixelFormatPacked,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB555    = kCGImagePixelFormatRGB555,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB565    = kCGImagePixelFormatRGB565,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB101010 = kCGImagePixelFormatRGB101010,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGBCIF10  = kCGImagePixelFormatRGBCIF10,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGBitmapInfo: u32 {
        #[doc(alias = "kCGBitmapAlphaInfoMask")]
        const AlphaInfoMask = 0x1F;
        const AlphaNone = kCGImageAlphaNone;
        const AlphaPremultipliedLast = kCGImageAlphaPremultipliedLast;
        const AlphaPremultipliedFirst = kCGImageAlphaPremultipliedFirst;
        const AlphaLast = kCGImageAlphaLast;
        const AlphaFirst = kCGImageAlphaFirst;
        const AlphaNoneSkipLast = kCGImageAlphaNoneSkipLast;
        const AlphaNoneSkipFirst = kCGImageAlphaNoneSkipFirst;
        const AlphaOnly = kCGImageAlphaOnly;
        #[doc(alias = "kCGBitmapFloatInfoMask")]
        const FloatInfoMask = 0xF00;
        #[doc(alias = "kCGBitmapFloatComponents")]
        const FloatComponents = 1 << 8;
        #[doc(alias = "kCGBitmapByteOrderMask")]
        const ByteOrderMask = kCGImageByteOrderMask;
        #[doc(alias = "kCGBitmapByteOrderDefault")]
        const ByteOrderDefault = kCGImageByteOrderDefault;
        #[doc(alias = "kCGBitmapByteOrder16Little")]
        const ByteOrder16Little = kCGImageByteOrder16Little;
        #[doc(alias = "kCGBitmapByteOrder32Little")]
        const ByteOrder32Little = kCGImageByteOrder32Little;
        #[doc(alias = "kCGBitmapByteOrder16Big")]
        const ByteOrder16Big = kCGImageByteOrder16Big;
        #[doc(alias = "kCGBitmapByteOrder32Big")]
        const ByteOrder32Big = kCGImageByteOrder32Big;
        #[doc(alias = "kCGBitmapByteOrder16Host")]
        const ByteOrder16Host = kCGImageByteOrder16Host;
        #[doc(alias = "kCGBitmapByteOrder32Host")]
        const ByteOrder32Host = kCGImageByteOrder32Host;
        const PixelFormatMask = kCGImagePixelFormatMask;
        const PixelFormatPacked = kCGImagePixelFormatPacked;
        const PixelFormatRGB555 = kCGImagePixelFormatRGB555;
        const PixelFormatRGB565 = kCGImagePixelFormatRGB565;
        const PixelFormatRGB101010 = kCGImagePixelFormatRGB101010;
        const PixelFormatRGBCIF10 = kCGImagePixelFormatRGBCIF10;
    }
}

pub struct CGImage(CGImageRef);

impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe { CGImageRelease(self.0) }
    }
}

impl_TCFType!(CGImage, CGImageRef, CGImageGetTypeID);
impl_CFTypeDescription!(CGImage);

impl CGImage {
    pub fn new(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bits_per_pixel: size_t,
        bytes_per_row: size_t,
        space: Option<&CGColorSpace>,
        bitmap_info: u32,
        provider: Option<&CGDataProvider>,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image = CGImageCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                bitmap_info,
                provider.map_or(null(), |p| p.as_concrete_TypeRef()),
                decode.map_or(null(), |d| d.as_ptr()),
                should_interpolate,
                intent,
            );
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_mask(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bits_per_pixel: size_t,
        bytes_per_row: size_t,
        provider: Option<&CGDataProvider>,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
    ) -> Option<Self> {
        unsafe {
            let image = CGImageMaskCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                provider.map_or(null(), |p| p.as_concrete_TypeRef()),
                decode.map_or(null(), |d| d.as_ptr()),
                should_interpolate,
            );
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_jpeg_data_provider(
        provider: &CGDataProvider,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image =
                CGImageCreateWithJPEGDataProvider(provider.as_concrete_TypeRef(), decode.map_or(null(), |d| d.as_ptr()), should_interpolate, intent);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_png_data_provider(
        provider: &CGDataProvider,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image =
                CGImageCreateWithPNGDataProvider(provider.as_concrete_TypeRef(), decode.map_or(null(), |d| d.as_ptr()), should_interpolate, intent);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_copy(&self) -> Option<Self> {
        unsafe {
            let image = CGImageCreateCopy(self.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_copy_with_color_space(&self, space: &CGColorSpace) -> Option<Self> {
        unsafe {
            let image = CGImageCreateCopyWithColorSpace(self.as_concrete_TypeRef(), space.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_with_mask(&self, mask: &CGImage) -> Option<Self> {
        unsafe {
            let image = CGImageCreateWithMask(self.as_concrete_TypeRef(), mask.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_with_masking_colors(&self, components: &[CGFloat]) -> Option<Self> {
        unsafe {
            let color_space = self.color_space()?;
            let num_components = color_space.number_of_components();
            if components.len() != num_components * 2 {
                return None;
            }
            let image = CGImageCreateWithMaskingColors(self.as_concrete_TypeRef(), components.as_ptr());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn cropped(&self, rect: CGRect) -> Option<Self> {
        unsafe {
            let image = CGImageCreateWithImageInRect(self.as_concrete_TypeRef(), rect);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn is_mask(&self) -> bool {
        unsafe { CGImageIsMask(self.as_concrete_TypeRef()) }
    }

    pub fn width(&self) -> size_t {
        unsafe { CGImageGetWidth(self.as_concrete_TypeRef()) }
    }

    pub fn height(&self) -> size_t {
        unsafe { CGImageGetHeight(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_component(&self) -> size_t {
        unsafe { CGImageGetBitsPerComponent(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_pixel(&self) -> size_t {
        unsafe { CGImageGetBitsPerPixel(self.as_concrete_TypeRef()) }
    }

    pub fn bytes_per_row(&self) -> size_t {
        unsafe { CGImageGetBytesPerRow(self.as_concrete_TypeRef()) }
    }

    pub fn color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let space = CGImageGetColorSpace(self.as_concrete_TypeRef());
            if space.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(space))
            }
        }
    }

    pub fn alpha_info(&self) -> CGImageAlphaInfo {
        unsafe { CGImageGetAlphaInfo(self.as_concrete_TypeRef()) }
    }

    pub fn data_provider(&self) -> Option<CGDataProvider> {
        unsafe {
            let provider = CGImageGetDataProvider(self.as_concrete_TypeRef());
            if provider.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(provider))
            }
        }
    }

    pub fn should_interpolate(&self) -> bool {
        unsafe { CGImageGetShouldInterpolate(self.as_concrete_TypeRef()) }
    }

    pub fn rendering_intent(&self) -> CGColorRenderingIntent {
        unsafe { CGImageGetRenderingIntent(self.as_concrete_TypeRef()) }
    }

    pub fn bitmap_info(&self) -> CGBitmapInfo {
        unsafe { CGImageGetBitmapInfo(self.as_concrete_TypeRef()) }
    }

    pub fn byte_order_info(&self) -> CGImageByteOrderInfo {
        unsafe { CGImageGetByteOrderInfo(self.as_concrete_TypeRef()) }
    }

    pub fn pixel_format_info(&self) -> CGImagePixelFormatInfo {
        unsafe { CGImageGetPixelFormatInfo(self.as_concrete_TypeRef()) }
    }

    pub fn ut_type(&self) -> Option<CFString> {
        unsafe {
            let ut_type = CGImageGetUTType(self.as_concrete_TypeRef());
            if ut_type.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(ut_type))
            }
        }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGImage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGImage", &[]));
}
