//! Macos screen have a y axis that goings up from the bottom of the screen and
//! an origin at the bottom left of the main display.
mod dispatcher;
mod display;
mod display_link;
mod events;

#[cfg(not(feature = "macos-blade"))]
mod metal_atlas;
#[cfg(not(feature = "macos-blade"))]
pub mod metal_renderer;

#[cfg(not(feature = "macos-blade"))]
use metal_renderer as renderer;

#[cfg(feature = "macos-blade")]
use crate::platform::blade as renderer;

mod open_type;
mod platform;
mod text_system;
mod window;
mod window_appearance;

use crate::{px, size, GlobalPixels, Pixels, Size};
use cocoa::{
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSNotFound, NSRect, NSSize, NSString, NSUInteger},
};

use objc::runtime::{BOOL, NO, YES};
use std::ops::Range;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use display_link::*;
pub(crate) use platform::*;
pub(crate) use text_system::*;
pub(crate) use window::*;

trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self {
            YES
        } else {
            NO
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
            let start = self.location as usize;
            let end = start + self.length as usize;
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

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{NSRange={}{}}}",
            NSUInteger::encode().as_str(),
            NSUInteger::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}

impl From<NSSize> for Size<Pixels> {
    fn from(value: NSSize) -> Self {
        Size {
            width: px(value.width as f32),
            height: px(value.height as f32),
        }
    }
}

impl From<NSRect> for Size<Pixels> {
    fn from(rect: NSRect) -> Self {
        let NSSize { width, height } = rect.size;
        size(width.into(), height.into())
    }
}

impl From<NSRect> for Size<GlobalPixels> {
    fn from(rect: NSRect) -> Self {
        let NSSize { width, height } = rect.size;
        size(width.into(), height.into())
    }
}
