use alloc::{collections::VecDeque, rc::Rc};
use core::{
    cell::RefCell,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use futures::Stream;

pub(crate) struct LocalChannel<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    closed: bool,
}

pub(crate) struct LocalReceiver<T> {
    channel: Rc<RefCell<LocalChannel<T>>>,
}

impl<T> Stream for LocalReceiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut channel = self.channel.borrow_mut();

        match channel.queue.pop_front() {
            Some(item) => Poll::Ready(Some(item)),
            None => {
                if channel.closed {
                    Poll::Ready(None)
                } else {
                    match &mut channel.waker {
                        Some(prev) => prev.clone_from(cx.waker()),
                        None => channel.waker = Some(cx.waker().clone()),
                    }
                    Poll::Pending
                }
            }
        }
    }
}

pub(crate) struct LocalSender<T> {
    channel: Rc<RefCell<LocalChannel<T>>>,
}

impl<T> LocalSender<T> {
    pub(crate) fn send(&self, item: T) {
        let mut channel = self.channel.borrow_mut();

        channel.queue.push_back(item);

        let _ = channel.waker.take().map(Waker::wake);
    }
}

impl<T> Drop for LocalSender<T> {
    fn drop(&mut self) {
        let mut channel = self.channel.borrow_mut();
        channel.closed = true;
        let _ = channel.waker.take().map(Waker::wake);
    }
}

pub(crate) fn local_channel<T>() -> (LocalSender<T>, LocalReceiver<T>) {
    let channel = Rc::new(RefCell::new(LocalChannel {
        queue: VecDeque::new(),
        waker: None,
        closed: false,
    }));

    (
        LocalSender {
            channel: channel.clone(),
        },
        LocalReceiver { channel },
    )
}
