use std::ffi::c_void;

use crate::{DisplayId, PlatformDisplayLinker};
use collections::HashMap;
use parking_lot::Mutex;
pub use sys::CVTimeStamp as VideoTimestamp;

pub struct MacDisplayLinker {
    links: Mutex<HashMap<DisplayId, MacDisplayLink>>,
}

struct MacDisplayLink {
    output_callback: Box<dyn FnMut(&VideoTimestamp, &VideoTimestamp)>,
    system_link: sys::DisplayLink,
}

unsafe impl Send for MacDisplayLink {}

impl MacDisplayLinker {
    pub fn new() -> Self {
        MacDisplayLinker {
            links: Default::default(),
        }
    }
}

impl PlatformDisplayLinker for MacDisplayLinker {
    fn set_output_callback(
        &self,
        display_id: DisplayId,
        mut output_callback: Box<dyn FnMut(&VideoTimestamp, &VideoTimestamp)>,
    ) {
        if let Some(mut system_link) = unsafe { sys::DisplayLink::on_display(display_id.0) } {
            unsafe {
                system_link.set_output_callback(
                    trampoline,
                    output_callback.as_mut() as *mut dyn FnMut(_, _) as *mut c_void,
                )
            }

            let previous = self.links.lock().insert(
                display_id,
                MacDisplayLink {
                    output_callback,
                    system_link,
                },
            );
            assert!(
                previous.is_none(),
                "You can currently only set an output callback once per display."
            )
        } else {
            return log::warn!("DisplayLink could not be obtained for {:?}", display_id);
        }
    }

    fn start(&self, display_id: DisplayId) {
        if let Some(link) = self.links.lock().get_mut(&display_id) {
            unsafe {
                link.system_link.start();
            }
        } else {
            log::warn!("No DisplayLink callback registered for {:?}", display_id)
        }
    }

    fn stop(&self, display_id: DisplayId) {
        if let Some(link) = self.links.lock().get_mut(&display_id) {
            unsafe {
                link.system_link.stop();
            }
        } else {
            log::warn!("No DisplayLink callback registered for {:?}", display_id)
        }
    }
}

unsafe extern "C" fn trampoline(
    _display_link_out: *mut sys::CVDisplayLink,
    current_time: *const sys::CVTimeStamp,
    output_time: *const sys::CVTimeStamp,
    _flags_in: i64,
    _flags_out: *mut i64,
    context: *mut c_void,
) -> i32 {
    let output_callback = &mut (*(context as *mut MacDisplayLink)).output_callback;
    if let Some((current_time, output_time)) = current_time.as_ref().zip(output_time.as_ref()) {
        output_callback(&current_time, &output_time);
    }
    0
}

mod sys {
    //! Derived from display-link crate under the fololwing license:
    //! https://github.com/BrainiumLLC/display-link/blob/master/LICENSE-MIT
    //! Apple docs: [CVDisplayLink](https://developer.apple.com/documentation/corevideo/cvdisplaylinkoutputcallback?language=objc)
    #![allow(dead_code, non_upper_case_globals)]

    use foreign_types::{foreign_type, ForeignType};
    use std::{
        ffi::c_void,
        fmt::{Debug, Formatter, Result},
    };

    #[derive(Debug)]
    pub enum CVDisplayLink {}

    foreign_type! {
        type CType = CVDisplayLink;
        fn drop = CVDisplayLinkRelease;
        fn clone = CVDisplayLinkRetain;
        pub struct DisplayLink;
        pub struct DisplayLinkRef;
    }

    impl Debug for DisplayLink {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            formatter
                .debug_tuple("DisplayLink")
                .field(&self.as_ptr())
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CVTimeStamp {
        pub version: u32,
        pub video_time_scale: i32,
        pub video_time: i64,
        pub host_time: u64,
        pub rate_scalar: f64,
        pub video_refresh_period: i64,
        pub smpte_time: CVSMPTETime,
        pub flags: u64,
        pub reserved: u64,
    }

    pub type CVTimeStampFlags = u64;

    pub const kCVTimeStampVideoTimeValid: CVTimeStampFlags = 1 << 0;
    pub const kCVTimeStampHostTimeValid: CVTimeStampFlags = 1 << 1;
    pub const kCVTimeStampSMPTETimeValid: CVTimeStampFlags = 1 << 2;
    pub const kCVTimeStampVideoRefreshPeriodValid: CVTimeStampFlags = 1 << 3;
    pub const kCVTimeStampRateScalarValid: CVTimeStampFlags = 1 << 4;
    pub const kCVTimeStampTopField: CVTimeStampFlags = 1 << 16;
    pub const kCVTimeStampBottomField: CVTimeStampFlags = 1 << 17;
    pub const kCVTimeStampVideoHostTimeValid: CVTimeStampFlags =
        kCVTimeStampVideoTimeValid | kCVTimeStampHostTimeValid;
    pub const kCVTimeStampIsInterlaced: CVTimeStampFlags =
        kCVTimeStampTopField | kCVTimeStampBottomField;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CVSMPTETime {
        pub subframes: i16,
        pub subframe_divisor: i16,
        pub counter: u32,
        pub time_type: u32,
        pub flags: u32,
        pub hours: i16,
        pub minutes: i16,
        pub seconds: i16,
        pub frames: i16,
    }

    pub type CVSMPTETimeType = u32;

    pub const kCVSMPTETimeType24: CVSMPTETimeType = 0;
    pub const kCVSMPTETimeType25: CVSMPTETimeType = 1;
    pub const kCVSMPTETimeType30Drop: CVSMPTETimeType = 2;
    pub const kCVSMPTETimeType30: CVSMPTETimeType = 3;
    pub const kCVSMPTETimeType2997: CVSMPTETimeType = 4;
    pub const kCVSMPTETimeType2997Drop: CVSMPTETimeType = 5;
    pub const kCVSMPTETimeType60: CVSMPTETimeType = 6;
    pub const kCVSMPTETimeType5994: CVSMPTETimeType = 7;

    pub type CVSMPTETimeFlags = u32;

    pub const kCVSMPTETimeValid: CVSMPTETimeFlags = 1 << 0;
    pub const kCVSMPTETimeRunning: CVSMPTETimeFlags = 1 << 1;

    pub type CVDisplayLinkOutputCallback = unsafe extern "C" fn(
        display_link_out: *mut CVDisplayLink,
        // A pointer to the current timestamp. This represents the timestamp when the callback is called.
        current_time: *const CVTimeStamp,
        // A pointer to the output timestamp. This represents the timestamp for when the frame will be displayed.
        output_time: *const CVTimeStamp,
        // Unused
        flags_in: i64,
        // Unused
        flags_out: *mut i64,
        // A pointer to app-defined data.
        display_link_context: *mut c_void,
    ) -> i32;

    #[link(name = "CoreFoundation", kind = "framework")]
    #[link(name = "CoreVideo", kind = "framework")]
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn CVDisplayLinkCreateWithActiveCGDisplays(
            display_link_out: *mut *mut CVDisplayLink,
        ) -> i32;
        pub fn CVDisplayLinkCreateWithCGDisplay(
            display_id: u32,
            display_link_out: *mut *mut CVDisplayLink,
        ) -> i32;
        pub fn CVDisplayLinkSetOutputCallback(
            display_link: &mut DisplayLinkRef,
            callback: CVDisplayLinkOutputCallback,
            user_info: *mut c_void,
        ) -> i32;
        pub fn CVDisplayLinkSetCurrentCGDisplay(
            display_link: &mut DisplayLinkRef,
            display_id: u32,
        ) -> i32;
        pub fn CVDisplayLinkStart(display_link: &mut DisplayLinkRef) -> i32;
        pub fn CVDisplayLinkStop(display_link: &mut DisplayLinkRef) -> i32;
        pub fn CVDisplayLinkRelease(display_link: *mut CVDisplayLink);
        pub fn CVDisplayLinkRetain(display_link: *mut CVDisplayLink) -> *mut CVDisplayLink;
    }

    impl DisplayLink {
        /// Apple docs: [CVDisplayLinkCreateWithActiveCGDisplays](https://developer.apple.com/documentation/corevideo/1456863-cvdisplaylinkcreatewithactivecgd?language=objc)
        pub unsafe fn new() -> Option<Self> {
            let mut display_link: *mut CVDisplayLink = 0 as _;
            let code = CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
            if code == 0 {
                Some(DisplayLink::from_ptr(display_link))
            } else {
                None
            }
        }

        /// Apple docs: [CVDisplayLinkCreateWithCGDisplay](https://developer.apple.com/documentation/corevideo/1456981-cvdisplaylinkcreatewithcgdisplay?language=objc)
        pub unsafe fn on_display(display_id: u32) -> Option<Self> {
            let mut display_link: *mut CVDisplayLink = 0 as _;
            let code = CVDisplayLinkCreateWithCGDisplay(display_id, &mut display_link);
            if code == 0 {
                Some(DisplayLink::from_ptr(display_link))
            } else {
                None
            }
        }
    }

    impl DisplayLinkRef {
        /// Apple docs: [CVDisplayLinkSetOutputCallback](https://developer.apple.com/documentation/corevideo/1457096-cvdisplaylinksetoutputcallback?language=objc)
        pub unsafe fn set_output_callback(
            &mut self,
            callback: CVDisplayLinkOutputCallback,
            user_info: *mut c_void,
        ) {
            assert_eq!(CVDisplayLinkSetOutputCallback(self, callback, user_info), 0);
        }

        /// Apple docs: [CVDisplayLinkSetCurrentCGDisplay](https://developer.apple.com/documentation/corevideo/1456768-cvdisplaylinksetcurrentcgdisplay?language=objc)
        pub unsafe fn set_current_display(&mut self, display_id: u32) {
            assert_eq!(CVDisplayLinkSetCurrentCGDisplay(self, display_id), 0);
        }

        /// Apple docs: [CVDisplayLinkStart](https://developer.apple.com/documentation/corevideo/1457193-cvdisplaylinkstart?language=objc)
        pub unsafe fn start(&mut self) {
            assert_eq!(CVDisplayLinkStart(self), 0);
        }

        /// Apple docs: [CVDisplayLinkStop](https://developer.apple.com/documentation/corevideo/1457281-cvdisplaylinkstop?language=objc)
        pub unsafe fn stop(&mut self) {
            assert_eq!(CVDisplayLinkStop(self), 0);
        }
    }
}
