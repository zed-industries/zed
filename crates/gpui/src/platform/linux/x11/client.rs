use std::{rc::Rc, sync::Arc};

use parking_lot::Mutex;
use xcb::{x, Xid as _};
use xkbcommon::xkb;

use collections::{HashMap, HashSet};

use crate::platform::linux::client::Client;
use crate::platform::{
    LinuxPlatformInner, PlatformWindow, X11Display, X11Window, X11WindowState, XcbAtoms,
};
use crate::{
    AnyWindowHandle, Bounds, DisplayId, PlatformDisplay, PlatformInput, Point, ScrollDelta, Size,
    TouchPhase, WindowOptions,
};

pub(crate) struct X11ClientState {
    pub(crate) windows: HashMap<x::Window, Rc<X11WindowState>>,
    xkb: xkbcommon::xkb::State,
}

pub(crate) struct X11Client {
    platform_inner: Rc<LinuxPlatformInner>,
    xcb_connection: Arc<xcb::Connection>,
    x_root_index: i32,
    atoms: XcbAtoms,
    state: Mutex<X11ClientState>,
}

impl X11Client {
    pub(crate) fn new(
        inner: Rc<LinuxPlatformInner>,
        xcb_connection: Arc<xcb::Connection>,
        x_root_index: i32,
        atoms: XcbAtoms,
    ) -> Self {
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

        Self {
            platform_inner: inner,
            xcb_connection,
            x_root_index,
            atoms,
            state: Mutex::new(X11ClientState {
                windows: HashMap::default(),
                xkb: xkb_state,
            }),
        }
    }

    fn get_window(&self, win: x::Window) -> Rc<X11WindowState> {
        let state = self.state.lock();
        Rc::clone(&state.windows[&win])
    }
}

impl Client for X11Client {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();
        let mut windows_to_refresh = HashSet::<x::Window>::default();
        while !self.platform_inner.state.lock().quit_requested {
            // We prioritize work in the following order:
            //   1. input events from X11
            //   2. runnables for the main thread
            //   3. drawing/presentation
            let event = if let Some(event) = self.xcb_connection.poll_for_event().unwrap() {
                event
            } else if let Ok(runnable) = self.platform_inner.main_receiver.try_recv() {
                runnable.run();
                continue;
            } else if let Some(x_window) = windows_to_refresh.iter().next().cloned() {
                windows_to_refresh.remove(&x_window);
                let window = self.get_window(x_window);
                window.refresh();
                window.request_refresh();
                continue;
            } else {
                profiling::scope!("Wait for event");
                self.xcb_connection.wait_for_event().unwrap()
            };

            match event {
                xcb::Event::X(x::Event::ClientMessage(ev)) => {
                    if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                        if atom == self.atoms.wm_del_window.resource_id() {
                            windows_to_refresh.remove(&ev.window());
                            // window "x" button clicked by user, we gracefully exit
                            let window = self.state.lock().windows.remove(&ev.window()).unwrap();
                            window.destroy();
                            let state = self.state.lock();
                            self.platform_inner.state.lock().quit_requested |=
                                state.windows.is_empty();
                        }
                    }
                }
                xcb::Event::X(x::Event::Expose(ev)) => {
                    windows_to_refresh.insert(ev.window());
                }
                xcb::Event::X(x::Event::ConfigureNotify(ev)) => {
                    let bounds = Bounds {
                        origin: Point {
                            x: ev.x().into(),
                            y: ev.y().into(),
                        },
                        size: Size {
                            width: ev.width().into(),
                            height: ev.height().into(),
                        },
                    };
                    self.get_window(ev.window()).configure(bounds)
                }
                xcb::Event::Present(xcb::present::Event::CompleteNotify(ev)) => {
                    windows_to_refresh.insert(ev.window());
                }
                xcb::Event::Present(xcb::present::Event::IdleNotify(_ev)) => {}
                xcb::Event::X(x::Event::FocusIn(ev)) => {
                    let window = self.get_window(ev.event());
                    window.set_focused(true);
                }
                xcb::Event::X(x::Event::FocusOut(ev)) => {
                    let window = self.get_window(ev.event());
                    window.set_focused(false);
                }
                xcb::Event::X(x::Event::KeyPress(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let keystroke = {
                        let code = ev.detail().into();
                        let mut state = self.state.lock();
                        let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                        state.xkb.update_key(code, xkb::KeyDirection::Down);
                        keystroke
                    };

                    window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                        keystroke,
                        is_held: false,
                    }));
                }
                xcb::Event::X(x::Event::KeyRelease(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let keystroke = {
                        let code = ev.detail().into();
                        let mut state = self.state.lock();
                        let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                        state.xkb.update_key(code, xkb::KeyDirection::Up);
                        keystroke
                    };

                    window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
                }
                xcb::Event::X(x::Event::ButtonPress(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let position =
                        Point::new((ev.event_x() as f32).into(), (ev.event_y() as f32).into());
                    if let Some(button) = super::button_of_key(ev.detail()) {
                        window.handle_input(PlatformInput::MouseDown(crate::MouseDownEvent {
                            button,
                            position,
                            modifiers,
                            click_count: 1,
                        }));
                    } else if ev.detail() >= 4 && ev.detail() <= 5 {
                        // https://stackoverflow.com/questions/15510472/scrollwheel-event-in-x11
                        let delta_x = if ev.detail() == 4 { 1.0 } else { -1.0 };
                        window.handle_input(PlatformInput::ScrollWheel(crate::ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Lines(Point::new(0.0, delta_x)),
                            modifiers,
                            touch_phase: TouchPhase::default(),
                        }));
                    } else {
                        log::warn!("Unknown button press: {ev:?}");
                    }
                }
                xcb::Event::X(x::Event::ButtonRelease(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let position =
                        Point::new((ev.event_x() as f32).into(), (ev.event_y() as f32).into());
                    if let Some(button) = super::button_of_key(ev.detail()) {
                        window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                            button,
                            position,
                            modifiers,
                            click_count: 1,
                        }));
                    }
                }
                xcb::Event::X(x::Event::MotionNotify(ev)) => {
                    let window = self.get_window(ev.event());
                    let pressed_button = super::button_from_state(ev.state());
                    let position =
                        Point::new((ev.event_x() as f32).into(), (ev.event_y() as f32).into());
                    let modifiers = super::modifiers_from_state(ev.state());
                    window.handle_input(PlatformInput::MouseMove(crate::MouseMoveEvent {
                        pressed_button,
                        position,
                        modifiers,
                    }));
                }
                xcb::Event::X(x::Event::LeaveNotify(ev)) => {
                    let window = self.get_window(ev.event());
                    let pressed_button = super::button_from_state(ev.state());
                    let position =
                        Point::new((ev.event_x() as f32).into(), (ev.event_y() as f32).into());
                    let modifiers = super::modifiers_from_state(ev.state());
                    window.handle_input(PlatformInput::MouseExited(crate::MouseExitEvent {
                        pressed_button,
                        position,
                        modifiers,
                    }));
                }
                _ => {}
            }
        }

        if let Some(ref mut fun) = self.platform_inner.callbacks.lock().quit {
            fun();
        }
    }

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

        self.state
            .lock()
            .windows
            .insert(x_window, Rc::clone(&window_ptr));
        Box::new(X11Window(window_ptr))
    }
}
