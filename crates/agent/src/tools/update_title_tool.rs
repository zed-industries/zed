use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use gpui::{App, SharedString, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const MAX_TITLE_LEN: usize = 200;

/// Updates the current session title.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct UpdateTitleToolInput {
    /// A concise, human-readable title for the current session.
    pub title: String,
}

pub struct UpdateTitleTool {
    thread: WeakEntity<Thread>,
}

impl UpdateTitleTool {
    pub fn new(thread: WeakEntity<Thread>) -> Self {
        Self { thread }
    }

    pub(crate) fn title_for_input(
        input: Result<UpdateTitleToolInput, serde_json::Value>,
    ) -> SharedString {
        let Ok(input) = input else {
            return "Update title".into();
        };
        let Ok(title) = normalize_title(&input.title) else {
            return "Update title".into();
        };
        format!("Update title: {title}").into()
    }
}

impl AgentTool for UpdateTitleTool {
    type Input = UpdateTitleToolInput;
    type Output = String;

    const NAME: &'static str = "update_title";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Think
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        Self::title_for_input(input)
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let thread = self.thread.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| error.to_string())?;
            let title = normalize_title(&input.title)?;

            thread
                .update(cx, |thread, cx| {
                    thread.set_title(title.into(), cx);
                })
                .map_err(|error| error.to_string())?;

            Ok("Session title updated".to_string())
        })
    }

    fn replay(
        &self,
        input: Self::Input,
        _output: Self::Output,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let title = self.initial_title(Ok(input), cx).to_string();
        event_stream.update_fields(acp::ToolCallUpdateFields::new().title(title));
        Ok(())
    }
}

fn normalize_title(title: &str) -> Result<String, String> {
    let title = title.lines().next().unwrap_or("").trim();
    if title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    Ok(util::truncate_and_trailoff(title, MAX_TITLE_LEN))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn test_normalize_title() {
        assert_eq!(
            normalize_title("  Title from model\nignored").unwrap(),
            "Title from model"
        );
        assert!(normalize_title(" \nignored").is_err());
    }

    #[gpui::test]
    async fn test_initial_title(cx: &mut TestAppContext) {
        let tool = UpdateTitleTool::new(WeakEntity::new_invalid());

        let title = cx.update(|cx| {
            tool.initial_title(
                Ok(UpdateTitleToolInput {
                    title: "Investigate title updates".to_string(),
                }),
                cx,
            )
        });
        assert_eq!(
            title,
            SharedString::from("Update title: Investigate title updates")
        );

        let title = cx.update(|cx| {
            tool.initial_title(
                Ok(UpdateTitleToolInput {
                    title: " ".to_string(),
                }),
                cx,
            )
        });
        assert_eq!(title, SharedString::from("Update title"));
    }
}
