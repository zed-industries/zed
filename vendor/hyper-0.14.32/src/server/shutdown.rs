use std::error::Error as StdError;
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::debug;

use super::accept::Accept;
use super::conn::UpgradeableConnection;
use super::server::{Server, Watcher};
use crate::body::{Body, HttpBody};
use crate::common::drain::{self, Draining, Signal, Watch, Watching};
use crate::common::exec::{ConnStreamExec, NewSvcExec};
use crate::service::{HttpService, MakeServiceRef};

pin_project! {
    #[allow(missing_debug_implementations)]
    pub struct Graceful<I, S, F, E> {
        #[pin]
        state: State<I, S, F, E>,
    }
}

pin_project! {
    #[project = StateProj]
    pub(super) enum State<I, S, F, E> {
        Running {
            drain: Option<(Signal, Watch)>,
            #[pin]
            server: Server<I, S, E>,
            #[pin]
            signal: F,
        },
        Draining { draining: Draining },
    }
}

impl<I, S, F, E> Graceful<I, S, F, E> {
    pub(super) fn new(server: Server<I, S, E>, signal: F) -> Self {
        let drain = Some(drain::channel());
        Graceful {
            state: State::Running {
                drain,
                server,
                signal,
            },
        }
    }
}

impl<I, IO, IE, S, B, F, E> Future for Graceful<I, S, F, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: MakeServiceRef<IO, Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    F: Future<Output = ()>,
    E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
    E: NewSvcExec<IO, S::Future, S::Service, E, GracefulWatcher>,
{
    type Output = crate::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        loop {
            let next = {
                match me.state.as_mut().project() {
                    StateProj::Running {
                        drain,
                        server,
                        signal,
                    } => match signal.poll(cx) {
                        Poll::Ready(()) => {
                            debug!("signal received, starting graceful shutdown");
                            let sig = drain.take().expect("drain channel").0;
                            State::Draining {
                                draining: sig.drain(),
                            }
                        }
                        Poll::Pending => {
                            let watch = drain.as_ref().expect("drain channel").1.clone();
                            return server.poll_watch(cx, &GracefulWatcher(watch));
                        }
                    },
                    StateProj::Draining { ref mut draining } => {
                        return Pin::new(draining).poll(cx).map(Ok);
                    }
                }
            };
            me.state.set(next);
        }
    }
}

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct GracefulWatcher(Watch);

impl<I, S, E> Watcher<I, S, E> for GracefulWatcher
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: HttpService<Body>,
    E: ConnStreamExec<S::Future, S::ResBody>,
    S::ResBody: 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Future =
        Watching<UpgradeableConnection<I, S, E>, fn(Pin<&mut UpgradeableConnection<I, S, E>>)>;

    fn watch(&self, conn: UpgradeableConnection<I, S, E>) -> Self::Future {
        self.0.clone().watch(conn, on_drain)
    }
}

fn on_drain<I, S, E>(conn: Pin<&mut UpgradeableConnection<I, S, E>>)
where
    S: HttpService<Body>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    S::ResBody: HttpBody + 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<S::Future, S::ResBody>,
{
    conn.graceful_shutdown()
}
