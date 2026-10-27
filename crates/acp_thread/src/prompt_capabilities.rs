use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};

pub fn from_v1(capabilities: acp_v1::PromptCapabilities) -> acp_v2::PromptCapabilities {
    acp_v2::PromptCapabilities::new()
        .image(
            capabilities
                .image
                .then(acp_v2::PromptImageCapabilities::new),
        )
        .audio(
            capabilities
                .audio
                .then(acp_v2::PromptAudioCapabilities::new),
        )
        .embedded_context(
            capabilities
                .embedded_context
                .then(acp_v2::PromptEmbeddedContextCapabilities::new),
        )
        .meta(capabilities.meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_flags_advertise_objects_without_losing_metadata() {
        for (legacy, expected) in [
            (acp_v1::PromptCapabilities::new(), json!({})),
            (
                acp_v1::PromptCapabilities::new()
                    .image(true)
                    .audio(true)
                    .embedded_context(true),
                json!({"image": {}, "audio": {}, "embeddedContext": {}}),
            ),
            (
                acp_v1::PromptCapabilities::new()
                    .image(true)
                    .embedded_context(true)
                    .meta(acp_v1::Meta::from_iter([(
                        "extension".into(),
                        json!({"nested": [1, {"value": "retained"}]}),
                    )])),
                json!({
                    "image": {},
                    "embeddedContext": {},
                    "_meta": {"extension": {"nested": [1, {"value": "retained"}]}}
                }),
            ),
            (
                acp_v1::PromptCapabilities::new()
                    .audio(true)
                    .meta(acp_v1::Meta::new()),
                json!({"audio": {}, "_meta": {}}),
            ),
        ] {
            let capabilities = from_v1(legacy);
            assert_eq!(
                serde_json::to_value(capabilities).expect("shared capabilities"),
                expected
            );
        }
    }
}
