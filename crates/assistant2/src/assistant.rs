mod active_thread;
mod assistant_panel;
mod context;
mod context_picker;
mod message_editor;
mod thread;
mod thread_history;
mod thread_store;
mod ui;

use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{Assistant2FeatureFlag, FeatureFlagAppExt};
use gpui::{actions, AppContext};

pub use crate::assistant_panel::AssistantPanel;

actions!(
    assistant2,
    [
        ToggleFocus,
        NewThread,
        ToggleModelSelector,
        OpenHistory,
        Chat
    ]
);

const NAMESPACE: &str = "assistant2";

/// Initializes the `assistant2` crate.
pub fn init(cx: &mut AppContext) {
    assistant_panel::init(cx);
    feature_gate_assistant2_actions(cx);
}

fn feature_gate_assistant2_actions(cx: &mut AppContext) {
    const ASSISTANT1_NAMESPACE: &str = "assistant";

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(NAMESPACE);
    });

    cx.observe_flag::<Assistant2FeatureFlag, _>(move |is_enabled, cx| {
        if is_enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.show_namespace(NAMESPACE);
                filter.hide_namespace(ASSISTANT1_NAMESPACE);
            });
        } else {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_namespace(NAMESPACE);
                filter.show_namespace(ASSISTANT1_NAMESPACE);
            });
        }
    })
    .detach();
}
