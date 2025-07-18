#[cfg(test)]
mod tests {
    use super::*;
    use language_model::{LanguageModelRequest, Message, Role, MessageContent, LanguageModelTool, LanguageModelToolChoice};
    use serde_json::json;

    #[test]
    fn test_into_google_with_multimodal_and_tool() {
        let request = LanguageModelRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![
                    MessageContent::Image(language_model::Image {
                        source: "base64-encoded-image".into(),
                    }),
                    MessageContent::Text("What is in this image?".to_string()),
                ],
            }],
            tools: vec![LanguageModelTool {
                name: "test-tool".to_string(),
                description: "A test tool".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string"
                        }
                    }
                }),
            }],
            tool_choice: Some(LanguageModelToolChoice::Auto),
            ..Default::default()
        };

        let google_request = into_google(request, "gemini-pro".to_string(), GoogleModelMode::Default);

        assert_eq!(google_request.contents.len(), 1);
        let content = &google_request.contents[0];
        assert_eq!(content.parts.len(), 2);
        assert!(matches!(content.parts[0], Part::InlineDataPart(_)));
        assert!(matches!(content.parts[1], Part::TextPart(_)));
        assert!(google_request.tools.is_some());
    }
}
