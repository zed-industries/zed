use crate::{
    adapters::{DebugAdapter, DebugAdapterBinary},
    transport::{IoKind, LogKind, TransportDelegate},
};
use anyhow::{anyhow, Result};
use dap_types::{
    messages::{Message, Response},
    requests::Request,
};
use futures::{channel::oneshot, select, FutureExt as _};
use gpui::{AppContext, AsyncAppContext, BackgroundExecutor};
use smol::channel::{Receiver, Sender};
use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use task::{DebugAdapterConfig, DebugRequestType};

const DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

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
    sequence_count: AtomicU64,
    executor: BackgroundExecutor,
    adapter: Arc<Box<dyn DebugAdapter>>,
    transport_delegate: TransportDelegate,
    config: Arc<Mutex<DebugAdapterConfig>>,
}

impl DebugAdapterClient {
    pub fn new(
        id: DebugAdapterClientId,
        config: DebugAdapterConfig,
        adapter: Arc<Box<dyn DebugAdapter>>,
        cx: &AsyncAppContext,
    ) -> Self {
        let transport_delegate = TransportDelegate::new(adapter.transport());

        Self {
            id,
            adapter,
            transport_delegate,
            sequence_count: AtomicU64::new(1),
            config: Arc::new(Mutex::new(config)),
            executor: cx.background_executor().clone(),
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
        cx.update(|cx| {
            cx.spawn({
                let server_tx = server_tx.clone();
                |mut cx| async move {
                    Self::handle_receive_messages(server_rx, server_tx, message_handler, &mut cx)
                        .await
                }
            })
            .detach_and_log_err(cx);
        })
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
        let result = loop {
            let message = match server_rx.recv().await {
                Ok(message) => message,
                Err(e) => break Err(e.into()),
            };

            if let Err(e) = match message {
                Message::Event(ev) => cx.update(|cx| event_handler(Message::Event(ev), cx)),
                Message::Request(req) => cx.update(|cx| event_handler(Message::Request(req), cx)),
                Message::Response(_) => unreachable!(),
            } {
                break Err(e);
            }
        };

        drop(client_tx);

        log::debug!("Handle receive messages dropped");

        result
    }

    /// Send a request to an adapter and get a response back
    /// Note: This function will block until a response is sent back from the adapter
    pub async fn request<R: Request>(&self, arguments: R::Arguments) -> Result<R::Response> {
        let serialized_arguments = serde_json::to_value(arguments)?;

        let (callback_tx, callback_rx) = oneshot::channel::<Result<Response>>();

        let sequence_id = self.next_sequence_id();

        let request = crate::messages::Request {
            seq: sequence_id,
            command: R::COMMAND.to_string(),
            arguments: Some(serialized_arguments),
        };

        self.transport_delegate
            .add_pending_request(sequence_id, callback_tx)
            .await;

        log::debug!(
            "Send `{}` request with sequence_id: {}",
            R::COMMAND.to_string(),
            sequence_id
        );

        self.send_message(Message::Request(request)).await?;

        log::debug!(
            "Start receiving response for: `{}` sequence_id: {}",
            R::COMMAND.to_string(),
            sequence_id
        );

        let mut timeout = self.executor.timer(DAP_REQUEST_TIMEOUT).fuse();
        let command = R::COMMAND.to_string();

        select! {
            response = callback_rx.fuse() => {
                log::debug!(
                    "Received response for: `{}` sequence_id: {}",
                    command,
                    sequence_id
                );

                let response = response??;
                match response.success {
                    true => Ok(serde_json::from_value(response.body.unwrap_or_default())?),
                    false => Err(anyhow!("Request failed")),
                }
            }

            _ = timeout => {
                self.transport_delegate.cancel_pending_request(&sequence_id).await;
                log::error!("Cancelled DAP request for {command:?} id {sequence_id} which took over {DAP_REQUEST_TIMEOUT:?}");
                anyhow::bail!("DAP request timeout");
            }
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<()> {
        self.transport_delegate.send_message(message).await
    }

    pub fn id(&self) -> DebugAdapterClientId {
        self.id
    }

    pub fn config(&self) -> DebugAdapterConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn adapter(&self) -> &Arc<Box<dyn DebugAdapter>> {
        &self.adapter
    }

    pub fn adapter_id(&self) -> String {
        self.adapter.name().to_string()
    }

    pub fn set_process_id(&self, process_id: u32) {
        let mut config = self.config.lock().unwrap();

        config.request = DebugRequestType::Attach(task::AttachConfig {
            process_id: Some(process_id),
        });
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
