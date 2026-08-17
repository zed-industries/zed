// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate has been deprecated in favour of the `objc2-io-surface` crate.
#![deprecated = "use the objc2-io-surface crate instead"]
#![crate_name = "io_surface"]
#![crate_type = "rlib"]

// Rust bindings to the IOSurface framework on macOS.

use cgl::{kCGLNoError, CGLErrorString, CGLGetCurrentContext, CGLTexImageIOSurface2D, GLenum};
use core::ffi::{c_int, c_void};
use core_foundation::base::{CFRelease, CFRetain, CFType, CFTypeID, CFTypeRef, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation_sys::base::mach_port_t;
use leaky_cow::LeakyCow;
use std::ffi::CStr;
use std::slice;

const BGRA: GLenum = 0x80E1;
const RGBA: GLenum = 0x1908;
const RGB: GLenum = 0x1907;
const TEXTURE_RECTANGLE_ARB: GLenum = 0x84F5;
const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IOSurfaceLockOptions {
    pub const kIOSurfaceLockReadOnly: u32 = 0x00000001;
    pub const kIOSurfaceLockAvoidSync: u32 = 0x00000002;
}

type IOReturn = c_int;

#[repr(C)]
pub struct __IOSurface(c_void);

pub type IOSurfaceRef = *const __IOSurface;

pub struct IOSurface {
    pub obj: IOSurfaceRef,
}

impl Drop for IOSurface {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

pub type IOSurfaceID = u32;

impl Clone for IOSurface {
    #[inline]
    fn clone(&self) -> IOSurface {
        unsafe { TCFType::wrap_under_get_rule(self.obj) }
    }
}

impl TCFType for IOSurface {
    type Ref = IOSurfaceRef;

    #[inline]
    fn as_concrete_TypeRef(&self) -> IOSurfaceRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: IOSurfaceRef) -> IOSurface {
        assert!(!obj.is_null(), "Attempted to create a NULL object.");
        IOSurface { obj }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { IOSurfaceGetTypeID() }
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: IOSurfaceRef) -> IOSurface {
        assert!(!reference.is_null(), "Attempted to create a NULL object.");
        let reference = CFRetain(reference as *const c_void) as IOSurfaceRef;
        TCFType::wrap_under_create_rule(reference)
    }
}

pub fn new(properties: &CFDictionary<CFString, CFType>) -> IOSurface {
    unsafe { TCFType::wrap_under_create_rule(IOSurfaceCreate(properties.as_concrete_TypeRef())) }
}

/// Looks up an `IOSurface` by its global ID.
///
/// FIXME(pcwalton): This should return an `Option`.
pub fn lookup(csid: IOSurfaceID) -> IOSurface {
    unsafe { TCFType::wrap_under_create_rule(IOSurfaceLookup(csid)) }
}

impl IOSurface {
    pub fn get_id(&self) -> IOSurfaceID {
        unsafe { IOSurfaceGetID(self.as_concrete_TypeRef()) }
    }

    /// Binds to the current GL texture.
    pub fn bind_to_gl_texture(&self, width: i32, height: i32, has_alpha: bool) {
        unsafe {
            let context = CGLGetCurrentContext();
            let gl_error = CGLTexImageIOSurface2D(
                context,
                TEXTURE_RECTANGLE_ARB,
                if has_alpha {
                    RGBA as GLenum
                } else {
                    RGB as GLenum
                },
                width,
                height,
                BGRA as GLenum,
                UNSIGNED_INT_8_8_8_8_REV,
                self.as_concrete_TypeRef() as *mut c_void,
                0,
            );

            if gl_error != kCGLNoError {
                let error_msg = CStr::from_ptr(CGLErrorString(gl_error));
                let error_msg = error_msg.to_string_lossy();
                // This will only actually leak memory if error_msg is a `Cow::Owned`, which
                // will only happen if the platform gives us invalid unicode.
                panic!("{}", error_msg.leak());
            }
        }
    }

    pub fn upload(&self, data: &[u8]) {
        unsafe {
            let surface = self.as_concrete_TypeRef();
            let mut seed = 0;

            IOSurfaceLock(surface, 0, &mut seed);

            let height = IOSurfaceGetHeight(surface);
            let stride = IOSurfaceGetBytesPerRow(surface);
            let size = height * stride;
            let address = IOSurfaceGetBaseAddress(surface) as *mut u8;
            let dest: &mut [u8] = slice::from_raw_parts_mut(address, size);
            dest.clone_from_slice(data);

            // FIXME(pcwalton): RAII
            IOSurfaceUnlock(surface, 0, &mut seed);
        }
    }
}

#[cfg_attr(feature = "link", link(name = "IOSurface", kind = "framework"))]
extern "C" {
    pub static kIOSurfaceAllocSize: CFStringRef;
    pub static kIOSurfaceWidth: CFStringRef;
    pub static kIOSurfaceHeight: CFStringRef;
    pub static kIOSurfaceBytesPerRow: CFStringRef;
    pub static kIOSurfaceBytesPerElement: CFStringRef;
    pub static kIOSurfaceElementWidth: CFStringRef;
    pub static kIOSurfaceElementHeight: CFStringRef;
    pub static kIOSurfaceOffset: CFStringRef;

    pub static kIOSurfacePlaneInfo: CFStringRef;
    pub static kIOSurfacePlaneWidth: CFStringRef;
    pub static kIOSurfacePlaneHeight: CFStringRef;
    pub static kIOSurfacePlaneBytesPerRow: CFStringRef;
    pub static kIOSurfacePlaneOffset: CFStringRef;
    pub static kIOSurfacePlaneSize: CFStringRef;

    pub static kIOSurfacePlaneBase: CFStringRef;
    pub static kIOSurfacePlaneBytesPerElement: CFStringRef;
    pub static kIOSurfacePlaneElementWidth: CFStringRef;
    pub static kIOSurfacePlaneElementHeight: CFStringRef;

    pub static kIOSurfaceCacheMode: CFStringRef;
    pub static kIOSurfaceIsGlobal: CFStringRef;
    pub static kIOSurfacePixelFormat: CFStringRef;

    pub fn IOSurfaceCreate(properties: CFDictionaryRef) -> IOSurfaceRef;
    pub fn IOSurfaceLookup(csid: IOSurfaceID) -> IOSurfaceRef;
    pub fn IOSurfaceGetID(buffer: IOSurfaceRef) -> IOSurfaceID;

    pub fn IOSurfaceGetTypeID() -> CFTypeID;

    pub fn IOSurfaceLock(buffer: IOSurfaceRef, options: u32, seed: *mut u32) -> IOReturn;
    pub fn IOSurfaceUnlock(buffer: IOSurfaceRef, options: u32, seed: *mut u32) -> IOReturn;
    pub fn IOSurfaceGetSeed(buffer: IOSurfaceRef) -> u32;

    pub fn IOSurfaceGetHeight(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetWidth(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetBytesPerRow(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetBaseAddress(buffer: IOSurfaceRef) -> *mut c_void;
    pub fn IOSurfaceGetElementHeight(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetElementWidth(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetBytesPerElement(buffer: IOSurfaceRef) -> usize;
    pub fn IOSurfaceGetAllocSize(buffer: IOSurfaceRef) -> usize;

    pub fn IOSurfaceGetPixelFormat(buffer: IOSurfaceRef) -> i32;

    pub fn IOSurfaceGetUseCount(buffer: IOSurfaceRef) -> i32;
    pub fn IOSurfaceIncrementUseCount(buffer: IOSurfaceRef);
    pub fn IOSurfaceDecrementUseCount(buffer: IOSurfaceRef);
    pub fn IOSurfaceIsInUse(buffer: IOSurfaceRef) -> bool;

    pub fn IOSurfaceCreateMachPort(buffer: IOSurfaceRef) -> mach_port_t;
    pub fn IOSurfaceLookupFromMachPort(port: mach_port_t) -> IOSurfaceRef;

    pub fn IOSurfaceGetPropertyAlignment(property: CFStringRef) -> usize;
    pub fn IOSurfaceGetPropertyMaximum(property: CFStringRef) -> usize;

    pub fn IOSurfaceCopyValue(buffer: IOSurfaceRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOSurfaceRemoveValue(buffer: IOSurfaceRef, key: CFStringRef);
    pub fn IOSurfaceSetValue(buffer: IOSurfaceRef, key: CFStringRef, value: CFTypeRef);

    pub fn IOSurfaceGetBaseAddressOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> *mut c_void;
    pub fn IOSurfaceGetBytesPerElementOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
    pub fn IOSurfaceGetBytesPerRowOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
    pub fn IOSurfaceGetElementHeightOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
    pub fn IOSurfaceGetElementWidthOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
    pub fn IOSurfaceGetHeightOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
    pub fn IOSurfaceGetWidthOfPlane(buffer: IOSurfaceRef, plane_index: usize) -> usize;
}
