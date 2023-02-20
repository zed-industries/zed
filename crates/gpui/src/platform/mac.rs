mod appearance;
mod atlas;
mod dispatcher;
mod event;
mod fonts;
mod geometry;
mod image_cache;
mod platform;
mod renderer;
mod screen;
mod sprite_cache;
mod status_item;
mod window;

use cocoa::{
    base::{id, nil, BOOL, NO, YES},
    foundation::{NSAutoreleasePool, NSNotFound, NSString, NSUInteger},
};
pub use dispatcher::Dispatcher;
pub use fonts::FontSystem;
use platform::{MacForegroundPlatform, MacPlatform};
pub use renderer::Surface;
use std::{ops::Range, rc::Rc, sync::Arc};
use window::Window;

use crate::executor;

pub(crate) fn platform() -> Arc<dyn super::Platform> {
    Arc::new(MacPlatform::new())
}

pub(crate) fn foreground_platform(
    foreground: Rc<executor::Foreground>,
) -> Rc<dyn super::ForegroundPlatform> {
    Rc::new(MacForegroundPlatform::new(foreground))
}

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
