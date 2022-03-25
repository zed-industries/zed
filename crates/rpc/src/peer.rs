use super::proto::{self, AnyTypedEnvelope, EnvelopedMessage, MessageStream, RequestMessage};
use super::Connection;
use anyhow::{anyhow, Context, Result};
use futures::{channel::oneshot, stream::BoxStream, FutureExt as _, StreamExt};
use parking_lot::{Mutex, RwLock};
use postage::{
    barrier, mpsc,
    prelude::{Sink as _, Stream as _},
};
use smol_timeout::TimeoutExt as _;
use std::sync::atomic::Ordering::SeqCst;
use std::{
    collections::HashMap,
    fmt,
    future::Future,
    marker::PhantomData,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
    time::Duration,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ConnectionId(pub u32);

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PeerId(pub u32);

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
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
    pub connections: RwLock<HashMap<ConnectionId, ConnectionState>>,
    next_connection_id: AtomicU32,
}

#[derive(Clone)]
pub struct ConnectionState {
    outgoing_tx: futures::channel::mpsc::UnboundedSender<proto::Message>,
    next_message_id: Arc<AtomicU32>,
    response_channels:
        Arc<Mutex<Option<HashMap<u32, oneshot::Sender<(proto::Envelope, barrier::Sender)>>>>>,
}

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(1);
const WRITE_TIMEOUT: Duration = Duration::from_secs(2);
const RECEIVE_TIMEOUT: Duration = Duration::from_secs(5);

impl Peer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            connections: Default::default(),
            next_connection_id: Default::default(),
        })
    }

    pub async fn add_connection<F, Fut, Out>(
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
        let (mut incoming_tx, incoming_rx) = mpsc::channel(64);
        let (outgoing_tx, mut outgoing_rx) = futures::channel::mpsc::unbounded();

        let connection_id = ConnectionId(self.next_connection_id.fetch_add(1, SeqCst));
        let connection_state = ConnectionState {
            outgoing_tx: outgoing_tx.clone(),
            next_message_id: Default::default(),
            response_channels: Arc::new(Mutex::new(Some(Default::default()))),
        };
        let mut writer = MessageStream::new(connection.tx);
        let mut reader = MessageStream::new(connection.rx);

        let this = self.clone();
        let response_channels = connection_state.response_channels.clone();
        let handle_io = async move {
            let _end_connection = util::defer(|| {
                response_channels.lock().take();
                this.connections.write().remove(&connection_id);
            });

            // Send messages on this frequency so the connection isn't closed.
            let keepalive_timer = create_timer(KEEPALIVE_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer);

            // Disconnect if we don't receive messages at least this frequently.
            let receive_timeout = create_timer(RECEIVE_TIMEOUT).fuse();
            futures::pin_mut!(receive_timeout);

            loop {
                let read_message = reader.read().fuse();
                futures::pin_mut!(read_message);

                loop {
                    futures::select_biased! {
                        outgoing = outgoing_rx.next().fuse() => match outgoing {
                            Some(outgoing) => {
                                if let Some(result) = writer.write(outgoing).timeout(WRITE_TIMEOUT).await {
                                    result.context("failed to write RPC message")?;
                                    keepalive_timer.set(create_timer(KEEPALIVE_INTERVAL).fuse());
                                } else {
                                    Err(anyhow!("timed out writing message"))?;
                                }
                            }
                            None => return Ok(()),
                        },
                        incoming = read_message => {
                            let incoming = incoming.context("received invalid RPC message")?;
                            receive_timeout.set(create_timer(RECEIVE_TIMEOUT).fuse());
                            if let proto::Message::Envelope(incoming) = incoming {
                                if incoming_tx.send(incoming).await.is_err() {
                                    return Ok(());
                                }
                            }
                            break;
                        },
                        _ = keepalive_timer => {
                            if let Some(result) = writer.write(proto::Message::Ping).timeout(WRITE_TIMEOUT).await {
                                result.context("failed to send keepalive")?;
                                keepalive_timer.set(create_timer(KEEPALIVE_INTERVAL).fuse());
                            } else {
                                Err(anyhow!("timed out sending keepalive"))?;
                            }
                        }
                        _ = receive_timeout => {
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
                if let Some(responding_to) = incoming.responding_to {
                    let channel = response_channels.lock().as_mut()?.remove(&responding_to);
                    if let Some(tx) = channel {
                        let mut requester_resumed = barrier::channel();
                        if let Err(error) = tx.send((incoming, requester_resumed.0)) {
                            log::debug!(
                                "received RPC but request future was dropped {:?}",
                                error.0
                            );
                        }
                        requester_resumed.1.recv().await;
                    } else {
                        log::warn!("received RPC response to unknown request {}", responding_to);
                    }

                    None
                } else {
                    proto::build_typed_envelope(connection_id, incoming).or_else(|| {
                        log::error!("unable to construct a typed envelope");
                        None
                    })
                }
            }
        });
        (connection_id, handle_io, incoming_rx.boxed())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn add_test_connection(
        self: &Arc<Self>,
        connection: Connection,
        executor: Arc<gpui::executor::Background>,
    ) -> (
        ConnectionId,
        impl Future<Output = anyhow::Result<()>> + Send,
        BoxStream<'static, Box<dyn AnyTypedEnvelope>>,
    ) {
        let executor = executor.clone();
        self.add_connection(connection, move |duration| executor.timer(duration))
            .await
    }

    pub fn disconnect(&self, connection_id: ConnectionId) {
        self.connections.write().remove(&connection_id);
    }

    pub fn reset(&self) {
        self.connections.write().clear();
    }

    pub fn request<T: RequestMessage>(
        &self,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(None, receiver_id, request)
    }

    pub fn forward_request<T: RequestMessage>(
        &self,
        sender_id: ConnectionId,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_internal(Some(sender_id), receiver_id, request)
    }

    pub fn request_internal<T: RequestMessage>(
        &self,
        original_sender_id: Option<ConnectionId>,
        receiver_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
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
                    original_sender_id.map(|id| id.0),
                )))
                .map_err(|_| anyhow!("connection was closed"))?;
            Ok(())
        });
        async move {
            send?;
            let (response, _barrier) = rx.await.map_err(|_| anyhow!("connection was closed"))?;
            if let Some(proto::envelope::Payload::Error(error)) = &response.payload {
                Err(anyhow!("RPC request failed - {}", error.message))
            } else {
                T::Response::from_envelope(response)
                    .ok_or_else(|| anyhow!("received response of the wrong type"))
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
                Some(sender_id.0),
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

    fn connection_state(&self, connection_id: ConnectionId) -> Result<ConnectionState> {
        let connections = self.connections.read();
        let connection = connections
            .get(&connection_id)
            .ok_or_else(|| anyhow!("no such connection: {}", connection_id))?;
        Ok(connection.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TypedEnvelope;
    use async_tungstenite::tungstenite::Message as WebSocketMessage;
    use gpui::TestAppContext;

    #[gpui::test(iterations = 50)]
    async fn test_request_response(cx: &mut TestAppContext) {
        let executor = cx.foreground();

        // create 2 clients connected to 1 server
        let server = Peer::new();
        let client1 = Peer::new();
        let client2 = Peer::new();

        let (client1_to_server_conn, server_to_client_1_conn, _kill) =
            Connection::in_memory(cx.background());
        let (client1_conn_id, io_task1, client1_incoming) = client1
            .add_test_connection(client1_to_server_conn, cx.background())
            .await;
        let (_, io_task2, server_incoming1) = server
            .add_test_connection(server_to_client_1_conn, cx.background())
            .await;

        let (client2_to_server_conn, server_to_client_2_conn, _kill) =
            Connection::in_memory(cx.background());
        let (client2_conn_id, io_task3, client2_incoming) = client2
            .add_test_connection(client2_to_server_conn, cx.background())
            .await;
        let (_, io_task4, server_incoming2) = server
            .add_test_connection(server_to_client_2_conn, cx.background())
            .await;

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
        let executor = cx.foreground();
        let server = Peer::new();
        let client = Peer::new();

        let (client_to_server_conn, server_to_client_conn, _kill) =
            Connection::in_memory(cx.background());
        let (client_to_server_conn_id, io_task1, mut client_incoming) = client
            .add_test_connection(client_to_server_conn, cx.background())
            .await;
        let (server_to_client_conn_id, io_task2, mut server_incoming) = server
            .add_test_connection(server_to_client_conn, cx.background())
            .await;

        executor.spawn(io_task1).detach();
        executor.spawn(io_task2).detach();

        executor
            .spawn(async move {
                let request = server_incoming
                    .next()
                    .await
                    .unwrap()
                    .into_any()
                    .downcast::<TypedEnvelope<proto::Ping>>()
                    .unwrap();

                server
                    .send(
                        server_to_client_conn_id,
                        proto::Error {
                            message: "message 1".to_string(),
                        },
                    )
                    .unwrap();
                server
                    .send(
                        server_to_client_conn_id,
                        proto::Error {
                            message: "message 2".to_string(),
                        },
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
        let executor = cx.foreground();
        let server = Peer::new();
        let client = Peer::new();

        let (client_to_server_conn, server_to_client_conn, _kill) =
            Connection::in_memory(cx.background());
        let (client_to_server_conn_id, io_task1, mut client_incoming) = client
            .add_test_connection(client_to_server_conn, cx.background())
            .await;
        let (server_to_client_conn_id, io_task2, mut server_incoming) = server
            .add_test_connection(server_to_client_conn, cx.background())
            .await;

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
                        proto::Error {
                            message: "message 1".to_string(),
                        },
                    )
                    .unwrap();
                server
                    .send(
                        server_to_client_conn_id,
                        proto::Error {
                            message: "message 2".to_string(),
                        },
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
        cx.background().simulate_random_delay().await;
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
        let executor = cx.foreground();

        let (client_conn, mut server_conn, _kill) = Connection::in_memory(cx.background());

        let client = Peer::new();
        let (connection_id, io_handler, mut incoming) = client
            .add_test_connection(client_conn, cx.background())
            .await;

        let (mut io_ended_tx, mut io_ended_rx) = postage::barrier::channel();
        executor
            .spawn(async move {
                io_handler.await.ok();
                io_ended_tx.send(()).await.unwrap();
            })
            .detach();

        let (mut messages_ended_tx, mut messages_ended_rx) = postage::barrier::channel();
        executor
            .spawn(async move {
                incoming.next().await;
                messages_ended_tx.send(()).await.unwrap();
            })
            .detach();

        client.disconnect(connection_id);

        io_ended_rx.recv().await;
        messages_ended_rx.recv().await;
        assert!(server_conn
            .send(WebSocketMessage::Binary(vec![]))
            .await
            .is_err());
    }

    #[gpui::test(iterations = 50)]
    async fn test_io_error(cx: &mut TestAppContext) {
        let executor = cx.foreground();
        let (client_conn, mut server_conn, _kill) = Connection::in_memory(cx.background());

        let client = Peer::new();
        let (connection_id, io_handler, mut incoming) = client
            .add_test_connection(client_conn, cx.background())
            .await;
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
