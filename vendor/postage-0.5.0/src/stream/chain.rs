use std::pin::Pin;

use atomic::{Atomic, Ordering};

use crate::stream::{PollRecv, Stream};
use crate::Context;
use pin_project::pin_project;

#[derive(Copy, Clone)]
enum State {
    Left,
    Right,
    Closed,
}

#[pin_project]
pub struct ChainStream<Left, Right> {
    state: Atomic<State>,
    #[pin]
    left: Left,
    #[pin]
    right: Right,
}

impl<Left, Right> ChainStream<Left, Right>
where
    Left: Stream,
    Right: Stream<Item = Left::Item>,
{
    pub fn new(left: Left, right: Right) -> Self {
        Self {
            state: Atomic::new(State::Left),
            left,
            right,
        }
    }
}

impl<Left, Right> Stream for ChainStream<Left, Right>
where
    Left: Stream,
    Right: Stream<Item = Left::Item>,
{
    type Item = Left::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.project();
        let mut state = this.state.load(Ordering::Acquire);

        if let State::Left = state {
            match this.left.poll_recv(cx) {
                PollRecv::Ready(v) => return PollRecv::Ready(v),
                PollRecv::Pending => return PollRecv::Pending,
                PollRecv::Closed => {
                    this.state.store(State::Right, Ordering::Release);
                    state = State::Right;
                }
            }
        }

        if let State::Right = state {
            match this.right.poll_recv(cx) {
                PollRecv::Ready(v) => return PollRecv::Ready(v),
                PollRecv::Pending => return PollRecv::Pending,
                PollRecv::Closed => {
                    this.state.store(State::Closed, Ordering::Release);
                    return PollRecv::Closed;
                }
            }
        }

        if let State::Closed = state {
            return PollRecv::Closed;
        }

        unreachable!();
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

    use super::ChainStream;

    #[test]
    fn chain() {
        let left = from_poll_iter(vec![PollRecv::Ready(1), PollRecv::Ready(2)]);
        let right = from_poll_iter(vec![PollRecv::Ready(3)]);
        let mut find = ChainStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(3), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn waits_for_right() {
        let left = from_poll_iter(vec![PollRecv::Pending]);
        let right = from_poll_iter(vec![PollRecv::Ready(1)]);
        let mut find = ChainStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Pending, Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn ignores_after_close() {
        let left = from_poll_iter(vec![PollRecv::Closed, PollRecv::Ready(1)]);
        let right = from_poll_iter(vec![PollRecv::Closed, PollRecv::Ready(2)]);
        let mut find = ChainStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }
}
