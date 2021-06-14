use anyhow::{anyhow, Result};
use gpui::executor::Background;
use parking_lot::Mutex;
use postage::{
    oneshot,
    prelude::{Sink, Stream},
};
use smol::{
    future::FutureExt,
    io::WriteHalf,
    prelude::{AsyncRead, AsyncWrite},
};
use std::{collections::HashMap, sync::Arc};
use zed_rpc::proto::{self, MessageStream, RequestMessage, SendMessage, ServerMessage};

pub struct RpcClient<Conn> {
    stream: MessageStream<WriteHalf<Conn>>,
    response_channels: Arc<Mutex<HashMap<i32, oneshot::Sender<proto::from_server::Variant>>>>,
    next_message_id: i32,
    _drop_tx: oneshot::Sender<()>,
}

impl<Conn> RpcClient<Conn>
where
    Conn: Clone + AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(conn: Conn, executor: Arc<Background>) -> Self {
        let (conn_rx, conn_tx) = smol::io::split(conn);
        let (drop_tx, mut drop_rx) = oneshot::channel();
        let response_channels = Arc::new(Mutex::new(HashMap::new()));
        let client = Self {
            next_message_id: 0,
            stream: MessageStream::new(conn_tx),
            response_channels: response_channels.clone(),
            _drop_tx: drop_tx,
        };

        executor
            .spawn::<Result<()>, _>(async move {
                enum Message {
                    Message(proto::FromServer),
                    ClientDropped,
                }

                let mut stream = MessageStream::new(conn_rx);
                let client_dropped = async move {
                    assert!(drop_rx.recv().await.is_none());
                    Ok(Message::ClientDropped) as Result<_>
                };
                smol::pin!(client_dropped);
                loop {
                    let message = async {
                        Ok(Message::Message(
                            stream.read_message::<proto::FromServer>().await?,
                        ))
                    };

                    match message.race(&mut client_dropped).await? {
                        Message::Message(message) => {
                            if let Some(variant) = message.variant {
                                if let Some(request_id) = message.request_id {
                                    let tx = response_channels.lock().remove(&request_id);
                                    if let Some(mut tx) = tx {
                                        tx.send(variant).await?;
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
                        Message::ClientDropped => break Ok(()),
                    }
                }
            })
            .detach();

        client
    }

    pub async fn request<T: RequestMessage>(&mut self, req: T) -> Result<T::Response> {
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let (tx, mut rx) = oneshot::channel();
        self.response_channels.lock().insert(message_id, tx);

        self.stream
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

    pub async fn send<T: SendMessage>(_: T) -> Result<()> {
        todo!()
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
        let mut client = RpcClient::new(client_conn, executor.clone());

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
        let error = loop {
            match server_conn.write(&[0]).await {
                Ok(_) => continue,
                Err(err) => break err,
            }
        };
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
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
