#![allow(unused)]

use std::cell::RefCell;
use std::env;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::anyhow;
use ashpd::desktop::file_chooser::{OpenFileRequest, SaveFileRequest};
use async_task::Runnable;
use calloop::{EventLoop, LoopHandle, LoopSignal};
use flume::{Receiver, Sender};
use futures::channel::oneshot;
use parking_lot::Mutex;
use time::UtcOffset;
use wayland_client::Connection;

use crate::platform::linux::client::Client;
use crate::platform::linux::wayland::WaylandClient;
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, LinuxTextSystem, Menu, PathPromptOptions,
    Platform, PlatformDisplay, PlatformInput, PlatformTextSystem, PlatformWindow, Result,
    SemanticVersion, Task, WindowOptions,
};

use super::x11::X11Client;

#[derive(Default)]
pub(crate) struct Callbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

pub(crate) struct LinuxPlatformInner {
    pub(crate) event_loop: RefCell<EventLoop<'static, ()>>,
    pub(crate) loop_handle: Rc<LoopHandle<'static, ()>>,
    pub(crate) loop_signal: LoopSignal,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) text_system: Arc<LinuxTextSystem>,
    pub(crate) callbacks: RefCell<Callbacks>,
}

pub(crate) struct LinuxPlatform {
    client: Rc<dyn Client>,
    inner: Rc<LinuxPlatformInner>,
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

        let (main_sender, main_receiver) = calloop::channel::channel::<Runnable>();
        let text_system = Arc::new(LinuxTextSystem::new());
        let callbacks = RefCell::new(Callbacks::default());

        let event_loop = EventLoop::try_new().unwrap();
        event_loop
            .handle()
            .insert_source(main_receiver, |event, _, _| {
                if let calloop::channel::Event::Msg(runnable) = event {
                    runnable.run();
                }
            });

        let dispatcher = Arc::new(LinuxDispatcher::new(main_sender));

        let inner = Rc::new(LinuxPlatformInner {
            loop_handle: Rc::new(event_loop.handle()),
            loop_signal: event_loop.get_signal(),
            event_loop: RefCell::new(event_loop),
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher.clone()),
            text_system,
            callbacks,
        });

        if use_wayland {
            Self {
                client: Rc::new(WaylandClient::new(Rc::clone(&inner))),
                inner,
            }
        } else {
            Self {
                client: X11Client::new(Rc::clone(&inner)),
                inner,
            }
        }
    }
}

const KEYRING_LABEL: &str = "zed-github-account";

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
        on_finish_launching();

        self.inner
            .event_loop
            .borrow_mut()
            .run(None, &mut (), |&mut ()| {})
            .expect("Run loop failed");

        if let Some(mut fun) = self.inner.callbacks.borrow_mut().quit.take() {
            fun();
        }
    }

    fn quit(&self) {
        self.inner.loop_signal.stop();
    }

    // todo(linux)
    fn restart(&self) {}

    // todo(linux)
    fn activate(&self, ignoring_other_apps: bool) {}

    // todo(linux)
    fn hide(&self) {}

    // todo(linux)
    fn hide_other_apps(&self) {}

    // todo(linux)
    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.client.displays()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        self.client.display(id)
    }

    // todo(linux)
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
        open::that(url);
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.borrow_mut().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        let (done_tx, done_rx) = oneshot::channel();
        self.inner
            .foreground_executor
            .spawn(async move {
                let title = if options.multiple {
                    if !options.files {
                        "Open folders"
                    } else {
                        "Open files"
                    }
                } else {
                    if !options.files {
                        "Open folder"
                    } else {
                        "Open file"
                    }
                };

                let result = OpenFileRequest::default()
                    .modal(true)
                    .title(title)
                    .accept_label("Select")
                    .multiple(options.multiple)
                    .directory(options.directories)
                    .send()
                    .await
                    .ok()
                    .and_then(|request| request.response().ok())
                    .and_then(|response| {
                        response
                            .uris()
                            .iter()
                            .map(|uri| uri.to_file_path().ok())
                            .collect()
                    });

                done_tx.send(result);
            })
            .detach();
        done_rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        let (done_tx, done_rx) = oneshot::channel();
        let directory = directory.to_owned();
        self.inner
            .foreground_executor
            .spawn(async move {
                let result = SaveFileRequest::default()
                    .modal(true)
                    .title("Select new path")
                    .accept_label("Accept")
                    .send()
                    .await
                    .ok()
                    .and_then(|request| request.response().ok())
                    .and_then(|response| {
                        response
                            .uris()
                            .first()
                            .and_then(|uri| uri.to_file_path().ok())
                    });

                done_tx.send(result);
            })
            .detach();
        done_rx
    }

    fn reveal_path(&self, path: &Path) {
        if path.is_dir() {
            open::that(path);
            return;
        }
        // If `path` is a file, the system may try to open it in a text editor
        let dir = path.parent().unwrap_or(Path::new(""));
        open::that(dir);
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().reopen = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.inner.callbacks.borrow_mut().event = Some(callback);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner.callbacks.borrow_mut().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner.callbacks.borrow_mut().validate_app_menu_command = Some(callback);
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

    //todo(linux)
    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow::Error::msg(
            "Platform<LinuxPlatform>::app_path is not implemented yet",
        ))
    }

    // todo(linux)
    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {}

    fn local_timezone(&self) -> UtcOffset {
        UtcOffset::UTC
    }

    //todo(linux)
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        Err(anyhow::Error::msg(
            "Platform<LinuxPlatform>::path_for_auxiliary_executable is not implemented yet",
        ))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        self.client.set_cursor_style(style)
    }

    // todo(linux)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        let clipboard = self.client.get_clipboard();
        clipboard.borrow_mut().set_contents(item.text);
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let clipboard = self.client.get_clipboard();
        let contents = clipboard.borrow_mut().get_contents();
        match contents {
            Ok(text) => Some(ClipboardItem {
                metadata: None,
                text,
            }),
            _ => None,
        }
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor().spawn(async move {
            let keyring = oo7::Keyring::new().await?;
            keyring.unlock().await?;
            keyring
                .create_item(
                    KEYRING_LABEL,
                    &vec![("url", &url), ("username", &username)],
                    password,
                    true,
                )
                .await?;
            Ok(())
        })
    }

    //todo(linux): add trait methods for accessing the primary selection
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let url = url.to_string();
        self.background_executor().spawn(async move {
            let keyring = oo7::Keyring::new().await?;
            keyring.unlock().await?;

            let items = keyring.search_items(&vec![("url", &url)]).await?;

            for item in items.into_iter() {
                if item.label().await.is_ok_and(|label| label == KEYRING_LABEL) {
                    let attributes = item.attributes().await?;
                    let username = attributes
                        .get("username")
                        .ok_or_else(|| anyhow!("Cannot find username in stored credentials"))?;
                    let secret = item.secret().await?;

                    // we lose the zeroizing capabilities at this boundary,
                    // a current limitation GPUI's credentials api
                    return Ok(Some((username.to_string(), secret.to_vec())));
                } else {
                    continue;
                }
            }
            Ok(None)
        })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let url = url.to_string();
        self.background_executor().spawn(async move {
            let keyring = oo7::Keyring::new().await?;
            keyring.unlock().await?;

            let items = keyring.search_items(&vec![("url", &url)]).await?;

            for item in items.into_iter() {
                if item.label().await.is_ok_and(|label| label == KEYRING_LABEL) {
                    item.delete().await?;
                    return Ok(());
                }
            }

            Ok(())
        })
    }

    fn window_appearance(&self) -> crate::WindowAppearance {
        crate::WindowAppearance::Light
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
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
