#![allow(unused)]

use std::env;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use async_task::Runnable;
use flume::{Receiver, Sender};
use futures::channel::oneshot;
use parking_lot::Mutex;
use time::UtcOffset;
use wayland_client::Connection;

use crate::platform::linux::client::Client;
use crate::platform::linux::client_dispatcher::ClientDispatcher;
use crate::platform::linux::wayland::{WaylandClient, WaylandClientDispatcher};
use crate::platform::{X11Client, X11ClientDispatcher, XcbAtoms};
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, LinuxTextSystem, Menu, PathPromptOptions,
    Platform, PlatformDisplay, PlatformInput, PlatformTextSystem, PlatformWindow, Result,
    SemanticVersion, Task, WindowOptions,
};

#[derive(Default)]
pub(crate) struct Callbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    pub(crate) quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

pub(crate) struct LinuxPlatformInner {
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) main_receiver: flume::Receiver<Runnable>,
    pub(crate) text_system: Arc<LinuxTextSystem>,
    pub(crate) callbacks: Mutex<Callbacks>,
    pub(crate) state: Mutex<LinuxPlatformState>,
}

pub(crate) struct LinuxPlatform {
    client: Arc<dyn Client>,
    inner: Arc<LinuxPlatformInner>,
}

pub(crate) struct LinuxPlatformState {
    pub(crate) quit_requested: bool,
}

impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxPlatform {
    pub(crate) fn new() -> Self {
        let wayland_display = env::var_os("WAYLAND_DISPLAY");
        let use_wayland = wayland_display.is_some() && !wayland_display.unwrap().is_empty();

        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let text_system = Arc::new(LinuxTextSystem::new());
        let callbacks = Mutex::new(Callbacks::default());
        let state = Mutex::new(LinuxPlatformState {
            quit_requested: false,
        });

        if use_wayland {
            Self::new_wayland(main_sender, main_receiver, text_system, callbacks, state)
        } else {
            Self::new_x11(main_sender, main_receiver, text_system, callbacks, state)
        }
    }

    fn new_wayland(
        main_sender: Sender<Runnable>,
        main_receiver: Receiver<Runnable>,
        text_system: Arc<LinuxTextSystem>,
        callbacks: Mutex<Callbacks>,
        state: Mutex<LinuxPlatformState>,
    ) -> Self {
        let conn = Arc::new(Connection::connect_to_env().unwrap());
        let client_dispatcher: Arc<dyn ClientDispatcher + Send + Sync> =
            Arc::new(WaylandClientDispatcher::new(&conn));
        let dispatcher = Arc::new(LinuxDispatcher::new(main_sender, &client_dispatcher));
        let inner = Arc::new(LinuxPlatformInner {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher.clone()),
            main_receiver,
            text_system,
            callbacks,
            state,
        });
        let client = Arc::new(WaylandClient::new(Arc::clone(&inner), Arc::clone(&conn)));
        Self {
            client,
            inner: Arc::clone(&inner),
        }
    }

    fn new_x11(
        main_sender: Sender<Runnable>,
        main_receiver: Receiver<Runnable>,
        text_system: Arc<LinuxTextSystem>,
        callbacks: Mutex<Callbacks>,
        state: Mutex<LinuxPlatformState>,
    ) -> Self {
        let (xcb_connection, x_root_index) = xcb::Connection::connect(None).unwrap();
        let atoms = XcbAtoms::intern_all(&xcb_connection).unwrap();
        let xcb_connection = Arc::new(xcb_connection);
        let client_dispatcher: Arc<dyn ClientDispatcher + Send + Sync> =
            Arc::new(X11ClientDispatcher::new(&xcb_connection, x_root_index));
        let dispatcher = Arc::new(LinuxDispatcher::new(main_sender, &client_dispatcher));
        let inner = Arc::new(LinuxPlatformInner {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher.clone()),
            main_receiver,
            text_system,
            callbacks,
            state,
        });
        let client = Arc::new(X11Client::new(
            Arc::clone(&inner),
            xcb_connection,
            x_root_index,
            atoms,
        ));
        Self {
            client,
            inner: Arc::clone(&inner),
        }
    }
}

impl Platform for LinuxPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.inner.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.inner.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.inner.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        self.client.run(on_finish_launching)
    }

    fn quit(&self) {
        self.inner.state.lock().quit_requested = true;
    }

    //todo!(linux)
    fn restart(&self) {}

    //todo!(linux)
    fn activate(&self, ignoring_other_apps: bool) {}

    //todo!(linux)
    fn hide(&self) {}

    //todo!(linux)
    fn hide_other_apps(&self) {}

    //todo!(linux)
    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.client.displays()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        self.client.display(id)
    }

    //todo!(linux)
    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        self.client.open_window(handle, options)
    }

    fn open_url(&self, url: &str) {
        unimplemented!()
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.lock().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        unimplemented!()
    }

    fn reveal_path(&self, path: &Path) {
        unimplemented!()
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().reopen = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.inner.callbacks.lock().event = Some(callback);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner.callbacks.lock().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner.callbacks.lock().validate_app_menu_command = Some(callback);
    }

    fn os_name(&self) -> &'static str {
        "Linux"
    }

    fn double_click_interval(&self) -> Duration {
        Duration::default()
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    fn app_path(&self) -> Result<PathBuf> {
        unimplemented!()
    }

    //todo!(linux)
    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {}

    fn local_timezone(&self) -> UtcOffset {
        UtcOffset::UTC
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unimplemented!()
    }

    //todo!(linux)
    fn set_cursor_style(&self, style: CursorStyle) {}

    //todo!(linux)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    //todo!(linux)
    fn write_to_clipboard(&self, item: ClipboardItem) {}

    //todo!(linux)
    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        unimplemented!()
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        unimplemented!()
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        unimplemented!()
    }

    fn window_appearance(&self) -> crate::WindowAppearance {
        crate::WindowAppearance::Light
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_platform() -> LinuxPlatform {
        let platform = LinuxPlatform::new();
        platform
    }
}
