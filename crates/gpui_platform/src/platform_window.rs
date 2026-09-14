//! Platform window contract shared by `gpui` and its platform backends.

#[cfg(all(target_os = "linux", feature = "wayland"))]
use crate::layer_shell;
use crate::{
    Decorations, DispatchEventResult, ExternalDragPayload, GpuSpecs, PlatformDisplay,
    PlatformInput, PlatformInputHandler, PromptButton, PromptLevel, RequestFrameOptions,
    ResizeEdge, SystemWindowTab, TextInputConfiguration, TextInputStateChange, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls, WindowDecorations,
    WindowInsets,
};
use futures::channel::oneshot;
use gpui_engine::SceneRenderer;
use gpui_types::{Bounds, Capslock, Modifiers, Pixels, Point, Size};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use std::any::Any;
use std::rc::Rc;

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
    /// Borrows this window's scene renderer without presenting a frame.
    ///
    /// Use this for renderer queries such as reading the sprite atlas or
    /// capturing an image; backends must not advance their frame loop here.
    fn with_renderer(&mut self, f: &mut dyn FnMut(&mut dyn SceneRenderer));
    /// Submits a frame through this window's scene renderer.
    ///
    /// `f` encodes and submits the frame and returns whether it was presented.
    /// Backends that pace themselves on compositor callbacks use that result
    /// to update their frame loop.
    fn present(&mut self, f: &mut dyn FnMut(&mut dyn SceneRenderer) -> bool);
    fn schedule_frame(&self) {}
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
    fn as_test(&mut self) -> Option<&mut dyn Any> {
        None
    }
}
