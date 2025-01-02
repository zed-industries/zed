use gpui::{prelude::*, Action, MouseButton};

use ui::prelude::*;

use crate::window_controls::{WindowControl, WindowControlType};

#[derive(IntoElement)]
pub struct LinuxWindowControls {
    close_window_action: Box<dyn Action>,
}

impl LinuxWindowControls {
    pub fn new(close_window_action: Box<dyn Action>) -> Self {
        Self {
            close_window_action,
        }
    }
}

impl RenderOnce for LinuxWindowControls {
    fn render(self, window: &mut Window, cx: &mut AppContext) -> impl IntoElement {
        h_flex()
            .id("generic-window-controls")
            .px_3()
            .gap_3()
            .on_mouse_down(MouseButton::Left, |_, window, cx| cx.stop_propagation())
            .child(WindowControl::new(
                "minimize",
                WindowControlType::Minimize,
                window, cx,
            ))
            .child(WindowControl::new(
                "maximize-or-restore",
                if window.is_maximized() {
                    WindowControlType::Restore
                } else {
                    WindowControlType::Maximize
                },
                window, cx,
            ))
            .child(WindowControl::new_close(
                "close",
                WindowControlType::Close,
                self.close_window_action,
                window, cx,
            ))
    }
}
