use std::cell::{Ref, RefCell, RefMut};
use std::ffi::c_void;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Arc;

use blade_graphics as gpu;
use collections::HashMap;
use futures::channel::oneshot::Receiver;

use raw_window_handle as rwh;
use wayland_backend::client::ObjectId;
use wayland_client::WEnum;
use wayland_client::{protocol::wl_surface, Proxy};
use wayland_protocols::wp::fractional_scale::v1::client::wp_fractional_scale_v1;
use wayland_protocols::wp::viewporter::client::wp_viewport;
use wayland_protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1;
use wayland_protocols::xdg::shell::client::xdg_surface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::{self};
use wayland_protocols_plasma::blur::client::org_kde_kwin_blur;

use crate::platform::blade::{BladeRenderer, BladeSurfaceConfig};
use crate::platform::linux::wayland::display::WaylandDisplay;
use crate::platform::linux::wayland::serial::SerialKind;
use crate::platform::{PlatformAtlas, PlatformInputHandler, PlatformWindow};
use crate::scene::Scene;
use crate::{
    px, size, AnyWindowHandle, Bounds, Globals, Modifiers, Output, Pixels, PlatformDisplay,
    PlatformInput, Point, PromptLevel, Size, WaylandClientStatePtr, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowParams,
};

#[derive(Default)]
pub(crate) struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> crate::DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

struct RawWindow {
    window: *mut c_void,
    display: *mut c_void,
}

impl rwh::HasWindowHandle for RawWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let window = NonNull::new(self.window).unwrap();
        let handle = rwh::WaylandWindowHandle::new(window);
        Ok(unsafe { rwh::WindowHandle::borrow_raw(handle.into()) })
    }
}
impl rwh::HasDisplayHandle for RawWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        let display = NonNull::new(self.display).unwrap();
        let handle = rwh::WaylandDisplayHandle::new(display);
        Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
    }
}

pub struct WaylandWindowState {
    xdg_surface: xdg_surface::XdgSurface,
    acknowledged_first_configure: bool,
    pub surface: wl_surface::WlSurface,
    decoration: Option<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
    appearance: WindowAppearance,
    blur: Option<org_kde_kwin_blur::OrgKdeKwinBlur>,
    toplevel: xdg_toplevel::XdgToplevel,
    viewport: Option<wp_viewport::WpViewport>,
    outputs: HashMap<ObjectId, Output>,
    display: Option<(ObjectId, Output)>,
    globals: Globals,
    renderer: BladeRenderer,
    bounds: Bounds<Pixels>,
    scale: f32,
    input_handler: Option<PlatformInputHandler>,
    decoration_state: WaylandDecorationState,
    fullscreen: bool,
    maximized: bool,
    windowed_bounds: Bounds<Pixels>,
    client: WaylandClientStatePtr,
    handle: AnyWindowHandle,
    active: bool,
}

#[derive(Clone)]
pub struct WaylandWindowStatePtr {
    state: Rc<RefCell<WaylandWindowState>>,
    callbacks: Rc<RefCell<Callbacks>>,
}

impl WaylandWindowState {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        handle: AnyWindowHandle,
        surface: wl_surface::WlSurface,
        xdg_surface: xdg_surface::XdgSurface,
        toplevel: xdg_toplevel::XdgToplevel,
        decoration: Option<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
        appearance: WindowAppearance,
        viewport: Option<wp_viewport::WpViewport>,
        client: WaylandClientStatePtr,
        globals: Globals,
        options: WindowParams,
    ) -> anyhow::Result<Self> {
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
            .map_err(|e| anyhow::anyhow!("{:?}", e))?,
        );
        let config = BladeSurfaceConfig {
            size: gpu::Extent {
                width: options.bounds.size.width.0 as u32,
                height: options.bounds.size.height.0 as u32,
                depth: 1,
            },
            transparent: options.window_background != WindowBackgroundAppearance::Opaque,
        };

        Ok(Self {
            xdg_surface,
            acknowledged_first_configure: false,
            surface,
            decoration,
            blur: None,
            toplevel,
            viewport,
            globals,
            outputs: HashMap::default(),
            display: None,
            renderer: BladeRenderer::new(gpu, config),
            bounds: options.bounds,
            scale: 1.0,
            input_handler: None,
            decoration_state: WaylandDecorationState::Client,
            fullscreen: false,
            maximized: false,
            windowed_bounds: options.bounds,
            client,
            appearance,
            handle,
            active: false,
        })
    }
}

pub(crate) struct WaylandWindow(pub WaylandWindowStatePtr);
pub enum ImeInput {
    InsertText(String),
    SetMarkedText(String),
    UnmarkText,
    DeleteText,
}

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        let mut state = self.0.state.borrow_mut();
        let surface_id = state.surface.id();
        let client = state.client.clone();

        state.renderer.destroy();
        if let Some(decoration) = &state.decoration {
            decoration.destroy();
        }
        if let Some(blur) = &state.blur {
            blur.release();
        }
        state.toplevel.destroy();
        if let Some(viewport) = &state.viewport {
            viewport.destroy();
        }
        state.xdg_surface.destroy();
        state.surface.destroy();

        let state_ptr = self.0.clone();
        state
            .globals
            .executor
            .spawn(async move {
                state_ptr.close();
                client.drop_window(&surface_id)
            })
            .detach();
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
        handle: AnyWindowHandle,
        globals: Globals,
        client: WaylandClientStatePtr,
        params: WindowParams,
        appearance: WindowAppearance,
    ) -> anyhow::Result<(Self, ObjectId)> {
        let surface = globals.compositor.create_surface(&globals.qh, ());
        let xdg_surface = globals
            .wm_base
            .get_xdg_surface(&surface, &globals.qh, surface.id());
        let toplevel = xdg_surface.get_toplevel(&globals.qh, surface.id());
        toplevel.set_min_size(200, 200);

        if let Some(fractional_scale_manager) = globals.fractional_scale_manager.as_ref() {
            fractional_scale_manager.get_fractional_scale(&surface, &globals.qh, surface.id());
        }

        // Attempt to set up window decorations based on the requested configuration
        let decoration = globals
            .decoration_manager
            .as_ref()
            .map(|decoration_manager| {
                let decoration = decoration_manager.get_toplevel_decoration(
                    &toplevel,
                    &globals.qh,
                    surface.id(),
                );
                decoration.set_mode(zxdg_toplevel_decoration_v1::Mode::ClientSide);
                decoration
            });

        let viewport = globals
            .viewporter
            .as_ref()
            .map(|viewporter| viewporter.get_viewport(&surface, &globals.qh, ()));

        let this = Self(WaylandWindowStatePtr {
            state: Rc::new(RefCell::new(WaylandWindowState::new(
                handle,
                surface.clone(),
                xdg_surface,
                toplevel,
                decoration,
                appearance,
                viewport,
                client,
                globals,
                params,
            )?)),
            callbacks: Rc::new(RefCell::new(Callbacks::default())),
        });

        // Kick things off
        surface.commit();

        Ok((this, surface.id()))
    }
}

impl WaylandWindowStatePtr {
    pub fn handle(&self) -> AnyWindowHandle {
        self.state.borrow().handle
    }

    pub fn surface(&self) -> wl_surface::WlSurface {
        self.state.borrow().surface.clone()
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
    }

    pub fn frame(&self, request_frame_callback: bool) {
        if request_frame_callback {
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
                let mut state = self.state.borrow_mut();
                state.xdg_surface.ack_configure(serial);
                let request_frame_callback = !state.acknowledged_first_configure;
                state.acknowledged_first_configure = true;
                drop(state);
                self.frame(request_frame_callback);
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
                    self.set_decoration_state(WaylandDecorationState::Client)
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
                let mut size = if width == 0 || height == 0 {
                    None
                } else {
                    Some(size(px(width as f32), px(height as f32)))
                };

                let fullscreen = states.contains(&(xdg_toplevel::State::Fullscreen as u8));
                let maximized = states.contains(&(xdg_toplevel::State::Maximized as u8));

                let mut state = self.state.borrow_mut();
                let got_unmaximized = state.maximized && !maximized;
                state.fullscreen = fullscreen;
                state.maximized = maximized;

                if got_unmaximized {
                    size = Some(state.windowed_bounds.size);
                } else if !fullscreen && !maximized {
                    if let Some(size) = size {
                        state.windowed_bounds = Bounds {
                            origin: Point::default(),
                            size,
                        };
                    }
                }

                drop(state);
                if let Some(size) = size {
                    self.resize(size);
                }

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
                    true
                }
            }
            _ => false,
        }
    }

    pub fn handle_surface_event(
        &self,
        event: wl_surface::Event,
        outputs: HashMap<ObjectId, Output>,
    ) {
        let mut state = self.state.borrow_mut();

        match event {
            wl_surface::Event::Enter { output } => {
                let id = output.id();

                let Some(output) = outputs.get(&id) else {
                    return;
                };

                state.outputs.insert(id, output.clone());

                let scale = primary_output_scale(&mut state);

                // We use `PreferredBufferScale` instead to set the scale if it's available
                if state.surface.version() < wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    state.surface.set_buffer_scale(scale);
                    drop(state);
                    self.rescale(scale as f32);
                }
            }
            wl_surface::Event::Leave { output } => {
                state.outputs.remove(&output.id());

                let scale = primary_output_scale(&mut state);

                // We use `PreferredBufferScale` instead to set the scale if it's available
                if state.surface.version() < wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    state.surface.set_buffer_scale(scale);
                    drop(state);
                    self.rescale(scale as f32);
                }
            }
            wl_surface::Event::PreferredBufferScale { factor } => {
                // We use `WpFractionalScale` instead to set the scale if it's available
                if state.globals.fractional_scale_manager.is_none() {
                    state.surface.set_buffer_scale(factor);
                    drop(state);
                    self.rescale(factor as f32);
                }
            }
            _ => {}
        }
    }

    pub fn handle_ime(&self, ime: ImeInput) {
        let mut state = self.state.borrow_mut();
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            match ime {
                ImeInput::InsertText(text) => {
                    input_handler.replace_text_in_range(None, &text);
                }
                ImeInput::SetMarkedText(text) => {
                    input_handler.replace_and_mark_text_in_range(None, &text, None);
                }
                ImeInput::UnmarkText => {
                    input_handler.unmark_text();
                }
                ImeInput::DeleteText => {
                    if let Some(marked) = input_handler.marked_text_range() {
                        input_handler.replace_text_in_range(Some(marked), "");
                    }
                }
            }
            self.state.borrow_mut().input_handler = Some(input_handler);
        }
    }

    pub fn get_ime_area(&self) -> Option<Bounds<Pixels>> {
        let mut state = self.state.borrow_mut();
        let mut bounds: Option<Bounds<Pixels>> = None;
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            if let Some(range) = input_handler.selected_text_range() {
                bounds = input_handler.bounds_for_range(range);
            }
            self.state.borrow_mut().input_handler = Some(input_handler);
        }
        bounds
    }

    pub fn set_size_and_scale(&self, size: Option<Size<Pixels>>, scale: Option<f32>) {
        let (size, scale) = {
            let mut state = self.state.borrow_mut();
            if size.map_or(true, |size| size == state.bounds.size)
                && scale.map_or(true, |scale| scale == state.scale)
            {
                return;
            }
            if let Some(size) = size {
                state.bounds.size = size;
            }
            if let Some(scale) = scale {
                state.scale = scale;
            }
            let device_bounds = state.bounds.to_device_pixels(state.scale);
            state.renderer.update_drawable_size(device_bounds.size);
            (state.bounds.size, state.scale)
        };

        if let Some(ref mut fun) = self.callbacks.borrow_mut().resize {
            fun(size, scale);
        }

        {
            let state = self.state.borrow();
            if let Some(viewport) = &state.viewport {
                viewport.set_destination(size.width.0 as i32, size.height.0 as i32);
            }
        }
    }

    pub fn resize(&self, size: Size<Pixels>) {
        self.set_size_and_scale(Some(size), None);
    }

    pub fn rescale(&self, scale: f32) {
        self.set_size_and_scale(None, Some(scale));
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
        self.state.borrow_mut().active = focus;
        if let Some(ref mut fun) = self.callbacks.borrow_mut().active_status_change {
            fun(focus);
        }
    }

    pub fn set_appearance(&mut self, appearance: WindowAppearance) {
        self.state.borrow_mut().appearance = appearance;

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = callbacks.appearance_changed {
            (fun)()
        }
    }
}

fn primary_output_scale(state: &mut RefMut<WaylandWindowState>) -> i32 {
    let mut scale = 1;
    let mut current_output = state.display.take();
    for (id, output) in state.outputs.iter() {
        if let Some((_, output_data)) = &current_output {
            if output.scale > output_data.scale {
                current_output = Some((id.clone(), output.clone()));
            }
        } else {
            current_output = Some((id.clone(), output.clone()));
        }
        scale = scale.max(output.scale);
    }
    state.display = current_output;
    scale
}

impl rwh::HasWindowHandle for WaylandWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        unimplemented!()
    }
}
impl rwh::HasDisplayHandle for WaylandWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        unimplemented!()
    }
}

impl PlatformWindow for WaylandWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        self.borrow().maximized
    }

    fn window_bounds(&self) -> WindowBounds {
        let state = self.borrow();
        if state.fullscreen {
            WindowBounds::Fullscreen(state.windowed_bounds)
        } else if state.maximized {
            WindowBounds::Maximized(state.windowed_bounds)
        } else {
            drop(state);
            WindowBounds::Windowed(self.bounds())
        }
    }

    fn content_size(&self) -> Size<Pixels> {
        self.borrow().bounds.size
    }

    fn scale_factor(&self) -> f32 {
        self.borrow().scale
    }

    fn appearance(&self) -> WindowAppearance {
        self.borrow().appearance
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.borrow();
        state.display.as_ref().map(|(id, display)| {
            Rc::new(WaylandDisplay {
                id: id.clone(),
                name: display.name.clone(),
                bounds: display.bounds.to_pixels(state.scale),
            }) as Rc<dyn PlatformDisplay>
        })
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.borrow()
            .client
            .get_client()
            .borrow()
            .mouse_location
            .unwrap_or_default()
    }

    fn modifiers(&self) -> Modifiers {
        self.borrow().client.get_client().borrow().modifiers
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> Option<Receiver<usize>> {
        None
    }

    fn activate(&self) {
        log::info!("Wayland does not support this API");
    }

    fn is_active(&self) -> bool {
        self.borrow().active
    }

    fn set_title(&mut self, title: &str) {
        self.borrow().toplevel.set_title(title.to_string());
    }

    fn set_app_id(&mut self, app_id: &str) {
        self.borrow().toplevel.set_app_id(app_id.to_owned());
    }

    fn set_background_appearance(&mut self, background_appearance: WindowBackgroundAppearance) {
        let opaque = background_appearance == WindowBackgroundAppearance::Opaque;
        let mut state = self.borrow_mut();
        state.renderer.update_transparency(!opaque);

        let region = state
            .globals
            .compositor
            .create_region(&state.globals.qh, ());
        region.add(0, 0, i32::MAX, i32::MAX);

        if opaque {
            // Promise the compositor that this region of the window surface
            // contains no transparent pixels. This allows the compositor to
            // do skip whatever is behind the surface for better performance.
            state.surface.set_opaque_region(Some(&region));
        } else {
            state.surface.set_opaque_region(None);
        }

        if let Some(ref blur_manager) = state.globals.blur_manager {
            if background_appearance == WindowBackgroundAppearance::Blurred {
                if state.blur.is_none() {
                    let blur = blur_manager.create(&state.surface, &state.globals.qh, ());
                    blur.set_region(Some(&region));
                    state.blur = Some(blur);
                }
                state.blur.as_ref().unwrap().commit();
            } else {
                // It probably doesn't hurt to clear the blur for opaque windows
                blur_manager.unset(&state.surface);
                if let Some(b) = state.blur.take() {
                    b.release()
                }
            }
        }

        region.destroy();
    }

    fn set_edited(&mut self, _edited: bool) {
        log::info!("ignoring macOS specific set_edited");
    }

    fn show_character_palette(&self) {
        log::info!("ignoring macOS specific show_character_palette");
    }

    fn minimize(&self) {
        self.borrow().toplevel.set_minimized();
    }

    fn zoom(&self) {
        let state = self.borrow();
        if !state.maximized {
            state.toplevel.set_maximized();
        } else {
            state.toplevel.unset_maximized();
        }
    }

    fn toggle_fullscreen(&self) {
        let mut state = self.borrow_mut();
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
        self.0.callbacks.borrow_mut().appearance_changed = Some(callback);
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

    fn show_window_menu(&self, position: Point<Pixels>) {
        let state = self.borrow();
        let serial = state.client.get_serial(SerialKind::MousePress);
        state.toplevel.show_window_menu(
            &state.globals.seat,
            serial,
            position.x.0 as i32,
            position.y.0 as i32,
        );
    }

    fn start_system_move(&self) {
        let state = self.borrow();
        let serial = state.client.get_serial(SerialKind::MousePress);
        state.toplevel._move(&state.globals.seat, serial);
    }

    fn should_render_window_controls(&self) -> bool {
        self.borrow().decoration_state == WaylandDecorationState::Client
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WaylandDecorationState {
    /// Decorations are to be provided by the client
    Client,

    /// Decorations are provided by the server
    Server,
}
