use std::{cell::UnsafeCell, pin::Pin};

use atomic::{Atomic, Ordering};

use crate::stream::{PollRecv, Stream};

use crate::Context;
#[derive(Copy, Clone)]
enum State {
    Ready,
    Taken,
}

pub struct OnceStream<T> {
    state: Atomic<State>,
    data: UnsafeCell<Option<T>>,
}

impl<T> OnceStream<T> {
    pub fn new(item: T) -> Self {
        Self {
            state: Atomic::new(State::Ready),
            data: UnsafeCell::new(Some(item)),
        }
    }
}

impl<T> Stream for OnceStream<T> {
    type Item = T;

    fn poll_recv(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        if self
            .state
            .compare_exchange(
                State::Ready,
                State::Taken,
                Ordering::AcqRel,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            let value = unsafe {
                let reference = self.data.get().as_mut().unwrap();
                reference.take().unwrap()
            };

            return PollRecv::Ready(value);
        }

        PollRecv::Closed
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use crate::{
        stream::{PollRecv, Stream},
        Context,
    };

    #[test]
    fn test() {
        let mut repeat = crate::stream::once(1usize);
        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut repeat).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut repeat).poll_recv(&mut cx));
    }
}
