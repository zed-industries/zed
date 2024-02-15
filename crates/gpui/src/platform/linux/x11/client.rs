use std::{rc::Rc, sync::Arc};

use parking_lot::Mutex;
use xcb::{x, Xid as _};
use xkbcommon::xkb;

use collections::HashMap;

use crate::platform::linux::client::Client;
use crate::platform::{
    LinuxPlatformInner, PlatformWindow, X11Display, X11Window, X11WindowState, XcbAtoms,
};
use crate::{
    AnyWindowHandle, Bounds, DisplayId, PlatformDisplay, PlatformInput, Point, Size, WindowOptions,
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
        //Note: here and below, don't keep the lock() open when calling
        // into window functions as they may invoke callbacks that need
        // to immediately access the platform (self).
        while !self.platform_inner.state.lock().quit_requested {
            let event = self.xcb_connection.wait_for_event().unwrap();
            match event {
                xcb::Event::X(x::Event::ClientMessage(ev)) => {
                    if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                        if atom == self.atoms.wm_del_window.resource_id() {
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
                    self.get_window(ev.window()).refresh();
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
                    let window = self.get_window(ev.window());
                    window.refresh();
                    window.request_refresh();
                }
                xcb::Event::Present(xcb::present::Event::IdleNotify(_ev)) => {}
                xcb::Event::X(x::Event::KeyPress(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let key = {
                        let code = ev.detail().into();
                        let mut state = self.state.lock();
                        let key = state.xkb.key_get_utf8(code);
                        state.xkb.update_key(code, xkb::KeyDirection::Down);
                        key
                    };
                    window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                        keystroke: crate::Keystroke {
                            modifiers,
                            key,
                            ime_key: None,
                        },
                        is_held: false,
                    }));
                }
                xcb::Event::X(x::Event::KeyRelease(ev)) => {
                    let window = self.get_window(ev.event());
                    let modifiers = super::modifiers_from_state(ev.state());
                    let key = {
                        let code = ev.detail().into();
                        let mut state = self.state.lock();
                        let key = state.xkb.key_get_utf8(code);
                        state.xkb.update_key(code, xkb::KeyDirection::Up);
                        key
                    };
                    window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent {
                        keystroke: crate::Keystroke {
                            modifiers,
                            key,
                            ime_key: None,
                        },
                    }));
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

            if let Ok(runnable) = self.platform_inner.main_receiver.try_recv() {
                runnable.run();
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
