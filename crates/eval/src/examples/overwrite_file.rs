use anyhow::Result;
use assistant_settings::AgentProfileId;
use assistant_tools::EditFileToolInput;
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct FileOverwriteExample;

#[async_trait(?Send)]
impl Example for FileOverwriteExample {
    fn meta(&self) -> ExampleMetadata {
        let thread_json = include_str!("threads/overwrite-file.json");

        ExampleMetadata {
            name: "file_overwrite".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "023a60806a8cc82e73bd8d88e63b4b07fc7a0040".to_string(),
            language_server: None,
            max_assertions: Some(3),
            profile_id: AgentProfileId::default(),
            existing_thread_json: Some(thread_json.to_string()),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        dbg!(self.meta());
        let response = cx.run_turn().await?;
        let tool_use = response.expect_tool("edit_file", cx)?;
        let input = tool_use.parse_input::<EditFileToolInput>()?;

        cx.assert(
            !input.create_or_overwrite,
            "File should be edited, not overwritten",
        )?;

        Ok(())
    }
}
