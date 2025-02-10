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
use gpui::{App, AsyncApp, BackgroundExecutor};
use smol::channel::{Receiver, Sender};
use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

#[cfg(any(test, feature = "test-support"))]
const DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

#[cfg(not(any(test, feature = "test-support")))]
const DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(12);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterClientId(pub usize);

impl DebugAdapterClientId {
    pub fn from_proto(client_id: u64) -> Self {
        Self(client_id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }
}

pub struct DebugAdapterClient {
    id: DebugAdapterClientId,
    sequence_count: AtomicU64,
    binary: DebugAdapterBinary,
    executor: BackgroundExecutor,
    adapter: Arc<dyn DebugAdapter>,
    transport_delegate: TransportDelegate,
}

impl DebugAdapterClient {
    pub fn new(
        id: DebugAdapterClientId,
        adapter: Arc<dyn DebugAdapter>,
        binary: DebugAdapterBinary,
        cx: &AsyncApp,
    ) -> Self {
        let transport_delegate = TransportDelegate::new(adapter.transport());

        Self {
            id,
            binary,
            adapter,
            transport_delegate,
            sequence_count: AtomicU64::new(1),
            executor: cx.background_executor().clone(),
        }
    }

    pub async fn reconnect<F>(&mut self, message_handler: F, cx: &mut AsyncApp) -> Result<()>
    where
        F: FnMut(Message, &mut App) + 'static + Send + Sync + Clone,
    {
        let (server_rx, server_tx) = self.transport_delegate.reconnect(cx).await?;
        log::info!("Successfully reconnected to debug adapter");

        let client_id = self.id;

        // start handling events/reverse requests
        cx.update(|cx| {
            cx.spawn({
                let server_tx = server_tx.clone();
                |mut cx| async move {
                    Self::handle_receive_messages(
                        client_id,
                        server_rx,
                        server_tx,
                        message_handler,
                        &mut cx,
                    )
                    .await
                }
            })
            .detach_and_log_err(cx);
        })
    }

    pub async fn start<F>(&mut self, message_handler: F, cx: &mut AsyncApp) -> Result<()>
    where
        F: FnMut(Message, &mut App) + 'static + Send + Sync + Clone,
    {
        let (server_rx, server_tx) = self.transport_delegate.start(&self.binary, cx).await?;
        log::info!("Successfully connected to debug adapter");

        let client_id = self.id;

        // start handling events/reverse requests
        cx.update(|cx| {
            cx.spawn({
                let server_tx = server_tx.clone();
                |mut cx| async move {
                    Self::handle_receive_messages(
                        client_id,
                        server_rx,
                        server_tx,
                        message_handler,
                        &mut cx,
                    )
                    .await
                }
            })
            .detach_and_log_err(cx);
        })
    }

    async fn handle_receive_messages<F>(
        client_id: DebugAdapterClientId,
        server_rx: Receiver<Message>,
        client_tx: Sender<Message>,
        mut event_handler: F,
        cx: &mut AsyncApp,
    ) -> Result<()>
    where
        F: FnMut(Message, &mut App) + 'static + Send + Sync + Clone,
    {
        let result = loop {
            let message = match server_rx.recv().await {
                Ok(message) => message,
                Err(e) => break Err(e.into()),
            };

            if let Err(e) = match message {
                Message::Event(ev) => {
                    log::debug!("Client {} received event `{}`", client_id.0, &ev);

                    cx.update(|cx| event_handler(Message::Event(ev), cx))
                }
                Message::Request(req) => cx.update(|cx| event_handler(Message::Request(req), cx)),
                Message::Response(response) => {
                    log::debug!("Received response after request timeout: {:#?}", response);

                    Ok(())
                }
            } {
                break Err(e);
            }

            smol::future::yield_now().await;
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
            "Client {} send `{}` request with sequence_id: {}",
            self.id.0,
            R::COMMAND.to_string(),
            sequence_id
        );

        self.send_message(Message::Request(request)).await?;

        let mut timeout = self.executor.timer(DAP_REQUEST_TIMEOUT).fuse();
        let command = R::COMMAND.to_string();

        select! {
            response = callback_rx.fuse() => {
                log::debug!(
                    "Client {} received response for: `{}` sequence_id: {}",
                    self.id.0,
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

    pub fn adapter(&self) -> &Arc<dyn DebugAdapter> {
        &self.adapter
    }

    pub fn binary(&self) -> &DebugAdapterBinary {
        &self.binary
    }

    pub fn adapter_id(&self) -> String {
        self.adapter.name().to_string()
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

    #[cfg(any(test, feature = "test-support"))]
    pub async fn on_request<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static
            + Send
            + FnMut(u64, R::Arguments) -> Result<R::Response, dap_types::ErrorResponse>,
    {
        let transport = self.transport_delegate.transport();

        transport.as_fake().on_request::<R, F>(handler).await;
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn fake_reverse_request<R: dap_types::requests::Request>(&self, args: R::Arguments) {
        self.send_message(Message::Request(dap_types::messages::Request {
            seq: self.sequence_count.load(Ordering::Relaxed),
            command: R::COMMAND.into(),
            arguments: serde_json::to_value(args).ok(),
        }))
        .await
        .unwrap();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn on_response<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static + Send + Fn(Response),
    {
        let transport = self.transport_delegate.transport();

        transport.as_fake().on_response::<R, F>(handler).await;
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn fake_event(&self, event: dap_types::messages::Events) {
        self.send_message(Message::Event(Box::new(event)))
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapters::FakeAdapter, client::DebugAdapterClient, debugger_settings::DebuggerSettings,
    };
    use dap_types::{
        messages::Events,
        requests::{Initialize, Request, RunInTerminal},
        Capabilities, InitializeRequestArguments, InitializeRequestArgumentsPathFormat,
        RunInTerminalRequestArguments,
    };
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::atomic::{AtomicBool, Ordering};

    pub fn init_test(cx: &mut gpui::TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            DebuggerSettings::register(cx);
        });
    }

    #[gpui::test]
    pub async fn test_initialize_client(cx: &mut TestAppContext) {
        init_test(cx);

        let adapter = Arc::new(FakeAdapter::new());

        let mut client = DebugAdapterClient::new(
            crate::client::DebugAdapterClientId(1),
            adapter,
            DebugAdapterBinary {
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                cwd: None,
            },
            &mut cx.to_async(),
        );

        client
            .on_request::<Initialize, _>(move |_, _| {
                Ok(dap_types::Capabilities {
                    supports_configuration_done_request: Some(true),
                    ..Default::default()
                })
            })
            .await;

        client
            .start(
                |_, _| panic!("Did not expect to hit this code path"),
                &mut cx.to_async(),
            )
            .await
            .unwrap();

        cx.run_until_parked();

        let response = client
            .request::<Initialize>(InitializeRequestArguments {
                client_id: Some("zed".to_owned()),
                client_name: Some("Zed".to_owned()),
                adapter_id: "fake-adapter".to_owned(),
                locale: Some("en-US".to_owned()),
                path_format: Some(InitializeRequestArgumentsPathFormat::Path),
                supports_variable_type: Some(true),
                supports_variable_paging: Some(false),
                supports_run_in_terminal_request: Some(true),
                supports_memory_references: Some(true),
                supports_progress_reporting: Some(false),
                supports_invalidated_event: Some(false),
                lines_start_at1: Some(true),
                columns_start_at1: Some(true),
                supports_memory_event: Some(false),
                supports_args_can_be_interpreted_by_shell: Some(false),
                supports_start_debugging_request: Some(true),
            })
            .await
            .unwrap();

        cx.run_until_parked();

        assert_eq!(
            dap_types::Capabilities {
                supports_configuration_done_request: Some(true),
                ..Default::default()
            },
            response
        );

        client.shutdown().await.unwrap();
    }

    #[gpui::test]
    pub async fn test_calls_event_handler(cx: &mut TestAppContext) {
        init_test(cx);

        let adapter = Arc::new(FakeAdapter::new());
        let called_event_handler = Arc::new(AtomicBool::new(false));

        let mut client = DebugAdapterClient::new(
            crate::client::DebugAdapterClientId(1),
            adapter,
            DebugAdapterBinary {
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                cwd: None,
            },
            &mut cx.to_async(),
        );

        client
            .start(
                {
                    let called_event_handler = called_event_handler.clone();
                    move |event, _| {
                        called_event_handler.store(true, Ordering::SeqCst);

                        assert_eq!(
                            Message::Event(Box::new(Events::Initialized(Some(
                                Capabilities::default()
                            )))),
                            event
                        );
                    }
                },
                &mut cx.to_async(),
            )
            .await
            .unwrap();

        cx.run_until_parked();

        client
            .fake_event(Events::Initialized(Some(Capabilities::default())))
            .await;

        cx.run_until_parked();

        assert!(
            called_event_handler.load(std::sync::atomic::Ordering::SeqCst),
            "Event handler was not called"
        );

        client.shutdown().await.unwrap();
    }

    #[gpui::test]
    pub async fn test_calls_event_handler_for_reverse_request(cx: &mut TestAppContext) {
        init_test(cx);

        let adapter = Arc::new(FakeAdapter::new());
        let called_event_handler = Arc::new(AtomicBool::new(false));

        let mut client = DebugAdapterClient::new(
            crate::client::DebugAdapterClientId(1),
            adapter,
            DebugAdapterBinary {
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                cwd: None,
            },
            &mut cx.to_async(),
        );

        client
            .start(
                {
                    let called_event_handler = called_event_handler.clone();
                    move |event, _| {
                        called_event_handler.store(true, Ordering::SeqCst);

                        assert_eq!(
                            Message::Request(dap_types::messages::Request {
                                seq: 1,
                                command: RunInTerminal::COMMAND.into(),
                                arguments: Some(json!({
                                    "cwd": "/project/path/src",
                                    "args": ["node", "test.js"],
                                }))
                            }),
                            event
                        );
                    }
                },
                &mut cx.to_async(),
            )
            .await
            .unwrap();

        cx.run_until_parked();

        client
            .fake_reverse_request::<RunInTerminal>(RunInTerminalRequestArguments {
                kind: None,
                title: None,
                cwd: "/project/path/src".into(),
                args: vec!["node".into(), "test.js".into()],
                env: None,
                args_can_be_interpreted_by_shell: None,
            })
            .await;

        cx.run_until_parked();

        assert!(
            called_event_handler.load(std::sync::atomic::Ordering::SeqCst),
            "Event handler was not called"
        );

        client.shutdown().await.unwrap();
    }
}
