use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use async_task::Runnable;
use calloop::timer::{TimeoutAction, Timer};
use calloop::{EventLoop, LoopHandle};
use calloop_wayland_source::WaylandSource;
use collections::HashMap;
use copypasta::wayland_clipboard::{create_clipboards_from_external, Clipboard, Primary};
use copypasta::ClipboardProvider;
use util::ResultExt;
use wayland_backend::client::ObjectId;
use wayland_backend::protocol::WEnum;
use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::wl_callback::{self, WlCallback};
use wayland_client::protocol::wl_output;
use wayland_client::protocol::wl_pointer::{AxisRelativeDirection, AxisSource};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat, wl_shm,
        wl_shm_pool, wl_surface,
    },
    Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols::wp::fractional_scale::v1::client::{
    wp_fractional_scale_manager_v1, wp_fractional_scale_v1,
};
use wayland_protocols::wp::viewporter::client::{wp_viewport, wp_viewporter};
use wayland_protocols::xdg::decoration::zv1::client::{
    zxdg_decoration_manager_v1, zxdg_toplevel_decoration_v1,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use xkbcommon::xkb::ffi::XKB_KEYMAP_FORMAT_TEXT_V1;
use xkbcommon::xkb::{self, Keycode, KEYMAP_COMPILE_NO_FLAGS};

use super::super::DOUBLE_CLICK_INTERVAL;
use super::window::{WaylandWindowState, WaylandWindowStatePtr};
use crate::platform::linux::is_within_click_distance;
use crate::platform::linux::wayland::cursor::Cursor;
use crate::platform::linux::wayland::window::WaylandWindow;
use crate::platform::linux::LinuxClient;
use crate::platform::PlatformWindow;
use crate::{point, px, ForegroundExecutor, MouseExitEvent};
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
    pub compositor: wl_compositor::WlCompositor,
    pub wm_base: xdg_wm_base::XdgWmBase,
    pub shm: wl_shm::WlShm,
    pub viewporter: Option<wp_viewporter::WpViewporter>,
    pub fractional_scale_manager:
        Option<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
    pub decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    pub executor: ForegroundExecutor,
}

impl Globals {
    fn new(
        globals: GlobalList,
        executor: ForegroundExecutor,
        qh: QueueHandle<WaylandClientStatePtr>,
    ) -> Self {
        Globals {
            compositor: globals
                .bind(
                    &qh,
                    wl_surface::REQ_SET_BUFFER_SCALE_SINCE
                        ..=wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE,
                    (),
                )
                .unwrap(),
            shm: globals.bind(&qh, 1..=1, ()).unwrap(),
            wm_base: globals.bind(&qh, 1..=1, ()).unwrap(),
            viewporter: globals.bind(&qh, 1..=1, ()).ok(),
            fractional_scale_manager: globals.bind(&qh, 1..=1, ()).ok(),
            decoration_manager: globals.bind(&qh, 1..=1, ()).ok(),
            executor,
            qh,
        }
    }
}

pub(crate) struct WaylandClientState {
    globals: Globals,
    wl_pointer: Option<wl_pointer::WlPointer>,
    // Surface to Window mapping
    windows: HashMap<ObjectId, WaylandWindowStatePtr>,
    // Output to scale mapping
    output_scales: HashMap<ObjectId, i32>,
    keymap_state: Option<xkb::State>,
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
    cursor_icon_name: String,
    cursor: Cursor,
    clipboard: Clipboard,
    primary: Primary,
    event_loop: Option<EventLoop<'static, WaylandClientStatePtr>>,
    common: LinuxCommon,
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
    current_keysym: Option<xkb::Keysym>,
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
                state.mouse_focused_window = Some(window);
            }
        }
    }
}

#[derive(Clone)]
pub struct WaylandClient(Rc<RefCell<WaylandClientState>>);

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
        let mut outputs = HashMap::default();

        globals.contents().with_list(|list| {
            for global in list {
                match &global.interface[..] {
                    "wl_seat" => {
                        globals.registry().bind::<wl_seat::WlSeat, _, _>(
                            global.name,
                            wl_seat_version(global.version),
                            &qh,
                            (),
                        );
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
        let (primary, clipboard) = unsafe { create_clipboards_from_external(display) };

        let event_loop = EventLoop::<WaylandClientStatePtr>::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle.insert_source(main_receiver, |event, _, _: &mut WaylandClientStatePtr| {
            if let calloop::channel::Event::Msg(runnable) = event {
                runnable.run();
            }
        });

        let globals = Globals::new(globals, common.foreground_executor.clone(), qh);

        let cursor = Cursor::new(&conn, &globals, 24);

        let mut state = Rc::new(RefCell::new(WaylandClientState {
            globals,
            wl_pointer: None,
            output_scales: outputs,
            windows: HashMap::default(),
            common,
            keymap_state: None,
            click: ClickState {
                last_click: Instant::now(),
                last_location: Point::new(px(0.0), px(0.0)),
                current_count: 0,
            },
            repeat: KeyRepeat {
                characters_per_second: 16,
                delay: Duration::from_millis(500),
                current_id: 0,
                current_keysym: None,
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
            cursor_icon_name: "arrow".to_string(),
            enter_token: None,
            cursor,
            clipboard,
            primary,
            event_loop: Some(event_loop),
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
        );
        state.windows.insert(surface_id, window.0.clone());

        Box::new(window)
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        // Based on cursor names from https://gitlab.gnome.org/GNOME/adwaita-icon-theme (GNOME)
        // and https://github.com/KDE/breeze (KDE). Both of them seem to be also derived from
        // Web CSS cursor names: https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#values
        let cursor_icon_name = match style {
            CursorStyle::Arrow => "arrow",
            CursorStyle::IBeam => "text",
            CursorStyle::Crosshair => "crosshair",
            CursorStyle::ClosedHand => "grabbing",
            CursorStyle::OpenHand => "grab",
            CursorStyle::PointingHand => "pointer",
            CursorStyle::ResizeLeft => "w-resize",
            CursorStyle::ResizeRight => "e-resize",
            CursorStyle::ResizeLeftRight => "ew-resize",
            CursorStyle::ResizeUp => "n-resize",
            CursorStyle::ResizeDown => "s-resize",
            CursorStyle::ResizeUpDown => "ns-resize",
            CursorStyle::DisappearingItem => "grabbing", // todo(linux) - couldn't find equivalent icon in linux
            CursorStyle::IBeamCursorForVerticalLayout => "vertical-text",
            CursorStyle::OperationNotAllowed => "not-allowed",
            CursorStyle::DragLink => "alias",
            CursorStyle::DragCopy => "copy",
            CursorStyle::ContextualMenu => "context-menu",
        }
        .to_string();

        let mut state = self.0.borrow_mut();
        state.cursor_icon_name = cursor_icon_name.clone();
        if state.mouse_focused_window.is_some() {
            let wl_pointer = state
                .wl_pointer
                .clone()
                .expect("window is focused by pointer");
            state.cursor.set_icon(&wl_pointer, &cursor_icon_name);
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
        self.0.borrow_mut().primary.set_contents(item.text);
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        self.0.borrow_mut().clipboard.set_contents(item.text);
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .primary
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

delegate_noop!(WaylandClientStatePtr: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientStatePtr: ignore wl_shm::WlShm);
delegate_noop!(WaylandClientStatePtr: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientStatePtr: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientStatePtr: ignore wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1);
delegate_noop!(WaylandClientStatePtr: ignore zxdg_decoration_manager_v1::ZxdgDecorationManagerV1);
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
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }
            if capabilities.contains(wl_seat::Capability::Pointer) {
                let client = state.get_client();
                let mut state = client.borrow_mut();
                state.wl_pointer = Some(seat.get_pointer(qh, ()));
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
                let keymap = unsafe {
                    xkb::Keymap::new_from_fd(
                        &xkb::Context::new(xkb::CONTEXT_NO_FLAGS),
                        fd,
                        size as usize,
                        XKB_KEYMAP_FORMAT_TEXT_V1,
                        KEYMAP_COMPILE_NO_FLAGS,
                    )
                    .log_err()
                    .flatten()
                    .expect("Failed to create keymap")
                };
                state.keymap_state = Some(xkb::State::new(&keymap));
            }
            wl_keyboard::Event::Enter { surface, .. } => {
                state.keyboard_focused_window = get_window(&mut state, &surface.id());

                if let Some(window) = state.keyboard_focused_window.clone() {
                    drop(state);
                    window.set_focused(true);
                }
            }
            wl_keyboard::Event::Leave { surface, .. } => {
                let keyboard_focused_window = get_window(&mut state, &surface.id());
                state.keyboard_focused_window = None;

                if let Some(window) = keyboard_focused_window {
                    drop(state);
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
                key,
                state: WEnum::Value(key_state),
                ..
            } => {
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
                        let input = PlatformInput::KeyDown(KeyDownEvent {
                            keystroke: Keystroke::from_xkb(keymap_state, state.modifiers, keycode),
                            is_held: false, // todo(linux)
                        });

                        state.repeat.current_id += 1;
                        state.repeat.current_keysym = Some(keysym);

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
                                        && state.repeat.current_keysym.is_some()
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

                        state.repeat.current_keysym = None;

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
        let cursor_icon_name = state.cursor_icon_name.clone();

        match event {
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                state.mouse_location = Some(point(px(surface_x as f32), px(surface_y as f32)));

                if let Some(window) = get_window(&mut state, &surface.id()) {
                    state.enter_token = Some(());
                    state.mouse_focused_window = Some(window.clone());
                    state.cursor.mark_dirty();
                    state.cursor.set_serial_id(serial);
                    state
                        .cursor
                        .set_icon(&wl_pointer, cursor_icon_name.as_str());
                    drop(state);
                    window.set_focused(true);
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                if let Some(focused_window) = state.mouse_focused_window.clone() {
                    state.enter_token.take();
                    let input = PlatformInput::MouseExited(MouseExitEvent {
                        position: state.mouse_location.unwrap(),
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    });
                    state.mouse_focused_window = None;
                    state.mouse_location = None;

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
                button,
                state: WEnum::Value(button_state),
                ..
            } => {
                let button = linux_button_to_gpui(button);
                let Some(button) = button else { return };
                if state.mouse_focused_window.is_none() {
                    return;
                }
                match button_state {
                    wl_pointer::ButtonState::Pressed => {
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
                let axis_source = state.axis_source;
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

                // TODO: Make nice feeling kinetic scrolling that integrates with the platform's scroll settings
                let modifier = 3.0;

                let scroll_delta = state.discrete_scroll_delta.get_or_insert(point(0.0, 0.0));
                match axis {
                    wl_pointer::Axis::VerticalScroll => {
                        scroll_delta.y += discrete as f32 * axis_modifier * modifier;
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        scroll_delta.x += discrete as f32 * axis_modifier * modifier;
                    }
                    _ => unreachable!(),
                }
            }
            wl_pointer::Event::AxisRelativeDirection {
                axis: WEnum::Value(axis),
                direction: WEnum::Value(direction),
            } => match (axis, direction) {
                (wl_pointer::Axis::VerticalScroll, AxisRelativeDirection::Identical) => {
                    state.vertical_modifier = -1.0
                }
                (wl_pointer::Axis::VerticalScroll, AxisRelativeDirection::Inverted) => {
                    state.vertical_modifier = 1.0
                }
                (wl_pointer::Axis::HorizontalScroll, AxisRelativeDirection::Identical) => {
                    state.horizontal_modifier = -1.0
                }
                (wl_pointer::Axis::HorizontalScroll, AxisRelativeDirection::Inverted) => {
                    state.horizontal_modifier = 1.0
                }
                _ => unreachable!(),
            },
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
                        scroll_delta.y += wheel_percent * axis_modifier;
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        scroll_delta.x += wheel_percent * axis_modifier;
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
