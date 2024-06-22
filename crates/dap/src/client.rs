use crate::{
    requests::{
        Continue, ContinueArguments, Initialize, InitializeArguments, Launch,
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
use gpui::AsyncWindowContext;
use serde_json::Value;
use smol::{
    io::BufReader,
    net::TcpStream,
    process::{self, Child},
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use util::ResultExt;

pub enum TransportType {
    TCP,
    STDIO,
}

#[derive(Debug)]
pub struct Client {
    _process: Option<Child>,
    server_tx: UnboundedSender<Payload>,
    request_count: AtomicU64,
    thread_id: Option<ThreadId>,
}

impl Client {
    pub async fn new(
        transport_type: TransportType,
        command: &str,
        args: Vec<&str>,
        port_arg: Option<&str>,
        cx: &mut AsyncWindowContext,
    ) -> Result<(Self, UnboundedReceiver<Payload>)> {
        match transport_type {
            TransportType::TCP => Self::create_tcp_client(command, args, port_arg, cx).await,
            TransportType::STDIO => Self::create_stdio_client(command, args, port_arg, cx).await,
        }
    }

    async fn create_tcp_client(
        command: &str,
        args: Vec<&str>,
        port_arg: Option<&str>,
        cx: &mut AsyncWindowContext,
    ) -> Result<(Self, UnboundedReceiver<Payload>)> {
        let mut command = process::Command::new("bun");
        command
            .current_dir("/Users/remcosmits/Documents/code/symfony_demo")
            .args([
                "/Users/remcosmits/Documents/code/vscode-php-debug/out/phpDebug.js",
                "--server=8123",
            ])
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

        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8123);

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
        port_arg: Option<&str>,
        cx: &mut AsyncWindowContext,
    ) -> Result<(Self, UnboundedReceiver<Payload>)> {
        todo!("not implemented")
    }

    pub fn handle_transport(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        process: Option<Child>,
        cx: &mut AsyncWindowContext,
    ) -> Result<(Self, UnboundedReceiver<Payload>)> {
        let (server_rx, server_tx) = Transport::start(rx, tx, err, cx);
        let (client_tx, client_rx) = unbounded::<Payload>();

        let client = Self {
            server_tx: server_tx.clone(),
            _process: process,
            request_count: AtomicU64::new(0),
            thread_id: Some(ThreadId(1)),
        };

        cx.spawn(move |_| Self::recv(server_rx, server_tx, client_tx))
            .detach();

        Ok((client, client_rx))
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
            client_name: Some("zed".to_owned()),
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

    pub async fn set_breakpoints(&self, line: usize) -> Result<Value> {
        self.request::<SetBreakpoints>(SetBreakpointsArguments {
            source: Source {
                path: Some("/Users/remcosmits/Documents/code/symfony_demo/src/Kernel.php".into()),
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
}
