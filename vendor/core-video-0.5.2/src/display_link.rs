use std::ptr::{null, null_mut};

use block::{Block, ConcreteBlock};
use core_foundation::{
    base::{Boolean, CFIndex, CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use core_graphics::display::CGDirectDisplayID;
use libc::c_void;

#[repr(C)]
pub struct __CVDisplayLink(c_void);

pub type CVDisplayLinkRef = *mut __CVDisplayLink;

use crate::{
    base::{CVOptionFlags, CVTime, CVTimeStamp},
    r#return::{kCVReturnSuccess, CVReturn},
    CGLContextObj, CGLPixelFormatObj,
};

pub type CVDisplayLinkOutputCallback = extern "C" fn(
    displayLink: CVDisplayLinkRef,
    inNow: *const CVTimeStamp,
    inOutputTime: *const CVTimeStamp,
    flagsIn: CVOptionFlags,
    flagsOut: *mut CVOptionFlags,
    displayLinkContext: *mut c_void,
) -> CVReturn;

pub type CVDisplayLinkOutputHandler =
    *const Block<(CVDisplayLinkRef, *const CVTimeStamp, *const CVTimeStamp, CVOptionFlags, *mut CVOptionFlags), CVReturn>;

pub type CGOpenGLDisplayMask = u32;

extern "C" {
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
    pub fn CVDisplayLinkCreateWithCGDisplays(
        displayArray: *const CGDirectDisplayID,
        count: CFIndex,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
    pub fn CVDisplayLinkCreateWithOpenGLDisplayMask(mask: CGOpenGLDisplayMask, displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkCreateWithCGDisplay(displayID: CGDirectDisplayID, displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkCreateWithActiveCGDisplays(displayLinkOut: *mut CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkSetCurrentCGDisplay(displayLink: CVDisplayLinkRef, displayID: CGDirectDisplayID) -> CVReturn;
    pub fn CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(
        displayLink: CVDisplayLinkRef,
        cglContext: CGLContextObj,
        CGLPixelFormatObj: CGLPixelFormatObj,
    ) -> CVReturn;
    pub fn CVDisplayLinkGetCurrentCGDisplay(displayLink: CVDisplayLinkRef) -> CGDirectDisplayID;
    pub fn CVDisplayLinkSetOutputCallback(displayLink: CVDisplayLinkRef, callback: CVDisplayLinkOutputCallback, userInfo: *mut c_void) -> CVReturn;
    pub fn CVDisplayLinkSetOutputHandler(displayLink: CVDisplayLinkRef, handler: CVDisplayLinkOutputHandler) -> CVReturn;
    pub fn CVDisplayLinkStart(displayLink: CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkStop(displayLink: CVDisplayLinkRef) -> CVReturn;
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef) -> CVTime;
    pub fn CVDisplayLinkGetOutputVideoLatency(displayLink: CVDisplayLinkRef) -> CVTime;
    pub fn CVDisplayLinkGetActualOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef) -> CVTime;
    pub fn CVDisplayLinkIsRunning(displayLink: CVDisplayLinkRef) -> Boolean;
    pub fn CVDisplayLinkGetCurrentTime(displayLink: CVDisplayLinkRef, outTime: *mut CVTime) -> CVReturn;
    pub fn CVDisplayLinkTranslateTime(displayLink: CVDisplayLinkRef, inTime: *const CVTime, outTime: *mut CVTime) -> CVReturn;
    pub fn CVDisplayLinkRetain(displayLink: CVDisplayLinkRef) -> CVDisplayLinkRef;
    pub fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}

pub struct CVDisplayLink(CVDisplayLinkRef);

impl Drop for CVDisplayLink {
    fn drop(&mut self) {
        unsafe { CVDisplayLinkRelease(self.0) }
    }
}

impl_TCFType!(CVDisplayLink, CVDisplayLinkRef, CVDisplayLinkGetTypeID);
impl_CFTypeDescription!(CVDisplayLink);

impl CVDisplayLink {
    #[inline]
    pub fn from_cg_displays(display_array: &[CGDirectDisplayID]) -> Result<CVDisplayLink, CVReturn> {
        let mut display_link: CVDisplayLinkRef = null_mut();
        unsafe {
            let result = CVDisplayLinkCreateWithCGDisplays(display_array.as_ptr(), display_array.len() as CFIndex, &mut display_link);
            if result == kCVReturnSuccess {
                Ok(TCFType::wrap_under_create_rule(display_link))
            } else {
                Err(result)
            }
        }
    }

    #[inline]
    pub fn from_opengl_display_mask(mask: CGOpenGLDisplayMask) -> Result<CVDisplayLink, CVReturn> {
        let mut display_link: CVDisplayLinkRef = null_mut();
        unsafe {
            let result = CVDisplayLinkCreateWithOpenGLDisplayMask(mask, &mut display_link);
            if result == kCVReturnSuccess {
                Ok(TCFType::wrap_under_create_rule(display_link))
            } else {
                Err(result)
            }
        }
    }

    #[inline]
    pub fn from_cg_display(display_id: CGDirectDisplayID) -> Result<CVDisplayLink, CVReturn> {
        let mut display_link: CVDisplayLinkRef = null_mut();
        unsafe {
            let result = CVDisplayLinkCreateWithCGDisplay(display_id, &mut display_link);
            if result == kCVReturnSuccess {
                Ok(TCFType::wrap_under_create_rule(display_link))
            } else {
                Err(result)
            }
        }
    }

    #[inline]
    pub fn from_active_cg_displays() -> Result<CVDisplayLink, CVReturn> {
        let mut display_link: CVDisplayLinkRef = null_mut();
        unsafe {
            let result = CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
            if result == kCVReturnSuccess {
                Ok(TCFType::wrap_under_create_rule(display_link))
            } else {
                Err(result)
            }
        }
    }

    #[inline]
    pub fn set_current_cg_display(&self, display_id: CGDirectDisplayID) -> Result<(), CVReturn> {
        let result = unsafe { CVDisplayLinkSetCurrentCGDisplay(self.as_concrete_TypeRef(), display_id) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[inline]
    pub unsafe fn set_current_cg_display_from_opengl_context(
        &self,
        cgl_context: CGLContextObj,
        cgl_pixel_format: CGLPixelFormatObj,
    ) -> Result<(), CVReturn> {
        let result = unsafe { CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(self.as_concrete_TypeRef(), cgl_context, cgl_pixel_format) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn get_current_cg_display(&self) -> CGDirectDisplayID {
        unsafe { CVDisplayLinkGetCurrentCGDisplay(self.as_concrete_TypeRef()) }
    }

    pub unsafe fn set_output_callback(&self, callback: CVDisplayLinkOutputCallback, user_info: *mut c_void) -> Result<(), CVReturn> {
        let result = unsafe { CVDisplayLinkSetOutputCallback(self.as_concrete_TypeRef(), callback, user_info) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn set_output_closure<F>(&self, closure: Option<F>) -> Result<(), CVReturn>
    where
        F: Fn(&CVDisplayLink, &CVTimeStamp, &CVTimeStamp, CVOptionFlags, &mut CVOptionFlags) -> CVReturn + 'static,
    {
        let handler = closure.map(|closure| {
            ConcreteBlock::new(
                move |display_link: CVDisplayLinkRef,
                      in_now: *const CVTimeStamp,
                      in_output_time: *const CVTimeStamp,
                      flags_in: CVOptionFlags,
                      flags_out: *mut CVOptionFlags|
                      -> CVReturn {
                    let display_link = unsafe { CVDisplayLink::wrap_under_get_rule(display_link) };
                    let in_now = unsafe { &*in_now };
                    let in_output_time = unsafe { &*in_output_time };
                    let flags_out = unsafe { &mut *flags_out };
                    closure(&display_link, &in_now, &in_output_time, flags_in, flags_out)
                },
            )
            .copy()
        });
        let result = unsafe { CVDisplayLinkSetOutputHandler(self.as_concrete_TypeRef(), handler.as_ref().map_or(null(), |h| &**h)) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn start(&self) -> Result<(), CVReturn> {
        let result = unsafe { CVDisplayLinkStart(self.as_concrete_TypeRef()) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn stop(&self) -> Result<(), CVReturn> {
        let result = unsafe { CVDisplayLinkStop(self.as_concrete_TypeRef()) };
        if result == kCVReturnSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn get_nominal_output_video_refresh_period(&self) -> CVTime {
        unsafe { CVDisplayLinkGetNominalOutputVideoRefreshPeriod(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_output_video_latency(&self) -> CVTime {
        unsafe { CVDisplayLinkGetOutputVideoLatency(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_actual_output_video_refresh_period(&self) -> CVTime {
        unsafe { CVDisplayLinkGetActualOutputVideoRefreshPeriod(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        unsafe { CVDisplayLinkIsRunning(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_current_time(&self) -> Result<CVTime, CVReturn> {
        let mut outTime = CVTime::default();
        let result = unsafe { CVDisplayLinkGetCurrentTime(self.as_concrete_TypeRef(), &mut outTime) };
        if result == kCVReturnSuccess {
            Ok(outTime)
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn translate_time(&self, in_time: &CVTime) -> Result<CVTime, CVReturn> {
        let mut out_time = CVTime::default();
        let result = unsafe { CVDisplayLinkTranslateTime(self.as_concrete_TypeRef(), in_time, &mut out_time) };
        if result == kCVReturnSuccess {
            Ok(out_time)
        } else {
            Err(result)
        }
    }
}
