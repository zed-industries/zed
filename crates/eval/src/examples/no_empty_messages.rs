use anyhow::Result;
use assistant_tools::{ListDirectoryToolInput, ReadFileToolInput};
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct NoEmptyMessagesExample;

#[async_trait(?Send)]
impl Example for NoEmptyMessagesExample {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "no_empty_messages".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "fcfeea4825c563715bcd1a1af809d88a37d12ccb".to_string(),
            language_server: None,
            max_assertions: Some(3),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(format!(
            r#"
                Summarize all the files in crates/assistant/src.
                Do tool calls only and do NOT include a description of what you are doing.
                Just give me an explanation of what you did after you finished all tool calls
        "#
        ));

        let response = cx.run_turn().await?;
        dbg!(response);
        // let tool_use = response.expect_tool("list_directory", cx)?;
        // let input = tool_use.parse_input::<ListDirectoryToolInput>()?;
        // cx.assert(
        //     &input.path == "zed/crates/assistant/src",
        //     "Path matches directory",
        // )?;

        for file in [
            "assistant.rs",
            "assistant_configuration.rs",
            "assistant_panel.rs",
            "inline_assistant.rs",
            "slash_command_settings.rs",
            "terminal_inline_assistant.rs",
        ] {
            let response = cx.run_turn().await?;
            dbg!(response);
            // let tool_use = response.expect_tool("read_file", cx)?;
            // let input = tool_use.parse_input::<ReadFileToolInput>()?;
            // dbg!(&input);
            // cx.assert(
            //     input.path == format!("zed/crates/assistant/src/{file}"),
            //     format!("Path {} matches file {file}", input.path),
            // )?;
        }

        cx.run_to_end().await?;

        Ok(())
    }
}
