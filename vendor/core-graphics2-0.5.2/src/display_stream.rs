use std::slice::from_raw_parts;

use block::{Block, ConcreteBlock, RcBlock};
use core_foundation::{
    base::{CFType, CFTypeID, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    runloop::{CFRunLoopSource, CFRunLoopSourceRef},
    string::{CFString, CFStringRef},
};
use dispatch2::{ffi::dispatch_queue_t, Queue};
use io_surface::{IOSurface, IOSurfaceRef};
use libc::{c_void, size_t};

use crate::{base::CGFloat, display::CGDirectDisplayID, error::CGError, geometry::CGRect};

#[repr(C)]
pub struct __CGDisplayStream(c_void);

pub type CGDisplayStreamRef = *mut __CGDisplayStream;

#[repr(C)]
pub struct __CGDisplayStreamUpdate(c_void);

pub type CGDisplayStreamUpdateRef = *const __CGDisplayStreamUpdate;

#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CGDisplayStreamUpdateRectType {
    #[doc(alias = "kCGDisplayStreamUpdateRefreshedRects")]
    RefreshedRects    = 0,
    #[doc(alias = "kCGDisplayStreamUpdateMovedRects")]
    MovedRects        = 1,
    #[doc(alias = "kCGDisplayStreamUpdateDirtyRects")]
    DirtyRects        = 2,
    #[doc(alias = "kCGDisplayStreamUpdateReducedDirtyRects")]
    ReducedDirtyRects = 3,
}

#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CGDisplayStreamFrameStatus {
    #[doc(alias = "kCGDisplayStreamFrameStatusFrameComplete")]
    FrameComplete = 0,
    #[doc(alias = "kCGDisplayStreamFrameStatusFrameIdle")]
    FrameIdle     = 1,
    #[doc(alias = "kCGDisplayStreamFrameStatusFrameBlank")]
    FrameBlank    = 2,
    #[doc(alias = "kCGDisplayStreamFrameStatusStopped")]
    Stopped       = 3,
}

pub type CGDisplayStreamFrameAvailableHandler = *const Block<(CGDisplayStreamFrameStatus, u64, IOSurfaceRef, CGDisplayStreamUpdateRef), ()>;

extern "C" {
    pub fn CGDisplayStreamUpdateGetTypeID() -> CFTypeID;
    pub fn CGDisplayStreamUpdateGetRects(
        update: CGDisplayStreamUpdateRef,
        rectType: CGDisplayStreamUpdateRectType,
        rectCount: *mut usize,
    ) -> *const CGRect;
    pub fn CGDisplayStreamUpdateCreateMergedUpdate(
        firstUpdate: CGDisplayStreamUpdateRef,
        secondUpdate: CGDisplayStreamUpdateRef,
    ) -> CGDisplayStreamUpdateRef;
    pub fn CGDisplayStreamUpdateGetMovedRectsDelta(updateRef: CGDisplayStreamUpdateRef, dx: *mut CGFloat, dy: *mut CGFloat);
    pub fn CGDisplayStreamUpdateGetDropCount(updateRef: CGDisplayStreamUpdateRef) -> size_t;

    pub static kCGDisplayStreamSourceRect: CFStringRef;
    pub static kCGDisplayStreamDestinationRect: CFStringRef;
    pub static kCGDisplayStreamPreserveAspectRatio: CFStringRef;
    pub static kCGDisplayStreamColorSpace: CFStringRef;
    pub static kCGDisplayStreamMinimumFrameTime: CFStringRef;
    pub static kCGDisplayStreamShowCursor: CFStringRef;
    pub static kCGDisplayStreamQueueDepth: CFStringRef;
    pub static kCGDisplayStreamYCbCrMatrix: CFStringRef;
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995: CFStringRef;

    pub fn CGDisplayStreamGetTypeID() -> CFTypeID;
    pub fn CGDisplayStreamCreate(
        display: CGDirectDisplayID,
        outputWidth: size_t,
        outputHeight: size_t,
        pixelFormat: i32,
        properties: CFDictionaryRef,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
    pub fn CGDisplayStreamCreateWithDispatchQueue(
        display: CGDirectDisplayID,
        outputWidth: size_t,
        outputHeight: size_t,
        pixelFormat: i32,
        properties: CFDictionaryRef,
        queue: dispatch_queue_t,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
    pub fn CGDisplayStreamStart(stream: CGDisplayStreamRef) -> CGError;
    pub fn CGDisplayStreamStop(stream: CGDisplayStreamRef) -> CGError;
    pub fn CGDisplayStreamGetRunLoopSource(stream: CGDisplayStreamRef) -> CFRunLoopSourceRef;
}

declare_TCFType!(CGDisplayStreamUpdate, CGDisplayStreamUpdateRef);
impl_TCFType!(CGDisplayStreamUpdate, CGDisplayStreamUpdateRef, CGDisplayStreamUpdateGetTypeID);
impl_CFTypeDescription!(CGDisplayStreamUpdate);

impl CGDisplayStreamUpdate {
    pub fn new_merged_update(&self, other: &CGDisplayStreamUpdate) -> Result<CGDisplayStreamUpdate, ()> {
        unsafe {
            let update = CGDisplayStreamUpdateCreateMergedUpdate(self.as_concrete_TypeRef(), other.as_concrete_TypeRef());
            if update.is_null() {
                Err(())
            } else {
                Ok(TCFType::wrap_under_create_rule(update))
            }
        }
    }

    pub fn rects(&self, rect_type: CGDisplayStreamUpdateRectType) -> &[CGRect] {
        unsafe {
            let mut rect_count = 0;
            let rects = CGDisplayStreamUpdateGetRects(self.as_concrete_TypeRef(), rect_type, &mut rect_count);
            if rects.is_null() || rect_count == 0 {
                &[]
            } else {
                from_raw_parts(rects, rect_count)
            }
        }
    }

    pub fn moved_rects_delta(&self) -> (CGFloat, CGFloat) {
        unsafe {
            let mut dx = 0.0;
            let mut dy = 0.0;
            CGDisplayStreamUpdateGetMovedRectsDelta(self.as_concrete_TypeRef(), &mut dx, &mut dy);
            (dx, dy)
        }
    }

    pub fn drop_count(&self) -> usize {
        unsafe { CGDisplayStreamUpdateGetDropCount(self.as_concrete_TypeRef()) }
    }
}

declare_TCFType!(CGDisplayStream, CGDisplayStreamRef);
impl_TCFType!(CGDisplayStream, CGDisplayStreamRef, CGDisplayStreamGetTypeID);
impl_CFTypeDescription!(CGDisplayStream);

impl CGDisplayStream {
    fn new_frame_available_handler<F>(closure: F) -> RcBlock<(CGDisplayStreamFrameStatus, u64, IOSurfaceRef, CGDisplayStreamUpdateRef), ()>
    where
        F: Fn(CGDisplayStreamFrameStatus, u64, Option<IOSurface>, Option<CGDisplayStreamUpdate>) + 'static,
    {
        ConcreteBlock::new(move |status: CGDisplayStreamFrameStatus, timestamp: u64, surface: IOSurfaceRef, update: CGDisplayStreamUpdateRef| {
            let surface = if surface.is_null() {
                None
            } else {
                Some(unsafe { IOSurface::wrap_under_get_rule(surface as IOSurfaceRef) })
            };
            let update = if update.is_null() {
                None
            } else {
                Some(unsafe { CGDisplayStreamUpdate::wrap_under_get_rule(update as CGDisplayStreamUpdateRef) })
            };
            closure(status, timestamp, surface, update);
        })
        .copy()
    }

    pub fn new<F>(
        display: CGDirectDisplayID,
        output_width: size_t,
        output_height: size_t,
        pixel_format: i32,
        properties: &CFDictionary<CFString, CFType>,
        closure: F,
    ) -> Result<CGDisplayStream, ()>
    where
        F: Fn(CGDisplayStreamFrameStatus, u64, Option<IOSurface>, Option<CGDisplayStreamUpdate>) + 'static,
    {
        let stream = unsafe {
            CGDisplayStreamCreate(
                display,
                output_width,
                output_height,
                pixel_format,
                properties.as_concrete_TypeRef(),
                &*Self::new_frame_available_handler(closure),
            )
        };
        if stream.is_null() {
            Err(())
        } else {
            Ok(unsafe { TCFType::wrap_under_create_rule(stream) })
        }
    }

    pub fn new_with_dispatch_queue<F>(
        display: CGDirectDisplayID,
        output_width: size_t,
        output_height: size_t,
        pixel_format: i32,
        properties: &CFDictionary<CFString, CFType>,
        queue: &Queue,
        closure: F,
    ) -> Result<CGDisplayStream, ()>
    where
        F: Fn(CGDisplayStreamFrameStatus, u64, Option<IOSurface>, Option<CGDisplayStreamUpdate>) + 'static,
    {
        let stream = unsafe {
            CGDisplayStreamCreateWithDispatchQueue(
                display,
                output_width,
                output_height,
                pixel_format,
                properties.as_concrete_TypeRef(),
                queue.as_raw(),
                &*Self::new_frame_available_handler(closure),
            )
        };
        if stream.is_null() {
            Err(())
        } else {
            Ok(unsafe { TCFType::wrap_under_create_rule(stream) })
        }
    }

    pub fn start(&self) -> CGError {
        unsafe { CGDisplayStreamStart(self.as_concrete_TypeRef()) }
    }

    pub fn stop(&self) -> CGError {
        unsafe { CGDisplayStreamStop(self.as_concrete_TypeRef()) }
    }

    pub fn run_loop_source(&self) -> Option<CFRunLoopSource> {
        unsafe {
            let source = CGDisplayStreamGetRunLoopSource(self.as_concrete_TypeRef());
            if source.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(source))
            }
        }
    }
}

pub enum CGDisplayStreamProperties {
    SourceRect,
    DestinationRect,
    PreserveAspectRatio,
    ColorSpace,
    MinimumFrameTime,
    ShowCursor,
    QueueDepth,
    YCbCrMatrix,
}

impl From<CGDisplayStreamProperties> for CFStringRef {
    fn from(key: CGDisplayStreamProperties) -> Self {
        unsafe {
            match key {
                CGDisplayStreamProperties::SourceRect => kCGDisplayStreamSourceRect,
                CGDisplayStreamProperties::DestinationRect => kCGDisplayStreamDestinationRect,
                CGDisplayStreamProperties::PreserveAspectRatio => kCGDisplayStreamPreserveAspectRatio,
                CGDisplayStreamProperties::ColorSpace => kCGDisplayStreamColorSpace,
                CGDisplayStreamProperties::MinimumFrameTime => kCGDisplayStreamMinimumFrameTime,
                CGDisplayStreamProperties::ShowCursor => kCGDisplayStreamShowCursor,
                CGDisplayStreamProperties::QueueDepth => kCGDisplayStreamQueueDepth,
                CGDisplayStreamProperties::YCbCrMatrix => kCGDisplayStreamYCbCrMatrix,
            }
        }
    }
}

impl From<CGDisplayStreamProperties> for CFString {
    fn from(key: CGDisplayStreamProperties) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub enum CGDisplayStreamYCbCrMatrices {
    ITU_R_709_2,
    ITU_R_601_4,
    SMPTE_240M_1995,
}

impl From<CGDisplayStreamYCbCrMatrices> for CFStringRef {
    fn from(matrix: CGDisplayStreamYCbCrMatrices) -> Self {
        unsafe {
            match matrix {
                CGDisplayStreamYCbCrMatrices::ITU_R_709_2 => kCGDisplayStreamYCbCrMatrix_ITU_R_709_2,
                CGDisplayStreamYCbCrMatrices::ITU_R_601_4 => kCGDisplayStreamYCbCrMatrix_ITU_R_601_4,
                CGDisplayStreamYCbCrMatrices::SMPTE_240M_1995 => kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995,
            }
        }
    }
}

impl From<CGDisplayStreamYCbCrMatrices> for CFString {
    fn from(matrix: CGDisplayStreamYCbCrMatrices) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(matrix)) }
    }
}
