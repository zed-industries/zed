use crate::{
    adapters::DebugAdapterBinary,
    transport::{IoKind, LogKind, TransportDelegate},
};
use anyhow::Result;
use dap_types::{
    messages::{Message, Response},
    requests::Request,
};
use futures::channel::oneshot;
use gpui::AsyncApp;
use std::{
    hash::Hash,
    sync::atomic::{AtomicU64, Ordering},
};

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
    transport_delegate: TransportDelegate,
}

pub type DapMessageHandler = Box<dyn FnMut(Message) + 'static + Send + Sync>;

impl DebugAdapterClient {
    pub async fn start(
        id: SessionId,
        binary: DebugAdapterBinary,
        message_handler: DapMessageHandler,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let transport_delegate = TransportDelegate::start(&binary, cx).await?;
        let this = Self {
            id,
            binary,
            transport_delegate,
            sequence_count: AtomicU64::new(1),
        };
        this.connect(message_handler, cx).await?;

        Ok(this)
    }

    pub fn should_reconnect_for_ssh(&self) -> bool {
        self.transport_delegate.tcp_arguments().is_some()
            && self.binary.command.as_deref() == Some("ssh")
    }

    pub async fn connect(
        &self,
        message_handler: DapMessageHandler,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        self.transport_delegate.connect(message_handler, cx).await
    }

    pub async fn create_child_connection(
        &self,
        session_id: SessionId,
        binary: DebugAdapterBinary,
        message_handler: DapMessageHandler,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let binary = if let Some(connection) = self.transport_delegate.tcp_arguments() {
            DebugAdapterBinary {
                command: None,
                arguments: Default::default(),
                envs: Default::default(),
                cwd: Default::default(),
                connection: Some(connection),
                request_args: binary.request_args,
            }
        } else {
            self.binary.clone()
        };

        Self::start(session_id, binary, message_handler, cx).await
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
            .pending_requests
            .lock()
            .insert(sequence_id, callback_tx)?;

        log::debug!(
            "Client {} send `{}` request with sequence_id: {}",
            self.id.0,
            R::COMMAND,
            sequence_id
        );

        self.send_message(Message::Request(request)).await?;

        let command = R::COMMAND.to_string();

        let response = callback_rx.await??;
        log::debug!(
            "Client {} received response for: `{}` sequence_id: {}",
            self.id.0,
            command,
            sequence_id
        );
        match response.success {
            true => {
                if let Some(json) = response.body {
                    Ok(serde_json::from_value(json)?)
                // Note: dap types configure themselves to return `None` when an empty object is received,
                // which then fails here...
                } else if let Ok(result) =
                    serde_json::from_value(serde_json::Value::Object(Default::default()))
                {
                    Ok(result)
                } else {
                    Ok(serde_json::from_value(Default::default())?)
                }
            }
            false => anyhow::bail!("Request failed: {}", response.message.unwrap_or_default()),
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<()> {
        self.transport_delegate.send_message(message).await
    }

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn binary(&self) -> &DebugAdapterBinary {
        &self.binary
    }

    /// Get the next sequence id to be used in a request
    pub fn next_sequence_id(&self) -> u64 {
        self.sequence_count.fetch_add(1, Ordering::Relaxed)
    }

    pub fn kill(&self) {
        log::debug!("Killing DAP process");
        self.transport_delegate.transport.lock().kill();
        self.transport_delegate.pending_requests.lock().shutdown();
    }

    pub fn has_adapter_logs(&self) -> bool {
        self.transport_delegate.has_adapter_logs()
    }

    pub fn add_log_handler<F>(&self, f: F, kind: LogKind)
    where
        F: 'static + Send + FnMut(IoKind, Option<&str>, &str),
    {
        self.transport_delegate.add_log_handler(f, kind);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn on_request<R: dap_types::requests::Request, F>(&self, mut handler: F)
    where
        F: 'static
            + Send
            + FnMut(u64, R::Arguments) -> Result<R::Response, dap_types::ErrorResponse>,
    {
        use crate::transport::RequestHandling;

        self.transport_delegate
            .transport
            .lock()
            .as_fake()
            .on_request::<R, _>(move |seq, request| {
                RequestHandling::Respond(handler(seq, request))
            });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn on_request_ext<R: dap_types::requests::Request, F>(&self, handler: F)
    where
        F: 'static
            + Send
            + FnMut(
                u64,
                R::Arguments,
            ) -> crate::transport::RequestHandling<
                Result<R::Response, dap_types::ErrorResponse>,
            >,
    {
        self.transport_delegate
            .transport
            .lock()
            .as_fake()
            .on_request::<R, F>(handler);
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
        self.transport_delegate
            .transport
            .lock()
            .as_fake()
            .on_response::<R, F>(handler);
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
        zlog::init_test();

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
                command: Some("command".into()),
                arguments: Default::default(),
                envs: Default::default(),
                connection: None,
                cwd: None,
                request_args: StartDebuggingRequestArguments {
                    configuration: serde_json::Value::Null,
                    request: dap_types::StartDebuggingRequestArgumentsRequest::Launch,
                },
            },
            Box::new(|_| {}),
            &mut cx.to_async(),
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
    }

    #[gpui::test]
    pub async fn test_calls_event_handler(cx: &mut TestAppContext) {
        init_test(cx);

        let called_event_handler = Arc::new(AtomicBool::new(false));

        let client = DebugAdapterClient::start(
            crate::client::SessionId(1),
            DebugAdapterBinary {
                command: Some("command".into()),
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
    }

    #[gpui::test]
    pub async fn test_calls_event_handler_for_reverse_request(cx: &mut TestAppContext) {
        init_test(cx);

        let called_event_handler = Arc::new(AtomicBool::new(false));

        let client = DebugAdapterClient::start(
            crate::client::SessionId(1),
            DebugAdapterBinary {
                command: Some("command".into()),
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
    }
}
