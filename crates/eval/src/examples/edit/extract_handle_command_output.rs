use crate::example::{Example, ExampleContext, ExampleMetadata};
use anyhow::Result;
use async_trait::async_trait;

pub struct ExtractHandleCommandOutput;

#[async_trait(?Send)]
impl Example for ExtractHandleCommandOutput {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "edit::extract_handle_command_output".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "58604fba86ebbffaa01f7c6834253e33bcd38c0f".to_string(),
            language_server: None,
            max_assertions: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(format!(
            r#"
            Read the `crates/git/src/blame.rs` file and extract a method in the final stanza of
            `run_git_blame` to deal with command failures, call it `handle_command_output`.

            IMPORTANT: You are only allowed to use the `read_file` and `edit_file` tools!
            "#
        ));
        cx.run_to_end().await?;
        // todo!("add assertions")

        Ok(())
    }
}
