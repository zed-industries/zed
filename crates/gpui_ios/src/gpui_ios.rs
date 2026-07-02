#![cfg(target_os = "ios")]
//! UIKit-based iOS platform implementation for GPUI, built on the Metal
//! renderer, GCD dispatcher, and Core Text text system shared with macOS
//! through the `gpui_apple` crate.

mod display;
mod platform;
mod text_input;
mod window;

pub use platform::IosPlatform;

pub(crate) use display::IosDisplay;
pub(crate) use window::IosWindow;

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use std::ffi::c_void;

#[allow(non_camel_case_types)]
pub(crate) type id = *mut Object;

#[allow(non_upper_case_globals)]
pub(crate) const nil: id = std::ptr::null_mut();

pub(crate) type CGFloat = f64;

// The `cocoa` crate that normally provides Core Graphics geometry types with
// `objc::Encode` impls doesn't build on iOS, and the `core-graphics` crate's
// types don't implement `Encode`, so declare the few we need locally.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

unsafe impl objc::Encode for CGPoint {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGPoint=dd}") }
    }
}

unsafe impl objc::Encode for CGSize {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGSize=dd}") }
    }
}

unsafe impl objc::Encode for CGRect {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
    }
}

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: u64 = 4;

pub(crate) unsafe fn ns_string(string: &str) -> id {
    unsafe {
        let ns_string: id = msg_send![class!(NSString), alloc];
        let ns_string: id = msg_send![
            ns_string,
            initWithBytes: string.as_ptr() as *const c_void
            length: string.len() as u64
            encoding: NSUTF8StringEncoding
        ];
        msg_send![ns_string, autorelease]
    }
}
