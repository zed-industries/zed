use anyhow::{anyhow, Result};
use futures::FutureExt;
use gpui::executor::Background;
use parking_lot::Mutex;
use postage::{
    mpsc,
    prelude::{Sink, Stream},
};
use smol::prelude::{AsyncRead, AsyncWrite};
use std::{
    collections::HashMap,
    io,
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
    outgoing_tx: mpsc::Sender<proto::FromClient>,
    next_message_id: AtomicI32,
}

impl RpcClient {
    pub fn new<Conn>(conn: Conn, executor: Arc<Background>) -> Self
    where
        Conn: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let response_channels = Arc::new(Mutex::new(HashMap::new()));
        let (outgoing_tx, mut outgoing_rx) = mpsc::channel(32);

        {
            let response_channels = response_channels.clone();
            executor
                .spawn(async move {
                    let (conn_rx, conn_tx) = smol::io::split(conn);
                    let mut stream_tx = MessageStream::new(conn_tx);
                    let mut stream_rx = MessageStream::new(conn_rx);
                    loop {
                        futures::select! {
                            incoming = stream_rx.read_message::<proto::FromServer>().fuse() => {
                                Self::handle_incoming(incoming, &response_channels).await;
                            }
                            outgoing = outgoing_rx.recv().fuse() => {
                                if let Some(outgoing) = outgoing {
                                    stream_tx.write_message(&outgoing).await;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                })
                .detach();
        }

        Self {
            response_channels,
            outgoing_tx,
            next_message_id: AtomicI32::new(0),
        }
    }

    async fn handle_incoming(
        incoming: io::Result<proto::FromServer>,
        response_channels: &Mutex<HashMap<i32, (mpsc::Sender<proto::from_server::Variant>, bool)>>,
    ) {
        match incoming {
            Ok(incoming) => {
                if let Some(variant) = incoming.variant {
                    if let Some(request_id) = incoming.request_id {
                        let channel = response_channels.lock().remove(&request_id);
                        if let Some((mut tx, oneshot)) = channel {
                            if tx.send(variant).await.is_ok() {
                                if !oneshot {
                                    response_channels.lock().insert(request_id, (tx, false));
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
            Err(error) => log::warn!("invalid incoming RPC message {:?}", error),
        }
    }

    pub async fn request<T: RequestMessage>(&self, req: T) -> Result<T::Response> {
        let message_id = self.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
        let (tx, mut rx) = mpsc::channel(1);
        self.response_channels.lock().insert(message_id, (tx, true));
        self.outgoing_tx
            .clone()
            .send(proto::FromClient {
                id: message_id,
                variant: Some(req.to_variant()),
            })
            .await
            .unwrap();
        let response = rx
            .recv()
            .await
            .expect("response channel was unexpectedly dropped");
        T::Response::from_variant(response)
            .ok_or_else(|| anyhow!("received response of the wrong t"))
    }

    pub async fn send<T: SendMessage>(&self, message: T) -> Result<()> {
        let message_id = self.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
        self.outgoing_tx
            .clone()
            .send(proto::FromClient {
                id: message_id,
                variant: Some(message.to_variant()),
            })
            .await
            .unwrap();
        Ok(())
    }

    pub async fn subscribe<T: SubscribeMessage>(
        &mut self,
        subscription: T,
    ) -> Result<impl Stream<Item = Result<T::Event>>> {
        let message_id = self.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
        let (tx, rx) = mpsc::channel(256);
        self.response_channels
            .lock()
            .insert(message_id, (tx, false));
        self.outgoing_tx
            .clone()
            .send(proto::FromClient {
                id: message_id,
                variant: Some(subscription.to_variant()),
            })
            .await
            .unwrap();
        Ok(rx.map(|event| {
            T::Event::from_variant(event).ok_or_else(|| anyhow!("invalid event {:?}"))
        }))
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
        let socket_dir_path = TempDir::new("request-response-socket").unwrap();
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
    async fn test_drop_client(cx: gpui::TestAppContext) {
        let executor = cx.read(|app| app.background_executor().clone());
        let socket_dir_path = TempDir::new("request-response-socket").unwrap();
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
