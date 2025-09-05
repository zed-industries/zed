use crate::{
    RemoteClientDelegate,
    json_log::LogRecord,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    SinkExt, StreamExt as _,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
};
use gpui::{App, AppContext as _, AsyncApp, Task};
use iroh::{Endpoint, EndpointAddr, EndpointId, TransportAddr};
use iroh_tickets::{ParseError, Ticket};
use rpc::proto::Envelope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::P2pConnection;
use std::{
    collections::BTreeSet,
    fmt::{self, Display},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};
use tokio_util::{bytes::Bytes, codec::LengthDelimitedCodec};
use util::paths::{PathStyle, RemotePathBuf};

/// The ALPN, or application-layer protocol negotiation, is exchanged in the
/// connection handshake, and the connection is aborted unless both endpoints pass
/// the same bytestring.
pub const ZED_ALPN: &[u8] = b"iroh/zed/remote/0";

// max length of an RPC message in bytes
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct IrohZedRemote {
    options: IrohConnectionOptions,
    endpoint: Endpoint,
    ssh_shell: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrohConnectionOptions {
    pub ticket: ZedIrohTicket,
    pub port_forwards: Option<Vec<IrohPortForwardOption>>,
    pub nickname: Option<String>,
}

impl From<P2pConnection> for IrohConnectionOptions {
    fn from(val: P2pConnection) -> Self {
        IrohConnectionOptions {
            ticket: val.ticket.parse().expect("invalid ticket"), // TODO: handle error
            port_forwards: Default::default(),
            nickname: val.nickname,
        }
    }
}
impl IrohConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let ticket = input.parse()?;
        Ok(Self {
            ticket,
            port_forwards: None,
            nickname: None,
        })
    }

    pub fn connection_string(&self) -> String {
        self.ticket.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct IrohPortForwardOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_host: Option<String>,
    pub local_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    pub remote_port: u16,
}

#[async_trait(?Send)]
impl RemoteConnection for IrohZedRemote {
    async fn kill(&self) -> Result<()> {
        self.endpoint.close().await;
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.endpoint.is_closed()
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Iroh(self.options.clone())
    }

    fn shell(&self) -> String {
        self.ssh_shell.clone()
    }

    fn default_system_shell(&self) -> String {
        // TODO: unused
        String::from("/bin/sh")
    }

    fn build_command(
        &self,
        _input_program: Option<String>,
        _input_args: &[String],
        input_env: &HashMap<String, String>,
        _working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        // TODO: unused?
        Ok(CommandTemplate {
            program: "iroh".into(),
            args: vec![],
            env: input_env.clone(),
        })
    }

    fn upload_directory(
        &self,
        _src_path: PathBuf,
        _dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        // TODO: unused?
        cx.background_spawn(async move { Ok(()) })
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        Err(anyhow::anyhow!("Not currently supported for iroh"))
    }

    fn start_proxy(
        &self,
        _unique_identifier: String,
        _reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Opening stream"), cx);

        let addr = self.options.ticket.addr.clone();
        match handle_rpc_messages(
            &self.endpoint,
            addr,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        ) {
            Ok(task) => task,
            Err(error) => Task::ready(Err(anyhow!("failed to spawn iroh server: {}", error))),
        }
    }

    fn path_style(&self) -> PathStyle {
        PathStyle::Posix
    }
}

impl IrohZedRemote {
    pub async fn new(
        connection_options: IrohConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        delegate.set_status(Some("Connecting"), cx);

        let this = gpui_tokio::Tokio::spawn(cx, async move {
            let endpoint = Endpoint::builder()
                .alpns(vec![ZED_ALPN.to_vec()])
                .bind()
                .await?;

            anyhow::Ok(Self {
                options: connection_options,
                endpoint,
                ssh_shell: "todo".into(),
            })
        })?
        .await??;
        Ok(this)
    }

    pub fn ticket(&self) -> ZedIrohTicket {
        let addr = self.endpoint.addr();
        ZedIrohTicket::new(addr)
    }
}

fn handle_rpc_messages(
    endpoint: &Endpoint,
    addr: EndpointAddr,
    incoming_tx: UnboundedSender<Envelope>,
    mut outgoing_rx: UnboundedReceiver<Envelope>,
    mut connection_activity_tx: Sender<()>,
    cx: &mut AsyncApp,
) -> Result<Task<Result<i32>>> {
    log::info!("iroh connecting to {:?}", addr);

    let ep = endpoint.clone();

    let task = gpui_tokio::Tokio::spawn(cx, async move {
        // Open a connection to the accepting endpoint
        let conn = ep.connect(addr, ZED_ALPN).await?;
        // Open a bidirectional QUIC stream
        let (send, recv) = conn.open_bi().await?;
        // Wrap the stream with length-prefixed framing
        let mut codec = LengthDelimitedCodec::builder();
        codec.max_frame_length(MAX_MESSAGE_SIZE);
        let mut write = codec.new_write(send);
        let mut read = codec.new_read(recv);

        log::info!("opened iroh connection");

        let writer_task = tokio::task::spawn({
            let mut connection_activity_tx = connection_activity_tx.clone();
            async move {
                while let Some(outgoing) = outgoing_rx.next().await {
                    log::debug!("sending {:?}", outgoing);
                    let encoded = postcard::to_stdvec(&Message::Envelope(outgoing))
                        .expect("invalid encoding");

                    connection_activity_tx.try_send(()).ok();
                    write.send(Bytes::from(encoded)).await?;
                }
                anyhow::Ok(())
            }
        });

        let reader_task = tokio::task::spawn(async move {
            while let Some(env_data) = read.next().await {
                let data = env_data?;
                let message: Message = postcard::from_bytes(&data)?;

                match message {
                    Message::Envelope(envelope) => {
                        log::debug!("receiving {:?}", envelope);
                        incoming_tx.unbounded_send(envelope).ok();
                        connection_activity_tx.try_send(()).ok();
                    }
                    Message::Log(record) => {
                        record.log(log::logger());
                    }
                }
            }
            anyhow::Ok(())
        });

        anyhow::Ok((writer_task, reader_task, conn))
    })?;

    let task = cx.background_spawn(async move {
        match task.await {
            Ok(Ok((writer_task, reader_task, conn))) => {
                let res = tokio::select! {
                    res = writer_task => {
                        res.context("writer")
                    }
                    res = reader_task => {
                        res.context("reader")
                    }
                };
                log::warn!("exiting iroh conn");
                conn.close(1u32.try_into().unwrap(), b"exit");
                res.map(|_| 0)
            }
            Ok(Err(error)) => Err(error),
            Err(error) => Err(anyhow!(error)),
        }
    });

    Ok(task)
}

#[derive(Serialize, Deserialize)]
struct Variant0EndpointAddr {
    id: EndpointId,
    info: Variant0AddrInfo,
}

#[derive(Serialize, Deserialize)]
struct Variant0AddrInfo {
    addrs: BTreeSet<TransportAddr>,
}

/// A token containing information for establishing a zed remote session via an iroh
/// transport
///
/// Contains
/// - The [`EndpointId`] of the endpoint to connect to (a 32-byte ed25519 public key).
/// - If used, the ['RelayUrl`] of on which the endpoint can be reached.
/// - Any *direct addresses* on which the endpoint might be reachable.
///
/// This allows establishing a connection to the endpoint in most circumstances where it is
/// possible to do so.
///
/// This [`EndpointTicket`] is a single item which can be easily serialized and deserialized and
/// implements the [`Ticket`] trait.  The [`Display`] and [`FromStr`] traits can also be
/// used to round-trip the ticket to string.
///
/// [`EndpointId`]: crate::key::EndpointId
/// [`Display`]: std::fmt::Display
/// [`FromStr`]: std::str::FromStr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZedIrohTicket {
    addr: EndpointAddr,
}

impl Display for ZedIrohTicket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Ticket::serialize(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message<'a> {
    Log(#[serde(borrow)] LogRecord<'a>),
    Envelope(Envelope),
}

/// Wire format for [`EndpointTicket`].
#[derive(Serialize, Deserialize)]
enum TicketWireFormat {
    Variant0(Variant0EndpointTicket),
}

#[derive(Serialize, Deserialize)]
struct Variant0EndpointTicket {
    endpoint: Variant0EndpointAddr,
}

impl Ticket for ZedIrohTicket {
    const KIND: &'static str = "zed";

    fn to_bytes(&self) -> Vec<u8> {
        let data = TicketWireFormat::Variant0(Variant0EndpointTicket {
            endpoint: Variant0EndpointAddr {
                id: self.addr.id,
                info: Variant0AddrInfo {
                    addrs: self.addr.addrs.clone(),
                },
            },
        });
        postcard::to_stdvec(&data).expect("postcard serialization failed")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let res: TicketWireFormat = postcard::from_bytes(bytes)?;
        let TicketWireFormat::Variant0(Variant0EndpointTicket { endpoint }) = res;
        Ok(Self {
            addr: EndpointAddr {
                id: endpoint.id,
                addrs: endpoint.info.addrs,
            },
        })
    }
}

impl FromStr for ZedIrohTicket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        iroh_tickets::Ticket::deserialize(s)
    }
}

impl ZedIrohTicket {
    /// Creates a new ticket.
    pub fn new(addr: EndpointAddr) -> Self {
        Self { addr }
    }

    /// The [`EndpointAddr`] of the provider for this ticket.
    pub fn addr(&self) -> &EndpointAddr {
        &self.addr
    }
}

impl From<EndpointAddr> for ZedIrohTicket {
    /// Creates a ticket from given addressing info.
    fn from(addr: EndpointAddr) -> Self {
        Self { addr }
    }
}

impl From<ZedIrohTicket> for EndpointAddr {
    /// Returns the addressing info from given ticket.
    fn from(ticket: ZedIrohTicket) -> Self {
        ticket.addr
    }
}

impl Serialize for ZedIrohTicket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            let ZedIrohTicket { addr } = self;
            (addr).serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for ZedIrohTicket {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Self::from_str(&s).map_err(serde::de::Error::custom)
        } else {
            let peer = Deserialize::deserialize(deserializer)?;
            Ok(Self::new(peer))
        }
    }
}
