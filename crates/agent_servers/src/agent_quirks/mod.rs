pub mod cursor;
pub mod grok;
pub mod kiro;

use agent_client_protocol::schema as acp;
use project::AgentId;
use serde::Deserialize;

pub(crate) fn apply_client_capability_quirks(meta: &mut acp::Meta, agent_id: &AgentId) {
    cursor::apply_client_capability_quirks(meta, agent_id);
    grok::apply_client_capability_quirks(meta, agent_id);
    kiro::apply_client_capability_quirks(meta, agent_id);
}

/// Logs agent-specific `session/new` response shape issues for diagnosis.
///
/// Does not mutate the response — once the mapping from custom formats to
/// `config_options` is defined, a translation layer can be added.
pub(crate) fn debug_session_new_shape_mismatch(agent_id: &AgentId, raw_result: &serde_json::Value) {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SessionNewResultShape {
        #[serde(default)]
        config_options: Option<serde_json::Value>,
        #[serde(default)]
        models: Option<serde_json::Value>,
    }

    match serde_json::from_value::<SessionNewResultShape>(raw_result.clone()) {
        Err(err) => {
            log::debug!(
                "session/new response did not match expected SessionConfigResponse shape (agent_id={agent_id}, error={err}, raw_result={raw_result})"
            );
            return;
        }
        Ok(shape) => {
            let has_models = shape.models.is_some();
            let has_config_options = shape.config_options.is_some();
            if has_models && !has_config_options {
                log::debug!(
                    "session/new agent shape mismatch: response has `models` but no `configOptions`/`config_options` (agent_id={agent_id}, raw_result={raw_result})"
                );
                // TODO: translate `models` into `SessionConfigOption` entries once the mapping is defined.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CURSOR_ID;
    use project::AgentId;

    #[test]
    fn apply_client_capability_quirks_enables_parameterized_model_picker_only_for_cursor() {
        let mut meta = acp::Meta::default();
        apply_client_capability_quirks(&mut meta, &AgentId::new(CURSOR_ID));

        assert_eq!(
            meta.get("parameterizedModelPicker"),
            Some(&serde_json::Value::Bool(true))
        );
    }

    #[test]
    fn apply_client_capability_quirks_leaves_other_agents_unchanged() {
        let mut meta = acp::Meta::default();
        apply_client_capability_quirks(&mut meta, &AgentId::new("test-agent"));

        assert!(!meta.contains_key("parameterizedModelPicker"));
    }

    #[test]
    fn debug_session_new_shape_mismatch_detects_models_without_config_options() {
        let raw_result = serde_json::json!({
            "sessionId": "session-1",
            "models": [{ "id": "grok-3", "name": "Grok 3" }]
        });

        debug_session_new_shape_mismatch(&AgentId::new(grok::GROK_ID), &raw_result);
    }
}
