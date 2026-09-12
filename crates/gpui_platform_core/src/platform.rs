//! The [`Platform`] trait implemented by each platform backend.
//!
//! `gpui` drives an application through a `dyn Platform`; backends
//! (`gpui_macos`, `gpui_linux`, `gpui_windows`, `gpui_web`) implement it
//! without depending on the framework itself.

use crate::{
    ActivityGuard, AppLifecyclePhase, BackgroundExecutor, ClipboardItem, ClipboardReadError,
    CursorStyle, ForegroundExecutor, MenuCommandId, PathPromptOptions, PlatformDisplay,
    PlatformGestures, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformMenu,
    PlatformMenuItem, PlatformTextSystem, PlatformWindow, ScreenCaptureSource, SystemNotification,
    SystemNotificationResponse, Task, ThermalState, WindowAppearance, WindowButtonLayout, WindowId,
    WindowParams,
};
use anyhow::Result;
use futures::channel::oneshot;
use smallvec::SmallVec;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

#[expect(missing_docs)]
pub trait Platform: 'static {
    fn background_executor(&self) -> BackgroundExecutor;
    fn foreground_executor(&self) -> ForegroundExecutor;
    fn text_system(&self) -> Arc<dyn PlatformTextSystem>;

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>);
    fn quit(&self);
    fn restart(&self, binary_path: Option<PathBuf>, arguments: Vec<OsString>);
    fn activate(&self, ignoring_other_apps: bool);
    fn hide(&self);
    fn hide_other_apps(&self);
    fn unhide_other_apps(&self);

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn active_window(&self) -> Option<WindowId>;
    fn window_stack(&self) -> Option<Vec<WindowId>> {
        None
    }

    fn is_screen_capture_supported(&self) -> bool {
        false
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<anyhow::Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
        let (sources_tx, sources_rx) = oneshot::channel();
        sources_tx
            .send(Err(anyhow::anyhow!(
                "gpui was compiled without the screen-capture feature"
            )))
            .ok();
        sources_rx
    }

    fn open_window(
        &self,
        handle: WindowId,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>>;

    /// Returns the appearance of the application's windows.
    fn window_appearance(&self) -> WindowAppearance;

    /// Overrides the appearance (light/dark) applied to the app's windows, independent
    /// of the OS-wide setting. Pass `None` to clear the override and follow the system
    /// again. The override is reflected by [`Platform::window_appearance`].
    ///
    /// Currently only implemented on macOS, where it sets `NSApplication.appearance` so
    /// the native window chrome (the window border and titlebar) of every window matches
    /// a dark app theme even when the system is in light mode (or vice versa). A no-op on
    /// other platforms.
    fn set_window_appearance(&self, _appearance: Option<WindowAppearance>) {}

    /// Returns the window button layout configuration when supported.
    fn button_layout(&self) -> Option<WindowButtonLayout> {
        None
    }

    fn open_url(&self, url: &str);
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>);
    fn register_url_scheme(&self, url: &str) -> Task<Result<()>>;

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>>;
    fn prompt_for_new_path(
        &self,
        directory: &Path,
        suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>>;
    fn can_select_mixed_files_and_dirs(&self) -> bool;
    fn reveal_path(&self, path: &Path);
    fn open_with_system(&self, path: &Path);

    fn on_quit(&self, callback: Box<dyn FnMut() -> bool>);
    fn on_reopen(&self, callback: Box<dyn FnMut()>);
    fn on_system_wake(&self, callback: Box<dyn FnMut()>);

    // Mobile platform methods. On mobile the OS owns the application
    // lifecycle: apps are backgrounded, foregrounded, and killed at the
    // system's discretion, and must react rather than decide.

    /// Registers a callback invoked whenever the application's lifecycle
    /// phase changes. See [`AppLifecyclePhase`] for the phase vocabulary and
    /// its mapping onto iOS and Android.
    ///
    /// Desktop platforms never invoke this.
    fn on_app_lifecycle(&self, _callback: Box<dyn FnMut(AppLifecyclePhase)>) {}

    /// Registers a callback invoked when the OS signals memory pressure
    /// (iOS `didReceiveMemoryWarning`, Android `onTrimMemory`).
    ///
    /// Desktop platforms never invoke this.
    fn on_memory_warning(&self, _callback: Box<dyn FnMut()>) {}

    /// The platform's gesture recognition services, if it provides any
    /// beyond gpui's portable recognizers. See
    /// [`PlatformGestures`](crate::PlatformGestures).
    fn gestures(&self) -> Option<Rc<dyn PlatformGestures>> {
        None
    }

    fn set_menus(&self, menus: Vec<PlatformMenu>);

    fn set_dock_menu(&self, menu: Vec<PlatformMenuItem>);
    fn perform_dock_menu_action(&self, _action: usize) {}
    fn add_recent_document(&self, _path: &Path) {}
    fn update_jump_list(
        &self,
        _menus: Vec<PlatformMenuItem>,
        _entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Task<Vec<SmallVec<[PathBuf; 2]>>> {
        Task::ready(Vec::new())
    }
    fn on_app_menu_action(&self, callback: Box<dyn FnMut(MenuCommandId)>);
    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>);
    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(MenuCommandId) -> bool>);

    fn thermal_state(&self) -> ThermalState;
    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>);
    fn prevent_idle_sleep(&self, reason: &str) -> Task<Result<ActivityGuard>>;

    /// Sets the application's process-wide identity and user-visible name.
    ///
    /// The identifier is used for platform identity mechanisms such as the
    /// Windows AppUserModelID. The name is used wherever the operating system
    /// presents the application to the user. Call this once, early in startup,
    /// before opening windows or posting notifications.
    fn set_app_identity(&self, identifier: &str, name: &str) {
        _ = (identifier, name);
    }

    /// Posts a notification to the operating system's notification center.
    ///
    /// Posting a notification whose [`SystemNotification::tag`] matches an
    /// earlier one replaces that notification where the platform supports it.
    /// No-op on platforms without notification support, or when delivery is
    /// unavailable (e.g. authorization was denied).
    fn show_system_notification(&self, notification: SystemNotification) {
        _ = notification;
    }

    /// Removes the delivered or pending notification with this tag.
    ///
    /// Best-effort: some platforms cannot retract a notification once shown,
    /// in which case it ages out of the notification center on its own.
    fn dismiss_system_notification(&self, tag: &str) {
        _ = tag;
    }

    /// Registers the callback invoked when the user activates a system
    /// notification, either by clicking its body or one of its action
    /// buttons.
    ///
    /// Implementations must invoke the callback on the main thread.
    fn on_system_notification_response(
        &self,
        callback: Box<dyn FnMut(SystemNotificationResponse)>,
    ) {
        _ = callback;
    }

    fn compositor_name(&self) -> &'static str {
        ""
    }
    fn app_path(&self) -> Result<PathBuf>;
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>;

    fn set_cursor_style(&self, style: CursorStyle);

    /// Hides the mouse cursor until the user moves the mouse over one of
    /// this application's windows.
    fn hide_cursor_until_mouse_moves(&self);

    /// Returns whether the mouse cursor is currently visible.
    fn is_cursor_visible(&self) -> bool;

    fn should_auto_hide_scrollbars(&self) -> bool;

    fn read_from_clipboard(&self) -> Option<ClipboardItem>;
    fn write_to_clipboard(&self, item: ClipboardItem);

    /// Reads the clipboard, resolving once its contents are available.
    ///
    /// Most platforms read synchronously and return a ready task. Platforms
    /// whose clipboard access is inherently asynchronous and permission-gated
    /// (e.g. the browser's async clipboard API) override this method; on those
    /// platforms [`Platform::read_from_clipboard`] cannot return the clipboard
    /// contents, so callers that can await should prefer this method.
    fn read_from_clipboard_async(&self) -> Task<Result<Option<ClipboardItem>, ClipboardReadError>> {
        Task::ready(Ok(self.read_from_clipboard()))
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn read_from_primary(&self) -> Option<ClipboardItem>;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn write_to_primary(&self, item: ClipboardItem);

    #[cfg(target_os = "macos")]
    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem>;
    #[cfg(target_os = "macos")]
    fn write_to_find_pasteboard(&self, item: ClipboardItem);

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>>;
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>;
    fn delete_credentials(&self, url: &str) -> Task<Result<()>>;

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout>;
    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper>;
    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>);
}
