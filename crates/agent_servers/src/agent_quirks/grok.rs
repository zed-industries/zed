use agent_client_protocol::schema as acp;
use project::AgentId;

pub(crate) const GROK_ID: &str = "grok";

/// Grok-specific client capability metadata injected during ACP `initialize`.
///
/// Currently no quirks needed for Grok initialization.
/// This function is a placeholder for future Grok-specific capabilities.
pub(crate) fn apply_client_capability_quirks(_meta: &mut acp::Meta, _agent_id: &AgentId) {}
