use anyhow::Result;
use assistant_tool::Tool;
use assistant_tools::{OpenTool, TerminalTool};
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion};

pub struct Planets;

#[async_trait(?Send)]
impl Example for Planets {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "planets".to_string(),
            url: "https://github.com/roc-lang/roc".to_string(), // This commit in this repo is just the Apache2 license,
            revision: "59e49c75214f60b4dc4a45092292061c8c26ce27".to_string(), // so effectively a blank project.
            language_server: None,
            max_assertions: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(
            r#"
            Make a plain JavaScript web page which renders an animated 3D solar system.
            Let me drag to rotate the camera around.
            Do not use npm.
            "#
            .to_string(),
        );

        let response = cx.run_to_end().await?;
        let mut open_tool_uses = 0;
        let mut terminal_tool_uses = 0;

        for tool_use in response.tool_uses() {
            if tool_use.name == OpenTool.name() {
                open_tool_uses += 1;
            } else if tool_use.name == TerminalTool::NAME {
                terminal_tool_uses += 1;
            }
        }

        // The open tool should only be used when requested, which it was not.
        cx.assert_eq(open_tool_uses, 0, "`open` tool was not used")
            .ok();
        // No reason to use the terminal if not using npm.
        cx.assert_eq(terminal_tool_uses, 0, "`terminal` tool was not used")
            .ok();

        Ok(())
    }

    fn diff_assertions(&self) -> Vec<JudgeAssertion> {
        vec![
            JudgeAssertion {
                id: "animated solar system".to_string(),
                description: "This page should render a solar system, and it should be animated."
                    .to_string(),
            },
            JudgeAssertion {
                id: "drag to rotate camera".to_string(),
                description: "The user can drag to rotate the camera around.".to_string(),
            },
            JudgeAssertion {
                id: "plain JavaScript".to_string(),
                description:
                    "The code base uses plain JavaScript and no npm, along with HTML and CSS."
                        .to_string(),
            },
        ]
    }
}
