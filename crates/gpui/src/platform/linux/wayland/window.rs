use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::ffi::c_void;
use std::num::NonZeroU32;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use blade_graphics as gpu;
use blade_rwh::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use collections::{HashMap, HashSet};
use futures::channel::oneshot::Receiver;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wayland_backend::client::ObjectId;
use wayland_client::WEnum;
use wayland_client::{protocol::wl_surface, Proxy};
use wayland_protocols::wp::fractional_scale::v1::client::wp_fractional_scale_v1;
use wayland_protocols::wp::viewporter::client::wp_viewport;
use wayland_protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1;
use wayland_protocols::xdg::shell::client::xdg_surface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::{self, WmCapabilities};

use crate::platform::blade::BladeRenderer;
use crate::platform::linux::wayland::display::WaylandDisplay;
use crate::platform::{PlatformAtlas, PlatformInputHandler, PlatformWindow};
use crate::scene::Scene;
use crate::{
    px, size, Bounds, DevicePixels, Globals, Modifiers, Pixels, PlatformDisplay, PlatformInput,
    Point, PromptLevel, Size, WaylandClientState, WaylandClientStatePtr, WindowAppearance,
    WindowBackgroundAppearance, WindowParams,
};

#[derive(Default)]
pub(crate) struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> crate::DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

struct RawWindow {
    window: *mut c_void,
    display: *mut c_void,
}

unsafe impl HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut wh = blade_rwh::WaylandWindowHandle::empty();
        wh.surface = self.window;
        wh.into()
    }
}

unsafe impl HasRawDisplayHandle for RawWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let mut dh = blade_rwh::WaylandDisplayHandle::empty();
        dh.display = self.display;
        dh.into()
    }
}

pub struct WaylandWindowState {
    xdg_surface: xdg_surface::XdgSurface,
    pub surface: wl_surface::WlSurface,
    toplevel: xdg_toplevel::XdgToplevel,
    viewport: Option<wp_viewport::WpViewport>,
    outputs: HashSet<ObjectId>,
    globals: Globals,
    renderer: BladeRenderer,
    bounds: Bounds<u32>,
    scale: f32,
    input_handler: Option<PlatformInputHandler>,
    decoration_state: WaylandDecorationState,
    fullscreen: bool,
    maximized: bool,
    client: WaylandClientStatePtr,
    callbacks: Callbacks,
}

#[derive(Clone)]
pub struct WaylandWindowStatePtr {
    state: Rc<RefCell<WaylandWindowState>>,
    callbacks: Rc<RefCell<Callbacks>>,
}

impl WaylandWindowState {
    pub(crate) fn new(
        surface: wl_surface::WlSurface,
        xdg_surface: xdg_surface::XdgSurface,
        viewport: Option<wp_viewport::WpViewport>,
        toplevel: xdg_toplevel::XdgToplevel,
        client: WaylandClientStatePtr,
        globals: Globals,
        options: WindowParams,
    ) -> Self {
        let bounds = options.bounds.map(|p| p.0 as u32);

        let raw = RawWindow {
            window: surface.id().as_ptr().cast::<c_void>(),
            display: surface
                .backend()
                .upgrade()
                .unwrap()
                .display_ptr()
                .cast::<c_void>(),
        };
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init_windowed(
                    &raw,
                    gpu::ContextDesc {
                        validation: false,
                        capture: false,
                        overlay: false,
                    },
                )
            }
            .unwrap(),
        );
        let extent = gpu::Extent {
            width: bounds.size.width,
            height: bounds.size.height,
            depth: 1,
        };

        Self {
            xdg_surface,
            surface,
            toplevel,
            viewport,
            globals,

            outputs: HashSet::default(),

            renderer: BladeRenderer::new(gpu, extent),
            bounds,
            scale: 1.0,
            input_handler: None,
            decoration_state: WaylandDecorationState::Client,
            fullscreen: false,
            maximized: false,
            callbacks: Callbacks::default(),
            client,
        }
    }
}

pub(crate) struct WaylandWindow(pub WaylandWindowStatePtr);

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        let mut state = self.0.state.borrow_mut();
        let surface_id = state.surface.id();
        let client = state.client.clone();
        state.renderer.destroy();
        state.toplevel.destroy();
        state.xdg_surface.destroy();
        state.surface.destroy();

        let state_ptr = self.0.clone();
        state.globals.executor.spawn(async move {
            state_ptr.close();
            client.drop_window(&surface_id)
        });
        drop(state);
    }
}

impl WaylandWindow {
    fn borrow(&self) -> Ref<WaylandWindowState> {
        self.0.state.borrow()
    }

    fn borrow_mut(&self) -> RefMut<WaylandWindowState> {
        self.0.state.borrow_mut()
    }

    pub fn new(
        globals: Globals,
        client: WaylandClientStatePtr,
        params: WindowParams,
    ) -> (Self, ObjectId) {
        let surface = globals.compositor.create_surface(&globals.qh, ());
        let xdg_surface = globals
            .wm_base
            .get_xdg_surface(&surface, &globals.qh, surface.id());
        let toplevel = xdg_surface.get_toplevel(&globals.qh, surface.id());

        if let Some(fractional_scale_manager) = globals.fractional_scale_manager.as_ref() {
            fractional_scale_manager.get_fractional_scale(&surface, &globals.qh, surface.id());
        }

        // Attempt to set up window decorations based on the requested configuration
        if let Some(decoration_manager) = globals.decoration_manager.as_ref() {
            let decoration =
                decoration_manager.get_toplevel_decoration(&toplevel, &globals.qh, surface.id());

            // Request client side decorations if possible
            decoration.set_mode(zxdg_toplevel_decoration_v1::Mode::ClientSide);
        }

        let viewport = globals
            .viewporter
            .as_ref()
            .map(|viewporter| viewporter.get_viewport(&surface, &globals.qh, ()));

        surface.frame(&globals.qh, surface.id());

        let this = Self(WaylandWindowStatePtr {
            state: Rc::new(RefCell::new(WaylandWindowState::new(
                surface.clone(),
                xdg_surface,
                viewport,
                toplevel,
                client,
                globals,
                params,
            ))),
            callbacks: Rc::new(RefCell::new(Callbacks::default())),
        });

        // Kick things off
        surface.commit();

        (this, surface.id())
    }
}

impl WaylandWindowStatePtr {
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
    }

    pub fn frame(&self, from_frame_callback: bool) {
        if from_frame_callback {
            let state = self.state.borrow_mut();
            state.surface.frame(&state.globals.qh, state.surface.id());
            drop(state);
        }
        let mut cb = self.callbacks.borrow_mut();
        if let Some(fun) = cb.request_frame.as_mut() {
            fun();
        }
    }

    pub fn handle_xdg_surface_event(&self, event: xdg_surface::Event) {
        match event {
            xdg_surface::Event::Configure { serial } => {
                let state = self.state.borrow();
                state.xdg_surface.ack_configure(serial);
                drop(state);
                self.frame(false);
            }
            _ => {}
        }
    }

    pub fn handle_toplevel_decoration_event(&self, event: zxdg_toplevel_decoration_v1::Event) {
        match event {
            zxdg_toplevel_decoration_v1::Event::Configure { mode } => match mode {
                WEnum::Value(zxdg_toplevel_decoration_v1::Mode::ServerSide) => {
                    self.set_decoration_state(WaylandDecorationState::Server)
                }
                WEnum::Value(zxdg_toplevel_decoration_v1::Mode::ClientSide) => {
                    self.set_decoration_state(WaylandDecorationState::Server)
                }
                WEnum::Value(_) => {
                    log::warn!("Unknown decoration mode");
                }
                WEnum::Unknown(v) => {
                    log::warn!("Unknown decoration mode: {}", v);
                }
            },
            _ => {}
        }
    }

    pub fn handle_fractional_scale_event(&self, event: wp_fractional_scale_v1::Event) {
        match event {
            wp_fractional_scale_v1::Event::PreferredScale { scale } => {
                self.rescale(scale as f32 / 120.0);
            }
            _ => {}
        }
    }

    pub fn handle_toplevel_event(&self, event: xdg_toplevel::Event) -> bool {
        match event {
            xdg_toplevel::Event::Configure {
                width,
                height,
                states,
            } => {
                let width = NonZeroU32::new(width as u32);
                let height = NonZeroU32::new(height as u32);
                let fullscreen = states.contains(&(xdg_toplevel::State::Fullscreen as u8));
                let maximized = states.contains(&(xdg_toplevel::State::Maximized as u8));
                self.resize(width, height);
                self.set_fullscreen(fullscreen);
                let mut state = self.state.borrow_mut();
                state.maximized = true;

                false
            }
            xdg_toplevel::Event::Close => {
                let mut cb = self.callbacks.borrow_mut();
                if let Some(mut should_close) = cb.should_close.take() {
                    let result = (should_close)();
                    cb.should_close = Some(should_close);
                    if result {
                        drop(cb);
                        self.close();
                    }
                    result
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn handle_surface_event(
        &self,
        event: wl_surface::Event,
        output_scales: HashMap<ObjectId, i32>,
    ) {
        let mut state = self.state.borrow_mut();

        // We use `WpFractionalScale` instead to set the scale if it's available
        if state.globals.fractional_scale_manager.is_some() {
            return;
        }

        match event {
            wl_surface::Event::Enter { output } => {
                // We use `PreferredBufferScale` instead to set the scale if it's available
                if state.surface.version() >= wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    return;
                }

                state.outputs.insert(output.id());

                let mut scale = 1;
                for output in state.outputs.iter() {
                    if let Some(s) = output_scales.get(output) {
                        scale = scale.max(*s)
                    }
                }

                state.surface.set_buffer_scale(scale);
                drop(state);
                self.rescale(scale as f32);
            }
            wl_surface::Event::Leave { output } => {
                // We use `PreferredBufferScale` instead to set the scale if it's available
                if state.surface.version() >= wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    return;
                }

                state.outputs.remove(&output.id());

                let mut scale = 1;
                for output in state.outputs.iter() {
                    if let Some(s) = output_scales.get(output) {
                        scale = scale.max(*s)
                    }
                }

                state.surface.set_buffer_scale(scale);
                drop(state);
                self.rescale(scale as f32);
            }
            wl_surface::Event::PreferredBufferScale { factor } => {
                state.surface.set_buffer_scale(factor);
                drop(state);
                self.rescale(factor as f32);
            }
            _ => {}
        }
    }

    pub fn set_size_and_scale(
        &self,
        width: Option<NonZeroU32>,
        height: Option<NonZeroU32>,
        scale: Option<f32>,
    ) {
        let (width, height, scale) = {
            let mut state = self.state.borrow_mut();
            if width.map_or(true, |width| width.get() == state.bounds.size.width)
                && height.map_or(true, |height| height.get() == state.bounds.size.height)
                && scale.map_or(true, |scale| scale == state.scale)
            {
                return;
            }
            if let Some(width) = width {
                state.bounds.size.width = width.get();
            }
            if let Some(height) = height {
                state.bounds.size.height = height.get();
            }
            if let Some(scale) = scale {
                state.scale = scale;
            }
            let width = state.bounds.size.width;
            let height = state.bounds.size.height;
            let scale = state.scale;
            state.renderer.update_drawable_size(size(
                width as f64 * scale as f64,
                height as f64 * scale as f64,
            ));
            (width, height, scale)
        };

        if let Some(ref mut fun) = self.callbacks.borrow_mut().resize {
            fun(
                Size {
                    width: px(width as f32),
                    height: px(height as f32),
                },
                scale,
            );
        }

        {
            let state = self.state.borrow();
            if let Some(viewport) = &state.viewport {
                viewport.set_destination(width as i32, height as i32);
            }
        }
    }

    pub fn resize(&self, width: Option<NonZeroU32>, height: Option<NonZeroU32>) {
        self.set_size_and_scale(width, height, None);
    }

    pub fn rescale(&self, scale: f32) {
        self.set_size_and_scale(None, None, Some(scale));
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        let mut state = self.state.borrow_mut();
        state.fullscreen = fullscreen;

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = callbacks.fullscreen {
            fun(fullscreen)
        }
    }

    /// Notifies the window of the state of the decorations.
    ///
    /// # Note
    ///
    /// This API is indirectly called by the wayland compositor and
    /// not meant to be called by a user who wishes to change the state
    /// of the decorations. This is because the state of the decorations
    /// is managed by the compositor and not the client.
    pub fn set_decoration_state(&self, state: WaylandDecorationState) {
        self.state.borrow_mut().decoration_state = state;
    }

    pub fn close(&self) {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(fun) = callbacks.close.take() {
            fun()
        }
    }

    pub fn handle_input(&self, input: PlatformInput) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().input {
            if !fun(input.clone()).propagate {
                return;
            }
        }
        if let PlatformInput::KeyDown(event) = input {
            if let Some(ime_key) = &event.keystroke.ime_key {
                let mut state = self.state.borrow_mut();
                if let Some(mut input_handler) = state.input_handler.take() {
                    drop(state);
                    input_handler.replace_text_in_range(None, ime_key);
                    self.state.borrow_mut().input_handler = Some(input_handler);
                }
            }
        }
    }

    pub fn set_focused(&self, focus: bool) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().active_status_change {
            fun(focus);
        }
    }
}

impl HasWindowHandle for WaylandWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unimplemented!()
    }
}

impl HasDisplayHandle for WaylandWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unimplemented!()
    }
}

impl PlatformWindow for WaylandWindow {
    fn bounds(&self) -> Bounds<DevicePixels> {
        self.borrow().bounds.map(|p| DevicePixels(p as i32))
    }

    fn is_maximized(&self) -> bool {
        self.borrow().maximized
    }

    fn is_minimized(&self) -> bool {
        // This cannot be determined by the client
        false
    }

    fn content_size(&self) -> Size<Pixels> {
        let state = self.borrow();
        Size {
            width: Pixels(state.bounds.size.width as f32),
            height: Pixels(state.bounds.size.height as f32),
        }
    }

    fn scale_factor(&self) -> f32 {
        self.borrow().scale
    }

    // todo(linux)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    // todo(linux)
    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::new(WaylandDisplay {})
    }

    // todo(linux)
    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    // todo(linux)
    fn modifiers(&self) -> Modifiers {
        crate::Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Option<Receiver<usize>> {
        None
    }

    fn activate(&self) {
        // todo(linux)
    }

    // todo(linux)
    fn is_active(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: &str) {
        self.borrow_mut().toplevel.set_title(title.to_string());
    }

    fn set_background_appearance(&mut self, _background_appearance: WindowBackgroundAppearance) {
        // todo(linux)
    }

    fn set_edited(&mut self, edited: bool) {
        // todo(linux)
    }

    fn show_character_palette(&self) {
        // todo(linux)
    }

    fn minimize(&self) {
        self.borrow_mut().toplevel.set_minimized();
    }

    fn zoom(&self) {
        // todo(linux)
    }

    fn toggle_fullscreen(&self) {
        let state = self.borrow_mut();
        if !state.fullscreen {
            state.toplevel.set_fullscreen(None);
        } else {
            state.toplevel.unset_fullscreen();
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.borrow().fullscreen
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> crate::DispatchEventResult>) {
        self.0.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.borrow_mut().fullscreen = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        // todo(linux)
    }

    // todo(linux)
    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool {
        false
    }

    fn draw(&self, scene: &Scene) {
        let mut state = self.borrow_mut();
        state.renderer.draw(scene);
    }

    fn completed_frame(&self) {
        let mut state = self.borrow_mut();
        state.surface.commit();
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        let state = self.borrow();
        state.renderer.sprite_atlas().clone()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WaylandDecorationState {
    /// Decorations are to be provided by the client
    Client,

    /// Decorations are provided by the server
    Server,
}
