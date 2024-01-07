use crate::{
    px, AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, Bounds, InputEvent, KeyDownEvent,
    Keystroke, Pixels, PlatformAtlas, PlatformDisplay, PlatformInputHandler, PlatformWindow, Point,
    Size, TestPlatform, TileId, WindowAppearance, WindowBounds, WindowOptions,
};
use collections::HashMap;
use parking_lot::Mutex;
use std::{
    rc::{Rc, Weak},
    sync::{self, Arc},
};

pub struct TestWindowState {
    pub(crate) bounds: WindowBounds,
    pub(crate) handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    pub(crate) title: Option<String>,
    pub(crate) edited: bool,
    platform: Weak<TestPlatform>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    pub(crate) should_close_handler: Option<Box<dyn FnMut() -> bool>>,
    input_callback: Option<Box<dyn FnMut(InputEvent) -> bool>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    input_handler: Option<Box<dyn PlatformInputHandler>>,
}

#[derive(Clone)]
pub struct TestWindow(pub(crate) Arc<Mutex<TestWindowState>>);

impl TestWindow {
    pub fn new(
        options: WindowOptions,
        handle: AnyWindowHandle,
        platform: Weak<TestPlatform>,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        Self(Arc::new(Mutex::new(TestWindowState {
            bounds: options.bounds,
            display,
            platform,
            handle,
            sprite_atlas: Arc::new(TestAtlas::new()),
            title: Default::default(),
            edited: false,
            should_close_handler: None,
            input_callback: None,
            active_status_change_callback: None,
            resize_callback: None,
            moved_callback: None,
            input_handler: None,
        })))
    }

    pub fn simulate_resize(&mut self, size: Size<Pixels>) {
        let scale_factor = self.scale_factor();
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.resize_callback.take() else {
            return;
        };
        match &mut lock.bounds {
            WindowBounds::Fullscreen | WindowBounds::Maximized => {
                lock.bounds = WindowBounds::Fixed(Bounds {
                    origin: Point::default(),
                    size: size.map(|pixels| f64::from(pixels).into()),
                });
            }
            WindowBounds::Fixed(bounds) => {
                bounds.size = size.map(|pixels| f64::from(pixels).into());
            }
        }
        drop(lock);
        callback(size, scale_factor);
        self.0.lock().resize_callback = Some(callback);
    }

    pub(crate) fn simulate_active_status_change(&self, active: bool) {
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.active_status_change_callback.take() else {
            return;
        };
        drop(lock);
        callback(active);
        self.0.lock().active_status_change_callback = Some(callback);
    }

    pub fn simulate_input(&mut self, event: InputEvent) -> bool {
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.input_callback.take() else {
            return false;
        };
        drop(lock);
        let result = callback(event);
        self.0.lock().input_callback = Some(callback);
        result
    }

    pub fn simulate_keystroke(&mut self, keystroke: Keystroke, is_held: bool) {
        if self.simulate_input(InputEvent::KeyDown(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held,
        })) {
            return;
        }

        let mut lock = self.0.lock();
        let Some(mut input_handler) = lock.input_handler.take() else {
            panic!(
                "simulate_keystroke {:?} input event was not handled and there was no active input",
                &keystroke
            );
        };
        drop(lock);
        let text = keystroke.ime_key.unwrap_or(keystroke.key);
        input_handler.replace_text_in_range(None, &text);

        self.0.lock().input_handler = Some(input_handler);
    }
    pub fn edited(&self) -> bool {
        self.0.lock().edited
    }
}

impl PlatformWindow for TestWindow {
    fn bounds(&self) -> WindowBounds {
        self.0.lock().bounds
    }

    fn content_size(&self) -> Size<Pixels> {
        let bounds = match self.bounds() {
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
        self.0.lock().display.clone()
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
        self.0.lock().input_handler = Some(input_handler);
    }

    fn clear_input_handler(&mut self) {
        self.0.lock().input_handler = None;
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        self.0
            .lock()
            .platform
            .upgrade()
            .expect("platform dropped")
            .prompt()
    }

    fn activate(&self) {
        self.0
            .lock()
            .platform
            .upgrade()
            .unwrap()
            .set_active_window(Some(self.clone()))
    }

    fn set_title(&mut self, title: &str) {
        self.0.lock().title = Some(title.to_owned());
    }

    fn set_edited(&mut self, edited: bool) {
        self.0.lock().edited = edited;
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
        self.0.lock().input_callback = Some(callback)
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().active_status_change_callback = Some(callback)
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().resize_callback = Some(callback)
    }

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {
        unimplemented!()
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().moved_callback = Some(callback)
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.lock().should_close_handler = Some(callback);
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
        self.0.lock().sprite_atlas.clone()
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
