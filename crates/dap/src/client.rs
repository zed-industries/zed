use crate::{
    adapters::{DebugAdapterBinary, DebugAdapterName},
    transport::{IoKind, LogKind, TransportDelegate},
};
use anyhow::{Result, anyhow};
use dap_types::{
    messages::{Message, Response},
    requests::Request,
};
use futures::{FutureExt as _, channel::oneshot, select};
use gpui::{AppContext, AsyncApp, BackgroundExecutor};
use smol::channel::{Receiver, Sender};
use std::{
    hash::Hash,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

#[cfg(any(test, feature = "test-support"))]
const DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

#[cfg(not(any(test, feature = "test-support")))]
const DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(12);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SessionId(pub u32);

impl SessionId {
    pub fn from_proto(client_id: u64) -> Self {
        Self(client_id as u32)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }
}

/// Represents a connection to the debug adapter process, either via stdout/stdin or a socket.
pub struct DebugAdapterClient {
    id: SessionId,
    sequence_count: AtomicU64,
    binary: DebugAdapterBinary,
    executor: BackgroundExecutor,
    transport_delegate: TransportDelegate,
}

pub type DapMessageHandler = Box<dyn FnMut(Message) + 'static + Send + Sync>;

impl DebugAdapterClient {
    pub async fn start(
        id: SessionId,
        binary: DebugAdapterBinary,
        message_handler: DapMessageHandler,
        cx: AsyncApp,
    ) -> Result<Self> {
        let ((server_rx, server_tx), transport_delegate) =
            TransportDelegate::start(&binary, cx.clone()).await?;
        let this = Self {
            id,
            binary,
            transport_delegate,
            sequence_count: AtomicU64::new(1),
            executor: cx.background_executor().clone(),
        };
        log::info!("Successfully connected to debug adapter");

        let client_id = this.id;

        // start handling events/reverse requests
        cx.background_spawn(Self::handle_receive_messages(
            client_id,
            server_rx,
            server_tx.clone(),
            message_handler,
        ))
        .detach();

        Ok(this)
    }

    pub async fn reconnect(
        &self,
        session_id: SessionId,
        binary: DebugAdapterBinary,
        message_handler: DapMessageHandler,
        cx: AsyncApp,
    ) -> Result<Self> {
        let binary = match self.transport_delegate.transport() {
            crate::transport::Transport::Tcp(tcp_transport) => DebugAdapterBinary {
                adapter_name: binary.adapter_name,
                command: binary.command,
                arguments: binary.arguments,
                envs: binary.envs,
                cwd: binary.cwd,
                connection: Some(crate::adapters::TcpArguments {
                    host: tcp_transport.host,
                    port: tcp_transport.port,
                    timeout: Some(tcp_transport.timeout),
                }),
                request_args: binary.request_args,
            },
            _ => self.binary.clone(),
        };

        Self::start(session_id, binary, message_handler, cx).await
    }

    async fn handle_receive_messages(
        client_id: SessionId,
        server_rx: Receiver<Message>,
        client_tx: Sender<Message>,
        mut message_handler: DapMessageHandler,
    ) -> Result<()> {
        let result = loop {
            let message = match server_rx.recv().await {
                Ok(message) => message,
                Err(e) => break Err(e.into()),
            };
            match message {
                Message::Event(ev) => {
                    log::debug!("Client {} received event `{}`", client_id.0, &ev);

                    message_handler(Message::Event(ev))
                }
                Message::Request(req) => {
                    log::debug!(
                        "Client {} received reverse request `{}`",
                        client_id.0,
                        &req.command
                    );

                    message_handler(Message::Request(req))
                }
                Message::Response(response) => {
                    log::debug!("Received response after request timeout: {:#?}", response);
                }
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
            R::COMMAND,
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
                    true => {
                        if let Some(json) = response.body {
                            Ok(serde_json::from_value(json)?)
                        // Note: dap types configure themselves to return `None` when an empty object is received,
                        // which then fails here...
                        } else if let Ok(result) = serde_json::from_value(serde_json::Value::Object(Default::default())) {
                            Ok(result)
                        } else {
                            Ok(serde_json::from_value(Default::default())?)
                        }
                    }
                    false => Err(anyhow!("Request failed: {}", response.message.unwrap_or_default())),
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

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn name(&self) -> DebugAdapterName {
        self.binary.adapter_name.clone()
    }
    pub fn binary(&self) -> &DebugAdapterBinary {
        &self.binary
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
    pub fn on_request<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static
            + Send
            + FnMut(u64, R::Arguments) -> Result<R::Response, dap_types::ErrorResponse>,
    {
        let transport = self.transport_delegate.transport().as_fake();
        transport.on_request::<R, F>(handler);
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
        let transport = self.transport_delegate.transport().as_fake();
        transport.on_response::<R, F>(handler).await;
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
    use crate::{client::DebugAdapterClient, debugger_settings::DebuggerSettings};
    use dap_types::{
        Capabilities, InitializeRequestArguments, InitializeRequestArgumentsPathFormat,
        RunInTerminalRequestArguments, StartDebuggingRequestArguments,
        messages::Events,
        requests::{Initialize, Request, RunInTerminal},
    };
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

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

        let client = DebugAdapterClient::start(
            crate::client::SessionId(1),
            DebugAdapterBinary {
                adapter_name: "adapter".into(),
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                connection: None,
                cwd: None,
                request_args: StartDebuggingRequestArguments {
                    configuration: serde_json::Value::Null,
                    request: dap_types::StartDebuggingRequestArgumentsRequest::Launch,
                },
            },
            Box::new(|_| panic!("Did not expect to hit this code path")),
            cx.to_async(),
        )
        .await
        .unwrap();

        client.on_request::<Initialize, _>(move |_, _| {
            Ok(dap_types::Capabilities {
                supports_configuration_done_request: Some(true),
                ..Default::default()
            })
        });

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
                supports_ansistyling: Some(false),
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

        let called_event_handler = Arc::new(AtomicBool::new(false));

        let client = DebugAdapterClient::start(
            crate::client::SessionId(1),
            DebugAdapterBinary {
                adapter_name: "adapter".into(),
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                connection: None,
                cwd: None,
                request_args: StartDebuggingRequestArguments {
                    configuration: serde_json::Value::Null,
                    request: dap_types::StartDebuggingRequestArgumentsRequest::Launch,
                },
            },
            Box::new({
                let called_event_handler = called_event_handler.clone();
                move |event| {
                    called_event_handler.store(true, Ordering::SeqCst);

                    assert_eq!(
                        Message::Event(Box::new(Events::Initialized(
                            Some(Capabilities::default())
                        ))),
                        event
                    );
                }
            }),
            cx.to_async(),
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

        let called_event_handler = Arc::new(AtomicBool::new(false));

        let client = DebugAdapterClient::start(
            crate::client::SessionId(1),
            DebugAdapterBinary {
                adapter_name: "test-adapter".into(),
                command: "command".into(),
                arguments: Default::default(),
                envs: Default::default(),
                connection: None,
                cwd: None,
                request_args: dap_types::StartDebuggingRequestArguments {
                    configuration: serde_json::Value::Null,
                    request: dap_types::StartDebuggingRequestArgumentsRequest::Launch,
                },
            },
            Box::new({
                let called_event_handler = called_event_handler.clone();
                move |event| {
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
            }),
            cx.to_async(),
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
