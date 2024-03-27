use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use collections::HashMap;
use copypasta::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};
use copypasta::ClipboardProvider;

use x11rb::connection::{Connection, RequestConnection};
use x11rb::errors::ConnectionError;
use x11rb::protocol::randr::ConnectionExt as _;
use x11rb::protocol::xkb::ConnectionExt as _;
use x11rb::protocol::xproto::ConnectionExt as _;
use x11rb::protocol::{randr, xkb, xproto, Event};
use x11rb::xcb_ffi::XCBConnection;
use xkbc::x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb as xkbc;

use crate::platform::linux::client::Client;
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::{
    px, AnyWindowHandle, Bounds, CursorStyle, DisplayId, Pixels, PlatformDisplay, PlatformInput,
    Point, ScrollDelta, Size, TouchPhase, WindowParams,
};

use super::{super::SCROLL_LINES, X11Display, X11Window, X11WindowState, XcbAtoms};
use crate::platform::linux::platform::DOUBLE_CLICK_INTERVAL;
use crate::platform::linux::util::is_within_click_distance;
use calloop::{
    generic::{FdWrapper, Generic},
    RegistrationToken,
};

struct WindowRef {
    state: Rc<X11WindowState>,
    refresh_event_token: RegistrationToken,
}

struct X11ClientState {
    windows: HashMap<xproto::Window, WindowRef>,
    xkb: xkbc::State,
    clipboard: Rc<RefCell<X11ClipboardContext<Clipboard>>>,
    primary: Rc<RefCell<X11ClipboardContext<Primary>>>,
    click_state: ClickState,
}

struct ClickState {
    last_click: Instant,
    last_location: Point<Pixels>,
    current_count: usize,
}

pub(crate) struct X11Client {
    platform_inner: Rc<LinuxPlatformInner>,
    xcb_connection: Rc<XCBConnection>,
    x_root_index: usize,
    atoms: XcbAtoms,
    state: RefCell<X11ClientState>,
}

impl X11Client {
    pub(crate) fn new(inner: Rc<LinuxPlatformInner>) -> Rc<Self> {
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

        let click_state = ClickState {
            last_click: Instant::now(),
            last_location: Point::new(px(0.0), px(0.0)),
            current_count: 0,
        };
        let client: Rc<X11Client> = Rc::new(Self {
            platform_inner: inner.clone(),
            xcb_connection: Rc::clone(&xcb_connection),
            x_root_index,
            atoms,
            state: RefCell::new(X11ClientState {
                windows: HashMap::default(),
                xkb: xkb_state,
                clipboard: Rc::new(RefCell::new(clipboard)),
                primary: Rc::new(RefCell::new(primary)),
                click_state,
            }),
        });

        // Safety: Safe if xcb::Connection always returns a valid fd
        let fd = unsafe { FdWrapper::new(Rc::clone(&xcb_connection)) };

        inner
            .loop_handle
            .insert_source(
                Generic::new_with_error::<ConnectionError>(
                    fd,
                    calloop::Interest::READ,
                    calloop::Mode::Level,
                ),
                {
                    let client = Rc::clone(&client);
                    move |_readiness, _, _| {
                        while let Some(event) = xcb_connection.poll_for_event()? {
                            client.handle_event(event);
                        }
                        Ok(calloop::PostAction::Continue)
                    }
                },
            )
            .expect("Failed to initialize x11 event source");

        client
    }

    fn get_window(&self, win: xproto::Window) -> Option<Rc<X11WindowState>> {
        let state = self.state.borrow();
        state.windows.get(&win).map(|wr| Rc::clone(&wr.state))
    }

    fn handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::ClientMessage(event) => {
                let [atom, ..] = event.data.as_data32();
                if atom == self.atoms.WM_DELETE_WINDOW {
                    // window "x" button clicked by user, we gracefully exit
                    let window_ref = self
                        .state
                        .borrow_mut()
                        .windows
                        .remove(&event.window)
                        .unwrap();

                    self.platform_inner
                        .loop_handle
                        .remove(window_ref.refresh_event_token);
                    window_ref.state.destroy();

                    if self.state.borrow().windows.is_empty() {
                        self.platform_inner.loop_signal.stop();
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
            }
            Event::FocusOut(event) => {
                let window = self.get_window(event.event)?;
                window.set_focused(false);
            }
            Event::KeyPress(event) => {
                let window = self.get_window(event.event)?;
                let modifiers = super::modifiers_from_state(event.state);
                let keystroke = {
                    let code = event.detail.into();
                    let mut state = self.state.borrow_mut();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Down);
                    keystroke
                };

                window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                    keystroke,
                    is_held: false,
                }));
            }
            Event::KeyRelease(event) => {
                let window = self.get_window(event.event)?;
                let modifiers = super::modifiers_from_state(event.state);
                let keystroke = {
                    let code = event.detail.into();
                    let mut state = self.state.borrow_mut();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Up);
                    keystroke
                };

                window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
            }
            Event::ButtonPress(event) => {
                let window = self.get_window(event.event)?;
                let modifiers = super::modifiers_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                if let Some(button) = super::button_of_key(event.detail) {
                    let mut state = self.state.borrow_mut();
                    let click_elapsed = state.click_state.last_click.elapsed();

                    if click_elapsed < DOUBLE_CLICK_INTERVAL
                        && is_within_click_distance(state.click_state.last_location, position)
                    {
                        state.click_state.current_count += 1;
                    } else {
                        state.click_state.current_count = 1;
                    }

                    state.click_state.last_click = Instant::now();
                    state.click_state.last_location = position;

                    window.handle_input(PlatformInput::MouseDown(crate::MouseDownEvent {
                        button,
                        position,
                        modifiers,
                        click_count: state.click_state.current_count,
                        first_mouse: false,
                    }));
                } else if event.detail >= 4 && event.detail <= 5 {
                    // https://stackoverflow.com/questions/15510472/scrollwheel-event-in-x11
                    let scroll_direction = if event.detail == 4 { 1.0 } else { -1.0 };
                    let scroll_y = SCROLL_LINES * scroll_direction;
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
                let modifiers = super::modifiers_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                let state = self.state.borrow();
                if let Some(button) = super::button_of_key(event.detail) {
                    window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                        button,
                        position,
                        modifiers,
                        click_count: state.click_state.current_count,
                    }));
                }
            }
            Event::MotionNotify(event) => {
                let window = self.get_window(event.event)?;
                let pressed_button = super::button_from_state(event.state);
                let position =
                    Point::new((event.event_x as f32).into(), (event.event_y as f32).into());
                let modifiers = super::modifiers_from_state(event.state);
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
                let modifiers = super::modifiers_from_state(event.state);
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

impl Client for X11Client {
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let setup = self.xcb_connection.setup();
        setup
            .roots
            .iter()
            .enumerate()
            .filter_map(|(root_id, _)| {
                Some(Rc::new(X11Display::new(&self.xcb_connection, root_id)?)
                    as Rc<dyn PlatformDisplay>)
            })
            .collect()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(X11Display::new(
            &self.xcb_connection,
            id.0 as usize,
        )?))
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(
            X11Display::new(&self.xcb_connection, self.x_root_index)
                .expect("There should always be a root index"),
        ))
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        let x_window = self.xcb_connection.generate_id().unwrap();

        let window_ptr = Rc::new(X11WindowState::new(
            options,
            &self.xcb_connection,
            self.x_root_index,
            x_window,
            &self.atoms,
        ));

        let screen_resources = self
            .xcb_connection
            .randr_get_screen_resources(x_window)
            .unwrap()
            .reply()
            .expect("TODO");

        let mode = screen_resources
            .crtcs
            .iter()
            .find_map(|crtc| {
                let crtc_info = self
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

        // .expect("Missing screen mode for crtc specified mode id");

        let refresh_event_token = self
            .platform_inner
            .loop_handle
            .insert_source(calloop::timer::Timer::immediate(), {
                let refresh_duration = mode_refresh_rate(mode);
                let xcb_connection = Rc::clone(&self.xcb_connection);
                move |mut instant, (), _| {
                    xcb_connection
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
                    let _ = xcb_connection.flush().unwrap();
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
            state: Rc::clone(&window_ptr),
            refresh_event_token,
        };
        self.state.borrow_mut().windows.insert(x_window, window_ref);
        Box::new(X11Window(window_ptr))
    }

    //todo(linux)
    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn get_clipboard(&self) -> Rc<RefCell<dyn ClipboardProvider>> {
        self.state.borrow().clipboard.clone()
    }

    fn get_primary(&self) -> Rc<RefCell<dyn ClipboardProvider>> {
        self.state.borrow().primary.clone()
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
