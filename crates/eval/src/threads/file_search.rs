use assistant_tools::PathSearchToolInput;
use regex::Regex;

use super::{EvalThread, EvalThreadMetadata, ThreadContext};

pub struct Thread;

impl EvalThread for Thread {
    fn meta() -> EvalThreadMetadata {
        EvalThreadMetadata {
            name: "test path_search",
            url: "https://github.com/zed-industries/zed.git",
            revision: "03ecb88fe30794873f191ddb728f597935b3101c",
            lang: "rs",
            max_assertions: 4,
        }
    }

    async fn run(cx: &mut ThreadContext) -> anyhow::Result<()> {
        const FILENAME: &str = "find_replace_file_tool.rs";
        cx.push_user_message(format!(
                r#"
        Look at the `{FILENAME}`. I want to implement a card for it. The card should implement the `Render` trait.

        The card should show a diff. It should be a beautifully presented diff. The card "box" should look like what we show for
        markdown codeblocks (look at `MarkdownElement`). I want to see a red background for lines that were deleted and a green
        background for lines that were added. We should have a div per diff line.
        "#
        ));

        let response = cx.run_turns(1).await?;

        let tool_use = response.expect_tool("path_search", cx)?;
        let input = tool_use.expect_input::<PathSearchToolInput>(cx)?;

        let glob = input.glob;
        cx.assert(
            glob.ends_with(FILENAME),
            "Expected path_search glob to end with {FILENAME:?}, but glob was {glob:?}",
        )?;

        let without_filename = glob.replace(FILENAME, "");
        let matches = Regex::new("(\\*\\*|zed)/(\\*\\*?/)?")
            .unwrap()
            .is_match(&without_filename);

        cx.assert(matches, "Expected path_search glob to start with either \"**/\" or \"zed/\", optionally with \"*/\" in the middle, but glob was {glob:?}")?;

        Ok(())
    }
}
