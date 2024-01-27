use crate::{
    px, AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, BladeAtlas, Bounds, KeyDownEvent,
    Keystroke, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, Size, TileId, WindowAppearance, WindowBounds, WindowOptions,
};
use collections::HashMap;
use parking_lot::Mutex;
use std::{
    rc::{Rc, Weak},
    sync::{self, Arc},
};

pub(crate) struct LinuxWindowState {
    display: Rc<dyn crate::PlatformDisplay>,
    sprite_atlas: Arc<BladeAtlas>,
}

#[derive(Clone)]
pub(crate) struct LinuxWindow(pub(crate) Arc<Mutex<LinuxWindowState>>);

impl LinuxWindow {
    pub fn new(
        options: WindowOptions,
        handle: AnyWindowHandle,
        display: Rc<dyn PlatformDisplay>,
        gpu: &Arc<blade::Context>,
    ) -> Self {
        Self(Arc::new(Mutex::new(LinuxWindowState {
            display,
            sprite_atlas: Arc::new(BladeAtlas::new(gpu)),
        })))
    }
}

impl PlatformWindow for LinuxWindow {
    fn bounds(&self) -> WindowBounds {
        unimplemented!()
    }

    fn content_size(&self) -> Size<Pixels> {
        unimplemented!()
    }

    fn scale_factor(&self) -> f32 {
        1.0
    }

    fn titlebar_height(&self) -> Pixels {
        unimplemented!()
    }

    fn appearance(&self) -> WindowAppearance {
        unimplemented!()
    }

    fn display(&self) -> Rc<dyn crate::PlatformDisplay> {
        Rc::clone(&self.0.lock().display)
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {}

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        None
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {}

    fn set_title(&mut self, title: &str) {}

    fn set_edited(&mut self, edited: bool) {}

    fn show_character_palette(&self) {
        unimplemented!()
    }

    fn minimize(&self) {
        unimplemented!()
    }

    fn zoom(&self) {
        unimplemented!()
    }

    fn toggle_full_screen(&self) {
        unimplemented!()
    }

    fn on_request_frame(&self, _callback: Box<dyn FnMut()>) {}

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {}

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {}

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_moved(&self, callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {}

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn invalidate(&self) {}

    fn draw(&self, _scene: &crate::Scene) {}

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }
}
