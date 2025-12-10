use crate::{
    example::Example,
    source_location::SourceLocation,
    training::{
        context::{ContextType, collect_context, strip_special_tags},
        llm_client::LlmClient,
    },
};
use anthropic::{Message, RequestContent, ResponseContent, Role};
use anyhow::Result;

pub struct TeacherModel {
    pub llm_name: String,
    pub context: ContextType,
    pub client: LlmClient,
}

#[derive(Debug, serde::Serialize)]
pub struct TeacherOutput {
    parsed_output: String,
    prompt: String,
    raw_llm_response: String,
    context: String,
    diff: String,
}

impl TeacherModel {
    const PROMPT: &str = include_str!("teacher.prompt.md");
    pub(crate) const REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const REGION_END: &str = "<|editable_region_end|>";
    pub(crate) const USER_CURSOR: &str = "<|user_cursor|>";

    /// Number of lines to include before the cursor position
    pub(crate) const LEFT_CONTEXT_SIZE: usize = 5;

    /// Number of lines to include after the cursor position
    pub(crate) const RIGHT_CONTEXT_SIZE: usize = 5;

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

    pub fn new(llm_name: String, context: ContextType, client: LlmClient) -> Self {
        TeacherModel {
            llm_name,
            context,
            client,
        }
    }

    pub async fn predict(&self, input: Example) -> Result<Option<TeacherOutput>> {
        let name = input.unique_name();
        let worktree_dir = input.setup_worktree(name).await?;
        let cursor: SourceLocation = input
            .cursor_position
            .parse()
            .expect("Failed to parse cursor position");

        let context = collect_context(&self.context, &worktree_dir, cursor.clone());
        let edit_history = Self::format_edit_history(&input.edit_history);

        let prompt = Self::PROMPT
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history);

        let messages = vec![Message {
            role: Role::User,
            content: vec![RequestContent::Text {
                text: prompt.clone(),
                cache_control: None,
            }],
        }];

        let Some(response) = self
            .client
            .generate(self.llm_name.clone(), 16384, messages)
            .await?
        else {
            return Ok(None);
        };

        let response_text = response
            .content
            .into_iter()
            .filter_map(|content| match content {
                ResponseContent::Text { text } => Some(text),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n");

        let parsed_output = self.parse_response(&response_text);

        let original_editable_region = Self::extract_editable_region(&context);
        let context_after_edit = context.replace(&original_editable_region, &parsed_output);
        let context_after_edit = strip_special_tags(&context_after_edit);
        let context_before_edit = strip_special_tags(&context);
        let diff = language::unified_diff(&context_before_edit, &context_after_edit);

        // zeta distill --batch batch_results.txt
        // zeta distill
        // 1. Run `zeta distill <2000 examples <- all examples>` for the first time
        //  - store LLM requests in a batch, don't actual send the request
        //  - send the batch (2000 requests) after all inputs are processed
        // 2. `zeta send-batches`
        //   - upload the batch to Anthropic

        // https://platform.claude.com/docs/en/build-with-claude/batch-processing
        // https://crates.io/crates/anthropic-sdk-rust

        //   - poll for results
        //   - when ready, store results in cache (a database)
        // 3. `zeta distill` again
        //    - use the cached results this time

        Ok(Some(TeacherOutput {
            parsed_output,
            prompt,
            raw_llm_response: response_text,
            context,
            diff,
        }))
    }

    fn parse_response(&self, content: &str) -> String {
        let codeblock = Self::extract_last_codeblock(content);
        let editable_region = Self::extract_editable_region(&codeblock);

        editable_region
    }

    /// Extract content from the last code-fenced block if any, or else return content as is
    fn extract_last_codeblock(text: &str) -> String {
        let mut last_block = None;
        let mut search_start = 0;

        while let Some(start) = text[search_start..].find("```") {
            let start = start + search_start;
            let bytes = text.as_bytes();
            let mut backtick_end = start;

            while backtick_end < bytes.len() && bytes[backtick_end] == b'`' {
                backtick_end += 1;
            }

            let backtick_count = backtick_end - start;
            let closing_backticks = "`".repeat(backtick_count);

            if let Some(end_pos) = text[backtick_end..].find(&closing_backticks) {
                let code_block = &text[backtick_end + 1..backtick_end + end_pos - 1];
                last_block = Some(code_block.to_string());
                search_start = backtick_end + end_pos + backtick_count;
            } else {
                break;
            }
        }

        last_block.unwrap_or_else(|| text.to_string())
    }

    fn extract_editable_region(text: &str) -> String {
        let start = text
            .find(Self::REGION_START)
            .map_or(0, |pos| pos + Self::REGION_START.len());
        let end = text.find(Self::REGION_END).unwrap_or(text.len());

        text[start..end].to_string()
    }

    /// Truncates edit history to a maximum length and removes comments (unified diff garbage lines)
    fn format_edit_history(edit_history: &str) -> String {
        let lines = edit_history
            .lines()
            .filter(|&s| Self::is_content_line(s))
            .collect::<Vec<_>>();

        let history_lines = if lines.len() > Self::MAX_HISTORY_LINES {
            &lines[lines.len() - Self::MAX_HISTORY_LINES..]
        } else {
            &lines
        };
        history_lines.join("\n")
    }

    fn is_content_line(s: &str) -> bool {
        s.starts_with("-")
            || s.starts_with("+")
            || s.starts_with(" ")
            || s.starts_with("---")
            || s.starts_with("+++")
            || s.starts_with("@@")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response() {
        let teacher = TeacherModel::new(
            "test".to_string(),
            ContextType::CurrentFile,
            LlmClient::dummy(),
        );
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

    #[test]
    fn test_extract_last_code_block() {
        let text = indoc::indoc! {"
            Some thinking

            ```
            first block
            ```

            `````
            last block
            `````
            "};
        let last_block = TeacherModel::extract_last_codeblock(text);
        assert_eq!(last_block, "last block");
    }

    #[test]
    fn test_extract_editable_region() {
        let teacher = TeacherModel::new(
            "test".to_string(),
            ContextType::CurrentFile,
            LlmClient::dummy(),
        );
        let response = indoc::indoc! {"
            some lines
            are
            here
            <|editable_region_start|>
            one
            two three

            <|editable_region_end|>
            more
            lines here
            "};
        let parsed = teacher.parse_response(response);
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three

            "}
        );
    }
}
