use anyhow::Context;

use crate::{
    platform::blade::{BladeRenderer, BladeSurfaceConfig},
    px, size, AnyWindowHandle, Bounds, Decorations, DevicePixels, ForegroundExecutor, Modifiers,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptLevel, ResizeEdge, Scene, Size, Tiling, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowKind, WindowParams,
    X11ClientStatePtr,
};

use blade_graphics as gpu;
use raw_window_handle as rwh;
use util::{maybe, ResultExt};
use x11rb::{
    connection::Connection,
    protocol::{
        randr::{self, ConnectionExt as _},
        sync,
        xinput::{self, ConnectionExt as _},
        xproto::{self, ClientMessageEvent, ConnectionExt, EventMask, TranslateCoordinatesReply},
    },
    wrapper::ConnectionExt as _,
    xcb_ffi::XCBConnection,
};

use std::{
    cell::RefCell, ffi::c_void, mem::size_of, num::NonZeroU32, ops::Div, ptr::NonNull, rc::Rc,
    sync::Arc, time::Duration,
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
        _NET_ACTIVE_WINDOW,
        _NET_WM_SYNC_REQUEST,
        _NET_WM_SYNC_REQUEST_COUNTER,
        _NET_WM_BYPASS_COMPOSITOR,
        _NET_WM_MOVERESIZE,
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_NOTIFICATION,
        _NET_WM_SYNC,
        _MOTIF_WM_HINTS,
        _GTK_SHOW_WINDOW_MENU,
        _GTK_FRAME_EXTENTS,
        _GTK_EDGE_CONSTRAINTS,
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

impl ResizeEdge {
    fn to_moveresize(&self) -> u32 {
        match self {
            ResizeEdge::TopLeft => 0,
            ResizeEdge::Top => 1,
            ResizeEdge::TopRight => 2,
            ResizeEdge::Right => 3,
            ResizeEdge::BottomRight => 4,
            ResizeEdge::Bottom => 5,
            ResizeEdge::BottomLeft => 6,
            ResizeEdge::Left => 7,
        }
    }
}

#[derive(Debug)]
struct EdgeConstraints {
    top_tiled: bool,
    #[allow(dead_code)]
    top_resizable: bool,

    right_tiled: bool,
    #[allow(dead_code)]
    right_resizable: bool,

    bottom_tiled: bool,
    #[allow(dead_code)]
    bottom_resizable: bool,

    left_tiled: bool,
    #[allow(dead_code)]
    left_resizable: bool,
}

impl EdgeConstraints {
    fn from_atom(atom: u32) -> Self {
        EdgeConstraints {
            top_tiled: (atom & (1 << 0)) != 0,
            top_resizable: (atom & (1 << 1)) != 0,
            right_tiled: (atom & (1 << 2)) != 0,
            right_resizable: (atom & (1 << 3)) != 0,
            bottom_tiled: (atom & (1 << 4)) != 0,
            bottom_resizable: (atom & (1 << 5)) != 0,
            left_tiled: (atom & (1 << 6)) != 0,
            left_resizable: (atom & (1 << 7)) != 0,
        }
    }

    fn to_tiling(&self) -> Tiling {
        Tiling {
            top: self.top_tiled,
            right: self.right_tiled,
            bottom: self.bottom_tiled,
            left: self.left_tiled,
        }
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

pub struct X11WindowState {
    pub destroyed: bool,
    refresh_rate: Duration,
    client: X11ClientStatePtr,
    executor: ForegroundExecutor,
    atoms: XcbAtoms,
    x_root_window: xproto::Window,
    pub(crate) counter_id: sync::Counter,
    pub(crate) last_sync_counter: Option<sync::Int64>,
    _raw: RawWindow,
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    renderer: BladeRenderer,
    display: Rc<dyn PlatformDisplay>,
    input_handler: Option<PlatformInputHandler>,
    appearance: WindowAppearance,
    background_appearance: WindowBackgroundAppearance,
    maximized_vertical: bool,
    maximized_horizontal: bool,
    hidden: bool,
    active: bool,
    fullscreen: bool,
    decorations: WindowDecorations,
    edge_constraints: Option<EdgeConstraints>,
    pub handle: AnyWindowHandle,
    last_insets: [u32; 4],
}

impl X11WindowState {
    fn is_transparent(&self) -> bool {
        self.background_appearance != WindowBackgroundAppearance::Opaque
    }
}

#[derive(Clone)]
pub(crate) struct X11WindowStatePtr {
    pub state: Rc<RefCell<X11WindowState>>,
    pub(crate) callbacks: Rc<RefCell<Callbacks>>,
    xcb_connection: Rc<XCBConnection>,
    pub x_window: xproto::Window,
}

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
        handle: AnyWindowHandle,
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
        scale_factor: f32,
        appearance: WindowAppearance,
    ) -> anyhow::Result<Self> {
        let x_screen_index = params
            .display_id
            .map_or(x_main_screen_index, |did| did.0 as usize);

        let visual_set = find_visuals(&xcb_connection, x_screen_index);

        let visual = match visual_set.transparent {
            Some(visual) => visual,
            None => {
                log::warn!("Unable to find a transparent visual",);
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
                .check()?;
            id
        };

        let win_aux = xproto::CreateWindowAux::new()
            // https://stackoverflow.com/questions/43218127/x11-xlib-xcb-creating-a-window-requires-border-pixel-if-specifying-colormap-wh
            .border_pixel(visual_set.black_pixel)
            .colormap(colormap)
            .event_mask(
                xproto::EventMask::EXPOSURE
                    | xproto::EventMask::STRUCTURE_NOTIFY
                    | xproto::EventMask::FOCUS_CHANGE
                    | xproto::EventMask::KEY_PRESS
                    | xproto::EventMask::KEY_RELEASE
                    | EventMask::PROPERTY_CHANGE,
            );

        let mut bounds = params.bounds.to_device_pixels(scale_factor);
        if bounds.size.width.0 == 0 || bounds.size.height.0 == 0 {
            log::warn!("Window bounds contain a zero value. height={}, width={}. Falling back to defaults.", bounds.size.height.0, bounds.size.width.0);
            bounds.size.width = 800.into();
            bounds.size.height = 600.into();
        }

        xcb_connection
            .create_window(
                visual.depth,
                x_window,
                visual_set.root,
                (bounds.origin.x.0 + 2) as i16,
                bounds.origin.y.0 as i16,
                bounds.size.width.0 as u16,
                bounds.size.height.0 as u16,
                0,
                xproto::WindowClass::INPUT_OUTPUT,
                visual.id,
                &win_aux,
            )
            .unwrap()
            .check().with_context(|| {
                format!("CreateWindow request to X server failed. depth: {}, x_window: {}, visual_set.root: {}, bounds.origin.x.0: {}, bounds.origin.y.0: {}, bounds.size.width.0: {}, bounds.size.height.0: {}",
                    visual.depth, x_window, visual_set.root, bounds.origin.x.0 + 2, bounds.origin.y.0, bounds.size.width.0, bounds.size.height.0)
            })?;

        let reply = xcb_connection
            .get_geometry(x_window)
            .unwrap()
            .reply()
            .unwrap();
        if reply.x == 0 && reply.y == 0 {
            bounds.origin.x.0 += 2;
            // Work around a bug where our rendered content appears
            // outside the window bounds when opened at the default position
            // (14px, 49px on X + Gnome + Ubuntu 22).
            xcb_connection
                .configure_window(
                    x_window,
                    &xproto::ConfigureWindowAux::new()
                        .x(bounds.origin.x.0)
                        .y(bounds.origin.y.0),
                )
                .unwrap();
        }
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
        if params.kind == WindowKind::PopUp {
            xcb_connection
                .change_property32(
                    xproto::PropMode::REPLACE,
                    x_window,
                    atoms._NET_WM_WINDOW_TYPE,
                    xproto::AtomEnum::ATOM,
                    &[atoms._NET_WM_WINDOW_TYPE_NOTIFICATION],
                )
                .unwrap();
        }

        xcb_connection
            .change_property32(
                xproto::PropMode::REPLACE,
                x_window,
                atoms.WM_PROTOCOLS,
                xproto::AtomEnum::ATOM,
                &[atoms.WM_DELETE_WINDOW, atoms._NET_WM_SYNC_REQUEST],
            )
            .unwrap();

        sync::initialize(xcb_connection, 3, 1).unwrap();
        let sync_request_counter = xcb_connection.generate_id().unwrap();
        sync::create_counter(
            xcb_connection,
            sync_request_counter,
            sync::Int64 { lo: 0, hi: 0 },
        )
        .unwrap();

        xcb_connection
            .change_property32(
                xproto::PropMode::REPLACE,
                x_window,
                atoms._NET_WM_SYNC_REQUEST_COUNTER,
                xproto::AtomEnum::CARDINAL,
                &[sync_request_counter],
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
            .map_err(|e| anyhow::anyhow!("{:?}", e))?,
        );

        let config = BladeSurfaceConfig {
            // Note: this has to be done after the GPU init, or otherwise
            // the sizes are immediately invalidated.
            size: query_render_extent(xcb_connection, x_window),
            // We set it to transparent by default, even if we have client-side
            // decorations, since those seem to work on X11 even without `true` here.
            // If the window appearance changes, then the renderer will get updated
            // too
            transparent: false,
        };
        xcb_connection.map_window(x_window).unwrap();

        let screen_resources = xcb_connection
            .randr_get_screen_resources(x_window)
            .unwrap()
            .reply()
            .expect("Could not find available screens");

        let mode = screen_resources
            .crtcs
            .iter()
            .find_map(|crtc| {
                let crtc_info = xcb_connection
                    .randr_get_crtc_info(*crtc, x11rb::CURRENT_TIME)
                    .ok()?
                    .reply()
                    .ok()?;

                screen_resources
                    .modes
                    .iter()
                    .find(|m| m.id == crtc_info.mode)
            })
            .expect("Unable to find screen refresh rate");

        let refresh_rate = mode_refresh_rate(&mode);

        Ok(Self {
            client,
            executor,
            display: Rc::new(
                X11Display::new(xcb_connection, scale_factor, x_screen_index).unwrap(),
            ),
            _raw: raw,
            x_root_window: visual_set.root,
            bounds: bounds.to_pixels(scale_factor),
            scale_factor,
            renderer: BladeRenderer::new(gpu, config),
            atoms: *atoms,
            input_handler: None,
            active: false,
            fullscreen: false,
            maximized_vertical: false,
            maximized_horizontal: false,
            hidden: false,
            appearance,
            handle,
            background_appearance: WindowBackgroundAppearance::Opaque,
            destroyed: false,
            decorations: WindowDecorations::Server,
            last_insets: [0, 0, 0, 0],
            edge_constraints: None,
            counter_id: sync_request_counter,
            last_sync_counter: None,
            refresh_rate,
        })
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

        let destroy_x_window = maybe!({
            self.0.xcb_connection.unmap_window(self.0.x_window)?;
            self.0.xcb_connection.destroy_window(self.0.x_window)?;
            self.0.xcb_connection.flush()?;

            anyhow::Ok(())
        })
        .context("unmapping and destroying X11 window")
        .log_err();

        if destroy_x_window.is_some() {
            // Mark window as destroyed so that we can filter out when X11 events
            // for it still come in.
            state.destroyed = true;

            let this_ptr = self.0.clone();
            let client_ptr = state.client.clone();
            state
                .executor
                .spawn(async move {
                    this_ptr.close();
                    client_ptr.drop_window(this_ptr.x_window);
                })
                .detach();
        }

        drop(state);
    }
}

enum WmHintPropertyState {
    // Remove = 0,
    // Add = 1,
    Toggle = 2,
}

impl X11Window {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        handle: AnyWindowHandle,
        client: X11ClientStatePtr,
        executor: ForegroundExecutor,
        params: WindowParams,
        xcb_connection: &Rc<XCBConnection>,
        x_main_screen_index: usize,
        x_window: xproto::Window,
        atoms: &XcbAtoms,
        scale_factor: f32,
        appearance: WindowAppearance,
    ) -> anyhow::Result<Self> {
        let ptr = X11WindowStatePtr {
            state: Rc::new(RefCell::new(X11WindowState::new(
                handle,
                client,
                executor,
                params,
                xcb_connection,
                x_main_screen_index,
                x_window,
                atoms,
                scale_factor,
                appearance,
            )?)),
            callbacks: Rc::new(RefCell::new(Callbacks::default())),
            xcb_connection: xcb_connection.clone(),
            x_window,
        };

        let state = ptr.state.borrow_mut();
        ptr.set_wm_properties(state);

        Ok(Self(ptr))
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
            .unwrap()
            .check()
            .unwrap();
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

    fn send_moveresize(&self, flag: u32) {
        let state = self.0.state.borrow();

        self.0
            .xcb_connection
            .ungrab_pointer(x11rb::CURRENT_TIME)
            .unwrap()
            .check()
            .unwrap();

        let pointer = self
            .0
            .xcb_connection
            .query_pointer(self.0.x_window)
            .unwrap()
            .reply()
            .unwrap();
        let message = ClientMessageEvent::new(
            32,
            self.0.x_window,
            state.atoms._NET_WM_MOVERESIZE,
            [
                pointer.root_x as u32,
                pointer.root_y as u32,
                flag,
                0, // Left mouse button
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

        self.0.xcb_connection.flush().unwrap();
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

    pub fn property_notify(&self, event: xproto::PropertyNotifyEvent) {
        let mut state = self.state.borrow_mut();
        if event.atom == state.atoms._NET_WM_STATE {
            self.set_wm_properties(state);
        } else if event.atom == state.atoms._GTK_EDGE_CONSTRAINTS {
            self.set_edge_constraints(state);
        }
    }

    fn set_edge_constraints(&self, mut state: std::cell::RefMut<X11WindowState>) {
        let reply = self
            .xcb_connection
            .get_property(
                false,
                self.x_window,
                state.atoms._GTK_EDGE_CONSTRAINTS,
                xproto::AtomEnum::CARDINAL,
                0,
                4,
            )
            .unwrap()
            .reply()
            .unwrap();

        if reply.value_len != 0 {
            let atom = u32::from_ne_bytes(reply.value[0..4].try_into().unwrap());
            let edge_constraints = EdgeConstraints::from_atom(atom);
            state.edge_constraints.replace(edge_constraints);
        }
    }

    fn set_wm_properties(&self, mut state: std::cell::RefMut<X11WindowState>) {
        let reply = self
            .xcb_connection
            .get_property(
                false,
                self.x_window,
                state.atoms._NET_WM_STATE,
                xproto::AtomEnum::ATOM,
                0,
                u32::MAX,
            )
            .unwrap()
            .reply()
            .unwrap();

        let atoms = reply
            .value
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));

        state.active = false;
        state.fullscreen = false;
        state.maximized_vertical = false;
        state.maximized_horizontal = false;
        state.hidden = true;

        for atom in atoms {
            if atom == state.atoms._NET_WM_STATE_FOCUSED {
                state.active = true;
            } else if atom == state.atoms._NET_WM_STATE_FULLSCREEN {
                state.fullscreen = true;
            } else if atom == state.atoms._NET_WM_STATE_MAXIMIZED_VERT {
                state.maximized_vertical = true;
            } else if atom == state.atoms._NET_WM_STATE_MAXIMIZED_HORZ {
                state.maximized_horizontal = true;
            } else if atom == state.atoms._NET_WM_STATE_HIDDEN {
                state.hidden = true;
            }
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
        let is_resize;
        {
            let mut state = self.state.borrow_mut();
            let bounds = bounds.map(|f| px(f as f32 / state.scale_factor));

            is_resize = bounds.size.width != state.bounds.size.width
                || bounds.size.height != state.bounds.size.height;

            // If it's a resize event (only width/height changed), we ignore `bounds.origin`
            // because it contains wrong values.
            if is_resize {
                state.bounds.size = bounds.size;
            } else {
                state.bounds = bounds;
            }

            let gpu_size = query_render_extent(&self.xcb_connection, self.x_window);
            if true {
                state.renderer.update_drawable_size(size(
                    DevicePixels(gpu_size.width as i32),
                    DevicePixels(gpu_size.height as i32),
                ));
                resize_args = Some((state.content_size(), state.scale_factor));
            }
            if let Some(value) = state.last_sync_counter.take() {
                sync::set_counter(&self.xcb_connection, state.counter_id, value).unwrap();
            }
        }

        let mut callbacks = self.callbacks.borrow_mut();
        if let Some((content_size, scale_factor)) = resize_args {
            if let Some(ref mut fun) = callbacks.resize {
                fun(content_size, scale_factor)
            }
        }
        if !is_resize {
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
        let mut state = self.state.borrow_mut();
        state.appearance = appearance;
        let is_transparent = state.is_transparent();
        state.renderer.update_transparency(is_transparent);
        state.appearance = appearance;
        drop(state);
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut fun) = callbacks.appearance_changed {
            (fun)()
        }
    }

    pub fn refresh_rate(&self) -> Duration {
        self.state.borrow().refresh_rate
    }
}

impl PlatformWindow for X11Window {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        let state = self.0.state.borrow();

        // A maximized window that gets minimized will still retain its maximized state.
        !state.hidden && state.maximized_vertical && state.maximized_horizontal
    }

    fn window_bounds(&self) -> WindowBounds {
        let state = self.0.state.borrow();
        if self.is_maximized() {
            WindowBounds::Maximized(state.bounds)
        } else {
            WindowBounds::Windowed(state.bounds)
        }
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

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.0.state.borrow().display.clone())
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

    fn modifiers(&self) -> Modifiers {
        self.0
            .state
            .borrow()
            .client
            .0
            .upgrade()
            .map(|ref_cell| ref_cell.borrow().modifiers)
            .unwrap_or_default()
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
        let data = [1, xproto::Time::CURRENT_TIME.into(), 0, 0, 0];
        let message = xproto::ClientMessageEvent::new(
            32,
            self.0.x_window,
            self.0.state.borrow().atoms._NET_ACTIVE_WINDOW,
            data,
        );
        self.0
            .xcb_connection
            .send_event(
                false,
                self.0.state.borrow().x_root_window,
                xproto::EventMask::SUBSTRUCTURE_REDIRECT | xproto::EventMask::SUBSTRUCTURE_NOTIFY,
                message,
            )
            .log_err();
        self.0
            .xcb_connection
            .set_input_focus(
                xproto::InputFocus::POINTER_ROOT,
                self.0.x_window,
                xproto::Time::CURRENT_TIME,
            )
            .log_err();
        self.0.xcb_connection.flush().unwrap();
    }

    fn is_active(&self) -> bool {
        self.0.state.borrow().active
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
        self.0.xcb_connection.flush().unwrap();
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
            .unwrap()
            .check()
            .unwrap();
    }

    fn set_edited(&mut self, _edited: bool) {
        log::info!("ignoring macOS specific set_edited");
    }

    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance) {
        let mut state = self.0.state.borrow_mut();
        state.background_appearance = background_appearance;
        let transparent = state.is_transparent();
        state.renderer.update_transparency(transparent);
    }

    fn show_character_palette(&self) {
        log::info!("ignoring macOS specific show_character_palette");
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
            .unwrap()
            .check()
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
        self.0.state.borrow().fullscreen
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

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
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
            .unwrap()
            .check()
            .unwrap();
    }

    fn start_window_move(&self) {
        const MOVERESIZE_MOVE: u32 = 8;
        self.send_moveresize(MOVERESIZE_MOVE);
    }

    fn start_window_resize(&self, edge: ResizeEdge) {
        self.send_moveresize(edge.to_moveresize());
    }

    fn window_decorations(&self) -> crate::Decorations {
        let state = self.0.state.borrow();

        match state.decorations {
            WindowDecorations::Server => Decorations::Server,
            WindowDecorations::Client => {
                let tiling = if let Some(edge_constraints) = &state.edge_constraints {
                    edge_constraints.to_tiling()
                } else {
                    // https://source.chromium.org/chromium/chromium/src/+/main:ui/ozone/platform/x11/x11_window.cc;l=2519;drc=1f14cc876cc5bf899d13284a12c451498219bb2d
                    Tiling {
                        top: state.maximized_vertical,
                        bottom: state.maximized_vertical,
                        left: state.maximized_horizontal,
                        right: state.maximized_horizontal,
                    }
                };

                Decorations::Client { tiling }
            }
        }
    }

    fn set_client_inset(&self, inset: Pixels) {
        let mut state = self.0.state.borrow_mut();

        let dp = (inset.0 * state.scale_factor) as u32;

        let insets = if let Some(edge_constraints) = &state.edge_constraints {
            let left = if edge_constraints.left_tiled { 0 } else { dp };
            let top = if edge_constraints.top_tiled { 0 } else { dp };
            let right = if edge_constraints.right_tiled { 0 } else { dp };
            let bottom = if edge_constraints.bottom_tiled { 0 } else { dp };

            [left, right, top, bottom]
        } else {
            let (left, right) = if state.maximized_horizontal {
                (0, 0)
            } else {
                (dp, dp)
            };
            let (top, bottom) = if state.maximized_vertical {
                (0, 0)
            } else {
                (dp, dp)
            };
            [left, right, top, bottom]
        };

        if state.last_insets != insets {
            state.last_insets = insets;

            self.0
                .xcb_connection
                .change_property(
                    xproto::PropMode::REPLACE,
                    self.0.x_window,
                    state.atoms._GTK_FRAME_EXTENTS,
                    xproto::AtomEnum::CARDINAL,
                    size_of::<u32>() as u8 * 8,
                    4,
                    bytemuck::cast_slice::<u32, u8>(&insets),
                )
                .unwrap()
                .check()
                .unwrap();
        }
    }

    fn request_decorations(&self, decorations: crate::WindowDecorations) {
        // https://github.com/rust-windowing/winit/blob/master/src/platform_impl/linux/x11/util/hint.rs#L53-L87
        let hints_data: [u32; 5] = match decorations {
            WindowDecorations::Server => [1 << 1, 0, 1, 0, 0],
            WindowDecorations::Client => [1 << 1, 0, 0, 0, 0],
        };

        let mut state = self.0.state.borrow_mut();

        self.0
            .xcb_connection
            .change_property(
                xproto::PropMode::REPLACE,
                self.0.x_window,
                state.atoms._MOTIF_WM_HINTS,
                state.atoms._MOTIF_WM_HINTS,
                std::mem::size_of::<u32>() as u8 * 8,
                5,
                bytemuck::cast_slice::<u32, u8>(&hints_data),
            )
            .unwrap()
            .check()
            .unwrap();

        match decorations {
            WindowDecorations::Server => {
                state.decorations = WindowDecorations::Server;
                let is_transparent = state.is_transparent();
                state.renderer.update_transparency(is_transparent);
            }
            WindowDecorations::Client => {
                state.decorations = WindowDecorations::Client;
                let is_transparent = state.is_transparent();
                state.renderer.update_transparency(is_transparent);
            }
        }

        drop(state);
        let mut callbacks = self.0.callbacks.borrow_mut();
        if let Some(appearance_changed) = callbacks.appearance_changed.as_mut() {
            appearance_changed();
        }
    }
}

// Adapted from:
// https://docs.rs/winit/0.29.11/src/winit/platform_impl/linux/x11/monitor.rs.html#103-111
pub fn mode_refresh_rate(mode: &randr::ModeInfo) -> Duration {
    if mode.dot_clock == 0 || mode.htotal == 0 || mode.vtotal == 0 {
        return Duration::from_millis(16);
    }

    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    let micros = 1_000_000_000 / millihertz;
    log::info!("Refreshing at {} micros", micros);
    Duration::from_micros(micros)
}
