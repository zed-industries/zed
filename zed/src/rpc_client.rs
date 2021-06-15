use anyhow::{anyhow, Result};
use futures::future::Either;
use gpui::executor::Background;
use postage::{
    barrier, oneshot,
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
        atomic::{self, AtomicU32},
        Arc,
    },
};
use zed_rpc::proto::{self, EnvelopedMessage, MessageStream, RequestMessage};

pub struct RpcClient {
    response_channels: Arc<Mutex<HashMap<u32, oneshot::Sender<proto::Envelope>>>>,
    outgoing: Mutex<MessageStream<BoxedWriter>>,
    next_message_id: AtomicU32,
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
            next_message_id: AtomicU32::new(0),
        })
    }

    async fn handle_incoming<Conn>(
        conn: ReadHalf<Conn>,
        mut drop_rx: barrier::Receiver,
        response_channels: Arc<Mutex<HashMap<u32, oneshot::Sender<proto::Envelope>>>>,
    ) where
        Conn: AsyncRead + Unpin,
    {
        let dropped = drop_rx.recv();
        smol::pin!(dropped);

        let mut stream = MessageStream::new(conn);
        loop {
            let read_message = stream.read_message();
            smol::pin!(read_message);

            match futures::future::select(read_message, &mut dropped).await {
                Either::Left((Ok(incoming), _)) => {
                    if let Some(responding_to) = incoming.responding_to {
                        let channel = response_channels.lock().await.remove(&responding_to);
                        if let Some(mut tx) = channel {
                            tx.send(incoming).await.ok();
                        } else {
                            log::warn!(
                                "received RPC response to unknown request {}",
                                responding_to
                            );
                        }
                    } else {
                        // unprompted message from server
                    }
                }
                Either::Left((Err(error), _)) => {
                    log::warn!("received invalid RPC message {:?}", error);
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
            let (tx, mut rx) = oneshot::channel();
            this.response_channels.lock().await.insert(message_id, tx);
            this.outgoing
                .lock()
                .await
                .write_message(&req.into_envelope(message_id, None))
                .await?;
            let response = rx
                .recv()
                .await
                .expect("response channel was unexpectedly dropped");
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received response of the wrong t"))
        }
    }

    pub fn send<T: EnvelopedMessage>(
        self: &Arc<Self>,
        message: T,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            let message_id = this.next_message_id.fetch_add(1, atomic::Ordering::SeqCst);
            this.outgoing
                .lock()
                .await
                .write_message(&message.into_envelope(message_id, None))
                .await?;
            Ok(())
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

        let client_req = client.request(proto::Auth {
            user_id: 42,
            access_token: "token".to_string(),
        });
        smol::pin!(client_req);
        let server_req = send_recv(&mut client_req, server_stream.read_message())
            .await
            .unwrap();
        assert_eq!(
            server_req.payload,
            Some(proto::envelope::Payload::Auth(proto::Auth {
                user_id: 42,
                access_token: "token".to_string()
            }))
        );

        // Respond to another request to ensure requests are properly matched up.
        server_stream
            .write_message(
                &proto::AuthResponse {
                    credentials_valid: false,
                }
                .into_envelope(1000, Some(999)),
            )
            .await
            .unwrap();
        server_stream
            .write_message(
                &proto::AuthResponse {
                    credentials_valid: true,
                }
                .into_envelope(1001, Some(server_req.id)),
            )
            .await
            .unwrap();
        assert_eq!(
            client_req.await.unwrap(),
            proto::AuthResponse {
                credentials_valid: true
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
            .request(proto::Auth {
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
