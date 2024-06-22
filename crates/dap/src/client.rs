use crate::{
    requests::{
        ConfigurationDone, Continue, ContinueArguments, Initialize, InitializeArguments, Launch,
        LaunchRequestArguments, Next, NextArguments, SetBreakpoints, SetBreakpointsArguments,
        StepIn, StepInArguments, StepOut, StepOutArguments,
    },
    transport::{self, Payload, Request, Transport},
    types::{Source, SourceBreakpoint, ThreadId},
};
use anyhow::{anyhow, Context, Result};
use futures::{
    channel::mpsc::{channel, unbounded, UnboundedReceiver, UnboundedSender},
    AsyncBufRead, AsyncReadExt, AsyncWrite, SinkExt as _, StreamExt,
};
use gpui::AsyncAppContext;
use serde_json::Value;
use smol::{
    io::BufReader,
    net::TcpStream,
    process::{self, Child},
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    process::Stdio,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use util::ResultExt;

pub enum TransportType {
    TCP,
    STDIO,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterClientId(pub usize);

#[derive(Debug)]
pub struct DebugAdapterClient {
    _process: Option<Child>,
    server_tx: UnboundedSender<Payload>,
    request_count: AtomicU64,
    thread_id: Option<ThreadId>,
    client_rx: UnboundedReceiver<Payload>,
}

impl DebugAdapterClient {
    pub async fn new(
        transport_type: TransportType,
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        match transport_type {
            TransportType::TCP => {
                Self::create_tcp_client(command, args, port, project_path, cx).await
            }
            TransportType::STDIO => {
                Self::create_stdio_client(command, args, port, project_path, cx).await
            }
        }
    }

    async fn create_tcp_client(
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let mut command = process::Command::new(command);
        command
            .current_dir(project_path)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let process = command
            .spawn()
            .with_context(|| "failed to spawn command.")?;

        // give the adapter some time to spin up the tcp server
        cx.background_executor()
            .timer(Duration::from_millis(500))
            .await;

        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

        let (rx, tx) = TcpStream::connect(address).await?.split();

        Self::handle_transport(
            Box::new(BufReader::new(rx)),
            Box::new(tx),
            None,
            Some(process),
            cx,
        )
    }

    async fn create_stdio_client(
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        todo!("not implemented")
    }

    pub fn handle_transport(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        process: Option<Child>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let (server_rx, server_tx) = Transport::start(rx, tx, err, cx);
        let (client_tx, client_rx) = unbounded::<Payload>();

        let client = Self {
            server_tx: server_tx.clone(),
            _process: process,
            request_count: AtomicU64::new(0),
            thread_id: Some(ThreadId(1)),
            client_rx, // TODO: remove this here
        };

        cx.spawn(move |_| Self::recv(server_rx, server_tx, client_tx))
            .detach();

        Ok(client)
    }

    async fn recv(
        mut server_rx: UnboundedReceiver<Payload>,
        mut server_tx: UnboundedSender<Payload>,
        mut client_tx: UnboundedSender<Payload>,
    ) {
        while let Some(payload) = server_rx.next().await {
            match payload {
                Payload::Event(ev) => client_tx.send(Payload::Event(ev)).await.log_err(),
                Payload::Response(res) => server_tx.send(Payload::Response(res)).await.log_err(),
                Payload::Request(req) => client_tx.send(Payload::Request(req)).await.log_err(),
            };
        }
    }

    pub async fn request<R: crate::requests::Request>(
        &self,
        arguments: R::Arguments,
    ) -> Result<Value> {
        let serialized_arguments = serde_json::to_value(arguments)?;

        let (callback_tx, mut callback_rx) = channel::<Result<transport::Response>>(1);

        let request = Request {
            back_ch: Some(callback_tx),
            seq: self.next_request_id(),
            command: R::COMMAND.to_string(),
            arguments: Some(serialized_arguments),
        };

        self.server_tx
            .clone()
            .send(Payload::Request(request))
            .await?;

        let response = callback_rx.next().await.ok_or(anyhow!("no response"))??;

        match response.success {
            true => Ok(response.body.unwrap_or_default()),
            false => Err(anyhow!("Request failed")),
        }
    }

    pub fn next_request_id(&self) -> u64 {
        self.request_count.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn initialize(&self) -> Result<Value> {
        let args = InitializeArguments {
            client_id: Some("zed".to_owned()),
            client_name: Some("Zed".to_owned()),
            adapter_id: "xdebug".into(),
            locale: Some("en-us".to_owned()),
            lines_start_at_one: Some(true),
            columns_start_at_one: Some(true),
            path_format: Some("path".to_owned()),
            supports_variable_type: Some(true),
            supports_variable_paging: Some(false),
            supports_run_in_terminal_request: Some(false),
            supports_memory_references: Some(false),
            supports_progress_reporting: Some(false),
            supports_invalidated_event: Some(false),
        };

        self.request::<Initialize>(args).await
    }

    pub async fn launch(&mut self) -> Result<Value> {
        self.request::<Launch>(LaunchRequestArguments {
            no_debug: Some(false),
            __restart: None,
        })
        .await
    }

    pub async fn next_thread(&self) {
        if let Some(thread_id) = self.thread_id {
            let _ = self
                .request::<Next>(NextArguments {
                    thread_id,
                    granularity: None,
                })
                .await;
        }
    }

    pub async fn continue_thread(&self) {
        if let Some(thread_id) = self.thread_id {
            let _ = self
                .request::<Continue>(ContinueArguments { thread_id })
                .await;
        }
    }

    pub async fn step_in(&self) {
        if let Some(thread_id) = self.thread_id {
            let _ = self
                .request::<StepIn>(StepInArguments {
                    thread_id,
                    target_id: None,
                    granularity: None,
                })
                .await;
        }
    }

    pub async fn step_out(&self) {
        if let Some(thread_id) = self.thread_id {
            let _ = self
                .request::<StepOut>(StepOutArguments {
                    thread_id,
                    granularity: None,
                })
                .await;
        }
    }

    pub async fn step_back(&self) {
        if let Some(thread_id) = self.thread_id {
            let _ = self
                .request::<StepIn>(StepInArguments {
                    thread_id,
                    target_id: None,
                    granularity: None,
                })
                .await;
        }
    }

    pub async fn set_breakpoints(&self, path: PathBuf, line: usize) -> Result<Value> {
        self.request::<SetBreakpoints>(SetBreakpointsArguments {
            source: Source {
                path: Some(path),
                ..Default::default()
            },
            breakpoints: Some(vec![SourceBreakpoint {
                line,
                column: None,
                condition: None,
                hit_condition: None,
                log_message: None,
            }]),
            source_modified: None,
        })
        .await
    }

    pub async fn configuration_done(&self) -> Result<Value> {
        self.request::<ConfigurationDone>(()).await
    }
}
