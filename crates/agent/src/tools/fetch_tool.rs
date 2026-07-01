use std::rc::Rc;
use std::sync::Arc;
use std::{borrow::Cow, cell::RefCell};

use agent_client_protocol::schema::v1 as acp;
use agent_settings::SandboxPermissions;
use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt as _, FutureExt as _};
use gpui::{App, AppContext as _, AsyncApp, Task};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};
use http_client::{AsyncBody, Host, HttpClientWithUrl, Url, http};
use http_proxy::{HostPattern, HostPatternError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use ui::SharedString;
use util::markdown::{MarkdownEscaped, MarkdownInlineCode};

use crate::sandboxing::{NetworkRequest, SandboxRequest, sandboxing_enabled};
use crate::{AgentTool, ToolCallEventStream, ToolInput, ToolPermissionContext};

const MAX_REDIRECTS: usize = 20;

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

struct FetchResponse {
    status: http::StatusCode,
    headers: http::HeaderMap,
    body: Vec<u8>,
}

enum FetchStep {
    Redirect { location: String },
    Done(String),
}

/// Network-host approvals collected during a single fetch (including its
/// redirect chain). This is intentionally *not* the thread-wide
/// [`crate::sandboxing::ThreadSandboxGrants`]: a fetch's "allow once" approval
/// applies only to the rest of that fetch and is never shared with later
/// fetches or with terminal commands. Persistent "allow always" grants from
/// settings are still honored on top of these (see
/// [`Self::covers_with_persistent`]).
#[derive(Default)]
struct FetchNetworkGrants {
    any_host: bool,
    hosts: Vec<HostPattern>,
}

impl FetchNetworkGrants {
    fn covers_with_persistent(
        &self,
        request: &NetworkRequest,
        persistent: &SandboxPermissions,
    ) -> bool {
        match request {
            NetworkRequest::None => true,
            NetworkRequest::AnyHost => self.any_host || persistent.allow_all_hosts,
            NetworkRequest::Hosts(requested) => {
                if self.any_host || persistent.allow_all_hosts {
                    return true;
                }

                let persistent_hosts = Self::parse_persistent_hosts(&persistent.network_hosts);
                requested.iter().all(|requested| {
                    self.hosts
                        .iter()
                        .chain(persistent_hosts.iter())
                        .any(|granted| granted.covers(requested))
                })
            }
        }
    }

    fn record(&mut self, request: &NetworkRequest) {
        match request {
            NetworkRequest::None => {}
            NetworkRequest::AnyHost => self.any_host = true,
            NetworkRequest::Hosts(hosts) => {
                for host in hosts {
                    crate::sandboxing::insert_host_pattern(&mut self.hosts, host.clone());
                }
            }
        }
    }

    fn parse_persistent_hosts(raw_hosts: &[String]) -> Vec<HostPattern> {
        raw_hosts
            .iter()
            .filter_map(|host| match HostPattern::parse(host) {
                Ok(pattern) => Some(pattern),
                Err(error) => {
                    log::warn!(
                        "ignoring invalid network host pattern '{host}' in sandbox settings: {error}"
                    );
                    None
                }
            })
            .collect()
    }
}

impl FetchTool {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }

    fn normalize_url(url: &str) -> Result<Url> {
        let url = url.trim();
        let url = if !url.starts_with("https://") && !url.starts_with("http://") {
            Cow::Owned(format!("https://{url}"))
        } else {
            Cow::Borrowed(url)
        };
        let url = Url::parse(&url).with_context(|| format!("invalid URL {url:?}"))?;
        Self::validate_fetch_url(&url)?;
        Ok(url)
    }

    fn validate_fetch_url(url: &Url) -> Result<()> {
        match url.scheme() {
            "http" | "https" => Ok(()),
            scheme => {
                bail!("unsupported URL scheme {scheme:?}; fetch only supports HTTP and HTTPS")
            }
        }
    }

    fn permission_inputs_for_url(
        url: &Url,
        inputs: impl IntoIterator<Item = String>,
    ) -> Vec<String> {
        let mut permission_inputs = Vec::new();
        for input in inputs {
            let input = input.trim();
            if !input.is_empty() && !permission_inputs.iter().any(|existing| existing == input) {
                permission_inputs.push(input.to_string());
            }
        }

        let normalized_url = url.as_str();
        if !permission_inputs
            .iter()
            .any(|existing| existing == normalized_url)
        {
            permission_inputs.push(normalized_url.to_string());
        }

        permission_inputs
    }

    /// Fetches `initial_url` and returns the page contents as Markdown.
    ///
    /// The sandboxing feature flag selects between two behaviors:
    ///
    /// * Off (the default today): the requested URL is authorized once and the
    ///   HTTP client follows redirects transparently — exactly how the fetch
    ///   tool has always behaved. None of the network-host gating below applies.
    /// * On: redirects are followed manually so every hop is re-authorized
    ///   against the fetch tool-permission rules *and* the sandbox network-host
    ///   allowlist before the request goes out, and localhost / IP-literal URLs
    ///   are rejected (see [`Self::authorize_network_for_url`]). Network grants
    ///   are scoped to this single fetch (including its redirect chain); they
    ///   are deliberately never recorded as thread-scoped or persistent grants.
    async fn build_message(
        http_client: Arc<HttpClientWithUrl>,
        event_stream: ToolCallEventStream,
        initial_url: Url,
        initial_permission_inputs: Vec<String>,
        sandboxing: bool,
        cx: &mut AsyncApp,
    ) -> Result<String> {
        if !sandboxing {
            Self::authorize_fetch_for_url(
                &event_stream,
                &initial_url,
                initial_permission_inputs,
                cx,
            )
            .await?;
            return cx
                .background_spawn(async move {
                    Self::fetch_following_redirects(http_client, initial_url).await
                })
                .await;
        }

        let mut network_grants = FetchNetworkGrants::default();
        let mut url = initial_url;
        let mut permission_inputs = initial_permission_inputs;

        for redirect_count in 0..=MAX_REDIRECTS {
            Self::authorize_fetch_for_url(&event_stream, &url, permission_inputs.clone(), cx)
                .await?;
            Self::authorize_network_for_url(&event_stream, &url, &mut network_grants, cx).await?;

            let step = cx
                .background_spawn({
                    let http_client = http_client.clone();
                    let url = url.clone();
                    async move { Self::fetch_once(http_client, url).await }
                })
                .await?;

            match step {
                FetchStep::Done(message) => return Ok(message),
                FetchStep::Redirect { location } => {
                    if redirect_count == MAX_REDIRECTS {
                        bail!("too many redirects fetching {url}");
                    }

                    let next_url = url.join(&location).with_context(|| {
                        format!("invalid redirect Location header {location:?}")
                    })?;
                    Self::validate_fetch_url(&next_url)?;
                    permission_inputs = Self::permission_inputs_for_url(&next_url, [location]);
                    url = next_url;
                }
            }
        }

        unreachable!("redirect loop exits by returning a response or bailing")
    }

    /// Fetches `url`, letting the HTTP client follow redirects transparently.
    /// Used when the sandboxing feature is off, preserving the fetch tool's
    /// original behavior; under the sandbox, redirects are instead followed
    /// manually via [`Self::fetch_once`] so each hop can be authorized.
    async fn fetch_following_redirects(
        http_client: Arc<HttpClientWithUrl>,
        url: Url,
    ) -> Result<String> {
        let mut response = http_client
            .get(url.as_str(), AsyncBody::default(), true)
            .await?;
        let status = response.status();
        let headers = response.headers().clone();

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("error reading response body from {url}"))?;

        Self::message_from_response(
            &url,
            FetchResponse {
                status,
                headers,
                body,
            },
        )
    }

    async fn authorize_fetch_for_url(
        event_stream: &ToolCallEventStream,
        url: &Url,
        permission_inputs: Vec<String>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let url_for_permissions = url.as_str().to_string();
        let title = format!("Fetch {}", MarkdownInlineCode(&url_for_permissions));
        let context = ToolPermissionContext::new(Self::NAME, vec![url_for_permissions]);

        cx.update(|cx| {
            event_stream.authorize_with_settings_check(
                title,
                context,
                move |cx| {
                    crate::tool_permissions::decide_permission_for_input_alternatives(
                        Self::NAME,
                        &permission_inputs,
                        agent_settings::AgentSettings::get_global(cx),
                    )
                },
                cx,
            )
        })
        .await
    }

    /// Gates the outbound request to `url`'s host on the sandbox network
    /// allowlist. Only reached when the sandboxing feature is enabled (see
    /// [`Self::build_message`]).
    ///
    /// localhost and IP-literal hosts are rejected outright and cannot be
    /// granted — not even via the persistent `allow_all_hosts` setting — because
    /// the host-pattern model can't express them and they're the classic SSRF
    /// target. A grant approved here lasts only for the current fetch (its
    /// `fetch_grants`); persistent settings grants are honored, but thread
    /// grants are intentionally neither consulted nor created.
    async fn authorize_network_for_url(
        event_stream: &ToolCallEventStream,
        url: &Url,
        fetch_grants: &mut FetchNetworkGrants,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let network = Self::network_request_for_url(url)?;
        let persistent = cx.update(|cx| {
            agent_settings::AgentSettings::get_global(cx)
                .sandbox_permissions
                .clone()
        });
        if fetch_grants.covers_with_persistent(&network, &persistent) {
            return Ok(());
        }

        let request = SandboxRequest {
            network,
            allow_git_access: false,
            allow_fs_write_all: false,
            unsandboxed: false,
            write_paths: Vec::new(),
        };
        if !request.needs_escalation() {
            return Ok(());
        }

        let title = Self::sandbox_approval_title(&request.network);
        let reason = format!("The fetch tool needs network access to retrieve {url}.");
        cx.update(|cx| {
            event_stream.authorize_sandbox_once(title, None, request.clone(), reason, cx)
        })
        .await?;

        fetch_grants.record(&request.network);
        Ok(())
    }

    fn fetch_network_sandboxing_enabled(cx: &App) -> bool {
        sandboxing_enabled(cx)
            && !agent_settings::AgentSettings::get_global(cx)
                .sandbox_permissions
                .allow_unsandboxed
            && cfg!(any(
                target_os = "macos",
                target_os = "linux",
                target_os = "windows"
            ))
    }

    fn network_request_for_url(url: &Url) -> Result<NetworkRequest> {
        let Some(host) = url.host() else {
            bail!("URL must include a host: {url}");
        };

        match host {
            Host::Domain(host) => {
                if Self::is_localhost_domain(host) {
                    bail!("fetch sandboxing does not allow localhost or IP literal URLs: {url}");
                }

                match HostPattern::parse(host) {
                    Ok(pattern) => Ok(NetworkRequest::Hosts(vec![pattern])),
                    Err(HostPatternError::IpLiteral(_)) => {
                        bail!("fetch sandboxing does not allow localhost or IP literal URLs: {url}")
                    }
                    Err(error) => Err(error).with_context(|| {
                        format!("cannot request sandbox network access for {host:?}")
                    }),
                }
            }
            Host::Ipv4(_) | Host::Ipv6(_) => {
                bail!("fetch sandboxing does not allow localhost or IP literal URLs: {url}")
            }
        }
    }

    fn is_localhost_domain(host: &str) -> bool {
        host.eq_ignore_ascii_case("localhost") || host.to_ascii_lowercase().ends_with(".localhost")
    }

    fn sandbox_approval_title(network: &NetworkRequest) -> String {
        match network {
            NetworkRequest::None => "Allow fetch network access?".to_string(),
            NetworkRequest::AnyHost => "Allow arbitrary network access for fetch?".to_string(),
            NetworkRequest::Hosts(hosts) => {
                let hosts = hosts.iter().map(ToString::to_string).collect::<Vec<_>>();
                match hosts.as_slice() {
                    [] => "Allow fetch network access?".to_string(),
                    [host] => format!("Allow fetch network access to {host}?"),
                    [first, second] => {
                        format!("Allow fetch network access to {first} and {second}?")
                    }
                    _ => {
                        if let Some((last, init)) = hosts.split_last() {
                            format!(
                                "Allow fetch network access to {}, and {last}?",
                                init.join(", ")
                            )
                        } else {
                            "Allow fetch network access?".to_string()
                        }
                    }
                }
            }
        }
    }

    async fn fetch_once(http_client: Arc<HttpClientWithUrl>, url: Url) -> Result<FetchStep> {
        let mut response = http_client
            .get(url.as_str(), AsyncBody::default(), false)
            .await?;
        let status = response.status();
        let headers = response.headers().clone();

        if Self::is_followable_redirect(status) {
            return Ok(FetchStep::Redirect {
                location: Self::redirect_location(&headers)?.to_string(),
            });
        }

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("error reading response body from {url}"))?;

        Self::message_from_response(
            &url,
            FetchResponse {
                status,
                headers,
                body,
            },
        )
        .map(FetchStep::Done)
    }

    fn is_followable_redirect(status: http::StatusCode) -> bool {
        matches!(
            status,
            http::StatusCode::MOVED_PERMANENTLY
                | http::StatusCode::FOUND
                | http::StatusCode::SEE_OTHER
                | http::StatusCode::TEMPORARY_REDIRECT
                | http::StatusCode::PERMANENT_REDIRECT
        )
    }

    fn redirect_location(headers: &http::HeaderMap) -> Result<&str> {
        let Some(location) = headers.get(http::header::LOCATION) else {
            bail!("redirect response missing Location header");
        };
        location.to_str().context("invalid Location header")
    }

    fn message_from_response(url: &Url, response: FetchResponse) -> Result<String> {
        if response.status.is_client_error() {
            let text = String::from_utf8_lossy(response.body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status.as_u16()
            );
        }

        let Some(content_type) = response.headers.get("content-type") else {
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
                if Self::is_wikipedia_url(url) {
                    use html_to_markdown::structure::wikipedia;

                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                    handlers.push(Rc::new(
                        RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                    ));
                } else {
                    handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
                }

                convert_html_to_markdown(&response.body[..], &mut handlers)
            }
            ContentType::Plaintext => Ok(std::str::from_utf8(&response.body)?.to_owned()),
            ContentType::Json => {
                let json: serde_json::Value = serde_json::from_slice(&response.body)?;

                Ok(format!(
                    "```json\n{}\n```",
                    serde_json::to_string_pretty(&json)?
                ))
            }
        }
    }

    fn is_wikipedia_url(url: &Url) -> bool {
        url.host_str()
            .is_some_and(|host| host == "wikipedia.org" || host.ends_with(".wikipedia.org"))
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
            let raw_url = input.url.trim().to_string();
            let url = Self::normalize_url(&input.url).map_err(|e| e.to_string())?;
            let sandboxing = cx.update(|cx| Self::fetch_network_sandboxing_enabled(cx));
            // Without the sandbox, the fetch permission rules are evaluated
            // against the model's raw URL only — exactly as before this feature.
            // Under the sandbox we additionally match the normalized URL so
            // per-host rules also apply to redirect hops and schemeless inputs.
            let permission_inputs = if sandboxing {
                Self::permission_inputs_for_url(&url, [raw_url])
            } else {
                vec![raw_url]
            };

            let event_stream_for_fetch = event_stream.clone();
            let fetch = async move {
                Self::build_message(
                    http_client,
                    event_stream_for_fetch,
                    url,
                    permission_inputs,
                    sandboxing,
                    cx,
                )
                .await
            }
            .fuse();
            futures::pin_mut!(fetch);

            let text = futures::select! {
                result = fetch => result.map_err(|e| e.to_string())?,
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
