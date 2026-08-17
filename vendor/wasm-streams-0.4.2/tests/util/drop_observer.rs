use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures_util::{Sink, Stream};
use pin_project::{pin_project, pinned_drop};

#[pin_project(PinnedDrop)]
pub struct DropObservable<S> {
    #[pin]
    inner: S,
    handle: Rc<RefCell<bool>>,
}

#[pinned_drop]
impl<T> PinnedDrop for DropObservable<T> {
    fn drop(self: Pin<&mut Self>) {
        *self.project().handle.borrow_mut() = true
    }
}

pub struct DropObserver {
    handle: Rc<RefCell<bool>>,
}

impl DropObserver {
    pub fn is_dropped(&self) -> bool {
        *self.handle.borrow()
    }
}

pub fn observe_drop<T>(inner: T) -> (DropObservable<T>, DropObserver) {
    let handle = Rc::new(RefCell::new(false));
    (
        DropObservable {
            inner,
            handle: handle.clone(),
        },
        DropObserver { handle },
    )
}

impl<S: Stream> Stream for DropObservable<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S, Item> Sink<Item> for DropObservable<S>
where
    S: Sink<Item>,
{
    type Error = S::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}
