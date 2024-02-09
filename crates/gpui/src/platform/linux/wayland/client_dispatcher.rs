use crate::platform::linux::client_dispatcher::ClientDispatcher;

pub(crate) struct WaylandClientDispatcher {

}

impl WaylandClientDispatcher {
   pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ClientDispatcher for WaylandClientDispatcher {
    fn dispatch_on_main_thread(&self) {
        println!("running wayland client");
    }
}
