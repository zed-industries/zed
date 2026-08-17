use std::{marker::PhantomData, pin::Pin};

use crate::stream::{PollRecv, Stream};
use crate::Context;
use pin_project::pin_project;

#[pin_project]
pub struct MapStream<From, Map, Into> {
    #[pin]
    from: From,

    map: Map,
    into: PhantomData<Into>,
}

impl<From, Map, Into> MapStream<From, Map, Into>
where
    From: Stream,
    Map: Fn(From::Item) -> Into,
{
    pub fn new(from: From, map: Map) -> Self {
        Self {
            from,
            map,
            into: PhantomData,
        }
    }
}

impl<From, Map, Into> Stream for MapStream<From, Map, Into>
where
    From: Stream,
    Map: Fn(From::Item) -> Into,
{
    type Item = Into;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        let this = self.project();

        match this.from.poll_recv(cx) {
            PollRecv::Ready(v) => PollRecv::Ready((this.map)(v)),
            PollRecv::Pending => PollRecv::Pending,
            PollRecv::Closed => PollRecv::Closed,
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
    use std::convert::identity;

    use super::MapStream;

    #[test]
    fn map() {
        let source = from_iter(vec![1, 2, 3]);
        let mut find = MapStream::new(source, |i| i + 10);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Ready(11), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(12), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Ready(13), Pin::new(&mut find).poll_recv(&mut cx));
        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_pending() {
        let source = pending::<usize>();
        let mut find = MapStream::new(source, identity);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Pending, Pin::new(&mut find).poll_recv(&mut cx));
    }

    #[test]
    fn forward_closed() {
        let source = closed::<usize>();
        let mut find = MapStream::new(source, identity);

        let mut cx = Context::empty();

        assert_eq!(PollRecv::Closed, Pin::new(&mut find).poll_recv(&mut cx));
    }
}
