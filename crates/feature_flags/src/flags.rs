use crate::{EnumFeatureFlag, FeatureFlag, PresenceFlag, register_feature_flag};

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
    type Value = PresenceFlag;
}
register_feature_flag!(NotebookFeatureFlag);

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
    type Value = PresenceFlag;
}
register_feature_flag!(PanicFeatureFlag);

/// A feature flag for granting access to beta ACP features.
///
/// We reuse this feature flag for new betas, so don't delete it if it is not currently in use.
pub struct AcpBetaFeatureFlag;

impl FeatureFlag for AcpBetaFeatureFlag {
    const NAME: &'static str = "acp-beta";
    type Value = PresenceFlag;
}
register_feature_flag!(AcpBetaFeatureFlag);

pub struct DiffReviewFeatureFlag;

impl FeatureFlag for DiffReviewFeatureFlag {
    const NAME: &'static str = "diff-review";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(DiffReviewFeatureFlag);

/// Gates the `create_thread` and `list_agents_and_models` tools, which let
/// the agent spawn independent sibling threads that show up in the agent
/// panel sidebar.
pub struct CreateThreadToolFeatureFlag;

impl FeatureFlag for CreateThreadToolFeatureFlag {
    const NAME: &'static str = "create-thread-tool";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(CreateThreadToolFeatureFlag);

pub struct LspToolFeatureFlag;

impl FeatureFlag for LspToolFeatureFlag {
    const NAME: &'static str = "lsp-tool";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(LspToolFeatureFlag);

pub struct RenameToolFeatureFlag;

impl FeatureFlag for RenameToolFeatureFlag {
    const NAME: &'static str = "rename-tool";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(RenameToolFeatureFlag);

pub struct ProjectPanelUndoRedoFeatureFlag;

impl FeatureFlag for ProjectPanelUndoRedoFeatureFlag {
    const NAME: &'static str = "project-panel-undo-redo";
    type Value = PresenceFlag;

    fn enabled_for_staff() -> bool {
        true
    }
}
register_feature_flag!(ProjectPanelUndoRedoFeatureFlag);

/// Controls how agent thread worktree chips are labeled in the sidebar.
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumFeatureFlag)]
pub enum AgentThreadWorktreeLabel {
    #[default]
    Both,
    Worktree,
    Branch,
}

pub struct AgentThreadWorktreeLabelFlag;

impl FeatureFlag for AgentThreadWorktreeLabelFlag {
    const NAME: &'static str = "agent-thread-worktree-label";
    type Value = AgentThreadWorktreeLabel;

    fn enabled_for_staff() -> bool {
        false
    }
}
register_feature_flag!(AgentThreadWorktreeLabelFlag);

/// Moves LLM provider and MCP server configuration out of the dedicated agent
/// panel page and into the settings UI. When enabled, the agent panel no longer
/// shows its configuration overlay and the settings UI exposes the "LLM
/// Providers" and "MCP Servers" sub-pages instead.
pub struct AgentSettingsUiFeatureFlag;

impl FeatureFlag for AgentSettingsUiFeatureFlag {
    const NAME: &'static str = "agent-settings-ui";
    type Value = PresenceFlag;
}
register_feature_flag!(AgentSettingsUiFeatureFlag);

/// Wraps agent-run terminal commands in an OS-level sandbox where supported
/// (currently macOS Seatbelt only). When off, terminal commands run with the
/// agent's full ambient permissions, as they always have.
pub struct SandboxingFeatureFlag;

impl FeatureFlag for SandboxingFeatureFlag {
    const NAME: &'static str = "sandboxing";
    type Value = PresenceFlag;
}
register_feature_flag!(SandboxingFeatureFlag);
