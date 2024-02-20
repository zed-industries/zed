use crate::platform::linux::client_dispatcher::ClientDispatcher;

pub(crate) struct WaylandClientDispatcher {
    eventfd: i32,
}

impl WaylandClientDispatcher {
    pub(crate) fn new(eventfd: i32) -> Self {
        Self { eventfd }
    }
}

impl Drop for WaylandClientDispatcher {
    fn drop(&mut self) {
        //todo!(linux)
    }
}

impl ClientDispatcher for WaylandClientDispatcher {
    fn dispatch_on_main_thread(&self) {
        // wake up the event loop
        unsafe { libc::eventfd_write(self.eventfd, 1u64) };
    }
}
