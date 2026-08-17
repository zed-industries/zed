//! An active neovim session.
use std::{
  future::Future,
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
};

use futures::{
  channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot,
  },
  future,
  io::{AsyncRead, AsyncReadExt, AsyncWrite},
  lock::Mutex,
  sink::SinkExt,
  stream::StreamExt,
  TryFutureExt,
};

use crate::{
  create::Spawner,
  error::{CallError, DecodeError, EncodeError, HandshakeError, LoopError},
  rpc::{
    handler::Handler,
    model,
    model::{IntoVal, RpcMessage},
  },
  uioptions::UiAttachOptions,
};
use rmpv::Value;

/// Pack the given arguments into a `Vec<Value>`, suitable for using it for a
/// [`call`](crate::neovim::Neovim::call) to neovim.
#[macro_export]
macro_rules! call_args {
    () => (Vec::new());
    ($($e:expr), +,) => (call_args![$($e),*]);
    ($($e:expr), +) => {{
        let vec = vec![
          $($e.into_val(),)*
        ];
        vec
    }};
}

type ResponseResult = Result<Result<Value, Value>, Arc<DecodeError>>;

type Queue = Arc<Mutex<Vec<(u64, oneshot::Sender<ResponseResult>)>>>;

/// An active Neovim session.
pub struct Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) writer: Arc<Mutex<W>>,
  pub(crate) queue: Queue,
  pub(crate) msgid_counter: Arc<AtomicU64>,
}

impl<W> Clone for Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn clone(&self) -> Self {
    Neovim {
      writer: self.writer.clone(),
      queue: self.queue.clone(),
      msgid_counter: self.msgid_counter.clone(),
    }
  }
}

impl<W> PartialEq for Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.writer, &other.writer)
  }
}
impl<W> Eq for Neovim<W> where W: AsyncWrite + Send + Unpin + 'static {}

impl<W> Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  #[allow(clippy::new_ret_no_self)]
  pub fn new<H, R>(
    reader: R,
    writer: W,
    handler: H,
  ) -> (
    Neovim<<H as Handler>::Writer>,
    impl Future<Output = Result<(), Box<LoopError>>>,
  )
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler<Writer = W> + Spawner,
  {
    let req = Neovim {
      writer: Arc::new(Mutex::new(writer)),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(Mutex::new(Vec::new())),
    };

    let (sender, receiver) = unbounded();
    let fut = future::try_join(
      req.clone().io_loop(reader, sender),
      req.clone().handler_loop(handler, receiver),
    )
    .map_ok(|_| ());

    (req, fut)
  }

  /// Create a new instance, immediately send a handshake message and
  /// wait for the response. Unlike `new`, this function is tolerant to extra
  /// data in the reader before the handshake response is received.
  ///
  /// `message` should be a unique string that is normally not found in the
  /// stdout. Due to the way Neovim packs strings, the length has to be either
  /// less than 20 characters or more than 31 characters long.
  /// See https://github.com/neovim/neovim/issues/32784 for more information.
  pub async fn handshake<H, R>(
    mut reader: R,
    writer: W,
    handler: H,
    message: &str,
  ) -> Result<
    (
      Neovim<<H as Handler>::Writer>,
      impl Future<Output = Result<(), Box<LoopError>>>,
    ),
    Box<HandshakeError>,
  >
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler<Writer = W> + Spawner,
  {
    let instance = Neovim {
      writer: Arc::new(Mutex::new(writer)),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(Mutex::new(Vec::new())),
    };

    let msgid = instance.msgid_counter.fetch_add(1, Ordering::SeqCst);
    // Nvim encodes fixed size strings with a length of 20-31 bytes wrong, so
    // avoid that
    let msg_len = message.len();
    assert!(
      !(20..=31).contains(&msg_len),
      "The message should be less than 20 characters or more than 31 characters
      long, but the length is {msg_len}."
    );

    let req = RpcMessage::RpcRequest {
      msgid,
      method: "nvim_exec_lua".to_owned(),
      params: call_args![format!("return '{message}'"), Vec::<Value>::new()],
    };
    model::encode(instance.writer.clone(), req).await?;

    let expected_resp = RpcMessage::RpcResponse {
      msgid,
      error: rmpv::Value::Nil,
      result: rmpv::Value::String(message.into()),
    };
    let mut expected_data = Vec::new();
    model::encode_sync(&mut expected_data, expected_resp)
      .expect("Encoding static data can't fail");
    let mut actual_data = Vec::new();
    let mut start = 0;
    let mut end = 0;
    while end - start != expected_data.len() {
      actual_data.resize(start + expected_data.len(), 0);

      let bytes_read =
        reader
          .read(&mut actual_data[start..])
          .await
          .map_err(|err| {
            (
              err,
              String::from_utf8_lossy(&actual_data[..end]).to_string(),
            )
          })?;
      if bytes_read == 0 {
        // The end of the stream has been reached when the reader returns Ok(0).
        // Since we haven't detected a suitable response yet, return an error.
        return Err(Box::new(HandshakeError::UnexpectedResponse(
          String::from_utf8_lossy(&actual_data[..end]).to_string(),
        )));
      }
      end += bytes_read;
      while end - start > 0 {
        if actual_data[start..end] == expected_data[..end - start] {
          break;
        }
        start += 1;
      }
    }

    let (sender, receiver) = unbounded();
    let fut = future::try_join(
      instance.clone().io_loop(reader, sender),
      instance.clone().handler_loop(handler, receiver),
    )
    .map_ok(|_| ());

    Ok((instance, fut))
  }

  async fn send_msg(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<oneshot::Receiver<ResponseResult>, Box<EncodeError>> {
    let msgid = self.msgid_counter.fetch_add(1, Ordering::SeqCst);

    let req = RpcMessage::RpcRequest {
      msgid,
      method: method.to_owned(),
      params: args,
    };

    let (sender, receiver) = oneshot::channel();

    self.queue.lock().await.push((msgid, sender));

    let writer = self.writer.clone();
    model::encode(writer, req).await?;

    Ok(receiver)
  }

  pub async fn call(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<Result<Value, Value>, Box<CallError>> {
    let receiver = self
      .send_msg(method, args)
      .await
      .map_err(|e| CallError::SendError(*e, method.to_string()))?;

    match receiver.await {
      // Result<Result<Result<Value, Value>, Arc<DecodeError>>, RecvError>
      Ok(Ok(r)) => Ok(r), // r is Result<Value, Value>, i.e. we got an answer
      Ok(Err(err)) => {
        // err is a Decode Error, i.e. the answer wasn't decodable
        Err(Box::new(CallError::DecodeError(err, method.to_string())))
      }
      Err(err) => {
        // err is RecvError
        Err(Box::new(CallError::InternalReceiveError(
          err,
          method.to_string(),
        )))
      }
    }
  }

  async fn send_error_to_callers(
    &self,
    queue: &Queue,
    err: DecodeError,
  ) -> Result<Arc<DecodeError>, Box<LoopError>> {
    let err = Arc::new(err);
    let mut v: Vec<u64> = vec![];

    let mut queue = queue.lock().await;
    queue.drain(0..).for_each(|sender| {
      let msgid = sender.0;
      sender
        .1
        .send(Err(err.clone()))
        .unwrap_or_else(|_| v.push(msgid));
    });

    if v.is_empty() {
      Ok(err)
    } else {
      Err((err, v).into())
    }
  }

  async fn handler_loop<H>(
    self,
    handler: H,
    mut receiver: UnboundedReceiver<RpcMessage>,
  ) -> Result<(), Box<LoopError>>
  where
    H: Handler<Writer = W> + Spawner,
  {
    loop {
      let msg = match receiver.next().await {
        Some(msg) => msg,
        /* If our receiver closes, that just means that io_handler started
         * shutting down. This is normal, so shut down along with it and don't
         * report an error
         */
        None => break Ok(()),
      };

      match msg {
        RpcMessage::RpcRequest {
          msgid,
          method,
          params,
        } => {
          let handler_c = handler.clone();
          let neovim = self.clone();
          let writer = self.writer.clone();

          handler.spawn(async move {
            let response = match handler_c
              .handle_request(method, params, neovim)
              .await
              {
                Ok(result) => RpcMessage::RpcResponse {
                  msgid,
                  result,
                  error: Value::Nil,
                },
                Err(error) => RpcMessage::RpcResponse {
                  msgid,
                  result: Value::Nil,
                  error,
                },
              };

            model::encode(writer, response)
              .await
              .unwrap_or_else(|e| {
                error!("Error sending response to request {}: '{}'", msgid, e);
              });
          });
        },
        RpcMessage::RpcNotification {
          method,
          params
        } => handler.handle_notify(method, params, self.clone()).await,
        RpcMessage::RpcResponse { .. } => unreachable!(),
      }
    }
  }

  async fn io_loop<R>(
    self,
    mut reader: R,
    mut sender: UnboundedSender<RpcMessage>,
  ) -> Result<(), Box<LoopError>>
  where
    R: AsyncRead + Send + Unpin + 'static,
  {
    let mut rest: Vec<u8> = vec![];

    loop {
      let msg = match model::decode(&mut reader, &mut rest).await {
        Ok(msg) => msg,
        Err(err) => {
          let e = self.send_error_to_callers(&self.queue, *err).await?;
          return Err(Box::new(LoopError::DecodeError(e, None)));
        }
      };

      debug!("Get message {:?}", msg);
      if let RpcMessage::RpcResponse { msgid, result, error, } = msg {
        let sender = find_sender(&self.queue, msgid).await?;
        if error == Value::Nil {
          sender
            .send(Ok(Ok(result)))
            .map_err(|r| (msgid, r.expect("This was an OK(_)")))?;
        } else {
          sender
            .send(Ok(Err(error)))
            .map_err(|r| (msgid, r.expect("This was an OK(_)")))?;
        }
      } else {
        // Send message to handler_loop()
        sender.send(msg).await.unwrap();
      }
    }
  }

  /// Register as a remote UI.
  ///
  /// After this method is called, the client will receive redraw notifications.
  pub async fn ui_attach(
    &self,
    width: i64,
    height: i64,
    opts: &UiAttachOptions,
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_ui_attach",
        call_args!(width, height, opts.to_value_map()),
      )
      .await?
      .map(|_| Ok(()))?
  }

  /// Send a quit command to Nvim.
  /// The quit command is 'qa!' which will make Nvim quit without
  /// saving anything.
  pub async fn quit_no_save(&self) -> Result<(), Box<CallError>> {
    self.command("qa!").await
  }
}

/* The idea to use Vec here instead of HashMap
 * is that Vec is faster on small queue sizes
 * in most cases Vec.len = 1 so we just take first item in iteration.
 */
async fn find_sender(
  queue: &Queue,
  msgid: u64,
) -> Result<oneshot::Sender<ResponseResult>, Box<LoopError>> {
  let mut queue = queue.lock().await;

  let pos = match queue.iter().position(|req| req.0 == msgid) {
    Some(p) => p,
    None => return Err(msgid.into()),
  };
  Ok(queue.remove(pos).1)
}

#[cfg(all(test, feature = "use_tokio"))]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_find_sender() {
    let queue = Arc::new(Mutex::new(Vec::new()));

    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((1, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((2, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((3, sender));
    }

    find_sender(&queue, 1).await.unwrap();
    assert_eq!(2, queue.lock().await.len());
    find_sender(&queue, 2).await.unwrap();
    assert_eq!(1, queue.lock().await.len());
    find_sender(&queue, 3).await.unwrap();
    assert!(queue.lock().await.is_empty());

    if let LoopError::MsgidNotFound(17) =
      *find_sender(&queue, 17).await.unwrap_err()
    {
    } else {
      panic!()
    }
  }
}
