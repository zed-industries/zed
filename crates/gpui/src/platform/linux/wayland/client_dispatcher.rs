use std::sync::Arc;

use wayland_client::{Connection, EventQueue};

use crate::platform::linux::client_dispatcher::ClientDispatcher;

pub(crate) struct WaylandClientDispatcher {
    conn: Arc<Connection>,
    event_queue: Arc<EventQueue<Connection>>,
}

impl WaylandClientDispatcher {
    pub(crate) fn new(conn: &Arc<Connection>) -> Self {
        let event_queue = conn.new_event_queue();
        Self {
            conn: Arc::clone(conn),
            event_queue: Arc::new(event_queue),
        }
    }
}

impl Drop for WaylandClientDispatcher {
    fn drop(&mut self) {
        //todo!(linux)
    }
}

impl ClientDispatcher for WaylandClientDispatcher {
    fn dispatch_on_main_thread(&self) {}
}
