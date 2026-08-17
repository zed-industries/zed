use std::task::Poll;

macro_rules! poll {
    ($self:ident, $cx:ident) => {{
        use crate::prelude::Stream;

        let mut cx = $cx.into();

        return match $self.poll_recv(&mut cx) {
            crate::stream::PollRecv::Ready(v) => Poll::Ready(Some(v)),
            crate::stream::PollRecv::Pending => Poll::Pending,
            crate::stream::PollRecv::Closed => Poll::Ready(None),
        };
    }};
}

impl futures::stream::Stream for crate::barrier::Receiver {
    type Item = ();

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

impl<T: Clone> futures::stream::Stream for crate::broadcast::Receiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

impl<T> futures::stream::Stream for crate::dispatch::Receiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

impl<T> futures::stream::Stream for crate::mpsc::Receiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

impl<T> futures::stream::Stream for crate::oneshot::Receiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

impl<T: Clone> futures::stream::Stream for crate::watch::Receiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        poll!(self, cx)
    }
}

#[cfg(test)]
mod sink_tests {
    use std::{pin::Pin, task::Poll};

    use crate::{barrier, dispatch, mpsc, oneshot, sink::SendError, watch};
    use futures::Sink;

    macro_rules! test_sink {
        ($chan:expr, $val:expr) => {
            let mut std_cx = futures_test::task::noop_context();

            let (mut tx, rx) = $chan;

            assert_eq!(
                Poll::Ready(Ok(())),
                Pin::new(&mut tx).poll_ready(&mut std_cx)
            );
            assert_eq!(Ok(()), Pin::new(&mut tx).start_send($val));

            assert_eq!(Poll::Pending, Pin::new(&mut tx).poll_ready(&mut std_cx));

            drop(rx);
            assert_eq!(
                Poll::Ready(Ok(())),
                Pin::new(&mut tx).poll_ready(&mut std_cx)
            );
            assert_eq!(Err(SendError($val)), Pin::new(&mut tx).start_send($val));
        };
    }

    macro_rules! test_sink_ready {
        ($chan:expr, $val:expr) => {
            let mut std_cx = futures_test::task::noop_context();

            let (mut tx, _rx) = $chan;

            assert_eq!(
                Poll::Ready(Ok(())),
                Pin::new(&mut tx).poll_ready(&mut std_cx)
            );
            assert_eq!(Ok(()), Pin::new(&mut tx).start_send($val));

            assert_eq!(
                Poll::Ready(Ok(())),
                Pin::new(&mut tx).poll_ready(&mut std_cx)
            );
            assert_eq!(Ok(()), Pin::new(&mut tx).start_send($val));
        };
    }

    #[test]
    fn barrier() {
        let mut std_cx = futures_test::task::noop_context();

        let (mut tx, _rx) = barrier::channel();

        assert_eq!(
            Poll::Ready(Ok(())),
            Pin::new(&mut tx).poll_ready(&mut std_cx)
        );
        assert_eq!(Ok(()), Pin::new(&mut tx).start_send(()));

        assert_eq!(
            Poll::Ready(Err(SendError(()))),
            Pin::new(&mut tx).poll_ready(&mut std_cx)
        );
        assert_eq!(Err(SendError(())), Pin::new(&mut tx).start_send(()));
    }

    // I couldn't implement the trait for the broadcast channel.
    // Values and Contexts are intricately linked to the internal structure of the broadcast channel.
    // Unfortunately, futures::Sink provides the Context and Item separately,
    // so there was no reasonable way to implement the futures trait.

    #[test]
    fn dispatch() {
        test_sink!(dispatch::channel(1), 1usize);
    }

    #[test]
    fn mpsc() {
        test_sink!(mpsc::channel(1), 1usize);
    }

    #[test]
    fn oneshot() {
        let mut std_cx = futures_test::task::noop_context();

        let (mut tx, rx) = oneshot::channel();

        assert_eq!(
            Poll::Ready(Ok(())),
            Pin::new(&mut tx).poll_ready(&mut std_cx)
        );
        assert_eq!(Ok(()), Pin::new(&mut tx).start_send(1usize));

        assert_eq!(
            Poll::Ready(Ok(())),
            Pin::new(&mut tx).poll_ready(&mut std_cx)
        );
        assert_eq!(Err(SendError(1usize)), Pin::new(&mut tx).start_send(1usize));

        drop(rx);

        assert_eq!(
            Poll::Ready(Ok(())),
            Pin::new(&mut tx).poll_ready(&mut std_cx)
        );
        assert_eq!(Err(SendError(1usize)), Pin::new(&mut tx).start_send(1usize));
    }

    #[test]
    fn watch() {
        test_sink_ready!(watch::channel(), 1usize);
    }
}

#[cfg(test)]
mod stream_tests {
    use std::{pin::Pin, task::Poll};

    use crate::{
        barrier, broadcast, dispatch, mpsc, oneshot,
        sink::{PollSend, Sink},
        watch,
    };
    use futures::Stream;

    macro_rules! test_stream {
        ($chan:expr, $val:expr) => {
            let mut std_cx = futures_test::task::noop_context();
            let mut cx = crate::test::noop_context();

            let (mut tx, mut rx) = $chan;
            assert_eq!(Poll::Pending, Pin::new(&mut rx).poll_next(&mut std_cx));

            assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, $val));

            assert_eq!(
                Poll::Ready(Some($val)),
                Pin::new(&mut rx).poll_next(&mut std_cx)
            );

            drop(tx);

            assert_eq!(Poll::Ready(None), Pin::new(&mut rx).poll_next(&mut std_cx));
        };
    }

    #[test]
    fn barrier() {
        let mut std_cx = futures_test::task::noop_context();
        let mut cx = crate::test::noop_context();

        let (mut tx, mut rx) = barrier::channel();
        assert_eq!(Poll::Pending, Pin::new(&mut rx).poll_next(&mut std_cx));

        assert_eq!(PollSend::Ready, Pin::new(&mut tx).poll_send(&mut cx, ()));

        assert_eq!(
            Poll::Ready(Some(())),
            Pin::new(&mut rx).poll_next(&mut std_cx)
        );

        drop(tx);

        assert_eq!(
            Poll::Ready(Some(())),
            Pin::new(&mut rx).poll_next(&mut std_cx)
        );
    }

    #[test]
    fn broadcast() {
        test_stream!(broadcast::channel(4), 1usize);
    }

    #[test]
    fn dispatch() {
        test_stream!(dispatch::channel(4), 1usize);
    }

    #[test]
    fn mpsc() {
        test_stream!(mpsc::channel(4), 1usize);
    }

    #[test]
    fn oneshot() {
        test_stream!(oneshot::channel(), 1usize);
    }

    #[test]
    fn watch() {
        let mut std_cx = futures_test::task::noop_context();
        let mut cx = crate::test::noop_context();

        let (mut tx, mut rx) = watch::channel();
        assert_eq!(
            Poll::Ready(Some(0usize)),
            Pin::new(&mut rx).poll_next(&mut std_cx)
        );

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut tx).poll_send(&mut cx, 1usize)
        );

        assert_eq!(
            Poll::Ready(Some(1usize)),
            Pin::new(&mut rx).poll_next(&mut std_cx)
        );

        drop(tx);

        assert_eq!(Poll::Ready(None), Pin::new(&mut rx).poll_next(&mut std_cx));
    }
}
