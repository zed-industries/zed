use agent_client_protocol::schema as acp;
use project::AgentId;

use crate::CURSOR_ID;

/// Cursor-specific client capability metadata injected during ACP `initialize`.
pub(crate) fn apply_client_capability_quirks(meta: &mut acp::Meta, agent_id: &AgentId) {
    if agent_id.as_ref() == CURSOR_ID {
        meta.insert("parameterizedModelPicker".into(), true.into());
    }
}
