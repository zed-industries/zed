#![allow(non_snake_case)]

use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use std::ffi::c_void;

pub mod io_surface {
    use super::*;

    #[repr(C)]
    pub struct __IOSurface(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type IOSurfaceRef = *const __IOSurface;

    declare_TCFType!(IOSurface, IOSurfaceRef);
    impl_TCFType!(IOSurface, IOSurfaceRef, IOSurfaceGetTypeID);
    impl_CFTypeDescription!(IOSurface);

    #[link(name = "IOSurface", kind = "framework")]
    extern "C" {
        fn IOSurfaceGetTypeID() -> CFTypeID;
    }
}

pub mod core_video {
    #![allow(non_snake_case)]

    use super::*;
    use io_surface::{IOSurface, IOSurfaceRef};

    #[repr(C)]
    pub struct __CVImageBuffer(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type CVImageBufferRef = *const __CVImageBuffer;

    declare_TCFType!(CVImageBuffer, CVImageBufferRef);
    impl_TCFType!(CVImageBuffer, CVImageBufferRef, CVImageBufferGetTypeID);
    impl_CFTypeDescription!(CVImageBuffer);

    impl CVImageBuffer {
        pub fn io_surface(&self) -> IOSurface {
            unsafe {
                IOSurface::wrap_under_get_rule(CVPixelBufferGetIOSurface(
                    self.as_concrete_TypeRef(),
                ))
            }
        }

        pub fn width(&self) -> usize {
            unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
        }

        pub fn height(&self) -> usize {
            unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
        }
    }

    extern "C" {
        fn CVImageBufferGetTypeID() -> CFTypeID;
        fn CVPixelBufferGetIOSurface(buffer: CVImageBufferRef) -> IOSurfaceRef;
        fn CVPixelBufferGetWidth(buffer: CVImageBufferRef) -> usize;
        fn CVPixelBufferGetHeight(buffer: CVImageBufferRef) -> usize;
    }
}
