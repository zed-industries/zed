#![allow(unused)]

use std::any::{type_name, Any};
use std::cell::{self, RefCell};
use std::env;
use std::ops::{Deref, DerefMut};
use std::panic::Location;
use std::{
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::anyhow;
use ashpd::desktop::file_chooser::{OpenFileRequest, SaveFileRequest};
use async_task::Runnable;
use calloop::channel::Channel;
use calloop::{EventLoop, LoopHandle, LoopSignal};
use copypasta::ClipboardProvider;
use flume::{Receiver, Sender};
use futures::channel::oneshot;
use parking_lot::Mutex;
use time::UtcOffset;
use wayland_client::Connection;
use xkbcommon::xkb::{self, Keycode, Keysym, State};

use crate::platform::linux::wayland::WaylandClient;
use crate::{
    px, Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CosmicTextSystem, CursorStyle,
    DisplayId, ForegroundExecutor, Keymap, Keystroke, LinuxDispatcher, Menu, Modifiers,
    PathPromptOptions, Pixels, Platform, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformTextSystem, PlatformWindow, Point, PromptLevel, Result, SemanticVersion, Size, Task,
    WindowAppearance, WindowOptions, WindowParams,
};

use super::x11::X11Client;

pub(crate) const SCROLL_LINES: f64 = 3.0;

// Values match the defaults on GTK.
// Taken from https://github.com/GNOME/gtk/blob/main/gtk/gtksettings.c#L320
pub(crate) const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(400);
pub(crate) const DOUBLE_CLICK_DISTANCE: Pixels = px(5.0);
pub(crate) const KEYRING_LABEL: &str = "zed-github-account";

pub trait LinuxClient {
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R;
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>;
    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Box<dyn PlatformWindow>;
    fn set_cursor_style(&self, style: CursorStyle);
    fn write_to_clipboard(&self, item: ClipboardItem);
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;
    fn run(&self);
}

#[derive(Default)]
pub(crate) struct PlatformHandlers {
    pub(crate) open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    pub(crate) become_active: Option<Box<dyn FnMut()>>,
    pub(crate) resign_active: Option<Box<dyn FnMut()>>,
    pub(crate) quit: Option<Box<dyn FnMut()>>,
    pub(crate) reopen: Option<Box<dyn FnMut()>>,
    pub(crate) event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    pub(crate) app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    pub(crate) will_open_app_menu: Option<Box<dyn FnMut()>>,
    pub(crate) validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

pub(crate) struct LinuxCommon {
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) text_system: Arc<CosmicTextSystem>,
    pub(crate) callbacks: PlatformHandlers,
    pub(crate) signal: LoopSignal,
}

impl LinuxCommon {
    pub fn new(signal: LoopSignal) -> (Self, Channel<Runnable>) {
        let (main_sender, main_receiver) = calloop::channel::channel::<Runnable>();
        let text_system = Arc::new(CosmicTextSystem::new());
        let callbacks = PlatformHandlers::default();

        let dispatcher = Arc::new(LinuxDispatcher::new(main_sender));

        let common = LinuxCommon {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher.clone()),
            text_system,
            callbacks,
            signal,
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

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();

        LinuxClient::run(self);

        self.with_common(|common| {
            if let Some(mut fun) = common.callbacks.quit.take() {
                fun();
            }
        });
    }

    fn quit(&self) {
        self.with_common(|common| common.signal.stop());
    }

    fn restart(&self) {
        use std::os::unix::process::CommandExt as _;

        // get the process id of the current process
        let app_pid = std::process::id().to_string();
        // get the path to the executable
        let app_path = match self.app_path() {
            Ok(path) => path,
            Err(err) => {
                log::error!("Failed to get app path: {:?}", err);
                return;
            }
        };

        // script to wait for the current process to exit  and then restart the app
        let script = format!(
            r#"
            while kill -O {pid} 2>/dev/null; do
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

    // todo(linux)
    fn activate(&self, ignoring_other_apps: bool) {}

    // todo(linux)
    fn hide(&self) {}

    fn hide_other_apps(&self) {
        log::warn!("hide_other_apps is not implemented on Linux, ignoring the call")
    }

    // todo(linux)
    fn unhide_other_apps(&self) {}

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.primary_display()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.displays()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        self.display(id)
    }

    // todo(linux)
    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        self.open_window(handle, options)
    }

    fn open_url(&self, url: &str) {
        open::that(url);
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.with_common(|common| common.callbacks.open_urls = Some(callback));
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
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
        self.foreground_executor()
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
        self.with_common(|common| {
            common.callbacks.become_active = Some(callback);
        });
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.with_common(|common| {
            common.callbacks.resign_active = Some(callback);
        });
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

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.with_common(|common| {
            common.callbacks.event = Some(callback);
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

    fn os_name(&self) -> &'static str {
        "Linux"
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion::new(1, 0, 0))
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion::new(1, 0, 0))
    }

    fn app_path(&self) -> Result<PathBuf> {
        // get the path of the executable of the current process
        let exe_path = std::env::current_exe()?;
        Ok(exe_path)
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
        self.set_cursor_style(style)
    }

    // todo(linux)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
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

    fn write_to_clipboard(&self, item: ClipboardItem) {
        self.write_to_clipboard(item)
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.read_from_clipboard()
    }
}

pub(super) fn is_within_click_distance(a: Point<Pixels>, b: Point<Pixels>) -> bool {
    let diff = a - b;
    diff.x.abs() <= DOUBLE_CLICK_DISTANCE && diff.y.abs() <= DOUBLE_CLICK_DISTANCE
}

impl Keystroke {
    pub(super) fn from_xkb(state: &State, modifiers: Modifiers, keycode: Keycode) -> Self {
        let mut modifiers = modifiers;

        let key_utf32 = state.key_get_utf32(keycode);
        let key_utf8 = state.key_get_utf8(keycode);
        let key_sym = state.key_get_one_sym(keycode);

        // The logic here tries to replicate the logic in `../mac/events.rs`
        // "Consumed" modifiers are modifiers that have been used to translate a key, for example
        // pressing "shift" and "1" on US layout produces the key `!` but "consumes" the shift.
        // Notes:
        //  - macOS gets the key character directly ("."), xkb gives us the key name ("period")
        //  - macOS logic removes consumed shift modifier for symbols: "{", not "shift-{"
        //  - macOS logic keeps consumed shift modifiers for letters: "shift-a", not "a" or "A"

        let mut handle_consumed_modifiers = true;
        let key = match key_sym {
            Keysym::Return => "enter".to_owned(),
            Keysym::Prior => "pageup".to_owned(),
            Keysym::Next => "pagedown".to_owned(),

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

            Keysym::ISO_Left_Tab => {
                handle_consumed_modifiers = false;
                "tab".to_owned()
            }

            _ => {
                handle_consumed_modifiers = false;
                xkb::keysym_get_name(key_sym).to_lowercase()
            }
        };

        // Ignore control characters (and DEL) for the purposes of ime_key,
        // but if key_utf32 is 0 then assume it isn't one
        let ime_key = ((key_utf32 == 0 || (key_utf32 >= 32 && key_utf32 != 127))
            && !key_utf8.is_empty())
        .then_some(key_utf8);

        if handle_consumed_modifiers {
            let mod_shift_index = state.get_keymap().mod_get_index(xkb::MOD_NAME_SHIFT);
            let is_shift_consumed = state.mod_index_is_consumed(keycode, mod_shift_index);

            if modifiers.shift && is_shift_consumed {
                modifiers.shift = false;
            }
        }

        Keystroke {
            modifiers,
            key,
            ime_key,
        }
    }
}

impl Modifiers {
    pub(super) fn from_xkb(keymap_state: &State) -> Self {
        let shift = keymap_state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE);
        let alt = keymap_state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE);
        let control =
            keymap_state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
        let platform =
            keymap_state.mod_name_is_active(xkb::MOD_NAME_LOGO, xkb::STATE_MODS_EFFECTIVE);
        Modifiers {
            shift,
            alt,
            control,
            platform,
            function: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{px, Point};

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
