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

use crate::sandboxing::{NetworkRequest, SandboxRequest};
use crate::{AgentTool, ToolCallEventStream, ToolInput};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

/// Fetches a URL and returns the content as Markdown.
///
/// This tool is not run inside the terminal OS sandbox, but it still refuses to
/// reach any host that hasn't been granted network access. It shares the same
/// per-host grants as the `terminal` tool: approving a host for one authorizes
/// it for the other, whether the grant is for this thread or saved permanently.
/// When unsandboxed access has been granted, these restrictions are lifted
/// entirely, matching the terminal, which is also how loopback and IP-literal
/// hosts (which can't be granted individually) become reachable.
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

/// Extracts the host from a fetch URL as a [`http_proxy::HostPattern`] so it can
/// be matched against the shared network grants. Mirrors the scheme handling in
/// [`FetchTool::build_message`] (defaulting to `https://` when none is given).
fn host_pattern_for_url(url: &str) -> Result<http_proxy::HostPattern> {
    let normalized = if !url.starts_with("https://") && !url.starts_with("http://") {
        Cow::Owned(format!("https://{url}"))
    } else {
        Cow::Borrowed(url)
    };
    let parsed =
        url::Url::parse(&normalized).with_context(|| format!("could not parse URL {url:?}"))?;
    let host = parsed
        .host_str()
        .with_context(|| format!("URL {url:?} has no host to authorize network access for"))?;
    http_proxy::HostPattern::parse(host).map_err(|error| match error {
        http_proxy::HostPatternError::IpLiteral(_) => anyhow::anyhow!(
            "cannot fetch {host:?}: loopback and IP-literal hosts can't be granted network \
             access individually. They are only reachable once unsandboxed access has been \
             granted (for example, via a terminal command that requests it)."
        ),
        error => anyhow::anyhow!("cannot authorize network access to {host:?}: {error}"),
    })
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

            // First, the standard tool-permission gate (honors the fetch tool's
            // allow/deny/confirm rules).
            let authorize = cx.update(|cx| {
                let context =
                    crate::ToolPermissionContext::new(Self::NAME, vec![input.url.clone()]);

                event_stream.authorize(
                    format!("Fetch {}", MarkdownInlineCode(&input.url)),
                    context,
                    cx,
                )
            });
            futures::select! {
                result = authorize.fuse() => result.map_err(|e| e.to_string())?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Fetch cancelled by user".to_string());
                }
            };

            // Then, unless unsandboxed access is already in effect, the per-host
            // network grant shared with the terminal tool. If the host isn't
            // already granted (for this thread or in saved settings) the user is
            // shown the same escalation prompt the terminal uses; a denial
            // aborts the fetch. This tool never runs inside the OS sandbox, so
            // the grant is only consulted to decide whether the request may
            // proceed. When unsandboxed access has been granted the terminal
            // already runs without isolation, so we drop fetch's restrictions
            // too — including reaching hosts that can't be granted individually
            // (loopback and IP literals).
            let unsandboxed = cx.update(|cx| event_stream.unsandboxed_access_granted(cx));
            if !unsandboxed {
                let host = host_pattern_for_url(&input.url).map_err(|e| e.to_string())?;
                let authorize_host = cx.update(|cx| {
                    let request = SandboxRequest {
                        network: NetworkRequest::Hosts(vec![host]),
                        ..Default::default()
                    };
                    event_stream.authorize_sandbox(request, String::new(), cx)
                });
                futures::select! {
                    result = authorize_host.fuse() => result.map_err(|e| e.to_string())?,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Fetch cancelled by user".to_string());
                    }
                };
            }

            let fetch_task = cx.background_spawn({
                let http_client = http_client.clone();
                let url = input.url.clone();
                async move { Self::build_message(http_client, &url).await }
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
