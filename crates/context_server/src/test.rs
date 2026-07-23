use anyhow::Context as _;
use collections::HashMap;
use futures::{FutureExt, Stream, StreamExt as _, future::BoxFuture, lock::Mutex as AsyncMutex};
use gpui::BackgroundExecutor;
use parking_lot::Mutex;
use std::{pin::Pin, sync::Arc};

use crate::{
    client,
    transport::Transport,
    types::{
        DiscoverResponse, Implementation, InitializeResponse, ProtocolVersion, ServerCapabilities,
        meta_keys,
    },
};

/// A fake for a legacy (pre-2026-07-28) server: it answers the `initialize`
/// handshake and rejects unknown methods — including the `server/discover`
/// probe — with a method-not-found error, like real legacy servers do.
pub fn create_fake_transport(
    name: impl Into<String>,
    executor: BackgroundExecutor,
) -> FakeTransport {
    let name = name.into();
    FakeTransport::new(executor).on_request::<crate::types::requests::Initialize, _>(
        move |_params| {
            let name = name.clone();
            async move { create_initialize_response(name.clone()) }
        },
    )
}

/// A fake for a modern (2026-07-28) server: it answers `server/discover` and
/// never sees an `initialize` handshake.
pub fn create_modern_fake_transport(
    name: impl Into<String>,
    executor: BackgroundExecutor,
) -> FakeTransport {
    create_modern_fake_transport_with_capabilities(name, ServerCapabilities::default(), executor)
}

pub fn create_modern_fake_transport_with_capabilities(
    name: impl Into<String>,
    capabilities: ServerCapabilities,
    executor: BackgroundExecutor,
) -> FakeTransport {
    let name = name.into();
    FakeTransport::new(executor).on_request::<crate::types::requests::ServerDiscover, _>(
        move |_params| {
            let name = name.clone();
            let capabilities = capabilities.clone();
            async move { create_discover_response(name, capabilities) }
        },
    )
}

fn create_initialize_response(server_name: String) -> InitializeResponse {
    InitializeResponse {
        protocol_version: ProtocolVersion(
            crate::types::LATEST_LEGACY_PROTOCOL_VERSION.to_string(),
        ),
        server_info: Implementation {
            name: server_name,
            title: None,
            version: "1.0.0".to_string(),
            description: None,
        },
        capabilities: ServerCapabilities::default(),
        meta: None,
    }
}

pub fn create_discover_response(
    server_name: String,
    capabilities: ServerCapabilities,
) -> DiscoverResponse {
    DiscoverResponse {
        supported_versions: vec![ProtocolVersion(
            crate::types::VERSION_2026_07_28.to_string(),
        )],
        capabilities,
        instructions: None,
        ttl_ms: None,
        cache_scope: None,
        meta: Some(HashMap::from_iter([(
            meta_keys::SERVER_INFO.to_string(),
            serde_json::json!({ "name": server_name, "version": "1.0.0" }),
        )])),
    }
}

type RequestHandler =
    Arc<dyn Send + Sync + Fn(serde_json::Value) -> BoxFuture<'static, HandlerResult>>;
/// `Ok(None)` means the fake server never answers the request, like a legacy
/// server that ignores unknown methods.
type HandlerResult = Result<Option<serde_json::Value>, client::Error>;

pub struct FakeTransport {
    request_handlers: HashMap<&'static str, RequestHandler>,
    tx: futures::channel::mpsc::UnboundedSender<String>,
    rx: Arc<AsyncMutex<futures::channel::mpsc::UnboundedReceiver<String>>>,
    received_messages: Arc<Mutex<Vec<serde_json::Value>>>,
    executor: BackgroundExecutor,
}

impl FakeTransport {
    pub fn new(executor: BackgroundExecutor) -> Self {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        Self {
            request_handlers: Default::default(),
            tx,
            rx: Arc::new(AsyncMutex::new(rx)),
            received_messages: Default::default(),
            executor,
        }
    }

    pub fn on_request<T, Fut>(
        self,
        handler: impl 'static + Send + Sync + Fn(T::Params) -> Fut,
    ) -> Self
    where
        T: crate::types::Request,
        Fut: 'static + Send + Future<Output = T::Response>,
    {
        self.on_raw_request(T::METHOD, move |params| {
            // Modern requests stamp `_meta` into params that are otherwise
            // absent or don't declare the field; strip it like a real server
            // that ignores unrecognized metadata.
            let params = match params {
                serde_json::Value::Object(mut object) => {
                    object.remove("_meta");
                    if object.is_empty() {
                        serde_json::Value::Null
                    } else {
                        serde_json::Value::Object(object)
                    }
                }
                params => params,
            };
            let params: T::Params =
                serde_json::from_value(params).expect("Invalid parameters received");
            let response = handler(params);
            async move { Ok(Some(serde_json::to_value(response.await).unwrap())) }.boxed()
        })
    }

    /// Register a handler on the raw params JSON, able to return a JSON-RPC
    /// error.
    pub fn on_raw_request<Fut>(
        mut self,
        method: &'static str,
        handler: impl 'static + Send + Sync + Fn(serde_json::Value) -> Fut,
    ) -> Self
    where
        Fut: 'static + Send + Future<Output = HandlerResult>,
    {
        self.request_handlers
            .insert(method, Arc::new(move |params| handler(params).boxed()));
        self
    }

    /// Every JSON-RPC message the client has sent over this transport.
    pub fn received_messages(&self) -> Arc<Mutex<Vec<serde_json::Value>>> {
        self.received_messages.clone()
    }
}

#[async_trait::async_trait]
impl Transport for FakeTransport {
    async fn send(&self, message: String) -> anyhow::Result<()> {
        let Ok(msg) = serde_json::from_str::<serde_json::Value>(&message) else {
            return Ok(());
        };
        self.received_messages.lock().push(msg.clone());

        let Some(method) = msg.get("method").and_then(|method| method.as_str()) else {
            return Ok(());
        };
        let id = msg.get("id").cloned();
        let params = msg
            .get("params")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        if let Some(handler) = self.request_handlers.get(method) {
            let response = match handler(params).await {
                Ok(Some(payload)) => serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id.unwrap_or(serde_json::Value::from(0)),
                    "result": payload
                }),
                Ok(None) => return Ok(()),
                Err(error) => serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id.unwrap_or(serde_json::Value::from(0)),
                    "error": error
                }),
            };
            self.tx
                .unbounded_send(response.to_string())
                .context("sending a message")?;
        } else if let Some(id) = id {
            // Real servers reject unknown request methods; notifications
            // (no id) are dropped silently.
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {method}")
                }
            });
            self.tx
                .unbounded_send(response.to_string())
                .context("sending a message")?;
        } else {
            log::debug!("No handler registered for MCP notification '{method}'");
        }
        Ok(())
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        let rx = self.rx.clone();
        let executor = self.executor.clone();
        Box::pin(futures::stream::unfold(rx, move |rx| {
            let executor = executor.clone();
            async move {
                let mut rx_guard = rx.lock().await;
                executor.simulate_random_delay().await;
                if let Some(message) = rx_guard.next().await {
                    drop(rx_guard);
                    Some((message, rx))
                } else {
                    None
                }
            }
        }))
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(futures::stream::empty())
    }
}
