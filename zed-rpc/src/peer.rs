use crate::proto::{self, EnvelopedMessage, MessageStream, RequestMessage};
use anyhow::{anyhow, Result};
use async_lock::{Mutex, RwLock};
use futures::{
    future::{BoxFuture, Either},
    AsyncRead, AsyncWrite, FutureExt,
};
use postage::{
    barrier, mpsc, oneshot,
    prelude::{Sink, Stream},
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
};

type BoxedWriter = Pin<Box<dyn AsyncWrite + 'static + Send>>;
type BoxedReader = Pin<Box<dyn AsyncRead + 'static + Send>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ConnectionId(u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PeerId(u32);

struct Connection {
    writer: Mutex<MessageStream<BoxedWriter>>,
    reader: Mutex<MessageStream<BoxedReader>>,
    response_channels: Mutex<HashMap<u32, oneshot::Sender<proto::Envelope>>>,
    next_message_id: AtomicU32,
}

type MessageHandler = Box<
    dyn Send + Sync + Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<BoxFuture<bool>>,
>;

#[derive(Clone, Copy)]
pub struct Receipt<T> {
    sender_id: ConnectionId,
    message_id: u32,
    payload_type: PhantomData<T>,
}

pub struct TypedEnvelope<T> {
    pub sender_id: ConnectionId,
    pub original_sender_id: Option<PeerId>,
    pub message_id: u32,
    pub payload: T,
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
    connections: RwLock<HashMap<ConnectionId, Arc<Connection>>>,
    connection_close_barriers: RwLock<HashMap<ConnectionId, barrier::Sender>>,
    message_handlers: RwLock<Vec<MessageHandler>>,
    handler_types: Mutex<HashSet<TypeId>>,
    next_connection_id: AtomicU32,
}

impl Peer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            connections: Default::default(),
            connection_close_barriers: Default::default(),
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

    pub async fn add_connection<Conn>(self: &Arc<Self>, conn: Conn) -> ConnectionId
    where
        Conn: Clone + AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let connection_id = ConnectionId(
            self.next_connection_id
                .fetch_add(1, atomic::Ordering::SeqCst),
        );
        self.connections.write().await.insert(
            connection_id,
            Arc::new(Connection {
                reader: Mutex::new(MessageStream::new(Box::pin(conn.clone()))),
                writer: Mutex::new(MessageStream::new(Box::pin(conn.clone()))),
                response_channels: Default::default(),
                next_message_id: Default::default(),
            }),
        );
        connection_id
    }

    pub async fn disconnect(&self, connection_id: ConnectionId) {
        self.connections.write().await.remove(&connection_id);
        self.connection_close_barriers
            .write()
            .await
            .remove(&connection_id);
    }

    pub fn handle_messages(
        self: &Arc<Self>,
        connection_id: ConnectionId,
    ) -> impl Future<Output = Result<()>> + 'static {
        let (close_tx, mut close_rx) = barrier::channel();
        let this = self.clone();
        async move {
            this.connection_close_barriers
                .write()
                .await
                .insert(connection_id, close_tx);
            let connection = this.connection(connection_id).await?;
            let closed = close_rx.recv();
            futures::pin_mut!(closed);

            loop {
                let mut reader = connection.reader.lock().await;
                let read_message = reader.read_message();
                futures::pin_mut!(read_message);

                match futures::future::select(read_message, &mut closed).await {
                    Either::Left((Ok(incoming), _)) => {
                        if let Some(responding_to) = incoming.responding_to {
                            let channel = connection
                                .response_channels
                                .lock()
                                .await
                                .remove(&responding_to);
                            if let Some(mut tx) = channel {
                                tx.send(incoming).await.ok();
                            } else {
                                log::warn!(
                                    "received RPC response to unknown request {}",
                                    responding_to
                                );
                            }
                        } else {
                            let mut envelope = Some(incoming);
                            let mut handler_index = None;
                            let mut handler_was_dropped = false;
                            for (i, handler) in
                                this.message_handlers.read().await.iter().enumerate()
                            {
                                if let Some(future) = handler(&mut envelope, connection_id) {
                                    handler_was_dropped = future.await;
                                    handler_index = Some(i);
                                    break;
                                }
                            }

                            if let Some(handler_index) = handler_index {
                                if handler_was_dropped {
                                    drop(this.message_handlers.write().await.remove(handler_index));
                                }
                            } else {
                                log::warn!("unhandled message: {:?}", envelope.unwrap().payload);
                            }
                        }
                    }
                    Either::Left((Err(error), _)) => {
                        log::warn!("received invalid RPC message: {}", error);
                        Err(error)?;
                    }
                    Either::Right(_) => return Ok(()),
                }
            }
        }
    }

    pub async fn receive<M: EnvelopedMessage>(
        self: &Arc<Self>,
        connection_id: ConnectionId,
    ) -> Result<TypedEnvelope<M>> {
        let connection = self.connection(connection_id).await?;
        let envelope = connection.reader.lock().await.read_message().await?;
        let original_sender_id = envelope.original_sender_id;
        let message_id = envelope.id;
        let payload =
            M::from_envelope(envelope).ok_or_else(|| anyhow!("unexpected message type"))?;
        Ok(TypedEnvelope {
            sender_id: connection_id,
            original_sender_id: original_sender_id.map(PeerId),
            message_id,
            payload,
        })
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
        let (tx, mut rx) = oneshot::channel();
        async move {
            let connection = this.connection(receiver_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .response_channels
                .lock()
                .await
                .insert(message_id, tx);
            connection
                .writer
                .lock()
                .await
                .write_message(&request.into_envelope(
                    message_id,
                    None,
                    original_sender_id.map(|id| id.0),
                ))
                .await?;
            let response = rx
                .recv()
                .await
                .expect("response channel was unexpectedly dropped");
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received response of the wrong type"))
        }
    }

    pub fn send<T: EnvelopedMessage>(
        self: &Arc<Self>,
        connection_id: ConnectionId,
        message: T,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let connection = this.connection(connection_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .writer
                .lock()
                .await
                .write_message(&message.into_envelope(message_id, None, None))
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
            let connection = this.connection(receiver_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .writer
                .lock()
                .await
                .write_message(&message.into_envelope(message_id, None, Some(sender_id.0)))
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
            let connection = this.connection(receipt.sender_id).await?;
            let message_id = connection
                .next_message_id
                .fetch_add(1, atomic::Ordering::SeqCst);
            connection
                .writer
                .lock()
                .await
                .write_message(&response.into_envelope(message_id, Some(receipt.message_id), None))
                .await?;
            Ok(())
        }
    }

    async fn connection(&self, id: ConnectionId) -> Result<Arc<Connection>> {
        Ok(self
            .connections
            .read()
            .await
            .get(&id)
            .ok_or_else(|| anyhow!("unknown connection: {}", id.0))?
            .clone())
    }
}

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
    use smol::{
        io::AsyncWriteExt,
        net::unix::{UnixListener, UnixStream},
    };
    use std::io;
    use tempdir::TempDir;

    #[test]
    fn test_request_response() {
        smol::block_on(async move {
            // create socket
            let socket_dir_path = TempDir::new("test-request-response").unwrap();
            let socket_path = socket_dir_path.path().join("test.sock");
            let listener = UnixListener::bind(&socket_path).unwrap();

            // create 2 clients connected to 1 server
            let server = Peer::new();
            let client1 = Peer::new();
            let client2 = Peer::new();
            let client1_conn_id = client1
                .add_connection(UnixStream::connect(&socket_path).await.unwrap())
                .await;
            let client2_conn_id = client2
                .add_connection(UnixStream::connect(&socket_path).await.unwrap())
                .await;
            let server_conn_id1 = server
                .add_connection(listener.accept().await.unwrap().0)
                .await;
            let server_conn_id2 = server
                .add_connection(listener.accept().await.unwrap().0)
                .await;
            smol::spawn(client1.handle_messages(client1_conn_id)).detach();
            smol::spawn(client2.handle_messages(client2_conn_id)).detach();
            smol::spawn(server.handle_messages(server_conn_id1)).detach();
            smol::spawn(server.handle_messages(server_conn_id2)).detach();

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
                id: 2,
            };
            let response3 = proto::OpenBufferResponse {
                buffer: Some(proto::Buffer {
                    content: "path/two content".to_string(),
                    history: vec![],
                }),
            };
            let request4 = proto::OpenBuffer {
                worktree_id: 2,
                id: 1,
            };
            let response4 = proto::OpenBufferResponse {
                buffer: Some(proto::Buffer {
                    content: "path/one content".to_string(),
                    history: vec![],
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
            let socket_dir_path = TempDir::new("drop-client").unwrap();
            let socket_path = socket_dir_path.path().join(".sock");
            let listener = UnixListener::bind(&socket_path).unwrap();
            let client_conn = UnixStream::connect(&socket_path).await.unwrap();
            let (mut server_conn, _) = listener.accept().await.unwrap();

            let client = Peer::new();
            let connection_id = client.add_connection(client_conn).await;
            let (mut incoming_messages_ended_tx, mut incoming_messages_ended_rx) =
                barrier::channel();
            let handle_messages = client.handle_messages(connection_id);
            smol::spawn(async move {
                handle_messages.await.ok();
                incoming_messages_ended_tx.send(()).await.unwrap();
            })
            .detach();
            client.disconnect(connection_id).await;

            incoming_messages_ended_rx.recv().await;

            let err = server_conn.write(&[]).await.unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
        });
    }

    #[test]
    fn test_io_error() {
        smol::block_on(async move {
            let socket_dir_path = TempDir::new("io-error").unwrap();
            let socket_path = socket_dir_path.path().join(".sock");
            let _listener = UnixListener::bind(&socket_path).unwrap();
            let mut client_conn = UnixStream::connect(&socket_path).await.unwrap();
            client_conn.close().await.unwrap();

            let client = Peer::new();
            let connection_id = client.add_connection(client_conn).await;
            smol::spawn(client.handle_messages(connection_id)).detach();

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
            assert_eq!(
                err.downcast_ref::<io::Error>().unwrap().kind(),
                io::ErrorKind::BrokenPipe
            );
        });
    }
}
