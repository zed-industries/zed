mod assistant_panel;

use gpui::{actions, AppContext};

pub use crate::assistant_panel::AssistantPanel;

actions!(assistant2, [ToggleFocus, NewChat]);

/// Initializes the `assistant2` crate.
pub fn init(cx: &mut AppContext) {
    assistant_panel::init(cx);
}
