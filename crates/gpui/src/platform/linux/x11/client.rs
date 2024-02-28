use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use xcb::{x, Xid as _};
use xkbcommon::xkb;

use collections::{HashMap, HashSet};

use crate::platform::linux::client::Client;
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::{
    AnyWindowHandle, Bounds, DisplayId, PlatformDisplay, PlatformInput, Point, ScrollDelta, Size,
    TouchPhase, WindowOptions,
};

use super::{X11Display, X11Window, X11WindowState, XcbAtoms};
use calloop::generic::{FdWrapper, Generic};

pub(crate) struct X11ClientState {
    pub(crate) windows: HashMap<x::Window, Rc<X11WindowState>>,
    pub(crate) windows_to_refresh: HashSet<x::Window>,
    xkb: xkbcommon::xkb::State,
}

pub(crate) struct X11Client {
    platform_inner: Rc<LinuxPlatformInner>,
    xcb_connection: Rc<xcb::Connection>,
    x_root_index: i32,
    atoms: XcbAtoms,
    refresh_millis: Cell<u64>,
    state: RefCell<X11ClientState>,
}

impl X11Client {
    pub(crate) fn new(inner: Rc<LinuxPlatformInner>) -> Rc<Self> {
        let (xcb_connection, x_root_index) = xcb::Connection::connect_with_extensions(
            None,
            &[
                xcb::Extension::Present,
                xcb::Extension::Xkb,
                xcb::Extension::RandR,
            ],
            &[],
        )
        .unwrap();

        let xkb_ver = xcb_connection
            .wait_for_reply(xcb_connection.send_request(&xcb::xkb::UseExtension {
                wanted_major: xcb::xkb::MAJOR_VERSION as u16,
                wanted_minor: xcb::xkb::MINOR_VERSION as u16,
            }))
            .unwrap();
        assert!(xkb_ver.supported());

        let atoms = XcbAtoms::intern_all(&xcb_connection).unwrap();
        let xcb_connection = Rc::new(xcb_connection);
        let xkb_context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let xkb_device_id = xkb::x11::get_core_keyboard_device_id(&xcb_connection);
        let xkb_keymap = xkb::x11::keymap_new_from_device(
            &xkb_context,
            &xcb_connection,
            xkb_device_id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );

        let xkb_state =
            xkb::x11::state_new_from_device(&xkb_keymap, &xcb_connection, xkb_device_id);

        let client: Rc<X11Client> = Rc::new(Self {
            platform_inner: inner.clone(),
            xcb_connection: xcb_connection.clone(),
            x_root_index,
            atoms,
            refresh_millis: Cell::new(16),
            state: RefCell::new(X11ClientState {
                windows: HashMap::default(),
                windows_to_refresh: HashSet::default(),
                xkb: xkb_state,
            }),
        });

        // Safety: Safe if xcb::Connection always returns a valid fd
        let fd = unsafe { FdWrapper::new(xcb_connection.clone()) };

        inner
            .loop_handle
            .insert_source(
                Generic::new_with_error::<xcb::Error>(
                    fd,
                    calloop::Interest::READ,
                    calloop::Mode::Level,
                ),
                {
                    let client = client.clone();
                    move |readiness, _, _| {
                        if readiness.readable || readiness.error {
                            while let Some(event) = xcb_connection.poll_for_event()? {
                                client.handle_event(event);
                            }
                        }
                        Ok(calloop::PostAction::Continue)
                    }
                },
            )
            .expect("Failed to initialize x11 event source");

        inner
            .loop_handle
            .insert_source(
                calloop::timer::Timer::from_duration(Duration::from_millis(
                    client.refresh_millis.get(),
                )),
                {
                    let client = client.clone();
                    move |_, _, _| {
                        client.present();
                        calloop::timer::TimeoutAction::ToDuration(Duration::from_millis(
                            client.refresh_millis.get(),
                        ))
                    }
                },
            )
            .expect("Failed to initialize refresh timer");

        client
    }

    fn get_window(&self, win: x::Window) -> Option<Rc<X11WindowState>> {
        let state = self.state.borrow();
        state.windows.get(&win).cloned()
    }

    fn present(&self) {
        let state = self.state.borrow_mut();
        for window_state in state.windows.values() {
            window_state.refresh();
        }
    }

    fn handle_event(&self, event: xcb::Event) -> Option<()> {
        match event {
            xcb::Event::X(x::Event::ClientMessage(event)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = event.data() {
                    if atom == self.atoms.wm_del_window.resource_id() {
                        self.state
                            .borrow_mut()
                            .windows_to_refresh
                            .remove(&event.window());
                        // window "x" button clicked by user, we gracefully exit
                        let window = self
                            .state
                            .borrow_mut()
                            .windows
                            .remove(&event.window())
                            .unwrap();
                        window.destroy();
                        let state = self.state.borrow();
                        if state.windows.is_empty() {
                            self.platform_inner.loop_signal.stop();
                        }
                    }
                }
            }
            xcb::Event::X(x::Event::ConfigureNotify(event)) => {
                let bounds = Bounds {
                    origin: Point {
                        x: event.x().into(),
                        y: event.y().into(),
                    },
                    size: Size {
                        width: event.width().into(),
                        height: event.height().into(),
                    },
                };
                let window = self.get_window(event.window())?;
                window.configure(bounds);
            }
            xcb::Event::X(x::Event::FocusIn(event)) => {
                let window = self.get_window(event.event())?;
                window.set_focused(true);
            }
            xcb::Event::X(x::Event::FocusOut(event)) => {
                let window = self.get_window(event.event())?;
                window.set_focused(false);
            }
            xcb::Event::X(x::Event::KeyPress(event)) => {
                let window = self.get_window(event.event())?;
                let modifiers = super::modifiers_from_state(event.state());
                let keystroke = {
                    let code = event.detail().into();
                    let mut state = self.state.borrow_mut();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkb::KeyDirection::Down);
                    keystroke
                };

                window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                    keystroke,
                    is_held: false,
                }));
            }
            xcb::Event::X(x::Event::KeyRelease(event)) => {
                let window = self.get_window(event.event())?;
                let modifiers = super::modifiers_from_state(event.state());
                let keystroke = {
                    let code = event.detail().into();
                    let mut state = self.state.borrow_mut();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkb::KeyDirection::Up);
                    keystroke
                };

                window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
            }
            xcb::Event::X(x::Event::ButtonPress(event)) => {
                let window = self.get_window(event.event())?;
                let modifiers = super::modifiers_from_state(event.state());
                let position = Point::new(
                    (event.event_x() as f32).into(),
                    (event.event_y() as f32).into(),
                );
                if let Some(button) = super::button_of_key(event.detail()) {
                    window.handle_input(PlatformInput::MouseDown(crate::MouseDownEvent {
                        button,
                        position,
                        modifiers,
                        click_count: 1,
                    }));
                } else if event.detail() >= 4 && event.detail() <= 5 {
                    // https://stackoverflow.com/questions/15510472/scrollwheel-event-in-x11
                    let delta_x = if event.detail() == 4 { 1.0 } else { -1.0 };
                    window.handle_input(PlatformInput::ScrollWheel(crate::ScrollWheelEvent {
                        position,
                        delta: ScrollDelta::Lines(Point::new(0.0, delta_x)),
                        modifiers,
                        touch_phase: TouchPhase::default(),
                    }));
                } else {
                    log::warn!("Unknown button press: {event:?}");
                }
            }
            xcb::Event::X(x::Event::ButtonRelease(event)) => {
                let window = self.get_window(event.event())?;
                let modifiers = super::modifiers_from_state(event.state());
                let position = Point::new(
                    (event.event_x() as f32).into(),
                    (event.event_y() as f32).into(),
                );
                if let Some(button) = super::button_of_key(event.detail()) {
                    window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                        button,
                        position,
                        modifiers,
                        click_count: 1,
                    }));
                }
            }
            xcb::Event::X(x::Event::MotionNotify(event)) => {
                let window = self.get_window(event.event())?;
                let pressed_button = super::button_from_state(event.state());
                let position = Point::new(
                    (event.event_x() as f32).into(),
                    (event.event_y() as f32).into(),
                );
                let modifiers = super::modifiers_from_state(event.state());
                window.handle_input(PlatformInput::MouseMove(crate::MouseMoveEvent {
                    pressed_button,
                    position,
                    modifiers,
                }));
            }
            xcb::Event::X(x::Event::LeaveNotify(event)) => {
                let window = self.get_window(event.event())?;
                let pressed_button = super::button_from_state(event.state());
                let position = Point::new(
                    (event.event_x() as f32).into(),
                    (event.event_y() as f32).into(),
                );
                let modifiers = super::modifiers_from_state(event.state());
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
        let setup = self.xcb_connection.get_setup();
        setup
            .roots()
            .enumerate()
            .map(|(root_id, _)| {
                Rc::new(X11Display::new(&self.xcb_connection, root_id as i32))
                    as Rc<dyn PlatformDisplay>
            })
            .collect()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(X11Display::new(&self.xcb_connection, id.0 as i32)))
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        let x_window = self.xcb_connection.generate_id();

        let window_ptr = Rc::new(X11WindowState::new(
            options,
            &self.xcb_connection,
            self.x_root_index,
            x_window,
            &self.atoms,
        ));
        window_ptr.request_refresh();

        let cookie = self
            .xcb_connection
            .send_request(&xcb::randr::GetScreenResourcesCurrent { window: x_window });
        let screen_resources = self.xcb_connection.wait_for_reply(cookie).expect("TODO");
        let crtc = screen_resources.crtcs().first().expect("TODO");

        let cookie = self.xcb_connection.send_request(&xcb::randr::GetCrtcInfo {
            crtc: crtc.to_owned(),
            config_timestamp: xcb::x::Time::CurrentTime as u32,
        });
        let crtc_info = self.xcb_connection.wait_for_reply(cookie).expect("TODO");

        let mode_id = crtc_info.mode().resource_id();
        let mode = screen_resources
            .modes()
            .iter()
            .find(|m| m.id == mode_id)
            .expect("Missing screen mode for crtc specified mode id");

        let refresh_millies = mode_refresh_rate_millis(mode);

        self.refresh_millis.set(refresh_millies);

        self.state
            .borrow_mut()
            .windows
            .insert(x_window, Rc::clone(&window_ptr));
        Box::new(X11Window(window_ptr))
    }
}

// Adatpted from:
// https://docs.rs/winit/0.29.11/src/winit/platform_impl/linux/x11/monitor.rs.html#103-111
pub fn mode_refresh_rate_millis(mode: &xcb::randr::ModeInfo) -> u64 {
    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    (millihertz as f64 / 1_000_000.) as u64
}
