use crate::headless_assistant::send_language_model_request;
use anyhow::anyhow;
use gpui::{App, Task};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use std::sync::Arc;

pub struct Judge {
    #[allow(dead_code)]
    pub original_diff: Option<String>,
    pub original_message: Option<String>,
    pub model: Arc<dyn LanguageModel>,
}

impl Judge {
    pub fn run_with_prompt(&self, cx: &mut App) -> Task<anyhow::Result<String>> {
        let Some(prompt) = self.original_message.as_ref() else {
            return Task::ready(Err(anyhow!("No prompt provided in original_message")));
        };

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(prompt.clone())],
                cache: false,
            }],
            temperature: Some(0.0),
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let model = self.model.clone();
        let request = request.clone();
        cx.spawn(async move |cx| send_language_model_request(model, request, cx).await)
    }
}
