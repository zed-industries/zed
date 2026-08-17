use std::ptr;

use crate::base::CGFloat;
use crate::color_space::CGColorSpace;
use crate::data_provider::{CGDataProvider, CGDataProviderRef};
use crate::geometry::CGRect;
use core_foundation::base::{CFRetain, CFTypeID};
use core_foundation::data::CFData;
use foreign_types::{foreign_type, ForeignType, ForeignTypeRef};
use libc::size_t;

#[repr(C)]
pub enum CGImageAlphaInfo {
    CGImageAlphaNone,               /* For example, RGB. */
    CGImageAlphaPremultipliedLast,  /* For example, premultiplied RGBA */
    CGImageAlphaPremultipliedFirst, /* For example, premultiplied ARGB */
    CGImageAlphaLast,               /* For example, non-premultiplied RGBA */
    CGImageAlphaFirst,              /* For example, non-premultiplied ARGB */
    CGImageAlphaNoneSkipLast,       /* For example, RBGX. */
    CGImageAlphaNoneSkipFirst,      /* For example, XRBG. */
    CGImageAlphaOnly,               /* No color data, alpha data only */
}

#[repr(C)]
pub enum CGImageByteOrderInfo {
    CGImageByteOrderMask = 0x7000,
    CGImageByteOrder16Little = 1 << 12,
    CGImageByteOrder32Little = 2 << 12,
    CGImageByteOrder16Big = 3 << 12,
    CGImageByteOrder32Big = 4 << 12,
}

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGImage {
        type CType = crate::sys::CGImage;
        fn drop = CGImageRelease;
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGImage {
    pub fn new(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bits_per_pixel: size_t,
        bytes_per_row: size_t,
        colorspace: &CGColorSpace,
        bitmap_info: u32,
        provider: &CGDataProvider,
        should_interpolate: bool,
        rendering_intent: u32,
    ) -> Self {
        unsafe {
            let result = CGImageCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                colorspace.as_ptr(),
                bitmap_info,
                provider.as_ptr(),
                ptr::null_mut(),
                should_interpolate,
                rendering_intent,
            );
            assert!(!result.is_null());
            Self::from_ptr(result)
        }
    }

    pub fn type_id() -> CFTypeID {
        unsafe { CGImageGetTypeID() }
    }
}

impl CGImageRef {
    pub fn width(&self) -> size_t {
        unsafe { CGImageGetWidth(self.as_ptr()) }
    }

    pub fn height(&self) -> size_t {
        unsafe { CGImageGetHeight(self.as_ptr()) }
    }

    pub fn bits_per_component(&self) -> size_t {
        unsafe { CGImageGetBitsPerComponent(self.as_ptr()) }
    }

    pub fn bits_per_pixel(&self) -> size_t {
        unsafe { CGImageGetBitsPerPixel(self.as_ptr()) }
    }

    pub fn bytes_per_row(&self) -> size_t {
        unsafe { CGImageGetBytesPerRow(self.as_ptr()) }
    }

    pub fn color_space(&self) -> CGColorSpace {
        unsafe {
            let cs = CGImageGetColorSpace(self.as_ptr());
            CFRetain(cs as *mut _);
            CGColorSpace::from_ptr(cs)
        }
    }

    /// Returns the raw image bytes wrapped in `CFData`. Note, the returned `CFData` owns the
    /// underlying buffer.
    pub fn data(&self) -> CFData {
        let data_provider =
            unsafe { CGDataProviderRef::from_ptr(CGImageGetDataProvider(self.as_ptr())) };
        data_provider.copy_data()
    }

    /// Returns a cropped image. If the `rect` specifies a rectangle which lies outside of the
    /// image bounds, the `None` is returned.
    pub fn cropped(&self, rect: CGRect) -> Option<CGImage> {
        let image_ptr = unsafe { CGImageCreateWithImageInRect(self.as_ptr(), rect) };
        if !image_ptr.is_null() {
            Some(unsafe { CGImage::from_ptr(image_ptr) })
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    fn CGImageGetTypeID() -> CFTypeID;
    fn CGImageGetWidth(image: crate::sys::CGImageRef) -> size_t;
    fn CGImageGetHeight(image: crate::sys::CGImageRef) -> size_t;
    fn CGImageGetBitsPerComponent(image: crate::sys::CGImageRef) -> size_t;
    fn CGImageGetBitsPerPixel(image: crate::sys::CGImageRef) -> size_t;
    fn CGImageGetBytesPerRow(image: crate::sys::CGImageRef) -> size_t;
    fn CGImageGetColorSpace(image: crate::sys::CGImageRef) -> crate::sys::CGColorSpaceRef;
    fn CGImageGetDataProvider(image: crate::sys::CGImageRef) -> crate::sys::CGDataProviderRef;
    fn CGImageRelease(image: crate::sys::CGImageRef);
    fn CGImageCreate(
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bitsPerPixel: size_t,
        bytesPerRow: size_t,
        space: crate::sys::CGColorSpaceRef,
        bitmapInfo: u32,
        provider: crate::sys::CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: u32,
    ) -> crate::sys::CGImageRef;
    fn CGImageCreateWithImageInRect(
        image: crate::sys::CGImageRef,
        rect: CGRect,
    ) -> crate::sys::CGImageRef;

    //fn CGImageGetAlphaInfo(image: ::sys::CGImageRef) -> CGImageAlphaInfo;
    //fn CGImageCreateCopyWithColorSpace(image: ::sys::CGImageRef, space: ::sys::CGColorSpaceRef) -> ::sys::CGImageRef
}
