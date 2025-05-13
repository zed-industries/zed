use anyhow::Result;
use assistant_settings::AgentProfileId;
use assistant_tools::{EditFileMode, EditFileToolInput};
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct FileOverwriteExample;

/*
This eval is a test for a fix of a destructive behavoior of `edit_file`.
It used to rewrite existing files too aggressively, leading to
a content loss.

Pass rate before the fix: 10%
Pass rate after the fix:  100%
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
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let response = cx.run_turns(1).await?;
        let file_overwritten = if let Some(tool_use) = response.find_tool_call("edit_file") {
            let input = tool_use.parse_input::<EditFileToolInput>()?;
            match input.mode {
                EditFileMode::Edit => false,
                EditFileMode::Create | EditFileMode::Overwrite => true,
            }
        } else {
            false
        };

        cx.assert(!file_overwritten, "File should be edited, not overwritten")
    }
}
