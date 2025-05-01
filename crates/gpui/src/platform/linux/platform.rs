use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
    sync::Arc,
};
#[cfg(any(feature = "wayland", feature = "x11"))]
use std::{
    ffi::OsString,
    fs::File,
    io::Read as _,
    os::fd::{AsFd, AsRawFd, FromRawFd},
    time::Duration,
};

use anyhow::{Context as _, anyhow};
use async_task::Runnable;
use calloop::{LoopSignal, channel::Channel};
use futures::channel::oneshot;
use util::ResultExt as _;
#[cfg(any(feature = "wayland", feature = "x11"))]
use xkbcommon::xkb::{self, Keycode, Keysym, State};

use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, Menu, MenuItem, OwnedMenu, PathPromptOptions,
    Pixels, Platform, PlatformDisplay, PlatformKeyboardLayout, PlatformTextSystem, PlatformWindow,
    Point, Result, ScreenCaptureSource, Task, WindowAppearance, WindowParams, px,
};

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) const SCROLL_LINES: f32 = 3.0;

// Values match the defaults on GTK.
// Taken from https://github.com/GNOME/gtk/blob/main/gtk/gtksettings.c#L320
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(400);
pub(crate) const DOUBLE_CLICK_DISTANCE: Pixels = px(5.0);
pub(crate) const KEYRING_LABEL: &str = "zed-github-account";

#[cfg(any(feature = "wayland", feature = "x11"))]
const FILE_PICKER_PORTAL_MISSING: &str =
    "Couldn't open file picker due to missing xdg-desktop-portal implementation.";

pub trait LinuxClient {
    fn compositor_name(&self) -> &'static str;
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R;
    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout>;
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    #[allow(unused)]
    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>;
    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn is_screen_capture_supported(&self) -> bool;
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>>;

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>>;
    fn set_cursor_style(&self, style: CursorStyle);
    fn open_uri(&self, uri: &str);
    fn reveal_path(&self, path: PathBuf);
    fn write_to_primary(&self, item: ClipboardItem);
    fn write_to_clipboard(&self, item: ClipboardItem);
    fn read_from_primary(&self) -> Option<ClipboardItem>;
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;
    fn active_window(&self) -> Option<AnyWindowHandle>;
    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>>;
    fn run(&self);
}

#[derive(Default)]
pub(crate) struct PlatformHandlers {
    pub(crate) open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    pub(crate) quit: Option<Box<dyn FnMut()>>,
    pub(crate) reopen: Option<Box<dyn FnMut()>>,
    pub(crate) app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    pub(crate) will_open_app_menu: Option<Box<dyn FnMut()>>,
    pub(crate) validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    pub(crate) keyboard_layout_change: Option<Box<dyn FnMut()>>,
}

pub(crate) struct LinuxCommon {
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) text_system: Arc<dyn PlatformTextSystem>,
    pub(crate) appearance: WindowAppearance,
    pub(crate) auto_hide_scrollbars: bool,
    pub(crate) callbacks: PlatformHandlers,
    pub(crate) signal: LoopSignal,
    pub(crate) menus: Vec<OwnedMenu>,
}

impl LinuxCommon {
    pub fn new(signal: LoopSignal) -> (Self, Channel<Runnable>) {
        let (main_sender, main_receiver) = calloop::channel::channel::<Runnable>();

        #[cfg(any(feature = "wayland", feature = "x11"))]
        let text_system = Arc::new(crate::CosmicTextSystem::new());
        #[cfg(not(any(feature = "wayland", feature = "x11")))]
        let text_system = Arc::new(crate::NoopTextSystem::new());

        let callbacks = PlatformHandlers::default();

        let dispatcher = Arc::new(LinuxDispatcher::new(main_sender.clone()));

        let background_executor = BackgroundExecutor::new(dispatcher.clone());

        let common = LinuxCommon {
            background_executor,
            foreground_executor: ForegroundExecutor::new(dispatcher.clone()),
            text_system,
            appearance: WindowAppearance::Light,
            auto_hide_scrollbars: false,
            callbacks,
            signal,
            menus: Vec::new(),
        };

        (common, main_receiver)
    }
}

impl<P: LinuxClient + 'static> Platform for P {
    fn background_executor(&self) -> BackgroundExecutor {
        self.with_common(|common| common.background_executor.clone())
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.with_common(|common| common.foreground_executor.clone())
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.with_common(|common| common.text_system.clone())
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        self.keyboard_layout()
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.with_common(|common| common.callbacks.keyboard_layout_change = Some(callback));
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();

        LinuxClient::run(self);

        let quit = self.with_common(|common| common.callbacks.quit.take());
        if let Some(mut fun) = quit {
            fun();
        }
    }

    fn quit(&self) {
        self.with_common(|common| common.signal.stop());
    }

    fn compositor_name(&self) -> &'static str {
        self.compositor_name()
    }

    fn restart(&self, binary_path: Option<PathBuf>) {
        use std::os::unix::process::CommandExt as _;

        // get the process id of the current process
        let app_pid = std::process::id().to_string();
        // get the path to the executable
        let app_path = if let Some(path) = binary_path {
            path
        } else {
            match self.app_path() {
                Ok(path) => path,
                Err(err) => {
                    log::error!("Failed to get app path: {:?}", err);
                    return;
                }
            }
        };

        log::info!("Restarting process, using app path: {:?}", app_path);

        // Script to wait for the current process to exit and then restart the app.
        let script = format!(
            r#"
            while kill -0 {pid} 2>/dev/null; do
                sleep 0.1
            done

            {app_path}
            "#,
            pid = app_pid,
            app_path = app_path.display()
        );

        // execute the script using /bin/bash
        let restart_process = Command::new("/bin/bash")
            .arg("-c")
            .arg(script)
            .process_group(0)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        log::info!("activate is not implemented on Linux, ignoring the call")
    }

    fn hide(&self) {
        log::info!("hide is not implemented on Linux, ignoring the call")
    }

    fn hide_other_apps(&self) {
        log::info!("hide_other_apps is not implemented on Linux, ignoring the call")
    }

    fn unhide_other_apps(&self) {
        log::info!("unhide_other_apps is not implemented on Linux, ignoring the call")
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.primary_display()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.displays()
    }

    fn is_screen_capture_supported(&self) -> bool {
        self.is_screen_capture_supported()
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>> {
        self.screen_capture_sources()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.active_window()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        self.window_stack()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        self.open_window(handle, options)
    }

    fn open_url(&self, url: &str) {
        self.open_uri(url);
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.with_common(|common| common.callbacks.open_urls = Some(callback));
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (done_tx, done_rx) = oneshot::channel();

        #[cfg(not(any(feature = "wayland", feature = "x11")))]
        let _ = (done_tx.send(Ok(None)), options);

        #[cfg(any(feature = "wayland", feature = "x11"))]
        self.foreground_executor()
            .spawn(async move {
                let title = if options.directories {
                    "Open Folder"
                } else {
                    "Open File"
                };

                let request = match ashpd::desktop::file_chooser::OpenFileRequest::default()
                    .modal(true)
                    .title(title)
                    .multiple(options.multiple)
                    .directory(options.directories)
                    .send()
                    .await
                {
                    Ok(request) => request,
                    Err(err) => {
                        let result = match err {
                            ashpd::Error::PortalNotFound(_) => anyhow!(FILE_PICKER_PORTAL_MISSING),
                            err => err.into(),
                        };
                        let _ = done_tx.send(Err(result));
                        return;
                    }
                };

                let result = match request.response() {
                    Ok(response) => Ok(Some(
                        response
                            .uris()
                            .iter()
                            .filter_map(|uri| uri.to_file_path().ok())
                            .collect::<Vec<_>>(),
                    )),
                    Err(ashpd::Error::Response(_)) => Ok(None),
                    Err(e) => Err(e.into()),
                };
                let _ = done_tx.send(result);
            })
            .detach();
        done_rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (done_tx, done_rx) = oneshot::channel();

        #[cfg(not(any(feature = "wayland", feature = "x11")))]
        let _ = (done_tx.send(Ok(None)), directory);

        #[cfg(any(feature = "wayland", feature = "x11"))]
        self.foreground_executor()
            .spawn({
                let directory = directory.to_owned();

                async move {
                    let request = match ashpd::desktop::file_chooser::SaveFileRequest::default()
                        .modal(true)
                        .title("Save File")
                        .current_folder(directory)
                        .expect("pathbuf should not be nul terminated")
                        .send()
                        .await
                    {
                        Ok(request) => request,
                        Err(err) => {
                            let result = match err {
                                ashpd::Error::PortalNotFound(_) => {
                                    anyhow!(FILE_PICKER_PORTAL_MISSING)
                                }
                                err => err.into(),
                            };
                            let _ = done_tx.send(Err(result));
                            return;
                        }
                    };

                    let result = match request.response() {
                        Ok(response) => Ok(response
                            .uris()
                            .first()
                            .and_then(|uri| uri.to_file_path().ok())),
                        Err(ashpd::Error::Response(_)) => Ok(None),
                        Err(e) => Err(e.into()),
                    };
                    let _ = done_tx.send(result);
                }
            })
            .detach();

        done_rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        // org.freedesktop.portal.FileChooser only supports "pick files" and "pick directories".
        false
    }

    fn reveal_path(&self, path: &Path) {
        self.reveal_path(path.to_owned());
    }

    fn open_with_system(&self, path: &Path) {
        let path = path.to_owned();
        self.background_executor()
            .spawn(async move {
                let _ = std::process::Command::new("xdg-open")
                    .arg(path)
                    .spawn()
                    .context("invoking xdg-open")
                    .log_err();
            })
            .detach();
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.with_common(|common| {
            common.callbacks.quit = Some(callback);
        });
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.with_common(|common| {
            common.callbacks.reopen = Some(callback);
        });
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.with_common(|common| {
            common.callbacks.app_menu_action = Some(callback);
        });
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.with_common(|common| {
            common.callbacks.will_open_app_menu = Some(callback);
        });
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.with_common(|common| {
            common.callbacks.validate_app_menu_command = Some(callback);
        });
    }

    fn app_path(&self) -> Result<PathBuf> {
        // get the path of the executable of the current process
        let exe_path = env::current_exe()?;
        Ok(exe_path)
    }

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        self.with_common(|common| {
            common.menus = menus.into_iter().map(|menu| menu.owned()).collect();
        })
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        self.with_common(|common| Some(common.menus.clone()))
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {
        // todo(linux)
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::Error::msg(
            "Platform<LinuxPlatform>::path_for_auxiliary_executable is not implemented yet",
        ))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        self.set_cursor_style(style)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        self.with_common(|common| common.auto_hide_scrollbars)
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

    fn window_appearance(&self) -> WindowAppearance {
        self.with_common(|common| common.appearance)
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }

    fn write_to_primary(&self, item: ClipboardItem) {
        self.write_to_primary(item)
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        self.write_to_clipboard(item)
    }

    fn read_from_primary(&self) -> Option<ClipboardItem> {
        self.read_from_primary()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.read_from_clipboard()
    }

    fn add_recent_document(&self, _path: &Path) {}
}

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(super) fn open_uri_internal(
    executor: BackgroundExecutor,
    uri: &str,
    activation_token: Option<String>,
) {
    if let Some(uri) = ashpd::url::Url::parse(uri).log_err() {
        executor
            .spawn(async move {
                match ashpd::desktop::open_uri::OpenFileRequest::default()
                    .activation_token(activation_token.clone().map(ashpd::ActivationToken::from))
                    .send_uri(&uri)
                    .await
                {
                    Ok(_) => return,
                    Err(e) => log::error!("Failed to open with dbus: {}", e),
                }

                for mut command in open::commands(uri.to_string()) {
                    if let Some(token) = activation_token.as_ref() {
                        command.env("XDG_ACTIVATION_TOKEN", token);
                    }
                    match command.spawn() {
                        Ok(_) => return,
                        Err(e) => {
                            log::error!("Failed to open with {:?}: {}", command.get_program(), e)
                        }
                    }
                }
            })
            .detach();
    }
}

#[cfg(any(feature = "x11", feature = "wayland"))]
pub(super) fn reveal_path_internal(
    executor: BackgroundExecutor,
    path: PathBuf,
    activation_token: Option<String>,
) {
    executor
        .spawn(async move {
            if let Some(dir) = File::open(path.clone()).log_err() {
                match ashpd::desktop::open_uri::OpenDirectoryRequest::default()
                    .activation_token(activation_token.map(ashpd::ActivationToken::from))
                    .send(&dir.as_fd())
                    .await
                {
                    Ok(_) => return,
                    Err(e) => log::error!("Failed to open with dbus: {}", e),
                }
                if path.is_dir() {
                    open::that_detached(path).log_err();
                } else {
                    open::that_detached(path.parent().unwrap_or(Path::new(""))).log_err();
                }
            }
        })
        .detach();
}

#[allow(unused)]
pub(super) fn is_within_click_distance(a: Point<Pixels>, b: Point<Pixels>) -> bool {
    let diff = a - b;
    diff.x.abs() <= DOUBLE_CLICK_DISTANCE && diff.y.abs() <= DOUBLE_CLICK_DISTANCE
}

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(super) fn get_xkb_compose_state(cx: &xkb::Context) -> Option<xkb::compose::State> {
    let mut locales = Vec::default();
    if let Some(locale) = env::var_os("LC_CTYPE") {
        locales.push(locale);
    }
    locales.push(OsString::from("C"));
    let mut state: Option<xkb::compose::State> = None;
    for locale in locales {
        if let Ok(table) =
            xkb::compose::Table::new_from_locale(&cx, &locale, xkb::compose::COMPILE_NO_FLAGS)
        {
            state = Some(xkb::compose::State::new(
                &table,
                xkb::compose::STATE_NO_FLAGS,
            ));
            break;
        }
    }
    state
}

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(super) unsafe fn read_fd(mut fd: filedescriptor::FileDescriptor) -> Result<Vec<u8>> {
    let mut file = unsafe { File::from_raw_fd(fd.as_raw_fd()) };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

impl CursorStyle {
    #[allow(unused)]
    pub(super) fn to_icon_name(&self) -> String {
        // Based on cursor names from https://gitlab.gnome.org/GNOME/adwaita-icon-theme (GNOME)
        // and https://github.com/KDE/breeze (KDE). Both of them seem to be also derived from
        // Web CSS cursor names: https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#values
        match self {
            CursorStyle::Arrow => "left_ptr",
            CursorStyle::IBeam => "text",
            CursorStyle::Crosshair => "crosshair",
            CursorStyle::ClosedHand => "grabbing",
            CursorStyle::OpenHand => "grab",
            CursorStyle::PointingHand => "pointer",
            CursorStyle::ResizeLeft => "w-resize",
            CursorStyle::ResizeRight => "e-resize",
            CursorStyle::ResizeLeftRight => "ew-resize",
            CursorStyle::ResizeUp => "n-resize",
            CursorStyle::ResizeDown => "s-resize",
            CursorStyle::ResizeUpDown => "ns-resize",
            CursorStyle::ResizeUpLeftDownRight => "nwse-resize",
            CursorStyle::ResizeUpRightDownLeft => "nesw-resize",
            CursorStyle::ResizeColumn => "col-resize",
            CursorStyle::ResizeRow => "row-resize",
            CursorStyle::IBeamCursorForVerticalLayout => "vertical-text",
            CursorStyle::OperationNotAllowed => "not-allowed",
            CursorStyle::DragLink => "alias",
            CursorStyle::DragCopy => "copy",
            CursorStyle::ContextualMenu => "context-menu",
            CursorStyle::None => {
                #[cfg(debug_assertions)]
                panic!("CursorStyle::None should be handled separately in the client");
                #[cfg(not(debug_assertions))]
                "default"
            }
        }
        .to_string()
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl crate::Keystroke {
    pub(super) fn from_xkb(
        state: &State,
        mut modifiers: crate::Modifiers,
        keycode: Keycode,
    ) -> Self {
        let key_utf32 = state.key_get_utf32(keycode);
        let key_utf8 = state.key_get_utf8(keycode);
        let key_sym = state.key_get_one_sym(keycode);

        let key = match key_sym {
            Keysym::Return => "enter".to_owned(),
            Keysym::Prior => "pageup".to_owned(),
            Keysym::Next => "pagedown".to_owned(),
            Keysym::ISO_Left_Tab => "tab".to_owned(),
            Keysym::KP_Prior => "pageup".to_owned(),
            Keysym::KP_Next => "pagedown".to_owned(),
            Keysym::XF86_Back => "back".to_owned(),
            Keysym::XF86_Forward => "forward".to_owned(),
            Keysym::XF86_Cut => "cut".to_owned(),
            Keysym::XF86_Copy => "copy".to_owned(),
            Keysym::XF86_Paste => "paste".to_owned(),
            Keysym::XF86_New => "new".to_owned(),
            Keysym::XF86_Open => "open".to_owned(),
            Keysym::XF86_Save => "save".to_owned(),

            Keysym::comma => ",".to_owned(),
            Keysym::period => ".".to_owned(),
            Keysym::less => "<".to_owned(),
            Keysym::greater => ">".to_owned(),
            Keysym::slash => "/".to_owned(),
            Keysym::question => "?".to_owned(),

            Keysym::semicolon => ";".to_owned(),
            Keysym::colon => ":".to_owned(),
            Keysym::apostrophe => "'".to_owned(),
            Keysym::quotedbl => "\"".to_owned(),

            Keysym::bracketleft => "[".to_owned(),
            Keysym::braceleft => "{".to_owned(),
            Keysym::bracketright => "]".to_owned(),
            Keysym::braceright => "}".to_owned(),
            Keysym::backslash => "\\".to_owned(),
            Keysym::bar => "|".to_owned(),

            Keysym::grave => "`".to_owned(),
            Keysym::asciitilde => "~".to_owned(),
            Keysym::exclam => "!".to_owned(),
            Keysym::at => "@".to_owned(),
            Keysym::numbersign => "#".to_owned(),
            Keysym::dollar => "$".to_owned(),
            Keysym::percent => "%".to_owned(),
            Keysym::asciicircum => "^".to_owned(),
            Keysym::ampersand => "&".to_owned(),
            Keysym::asterisk => "*".to_owned(),
            Keysym::parenleft => "(".to_owned(),
            Keysym::parenright => ")".to_owned(),
            Keysym::minus => "-".to_owned(),
            Keysym::underscore => "_".to_owned(),
            Keysym::equal => "=".to_owned(),
            Keysym::plus => "+".to_owned(),

            _ => {
                let name = xkb::keysym_get_name(key_sym).to_lowercase();
                if key_sym.is_keypad_key() {
                    name.replace("kp_", "")
                } else {
                    name
                }
            }
        };

        if modifiers.shift {
            // we only include the shift for upper-case letters by convention,
            // so don't include for numbers and symbols, but do include for
            // tab/enter, etc.
            if key.chars().count() == 1 && key.to_lowercase() == key.to_uppercase() {
                modifiers.shift = false;
            }
        }

        // Ignore control characters (and DEL) for the purposes of key_char
        let key_char =
            (key_utf32 >= 32 && key_utf32 != 127 && !key_utf8.is_empty()).then_some(key_utf8);

        Self {
            modifiers,
            key,
            key_char,
        }
    }

    /**
     * Returns which symbol the dead key represents
     * <https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values#dead_keycodes_for_linux>
     */
    pub fn underlying_dead_key(keysym: Keysym) -> Option<String> {
        match keysym {
            Keysym::dead_grave => Some("`".to_owned()),
            Keysym::dead_acute => Some("´".to_owned()),
            Keysym::dead_circumflex => Some("^".to_owned()),
            Keysym::dead_tilde => Some("~".to_owned()),
            Keysym::dead_macron => Some("¯".to_owned()),
            Keysym::dead_breve => Some("˘".to_owned()),
            Keysym::dead_abovedot => Some("˙".to_owned()),
            Keysym::dead_diaeresis => Some("¨".to_owned()),
            Keysym::dead_abovering => Some("˚".to_owned()),
            Keysym::dead_doubleacute => Some("˝".to_owned()),
            Keysym::dead_caron => Some("ˇ".to_owned()),
            Keysym::dead_cedilla => Some("¸".to_owned()),
            Keysym::dead_ogonek => Some("˛".to_owned()),
            Keysym::dead_iota => Some("ͅ".to_owned()),
            Keysym::dead_voiced_sound => Some("゙".to_owned()),
            Keysym::dead_semivoiced_sound => Some("゚".to_owned()),
            Keysym::dead_belowdot => Some("̣̣".to_owned()),
            Keysym::dead_hook => Some("̡".to_owned()),
            Keysym::dead_horn => Some("̛".to_owned()),
            Keysym::dead_stroke => Some("̶̶".to_owned()),
            Keysym::dead_abovecomma => Some("̓̓".to_owned()),
            Keysym::dead_abovereversedcomma => Some("ʽ".to_owned()),
            Keysym::dead_doublegrave => Some("̏".to_owned()),
            Keysym::dead_belowring => Some("˳".to_owned()),
            Keysym::dead_belowmacron => Some("̱".to_owned()),
            Keysym::dead_belowcircumflex => Some("ꞈ".to_owned()),
            Keysym::dead_belowtilde => Some("̰".to_owned()),
            Keysym::dead_belowbreve => Some("̮".to_owned()),
            Keysym::dead_belowdiaeresis => Some("̤".to_owned()),
            Keysym::dead_invertedbreve => Some("̯".to_owned()),
            Keysym::dead_belowcomma => Some("̦".to_owned()),
            Keysym::dead_currency => None,
            Keysym::dead_lowline => None,
            Keysym::dead_aboveverticalline => None,
            Keysym::dead_belowverticalline => None,
            Keysym::dead_longsolidusoverlay => None,
            Keysym::dead_a => None,
            Keysym::dead_A => None,
            Keysym::dead_e => None,
            Keysym::dead_E => None,
            Keysym::dead_i => None,
            Keysym::dead_I => None,
            Keysym::dead_o => None,
            Keysym::dead_O => None,
            Keysym::dead_u => None,
            Keysym::dead_U => None,
            Keysym::dead_small_schwa => Some("ə".to_owned()),
            Keysym::dead_capital_schwa => Some("Ə".to_owned()),
            Keysym::dead_greek => None,
            _ => None,
        }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl crate::Modifiers {
    pub(super) fn from_xkb(keymap_state: &State) -> Self {
        let shift = keymap_state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE);
        let alt = keymap_state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE);
        let control =
            keymap_state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
        let platform =
            keymap_state.mod_name_is_active(xkb::MOD_NAME_LOGO, xkb::STATE_MODS_EFFECTIVE);
        Self {
            shift,
            alt,
            control,
            platform,
            function: false,
            capslock: false, // TODO:
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, px};

    #[test]
    fn test_is_within_click_distance() {
        let zero = Point::new(px(0.0), px(0.0));
        assert_eq!(
            is_within_click_distance(zero, Point::new(px(5.0), px(5.0))),
            true
        );
        assert_eq!(
            is_within_click_distance(zero, Point::new(px(-4.9), px(5.0))),
            true
        );
        assert_eq!(
            is_within_click_distance(Point::new(px(3.0), px(2.0)), Point::new(px(-2.0), px(-2.0))),
            true
        );
        assert_eq!(
            is_within_click_distance(zero, Point::new(px(5.0), px(5.1))),
            false
        );
    }
}
