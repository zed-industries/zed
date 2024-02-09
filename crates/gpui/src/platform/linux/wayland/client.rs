use std::rc::Rc;

use crate::{AnyWindowHandle, DisplayId, PlatformDisplay, WindowOptions};
use crate::platform::linux::client::Client;
use crate::platform::PlatformWindow;

pub(crate) struct WaylandClient {

}

impl WaylandClient {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Client for WaylandClient {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        todo!()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        todo!()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        todo!()
    }

    fn open_window(&self, handle: AnyWindowHandle, options: WindowOptions) -> Box<dyn PlatformWindow> {
        todo!()
    }
}