use crate::{
    RemoteClientDelegate,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    SinkExt, StreamExt as _, TryStreamExt,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
    lock::Mutex,
};
use gpui::{App, AppContext as _, AsyncApp, Task};
use iroh::{
    Endpoint, NodeAddr, NodeId, RelayUrl, Watcher,
    endpoint::{Connection, RecvStream, SendStream},
    protocol::{AcceptError, ProtocolHandler, Router},
};
use iroh_base::ticket::{self, ParseError, Ticket};
use rpc::proto::Envelope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{self, Display},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};
use tokio_util::{
    bytes::Bytes,
    codec::{FramedRead, FramedWrite, LengthDelimitedCodec},
};
use util::paths::{PathStyle, RemotePathBuf};

/// The ALPN, or application-layer protocol negotiation, is exchanged in the
/// connection handshake, and the connection is aborted unless both nodes pass
/// the same bytestring.
pub const ZED_ALPN: &[u8] = b"iroh/zed/remote/0";

// max length of an RPC message in bytes
const MAX_MESSAGE_SIZE: usize = 10000;

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

impl IrohConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let ticket = input.parse()?;
        Ok(Self {
            ticket,
            port_forwards: None,
            nickname: None,
        })
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

    fn build_command(
        &self,
        _input_program: Option<String>,
        _input_args: &[String],
        input_env: &HashMap<String, String>,
        _working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
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
        cx.background_spawn(async move {
            // TODO(b5): no-op for now?
            Ok(())
        })
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
        let addr = self.options.ticket.node.clone();
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
                .discovery_n0()
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

    pub async fn ticket(&self) -> ZedIrohTicket {
        let addr = self.endpoint.node_addr().initialized().await;
        ZedIrohTicket::new(addr)
    }
}

fn handle_rpc_messages(
    endpoint: &Endpoint,
    addr: NodeAddr,
    incoming_tx: UnboundedSender<Envelope>,
    mut outgoing_rx: UnboundedReceiver<Envelope>,
    mut connection_activity_tx: Sender<()>,
    cx: &mut AsyncApp,
) -> Result<Task<Result<i32>>> {
    log::info!("iroh connecting to {:?}", addr);

    let ep = endpoint.clone();

    let task = gpui_tokio::Tokio::spawn(cx, async move {
        // Open a connection to the accepting node
        let conn = ep.connect(addr, ZED_ALPN).await?;
        // Open a bidirectional QUIC stream
        let bi_stream = conn.open_bi().await?;
        // Wrap the stream with length-prefixed framing
        let mut stream = FramedBiStream::new(bi_stream);

        log::info!("opened iroh connection");

        tokio::task::spawn({
            let mut connection_activity_tx = connection_activity_tx.clone();
            async move {
                while let Some(outgoing) = outgoing_rx.next().await {
                    log::debug!("sending {:?}", outgoing);
                    let encoded = postcard::to_stdvec(&outgoing).expect("invalid encoding");
                    connection_activity_tx.try_send(()).ok();
                    if let Err(error) = stream.write.send(Bytes::from(encoded)).await {
                        log::error!("failed to send rpc message: {:?}", error);
                    }
                }
            }
        });

        tokio::task::spawn(async move {
            while let Some(env_data) = stream.read.next().await {
                connection_activity_tx.try_send(()).ok();
                match env_data {
                    Ok(data) => match postcard::from_bytes(&data) {
                        Ok(envelope) => {
                            log::debug!("receiving {:?}", envelope);
                            if let Err(error) = incoming_tx.unbounded_send(envelope) {
                                log::error!("failed to propagate rpc message: {:?}", error);
                            }
                        }
                        Err(error) => {
                            log::error!("invalid rpc message: {:?}", error);
                        }
                    },
                    Err(error) => {
                        log::error!("invalid incoming message: {:?}", error);
                    }
                }
            }
        });

        anyhow::Ok(0)
    })?;

    let task = cx.background_spawn(async move {
        let result = task.await??;
        anyhow::Ok(result)
    });

    Ok(task)
}

#[derive(Debug, Clone)]
pub struct IrohZedListener {
    endpoint: Endpoint,
    router: Router,
}

impl IrohZedListener {
    pub async fn shutdown(self) {
        if let Err(err) = self.router.shutdown().await {
            log::warn!("failed to shutdown iroh: {:?}", err);
        }
    }

    pub async fn accept(
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
    ) -> Result<Self> {
        let endpoint = Endpoint::builder()
            .discovery_n0()
            .alpns(vec![ZED_ALPN.to_vec()])
            .bind()
            .await?;

        let router = Router::builder(endpoint.clone())
            .accept(
                ZED_ALPN,
                IrohZedProtocolHandler::new(incoming_tx, outgoing_rx),
            )
            .spawn();

        Ok(Self { endpoint, router })
    }

    pub async fn ticket(&self) -> ZedIrohTicket {
        let addr = self.endpoint.node_addr().initialized().await;
        ZedIrohTicket::new(addr)
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }
}

#[derive(Debug)]
pub struct IrohZedProtocolHandler {
    incoming_tx: UnboundedSender<Envelope>,
    outgoing_rx: Arc<Mutex<UnboundedReceiver<Envelope>>>,
}

impl IrohZedProtocolHandler {
    fn new(
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
    ) -> Self {
        Self {
            incoming_tx,
            outgoing_rx: Arc::new(Mutex::new(outgoing_rx)),
        }
    }
}

impl ProtocolHandler for IrohZedProtocolHandler {
    /// The `accept` method is called for each incoming connection for our ALPN.
    ///
    /// The returned future runs on a newly spawned tokio task, so it can run as long as
    /// the connection lasts.
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        // Our protocol is a simple request-response protocol, so we expect the
        // connecting peer to open a single bi-directional stream.
        let (send, recv) = connection.accept_bi().await?;
        let mut codec = LengthDelimitedCodec::builder();
        codec.max_frame_length(MAX_MESSAGE_SIZE);
        let mut write = codec.new_write(send);
        let mut read = codec.new_read(recv);

        let outgoing_rx = self.outgoing_rx.clone();
        tokio::task::spawn(async move {
            while let Some(outgoing_message) = outgoing_rx.lock().await.next().await {
                let encoded = postcard::to_stdvec(&outgoing_message).expect("invalid encoding");

                if let Err(error) = write.send(Bytes::from(encoded)).await {
                    log::error!("failed to write outgoing message: {:?}", error);
                }
            }
        });

        while let Some(encoded_env) = read.try_next().await? {
            match postcard::from_bytes(&encoded_env) {
                Ok(envelope) => {
                    log::info!("received message {:?}", envelope);
                    if let Err(error) = self.incoming_tx.unbounded_send(envelope) {
                        log::error!("failed to send message to application: {error:?}. exiting.");
                        break;
                    }
                }
                Err(error) => {
                    log::error!("received in valid message: {error:?}.");
                }
            }
        }

        // Wait until the remote closes the connection, which it does once it
        // received the response.
        connection.closed().await;

        Ok(())
    }
}

pub struct FramedBiStream {
    pub write: FramedWrite<SendStream, LengthDelimitedCodec>,
    pub read: FramedRead<RecvStream, LengthDelimitedCodec>,
}

impl FramedBiStream {
    pub fn new((send, recv): (SendStream, RecvStream)) -> Self {
        let mut codec = LengthDelimitedCodec::builder();
        codec.max_frame_length(MAX_MESSAGE_SIZE);
        Self {
            write: codec.new_write(send),
            read: codec.new_read(recv),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Variant0NodeAddr {
    node_id: NodeId,
    info: Variant0AddrInfo,
}

#[derive(Serialize, Deserialize)]
struct Variant0AddrInfo {
    relay_url: Option<RelayUrl>,
    direct_addresses: BTreeSet<SocketAddr>,
}

/// A token containing information for establishing a zed remote session via an iroh
/// transport
///
/// Contains
/// - The [`NodeId`] of the node to connect to (a 32-byte ed25519 public key).
/// - If used, the ['RelayUrl`] of on which the node can be reached.
/// - Any *direct addresses* on which the node might be reachable.
///
/// This allows establishing a connection to the node in most circumstances where it is
/// possible to do so.
///
/// This [`NodeTicket`] is a single item which can be easily serialized and deserialized and
/// implements the [`Ticket`] trait.  The [`Display`] and [`FromStr`] traits can also be
/// used to round-trip the ticket to string.
///
/// [`NodeId`]: crate::key::NodeId
/// [`Display`]: std::fmt::Display
/// [`FromStr`]: std::str::FromStr
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZedIrohTicket {
    node: NodeAddr,
}

impl Display for ZedIrohTicket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Ticket::serialize(self))
    }
}

/// Wire format for [`NodeTicket`].
#[derive(Serialize, Deserialize)]
enum TicketWireFormat {
    Variant0(Variant0NodeTicket),
}

#[derive(Serialize, Deserialize)]
struct Variant0NodeTicket {
    node: Variant0NodeAddr,
}

impl Ticket for ZedIrohTicket {
    const KIND: &'static str = "zed";

    fn to_bytes(&self) -> Vec<u8> {
        let data = TicketWireFormat::Variant0(Variant0NodeTicket {
            node: Variant0NodeAddr {
                node_id: self.node.node_id,
                info: Variant0AddrInfo {
                    relay_url: self.node.relay_url.clone(),
                    direct_addresses: self.node.direct_addresses.clone(),
                },
            },
        });
        postcard::to_stdvec(&data).expect("postcard serialization failed")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let res: TicketWireFormat = postcard::from_bytes(bytes)?;
        let TicketWireFormat::Variant0(Variant0NodeTicket { node }) = res;
        Ok(Self {
            node: NodeAddr {
                node_id: node.node_id,
                relay_url: node.info.relay_url,
                direct_addresses: node.info.direct_addresses,
            },
        })
    }
}

impl FromStr for ZedIrohTicket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ticket::Ticket::deserialize(s)
    }
}

impl ZedIrohTicket {
    /// Creates a new ticket.
    pub fn new(node: NodeAddr) -> Self {
        Self { node }
    }

    /// The [`NodeAddr`] of the provider for this ticket.
    pub fn node_addr(&self) -> &NodeAddr {
        &self.node
    }
}

impl From<NodeAddr> for ZedIrohTicket {
    /// Creates a ticket from given addressing info.
    fn from(addr: NodeAddr) -> Self {
        Self { node: addr }
    }
}

impl From<ZedIrohTicket> for NodeAddr {
    /// Returns the addressing info from given ticket.
    fn from(ticket: ZedIrohTicket) -> Self {
        ticket.node
    }
}

impl Serialize for ZedIrohTicket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            let ZedIrohTicket { node } = self;
            (node).serialize(serializer)
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
