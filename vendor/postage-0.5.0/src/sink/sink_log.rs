use crate::sink::{PollSend, Sink};
use log::log_enabled;
use pin_project::pin_project;
use std::{fmt::Debug, pin::Pin};

use crate::Context;
#[pin_project]
pub struct SinkLog<S> {
    #[pin]
    sink: S,
    type_name: &'static str,
    level: log::Level,
}

impl<S> SinkLog<S>
where
    S: Sink,
{
    pub fn new(sink: S, level: log::Level) -> Self {
        let type_name = std::any::type_name::<S::Item>();
        SinkLog {
            sink,
            type_name,
            level,
        }
    }
}

impl<S> Sink for SinkLog<S>
where
    S: Sink,
    S::Item: Debug,
{
    type Item = S::Item;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        let this = self.project();
        let level = *this.level;

        let debug_repr = if log_enabled!(level) {
            Some(format!("<{}> {:?}", this.type_name, &value))
        } else {
            None
        };

        match this.sink.poll_send(cx, value) {
            PollSend::Ready => {
                if let Some(msg) = debug_repr {
                    log::log!(level, "{}", msg);
                }
                PollSend::Ready
            }
            PollSend::Pending(v) => PollSend::Pending(v),
            PollSend::Rejected(v) => PollSend::Rejected(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use log::Level;

    use crate::test::sink::*;
    use crate::{
        sink::{PollSend, Sink},
        Context,
    };

    use super::SinkLog;

    #[derive(Debug, Clone, PartialEq)]
    struct Message(usize);

    #[derive(Debug, Clone, PartialEq)]
    enum MessageEnum {
        Variant(usize),
    }

    #[test]
    fn ready_forwarded() {
        crate::logging::enable_log();
        let mut repeat = SinkLog::new(ready(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut repeat).poll_send(&mut cx, Message(1usize))
        );
    }

    #[test]
    fn ready_enum() {
        crate::logging::enable_log();
        let mut repeat = SinkLog::new(ready(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut repeat).poll_send(&mut cx, MessageEnum::Variant(1usize))
        );
    }

    #[test]
    fn pending_forwarded() {
        crate::logging::enable_log();
        let mut repeat = SinkLog::new(pending(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Pending(Message(1usize)),
            Pin::new(&mut repeat).poll_send(&mut cx, Message(1usize))
        );
    }

    #[test]
    fn closed_forwarded() {
        crate::logging::enable_log();
        let mut repeat = SinkLog::new(rejected(), Level::Info);
        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Rejected(Message(1usize)),
            Pin::new(&mut repeat).poll_send(&mut cx, Message(1usize))
        );
    }
}
