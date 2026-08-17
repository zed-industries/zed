use crate::stream::{PollRecv, Stream};
use pin_project::pin_project;
use std::pin::Pin;

use crate::Context;
#[derive(Copy, Clone)]
enum State {
    Left,
    Right,
}

impl State {
    pub fn swap(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[pin_project]
pub struct MergeStream<Left, Right> {
    state: State,
    #[pin]
    left: Left,
    #[pin]
    right: Right,
}

impl<Left, Right> MergeStream<Left, Right>
where
    Left: Stream,
    Right: Stream<Item = Left::Item>,
{
    pub fn new(left: Left, right: Right) -> Self {
        Self {
            state: State::Left,
            left,
            right,
        }
    }
}

impl<Left, Right> Stream for MergeStream<Left, Right>
where
    Left: Stream,
    Right: Stream<Item = Left::Item>,
{
    type Item = Left::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.project();

        let poll = match this.state {
            State::Left => poll(this.left, this.right, cx),
            State::Right => poll(this.right, this.left, cx),
        };

        if poll.swap() {
            *this.state = this.state.swap();
        }

        poll.into_recv()
    }
}

enum MergePoll<T> {
    First(PollRecv<T>),
    Second(PollRecv<T>),
}

impl<T> MergePoll<T> {
    pub fn into_recv(self) -> PollRecv<T> {
        match self {
            MergePoll::First(p) => p,
            MergePoll::Second(p) => p,
        }
    }

    pub fn swap(&self) -> bool {
        match self {
            MergePoll::First(_) => true,
            MergePoll::Second(PollRecv::Ready(_)) => true,
            MergePoll::Second(PollRecv::Pending) => true,
            MergePoll::Second(PollRecv::Closed) => false,
        }
    }
}

fn poll<A, B>(first: Pin<&mut A>, second: Pin<&mut B>, cx: &mut Context<'_>) -> MergePoll<A::Item>
where
    A: Stream,
    B: Stream<Item = A::Item>,
{
    match first.poll_recv(cx) {
        PollRecv::Ready(v) => MergePoll::First(PollRecv::Ready(v)),
        PollRecv::Pending => MergePoll::Second(second.poll_recv(cx)),
        PollRecv::Closed => MergePoll::Second(second.poll_recv(cx)),
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

    use super::MergeStream;

    #[test]
    fn simple_merge() {
        let left = from_poll_iter(vec![PollRecv::Ready(1), PollRecv::Ready(3)]);
        let right = from_poll_iter(vec![PollRecv::Ready(2), PollRecv::Ready(4)]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(3), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(4), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn swap_ready() {
        let left = from_poll_iter(vec![PollRecv::Ready(1), PollRecv::Ready(3)]);
        let right = from_poll_iter(vec![PollRecv::Ready(2)]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(3), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn swap_pending() {
        let left = from_poll_iter(vec![PollRecv::Pending, PollRecv::Ready(2)]);
        let right = from_poll_iter(vec![PollRecv::Ready(1)]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn swap_closed() {
        let left = from_poll_iter(vec![PollRecv::Closed, PollRecv::Closed]);
        let right = from_poll_iter(vec![PollRecv::Ready(1), PollRecv::Ready(2)]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(2), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn pending_uses_right() {
        let left = from_poll_iter(vec![PollRecv::Pending]);
        let right = from_poll_iter(vec![PollRecv::Ready(1)]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn pending_uses_left() {
        let left = from_poll_iter(vec![PollRecv::Ready(1)]);
        let right = from_poll_iter(vec![PollRecv::Pending]);
        let mut find = MergeStream::new(left, right);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }
}
