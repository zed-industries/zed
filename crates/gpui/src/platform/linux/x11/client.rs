use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::{Duration, Instant};

use calloop::{EventLoop, LoopHandle};
use collections::HashMap;
use copypasta::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};
use copypasta::ClipboardProvider;

use util::ResultExt;
use x11rb::connection::{Connection, RequestConnection};
use x11rb::errors::ConnectionError;
use x11rb::protocol::randr::ConnectionExt as _;
use x11rb::protocol::xkb::ConnectionExt as _;
use x11rb::protocol::xproto::ConnectionExt as _;
use x11rb::protocol::{randr, xkb, xproto, Event};
use x11rb::xcb_ffi::XCBConnection;
use xkbc::x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb as xkbc;

use crate::platform::linux::LinuxClient;
use crate::platform::{LinuxCommon, PlatformWindow};
use crate::{
    px, AnyWindowHandle, Bounds, CursorStyle, DisplayId, Modifiers, ModifiersChangedEvent, Pixels,
    PlatformDisplay, PlatformInput, Point, ScrollDelta, Size, TouchPhase, WindowParams,
};

use super::{super::SCROLL_LINES, X11Display, X11Window, XcbAtoms};
use super::{button_of_key, modifiers_from_state};
use crate::platform::linux::is_within_click_distance;
use crate::platform::linux::platform::DOUBLE_CLICK_INTERVAL;
use calloop::{
    generic::{FdWrapper, Generic},
    RegistrationToken,
};

pub(crate) struct WindowRef {
    window: X11Window,
    refresh_event_token: RegistrationToken,
}

impl Deref for WindowRef {
    type Target = X11Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

pub struct X11ClientState {
    pub(crate) loop_handle: LoopHandle<'static, X11Client>,
    pub(crate) event_loop: Option<calloop::EventLoop<'static, X11Client>>,

    pub(crate) last_click: Instant,
    pub(crate) last_location: Point<Pixels>,
    pub(crate) current_count: usize,

    pub(crate) xcb_connection: Rc<XCBConnection>,
    pub(crate) x_root_index: usize,
    pub(crate) atoms: XcbAtoms,
    pub(crate) windows: HashMap<xproto::Window, WindowRef>,
    pub(crate) focused_window: Option<xproto::Window>,
    pub(crate) xkb: xkbc::State,

    pub(crate) common: LinuxCommon,
    pub(crate) clipboard: X11ClipboardContext<Clipboard>,
    pub(crate) primary: X11ClipboardContext<Primary>,
}

#[derive(Clone)]
pub(crate) struct X11Client(Rc<RefCell<X11ClientState>>);

impl X11Client {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle.insert_source(main_receiver, |event, _, _: &mut X11Client| {
            if let calloop::channel::Event::Msg(runnable) = event {
                runnable.run();
            }
        });

        let (xcb_connection, x_root_index) = XCBConnection::connect(None).unwrap();
        xcb_connection
            .prefetch_extension_information(xkb::X11_EXTENSION_NAME)
            .unwrap();
        xcb_connection
            .prefetch_extension_information(randr::X11_EXTENSION_NAME)
            .unwrap();

        let atoms = XcbAtoms::new(&xcb_connection).unwrap();
        let xkb = xcb_connection
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap();

        let atoms = atoms.reply().unwrap();
        let xkb = xkb.reply().unwrap();
        let events = xkb::EventType::STATE_NOTIFY;
        xcb_connection
            .xkb_select_events(
                xkb::ID::USE_CORE_KBD.into(),
                0u8.into(),
                events,
                0u8.into(),
                0u8.into(),
                &xkb::SelectEventsAux::new(),
            )
            .unwrap();
        assert!(xkb.supported);

        let xkb_state = {
            let xkb_context = xkbc::Context::new(xkbc::CONTEXT_NO_FLAGS);
            let xkb_device_id = xkbc::x11::get_core_keyboard_device_id(&xcb_connection);
            let xkb_keymap = xkbc::x11::keymap_new_from_device(
                &xkb_context,
                &xcb_connection,
                xkb_device_id,
                xkbc::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkbc::x11::state_new_from_device(&xkb_keymap, &xcb_connection, xkb_device_id)
        };

        let clipboard = X11ClipboardContext::<Clipboard>::new().unwrap();
        let primary = X11ClipboardContext::<Primary>::new().unwrap();

        let xcb_connection = Rc::new(xcb_connection);

        // Safety: Safe if xcb::Connection always returns a valid fd
        let fd = unsafe { FdWrapper::new(Rc::clone(&xcb_connection)) };

        handle
            .insert_source(
                Generic::new_with_error::<ConnectionError>(
                    fd,
                    calloop::Interest::READ,
                    calloop::Mode::Level,
                ),
                {
                    let xcb_connection = xcb_connection.clone();
                    move |_readiness, _, client| {
                        while let Some(event) = xcb_connection.poll_for_event()? {
                            client.handle_event(event);
                        }
                        Ok(calloop::PostAction::Continue)
                    }
                },
            )
            .expect("Failed to initialize x11 event source");

        X11Client(Rc::new(RefCell::new(X11ClientState {
            event_loop: Some(event_loop),
            loop_handle: handle,
            common,
            last_click: Instant::now(),
            last_location: Point::new(px(0.0), px(0.0)),
            current_count: 0,

            xcb_connection,
            x_root_index,
            atoms,
            windows: HashMap::default(),
            focused_window: None,
            xkb: xkb_state,
            clipboard,
            primary,
        })))
    }

    fn get_window(&self, win: xproto::Window) -> Option<X11Window> {
        let state = self.0.borrow();
        state
            .windows
            .get(&win)
            .map(|window_reference| window_reference.window.clone())
    }

    fn handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::ClientMessage(event) => {
                let [atom, ..] = event.data.as_data32();
                let mut state = self.0.borrow_mut();

                if atom == state.atoms.WM_DELETE_WINDOW {
                    // window "x" button clicked by user, we gracefully exit
                    let window_ref = state.windows.remove(&event.window)?;

                    state.loop_handle.remove(window_ref.refresh_event_token);
                    window_ref.window.destroy();

                    if state.windows.is_empty() {
                        state.common.signal.stop();
                    }
                }
            }
            Event::ConfigureNotify(event) => {
                let bounds = Bounds {
                    origin: Point {
                        x: event.x.into(),
                        y: event.y.into(),
                    },
                    size: Size {
                        width: event.width.into(),
                        height: event.height.into(),
                    },
                };
                let window = self.get_window(event.window)?;
                window.configure(bounds);
            }
            Event::Expose(event) => {
                let window = self.get_window(event.window)?;
                window.refresh();
            }
            Event::FocusIn(event) => {
                let window = self.get_window(event.event)?;
                window.set_focused(true);
                self.0.borrow_mut().focused_window = Some(event.event);
            }
            Event::FocusOut(event) => {
                let window = self.get_window(event.event)?;
                window.set_focused(false);
                self.0.borrow_mut().focused_window = None;
            }
            Event::XkbStateNotify(event) => {
                let mut state = self.0.borrow_mut();
                state.xkb.update_mask(
                    event.base_mods.into(),
                    event.latched_mods.into(),
                    event.locked_mods.into(),
                    0,
                    0,
                    event.locked_group.into(),
                );
                let modifiers = Modifiers::from_xkb(&state.xkb);
                let focused_window_id = state.focused_window?;
                drop(state);

                let focused_window = self.get_window(focused_window_id)?;
                focused_window.handle_input(PlatformInput::ModifiersChanged(
                    ModifiersChangedEvent { modifiers },
                ));
            }
            Event::KeyPress(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                let keystroke = {
                    let code = event.detail.into();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Down);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    keystroke
                };

                drop(state);
                window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                    keystroke,
                    is_held: false,
                }));
            }
            Event::KeyRelease(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                let keystroke = {
                    let code = event.detail.into();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Up);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    keystroke
                };
                drop(state);
                window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
            }
            Event::ButtonPress(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                if let Some(button) = button_of_key(event.detail) {
                    let click_elapsed = state.last_click.elapsed();

                    if click_elapsed < DOUBLE_CLICK_INTERVAL
                        && is_within_click_distance(state.last_location, position)
                    {
                        state.current_count += 1;
                    } else {
                        state.current_count = 1;
                    }

                    state.last_click = Instant::now();
                    state.last_location = position;
                    let current_count = state.current_count;

                    drop(state);
                    window.handle_input(PlatformInput::MouseDown(crate::MouseDownEvent {
                        button,
                        position,
                        modifiers,
                        click_count: current_count,
                        first_mouse: false,
                    }));
                } else if event.detail >= 4 && event.detail <= 5 {
                    // https://stackoverflow.com/questions/15510472/scrollwheel-event-in-x11
                    let scroll_direction = if event.detail == 4 { 1.0 } else { -1.0 };
                    let scroll_y = SCROLL_LINES * scroll_direction;

                    drop(state);
                    window.handle_input(PlatformInput::ScrollWheel(crate::ScrollWheelEvent {
                        position,
                        delta: ScrollDelta::Lines(Point::new(0.0, scroll_y as f32)),
                        modifiers,
                        touch_phase: TouchPhase::Moved,
                    }));
                } else {
                    log::warn!("Unknown button press: {event:?}");
                }
            }
            Event::ButtonRelease(event) => {
                let window = self.get_window(event.event)?;
                let state = self.0.borrow();
                let modifiers = modifiers_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                if let Some(button) = button_of_key(event.detail) {
                    let click_count = state.current_count;
                    drop(state);
                    window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                        button,
                        position,
                        modifiers,
                        click_count,
                    }));
                }
            }
            Event::MotionNotify(event) => {
                let window = self.get_window(event.event)?;
                let pressed_button = super::button_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                let modifiers = modifiers_from_state(event.state);
                window.handle_input(PlatformInput::MouseMove(crate::MouseMoveEvent {
                    pressed_button,
                    position,
                    modifiers,
                }));
            }
            Event::LeaveNotify(event) => {
                let window = self.get_window(event.event)?;
                let pressed_button = super::button_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                let modifiers = modifiers_from_state(event.state);
                window.handle_input(PlatformInput::MouseExited(crate::MouseExitEvent {
                    pressed_button,
                    position,
                    modifiers,
                }));
            }
            _ => {}
        };

        Some(())
    }
}

impl LinuxClient for X11Client {
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();
        let setup = state.xcb_connection.setup();
        setup
            .roots
            .iter()
            .enumerate()
            .filter_map(|(root_id, _)| {
                Some(Rc::new(X11Display::new(&state.xcb_connection, root_id)?)
                    as Rc<dyn PlatformDisplay>)
            })
            .collect()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(
            X11Display::new(&state.xcb_connection, state.x_root_index)
                .expect("There should always be a root index"),
        ))
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(X11Display::new(
            &state.xcb_connection,
            id.0 as usize,
        )?))
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.0.borrow_mut();
        let x_window = state.xcb_connection.generate_id().unwrap();

        let window = X11Window::new(
            params,
            &state.xcb_connection,
            state.x_root_index,
            x_window,
            &state.atoms,
        );

        let screen_resources = state
            .xcb_connection
            .randr_get_screen_resources(x_window)
            .unwrap()
            .reply()
            .expect("Could not find available screens");

        let mode = screen_resources
            .crtcs
            .iter()
            .find_map(|crtc| {
                let crtc_info = state
                    .xcb_connection
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

        let refresh_event_token = state
            .loop_handle
            .insert_source(calloop::timer::Timer::immediate(), {
                let refresh_duration = mode_refresh_rate(mode);
                move |mut instant, (), client| {
                    let state = client.0.borrow_mut();
                    state
                        .xcb_connection
                        .send_event(
                            false,
                            x_window,
                            xproto::EventMask::EXPOSURE,
                            xproto::ExposeEvent {
                                response_type: xproto::EXPOSE_EVENT,
                                sequence: 0,
                                window: x_window,
                                x: 0,
                                y: 0,
                                width: 0,
                                height: 0,
                                count: 1,
                            },
                        )
                        .unwrap();
                    let _ = state.xcb_connection.flush().unwrap();
                    // Take into account that some frames have been skipped
                    let now = time::Instant::now();
                    while instant < now {
                        instant += refresh_duration;
                    }
                    calloop::timer::TimeoutAction::ToInstant(instant)
                }
            })
            .expect("Failed to initialize refresh timer");

        let window_ref = WindowRef {
            window: window.clone(),
            refresh_event_token,
        };

        state.windows.insert(x_window, window_ref);
        Box::new(window)
    }

    //todo(linux)
    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        self.0.borrow_mut().clipboard.set_contents(item.text);
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .clipboard
            .get_contents()
            .ok()
            .map(|text| crate::ClipboardItem {
                text,
                metadata: None,
            })
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .borrow_mut()
            .event_loop
            .take()
            .expect("App is already running");

        event_loop.run(None, &mut self.clone(), |_| {}).log_err();
    }
}

// Adatpted from:
// https://docs.rs/winit/0.29.11/src/winit/platform_impl/linux/x11/monitor.rs.html#103-111
pub fn mode_refresh_rate(mode: &randr::ModeInfo) -> Duration {
    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    let micros = 1_000_000_000 / millihertz;
    log::info!("Refreshing at {} micros", micros);
    Duration::from_micros(micros)
}
