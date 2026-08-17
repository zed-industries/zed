use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use futures_util::{Sink, Stream};
use pin_project::pin_project;

#[pin_project]
#[derive(Debug)]
pub struct SimpleChannel<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    closed: bool,
}

impl<T> SimpleChannel<T> {
    pub fn new() -> Self {
        SimpleChannel {
            queue: VecDeque::new(),
            waker: None,
            closed: false,
        }
    }
}

impl<T> Default for SimpleChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stream for SimpleChannel<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.queue.pop_front() {
            Some(item) => Poll::Ready(Some(item)),
            None if *this.closed => Poll::Ready(None),
            None => {
                *this.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.len(), None)
    }
}

impl<T> Sink<T> for SimpleChannel<T> {
    type Error = ();

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let this = self.project();
        this.queue.push_back(item);
        if let Some(waker) = this.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        *this.closed = true;
        if let Some(waker) = this.waker.take() {
            waker.wake();
        }
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use futures_util::future::join;
    use futures_util::stream::iter;
    use futures_util::{SinkExt, StreamExt};

    use super::*;

    #[tokio::test]
    async fn test_write_then_read() {
        let channel = SimpleChannel::<u32>::new();
        let (mut sink, mut stream) = channel.split();

        send_many(&mut sink, vec![1, 2, 3]).await.unwrap();
        assert_eq!(stream.next().await.unwrap(), 1);
        assert_eq!(stream.next().await.unwrap(), 2);

        send_many(&mut sink, vec![4, 5]).await.unwrap();
        assert_eq!(stream.next().await.unwrap(), 3);
        assert_eq!(stream.next().await.unwrap(), 4);
        assert_eq!(stream.next().await.unwrap(), 5);

        sink.close().await.unwrap();
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn test_read_then_write() {
        let channel = SimpleChannel::<u32>::new();
        let (mut sink, mut stream) = channel.split();

        join(
            async {
                assert_eq!(stream.next().await.unwrap(), 1);
                assert_eq!(stream.next().await.unwrap(), 2);
                assert_eq!(stream.next().await.unwrap(), 3);
            },
            async {
                send_many(&mut sink, vec![1, 2, 3, 4]).await.unwrap();
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_read_then_close() {
        let channel = SimpleChannel::<u32>::new();
        let (mut sink, mut stream) = channel.split();

        join(
            async {
                assert_eq!(stream.next().await, None);
            },
            async {
                sink.close().await.unwrap();
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_close_then_read() {
        let channel = SimpleChannel::<u32>::new();
        let (mut sink, stream) = channel.split();

        send_many(&mut sink, vec![1, 2, 3]).await.unwrap();
        sink.close().await.unwrap();

        // should still read items from queue
        assert_eq!(stream.collect::<Vec<_>>().await, vec![1, 2, 3]);
    }

    async fn send_many<T, Si>(
        sink: &mut Si,
        values: impl IntoIterator<Item = T>,
    ) -> Result<(), Si::Error>
    where
        Si: Sink<T> + Unpin,
    {
        sink.send_all(&mut iter(values).map(Ok)).await
    }
}
