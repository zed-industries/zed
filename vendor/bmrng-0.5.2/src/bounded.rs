use crate::error::{ReceiveError, RequestError, RespondError, SendError};

use tokio::sync::{mpsc, oneshot};
use tokio::time::{timeout, Duration};

use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// The internal data sent in the MPSC request channel, a tuple that contains the request and the oneshot response channel responder
pub type Payload<Req, Res> = (Req, Responder<Res>);

/// Send values to the associated [`RequestReceiver`].
#[derive(Debug)]
pub struct RequestSender<Req, Res> {
    request_sender: mpsc::Sender<Payload<Req, Res>>,
    timeout_duration: Option<Duration>,
}

/// Receive requests values from the associated [`RequestSender`]
///
/// Instances are created by the [`channel`] function.
#[derive(Debug)]
pub struct RequestReceiver<Req, Res> {
    request_receiver: mpsc::Receiver<Payload<Req, Res>>,
}

/// Send values back to the [`RequestSender`] or [`RequestReceiver`]
///
/// Instances are created by calling [`RequestSender::send_receive()`] or [`RequestSender::send()`]
#[derive(Debug)]
pub struct Responder<Res> {
    response_sender: oneshot::Sender<Res>,
}

/// Receive responses from a [`Responder`]
///
/// Instances are created by calling [`RequestSender::send_receive()`] or [`RequestSender::send()`]
#[derive(Debug)]
pub struct ResponseReceiver<Res> {
    pub(crate) response_receiver: Option<oneshot::Receiver<Res>>,
    pub(crate) timeout_duration: Option<Duration>,
}

impl<Req, Res> RequestSender<Req, Res> {
    fn new(
        request_sender: mpsc::Sender<Payload<Req, Res>>,
        timeout_duration: Option<Duration>,
    ) -> Self {
        RequestSender {
            request_sender,
            timeout_duration,
        }
    }

    /// Send a request over the MPSC channel, open the response channel
    ///
    /// Return the [`ResponseReceiver`] which can be used to wait for a response
    ///
    /// This call waits if the request channel is full. It does not wait for a response
    pub async fn send(&self, request: Req) -> Result<ResponseReceiver<Res>, SendError<Req>> {
        let (response_sender, response_receiver) = oneshot::channel::<Res>();
        let responder = Responder::new(response_sender);
        let payload = (request, responder);
        self.request_sender
            .send(payload)
            .await
            .map_err(|payload| SendError(payload.0 .0))?;
        let receiver = ResponseReceiver::new(response_receiver, self.timeout_duration);
        Ok(receiver)
    }

    /// Send a request over the MPSC channel, wait for the response and return it
    ///
    /// This call waits if the request channel is full, and while waiting for the response
    pub async fn send_receive(&self, request: Req) -> Result<Res, RequestError<Req>> {
        let mut receiver = self.send(request).await?;
        receiver.recv().await.map_err(|err| err.into())
    }

    /// Checks if the channel has been closed.
    pub fn is_closed(&self) -> bool {
        self.request_sender.is_closed()
    }
}

impl<Req, Res> Clone for RequestSender<Req, Res> {
    fn clone(&self) -> Self {
        RequestSender {
            request_sender: self.request_sender.clone(),
            timeout_duration: self.timeout_duration,
        }
    }
}

impl<Req, Res> RequestReceiver<Req, Res> {
    fn new(receiver: mpsc::Receiver<Payload<Req, Res>>) -> Self {
        RequestReceiver {
            request_receiver: receiver,
        }
    }

    /// Receives the next value for this receiver.
    pub async fn recv(&mut self) -> Result<Payload<Req, Res>, RequestError<Req>> {
        match self.request_receiver.recv().await {
            Some(payload) => Ok(payload),
            None => Err(RequestError::RecvError),
        }
    }

    /// Closes the receiving half of a channel without dropping it.
    pub fn close(&mut self) {
        self.request_receiver.close()
    }

    /// Converts this receiver into a stream
    pub fn into_stream(self) -> impl Stream<Item = Payload<Req, Res>> {
        let stream: RequestReceiverStream<Req, Res> = self.into();
        stream
    }
}

impl<Res> ResponseReceiver<Res> {
    pub(crate) fn new(
        response_receiver: oneshot::Receiver<Res>,
        timeout_duration: Option<Duration>,
    ) -> Self {
        Self {
            response_receiver: Some(response_receiver),
            timeout_duration,
        }
    }

    /// Receives the next value for this receiver.
    ///
    /// If there is a `timeout_duration` set, and the sender takes longer than
    /// the timeout_duration to send the response, it aborts waiting and returns
    /// [`ReceiveError::TimeoutError`].
    pub async fn recv(&mut self) -> Result<Res, ReceiveError> {
        match self.response_receiver.take() {
            Some(response_receiver) => match self.timeout_duration {
                Some(duration) => match timeout(duration, response_receiver).await {
                    Ok(response_result) => response_result.map_err(|err| err.into()),
                    Err(..) => Err(ReceiveError::TimeoutError),
                },
                None => Ok(response_receiver.await?),
            },
            None => Err(ReceiveError::RecvError),
        }
    }
}

impl<Res> Responder<Res> {
    pub(crate) fn new(response_sender: oneshot::Sender<Res>) -> Self {
        Self { response_sender }
    }

    /// Responds a request from the [`RequestSender`] which finishes the request
    pub fn respond(self, response: Res) -> Result<(), RespondError<Res>> {
        self.response_sender.send(response).map_err(RespondError)
    }

    /// Checks if the associated receiver handle for the response listener has been dropped.
    pub fn is_closed(&self) -> bool {
        self.response_sender.is_closed()
    }
}

/// Creates a bounded mpsc request-response channel for communicating between
/// asynchronous tasks with backpressure
///
/// # Panics
///
/// Panics if the buffer capacity is 0, just like the Tokio MPSC channel
///
/// # Examples
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let buffer_size = 100;
///     let (tx, mut rx) = bmrng::channel::<i32, i32>(buffer_size);
///     tokio::spawn(async move {
///         while let Ok((input, mut responder)) = rx.recv().await {
///             if let Err(err) = responder.respond(input * input) {
///                 println!("sender dropped the response channel");
///             }
///         }
///     });
///     for i in 1..=10 {
///         if let Ok(response) = tx.send_receive(i).await {
///             println!("Requested {}, got {}", i, response);
///             assert_eq!(response, i * i);
///         }
///     }
/// }
/// ```
pub fn channel<Req, Res>(buffer: usize) -> (RequestSender<Req, Res>, RequestReceiver<Req, Res>) {
    let (sender, receiver) = mpsc::channel::<Payload<Req, Res>>(buffer);
    let request_sender = RequestSender::new(sender, None);
    let request_receiver = RequestReceiver::new(receiver);
    (request_sender, request_receiver)
}

/// Creates a bounded mpsc request-response channel for communicating between
/// asynchronous tasks with backpressure and a request timeout
///
/// # Panics
///
/// Panics if the buffer capacity is 0, just like the Tokio MPSC channel
///
/// # Examples
///
/// ```rust
/// use tokio::time::{Duration, sleep};
/// #[tokio::main]
/// async fn main() {
///     let (tx, mut rx) = bmrng::channel_with_timeout::<i32, i32>(100, Duration::from_millis(100));
///     tokio::spawn(async move {
///         match rx.recv().await {
///             Ok((input, mut responder)) => {
///                 sleep(Duration::from_millis(200)).await;
///                 let res = responder.respond(input * input);
///                 assert_eq!(res.is_ok(), true);
///             }
///             Err(err) => {
///                 println!("all request senders dropped");
///             }
///         }
///     });
///     let response = tx.send_receive(8).await;
///     assert_eq!(response, Err(bmrng::error::RequestError::<i32>::RecvTimeoutError));
/// }
/// ```
pub fn channel_with_timeout<Req, Res>(
    buffer: usize,
    timeout_duration: Duration,
) -> (RequestSender<Req, Res>, RequestReceiver<Req, Res>) {
    let (sender, receiver) = mpsc::channel::<Payload<Req, Res>>(buffer);
    let request_sender = RequestSender::new(sender, Some(timeout_duration));
    let request_receiver = RequestReceiver::new(receiver);
    (request_sender, request_receiver)
}

/// A wrapper around [`RequestReceiver`] that implements [`Stream`].
#[derive(Debug)]
pub struct RequestReceiverStream<Req, Res> {
    inner: RequestReceiver<Req, Res>,
}

impl<Req, Res> RequestReceiverStream<Req, Res> {
    /// Create a new `RequestReceiverStream`.
    pub fn new(recv: RequestReceiver<Req, Res>) -> Self {
        Self { inner: recv }
    }

    /// Get back the inner `Receiver`.
    #[cfg(not(tarpaulin_include))]
    pub fn into_inner(self) -> RequestReceiver<Req, Res> {
        self.inner
    }

    /// Closes the receiving half of a channel without dropping it.
    #[cfg(not(tarpaulin_include))]
    pub fn close(&mut self) {
        self.inner.close()
    }
}

impl<Req, Res> Stream for RequestReceiverStream<Req, Res> {
    type Item = Payload<Req, Res>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.request_receiver.poll_recv(cx)
    }
}

impl<Req, Res> AsRef<RequestReceiver<Req, Res>> for RequestReceiverStream<Req, Res> {
    #[cfg(not(tarpaulin_include))]
    fn as_ref(&self) -> &RequestReceiver<Req, Res> {
        &self.inner
    }
}

impl<Req, Res> AsMut<RequestReceiver<Req, Res>> for RequestReceiverStream<Req, Res> {
    #[cfg(not(tarpaulin_include))]
    fn as_mut(&mut self) -> &mut RequestReceiver<Req, Res> {
        &mut self.inner
    }
}

impl<Req, Res> From<RequestReceiver<Req, Res>> for RequestReceiverStream<Req, Res> {
    fn from(receiver: RequestReceiver<Req, Res>) -> Self {
        RequestReceiverStream::new(receiver)
    }
}
