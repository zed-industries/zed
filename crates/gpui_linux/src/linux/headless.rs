mod window;

use std::rc::Rc;

use gpui::{AnyWindowHandle, PlatformDisplay, PlatformWindow, WindowParams};

pub(crate) use window::{HeadlessDisplay, HeadlessWindow};

/// The state of a `LinuxPlatform` with no display server.
///
/// It reports one fake display and opens [`HeadlessWindow`]s, which lay out and handle input but
/// draw nothing.
pub(crate) struct HeadlessConnection {
    display: Rc<dyn PlatformDisplay>,
    /// Cloned into every open window, so its count tells whether windows are open.
    window_lease: Rc<()>,
}

impl HeadlessConnection {
    pub(crate) fn new() -> Self {
        Self {
            display: Rc::new(HeadlessDisplay::new()),
            window_lease: Rc::new(()),
        }
    }

    pub(crate) fn has_windows(&self) -> bool {
        Rc::strong_count(&self.window_lease) > 1
    }

    pub(crate) fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.display.clone()]
    }

    pub(crate) fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    pub(crate) fn open_window(
        &self,
        _handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(HeadlessWindow::new(
            params,
            self.display.clone(),
            self.window_lease.clone(),
        )))
    }
}
