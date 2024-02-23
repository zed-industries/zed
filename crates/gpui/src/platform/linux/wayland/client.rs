use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use smol::Timer;
use wayland_backend::client::ObjectId;
use wayland_backend::protocol::WEnum;
use wayland_client::protocol::wl_callback::WlCallback;
use wayland_client::protocol::wl_pointer::AxisRelativeDirection;
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat,
        wl_shm, wl_shm_pool,
        wl_surface::{self, WlSurface},
    },
    Connection, Dispatch, EventQueue, Proxy, QueueHandle,
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

use crate::platform::linux::client::Client;
use crate::platform::linux::wayland::window::{WaylandDecorationState, WaylandWindow};
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::{
    platform::linux::wayland::window::WaylandWindowState, AnyWindowHandle, DisplayId, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    NavigationDirection, Pixels, PlatformDisplay, PlatformInput, Point, ScrollDelta,
    ScrollWheelEvent, TouchPhase, WindowOptions,
};

const MIN_KEYCODE: u32 = 8; // used to convert evdev scancode to xkb scancode

pub(crate) struct WaylandClientStateInner {
    compositor: Option<wl_compositor::WlCompositor>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    viewporter: Option<wp_viewporter::WpViewporter>,
    fractional_scale_manager: Option<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
    decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    windows: Vec<(xdg_surface::XdgSurface, Rc<WaylandWindowState>)>,
    platform_inner: Rc<LinuxPlatformInner>,
    wl_seat: Option<wl_seat::WlSeat>,
    keymap_state: Option<xkb::State>,
    repeat: KeyRepeat,
    modifiers: Modifiers,
    scroll_direction: f64,
    mouse_location: Option<Point<Pixels>>,
    button_pressed: Option<MouseButton>,
    mouse_focused_window: Option<Rc<WaylandWindowState>>,
    keyboard_focused_window: Option<Rc<WaylandWindowState>>,
}

#[derive(Clone)]
pub(crate) struct WaylandClientState(Rc<RefCell<WaylandClientStateInner>>);

pub(crate) struct KeyRepeat {
    rate: i32,
    delay: i32,
    current_id: u64,
    current_keysym: Option<xkb::Keysym>,
}

pub(crate) struct WaylandClient {
    platform_inner: Rc<LinuxPlatformInner>,
    conn: Arc<Connection>,
    state: WaylandClientState,
    event_queue: Mutex<EventQueue<WaylandClientState>>,
    qh: Arc<QueueHandle<WaylandClientState>>,
}

impl WaylandClient {
    pub(crate) fn new(linux_platform_inner: Rc<LinuxPlatformInner>, conn: Arc<Connection>) -> Self {
        let state = WaylandClientState(Rc::new(RefCell::new(WaylandClientStateInner {
            compositor: None,
            buffer: None,
            wm_base: None,
            viewporter: None,
            fractional_scale_manager: None,
            decoration_manager: None,
            windows: Vec::new(),
            platform_inner: Rc::clone(&linux_platform_inner),
            wl_seat: None,
            keymap_state: None,
            repeat: KeyRepeat {
                rate: 16,
                delay: 500,
                current_id: 0,
                current_keysym: None,
            },
            modifiers: Modifiers {
                shift: false,
                control: false,
                alt: false,
                function: false,
                command: false,
            },
            scroll_direction: -1.0,
            mouse_location: None,
            button_pressed: None,
            mouse_focused_window: None,
            keyboard_focused_window: None,
        })));
        let event_queue: EventQueue<WaylandClientState> = conn.new_event_queue();
        let qh = event_queue.handle();
        Self {
            platform_inner: linux_platform_inner,
            conn,
            state,
            event_queue: Mutex::new(event_queue),
            qh: Arc::new(qh),
        }
    }
}

impl Client for WaylandClient {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        let display = self.conn.display();
        let mut eq = self.event_queue.lock();
        let _registry = display.get_registry(&self.qh, ());

        eq.roundtrip(&mut self.state.clone()).unwrap();

        on_finish_launching();
        while !self.platform_inner.state.lock().quit_requested {
            eq.flush().unwrap();
            eq.dispatch_pending(&mut self.state.clone()).unwrap();
            if let Some(guard) = self.conn.prepare_read() {
                guard.read().unwrap();
                eq.dispatch_pending(&mut self.state.clone()).unwrap();
            }
            if let Ok(runnable) = self.platform_inner.main_receiver.try_recv() {
                runnable.run();
            }
        }
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        Vec::new()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        unimplemented!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.state.0.borrow_mut();

        let wm_base = state.wm_base.as_ref().unwrap();
        let compositor = state.compositor.as_ref().unwrap();
        let wl_surface = compositor.create_surface(&self.qh, ());
        let xdg_surface = wm_base.get_xdg_surface(&wl_surface, &self.qh, ());
        let toplevel = xdg_surface.get_toplevel(&self.qh, ());
        let wl_surface = Arc::new(wl_surface);

        // Attempt to set up window decorations based on the requested configuration
        //
        // Note that wayland compositors may either not support decorations at all, or may
        // support them but not allow clients to choose whether they are enabled or not.
        // We attempt to account for these cases here.

        if let Some(decoration_manager) = state.decoration_manager.as_ref() {
            // The protocol for managing decorations is present at least, but that doesn't
            // mean that the compositor will allow us to use it.

            let decoration =
                decoration_manager.get_toplevel_decoration(&toplevel, &self.qh, xdg_surface.id());

            // todo!(linux) - options.titlebar is lacking information required for wayland.
            //                Especially, whether a titlebar is wanted in itself.
            //
            // Removing the titlebar also removes the entire window frame (ie. the ability to
            // close, move and resize the window [snapping still works]). This needs additional
            // handling in Zed, in order to implement drag handlers on a titlebar element.
            //
            // Since all of this handling is not present, we request server-side decorations
            // for now as a stopgap solution.
            decoration.set_mode(zxdg_toplevel_decoration_v1::Mode::ServerSide);
        }

        let viewport = state
            .viewporter
            .as_ref()
            .map(|viewporter| viewporter.get_viewport(&wl_surface, &self.qh, ()));

        wl_surface.frame(&self.qh, wl_surface.clone());
        wl_surface.commit();

        let window_state = Rc::new(WaylandWindowState::new(
            &self.conn,
            wl_surface.clone(),
            viewport,
            Arc::new(toplevel),
            options,
        ));

        if let Some(fractional_scale_manager) = state.fractional_scale_manager.as_ref() {
            fractional_scale_manager.get_fractional_scale(&wl_surface, &self.qh, xdg_surface.id());
        }

        state.windows.push((xdg_surface, Rc::clone(&window_state)));
        Box::new(WaylandWindow(window_state))
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    state.compositor = Some(compositor);
                }
                "xdg_wm_base" => {
                    let wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, 1, qh, ());
                    state.wm_base = Some(wm_base);
                }
                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                    state.wl_seat = Some(seat);
                }
                "wp_fractional_scale_manager_v1" => {
                    let manager = registry
                        .bind::<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );
                    state.fractional_scale_manager = Some(manager);
                }
                "wp_viewporter" => {
                    let view_porter =
                        registry.bind::<wp_viewporter::WpViewporter, _, _>(name, 1, qh, ());
                    state.viewporter = Some(view_porter);
                }
                "zxdg_decoration_manager_v1" => {
                    // Unstable and optional
                    let decoration_manager = registry
                        .bind::<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );
                    state.decoration_manager = Some(decoration_manager);
                }
                _ => {}
            };
        }
    }
}

delegate_noop!(WaylandClientState: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientState: ignore wl_surface::WlSurface);
delegate_noop!(WaylandClientState: ignore wl_shm::WlShm);
delegate_noop!(WaylandClientState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientState: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientState: ignore wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1);
delegate_noop!(WaylandClientState: ignore zxdg_decoration_manager_v1::ZxdgDecorationManagerV1);
delegate_noop!(WaylandClientState: ignore wp_viewporter::WpViewporter);
delegate_noop!(WaylandClientState: ignore wp_viewport::WpViewport);

impl Dispatch<WlCallback, Arc<WlSurface>> for WaylandClientState {
    fn event(
        state: &mut Self,
        _: &WlCallback,
        event: wl_callback::Event,
        surf: &Arc<WlSurface>,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let wl_callback::Event::Done { .. } = event {
            for window in &state.windows {
                if window.1.surface.id() == surf.id() {
                    window.1.surface.frame(qh, surf.clone());
                    window.1.update();
                    window.1.surface.commit();
                }
            }
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);
            for window in &state.windows {
                if &window.0 == xdg_surface {
                    window.1.update();
                    window.1.surface.commit();
                    return;
                }
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        xdg_toplevel: &xdg_toplevel::XdgToplevel,
        event: <xdg_toplevel::XdgToplevel as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let xdg_toplevel::Event::Configure {
            width,
            height,
            states: _states,
        } = event
        {
            if width == 0 || height == 0 {
                return;
            }
            for window in &state.windows {
                if window.1.toplevel.id() == xdg_toplevel.id() {
                    window.1.resize(width, height);
                    window.1.surface.commit();
                    return;
                }
            }
        } else if let xdg_toplevel::Event::Close = event {
            state.windows.retain(|(_, window)| {
                if window.toplevel.id() == xdg_toplevel.id() {
                    window.toplevel.destroy();
                    false
                } else {
                    true
                }
            });
            state.platform_inner.state.lock().quit_requested |= state.windows.is_empty();
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for WaylandClientState {
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

impl Dispatch<wl_seat::WlSeat, ()> for WaylandClientState {
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
                seat.get_pointer(qh, ());
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for WaylandClientState {
    fn event(
        state_container: &mut Self,
        keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = state_container.0.borrow_mut();
        match event {
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                state.repeat.rate = rate;
                state.repeat.delay = delay;
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
                    .unwrap()
                }
                .unwrap();
                state.keymap_state = Some(xkb::State::new(&keymap));
            }
            wl_keyboard::Event::Enter { surface, .. } => {
                state.keyboard_focused_window = state
                    .windows
                    .iter()
                    .find(|&w| w.1.surface.id() == surface.id())
                    .map(|w| w.1.clone());

                if let Some(window) = &state.keyboard_focused_window {
                    window.set_focused(true);
                }
            }
            wl_keyboard::Event::Leave { surface, .. } => {
                let keyboard_focused_window = state
                    .windows
                    .iter()
                    .find(|&w| w.1.surface.id() == surface.id())
                    .map(|w| w.1.clone());

                if let Some(window) = keyboard_focused_window {
                    window.set_focused(false);
                }

                state.keyboard_focused_window = None;
            }
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => {
                let keymap_state = state.keymap_state.as_mut().unwrap();
                keymap_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);

                let shift =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE);
                let alt =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE);
                let control =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
                let command =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_LOGO, xkb::STATE_MODS_EFFECTIVE);

                state.modifiers.shift = shift;
                state.modifiers.alt = alt;
                state.modifiers.control = control;
                state.modifiers.command = command;
            }
            wl_keyboard::Event::Key {
                key,
                state: WEnum::Value(key_state),
                ..
            } => {
                let focused_window = &state.keyboard_focused_window;
                let Some(focused_window) = focused_window else {
                    return;
                };

                let keymap_state = state.keymap_state.as_ref().unwrap();
                let keycode = Keycode::from(key + MIN_KEYCODE);
                let keysym = keymap_state.key_get_one_sym(keycode);

                match key_state {
                    wl_keyboard::KeyState::Pressed => {
                        let input = PlatformInput::KeyDown(KeyDownEvent {
                            keystroke: Keystroke::from_xkb(keymap_state, state.modifiers, keycode),
                            is_held: false, // todo!(linux)
                        });

                        focused_window.handle_input(input.clone());

                        if !keysym.is_modifier_key() {
                            state.repeat.current_id += 1;
                            state.repeat.current_keysym = Some(keysym);

                            let rate = state.repeat.rate;
                            let delay = state.repeat.delay;
                            let id = state.repeat.current_id;
                            let keysym = state.repeat.current_keysym;
                            let state_container = state_container.clone();

                            state
                                .platform_inner
                                .foreground_executor
                                .spawn(async move {
                                    let mut wait_time = Duration::from_millis(delay as u64);

                                    loop {
                                        Timer::after(wait_time).await;

                                        let state = state_container.0.borrow_mut();
                                        let is_repeating = id == state.repeat.current_id
                                            && state.repeat.current_keysym.is_some()
                                            && state.keyboard_focused_window.is_some();
                                        if !is_repeating {
                                            return;
                                        }

                                        state
                                            .keyboard_focused_window
                                            .as_ref()
                                            .unwrap()
                                            .handle_input(input.clone());

                                        wait_time = Duration::from_millis(1000 / rate as u64);
                                    }
                                })
                                .detach();
                        }
                    }
                    wl_keyboard::KeyState::Released => {
                        focused_window.handle_input(PlatformInput::KeyUp(KeyUpEvent {
                            keystroke: Keystroke::from_xkb(keymap_state, state.modifiers, keycode),
                        }));

                        if !keysym.is_modifier_key() {
                            state.repeat.current_keysym = None;
                        }
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

impl Dispatch<wl_pointer::WlPointer, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        wl_pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        match event {
            wl_pointer::Event::Enter {
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                let mut mouse_focused_window = None;
                for window in &state.windows {
                    if window.1.surface.id() == surface.id() {
                        mouse_focused_window = Some(Rc::clone(&window.1));
                    }
                }
                if mouse_focused_window.is_some() {
                    state.mouse_focused_window = mouse_focused_window;
                }

                state.mouse_location = Some(Point {
                    x: Pixels::from(surface_x),
                    y: Pixels::from(surface_y),
                });
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
                state.mouse_location = Some(Point {
                    x: Pixels::from(surface_x),
                    y: Pixels::from(surface_y),
                });
                state.mouse_focused_window.as_ref().unwrap().handle_input(
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position: state.mouse_location.unwrap(),
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    }),
                );
            }
            wl_pointer::Event::Button {
                button,
                state: WEnum::Value(button_state),
                ..
            } => {
                let button = linux_button_to_gpui(button);
                let Some(button) = button else { return };
                if state.mouse_focused_window.is_none() || state.mouse_location.is_none() {
                    return;
                }
                match button_state {
                    wl_pointer::ButtonState::Pressed => {
                        state.button_pressed = Some(button);
                        state.mouse_focused_window.as_ref().unwrap().handle_input(
                            PlatformInput::MouseDown(MouseDownEvent {
                                button,
                                position: state.mouse_location.unwrap(),
                                modifiers: state.modifiers,
                                click_count: 1,
                            }),
                        );
                    }
                    wl_pointer::ButtonState::Released => {
                        state.button_pressed = None;
                        state.mouse_focused_window.as_ref().unwrap().handle_input(
                            PlatformInput::MouseUp(MouseUpEvent {
                                button,
                                position: state.mouse_location.unwrap(),
                                modifiers: Modifiers::default(),
                                click_count: 1,
                            }),
                        );
                    }
                    _ => {}
                }
            }
            wl_pointer::Event::AxisRelativeDirection {
                direction: WEnum::Value(direction),
                ..
            } => {
                state.scroll_direction = match direction {
                    AxisRelativeDirection::Identical => -1.0,
                    AxisRelativeDirection::Inverted => 1.0,
                    _ => -1.0,
                }
            }
            wl_pointer::Event::Axis {
                time,
                axis: WEnum::Value(axis),
                value,
                ..
            } => {
                let focused_window = &state.mouse_focused_window;
                let mouse_location = &state.mouse_location;
                if let (Some(focused_window), Some(mouse_location)) =
                    (focused_window, mouse_location)
                {
                    let value = value * state.scroll_direction;
                    focused_window.handle_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: *mouse_location,
                        delta: match axis {
                            wl_pointer::Axis::VerticalScroll => {
                                ScrollDelta::Pixels(Point::new(Pixels(0.0), Pixels(value as f32)))
                            }
                            wl_pointer::Axis::HorizontalScroll => {
                                ScrollDelta::Pixels(Point::new(Pixels(value as f32), Pixels(0.0)))
                            }
                            _ => unimplemented!(),
                        },
                        modifiers: state.modifiers,
                        touch_phase: TouchPhase::Started,
                    }))
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                let focused_window = &state.mouse_focused_window;
                if let Some(focused_window) = focused_window {
                    focused_window.handle_input(PlatformInput::MouseMove(MouseMoveEvent {
                        position: Point::<Pixels>::default(),
                        pressed_button: None,
                        modifiers: Modifiers::default(),
                    }));
                }
                state.mouse_focused_window = None;
                state.mouse_location = None;
            }
            _ => {}
        }
    }
}

impl Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, ObjectId> for WaylandClientState {
    fn event(
        state: &mut Self,
        _: &wp_fractional_scale_v1::WpFractionalScaleV1,
        event: <wp_fractional_scale_v1::WpFractionalScaleV1 as Proxy>::Event,
        id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let wp_fractional_scale_v1::Event::PreferredScale { scale, .. } = event {
            for window in &state.windows {
                if window.0.id() == *id {
                    window.1.rescale(scale as f32 / 120.0);
                    return;
                }
            }
        }
    }
}

impl Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ObjectId>
    for WaylandClientState
{
    fn event(
        state: &mut Self,
        _: &zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
        event: zxdg_toplevel_decoration_v1::Event,
        surface_id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.0.borrow_mut();
        if let zxdg_toplevel_decoration_v1::Event::Configure { mode, .. } = event {
            for window in &state.windows {
                if window.0.id() == *surface_id {
                    match mode {
                        WEnum::Value(zxdg_toplevel_decoration_v1::Mode::ServerSide) => {
                            window
                                .1
                                .set_decoration_state(WaylandDecorationState::Server);
                        }
                        WEnum::Value(zxdg_toplevel_decoration_v1::Mode::ClientSide) => {
                            window
                                .1
                                .set_decoration_state(WaylandDecorationState::Client);
                        }
                        WEnum::Value(_) => {
                            log::warn!("Unknown decoration mode");
                        }
                        WEnum::Unknown(v) => {
                            log::warn!("Unknown decoration mode: {}", v);
                        }
                    }
                    return;
                }
            }
        }
    }
}
