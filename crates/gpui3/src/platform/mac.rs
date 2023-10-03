///! Macos screen have a y axis that goings up from the bottom of the screen and
///! an origin at the bottom left of the main display.
mod dispatcher;
mod events;
mod metal_atlas;
mod metal_renderer;
mod open_type;
mod platform;
mod screen;
mod text_system;
mod window;
mod window_appearence;

use crate::{px, size, Pixels, Size};
use anyhow::anyhow;
use cocoa::{
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSNotFound, NSRect, NSSize, NSString, NSUInteger, NSURL},
};
use metal_renderer::*;
use objc::{
    msg_send,
    runtime::{BOOL, NO, YES},
    sel, sel_impl,
};
use std::{
    ffi::{c_char, CStr, OsStr},
    ops::Range,
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

pub use dispatcher::*;
pub use metal_atlas::*;
pub use platform::*;
pub use screen::*;
pub use text_system::*;
pub use window::*;

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

pub trait NSRectExt {
    fn size(&self) -> Size<Pixels>;
    fn intersects(&self, other: Self) -> bool;
}

impl NSRectExt for NSRect {
    fn size(&self) -> Size<Pixels> {
        size(px(self.size.width as f32), px(self.size.height as f32))
    }

    fn intersects(&self, other: Self) -> bool {
        self.size.width > 0.
            && self.size.height > 0.
            && other.size.width > 0.
            && other.size.height > 0.
            && self.origin.x <= other.origin.x + other.size.width
            && self.origin.x + self.size.width >= other.origin.x
            && self.origin.y <= other.origin.y + other.size.height
            && self.origin.y + self.size.height >= other.origin.y
    }
}

// todo!
#[allow(unused)]
unsafe fn ns_url_to_path(url: id) -> crate::Result<PathBuf> {
    let path: *mut c_char = msg_send![url, fileSystemRepresentation];
    if path.is_null() {
        Err(anyhow!(
            "url is not a file path: {}",
            CStr::from_ptr(url.absoluteString().UTF8String()).to_string_lossy()
        ))
    } else {
        Ok(PathBuf::from(OsStr::from_bytes(
            CStr::from_ptr(path).to_bytes(),
        )))
    }
}
