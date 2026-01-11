use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use cloud_llm_client::WebSearchResponse;
use futures::FutureExt as _;
use gpui::{App, AppContext, Task};
use language_model::{
    LanguageModelProviderId, LanguageModelToolResultContent, ZED_CLOUD_PROVIDER_ID,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use web_search::WebSearchRegistry;

/// Search the web for information using your query.
/// Use this when you need real-time information, facts, or data that might not be in your training.
/// Results will include snippets and links from relevant web pages.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchToolInput {
    /// The search term or question to query on the web.
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebSearchToolOutput(WebSearchResponse);

impl From<WebSearchToolOutput> for LanguageModelToolResultContent {
    fn from(value: WebSearchToolOutput) -> Self {
        serde_json::to_string(&value.0)
            .expect("Failed to serialize WebSearchResponse")
            .into()
    }
}

pub struct WebSearchTool;

impl AgentTool for WebSearchTool {
    type Input = WebSearchToolInput;
    type Output = WebSearchToolOutput;

    fn name() -> &'static str {
        "web_search"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Fetch
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Searching the Web".into()
    }

    /// We currently only support Zed Cloud as a provider.
    fn supports_provider(provider: &LanguageModelProviderId) -> bool {
        provider == &ZED_CLOUD_PROVIDER_ID
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let Some(provider) = WebSearchRegistry::read_global(cx).active_provider() else {
            return Task::ready(Err(anyhow!("Web search is not available.")));
        };

        let search_task = provider.search(input.query, cx);
        cx.background_spawn(async move {
            let response = futures::select! {
                result = search_task.fuse() => {
                    match result {
                        Ok(response) => response,
                        Err(err) => {
                            event_stream
                                .update_fields(acp::ToolCallUpdateFields::new().title("Web Search Failed"));
                            return Err(err);
                        }
                    }
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Web search cancelled by user");
                }
            };

            emit_update(&response, &event_stream);
            Ok(WebSearchToolOutput(response))
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        emit_update(&output.0, &event_stream);
        Ok(())
    }
}

fn emit_update(response: &WebSearchResponse, event_stream: &ToolCallEventStream) {
    let result_text = if response.results.len() == 1 {
        "1 result".to_string()
    } else {
        format!("{} results", response.results.len())
    };
    event_stream.update_fields(
        acp::ToolCallUpdateFields::new()
            .title(format!("Searched the web: {result_text}"))
            .content(
                response
                    .results
                    .iter()
                    .map(|result| {
                        acp::ToolCallContent::Content(acp::Content::new(
                            acp::ContentBlock::ResourceLink(
                                acp::ResourceLink::new(result.title.clone(), result.url.clone())
                                    .title(result.title.clone())
                                    .description(result.text.clone()),
                            ),
                        ))
                    })
                    .collect::<Vec<_>>(),
            ),
    );
}
