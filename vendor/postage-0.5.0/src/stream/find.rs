use std::pin::Pin;

use crate::Context;
use atomic::{Atomic, Ordering};

use crate::stream::{PollRecv, Stream};

#[derive(Copy, Clone)]
enum State {
    Reading,
    Closed,
}

pub struct FindStream<From, Condition> {
    state: Atomic<State>,
    from: From,
    condition: Condition,
}

impl<From, Condition> FindStream<From, Condition>
where
    From: Stream,
    Condition: Fn(&From::Item) -> bool,
{
    pub fn new(from: From, condition: Condition) -> Self {
        Self {
            state: Atomic::new(State::Reading),
            from,
            condition,
        }
    }
}

impl<From, Condition> Stream for FindStream<From, Condition>
where
    From: Stream + Unpin,
    Condition: Fn(&From::Item) -> bool + Unpin,
{
    type Item = From::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.get_mut();

        if let State::Closed = this.state.load(Ordering::Acquire) {
            return PollRecv::Closed;
        }

        loop {
            let from = Pin::new(&mut this.from);
            match from.poll_recv(cx) {
                PollRecv::Ready(value) => {
                    if (this.condition)(&value) {
                        this.state.store(State::Closed, Ordering::Release);

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

    use super::FindStream;

    #[test]
    fn find() {
        let source = from_iter(vec![1, 2, 3]);
        let mut find = FindStream::new(source, |i| *i == 2);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn find_none() {
        let source = from_iter(vec![1, 3]);
        let mut find = FindStream::new(source, |i| *i == 2);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn find_only_once() {
        let source = from_iter(vec![1, 2, 2]);
        let mut find = FindStream::new(source, |i| *i == 2);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_pending() {
        let source = pending::<usize>();
        let mut find = FindStream::new(source, |i| *i == 2);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Pending, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_closed() {
        let source = closed::<usize>();
        let mut find = FindStream::new(source, |i| *i == 2);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }
}
