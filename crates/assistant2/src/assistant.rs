mod active_thread;
mod assistant_panel;
mod assistant_settings;
mod context;
mod context_picker;
mod context_store;
mod context_strip;
mod inline_assistant;
mod message_editor;
mod prompts;
mod streaming_diff;
mod terminal_inline_assistant;
mod thread;
mod thread_history;
mod thread_store;
mod ui;

use std::any::TypeId;
use std::sync::Arc;

use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{Assistant2FeatureFlag, FeatureFlagAppExt};
use fs::Fs;
use gpui::{actions, AppContext};
use prompts::PromptLoadingParams;
use settings::Settings as _;
use util::ResultExt;

pub use crate::assistant_panel::AssistantPanel;
use crate::assistant_settings::AssistantSettings;
pub use crate::inline_assistant::InlineAssistant;

actions!(
    assistant2,
    [
        ToggleFocus,
        NewThread,
        ToggleContextPicker,
        ToggleModelSelector,
        OpenHistory,
        Chat,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist
    ]
);

const NAMESPACE: &str = "assistant2";

/// Initializes the `assistant2` crate.
pub fn init(fs: Arc<dyn Fs>, client: Arc<Client>, stdout_is_a_pty: bool, cx: &mut AppContext) {
    AssistantSettings::register(cx);
    assistant_panel::init(cx);

    let prompt_builder = prompts::PromptBuilder::new(Some(PromptLoadingParams {
        fs: fs.clone(),
        repo_path: stdout_is_a_pty
            .then(|| std::env::current_dir().log_err())
            .flatten(),
        cx,
    }))
    .log_err()
    .map(Arc::new)
    .unwrap_or_else(|| Arc::new(prompts::PromptBuilder::new(None).unwrap()));
    inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );
    terminal_inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );

    feature_gate_assistant2_actions(cx);
}

fn feature_gate_assistant2_actions(cx: &mut AppContext) {
    const ASSISTANT1_NAMESPACE: &str = "assistant";

    let inline_assist_actions = [TypeId::of::<zed_actions::InlineAssist>()];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(NAMESPACE);
    });

    cx.observe_flag::<Assistant2FeatureFlag, _>(move |is_enabled, cx| {
        if is_enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.show_namespace(NAMESPACE);
                filter.hide_namespace(ASSISTANT1_NAMESPACE);

                // We're hiding all of the `assistant: ` actions, but we want to
                // keep the inline assist action around so we can use the same
                // one in Assistant2.
                filter.show_action_types(inline_assist_actions.iter());
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
