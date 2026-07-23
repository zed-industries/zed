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

use std::time::Duration;

use anyhow::Result;
use collections::HashMap;
use futures::{channel::oneshot, future::BoxFuture};
use gpui::AsyncApp;
use serde_json::Value;

use crate::client::{self, Client, NotificationSubscription, RequestTimedOut};
use crate::oauth::WwwAuthenticate;
use crate::types::{self, Notification, Request};

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
            .request::<types::DiscoverResponse>(
                types::requests::ServerDiscover::METHOD,
                serde_json::json!({ "_meta": request_meta }),
            )
            .await;

        match discover_result {
            Ok(response) => {
                self.finish_discovery(client_info, request_meta, response)
                    .await
            }
            Err(error) => {
                if let Some(rpc_error) = error.downcast_ref::<client::Error>() {
                    if rpc_error.code == types::error_codes::UNSUPPORTED_PROTOCOL_VERSION {
                        // A modern server that rejects our modern version. If
                        // it also speaks a legacy version we support, use the
                        // handshake it expects for that version; otherwise
                        // there is no compatible version.
                        let supported = rpc_error
                            .data
                            .clone()
                            .and_then(|data| {
                                serde_json::from_value::<types::UnsupportedProtocolVersionData>(
                                    data,
                                )
                                .ok()
                            })
                            .map(|data| data.supported)
                            .unwrap_or_default();
                        return self.negotiate_from_supported(client_info, &supported).await;
                    }
                    // Any other JSON-RPC error identifies a legacy server
                    // (they commonly answer unknown pre-initialize methods
                    // with -32601 or -32602).
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
        self.inner
            .request(T::METHOD, self.prepare_params(params)?)
            .await
    }

    pub async fn request_with<T: Request>(
        &self,
        params: T::Params,
        cancel_rx: Option<oneshot::Receiver<()>>,
        timeout: Option<Duration>,
    ) -> Result<T::Response> {
        self.inner
            .request_with(T::METHOD, self.prepare_params(params)?, cancel_rx, timeout)
            .await
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
    async fn test_modern_fake_transport_helper(cx: &mut TestAppContext) {
        let (protocol, _) = connect(
            create_modern_fake_transport("modern-server", cx.executor()),
            cx,
        )
        .await;
        assert!(protocol.unwrap().is_modern());
    }
}
