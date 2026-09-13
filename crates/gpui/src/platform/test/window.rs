use crate::{
    AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, Bounds, DevicePixels,
    DispatchEventResult, GpuSpecs, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformHeadlessRenderer, PlatformInput, PlatformInputHandler, PlatformWindow, Point,
    PromptButton, RequestFrameOptions, Scene, Size, TestPlatform, TextInputConfiguration,
    TextInputStateChange, TileId, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    WindowControlArea, WindowInsets, WindowParams, WindowVisibility,
};
use collections::HashMap;
use gpui_util::ResultExt as _;
#[cfg(any(test, feature = "test-support"))]
use image::RgbaImage;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    cell::Cell,
    path::PathBuf,
    rc::{Rc, Weak},
    sync::{self, Arc},
};

pub(crate) struct TestWindowState {
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    pub(crate) title: Option<String>,
    pub(crate) edited: bool,
    pub(crate) document_path: Option<std::path::PathBuf>,
    platform: Weak<TestPlatform>,
    // TODO: Replace with `Rc`
    sprite_atlas: Arc<dyn PlatformAtlas>,
    renderer: Option<Box<dyn PlatformHeadlessRenderer>>,
    pub(crate) should_close_handler: Option<Box<dyn FnMut() -> bool>>,
    hit_test_window_control_callback: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    visibility: WindowVisibility,
    visibility_callback: Option<Box<dyn FnMut(WindowVisibility)>>,
    hover_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    visual_viewport: Option<Bounds<Pixels>>,
    visual_viewport_callback: Option<Box<dyn FnMut()>>,
    insets: WindowInsets,
    insets_callback: Option<Box<dyn FnMut(WindowInsets)>>,
    virtual_keyboard_requests: usize,
    virtual_keyboard_dismissals: usize,
    moved_callback: Option<Box<dyn FnMut()>>,
    appearance_change_callback: Option<Box<dyn FnMut()>>,
    request_frame_callback: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    frame_wake_count: Rc<Cell<usize>>,
    frame_scheduled: bool,
    frame_callback_pending: bool,
    input_handler: Option<PlatformInputHandler>,
    text_input_configurations: Vec<TextInputConfiguration>,
    text_input_state_changes: Vec<TextInputStateChange>,
    is_fullscreen: bool,
    scale_factor: f32,
    appearance: WindowAppearance,
    external_drag_files: Vec<(PathBuf, bool)>,
    start_external_drag_result: bool,
}

#[derive(Clone)]
pub struct TestWindow(pub(crate) Rc<Mutex<TestWindowState>>);

// Test windows are not backed by a real platform window, so there is no raw
// handle to report; `NotSupported` is `raw_window_handle`'s variant for exactly this.
impl HasWindowHandle for TestWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl HasDisplayHandle for TestWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl TestWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        platform: Weak<TestPlatform>,
        display: Rc<dyn PlatformDisplay>,
        renderer: Option<Box<dyn PlatformHeadlessRenderer>>,
    ) -> Self {
        let sprite_atlas: Arc<dyn PlatformAtlas> = match &renderer {
            Some(r) => r.sprite_atlas(),
            None => Arc::new(TestAtlas::new()),
        };
        Self(Rc::new(Mutex::new(TestWindowState {
            bounds: params.bounds,
            display,
            platform,
            handle,
            sprite_atlas,
            renderer,
            title: Default::default(),
            edited: false,
            document_path: None,
            should_close_handler: None,
            hit_test_window_control_callback: None,
            input_callback: None,
            active_status_change_callback: None,
            visibility: WindowVisibility::Visible,
            visibility_callback: None,
            hover_status_change_callback: None,
            resize_callback: None,
            visual_viewport: None,
            visual_viewport_callback: None,
            insets: WindowInsets::default(),
            insets_callback: None,
            virtual_keyboard_requests: 0,
            virtual_keyboard_dismissals: 0,
            moved_callback: None,
            appearance_change_callback: None,
            request_frame_callback: None,
            frame_wake_count: Rc::new(Cell::new(0)),
            frame_scheduled: false,
            frame_callback_pending: false,
            input_handler: None,
            text_input_configurations: Vec::new(),
            text_input_state_changes: Vec::new(),
            is_fullscreen: false,
            // Preserve the test platform's historical 2x default.
            scale_factor: 2.0,
            appearance: WindowAppearance::Light,
            external_drag_files: Vec::new(),
            start_external_drag_result: false,
        })))
    }
    pub fn simulate_scheduled_frame(&self) -> bool {
        let callback = {
            let mut state = self.0.lock();
            if !std::mem::take(&mut state.frame_scheduled) {
                return false;
            }
            state.frame_callback_pending = false;
            state.request_frame_callback.take()
        };
        let Some(mut callback) = callback else {
            self.0.lock().frame_scheduled = true;
            return false;
        };

        callback(RequestFrameOptions::default());
        self.0.lock().request_frame_callback = Some(callback);
        true
    }

    pub fn frame_scheduled(&self) -> bool {
        self.0.lock().frame_scheduled
    }

    pub fn simulate_visibility_change(&self, visibility: WindowVisibility) {
        let callback = {
            let mut state = self.0.lock();
            state.visibility = visibility;
            state.visibility_callback.take()
        };
        if let Some(mut callback) = callback {
            callback(visibility);
            self.0.lock().visibility_callback = Some(callback);
        }
    }

    pub fn simulate_visual_viewport_change(&self, bounds: Bounds<Pixels>) {
        let callback = {
            let mut state = self.0.lock();
            state.visual_viewport = Some(bounds);
            state.visual_viewport_callback.take()
        };
        if let Some(mut callback) = callback {
            callback();
            self.0.lock().visual_viewport_callback = Some(callback);
        }
    }

    pub fn simulate_insets_change(&self, insets: WindowInsets) {
        let callback = {
            let mut state = self.0.lock();
            state.insets = insets.clone();
            state.insets_callback.take()
        };
        if let Some(mut callback) = callback {
            callback(insets);
            self.0.lock().insets_callback = Some(callback);
        }
    }

    pub fn virtual_keyboard_requests(&self) -> usize {
        self.0.lock().virtual_keyboard_requests
    }

    pub fn virtual_keyboard_dismissals(&self) -> usize {
        self.0.lock().virtual_keyboard_dismissals
    }

    /// Every [`TextInputConfiguration`] forwarded to this window, in order.
    pub fn text_input_configurations(&self) -> Vec<TextInputConfiguration> {
        self.0.lock().text_input_configurations.clone()
    }

    pub fn text_input_state_changes(&self) -> Vec<TextInputStateChange> {
        self.0.lock().text_input_state_changes.clone()
    }

    pub fn simulate_resize(&mut self, size: Size<Pixels>) {
        let scale_factor = self.scale_factor();
        let mut lock = self.0.lock();
        // Always update bounds, even if no callback is registered
        lock.bounds.size = size;
        let Some(mut callback) = lock.resize_callback.take() else {
            return;
        };
        drop(lock);
        callback(size, scale_factor);
        self.0.lock().resize_callback = Some(callback);
    }

    /// Simulates a display scale change through the resize callback, preserving logical bounds.
    pub fn simulate_scale_factor_change(&mut self, scale_factor: f32) {
        let size = {
            let mut lock = self.0.lock();
            lock.scale_factor = scale_factor;
            lock.bounds.size
        };
        self.simulate_resize(size);
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

    pub fn simulate_appearance_change(&self, appearance: WindowAppearance) {
        let mut lock = self.0.lock();
        lock.appearance = appearance;
        let Some(mut callback) = lock.appearance_change_callback.take() else {
            return;
        };
        drop(lock);
        callback();
        self.0.lock().appearance_change_callback = Some(callback);
    }

    /// Returns how many times this window's frame waker has been invoked.
    pub fn frame_wake_count(&self) -> usize {
        self.0.lock().frame_wake_count.get()
    }

    /// Delivers a frame request to the window, as the platform's frame source
    /// would.
    pub fn simulate_frame_request(&self, options: RequestFrameOptions) {
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.request_frame_callback.take() else {
            return;
        };
        drop(lock);
        callback(options);
        self.0.lock().request_frame_callback = Some(callback);
    }

    pub fn simulate_input(&mut self, event: PlatformInput) -> bool {
        let mut lock = self.0.lock();
        let Some(mut callback) = lock.input_callback.take() else {
            return false;
        };
        drop(lock);
        let result = callback(event);
        self.0.lock().input_callback = Some(callback);
        !result.propagate
    }

    pub fn external_drag_files(&self) -> Vec<(PathBuf, bool)> {
        self.0.lock().external_drag_files.clone()
    }

    pub fn set_start_external_drag_result(&self, result: bool) {
        self.0.lock().start_external_drag_result = result;
    }
}

impl PlatformWindow for TestWindow {
    fn visual_viewport_bounds(&self) -> Bounds<Pixels> {
        let state = self.0.lock();
        state
            .visual_viewport
            .unwrap_or_else(|| Bounds::new(Point::default(), state.bounds.size))
    }

    fn on_visual_viewport_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().visual_viewport_callback = Some(callback);
    }

    fn insets(&self) -> WindowInsets {
        self.0.lock().insets.clone()
    }

    fn on_insets_changed(&self, callback: Box<dyn FnMut(WindowInsets)>) {
        self.0.lock().insets_callback = Some(callback);
    }

    fn show_soft_keyboard(&self) {
        self.0.lock().virtual_keyboard_requests += 1;
    }

    fn hide_soft_keyboard(&self) {
        self.0.lock().virtual_keyboard_dismissals += 1;
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.0.lock().bounds
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        let mut lock = self.0.lock();
        lock.bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        self.0.lock().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.0.lock().appearance
    }

    fn display(&self) -> Option<std::rc::Rc<dyn crate::PlatformDisplay>> {
        Some(self.0.lock().display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn capslock(&self) -> crate::Capslock {
        crate::Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.lock().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.lock().input_handler.take()
    }

    fn set_text_input_configuration(&mut self, configuration: TextInputConfiguration) {
        self.0.lock().text_input_configurations.push(configuration);
    }

    fn text_input_state_changed(&self, change: TextInputStateChange) {
        self.0.lock().text_input_state_changes.push(change);
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        Some(
            self.0
                .lock()
                .platform
                .upgrade()
                .expect("platform dropped")
                .prompt(msg, detail, answers),
        )
    }

    fn activate(&self) {
        self.0
            .lock()
            .platform
            .upgrade()
            .unwrap()
            .set_active_window(Some(self.clone()))
    }

    fn is_active(&self) -> bool {
        false
    }

    fn visibility(&self) -> WindowVisibility {
        self.0.lock().visibility
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: &str) {
        self.0.lock().title = Some(title.to_owned());
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn set_edited(&mut self, edited: bool) {
        self.0.lock().edited = edited;
    }

    fn set_document_path(&self, path: Option<&std::path::Path>) {
        self.0.lock().document_path = path.map(|p| p.to_path_buf());
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

    fn toggle_fullscreen(&self) {
        let mut lock = self.0.lock();
        lock.is_fullscreen = !lock.is_fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.0.lock().is_fullscreen
    }

    fn frame_waker(&self) -> Option<Rc<dyn Fn()>> {
        // Tests can inspect wakes without delivering a frame synchronously.
        let frame_wake_count = self.0.lock().frame_wake_count.clone();
        #[cfg(feature = "bench-support")]
        let window = Rc::downgrade(&self.0);
        Some(Rc::new(move || {
            frame_wake_count.set(frame_wake_count.get() + 1);
            #[cfg(feature = "bench-support")]
            if let Some(window) = window.upgrade() {
                TestWindow(window).schedule_frame();
            }
        }))
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.0.lock().request_frame_callback = Some(callback);
    }

    fn schedule_frame(&self) {
        let mut state = self.0.lock();
        if !state.frame_callback_pending {
            state.frame_scheduled = true;
        }
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> DispatchEventResult>) {
        self.0.lock().input_callback = Some(callback)
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().active_status_change_callback = Some(callback)
    }

    fn on_visibility_change(&self, callback: Box<dyn FnMut(WindowVisibility)>) {
        self.0.lock().visibility_callback = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().hover_status_change_callback = Some(callback)
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().resize_callback = Some(callback)
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().moved_callback = Some(callback)
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.lock().should_close_handler = Some(callback);
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.0.lock().hit_test_window_control_callback = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().appearance_change_callback = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        let scale_factor = self.scale_factor();
        let mut state = self.0.lock();
        state.frame_callback_pending = true;
        state.frame_scheduled = true;
        let device_size: Size<DevicePixels> = state.bounds.size.to_device_pixels(scale_factor);
        if let Some(renderer) = &mut state.renderer {
            renderer.render_scene(scene, device_size).warn_on_err();
        }
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    fn render_to_image(&self, scene: &Scene) -> anyhow::Result<RgbaImage> {
        let scale_factor = self.scale_factor();
        let mut state = self.0.lock();
        let size = state.bounds.size;
        if let Some(renderer) = &mut state.renderer {
            let device_size: Size<DevicePixels> = size.to_device_pixels(scale_factor);
            renderer.render_scene_to_image(scene, device_size)
        } else {
            anyhow::bail!("render_to_image not available: no HeadlessRenderer configured")
        }
    }

    fn as_test(&mut self) -> Option<&mut TestWindow> {
        Some(self)
    }

    #[cfg(target_os = "windows")]
    fn get_raw_handle(&self) -> windows::Win32::Foundation::HWND {
        unimplemented!()
    }

    fn show_window_menu(&self, _position: Point<Pixels>) {
        unimplemented!()
    }

    fn start_window_move(&self) {
        unimplemented!()
    }

    fn can_start_external_drag(&self) -> bool {
        true
    }

    fn start_external_drag(&self, payload: &crate::ExternalDragPayload) -> bool {
        let mut state = self.0.lock();
        match payload {
            crate::ExternalDragPayload::Files(paths) => {
                state.external_drag_files.extend_from_slice(paths.entries());
            }
        }
        state.start_external_drag_result
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
}

pub(crate) struct TestAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
    updates: Vec<TestAtlasUpdate>,
    resource_generation: u64,
}

#[derive(Clone, PartialEq, Eq)]
struct TestAtlasUpdate {
    key: AtlasKey,
    bounds: Bounds<DevicePixels>,
    bytes: Vec<u8>,
}

pub(crate) struct TestAtlas(Mutex<TestAtlasState>);

impl TestAtlas {
    pub fn new() -> Self {
        TestAtlas(Mutex::new(TestAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
            updates: Vec::new(),
            resource_generation: 0,
        }))
    }
}

impl PlatformAtlas for TestAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &crate::AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<crate::DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<crate::AtlasTile>> {
        let mut state = self.0.lock();
        if let Some(&tile) = state.tiles.get(key) {
            return Ok(Some(tile));
        }
        drop(state);

        let Some((size, _)) = build()? else {
            return Ok(None);
        };

        let mut state = self.0.lock();
        state.next_id += 1;
        let texture_id = state.next_id;
        state.next_id += 1;
        let tile_id = state.next_id;

        state.tiles.insert(
            key.clone(),
            crate::AtlasTile {
                texture_id: AtlasTextureId {
                    index: texture_id,
                    kind: key.texture_kind(),
                },
                tile_id: TileId(tile_id),
                padding: 0,
                bounds: crate::Bounds {
                    origin: Point::default(),
                    size,
                },
            },
        );

        Ok(Some(state.tiles[key]))
    }

    fn update(
        &self,
        key: &AtlasKey,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) -> anyhow::Result<()> {
        let mut state = self.0.lock();
        if state.tiles.contains_key(key) {
            state.updates.push(TestAtlasUpdate {
                key: key.clone(),
                bounds,
                bytes: bytes.to_vec(),
            });
        }
        Ok(())
    }

    fn resource_generation(&self) -> u64 {
        self.0.lock().resource_generation
    }

    fn remove(&self, key: &AtlasKey) {
        let mut state = self.0.lock();
        state.tiles.remove(key);
        state.updates.retain(|update| &update.key != key);
    }

    fn contains(&self, key: &AtlasKey) -> bool {
        self.0.lock().tiles.contains_key(key)
    }
}

#[cfg(test)]
mod dynamic_texture_tests {
    use super::*;
    use crate::{DynamicTextureId, DynamicTextureParams};
    use std::borrow::Cow;

    #[test]
    fn test_atlas_records_dynamic_texture_updates_and_generation() {
        let atlas = TestAtlas::new();
        let key = AtlasKey::DynamicTexture(DynamicTextureParams {
            texture_id: DynamicTextureId(11),
        });
        let texture_size = Size {
            width: DevicePixels(4),
            height: DevicePixels(3),
        };
        let mut build = || Ok(Some((texture_size, Cow::Owned(vec![0; 48]))));
        let tile = atlas.get_or_insert_with(&key, &mut build).unwrap().unwrap();
        let update_bounds = Bounds {
            origin: Point {
                x: DevicePixels(1),
                y: DevicePixels(1),
            },
            size: Size {
                width: DevicePixels(2),
                height: DevicePixels(1),
            },
        };
        let update_bytes = vec![7; 8];

        atlas.update(&key, update_bounds, &update_bytes).unwrap();
        atlas.0.lock().resource_generation = 3;

        let state = atlas.0.lock();
        assert_eq!(tile.texture_id.kind, crate::AtlasTextureKind::Polychrome);
        assert_eq!(state.updates.len(), 1);
        assert!(state.updates[0].key == key);
        assert_eq!(state.updates[0].bounds, update_bounds);
        assert_eq!(state.updates[0].bytes, update_bytes);
        drop(state);
        assert_eq!(atlas.resource_generation(), 3);
    }

    #[test]
    fn test_atlas_ignores_updates_for_missing_entries() {
        let atlas = TestAtlas::new();
        let key = AtlasKey::DynamicTexture(DynamicTextureParams {
            texture_id: DynamicTextureId(12),
        });
        let bounds = Bounds {
            origin: Point::default(),
            size: Size {
                width: DevicePixels(1),
                height: DevicePixels(1),
            },
        };

        atlas.update(&key, bounds, &[0; 4]).unwrap();

        assert!(atlas.0.lock().updates.is_empty());
    }
}
