use agent::GrepToolInput;
use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct GrepParamsEscapementExample;

/*

This eval checks that the model doesn't use HTML escapement for characters like `<` and
`>` in tool parameters.

                      original     +system_prompt change    +tool description
  claude-opus-4        89%          92%                     97%+
  claude-sonnet-4      100%
  gpt-4.1-mini         100%
  gemini-2.5-pro                    98%

*/

#[async_trait(?Send)]
impl Example for GrepParamsEscapementExample {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "grep_params_escapement".to_string(),
            url: "https://github.com/octocat/hello-world".to_string(),
            revision: "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d".to_string(),
            language_server: None,
            max_assertions: Some(1),
            profile_id: AgentProfileId::default(),
            existing_thread_json: None,
            max_turns: Some(2),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let response = cx
            .prompt_with_max_turns("Search for files containing the characters `>` or `<`", 2)
            .await?;
        let grep_input = response
            .find_tool_call("grep")
            .and_then(|tool_use| tool_use.parse_input::<GrepToolInput>().ok());

        cx.assert_some(grep_input.as_ref(), "`grep` tool should be called")?;

        cx.assert(
            !contains_html_entities(&grep_input.unwrap().regex),
            "Tool parameters should not be escaped",
        )
    }
}

fn contains_html_entities(pattern: &str) -> bool {
    regex::Regex::new(r"&[a-zA-Z]+;|&#[0-9]+;|&#x[0-9a-fA-F]+;")
        .unwrap()
        .is_match(pattern)
}
