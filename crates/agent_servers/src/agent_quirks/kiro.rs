use agent_client_protocol::schema as acp;
use project::AgentId;

pub(crate) const KIRO_ID: &str = "kiro";

/// Kiro-specific client capability metadata injected during ACP `initialize`.
///
/// Currently no quirks needed for Kiro initialization.
/// This function is a placeholder for future Kiro-specific capabilities.
///
/// Note: Kiro uses custom notification `_kiro.dev/subagent/list_update` which
/// is currently not handled by Zed. This may be addressed in a future update
/// once the subagent selection API is understood.
pub(crate) fn apply_client_capability_quirks(_meta: &mut acp::Meta, _agent_id: &AgentId) {}
