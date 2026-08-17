use crate::sink::{PollSend, Sink};
use crate::Context;
use atomic::{Atomic, Ordering};
use pin_project::pin_project;
use std::pin::Pin;

#[derive(Copy, Clone)]
enum State {
    WritingLeft,
    WritingRight,
    Closed,
}
#[pin_project]
pub struct ChainSink<Left, Right> {
    state: Atomic<State>,

    #[pin]
    left: Left,
    #[pin]
    right: Right,
}

impl<Left, Right> ChainSink<Left, Right>
where
    Left: Sink,
    Right: Sink<Item = Left::Item>,
{
    pub fn new(left: Left, right: Right) -> Self {
        Self {
            state: Atomic::new(State::WritingLeft),
            left,
            right,
        }
    }
}

impl<Left, Right> Sink for ChainSink<Left, Right>
where
    Left: Sink,
    Right: Sink<Item = Left::Item>,
{
    type Item = Left::Item;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut value: Self::Item,
    ) -> PollSend<Self::Item> {
        let this = self.project();
        let mut state = this.state.load(Ordering::Acquire);

        if let State::WritingLeft = state {
            match this.left.poll_send(cx, value) {
                PollSend::Ready => return PollSend::Ready,
                PollSend::Pending(value) => return PollSend::Pending(value),
                PollSend::Rejected(returned_value) => {
                    value = returned_value;
                    this.state.store(State::WritingRight, Ordering::Release);
                    state = State::WritingRight;
                }
            }
        }

        if let State::WritingRight = state {
            match this.right.poll_send(cx, value) {
                PollSend::Ready => return PollSend::Ready,
                PollSend::Pending(value) => return PollSend::Pending(value),
                PollSend::Rejected(returned_value) => {
                    value = returned_value;

                    this.state.store(State::Closed, Ordering::Release);
                    return PollSend::Rejected(value);
                }
            }
        }

        if let State::Closed = state {
            return PollSend::Rejected(value);
        }

        unreachable!();
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

    use super::ChainSink;

    #[test]
    fn simple() {
        let mut left = test_sink(vec![PollSend::Ready]);
        let mut right = test_sink(vec![PollSend::Ready]);
        let mut chain = ChainSink::new(&mut left, &mut right);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Ready,
            Pin::new(&mut chain).poll_send(&mut cx, 1usize)
        );
        assert_eq!(PollSend::Ready, Pin::new(&mut chain).poll_send(&mut cx, 2));
        assert_eq!(
            PollSend::Rejected(3),
            Pin::new(&mut chain).poll_send(&mut cx, 3)
        );

        drop(chain);

        assert_eq!(&[1], left.values());
        assert_eq!(&[2], right.values());
    }

    #[test]
    fn waits_for_right() {
        let mut left = test_sink(vec![PollSend::Pending(1)]);
        let mut right = test_sink(vec![PollSend::Ready]);
        let mut chain = ChainSink::new(&mut left, &mut right);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Pending(1),
            Pin::new(&mut chain).poll_send(&mut cx, 1usize)
        );
        assert_eq!(PollSend::Ready, Pin::new(&mut chain).poll_send(&mut cx, 2));
        assert_eq!(
            PollSend::Rejected(3),
            Pin::new(&mut chain).poll_send(&mut cx, 3)
        );

        drop(chain);

        assert_eq!(Vec::<usize>::new(), left.values());
        assert_eq!(&[2], right.values());
    }

    #[test]
    fn ignores_after_close() {
        let mut left = test_sink(vec![PollSend::Rejected(1), PollSend::Ready]);
        let mut right = test_sink(vec![PollSend::Rejected(1), PollSend::Ready]);
        let mut chain = ChainSink::new(&mut left, &mut right);

        let mut cx = Context::empty();

        assert_eq!(
            PollSend::Rejected(1),
            Pin::new(&mut chain).poll_send(&mut cx, 1usize)
        );
        assert_eq!(
            PollSend::Rejected(2),
            Pin::new(&mut chain).poll_send(&mut cx, 2)
        );
        assert_eq!(
            PollSend::Rejected(3),
            Pin::new(&mut chain).poll_send(&mut cx, 3)
        );
        assert_eq!(
            PollSend::Rejected(4),
            Pin::new(&mut chain).poll_send(&mut cx, 4)
        );

        drop(chain);

        assert_eq!(Vec::<usize>::new(), left.values());
        assert_eq!(Vec::<usize>::new(), right.values());
    }
}
