use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use tokio::sync::watch;

pub(crate) fn channel() -> (Signal, Watch) {
    let (tx, rx) = watch::channel(());
    (Signal { tx }, Watch { rx })
}

pub(crate) struct Signal {
    tx: watch::Sender<()>,
}

pub(crate) struct Draining(Pin<Box<dyn Future<Output = ()> + Send + Sync>>);

#[derive(Clone)]
pub(crate) struct Watch {
    rx: watch::Receiver<()>,
}

pin_project! {
    #[allow(missing_debug_implementations)]
    pub struct Watching<F, FN> {
        #[pin]
        future: F,
        state: State<FN>,
        watch: Pin<Box<dyn Future<Output = ()> + Send + Sync>>,
        _rx: watch::Receiver<()>,
    }
}

enum State<F> {
    Watch(F),
    Draining,
}

impl Signal {
    pub(crate) fn drain(self) -> Draining {
        let _ = self.tx.send(());
        Draining(Box::pin(async move { self.tx.closed().await }))
    }
}

impl Future for Draining {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.as_mut().0).poll(cx)
    }
}

impl Watch {
    pub(crate) fn watch<F, FN>(self, future: F, on_drain: FN) -> Watching<F, FN>
    where
        F: Future,
        FN: FnOnce(Pin<&mut F>),
    {
        let Self { mut rx } = self;
        let _rx = rx.clone();
        Watching {
            future,
            state: State::Watch(on_drain),
            watch: Box::pin(async move {
                let _ = rx.changed().await;
            }),
            // Keep the receiver alive until the future completes, so that
            // dropping it can signal that draining has completed.
            _rx,
        }
    }
}

impl<F, FN> Future for Watching<F, FN>
where
    F: Future,
    FN: FnOnce(Pin<&mut F>),
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        loop {
            match mem::replace(me.state, State::Draining) {
                State::Watch(on_drain) => {
                    match Pin::new(&mut me.watch).poll(cx) {
                        Poll::Ready(()) => {
                            // Drain has been triggered!
                            on_drain(me.future.as_mut());
                        }
                        Poll::Pending => {
                            *me.state = State::Watch(on_drain);
                            return me.future.poll(cx);
                        }
                    }
                }
                State::Draining => return me.future.poll(cx),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMe {
        draining: bool,
        finished: bool,
        poll_cnt: usize,
    }

    impl Future for TestMe {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            self.poll_cnt += 1;
            if self.finished {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    #[test]
    fn watch() {
        let mut mock = tokio_test::task::spawn(());
        mock.enter(|cx, _| {
            let (tx, rx) = channel();
            let fut = TestMe {
                draining: false,
                finished: false,
                poll_cnt: 0,
            };

            let mut watch = rx.watch(fut, |mut fut| {
                fut.draining = true;
            });

            assert_eq!(watch.future.poll_cnt, 0);

            // First poll should poll the inner future
            assert!(Pin::new(&mut watch).poll(cx).is_pending());
            assert_eq!(watch.future.poll_cnt, 1);

            // Second poll should poll the inner future again
            assert!(Pin::new(&mut watch).poll(cx).is_pending());
            assert_eq!(watch.future.poll_cnt, 2);

            let mut draining = tx.drain();
            // Drain signaled, but needs another poll to be noticed.
            assert!(!watch.future.draining);
            assert_eq!(watch.future.poll_cnt, 2);

            // Now, poll after drain has been signaled.
            assert!(Pin::new(&mut watch).poll(cx).is_pending());
            assert_eq!(watch.future.poll_cnt, 3);
            assert!(watch.future.draining);

            // Draining is not ready until watcher completes
            assert!(Pin::new(&mut draining).poll(cx).is_pending());

            // Finishing up the watch future
            watch.future.finished = true;
            assert!(Pin::new(&mut watch).poll(cx).is_ready());
            assert_eq!(watch.future.poll_cnt, 4);
            drop(watch);

            assert!(Pin::new(&mut draining).poll(cx).is_ready());
        })
    }

    #[test]
    fn watch_clones() {
        let mut mock = tokio_test::task::spawn(());
        mock.enter(|cx, _| {
            let (tx, rx) = channel();

            let fut1 = TestMe {
                draining: false,
                finished: false,
                poll_cnt: 0,
            };
            let fut2 = TestMe {
                draining: false,
                finished: false,
                poll_cnt: 0,
            };

            let watch1 = rx.clone().watch(fut1, |mut fut| {
                fut.draining = true;
            });
            let watch2 = rx.watch(fut2, |mut fut| {
                fut.draining = true;
            });

            let mut draining = tx.drain();

            // Still 2 outstanding watchers
            assert!(Pin::new(&mut draining).poll(cx).is_pending());

            // drop 1 for whatever reason
            drop(watch1);

            // Still not ready, 1 other watcher still pending
            assert!(Pin::new(&mut draining).poll(cx).is_pending());

            drop(watch2);

            // Now all watchers are gone, draining is complete
            assert!(Pin::new(&mut draining).poll(cx).is_ready());
        });
    }
}
