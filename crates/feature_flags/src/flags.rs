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

pub struct UserSlashCommandsFeatureFlag;

impl FeatureFlag for UserSlashCommandsFeatureFlag {
    const NAME: &'static str = "slash-commands";
}

pub struct ToolPermissionsFeatureFlag;

impl FeatureFlag for ToolPermissionsFeatureFlag {
    const NAME: &'static str = "tool-permissions";

    fn enabled_for_staff() -> bool {
        false
    }
}

pub struct AgentSharingFeatureFlag;

impl FeatureFlag for AgentSharingFeatureFlag {
    const NAME: &'static str = "agent-sharing";
}

pub struct SubagentsFeatureFlag;

impl FeatureFlag for SubagentsFeatureFlag {
    const NAME: &'static str = "subagents";

    fn enabled_for_staff() -> bool {
        true
    }
}

pub struct DiffReviewFeatureFlag;

impl FeatureFlag for DiffReviewFeatureFlag {
    const NAME: &'static str = "diff-review";

    fn enabled_for_staff() -> bool {
        false
    }
}
