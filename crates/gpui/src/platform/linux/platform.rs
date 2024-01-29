#![allow(unused)]

use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, LinuxDisplay, LinuxTextSystem, LinuxWindow,
    LinuxWindowState, LinuxWindowStatePtr, Menu, PathPromptOptions, Platform, PlatformDisplay,
    PlatformInput, PlatformTextSystem, PlatformWindow, Result, SemanticVersion, Task,
    WindowOptions,
};

use collections::{HashMap, HashSet};
use futures::channel::oneshot;
use parking_lot::Mutex;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use time::UtcOffset;
use xcb::{x, Xid as _};

xcb::atoms_struct! {
    #[derive(Debug)]
    pub(crate) struct XcbAtoms {
        pub wm_protocols    => b"WM_PROTOCOLS",
        pub wm_del_window   => b"WM_DELETE_WINDOW",
        wm_state        => b"_NET_WM_STATE",
        wm_state_maxv   => b"_NET_WM_STATE_MAXIMIZED_VERT",
        wm_state_maxh   => b"_NET_WM_STATE_MAXIMIZED_HORZ",
    }
}

pub(crate) struct LinuxPlatform(Mutex<LinuxPlatformState>);

pub(crate) struct LinuxPlatformState {
    xcb_connection: xcb::Connection,
    x_root_index: i32,
    atoms: XcbAtoms,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    windows: HashMap<x::Window, LinuxWindowStatePtr>,
    text_system: Arc<LinuxTextSystem>,
}

impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxPlatform {
    pub(crate) fn new() -> Self {
        let (xcb_connection, x_root_index) = xcb::Connection::connect(None).unwrap();
        let atoms = XcbAtoms::intern_all(&xcb_connection).unwrap();

        let dispatcher = Arc::new(LinuxDispatcher::new());

        Self(Mutex::new(LinuxPlatformState {
            xcb_connection,
            x_root_index,
            atoms,
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            windows: HashMap::default(),
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
        on_finish_launching();

        while !self.0.lock().windows.is_empty() {
            let event = self.0.lock().xcb_connection.wait_for_event().unwrap();
            let mut repaint_x_window = None;
            match event {
                xcb::Event::X(x::Event::ClientMessage(ev)) => {
                    if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                        let mut this = self.0.lock();
                        if atom == this.atoms.wm_del_window.resource_id() {
                            // window "x" button clicked by user, we gracefully exit
                            {
                                let mut window = this.windows[&ev.window()].lock();
                                window.destroy();
                            }
                            this.xcb_connection.send_request(&x::UnmapWindow {
                                window: ev.window(),
                            });
                            this.xcb_connection.send_request(&x::DestroyWindow {
                                window: ev.window(),
                            });
                            this.windows.remove(&ev.window());
                            break;
                        }
                    }
                }
                xcb::Event::X(x::Event::Expose(ev)) => {
                    repaint_x_window = Some(ev.window());
                }
                xcb::Event::X(x::Event::ResizeRequest(ev)) => {
                    let this = self.0.lock();
                    LinuxWindowState::resize(&this.windows[&ev.window()], ev.width(), ev.height());
                }
                _ => {}
            }

            if let Some(x_window) = repaint_x_window {
                let this = self.0.lock();
                LinuxWindowState::request_frame(&this.windows[&x_window]);
                this.xcb_connection.flush();
            }
        }
    }

    fn quit(&self) {}

    fn restart(&self) {}

    fn activate(&self, ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let this = self.0.lock();
        let setup = this.xcb_connection.get_setup();
        setup
            .roots()
            .enumerate()
            .map(|(root_id, _)| {
                Rc::new(LinuxDisplay::new(&this.xcb_connection, root_id as i32))
                    as Rc<dyn PlatformDisplay>
            })
            .collect()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        let this = self.0.lock();
        Some(Rc::new(LinuxDisplay::new(
            &this.xcb_connection,
            id.0 as i32,
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
        let mut this = self.0.lock();
        let x_window = this.xcb_connection.generate_id();

        let window_ptr = LinuxWindowState::new_ptr(
            options,
            handle,
            &this.xcb_connection,
            this.x_root_index,
            x_window,
            &this.atoms,
        );
        this.windows.insert(x_window, window_ptr.clone());
        Box::new(LinuxWindow(window_ptr))
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
