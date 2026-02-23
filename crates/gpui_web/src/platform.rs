use crate::dispatcher::WebDispatcher;
use crate::display::WebDisplay;
use crate::keyboard::WebKeyboardLayout;
use crate::window::WebWindow;
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Task,
    ThermalState, WindowAppearance, WindowParams,
};
use gpui_wgpu::WgpuContext;
use std::{
    borrow::Cow,
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use wasm_bindgen::prelude::*;

static BUNDLED_FONTS: &[&[u8]] = &[
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf"),
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Italic.ttf"),
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf"),
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf"),
    include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf"),
    include_bytes!("../../../assets/fonts/lilex/Lilex-Bold.ttf"),
    include_bytes!("../../../assets/fonts/lilex/Lilex-Italic.ttf"),
    include_bytes!("../../../assets/fonts/lilex/Lilex-BoldItalic.ttf"),
];

const CREDENTIAL_KEY_PREFIX: &str = "zed-credential:";

pub struct WebPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    active_window: RefCell<Option<AnyWindowHandle>>,
    active_display: Rc<dyn PlatformDisplay>,
    clipboard: Rc<RefCell<Option<ClipboardItem>>>,
    callbacks: RefCell<WebPlatformCallbacks>,
    wgpu_context: Rc<RefCell<Option<WgpuContext>>>,
    _paste_closure: RefCell<Option<Closure<dyn FnMut(web_sys::ClipboardEvent)>>>,
}

#[derive(Default)]
struct WebPlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_state_change: Option<Box<dyn FnMut()>>,
}

fn get_browser_window() -> Option<web_sys::Window> {
    web_sys::window()
}

fn get_document() -> Option<web_sys::Document> {
    get_browser_window()?.document()
}

fn get_local_storage() -> Option<web_sys::Storage> {
    get_browser_window()?.local_storage().ok()?
}

fn detect_window_appearance() -> WindowAppearance {
    let Some(window) = get_browser_window() else {
        return WindowAppearance::Light;
    };
    let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") else {
        return WindowAppearance::Light;
    };
    if media_query.matches() {
        WindowAppearance::Dark
    } else {
        WindowAppearance::Light
    }
}

fn write_text_to_navigator_clipboard(text: &str) {
    let Some(window) = get_browser_window() else {
        return;
    };
    let clipboard = window.navigator().clipboard();
    let promise = clipboard.write_text(text);
    let future = wasm_bindgen_futures::JsFuture::from(promise);
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = future.await {
            log::warn!("Failed to write to navigator.clipboard: {error:?}");
        }
    });
}

fn register_paste_listener(
    clipboard_cache: Rc<RefCell<Option<ClipboardItem>>>,
) -> Option<Closure<dyn FnMut(web_sys::ClipboardEvent)>> {
    let document = get_document()?;
    let closure = Closure::wrap(Box::new(move |event: web_sys::ClipboardEvent| {
        if let Some(data_transfer) = event.clipboard_data() {
            if let Ok(text) = data_transfer.get_data("text/plain") {
                if !text.is_empty() {
                    *clipboard_cache.borrow_mut() = Some(ClipboardItem::new_string(text));
                }
            }
        }
    }) as Box<dyn FnMut(web_sys::ClipboardEvent)>);

    document
        .add_event_listener_with_callback("paste", closure.as_ref().unchecked_ref())
        .ok()?;

    Some(closure)
}

impl WebPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(WebDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let text_system =
            Arc::new(gpui_cosmic_text::CosmicTextSystem::new_without_system_fonts("IBM Plex Sans"));
        let fonts = BUNDLED_FONTS
            .iter()
            .map(|bytes| Cow::Borrowed(*bytes))
            .collect();
        if let Err(error) = text_system.add_fonts(fonts) {
            log::error!("failed to load bundled fonts: {error:#}");
        }
        let text_system: Arc<dyn PlatformTextSystem> = text_system;
        let active_display: Rc<dyn PlatformDisplay> = Rc::new(WebDisplay::new());
        let clipboard: Rc<RefCell<Option<ClipboardItem>>> = Rc::new(RefCell::new(None));

        let paste_closure = register_paste_listener(Rc::clone(&clipboard));

        Self {
            background_executor,
            foreground_executor,
            text_system,
            active_window: RefCell::new(None),
            active_display,
            clipboard,
            callbacks: RefCell::new(WebPlatformCallbacks::default()),
            wgpu_context: Rc::new(RefCell::new(None)),
            _paste_closure: RefCell::new(paste_closure),
        }
    }
}

impl Platform for WebPlatform {
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
        let wgpu_context = self.wgpu_context.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match WgpuContext::new_async().await {
                Ok(context) => {
                    log::info!("WebGPU context initialized successfully");
                    *wgpu_context.borrow_mut() = Some(context);
                    on_finish_launching();
                }
                Err(err) => {
                    log::error!("Failed to initialize WebGPU context: {err:#}");
                    on_finish_launching();
                }
            }
        });
    }

    fn quit(&self) {
        log::warn!("WebPlatform::quit called, but quitting is not supported in the browser");
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.active_display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.active_display.clone())
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        *self.active_window.borrow()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let context_ref = self.wgpu_context.borrow();
        let context = context_ref.as_ref().ok_or_else(|| {
            anyhow::anyhow!("WebGPU context not initialized. Was Platform::run() called?")
        })?;

        let window = WebWindow::new(handle, params, context)?;
        *self.active_window.borrow_mut() = Some(handle);
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        detect_window_appearance()
    }

    fn open_url(&self, url: &str) {
        if let Some(window) = get_browser_window() {
            if let Err(error) = window.open_with_url(url) {
                log::warn!("Failed to open URL '{url}': {error:?}");
            }
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.callbacks.borrow_mut().open_urls = Some(callback);
    }

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow::anyhow!(
            "prompt_for_paths is not supported on the web"
        )))
        .ok();
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (sender, receiver) = oneshot::channel();
        sender
            .send(Err(anyhow::anyhow!(
                "prompt_for_new_path is not supported on the web"
            )))
            .ok();
        receiver
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().reopen = Some(callback);
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.callbacks.borrow_mut().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.callbacks.borrow_mut().validate_app_menu_command = Some(callback);
    }

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().thermal_state_change = Some(callback);
    }

    fn compositor_name(&self) -> &'static str {
        "Web"
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow::anyhow!("app_path is not available on the web"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "path_for_auxiliary_executable is not available on the web"
        ))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let css_cursor = match style {
            CursorStyle::Arrow => "default",
            CursorStyle::IBeam => "text",
            CursorStyle::Crosshair => "crosshair",
            CursorStyle::ClosedHand => "grabbing",
            CursorStyle::OpenHand => "grab",
            CursorStyle::PointingHand => "pointer",
            CursorStyle::ResizeLeft | CursorStyle::ResizeRight | CursorStyle::ResizeLeftRight => {
                "ew-resize"
            }
            CursorStyle::ResizeUp | CursorStyle::ResizeDown | CursorStyle::ResizeUpDown => {
                "ns-resize"
            }
            CursorStyle::ResizeUpLeftDownRight => "nesw-resize",
            CursorStyle::ResizeUpRightDownLeft => "nwse-resize",
            CursorStyle::ResizeColumn => "col-resize",
            CursorStyle::ResizeRow => "row-resize",
            CursorStyle::IBeamCursorForVerticalLayout => "vertical-text",
            CursorStyle::OperationNotAllowed => "not-allowed",
            CursorStyle::DragLink => "alias",
            CursorStyle::DragCopy => "copy",
            CursorStyle::ContextualMenu => "context-menu",
            CursorStyle::None => "none",
        };

        if let Some(document) = get_document() {
            if let Some(body) = document.body() {
                if let Err(error) = body.style().set_property("cursor", css_cursor) {
                    log::warn!("Failed to set cursor style: {error:?}");
                }
            }
        }
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.clipboard.borrow().clone()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        if let Some(text) = item.text() {
            write_text_to_navigator_clipboard(&text);
        }
        *self.clipboard.borrow_mut() = Some(item);
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "credential storage is not available on the web"
        )))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "credential storage is not available on the web"
        )))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(WebKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().keyboard_layout_change = Some(callback);
    }
}
