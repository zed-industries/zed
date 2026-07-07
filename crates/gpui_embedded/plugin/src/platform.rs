use crate::dispatcher::PluginDispatcher;
use crate::text_system::PluginTextSystem;
use crate::window::{PluginWindow, PluginWindowState};
use crate::wit;
use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, Bounds, ClipboardItem, CursorStyle,
    DummyKeyboardMapper, ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Pixels,
    Platform, PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, Point, Size, Task, ThermalState, WindowAppearance, WindowParams, px, size,
};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

pub(crate) struct PendingView {
    pub view_id: u32,
    pub size: Size<Pixels>,
    pub scale_factor: f32,
}

/// The GPUI [`Platform`] implementation for Wasm plugin guests. There is no real display or
/// window here: each "window" is a view slot in the host application, and all rendering,
/// text shaping, and scheduling is delegated across the WIT boundary.
pub struct PluginPlatform {
    dispatcher: Arc<PluginDispatcher>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<PluginTextSystem>,
    display: Rc<PluginDisplay>,
    windows: RefCell<HashMap<u32, Rc<PluginWindowState>>>,
    pending_view: Cell<Option<PendingView>>,
}

impl PluginPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(PluginDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());
        Self {
            dispatcher,
            background_executor,
            foreground_executor,
            text_system: Arc::new(PluginTextSystem::new()),
            display: Rc::new(PluginDisplay::new()),
            windows: RefCell::new(HashMap::new()),
            pending_view: Cell::new(None),
        }
    }

    pub fn dispatcher(&self) -> &PluginDispatcher {
        &self.dispatcher
    }

    /// Bind the next `open_window` call to the given host view. See the `create-view` export.
    pub fn set_pending_view(&self, view_id: u32, size: Size<Pixels>, scale_factor: f32) {
        self.pending_view.set(Some(PendingView {
            view_id,
            size,
            scale_factor,
        }));
    }

    pub fn window(&self, view_id: u32) -> Option<Rc<PluginWindowState>> {
        self.windows.borrow().get(&view_id).cloned()
    }

    pub fn window_states(&self) -> Vec<Rc<PluginWindowState>> {
        self.windows.borrow().values().cloned().collect()
    }
}

impl Platform for PluginPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        // The host drives the run loop through the `tick` export; launching completes
        // synchronously. The caller must hold `Application::app_cell` to keep the app alive.
        on_finish_launching();
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        _params: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let pending = self.pending_view.take().ok_or_else(|| {
            anyhow!("plugin windows can only be opened for a host-created view (create-view)")
        })?;
        let state = Rc::new(PluginWindowState::new(
            pending.view_id,
            pending.size,
            pending.scale_factor,
            self.text_system.clone(),
        ));
        self.windows
            .borrow_mut()
            .insert(pending.view_id, state.clone());
        Ok(Box::new(PluginWindow::new(state, self.display.clone())))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("url schemes are not supported in plugins")))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (sender, receiver) = oneshot::channel();
        sender
            .send(Err(anyhow!("path prompts are not supported in plugins")))
            .ok();
        receiver
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (sender, receiver) = oneshot::channel();
        sender
            .send(Err(anyhow!("path prompts are not supported in plugins")))
            .ok();
        receiver
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn Action) -> bool>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, _callback: Box<dyn FnMut()>) {}

    fn is_cursor_visible(&self) -> bool {
        true
    }

    fn compositor_name(&self) -> &'static str {
        "GpuiPlugin"
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow!("app_path is not available in plugins"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow!(
            "auxiliary executables are not available in plugins"
        ))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        wit::set_cursor_style(cursor_style_to_wire(style));
    }

    fn hide_cursor_until_mouse_moves(&self) {}

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {}

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("credentials are not available in plugins")))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("credentials are not available in plugins")))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(PluginKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}
}

fn cursor_style_to_wire(style: CursorStyle) -> wit::CursorStyle {
    match style {
        CursorStyle::Arrow | CursorStyle::ContextualMenu => wit::CursorStyle::Arrow,
        CursorStyle::IBeam | CursorStyle::IBeamCursorForVerticalLayout => wit::CursorStyle::Ibeam,
        CursorStyle::Crosshair => wit::CursorStyle::Crosshair,
        CursorStyle::ClosedHand => wit::CursorStyle::ClosedHand,
        CursorStyle::OpenHand => wit::CursorStyle::OpenHand,
        CursorStyle::PointingHand | CursorStyle::DragLink | CursorStyle::DragCopy => {
            wit::CursorStyle::PointingHand
        }
        CursorStyle::ResizeLeft
        | CursorStyle::ResizeRight
        | CursorStyle::ResizeLeftRight
        | CursorStyle::ResizeColumn => wit::CursorStyle::ResizeLeftRight,
        CursorStyle::ResizeUp
        | CursorStyle::ResizeDown
        | CursorStyle::ResizeUpDown
        | CursorStyle::ResizeRow
        | CursorStyle::ResizeUpLeftDownRight
        | CursorStyle::ResizeUpRightDownLeft => wit::CursorStyle::ResizeUpDown,
        CursorStyle::OperationNotAllowed => wit::CursorStyle::OperationNotAllowed,
    }
}

struct PluginKeyboardLayout;

impl PlatformKeyboardLayout for PluginKeyboardLayout {
    fn id(&self) -> &str {
        "gpui-plugin"
    }

    fn name(&self) -> &str {
        "GPUI Plugin"
    }
}

#[derive(Debug)]
pub struct PluginDisplay {
    uuid: uuid::Uuid,
}

impl PluginDisplay {
    fn new() -> Self {
        Self {
            uuid: uuid::Uuid::from_u128(0x6770_7569_5f70_6c75_6769_6e00_0000_0001),
        }
    }
}

impl PlatformDisplay for PluginDisplay {
    fn id(&self) -> gpui::DisplayId {
        gpui::DisplayId::new(1)
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        Bounds {
            origin: Point::default(),
            size: size(px(8192.), px(8192.)),
        }
    }
}
