mod active_thread;
mod agent_diff;
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
mod profile_selector;
mod terminal_codegen;
mod terminal_inline_assistant;
mod thread;
mod thread_history;
mod thread_store;
mod tool_compatibility;
mod tool_use;
mod ui;

use std::sync::Arc;

use assistant_settings::{AgentProfileId, AssistantSettings};
use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{Assistant2FeatureFlag, FeatureFlagAppExt};
use fs::Fs;
use gpui::{App, actions, impl_actions};
use prompt_store::PromptBuilder;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings as _;
use thread::ThreadId;

pub use crate::active_thread::ActiveThread;
use crate::assistant_configuration::{AddContextServerModal, ManageProfilesModal};
pub use crate::assistant_panel::{AssistantPanel, ConcreteAssistantPanelDelegate};
pub use crate::context::{ContextLoadResult, LoadedContext};
pub use crate::inline_assistant::InlineAssistant;
pub use crate::thread::{Message, MessageSegment, Thread, ThreadEvent};
pub use crate::thread_store::ThreadStore;
pub use agent_diff::{AgentDiff, AgentDiffToolbar};
pub use ui::{all_agent_previews, get_agent_preview};

actions!(
    agent,
    [
        NewTextThread,
        ToggleContextPicker,
        ToggleNavigationMenu,
        ToggleOptionsMenu,
        DeleteRecentlyOpenThread,
        ToggleProfileSelector,
        RemoveAllContext,
        ExpandMessageEditor,
        OpenHistory,
        AddContextServer,
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
        AcceptSuggestedContext,
        OpenActiveThreadAsMarkdown,
        OpenAgentDiff,
        Keep,
        Reject,
        RejectAll,
        KeepAll
    ]
);

#[derive(Default, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct NewThread {
    #[serde(default)]
    from_thread_id: Option<ThreadId>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct ManageProfiles {
    #[serde(default)]
    pub customize_tools: Option<AgentProfileId>,
}

impl ManageProfiles {
    pub fn customize_tools(profile_id: AgentProfileId) -> Self {
        Self {
            customize_tools: Some(profile_id),
        }
    }
}

impl_actions!(agent, [NewThread, ManageProfiles]);

const NAMESPACE: &str = "agent";

/// Initializes the `agent` crate.
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
    cx.observe_new(AddContextServerModal::register).detach();
    cx.observe_new(ManageProfilesModal::register).detach();

    feature_gate_agent_actions(cx);
}

fn feature_gate_agent_actions(cx: &mut App) {
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
