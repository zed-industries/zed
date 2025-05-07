mod active_thread;
mod agent_diff;
mod assistant_configuration;
mod assistant_model_selector;
mod assistant_panel;
mod buffer_codegen;
mod context;
mod context_picker;
mod context_server_configuration;
mod context_server_tool;
mod context_store;
mod context_strip;
mod debug;
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
use fs::Fs;
use gpui::{App, actions, impl_actions};
use language::LanguageRegistry;
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
pub use crate::thread_store::{TextThreadStore, ThreadStore};
pub use agent_diff::{AgentDiffPane, AgentDiffToolbar};
pub use context_store::ContextStore;
pub use ui::preview::{all_agent_previews, get_agent_preview};

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
        KeepAll,
        Follow,
        ResetTrialUpsell,
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

/// Initializes the `agent` crate.
pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    prompt_builder: Arc<PromptBuilder>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut App,
) {
    AssistantSettings::register(cx);
    assistant_context_editor::init(client.clone(), cx);
    rules_library::init(cx);
    assistant_slash_command::init(cx);
    thread_store::init(cx);
    assistant_panel::init(cx);
    context_server_configuration::init(language_registry, cx);

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
}
