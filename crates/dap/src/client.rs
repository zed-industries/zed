use crate::transport::{Payload, Request, Transport};
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
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
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
        let mut command = process::Command::new("python3");
        command
            .current_dir("/Users/remcosmits/Documents/code/debugpy")
            .arg(format!(
                "-m debugpy --listen localhost:{} --wait-for-client test.py",
                5679
            ))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);

        let process = command
            .spawn()
            .with_context(|| "failed to spawn command.")?;

        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5679);

        dbg!(addr);

        let (rx, tx) = TcpStream::connect(addr).await?.split();

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
        &mut self,
        arguments: R::Arguments,
    ) -> Result<Value> {
        let arguments = Some(serde_json::to_value(arguments)?);

        let (callback_tx, mut callback_rx) = channel(1);

        let request = Request {
            back_ch: Some(callback_tx),
            seq: self.next_request_id(),
            command: R::COMMAND.to_string(),
            arguments,
        };

        self.server_tx.send(Payload::Request(request)).await?;

        callback_rx
            .next()
            .await
            .ok_or(anyhow!("no response"))?
            .map(|response| response.body.unwrap_or_default())
    }

    fn next_request_id(&mut self) -> u64 {
        self.request_count.fetch_add(1, Ordering::Relaxed)
    }
}
