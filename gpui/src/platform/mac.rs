mod atlas;
mod dispatcher;
mod event;
mod fonts;
mod geometry;
mod platform;
mod renderer;
mod sprite_cache;
mod window;

use cocoa::base::{BOOL, NO, YES};
pub use dispatcher::Dispatcher;
pub use fonts::FontSystem;
use platform::{MacLifecycle, MacPlatform};
use std::rc::Rc;
use window::Window;

pub(crate) fn lifecycle() -> Rc<dyn super::Lifecycle> {
    Rc::new(MacLifecycle::default())
}

pub(crate) fn platform() -> Rc<dyn super::Platform> {
    Rc::new(MacPlatform::new())
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
