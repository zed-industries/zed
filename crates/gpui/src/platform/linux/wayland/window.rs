use std::any::Any;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::Arc;

use blade_graphics as gpu;
use blade_rwh::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use futures::channel::oneshot::Receiver;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wayland_client::{protocol::wl_surface, Proxy};
use wayland_protocols::wp::viewporter::client::wp_viewport;
use wayland_protocols::xdg::shell::client::xdg_toplevel;

use crate::platform::blade::BladeRenderer;
use crate::platform::linux::wayland::display::WaylandDisplay;
use crate::platform::{PlatformAtlas, PlatformInputHandler, PlatformWindow};
use crate::scene::Scene;
use crate::{
    px, size, Bounds, Modifiers, Pixels, PlatformDisplay, PlatformInput, Point, PromptLevel, Size,
    WindowAppearance, WindowBounds, WindowOptions,
};

#[derive(Default)]
pub(crate) struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> bool>>,
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
    bounds: Bounds<i32>,
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
    fn new(wl_surf: &Arc<wl_surface::WlSurface>, bounds: Bounds<i32>) -> Self {
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
                    },
                )
            }
            .unwrap(),
        );
        let extent = gpu::Extent {
            width: bounds.size.width as u32,
            height: bounds.size.height as u32,
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
    viewport: Option<wp_viewport::WpViewport>,
}

impl WaylandWindowState {
    pub(crate) fn new(
        wl_surf: Arc<wl_surface::WlSurface>,
        viewport: Option<wp_viewport::WpViewport>,
        toplevel: Arc<xdg_toplevel::XdgToplevel>,
        options: WindowOptions,
    ) -> Self {
        if options.bounds == WindowBounds::Maximized {
            toplevel.set_maximized();
        } else if options.bounds == WindowBounds::Fullscreen {
            toplevel.set_fullscreen(None);
        }

        let bounds: Bounds<i32> = match options.bounds {
            WindowBounds::Fullscreen | WindowBounds::Maximized => Bounds {
                origin: Point::default(),
                size: Size {
                    width: 500,
                    height: 500,
                }, //todo!(implement)
            },
            WindowBounds::Fixed(bounds) => bounds.map(|p| p.0 as i32),
        };

        Self {
            surface: Arc::clone(&wl_surf),
            inner: RefCell::new(WaylandWindowInner::new(&wl_surf, bounds)),
            callbacks: RefCell::new(Callbacks::default()),
            toplevel,
            viewport,
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

    pub fn set_size_and_scale(&self, width: i32, height: i32, scale: f32) {
        self.inner.borrow_mut().scale = scale;
        self.inner.borrow_mut().bounds.size.width = width;
        self.inner.borrow_mut().bounds.size.height = height;
        self.inner.borrow_mut().renderer.update_drawable_size(size(
            width as f64 * scale as f64,
            height as f64 * scale as f64,
        ));

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
            viewport.set_destination(width, height);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        let scale = self.inner.borrow_mut().scale;
        self.set_size_and_scale(width, height, scale);
    }

    pub fn rescale(&self, scale: f32) {
        let bounds = self.inner.borrow_mut().bounds;
        self.set_size_and_scale(bounds.size.width, bounds.size.height, scale)
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
        // todo!(linux) - Handle this properly
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
            if fun(input.clone()) {
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
    //todo!(linux)
    fn bounds(&self) -> WindowBounds {
        WindowBounds::Maximized
    }

    fn content_size(&self) -> Size<Pixels> {
        let inner = self.0.inner.borrow_mut();
        Size {
            width: Pixels(inner.bounds.size.width as f32),
            height: Pixels(inner.bounds.size.height as f32),
        }
    }

    fn scale_factor(&self) -> f32 {
        self.0.inner.borrow_mut().scale
    }

    //todo!(linux)
    fn titlebar_height(&self) -> Pixels {
        unimplemented!()
    }

    // todo!(linux)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    // todo!(linux)
    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::new(WaylandDisplay {})
    }

    // todo!(linux)
    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    //todo!(linux)
    fn modifiers(&self) -> Modifiers {
        crate::Modifiers::default()
    }

    //todo!(linux)
    fn as_any_mut(&mut self) -> &mut dyn Any {
        unimplemented!()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.inner.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.inner.borrow_mut().input_handler.take()
    }

    //todo!(linux)
    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {
        //todo!(linux)
    }

    fn set_title(&mut self, title: &str) {
        self.0.toplevel.set_title(title.to_string());
    }

    fn set_edited(&mut self, edited: bool) {
        //todo!(linux)
    }

    fn show_character_palette(&self) {
        //todo!(linux)
    }

    fn minimize(&self) {
        //todo!(linux)
    }

    fn zoom(&self) {
        //todo!(linux)
    }

    fn toggle_full_screen(&self) {
        //todo!(linux)
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.0.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        //todo!(linux)
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
        //todo!(linux)
    }

    // todo!(linux)
    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool {
        false
    }

    fn draw(&self, scene: &Scene) {
        let mut inner = self.0.inner.borrow_mut();
        inner.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        let inner = self.0.inner.borrow_mut();
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
