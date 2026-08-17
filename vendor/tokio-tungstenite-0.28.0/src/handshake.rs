#[cfg(feature = "handshake")]
use crate::compat::SetWaker;
use crate::{compat::AllowStd, WebSocketStream};
use log::*;
use std::{
    future::Future,
    io::{Read, Write},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tungstenite::WebSocket;
#[cfg(feature = "handshake")]
use tungstenite::{
    handshake::{
        client::Response, server::Callback, HandshakeError as Error, HandshakeRole,
        MidHandshake as WsHandshake,
    },
    ClientHandshake, ServerHandshake,
};

pub(crate) async fn without_handshake<F, S>(stream: S, f: F) -> WebSocketStream<S>
where
    F: FnOnce(AllowStd<S>) -> WebSocket<AllowStd<S>> + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    let start = SkippedHandshakeFuture(Some(SkippedHandshakeFutureInner { f, stream }));

    let ws = start.await;

    WebSocketStream::new(ws)
}

struct SkippedHandshakeFuture<F, S>(Option<SkippedHandshakeFutureInner<F, S>>);
struct SkippedHandshakeFutureInner<F, S> {
    f: F,
    stream: S,
}

impl<F, S> Future for SkippedHandshakeFuture<F, S>
where
    F: FnOnce(AllowStd<S>) -> WebSocket<AllowStd<S>> + Unpin,
    S: Unpin,
    AllowStd<S>: Read + Write,
{
    type Output = WebSocket<AllowStd<S>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut().0.take().expect("future polled after completion");
        trace!("Setting context when skipping handshake");
        let stream = AllowStd::new(inner.stream, ctx.waker());

        Poll::Ready((inner.f)(stream))
    }
}

#[cfg(feature = "handshake")]
struct MidHandshake<Role: HandshakeRole>(Option<WsHandshake<Role>>);

#[cfg(feature = "handshake")]
enum StartedHandshake<Role: HandshakeRole> {
    Done(Role::FinalResult),
    Mid(WsHandshake<Role>),
}

#[cfg(feature = "handshake")]
struct StartedHandshakeFuture<F, S>(Option<StartedHandshakeFutureInner<F, S>>);
#[cfg(feature = "handshake")]
struct StartedHandshakeFutureInner<F, S> {
    f: F,
    stream: S,
}

#[cfg(feature = "handshake")]
async fn handshake<Role, F, S>(stream: S, f: F) -> Result<Role::FinalResult, Error<Role>>
where
    Role: HandshakeRole + Unpin,
    Role::InternalStream: SetWaker + Unpin,
    F: FnOnce(AllowStd<S>) -> Result<Role::FinalResult, Error<Role>> + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    let start = StartedHandshakeFuture(Some(StartedHandshakeFutureInner { f, stream }));

    match start.await? {
        StartedHandshake::Done(r) => Ok(r),
        StartedHandshake::Mid(s) => {
            let res: Result<Role::FinalResult, Error<Role>> = MidHandshake::<Role>(Some(s)).await;
            res
        }
    }
}

#[cfg(feature = "handshake")]
pub(crate) async fn client_handshake<F, S>(
    stream: S,
    f: F,
) -> Result<(WebSocketStream<S>, Response), Error<ClientHandshake<AllowStd<S>>>>
where
    F: FnOnce(
            AllowStd<S>,
        ) -> Result<
            <ClientHandshake<AllowStd<S>> as HandshakeRole>::FinalResult,
            Error<ClientHandshake<AllowStd<S>>>,
        > + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    let result = handshake(stream, f).await?;
    let (s, r) = result;
    Ok((WebSocketStream::new(s), r))
}

#[cfg(feature = "handshake")]
pub(crate) async fn server_handshake<C, F, S>(
    stream: S,
    f: F,
) -> Result<WebSocketStream<S>, Error<ServerHandshake<AllowStd<S>, C>>>
where
    C: Callback + Unpin,
    F: FnOnce(
            AllowStd<S>,
        ) -> Result<
            <ServerHandshake<AllowStd<S>, C> as HandshakeRole>::FinalResult,
            Error<ServerHandshake<AllowStd<S>, C>>,
        > + Unpin,
    S: AsyncRead + AsyncWrite + Unpin,
{
    let s: WebSocket<AllowStd<S>> = handshake(stream, f).await?;
    Ok(WebSocketStream::new(s))
}

#[cfg(feature = "handshake")]
impl<Role, F, S> Future for StartedHandshakeFuture<F, S>
where
    Role: HandshakeRole,
    Role::InternalStream: SetWaker + Unpin,
    F: FnOnce(AllowStd<S>) -> Result<Role::FinalResult, Error<Role>> + Unpin,
    S: Unpin,
    AllowStd<S>: Read + Write,
{
    type Output = Result<StartedHandshake<Role>, Error<Role>>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.0.take().expect("future polled after completion");
        trace!("Setting ctx when starting handshake");
        let stream = AllowStd::new(inner.stream, ctx.waker());

        match (inner.f)(stream) {
            Ok(r) => Poll::Ready(Ok(StartedHandshake::Done(r))),
            Err(Error::Interrupted(mid)) => Poll::Ready(Ok(StartedHandshake::Mid(mid))),
            Err(Error::Failure(e)) => Poll::Ready(Err(Error::Failure(e))),
        }
    }
}

#[cfg(feature = "handshake")]
impl<Role> Future for MidHandshake<Role>
where
    Role: HandshakeRole + Unpin,
    Role::InternalStream: SetWaker + Unpin,
{
    type Output = Result<Role::FinalResult, Error<Role>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut s = self.as_mut().0.take().expect("future polled after completion");

        let machine = s.get_mut();
        trace!("Setting context in handshake");
        machine.get_mut().set_waker(cx.waker());

        match s.handshake() {
            Ok(stream) => Poll::Ready(Ok(stream)),
            Err(Error::Failure(e)) => Poll::Ready(Err(Error::Failure(e))),
            Err(Error::Interrupted(mid)) => {
                self.0 = Some(mid);
                Poll::Pending
            }
        }
    }
}
