use crate::{
    px, AtlasKey, AtlasTextureId, AtlasTile, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformInputHandler, PlatformWindow, Point, Scene, Size, TestPlatform, TileId,
    WindowAppearance, WindowBounds, WindowOptions,
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
    bounds: WindowBounds,
    current_scene: Mutex<Option<Scene>>,
    display: Rc<dyn PlatformDisplay>,
    pub(crate) window_title: Option<String>,
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
            current_scene: Default::default(),
            display,
            platform,
            input_handler: None,
            sprite_atlas: Arc::new(TestAtlas::new()),
            handlers: Default::default(),
            window_title: Default::default(),
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
        self
    }

    fn set_input_handler(&mut self, input_handler: Box<dyn crate::PlatformInputHandler>) {
        self.input_handler = Some(Arc::new(Mutex::new(input_handler)));
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
        todo!()
    }

    fn set_title(&mut self, title: &str) {
        self.window_title = Some(title.to_owned());
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

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.sprite_atlas.clone()
    }

    fn as_test(&self) -> Option<&TestWindow> {
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
                    origin: Point::zero(),
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
