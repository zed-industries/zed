use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;
use xcb::{x, Xid};

use collections::HashMap;

use crate::platform::linux::client::Client;
use crate::platform::{
    LinuxPlatformInner, PlatformWindow, X11Display, X11Window, X11WindowState, XcbAtoms,
};
use crate::{AnyWindowHandle, Bounds, DisplayId, PlatformDisplay, Point, Size, WindowOptions};

pub(crate) struct X11ClientState {
    pub(crate) windows: HashMap<x::Window, Rc<X11WindowState>>,
}

pub(crate) struct X11Client {
    platform_inner: Arc<LinuxPlatformInner>,
    xcb_connection: Arc<xcb::Connection>,
    x_root_index: i32,
    atoms: XcbAtoms,
    state: Mutex<X11ClientState>,
}

impl X11Client {
    pub(crate) fn new(
        inner: Arc<LinuxPlatformInner>,
        xcb_connection: Arc<xcb::Connection>,
        x_root_index: i32,
        atoms: XcbAtoms,
    ) -> Self {
        Self {
            platform_inner: inner,
            xcb_connection,
            x_root_index,
            atoms,
            state: Mutex::new(X11ClientState {
                windows: HashMap::default(),
            }),
        }
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
                            let mut state = self.state.lock();
                            self.platform_inner.state.lock().quit_requested |=
                                state.windows.is_empty();
                        }
                    }
                }
                xcb::Event::X(x::Event::Expose(ev)) => {
                    let window = {
                        let state = self.state.lock();
                        Rc::clone(&state.windows[&ev.window()])
                    };
                    window.expose();
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
                    let window = {
                        let state = self.state.lock();
                        Rc::clone(&state.windows[&ev.window()])
                    };
                    window.configure(bounds)
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
        handle: AnyWindowHandle,
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

        self.state
            .lock()
            .windows
            .insert(x_window, Rc::clone(&window_ptr));
        Box::new(X11Window(window_ptr))
    }
}
