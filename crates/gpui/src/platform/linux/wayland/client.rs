use core::hash;
use std::cell::{RefCell, RefMut};
use std::ffi::OsString;
use std::ops::{Deref, DerefMut};
use std::os::fd::{AsRawFd, BorrowedFd};
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_task::Runnable;
use calloop::timer::{TimeoutAction, Timer};
use calloop::{EventLoop, LoopHandle};
use calloop_wayland_source::WaylandSource;
use collections::HashMap;
use copypasta::wayland_clipboard::{create_clipboards_from_external, Clipboard, Primary};
use copypasta::ClipboardProvider;
use filedescriptor::Pipe;
use parking_lot::Mutex;
use smallvec::SmallVec;
use util::ResultExt;
use wayland_backend::client::ObjectId;
use wayland_backend::protocol::WEnum;
use wayland_client::event_created_child;
use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::wl_callback::{self, WlCallback};
use wayland_client::protocol::wl_data_device_manager::DndAction;
use wayland_client::protocol::wl_pointer::AxisSource;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::{
    wl_data_device, wl_data_device_manager, wl_data_offer, wl_data_source, wl_output, wl_region,
};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat, wl_shm,
        wl_shm_pool, wl_surface,
    },
    Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1::Shape;
use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1, wp_cursor_shape_manager_v1,
};
use wayland_protocols::wp::fractional_scale::v1::client::{
    wp_fractional_scale_manager_v1, wp_fractional_scale_v1,
};
use wayland_protocols::wp::text_input::zv3::client::zwp_text_input_v3::{
    ContentHint, ContentPurpose,
};
use wayland_protocols::wp::text_input::zv3::client::{
    zwp_text_input_manager_v3, zwp_text_input_v3,
};
use wayland_protocols::wp::viewporter::client::{wp_viewport, wp_viewporter};
use wayland_protocols::xdg::activation::v1::client::{xdg_activation_token_v1, xdg_activation_v1};
use wayland_protocols::xdg::decoration::zv1::client::{
    zxdg_decoration_manager_v1, zxdg_toplevel_decoration_v1,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use wayland_protocols_plasma::blur::client::{org_kde_kwin_blur, org_kde_kwin_blur_manager};
use xkbcommon::xkb::ffi::XKB_KEYMAP_FORMAT_TEXT_V1;
use xkbcommon::xkb::{self, Keycode, KEYMAP_COMPILE_NO_FLAGS};

use super::super::{open_uri_internal, read_fd, DOUBLE_CLICK_INTERVAL};
use super::window::{ImeInput, WaylandWindowState, WaylandWindowStatePtr};
use crate::platform::linux::is_within_click_distance;
use crate::platform::linux::wayland::cursor::Cursor;
use crate::platform::linux::wayland::serial::{SerialKind, SerialTracker};
use crate::platform::linux::wayland::window::WaylandWindow;
use crate::platform::linux::xdg_desktop_portal::{Event as XDPEvent, XDPEventSource};
use crate::platform::linux::LinuxClient;
use crate::platform::PlatformWindow;
use crate::{
    point, px, Bounds, FileDropEvent, ForegroundExecutor, MouseExitEvent, WindowAppearance,
    SCROLL_LINES,
};
use crate::{
    AnyWindowHandle, CursorStyle, DisplayId, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    NavigationDirection, Pixels, PlatformDisplay, PlatformInput, Point, ScrollDelta,
    ScrollWheelEvent, TouchPhase,
};
use crate::{LinuxCommon, WindowParams};

/// Used to convert evdev scancode to xkb scancode
const MIN_KEYCODE: u32 = 8;

#[derive(Clone)]
pub struct Globals {
    pub qh: QueueHandle<WaylandClientStatePtr>,
    pub activation: Option<xdg_activation_v1::XdgActivationV1>,
    pub compositor: wl_compositor::WlCompositor,
    pub cursor_shape_manager: Option<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1>,
    pub data_device_manager: Option<wl_data_device_manager::WlDataDeviceManager>,
    pub wm_base: xdg_wm_base::XdgWmBase,
    pub shm: wl_shm::WlShm,
    pub seat: wl_seat::WlSeat,
    pub viewporter: Option<wp_viewporter::WpViewporter>,
    pub fractional_scale_manager:
        Option<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
    pub decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    pub blur_manager: Option<org_kde_kwin_blur_manager::OrgKdeKwinBlurManager>,
    pub text_input_manager: Option<zwp_text_input_manager_v3::ZwpTextInputManagerV3>,
    pub executor: ForegroundExecutor,
}

impl Globals {
    fn new(
        globals: GlobalList,
        executor: ForegroundExecutor,
        qh: QueueHandle<WaylandClientStatePtr>,
        seat: wl_seat::WlSeat,
    ) -> Self {
        Globals {
            activation: globals.bind(&qh, 1..=1, ()).ok(),
            compositor: globals
                .bind(
                    &qh,
                    wl_surface::REQ_SET_BUFFER_SCALE_SINCE
                        ..=wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE,
                    (),
                )
                .unwrap(),
            cursor_shape_manager: globals.bind(&qh, 1..=1, ()).ok(),
            data_device_manager: globals
                .bind(
                    &qh,
                    WL_DATA_DEVICE_MANAGER_VERSION..=WL_DATA_DEVICE_MANAGER_VERSION,
                    (),
                )
                .ok(),
            shm: globals.bind(&qh, 1..=1, ()).unwrap(),
            seat,
            wm_base: globals.bind(&qh, 1..=1, ()).unwrap(),
            viewporter: globals.bind(&qh, 1..=1, ()).ok(),
            fractional_scale_manager: globals.bind(&qh, 1..=1, ()).ok(),
            decoration_manager: globals.bind(&qh, 1..=1, ()).ok(),
            blur_manager: globals.bind(&qh, 1..=1, ()).ok(),
            text_input_manager: globals.bind(&qh, 1..=1, ()).ok(),
            executor,
            qh,
        }
    }
}

pub(crate) struct WaylandClientState {
    serial_tracker: SerialTracker,
    globals: Globals,
    wl_seat: wl_seat::WlSeat, // todo(linux): multi-seat support
    wl_pointer: Option<wl_pointer::WlPointer>,
    cursor_shape_device: Option<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1>,
    data_device: Option<wl_data_device::WlDataDevice>,
    text_input: Option<zwp_text_input_v3::ZwpTextInputV3>,
    pre_edit_text: Option<String>,
    composing: bool,
    // Surface to Window mapping
    windows: HashMap<ObjectId, WaylandWindowStatePtr>,
    // Output to scale mapping
    output_scales: HashMap<ObjectId, i32>,
    keymap_state: Option<xkb::State>,
    compose_state: Option<xkb::compose::State>,
    drag: DragState,
    click: ClickState,
    repeat: KeyRepeat,
    modifiers: Modifiers,
    axis_source: AxisSource,
    mouse_location: Option<Point<Pixels>>,
    continuous_scroll_delta: Option<Point<Pixels>>,
    discrete_scroll_delta: Option<Point<f32>>,
    vertical_modifier: f32,
    horizontal_modifier: f32,
    scroll_event_received: bool,
    enter_token: Option<()>,
    button_pressed: Option<MouseButton>,
    mouse_focused_window: Option<WaylandWindowStatePtr>,
    keyboard_focused_window: Option<WaylandWindowStatePtr>,
    loop_handle: LoopHandle<'static, WaylandClientStatePtr>,
    cursor_style: Option<CursorStyle>,
    cursor: Cursor,
    clipboard: Option<Clipboard>,
    primary: Option<Primary>,
    event_loop: Option<EventLoop<'static, WaylandClientStatePtr>>,
    common: LinuxCommon,

    pending_open_uri: Option<String>,
}

pub struct DragState {
    data_offer: Option<wl_data_offer::WlDataOffer>,
    window: Option<WaylandWindowStatePtr>,
    position: Point<Pixels>,
}

pub struct ClickState {
    last_click: Instant,
    last_location: Point<Pixels>,
    current_count: usize,
}

pub(crate) struct KeyRepeat {
    characters_per_second: u32,
    delay: Duration,
    current_id: u64,
    current_keycode: Option<xkb::Keycode>,
}

/// This struct is required to conform to Rust's orphan rules, so we can dispatch on the state but hand the
/// window to GPUI.
#[derive(Clone)]
pub struct WaylandClientStatePtr(Weak<RefCell<WaylandClientState>>);

impl WaylandClientStatePtr {
    fn get_client(&self) -> Rc<RefCell<WaylandClientState>> {
        self.0
            .upgrade()
            .expect("The pointer should always be valid when dispatching in wayland")
    }

    pub fn get_serial(&self, kind: SerialKind) -> u32 {
        self.0.upgrade().unwrap().borrow().serial_tracker.get(kind)
    }

    pub fn enable_ime(&self) {
        let client = self.get_client();
        let mut state = client.borrow_mut();
        let Some(mut text_input) = state.text_input.take() else {
            return;
        };

        text_input.enable();
        text_input.set_content_type(ContentHint::None, ContentPurpose::Normal);
        if let Some(window) = state.keyboard_focused_window.clone() {
            drop(state);
            if let Some(area) = window.get_ime_area() {
                text_input.set_cursor_rectangle(
                    area.origin.x.0 as i32,
                    area.origin.y.0 as i32,
                    area.size.width.0 as i32,
                    area.size.height.0 as i32,
                );
            }
            state = client.borrow_mut();
        }
        text_input.commit();
        state.text_input = Some(text_input);
    }

    pub fn disable_ime(&self) {
        let client = self.get_client();
        let mut state = client.borrow_mut();
        state.composing = false;
        if let Some(text_input) = &state.text_input {
            text_input.disable();
            text_input.commit();
        }
    }

    pub fn drop_window(&self, surface_id: &ObjectId) {
        let mut client = self.get_client();
        let mut state = client.borrow_mut();
        let closed_window = state.windows.remove(surface_id).unwrap();
        if let Some(window) = state.mouse_focused_window.take() {
            if !window.ptr_eq(&closed_window) {
                state.mouse_focused_window = Some(window);
            }
        }
        if let Some(window) = state.keyboard_focused_window.take() {
            if !window.ptr_eq(&closed_window) {
                state.keyboard_focused_window = Some(window);
            }
        }
        if state.windows.is_empty() {
            state.common.signal.stop();
        }
    }
}

#[derive(Clone)]
pub struct WaylandClient(Rc<RefCell<WaylandClientState>>);

impl Drop for WaylandClient {
    fn drop(&mut self) {
        let mut state = self.0.borrow_mut();
        state.windows.clear();

        // Drop the clipboard to prevent a seg fault after we've closed all Wayland connections.
        state.primary = None;
        state.clipboard = None;
        if let Some(wl_pointer) = &state.wl_pointer {
            wl_pointer.release();
        }
        if let Some(cursor_shape_device) = &state.cursor_shape_device {
            cursor_shape_device.destroy();
        }
        if let Some(data_device) = &state.data_device {
            data_device.release();
        }
        if let Some(text_input) = &state.text_input {
            text_input.destroy();
        }
    }
}

const WL_DATA_DEVICE_MANAGER_VERSION: u32 = 3;
const WL_OUTPUT_VERSION: u32 = 2;

fn wl_seat_version(version: u32) -> u32 {
    // We rely on the wl_pointer.frame event
    const WL_SEAT_MIN_VERSION: u32 = 5;
    const WL_SEAT_MAX_VERSION: u32 = 9;

    if version < WL_SEAT_MIN_VERSION {
        panic!(
            "wl_seat below required version: {} < {}",
            version, WL_SEAT_MIN_VERSION
        );
    }

    version.clamp(WL_SEAT_MIN_VERSION, WL_SEAT_MAX_VERSION)
}

impl WaylandClient {
    pub(crate) fn new() -> Self {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, mut event_queue) =
            registry_queue_init::<WaylandClientStatePtr>(&conn).unwrap();
        let qh = event_queue.handle();

        let mut seat: Option<wl_seat::WlSeat> = None;
        let mut outputs = HashMap::default();
        globals.contents().with_list(|list| {
            for global in list {
                match &global.interface[..] {
                    "wl_seat" => {
                        seat = Some(globals.registry().bind::<wl_seat::WlSeat, _, _>(
                            global.name,
                            wl_seat_version(global.version),
                            &qh,
                            (),
                        ));
                    }
                    "wl_output" => {
                        let output = globals.registry().bind::<wl_output::WlOutput, _, _>(
                            global.name,
                            WL_OUTPUT_VERSION,
                            &qh,
                            (),
                        );
                        outputs.insert(output.id(), 1);
                    }
                    _ => {}
                }
            }
        });

        let display = conn.backend().display_ptr() as *mut std::ffi::c_void;

        let event_loop = EventLoop::<WaylandClientStatePtr>::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();
        handle.insert_source(main_receiver, |event, _, _: &mut WaylandClientStatePtr| {
            if let calloop::channel::Event::Msg(runnable) = event {
                runnable.run();
            }
        });

        let seat = seat.unwrap();
        let globals = Globals::new(
            globals,
            common.foreground_executor.clone(),
            qh.clone(),
            seat.clone(),
        );

        let data_device = globals
            .data_device_manager
            .as_ref()
            .map(|data_device_manager| data_device_manager.get_data_device(&seat, &qh, ()));

        let (primary, clipboard) = unsafe { create_clipboards_from_external(display) };

        let cursor = Cursor::new(&conn, &globals, 24);

        handle.insert_source(XDPEventSource::new(&common.background_executor), {
            move |event, _, client| match event {
                XDPEvent::WindowAppearance(appearance) => {
                    if let Some(client) = client.0.upgrade() {
                        let mut client = client.borrow_mut();

                        client.common.appearance = appearance;

                        for (_, window) in &mut client.windows {
                            window.set_appearance(appearance);
                        }
                    }
                }
            }
        });

        let mut state = Rc::new(RefCell::new(WaylandClientState {
            serial_tracker: SerialTracker::new(),
            globals,
            wl_seat: seat,
            wl_pointer: None,
            cursor_shape_device: None,
            data_device,
            text_input: None,
            pre_edit_text: None,
            composing: false,
            output_scales: outputs,
            windows: HashMap::default(),
            common,
            keymap_state: None,
            compose_state: None,
            drag: DragState {
                data_offer: None,
                window: None,
                position: Point::default(),
            },
            click: ClickState {
                last_click: Instant::now(),
                last_location: Point::default(),
                current_count: 0,
            },
            repeat: KeyRepeat {
                characters_per_second: 16,
                delay: Duration::from_millis(500),
                current_id: 0,
                current_keycode: None,
            },
            modifiers: Modifiers {
                shift: false,
                control: false,
                alt: false,
                function: false,
                platform: false,
            },
            scroll_event_received: false,
            axis_source: AxisSource::Wheel,
            mouse_location: None,
            continuous_scroll_delta: None,
            discrete_scroll_delta: None,
            vertical_modifier: -1.0,
            horizontal_modifier: -1.0,
            button_pressed: None,
            mouse_focused_window: None,
            keyboard_focused_window: None,
            loop_handle: handle.clone(),
            enter_token: None,
            cursor_style: None,
            cursor,
            clipboard: Some(clipboard),
            primary: Some(primary),
            event_loop: Some(event_loop),

            pending_open_uri: None,
        }));

        WaylandSource::new(conn, event_queue).insert(handle);

        Self(state)
    }
}

impl LinuxClient for WaylandClient {
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        Vec::new()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        unimplemented!()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.0.borrow_mut();

        let (window, surface_id) = WaylandWindow::new(
            state.globals.clone(),
            WaylandClientStatePtr(Rc::downgrade(&self.0)),
            params,
            state.common.appearance,
        );
        state.windows.insert(surface_id, window.0.clone());

        Box::new(window)
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let mut state = self.0.borrow_mut();

        let need_update = state
            .cursor_style
            .map_or(true, |current_style| current_style != style);

        if need_update {
            let serial = state.serial_tracker.get(SerialKind::MouseEnter);
            state.cursor_style = Some(style);

            if let Some(cursor_shape_device) = &state.cursor_shape_device {
                cursor_shape_device.set_shape(serial, style.to_shape());
            } else if state.mouse_focused_window.is_some() {
                // cursor-shape-v1 isn't supported, set the cursor using a surface.
                let wl_pointer = state
                    .wl_pointer
                    .clone()
                    .expect("window is focused by pointer");
                state
                    .cursor
                    .set_icon(&wl_pointer, serial, &style.to_icon_name());
            }
        }
    }

    fn open_uri(&self, uri: &str) {
        let mut state = self.0.borrow_mut();
        if let (Some(activation), Some(window)) = (
            state.globals.activation.clone(),
            state.mouse_focused_window.clone(),
        ) {
            state.pending_open_uri = Some(uri.to_owned());
            let token = activation.get_activation_token(&state.globals.qh, ());
            let serial = state.serial_tracker.get(SerialKind::MousePress);
            token.set_serial(serial, &state.wl_seat);
            token.set_surface(&window.surface());
            token.commit();
        } else {
            open_uri_internal(uri, None);
        }
    }

    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .borrow_mut()
            .event_loop
            .take()
            .expect("App is already running");

        event_loop
            .run(
                None,
                &mut WaylandClientStatePtr(Rc::downgrade(&self.0)),
                |_| {},
            )
            .log_err();
    }

    fn write_to_primary(&self, item: crate::ClipboardItem) {
        self.0
            .borrow_mut()
            .primary
            .as_mut()
            .unwrap()
            .set_contents(item.text);
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        self.0
            .borrow_mut()
            .clipboard
            .as_mut()
            .unwrap()
            .set_contents(item.text);
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .primary
            .as_mut()
            .unwrap()
            .get_contents()
            .ok()
            .map(|s| crate::ClipboardItem {
                text: s,
                metadata: None,
            })
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .clipboard
            .as_mut()
            .unwrap()
            .get_contents()
            .ok()
            .map(|s| crate::ClipboardItem {
                text: s,
                metadata: None,
            })
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut client = this.get_client();
        let mut state = client.borrow_mut();

        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => match &interface[..] {
                "wl_seat" => {
                    state.wl_pointer = None;
                    registry.bind::<wl_seat::WlSeat, _, _>(name, wl_seat_version(version), qh, ());
                }
                "wl_output" => {
                    let output =
                        registry.bind::<wl_output::WlOutput, _, _>(name, WL_OUTPUT_VERSION, qh, ());

                    state.output_scales.insert(output.id(), 1);
                }
                _ => {}
            },
            wl_registry::Event::GlobalRemove { name: _ } => {}
            _ => {}
        }
    }
}

delegate_noop!(WaylandClientStatePtr: ignore xdg_activation_v1::XdgActivationV1);
delegate_noop!(WaylandClientStatePtr: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientStatePtr: ignore wp_cursor_shape_device_v1::WpCursorShapeDeviceV1);
delegate_noop!(WaylandClientStatePtr: ignore wp_cursor_shape_manager_v1::WpCursorShapeManagerV1);
delegate_noop!(WaylandClientStatePtr: ignore wl_data_device_manager::WlDataDeviceManager);
delegate_noop!(WaylandClientStatePtr: ignore wl_shm::WlShm);
delegate_noop!(WaylandClientStatePtr: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientStatePtr: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientStatePtr: ignore wl_region::WlRegion);
delegate_noop!(WaylandClientStatePtr: ignore wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1);
delegate_noop!(WaylandClientStatePtr: ignore zxdg_decoration_manager_v1::ZxdgDecorationManagerV1);
delegate_noop!(WaylandClientStatePtr: ignore org_kde_kwin_blur_manager::OrgKdeKwinBlurManager);
delegate_noop!(WaylandClientStatePtr: ignore zwp_text_input_manager_v3::ZwpTextInputManagerV3);
delegate_noop!(WaylandClientStatePtr: ignore org_kde_kwin_blur::OrgKdeKwinBlur);
delegate_noop!(WaylandClientStatePtr: ignore wp_viewporter::WpViewporter);
delegate_noop!(WaylandClientStatePtr: ignore wp_viewport::WpViewport);

impl Dispatch<WlCallback, ObjectId> for WaylandClientStatePtr {
    fn event(
        state: &mut WaylandClientStatePtr,
        _: &wl_callback::WlCallback,
        event: wl_callback::Event,
        surface_id: &ObjectId,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let client = state.get_client();
        let mut state = client.borrow_mut();
        let Some(window) = get_window(&mut state, surface_id) else {
            return;
        };
        drop(state);

        match event {
            wl_callback::Event::Done { callback_data } => {
                window.frame(true);
            }
            _ => {}
        }
    }
}

fn get_window(
    mut state: &mut RefMut<WaylandClientState>,
    surface_id: &ObjectId,
) -> Option<WaylandWindowStatePtr> {
    state.windows.get(surface_id).cloned()
}

impl Dispatch<wl_surface::WlSurface, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        surface: &wl_surface::WlSurface,
        event: <wl_surface::WlSurface as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut client = this.get_client();
        let mut state = client.borrow_mut();

        let Some(window) = get_window(&mut state, &surface.id()) else {
            return;
        };
        let scales = state.output_scales.clone();
        drop(state);

        window.handle_surface_event(event, scales);
    }
}

impl Dispatch<wl_output::WlOutput, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        output: &wl_output::WlOutput,
        event: <wl_output::WlOutput as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut client = this.get_client();
        let mut state = client.borrow_mut();

        let Some(mut output_scale) = state.output_scales.get_mut(&output.id()) else {
            return;
        };

        match event {
            wl_output::Event::Scale { factor } => {
                *output_scale = factor;
            }
            _ => {}
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ObjectId> for WaylandClientStatePtr {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        surface_id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = state.get_client();
        let mut state = client.borrow_mut();
        let Some(window) = get_window(&mut state, surface_id) else {
            return;
        };
        drop(state);
        window.handle_xdg_surface_event(event);
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ObjectId> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        xdg_toplevel: &xdg_toplevel::XdgToplevel,
        event: <xdg_toplevel::XdgToplevel as Proxy>::Event,
        surface_id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        let Some(window) = get_window(&mut state, surface_id) else {
            return;
        };

        drop(state);
        let should_close = window.handle_toplevel_event(event);

        if should_close {
            this.drop_window(surface_id);
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for WaylandClientStatePtr {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: <xdg_wm_base::XdgWmBase as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<xdg_activation_token_v1::XdgActivationTokenV1, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        token: &xdg_activation_token_v1::XdgActivationTokenV1,
        event: <xdg_activation_token_v1::XdgActivationTokenV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        if let xdg_activation_token_v1::Event::Done { token } = event {
            if let Some(uri) = state.pending_open_uri.take() {
                open_uri_internal(&uri, Some(&token));
            } else {
                log::error!("called while pending_open_uri is None");
            }
        }
        token.destroy();
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for WaylandClientStatePtr {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            let client = state.get_client();
            let mut state = client.borrow_mut();
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
                state.text_input = state
                    .globals
                    .text_input_manager
                    .as_ref()
                    .map(|text_input_manager| text_input_manager.get_text_input(&seat, qh, ()));
            }
            if capabilities.contains(wl_seat::Capability::Pointer) {
                let pointer = seat.get_pointer(qh, ());
                state.cursor_shape_device = state
                    .globals
                    .cursor_shape_manager
                    .as_ref()
                    .map(|cursor_shape_manager| cursor_shape_manager.get_pointer(&pointer, qh, ()));
                state.wl_pointer = Some(pointer);
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut client = this.get_client();
        let mut state = client.borrow_mut();
        match event {
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                state.repeat.characters_per_second = rate as u32;
                state.repeat.delay = Duration::from_millis(delay as u64);
            }
            wl_keyboard::Event::Keymap {
                format: WEnum::Value(format),
                fd,
                size,
                ..
            } => {
                assert_eq!(
                    format,
                    wl_keyboard::KeymapFormat::XkbV1,
                    "Unsupported keymap format"
                );
                let xkb_context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
                let keymap = unsafe {
                    xkb::Keymap::new_from_fd(
                        &xkb_context,
                        fd,
                        size as usize,
                        XKB_KEYMAP_FORMAT_TEXT_V1,
                        KEYMAP_COMPILE_NO_FLAGS,
                    )
                    .log_err()
                    .flatten()
                    .expect("Failed to create keymap")
                };
                let table = {
                    let locale = std::env::var_os("LC_CTYPE").unwrap_or(OsString::from("C"));
                    xkb::compose::Table::new_from_locale(
                        &xkb_context,
                        &locale,
                        xkb::compose::COMPILE_NO_FLAGS,
                    )
                    .log_err()
                    .unwrap()
                };
                state.keymap_state = Some(xkb::State::new(&keymap));
                state.compose_state = Some(xkb::compose::State::new(
                    &table,
                    xkb::compose::STATE_NO_FLAGS,
                ));
            }
            wl_keyboard::Event::Enter { surface, .. } => {
                state.keyboard_focused_window = get_window(&mut state, &surface.id());
                state.enter_token = Some(());

                if let Some(window) = state.keyboard_focused_window.clone() {
                    drop(state);
                    window.set_focused(true);
                }
            }
            wl_keyboard::Event::Leave { surface, .. } => {
                let keyboard_focused_window = get_window(&mut state, &surface.id());
                state.keyboard_focused_window = None;
                state.enter_token.take();

                if let Some(window) = keyboard_focused_window {
                    if let Some(ref mut compose) = state.compose_state {
                        compose.reset();
                    }
                    state.pre_edit_text.take();
                    drop(state);
                    window.handle_ime(ImeInput::DeleteText);
                    window.set_focused(false);
                }
            }
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => {
                let focused_window = state.keyboard_focused_window.clone();
                let Some(focused_window) = focused_window else {
                    return;
                };

                let keymap_state = state.keymap_state.as_mut().unwrap();
                keymap_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
                state.modifiers = Modifiers::from_xkb(keymap_state);

                let input = PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers: state.modifiers,
                });

                drop(state);
                focused_window.handle_input(input);
            }
            wl_keyboard::Event::Key {
                serial,
                key,
                state: WEnum::Value(key_state),
                ..
            } => {
                state.serial_tracker.update(SerialKind::KeyPress, serial);

                let focused_window = state.keyboard_focused_window.clone();
                let Some(focused_window) = focused_window else {
                    return;
                };
                let focused_window = focused_window.clone();

                let keymap_state = state.keymap_state.as_ref().unwrap();
                let keycode = Keycode::from(key + MIN_KEYCODE);
                let keysym = keymap_state.key_get_one_sym(keycode);

                match key_state {
                    wl_keyboard::KeyState::Pressed if !keysym.is_modifier_key() => {
                        let mut keystroke =
                            Keystroke::from_xkb(&keymap_state, state.modifiers, keycode);
                        if let Some(mut compose) = state.compose_state.take() {
                            compose.feed(keysym);
                            match compose.status() {
                                xkb::Status::Composing => {
                                    state.pre_edit_text =
                                        compose.utf8().or(Keystroke::underlying_dead_key(keysym));
                                    let pre_edit =
                                        state.pre_edit_text.clone().unwrap_or(String::default());
                                    drop(state);
                                    focused_window.handle_ime(ImeInput::SetMarkedText(pre_edit));
                                    state = client.borrow_mut();
                                }

                                xkb::Status::Composed => {
                                    state.pre_edit_text.take();
                                    keystroke.ime_key = compose.utf8();
                                    keystroke.key = xkb::keysym_get_name(compose.keysym().unwrap());
                                }
                                xkb::Status::Cancelled => {
                                    let pre_edit = state.pre_edit_text.take();
                                    drop(state);
                                    if let Some(pre_edit) = pre_edit {
                                        focused_window.handle_ime(ImeInput::InsertText(pre_edit));
                                    }
                                    if let Some(current_key) =
                                        Keystroke::underlying_dead_key(keysym)
                                    {
                                        focused_window
                                            .handle_ime(ImeInput::SetMarkedText(current_key));
                                    }
                                    compose.feed(keysym);
                                    state = client.borrow_mut();
                                }
                                _ => {}
                            }
                            state.compose_state = Some(compose);
                        }
                        let input = PlatformInput::KeyDown(KeyDownEvent {
                            keystroke: keystroke,
                            is_held: false, // todo(linux)
                        });

                        state.repeat.current_id += 1;
                        state.repeat.current_keycode = Some(keycode);

                        let rate = state.repeat.characters_per_second;
                        let id = state.repeat.current_id;
                        state
                            .loop_handle
                            .insert_source(Timer::from_duration(state.repeat.delay), {
                                let input = input.clone();
                                move |event, _metadata, this| {
                                    let mut client = this.get_client();
                                    let mut state = client.borrow_mut();
                                    let is_repeating = id == state.repeat.current_id
                                        && state.repeat.current_keycode.is_some()
                                        && state.keyboard_focused_window.is_some();

                                    if !is_repeating {
                                        return TimeoutAction::Drop;
                                    }

                                    let focused_window =
                                        state.keyboard_focused_window.as_ref().unwrap().clone();

                                    drop(state);
                                    focused_window.handle_input(input.clone());

                                    TimeoutAction::ToDuration(Duration::from_secs(1) / rate)
                                }
                            })
                            .unwrap();

                        drop(state);
                        focused_window.handle_input(input);
                    }
                    wl_keyboard::KeyState::Released if !keysym.is_modifier_key() => {
                        let input = PlatformInput::KeyUp(KeyUpEvent {
                            keystroke: Keystroke::from_xkb(keymap_state, state.modifiers, keycode),
                        });

                        if state.repeat.current_keycode == Some(keycode) {
                            state.repeat.current_keycode = None;
                        }

                        drop(state);
                        focused_window.handle_input(input);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
impl Dispatch<zwp_text_input_v3::ZwpTextInputV3, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        text_input: &zwp_text_input_v3::ZwpTextInputV3,
        event: <zwp_text_input_v3::ZwpTextInputV3 as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        match event {
            zwp_text_input_v3::Event::Enter { surface } => {
                drop(state);
                this.enable_ime();
            }
            zwp_text_input_v3::Event::Leave { surface } => {
                drop(state);
                this.disable_ime();
            }
            zwp_text_input_v3::Event::CommitString { text } => {
                state.composing = false;
                let Some(window) = state.keyboard_focused_window.clone() else {
                    return;
                };

                if let Some(commit_text) = text {
                    drop(state);
                    // IBus Intercepts keys like `a`, `b`, but those keys are needed for vim mode.
                    // We should only send ASCII characters to Zed, otherwise a user could remap a letter like `か` or `相`.
                    if commit_text.len() == 1 {
                        window.handle_input(PlatformInput::KeyDown(KeyDownEvent {
                            keystroke: Keystroke {
                                modifiers: Modifiers::default(),
                                key: commit_text.clone(),
                                ime_key: Some(commit_text),
                            },
                            is_held: false,
                        }));
                    } else {
                        window.handle_ime(ImeInput::InsertText(commit_text));
                    }
                }
            }
            zwp_text_input_v3::Event::PreeditString {
                text,
                cursor_begin,
                cursor_end,
            } => {
                state.composing = true;
                state.pre_edit_text = text;
            }
            zwp_text_input_v3::Event::Done { serial } => {
                let last_serial = state.serial_tracker.get(SerialKind::InputMethod);
                state.serial_tracker.update(SerialKind::InputMethod, serial);
                let Some(window) = state.keyboard_focused_window.clone() else {
                    return;
                };

                if let Some(text) = state.pre_edit_text.take() {
                    drop(state);
                    window.handle_ime(ImeInput::SetMarkedText(text));
                    if let Some(area) = window.get_ime_area() {
                        text_input.set_cursor_rectangle(
                            area.origin.x.0 as i32,
                            area.origin.y.0 as i32,
                            area.size.width.0 as i32,
                            area.size.height.0 as i32,
                        );
                        if last_serial == serial {
                            text_input.commit();
                        }
                    }
                } else {
                    drop(state);
                    window.handle_ime(ImeInput::DeleteText);
                }
            }
            _ => {}
        }
    }
}

fn linux_button_to_gpui(button: u32) -> Option<MouseButton> {
    // These values are coming from <linux/input-event-codes.h>.
    const BTN_LEFT: u32 = 0x110;
    const BTN_RIGHT: u32 = 0x111;
    const BTN_MIDDLE: u32 = 0x112;
    const BTN_SIDE: u32 = 0x113;
    const BTN_EXTRA: u32 = 0x114;
    const BTN_FORWARD: u32 = 0x115;
    const BTN_BACK: u32 = 0x116;

    Some(match button {
        BTN_LEFT => MouseButton::Left,
        BTN_RIGHT => MouseButton::Right,
        BTN_MIDDLE => MouseButton::Middle,
        BTN_BACK | BTN_SIDE => MouseButton::Navigate(NavigationDirection::Back),
        BTN_FORWARD | BTN_EXTRA => MouseButton::Navigate(NavigationDirection::Forward),
        _ => return None,
    })
}

impl Dispatch<wl_pointer::WlPointer, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        wl_pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut client = this.get_client();
        let mut state = client.borrow_mut();

        match event {
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                state.serial_tracker.update(SerialKind::MouseEnter, serial);
                state.mouse_location = Some(point(px(surface_x as f32), px(surface_y as f32)));
                state.button_pressed = None;

                if let Some(window) = get_window(&mut state, &surface.id()) {
                    state.mouse_focused_window = Some(window.clone());
                    if state.enter_token.is_some() {
                        state.enter_token = None;
                    }
                    if let Some(style) = state.cursor_style {
                        if let Some(cursor_shape_device) = &state.cursor_shape_device {
                            cursor_shape_device.set_shape(serial, style.to_shape());
                        } else {
                            state
                                .cursor
                                .set_icon(&wl_pointer, serial, &style.to_icon_name());
                        }
                    }
                    drop(state);
                    window.set_focused(true);
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                if let Some(focused_window) = state.mouse_focused_window.clone() {
                    let input = PlatformInput::MouseExited(MouseExitEvent {
                        position: state.mouse_location.unwrap(),
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    });
                    state.mouse_focused_window = None;
                    state.mouse_location = None;
                    state.button_pressed = None;

                    drop(state);
                    focused_window.handle_input(input);
                    focused_window.set_focused(false);
                }
            }
            wl_pointer::Event::Motion {
                time,
                surface_x,
                surface_y,
                ..
            } => {
                if state.mouse_focused_window.is_none() {
                    return;
                }
                state.mouse_location = Some(point(px(surface_x as f32), px(surface_y as f32)));

                if let Some(window) = state.mouse_focused_window.clone() {
                    if state
                        .keyboard_focused_window
                        .as_ref()
                        .map_or(false, |keyboard_window| window.ptr_eq(&keyboard_window))
                    {
                        state.enter_token = None;
                    }
                    let input = PlatformInput::MouseMove(MouseMoveEvent {
                        position: state.mouse_location.unwrap(),
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    });
                    drop(state);
                    window.handle_input(input);
                }
            }
            wl_pointer::Event::Button {
                serial,
                button,
                state: WEnum::Value(button_state),
                ..
            } => {
                state.serial_tracker.update(SerialKind::MousePress, serial);
                let button = linux_button_to_gpui(button);
                let Some(button) = button else { return };
                if state.mouse_focused_window.is_none() {
                    return;
                }
                match button_state {
                    wl_pointer::ButtonState::Pressed => {
                        if let Some(window) = state.keyboard_focused_window.clone() {
                            if state.composing && state.text_input.is_some() {
                                let text_input = state.text_input.as_ref().unwrap();
                                drop(state);
                                // text_input_v3 don't have something like a reset function
                                this.disable_ime();
                                this.enable_ime();
                                window.handle_ime(ImeInput::UnmarkText);
                                state = client.borrow_mut();
                            } else if let (Some(text), Some(compose)) =
                                (state.pre_edit_text.take(), state.compose_state.as_mut())
                            {
                                compose.reset();
                                drop(state);
                                window.handle_ime(ImeInput::InsertText(text));
                                state = client.borrow_mut();
                            }
                        }
                        let click_elapsed = state.click.last_click.elapsed();

                        if click_elapsed < DOUBLE_CLICK_INTERVAL
                            && is_within_click_distance(
                                state.click.last_location,
                                state.mouse_location.unwrap(),
                            )
                        {
                            state.click.current_count += 1;
                        } else {
                            state.click.current_count = 1;
                        }

                        state.click.last_click = Instant::now();
                        state.click.last_location = state.mouse_location.unwrap();

                        state.button_pressed = Some(button);

                        if let Some(window) = state.mouse_focused_window.clone() {
                            let input = PlatformInput::MouseDown(MouseDownEvent {
                                button,
                                position: state.mouse_location.unwrap(),
                                modifiers: state.modifiers,
                                click_count: state.click.current_count,
                                first_mouse: state.enter_token.take().is_some(),
                            });
                            drop(state);
                            window.handle_input(input);
                        }
                    }
                    wl_pointer::ButtonState::Released => {
                        state.button_pressed = None;

                        if let Some(window) = state.mouse_focused_window.clone() {
                            let input = PlatformInput::MouseUp(MouseUpEvent {
                                button,
                                position: state.mouse_location.unwrap(),
                                modifiers: state.modifiers,
                                click_count: state.click.current_count,
                            });
                            drop(state);
                            window.handle_input(input);
                        }
                    }
                    _ => {}
                }
            }

            // Axis Events
            wl_pointer::Event::AxisSource {
                axis_source: WEnum::Value(axis_source),
            } => {
                state.axis_source = axis_source;
            }
            wl_pointer::Event::Axis {
                time,
                axis: WEnum::Value(axis),
                value,
                ..
            } => {
                if state.axis_source == AxisSource::Wheel {
                    return;
                }
                let axis_modifier = match axis {
                    wl_pointer::Axis::VerticalScroll => state.vertical_modifier,
                    wl_pointer::Axis::HorizontalScroll => state.horizontal_modifier,
                    _ => 1.0,
                };
                let supports_relative_direction =
                    wl_pointer.version() >= wl_pointer::EVT_AXIS_RELATIVE_DIRECTION_SINCE;
                state.scroll_event_received = true;
                let scroll_delta = state
                    .continuous_scroll_delta
                    .get_or_insert(point(px(0.0), px(0.0)));
                // TODO: Make nice feeling kinetic scrolling that integrates with the platform's scroll settings
                let modifier = 3.0;
                match axis {
                    wl_pointer::Axis::VerticalScroll => {
                        scroll_delta.y += px(value as f32 * modifier * axis_modifier);
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        scroll_delta.x += px(value as f32 * modifier * axis_modifier);
                    }
                    _ => unreachable!(),
                }
            }
            wl_pointer::Event::AxisDiscrete {
                axis: WEnum::Value(axis),
                discrete,
            } => {
                state.scroll_event_received = true;
                let axis_modifier = match axis {
                    wl_pointer::Axis::VerticalScroll => state.vertical_modifier,
                    wl_pointer::Axis::HorizontalScroll => state.horizontal_modifier,
                    _ => 1.0,
                };

                let scroll_delta = state.discrete_scroll_delta.get_or_insert(point(0.0, 0.0));
                match axis {
                    wl_pointer::Axis::VerticalScroll => {
                        scroll_delta.y += discrete as f32 * axis_modifier * SCROLL_LINES as f32;
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        scroll_delta.x += discrete as f32 * axis_modifier * SCROLL_LINES as f32;
                    }
                    _ => unreachable!(),
                }
            }
            wl_pointer::Event::AxisValue120 {
                axis: WEnum::Value(axis),
                value120,
            } => {
                state.scroll_event_received = true;
                let axis_modifier = match axis {
                    wl_pointer::Axis::VerticalScroll => state.vertical_modifier,
                    wl_pointer::Axis::HorizontalScroll => state.horizontal_modifier,
                    _ => unreachable!(),
                };

                let scroll_delta = state.discrete_scroll_delta.get_or_insert(point(0.0, 0.0));
                let wheel_percent = value120 as f32 / 120.0;
                match axis {
                    wl_pointer::Axis::VerticalScroll => {
                        scroll_delta.y += wheel_percent * axis_modifier * SCROLL_LINES as f32;
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        scroll_delta.x += wheel_percent * axis_modifier * SCROLL_LINES as f32;
                    }
                    _ => unreachable!(),
                }
            }
            wl_pointer::Event::Frame => {
                if state.scroll_event_received {
                    state.scroll_event_received = false;
                    let continuous = state.continuous_scroll_delta.take();
                    let discrete = state.discrete_scroll_delta.take();
                    if let Some(continuous) = continuous {
                        if let Some(window) = state.mouse_focused_window.clone() {
                            let input = PlatformInput::ScrollWheel(ScrollWheelEvent {
                                position: state.mouse_location.unwrap(),
                                delta: ScrollDelta::Pixels(continuous),
                                modifiers: state.modifiers,
                                touch_phase: TouchPhase::Moved,
                            });
                            drop(state);
                            window.handle_input(input);
                        }
                    } else if let Some(discrete) = discrete {
                        if let Some(window) = state.mouse_focused_window.clone() {
                            let input = PlatformInput::ScrollWheel(ScrollWheelEvent {
                                position: state.mouse_location.unwrap(),
                                delta: ScrollDelta::Lines(discrete),
                                modifiers: state.modifiers,
                                touch_phase: TouchPhase::Moved,
                            });
                            drop(state);
                            window.handle_input(input);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, ObjectId> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        _: &wp_fractional_scale_v1::WpFractionalScaleV1,
        event: <wp_fractional_scale_v1::WpFractionalScaleV1 as Proxy>::Event,
        surface_id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();

        let Some(window) = get_window(&mut state, surface_id) else {
            return;
        };

        drop(state);
        window.handle_fractional_scale_event(event);
    }
}

impl Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ObjectId>
    for WaylandClientStatePtr
{
    fn event(
        this: &mut Self,
        _: &zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
        event: zxdg_toplevel_decoration_v1::Event,
        surface_id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();
        let Some(window) = get_window(&mut state, surface_id) else {
            return;
        };

        drop(state);
        window.handle_toplevel_decoration_event(event);
    }
}

const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

impl Dispatch<wl_data_device::WlDataDevice, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        _: &wl_data_device::WlDataDevice,
        event: wl_data_device::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();

        match event {
            wl_data_device::Event::Enter {
                serial,
                surface,
                x,
                y,
                id: data_offer,
            } => {
                state.serial_tracker.update(SerialKind::DataDevice, serial);
                if let Some(data_offer) = data_offer {
                    let Some(drag_window) = get_window(&mut state, &surface.id()) else {
                        return;
                    };

                    const ACTIONS: DndAction = DndAction::Copy;
                    data_offer.set_actions(ACTIONS, ACTIONS);

                    let pipe = Pipe::new().unwrap();
                    data_offer.receive(FILE_LIST_MIME_TYPE.to_string(), unsafe {
                        BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
                    });
                    let fd = pipe.read;
                    drop(pipe.write);

                    let read_task = state
                        .common
                        .background_executor
                        .spawn(async { unsafe { read_fd(fd) } });

                    let this = this.clone();
                    state
                        .common
                        .foreground_executor
                        .spawn(async move {
                            let file_list = match read_task.await {
                                Ok(list) => list,
                                Err(err) => {
                                    log::error!("error reading drag and drop pipe: {err:?}");
                                    return;
                                }
                            };

                            let paths: SmallVec<[_; 2]> = file_list
                                .lines()
                                .map(|path| PathBuf::from(path.replace("file://", "")))
                                .collect();
                            let position = Point::new(x.into(), y.into());

                            // Prevent dropping text from other programs.
                            if paths.is_empty() {
                                data_offer.finish();
                                data_offer.destroy();
                                return;
                            }

                            let input = PlatformInput::FileDrop(FileDropEvent::Entered {
                                position,
                                paths: crate::ExternalPaths(paths),
                            });

                            let client = this.get_client();
                            let mut state = client.borrow_mut();
                            state.drag.data_offer = Some(data_offer);
                            state.drag.window = Some(drag_window.clone());
                            state.drag.position = position;

                            drop(state);
                            drag_window.handle_input(input);
                        })
                        .detach();
                }
            }
            wl_data_device::Event::Motion { x, y, .. } => {
                let Some(drag_window) = state.drag.window.clone() else {
                    return;
                };
                let position = Point::new(x.into(), y.into());
                state.drag.position = position;

                let input = PlatformInput::FileDrop(FileDropEvent::Pending { position });
                drop(state);
                drag_window.handle_input(input);
            }
            wl_data_device::Event::Leave => {
                let Some(drag_window) = state.drag.window.clone() else {
                    return;
                };
                let data_offer = state.drag.data_offer.clone().unwrap();
                data_offer.destroy();

                state.drag.data_offer = None;
                state.drag.window = None;

                let input = PlatformInput::FileDrop(FileDropEvent::Exited {});
                drop(state);
                drag_window.handle_input(input);
            }
            wl_data_device::Event::Drop => {
                let Some(drag_window) = state.drag.window.clone() else {
                    return;
                };
                let data_offer = state.drag.data_offer.clone().unwrap();
                data_offer.finish();
                data_offer.destroy();

                state.drag.data_offer = None;
                state.drag.window = None;

                let input = PlatformInput::FileDrop(FileDropEvent::Submit {
                    position: state.drag.position,
                });
                drop(state);
                drag_window.handle_input(input);
            }
            _ => {}
        }
    }

    event_created_child!(WaylandClientStatePtr, wl_data_device::WlDataDevice, [
        wl_data_device::EVT_DATA_OFFER_OPCODE => (wl_data_offer::WlDataOffer, ()),
    ]);
}

impl Dispatch<wl_data_offer::WlDataOffer, ()> for WaylandClientStatePtr {
    fn event(
        this: &mut Self,
        data_offer: &wl_data_offer::WlDataOffer,
        event: wl_data_offer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let client = this.get_client();
        let mut state = client.borrow_mut();

        match event {
            wl_data_offer::Event::Offer { mime_type } => {
                if mime_type == FILE_LIST_MIME_TYPE {
                    let serial = state.serial_tracker.get(SerialKind::DataDevice);
                    data_offer.accept(serial, Some(mime_type));
                }
            }
            _ => {}
        }
    }
}
