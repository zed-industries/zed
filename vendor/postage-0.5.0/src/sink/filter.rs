use std::pin::Pin;

use crate::Context;

use crate::sink::{PollSend, Sink};
use pin_project::pin_project;

#[pin_project]
pub struct FilterSink<Filter, Into> {
    filter: Filter,
    #[pin]
    into: Into,
}

impl<Filter, Into> FilterSink<Filter, Into>
where
    Into: Sink,
    Filter: FnMut(&Into::Item) -> bool,
{
    pub fn new(filter: Filter, into: Into) -> Self {
        Self { filter, into }
    }
}

impl<Filter, Into> Sink for FilterSink<Filter, Into>
where
    Into: Sink,
    Filter: FnMut(&Into::Item) -> bool,
{
    type Item = Into::Item;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        let this = self.project();
        if !(this.filter)(&value) {
            return PollSend::Ready;
        }

        this.into.poll_send(cx, value)
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use crate::test::sink::*;
    use crate::{
        sink::{PollSend, Sink},
        Context,
    };

    use super::FilterSink;

    #[test]
    fn simple() {
        let mut test_sink = test_sink(vec![PollSend::Ready, PollSend::Ready]);
        let mut filter = FilterSink::new(|i| i % 2 == 0, &mut test_sink);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut filter).poll_send(&mut cx, 1usize)
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut filter).poll_send(&mut cx, 2usize)
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut filter).poll_send(&mut cx, 3usize)
        );
        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut filter).poll_send(&mut cx, 4usize)
        );

        drop(filter);

        assert_eq!(&[2, 4], test_sink.values());
    }

    #[test]
    fn forward_pending() {
        let source = pending::<usize>();
        let mut find = FilterSink::new(|_| true, source);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Pending(1),
            Pin::new(&mut find).poll_send(&mut cx, 1)
        );
    }

    #[test]
    fn forward_closed() {
        let source = rejected::<usize>();
        let mut find = FilterSink::new(|_| true, source);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Rejected(1),
            Pin::new(&mut find).poll_send(&mut cx, 1)
        );
    }

    #[test]
    fn ignored_ready() {
        let source = rejected::<usize>();
        let mut find = FilterSink::new(|_| false, source);

        let mut cx = Context::empty();

        assert_eq!(PollSend::Ready, Pin::new(&mut find).poll_send(&mut cx, 1));
    }
}
