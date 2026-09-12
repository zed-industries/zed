mod app_menu;

#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
mod threaded_dispatcher;

#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
mod test;

#[cfg(all(target_os = "macos", any(test, feature = "test-support")))]
mod visual_test;

#[cfg(all(
    feature = "screen-capture",
    any(target_os = "windows", target_os = "linux", target_os = "freebsd",)
))]
pub mod scap_screen_capture;

#[cfg(all(
    any(target_os = "windows", target_os = "linux"),
    feature = "screen-capture"
))]
pub(crate) type PlatformScreenCaptureFrame = scap::frame::Frame;
#[cfg(not(feature = "screen-capture"))]
pub(crate) type PlatformScreenCaptureFrame = ();
#[cfg(all(target_os = "macos", feature = "screen-capture"))]
pub(crate) type PlatformScreenCaptureFrame = core_video::image_buffer::CVImageBuffer;

#[cfg(all(target_os = "linux", feature = "wayland"))]
use crate::layer_shell;
use crate::{
    Action, ActivityGuard, App, AppLifecyclePhase, AsyncWindowContext, BackgroundExecutor, Bounds,
    BoundsExt, Capslock, ClipboardItem, ClipboardReadError, CursorStyle, Decorations,
    DispatchEventResult, ExternalDragPayload, ForegroundExecutor, GpuSpecs, Image, ImageFormat,
    ImageSource, Keymap, Modifiers, PathPromptOptions, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformGestures, PlatformInput, PlatformKeyboardLayout, PlatformKeyboardMapper,
    PlatformTextSystem, Point, PromptButton, PromptLevel, RenderImage, RequestFrameOptions,
    ResizeEdge, Scene, Size, SourceMetadata, SvgRenderer, SystemNotification,
    SystemNotificationResponse, SystemWindowTab, Task, TextInputConfiguration,
    TextInputStateChange, ThermalState, Window, WindowAppearance, WindowBackgroundAppearance,
    WindowBounds, WindowButtonLayout, WindowControlArea, WindowControls, WindowDecorations,
    WindowId, WindowInsets, WindowParams, px,
};
use anyhow::{Context as _, Result};
use futures::channel::oneshot;
#[cfg(any(test, feature = "test-support"))]
use image::RgbaImage;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder as _, DynamicImage, Frame};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
pub use scheduler::RunnableMeta;
use smallvec::SmallVec;
use std::io::Cursor;
use std::{
    ffi::OsString,
    fmt::Debug,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub use app_menu::*;

#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
pub(crate) use test::*;

#[cfg(any(test, feature = "test-support"))]
pub use test::{TestScreenCaptureSource, TestScreenCaptureStream};

#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
pub use threaded_dispatcher::{PlatformDispatcherExt, ThreadedDispatcher};

#[cfg(all(target_os = "macos", any(test, feature = "test-support")))]
pub use visual_test::VisualTestPlatform;

// TODO(jk): return an enum instead of a string
/// Return which compositor we're guessing we'll use.
/// Does not attempt to connect to the given compositor.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[inline]
pub fn guess_compositor() -> &'static str {
    if std::env::var_os("ZED_HEADLESS").is_some() {
        return "Headless";
    }

    #[cfg(feature = "wayland")]
    let wayland_display = std::env::var_os("WAYLAND_DISPLAY");
    #[cfg(not(feature = "wayland"))]
    let wayland_display: Option<std::ffi::OsString> = None;

    #[cfg(feature = "x11")]
    let x11_display = std::env::var_os("DISPLAY");
    #[cfg(not(feature = "x11"))]
    let x11_display: Option<std::ffi::OsString> = None;

    let use_wayland = wayland_display.is_some_and(|display| !display.is_empty());
    let use_x11 = x11_display.is_some_and(|display| !display.is_empty());

    if use_wayland {
        "Wayland"
    } else if use_x11 {
        "X11"
    } else {
        "Headless"
    }
}

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

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap);
    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        None
    }

    fn set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap);
    fn perform_dock_menu_action(&self, _action: usize) {}
    fn add_recent_document(&self, _path: &Path) {}
    fn update_jump_list(
        &self,
        _menus: Vec<MenuItem>,
        _entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Task<Vec<SmallVec<[PathBuf; 2]>>> {
        Task::ready(Vec::new())
    }
    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>);
    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>);
    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>);

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

/// A source of on-screen video content that can be captured.
pub trait ScreenCaptureSource {
    /// Returns metadata for this source.
    fn metadata(&self) -> Result<SourceMetadata>;

    /// Start capture video from this source, invoking the given callback
    /// with each frame.
    fn stream(
        &self,
        foreground_executor: &ForegroundExecutor,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>>;
}

/// A video stream captured from a screen.
pub trait ScreenCaptureStream {
    /// Returns metadata for this source.
    fn metadata(&self) -> Result<SourceMetadata>;
}

/// A frame of video captured from a screen.
pub struct ScreenCaptureFrame(pub PlatformScreenCaptureFrame);

/// Callbacks for the accessibility adapter.
pub struct A11yCallbacks {
    /// Called when the adapter is activated (a screen reader connects).
    pub activation: Box<dyn Fn() -> Option<accesskit::TreeUpdate> + Send + 'static>,
    /// Called when an action is requested by the screen reader.
    pub action: Box<dyn Fn(accesskit::ActionRequest) + Send + 'static>,
    /// Called when the adapter is deactivated (screen reader disconnects).
    pub deactivation: Box<dyn Fn() + Send + 'static>,
}

#[expect(missing_docs)]
pub trait PlatformWindow: HasWindowHandle + HasDisplayHandle {
    fn bounds(&self) -> Bounds<Pixels>;
    fn is_maximized(&self) -> bool;
    fn window_bounds(&self) -> WindowBounds;
    fn content_size(&self) -> Size<Pixels>;
    /// Returns the visible viewport in logical pixels relative to the content origin.
    ///
    /// This may be smaller or offset when a keyboard or zoom obscures content;
    /// it must not change the full layout size returned by `content_size`.
    /// Implementations should return a frame snapshot, not query platform layout here.
    fn visual_viewport_bounds(&self) -> Bounds<Pixels> {
        Bounds::new(Point::default(), self.content_size())
    }
    /// Registers a callback when visible geometry may have changed.
    ///
    /// This requests a frame; backends can sample the new viewport and safe-area
    /// geometry in `prepare_frame` rather than updating it inside the callback.
    fn on_visual_viewport_changed(&self, _callback: Box<dyn FnMut()>) {}
    /// Samples platform geometry before a draw, returning whether view caches must be invalidated.
    ///
    /// Geometry getters must remain consistent throughout the ensuing draw.
    /// Do not invoke callbacks here: GPUI is already updating this window.
    fn prepare_frame(&self) -> bool {
        false
    }
    fn resize(&mut self, size: Size<Pixels>);
    fn scale_factor(&self) -> f32;
    fn appearance(&self) -> WindowAppearance;
    fn display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn mouse_position(&self) -> Point<Pixels>;
    fn modifiers(&self) -> Modifiers;
    fn capslock(&self) -> Capslock;
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler);
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler>;
    /// Apply the focused text region's [`TextInputConfiguration`] to the
    /// platform's text input session (e.g. attributes of the hidden editable
    /// element on web). Called only when the configuration changes, because
    /// reconfiguring a live input session can restart the IME connection.
    fn set_text_input_configuration(&mut self, _configuration: TextInputConfiguration) {}
    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>>;
    fn activate(&self);
    /// Requests that the operating system draw attention to this window.
    fn request_attention(&self) {}
    fn is_active(&self) -> bool;
    fn is_hovered(&self) -> bool;
    fn background_appearance(&self) -> WindowBackgroundAppearance;
    fn set_title(&mut self, title: &str);
    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance);
    fn minimize(&self);
    fn zoom(&self);
    fn toggle_fullscreen(&self);
    fn is_fullscreen(&self) -> bool;
    fn frame_waker(&self) -> Option<Rc<dyn Fn()>> {
        None
    }
    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>);
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>);
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>);
    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>);
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>);
    fn on_moved(&self, callback: Box<dyn FnMut()>);
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>);
    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>);
    fn on_close(&self, callback: Box<dyn FnOnce()>);
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>);
    fn on_button_layout_changed(&self, _callback: Box<dyn FnMut()>) {}
    fn draw(&self, scene: &Scene);
    fn schedule_frame(&self) {}
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;
    fn is_subpixel_rendering_supported(&self) -> bool;

    // macOS specific methods
    fn get_title(&self) -> String {
        String::new()
    }
    fn tabbed_windows(&self) -> Option<Vec<SystemWindowTab>> {
        None
    }
    fn tab_bar_visible(&self) -> bool {
        false
    }
    fn set_edited(&mut self, _edited: bool) {}
    fn set_document_path(&self, _path: Option<&std::path::Path>) {}
    fn toggle_simple_fullscreen(&self) {}
    fn is_simple_fullscreen(&self) -> bool {
        false
    }
    #[cfg(target_os = "macos")]
    fn set_traffic_light_position(&self, _position: Point<Pixels>) {}
    fn show_character_palette(&self) {}
    fn titlebar_double_click(&self, _is_resizable: bool, _is_minimizable: bool) {}
    fn on_move_tab_to_new_window(&self, _callback: Box<dyn FnMut()>) {}
    fn on_merge_all_windows(&self, _callback: Box<dyn FnMut()>) {}
    fn on_select_previous_tab(&self, _callback: Box<dyn FnMut()>) {}
    fn on_select_next_tab(&self, _callback: Box<dyn FnMut()>) {}
    fn on_toggle_tab_bar(&self, _callback: Box<dyn FnMut()>) {}
    fn merge_all_windows(&self) {}
    fn move_tab_to_new_window(&self) {}
    fn toggle_window_tab_overview(&self) {}
    fn set_tabbing_identifier(&self, _identifier: Option<String>) {}

    fn native_window_state(&self) -> Option<Vec<u8>> {
        None
    }
    fn restore_native_window_state(&self, _state: &[u8]) {}

    #[cfg(target_os = "windows")]
    fn get_raw_handle(&self) -> windows::Win32::Foundation::HWND;

    // Linux specific methods
    fn inner_window_bounds(&self) -> WindowBounds {
        self.window_bounds()
    }
    fn request_decorations(&self, _decorations: WindowDecorations) {}
    fn show_window_menu(&self, _position: Point<Pixels>) {}
    fn start_window_move(&self) {}
    fn can_start_external_drag(&self) -> bool {
        false
    }
    fn start_external_drag(&self, _payload: &ExternalDragPayload) -> bool {
        false
    }
    fn start_window_resize(&self, _edge: ResizeEdge) {}
    fn set_exclusive_zone(&self, _zone: Pixels) {}
    #[cfg(all(target_os = "linux", feature = "wayland"))]
    fn set_exclusive_edge(&self, _edge: layer_shell::Anchor) {}
    fn set_input_region(&self, _region: Option<&[Bounds<Pixels>]>) {}
    fn window_decorations(&self) -> Decorations {
        Decorations::Server
    }
    fn set_app_id(&mut self, _app_id: &str) {}
    fn map_window(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn window_controls(&self) -> WindowControls {
        WindowControls::default()
    }
    fn set_client_inset(&self, _inset: Pixels) {}
    fn gpu_specs(&self) -> Option<GpuSpecs>;

    fn update_ime_position(&self, _bounds: Bounds<Pixels>);

    // Mobile platform methods.

    /// The regions of this window currently obscured or reserved by the
    /// system. Zero on platforms without such regions.
    fn insets(&self) -> WindowInsets {
        WindowInsets::default()
    }

    /// Registers a callback invoked whenever [`Self::insets`] change.
    ///
    /// Contract: fires continuously during animated transitions (Android
    /// `WindowInsetsAnimation` progress; on iOS the platform interpolates
    /// the keyboard animation curve on frame ticks) and is exact at rest.
    fn on_insets_changed(&self, _callback: Box<dyn FnMut(WindowInsets)>) {}

    /// Sets the handler for the system back action (Android back
    /// button/gesture; no source on iOS or desktop).
    fn set_back_handler(&self, _callback: Box<dyn FnMut()>) {}

    /// Declares whether the application would currently handle the system
    /// back action (e.g. navigation depth > 0).
    fn set_back_enabled(&self, _enabled: bool) {}

    /// Requests that the soft keyboard be shown.
    fn show_soft_keyboard(&self) {}

    /// Requests that the soft keyboard be hidden.
    fn hide_soft_keyboard(&self) {}

    /// Inform the operating system that the text input state has changed
    fn text_input_state_changed(&self, _change: TextInputStateChange) {}

    fn play_system_bell(&self) {}

    /// Initialize the accessibility adapter with callbacks.
    fn a11y_init(&self, _callbacks: A11yCallbacks) {}

    /// Provide a TreeUpdate to the accessibility adapter.
    fn a11y_tree_update(&self, _tree_update: accesskit::TreeUpdate) {}

    /// Inform the adapter of updated window bounds.
    fn a11y_update_window_bounds(&self) {}

    #[cfg(any(test, feature = "test-support", feature = "bench-support"))]
    fn as_test(&mut self) -> Option<&mut TestWindow> {
        None
    }

    /// Renders the given scene to a texture and returns the pixel data as an RGBA image.
    /// This does not present the frame to screen - useful for visual testing where we want
    /// to capture what would be rendered without displaying it or requiring the window to be visible.
    #[cfg(any(test, feature = "test-support"))]
    fn render_to_image(&self, _scene: &Scene) -> Result<RgbaImage> {
        anyhow::bail!("render_to_image not implemented for this platform")
    }
}

#[doc(hidden)]
pub enum TasksIncluded {
    OnlyCompleted,
    CompletedAndRunning,
}

#[expect(missing_docs)]
pub struct PlatformInputHandler {
    cx: AsyncWindowContext,
    handler: Box<dyn InputHandler>,
}

#[expect(missing_docs)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
impl PlatformInputHandler {
    pub fn new(cx: AsyncWindowContext, handler: Box<dyn InputHandler>) -> Self {
        Self { cx, handler }
    }

    pub fn selected_text_range(&mut self, ignore_disabled_input: bool) -> Option<UTF16Selection> {
        self.cx
            .update(|window, cx| {
                self.handler
                    .selected_text_range(ignore_disabled_input, window, cx)
            })
            .ok()
            .flatten()
    }

    #[cfg_attr(target_os = "windows", allow(dead_code))]
    pub fn marked_text_range(&mut self) -> Option<Range<usize>> {
        self.cx
            .update(|window, cx| self.handler.marked_text_range(window, cx))
            .ok()
            .flatten()
    }

    #[cfg_attr(
        any(target_os = "linux", target_os = "freebsd", target_os = "windows"),
        allow(dead_code)
    )]
    pub fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
    ) -> Option<String> {
        self.cx
            .update(|window, cx| {
                self.handler
                    .text_for_range(range_utf16, adjusted, window, cx)
            })
            .ok()
            .flatten()
    }

    pub fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.cx
            .update(|window, cx| {
                self.handler
                    .replace_text_in_range(replacement_range, text, window, cx);
            })
            .ok();
    }

    pub fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.cx
            .update(|window, cx| {
                self.handler.replace_and_mark_text_in_range(
                    range_utf16,
                    new_text,
                    new_selected_range,
                    window,
                    cx,
                )
            })
            .ok();
    }

    #[cfg_attr(target_os = "windows", allow(dead_code))]
    pub fn unmark_text(&mut self) {
        self.cx
            .update(|window, cx| self.handler.unmark_text(window, cx))
            .ok();
    }

    pub fn paste(&mut self, item: ClipboardItem) {
        self.cx
            .update(|window, cx| self.handler.paste(item, window, cx))
            .ok();
    }

    pub fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>> {
        self.cx
            .update(|window, cx| self.handler.bounds_for_range(range_utf16, window, cx))
            .ok()
            .flatten()
    }

    #[allow(dead_code)]
    pub fn apple_press_and_hold_enabled(&mut self) -> bool {
        self.handler.apple_press_and_hold_enabled()
    }

    pub fn dispatch_input(&mut self, input: &str, window: &mut Window, cx: &mut App) {
        self.handler.replace_text_in_range(None, input, window, cx);
    }

    pub fn compute_ime_candidate_bounds(
        marked_range: Option<Range<usize>>,
        selection: &UTF16Selection,
        mut bounds_for_range: impl FnMut(Range<usize>) -> Option<Bounds<Pixels>>,
    ) -> Option<Bounds<Pixels>> {
        if let Some(marked_range) = marked_range {
            // Default to the start of the marked (composing) range.
            let mut line_start = marked_range.start;

            // Walk backward from the caret looking for a line break. A change in
            // the Y coordinate means we crossed into the previous visual line, so
            // the line start is one position after the break point.
            let caret = selection.range.end;
            if let Some(caret_bounds) = bounds_for_range(caret..caret) {
                for i in (marked_range.start..caret).rev() {
                    if let Some(b) = bounds_for_range(i..i) {
                        if (b.origin.y - caret_bounds.origin.y).abs() > px(0.1) {
                            line_start = i + 1;
                            break;
                        }
                    }
                }
            }
            bounds_for_range(line_start..line_start)
        } else {
            // No active composition — use the selection endpoint.
            let offset = if selection.reversed {
                selection.range.start
            } else {
                selection.range.end
            };
            bounds_for_range(offset..offset)
        }
    }

    pub fn selected_bounds(&mut self, window: &mut Window, cx: &mut App) -> Option<Bounds<Pixels>> {
        let marked_range = self.handler.marked_text_range(window, cx);
        let selection = self.handler.selected_text_range(true, window, cx)?;
        Self::compute_ime_candidate_bounds(marked_range, &selection, |range| {
            self.handler.bounds_for_range(range, window, cx)
        })
    }

    pub fn ime_candidate_bounds(&mut self) -> Option<Bounds<Pixels>> {
        let marked_range = self.marked_text_range();
        let selection = self.selected_text_range(true)?;
        Self::compute_ime_candidate_bounds(marked_range, &selection, |range| {
            self.bounds_for_range(range)
        })
    }

    #[allow(unused)]
    pub fn character_index_for_point(&mut self, point: Point<Pixels>) -> Option<usize> {
        self.cx
            .update(|window, cx| self.handler.character_index_for_point(point, window, cx))
            .ok()
            .flatten()
    }

    /// See [`InputHandler::set_selected_text_range`].
    pub fn set_selected_text_range(&mut self, range_utf16: Range<usize>) {
        self.cx
            .update(|window, cx| {
                self.handler
                    .set_selected_text_range(range_utf16, window, cx)
            })
            .ok();
    }

    /// See [`InputHandler::element_bounds`].
    pub fn element_bounds(&mut self) -> Option<Bounds<Pixels>> {
        self.cx
            .update(|window, cx| self.handler.element_bounds(window, cx))
            .ok()
            .flatten()
    }

    /// See [`InputHandler::text_length_utf16`].
    pub fn text_length_utf16(&mut self) -> Option<usize> {
        self.cx
            .update(|window, cx| self.handler.text_length_utf16(window, cx))
            .ok()
            .flatten()
    }

    #[allow(dead_code)]
    pub fn accepts_text_input(&mut self, window: &mut Window, cx: &mut App) -> bool {
        self.handler.accepts_text_input(window, cx)
    }

    #[allow(dead_code)]
    pub fn query_accepts_text_input(&mut self) -> bool {
        self.cx
            .update(|window, cx| self.handler.accepts_text_input(window, cx))
            .unwrap_or(true)
    }

    /// See [`InputHandler::prefers_ime_for_printable_keys`].
    ///
    /// This is not a pure delegation to the handler: while a multi-stroke binding is pending this
    /// returns `false` regardless of the handler's preference, because the next printable key may
    /// complete a binding whose prefix already bypassed the IME.
    pub fn query_prefers_ime_for_printable_keys(&mut self) -> bool {
        self.cx
            .update(|window, cx| {
                // The next printable key may complete a chord whose prefix bypassed the IME.
                !window.has_pending_keystrokes()
                    && self.handler.prefers_ime_for_printable_keys(window, cx)
            })
            .unwrap_or(false)
    }

    /// See [`InputHandler::text_input_configuration`].
    pub fn text_input_configuration(
        &mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> TextInputConfiguration {
        self.handler.text_input_configuration(window, cx)
    }

    /// See [`InputHandler::text_input_editable_range`].
    pub fn text_input_editable_range(&mut self) -> Option<Range<usize>> {
        self.cx
            .update(|window, cx| self.handler.text_input_editable_range(window, cx))
            .ok()
            .flatten()
    }
}

/// A struct representing a selection in a text buffer, in UTF16 characters.
/// This is different from a range because the head may be before the tail.
#[derive(Debug)]
pub struct UTF16Selection {
    /// The range of text in the document this selection corresponds to
    /// in UTF16 characters.
    pub range: Range<usize>,
    /// Whether the head of this selection is at the start (true), or end (false)
    /// of the range
    pub reversed: bool,
}

/// Zed's interface for handling text input from the platform's IME system
/// This is currently a 1:1 exposure of the NSTextInputClient API:
///
/// <https://developer.apple.com/documentation/appkit/nstextinputclient>
pub trait InputHandler: 'static {
    /// Get the range of the user's currently selected text, if any
    /// Corresponds to [selectedRange()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438242-selectedrange)
    ///
    /// Return value is in terms of UTF-16 characters, from 0 to the length of the document
    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<UTF16Selection>;

    /// Get the range of the currently marked text, if any
    /// Corresponds to [markedRange()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438250-markedrange)
    ///
    /// Return value is in terms of UTF-16 characters, from 0 to the length of the document
    fn marked_text_range(&mut self, window: &mut Window, cx: &mut App) -> Option<Range<usize>>;

    /// Get the text for the given document range in UTF-16 characters
    /// Corresponds to [attributedSubstring(forProposedRange: actualRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438238-attributedsubstring)
    ///
    /// range_utf16 is in terms of UTF-16 characters
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<String>;

    /// Replace the text in the given document range with the given text
    /// Corresponds to [insertText(_:replacementRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438258-inserttext)
    ///
    /// replacement_range is in terms of UTF-16 characters
    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut App,
    );

    /// Replace the text in the given document range with the given text,
    /// and mark the given text as part of an IME 'composing' state
    /// Corresponds to [setMarkedText(_:selectedRange:replacementRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438246-setmarkedtext)
    ///
    /// range_utf16 is in terms of UTF-16 characters
    /// new_selected_range is in terms of UTF-16 characters
    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    );

    /// Remove the IME 'composing' state from the document
    /// Corresponds to [unmarkText()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438239-unmarktext)
    fn unmark_text(&mut self, window: &mut Window, cx: &mut App);

    /// Insert a platform-initiated paste at the current selection.
    ///
    /// Platforms that deliver paste as an input event rather than through an
    /// application-defined action (e.g. the DOM `paste` event on web) call
    /// this with the full clipboard contents. The default implementation
    /// inserts only the plain-text portion of the item.
    fn paste(&mut self, item: ClipboardItem, window: &mut Window, cx: &mut App) {
        if let Some(text) = item.text() {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    /// Get the bounds of the given document range in screen coordinates
    /// Corresponds to [firstRect(forCharacterRange:actualRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438240-firstrect)
    ///
    /// This is used for positioning the IME candidate window
    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>>;

    /// Get the character offset for the given point in terms of UTF16 characters
    ///
    /// Corresponds to [characterIndexForPoint:](https://developer.apple.com/documentation/appkit/nstextinputclient/characterindex(for:))
    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize>;

    /// Set the range of the user's currently selected text.
    ///
    /// This is the reverse data-flow direction from [`Self::selected_text_range`]:
    /// platforms call it when the system text machinery moves the selection on the
    /// application's behalf — e.g. the user drags a system selection handle or
    /// invokes Select All from system UI (iOS `UITextInput setSelectedTextRange:`,
    /// Android `InputConnection.setSelection`).
    ///
    /// range_utf16 is in terms of UTF-16 characters, from 0 to the length of the document
    fn set_selected_text_range(
        &mut self,
        _range_utf16: Range<usize>,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    /// Get the bounds of the focused text element in window coordinates, if known.
    ///
    /// This is the pull counterpart to the [`PlatformWindow::update_ime_position`]
    /// push: mobile platforms ask for the focused element's geometry when they
    /// need it (e.g. to frame system text-interaction UI overlaid on the focused
    /// element).
    fn element_bounds(&mut self, _window: &mut Window, _cx: &mut App) -> Option<Bounds<Pixels>> {
        None
    }

    /// Get the length of the document in UTF-16 characters, if known.
    fn text_length_utf16(&mut self, _window: &mut Window, _cx: &mut App) -> Option<usize> {
        None
    }

    /// Allows a given input context to opt into getting raw key repeats instead of
    /// sending these to the platform.
    /// TODO: Ideally we should be able to set ApplePressAndHoldEnabled in NSUserDefaults
    /// (which is how iTerm does it) but it doesn't seem to work for me.
    #[allow(dead_code)]
    fn apple_press_and_hold_enabled(&mut self) -> bool {
        true
    }

    /// Returns whether this handler is accepting text input to be inserted.
    fn accepts_text_input(&mut self, _window: &mut Window, _cx: &mut App) -> bool {
        true
    }

    /// The contiguous range of text, in UTF-16 code units, that platform text
    /// input may read and edit around the current selection.
    ///
    /// Platforms that mirror document text into an IME-editable buffer clamp
    /// the mirrored window to this range, so multi-step IME edit gestures
    /// (word deletion, autocorrect rewrites, suggestion picks) cannot reach
    /// content outside it. The range should contain the current selection;
    /// when it cannot (a selection spanning a region boundary), platforms
    /// degrade the mirrored IME context rather than widening the range.
    /// `None` places no bound.
    fn text_input_editable_range(
        &mut self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Range<usize>> {
        None
    }

    /// Returns whether printable keys should be routed to the IME before keybinding
    /// matching when a non-ASCII input source (e.g. Japanese, Korean, Chinese IME)
    /// is active. This prevents multi-stroke keybindings like `jj` from intercepting
    /// keys that the IME should compose.
    ///
    /// Defaults to `false`. The editor overrides this based on whether it expects
    /// character input (e.g. Vim insert mode returns `true`, normal mode returns `false`).
    /// The terminal keeps the default `false` so that raw keys reach the terminal process.
    fn prefers_ime_for_printable_keys(&mut self, _window: &mut Window, _cx: &mut App) -> bool {
        false
    }

    /// Get this handler's preferences for platform text assistance.
    ///
    /// GPUI re-queries this every frame and forwards it to the platform window
    /// only when it changes, so implementations must be cheap and may vary the
    /// result with application state (e.g. with the cursor's position).
    fn text_input_configuration(
        &mut self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> TextInputConfiguration {
        TextInputConfiguration::default()
    }
}

/// Window-bounds helpers that require access to the application's display state.
pub trait WindowBoundsExt: Sized {
    /// Creates a new window bounds that centers the window on the screen.
    fn centered(size: Size<Pixels>, cx: &App) -> Self;
}

impl WindowBoundsExt for WindowBounds {
    fn centered(size: Size<Pixels>, cx: &App) -> Self {
        WindowBounds::Windowed(Bounds::centered(None, size, cx))
    }
}

pub(crate) fn decode_static_image(
    bytes: &[u8],
    format: image::ImageFormat,
) -> Result<SmallVec<[Frame; 1]>> {
    let decoder = image::ImageReader::with_format(Cursor::new(bytes), format)
        .into_decoder()
        .context("creating image decoder")?;
    decode_static_image_from_decoder(decoder)
}

pub(crate) fn decode_static_image_from_decoder(
    mut decoder: impl image::ImageDecoder,
) -> Result<SmallVec<[Frame; 1]>> {
    let orientation = decoder
        .orientation()
        .context("reading decoder's orientation")?;
    let mut image = DynamicImage::from_decoder(decoder).context("decoding image")?;
    image.apply_orientation(orientation);

    let mut data = image.into_rgba8();
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    Ok(SmallVec::from_elem(Frame::new(data), 1))
}

/// Operations on [`Image`] that integrate it with GPUI's rendering and asset
/// systems.
///
/// The data-only parts of [`Image`] live in `gpui_platform_core`; this trait adds
/// the operations that require [`App`], a [`Window`], or the render pipeline.
pub trait ImageExt: Sized {
    /// Use the GPUI `use_asset` API to make this image renderable
    fn use_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>>;

    /// Use the GPUI `get_asset` API to make this image renderable
    fn get_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>>;

    /// Use the GPUI `remove_asset` API to drop this image, if possible.
    fn remove_asset(self: Arc<Self>, cx: &mut App);

    /// Check whether this image is present in GPUI's asset cache (loading or
    /// loaded), without fetching it.
    #[cfg(any(test, feature = "test-support"))]
    fn is_asset_cached(self: &Arc<Self>, cx: &App) -> bool;

    /// Convert the clipboard image to an `ImageData` object.
    fn to_image_data(&self, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>>;
}

impl ImageExt for Image {
    fn use_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>> {
        ImageSource::Image(self)
            .use_data(None, window, cx)
            .and_then(|result| result.ok())
    }

    fn get_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>> {
        ImageSource::Image(self)
            .get_data(None, window, cx)
            .and_then(|result| result.ok())
    }

    fn remove_asset(self: Arc<Self>, cx: &mut App) {
        ImageSource::Image(self).remove_asset(cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    fn is_asset_cached(self: &Arc<Self>, cx: &App) -> bool {
        ImageSource::Image(self.clone()).is_asset_cached(cx)
    }

    fn to_image_data(&self, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>> {
        let frames = match self.format {
            ImageFormat::Gif => {
                let decoder = GifDecoder::new(Cursor::new(&self.bytes))?;
                let mut frames = SmallVec::new();

                for frame in decoder.into_frames() {
                    match frame {
                        Ok(mut frame) => {
                            // Convert from RGBA to BGRA.
                            for pixel in frame.buffer_mut().chunks_exact_mut(4) {
                                pixel.swap(0, 2);
                            }
                            frames.push(frame);
                        }
                        Err(err) => {
                            log::debug!("Skipping GIF frame due to decode error: {err}");
                        }
                    }
                }

                if frames.is_empty() {
                    anyhow::bail!("GIF could not be decoded: all frames failed");
                }

                frames
            }
            ImageFormat::Png => decode_static_image(&self.bytes, image::ImageFormat::Png)?,
            ImageFormat::Jpeg => decode_static_image(&self.bytes, image::ImageFormat::Jpeg)?,
            ImageFormat::Webp => decode_static_image(&self.bytes, image::ImageFormat::WebP)?,
            ImageFormat::Bmp => decode_static_image(&self.bytes, image::ImageFormat::Bmp)?,
            ImageFormat::Tiff => decode_static_image(&self.bytes, image::ImageFormat::Tiff)?,
            ImageFormat::Ico => decode_static_image(&self.bytes, image::ImageFormat::Ico)?,
            ImageFormat::Svg => {
                return svg_renderer
                    .render_single_frame(&self.bytes, 1.0)
                    .map_err(Into::into);
            }
            ImageFormat::Pnm => decode_static_image(&self.bytes, image::ImageFormat::Pnm)?,
        };

        Ok(Arc::new(RenderImage::new(frames)))
    }
}

#[cfg(test)]
mod image_tests {
    use super::*;
    use crate::size;
    use std::sync::Arc;

    #[test]
    fn test_image_to_image_data_applies_exif_orientation() {
        let image = Image::from_bytes(
            ImageFormat::Jpeg,
            include_bytes!("../examples/image/exif-orientation-rotate-180.jpg").to_vec(),
        );

        let render_image = image.to_image_data(SvgRenderer::new(Arc::new(()))).unwrap();

        assert_eq!(render_image.size(0), size(16.into(), 32.into()));

        let bytes = render_image.as_bytes(0).unwrap();
        assert_eq!(&bytes[..4], &[255, 255, 255, 255]);
        assert_eq!(&bytes[(16 * 32 - 1) * 4..], &[0, 0, 0, 255]);
    }

    #[test]
    fn test_svg_image_to_image_data_converts_to_bgra() {
        let image = Image::from_bytes(
            ImageFormat::Svg,
            br##"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1">
<rect width="1" height="1" fill="#38BDF8"/>
</svg>"##
                .to_vec(),
        );

        let render_image = image.to_image_data(SvgRenderer::new(Arc::new(()))).unwrap();
        let bytes = render_image.as_bytes(0).unwrap();

        for pixel in bytes.chunks_exact(4) {
            assert_eq!(pixel, &[0xF8, 0xBD, 0x38, 0xFF]);
        }
    }
}

#[cfg(all(test, any(target_os = "linux", target_os = "freebsd")))]
mod tests {
    use super::*;
    use crate::WindowButton;
    use std::collections::HashSet;

    #[test]
    fn test_window_button_layout_parse_standard() {
        let layout = WindowButtonLayout::parse("close,minimize:maximize").unwrap();
        assert_eq!(
            layout.left,
            [
                Some(WindowButton::Close),
                Some(WindowButton::Minimize),
                None
            ]
        );
        assert_eq!(layout.right, [Some(WindowButton::Maximize), None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_right_only() {
        let layout = WindowButtonLayout::parse("minimize,maximize,close").unwrap();
        assert_eq!(layout.left, [None, None, None]);
        assert_eq!(
            layout.right,
            [
                Some(WindowButton::Minimize),
                Some(WindowButton::Maximize),
                Some(WindowButton::Close)
            ]
        );
    }

    #[test]
    fn test_window_button_layout_parse_left_only() {
        let layout = WindowButtonLayout::parse("close,minimize,maximize:").unwrap();
        assert_eq!(
            layout.left,
            [
                Some(WindowButton::Close),
                Some(WindowButton::Minimize),
                Some(WindowButton::Maximize)
            ]
        );
        assert_eq!(layout.right, [None, None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_with_whitespace() {
        let layout = WindowButtonLayout::parse(" close , minimize : maximize ").unwrap();
        assert_eq!(
            layout.left,
            [
                Some(WindowButton::Close),
                Some(WindowButton::Minimize),
                None
            ]
        );
        assert_eq!(layout.right, [Some(WindowButton::Maximize), None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_empty() {
        let layout = WindowButtonLayout::parse("").unwrap();
        assert_eq!(layout.left, [None, None, None]);
        assert_eq!(layout.right, [None, None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_intentionally_empty() {
        let layout = WindowButtonLayout::parse(":").unwrap();
        assert_eq!(layout.left, [None, None, None]);
        assert_eq!(layout.right, [None, None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_invalid_buttons() {
        let layout = WindowButtonLayout::parse("close,invalid,minimize:maximize,foo").unwrap();
        assert_eq!(
            layout.left,
            [
                Some(WindowButton::Close),
                Some(WindowButton::Minimize),
                None
            ]
        );
        assert_eq!(layout.right, [Some(WindowButton::Maximize), None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_deduplicates_same_side_buttons() {
        let layout = WindowButtonLayout::parse("close,close,minimize").unwrap();
        assert_eq!(
            layout.right,
            [
                Some(WindowButton::Close),
                Some(WindowButton::Minimize),
                None
            ]
        );
        assert_eq!(layout.format(), ":close,minimize");
    }

    #[test]
    fn test_window_button_layout_parse_deduplicates_buttons_across_sides() {
        let layout = WindowButtonLayout::parse("close:maximize,close,minimize").unwrap();
        assert_eq!(layout.left, [Some(WindowButton::Close), None, None]);
        assert_eq!(
            layout.right,
            [
                Some(WindowButton::Maximize),
                Some(WindowButton::Minimize),
                None
            ]
        );

        let button_ids: Vec<_> = layout
            .left
            .iter()
            .chain(layout.right.iter())
            .flatten()
            .map(WindowButton::id)
            .collect();
        let unique_button_ids = button_ids.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique_button_ids.len(), button_ids.len());
        assert_eq!(layout.format(), "close:maximize,minimize");
    }

    #[test]
    fn test_window_button_layout_parse_gnome_style() {
        let layout = WindowButtonLayout::parse("close").unwrap();
        assert_eq!(layout.left, [None, None, None]);
        assert_eq!(layout.right, [Some(WindowButton::Close), None, None]);
    }

    #[test]
    fn test_window_button_layout_parse_elementary_style() {
        let layout = WindowButtonLayout::parse("close:maximize").unwrap();
        assert_eq!(layout.left, [Some(WindowButton::Close), None, None]);
        assert_eq!(layout.right, [Some(WindowButton::Maximize), None, None]);
    }

    #[test]
    fn test_window_button_layout_round_trip() {
        let cases = [
            "close:minimize,maximize",
            "minimize,maximize,close:",
            ":close",
            "close:",
            "close:maximize",
            ":",
        ];

        for case in cases {
            let layout = WindowButtonLayout::parse(case).unwrap();
            assert_eq!(layout.format(), case, "Round-trip failed for: {}", case);
        }
    }

    #[test]
    fn test_window_button_layout_linux_default() {
        let layout = WindowButtonLayout::linux_default();
        assert_eq!(layout.left, [None, None, None]);
        assert_eq!(
            layout.right,
            [
                Some(WindowButton::Minimize),
                Some(WindowButton::Maximize),
                Some(WindowButton::Close)
            ]
        );

        let round_tripped = WindowButtonLayout::parse(&layout.format()).unwrap();
        assert_eq!(round_tripped, layout);
    }

    #[test]
    fn test_window_button_layout_parse_all_invalid() {
        assert!(WindowButtonLayout::parse("asdfghjkl").is_err());
    }
}
