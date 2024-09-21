use crate::transport::Transport;
use anyhow::{anyhow, Context, Result};

use crate::adapters::{build_adapter, DebugAdapter};
use dap_types::{
    messages::{Message, Response},
    requests::Request,
};
use futures::{AsyncBufRead, AsyncWrite};
use gpui::{AppContext, AsyncAppContext};
use parking_lot::Mutex;
use serde_json::Value;
use smol::{
    channel::{bounded, Receiver, Sender},
    process::Child,
};
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
    adapter: Arc<Box<dyn DebugAdapter>>,
    transport: Arc<Transport>,
    _process: Arc<Mutex<Option<Child>>>,
    sequence_count: AtomicU64,
    config: DebugAdapterConfig,
}

pub struct TransportParams {
    rx: Box<dyn AsyncBufRead + Unpin + Send>,
    tx: Box<dyn AsyncWrite + Unpin + Send>,
    err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
    process: Option<Child>,
}

impl TransportParams {
    pub fn new(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        process: Option<Child>,
    ) -> Self {
        TransportParams {
            rx,
            tx,
            err,
            process,
        }
    }
}

impl DebugAdapterClient {
    pub async fn new<F>(
        id: DebugAdapterClientId,
        config: DebugAdapterConfig,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<Self>>
    where
        F: FnMut(Message, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let adapter = Arc::new(build_adapter(&config).context("Creating debug adapter")?);
        let transport_params = adapter.connect(cx).await?;

        let transport = Self::handle_transport(
            transport_params.rx,
            transport_params.tx,
            transport_params.err,
            event_handler,
            cx,
        );

        Ok(Arc::new(Self {
            id,
            config,
            adapter,
            transport,
            sequence_count: AtomicU64::new(1),
            _process: Arc::new(Mutex::new(transport_params.process)),
        }))
    }

    pub fn handle_transport<F>(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        event_handler: F,
        cx: &mut AsyncAppContext,
    ) -> Arc<Transport>
    where
        F: FnMut(Message, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let transport = Transport::start(rx, tx, err, cx);

        let server_rx = transport.server_rx.clone();
        let server_tr = transport.server_tx.clone();
        cx.spawn(|mut cx| async move {
            Self::handle_recv(server_rx, server_tr, event_handler, &mut cx).await
        })
        .detach();

        transport
    }

    async fn handle_recv<F>(
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

        {
            self.transport
                .current_requests
                .lock()
                .await
                .insert(sequence_id, callback_tx);
        }

        self.transport
            .server_tx
            .send(Message::Request(request))
            .await?;

        let response = callback_rx.recv().await??;

        match response.success {
            true => Ok(serde_json::from_value(response.body.unwrap_or_default())?),
            false => Err(anyhow!("Request failed")),
        }
    }

    pub fn id(&self) -> DebugAdapterClientId {
        self.id
    }

    pub fn config(&self) -> DebugAdapterConfig {
        self.config.clone()
    }

    pub fn adapter(&self) -> Arc<Box<dyn DebugAdapter>> {
        self.adapter.clone()
    }

    pub fn request_args(&self) -> Value {
        self.adapter.request_args()
    }

    pub fn request_type(&self) -> DebugRequestType {
        self.config.request.clone()
    }

    /// Get the next sequence id to be used in a request
    pub fn next_sequence_id(&self) -> u64 {
        self.sequence_count.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.transport.server_tx.close();
        self.transport.server_rx.close();

        let mut adapter = self._process.lock().take();

        async move {
            let mut current_requests = self.transport.current_requests.lock().await;
            let mut pending_requests = self.transport.pending_requests.lock().await;

            current_requests.clear();
            pending_requests.clear();

            if let Some(mut adapter) = adapter.take() {
                adapter.kill()?;
            }

            drop(current_requests);
            drop(pending_requests);
            drop(adapter);

            anyhow::Ok(())
        }
        .await
    }
}
