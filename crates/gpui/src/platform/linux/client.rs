use std::rc::Rc;

use crate::platform::PlatformWindow;
use crate::{AnyWindowHandle, CursorStyle, DisplayId, PlatformDisplay, WindowOptions};

pub trait Client {
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>;
    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;
    fn set_cursor_style(&self, style: CursorStyle);
}
