use gpui::{prelude::*, Action};

use ui::prelude::*;

use super::platform_generic::GenericWindowControls;

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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        GenericWindowControls::new(self.close_window_action.boxed_clone()).into_any_element()
    }
}
