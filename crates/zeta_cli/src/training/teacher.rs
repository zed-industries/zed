use crate::{example::Example, training::context::ContextType};
use anthropic_sdk::{Anthropic, ContentBlock, MessageCreateBuilder};
use anyhow::Result;

pub struct TeacherModel {
    llm_name: String,
    context: ContextType,
}

impl TeacherModel {
    pub fn new(llm_name: String, context: ContextType) -> Self {
        TeacherModel { llm_name, context }
    }

    pub async fn predict(&self, input: Example) -> Result<String> {
        static PROMPT: &str = include_str!("teacher.prompt.md");
        // todo: setup_worktree

        let context = "";

        let prompt = PROMPT
            .replace("{{context}}", context)
            .replace("{{edit_history}}", &input.edit_history);

        let client = Anthropic::from_env()?;
        let response = client
            .messages()
            .create(
                MessageCreateBuilder::new(self.llm_name.clone(), 16384)
                    .user(prompt)
                    .build(),
            )
            .await?;

        let response_text = &response
            .content
            .into_iter()
            .filter_map(|content| {
                if let ContentBlock::Text { text } = content {
                    Some(text)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let parsed = self.parse_predictions(response_text);

        Ok(parsed)
    }

    fn parse_predictions(&self, content: &str) -> String {
        // todo: parse predictions
        content.to_string()
    }
}
