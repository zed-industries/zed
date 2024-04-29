// todo(linux): remove
#![allow(unused)]

use crate::{
    platform::blade::BladeRenderer, size, Bounds, DevicePixels, ForegroundExecutor, Modifiers,
    Pixels, Platform, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, PromptLevel, Scene, Size, WindowAppearance, WindowBackgroundAppearance,
    WindowOptions, WindowParams, X11Client, X11ClientState, X11ClientStatePtr,
};
use blade_graphics as gpu;
use parking_lot::Mutex;
use raw_window_handle as rwh;
use util::ResultExt;
use x11rb::{
    connection::Connection,
    protocol::{
        xinput,
        xproto::{self, ConnectionExt as _, CreateWindowAux},
    },
    wrapper::ConnectionExt,
    xcb_ffi::XCBConnection,
};

use std::{
    cell::{Ref, RefCell, RefMut},
    ffi::c_void,
    iter::Zip,
    mem,
    num::NonZeroU32,
    ptr::NonNull,
    rc::Rc,
    sync::{self, Arc},
};

use super::X11Display;

x11rb::atom_manager! {
    pub XcbAtoms: AtomsCookie {
        UTF8_STRING,
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        _NET_WM_STATE,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_STATE_MAXIMIZED_HORZ,
    }
}

fn query_render_extent(xcb_connection: &XCBConnection, x_window: xproto::Window) -> gpu::Extent {
    let reply = xcb_connection
        .get_geometry(x_window)
        .unwrap()
        .reply()
        .unwrap();
    gpu::Extent {
        width: reply.width as u32,
        height: reply.height as u32,
        depth: 1,
    }
}

struct RawWindow {
    connection: *mut c_void,
    screen_id: usize,
    window_id: u32,
    visual_id: u32,
}

#[derive(Default)]
pub struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> crate::DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct X11WindowState {
    client: X11ClientStatePtr,
    executor: ForegroundExecutor,
    atoms: XcbAtoms,
    raw: RawWindow,
    bounds: Bounds<i32>,
    scale_factor: f32,
    renderer: BladeRenderer,
    display: Rc<dyn PlatformDisplay>,

    input_handler: Option<PlatformInputHandler>,
}

#[derive(Clone)]
pub(crate) struct X11WindowStatePtr {
    pub(crate) state: Rc<RefCell<X11WindowState>>,
    pub(crate) callbacks: Rc<RefCell<Callbacks>>,
    xcb_connection: Rc<XCBConnection>,
    x_window: xproto::Window,
}

// todo(linux): Remove other RawWindowHandle implementation
impl rwh::HasWindowHandle for RawWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle, rwh::HandleError> {
        let non_zero = NonZeroU32::new(self.window_id).unwrap();
        let handle = rwh::XcbWindowHandle::new(non_zero);
        Ok(unsafe { rwh::WindowHandle::borrow_raw(handle.into()) })
    }
}
impl rwh::HasDisplayHandle for RawWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle, rwh::HandleError> {
        let non_zero = NonNull::new(self.connection).unwrap();
        let handle = rwh::XcbDisplayHandle::new(Some(non_zero), self.screen_id as i32);
        Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
    }
}

impl rwh::HasWindowHandle for X11Window {
    fn window_handle(&self) -> Result<rwh::WindowHandle, rwh::HandleError> {
        unimplemented!()
    }
}
impl rwh::HasDisplayHandle for X11Window {
    fn display_handle(&self) -> Result<rwh::DisplayHandle, rwh::HandleError> {
        unimplemented!()
    }
}

impl X11WindowState {
    pub fn new(
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
    ) -> Self {
        let x_screen_index = params
            .display_id
            .map_or(x_main_screen_index, |did| did.0 as usize);
        let screen = xcb_connection.setup().roots.get(x_screen_index).unwrap();

        let win_aux = xproto::CreateWindowAux::new().event_mask(
            xproto::EventMask::EXPOSURE
                | xproto::EventMask::STRUCTURE_NOTIFY
                | xproto::EventMask::ENTER_WINDOW
                | xproto::EventMask::LEAVE_WINDOW
                | xproto::EventMask::FOCUS_CHANGE
                | xproto::EventMask::KEY_PRESS
                | xproto::EventMask::KEY_RELEASE
                | xproto::EventMask::BUTTON_PRESS
                | xproto::EventMask::BUTTON_RELEASE
                | xproto::EventMask::POINTER_MOTION
                | xproto::EventMask::BUTTON1_MOTION
                | xproto::EventMask::BUTTON2_MOTION
                | xproto::EventMask::BUTTON3_MOTION
                | xproto::EventMask::BUTTON_MOTION,
        );

        xcb_connection
            .create_window(
                x11rb::COPY_FROM_PARENT as _,
                x_window,
                screen.root,
                params.bounds.origin.x.0 as i16,
                params.bounds.origin.y.0 as i16,
                params.bounds.size.width.0 as u16,
                params.bounds.size.height.0 as u16,
                0,
                xproto::WindowClass::INPUT_OUTPUT,
                screen.root_visual,
                &win_aux,
            )
            .unwrap();

        xinput::ConnectionExt::xinput_xi_select_events(
            &xcb_connection,
            x_window,
            &[xinput::EventMask {
                deviceid: 1,
                mask: vec![xinput::XIEventMask::MOTION],
            }],
        )
        .unwrap()
        .check()
        .unwrap();

        if let Some(titlebar) = params.titlebar {
            if let Some(title) = titlebar.title {
                xcb_connection
                    .change_property8(
                        xproto::PropMode::REPLACE,
                        x_window,
                        xproto::AtomEnum::WM_NAME,
                        xproto::AtomEnum::STRING,
                        title.as_bytes(),
                    )
                    .unwrap();
            }
        }

        xcb_connection
            .change_property32(
                xproto::PropMode::REPLACE,
                x_window,
                atoms.WM_PROTOCOLS,
                xproto::AtomEnum::ATOM,
                &[atoms.WM_DELETE_WINDOW],
            )
            .unwrap();

        xcb_connection.map_window(x_window).unwrap();
        xcb_connection.flush().unwrap();

        let raw = RawWindow {
            connection: as_raw_xcb_connection::AsRawXcbConnection::as_raw_xcb_connection(
                xcb_connection,
            ) as *mut _,
            screen_id: x_screen_index,
            window_id: x_window,
            visual_id: screen.root_visual,
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

        // Note: this has to be done after the GPU init, or otherwise
        // the sizes are immediately invalidated.
        let gpu_extent = query_render_extent(xcb_connection, x_window);

        Self {
            client,
            executor,
            display: Rc::new(X11Display::new(xcb_connection, x_screen_index).unwrap()),
            raw,
            bounds: params.bounds.map(|v| v.0),
            scale_factor: 1.0,
            renderer: BladeRenderer::new(gpu, gpu_extent),
            atoms: *atoms,

            input_handler: None,
        }
    }

    fn content_size(&self) -> Size<Pixels> {
        let size = self.renderer.viewport_size();
        Size {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

pub(crate) struct X11Window(pub X11WindowStatePtr);

impl Drop for X11Window {
    fn drop(&mut self) {
        let mut state = self.0.state.borrow_mut();
        state.renderer.destroy();

        self.0.xcb_connection.unmap_window(self.0.x_window).unwrap();
        self.0
            .xcb_connection
            .destroy_window(self.0.x_window)
            .unwrap();
        self.0.xcb_connection.flush().unwrap();

        let this_ptr = self.0.clone();
        let client_ptr = state.client.clone();
        state
            .executor
            .spawn(async move {
                this_ptr.close();
                client_ptr.drop_window(this_ptr.x_window);
            })
            .detach();
        drop(state);
    }
}

impl X11Window {
    pub fn new(
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
    ) -> Self {
        Self(X11WindowStatePtr {
            state: Rc::new(RefCell::new(X11WindowState::new(
                client,
                executor,
                params,
                xcb_connection,
                x_main_screen_index,
                x_window,
                atoms,
            ))),
            callbacks: Rc::new(RefCell::new(Callbacks::default())),
            xcb_connection: xcb_connection.clone(),
            x_window,
        })
    }
}

impl X11WindowStatePtr {
    pub fn should_close(&self) -> bool {
        let mut cb = self.callbacks.borrow_mut();
        if let Some(mut should_close) = cb.should_close.take() {
            let result = (should_close)();
            cb.should_close = Some(should_close);
            result
        } else {
            true
        }
    }

    pub fn close(&self) {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(fun) = callbacks.close.take() {
            fun()
        }
    }

    pub fn refresh(&self) {
        let mut cb = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = cb.request_frame {
            fun();
        }
    }

    pub fn handle_input(&self, input: PlatformInput) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().input {
            if !fun(input.clone()).propagate {
                return;
            }
        }
        if let PlatformInput::KeyDown(event) = input {
            let mut state = self.state.borrow_mut();
            if let Some(mut input_handler) = state.input_handler.take() {
                if let Some(ime_key) = &event.keystroke.ime_key {
                    drop(state);
                    input_handler.replace_text_in_range(None, ime_key);
                    state = self.state.borrow_mut();
                }
                state.input_handler = Some(input_handler);
            }
        }
    }

    pub fn configure(&self, bounds: Bounds<i32>) {
        let mut resize_args = None;
        let do_move;
        {
            let mut state = self.state.borrow_mut();
            let old_bounds = mem::replace(&mut state.bounds, bounds);
            do_move = old_bounds.origin != bounds.origin;
            // todo(linux): use normal GPUI types here, refactor out the double
            // viewport check and extra casts ( )
            let gpu_size = query_render_extent(&self.xcb_connection, self.x_window);
            if state.renderer.viewport_size() != gpu_size {
                state
                    .renderer
                    .update_drawable_size(size(gpu_size.width as f64, gpu_size.height as f64));
                resize_args = Some((state.content_size(), state.scale_factor));
            }
        }

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some((content_size, scale_factor)) = resize_args {
            if let Some(ref mut fun) = callbacks.resize {
                fun(content_size, scale_factor)
            }
        }
        if do_move {
            if let Some(ref mut fun) = callbacks.moved {
                fun()
            }
        }
    }

    pub fn set_focused(&self, focus: bool) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().active_status_change {
            fun(focus);
        }
    }
}

impl PlatformWindow for X11Window {
    fn bounds(&self) -> Bounds<DevicePixels> {
        self.0.state.borrow_mut().bounds.map(|v| v.into())
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
        self.0.state.borrow_mut().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.state.borrow_mut().scale_factor
    }

    // todo(linux)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        self.0.state.borrow().display.clone()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let reply = self
            .0
            .xcb_connection
            .query_pointer(self.0.x_window)
            .unwrap()
            .reply()
            .unwrap();
        Point::new((reply.root_x as u32).into(), (reply.root_y as u32).into())
    }

    // todo(linux)
    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.state.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {
        let win_aux = xproto::ConfigureWindowAux::new().stack_mode(xproto::StackMode::ABOVE);
        self.0
            .xcb_connection
            .configure_window(self.0.x_window, &win_aux)
            .log_err();
    }

    // todo(linux)
    fn is_active(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: &str) {
        self.0
            .xcb_connection
            .change_property8(
                xproto::PropMode::REPLACE,
                self.0.x_window,
                xproto::AtomEnum::WM_NAME,
                xproto::AtomEnum::STRING,
                title.as_bytes(),
            )
            .unwrap();

        self.0
            .xcb_connection
            .change_property8(
                xproto::PropMode::REPLACE,
                self.0.x_window,
                self.0.state.borrow().atoms._NET_WM_NAME,
                self.0.state.borrow().atoms.UTF8_STRING,
                title.as_bytes(),
            )
            .unwrap();
    }

    fn set_app_id(&mut self, app_id: &str) {
        let mut data = Vec::with_capacity(app_id.len() * 2 + 1);
        data.extend(app_id.bytes()); // instance https://unix.stackexchange.com/a/494170
        data.push(b'\0');
        data.extend(app_id.bytes()); // class

        self.0.xcb_connection.change_property8(
            xproto::PropMode::REPLACE,
            self.0.x_window,
            xproto::AtomEnum::WM_CLASS,
            xproto::AtomEnum::STRING,
            &data,
        );
    }

    // todo(linux)
    fn set_edited(&mut self, edited: bool) {}

    fn set_background_appearance(&mut self, _background_appearance: WindowBackgroundAppearance) {
        // todo(linux)
    }

    // todo(linux), this corresponds to `orderFrontCharacterPalette` on macOS,
    // but it looks like the equivalent for Linux is GTK specific:
    //
    // https://docs.gtk.org/gtk3/signal.Entry.insert-emoji.html
    //
    // This API might need to change, or we might need to build an emoji picker into GPUI
    fn show_character_palette(&self) {
        unimplemented!()
    }

    // todo(linux)
    fn minimize(&self) {
        unimplemented!()
    }

    // todo(linux)
    fn zoom(&self) {
        unimplemented!()
    }

    // todo(linux)
    fn toggle_fullscreen(&self) {
        unimplemented!()
    }

    // todo(linux)
    fn is_fullscreen(&self) -> bool {
        false
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
        self.0.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    // todo(linux)
    fn is_topmost_for_position(&self, _position: Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn draw(&self, scene: &Scene) {
        let mut inner = self.0.state.borrow_mut();
        inner.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn PlatformAtlas> {
        let inner = self.0.state.borrow();
        inner.renderer.sprite_atlas().clone()
    }
}
