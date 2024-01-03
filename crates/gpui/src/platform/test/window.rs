use crate::{
    px, AtlasKey, AtlasTextureId, AtlasTile, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformInputHandler, PlatformWindow, Point, Size, TestPlatform, TileId, WindowAppearance,
    WindowBounds, WindowOptions,
};
use collections::HashMap;
use parking_lot::Mutex;
use std::{
    rc::{Rc, Weak},
    sync::{self, Arc},
};

#[derive(Default)]
pub(crate) struct TestWindowHandlers {
    pub(crate) active_status_change: Vec<Box<dyn FnMut(bool)>>,
    pub(crate) input: Vec<Box<dyn FnMut(crate::InputEvent) -> bool>>,
    pub(crate) moved: Vec<Box<dyn FnMut()>>,
    pub(crate) resize: Vec<Box<dyn FnMut(Size<Pixels>, f32)>>,
}

pub struct TestWindow {
    pub(crate) bounds: WindowBounds,
    display: Rc<dyn PlatformDisplay>,
    pub(crate) title: Option<String>,
    pub(crate) edited: bool,
    pub(crate) input_handler: Option<Arc<Mutex<Box<dyn PlatformInputHandler>>>>,
    pub(crate) handlers: Arc<Mutex<TestWindowHandlers>>,
    platform: Weak<TestPlatform>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
}

impl TestWindow {
    pub fn new(
        options: WindowOptions,
        platform: Weak<TestPlatform>,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        Self {
            bounds: options.bounds,
            display,
            platform,
            input_handler: None,
            sprite_atlas: Arc::new(TestAtlas::new()),
            handlers: Default::default(),
            title: Default::default(),
            edited: false,
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
        unimplemented!()
    }

    fn appearance(&self) -> WindowAppearance {
        unimplemented!()
    }

    fn display(&self) -> std::rc::Rc<dyn crate::PlatformDisplay> {
        self.display.clone()
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

    fn set_input_handler(&mut self, input_handler: Box<dyn crate::PlatformInputHandler>) {
        self.input_handler = Some(Arc::new(Mutex::new(input_handler)));
    }

    fn clear_input_handler(&mut self) {
        self.input_handler = None;
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        self.platform.upgrade().expect("platform dropped").prompt()
    }

    fn activate(&self) {
        unimplemented!()
    }

    fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_owned());
    }

    fn set_edited(&mut self, edited: bool) {
        self.edited = edited;
    }

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
        unimplemented!()
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.handlers.lock().moved.push(callback)
    }

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {
        unimplemented!()
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn invalidate(&self) {
        // (self.draw.lock())().unwrap();
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.sprite_atlas.clone()
    }

    fn as_test(&mut self) -> Option<&mut TestWindow> {
        Some(self)
    }
}

pub struct TestAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
}

pub struct TestAtlas(Mutex<TestAtlasState>);

impl TestAtlas {
    pub fn new() -> Self {
        TestAtlas(Mutex::new(TestAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
        }))
    }
}

impl PlatformAtlas for TestAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &crate::AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<(
            Size<crate::DevicePixels>,
            std::borrow::Cow<'a, [u8]>,
        )>,
    ) -> anyhow::Result<crate::AtlasTile> {
        let mut state = self.0.lock();
        if let Some(tile) = state.tiles.get(key) {
            return Ok(tile.clone());
        }

        state.next_id += 1;
        let texture_id = state.next_id;
        state.next_id += 1;
        let tile_id = state.next_id;

        drop(state);
        let (size, _) = build()?;
        let mut state = self.0.lock();

        state.tiles.insert(
            key.clone(),
            crate::AtlasTile {
                texture_id: AtlasTextureId {
                    index: texture_id,
                    kind: crate::AtlasTextureKind::Path,
                },
                tile_id: TileId(tile_id),
                bounds: crate::Bounds {
                    origin: Point::default(),
                    size,
                },
            },
        );

        Ok(state.tiles[key].clone())
    }

    fn clear(&self) {
        let mut state = self.0.lock();
        state.tiles = HashMap::default();
        state.next_id = 0;
    }
}
