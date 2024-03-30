use std::any::Any;
use std::cell::RefCell;
use std::ffi::c_void;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;

use blade_graphics as gpu;
use blade_rwh::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use collections::HashSet;
use futures::channel::oneshot::Receiver;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wayland_backend::client::ObjectId;
use wayland_client::{protocol::wl_surface, Proxy};
use wayland_protocols::wp::viewporter::client::wp_viewport;
use wayland_protocols::xdg::shell::client::xdg_toplevel;

use crate::platform::blade::BladeRenderer;
use crate::platform::linux::wayland::display::WaylandDisplay;
use crate::platform::{PlatformAtlas, PlatformInputHandler, PlatformWindow};
use crate::scene::Scene;
use crate::{
    px, size, Bounds, DevicePixels, Modifiers, Pixels, PlatformDisplay, PlatformInput, Point,
    PromptLevel, Size, WindowAppearance, WindowBackgroundAppearance, WindowParams,
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

struct WaylandWindowInner {
    renderer: BladeRenderer,
    bounds: Bounds<u32>,
    scale: f32,
    input_handler: Option<PlatformInputHandler>,
    decoration_state: WaylandDecorationState,
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

impl WaylandWindowInner {
    fn new(wl_surf: &Arc<wl_surface::WlSurface>, bounds: Bounds<u32>) -> Self {
        let raw = RawWindow {
            window: wl_surf.id().as_ptr().cast::<c_void>(),
            display: wl_surf
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
            renderer: BladeRenderer::new(gpu, extent),
            bounds,
            scale: 1.0,
            input_handler: None,

            // On wayland, decorations are by default provided by the client
            decoration_state: WaylandDecorationState::Client,
        }
    }
}

pub(crate) struct WaylandWindowState {
    inner: RefCell<WaylandWindowInner>,
    pub(crate) callbacks: RefCell<Callbacks>,
    pub(crate) surface: Arc<wl_surface::WlSurface>,
    pub(crate) toplevel: Arc<xdg_toplevel::XdgToplevel>,
    pub(crate) outputs: RefCell<HashSet<ObjectId>>,
    viewport: Option<wp_viewport::WpViewport>,
    fullscreen: RefCell<bool>,
}

impl WaylandWindowState {
    pub(crate) fn new(
        wl_surf: Arc<wl_surface::WlSurface>,
        viewport: Option<wp_viewport::WpViewport>,
        toplevel: Arc<xdg_toplevel::XdgToplevel>,
        options: WindowParams,
    ) -> Self {
        let bounds = options.bounds.map(|p| p.0 as u32);

        Self {
            surface: Arc::clone(&wl_surf),
            inner: RefCell::new(WaylandWindowInner::new(&wl_surf, bounds)),
            callbacks: RefCell::new(Callbacks::default()),
            outputs: RefCell::new(HashSet::default()),
            toplevel,
            viewport,
            fullscreen: RefCell::new(false),
        }
    }

    pub fn update(&self) {
        let mut cb = self.callbacks.borrow_mut();
        if let Some(mut fun) = cb.request_frame.take() {
            drop(cb);
            fun();
            self.callbacks.borrow_mut().request_frame = Some(fun);
        }
    }

    pub fn set_size_and_scale(
        &self,
        width: Option<NonZeroU32>,
        height: Option<NonZeroU32>,
        scale: Option<f32>,
    ) {
        let (width, height, scale) = {
            let mut inner = self.inner.borrow_mut();
            if width.map_or(true, |width| width.get() == inner.bounds.size.width)
                && height.map_or(true, |height| height.get() == inner.bounds.size.height)
                && scale.map_or(true, |scale| scale == inner.scale)
            {
                return;
            }
            if let Some(width) = width {
                inner.bounds.size.width = width.get();
            }
            if let Some(height) = height {
                inner.bounds.size.height = height.get();
            }
            if let Some(scale) = scale {
                inner.scale = scale;
            }
            let width = inner.bounds.size.width;
            let height = inner.bounds.size.height;
            let scale = inner.scale;
            inner.renderer.update_drawable_size(size(
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

        if let Some(viewport) = &self.viewport {
            viewport.set_destination(width as i32, height as i32);
        }
    }

    pub fn resize(&self, width: Option<NonZeroU32>, height: Option<NonZeroU32>) {
        self.set_size_and_scale(width, height, None);
    }

    pub fn rescale(&self, scale: f32) {
        self.set_size_and_scale(None, None, Some(scale));
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = callbacks.fullscreen {
            fun(fullscreen)
        }
        self.fullscreen.replace(fullscreen);
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
        self.inner.borrow_mut().decoration_state = state;
        log::trace!("Window decorations are now handled by {:?}", state);
        // todo(linux) - Handle this properly
    }

    pub fn close(&self) {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(fun) = callbacks.close.take() {
            fun()
        }
        self.toplevel.destroy();
    }

    pub fn handle_input(&self, input: PlatformInput) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().input {
            if !fun(input.clone()).propagate {
                return;
            }
        }
        if let PlatformInput::KeyDown(event) = input {
            let mut inner = self.inner.borrow_mut();
            if let Some(ref mut input_handler) = inner.input_handler {
                if let Some(ime_key) = &event.keystroke.ime_key {
                    input_handler.replace_text_in_range(None, ime_key);
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

#[derive(Clone)]
pub(crate) struct WaylandWindow(pub(crate) Rc<WaylandWindowState>);

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
    // todo(linux)
    fn bounds(&self) -> Bounds<DevicePixels> {
        unimplemented!()
    }

    // todo(linux)
    fn is_maximized(&self) -> bool {
        false
    }

    // todo(linux)
    fn is_minimized(&self) -> bool {
        false
    }

    fn content_size(&self) -> Size<Pixels> {
        let inner = self.0.inner.borrow();
        Size {
            width: Pixels(inner.bounds.size.width as f32),
            height: Pixels(inner.bounds.size.height as f32),
        }
    }

    fn scale_factor(&self) -> f32 {
        self.0.inner.borrow().scale
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
        self.0.inner.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.inner.borrow_mut().input_handler.take()
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
        self.0.toplevel.set_title(title.to_string());
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
        self.0.toplevel.set_minimized();
    }

    fn zoom(&self) {
        // todo(linux)
    }

    fn toggle_fullscreen(&self) {
        if !(*self.0.fullscreen.borrow()) {
            self.0.toplevel.set_fullscreen(None);
        } else {
            self.0.toplevel.unset_fullscreen();
        }
    }

    fn is_fullscreen(&self) -> bool {
        *self.0.fullscreen.borrow()
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
        self.0.inner.borrow_mut().renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        let inner = self.0.inner.borrow();
        inner.renderer.sprite_atlas().clone()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WaylandDecorationState {
    /// Decorations are to be provided by the client
    Client,

    /// Decorations are provided by the server
    Server,
}
