use std::sync::Arc;

use xcb::x;

use crate::platform::linux::client_dispatcher::ClientDispatcher;

pub(crate) struct X11ClientDispatcher {
    xcb_connection: Arc<xcb::Connection>,
    x_listener_window: x::Window,
}

impl X11ClientDispatcher {
    pub fn new(xcb_connection: &Arc<xcb::Connection>, x_root_index: i32) -> Self {
        let x_listener_window = xcb_connection.generate_id();
        let screen = xcb_connection
            .get_setup()
            .roots()
            .nth(x_root_index as usize)
            .unwrap();
        xcb_connection.send_request(&x::CreateWindow {
            depth: 0,
            wid: x_listener_window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            border_width: 0,
            class: x::WindowClass::InputOnly,
            visual: screen.root_visual(),
            value_list: &[],
        });

        Self {
            xcb_connection: Arc::clone(xcb_connection),
            x_listener_window,
        }
    }
}

impl Drop for X11ClientDispatcher {
    fn drop(&mut self) {
        self.xcb_connection.send_request(&x::DestroyWindow {
            window: self.x_listener_window,
        });
    }
}

impl ClientDispatcher for X11ClientDispatcher {
    fn dispatch_on_main_thread(&self) {
        // Send a message to the invisible window, forcing
        // the main loop to wake up and dispatch the runnable.
        self.xcb_connection.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(self.x_listener_window),
            event_mask: x::EventMask::NO_EVENT,
            event: &x::VisibilityNotifyEvent::new(
                self.x_listener_window,
                x::Visibility::Unobscured,
            ),
        });
        self.xcb_connection.flush().unwrap();
    }
}
