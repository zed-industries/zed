use gpui::{prelude::*, Action};

use ui::prelude::*;

use crate::window_controls::{WindowControl, WindowControlType};

#[derive(IntoElement)]
pub struct GenericWindowControls {
    close_window_action: Box<dyn Action>,
}

impl GenericWindowControls {
    pub fn new(close_action: Box<dyn Action>) -> Self {
        Self {
            close_window_action: close_action,
        }
    }
}

impl RenderOnce for GenericWindowControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id("generic-window-controls")
            .px_3()
            .gap_1p5()
            .child(WindowControl::new(
                "minimize",
                WindowControlType::Minimize,
                cx,
            ))
            .child(WindowControl::new(
                "maximize-or-restore",
                if cx.is_maximized() {
                    WindowControlType::Restore
                } else {
                    WindowControlType::Maximize
                },
                cx,
            ))
            .child(WindowControl::new_close(
                "close",
                WindowControlType::Close,
                self.close_window_action,
                cx,
            ))
    }
}
