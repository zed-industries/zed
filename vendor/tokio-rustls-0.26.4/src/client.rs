use std::future::Future;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};
use std::pin::Pin;
#[cfg(feature = "early-data")]
use std::task::Waker;
use std::task::{Context, Poll};
use std::{
    io::{self, BufRead as _},
    sync::Arc,
};

use rustls::{pki_types::ServerName, ClientConfig, ClientConnection};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, ReadBuf};

use crate::common::{IoSession, MidHandshake, Stream, TlsState};

/// A wrapper around a `rustls::ClientConfig`, providing an async `connect` method.
#[derive(Clone)]
pub struct TlsConnector {
    inner: Arc<ClientConfig>,
    #[cfg(feature = "early-data")]
    early_data: bool,
}

impl TlsConnector {
    /// Enable 0-RTT.
    ///
    /// If you want to use 0-RTT,
    /// You must also set `ClientConfig.enable_early_data` to `true`.
    #[cfg(feature = "early-data")]
    pub fn early_data(mut self, flag: bool) -> Self {
        self.early_data = flag;
        self
    }

    #[inline]
    pub fn connect<IO>(&self, domain: ServerName<'static>, stream: IO) -> Connect<IO>
    where
        IO: AsyncRead + AsyncWrite + Unpin,
    {
        self.connect_impl(domain, stream, None, |_| ())
    }

    #[inline]
    pub fn connect_with<IO, F>(&self, domain: ServerName<'static>, stream: IO, f: F) -> Connect<IO>
    where
        IO: AsyncRead + AsyncWrite + Unpin,
        F: FnOnce(&mut ClientConnection),
    {
        self.connect_impl(domain, stream, None, f)
    }

    fn connect_impl<IO, F>(
        &self,
        domain: ServerName<'static>,
        stream: IO,
        alpn_protocols: Option<Vec<Vec<u8>>>,
        f: F,
    ) -> Connect<IO>
    where
        IO: AsyncRead + AsyncWrite + Unpin,
        F: FnOnce(&mut ClientConnection),
    {
        let alpn = alpn_protocols.unwrap_or_else(|| self.inner.alpn_protocols.clone());
        let mut session = match ClientConnection::new_with_alpn(self.inner.clone(), domain, alpn) {
            Ok(session) => session,
            Err(error) => {
                return Connect(MidHandshake::Error {
                    io: stream,
                    // TODO(eliza): should this really return an `io::Error`?
                    // Probably not...
                    error: io::Error::new(io::ErrorKind::Other, error),
                });
            }
        };
        f(&mut session);

        Connect(MidHandshake::Handshaking(TlsStream {
            io: stream,

            #[cfg(not(feature = "early-data"))]
            state: TlsState::Stream,

            #[cfg(feature = "early-data")]
            state: if self.early_data && session.early_data().is_some() {
                TlsState::EarlyData(0, Vec::new())
            } else {
                TlsState::Stream
            },

            need_flush: false,

            #[cfg(feature = "early-data")]
            early_waker: None,

            session,
        }))
    }

    pub fn with_alpn(&self, alpn_protocols: Vec<Vec<u8>>) -> TlsConnectorWithAlpn<'_> {
        TlsConnectorWithAlpn {
            inner: self,
            alpn_protocols,
        }
    }

    /// Get a read-only reference to underlying config
    pub fn config(&self) -> &Arc<ClientConfig> {
        &self.inner
    }
}

impl From<Arc<ClientConfig>> for TlsConnector {
    fn from(inner: Arc<ClientConfig>) -> Self {
        Self {
            inner,
            #[cfg(feature = "early-data")]
            early_data: false,
        }
    }
}

pub struct TlsConnectorWithAlpn<'c> {
    inner: &'c TlsConnector,
    alpn_protocols: Vec<Vec<u8>>,
}

impl TlsConnectorWithAlpn<'_> {
    #[inline]
    pub fn connect<IO>(self, domain: ServerName<'static>, stream: IO) -> Connect<IO>
    where
        IO: AsyncRead + AsyncWrite + Unpin,
    {
        self.inner
            .connect_impl(domain, stream, Some(self.alpn_protocols), |_| ())
    }

    #[inline]
    pub fn connect_with<IO, F>(self, domain: ServerName<'static>, stream: IO, f: F) -> Connect<IO>
    where
        IO: AsyncRead + AsyncWrite + Unpin,
        F: FnOnce(&mut ClientConnection),
    {
        self.inner
            .connect_impl(domain, stream, Some(self.alpn_protocols), f)
    }
}

/// Future returned from `TlsConnector::connect` which will resolve
/// once the connection handshake has finished.
pub struct Connect<IO>(MidHandshake<TlsStream<IO>>);

impl<IO> Connect<IO> {
    #[inline]
    pub fn into_fallible(self) -> FallibleConnect<IO> {
        FallibleConnect(self.0)
    }

    pub fn get_ref(&self) -> Option<&IO> {
        match &self.0 {
            MidHandshake::Handshaking(sess) => Some(sess.get_ref().0),
            MidHandshake::SendAlert { io, .. } => Some(io),
            MidHandshake::Error { io, .. } => Some(io),
            MidHandshake::End => None,
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut IO> {
        match &mut self.0 {
            MidHandshake::Handshaking(sess) => Some(sess.get_mut().0),
            MidHandshake::SendAlert { io, .. } => Some(io),
            MidHandshake::Error { io, .. } => Some(io),
            MidHandshake::End => None,
        }
    }
}

impl<IO: AsyncRead + AsyncWrite + Unpin> Future for Connect<IO> {
    type Output = io::Result<TlsStream<IO>>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map_err(|(err, _)| err)
    }
}

impl<IO: AsyncRead + AsyncWrite + Unpin> Future for FallibleConnect<IO> {
    type Output = Result<TlsStream<IO>, (io::Error, IO)>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

/// Like [Connect], but returns `IO` on failure.
pub struct FallibleConnect<IO>(MidHandshake<TlsStream<IO>>);

/// A wrapper around an underlying raw stream which implements the TLS or SSL
/// protocol.
#[derive(Debug)]
pub struct TlsStream<IO> {
    pub(crate) io: IO,
    pub(crate) session: ClientConnection,
    pub(crate) state: TlsState,
    pub(crate) need_flush: bool,

    #[cfg(feature = "early-data")]
    pub(crate) early_waker: Option<Waker>,
}

impl<IO> TlsStream<IO> {
    #[inline]
    pub fn get_ref(&self) -> (&IO, &ClientConnection) {
        (&self.io, &self.session)
    }

    #[inline]
    pub fn get_mut(&mut self) -> (&mut IO, &mut ClientConnection) {
        (&mut self.io, &mut self.session)
    }

    #[inline]
    pub fn into_inner(self) -> (IO, ClientConnection) {
        (self.io, self.session)
    }
}

#[cfg(unix)]
impl<S> AsRawFd for TlsStream<S>
where
    S: AsRawFd,
{
    fn as_raw_fd(&self) -> RawFd {
        self.get_ref().0.as_raw_fd()
    }
}

#[cfg(windows)]
impl<S> AsRawSocket for TlsStream<S>
where
    S: AsRawSocket,
{
    fn as_raw_socket(&self) -> RawSocket {
        self.get_ref().0.as_raw_socket()
    }
}

impl<IO> IoSession for TlsStream<IO> {
    type Io = IO;
    type Session = ClientConnection;

    #[inline]
    fn skip_handshake(&self) -> bool {
        self.state.is_early_data()
    }

    #[inline]
    fn get_mut(&mut self) -> (&mut TlsState, &mut Self::Io, &mut Self::Session, &mut bool) {
        (
            &mut self.state,
            &mut self.io,
            &mut self.session,
            &mut self.need_flush,
        )
    }

    #[inline]
    fn into_io(self) -> Self::Io {
        self.io
    }
}

#[cfg(feature = "early-data")]
impl<IO> TlsStream<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_early_data(&mut self, cx: &mut Context<'_>) {
        // In the EarlyData state, we have not really established a Tls connection.
        // Before writing data through `AsyncWrite` and completing the tls handshake,
        // we ignore read readiness and return to pending.
        //
        // In order to avoid event loss,
        // we need to register a waker and wake it up after tls is connected.
        if self
            .early_waker
            .as_ref()
            .filter(|waker| cx.waker().will_wake(waker))
            .is_none()
        {
            self.early_waker = Some(cx.waker().clone());
        }
    }
}

impl<IO> AsyncRead for TlsStream<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let data = ready!(self.as_mut().poll_fill_buf(cx))?;
        let len = data.len().min(buf.remaining());
        buf.put_slice(&data[..len]);
        self.consume(len);
        Poll::Ready(Ok(()))
    }
}

impl<IO> AsyncBufRead for TlsStream<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        match self.state {
            #[cfg(feature = "early-data")]
            TlsState::EarlyData(..) => {
                self.get_mut().poll_early_data(cx);
                Poll::Pending
            }
            TlsState::Stream | TlsState::WriteShutdown => {
                let this = self.get_mut();
                let stream =
                    Stream::new(&mut this.io, &mut this.session).set_eof(!this.state.readable());

                match stream.poll_fill_buf(cx) {
                    Poll::Ready(Ok(buf)) => {
                        if buf.is_empty() {
                            this.state.shutdown_read();
                        }

                        Poll::Ready(Ok(buf))
                    }
                    Poll::Ready(Err(err)) if err.kind() == io::ErrorKind::ConnectionAborted => {
                        this.state.shutdown_read();
                        Poll::Ready(Err(err))
                    }
                    output => output,
                }
            }
            TlsState::ReadShutdown | TlsState::FullyShutdown => Poll::Ready(Ok(&[])),
        }
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        self.session.reader().consume(amt);
    }
}

impl<IO> AsyncWrite for TlsStream<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    /// Note: that it does not guarantee the final data to be sent.
    /// To be cautious, you must manually call `flush`.
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let mut stream = Stream::new(&mut this.io, &mut this.session)
            .set_eof(!this.state.readable())
            .set_need_flush(this.need_flush);

        #[cfg(feature = "early-data")]
        {
            let bufs = [io::IoSlice::new(buf)];
            let written = poll_handle_early_data(
                &mut this.state,
                &mut stream,
                &mut this.early_waker,
                cx,
                &bufs,
            )?;
            match written {
                Poll::Ready(0) => {}
                Poll::Ready(written) => return Poll::Ready(Ok(written)),
                Poll::Pending => {
                    this.need_flush = stream.need_flush;
                    return Poll::Pending;
                }
            }
        }

        stream.as_mut_pin().poll_write(cx, buf)
    }

    /// Note: that it does not guarantee the final data to be sent.
    /// To be cautious, you must manually call `flush`.
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let mut stream = Stream::new(&mut this.io, &mut this.session)
            .set_eof(!this.state.readable())
            .set_need_flush(this.need_flush);

        #[cfg(feature = "early-data")]
        {
            let written = poll_handle_early_data(
                &mut this.state,
                &mut stream,
                &mut this.early_waker,
                cx,
                bufs,
            )?;
            match written {
                Poll::Ready(0) => {}
                Poll::Ready(written) => return Poll::Ready(Ok(written)),
                Poll::Pending => {
                    this.need_flush = stream.need_flush;
                    return Poll::Pending;
                }
            }
        }

        stream.as_mut_pin().poll_write_vectored(cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let mut stream = Stream::new(&mut this.io, &mut this.session)
            .set_eof(!this.state.readable())
            .set_need_flush(this.need_flush);

        #[cfg(feature = "early-data")]
        {
            let written = poll_handle_early_data(
                &mut this.state,
                &mut stream,
                &mut this.early_waker,
                cx,
                &[],
            )?;
            if written.is_pending() {
                this.need_flush = stream.need_flush;
                return Poll::Pending;
            }
        }

        stream.as_mut_pin().poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        #[cfg(feature = "early-data")]
        {
            // complete handshake
            if matches!(self.state, TlsState::EarlyData(..)) {
                ready!(self.as_mut().poll_flush(cx))?;
            }
        }

        if self.state.writeable() {
            self.session.send_close_notify();
            self.state.shutdown_write();
        }

        let this = self.get_mut();
        let mut stream =
            Stream::new(&mut this.io, &mut this.session).set_eof(!this.state.readable());
        stream.as_mut_pin().poll_shutdown(cx)
    }
}

#[cfg(feature = "early-data")]
fn poll_handle_early_data<IO>(
    state: &mut TlsState,
    stream: &mut Stream<IO, ClientConnection>,
    early_waker: &mut Option<Waker>,
    cx: &mut Context<'_>,
    bufs: &[io::IoSlice<'_>],
) -> Poll<io::Result<usize>>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    if let TlsState::EarlyData(pos, data) = state {
        use std::io::Write;

        // write early data
        if let Some(mut early_data) = stream.session.early_data() {
            let mut written = 0;

            for buf in bufs {
                if buf.is_empty() {
                    continue;
                }

                let len = match early_data.write(buf) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(err) => return Poll::Ready(Err(err)),
                };

                written += len;
                data.extend_from_slice(&buf[..len]);

                if len < buf.len() {
                    break;
                }
            }

            if written != 0 {
                return Poll::Ready(Ok(written));
            }
        }

        // complete handshake
        while stream.session.is_handshaking() {
            ready!(stream.handshake(cx))?;
        }

        // write early data (fallback)
        if !stream.session.is_early_data_accepted() {
            while *pos < data.len() {
                let len = ready!(stream.as_mut_pin().poll_write(cx, &data[*pos..]))?;
                *pos += len;
            }
        }

        // end
        *state = TlsState::Stream;

        if let Some(waker) = early_waker.take() {
            waker.wake();
        }
    }

    Poll::Ready(Ok(0))
}
