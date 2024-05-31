// todo(linux): remove
#![allow(unused)]

use crate::{
    platform::blade::{BladeRenderer, BladeSurfaceConfig},
    size, Bounds, DevicePixels, ForegroundExecutor, Modifiers, Pixels, Platform, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptLevel,
    Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowOptions,
    WindowParams, X11Client, X11ClientState, X11ClientStatePtr,
};

use blade_graphics as gpu;
use parking_lot::Mutex;
use raw_window_handle as rwh;
use util::ResultExt;
use x11rb::{
    connection::{Connection as _, RequestConnection as _},
    protocol::{
        render,
        xinput::{self, ConnectionExt as _},
        xproto::{
            self, Atom, ClientMessageEvent, ConnectionExt as _, CreateWindowAux, EventMask,
            TranslateCoordinatesReply,
        },
    },
    resource_manager::Database,
    wrapper::ConnectionExt as _,
    xcb_ffi::XCBConnection,
};

use std::ops::Deref;
use std::rc::Weak;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ffi::c_void,
    iter::Zip,
    mem,
    num::NonZeroU32,
    ops::Div,
    ptr::NonNull,
    rc::Rc,
    sync::{self, Arc},
};

use super::{X11Display, XINPUT_MASTER_DEVICE};

x11rb::atom_manager! {
    pub XcbAtoms: AtomsCookie {
        UTF8_STRING,
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        WM_CHANGE_STATE,
        _NET_WM_NAME,
        _NET_WM_STATE,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_STATE_MAXIMIZED_HORZ,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_FOCUSED,
        _NET_WM_MOVERESIZE,
        _GTK_SHOW_WINDOW_MENU,
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

#[derive(Debug)]
struct Visual {
    id: xproto::Visualid,
    colormap: u32,
    depth: u8,
}

struct VisualSet {
    inherit: Visual,
    opaque: Option<Visual>,
    transparent: Option<Visual>,
    root: u32,
    black_pixel: u32,
}

fn find_visuals(xcb_connection: &XCBConnection, screen_index: usize) -> VisualSet {
    let screen = &xcb_connection.setup().roots[screen_index];
    let mut set = VisualSet {
        inherit: Visual {
            id: screen.root_visual,
            colormap: screen.default_colormap,
            depth: screen.root_depth,
        },
        opaque: None,
        transparent: None,
        root: screen.root,
        black_pixel: screen.black_pixel,
    };

    for depth_info in screen.allowed_depths.iter() {
        for visual_type in depth_info.visuals.iter() {
            let visual = Visual {
                id: visual_type.visual_id,
                colormap: 0,
                depth: depth_info.depth,
            };
            log::debug!("Visual id: {}, class: {:?}, depth: {}, bits_per_value: {}, masks: 0x{:x} 0x{:x} 0x{:x}",
                visual_type.visual_id,
                visual_type.class,
                depth_info.depth,
                visual_type.bits_per_rgb_value,
                visual_type.red_mask, visual_type.green_mask, visual_type.blue_mask,
            );

            if (
                visual_type.red_mask,
                visual_type.green_mask,
                visual_type.blue_mask,
            ) != (0xFF0000, 0xFF00, 0xFF)
            {
                continue;
            }
            let color_mask = visual_type.red_mask | visual_type.green_mask | visual_type.blue_mask;
            let alpha_mask = color_mask as usize ^ ((1usize << depth_info.depth) - 1);

            if alpha_mask == 0 {
                if set.opaque.is_none() {
                    set.opaque = Some(visual);
                }
            } else {
                if set.transparent.is_none() {
                    set.transparent = Some(visual);
                }
            }
        }
    }

    set
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
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct X11WindowState {
    client: X11ClientStatePtr,
    executor: ForegroundExecutor,
    atoms: XcbAtoms,
    x_root_window: xproto::Window,
    raw: RawWindow,
    bounds: Bounds<i32>,
    scale_factor: f32,
    renderer: BladeRenderer,
    display: Rc<dyn PlatformDisplay>,
    input_handler: Option<PlatformInputHandler>,
    appearance: WindowAppearance,
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
        let mut handle = rwh::XcbWindowHandle::new(non_zero);
        handle.visual_id = NonZeroU32::new(self.visual_id);
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
        scale_factor: f32,
        appearance: WindowAppearance,
    ) -> Self {
        let x_screen_index = params
            .display_id
            .map_or(x_main_screen_index, |did| did.0 as usize);

        let visual_set = find_visuals(&xcb_connection, x_screen_index);
        let visual_maybe = match params.window_background {
            WindowBackgroundAppearance::Opaque => visual_set.opaque,
            WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred => {
                visual_set.transparent
            }
        };
        let visual = match visual_maybe {
            Some(visual) => visual,
            None => {
                log::warn!(
                    "Unable to find a matching visual for {:?}",
                    params.window_background
                );
                visual_set.inherit
            }
        };
        log::info!("Using {:?}", visual);

        let colormap = if visual.colormap != 0 {
            visual.colormap
        } else {
            let id = xcb_connection.generate_id().unwrap();
            log::info!("Creating colormap {}", id);
            xcb_connection
                .create_colormap(xproto::ColormapAlloc::NONE, id, visual_set.root, visual.id)
                .unwrap()
                .check()
                .unwrap();
            id
        };

        let win_aux = xproto::CreateWindowAux::new()
            .background_pixel(x11rb::NONE)
            // https://stackoverflow.com/questions/43218127/x11-xlib-xcb-creating-a-window-requires-border-pixel-if-specifying-colormap-wh
            .border_pixel(visual_set.black_pixel)
            .colormap(colormap)
            .event_mask(
                xproto::EventMask::EXPOSURE
                    | xproto::EventMask::STRUCTURE_NOTIFY
                    | xproto::EventMask::ENTER_WINDOW
                    | xproto::EventMask::LEAVE_WINDOW
                    | xproto::EventMask::FOCUS_CHANGE
                    | xproto::EventMask::KEY_PRESS
                    | xproto::EventMask::KEY_RELEASE,
            );

        xcb_connection
            .create_window(
                visual.depth,
                x_window,
                visual_set.root,
                params.bounds.origin.x.0 as i16,
                params.bounds.origin.y.0 as i16,
                params.bounds.size.width.0 as u16,
                params.bounds.size.height.0 as u16,
                0,
                xproto::WindowClass::INPUT_OUTPUT,
                visual.id,
                &win_aux,
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

        xcb_connection
            .xinput_xi_select_events(
                x_window,
                &[xinput::EventMask {
                    deviceid: XINPUT_MASTER_DEVICE,
                    mask: vec![
                        xinput::XIEventMask::MOTION
                            | xinput::XIEventMask::BUTTON_PRESS
                            | xinput::XIEventMask::BUTTON_RELEASE
                            | xinput::XIEventMask::LEAVE,
                    ],
                }],
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
            visual_id: visual.id,
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

        let config = BladeSurfaceConfig {
            // Note: this has to be done after the GPU init, or otherwise
            // the sizes are immediately invalidated.
            size: query_render_extent(xcb_connection, x_window),
            transparent: params.window_background != WindowBackgroundAppearance::Opaque,
        };

        Self {
            client,
            executor,
            display: Rc::new(X11Display::new(xcb_connection, x_screen_index).unwrap()),
            raw,
            x_root_window: visual_set.root,
            bounds: params.bounds.map(|v| v.0),
            scale_factor,
            renderer: BladeRenderer::new(gpu, config),
            atoms: *atoms,
            input_handler: None,
            appearance,
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

enum WmHintPropertyState {
    Remove = 0,
    Add = 1,
    Toggle = 2,
}

impl X11Window {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
        scale_factor: f32,
        appearance: WindowAppearance,
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
                scale_factor,
                appearance,
            ))),
            callbacks: Rc::new(RefCell::new(Callbacks::default())),
            xcb_connection: xcb_connection.clone(),
            x_window,
        })
    }

    fn set_wm_hints(&self, wm_hint_property_state: WmHintPropertyState, prop1: u32, prop2: u32) {
        let state = self.0.state.borrow();
        let message = ClientMessageEvent::new(
            32,
            self.0.x_window,
            state.atoms._NET_WM_STATE,
            [wm_hint_property_state as u32, prop1, prop2, 1, 0],
        );
        self.0
            .xcb_connection
            .send_event(
                false,
                state.x_root_window,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                message,
            )
            .unwrap();
    }

    fn get_wm_hints(&self) -> Vec<u32> {
        let reply = self
            .0
            .xcb_connection
            .get_property(
                false,
                self.0.x_window,
                self.0.state.borrow().atoms._NET_WM_STATE,
                xproto::AtomEnum::ATOM,
                0,
                u32::MAX,
            )
            .unwrap()
            .reply()
            .unwrap();
        // Reply is in u8 but atoms are represented as u32
        reply
            .value
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }

    fn get_root_position(&self, position: Point<Pixels>) -> TranslateCoordinatesReply {
        let state = self.0.state.borrow();
        self.0
            .xcb_connection
            .translate_coordinates(
                self.0.x_window,
                state.x_root_window,
                (position.x.0 * state.scale_factor) as i16,
                (position.y.0 * state.scale_factor) as i16,
            )
            .unwrap()
            .reply()
            .unwrap()
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

    pub fn handle_ime_commit(&self, text: String) {
        let mut state = self.state.borrow_mut();
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            input_handler.replace_text_in_range(None, &text);
            let mut state = self.state.borrow_mut();
            state.input_handler = Some(input_handler);
        }
    }

    pub fn handle_ime_preedit(&self, text: String) {
        let mut state = self.state.borrow_mut();
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            input_handler.replace_and_mark_text_in_range(None, &text, None);
            let mut state = self.state.borrow_mut();
            state.input_handler = Some(input_handler);
        }
    }

    pub fn handle_ime_unmark(&self) {
        let mut state = self.state.borrow_mut();
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            input_handler.unmark_text();
            let mut state = self.state.borrow_mut();
            state.input_handler = Some(input_handler);
        }
    }

    pub fn handle_ime_delete(&self) {
        let mut state = self.state.borrow_mut();
        if let Some(mut input_handler) = state.input_handler.take() {
            drop(state);
            if let Some(marked) = input_handler.marked_text_range() {
                input_handler.replace_text_in_range(Some(marked), "");
            }
            let mut state = self.state.borrow_mut();
            state.input_handler = Some(input_handler);
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
            let mut state = self.state.borrow_mut();
            state.input_handler = Some(input_handler);
        };
        bounds
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

    pub fn set_appearance(&mut self, appearance: WindowAppearance) {
        self.state.borrow_mut().appearance = appearance;

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = callbacks.appearance_changed {
            (fun)()
        }
    }
}

impl PlatformWindow for X11Window {
    fn bounds(&self) -> Bounds<DevicePixels> {
        self.0.state.borrow().bounds.map(|v| v.into())
    }

    fn is_maximized(&self) -> bool {
        let state = self.0.state.borrow();
        let wm_hints = self.get_wm_hints();
        // A maximized window that gets minimized will still retain its maximized state.
        !wm_hints.contains(&state.atoms._NET_WM_STATE_HIDDEN)
            && wm_hints.contains(&state.atoms._NET_WM_STATE_MAXIMIZED_VERT)
            && wm_hints.contains(&state.atoms._NET_WM_STATE_MAXIMIZED_HORZ)
    }

    fn window_bounds(&self) -> WindowBounds {
        let state = self.0.state.borrow();
        WindowBounds::Windowed(state.bounds.map(|p| DevicePixels(p)))
    }

    fn content_size(&self) -> Size<Pixels> {
        // We divide by the scale factor here because this value is queried to determine how much to draw,
        // but it will be multiplied later by the scale to adjust for scaling.
        let state = self.0.state.borrow();
        state
            .content_size()
            .map(|size| size.div(state.scale_factor))
    }

    fn scale_factor(&self) -> f32 {
        self.0.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.0.state.borrow().appearance
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

    fn is_active(&self) -> bool {
        let state = self.0.state.borrow();
        self.get_wm_hints()
            .contains(&state.atoms._NET_WM_STATE_FOCUSED)
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

        self.0
            .xcb_connection
            .change_property8(
                xproto::PropMode::REPLACE,
                self.0.x_window,
                xproto::AtomEnum::WM_CLASS,
                xproto::AtomEnum::STRING,
                &data,
            )
            .unwrap();
    }

    // todo(linux)
    fn set_edited(&mut self, edited: bool) {}

    fn set_background_appearance(&mut self, background_appearance: WindowBackgroundAppearance) {
        let mut inner = self.0.state.borrow_mut();
        let transparent = background_appearance != WindowBackgroundAppearance::Opaque;
        inner.renderer.update_transparency(transparent);
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

    fn minimize(&self) {
        let state = self.0.state.borrow();
        const WINDOW_ICONIC_STATE: u32 = 3;
        let message = ClientMessageEvent::new(
            32,
            self.0.x_window,
            state.atoms.WM_CHANGE_STATE,
            [WINDOW_ICONIC_STATE, 0, 0, 0, 0],
        );
        self.0
            .xcb_connection
            .send_event(
                false,
                state.x_root_window,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                message,
            )
            .unwrap();
    }

    fn zoom(&self) {
        let state = self.0.state.borrow();
        self.set_wm_hints(
            WmHintPropertyState::Toggle,
            state.atoms._NET_WM_STATE_MAXIMIZED_VERT,
            state.atoms._NET_WM_STATE_MAXIMIZED_HORZ,
        );
    }

    fn toggle_fullscreen(&self) {
        let state = self.0.state.borrow();
        self.set_wm_hints(
            WmHintPropertyState::Toggle,
            state.atoms._NET_WM_STATE_FULLSCREEN,
            xproto::AtomEnum::NONE.into(),
        );
    }

    fn is_fullscreen(&self) -> bool {
        let state = self.0.state.borrow();
        self.get_wm_hints()
            .contains(&state.atoms._NET_WM_STATE_FULLSCREEN)
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
        let mut inner = self.0.state.borrow_mut();
        inner.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn PlatformAtlas> {
        let inner = self.0.state.borrow();
        inner.renderer.sprite_atlas().clone()
    }

    fn show_window_menu(&self, position: Point<Pixels>) {
        let state = self.0.state.borrow();
        let coords = self.get_root_position(position);
        let message = ClientMessageEvent::new(
            32,
            self.0.x_window,
            state.atoms._GTK_SHOW_WINDOW_MENU,
            [
                XINPUT_MASTER_DEVICE as u32,
                coords.dst_x as u32,
                coords.dst_y as u32,
                0,
                0,
            ],
        );
        self.0
            .xcb_connection
            .send_event(
                false,
                state.x_root_window,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                message,
            )
            .unwrap();
    }

    fn start_system_move(&self) {
        let state = self.0.state.borrow();
        let pointer = self
            .0
            .xcb_connection
            .query_pointer(self.0.x_window)
            .unwrap()
            .reply()
            .unwrap();
        const MOVERESIZE_MOVE: u32 = 8;
        let message = ClientMessageEvent::new(
            32,
            self.0.x_window,
            state.atoms._NET_WM_MOVERESIZE,
            [
                pointer.root_x as u32,
                pointer.root_y as u32,
                MOVERESIZE_MOVE,
                1, // Left mouse button
                1,
            ],
        );
        self.0
            .xcb_connection
            .send_event(
                false,
                state.x_root_window,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                message,
            )
            .unwrap();
    }

    fn should_render_window_controls(&self) -> bool {
        false
    }
}
