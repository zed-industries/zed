use anyhow::{anyhow, Result};
use gpui::executor::Background;
use parking_lot::Mutex;
use postage::{
    oneshot,
    prelude::{Sink, Stream},
};
use smol::prelude::{AsyncRead, AsyncWrite};
use std::{collections::HashMap, sync::Arc};
use zed_rpc::proto::{self, MessageStream, RequestMessage, SendMessage, ServerMessage};

pub struct RpcClient<Conn, ShutdownFn>
where
    ShutdownFn: FnMut(&mut Conn),
{
    stream: MessageStream<Conn>,
    response_channels: Arc<Mutex<HashMap<i32, oneshot::Sender<proto::from_server::Variant>>>>,
    next_message_id: i32,
    shutdown_fn: ShutdownFn,
}

impl<Conn, ShutdownFn> RpcClient<Conn, ShutdownFn>
where
    Conn: Clone + AsyncRead + AsyncWrite + Unpin + Send + 'static,
    ShutdownFn: FnMut(&mut Conn),
{
    pub fn new(conn: Conn, executor: Arc<Background>, shutdown_fn: ShutdownFn) -> Self {
        let response_channels = Arc::new(Mutex::new(HashMap::new()));

        let result = Self {
            next_message_id: 0,
            stream: MessageStream::new(conn.clone()),
            response_channels: response_channels.clone(),
            shutdown_fn,
        };

        executor
            .spawn::<Result<()>, _>(async move {
                let mut stream = MessageStream::new(conn);
                loop {
                    let message = stream.read_message::<proto::FromServer>().await?;
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
            })
            .detach();

        result
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

impl<Conn, ShutdownFn> Drop for RpcClient<Conn, ShutdownFn>
where
    ShutdownFn: FnMut(&mut Conn),
{
    fn drop(&mut self) {
        (self.shutdown_fn)(self.stream.inner_mut())
    }
}
