use crate::Context;
use pin_project::pin_project;
use std::{fmt::Debug, pin::Pin};

use log::log;

use super::{PollRecv, Stream};
#[pin_project]
pub struct StreamLog<S> {
    #[pin]
    stream: S,
    type_name: &'static str,
    level: log::Level,
}

impl<S> StreamLog<S>
where
    S: Stream,
{
    pub fn new(stream: S, level: log::Level) -> Self {
        let type_name = std::any::type_name::<S::Item>();
        StreamLog {
            stream,
            type_name,
            level,
        }
    }
}

impl<S> Stream for StreamLog<S>
where
    S: Stream,
    S::Item: Debug,
{
    type Item = S::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.project();
        match this.stream.poll_recv(cx) {
            PollRecv::Ready(value) => {
                log!(*this.level, "<{}> {:?}", this.type_name, &value);
                PollRecv::Ready(value)
            }
            PollRecv::Pending => PollRecv::Pending,
            PollRecv::Closed => PollRecv::Closed,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use log::Level;

    use crate::test::stream::*;
    use crate::{
        stream::{PollRecv, Stream},
        Context,
    };

    use super::StreamLog;

    #[derive(Debug, Clone, PartialEq)]
    struct Message(usize);

    #[derive(Debug, Clone, PartialEq)]
    enum MessageEnum {
        Variant(usize),
    }

    #[test]
    fn ready_forwarded() {
        crate::logging::enable_log();
        let mut repeat = StreamLog::new(ready(Message(1usize)), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollRecv::Ready(Message(1usize)),
            Pin::new(&mut repeat).poll_recv(&mut cx)
        );
    }

    #[test]
    fn ready_enum() {
        crate::logging::enable_log();
        let mut repeat = StreamLog::new(ready(MessageEnum::Variant(1usize)), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollRecv::Ready(MessageEnum::Variant(1usize)),
            Pin::new(&mut repeat).poll_recv(&mut cx)
        );
    }

    #[test]
    fn pending_forwarded() {
        crate::logging::enable_log();
        let mut repeat = StreamLog::new(pending::<usize>(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(PollRecv::Pending, Pin::new(&mut repeat).poll_recv(&mut cx));
    }

    #[test]
    fn closed_forwarded() {
        crate::logging::enable_log();
        let mut repeat = StreamLog::new(closed::<usize>(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut repeat).poll_recv(&mut cx));
    }
}
