use crate::{
    adapters::{DebugAdapter, DebugAdapterBinary},
    transport::{IoKind, LogKind, TransportDelegate},
};
use anyhow::{anyhow, Result};

use dap_types::{
    messages::{Message, Response},
    requests::Request,
};
use gpui::{AppContext, AsyncAppContext};
use serde_json::Value;
use smol::channel::{bounded, Receiver, Sender};
use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use task::{DebugAdapterConfig, DebugRequestType};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThreadStatus {
    #[default]
    Running,
    Stopped,
    Exited,
    Ended,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterClientId(pub usize);

pub struct DebugAdapterClient {
    id: DebugAdapterClientId,
    adapter_id: String,
    request_args: Value,
    sequence_count: AtomicU64,
    config: DebugAdapterConfig,
    transport_delegate: TransportDelegate,
}

impl DebugAdapterClient {
    pub fn new(
        id: DebugAdapterClientId,
        request_args: Value,
        config: DebugAdapterConfig,
        adapter: Arc<Box<dyn DebugAdapter>>,
    ) -> Self {
        Self {
            id,
            config,
            request_args,
            sequence_count: AtomicU64::new(1),
            adapter_id: adapter.name().to_string(),
            transport_delegate: TransportDelegate::new(adapter.transport()),
        }
    }

    pub async fn start<F>(
        &mut self,
        binary: &DebugAdapterBinary,
        message_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<()>
    where
        F: FnMut(Message, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let (server_rx, server_tx) = self.transport_delegate.start(binary, cx).await?;

        // start handling events/reverse requests
        cx.spawn(|mut cx| async move {
            Self::handle_receive_messages(server_rx, server_tx, message_handler, &mut cx).await
        })
        .detach();

        Ok(())
    }

    async fn handle_receive_messages<F>(
        server_rx: Receiver<Message>,
        client_tx: Sender<Message>,
        mut event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<()>
    where
        F: FnMut(Message, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        while let Ok(payload) = server_rx.recv().await {
            match payload {
                Message::Event(ev) => cx.update(|cx| event_handler(Message::Event(ev), cx))?,
                Message::Response(_) => unreachable!(),
                Message::Request(req) => {
                    cx.update(|cx| event_handler(Message::Request(req), cx))?
                }
            };
        }

        drop(client_tx);

        anyhow::Ok(())
    }

    /// Send a request to an adapter and get a response back
    /// Note: This function will block until a response is sent back from the adapter
    pub async fn request<R: Request>(&self, arguments: R::Arguments) -> Result<R::Response> {
        let serialized_arguments = serde_json::to_value(arguments)?;

        let (callback_tx, callback_rx) = bounded::<Result<Response>>(1);

        let sequence_id = self.next_sequence_id();

        let request = crate::messages::Request {
            seq: sequence_id,
            command: R::COMMAND.to_string(),
            arguments: Some(serialized_arguments),
        };

        self.transport_delegate
            .add_pending_request(sequence_id, callback_tx)
            .await;

        self.send_message(Message::Request(request)).await?;

        let response = callback_rx.recv().await??;

        match response.success {
            true => Ok(serde_json::from_value(response.body.unwrap_or_default())?),
            false => Err(anyhow!("Request failed")),
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<()> {
        self.transport_delegate.send_message(message).await
    }

    pub fn id(&self) -> DebugAdapterClientId {
        self.id
    }

    pub fn config(&self) -> DebugAdapterConfig {
        self.config.clone()
    }

    pub fn adapter_id(&self) -> String {
        self.adapter_id.clone()
    }

    pub fn request_args(&self) -> Value {
        self.request_args.clone()
    }

    pub fn request_type(&self) -> DebugRequestType {
        self.config.request.clone()
    }

    /// Get the next sequence id to be used in a request
    pub fn next_sequence_id(&self) -> u64 {
        self.sequence_count.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.transport_delegate.shutdown().await
    }

    pub fn has_adapter_logs(&self) -> bool {
        self.transport_delegate.has_adapter_logs()
    }

    pub fn add_log_handler<F>(&self, f: F, kind: LogKind)
    where
        F: 'static + Send + FnMut(IoKind, &str),
    {
        self.transport_delegate.add_log_handler(f, kind);
    }
}
