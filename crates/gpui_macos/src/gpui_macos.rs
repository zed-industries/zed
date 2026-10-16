#![cfg(target_os = "macos")]
//! macOS platform implementation for GPUI.
//!
//! macOS screens have a y axis that goes up from the bottom of the screen and
//! an origin at the bottom left of the main display.

mod display;
mod display_link;
mod events;
mod keyboard;
mod pasteboard;
mod system_notifications;

#[cfg(feature = "screen-capture")]
mod screen_capture;

use gpui_apple::metal_renderer as renderer;

pub mod metal_renderer {
    pub use gpui_apple::metal_renderer::{PathRasterizationVertex, PathSprite, SurfaceBounds};

    #[cfg(any(test, feature = "bench-support", feature = "test-support"))]
    pub use gpui_apple::metal_renderer::MetalHeadlessRenderer;
}

mod platform;
mod window;
mod window_appearance;

use objc2::{
    Encode, Encoding, RefEncode,
    rc::Retained,
    runtime::{AnyObject, Bool},
};
use objc2_foundation::{NSNotFound, NSString, NSUInteger};
use std::{ffi::CStr, ops::Range};

pub(crate) use display::*;
pub(crate) use display_link::*;
pub(crate) use gpui_apple::{AppleActivity, AppleDispatcher};
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub(crate) use window::*;

pub use platform::MacPlatform;

trait BoolExt {
    fn to_objc(self) -> Bool;
}

impl BoolExt for bool {
    fn to_objc(self) -> Bool {
        Bool::new(self)
    }
}

trait NSStringExt {
    unsafe fn to_str(&self) -> &str;
}

impl NSStringExt for *mut AnyObject {
    unsafe fn to_str(&self) -> &str {
        unsafe {
            let Some(string) = self.cast::<NSString>().as_ref() else {
                return "";
            };
            let cstr = string.UTF8String();
            if cstr.is_null() {
                ""
            } else {
                CStr::from_ptr(cstr).to_str().unwrap()
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    fn invalid() -> Self {
        Self {
            location: NSNotFound as NSUInteger,
            length: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.location != NSNotFound as NSUInteger
    }

    fn to_range(self) -> Option<Range<usize>> {
        if self.is_valid() {
            let start = self.location;
            let end = start + self.length;
            Some(start..end)
        } else {
            None
        }
    }
}

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        NSRange {
            location: range.start as NSUInteger,
            length: range.len() as NSUInteger,
        }
    }
}

unsafe impl Encode for NSRange {
    const ENCODING: Encoding = objc2_foundation::NSRange::ENCODING;
}

unsafe impl RefEncode for NSRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe fn ns_string(string: &str) -> *mut AnyObject {
    Retained::autorelease_ptr(NSString::from_str(string)).cast()
}
