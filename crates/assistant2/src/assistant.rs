mod active_thread;
mod assistant_model_selector;
mod assistant_panel;
mod buffer_codegen;
mod context;
mod context_picker;
mod context_store;
mod context_strip;
mod inline_assistant;
mod inline_prompt_editor;
mod message_editor;
mod terminal_codegen;
mod terminal_inline_assistant;
mod thread;
mod thread_history;
mod thread_store;
mod ui;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{Assistant2FeatureFlag, FeatureFlagAppExt};
use fs::Fs;
use gpui::{actions, AppContext};
use prompt_library::{PromptBuilder, PromptLoadingParams};
use settings::Settings as _;
use util::ResultExt;

pub use crate::assistant_panel::AssistantPanel;
pub use crate::inline_assistant::InlineAssistant;

actions!(
    assistant2,
    [
        ToggleFocus,
        NewThread,
        ToggleContextPicker,
        ToggleModelSelector,
        RemoveAllContext,
        OpenHistory,
        RemoveSelectedThread,
        Chat,
        ChatMode,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist,
        FocusUp,
        FocusDown,
        FocusLeft,
        FocusRight,
        RemoveFocusedContext,
        AcceptSuggestedContext
    ]
);

const NAMESPACE: &str = "assistant2";

/// Initializes the `assistant2` crate.
pub fn init(fs: Arc<dyn Fs>, client: Arc<Client>, stdout_is_a_pty: bool, cx: &mut AppContext) {
    AssistantSettings::register(cx);
    assistant_panel::init(cx);

    let prompt_builder = PromptBuilder::new(Some(PromptLoadingParams {
        fs: fs.clone(),
        repo_path: stdout_is_a_pty
            .then(|| std::env::current_dir().log_err())
            .flatten(),
        cx,
    }))
    .log_err()
    .map(Arc::new)
    .unwrap_or_else(|| Arc::new(PromptBuilder::new(None).unwrap()));
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
    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(NAMESPACE);
    });

    cx.observe_flag::<Assistant2FeatureFlag, _>(move |is_enabled, cx| {
        if is_enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.show_namespace(NAMESPACE);
            });
        } else {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_namespace(NAMESPACE);
            });
        }
    })
    .detach();
}
