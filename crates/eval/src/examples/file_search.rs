use anyhow::Result;
use assistant_tools::PathSearchToolInput;
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
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let ends_with_filename = cx.assertion("ends_with_filename");
        let correct_glob = cx.assertion("correct_glob");
        let used_path_search = cx.assertion("used_path_search");

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
        let tool_use = response.expect_tool(used_path_search, "path_search", cx)?;
        let input = tool_use.parse_input::<PathSearchToolInput>()?;

        let glob = input.glob;
        ends_with_filename.assert(
            glob.ends_with(FILENAME),
            format!("glob ends with `{FILENAME}`"),
            cx,
        )?;

        let without_filename = glob.replace(FILENAME, "");
        let matches = Regex::new("(\\*\\*|zed)/(\\*\\*?/)?")
            .unwrap()
            .is_match(&without_filename);

        correct_glob.assert(matches, "glob starts with either `**` or `zed`", cx)?;

        Ok(())
    }
}
