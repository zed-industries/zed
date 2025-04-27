use crate::example::{Example, ExampleContext, ExampleMetadata};
use anyhow::Result;
use async_trait::async_trait;

pub struct DeleteRunGitBlame;

#[async_trait(?Send)]
impl Example for DeleteRunGitBlame {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "edit::delete_run_git_blame".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "58604fba86ebbffaa01f7c6834253e33bcd38c0f".to_string(),
            language_server: None,
            max_assertions: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(format!(
            r#"
            Read the `crates/git/src/blame.rs` file and delete `run_git_blame`. Just that
            one function, not its usages.

            IMPORTANT: You are only allowed to use the `read_file` and `edit_file` tools!
            "#
        ));
        cx.run_to_end().await?;
        // todo!("add assertions")

        Ok(())
    }
}
