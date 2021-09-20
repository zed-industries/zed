mod atlas;
mod dispatcher;
mod event;
mod fonts;
mod geometry;
mod image_cache;
mod platform;
mod renderer;
mod sprite_cache;
mod window;

use cocoa::base::{BOOL, NO, YES};
pub use dispatcher::Dispatcher;
pub use fonts::FontSystem;
use platform::{MacForegroundPlatform, MacPlatform};
use std::{rc::Rc, sync::Arc};
use window::Window;

pub(crate) fn platform() -> Arc<dyn super::Platform> {
    Arc::new(MacPlatform::new())
}

pub(crate) fn foreground_platform() -> Rc<dyn super::ForegroundPlatform> {
    Rc::new(MacForegroundPlatform::default())
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
