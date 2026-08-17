use std::pin::Pin;

use crate::stream::{PollRecv, Stream};
use crate::Context;

pub struct FilterStream<From, Filter> {
    from: From,
    filter: Filter,
}

impl<From, Filter> FilterStream<From, Filter>
where
    From: Stream + Unpin,
    Filter: FnMut(&From::Item) -> bool + Unpin,
{
    pub fn new(from: From, filter: Filter) -> Self {
        Self { from, filter }
    }
}

impl<From, Filter> Stream for FilterStream<From, Filter>
where
    From: Stream + Unpin,
    Filter: FnMut(&From::Item) -> bool + Unpin,
{
    type Item = From::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.get_mut();
        loop {
            let from = Pin::new(&mut this.from);
            match from.poll_recv(cx) {
                PollRecv::Ready(value) => {
                    if (this.filter)(&value) {
                        return PollRecv::Ready(value);
                    }
                }
                PollRecv::Pending => return PollRecv::Pending,
                PollRecv::Closed => return PollRecv::Closed,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use crate::test::stream::*;
    use crate::{
        stream::{PollRecv, Stream},
        Context,
    };

    use super::FilterStream;

    #[test]
    fn filter() {
        let source = from_iter(vec![1, 2, 3, 4]);
        let mut find = FilterStream::new(source, |i| i % 2 == 0);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(4), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_pending() {
        let source = pending::<usize>();
        let mut find = FilterStream::new(source, |_| true);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Pending, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_closed() {
        let source = closed::<usize>();
        let mut find = FilterStream::new(source, |_| true);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }
}
