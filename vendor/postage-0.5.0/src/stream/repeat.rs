use std::pin::Pin;

use crate::stream::{PollRecv, Stream};
use crate::Context;

pub struct RepeatStream<T> {
    data: T,
}

impl<T> RepeatStream<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Stream for RepeatStream<T>
where
    T: Clone,
{
    type Item = T;

    fn poll_recv(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        PollRecv::Ready(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use crate::{
        stream::{PollRecv, Stream},
        Context,
    };

    use super::RepeatStream;

    #[test]
    fn test() {
        let mut repeat = RepeatStream::new(1usize);
        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(1), Pin::new(&mut repeat).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(1), Pin::new(&mut repeat).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(1), Pin::new(&mut repeat).poll_recv(&mut cx));
    }
}
