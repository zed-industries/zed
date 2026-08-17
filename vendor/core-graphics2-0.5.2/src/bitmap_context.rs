use std::{mem, ptr::null_mut, slice};

use core_foundation::base::TCFType;
use libc::{c_void, size_t};

use crate::{
    color_space::{CGColorSpace, CGColorSpaceRef},
    context::{CGContext, CGContextRef},
    image::{CGBitmapInfo, CGImage, CGImageAlphaInfo, CGImageRef},
};

pub type CGBitmapContextReleaseDataCallback = extern "C" fn(*mut c_void, *mut c_void);

extern "C" {
    pub fn CGBitmapContextCreateWithData(
        data: *mut c_void,
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bytesPerRow: size_t,
        space: CGColorSpaceRef,
        bitmapInfo: u32,
        releaseCallback: CGBitmapContextReleaseDataCallback,
        releaseInfo: *mut c_void,
    ) -> CGContextRef;
    pub fn CGBitmapContextCreate(
        data: *mut c_void,
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bytesPerRow: size_t,
        space: CGColorSpaceRef,
        bitmapInfo: u32,
    ) -> CGContextRef;
    pub fn CGBitmapContextGetData(context: CGContextRef) -> *mut c_void;
    pub fn CGBitmapContextGetWidth(context: CGContextRef) -> size_t;
    pub fn CGBitmapContextGetHeight(context: CGContextRef) -> size_t;
    pub fn CGBitmapContextGetBitsPerComponent(context: CGContextRef) -> size_t;
    pub fn CGBitmapContextGetBitsPerPixel(context: CGContextRef) -> size_t;
    pub fn CGBitmapContextGetBytesPerRow(context: CGContextRef) -> size_t;
    pub fn CGBitmapContextGetColorSpace(context: CGContextRef) -> CGColorSpaceRef;
    pub fn CGBitmapContextGetAlphaInfo(context: CGContextRef) -> CGImageAlphaInfo;
    pub fn CGBitmapContextGetBitmapInfo(context: CGContextRef) -> CGBitmapInfo;
    pub fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;
}

pub trait BitmapData {
    unsafe fn ptr(&self) -> *const u8;
    unsafe fn mut_ptr(&self) -> *mut u8;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn bytes_per_row(&self) -> usize;
}

impl CGContext {
    pub fn new_bitmap_context(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bytes_per_row: size_t,
        space: Option<&CGColorSpace>,
        bitmap_info: u32,
    ) -> Option<Self> {
        unsafe {
            let context = CGBitmapContextCreate(
                null_mut(),
                width,
                height,
                bits_per_component,
                bytes_per_row,
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                bitmap_info,
            );
            if context.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(context))
            }
        }
    }

    pub unsafe fn new_bitmap_context_with_data(
        data: &mut [u8],
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bytes_per_row: size_t,
        space: Option<&CGColorSpace>,
        bitmap_info: u32,
    ) -> Option<Self> {
        if data.len() < height * bytes_per_row {
            return None;
        }
        unsafe {
            let context = CGBitmapContextCreate(
                data.as_mut_ptr() as *mut c_void,
                width,
                height,
                bits_per_component,
                bytes_per_row,
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                bitmap_info,
            );
            if context.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(context))
            }
        }
    }

    pub fn new_bitmap_context_with_bitmap_data(
        bitmap_data: Box<Box<dyn BitmapData>>,
        bits_per_component: size_t,
        space: Option<&CGColorSpace>,
        bitmap_info: u32,
    ) -> Option<Self> {
        let (width, height, bytes_per_row) = (bitmap_data.width() as size_t, bitmap_data.height() as size_t, bitmap_data.bytes_per_row() as size_t);
        unsafe {
            let ptr = bitmap_data.mut_ptr() as *mut c_void;
            let info = mem::transmute::<Box<Box<dyn BitmapData>>, &mut c_void>(bitmap_data);
            let context = CGBitmapContextCreateWithData(
                ptr,
                width,
                height,
                bits_per_component,
                bytes_per_row,
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                bitmap_info,
                release,
                info,
            );
            if context.is_null() {
                drop(mem::transmute::<*mut c_void, Box<Box<dyn BitmapData>>>(info));
                return None;
            } else {
                return Some(TCFType::wrap_under_create_rule(context));
            }
        }

        extern "C" fn release(release_info: *mut c_void, _: *mut c_void) {
            unsafe { drop(mem::transmute::<*mut c_void, Box<Box<dyn BitmapData>>>(release_info)) }
        }
    }

    pub fn new_image(&self) -> Option<CGImage> {
        unsafe {
            let image = CGBitmapContextCreateImage(self.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn data(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(CGBitmapContextGetData(self.as_concrete_TypeRef()) as *mut u8, self.height() * self.bytes_per_row()) }
    }

    pub fn width(&self) -> usize {
        unsafe { CGBitmapContextGetWidth(self.as_concrete_TypeRef()) }
    }

    pub fn height(&self) -> usize {
        unsafe { CGBitmapContextGetHeight(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_component(&self) -> usize {
        unsafe { CGBitmapContextGetBitsPerComponent(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_pixel(&self) -> usize {
        unsafe { CGBitmapContextGetBitsPerPixel(self.as_concrete_TypeRef()) }
    }

    pub fn bytes_per_row(&self) -> usize {
        unsafe { CGBitmapContextGetBytesPerRow(self.as_concrete_TypeRef()) }
    }

    pub fn color_space(&self) -> CGColorSpace {
        unsafe { TCFType::wrap_under_get_rule(CGBitmapContextGetColorSpace(self.as_concrete_TypeRef())) }
    }

    pub fn alpha_info(&self) -> CGImageAlphaInfo {
        unsafe { CGBitmapContextGetAlphaInfo(self.as_concrete_TypeRef()) }
    }

    pub fn bitmap_info(&self) -> CGBitmapInfo {
        unsafe { CGBitmapContextGetBitmapInfo(self.as_concrete_TypeRef()) }
    }
}
