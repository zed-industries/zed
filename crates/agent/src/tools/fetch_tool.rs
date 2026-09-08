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

/// The maximum number of HTTP redirects the fetch tool will follow. Each hop is
/// re-authorized against the shared network grants before being followed.
const MAX_REDIRECTS: usize = 20;

/// The outcome of a single (non-redirect-following) HTTP request.
enum FetchStep {
    /// The server responded with a redirect to this absolute URL. Its host must
    /// be authorized before the redirect is followed.
    Redirect(String),
    /// A terminal response was received and converted to Markdown.
    Complete(String),
}

/// Prepends `https://` when the URL has no explicit HTTP(S) scheme, matching the
/// behavior the fetch tool has always had for user/model-supplied URLs.
fn normalize_url(url: &str) -> Cow<'_, str> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        Cow::Owned(format!("https://{url}"))
    } else {
        Cow::Borrowed(url)
    }
}

/// Fetches a URL and returns the content as Markdown.
///
/// This tool is not run inside the terminal OS sandbox, but it still refuses to
/// reach any host that hasn't been granted network access. It shares the same
/// per-host grants as the `terminal` tool: approving a host for one authorizes
/// it for the other, whether the grant is for this thread or saved permanently.
/// HTTP redirects are followed one hop at a time, and each hop's host must be
/// granted the same way, so a granted host can't redirect the request to a host
/// that hasn't been approved.
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

    /// Performs a single HTTP GET *without* following redirects, so the tool can
    /// re-authorize each hop against the shared network grants before following
    /// it. Returns the redirect target when the server responds with a 3xx, or
    /// the final content converted to Markdown otherwise.
    async fn fetch_step(http_client: Arc<HttpClientWithUrl>, url: &str) -> Result<FetchStep> {
        let normalized = normalize_url(url);

        let mut response = http_client
            .get(&normalized, AsyncBody::default(), false)
            .await?;

        let status = response.status();
        if status.is_redirection() {
            let location = response
                .headers()
                .get("location")
                .context("redirect response is missing a Location header")?
                .to_str()
                .context("redirect response has an invalid Location header")?;
            let target = url::Url::parse(&normalized)
                .with_context(|| format!("could not parse URL {normalized:?}"))?
                .join(location)
                .with_context(|| format!("invalid redirect target {location:?}"))?;
            anyhow::ensure!(
                matches!(target.scheme(), "http" | "https"),
                "refusing to follow redirect to non-HTTP(S) URL {target}"
            );
            return Ok(FetchStep::Redirect(target.to_string()));
        }

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading response body")?;

        if status.is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!("status error {}, response: {text:?}", status.as_u16());
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

        let text = match content_type {
            ContentType::Html => {
                let mut handlers: Vec<TagHandler> = vec![
                    Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                    Rc::new(RefCell::new(markdown::ParagraphHandler)),
                    Rc::new(RefCell::new(markdown::HeadingHandler)),
                    Rc::new(RefCell::new(markdown::ListHandler)),
                    Rc::new(RefCell::new(markdown::TableHandler::new())),
                    Rc::new(RefCell::new(markdown::StyledTextHandler)),
                ];
                if normalized.contains("wikipedia.org") {
                    use html_to_markdown::structure::wikipedia;

                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                    handlers.push(Rc::new(
                        RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                    ));
                } else {
                    handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
                }

                convert_html_to_markdown(&body[..], &mut handlers)?
            }
            ContentType::Plaintext => std::str::from_utf8(&body)?.to_owned(),
            ContentType::Json => {
                let json: serde_json::Value = serde_json::from_slice(&body)?;

                format!("```json\n{}\n```", serde_json::to_string_pretty(&json)?)
            }
        };

        Ok(FetchStep::Complete(text))
    }
}

/// Resolve the host of `url` and confirm it doesn't point into loopback /
/// private / link-local space, applying the same forbidden-IP policy the
/// terminal sandbox's proxy uses. Returns an error (including "resolves only to
/// forbidden addresses") that aborts the fetch.
///
/// DNS resolution blocks, so callers should run this off the foreground thread.
/// See the caller for why this is a gate rather than a full resolve-to-connect
/// pin.
fn verify_host_not_forbidden(url: &str) -> Result<()> {
    let normalized = normalize_url(url);
    let parsed =
        url::Url::parse(&normalized).with_context(|| format!("could not parse URL {url:?}"))?;
    let host = parsed
        .host_str()
        .with_context(|| format!("URL {url:?} has no host to reach"))?;
    // Default to the scheme's port when the URL omits one; resolution needs a
    // port but the value doesn't affect which IPs a host resolves to.
    let port = parsed
        .port_or_known_default()
        .unwrap_or(if parsed.scheme() == "http" { 80 } else { 443 });

    http_proxy::PinnedHost::resolve(host, port).map(|_pinned| ())?;
    Ok(())
}

/// Extracts the host from a fetch URL as a [`http_proxy::HostPattern`] so it can
/// be matched against the shared network grants. Mirrors the scheme handling in
/// [`normalize_url`] (defaulting to `https://` when none is given).
fn host_pattern_for_url(url: &str) -> Result<http_proxy::HostPattern> {
    let normalized = normalize_url(url);
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
            //
            // Crucially, this authorization is applied to every redirect hop as
            // well as the initial URL, so a granted host can't 30x-redirect the
            // fetch to a host the user never approved. We disable the HTTP
            // client's own redirect following and re-run the grant for each hop
            // before requesting it.
            //
            // When the sandboxing feature flag is off the terminal isn't
            // sandboxed either, so gating fetch by host would provide no
            // isolation; skip it entirely (the pre-sandboxing behavior).
            let unsandboxed = cx.update(|cx| {
                !crate::sandboxing::sandboxing_enabled(cx)
                    || event_stream.unsandboxed_access_granted(cx)
            });

            let mut current_url = input.url.clone();
            let mut redirects = 0;
            let text = loop {
                if !unsandboxed {
                    let host = host_pattern_for_url(&current_url).map_err(|e| e.to_string())?;
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

                    // Authorizing the *hostname* is not enough: a granted host
                    // (or a redirect to one) whose DNS points into loopback /
                    // private / link-local space would otherwise let the model
                    // reach the local machine or LAN (SSRF / DNS rebinding).
                    // Resolve and vet the host now, applying the same
                    // forbidden-IP policy the terminal sandbox's proxy enforces.
                    //
                    // NOTE: this is a gate, not a full pin. `HttpClientWithUrl`
                    // resolves the hostname again when it connects, so a DNS
                    // answer that flips between this check and that connect could
                    // still slip through. Closing that residual window would
                    // require the HTTP client to connect to a pre-vetted IP
                    // (`PinnedHost::socket_addrs`) rather than re-resolving;
                    // until then this blocks the realistic case of a stably
                    // resolving host that points at forbidden space.
                    let verify_task = cx.background_spawn({
                        let url = current_url.clone();
                        async move { verify_host_not_forbidden(&url) }
                    });
                    futures::select! {
                        result = verify_task.fuse() => result.map_err(|e| e.to_string())?,
                        _ = event_stream.cancelled_by_user().fuse() => {
                            return Err("Fetch cancelled by user".to_string());
                        }
                    };
                }

                let fetch_task = cx.background_spawn({
                    let http_client = http_client.clone();
                    let url = current_url.clone();
                    async move { Self::fetch_step(http_client, &url).await }
                });

                let step = futures::select! {
                    result = fetch_task.fuse() => result.map_err(|e| e.to_string())?,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Fetch cancelled by user".to_string());
                    }
                };

                match step {
                    FetchStep::Complete(text) => break text,
                    FetchStep::Redirect(target) => {
                        redirects += 1;
                        if redirects > MAX_REDIRECTS {
                            return Err(format!(
                                "exceeded the maximum of {MAX_REDIRECTS} redirects"
                            ));
                        }
                        current_url = target;
                    }
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

    // These use IP-literal URLs, which "resolve" to themselves, so the SSRF gate
    // is exercised without depending on real DNS. IP literals can't be *granted*
    // network access (that's a separate, earlier check), but they can be the
    // target a granted hostname redirects to or resolves into — which is exactly
    // the case this gate defends.

    #[test]
    fn verify_host_rejects_loopback_literal() {
        let error = verify_host_not_forbidden("http://127.0.0.1/internal")
            .expect_err("loopback must be refused");
        assert!(
            error.to_string().contains("loopback"),
            "error should explain the forbidden range, got: {error}"
        );
    }

    #[test]
    fn verify_host_rejects_private_and_metadata_literals() {
        for url in [
            "http://10.0.0.5/",
            "https://192.168.1.1/",
            "http://169.254.169.254/latest/meta-data/", // cloud metadata
            "http://[::1]/",
        ] {
            assert!(
                verify_host_not_forbidden(url).is_err(),
                "expected {url} to be refused as a forbidden destination"
            );
        }
    }

    #[test]
    fn verify_host_allows_public_literal() {
        verify_host_not_forbidden("https://93.184.215.14/")
            .expect("a public address must be allowed through the gate");
    }
}
