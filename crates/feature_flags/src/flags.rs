use crate::FeatureFlag;

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}

pub struct AgentV2FeatureFlag;

impl FeatureFlag for AgentV2FeatureFlag {
    const NAME: &'static str = "agent-v2";
}

pub struct AcpBetaFeatureFlag;

impl FeatureFlag for AcpBetaFeatureFlag {
    const NAME: &'static str = "acp-beta";
}

pub struct ToolPermissionsFeatureFlag;

impl FeatureFlag for ToolPermissionsFeatureFlag {
    const NAME: &'static str = "tool-permissions";
}

pub struct AgentSharingFeatureFlag;

impl FeatureFlag for AgentSharingFeatureFlag {
    const NAME: &'static str = "agent-sharing";
}

pub struct SubagentsFeatureFlag;

impl FeatureFlag for SubagentsFeatureFlag {
    const NAME: &'static str = "subagents";

    fn enabled_for_staff() -> bool {
        false
    }
}

/// Whether to use the OpenAI Responses API format when sending requests to Cloud.
pub struct OpenAiResponsesApiFeatureFlag;

impl FeatureFlag for OpenAiResponsesApiFeatureFlag {
    const NAME: &'static str = "open-ai-responses-api";

    fn enabled_for_staff() -> bool {
        // Add yourself to the flag manually to test it out.
        false
    }
}
