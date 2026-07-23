//! This module implements parts of the Model Context Protocol.
//!
//! It handles connection establishment across protocol eras, and provides a
//! general interface to interacting with an MCP server. It uses the generic
//! JSON-RPC client to read/write messages and the types from types.rs for
//! serialization/deserialization of messages.
//!
//! Two protocol eras exist (see the spec's versioning page):
//!
//! * **Modern** (2026-07-28 and later): no handshake; every request carries
//!   the protocol version, client info, and client capabilities in `_meta`.
//!   Server identity and capabilities come from `server/discover`.
//! * **Legacy** (2025-11-25 and earlier): an `initialize` request followed by
//!   a `notifications/initialized` notification establishes a session.
//!
//! Connecting probes with `server/discover` first and falls back to the
//! legacy handshake, per the spec's backward-compatibility rules.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use collections::HashMap;
use futures::{channel::oneshot, future::BoxFuture};
use gpui::AsyncApp;
use serde_json::Value;

use crate::client::{self, Client, NotificationSubscription, RequestTimedOut};
use crate::oauth::WwwAuthenticate;
use crate::types::{self, Notification, Request};

/// The client requests that may receive interim `input_required` results
/// under the multi round-trip request pattern (MCP 2026-07-28).
const MRTR_METHODS: &[&str] = &[
    types::requests::CallTool::METHOD,
    types::requests::ResourcesRead::METHOD,
    types::requests::PromptsGet::METHOD,
];

/// Bounds retries of a request whose server keeps answering
/// `input_required`; the spec allows servers to re-prompt indefinitely, but
/// without input requests to fulfill there is no point retrying forever.
const MAX_INPUT_REQUIRED_RETRIES: usize = 8;

pub struct ModelContextProtocol {
    inner: Client,
}

impl ModelContextProtocol {
    pub(crate) fn new(inner: Client) -> Self {
        Self { inner }
    }

    fn supported_legacy_protocols() -> Vec<types::ProtocolVersion> {
        vec![
            types::ProtocolVersion(types::VERSION_2025_11_25.to_string()),
            types::ProtocolVersion(types::VERSION_2025_06_18.to_string()),
            types::ProtocolVersion(types::VERSION_2025_03_26.to_string()),
            types::ProtocolVersion(types::VERSION_2024_11_05.to_string()),
        ]
    }

    /// The `_meta` object stamped on every request to a modern server.
    fn modern_request_meta(client_info: &types::Implementation) -> HashMap<String, Value> {
        HashMap::from_iter([
            (
                types::meta_keys::PROTOCOL_VERSION.to_string(),
                Value::String(types::VERSION_2026_07_28.to_string()),
            ),
            (
                types::meta_keys::CLIENT_INFO.to_string(),
                serde_json::to_value(client_info).unwrap_or(Value::Null),
            ),
            (
                types::meta_keys::CLIENT_CAPABILITIES.to_string(),
                serde_json::to_value(types::ClientCapabilities::default())
                    .unwrap_or_else(|_| Value::Object(Default::default())),
            ),
        ])
    }

    pub async fn initialize(
        self,
        client_info: types::Implementation,
    ) -> Result<InitializedContextServerProtocol> {
        let request_meta = Self::modern_request_meta(&client_info);

        let discover_result = self
            .inner
            .request::<Value>(
                types::requests::ServerDiscover::METHOD,
                serde_json::json!({ "_meta": request_meta }),
            )
            .await;

        match discover_result {
            Ok(result) => match serde_json::from_value::<types::DiscoverResponse>(result) {
                Ok(response) => {
                    self.finish_discovery(client_info, request_meta, response)
                        .await
                }
                Err(error) => {
                    // A modern server must return a valid DiscoverResult;
                    // anything else is a legacy server answering an unknown
                    // method leniently.
                    log::debug!(
                        "treating malformed server/discover result as a legacy server: {error}"
                    );
                    self.legacy_initialize(client_info, None).await
                }
            },
            Err(error) => {
                if let Some(rpc_error) = error.downcast_ref::<client::Error>() {
                    // A well-formed UnsupportedProtocolVersionError is the
                    // recognized modern negotiation signal: the server is
                    // modern but rejects our version, and advertises the
                    // versions it supports. If it also speaks a legacy
                    // version we support, use the handshake it expects for
                    // that version; otherwise there is no compatible
                    // version. A -32022 without the data payload could as
                    // well be a legacy server's implementation-defined
                    // error, so it falls through to the legacy fallback.
                    if rpc_error.code == types::error_codes::UNSUPPORTED_PROTOCOL_VERSION
                        && let Some(data) = rpc_error.data.clone().and_then(|data| {
                            serde_json::from_value::<types::UnsupportedProtocolVersionData>(data)
                                .ok()
                        })
                    {
                        return self
                            .negotiate_from_supported(client_info, &data.supported)
                            .await;
                    }
                    // Any other JSON-RPC error identifies a legacy server:
                    // they commonly answer unknown pre-initialize methods
                    // with -32601 or -32602, but the spec forbids keying the
                    // fallback to specific codes, since pre-2026 servers
                    // could use any implementation-defined code. A modern
                    // server answers a valid probe with a DiscoverResult.
                    return self.legacy_initialize(client_info, None).await;
                }
                if error.is::<RequestTimedOut>() {
                    // Some legacy servers do not answer unknown methods at
                    // all; the spec's probe rules treat a timeout as legacy.
                    return self.legacy_initialize(client_info, None).await;
                }
                // Transport-level failures (e.g. an authentication challenge
                // or an unreachable server) are not era signals.
                Err(error)
            }
        }
    }

    async fn finish_discovery(
        self,
        client_info: types::Implementation,
        request_meta: HashMap<String, Value>,
        response: types::DiscoverResponse,
    ) -> Result<InitializedContextServerProtocol> {
        log::trace!("mcp server discover response {:?}", response);

        if response
            .supported_versions
            .iter()
            .any(|version| version.0 == types::VERSION_2026_07_28)
        {
            self.inner.set_protocol_version(types::VERSION_2026_07_28);
            let server_info = response.server_info();
            Ok(InitializedContextServerProtocol {
                inner: self.inner,
                protocol_version: types::ProtocolVersion(types::VERSION_2026_07_28.to_string()),
                capabilities: response.capabilities,
                server_info,
                instructions: response.instructions,
                request_meta: Some(request_meta),
            })
        } else {
            // A server that implements `server/discover` but does not list a
            // modern version: negotiate a mutual legacy version.
            self.negotiate_from_supported(client_info, &response.supported_versions)
                .await
        }
    }

    async fn negotiate_from_supported(
        self,
        client_info: types::Implementation,
        server_supported: &[types::ProtocolVersion],
    ) -> Result<InitializedContextServerProtocol> {
        let mutual_legacy = Self::supported_legacy_protocols()
            .into_iter()
            .find(|version| server_supported.contains(version));
        match mutual_legacy {
            Some(version) => self.legacy_initialize(client_info, Some(version)).await,
            None => anyhow::bail!(
                "No mutually supported protocol version (server supports {:?})",
                server_supported
                    .iter()
                    .map(|version| version.0.as_str())
                    .collect::<Vec<_>>()
            ),
        }
    }

    async fn legacy_initialize(
        self,
        client_info: types::Implementation,
        protocol_version: Option<types::ProtocolVersion>,
    ) -> Result<InitializedContextServerProtocol> {
        let params = types::InitializeParams {
            protocol_version: protocol_version.unwrap_or(types::ProtocolVersion(
                types::LATEST_LEGACY_PROTOCOL_VERSION.to_string(),
            )),
            capabilities: types::ClientCapabilities::default(),
            meta: None,
            client_info,
        };

        let response: types::InitializeResponse = self
            .inner
            .request(types::requests::Initialize::METHOD, params)
            .await?;

        anyhow::ensure!(
            Self::supported_legacy_protocols().contains(&response.protocol_version),
            "Unsupported protocol version: {:?}",
            response.protocol_version
        );

        log::trace!("mcp server info {:?}", response.server_info);

        // Per MCP 2025-06-18, HTTP transport must attach the negotiated version
        // as `MCP-Protocol-Version` on every post-initialize request.
        self.inner
            .set_protocol_version(&response.protocol_version.0);

        let initialized_protocol = InitializedContextServerProtocol {
            inner: self.inner,
            protocol_version: response.protocol_version,
            capabilities: response.capabilities,
            server_info: Some(response.server_info),
            instructions: None,
            request_meta: None,
        };

        initialized_protocol.notify::<types::notifications::Initialized>(())?;

        Ok(initialized_protocol)
    }
}

pub struct InitializedContextServerProtocol {
    inner: Client,
    pub protocol_version: types::ProtocolVersion,
    pub capabilities: types::ServerCapabilities,
    pub server_info: Option<types::Implementation>,
    pub instructions: Option<String>,
    /// `Some` when speaking a modern (2026-07-28 or later) revision: the
    /// `_meta` entries stamped on every outgoing request.
    request_meta: Option<HashMap<String, Value>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServerCapability {
    Experimental,
    Logging,
    Prompts,
    Resources,
    Tools,
}

impl InitializedContextServerProtocol {
    /// Check if the server supports a specific capability
    pub fn capable(&self, capability: ServerCapability) -> bool {
        match capability {
            ServerCapability::Experimental => self.capabilities.experimental.is_some(),
            ServerCapability::Logging => self.capabilities.logging.is_some(),
            ServerCapability::Prompts => self.capabilities.prompts.is_some(),
            ServerCapability::Resources => self.capabilities.resources.is_some(),
            ServerCapability::Tools => self.capabilities.tools.is_some(),
        }
    }

    /// Whether the connection speaks a modern (2026-07-28 or later) protocol
    /// revision.
    pub fn is_modern(&self) -> bool {
        self.request_meta.is_some()
    }

    /// Serialize the params, merging in the modern per-request `_meta`
    /// entries when this connection speaks a modern protocol revision.
    /// Entries already present in the params' own `_meta` win.
    fn prepare_params(&self, params: impl serde::Serialize) -> Result<Value> {
        let mut value = serde_json::to_value(params)?;
        let Some(request_meta) = &self.request_meta else {
            return Ok(value);
        };
        if value.is_null() {
            value = Value::Object(Default::default());
        }
        if let Value::Object(object) = &mut value {
            let meta = object
                .entry("_meta")
                .or_insert_with(|| Value::Object(Default::default()));
            if let Value::Object(meta) = meta {
                for (key, entry) in request_meta {
                    meta.entry(key.clone()).or_insert_with(|| entry.clone());
                }
            }
        }
        Ok(value)
    }

    pub async fn request<T: Request>(&self, params: T::Params) -> Result<T::Response> {
        self.request_with::<T>(params, None, self.inner.default_request_timeout())
            .await
    }

    pub async fn request_with<T: Request>(
        &self,
        params: T::Params,
        cancel_rx: Option<oneshot::Receiver<()>>,
        timeout: Option<Duration>,
    ) -> Result<T::Response> {
        let mut params = self.prepare_params(params)?;
        let mut cancel_rx = cancel_rx;
        let mut attempts_left = MAX_INPUT_REQUIRED_RETRIES;

        loop {
            let result: Value = self
                .inner
                .request_with(T::METHOD, &params, cancel_rx.as_mut(), timeout)
                .await?;

            // Multi round-trip requests (MCP 2026-07-28): a modern server
            // may answer with an interim `input_required` result instead of
            // a final one. Legacy results have no `resultType` and are final
            // by definition.
            let input_required = self.is_modern()
                && MRTR_METHODS.contains(&T::METHOD)
                && result.get("resultType").and_then(|value| value.as_str())
                    == Some("input_required");
            if !input_required {
                return Ok(serde_json::from_value(result)?);
            }

            let interim: types::InputRequiredResult = serde_json::from_value(result)?;
            if let Some(input_requests) = &interim.input_requests
                && !input_requests.is_empty()
            {
                // Servers must not request input capabilities the client did
                // not declare, and Zed declares none.
                let methods = input_requests
                    .values()
                    .map(|request| request.method.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                anyhow::bail!(
                    "Server requested additional input ({methods}) that Zed does not support"
                );
            }

            anyhow::ensure!(
                attempts_left > 0,
                "Server kept responding with input_required results"
            );
            attempts_left -= 1;

            // Retry the original request (with a fresh request id), echoing
            // the opaque request state, if any.
            if let Value::Object(object) = &mut params {
                match interim.request_state {
                    Some(state) => {
                        object.insert("requestState".to_string(), Value::String(state));
                    }
                    None => {
                        object.remove("requestState");
                    }
                }
            }
        }
    }

    /// Drive a long-lived `subscriptions/listen` stream for the given
    /// notification types (MCP 2026-07-28 and later). The subscribed
    /// notifications are dispatched to handlers registered via
    /// [`Self::on_notification`]; the returned future resolves when the
    /// server gracefully closes the subscription, and dropping it abandons
    /// the subscription. On legacy connections it resolves immediately:
    /// legacy servers push these notifications unsolicited.
    pub fn listen_to_notifications(
        self: &Arc<Self>,
        filter: types::NotificationFilter,
    ) -> impl Future<Output = Result<()>> + 'static {
        let this = self.clone();
        async move {
            if !this.is_modern() {
                return Ok(());
            }
            // The server acknowledges with the subset of the filter it will
            // honor; surface disagreements in logs rather than failing.
            let _ack_subscription = this.on_notification(
                types::notifications::SubscriptionsAcknowledged::METHOD,
                Box::new(|params, _cx| {
                    match serde_json::from_value::<types::SubscriptionsAcknowledgedParams>(params) {
                        Ok(ack) => {
                            log::debug!("mcp subscription acknowledged: {:?}", ack.notifications)
                        }
                        Err(error) => {
                            log::error!("invalid subscriptions/listen acknowledgment: {error}")
                        }
                    }
                }),
            );
            this.request_with::<types::requests::SubscriptionsListen>(
                types::SubscriptionsListenParams {
                    notifications: filter,
                    meta: None,
                },
                None,
                None,
            )
            .await?;
            Ok(())
        }
    }

    pub fn notify<T: Notification>(&self, params: T::Params) -> Result<()> {
        self.inner.notify(T::METHOD, params)
    }

    /// A future that resolves once the underlying transport's output loop has
    /// terminated — after a send failure, or when the client is dropped —
    /// yielding the authentication challenge recorded by the transport if it
    /// shut down on a `401 Unauthorized` response.
    ///
    /// Servers may accept the first request unauthenticated and only
    /// challenge a later request or notification. Awaiting this is what lets
    /// the owner of the connection notice such a challenge even when no
    /// request was in flight to carry a typed error back. Returns `None` if
    /// the shutdown signal was already claimed: there is a single signal per
    /// client.
    pub fn wait_for_shutdown(&self) -> Option<BoxFuture<'static, Option<WwwAuthenticate>>> {
        self.inner.wait_for_shutdown()
    }

    pub fn on_notification(
        &self,
        method: &'static str,
        f: Box<dyn 'static + Send + FnMut(Value, AsyncApp)>,
    ) -> NotificationSubscription {
        self.inner.on_notification(method, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ContextServerId;
    use crate::test::{
        FakeTransport, create_fake_transport, create_modern_fake_transport,
        create_modern_fake_transport_with_capabilities,
    };
    use crate::types::requests;
    use gpui::TestAppContext;
    use parking_lot::Mutex;
    use std::sync::Arc;

    fn client_info() -> types::Implementation {
        types::Implementation {
            name: "Zed".to_string(),
            title: None,
            version: "1.0.0".to_string(),
            description: None,
        }
    }

    async fn connect(
        transport: FakeTransport,
        cx: &mut TestAppContext,
    ) -> (
        Result<InitializedContextServerProtocol>,
        Arc<Mutex<Vec<Value>>>,
    ) {
        let received_messages = transport.received_messages();
        let client = Client::new(
            ContextServerId("test-server".into()),
            "test-server".into(),
            Arc::new(transport),
            None,
            cx.to_async(),
        )
        .unwrap();
        let protocol = ModelContextProtocol::new(client)
            .initialize(client_info())
            .await;
        (protocol, received_messages)
    }

    fn messages_with_method(messages: &[Value], method: &str) -> Vec<Value> {
        messages
            .iter()
            .filter(|message| {
                message.get("method").and_then(|m| m.as_str()) == Some(method)
            })
            .cloned()
            .collect()
    }

    #[gpui::test]
    async fn test_modern_server_negotiation(cx: &mut TestAppContext) {
        let transport = create_modern_fake_transport_with_capabilities(
            "modern-server",
            types::ServerCapabilities {
                tools: Some(types::ToolsCapabilities { list_changed: None }),
                ..Default::default()
            },
            cx.executor(),
        )
        .on_request::<requests::ListTools, _>(|_params| async {
            types::ListToolsResponse {
                tools: Vec::new(),
                next_cursor: None,
                ttl_ms: Some(60_000),
                cache_scope: Some(types::CacheScope::Private),
                meta: None,
            }
        });

        let (protocol, received_messages) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        assert!(protocol.is_modern());
        assert!(protocol.capable(ServerCapability::Tools));
        assert!(!protocol.capable(ServerCapability::Prompts));
        assert_eq!(
            protocol.protocol_version,
            types::ProtocolVersion(types::VERSION_2026_07_28.to_string())
        );
        assert_eq!(
            protocol.server_info.as_ref().map(|info| info.name.as_str()),
            Some("modern-server")
        );

        protocol
            .request::<requests::ListTools>(())
            .await
            .unwrap();

        let messages = received_messages.lock().clone();
        // No legacy handshake took place.
        assert!(messages_with_method(&messages, "initialize").is_empty());
        assert!(messages_with_method(&messages, "notifications/initialized").is_empty());

        // Every request carries the modern `_meta`, including the probe.
        for method in ["server/discover", "tools/list"] {
            let requests = messages_with_method(&messages, method);
            assert_eq!(requests.len(), 1, "expected exactly one {method} request");
            let meta = &requests[0]["params"]["_meta"];
            assert_eq!(
                meta[types::meta_keys::PROTOCOL_VERSION],
                types::VERSION_2026_07_28
            );
            assert_eq!(meta[types::meta_keys::CLIENT_INFO]["name"], "Zed");
            assert!(meta[types::meta_keys::CLIENT_CAPABILITIES].is_object());
        }
    }

    #[gpui::test]
    async fn test_legacy_server_fallback(cx: &mut TestAppContext) {
        let transport = create_fake_transport("legacy-server", cx.executor())
            .on_request::<requests::ListTools, _>(|_params| async {
                types::ListToolsResponse {
                    tools: Vec::new(),
                    next_cursor: None,
                    ttl_ms: None,
                    cache_scope: None,
                    meta: None,
                }
            });

        let (protocol, received_messages) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        assert!(!protocol.is_modern());
        assert_eq!(
            protocol.protocol_version,
            types::ProtocolVersion(types::LATEST_LEGACY_PROTOCOL_VERSION.to_string())
        );
        assert_eq!(
            protocol.server_info.as_ref().map(|info| info.name.as_str()),
            Some("legacy-server")
        );

        protocol
            .request::<requests::ListTools>(())
            .await
            .unwrap();

        let messages = received_messages.lock().clone();
        // The probe was rejected, and the legacy handshake ran.
        assert_eq!(messages_with_method(&messages, "server/discover").len(), 1);
        let initialize = messages_with_method(&messages, "initialize");
        assert_eq!(initialize.len(), 1);
        assert_eq!(
            initialize[0]["params"]["protocolVersion"],
            types::LATEST_LEGACY_PROTOCOL_VERSION
        );
        assert_eq!(
            messages_with_method(&messages, "notifications/initialized").len(),
            1
        );

        // Legacy requests are not stamped with modern `_meta`.
        let list_tools = messages_with_method(&messages, "tools/list");
        assert_eq!(list_tools.len(), 1);
        assert!(list_tools[0].get("params").is_none_or(|params| {
            params.get("_meta").is_none()
        }));
    }

    #[gpui::test]
    async fn test_modern_error_negotiates_legacy_version(cx: &mut TestAppContext) {
        // A dual-era server that rejects 2026-07-28 with a modern error
        // advertising a legacy version.
        let transport = FakeTransport::new(cx.executor())
            .on_raw_request(requests::ServerDiscover::METHOD, |_params| async {
                Err(client::Error {
                    code: types::error_codes::UNSUPPORTED_PROTOCOL_VERSION,
                    message: "Unsupported protocol version".to_string(),
                    data: Some(serde_json::json!({
                        "supported": ["2025-11-25"],
                        "requested": "2026-07-28"
                    })),
                })
            })
            .on_request::<requests::Initialize, _>(|params| async move {
                types::InitializeResponse {
                    protocol_version: params.protocol_version,
                    server_info: types::Implementation {
                        name: "dual-era-server".to_string(),
                        title: None,
                        version: "1.0.0".to_string(),
                        description: None,
                    },
                    capabilities: types::ServerCapabilities::default(),
                    meta: None,
                }
            });

        let (protocol, received_messages) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        assert!(!protocol.is_modern());
        assert_eq!(
            protocol.protocol_version,
            types::ProtocolVersion(types::VERSION_2025_11_25.to_string())
        );

        let messages = received_messages.lock().clone();
        let initialize = messages_with_method(&messages, "initialize");
        assert_eq!(initialize.len(), 1);
        assert_eq!(
            initialize[0]["params"]["protocolVersion"],
            types::VERSION_2025_11_25
        );
    }

    #[gpui::test]
    async fn test_discover_advertising_only_legacy_versions_falls_back(cx: &mut TestAppContext) {
        // A server that implements server/discover but lists only legacy
        // versions gets the handshake those versions require.
        let transport = FakeTransport::new(cx.executor())
            .on_raw_request(requests::ServerDiscover::METHOD, |_params| async {
                Ok(Some(serde_json::json!({
                    "resultType": "complete",
                    "supportedVersions": ["2025-11-25", "2025-06-18"],
                    "capabilities": {}
                })))
            })
            .on_request::<requests::Initialize, _>(|params| async move {
                types::InitializeResponse {
                    protocol_version: params.protocol_version,
                    server_info: types::Implementation {
                        name: "discoverable-legacy-server".to_string(),
                        title: None,
                        version: "1.0.0".to_string(),
                        description: None,
                    },
                    capabilities: types::ServerCapabilities::default(),
                    meta: None,
                }
            });

        let (protocol, _) = connect(transport, cx).await;
        let protocol = protocol.unwrap();
        assert!(!protocol.is_modern());
        assert_eq!(
            protocol.protocol_version,
            types::ProtocolVersion(types::VERSION_2025_11_25.to_string())
        );
    }

    #[gpui::test]
    async fn test_malformed_discover_result_falls_back_to_legacy(cx: &mut TestAppContext) {
        // Some permissive legacy servers acknowledge unknown methods with an
        // empty success result.
        let transport = FakeTransport::new(cx.executor())
            .on_raw_request(requests::ServerDiscover::METHOD, |_params| async {
                Ok(Some(serde_json::json!({})))
            })
            .on_request::<requests::Initialize, _>(|_params| async {
                types::InitializeResponse {
                    protocol_version: types::ProtocolVersion(
                        types::LATEST_LEGACY_PROTOCOL_VERSION.to_string(),
                    ),
                    server_info: types::Implementation {
                        name: "permissive-legacy-server".to_string(),
                        title: None,
                        version: "1.0.0".to_string(),
                        description: None,
                    },
                    capabilities: types::ServerCapabilities::default(),
                    meta: None,
                }
            });

        let (protocol, _) = connect(transport, cx).await;
        assert!(!protocol.unwrap().is_modern());
    }

    #[gpui::test]
    async fn test_bare_unsupported_version_error_falls_back_to_legacy(cx: &mut TestAppContext) {
        // A -32022 without data.supported could be a legacy server's
        // implementation-defined error, not the modern negotiation signal.
        let transport = FakeTransport::new(cx.executor())
            .on_raw_request(requests::ServerDiscover::METHOD, |_params| async {
                Err(client::Error {
                    code: types::error_codes::UNSUPPORTED_PROTOCOL_VERSION,
                    message: "nope".to_string(),
                    data: None,
                })
            })
            .on_request::<requests::Initialize, _>(|_params| async {
                types::InitializeResponse {
                    protocol_version: types::ProtocolVersion(
                        types::LATEST_LEGACY_PROTOCOL_VERSION.to_string(),
                    ),
                    server_info: types::Implementation {
                        name: "legacy-server".to_string(),
                        title: None,
                        version: "1.0.0".to_string(),
                        description: None,
                    },
                    capabilities: types::ServerCapabilities::default(),
                    meta: None,
                }
            });

        let (protocol, _) = connect(transport, cx).await;
        assert!(!protocol.unwrap().is_modern());
    }

    #[gpui::test]
    async fn test_no_mutual_version_fails(cx: &mut TestAppContext) {
        let transport = FakeTransport::new(cx.executor()).on_raw_request(
            requests::ServerDiscover::METHOD,
            |_params| async {
                Err(client::Error {
                    code: types::error_codes::UNSUPPORTED_PROTOCOL_VERSION,
                    message: "Unsupported protocol version".to_string(),
                    data: Some(serde_json::json!({ "supported": ["2100-01-01"] })),
                })
            },
        );

        let (protocol, _) = connect(transport, cx).await;
        let error = protocol.err().unwrap();
        assert!(
            error.to_string().contains("No mutually supported"),
            "unexpected error: {error}"
        );
    }

    #[gpui::test]
    async fn test_probe_timeout_falls_back_to_legacy(cx: &mut TestAppContext) {
        // A legacy server that never answers unknown methods at all.
        let name = "silent-legacy-server";
        let transport = FakeTransport::new(cx.executor())
            .on_raw_request(requests::ServerDiscover::METHOD, |_params| async {
                Ok(None)
            })
            .on_request::<requests::Initialize, _>(move |_params| async move {
                types::InitializeResponse {
                    protocol_version: types::ProtocolVersion(
                        types::LATEST_LEGACY_PROTOCOL_VERSION.to_string(),
                    ),
                    server_info: types::Implementation {
                        name: name.to_string(),
                        title: None,
                        version: "1.0.0".to_string(),
                        description: None,
                    },
                    capabilities: types::ServerCapabilities::default(),
                    meta: None,
                }
            });

        let client = Client::new(
            ContextServerId("test-server".into()),
            "test-server".into(),
            Arc::new(transport),
            None,
            cx.to_async(),
        )
        .unwrap();

        let task = cx.executor().spawn(async move {
            ModelContextProtocol::new(client)
                .initialize(client_info())
                .await
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(61));

        let protocol = task.await.unwrap();
        assert!(!protocol.is_modern());
        assert_eq!(
            protocol.server_info.as_ref().map(|info| info.name.as_str()),
            Some("silent-legacy-server")
        );
    }

    #[gpui::test]
    async fn test_input_required_result_retries_with_request_state(cx: &mut TestAppContext) {
        let call_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let transport = create_modern_fake_transport("modern-server", cx.executor())
            .on_raw_request(requests::CallTool::METHOD, {
                let call_count = call_count.clone();
                move |params| {
                    let call_count = call_count.clone();
                    async move {
                        if call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 0 {
                            assert!(params.get("requestState").is_none());
                            Ok(Some(serde_json::json!({
                                "resultType": "input_required",
                                "requestState": "opaque-state"
                            })))
                        } else {
                            assert_eq!(params["requestState"], "opaque-state");
                            assert_eq!(params["name"], "my-tool");
                            // The retry carries the modern `_meta` like the
                            // original request.
                            assert_eq!(
                                params["_meta"][types::meta_keys::PROTOCOL_VERSION],
                                types::VERSION_2026_07_28
                            );
                            Ok(Some(serde_json::json!({
                                "resultType": "complete",
                                "content": [{ "type": "text", "text": "done" }]
                            })))
                        }
                    }
                }
            });

        let (protocol, received_messages) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        let response = protocol
            .request::<requests::CallTool>(types::CallToolParams {
                name: "my-tool".to_string(),
                arguments: None,
                meta: None,
            })
            .await
            .unwrap();
        assert_eq!(response.text_contents(), "done");
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 2);

        // The retry is an independent request with a fresh id.
        let messages = received_messages.lock().clone();
        let calls = messages_with_method(&messages, "tools/call");
        assert_eq!(calls.len(), 2);
        assert_ne!(calls[0]["id"], calls[1]["id"]);
    }

    #[gpui::test]
    async fn test_input_required_result_with_input_requests_fails(cx: &mut TestAppContext) {
        let transport = create_modern_fake_transport("modern-server", cx.executor())
            .on_raw_request(requests::CallTool::METHOD, |_params| async {
                Ok(Some(serde_json::json!({
                    "resultType": "input_required",
                    "inputRequests": {
                        "login": {
                            "method": "elicitation/create",
                            "params": { "message": "Log in please" }
                        }
                    },
                    "requestState": "state"
                })))
            });

        let (protocol, _) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        let error = protocol
            .request::<requests::CallTool>(types::CallToolParams {
                name: "my-tool".to_string(),
                arguments: None,
                meta: None,
            })
            .await
            .unwrap_err();
        assert!(
            error.to_string().contains("elicitation/create"),
            "unexpected error: {error}"
        );
    }

    #[gpui::test]
    async fn test_input_required_retries_are_bounded(cx: &mut TestAppContext) {
        let transport = create_modern_fake_transport("modern-server", cx.executor())
            .on_raw_request(requests::CallTool::METHOD, |_params| async {
                Ok(Some(serde_json::json!({
                    "resultType": "input_required",
                    "requestState": "state"
                })))
            });

        let (protocol, _) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        let error = protocol
            .request::<requests::CallTool>(types::CallToolParams {
                name: "my-tool".to_string(),
                arguments: None,
                meta: None,
            })
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("kept responding with input_required"),
            "unexpected error: {error}"
        );
    }

    #[gpui::test]
    async fn test_legacy_results_without_result_type_are_final(cx: &mut TestAppContext) {
        let transport = create_fake_transport("legacy-server", cx.executor())
            .on_request::<requests::CallTool, _>(|_params| async {
                types::CallToolResponse {
                    content: vec![types::ToolResponseContent::Text {
                        text: "done".to_string(),
                    }],
                    is_error: None,
                    meta: None,
                    structured_content: None,
                }
            });

        let (protocol, _) = connect(transport, cx).await;
        let protocol = protocol.unwrap();

        let response = protocol
            .request::<requests::CallTool>(types::CallToolParams {
                name: "my-tool".to_string(),
                arguments: None,
                meta: None,
            })
            .await
            .unwrap();
        assert_eq!(response.text_contents(), "done");
    }

    #[gpui::test]
    async fn test_listen_to_notifications_on_modern_server(cx: &mut TestAppContext) {
        let transport = Arc::new(
            create_modern_fake_transport("modern-server", cx.executor()).on_raw_request(
                requests::SubscriptionsListen::METHOD,
                |params| async move {
                    assert_eq!(params["notifications"]["toolsListChanged"], true);
                    // The stream stays open: no response.
                    Ok(None)
                },
            ),
        );
        let received_messages = transport.received_messages();

        let client = Client::new(
            ContextServerId("test-server".into()),
            "test-server".into(),
            transport.clone(),
            None,
            cx.to_async(),
        )
        .unwrap();
        let protocol = Arc::new(
            ModelContextProtocol::new(client)
                .initialize(client_info())
                .await
                .unwrap(),
        );

        let notified = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let _subscription = protocol.on_notification(
            "notifications/tools/list_changed",
            Box::new({
                let notified = notified.clone();
                move |_params, _cx| {
                    notified.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }),
        );

        let _listen_task = cx
            .foreground_executor()
            .spawn(protocol.listen_to_notifications(types::NotificationFilter {
                tools_list_changed: Some(true),
                ..Default::default()
            }));
        cx.run_until_parked();

        let messages = received_messages.lock().clone();
        let listen_requests = messages_with_method(&messages, "subscriptions/listen");
        assert_eq!(listen_requests.len(), 1);
        // The listen request carries the modern `_meta` like any other.
        assert_eq!(
            listen_requests[0]["params"]["_meta"][types::meta_keys::PROTOCOL_VERSION],
            types::VERSION_2026_07_28
        );

        let subscription_id = listen_requests[0]["id"].clone();
        transport.send_notification(
            "notifications/subscriptions/acknowledged",
            serde_json::json!({
                "_meta": { types::meta_keys::SUBSCRIPTION_ID: subscription_id },
                "notifications": { "toolsListChanged": true }
            }),
        );
        transport.send_notification(
            "notifications/tools/list_changed",
            serde_json::json!({
                "_meta": { types::meta_keys::SUBSCRIPTION_ID: subscription_id }
            }),
        );
        cx.run_until_parked();

        assert_eq!(notified.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_listen_to_notifications_is_noop_on_legacy_server(cx: &mut TestAppContext) {
        let transport = create_fake_transport("legacy-server", cx.executor());
        let received_messages = transport.received_messages();
        let (protocol, _) = connect(transport, cx).await;
        let protocol = Arc::new(protocol.unwrap());

        protocol
            .listen_to_notifications(types::NotificationFilter {
                tools_list_changed: Some(true),
                ..Default::default()
            })
            .await
            .unwrap();

        let messages = received_messages.lock().clone();
        assert!(messages_with_method(&messages, "subscriptions/listen").is_empty());
    }

    #[gpui::test]
    async fn test_dropping_a_request_cancels_it(cx: &mut TestAppContext) {
        let transport = Arc::new(create_modern_fake_transport("modern-server", cx.executor())
            .on_raw_request(requests::CallTool::METHOD, |_params| async {
                // Never answers.
                Ok(None)
            }));
        let received_messages = transport.received_messages();

        let client = Client::new(
            ContextServerId("test-server".into()),
            "test-server".into(),
            transport.clone(),
            None,
            cx.to_async(),
        )
        .unwrap();
        let protocol = Arc::new(
            ModelContextProtocol::new(client)
                .initialize(client_info())
                .await
                .unwrap(),
        );

        let call_task = cx.foreground_executor().spawn({
            let protocol = protocol.clone();
            async move {
                protocol
                    .request::<requests::CallTool>(types::CallToolParams {
                        name: "my-tool".to_string(),
                        arguments: None,
                        meta: None,
                    })
                    .await
            }
        });
        cx.run_until_parked();

        let call_id = {
            let messages = received_messages.lock();
            let calls = messages_with_method(&messages, "tools/call");
            assert_eq!(calls.len(), 1);
            calls[0]["id"].clone()
        };

        // Abandoning the request (e.g. a tool call raced against user
        // cancellation) must cancel it server-side.
        drop(call_task);
        cx.run_until_parked();

        let messages = received_messages.lock().clone();
        let cancellations = messages_with_method(&messages, "notifications/cancelled");
        assert_eq!(cancellations.len(), 1);
        assert_eq!(cancellations[0]["params"]["requestId"], call_id);

        // The connection itself must survive the abandoned request: the
        // outer handle keeps the client alive while the cancellation flows.
        assert!(protocol.is_modern());
    }

    #[gpui::test]
    async fn test_modern_fake_transport_helper(cx: &mut TestAppContext) {
        let (protocol, _) = connect(
            create_modern_fake_transport("modern-server", cx.executor()),
            cx,
        )
        .await;
        assert!(protocol.unwrap().is_modern());
    }
}
