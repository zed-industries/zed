use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use xcb::{x, Xid as _};
use xkbcommon::xkb;

use collections::HashMap;
use copypasta::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};
use copypasta::ClipboardProvider;

use crate::platform::linux::client::Client;
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::{
    AnyWindowHandle, Bounds, CursorStyle, DisplayId, PlatformDisplay, PlatformInput, Point,
    ScrollDelta, Size, TouchPhase, WindowOptions,
};

use super::{X11Display, X11Window, X11WindowState, XcbAtoms};
use calloop::{
    generic::{FdWrapper, Generic},
    RegistrationToken,
};

struct WindowRef {
    state: Rc<X11WindowState>,
    refresh_event_token: RegistrationToken,
}

struct X11ClientState {
    windows: HashMap<x::Window, WindowRef>,
    xkb: xkbcommon::xkb::State,
    clipboard: Rc<RefCell<X11ClipboardContext<Clipboard>>>,
    primary: Rc<RefCell<X11ClipboardContext<Primary>>>,
}

pub(crate) struct X11Client {
    platform_inner: Rc<LinuxPlatformInner>,
    xcb_connection: Rc<xcb::Connection>,
    x_root_index: i32,
    atoms: XcbAtoms,
    state: RefCell<X11ClientState>,
}

impl X11Client {
    pub(crate) fn new(inner: Rc<LinuxPlatformInner>) -> Rc<Self> {
        let (xcb_connection, x_root_index) = xcb::Connection::connect_with_extensions(
            None,
            &[xcb::Extension::RandR, xcb::Extension::Xkb],
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

        let xkb_state = {
            let xkb_context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
            let xkb_device_id = xkb::x11::get_core_keyboard_device_id(&xcb_connection);
            let xkb_keymap = xkb::x11::keymap_new_from_device(
                &xkb_context,
                &xcb_connection,
                xkb_device_id,
                xkb::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkb::x11::state_new_from_device(&xkb_keymap, &xcb_connection, xkb_device_id)
        };

        let clipboard = X11ClipboardContext::<Clipboard>::new().unwrap();
        let primary = X11ClipboardContext::<Primary>::new().unwrap();

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
            }),
        });

        // Safety: Safe if xcb::Connection always returns a valid fd
        let fd = unsafe { FdWrapper::new(Rc::clone(&xcb_connection)) };

        inner
            .loop_handle
            .insert_source(
                Generic::new_with_error::<xcb::Error>(
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

    fn get_window(&self, win: x::Window) -> Option<Rc<X11WindowState>> {
        let state = self.state.borrow();
        state.windows.get(&win).map(|wr| Rc::clone(&wr.state))
    }

    fn handle_event(&self, event: xcb::Event) -> Option<()> {
        match event {
            xcb::Event::X(x::Event::ClientMessage(event)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = event.data() {
                    if atom == self.atoms.wm_del_window.resource_id() {
                        // window "x" button clicked by user, we gracefully exit
                        let window_ref = self
                            .state
                            .borrow_mut()
                            .windows
                            .remove(&event.window())
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
            xcb::Event::X(x::Event::Expose(event)) => {
                let window = self.get_window(event.window())?;
                window.refresh();
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

        let cookie = self
            .xcb_connection
            .send_request(&xcb::randr::GetScreenResourcesCurrent { window: x_window });
        let screen_resources = self.xcb_connection.wait_for_reply(cookie).expect("TODO");
        let mode = screen_resources
            .crtcs()
            .iter()
            .find_map(|crtc| {
                let cookie = self.xcb_connection.send_request(&xcb::randr::GetCrtcInfo {
                    crtc: crtc.to_owned(),
                    config_timestamp: xcb::x::Time::CurrentTime as u32,
                });
                let crtc_info = self.xcb_connection.wait_for_reply(cookie).expect("TODO");

                let mode_id = crtc_info.mode().resource_id();
                screen_resources.modes().iter().find(|m| m.id == mode_id)
            })
            .expect("Missing screen mode for crtc specified mode id");

        let refresh_event_token = self
            .platform_inner
            .loop_handle
            .insert_source(calloop::timer::Timer::immediate(), {
                let refresh_duration = mode_refresh_rate(mode);
                let xcb_connection = Rc::clone(&self.xcb_connection);
                move |mut instant, (), _| {
                    xcb_connection.send_request(&x::SendEvent {
                        propagate: false,
                        destination: x::SendEventDest::Window(x_window),
                        event_mask: x::EventMask::EXPOSURE,
                        event: &x::ExposeEvent::new(x_window, 0, 0, 0, 0, 1),
                    });
                    let _ = xcb_connection.flush();
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
pub fn mode_refresh_rate(mode: &xcb::randr::ModeInfo) -> Duration {
    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    let micros = 1_000_000_000 / millihertz;
    log::info!("Refreshing at {} micros", micros);
    Duration::from_micros(micros)
}
