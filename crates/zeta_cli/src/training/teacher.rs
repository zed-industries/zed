use crate::{example::Example, training::context::ContextType};
use anthropic_sdk::{Anthropic, ContentBlock, MessageCreateBuilder};
use anyhow::Result;
use std::hash::{Hash, Hasher};

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

        let mut hasher = std::hash::DefaultHasher::new();
        input.hash(&mut hasher);
        let disambiguator = hasher.finish();
        let hash = format!("{:04x}", disambiguator);
        let file_name = format!("{}_{}", &input.revision[..8], &hash[..4]);
        input.setup_worktree(file_name).await?;

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

        let parsed = self.parse_response(response_text);

        Ok(parsed)
    }

    fn parse_response(&self, content: &str) -> String {
        Self::extract_codeblock(content)
    }

    /// Extract content from code fences if any, or else return content as is
    fn extract_codeblock(text: &str) -> String {
        if let Some(start) = text.find("```") {
            let bytes = text.as_bytes();
            let mut backtick_end = start;

            while backtick_end < bytes.len() && bytes[backtick_end] == b'`' {
                backtick_end += 1;
            }

            let backtick_count = backtick_end - start;
            let closing_backticks = "`".repeat(backtick_count);

            if let Some(end_pos) = text[backtick_end..].find(&closing_backticks) {
                let code_block = &text[backtick_end..backtick_end + end_pos];
                return code_block.trim().to_string();
            }
        }

        text.to_string()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_response() {
        let teacher = TeacherModel::new("test".to_string(), ContextType::CurrentFile);
        let response = "This is a test response.";
        let parsed = teacher.parse_response(response);
        assert_eq!(parsed, response.to_string());

        let response = indoc::indoc! {"
            Some thinking

            `````
            actual response
            `````
            "};
        let parsed = teacher.parse_response(response);
        assert_eq!(parsed, "actual response");
    }
}
