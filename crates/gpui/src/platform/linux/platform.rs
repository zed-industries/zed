#![allow(unused)]

use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, LinuxDisplay, LinuxTextSystem, LinuxWindow, Menu,
    PathPromptOptions, Platform, PlatformDisplay, PlatformInput, PlatformTextSystem,
    PlatformWindow, Result, SemanticVersion, Task, WindowOptions,
};

use futures::channel::oneshot;
use parking_lot::Mutex;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use time::UtcOffset;
use x11rb::{connection::Connection as _, rust_connection::RustConnection};

pub(crate) struct LinuxPlatform(Mutex<LinuxPlatformState>);

pub(crate) struct LinuxPlatformState {
    x11_connection: RustConnection,
    x11_root_index: usize,
    gpu: Arc<blade::Context>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<LinuxTextSystem>,
}

impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxPlatform {
    pub(crate) fn new() -> Self {
        let (x11_connection, x11_root_index) = x11rb::connect(None).unwrap();

        let dispatcher = Arc::new(LinuxDispatcher::new());
        let gpu = Arc::new(
            unsafe {
                blade::Context::init(blade::ContextDesc {
                    validation: cfg!(debug_assertions),
                    capture: false,
                })
            }
            .unwrap(),
        );

        Self(Mutex::new(LinuxPlatformState {
            x11_connection,
            x11_root_index,
            gpu,
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(LinuxTextSystem::new()),
        }))
    }
}

impl Platform for LinuxPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> crate::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching()
    }

    fn quit(&self) {}

    fn restart(&self) {}

    fn activate(&self, ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let lock = self.0.lock();
        let setup = lock.x11_connection.setup();
        (0..setup.roots.len())
            .map(|id| {
                Rc::new(LinuxDisplay::new(&lock.x11_connection, id)) as Rc<dyn PlatformDisplay>
            })
            .collect()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        let lock = self.0.lock();
        Some(Rc::new(LinuxDisplay::new(
            &lock.x11_connection,
            id.0 as usize,
        )))
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        let lock = self.0.lock();
        Box::new(LinuxWindow::new(
            options,
            handle,
            &lock.x11_connection,
            lock.x11_root_index,
            &lock.gpu,
        ))
    }

    fn set_display_link_output_callback(
        &self,
        display_id: DisplayId,
        callback: Box<dyn FnMut() + Send>,
    ) {
        unimplemented!()
    }

    fn start_display_link(&self, display_id: DisplayId) {}

    fn stop_display_link(&self, display_id: DisplayId) {}

    fn open_url(&self, url: &str) {}

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {}

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        unimplemented!()
    }

    fn reveal_path(&self, path: &Path) {}

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {}

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {}

    fn on_quit(&self, callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {}

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {}

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {}

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

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {}

    fn local_timezone(&self) -> UtcOffset {
        UtcOffset::UTC
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: CursorStyle) {}

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {}

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
}

#[cfg(test)]
mod tests {
    use crate::ClipboardItem;

    use super::*;

    fn build_platform() -> LinuxPlatform {
        let platform = LinuxPlatform::new();
        platform
    }
}
