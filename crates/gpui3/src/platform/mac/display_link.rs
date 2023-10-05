use crate::PlatformDisplayLink;
use std::ffi::c_void;

pub use sys::CVTimeStamp as VideoTimestamp;

pub struct MacDisplayLink {
    sys_link: sys::DisplayLink,
    output_callback: Option<Box<dyn FnMut(&VideoTimestamp, &VideoTimestamp)>>,
}

impl MacDisplayLink {
    pub unsafe fn new(display_id: u32) -> Self {
        Self {
            sys_link: sys::DisplayLink::on_display(display_id).unwrap(),
            output_callback: None,
        }
    }
}

impl PlatformDisplayLink for MacDisplayLink {
    fn set_output_callback(&mut self, callback: Box<dyn FnMut(&VideoTimestamp, &VideoTimestamp)>) {
        unsafe {
            self.sys_link.set_output_callback(
                trampoline,
                self.output_callback.as_mut().unwrap()
                    as *mut dyn FnMut(&VideoTimestamp, &VideoTimestamp)
                    as *mut c_void,
            );
        }
        self.output_callback = Some(callback);
    }

    fn start(&mut self) {
        unsafe { self.sys_link.start() }
    }

    fn stop(&mut self) {
        unsafe { self.sys_link.stop() }
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
    if let Some(callback) = output_callback {
        if let Some((current_time, output_time)) = current_time.as_ref().zip(output_time.as_ref()) {
            // convert sys::CVTimeStamp to VideoTimestamp
            callback(&current_time, &output_time);
        }
    }
    0
}

mod sys {
    //! Derived from display-link crate under the fololwing license:
    //! https://github.com/BrainiumLLC/display-link/blob/master/LICENSE-MIT
    //! Apple docs: [CVDisplayLink](https://developer.apple.com/documentation/corevideo/cvdisplaylinkoutputcallback?language=objc)
    #![allow(dead_code)]

    pub use cocoa::quartzcore::CVTimeStamp;
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
