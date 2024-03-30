use std::cell::RefCell;
use std::mem;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use calloop::timer::{TimeoutAction, Timer};
use calloop::LoopHandle;
use calloop_wayland_source::WaylandSource;
use copypasta::wayland_clipboard::{create_clipboards_from_external, Clipboard, Primary};
use copypasta::ClipboardProvider;
use wayland_backend::client::ObjectId;
use wayland_backend::protocol::WEnum;
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_callback::WlCallback;
use wayland_client::protocol::wl_output;
use wayland_client::protocol::wl_pointer::{AxisRelativeDirection, AxisSource};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat,
        wl_shm, wl_shm_pool,
        wl_surface::{self, WlSurface},
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
use crate::platform::linux::client::Client;
use crate::platform::linux::util::is_within_click_distance;
use crate::platform::linux::wayland::cursor::Cursor;
use crate::platform::linux::wayland::window::{WaylandDecorationState, WaylandWindow};
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::WindowParams;
use crate::{
    platform::linux::wayland::window::WaylandWindowState, AnyWindowHandle, CursorStyle, DisplayId,
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection, Pixels, PlatformDisplay,
    PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchPhase,
};
use crate::{point, px};

/// Used to convert evdev scancode to xkb scancode
const MIN_KEYCODE: u32 = 8;

pub(crate) struct WaylandClientStateInner {
    compositor: wl_compositor::WlCompositor,
    wm_base: xdg_wm_base::XdgWmBase,
    shm: wl_shm::WlShm,
    viewporter: Option<wp_viewporter::WpViewporter>,
    fractional_scale_manager: Option<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
    decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    windows: Vec<(xdg_surface::XdgSurface, Rc<WaylandWindowState>)>,
    outputs: Vec<(wl_output::WlOutput, Rc<RefCell<OutputState>>)>,
    platform_inner: Rc<LinuxPlatformInner>,
    keymap_state: Option<xkb::State>,
    click_state: ClickState,
    repeat: KeyRepeat,
    modifiers: Modifiers,
    scroll_direction: f64,
    axis_source: AxisSource,
    mouse_location: Point<Pixels>,
    button_pressed: Option<MouseButton>,
    mouse_focused_window: Option<Rc<WaylandWindowState>>,
    keyboard_focused_window: Option<Rc<WaylandWindowState>>,
    loop_handle: Rc<LoopHandle<'static, ()>>,
}

pub(crate) struct CursorState {
    cursor_icon_name: String,
    cursor: Option<Cursor>,
}

#[derive(Clone)]
pub(crate) struct WaylandClientState {
    client_state_inner: Rc<RefCell<WaylandClientStateInner>>,
    cursor_state: Rc<RefCell<CursorState>>,
    clipboard: Rc<RefCell<Clipboard>>,
    primary: Rc<RefCell<Primary>>,
}

struct ClickState {
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

pub(crate) struct WaylandClient {
    platform_inner: Rc<LinuxPlatformInner>,
    state: WaylandClientState,
    qh: Arc<QueueHandle<WaylandClientState>>,
}

const WL_SEAT_MIN_VERSION: u32 = 4;
const WL_OUTPUT_VERSION: u32 = 2;

fn wl_seat_version(version: u32) -> u32 {
    if version >= wl_pointer::EVT_AXIS_VALUE120_SINCE {
        wl_pointer::EVT_AXIS_VALUE120_SINCE
    } else if version >= WL_SEAT_MIN_VERSION {
        WL_SEAT_MIN_VERSION
    } else {
        panic!(
            "wl_seat below required version: {} < {}",
            version, WL_SEAT_MIN_VERSION
        );
    }
}

impl WaylandClient {
    pub(crate) fn new(linux_platform_inner: Rc<LinuxPlatformInner>) -> Self {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, mut event_queue) = registry_queue_init::<WaylandClientState>(&conn).unwrap();
        let qh = event_queue.handle();
        let mut outputs = Vec::new();

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
                    "wl_output" => outputs.push((
                        globals.registry().bind::<wl_output::WlOutput, _, _>(
                            global.name,
                            WL_OUTPUT_VERSION,
                            &qh,
                            (),
                        ),
                        Rc::new(RefCell::new(OutputState::default())),
                    )),
                    _ => {}
                }
            }
        });

        let display = conn.backend().display_ptr() as *mut std::ffi::c_void;
        let (primary, clipboard) = unsafe { create_clipboards_from_external(display) };

        let mut state_inner = Rc::new(RefCell::new(WaylandClientStateInner {
            compositor: globals
                .bind(&qh, 1..=wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE, ())
                .unwrap(),
            wm_base: globals.bind(&qh, 1..=1, ()).unwrap(),
            shm: globals.bind(&qh, 1..=1, ()).unwrap(),
            outputs,
            viewporter: globals.bind(&qh, 1..=1, ()).ok(),
            fractional_scale_manager: globals.bind(&qh, 1..=1, ()).ok(),
            decoration_manager: globals.bind(&qh, 1..=1, ()).ok(),
            windows: Vec::new(),
            platform_inner: Rc::clone(&linux_platform_inner),
            keymap_state: None,
            click_state: ClickState {
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
                command: false,
            },
            scroll_direction: -1.0,
            axis_source: AxisSource::Wheel,
            mouse_location: point(px(0.0), px(0.0)),
            button_pressed: None,
            mouse_focused_window: None,
            keyboard_focused_window: None,
            loop_handle: Rc::clone(&linux_platform_inner.loop_handle),
        }));

        let mut cursor_state = Rc::new(RefCell::new(CursorState {
            cursor_icon_name: "arrow".to_string(),
            cursor: None,
        }));

        let source = WaylandSource::new(conn, event_queue);

        let mut state = WaylandClientState {
            client_state_inner: Rc::clone(&state_inner),
            cursor_state: Rc::clone(&cursor_state),
            clipboard: Rc::new(RefCell::new(clipboard)),
            primary: Rc::new(RefCell::new(primary)),
        };
        let mut state_loop = state.clone();
        linux_platform_inner
            .loop_handle
            .insert_source(source, move |_, queue, _| {
                queue.dispatch_pending(&mut state_loop)
            })
            .unwrap();

        Self {
            platform_inner: linux_platform_inner,
            state,
            qh: Arc::new(qh),
        }
    }
}

impl Client for WaylandClient {
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
        options: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.state.client_state_inner.borrow_mut();

        let wl_surface = state.compositor.create_surface(&self.qh, ());
        let xdg_surface = state.wm_base.get_xdg_surface(&wl_surface, &self.qh, ());
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

            // todo(linux) - options.titlebar is lacking information required for wayland.
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

        let window_state: Rc<WaylandWindowState> = Rc::new(WaylandWindowState::new(
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

        self.state.cursor_state.borrow_mut().cursor_icon_name = cursor_icon_name;
    }

    fn get_clipboard(&self) -> Rc<RefCell<dyn ClipboardProvider>> {
        self.state.clipboard.clone()
    }

    fn get_primary(&self) -> Rc<RefCell<dyn ClipboardProvider>> {
        self.state.primary.clone()
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for WaylandClientState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = state.client_state_inner.borrow_mut();
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => match &interface[..] {
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, wl_seat_version(version), qh, ());
                }
                "wl_output" => {
                    state.outputs.push((
                        registry.bind::<wl_output::WlOutput, _, _>(name, WL_OUTPUT_VERSION, qh, ()),
                        Rc::new(RefCell::new(OutputState::default())),
                    ));
                }
                _ => {}
            },
            wl_registry::Event::GlobalRemove { name: _ } => {}
            _ => {}
        }
    }
}

delegate_noop!(WaylandClientState: ignore wl_compositor::WlCompositor);
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
        let mut state = state.client_state_inner.borrow_mut();
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

impl Dispatch<wl_surface::WlSurface, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        surface: &wl_surface::WlSurface,
        event: <wl_surface::WlSurface as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.client_state_inner.borrow_mut();

        // We use `WpFractionalScale` instead to set the scale if it's available
        // or give up on scaling if `WlSurface::set_buffer_scale` isn't available
        if state.fractional_scale_manager.is_some()
            || state.compositor.version() < wl_surface::REQ_SET_BUFFER_SCALE_SINCE
        {
            return;
        }

        let Some(window) = state
            .windows
            .iter()
            .map(|(_, state)| state)
            .find(|state| &*state.surface == surface)
        else {
            return;
        };

        let mut outputs = window.outputs.borrow_mut();

        match event {
            wl_surface::Event::Enter { output } => {
                // We use `PreferredBufferScale` instead to set the scale if it's available
                if surface.version() >= wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    return;
                }
                let mut scale = 1;
                for global_output in &state.outputs {
                    if output == global_output.0 {
                        outputs.insert(output.id());
                        scale = scale.max(global_output.1.borrow().scale.get());
                    } else if outputs.contains(&global_output.0.id()) {
                        scale = scale.max(global_output.1.borrow().scale.get());
                    }
                }
                window.rescale(scale as f32);
                window.surface.set_buffer_scale(scale as i32);
            }
            wl_surface::Event::Leave { output } => {
                // We use `PreferredBufferScale` instead to set the scale if it's available
                if surface.version() >= wl_surface::EVT_PREFERRED_BUFFER_SCALE_SINCE {
                    return;
                }

                outputs.remove(&output.id());

                let mut scale = 1;
                for global_output in &state.outputs {
                    if outputs.contains(&global_output.0.id()) {
                        scale = scale.max(global_output.1.borrow().scale.get());
                    }
                }
                window.rescale(scale as f32);
                window.surface.set_buffer_scale(scale as i32);
            }
            wl_surface::Event::PreferredBufferScale { factor } => {
                window.rescale(factor as f32);
                surface.set_buffer_scale(factor);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct OutputState {
    scale: NonZeroU32,
}

impl Default for OutputState {
    fn default() -> Self {
        Self {
            scale: NonZeroU32::new(1).unwrap(),
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        output: &wl_output::WlOutput,
        event: <wl_output::WlOutput as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let mut state = state.client_state_inner.borrow_mut();
        let mut output_state = state
            .outputs
            .iter_mut()
            .find(|(o, _)| o == output)
            .map(|(_, state)| state)
            .unwrap()
            .borrow_mut();
        match event {
            wl_output::Event::Scale { factor } => {
                if factor > 0 {
                    output_state.scale = NonZeroU32::new(factor as u32).unwrap();
                }
            }
            _ => {}
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
        let mut state = state.client_state_inner.borrow_mut();
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
        let mut state = state.client_state_inner.borrow_mut();
        if let xdg_toplevel::Event::Configure {
            width,
            height,
            states,
        } = event
        {
            let width = NonZeroU32::new(width as u32);
            let height = NonZeroU32::new(height as u32);
            let fullscreen = states.contains(&(xdg_toplevel::State::Fullscreen as u8));
            for window in &state.windows {
                if window.1.toplevel.id() == xdg_toplevel.id() {
                    window.1.resize(width, height);
                    window.1.set_fullscreen(fullscreen);
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
            state.platform_inner.loop_signal.stop();
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
        this: &mut Self,
        keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = this.client_state_inner.borrow_mut();
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
                let focused_window = focused_window.clone();

                let keymap_state = state.keymap_state.as_ref().unwrap();
                let keycode = Keycode::from(key + MIN_KEYCODE);
                let keysym = keymap_state.key_get_one_sym(keycode);

                match key_state {
                    wl_keyboard::KeyState::Pressed => {
                        let input = if keysym.is_modifier_key() {
                            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                                modifiers: state.modifiers,
                            })
                        } else {
                            PlatformInput::KeyDown(KeyDownEvent {
                                keystroke: Keystroke::from_xkb(
                                    keymap_state,
                                    state.modifiers,
                                    keycode,
                                ),
                                is_held: false, // todo(linux)
                            })
                        };

                        if !keysym.is_modifier_key() {
                            state.repeat.current_id += 1;
                            state.repeat.current_keysym = Some(keysym);

                            let rate = state.repeat.characters_per_second;
                            let delay = state.repeat.delay;
                            let id = state.repeat.current_id;
                            let this = this.clone();

                            let timer = Timer::from_duration(delay);
                            let state_ = Rc::clone(&this.client_state_inner);
                            let input_ = input.clone();
                            state
                                .loop_handle
                                .insert_source(timer, move |event, _metadata, shared_data| {
                                    let state_ = state_.borrow_mut();
                                    let is_repeating = id == state_.repeat.current_id
                                        && state_.repeat.current_keysym.is_some()
                                        && state_.keyboard_focused_window.is_some();

                                    if !is_repeating {
                                        return TimeoutAction::Drop;
                                    }

                                    let focused_window =
                                        state_.keyboard_focused_window.as_ref().unwrap().clone();

                                    drop(state_);

                                    focused_window.handle_input(input_.clone());

                                    TimeoutAction::ToDuration(Duration::from_secs(1) / rate)
                                })
                                .unwrap();
                        }

                        drop(state);

                        focused_window.handle_input(input);
                    }
                    wl_keyboard::KeyState::Released => {
                        let input = if keysym.is_modifier_key() {
                            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                                modifiers: state.modifiers,
                            })
                        } else {
                            PlatformInput::KeyUp(KeyUpEvent {
                                keystroke: Keystroke::from_xkb(
                                    keymap_state,
                                    state.modifiers,
                                    keycode,
                                ),
                            })
                        };

                        if !keysym.is_modifier_key()
                            && (match state.repeat.current_keysym {
                                Some(repeat_keysym) => keysym == repeat_keysym,
                                None => false,
                            })
                        {
                            state.repeat.current_keysym = None;
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
        self_state: &mut Self,
        wl_pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let mut state = self_state.client_state_inner.borrow_mut();
        {
            let mut cursor_state = self_state.cursor_state.borrow_mut();
            if cursor_state.cursor.is_none() {
                cursor_state.cursor =
                    Some(Cursor::new(&conn, &state.compositor, &qh, &state.shm, 24));
            }
        }

        match event {
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                let windows = mem::take(&mut state.windows);
                for window in windows.iter() {
                    if window.1.surface.id() == surface.id() {
                        window.1.set_focused(true);
                        state.mouse_focused_window = Some(window.1.clone());
                        let mut cursor_state = self_state.cursor_state.borrow_mut();
                        let cursor_icon_name = cursor_state.cursor_icon_name.clone();
                        if let Some(mut cursor) = cursor_state.cursor.as_mut() {
                            cursor.set_serial_id(serial);
                            cursor.set_icon(&wl_pointer, cursor_icon_name);
                        }
                    }
                }
                state.windows = windows;
                state.mouse_location = point(px(surface_x as f32), px(surface_y as f32));
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
                state.mouse_location = point(px(surface_x as f32), px(surface_y as f32));

                state.mouse_focused_window.as_ref().unwrap().handle_input(
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position: state.mouse_location,
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    }),
                );
                let mut cursor_state = self_state.cursor_state.borrow_mut();
                let cursor_icon_name = cursor_state.cursor_icon_name.clone();
                if let Some(mut cursor) = cursor_state.cursor.as_mut() {
                    cursor.set_icon(&wl_pointer, cursor_icon_name);
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
                        let click_elapsed = state.click_state.last_click.elapsed();

                        if click_elapsed < DOUBLE_CLICK_INTERVAL
                            && is_within_click_distance(
                                state.click_state.last_location,
                                state.mouse_location,
                            )
                        {
                            state.click_state.current_count += 1;
                        } else {
                            state.click_state.current_count = 1;
                        }

                        state.click_state.last_click = Instant::now();
                        state.click_state.last_location = state.mouse_location;

                        state.button_pressed = Some(button);
                        state.mouse_focused_window.as_ref().unwrap().handle_input(
                            PlatformInput::MouseDown(MouseDownEvent {
                                button,
                                position: state.mouse_location,
                                modifiers: state.modifiers,
                                click_count: state.click_state.current_count,
                                first_mouse: false,
                            }),
                        );
                    }
                    wl_pointer::ButtonState::Released => {
                        state.button_pressed = None;
                        state.mouse_focused_window.as_ref().unwrap().handle_input(
                            PlatformInput::MouseUp(MouseUpEvent {
                                button,
                                position: state.mouse_location,
                                modifiers: state.modifiers,
                                click_count: state.click_state.current_count,
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
            wl_pointer::Event::AxisSource {
                axis_source: WEnum::Value(axis_source),
            } => {
                state.axis_source = axis_source;
            }
            wl_pointer::Event::AxisValue120 {
                axis: WEnum::Value(axis),
                value120,
            } => {
                if let Some(focused_window) = &state.mouse_focused_window {
                    let value = value120 as f64 * state.scroll_direction;
                    focused_window.handle_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: state.mouse_location,
                        delta: match axis {
                            wl_pointer::Axis::VerticalScroll => {
                                ScrollDelta::Pixels(point(px(0.0), px(value as f32)))
                            }
                            wl_pointer::Axis::HorizontalScroll => {
                                ScrollDelta::Pixels(point(px(value as f32), px(0.0)))
                            }
                            _ => unimplemented!(),
                        },
                        modifiers: state.modifiers,
                        touch_phase: TouchPhase::Moved,
                    }))
                }
            }
            wl_pointer::Event::Axis {
                time,
                axis: WEnum::Value(axis),
                value,
                ..
            } => {
                // We handle discrete scroll events with `AxisValue120`.
                if wl_pointer.version() >= wl_pointer::EVT_AXIS_VALUE120_SINCE
                    && state.axis_source == AxisSource::Wheel
                {
                    return;
                }
                if let Some(focused_window) = &state.mouse_focused_window {
                    let value = value * state.scroll_direction;
                    focused_window.handle_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: state.mouse_location,
                        delta: match axis {
                            wl_pointer::Axis::VerticalScroll => {
                                ScrollDelta::Pixels(point(px(0.0), px(value as f32)))
                            }
                            wl_pointer::Axis::HorizontalScroll => {
                                ScrollDelta::Pixels(point(px(value as f32), px(0.0)))
                            }
                            _ => unimplemented!(),
                        },
                        modifiers: state.modifiers,
                        touch_phase: TouchPhase::Moved,
                    }))
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                if let Some(focused_window) = &state.mouse_focused_window {
                    focused_window.handle_input(PlatformInput::MouseMove(MouseMoveEvent {
                        position: Point::default(),
                        pressed_button: None,
                        modifiers: Modifiers::default(),
                    }));
                    focused_window.set_focused(false);
                }
                state.mouse_focused_window = None;
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
        let mut state = state.client_state_inner.borrow_mut();
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
        let mut state = state.client_state_inner.borrow_mut();
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
