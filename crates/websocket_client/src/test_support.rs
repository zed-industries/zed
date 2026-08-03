//! Test WebSocket implementations with scripted connection results and messages.

use std::collections::VecDeque;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use futures::FutureExt as _;
use futures::future::{self, BoxFuture};
use http_client::http::HeaderMap;
use parking_lot::Mutex;

use crate::{WebSocketClient, WebSocketConnection, WebSocketMessage};

/// A client that records connection attempts and returns scripted results.
pub struct FakeWebSocketClient {
    connection_attempts: Arc<Mutex<Vec<(String, HeaderMap)>>>,
    connect_results: Mutex<VecDeque<Result<Box<dyn WebSocketConnection>>>>,
}

impl FakeWebSocketClient {
    pub fn new(connect_results: Vec<Result<Box<dyn WebSocketConnection>>>) -> Self {
        Self {
            connection_attempts: Arc::default(),
            connect_results: Mutex::new(connect_results.into()),
        }
    }

    pub fn connection_attempts(&self) -> Arc<Mutex<Vec<(String, HeaderMap)>>> {
        self.connection_attempts.clone()
    }
}

impl WebSocketClient for FakeWebSocketClient {
    fn connect(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> BoxFuture<'static, Result<Box<dyn WebSocketConnection>>> {
        self.connection_attempts
            .lock()
            .push((url.to_string(), headers));
        let result = self
            .connect_results
            .lock()
            .pop_front()
            .unwrap_or_else(|| Err(anyhow!("no scripted WebSocket connection left")));
        future::ready(result).boxed()
    }
}

/// A connection that records sent messages and returns scripted incoming messages.
pub struct ScriptedWebSocketConnection {
    sent_messages: Arc<Mutex<Vec<WebSocketMessage>>>,
    incoming_messages: VecDeque<Result<WebSocketMessage>>,
}

impl ScriptedWebSocketConnection {
    pub fn new(incoming_messages: Vec<Result<WebSocketMessage>>) -> Self {
        Self::with_sent_messages(Arc::default(), incoming_messages)
    }

    pub fn with_sent_messages(
        sent_messages: Arc<Mutex<Vec<WebSocketMessage>>>,
        incoming_messages: Vec<Result<WebSocketMessage>>,
    ) -> Self {
        Self {
            sent_messages,
            incoming_messages: incoming_messages.into(),
        }
    }

    pub fn sent_messages(&self) -> Arc<Mutex<Vec<WebSocketMessage>>> {
        self.sent_messages.clone()
    }
}

impl WebSocketConnection for ScriptedWebSocketConnection {
    fn send(&mut self, message: WebSocketMessage) -> BoxFuture<'_, Result<()>> {
        self.sent_messages.lock().push(message);
        future::ready(Ok(())).boxed()
    }

    fn receive(&mut self) -> BoxFuture<'_, Option<Result<WebSocketMessage>>> {
        future::ready(self.incoming_messages.pop_front()).boxed()
    }
}
