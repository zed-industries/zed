use super::{AppVersion, CursorStyle, WindowBounds};
use crate::{
    geometry::vector::{vec2f, Vector2F},
    keymap, Action, ClipboardItem,
};
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use postage::oneshot;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use time::UtcOffset;

pub struct Platform {
    dispatcher: Arc<dyn super::Dispatcher>,
    fonts: Arc<dyn super::FontSystem>,
    current_clipboard_item: Mutex<Option<ClipboardItem>>,
    cursor: Mutex<CursorStyle>,
}

#[derive(Default)]
pub struct ForegroundPlatform {
    last_prompt_for_new_path_args: RefCell<Option<(PathBuf, oneshot::Sender<Option<PathBuf>>)>>,
}

struct Dispatcher;

pub struct Window {
    size: Vector2F,
    scale_factor: f32,
    current_scene: Option<crate::Scene>,
    event_handlers: Vec<Box<dyn FnMut(super::Event)>>,
    resize_handlers: Vec<Box<dyn FnMut()>>,
    close_handlers: Vec<Box<dyn FnOnce()>>,
    pub(crate) last_prompt: Cell<Option<oneshot::Sender<usize>>>,
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

    fn on_event(&self, _: Box<dyn FnMut(crate::Event) -> bool>) {}

    fn on_open_urls(&self, _: Box<dyn FnMut(Vec<String>)>) {}

    fn run(&self, _on_finish_launching: Box<dyn FnOnce() -> ()>) {
        unimplemented!()
    }

    fn on_menu_command(&self, _: Box<dyn FnMut(&dyn Action)>) {}
    fn on_validate_menu_command(&self, _: Box<dyn FnMut(&dyn Action) -> bool>) {}
    fn on_will_open_menu(&self, _: Box<dyn FnMut()>) {}
    fn set_menus(&self, _: Vec<crate::Menu>, _: &keymap::Matcher) {}

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
}

impl Platform {
    fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(super::current::FontSystem::new()),
            current_clipboard_item: Default::default(),
            cursor: Mutex::new(CursorStyle::Arrow),
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

    fn open_window(
        &self,
        _: usize,
        options: super::WindowOptions,
        _executor: Rc<super::executor::Foreground>,
    ) -> Box<dyn super::Window> {
        Box::new(Window::new(match options.bounds {
            WindowBounds::Maximized => vec2f(1024., 768.),
            WindowBounds::Fixed(rect) => rect.size(),
        }))
    }

    fn key_window_id(&self) -> Option<usize> {
        None
    }

    fn quit(&self) {}

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
}

impl Window {
    fn new(size: Vector2F) -> Self {
        Self {
            size,
            event_handlers: Vec::new(),
            resize_handlers: Vec::new(),
            close_handlers: Vec::new(),
            scale_factor: 1.0,
            current_scene: None,
            last_prompt: Default::default(),
        }
    }
}

impl super::Dispatcher for Dispatcher {
    fn is_main_thread(&self) -> bool {
        true
    }

    fn run_on_main_thread(&self, task: async_task::Runnable) {
        task.run();
    }
}

impl super::WindowContext for Window {
    fn size(&self) -> Vector2F {
        self.size
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn titlebar_height(&self) -> f32 {
        24.
    }

    fn present_scene(&mut self, scene: crate::Scene) {
        self.current_scene = Some(scene);
    }
}

impl super::Window for Window {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(crate::Event)>) {
        self.event_handlers.push(callback);
    }

    fn on_active_status_change(&mut self, _: Box<dyn FnMut(bool)>) {}

    fn on_resize(&mut self, callback: Box<dyn FnMut()>) {
        self.resize_handlers.push(callback);
    }

    fn on_close(&mut self, callback: Box<dyn FnOnce()>) {
        self.close_handlers.push(callback);
    }

    fn prompt(&self, _: crate::PromptLevel, _: &str, _: &[&str]) -> oneshot::Receiver<usize> {
        let (done_tx, done_rx) = oneshot::channel();
        self.last_prompt.replace(Some(done_tx));
        done_rx
    }

    fn activate(&self) {}
}

pub fn platform() -> Platform {
    Platform::new()
}

pub fn foreground_platform() -> ForegroundPlatform {
    ForegroundPlatform::default()
}
