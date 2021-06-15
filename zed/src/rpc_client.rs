use anyhow::{anyhow, Result};
use futures::future::Either;
use gpui::executor::Background;
use postage::{
    barrier, mpsc,
    prelude::{Sink, Stream},
};
use smol::{
    io::{BoxedWriter, ReadHalf},
    lock::Mutex,
    prelude::{AsyncRead, AsyncWrite},
};
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{self, AtomicI32},
        Arc,
    },
};
use zed_rpc::proto::{
    self, MessageStream, RequestMessage, SendMessage, ServerMessage, SubscribeMessage,
};

pub struct RpcClient {
    response_channels: Arc<Mutex<HashMap<i32, (mpsc::Sender<proto::from_server::Variant>, bool)>>>,
    outgoing: Mutex<MessageStream<BoxedWriter>>,
    next_message_id: AtomicI32,
    _drop_tx: barrier::Sender,
}

impl RpcClient {
    pub fn new<Conn>(conn: Conn, executor: Arc<Background>) -> Arc<Self>
    where
        Conn: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let response_channels = Arc::new(Mutex::new(HashMap::new()));
        let (conn_rx, conn_tx) = smol::io::split(conn);
        let (_drop_tx, drop_rx) = barrier::channel();

        executor
            .spawn(Self::handle_incoming(
                conn_rx,
                drop_rx,
                response_channels.clone(),
            ))
            .detach();

        Arc::new(Self {
            response_channels,
            outgoing: Mutex::new(MessageStream::new(Box::pin(conn_tx))),
            _drop_tx,
            next_message_id: AtomicI32::new(0),
        })
    }

    async fn handle_incoming<Conn>(
        conn: ReadHalf<Conn>,
        mut drop_rx: barrier::Receiver,
        response_channels: Arc<
            Mutex<HashMap<i32, (mpsc::Sender<proto::from_server::Variant>, bool)>>,
        >,
    ) where
        Conn: AsyncRead + Unpin,
    {
        let dropped = drop_rx.recv();
        smol::pin!(dropped);

        let mut stream = MessageStream::new(conn);
        loop {
            let read_message = stream.read_message::<proto::FromServer>();
            smol::pin!(read_message);

            match futures::future::select(read_message, &mut dropped).await {
                Either::Left((Ok(incoming), _)) => {
                    if let Some(variant) = incoming.variant {
                        if let Some(request_id) = incoming.request_id {
                            let channel = response_channels.lock().await.remove(&request_id);
                            if let Some((mut tx, oneshot)) = channel {
                                if tx.send(variant).await.is_ok() {
                                    if !oneshot {
                                        response_channels
                                            .lock()
                                            .await
                                            .insert(request_id, (tx, false));
                                    }
                                }
                            } else {
                                log::warn!(
                                    "received RPC response to unknown request id {}",
                                    request_id
                                );
                            }
                        }
                    } else {
                        log::warn!("received RPC message with no content");
                    }
                }
                Either::Left((Err(error), _)) => {
                    log::warn!("invalid incoming RPC message {:?}", error);
                }
                Either::Right(_) => break,
            }
        }
    }

    pub fn request<T: RequestMessage>(
        self: &Arc<Self>,
        req: T,
    ) -> impl Future<Output = Result<T::Response>> {
        let this = self.clone();
        async move {
            let message_id = this.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
            let (tx, mut rx) = mpsc::channel(1);
            this.response_channels
                .lock()
                .await
                .insert(message_id, (tx, true));
            this.outgoing
                .lock()
                .await
                .write_message(&proto::FromClient {
                    id: message_id,
                    variant: Some(req.to_variant()),
                })
                .await?;
            let response = rx
                .recv()
                .await
                .expect("response channel was unexpectedly dropped");
            T::Response::from_variant(response)
                .ok_or_else(|| anyhow!("received response of the wrong t"))
        }
    }

    pub fn send<T: SendMessage>(self: &Arc<Self>, message: T) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let message_id = this.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
            this.outgoing
                .lock()
                .await
                .write_message(&proto::FromClient {
                    id: message_id,
                    variant: Some(message.to_variant()),
                })
                .await?;
            Ok(())
        }
    }

    pub fn subscribe<T: SubscribeMessage>(
        self: &Arc<Self>,
        subscription: T,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<T::Event>>>> {
        let this = self.clone();
        async move {
            let message_id = this.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
            let (tx, rx) = mpsc::channel(256);
            this.response_channels
                .lock()
                .await
                .insert(message_id, (tx, false));
            this.outgoing
                .lock()
                .await
                .write_message(&proto::FromClient {
                    id: message_id,
                    variant: Some(subscription.to_variant()),
                })
                .await?;
            Ok(rx.map(|event| {
                T::Event::from_variant(event).ok_or_else(|| anyhow!("invalid event {:?}"))
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol::{
        future::poll_once,
        io::AsyncWriteExt,
        net::unix::{UnixListener, UnixStream},
    };
    use std::{future::Future, io};
    use tempdir::TempDir;

    #[gpui::test]
    async fn test_request_response(cx: gpui::TestAppContext) {
        let executor = cx.read(|app| app.background_executor().clone());
        let socket_dir_path = TempDir::new("request-response").unwrap();
        let socket_path = socket_dir_path.path().join(".sock");
        let listener = UnixListener::bind(&socket_path).unwrap();
        let client_conn = UnixStream::connect(&socket_path).await.unwrap();
        let (server_conn, _) = listener.accept().await.unwrap();

        let mut server_stream = MessageStream::new(server_conn);
        let client = RpcClient::new(client_conn, executor.clone());

        let client_req = client.request(proto::from_client::Auth {
            user_id: 42,
            access_token: "token".to_string(),
        });
        smol::pin!(client_req);
        let server_req = send_recv(
            &mut client_req,
            server_stream.read_message::<proto::FromClient>(),
        )
        .await
        .unwrap();
        assert_eq!(
            server_req.variant,
            Some(proto::from_client::Variant::Auth(
                proto::from_client::Auth {
                    user_id: 42,
                    access_token: "token".to_string()
                }
            ))
        );

        // Respond to another request to ensure requests are properly matched up.
        server_stream
            .write_message(&proto::FromServer {
                request_id: Some(999),
                variant: Some(proto::from_server::Variant::AuthResponse(
                    proto::from_server::AuthResponse {
                        credentials_valid: false,
                    },
                )),
            })
            .await
            .unwrap();
        server_stream
            .write_message(&proto::FromServer {
                request_id: Some(server_req.id),
                variant: Some(proto::from_server::Variant::AuthResponse(
                    proto::from_server::AuthResponse {
                        credentials_valid: true,
                    },
                )),
            })
            .await
            .unwrap();
        assert_eq!(
            client_req.await.unwrap(),
            proto::from_server::AuthResponse {
                credentials_valid: true
            }
        );
    }

    #[gpui::test]
    async fn test_subscribe(cx: gpui::TestAppContext) {
        let executor = cx.read(|app| app.background_executor().clone());
        let socket_dir_path = TempDir::new("subscribe").unwrap();
        let socket_path = socket_dir_path.path().join(".sock");
        let listener = UnixListener::bind(&socket_path).unwrap();
        let client_conn = UnixStream::connect(&socket_path).await.unwrap();
        let (server_conn, _) = listener.accept().await.unwrap();

        let mut server_stream = MessageStream::new(server_conn);
        let client = RpcClient::new(client_conn, executor.clone());

        let mut events = client
            .subscribe(proto::from_client::SubscribeToPathRequests {})
            .await
            .unwrap();

        let subscription = server_stream
            .read_message::<proto::FromClient>()
            .await
            .unwrap();
        assert_eq!(
            subscription.variant,
            Some(proto::from_client::Variant::SubscribeToPathRequests(
                proto::from_client::SubscribeToPathRequests {}
            ))
        );
        server_stream
            .write_message(&proto::FromServer {
                request_id: Some(subscription.id),
                variant: Some(proto::from_server::Variant::PathRequest(
                    proto::from_server::PathRequest {
                        path: b"path-1".to_vec(),
                    },
                )),
            })
            .await
            .unwrap();
        server_stream
            .write_message(&proto::FromServer {
                request_id: Some(99999),
                variant: Some(proto::from_server::Variant::PathRequest(
                    proto::from_server::PathRequest {
                        path: b"path-2".to_vec(),
                    },
                )),
            })
            .await
            .unwrap();
        server_stream
            .write_message(&proto::FromServer {
                request_id: Some(subscription.id),
                variant: Some(proto::from_server::Variant::PathRequest(
                    proto::from_server::PathRequest {
                        path: b"path-3".to_vec(),
                    },
                )),
            })
            .await
            .unwrap();

        assert_eq!(
            events.recv().await.unwrap().unwrap(),
            proto::from_server::PathRequest {
                path: b"path-1".to_vec()
            }
        );
        assert_eq!(
            events.recv().await.unwrap().unwrap(),
            proto::from_server::PathRequest {
                path: b"path-3".to_vec()
            }
        );
    }

    #[gpui::test]
    async fn test_drop_client(cx: gpui::TestAppContext) {
        let executor = cx.read(|app| app.background_executor().clone());
        let socket_dir_path = TempDir::new("drop-client").unwrap();
        let socket_path = socket_dir_path.path().join(".sock");
        let listener = UnixListener::bind(&socket_path).unwrap();
        let client_conn = UnixStream::connect(&socket_path).await.unwrap();
        let (mut server_conn, _) = listener.accept().await.unwrap();

        let client = RpcClient::new(client_conn, executor.clone());
        drop(client);

        // Try sending an empty payload over and over, until the client is dropped and hangs up.
        loop {
            match server_conn.write(&[]).await {
                Ok(_) => {}
                Err(err) => {
                    if err.kind() == io::ErrorKind::BrokenPipe {
                        break;
                    }
                }
            }
        }
    }

    #[gpui::test]
    async fn test_io_error(cx: gpui::TestAppContext) {
        let executor = cx.read(|app| app.background_executor().clone());
        let socket_dir_path = TempDir::new("io-error").unwrap();
        let socket_path = socket_dir_path.path().join(".sock");
        let _listener = UnixListener::bind(&socket_path).unwrap();
        let mut client_conn = UnixStream::connect(&socket_path).await.unwrap();
        client_conn.close().await.unwrap();

        let client = RpcClient::new(client_conn, executor.clone());
        let err = client
            .request(proto::from_client::Auth {
                user_id: 42,
                access_token: "token".to_string(),
            })
            .await
            .unwrap_err();
        assert_eq!(
            err.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::BrokenPipe
        );
    }

    async fn send_recv<S, R, O>(mut sender: S, receiver: R) -> O
    where
        S: Unpin + Future,
        R: Future<Output = O>,
    {
        smol::pin!(receiver);
        loop {
            poll_once(&mut sender).await;
            match poll_once(&mut receiver).await {
                Some(message) => break message,
                None => continue,
            }
        }
    }
}
