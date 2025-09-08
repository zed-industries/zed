use crate::{
    RemoteClientDelegate,
    remote_client::{CommandTemplate, RemoteConnection, RemoteConnectionOptions},
};

use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    SinkExt, StreamExt as _, TryStreamExt,
    channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
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
    bytes::BytesMut,
    codec::{FramedRead, FramedWrite, LengthDelimitedCodec},
};
use util::paths::{PathStyle, RemotePathBuf};

/// The ALPN, or application-layer protocol negotiation, is exchanged in the
/// connection handshake, and the connection is aborted unless both nodes pass
/// the same bytestring.
const ZED_ALPN: &[u8] = b"iroh/zed/remote/0";

// max length of an RPC message in bytes
const MAX_MESSAGE_SIZE: usize = 10000;

#[derive(Debug, Clone)]
pub struct IrohZedRemote {
    options: IrohConnectionOptions,
    endpoint: Endpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrohConnectionOptions {
    pub ticket: ZedIrohTicket,
    pub port_forwards: Option<Vec<IrohPortForwardOption>>,
    pub nickname: Option<String>,
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
        todo!();
        // self.ssh_shell.clone()
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

        self.handle_rpc_messages(addr, incoming_tx, outgoing_rx, connection_activity_tx, cx)
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

        let endpoint = Endpoint::builder()
            .discovery_n0()
            .alpns(vec![ZED_ALPN.to_vec()])
            .bind()
            .await?;

        Ok(Self {
            options: connection_options,
            endpoint,
        })
    }

    // TODO(b5) - break providing out into a separate struct.
    // it should be defined here & consumed in remote_server
    pub async fn provide(&self) -> Router {
        Router::builder(self.endpoint.clone())
            .accept(ZED_ALPN, self.clone())
            .spawn()
    }

    pub async fn ticket(&self) -> ZedIrohTicket {
        let addr = self.endpoint.node_addr().initialized().await;
        ZedIrohTicket::new(addr)
    }

    fn handle_rpc_messages(
        &self,
        addr: NodeAddr,
        mut incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        mut connection_activity_tx: Sender<()>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        let ep = self.endpoint.clone();

        // TODO (b5) - I'm using tokio tasks here because it seems
        // `cx.background_spawn` is intentionally not nestable, but we need an
        // async block to construct the connection. I'm guessing there's a
        // better way to do this.
        cx.background_spawn(async move {
            // Open a connection to the accepting node
            let conn = ep.connect(addr, ZED_ALPN).await?;
            // Open a bidirectional QUIC stream
            let bi_stream = conn.open_bi().await?;
            // Wrap the stream with length-prefixed framing
            let mut stream = FramedBiStream::new(bi_stream);

            tokio::spawn({
                let mut connection_activity_tx = connection_activity_tx.clone();
                async move {
                    while let Some(outgoing) = outgoing_rx.next().await {
                        // TODO(b5): don't swallow errors
                        let encoded = postcard::to_extend(&outgoing, BytesMut::new())
                            .unwrap()
                            .freeze();
                        connection_activity_tx.try_send(()).ok();
                        stream.write.send(encoded).await.unwrap();
                    }
                }
            });

            tokio::spawn({
                let mut connection_activity_tx = connection_activity_tx.clone();
                async move {
                    while let Some(env_data) = stream.read.next().await {
                        // TODO(b5): don't swallow errors
                        let env_data = env_data.unwrap();
                        let decoded: Envelope = postcard::from_bytes(&env_data).unwrap();
                        connection_activity_tx.try_send(()).ok();
                        incoming_tx.unbounded_send(decoded).ok();
                    }
                }
            });

            anyhow::Ok(0)
        })
    }
}

impl ProtocolHandler for IrohZedRemote {
    /// The `accept` method is called for each incoming connection for our ALPN.
    ///
    /// The returned future runs on a newly spawned tokio task, so it can run as long as
    /// the connection lasts.
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        // Our protocol is a simple request-response protocol, so we expect the
        // connecting peer to open a single bi-directional stream.
        let bi_stream = connection.accept_bi().await?;
        let mut stream = FramedBiStream::new(bi_stream);

        while let Some(encoded_env) = stream.read.try_next().await? {
            let msg: Envelope =
                postcard::from_bytes(&encoded_env).map_err(AcceptError::from_err)?;

            log::info!("received message {:?}", msg);
            // TODO(b5) - This needs to be wired up on the provide side.
            // It's likely easiest to do a separate server struct with fields for
            // the corresponding send & receive message pipes
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
        write!(f, "{}", self.to_string())
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

// #[cfg(test)]
// mod tests {
//     use std::net::{Ipv4Addr, SocketAddr};

//     use data_encoding::HEXLOWER;

//     use super::*;
//     use iroh::{PublicKey, SecretKey};

//     fn make_ticket() -> ZedIrohTicket {
//         let peer = SecretKey::generate(&mut rand::thread_rng()).public();
//         let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 1234));
//         let relay_url = None;
//         ZedIrohTicket {
//             node: NodeAddr::from_parts(peer, relay_url, [addr]),
//         }
//     }

//     #[test]
//     fn test_ticket_postcard() {
//         let ticket = make_ticket();
//         let bytes = postcard::to_stdvec(&ticket).unwrap();
//         let ticket2: ZedIrohTicket = postcard::from_bytes(&bytes).unwrap();
//         assert_eq!(ticket2, ticket);
//     }

//     #[test]
//     fn test_ticket_json() {
//         let ticket = make_ticket();
//         let json = serde_json::to_string(&ticket).unwrap();
//         let ticket2: ZedIrohTicket = serde_json::from_str(&json).unwrap();
//         assert_eq!(ticket2, ticket);
//     }

//     #[test]
//     fn test_ticket_base32() {
//         let node_id =
//             PublicKey::from_str("ae58ff8833241ac82d6ff7611046ed67b5072d142c588d0063e942d9a75502b6")
//                 .unwrap();

//         let ticket = ZedIrohTicket {
//             node: NodeAddr::from_parts(
//                 node_id,
//                 Some("http://derp.me./".parse().unwrap()),
//                 ["127.0.0.1:1024".parse().unwrap()],
//             ),
//         };
//         let base32 = data_encoding::BASE32_NOPAD
//             .decode(
//                 ticket
//                     .to_string()
//                     .strip_prefix("node")
//                     .unwrap()
//                     .to_ascii_uppercase()
//                     .as_bytes(),
//             )
//             .unwrap();
//         let expected = [
//             // variant
//             "00",
//             // node id, 32 bytes, see above
//             "ae58ff8833241ac82d6ff7611046ed67b5072d142c588d0063e942d9a75502b6",
//             // relay url present
//             "01",
//             // relay url, 16 bytes, see above
//             "10",
//             "687474703a2f2f646572702e6d652e2f",
//             // one direct address
//             "01",
//             // ipv4
//             "00",
//             // address, see above
//             "7f0000018008",
//         ];
//         let expected = HEXLOWER.decode(expected.concat().as_bytes()).unwrap();
//         assert_eq!(base32, expected);
//     }
// }
