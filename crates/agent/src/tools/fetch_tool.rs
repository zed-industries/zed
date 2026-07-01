use std::rc::Rc;
use std::sync::Arc;
use std::{borrow::Cow, cell::RefCell};

use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt as _, FutureExt as _};
use gpui::{App, AppContext as _, Task};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};
use http_client::{AsyncBody, HttpClientWithUrl};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::SharedString;
use util::markdown::{MarkdownEscaped, MarkdownInlineCode};

use crate::{AgentTool, ToolCallEventStream, ToolInput};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

const DEFAULT_MAX_LENGTH: usize = 5000;

fn default_max_length() -> usize {
    DEFAULT_MAX_LENGTH
}

/// Fetches a URL and returns the content as Markdown.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FetchToolInput {
    /// The URL to fetch.
    url: String,
    /// Maximum number of characters to return. Defaults to 5000. Increase this
    /// to read more of a large page in one call, keeping in mind that the
    /// content counts against the context window.
    #[serde(default = "default_max_length")]
    max_length: usize,
    /// Character index to start returning content from. Defaults to 0. Use this
    /// together with `max_length` to page through content that was truncated by
    /// a previous call.
    #[serde(default)]
    start_index: usize,
}

pub struct FetchTool {
    http_client: Arc<HttpClientWithUrl>,
}

/// Returns the `max_length`-character window of `content` starting at
/// `start_index`. When more content follows the window, a note is appended
/// telling the caller how to fetch the rest.
fn window_content(content: &str, start_index: usize, max_length: usize) -> String {
    let total = content.chars().count();
    let start = start_index.min(total);
    let window: String = content.chars().skip(start).take(max_length).collect();
    let end = start + window.chars().count();

    if end < total {
        format!(
            "{window}\n\n[Showing characters {start}-{end} of {total}. \
             Call fetch again with start_index={end} to read more.]"
        )
    } else {
        window
    }
}

impl FetchTool {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }

    async fn build_message(
        http_client: Arc<HttpClientWithUrl>,
        url: &str,
        start_index: usize,
        max_length: usize,
    ) -> Result<String> {
        let url = if !url.starts_with("https://") && !url.starts_with("http://") {
            Cow::Owned(format!("https://{url}"))
        } else {
            Cow::Borrowed(url)
        };

        let mut response = http_client.get(&url, AsyncBody::default(), true).await?;

        let mut body = Vec::new();
        response
            .body_mut()
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

        let content = match content_type {
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
        }?;

        Ok(window_content(&content, start_index, max_length))
    }
}

impl AgentTool for FetchTool {
    type Input = FetchToolInput;
    type Output = String;

    const NAME: &'static str = "fetch";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Fetch
    }

    fn allow_in_restricted_mode() -> bool {
        false
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
            let input: FetchToolInput = input.recv().await.map_err(|e| e.to_string())?;

            let authorize = cx.update(|cx| {
                let context =
                    crate::ToolPermissionContext::new(Self::NAME, vec![input.url.clone()]);

                event_stream.authorize(
                    format!("Fetch {}", MarkdownInlineCode(&input.url)),
                    context,
                    cx,
                )
            });

            let fetch_task = cx.background_spawn({
                let http_client = http_client.clone();
                let url = input.url.clone();
                let start_index = input.start_index;
                let max_length = input.max_length;
                async move {
                    authorize.await?;
                    Self::build_message(http_client, &url, start_index, max_length).await
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
            Ok(text)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_content_is_returned_unchanged() {
        assert_eq!(window_content("hello", 0, 5000), "hello");
    }

    #[test]
    fn long_content_is_truncated_with_a_note() {
        let content = "abcdefghij";
        assert_eq!(
            window_content(content, 0, 4),
            "abcd\n\n[Showing characters 0-4 of 10. \
             Call fetch again with start_index=4 to read more.]"
        );
    }

    #[test]
    fn start_index_pages_through_content() {
        let content = "abcdefghij";
        assert_eq!(
            window_content(content, 4, 4),
            "efgh\n\n[Showing characters 4-8 of 10. \
             Call fetch again with start_index=8 to read more.]"
        );
        // Reaching the end drops the note.
        assert_eq!(window_content(content, 8, 4), "ij");
    }

    #[test]
    fn start_index_past_the_end_yields_nothing() {
        assert_eq!(window_content("abc", 10, 4), "");
    }

    #[test]
    fn truncation_respects_character_boundaries() {
        // Three 4-byte emoji; a byte-based slice would panic or split them.
        let content = "😀😁😂";
        assert_eq!(
            window_content(content, 0, 2),
            "😀😁\n\n[Showing characters 0-2 of 3. \
             Call fetch again with start_index=2 to read more.]"
        );
    }
}
