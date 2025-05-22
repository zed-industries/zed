use anyhow::Result;
use assistant_settings::AgentProfileId;
use assistant_tools::FindPathToolInput;
use async_trait::async_trait;
use regex::Regex;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct FileSearchExample;

#[async_trait(?Send)]
impl Example for FileSearchExample {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "file_search".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "03ecb88fe30794873f191ddb728f597935b3101c".to_string(),
            language_server: None,
            max_assertions: Some(3),
            profile_id: AgentProfileId::default(),
            existing_thread_json: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        const FILENAME: &str = "find_replace_file_tool.rs";
        cx.push_user_message(format!(
                r#"
        Look at the `{FILENAME}`. I want to implement a card for it. The card should implement the `Render` trait.

        The card should show a diff. It should be a beautifully presented diff. The card "box" should look like what we show for
        markdown codeblocks (look at `MarkdownElement`). I want to see a red background for lines that were deleted and a green
        background for lines that were added. We should have a div per diff line.
        "#
        ));

        let response = cx.run_turn().await?;
        let tool_use = response.expect_tool("find_path", cx)?;
        let input = tool_use.parse_input::<FindPathToolInput>()?;

        let glob = input.glob;
        cx.assert(glob.ends_with(FILENAME), "glob ends with file name")?;

        let without_filename = glob.replace(FILENAME, "");
        let matches = Regex::new("(\\*\\*|zed)/(\\*\\*?/)?")
            .unwrap()
            .is_match(&without_filename);

        cx.assert(matches, "glob starts with `**` or project")?;

        Ok(())
    }
}
