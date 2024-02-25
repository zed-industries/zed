use std::rc::Rc;

use crate::platform::PlatformWindow;
use crate::{AnyWindowHandle, DisplayId, ClipboardItem, PlatformDisplay, WindowOptions};

pub trait Client {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>);
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>;
    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;
}
