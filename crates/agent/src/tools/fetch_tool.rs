use std::rc::Rc;
use std::sync::Arc;
use std::{borrow::Cow, cell::RefCell};

use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt as _, FutureExt as _, StreamExt as _};
use gpui::{App, AppContext as _, Task};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};
use http_client::{AsyncBody, HttpClientWithUrl};
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use ui::SharedString;
use util::markdown::{MarkdownEscaped, MarkdownInlineCode};

use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision,
    decide_permission_from_settings,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

/// Fetches a URL and returns the content as Markdown.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FetchToolInput {
    /// The URL to fetch.
    url: String,
}

pub struct FetchTool {
    http_client: Arc<HttpClientWithUrl>,
}

impl FetchTool {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }

    async fn build_message(http_client: Arc<HttpClientWithUrl>, url: &str) -> Result<String> {
        const MAX_BODY_SIZE: usize = 512 * 1024;

        let url = if !url.starts_with("https://") && !url.starts_with("http://") {
            Cow::Owned(format!("https://{url}"))
        } else {
            Cow::Borrowed(url)
        };

        let mut response = http_client.get(&url, AsyncBody::default(), true).await?;

        let mut body = Vec::with_capacity(MAX_BODY_SIZE);
        response
            .body_mut()
            .take(MAX_BODY_SIZE as u64)
            .read_to_end(&mut body)
            .await
            .context("error reading response body")?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        let Some(content_type) = response.headers().get("content-type") else {
            bail!("missing Content-Type header");
        };
        let content_type = content_type
            .to_str()
            .context("invalid Content-Type header")?;

        let content_type = if content_type.starts_with("text/plain") {
            ContentType::Plaintext
        } else if content_type.starts_with("application/json") {
            ContentType::Json
        } else {
            ContentType::Html
        };

        match content_type {
            ContentType::Html => {
                let mut handlers: Vec<TagHandler> = vec![
                    Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                    Rc::new(RefCell::new(markdown::ParagraphHandler)),
                    Rc::new(RefCell::new(markdown::HeadingHandler)),
                    Rc::new(RefCell::new(markdown::ListHandler)),
                    Rc::new(RefCell::new(markdown::TableHandler::new())),
                    Rc::new(RefCell::new(markdown::StyledTextHandler)),
                ];
                if url.contains("wikipedia.org") {
                    use html_to_markdown::structure::wikipedia;

                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                    handlers.push(Rc::new(
                        RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                    ));
                } else {
                    handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
                }

                convert_html_to_markdown(&body[..], &mut handlers)
            }
            ContentType::Plaintext => Ok(std::str::from_utf8(&body)?.to_owned()),
            ContentType::Json => {
                let json: serde_json::Value = serde_json::from_slice(&body)?;

                Ok(format!(
                    "```json\n{}\n```",
                    serde_json::to_string_pretty(&json)?
                ))
            }
        }
    }
}

impl AgentTool for FetchTool {
    type Input = FetchToolInput;
    type Output = String;

    const NAME: &'static str = "fetch";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Fetch
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => format!("Fetch {}", MarkdownEscaped(&input.url)).into(),
            Err(_) => "Fetch URL".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let http_client = self.http_client.clone();
        cx.spawn(async move |cx| {
            let input: FetchToolInput = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let decision = cx.update(|cx| {
                decide_permission_from_settings(
                    Self::NAME,
                    std::slice::from_ref(&input.url),
                    AgentSettings::get_global(cx),
                )
            });

            let authorize = match decision {
                ToolPermissionDecision::Allow => None,
                ToolPermissionDecision::Deny(reason) => {
                    return Err(reason);
                }
                ToolPermissionDecision::Confirm => Some(cx.update(|cx| {
                    let context =
                        crate::ToolPermissionContext::new(Self::NAME, vec![input.url.clone()]);
                    event_stream.authorize(
                        format!("Fetch {}", MarkdownInlineCode(&input.url)),
                        context,
                        cx,
                    )
                })),
            };

            let fetch_task = cx.background_spawn({
                let http_client = http_client.clone();
                let url = input.url.clone();
                async move {
                    if let Some(authorize) = authorize {
                        authorize.await?;
                    }
                    Self::build_message(http_client, &url).await
                }
            });

            let text = futures::select! {
                result = fetch_task.fuse() => result.map_err(|e| e.to_string())?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Fetch cancelled by user".to_string());
                }
            };
            if text.trim().is_empty() {
                return Err("no textual content found".to_string());
            }

            const MAX_CONTENT_LENGTH: usize = 30_000;

            if text.len() <= MAX_CONTENT_LENGTH {
                return Ok(text);
            }

            let total_length = text.len();

            let model = cx.update(|cx| {
                LanguageModelRegistry::global(cx).update(cx, |registry, _cx| {
                    registry.default_model().map(|configured| configured.model)
                })
            });

            if let Some(model) = model {
                let request = LanguageModelRequest {
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![format!(
                            "Summarize the following web page content. Preserve key facts, \
                             data, and structure. Be concise but thorough.\n\n{text}"
                        )
                        .into()],
                        cache: false,
                        reasoning_details: None,
                    }],
                    ..Default::default()
                };

                match model.stream_completion_text(request, cx).await {
                    Ok(text_stream) => {
                        let mut summary = String::new();
                        let mut stream = text_stream.stream;
                        while let Some(chunk) = stream.next().await {
                            match chunk {
                                Ok(text) => summary.push_str(&text),
                                Err(error) => {
                                    log::warn!(
                                        "error during fetch summarization stream: {error}"
                                    );
                                    break;
                                }
                            }
                        }
                        if !summary.trim().is_empty() {
                            return Ok(format!(
                                "[Summarized from {total_length} characters using a language model]\n\n\
                                 {summary}"
                            ));
                        }
                    }
                    Err(error) => {
                        log::warn!("failed to start fetch summarization: {error}");
                    }
                }
            }

            let truncated = truncate_at_boundary(&text, MAX_CONTENT_LENGTH);
            Ok(format!(
                "{truncated}\n\n[Content truncated: showing {shown} of {total_length} characters]",
                shown = truncated.len(),
            ))
        })
    }
}

fn truncate_at_boundary(text: &str, max_length: usize) -> &str {
    if text.len() <= max_length {
        return text;
    }

    let safe_end = text.floor_char_boundary(max_length);
    let search_region = &text[..safe_end];
    if let Some(position) = search_region.rfind("\n\n") {
        return &text[..position];
    }
    if let Some(position) = search_region.rfind('\n') {
        return &text[..position];
    }

    search_region
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_under_limit_returns_unchanged() {
        let text = "short text";
        assert_eq!(truncate_at_boundary(text, 100), "short text");
    }

    #[test]
    fn test_truncate_at_paragraph_boundary() {
        let text = "first paragraph\n\nsecond paragraph\n\nthird paragraph";
        let result = truncate_at_boundary(text, 40);
        assert_eq!(result, "first paragraph\n\nsecond paragraph");
    }

    #[test]
    fn test_truncate_at_line_boundary() {
        let text = "line one\nline two\nline three";
        let result = truncate_at_boundary(text, 20);
        assert_eq!(result, "line one\nline two");
    }

    #[test]
    fn test_truncate_with_no_newlines() {
        let text = "a".repeat(100);
        let result = truncate_at_boundary(&text, 50);
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_truncate_with_multibyte_utf8_near_boundary() {
        let text = "hello é world";
        let result = truncate_at_boundary(text, 7);
        assert!(!result.is_empty());
        assert!(result.len() <= 7);
    }
}
