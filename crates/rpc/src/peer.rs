use crate::{ErrorCode, ErrorCodeExt, ErrorExt, RpcError};

use super::{
    proto::{self, AnyTypedEnvelope, EnvelopedMessage, MessageStream, PeerId, RequestMessage},
    Connection,
};
use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    stream::BoxStream,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};
use parking_lot::{Mutex, RwLock};
use serde::{ser::SerializeStruct, Serialize};
use std::{fmt, sync::atomic::Ordering::SeqCst};
use std::{
    future::Future,
    marker::PhantomData,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
    time::Duration,
};
use tracing::instrument;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct ConnectionId {
    pub owner_id: u32,
    pub id: u32,
}

impl Into<PeerId> for ConnectionId {
    fn into(self) -> PeerId {
        PeerId {
            owner_id: self.owner_id,
            id: self.id,
        }
    }
}

impl From<PeerId> for ConnectionId {
    fn from(peer_id: PeerId) -> Self {
        Self {
            owner_id: peer_id.owner_id,
            id: peer_id.id,
        }
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner_id, self.id)
    }
}

pub struct Receipt<T> {
    pub sender_id: ConnectionId,
    pub message_id: u32,
    payload_type: PhantomData<T>,
}

impl<T> Clone for Receipt<T> {
    fn clone(&self) -> Self {
        Self {
            sender_id: self.sender_id,
            message_id: self.message_id,
            payload_type: PhantomData,
        }
    }
}

impl<T> Copy for Receipt<T> {}

#[derive(Clone, Debug)]
pub struct TypedEnvelope<T> {
    pub sender_id: ConnectionId,
    pub original_sender_id: Option<PeerId>,
    pub message_id: u32,
    pub payload: T,
}

impl<T> TypedEnvelope<T> {
    pub fn original_sender_id(&self) -> Result<PeerId> {
        self.original_sender_id
            .ok_or_else(|| anyhow!("missing original_sender_id"))
    }
}

impl<T: RequestMessage> TypedEnvelope<T> {
    pub fn receipt(&self) -> Receipt<T> {
        Receipt {
            sender_id: self.sender_id,
            message_id: self.message_id,
            payload_type: PhantomData,
        }
    }
}

pub struct Peer {
    epoch: AtomicU32,
    pub connections: RwLock<HashMap<ConnectionId, ConnectionState>>,
    next_connection_id: AtomicU32,
}

#[derive(Clone, Serialize)]
pub struct ConnectionState {
    #[serde(skip)]
    outgoing_tx: mpsc::UnboundedSender<proto::Message>,
    next_message_id: Arc<AtomicU32>,
    #[allow(clippy::type_complexity)]
    #[serde(skip)]
    response_channels:
        Arc<Mutex<Option<HashMap<u32, oneshot::Sender<(proto::Envelope, oneshot::Sender<()>)>>>>>,
}

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);
const WRITE_TIMEOUT: Duration = Duration::from_secs(2);
pub const RECEIVE_TIMEOUT: Duration = Duration::from_secs(10);

impl Peer {
    pub fn new(epoch: u32) -> Arc<Self> {
        Arc::new(Self {
            epoch: AtomicU32::new(epoch),
            connections: Default::default(),
            next_connection_id: Default::default(),
        })
    }

    pub fn epoch(&self) -> u32 {
        self.epoch.load(SeqCst)
    }

    #[instrument(skip_all)]
    pub fn add_connection<F, Fut, Out>(
        self: &Arc<Self>,
        connection: Connection,
        create_timer: F,
    ) -> (
        ConnectionId,
        impl Future<Output = anyhow::Result<()>> + Send,
        BoxStream<'static, Box<dyn AnyTypedEnvelope>>,
    )
    where
        F: Send + Fn(Duration) -> Fut,
        Fut: Send + Future<Output = Out>,
        Out: Send,
    {
        // For outgoing messages, use an unbounded channel so that application code
        // can always send messages without yielding. For incoming messages, use a
        // bounded channel so that other peers will receive backpressure if they send
        // messages faster than this peer can process them.
        #[cfg(any(test, feature = "test-support"))]
        const INCOMING_BUFFER_SIZE: usize = 1;
        #[cfg(not(any(test, feature = "test-support")))]
        const INCOMING_BUFFER_SIZE: usize = 64;
        let (mut incoming_tx, incoming_rx) = mpsc::channel(INCOMING_BUFFER_SIZE);
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded();

        let connection_id = ConnectionId {
            owner_id: self.epoch.load(SeqCst),
            id: self.next_connection_id.fetch_add(1, SeqCst),
        };
        let connection_state = ConnectionState {
            outgoing_tx,
            next_message_id: Default::default(),
            response_channels: Arc::new(Mutex::new(Some(Default::default()))),
        };
        let mut writer = MessageStream::new(connection.tx);
        let mut reader = MessageStream::new(connection.rx);

        let this = self.clone();
        let response_channels = connection_state.response_channels.clone();
        let handle_io = async move {
            tracing::trace!(%connection_id, "handle io future: start");

            let _end_connection = util::defer(|| {
                response_channels.lock().take();
                this.connections.write().remove(&connection_id);
                tracing::trace!(%connection_id, "handle io future: end");
            });

            // Send messages on this frequency so the connection isn't closed.
            let keepalive_timer = create_timer(KEEPALIVE_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer);

            // Disconnect if we don't receive messages at least this frequently.
            let receive_timeout = create_timer(RECEIVE_TIMEOUT).fuse();
            futures::pin_mut!(receive_timeout);

            loop {
                tracing::trace!(%connection_id, "outer loop iteration start");
                let read_message = reader.read().fuse();
                futures::pin_mut!(read_message);

                loop {
                    tracing::trace!(%connection_id, "inner loop iteration start");
                    futures::select_biased! {
                        outgoing = outgoing_rx.next().fuse() => match outgoing {
                            Some(outgoing) => {
                                tracing::trace!(%connection_id, "outgoing rpc message: writing");
                                futures::select_biased! {
                                    result = writer.write(outgoing).fuse() => {
                                        tracing::trace!(%connection_id, "outgoing rpc message: done writing");
                                        result.context("failed to write RPC message")?;
                                        tracing::trace!(%connection_id, "keepalive interval: resetting after sending message");
                                        keepalive_timer.set(create_timer(KEEPALIVE_INTERVAL).fuse());
                                    }
                                    _ = create_timer(WRITE_TIMEOUT).fuse() => {
                                        tracing::trace!(%connection_id, "outgoing rpc message: writing timed out");
                                        Err(anyhow!("timed out writing message"))?;
                                    }
                                }
                            }
                            None => {
                                tracing::trace!(%connection_id, "outgoing rpc message: channel closed");
                                return Ok(())
                            },
                        },
                        _ = keepalive_timer => {
                            tracing::trace!(%connection_id, "keepalive interval: pinging");
                            futures::select_biased! {
                                result = writer.write(proto::Message::Ping).fuse() => {
                                    tracing::trace!(%connection_id, "keepalive interval: done pinging");
                                    result.context("failed to send keepalive")?;
                                    tracing::trace!(%connection_id, "keepalive interval: resetting after pinging");
                                    keepalive_timer.set(create_timer(KEEPALIVE_INTERVAL).fuse());
                                }
                                _ = create_timer(WRITE_TIMEOUT).fuse() => {
                                    tracing::trace!(%connection_id, "keepalive interval: pinging timed out");
                                    Err(anyhow!("timed out sending keepalive"))?;
                                }
                            }
                        }
                        incoming = read_message => {
                            let incoming = incoming.context("error reading rpc message from socket")?;
                            tracing::trace!(%connection_id, "incoming rpc message: received");
                            tracing::trace!(%connection_id, "receive timeout: resetting");
                            receive_timeout.set(create_timer(RECEIVE_TIMEOUT).fuse());
                            if let proto::Message::Envelope(incoming) = incoming {
                                tracing::trace!(%connection_id, "incoming rpc message: processing");
                                futures::select_biased! {
                                    result = incoming_tx.send(incoming).fuse() => match result {
                                        Ok(_) => {
                                            tracing::trace!(%connection_id, "incoming rpc message: processed");
                                        }
                                        Err(_) => {
                                            tracing::trace!(%connection_id, "incoming rpc message: channel closed");
                                            return Ok(())
                                        }
                                    },
                                    _ = create_timer(WRITE_TIMEOUT).fuse() => {
                                        tracing::trace!(%connection_id, "incoming rpc message: processing timed out");
                                        Err(anyhow!("timed out processing incoming message"))?
                                    }
                                }
                            }
                            break;
                        },
                        _ = receive_timeout => {
                            tracing::trace!(%connection_id, "receive timeout: delay between messages too long");
                            Err(anyhow!("delay between messages too long"))?
                        }
                    }
                }
            }
        };

        let response_channels = connection_state.response_channels.clone();
        self.connections
            .write()
            .insert(connection_id, connection_state);

        let incoming_rx = incoming_rx.filter_map(move |incoming| {
            let response_channels = response_channels.clone();
            async move {
                let message_id = incoming.id;
                tracing::trace!(?incoming, "incoming message future: start");
                let _end = util::defer(move || {
                    tracing::trace!(%connection_id, message_id, "incoming message future: end");
                });

                if let Some(responding_to) = incoming.responding_to {
                    tracing::trace!(
                        %connection_id,
                        message_id,
                        responding_to,
                        "incoming response: received"
                    );
                    let channel = response_channels.lock().as_mut()?.remove(&responding_to);
                    if let Some(tx) = channel {
                        let requester_resumed = oneshot::channel();
                        if let Err(error) = tx.send((incoming, requester_resumed.0)) {
                            tracing::trace!(
                                %connection_id,
                                message_id,
                                responding_to = responding_to,
                                ?error,
                                "incoming response: request future dropped",
                            );
                        }

                        tracing::trace!(
                            %connection_id,
                            message_id,
                            responding_to,
                            "incoming response: waiting to resume requester"
                        );
                        let _ = requester_resumed.1.await;
                        tracing::trace!(
                            %connection_id,
                            message_id,
                            responding_to,
                            "incoming response: requester resumed"
                        );
                    } else {
                        let message_type = proto::build_typed_envelope(connection_id, incoming)
                            .map(|p| p.payload_type_name());
                        tracing::warn!(
                            %connection_id,
                            message_id,
                            responding_to,
                            message_type,
                            "incoming response: unknown request"
                        );
                    }

                    None
                } else {
                    tracing::trace!(%connection_id, message_id, "incoming message: received");
                    proto::build_typed_envelope(connection_id, incoming).or_else(|| {
                        tracing::error!(
                            %connection_id,
                            message_id,
                            "unable to construct a typed envelope"
                        );
                        None
                    })
                }
            }
        });
        (connection_id, handle_io, incoming_rx.boxed())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn add_test_connection(
        self: &Arc<Self>,
        connection: Connection,
        executor: gpui::BackgroundExecutor,
    ) -> (
        ConnectionId,
        impl Future<Output = anyhow::Result<()>> + Send,
        BoxStream<'static, Box<dyn AnyTypedEnvelope>>,
    ) {
        let executor = executor.clone();
        self.add_connection(connection, move |duration| executor.timer(duration))
    }

    pub fn disconnect(&self, connection_id: ConnectionId) {
        self.connections.write().remove(&connection_id);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn reset(&self, epoch: u32) {
        self.next_connection_id.store(0, SeqCst);
        self.epoch.store(epoch, SeqCst);
    }

    pub fn teardown(&self) {
        self.connections.write().clear();
    }

    pub fn request<T: RequestMessage>(
        &self,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(None, receiver_id, request)
            .map_ok(|envelope| envelope.payload)
    }

    pub fn request_envelope<T: RequestMessage>(
        &self,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<TypedEnvelope<T::Response>>> {
        self.request_internal(None, receiver_id, request)
    }

    pub fn forward_request<T: RequestMessage>(
        &self,
        sender_id: ConnectionId,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(Some(sender_id), receiver_id, request)
            .map_ok(|envelope| envelope.payload)
    }

    pub fn request_internal<T: RequestMessage>(
        &self,
        original_sender_id: Option<ConnectionId>,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<TypedEnvelope<T::Response>>> {
        let (tx, rx) = oneshot::channel();
        let send = self.connection_state(receiver_id).and_then(|connection| {
            let message_id = connection.next_message_id.fetch_add(1, SeqCst);
            connection
                .response_channels
                .lock()
                .as_mut()
                .ok_or_else(|| anyhow!("connection was closed"))?
                .insert(message_id, tx);
            connection
                .outgoing_tx
                .unbounded_send(proto::Message::Envelope(request.into_envelope(
                    message_id,
                    None,
                    original_sender_id.map(Into::into),
                )))
                .map_err(|_| anyhow!("connection was closed"))?;
            Ok(())
        });
        async move {
            send?;
            let (response, _barrier) = rx.await.map_err(|_| anyhow!("connection was closed"))?;

            if let Some(proto::envelope::Payload::Error(error)) = &response.payload {
                Err(RpcError::from_proto(&error, T::NAME))
            } else {
                Ok(TypedEnvelope {
                    message_id: response.id,
                    sender_id: receiver_id,
                    original_sender_id: response.original_sender_id,
                    payload: T::Response::from_envelope(response)
                        .ok_or_else(|| anyhow!("received response of the wrong type"))?,
                })
            }
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, receiver_id: ConnectionId, message: T) -> Result<()> {
        let connection = self.connection_state(receiver_id)?;
        let message_id = connection
            .next_message_id
            .fetch_add(1, atomic::Ordering::SeqCst);
        connection
            .outgoing_tx
            .unbounded_send(proto::Message::Envelope(
                message.into_envelope(message_id, None, None),
            ))?;
        Ok(())
    }

    pub fn forward_send<T: EnvelopedMessage>(
        &self,
        sender_id: ConnectionId,
        receiver_id: ConnectionId,
        message: T,
    ) -> Result<()> {
        let connection = self.connection_state(receiver_id)?;
        let message_id = connection
            .next_message_id
            .fetch_add(1, atomic::Ordering::SeqCst);
        connection
            .outgoing_tx
            .unbounded_send(proto::Message::Envelope(message.into_envelope(
                message_id,
                None,
                Some(sender_id.into()),
            )))?;
        Ok(())
    }

    pub fn respond<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        response: T::Response,
    ) -> Result<()> {
        let connection = self.connection_state(receipt.sender_id)?;
        let message_id = connection
            .next_message_id
            .fetch_add(1, atomic::Ordering::SeqCst);
        connection
            .outgoing_tx
            .unbounded_send(proto::Message::Envelope(response.into_envelope(
                message_id,
                Some(receipt.message_id),
                None,
            )))?;
        Ok(())
    }

    pub fn respond_with_error<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        response: proto::Error,
    ) -> Result<()> {
        let connection = self.connection_state(receipt.sender_id)?;
        let message_id = connection
            .next_message_id
            .fetch_add(1, atomic::Ordering::SeqCst);
        connection
            .outgoing_tx
            .unbounded_send(proto::Message::Envelope(response.into_envelope(
                message_id,
                Some(receipt.message_id),
                None,
            )))?;
        Ok(())
    }

    pub fn respond_with_unhandled_message(
        &self,
        envelope: Box<dyn AnyTypedEnvelope>,
    ) -> Result<()> {
        let connection = self.connection_state(envelope.sender_id())?;
        let response = ErrorCode::Internal
            .message(format!(
                "message {} was not handled",
                envelope.payload_type_name()
            ))
            .to_proto();
        let message_id = connection
            .next_message_id
            .fetch_add(1, atomic::Ordering::SeqCst);
        connection
            .outgoing_tx
            .unbounded_send(proto::Message::Envelope(response.into_envelope(
                message_id,
                Some(envelope.message_id()),
                None,
            )))?;
        Ok(())
    }

    fn connection_state(&self, connection_id: ConnectionId) -> Result<ConnectionState> {
        let connections = self.connections.read();
        let connection = connections
            .get(&connection_id)
            .ok_or_else(|| anyhow!("no such connection: {}", connection_id))?;
        Ok(connection.clone())
    }
}

impl Serialize for Peer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Peer", 2)?;
        state.serialize_field("connections", &*self.connections.read())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TypedEnvelope;
    use async_tungstenite::tungstenite::Message as WebSocketMessage;
    use gpui::TestAppContext;

    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test(iterations = 50)]
    async fn test_request_response(cx: &mut TestAppContext) {
        init_logger();

        let executor = cx.executor();

        // create 2 clients connected to 1 server
        let server = Peer::new(0);
        let client1 = Peer::new(0);
        let client2 = Peer::new(0);

        let (client1_to_server_conn, server_to_client_1_conn, _kill) =
            Connection::in_memory(cx.executor());
        let (client1_conn_id, io_task1, client1_incoming) =
            client1.add_test_connection(client1_to_server_conn, cx.executor());
        let (_, io_task2, server_incoming1) =
            server.add_test_connection(server_to_client_1_conn, cx.executor());

        let (client2_to_server_conn, server_to_client_2_conn, _kill) =
            Connection::in_memory(cx.executor());
        let (client2_conn_id, io_task3, client2_incoming) =
            client2.add_test_connection(client2_to_server_conn, cx.executor());
        let (_, io_task4, server_incoming2) =
            server.add_test_connection(server_to_client_2_conn, cx.executor());

        executor.spawn(io_task1).detach();
        executor.spawn(io_task2).detach();
        executor.spawn(io_task3).detach();
        executor.spawn(io_task4).detach();
        executor
            .spawn(handle_messages(server_incoming1, server.clone()))
            .detach();
        executor
            .spawn(handle_messages(client1_incoming, client1.clone()))
            .detach();
        executor
            .spawn(handle_messages(server_incoming2, server.clone()))
            .detach();
        executor
            .spawn(handle_messages(client2_incoming, client2.clone()))
            .detach();

        assert_eq!(
            client1
                .request(client1_conn_id, proto::Ping {},)
                .await
                .unwrap(),
            proto::Ack {}
        );

        assert_eq!(
            client2
                .request(client2_conn_id, proto::Ping {},)
                .await
                .unwrap(),
            proto::Ack {}
        );

        assert_eq!(
            client1
                .request(client1_conn_id, proto::Test { id: 1 },)
                .await
                .unwrap(),
            proto::Test { id: 1 }
        );

        assert_eq!(
            client2
                .request(client2_conn_id, proto::Test { id: 2 })
                .await
                .unwrap(),
            proto::Test { id: 2 }
        );

        client1.disconnect(client1_conn_id);
        client2.disconnect(client1_conn_id);

        async fn handle_messages(
            mut messages: BoxStream<'static, Box<dyn AnyTypedEnvelope>>,
            peer: Arc<Peer>,
        ) -> Result<()> {
            while let Some(envelope) = messages.next().await {
                let envelope = envelope.into_any();
                if let Some(envelope) = envelope.downcast_ref::<TypedEnvelope<proto::Ping>>() {
                    let receipt = envelope.receipt();
                    peer.respond(receipt, proto::Ack {})?
                } else if let Some(envelope) = envelope.downcast_ref::<TypedEnvelope<proto::Test>>()
                {
                    peer.respond(envelope.receipt(), envelope.payload.clone())?
                } else {
                    panic!("unknown message type");
                }
            }

            Ok(())
        }
    }

    #[gpui::test(iterations = 50)]
    async fn test_order_of_response_and_incoming(cx: &mut TestAppContext) {
        let executor = cx.executor();
        let server = Peer::new(0);
        let client = Peer::new(0);

        let (client_to_server_conn, server_to_client_conn, _kill) =
            Connection::in_memory(executor.clone());
        let (client_to_server_conn_id, io_task1, mut client_incoming) =
            client.add_test_connection(client_to_server_conn, executor.clone());

        let (server_to_client_conn_id, io_task2, mut server_incoming) =
            server.add_test_connection(server_to_client_conn, executor.clone());

        executor.spawn(io_task1).detach();
        executor.spawn(io_task2).detach();

        executor
            .spawn(async move {
                let future = server_incoming.next().await;
                let request = future
                    .unwrap()
                    .into_any()
                    .downcast::<TypedEnvelope<proto::Ping>>()
                    .unwrap();

                server
                    .send(
                        server_to_client_conn_id,
                        ErrorCode::Internal
                            .message("message 1".to_string())
                            .to_proto(),
                    )
                    .unwrap();
                server
                    .send(
                        server_to_client_conn_id,
                        ErrorCode::Internal
                            .message("message 2".to_string())
                            .to_proto(),
                    )
                    .unwrap();
                server.respond(request.receipt(), proto::Ack {}).unwrap();

                // Prevent the connection from being dropped
                server_incoming.next().await;
            })
            .detach();

        let events = Arc::new(Mutex::new(Vec::new()));

        let response = client.request(client_to_server_conn_id, proto::Ping {});
        let response_task = executor.spawn({
            let events = events.clone();
            async move {
                response.await.unwrap();
                events.lock().push("response".to_string());
            }
        });

        executor
            .spawn({
                let events = events.clone();
                async move {
                    let incoming1 = client_incoming
                        .next()
                        .await
                        .unwrap()
                        .into_any()
                        .downcast::<TypedEnvelope<proto::Error>>()
                        .unwrap();
                    events.lock().push(incoming1.payload.message);
                    let incoming2 = client_incoming
                        .next()
                        .await
                        .unwrap()
                        .into_any()
                        .downcast::<TypedEnvelope<proto::Error>>()
                        .unwrap();
                    events.lock().push(incoming2.payload.message);

                    // Prevent the connection from being dropped
                    client_incoming.next().await;
                }
            })
            .detach();

        response_task.await;
        assert_eq!(
            &*events.lock(),
            &[
                "message 1".to_string(),
                "message 2".to_string(),
                "response".to_string()
            ]
        );
    }

    #[gpui::test(iterations = 50)]
    async fn test_dropping_request_before_completion(cx: &mut TestAppContext) {
        let executor = cx.executor();
        let server = Peer::new(0);
        let client = Peer::new(0);

        let (client_to_server_conn, server_to_client_conn, _kill) =
            Connection::in_memory(cx.executor());
        let (client_to_server_conn_id, io_task1, mut client_incoming) =
            client.add_test_connection(client_to_server_conn, cx.executor());
        let (server_to_client_conn_id, io_task2, mut server_incoming) =
            server.add_test_connection(server_to_client_conn, cx.executor());

        executor.spawn(io_task1).detach();
        executor.spawn(io_task2).detach();

        executor
            .spawn(async move {
                let request1 = server_incoming
                    .next()
                    .await
                    .unwrap()
                    .into_any()
                    .downcast::<TypedEnvelope<proto::Ping>>()
                    .unwrap();
                let request2 = server_incoming
                    .next()
                    .await
                    .unwrap()
                    .into_any()
                    .downcast::<TypedEnvelope<proto::Ping>>()
                    .unwrap();

                server
                    .send(
                        server_to_client_conn_id,
                        ErrorCode::Internal
                            .message("message 1".to_string())
                            .to_proto(),
                    )
                    .unwrap();
                server
                    .send(
                        server_to_client_conn_id,
                        ErrorCode::Internal
                            .message("message 2".to_string())
                            .to_proto(),
                    )
                    .unwrap();
                server.respond(request1.receipt(), proto::Ack {}).unwrap();
                server.respond(request2.receipt(), proto::Ack {}).unwrap();

                // Prevent the connection from being dropped
                server_incoming.next().await;
            })
            .detach();

        let events = Arc::new(Mutex::new(Vec::new()));

        let request1 = client.request(client_to_server_conn_id, proto::Ping {});
        let request1_task = executor.spawn(request1);
        let request2 = client.request(client_to_server_conn_id, proto::Ping {});
        let request2_task = executor.spawn({
            let events = events.clone();
            async move {
                request2.await.unwrap();
                events.lock().push("response 2".to_string());
            }
        });

        executor
            .spawn({
                let events = events.clone();
                async move {
                    let incoming1 = client_incoming
                        .next()
                        .await
                        .unwrap()
                        .into_any()
                        .downcast::<TypedEnvelope<proto::Error>>()
                        .unwrap();
                    events.lock().push(incoming1.payload.message);
                    let incoming2 = client_incoming
                        .next()
                        .await
                        .unwrap()
                        .into_any()
                        .downcast::<TypedEnvelope<proto::Error>>()
                        .unwrap();
                    events.lock().push(incoming2.payload.message);

                    // Prevent the connection from being dropped
                    client_incoming.next().await;
                }
            })
            .detach();

        // Allow the request to make some progress before dropping it.
        cx.executor().simulate_random_delay().await;
        drop(request1_task);

        request2_task.await;
        assert_eq!(
            &*events.lock(),
            &[
                "message 1".to_string(),
                "message 2".to_string(),
                "response 2".to_string()
            ]
        );
    }

    #[gpui::test(iterations = 50)]
    async fn test_disconnect(cx: &mut TestAppContext) {
        let executor = cx.executor();

        let (client_conn, mut server_conn, _kill) = Connection::in_memory(executor.clone());

        let client = Peer::new(0);
        let (connection_id, io_handler, mut incoming) =
            client.add_test_connection(client_conn, executor.clone());

        let (io_ended_tx, io_ended_rx) = oneshot::channel();
        executor
            .spawn(async move {
                io_handler.await.ok();
                io_ended_tx.send(()).unwrap();
            })
            .detach();

        let (messages_ended_tx, messages_ended_rx) = oneshot::channel();
        executor
            .spawn(async move {
                incoming.next().await;
                messages_ended_tx.send(()).unwrap();
            })
            .detach();

        client.disconnect(connection_id);

        let _ = io_ended_rx.await;
        let _ = messages_ended_rx.await;
        assert!(server_conn
            .send(WebSocketMessage::Binary(vec![]))
            .await
            .is_err());
    }

    #[gpui::test(iterations = 50)]
    async fn test_io_error(cx: &mut TestAppContext) {
        let executor = cx.executor();
        let (client_conn, mut server_conn, _kill) = Connection::in_memory(executor.clone());

        let client = Peer::new(0);
        let (connection_id, io_handler, mut incoming) =
            client.add_test_connection(client_conn, executor.clone());
        executor.spawn(io_handler).detach();
        executor
            .spawn(async move { incoming.next().await })
            .detach();

        let response = executor.spawn(client.request(connection_id, proto::Ping {}));
        let _request = server_conn.rx.next().await.unwrap().unwrap();

        drop(server_conn);
        assert_eq!(
            response.await.unwrap_err().to_string(),
            "connection was closed"
        );
    }
}
