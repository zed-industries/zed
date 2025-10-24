use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion};
use agent::{EditFileMode, EditFileToolInput};
use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;

pub struct CommentTranslation;

#[async_trait(?Send)]
impl Example for CommentTranslation {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "comment_translation".to_string(),
            url: "https://github.com/servo/font-kit.git".to_string(),
            revision: "504d084e29bce4f60614bc702e91af7f7d9e60ad".to_string(),
            language_server: None,
            max_assertions: Some(1),
            profile_id: AgentProfileId::default(),
            existing_thread_json: None,
            max_turns: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let response = cx.prompt(
            r#"
                Edit the following files and translate all their comments to italian, in this exact order:

                - font-kit/src/family.rs
                - font-kit/src/canvas.rs
                - font-kit/src/error.rs
            "#
        ).await?;

        let mut create_or_overwrite_count = 0;
        for tool_call in response.tool_calls() {
            if tool_call.name == "edit_file" {
                let input = tool_call.parse_input::<EditFileToolInput>()?;
                if !matches!(input.mode, EditFileMode::Edit) {
                    create_or_overwrite_count += 1;
                }
            }
        }

        cx.assert_eq(create_or_overwrite_count, 0, "no_creation_or_overwrite")?;

        Ok(())
    }

    fn diff_assertions(&self) -> Vec<JudgeAssertion> {
        vec![JudgeAssertion {
            id: "comments_translated".to_string(),
            description: concat!(
                "- Only `family.rs`, `canvas.rs` and `error.rs` should have changed.\n",
                "- Their doc comments should have been all translated to Italian."
            )
            .into(),
        }]
    }
}
