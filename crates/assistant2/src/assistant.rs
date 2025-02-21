mod active_thread;
mod assistant_configuration;
mod assistant_model_selector;
mod assistant_panel;
mod buffer_codegen;
mod context;
mod context_picker;
mod context_store;
mod context_strip;
mod history_store;
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
use gpui::{actions, App};
use prompt_library::PromptBuilder;
use settings::Settings as _;

pub use crate::assistant_panel::{AssistantPanel, ConcreteAssistantPanelDelegate};
pub use crate::inline_assistant::InlineAssistant;

actions!(
    assistant2,
    [
        NewThread,
        NewPromptEditor,
        ToggleContextPicker,
        ToggleModelSelector,
        RemoveAllContext,
        OpenHistory,
        OpenConfiguration,
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
pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    prompt_builder: Arc<PromptBuilder>,
    cx: &mut App,
) {
    AssistantSettings::register(cx);
    thread_store::init(cx);
    assistant_panel::init(cx);

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

fn feature_gate_assistant2_actions(cx: &mut App) {
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
