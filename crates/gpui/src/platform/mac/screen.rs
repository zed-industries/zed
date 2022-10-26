use std::any::Any;

use crate::{
    geometry::vector::{vec2f, Vector2F},
    platform,
};
use cocoa::{
    appkit::NSScreen,
    base::{id, nil},
    foundation::NSArray,
};

#[derive(Debug)]
pub struct Screen {
    pub(crate) native_screen: id,
}

impl Screen {
    pub fn all() -> Vec<Self> {
        let mut screens = Vec::new();
        unsafe {
            let native_screens = NSScreen::screens(nil);
            for ix in 0..native_screens.count() {
                screens.push(Screen {
                    native_screen: native_screens.objectAtIndex(ix),
                });
            }
        }
        screens
    }
}

impl platform::Screen for Screen {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn size(&self) -> Vector2F {
        unsafe {
            let frame = self.native_screen.frame();
            vec2f(frame.size.width as f32, frame.size.height as f32)
        }
    }
}
