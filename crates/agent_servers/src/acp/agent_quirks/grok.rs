use agent_client_protocol::schema as acp;
use project::AgentId;

pub(crate) const GROK_ID: &str = "grok";

pub(crate) fn apply_client_capability_quirks(_meta: &mut acp::Meta, _agent_id: &AgentId) {}
