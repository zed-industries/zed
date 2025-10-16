use anyhow::Context as _;
use collections::HashMap;
use futures::{FutureExt, Stream, StreamExt as _, future::BoxFuture, lock::Mutex};
use gpui::BackgroundExecutor;
use std::{pin::Pin, sync::Arc};

use crate::{
    transport::Transport,
    types::{Implementation, InitializeResponse, ProtocolVersion, ServerCapabilities},
};

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

fn create_initialize_response(server_name: String) -> InitializeResponse {
    InitializeResponse {
        protocol_version: ProtocolVersion(crate::types::LATEST_PROTOCOL_VERSION.to_string()),
        server_info: Implementation {
            name: server_name,
            version: "1.0.0".to_string(),
        },
        capabilities: ServerCapabilities::default(),
        meta: None,
    }
}

pub struct FakeTransport {
    request_handlers: HashMap<
        &'static str,
        Arc<dyn Send + Sync + Fn(serde_json::Value) -> BoxFuture<'static, serde_json::Value>>,
    >,
    tx: futures::channel::mpsc::UnboundedSender<String>,
    rx: Arc<Mutex<futures::channel::mpsc::UnboundedReceiver<String>>>,
    executor: BackgroundExecutor,
}

impl FakeTransport {
    pub fn new(executor: BackgroundExecutor) -> Self {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        Self {
            request_handlers: Default::default(),
            tx,
            rx: Arc::new(Mutex::new(rx)),
            executor,
        }
    }

    pub fn on_request<T, Fut>(
        mut self,
        handler: impl 'static + Send + Sync + Fn(T::Params) -> Fut,
    ) -> Self
    where
        T: crate::types::Request,
        Fut: 'static + Send + Future<Output = T::Response>,
    {
        self.request_handlers.insert(
            T::METHOD,
            Arc::new(move |value| {
                let params = value
                    .get("params")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                let params: T::Params =
                    serde_json::from_value(params).expect("Invalid parameters received");
                let response = handler(params);
                async move { serde_json::to_value(response.await).unwrap() }.boxed()
            }),
        );
        self
    }
}

#[async_trait::async_trait]
impl Transport for FakeTransport {
    async fn send(&self, message: String) -> anyhow::Result<()> {
        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&message) {
            let id = msg.get("id").and_then(|id| id.as_u64()).unwrap_or(0);

            if let Some(method) = msg.get("method") {
                let method = method.as_str().expect("Invalid method received");
                if let Some(handler) = self.request_handlers.get(method) {
                    let payload = handler(msg).await;
                    let response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": payload
                    });
                    self.tx
                        .unbounded_send(response.to_string())
                        .context("sending a message")?;
                } else {
                    log::debug!("No handler registered for MCP request '{method}'");
                }
            }
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
