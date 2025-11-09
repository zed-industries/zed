use agent::{EditFileMode, EditFileToolInput};
use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct FileOverwriteExample;

/*
This eval tests a fix for a destructive behavior of the `edit_file` tool.
Previously, it would rewrite existing files too aggressively, which often
resulted in content loss.

Model           | Pass rate
----------------|----------
Sonnet 3.7      | 100%
Gemini 2.5 Pro  |  80%
*/

#[async_trait(?Send)]
impl Example for FileOverwriteExample {
    fn meta(&self) -> ExampleMetadata {
        let thread_json = include_str!("threads/overwrite-file.json");

        ExampleMetadata {
            name: "file_overwrite".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "023a60806a8cc82e73bd8d88e63b4b07fc7a0040".to_string(),
            language_server: None,
            max_assertions: Some(1),
            profile_id: AgentProfileId::default(),
            existing_thread_json: Some(thread_json.to_string()),
            max_turns: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let response = cx.proceed_with_max_turns(1).await?;
        let tool_use = response.expect_tool_call("edit_file", cx)?;
        let input = tool_use.parse_input::<EditFileToolInput>()?;
        let file_overwritten = match input.mode {
            EditFileMode::Edit => false,
            EditFileMode::Create | EditFileMode::Overwrite => {
                input.path.ends_with("src/language_model_selector.rs")
            }
        };

        cx.assert(!file_overwritten, "File should be edited, not overwritten")
    }
}
