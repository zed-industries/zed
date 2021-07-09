use crate::proto::{self, EnvelopedMessage, MessageStream, RequestMessage};
use anyhow::{anyhow, Context, Result};
use async_lock::{Mutex, RwLock};
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{
    future::BoxFuture,
    stream::{SplitSink, SplitStream},
    FutureExt, StreamExt,
};
use postage::{
    mpsc,
    prelude::{Sink, Stream},
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt,
    future::Future,
    marker::PhantomData,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ConnectionId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PeerId(pub u32);

type MessageHandler = Box<
    dyn Send + Sync + Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<BoxFuture<bool>>,
>;

pub struct Receipt<T> {
    sender_id: ConnectionId,
    message_id: u32,
    payload_type: PhantomData<T>,
}

pub struct TypedEnvelope<T> {
    pub sender_id: ConnectionId,
    original_sender_id: Option<PeerId>,
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
    connections: RwLock<HashMap<ConnectionId, Connection>>,
    message_handlers: RwLock<Vec<MessageHandler>>,
    handler_types: Mutex<HashSet<TypeId>>,
    next_connection_id: AtomicU32,
}

#[derive(Clone)]
struct Connection {
    outgoing_tx: mpsc::Sender<proto::Envelope>,
    next_message_id: Arc<AtomicU32>,
    response_channels: ResponseChannels,
}

pub struct ConnectionHandler<W, R> {
    peer: Arc<Peer>,
    connection_id: ConnectionId,
    response_channels: ResponseChannels,
    outgoing_rx: mpsc::Receiver<proto::Envelope>,
    writer: MessageStream<W>,
    reader: MessageStream<R>,
}

type ResponseChannels = Arc<Mutex<HashMap<u32, mpsc::Sender<proto::Envelope>>>>;

impl Peer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            connections: Default::default(),
            message_handlers: Default::default(),
            handler_types: Default::default(),
            next_connection_id: Default::default(),
        })
    }

    pub async fn add_message_handler<T: EnvelopedMessage>(
        &self,
    ) -> mpsc::Receiver<TypedEnvelope<T>> {
        if !self.handler_types.lock().await.insert(TypeId::of::<T>()) {
            panic!("duplicate handler type");
        }

        let (tx, rx) = mpsc::channel(256);
        self.message_handlers
            .write()
            .await
            .push(Box::new(move |envelope, connection_id| {
                if envelope.as_ref().map_or(false, T::matches_envelope) {
                    let envelope = Option::take(envelope).unwrap();
                    let mut tx = tx.clone();
                    Some(
                        async move {
                            tx.send(TypedEnvelope {
                                sender_id: connection_id,
                                original_sender_id: envelope.original_sender_id.map(PeerId),
                                message_id: envelope.id,
                                payload: T::from_envelope(envelope).unwrap(),
                            })
                            .await
                            .is_err()
                        }
                        .boxed(),
                    )
                } else {
                    None
                }
            }));
        rx
    }

    pub async fn add_connection<Conn>(
        self: &Arc<Self>,
        conn: Conn,
    ) -> (
        ConnectionId,
        ConnectionHandler<SplitSink<Conn, WebSocketMessage>, SplitStream<Conn>>,
    )
    where
        Conn: futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
            + Unpin,
    {
        let (tx, rx) = conn.split();
        let connection_id = ConnectionId(
            self.next_connection_id
                .fetch_add(1, atomic::Ordering::SeqCst),
        );
        let (outgoing_tx, outgoing_rx) = mpsc::channel(64);
        let connection = Connection {
            outgoing_tx,
            next_message_id: Default::default(),
            response_channels: Default::default(),
        };
        let handler = ConnectionHandler {
            peer: self.clone(),
            connection_id,
            response_channels: connection.response_channels.clone(),
            outgoing_rx,
            writer: MessageStream::new(tx),
            reader: MessageStream::new(rx),
        };
        self.connections
            .write()
            .await
            .insert(connection_id, connection);
        (connection_id, handler)
    }

    pub async fn disconnect(&self, connection_id: ConnectionId) {
        self.connections.write().await.remove(&connection_id);
    }

    pub async fn reset(&self) {
        self.connections.write().await.clear();
        self.handler_types.lock().await.clear();
        self.message_handlers.write().await.clear();
    }

    pub fn request<T: RequestMessage>(
        self: &Arc<Self>,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(None, receiver_id, request)
    }

    pub fn forward_request<T: RequestMessage>(
        self: &Arc<Self>,
        sender_id: ConnectionId,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(Some(sender_id), receiver_id, request)
    }

    pub fn request_internal<T: RequestMessage>(
        self: &Arc<Self>,
        original_sender_id: Option<ConnectionId>,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        let this = self.clone();
        let (tx, mut rx) = mpsc::channel(1);
        async move {
            let mut connection = this.connection(receiver_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .response_channels
                .lock()
                .await
                .insert(message_id, tx);
            connection
                .outgoing_tx
                .send(request.into_envelope(message_id, None, original_sender_id.map(|id| id.0)))
                .await?;
            let response = rx
                .recv()
                .await
                .ok_or_else(|| anyhow!("connection was closed"))?;
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received response of the wrong type"))
        }
    }

    pub fn send<T: EnvelopedMessage>(
        self: &Arc<Self>,
        receiver_id: ConnectionId,
        message: T,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let mut connection = this.connection(receiver_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .outgoing_tx
                .send(message.into_envelope(message_id, None, None))
                .await?;
            Ok(())
        }
    }

    pub fn forward_send<T: EnvelopedMessage>(
        self: &Arc<Self>,
        sender_id: ConnectionId,
        receiver_id: ConnectionId,
        message: T,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let mut connection = this.connection(receiver_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .outgoing_tx
                .send(message.into_envelope(message_id, None, Some(sender_id.0)))
                .await?;
            Ok(())
        }
    }

    pub fn respond<T: RequestMessage>(
        self: &Arc<Self>,
        receipt: Receipt<T>,
        response: T::Response,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let mut connection = this.connection(receipt.sender_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .outgoing_tx
                .send(response.into_envelope(message_id, Some(receipt.message_id), None))
                .await?;
            Ok(())
        }
    }

    fn connection(
        self: &Arc<Self>,
        connection_id: ConnectionId,
    ) -> impl Future<Output = Result<Connection>> {
        let this = self.clone();
        async move {
            let connections = this.connections.read().await;
            let connection = connections
                .get(&connection_id)
                .ok_or_else(|| anyhow!("no such connection: {}", connection_id))?;
            Ok(connection.clone())
        }
    }
}

impl<W, R> ConnectionHandler<W, R>
where
    W: futures::Sink<WebSocketMessage, Error = WebSocketError> + Unpin,
    R: futures::Stream<Item = Result<WebSocketMessage, WebSocketError>> + Unpin,
{
    pub async fn run(mut self) -> Result<()> {
        loop {
            let read_message = self.reader.read_message().fuse();
            futures::pin_mut!(read_message);
            loop {
                futures::select_biased! {
                    incoming = read_message => match incoming {
                        Ok(incoming) => {
                            Self::handle_incoming_message(incoming, &self.peer, self.connection_id, &self.response_channels).await;
                            break;
                        }
                        Err(error) => {
                            self.response_channels.lock().await.clear();
                            Err(error).context("received invalid RPC message")?;
                        }
                    },
                    outgoing = self.outgoing_rx.recv().fuse() => match outgoing {
                        Some(outgoing) => {
                            if let Err(result) = self.writer.write_message(&outgoing).await {
                                self.response_channels.lock().await.clear();
                                Err(result).context("failed to write RPC message")?;
                            }
                        }
                        None => return Ok(()),
                    }
                }
            }
        }
    }

    pub async fn receive<M: EnvelopedMessage>(&mut self) -> Result<TypedEnvelope<M>> {
        let envelope = self.reader.read_message().await?;
        let original_sender_id = envelope.original_sender_id;
        let message_id = envelope.id;
        let payload =
            M::from_envelope(envelope).ok_or_else(|| anyhow!("unexpected message type"))?;
        Ok(TypedEnvelope {
            sender_id: self.connection_id,
            original_sender_id: original_sender_id.map(PeerId),
            message_id,
            payload,
        })
    }

    async fn handle_incoming_message(
        message: proto::Envelope,
        peer: &Arc<Peer>,
        connection_id: ConnectionId,
        response_channels: &ResponseChannels,
    ) {
        if let Some(responding_to) = message.responding_to {
            let channel = response_channels.lock().await.remove(&responding_to);
            if let Some(mut tx) = channel {
                tx.send(message).await.ok();
            } else {
                log::warn!("received RPC response to unknown request {}", responding_to);
            }
        } else {
            let mut envelope = Some(message);
            let mut handler_index = None;
            let mut handler_was_dropped = false;
            for (i, handler) in peer.message_handlers.read().await.iter().enumerate() {
                if let Some(future) = handler(&mut envelope, connection_id) {
                    handler_was_dropped = future.await;
                    handler_index = Some(i);
                    break;
                }
            }

            if let Some(handler_index) = handler_index {
                if handler_was_dropped {
                    drop(peer.message_handlers.write().await.remove(handler_index));
                }
            } else {
                log::warn!("unhandled message: {:?}", envelope.unwrap().payload);
            }
        }
    }
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

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use postage::oneshot;

    #[test]
    fn test_request_response() {
        smol::block_on(async move {
            // create 2 clients connected to 1 server
            let server = Peer::new();
            let client1 = Peer::new();
            let client2 = Peer::new();

            let (client1_to_server_conn, server_to_client_1_conn) = test::Channel::bidirectional();
            let (client1_conn_id, task1) = client1.add_connection(client1_to_server_conn).await;
            let (_, task2) = server.add_connection(server_to_client_1_conn).await;

            let (client2_to_server_conn, server_to_client_2_conn) = test::Channel::bidirectional();
            let (client2_conn_id, task3) = client2.add_connection(client2_to_server_conn).await;
            let (_, task4) = server.add_connection(server_to_client_2_conn).await;

            smol::spawn(task1.run()).detach();
            smol::spawn(task2.run()).detach();
            smol::spawn(task3.run()).detach();
            smol::spawn(task4.run()).detach();

            // define the expected requests and responses
            let request1 = proto::Auth {
                user_id: 1,
                access_token: "token-1".to_string(),
            };
            let response1 = proto::AuthResponse {
                credentials_valid: true,
            };
            let request2 = proto::Auth {
                user_id: 2,
                access_token: "token-2".to_string(),
            };
            let response2 = proto::AuthResponse {
                credentials_valid: false,
            };
            let request3 = proto::OpenBuffer {
                worktree_id: 1,
                path: "path/two".to_string(),
            };
            let response3 = proto::OpenBufferResponse {
                buffer: Some(proto::Buffer {
                    id: 2,
                    content: "path/two content".to_string(),
                    history: vec![],
                    selections: vec![],
                }),
            };
            let request4 = proto::OpenBuffer {
                worktree_id: 2,
                path: "path/one".to_string(),
            };
            let response4 = proto::OpenBufferResponse {
                buffer: Some(proto::Buffer {
                    id: 1,
                    content: "path/one content".to_string(),
                    history: vec![],
                    selections: vec![],
                }),
            };

            // on the server, respond to two requests for each client
            let mut open_buffer_rx = server.add_message_handler::<proto::OpenBuffer>().await;
            let mut auth_rx = server.add_message_handler::<proto::Auth>().await;
            let (mut server_done_tx, mut server_done_rx) = oneshot::channel::<()>();
            smol::spawn({
                let request1 = request1.clone();
                let request2 = request2.clone();
                let request3 = request3.clone();
                let request4 = request4.clone();
                let response1 = response1.clone();
                let response2 = response2.clone();
                let response3 = response3.clone();
                let response4 = response4.clone();
                async move {
                    let msg = auth_rx.recv().await.unwrap();
                    assert_eq!(msg.payload, request1);
                    server
                        .respond(msg.receipt(), response1.clone())
                        .await
                        .unwrap();

                    let msg = auth_rx.recv().await.unwrap();
                    assert_eq!(msg.payload, request2.clone());
                    server
                        .respond(msg.receipt(), response2.clone())
                        .await
                        .unwrap();

                    let msg = open_buffer_rx.recv().await.unwrap();
                    assert_eq!(msg.payload, request3.clone());
                    server
                        .respond(msg.receipt(), response3.clone())
                        .await
                        .unwrap();

                    let msg = open_buffer_rx.recv().await.unwrap();
                    assert_eq!(msg.payload, request4.clone());
                    server
                        .respond(msg.receipt(), response4.clone())
                        .await
                        .unwrap();

                    server_done_tx.send(()).await.unwrap();
                }
            })
            .detach();

            assert_eq!(
                client1.request(client1_conn_id, request1).await.unwrap(),
                response1
            );
            assert_eq!(
                client2.request(client2_conn_id, request2).await.unwrap(),
                response2
            );
            assert_eq!(
                client2.request(client2_conn_id, request3).await.unwrap(),
                response3
            );
            assert_eq!(
                client1.request(client1_conn_id, request4).await.unwrap(),
                response4
            );

            client1.disconnect(client1_conn_id).await;
            client2.disconnect(client1_conn_id).await;

            server_done_rx.recv().await.unwrap();
        });
    }

    #[test]
    fn test_disconnect() {
        smol::block_on(async move {
            let (client_conn, mut server_conn) = test::Channel::bidirectional();

            let client = Peer::new();
            let (connection_id, handler) = client.add_connection(client_conn).await;
            let (mut incoming_messages_ended_tx, mut incoming_messages_ended_rx) =
                postage::barrier::channel();
            smol::spawn(async move {
                handler.run().await.ok();
                incoming_messages_ended_tx.send(()).await.unwrap();
            })
            .detach();
            client.disconnect(connection_id).await;

            incoming_messages_ended_rx.recv().await;
            assert!(
                futures::SinkExt::send(&mut server_conn, WebSocketMessage::Binary(vec![]))
                    .await
                    .is_err()
            );
        });
    }

    #[test]
    fn test_io_error() {
        smol::block_on(async move {
            let (client_conn, server_conn) = test::Channel::bidirectional();
            drop(server_conn);

            let client = Peer::new();
            let (connection_id, handler) = client.add_connection(client_conn).await;
            smol::spawn(handler.run()).detach();

            let err = client
                .request(
                    connection_id,
                    proto::Auth {
                        user_id: 42,
                        access_token: "token".to_string(),
                    },
                )
                .await
                .unwrap_err();
            assert_eq!(err.to_string(), "connection was closed");
        });
    }
}
