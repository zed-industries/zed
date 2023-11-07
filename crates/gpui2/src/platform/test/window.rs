use std::{rc::Rc, sync::Arc};

use parking_lot::Mutex;

use crate::{
    px, Pixels, PlatformAtlas, PlatformDisplay, PlatformWindow, Point, Scene, Size,
    WindowAppearance, WindowBounds, WindowOptions,
};

#[derive(Default)]
struct Handlers {
    active_status_change: Vec<Box<dyn FnMut(bool)>>,
    input: Vec<Box<dyn FnMut(crate::InputEvent) -> bool>>,
    moved: Vec<Box<dyn FnMut()>>,
    resize: Vec<Box<dyn FnMut(Size<Pixels>, f32)>>,
}

pub struct TestWindow {
    bounds: WindowBounds,
    current_scene: Mutex<Option<Scene>>,
    display: Rc<dyn PlatformDisplay>,

    handlers: Mutex<Handlers>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
}
impl TestWindow {
    pub fn new(options: WindowOptions, display: Rc<dyn PlatformDisplay>) -> Self {
        Self {
            bounds: options.bounds,
            current_scene: Default::default(),
            display,

            sprite_atlas: Arc::new(TestAtlas),
            handlers: Default::default(),
        }
    }
}

impl PlatformWindow for TestWindow {
    fn bounds(&self) -> WindowBounds {
        self.bounds
    }

    fn content_size(&self) -> Size<Pixels> {
        let bounds = match self.bounds {
            WindowBounds::Fixed(bounds) => bounds,
            WindowBounds::Maximized | WindowBounds::Fullscreen => self.display().bounds(),
        };
        bounds.size.map(|p| px(p.0))
    }

    fn scale_factor(&self) -> f32 {
        2.0
    }

    fn titlebar_height(&self) -> Pixels {
        todo!()
    }

    fn appearance(&self) -> WindowAppearance {
        todo!()
    }

    fn display(&self) -> std::rc::Rc<dyn crate::PlatformDisplay> {
        self.display.clone()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::zero()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn set_input_handler(&mut self, _input_handler: Box<dyn crate::PlatformInputHandler>) {
        todo!()
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        todo!()
    }

    fn activate(&self) {
        todo!()
    }

    fn set_title(&mut self, _title: &str) {
        todo!()
    }

    fn set_edited(&mut self, _edited: bool) {
        todo!()
    }

    fn show_character_palette(&self) {
        todo!()
    }

    fn minimize(&self) {
        todo!()
    }

    fn zoom(&self) {
        todo!()
    }

    fn toggle_full_screen(&self) {
        todo!()
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::InputEvent) -> bool>) {
        self.handlers.lock().input.push(callback)
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.handlers.lock().active_status_change.push(callback)
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.handlers.lock().resize.push(callback)
    }

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {
        todo!()
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.handlers.lock().moved.push(callback)
    }

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {
        todo!()
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        todo!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        todo!()
    }

    fn draw(&self, scene: crate::Scene) {
        self.current_scene.lock().replace(scene);
    }

    fn sprite_atlas(&self) -> std::sync::Arc<dyn crate::PlatformAtlas> {
        self.sprite_atlas.clone()
    }
}

pub struct TestAtlas;

impl PlatformAtlas for TestAtlas {
    fn get_or_insert_with<'a>(
        &self,
        _key: &crate::AtlasKey,
        _build: &mut dyn FnMut() -> anyhow::Result<(
            Size<crate::DevicePixels>,
            std::borrow::Cow<'a, [u8]>,
        )>,
    ) -> anyhow::Result<crate::AtlasTile> {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }
}
