use super::{AppVersion, CursorStyle, WindowBounds};
use anyhow::{anyhow, Result};
use collections::VecDeque;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    keymap_matcher::KeymapMatcher,
    Action, AnyWindowHandle, ClipboardItem, Menu,
};
use parking_lot::Mutex;
use postage::oneshot;
use std::{
    any::Any,
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use time::UtcOffset;

struct Dispatcher;

impl super::Dispatcher for Dispatcher {
    fn is_main_thread(&self) -> bool {
        true
    }

    fn run_on_main_thread(&self, task: async_task::Runnable) {
        task.run();
    }
}

pub fn foreground_platform() -> ForegroundPlatform {
    ForegroundPlatform::default()
}

#[derive(Default)]
pub struct ForegroundPlatform {
    last_prompt_for_new_path_args: RefCell<Option<(PathBuf, oneshot::Sender<Option<PathBuf>>)>>,
}

#[cfg(any(test, feature = "test-support"))]
impl ForegroundPlatform {
    pub(crate) fn simulate_new_path_selection(
        &self,
        result: impl FnOnce(PathBuf) -> Option<PathBuf>,
    ) {
        let (dir_path, mut done_tx) = self
            .last_prompt_for_new_path_args
            .take()
            .expect("prompt_for_new_path was not called");
        let _ = postage::sink::Sink::try_send(&mut done_tx, result(dir_path));
    }

    pub(crate) fn did_prompt_for_new_path(&self) -> bool {
        self.last_prompt_for_new_path_args.borrow().is_some()
    }
}

impl super::ForegroundPlatform for ForegroundPlatform {
    fn on_become_active(&self, _: Box<dyn FnMut()>) {}
    fn on_resign_active(&self, _: Box<dyn FnMut()>) {}
    fn on_quit(&self, _: Box<dyn FnMut()>) {}
    fn on_reopen(&self, _: Box<dyn FnMut()>) {}
    fn on_event(&self, _: Box<dyn FnMut(super::Event) -> bool>) {}
    fn on_open_urls(&self, _: Box<dyn FnMut(Vec<String>)>) {}

    fn run(&self, _on_finish_launching: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn on_menu_command(&self, _: Box<dyn FnMut(&dyn Action)>) {}
    fn on_validate_menu_command(&self, _: Box<dyn FnMut(&dyn Action) -> bool>) {}
    fn on_will_open_menu(&self, _: Box<dyn FnMut()>) {}
    fn set_menus(&self, _: Vec<Menu>, _: &KeymapMatcher) {}

    fn prompt_for_paths(
        &self,
        _: super::PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        let (_done_tx, done_rx) = oneshot::channel();
        done_rx
    }

    fn prompt_for_new_path(&self, path: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        let (done_tx, done_rx) = oneshot::channel();
        *self.last_prompt_for_new_path_args.borrow_mut() = Some((path.to_path_buf(), done_tx));
        done_rx
    }

    fn reveal_path(&self, _: &Path) {}
}

pub struct Platform {
    dispatcher: Arc<dyn super::Dispatcher>,
    fonts: Arc<dyn super::FontSystem>,
    current_clipboard_item: Mutex<Option<ClipboardItem>>,
    cursor: Mutex<CursorStyle>,
    active_window: Arc<Mutex<Option<AnyWindowHandle>>>,
}

impl Platform {
    pub fn new(fonts: Arc<dyn super::FontSystem>) -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts,
            current_clipboard_item: Default::default(),
            cursor: Mutex::new(CursorStyle::Arrow),
            active_window: Default::default(),
        }
    }
}

impl super::Platform for Platform {
    fn dispatcher(&self) -> Arc<dyn super::Dispatcher> {
        self.dispatcher.clone()
    }

    fn fonts(&self) -> std::sync::Arc<dyn super::FontSystem> {
        self.fonts.clone()
    }

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn quit(&self) {}

    fn screen_by_id(&self, _id: uuid::Uuid) -> Option<Rc<dyn super::Screen>> {
        None
    }

    fn screens(&self) -> Vec<Rc<dyn super::Screen>> {
        Default::default()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: super::WindowOptions,
        _executor: Rc<super::executor::Foreground>,
    ) -> Box<dyn super::Window> {
        *self.active_window.lock() = Some(handle);
        Box::new(Window::new(
            handle,
            match options.bounds {
                WindowBounds::Maximized | WindowBounds::Fullscreen => vec2f(1024., 768.),
                WindowBounds::Fixed(rect) => rect.size(),
            },
            self.active_window.clone(),
        ))
    }

    fn main_window(&self) -> Option<AnyWindowHandle> {
        self.active_window.lock().clone()
    }

    fn add_status_item(&self, handle: AnyWindowHandle) -> Box<dyn super::Window> {
        Box::new(Window::new(
            handle,
            vec2f(24., 24.),
            self.active_window.clone(),
        ))
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.current_clipboard_item.lock() = Some(item);
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.current_clipboard_item.lock().clone()
    }

    fn open_url(&self, _: &str) {}

    fn write_credentials(&self, _: &str, _: &str, _: &[u8]) -> Result<()> {
        Ok(())
    }

    fn read_credentials(&self, _: &str) -> Result<Option<(String, Vec<u8>)>> {
        Ok(None)
    }

    fn delete_credentials(&self, _: &str) -> Result<()> {
        Ok(())
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        *self.cursor.lock() = style;
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn local_timezone(&self) -> UtcOffset {
        UtcOffset::UTC
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow!("app not running inside a bundle"))
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow!("app not running inside a bundle"))
    }

    fn app_version(&self) -> Result<AppVersion> {
        Ok(AppVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    fn os_name(&self) -> &'static str {
        "test"
    }

    fn os_version(&self) -> Result<AppVersion> {
        Ok(AppVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    fn restart(&self) {}
}

#[derive(Debug)]
pub struct Screen;

impl super::Screen for Screen {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bounds(&self) -> RectF {
        RectF::new(Vector2F::zero(), Vector2F::new(1920., 1080.))
    }

    fn content_bounds(&self) -> RectF {
        RectF::new(Vector2F::zero(), Vector2F::new(1920., 1080.))
    }

    fn display_uuid(&self) -> Option<uuid::Uuid> {
        Some(uuid::Uuid::new_v4())
    }
}

pub struct Window {
    handle: AnyWindowHandle,
    pub(crate) size: Vector2F,
    scale_factor: f32,
    current_scene: Option<crate::Scene>,
    event_handlers: Vec<Box<dyn FnMut(super::Event) -> bool>>,
    pub(crate) resize_handlers: Vec<Box<dyn FnMut()>>,
    pub(crate) moved_handlers: Vec<Box<dyn FnMut()>>,
    close_handlers: Vec<Box<dyn FnOnce()>>,
    fullscreen_handlers: Vec<Box<dyn FnMut(bool)>>,
    pub(crate) active_status_change_handlers: Vec<Box<dyn FnMut(bool)>>,
    pub(crate) should_close_handler: Option<Box<dyn FnMut() -> bool>>,
    pub(crate) title: Option<String>,
    pub(crate) edited: bool,
    pub(crate) pending_prompts: RefCell<VecDeque<oneshot::Sender<usize>>>,
    active_window: Arc<Mutex<Option<AnyWindowHandle>>>,
}

impl Window {
    pub fn new(
        handle: AnyWindowHandle,
        size: Vector2F,
        active_window: Arc<Mutex<Option<AnyWindowHandle>>>,
    ) -> Self {
        Self {
            handle,
            size,
            event_handlers: Default::default(),
            resize_handlers: Default::default(),
            moved_handlers: Default::default(),
            close_handlers: Default::default(),
            should_close_handler: Default::default(),
            active_status_change_handlers: Default::default(),
            fullscreen_handlers: Default::default(),
            scale_factor: 1.0,
            current_scene: None,
            title: None,
            edited: false,
            pending_prompts: Default::default(),
            active_window,
        }
    }
}

impl super::Window for Window {
    fn bounds(&self) -> WindowBounds {
        WindowBounds::Fixed(RectF::new(Vector2F::zero(), self.size))
    }

    fn content_size(&self) -> Vector2F {
        self.size
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn titlebar_height(&self) -> f32 {
        24.
    }

    fn appearance(&self) -> super::Appearance {
        super::Appearance::Light
    }

    fn screen(&self) -> Rc<dyn super::Screen> {
        Rc::new(Screen)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_input_handler(&mut self, _: Box<dyn super::InputHandler>) {}

    fn prompt(
        &self,
        _: super::PromptLevel,
        _: &str,
        _: &[&str],
    ) -> oneshot::Receiver<usize> {
        let (done_tx, done_rx) = oneshot::channel();
        self.pending_prompts.borrow_mut().push_back(done_tx);
        done_rx
    }

    fn activate(&self) {
        *self.active_window.lock() = Some(self.handle);
    }

    fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string())
    }

    fn set_edited(&mut self, edited: bool) {
        self.edited = edited;
    }

    fn show_character_palette(&self) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn present_scene(&mut self, scene: crate::Scene) {
        self.current_scene = Some(scene);
    }

    fn toggle_full_screen(&self) {}

    fn on_event(&mut self, callback: Box<dyn FnMut(super::Event) -> bool>) {
        self.event_handlers.push(callback);
    }

    fn on_active_status_change(&mut self, callback: Box<dyn FnMut(bool)>) {
        self.active_status_change_handlers.push(callback);
    }

    fn on_resize(&mut self, callback: Box<dyn FnMut()>) {
        self.resize_handlers.push(callback);
    }

    fn on_fullscreen(&mut self, callback: Box<dyn FnMut(bool)>) {
        self.fullscreen_handlers.push(callback)
    }

    fn on_moved(&mut self, callback: Box<dyn FnMut()>) {
        self.moved_handlers.push(callback);
    }

    fn on_should_close(&mut self, callback: Box<dyn FnMut() -> bool>) {
        self.should_close_handler = Some(callback);
    }

    fn on_close(&mut self, callback: Box<dyn FnOnce()>) {
        self.close_handlers.push(callback);
    }

    fn on_appearance_changed(&mut self, _: Box<dyn FnMut()>) {}

    fn is_topmost_for_position(&self, _position: Vector2F) -> bool {
        true
    }
}
