use crate::proto::{self, EnvelopedMessage, MessageStream, RequestMessage};
use anyhow::{anyhow, Context, Result};
use async_lock::{Mutex, RwLock};
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{
    future::{BoxFuture, LocalBoxFuture},
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
    dyn Send
        + Sync
        + Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<BoxFuture<'static, ()>>,
>;

type ForegroundMessageHandler =
    Box<dyn Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<LocalBoxFuture<'static, ()>>>;

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

pub type Router = RouterInternal<MessageHandler>;
pub type ForegroundRouter = RouterInternal<ForegroundMessageHandler>;
pub struct RouterInternal<H> {
    message_handlers: Vec<H>,
    handler_types: HashSet<TypeId>,
}

pub struct Peer {
    connections: RwLock<HashMap<ConnectionId, Connection>>,
    next_connection_id: AtomicU32,
}

#[derive(Clone)]
struct Connection {
    outgoing_tx: mpsc::Sender<proto::Envelope>,
    next_message_id: Arc<AtomicU32>,
    response_channels: Arc<Mutex<HashMap<u32, mpsc::Sender<proto::Envelope>>>>,
}

pub struct IOHandler<W, R> {
    connection_id: ConnectionId,
    incoming_tx: mpsc::Sender<proto::Envelope>,
    outgoing_rx: mpsc::Receiver<proto::Envelope>,
    writer: MessageStream<W>,
    reader: MessageStream<R>,
}

impl Peer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            connections: Default::default(),
            next_connection_id: Default::default(),
        })
    }

    pub async fn add_connection<Conn, H, Fut>(
        self: &Arc<Self>,
        conn: Conn,
        router: Arc<RouterInternal<H>>,
    ) -> (
        ConnectionId,
        IOHandler<SplitSink<Conn, WebSocketMessage>, SplitStream<Conn>>,
        impl Future<Output = anyhow::Result<()>>,
    )
    where
        H: Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<Fut>,
        Fut: Future<Output = ()>,
        Conn: futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
            + Unpin,
    {
        let (tx, rx) = conn.split();
        let connection_id = ConnectionId(
            self.next_connection_id
                .fetch_add(1, atomic::Ordering::SeqCst),
        );
        let (incoming_tx, mut incoming_rx) = mpsc::channel(64);
        let (outgoing_tx, outgoing_rx) = mpsc::channel(64);
        let connection = Connection {
            outgoing_tx,
            next_message_id: Default::default(),
            response_channels: Default::default(),
        };
        let handle_io = IOHandler {
            connection_id,
            outgoing_rx,
            incoming_tx,
            writer: MessageStream::new(tx),
            reader: MessageStream::new(rx),
        };

        let response_channels = connection.response_channels.clone();
        let handle_messages = async move {
            while let Some(message) = incoming_rx.recv().await {
                if let Some(responding_to) = message.responding_to {
                    let channel = response_channels.lock().await.remove(&responding_to);
                    if let Some(mut tx) = channel {
                        tx.send(message).await.ok();
                    } else {
                        log::warn!("received RPC response to unknown request {}", responding_to);
                    }
                } else {
                    router.handle(connection_id, message).await;
                }
            }
            response_channels.lock().await.clear();
            Ok(())
        };

        self.connections
            .write()
            .await
            .insert(connection_id, connection);

        (connection_id, handle_io, handle_messages)
    }

    pub async fn disconnect(&self, connection_id: ConnectionId) {
        self.connections.write().await.remove(&connection_id);
    }

    pub async fn reset(&self) {
        self.connections.write().await.clear();
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

impl<H, Fut> RouterInternal<H>
where
    H: Fn(&mut Option<proto::Envelope>, ConnectionId) -> Option<Fut>,
    Fut: Future<Output = ()>,
{
    pub fn new() -> Self {
        Self {
            message_handlers: Default::default(),
            handler_types: Default::default(),
        }
    }

    async fn handle(&self, connection_id: ConnectionId, message: proto::Envelope) {
        let mut envelope = Some(message);
        for handler in self.message_handlers.iter() {
            if let Some(future) = handler(&mut envelope, connection_id) {
                future.await;
                return;
            }
        }
        log::warn!("unhandled message: {:?}", envelope.unwrap().payload);
    }
}

impl Router {
    pub fn add_message_handler<T, Fut, F>(&mut self, handler: F)
    where
        T: EnvelopedMessage,
        Fut: 'static + Send + Future<Output = Result<()>>,
        F: 'static + Send + Sync + Fn(TypedEnvelope<T>) -> Fut,
    {
        if !self.handler_types.insert(TypeId::of::<T>()) {
            panic!("duplicate handler type");
        }

        self.message_handlers
            .push(Box::new(move |envelope, connection_id| {
                if envelope.as_ref().map_or(false, T::matches_envelope) {
                    let envelope = Option::take(envelope).unwrap();
                    let message_id = envelope.id;
                    let future = handler(TypedEnvelope {
                        sender_id: connection_id,
                        original_sender_id: envelope.original_sender_id.map(PeerId),
                        message_id,
                        payload: T::from_envelope(envelope).unwrap(),
                    });
                    Some(
                        async move {
                            if let Err(error) = future.await {
                                log::error!(
                                    "error handling message {} {}: {:?}",
                                    T::NAME,
                                    message_id,
                                    error
                                );
                            }
                        }
                        .boxed(),
                    )
                } else {
                    None
                }
            }));
    }
}

impl ForegroundRouter {
    pub fn add_message_handler<T, Fut, F>(&mut self, handler: F)
    where
        T: EnvelopedMessage,
        Fut: 'static + Future<Output = Result<()>>,
        F: 'static + Fn(TypedEnvelope<T>) -> Fut,
    {
        if !self.handler_types.insert(TypeId::of::<T>()) {
            panic!("duplicate handler type");
        }

        self.message_handlers
            .push(Box::new(move |envelope, connection_id| {
                if envelope.as_ref().map_or(false, T::matches_envelope) {
                    let envelope = Option::take(envelope).unwrap();
                    let message_id = envelope.id;
                    let future = handler(TypedEnvelope {
                        sender_id: connection_id,
                        original_sender_id: envelope.original_sender_id.map(PeerId),
                        message_id,
                        payload: T::from_envelope(envelope).unwrap(),
                    });
                    Some(
                        async move {
                            if let Err(error) = future.await {
                                log::error!(
                                    "error handling message {} {}: {:?}",
                                    T::NAME,
                                    message_id,
                                    error
                                );
                            }
                        }
                        .boxed_local(),
                    )
                } else {
                    None
                }
            }));
    }
}

impl<W, R> IOHandler<W, R>
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
                            if self.incoming_tx.send(incoming).await.is_err() {
                                return Ok(());
                            }
                            break;
                        }
                        Err(error) => {
                            Err(error).context("received invalid RPC message")?;
                        }
                    },
                    outgoing = self.outgoing_rx.recv().fuse() => match outgoing {
                        Some(outgoing) => {
                            if let Err(result) = self.writer.write_message(&outgoing).await {
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

    #[test]
    fn test_request_response() {
        smol::block_on(async move {
            // create 2 clients connected to 1 server
            let server = Peer::new();
            let client1 = Peer::new();
            let client2 = Peer::new();

            let mut router = Router::new();
            router.add_message_handler({
                let server = server.clone();
                move |envelope: TypedEnvelope<proto::Auth>| {
                    let server = server.clone();
                    async move {
                        let receipt = envelope.receipt();
                        let message = envelope.payload;
                        server
                            .respond(
                                receipt,
                                match message.user_id {
                                    1 => {
                                        assert_eq!(message.access_token, "access-token-1");
                                        proto::AuthResponse {
                                            credentials_valid: true,
                                        }
                                    }
                                    2 => {
                                        assert_eq!(message.access_token, "access-token-2");
                                        proto::AuthResponse {
                                            credentials_valid: false,
                                        }
                                    }
                                    _ => {
                                        panic!("unexpected user id {}", message.user_id);
                                    }
                                },
                            )
                            .await
                    }
                }
            });

            router.add_message_handler({
                let server = server.clone();
                move |envelope: TypedEnvelope<proto::OpenBuffer>| {
                    let server = server.clone();
                    async move {
                        let receipt = envelope.receipt();
                        let message = envelope.payload;
                        server
                            .respond(
                                receipt,
                                match message.path.as_str() {
                                    "path/one" => {
                                        assert_eq!(message.worktree_id, 1);
                                        proto::OpenBufferResponse {
                                            buffer: Some(proto::Buffer {
                                                id: 101,
                                                content: "path/one content".to_string(),
                                                history: vec![],
                                                selections: vec![],
                                            }),
                                        }
                                    }
                                    "path/two" => {
                                        assert_eq!(message.worktree_id, 2);
                                        proto::OpenBufferResponse {
                                            buffer: Some(proto::Buffer {
                                                id: 102,
                                                content: "path/two content".to_string(),
                                                history: vec![],
                                                selections: vec![],
                                            }),
                                        }
                                    }
                                    _ => {
                                        panic!("unexpected path {}", message.path);
                                    }
                                },
                            )
                            .await
                    }
                }
            });
            let router = Arc::new(router);

            let (client1_to_server_conn, server_to_client_1_conn) = test::Channel::bidirectional();
            let (client1_conn_id, io_task1, msg_task1) = client1
                .add_connection(client1_to_server_conn, router.clone())
                .await;
            let (_, io_task2, msg_task2) = server
                .add_connection(server_to_client_1_conn, router.clone())
                .await;

            let (client2_to_server_conn, server_to_client_2_conn) = test::Channel::bidirectional();
            let (client2_conn_id, io_task3, msg_task3) = client2
                .add_connection(client2_to_server_conn, router.clone())
                .await;
            let (_, io_task4, msg_task4) = server
                .add_connection(server_to_client_2_conn, router.clone())
                .await;

            smol::spawn(io_task1.run()).detach();
            smol::spawn(io_task2.run()).detach();
            smol::spawn(io_task3.run()).detach();
            smol::spawn(io_task4.run()).detach();
            smol::spawn(msg_task1).detach();
            smol::spawn(msg_task2).detach();
            smol::spawn(msg_task3).detach();
            smol::spawn(msg_task4).detach();

            assert_eq!(
                client1
                    .request(
                        client1_conn_id,
                        proto::Auth {
                            user_id: 1,
                            access_token: "access-token-1".to_string(),
                        },
                    )
                    .await
                    .unwrap(),
                proto::AuthResponse {
                    credentials_valid: true,
                }
            );

            assert_eq!(
                client2
                    .request(
                        client2_conn_id,
                        proto::Auth {
                            user_id: 2,
                            access_token: "access-token-2".to_string(),
                        },
                    )
                    .await
                    .unwrap(),
                proto::AuthResponse {
                    credentials_valid: false,
                }
            );

            assert_eq!(
                client1
                    .request(
                        client1_conn_id,
                        proto::OpenBuffer {
                            worktree_id: 1,
                            path: "path/one".to_string(),
                        },
                    )
                    .await
                    .unwrap(),
                proto::OpenBufferResponse {
                    buffer: Some(proto::Buffer {
                        id: 101,
                        content: "path/one content".to_string(),
                        history: vec![],
                        selections: vec![],
                    }),
                }
            );

            assert_eq!(
                client2
                    .request(
                        client2_conn_id,
                        proto::OpenBuffer {
                            worktree_id: 2,
                            path: "path/two".to_string(),
                        },
                    )
                    .await
                    .unwrap(),
                proto::OpenBufferResponse {
                    buffer: Some(proto::Buffer {
                        id: 102,
                        content: "path/two content".to_string(),
                        history: vec![],
                        selections: vec![],
                    }),
                }
            );

            client1.disconnect(client1_conn_id).await;
            client2.disconnect(client1_conn_id).await;
        });
    }

    #[test]
    fn test_disconnect() {
        smol::block_on(async move {
            let (client_conn, mut server_conn) = test::Channel::bidirectional();

            let client = Peer::new();
            let router = Arc::new(Router::new());
            let (connection_id, io_handler, message_handler) =
                client.add_connection(client_conn, router).await;

            let (mut io_ended_tx, mut io_ended_rx) = postage::barrier::channel();
            smol::spawn(async move {
                io_handler.run().await.ok();
                io_ended_tx.send(()).await.unwrap();
            })
            .detach();

            let (mut messages_ended_tx, mut messages_ended_rx) = postage::barrier::channel();
            smol::spawn(async move {
                message_handler.await.ok();
                messages_ended_tx.send(()).await.unwrap();
            })
            .detach();

            client.disconnect(connection_id).await;

            io_ended_rx.recv().await;
            messages_ended_rx.recv().await;
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
            let router = Arc::new(Router::new());
            let (connection_id, io_handler, message_handler) =
                client.add_connection(client_conn, router).await;
            smol::spawn(io_handler.run()).detach();
            smol::spawn(message_handler).detach();

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
