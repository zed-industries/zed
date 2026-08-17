use std::borrow::Cow;
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
use std::convert::Infallible;
#[cfg(feature = "stream")]
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_channel::mpsc;
use futures_channel::oneshot;
use futures_core::Stream; // for mpsc::Receiver
#[cfg(feature = "stream")]
use futures_util::TryStreamExt;
use http::HeaderMap;
use http_body::{Body as HttpBody, SizeHint};

use super::DecodedLength;
#[cfg(feature = "stream")]
use crate::common::sync_wrapper::SyncWrapper;
use crate::common::watch;
#[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
use crate::proto::h2::ping;

type BodySender = mpsc::Sender<Result<Bytes, crate::Error>>;
type TrailersSender = oneshot::Sender<HeaderMap>;

/// A stream of `Bytes`, used when receiving bodies.
///
/// A good default [`HttpBody`](crate::body::HttpBody) to use in many
/// applications.
///
/// Note: To read the full body, use [`body::to_bytes`](crate::body::to_bytes())
/// or [`body::aggregate`](crate::body::aggregate()).
#[must_use = "streams do nothing unless polled"]
pub struct Body {
    kind: Kind,
    /// Keep the extra bits in an `Option<Box<Extra>>`, so that
    /// Body stays small in the common case (no extras needed).
    extra: Option<Box<Extra>>,
}

enum Kind {
    Once(Option<Bytes>),
    Chan {
        content_length: DecodedLength,
        want_tx: watch::Sender,
        data_rx: mpsc::Receiver<Result<Bytes, crate::Error>>,
        trailers_rx: oneshot::Receiver<HeaderMap>,
    },
    #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
    H2 {
        ping: ping::Recorder,
        content_length: DecodedLength,
        recv: h2::RecvStream,
    },
    #[cfg(feature = "ffi")]
    Ffi(crate::ffi::UserBody),
    #[cfg(feature = "stream")]
    Wrapped(
        SyncWrapper<
            Pin<Box<dyn Stream<Item = Result<Bytes, Box<dyn StdError + Send + Sync>>> + Send>>,
        >,
    ),
}

struct Extra {
    /// Allow the client to pass a future to delay the `Body` from returning
    /// EOF. This allows the `Client` to try to put the idle connection
    /// back into the pool before the body is "finished".
    ///
    /// The reason for this is so that creating a new request after finishing
    /// streaming the body of a response could sometimes result in creating
    /// a brand new connection, since the pool didn't know about the idle
    /// connection yet.
    delayed_eof: Option<DelayEof>,
}

#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
type DelayEofUntil = oneshot::Receiver<Infallible>;

enum DelayEof {
    /// Initial state, stream hasn't seen EOF yet.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    NotEof(DelayEofUntil),
    /// Transitions to this state once we've seen `poll` try to
    /// return EOF (`None`). This future is then polled, and
    /// when it completes, the Body finally returns EOF (`None`).
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    Eof(DelayEofUntil),
}

/// A sender half created through [`Body::channel()`].
///
/// Useful when wanting to stream chunks from another thread.
///
/// ## Body Closing
///
/// Note that the request body will always be closed normally when the sender is dropped (meaning
/// that the empty terminating chunk will be sent to the remote). If you desire to close the
/// connection with an incomplete response (e.g. in the case of an error during asynchronous
/// processing), call the [`Sender::abort()`] method to abort the body in an abnormal fashion.
///
/// [`Body::channel()`]: struct.Body.html#method.channel
/// [`Sender::abort()`]: struct.Sender.html#method.abort
#[must_use = "Sender does nothing unless sent on"]
pub struct Sender {
    want_rx: watch::Receiver,
    data_tx: BodySender,
    trailers_tx: Option<TrailersSender>,
}

const WANT_PENDING: usize = 1;
const WANT_READY: usize = 2;

impl Body {
    /// Create an empty `Body` stream.
    ///
    /// # Example
    ///
    /// ```
    /// use hyper::{Body, Request};
    ///
    /// // create a `GET /` request
    /// let get = Request::new(Body::empty());
    /// ```
    #[inline]
    pub fn empty() -> Body {
        Body::new(Kind::Once(None))
    }

    /// Create a `Body` stream with an associated sender half.
    ///
    /// Useful when wanting to stream chunks from another thread.
    #[inline]
    pub fn channel() -> (Sender, Body) {
        Self::new_channel(DecodedLength::CHUNKED, /*wanter =*/ false)
    }

    pub(crate) fn new_channel(content_length: DecodedLength, wanter: bool) -> (Sender, Body) {
        let (data_tx, data_rx) = mpsc::channel(0);
        let (trailers_tx, trailers_rx) = oneshot::channel();

        // If wanter is true, `Sender::poll_ready()` won't becoming ready
        // until the `Body` has been polled for data once.
        let want = if wanter { WANT_PENDING } else { WANT_READY };

        let (want_tx, want_rx) = watch::channel(want);

        let tx = Sender {
            want_rx,
            data_tx,
            trailers_tx: Some(trailers_tx),
        };
        let rx = Body::new(Kind::Chan {
            content_length,
            want_tx,
            data_rx,
            trailers_rx,
        });

        (tx, rx)
    }

    /// Wrap a futures `Stream` in a box inside `Body`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hyper::Body;
    /// let chunks: Vec<Result<_, std::io::Error>> = vec![
    ///     Ok("hello"),
    ///     Ok(" "),
    ///     Ok("world"),
    /// ];
    ///
    /// let stream = futures_util::stream::iter(chunks);
    ///
    /// let body = Body::wrap_stream(stream);
    /// ```
    ///
    /// # Optional
    ///
    /// This function requires enabling the `stream` feature in your
    /// `Cargo.toml`.
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    pub fn wrap_stream<S, O, E>(stream: S) -> Body
    where
        S: Stream<Item = Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<Box<dyn StdError + Send + Sync>> + 'static,
    {
        let mapped = stream.map_ok(Into::into).map_err(Into::into);
        Body::new(Kind::Wrapped(SyncWrapper::new(Box::pin(mapped))))
    }

    fn new(kind: Kind) -> Body {
        Body { kind, extra: None }
    }

    #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
    pub(crate) fn h2(
        recv: h2::RecvStream,
        mut content_length: DecodedLength,
        ping: ping::Recorder,
    ) -> Self {
        // If the stream is already EOS, then the "unknown length" is clearly
        // actually ZERO.
        if !content_length.is_exact() && recv.is_end_stream() {
            content_length = DecodedLength::ZERO;
        }
        let body = Body::new(Kind::H2 {
            ping,
            content_length,
            recv,
        });

        body
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(crate) fn delayed_eof(&mut self, fut: DelayEofUntil) {
        self.extra_mut().delayed_eof = Some(DelayEof::NotEof(fut));
    }

    fn take_delayed_eof(&mut self) -> Option<DelayEof> {
        self.extra
            .as_mut()
            .and_then(|extra| extra.delayed_eof.take())
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    fn extra_mut(&mut self) -> &mut Extra {
        self.extra
            .get_or_insert_with(|| Box::new(Extra { delayed_eof: None }))
    }

    fn poll_eof(&mut self, cx: &mut Context<'_>) -> Poll<Option<crate::Result<Bytes>>> {
        match self.take_delayed_eof() {
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "client")]
            Some(DelayEof::NotEof(mut delay)) => match self.poll_inner(cx) {
                ok @ Poll::Ready(Some(Ok(..))) | ok @ Poll::Pending => {
                    self.extra_mut().delayed_eof = Some(DelayEof::NotEof(delay));
                    ok
                }
                Poll::Ready(None) => match Pin::new(&mut delay).poll(cx) {
                    Poll::Ready(Ok(never)) => match never {},
                    Poll::Pending => {
                        self.extra_mut().delayed_eof = Some(DelayEof::Eof(delay));
                        Poll::Pending
                    }
                    Poll::Ready(Err(_done)) => Poll::Ready(None),
                },
                Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            },
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "client")]
            Some(DelayEof::Eof(mut delay)) => match Pin::new(&mut delay).poll(cx) {
                Poll::Ready(Ok(never)) => match never {},
                Poll::Pending => {
                    self.extra_mut().delayed_eof = Some(DelayEof::Eof(delay));
                    Poll::Pending
                }
                Poll::Ready(Err(_done)) => Poll::Ready(None),
            },
            #[cfg(any(
                not(any(feature = "http1", feature = "http2")),
                not(feature = "client")
            ))]
            Some(delay_eof) => match delay_eof {},
            None => self.poll_inner(cx),
        }
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn as_ffi_mut(&mut self) -> &mut crate::ffi::UserBody {
        match self.kind {
            Kind::Ffi(ref mut body) => return body,
            _ => {
                self.kind = Kind::Ffi(crate::ffi::UserBody::new());
            }
        }

        match self.kind {
            Kind::Ffi(ref mut body) => body,
            _ => unreachable!(),
        }
    }

    fn poll_inner(&mut self, cx: &mut Context<'_>) -> Poll<Option<crate::Result<Bytes>>> {
        match self.kind {
            Kind::Once(ref mut val) => Poll::Ready(val.take().map(Ok)),
            Kind::Chan {
                content_length: ref mut len,
                ref mut data_rx,
                ref mut want_tx,
                ..
            } => {
                want_tx.send(WANT_READY);

                match ready!(Pin::new(data_rx).poll_next(cx)?) {
                    Some(chunk) => {
                        len.sub_if(chunk.len() as u64);
                        Poll::Ready(Some(Ok(chunk)))
                    }
                    None => Poll::Ready(None),
                }
            }
            #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
            Kind::H2 {
                ref ping,
                recv: ref mut h2,
                content_length: ref mut len,
            } => match ready!(h2.poll_data(cx)) {
                Some(Ok(bytes)) => {
                    let _ = h2.flow_control().release_capacity(bytes.len());
                    len.sub_if(bytes.len() as u64);
                    ping.record_data(bytes.len());
                    Poll::Ready(Some(Ok(bytes)))
                }
                Some(Err(e)) => match e.reason() {
                    // These reasons should cause stop of body reading, but nor fail it.
                    // The same logic as for `AsyncRead for H2Upgraded` is applied here.
                    Some(h2::Reason::NO_ERROR) | Some(h2::Reason::CANCEL) => Poll::Ready(None),
                    _ => Poll::Ready(Some(Err(crate::Error::new_body(e)))),
                },
                None => Poll::Ready(None),
            },

            #[cfg(feature = "ffi")]
            Kind::Ffi(ref mut body) => body.poll_data(cx),

            #[cfg(feature = "stream")]
            Kind::Wrapped(ref mut s) => match ready!(s.get_mut().as_mut().poll_next(cx)) {
                Some(res) => Poll::Ready(Some(res.map_err(crate::Error::new_body))),
                None => Poll::Ready(None),
            },
        }
    }

    #[cfg(feature = "http1")]
    pub(super) fn take_full_data(&mut self) -> Option<Bytes> {
        if let Kind::Once(ref mut chunk) = self.kind {
            chunk.take()
        } else {
            None
        }
    }
}

impl Default for Body {
    /// Returns `Body::empty()`.
    #[inline]
    fn default() -> Body {
        Body::empty()
    }
}

impl HttpBody for Body {
    type Data = Bytes;
    type Error = crate::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        self.poll_eof(cx)
    }

    fn poll_trailers(
        #[cfg_attr(not(feature = "http2"), allow(unused_mut))] mut self: Pin<&mut Self>,
        #[cfg_attr(not(feature = "http2"), allow(unused))] cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        match self.kind {
            #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
            Kind::H2 {
                recv: ref mut h2,
                ref ping,
                ..
            } => match ready!(h2.poll_trailers(cx)) {
                Ok(t) => {
                    ping.record_non_data();
                    Poll::Ready(Ok(t))
                }
                Err(e) => Poll::Ready(Err(crate::Error::new_h2(e))),
            },
            Kind::Chan {
                ref mut trailers_rx,
                ..
            } => match ready!(Pin::new(trailers_rx).poll(cx)) {
                Ok(t) => Poll::Ready(Ok(Some(t))),
                Err(_) => Poll::Ready(Ok(None)),
            },
            #[cfg(feature = "ffi")]
            Kind::Ffi(ref mut body) => body.poll_trailers(cx),
            _ => Poll::Ready(Ok(None)),
        }
    }

    fn is_end_stream(&self) -> bool {
        match self.kind {
            Kind::Once(ref val) => val.is_none(),
            Kind::Chan { content_length, .. } => content_length == DecodedLength::ZERO,
            #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
            Kind::H2 { recv: ref h2, .. } => h2.is_end_stream(),
            #[cfg(feature = "ffi")]
            Kind::Ffi(..) => false,
            #[cfg(feature = "stream")]
            Kind::Wrapped(..) => false,
        }
    }

    fn size_hint(&self) -> SizeHint {
        macro_rules! opt_len {
            ($content_length:expr) => {{
                let mut hint = SizeHint::default();

                if let Some(content_length) = $content_length.into_opt() {
                    hint.set_exact(content_length);
                }

                hint
            }};
        }

        match self.kind {
            Kind::Once(Some(ref val)) => SizeHint::with_exact(val.len() as u64),
            Kind::Once(None) => SizeHint::with_exact(0),
            #[cfg(feature = "stream")]
            Kind::Wrapped(..) => SizeHint::default(),
            Kind::Chan { content_length, .. } => opt_len!(content_length),
            #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
            Kind::H2 { content_length, .. } => opt_len!(content_length),
            #[cfg(feature = "ffi")]
            Kind::Ffi(..) => SizeHint::default(),
        }
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        struct Streaming;
        #[derive(Debug)]
        struct Empty;
        #[derive(Debug)]
        struct Full<'a>(&'a Bytes);

        let mut builder = f.debug_tuple("Body");
        match self.kind {
            Kind::Once(None) => builder.field(&Empty),
            Kind::Once(Some(ref chunk)) => builder.field(&Full(chunk)),
            _ => builder.field(&Streaming),
        };

        builder.finish()
    }
}

/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
#[cfg(feature = "stream")]
impl Stream for Body {
    type Item = crate::Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        HttpBody::poll_data(self, cx)
    }
}

/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
#[cfg(feature = "stream")]
impl From<Box<dyn Stream<Item = Result<Bytes, Box<dyn StdError + Send + Sync>>> + Send>> for Body {
    #[inline]
    fn from(
        stream: Box<dyn Stream<Item = Result<Bytes, Box<dyn StdError + Send + Sync>>> + Send>,
    ) -> Body {
        Body::new(Kind::Wrapped(SyncWrapper::new(stream.into())))
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from(chunk: Bytes) -> Body {
        if chunk.is_empty() {
            Body::empty()
        } else {
            Body::new(Kind::Once(Some(chunk)))
        }
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(vec: Vec<u8>) -> Body {
        Body::from(Bytes::from(vec))
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(slice: &'static [u8]) -> Body {
        Body::from(Bytes::from(slice))
    }
}

impl From<Cow<'static, [u8]>> for Body {
    #[inline]
    fn from(cow: Cow<'static, [u8]>) -> Body {
        match cow {
            Cow::Borrowed(b) => Body::from(b),
            Cow::Owned(o) => Body::from(o),
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        Body::from(Bytes::from(s.into_bytes()))
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(slice: &'static str) -> Body {
        Body::from(Bytes::from(slice.as_bytes()))
    }
}

impl From<Cow<'static, str>> for Body {
    #[inline]
    fn from(cow: Cow<'static, str>) -> Body {
        match cow {
            Cow::Borrowed(b) => Body::from(b),
            Cow::Owned(o) => Body::from(o),
        }
    }
}

impl Sender {
    /// Check to see if this `Sender` can send more data.
    pub fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        // Check if the receiver end has tried polling for the body yet
        ready!(self.poll_want(cx)?);
        self.data_tx
            .poll_ready(cx)
            .map_err(|_| crate::Error::new_closed())
    }

    fn poll_want(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        match self.want_rx.load(cx) {
            WANT_READY => Poll::Ready(Ok(())),
            WANT_PENDING => Poll::Pending,
            watch::CLOSED => Poll::Ready(Err(crate::Error::new_closed())),
            unexpected => unreachable!("want_rx value: {}", unexpected),
        }
    }

    async fn ready(&mut self) -> crate::Result<()> {
        futures_util::future::poll_fn(|cx| self.poll_ready(cx)).await
    }

    /// Send data on data channel when it is ready.
    pub async fn send_data(&mut self, chunk: Bytes) -> crate::Result<()> {
        self.ready().await?;
        self.data_tx
            .try_send(Ok(chunk))
            .map_err(|_| crate::Error::new_closed())
    }

    /// Send trailers on trailers channel.
    pub async fn send_trailers(&mut self, trailers: HeaderMap) -> crate::Result<()> {
        let tx = match self.trailers_tx.take() {
            Some(tx) => tx,
            None => return Err(crate::Error::new_closed()),
        };
        tx.send(trailers).map_err(|_| crate::Error::new_closed())
    }

    /// Try to send data on this channel.
    ///
    /// # Errors
    ///
    /// Returns `Err(Bytes)` if the channel could not (currently) accept
    /// another `Bytes`.
    ///
    /// # Note
    ///
    /// This is mostly useful for when trying to send from some other thread
    /// that doesn't have an async context. If in an async context, prefer
    /// `send_data()` instead.
    pub fn try_send_data(&mut self, chunk: Bytes) -> Result<(), Bytes> {
        self.data_tx
            .try_send(Ok(chunk))
            .map_err(|err| err.into_inner().expect("just sent Ok"))
    }

    /// Aborts the body in an abnormal fashion.
    pub fn abort(mut self) {
        self.send_error(crate::Error::new_body_write_aborted());
    }

    pub(crate) fn send_error(&mut self, err: crate::Error) {
        let _ = self
            .data_tx
            // clone so the send works even if buffer is full
            .clone()
            .try_send(Err(err));
    }
}

impl fmt::Debug for Sender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        struct Open;
        #[derive(Debug)]
        struct Closed;

        let mut builder = f.debug_tuple("Sender");
        match self.want_rx.peek() {
            watch::CLOSED => builder.field(&Closed),
            _ => builder.field(&Open),
        };

        builder.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::task::Poll;

    use super::{Body, DecodedLength, HttpBody, Sender, SizeHint};

    #[test]
    fn test_size_of() {
        // These are mostly to help catch *accidentally* increasing
        // the size by too much.

        let body_size = mem::size_of::<Body>();
        let body_expected_size = mem::size_of::<u64>() * 6;
        assert!(
            body_size <= body_expected_size,
            "Body size = {} <= {}",
            body_size,
            body_expected_size,
        );

        assert_eq!(body_size, mem::size_of::<Option<Body>>(), "Option<Body>");

        assert_eq!(
            mem::size_of::<Sender>(),
            mem::size_of::<usize>() * 5,
            "Sender"
        );

        assert_eq!(
            mem::size_of::<Sender>(),
            mem::size_of::<Option<Sender>>(),
            "Option<Sender>"
        );
    }

    #[test]
    fn size_hint() {
        fn eq(body: Body, b: SizeHint, note: &str) {
            let a = body.size_hint();
            assert_eq!(a.lower(), b.lower(), "lower for {:?}", note);
            assert_eq!(a.upper(), b.upper(), "upper for {:?}", note);
        }

        eq(Body::from("Hello"), SizeHint::with_exact(5), "from str");

        eq(Body::empty(), SizeHint::with_exact(0), "empty");

        eq(Body::channel().1, SizeHint::new(), "channel");

        eq(
            Body::new_channel(DecodedLength::new(4), /*wanter =*/ false).1,
            SizeHint::with_exact(4),
            "channel with length",
        );
    }

    #[tokio::test]
    async fn channel_abort() {
        let (tx, mut rx) = Body::channel();

        tx.abort();

        let err = rx.data().await.unwrap().unwrap_err();
        assert!(err.is_body_write_aborted(), "{:?}", err);
    }

    #[tokio::test]
    async fn channel_abort_when_buffer_is_full() {
        let (mut tx, mut rx) = Body::channel();

        tx.try_send_data("chunk 1".into()).expect("send 1");
        // buffer is full, but can still send abort
        tx.abort();

        let chunk1 = rx.data().await.expect("item 1").expect("chunk 1");
        assert_eq!(chunk1, "chunk 1");

        let err = rx.data().await.unwrap().unwrap_err();
        assert!(err.is_body_write_aborted(), "{:?}", err);
    }

    #[test]
    fn channel_buffers_one() {
        let (mut tx, _rx) = Body::channel();

        tx.try_send_data("chunk 1".into()).expect("send 1");

        // buffer is now full
        let chunk2 = tx.try_send_data("chunk 2".into()).expect_err("send 2");
        assert_eq!(chunk2, "chunk 2");
    }

    #[tokio::test]
    async fn channel_empty() {
        let (_, mut rx) = Body::channel();

        assert!(rx.data().await.is_none());
    }

    #[test]
    fn channel_ready() {
        let (mut tx, _rx) = Body::new_channel(DecodedLength::CHUNKED, /*wanter = */ false);

        let mut tx_ready = tokio_test::task::spawn(tx.ready());

        assert!(tx_ready.poll().is_ready(), "tx is ready immediately");
    }

    #[test]
    fn channel_wanter() {
        let (mut tx, mut rx) = Body::new_channel(DecodedLength::CHUNKED, /*wanter = */ true);

        let mut tx_ready = tokio_test::task::spawn(tx.ready());
        let mut rx_data = tokio_test::task::spawn(rx.data());

        assert!(
            tx_ready.poll().is_pending(),
            "tx isn't ready before rx has been polled"
        );

        assert!(rx_data.poll().is_pending(), "poll rx.data");
        assert!(tx_ready.is_woken(), "rx poll wakes tx");

        assert!(
            tx_ready.poll().is_ready(),
            "tx is ready after rx has been polled"
        );
    }

    #[test]
    fn channel_notices_closure() {
        let (mut tx, rx) = Body::new_channel(DecodedLength::CHUNKED, /*wanter = */ true);

        let mut tx_ready = tokio_test::task::spawn(tx.ready());

        assert!(
            tx_ready.poll().is_pending(),
            "tx isn't ready before rx has been polled"
        );

        drop(rx);
        assert!(tx_ready.is_woken(), "dropping rx wakes tx");

        match tx_ready.poll() {
            Poll::Ready(Err(ref e)) if e.is_closed() => (),
            unexpected => panic!("tx poll ready unexpected: {:?}", unexpected),
        }
    }
}
