use std::sync::Arc;

use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision,
    decide_permission_from_settings,
};
use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::Result;
use cloud_llm_client::WebSearchResponse;
use futures::FutureExt as _;
use gpui::{App, Task};
use language_model::{
    LanguageModelProviderId, LanguageModelToolResultContent, ZED_CLOUD_PROVIDER_ID,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use ui::prelude::*;
use util::markdown::MarkdownInlineCode;
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
#[serde(untagged)]
pub enum WebSearchToolOutput {
    Success(WebSearchResponse),
    Error { error: String },
}

impl From<WebSearchToolOutput> for LanguageModelToolResultContent {
    fn from(value: WebSearchToolOutput) -> Self {
        match value {
            WebSearchToolOutput::Success(response) => serde_json::to_string(&response)
                .unwrap_or_else(|e| format!("Failed to serialize web search response: {e}"))
                .into(),
            WebSearchToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct WebSearchTool;

impl AgentTool for WebSearchTool {
    type Input = WebSearchToolInput;
    type Output = WebSearchToolOutput;

    const NAME: &'static str = "web_search";

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
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| WebSearchToolOutput::Error {
                    error: format!("Failed to receive tool input: {e}"),
                })?;

            let (authorize, search_task) = cx.update(|cx| {
                let decision = decide_permission_from_settings(
                    Self::NAME,
                    std::slice::from_ref(&input.query),
                    AgentSettings::get_global(cx),
                );

                let authorize = match decision {
                    ToolPermissionDecision::Allow => None,
                    ToolPermissionDecision::Deny(reason) => {
                        return Err(WebSearchToolOutput::Error { error: reason });
                    }
                    ToolPermissionDecision::Confirm => {
                        let context =
                            crate::ToolPermissionContext::new(Self::NAME, vec![input.query.clone()]);
                        Some(event_stream.authorize(
                            format!("Search the web for {}", MarkdownInlineCode(&input.query)),
                            context,
                            cx,
                        ))
                    }
                };

                let Some(provider) = WebSearchRegistry::read_global(cx).active_provider() else {
                    return Err(WebSearchToolOutput::Error {
                        error: "Web search is not available.".to_string(),
                    });
                };

                let search_task = provider.search(input.query, cx);
                Ok((authorize, search_task))
            })?;

            if let Some(authorize) = authorize {
                authorize.await.map_err(|e| WebSearchToolOutput::Error { error: e.to_string() })?;
            }

            let response = futures::select! {
                result = search_task.fuse() => {
                    match result {
                        Ok(response) => response,
                        Err(err) => {
                            event_stream
                                .update_fields(acp::ToolCallUpdateFields::new().title("Web Search Failed"));
                            return Err(WebSearchToolOutput::Error { error: err.to_string() });
                        }
                    }
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err(WebSearchToolOutput::Error { error: "Web search cancelled by user".to_string() });
                }
            };

            emit_update(&response, &event_stream);
            Ok(WebSearchToolOutput::Success(response))
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        if let WebSearchToolOutput::Success(response) = &output {
            emit_update(response, &event_stream);
        }
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
