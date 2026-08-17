//! Adapt a [`Stream`] into an event source
//!
//! Only available with the `stream` cargo feature of `calloop`.
//!
//! The stream will be polled by the event loop, allowing the event source's
//! callback to handle each event from the stream. This allows streams that
//! are woken by events in a different thread to be naturally integrated into
//! a `calloop` event loop.

use futures_core::Stream;
use std::{
    fmt,
    pin::pin,
    sync::Arc,
    task::{Context, Wake, Waker},
};

use crate::{
    ping::{make_ping, Ping, PingError, PingSource},
    EventSource, Poll, PostAction, Readiness, Token, TokenFactory,
};

struct PingWaker(Ping);

impl Wake for PingWaker {
    fn wake(self: Arc<Self>) {
        self.0.ping();
    }

    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.ping();
    }
}

/// [`Stream`]-based event source.
#[derive(Debug)]
pub struct StreamSource<S: Stream + Unpin> {
    stream: S,
    source: PingSource,
    waker: Waker,
}

impl<S: Stream + Unpin> StreamSource<S> {
    /// Create event source for a [`Stream`].
    pub fn new(stream: S) -> crate::Result<Self> {
        let (ping, source) = make_ping()?;

        // Signal the ping source so the stream will be polled initially,
        // and the waker registered.
        ping.ping();

        let waker = Waker::from(Arc::new(PingWaker(ping)));

        Ok(Self {
            stream: stream,
            source,
            waker,
        })
    }
}

impl<S: Stream + Unpin> EventSource for StreamSource<S> {
    type Event = Option<S::Item>;
    type Metadata = ();
    type Ret = ();
    type Error = StreamError;

    fn process_events<F>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Option<S::Item>, &mut ()),
    {
        let mut context = Context::from_waker(&self.waker);
        let mut stream = pin!(&mut self.stream);
        let mut end_of_stream = false;
        let action = self
            .source
            .process_events(readiness, token, |(), &mut ()| {
                while let std::task::Poll::Ready(evt) = stream.as_mut().poll_next(&mut context) {
                    if let Some(evt) = evt {
                        callback(Some(evt), &mut ());
                    } else {
                        callback(None, &mut ());
                        end_of_stream = true;
                        break;
                    }
                }
            })?;
        if end_of_stream {
            Ok(PostAction::Remove)
        } else {
            Ok(action)
        }
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> crate::Result<()> {
        self.source.register(poll, token_factory)?;
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> crate::Result<()> {
        self.source.reregister(poll, token_factory)?;
        Ok(())
    }

    fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
        self.source.unregister(poll)?;
        Ok(())
    }
}

/// An error arising from processing events for a stream.
#[derive(Debug)]
pub struct StreamError(PingError);

impl fmt::Display for StreamError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for StreamError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<PingError> for StreamError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn from(err: PingError) -> Self {
        Self(err)
    }
}

#[cfg(test)]
mod tests {
    use futures::channel::mpsc;
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn channel_stream() {
        let mut event_loop = crate::EventLoop::<usize>::try_new().unwrap();

        let (mut sender, receiver) = mpsc::channel(5);

        let source = StreamSource::new(receiver).unwrap();
        let token = event_loop
            .handle()
            .insert_source(source, |evt, (), count| {
                if let Some(()) = evt {
                    *count += 1;
                }
            })
            .unwrap();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            sender.try_send(()).unwrap();
            thread::sleep(Duration::from_millis(100));
            sender.try_send(()).unwrap();
        });

        let mut count = 0;
        event_loop.dispatch(Duration::ZERO, &mut count).unwrap();
        event_loop.handle().update(&token);
        while count < 2 {
            event_loop.dispatch(Duration::ZERO, &mut count).unwrap();
        }
    }
}
