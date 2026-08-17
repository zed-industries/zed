use futures::task::{waker_ref, ArcWake};
use futures::Stream;
use parking_lot::Mutex;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::pin::Pin;
use std::sync::atomic;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

pub(crate) struct QueueInner<S, K: Clone> {
    counter: atomic::AtomicUsize,
    ready_queue: BinaryHeap<ReadyEvent<K>>,
    streams: HashMap<K, Pin<Box<S>>>,
    waker: Option<Waker>,
}

impl<S, K: Clone + Eq + Hash> QueueInner<S, K> {
    pub fn insert(&mut self, k: K, s: S) {
        self.streams.insert(k.clone(), Box::pin(s));
        self.ready_queue.push(ReadyEvent {
            priority: self.counter.fetch_add(1, atomic::Ordering::Relaxed),
            key: k,
        });
        if let Some(w) = &self.waker {
            w.wake_by_ref();
        }
    }

    pub fn remove(&mut self, k: &K) {
        self.streams.remove(k);
    }
}

pub struct FairQueue<S, K: Clone> {
    block_on_no_clients: bool,
    inner: Arc<Mutex<QueueInner<S, K>>>,
}

#[derive(Clone)]
struct ReadyEvent<K: Clone> {
    priority: usize,
    key: K,
}

impl<K: Clone> PartialEq for ReadyEvent<K> {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}
impl<K: Clone> Eq for ReadyEvent<K> {}

impl<K: Clone> PartialOrd for ReadyEvent<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Clone> Ord for ReadyEvent<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

struct StreamWaker<S, K: Clone> {
    inner: Arc<Mutex<QueueInner<S, K>>>,
    event: ReadyEvent<K>,
}

impl<S, K> ArcWake for StreamWaker<S, K>
where
    S: Send,
    K: Clone + Send + Sync,
{
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let mut inner = arc_self.inner.lock();
        inner.ready_queue.push(arc_self.event.clone());
        if let Some(waker) = inner.waker.take() {
            waker.wake_by_ref();
        }
    }
}

impl<S, T, K> Stream for FairQueue<S, K>
where
    T: Send,
    S: Stream<Item = T> + Send + 'static,
    K: Eq + Hash + Unpin + Clone + Send + Sync + 'static,
{
    type Item = (K, T);

    #[allow(clippy::needless_continue)]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let fair_queue = self.get_mut();
        loop {
            let (event, mut io_stream) = {
                let mut inner = fair_queue.inner.lock();
                inner.waker = Some(cx.waker().clone());
                let event = match inner.ready_queue.pop() {
                    Some(s) => s,
                    None => {
                        return if !inner.streams.is_empty() || fair_queue.block_on_no_clients {
                            Poll::Pending
                        } else {
                            Poll::Ready(None)
                        }
                    }
                };
                match inner.streams.remove(&event.key) {
                    Some(stream) => (event, stream),
                    None => continue,
                }
            };

            let waker = Arc::new(StreamWaker {
                inner: fair_queue.inner.clone(),
                event: event.clone(),
            });
            let waker_ref = waker_ref(&waker);
            let mut cx = Context::from_waker(&waker_ref);
            match io_stream.as_mut().poll_next(&mut cx) {
                Poll::Ready(Some(res)) => {
                    let item = Some((event.key.clone(), res));
                    let mut inner = fair_queue.inner.lock();
                    let priority = inner.counter.fetch_add(1, atomic::Ordering::Relaxed);
                    inner.ready_queue.push(ReadyEvent {
                        priority,
                        key: event.key.clone(),
                    });
                    inner.streams.insert(event.key, io_stream);
                    return Poll::Ready(item);
                }
                Poll::Ready(None) => {
                    // Peer disconnected. Don't put the stream back.
                    // Continue to poll other streams instead of returning None immediately.
                    continue;
                }
                Poll::Pending => {
                    let mut inner = fair_queue.inner.lock();
                    inner.streams.insert(event.key, io_stream);
                    continue;
                }
            }
        }
    }
}

impl<S, K: Clone> FairQueue<S, K> {
    pub fn new(block_on_no_clients: bool) -> Self {
        Self {
            block_on_no_clients,
            inner: Arc::new(Mutex::new(QueueInner {
                counter: atomic::AtomicUsize::new(0),
                ready_queue: BinaryHeap::new(),
                streams: HashMap::new(),
                waker: None,
            })),
        }
    }

    pub(crate) fn inner(&self) -> Arc<Mutex<QueueInner<S, K>>> {
        self.inner.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::async_rt;
    use crate::fair_queue::FairQueue;
    use futures::task::noop_waker;
    use futures::{stream, Stream, StreamExt};
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Test stream that yields Pending for the first N polls, then emits messages FIFO
    struct TestStream {
        pending_polls: usize,
        messages: VecDeque<&'static str>,
    }

    impl TestStream {
        fn new(pending_polls: usize, messages: &[&'static str]) -> Self {
            Self {
                pending_polls,
                messages: messages.iter().copied().collect(),
            }
        }

        fn ready(messages: &[&'static str]) -> Self {
            Self::new(0, messages)
        }

        fn pending_once(messages: &[&'static str]) -> Self {
            Self::new(1, messages)
        }
    }

    impl Stream for TestStream {
        type Item = &'static str;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            if this.pending_polls > 0 {
                this.pending_polls -= 1;
                return Poll::Pending;
            }
            Poll::Ready(this.messages.pop_front())
        }
    }

    enum UnifiedStream {
        Test(TestStream),
    }

    impl Stream for UnifiedStream {
        type Item = &'static str;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            match self.get_mut() {
                UnifiedStream::Test(stream) => Pin::new(stream).poll_next(cx),
            }
        }
    }

    #[async_rt::test]
    async fn test_fair_queue_ready() {
        let a = stream::iter(vec!["a1", "a2", "a3"]);
        let b = stream::iter(vec!["b1", "b2", "b3"]);
        let c = stream::iter(vec!["c1", "c2", "c3"]);

        let mut f_queue: FairQueue<_, u64> = FairQueue::new(false);
        {
            let inner = f_queue.inner();
            let mut inner_lock = inner.lock();
            inner_lock.insert(1, a);
            inner_lock.insert(2, b);
            inner_lock.insert(3, c);
        }

        let mut results = Vec::new();
        while let Some(i) = f_queue.next().await {
            results.push(i);
        }

        assert_eq!(
            results,
            vec![
                (1, "a1"),
                (2, "b1"),
                (3, "c1"),
                (1, "a2"),
                (2, "b2"),
                (3, "c2"),
                (1, "a3"),
                (2, "b3"),
                (3, "c3")
            ]
        );
    }

    #[async_rt::test]
    async fn test_fair_queue_different_size() {
        let a = stream::iter(vec!["a1", "a2", "a3"]);
        let b = stream::iter(vec!["b1"]);
        let c = stream::iter(vec!["c1", "c2"]);

        let mut f_queue: FairQueue<_, u64> = FairQueue::new(false);
        {
            let inner = f_queue.inner();
            let mut inner_lock = inner.lock();
            inner_lock.insert(1, a);
            inner_lock.insert(2, b);
            inner_lock.insert(3, c);
        }

        let mut results = Vec::new();
        while let Some(i) = f_queue.next().await {
            results.push(i);
        }

        // FairQueue continues polling all streams until all are exhausted
        assert_eq!(
            results,
            vec![
                (1, "a1"),
                (2, "b1"),
                (3, "c1"),
                (1, "a2"),
                (3, "c2"),
                (1, "a3")
            ]
        );
    }

    #[test]
    fn test_fair_queue_continues_on_pending() {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        let mut fair_queue: FairQueue<UnifiedStream, &str> = FairQueue::new(false);
        {
            let inner = fair_queue.inner();
            let mut lock = inner.lock();
            lock.insert(
                "slow",
                UnifiedStream::Test(TestStream::pending_once(&["s1"])),
            );
            lock.insert(
                "fast",
                UnifiedStream::Test(TestStream::ready(&["f1", "f2"])),
            );
        }

        // First poll should return fast stream (regression test: no starvation)
        let result = Pin::new(&mut fair_queue).poll_next(&mut cx);
        match result {
            Poll::Ready(Some((key, value))) => {
                assert_eq!(key, "fast");
                assert_eq!(value, "f1");
            }
            other => panic!("Expected fast stream first, got: {:#?}", other),
        }

        // Second poll: fast stream still ready, slow stream pending
        let result = Pin::new(&mut fair_queue).poll_next(&mut cx);
        match result {
            Poll::Ready(Some((key, value))) => {
                assert_eq!(key, "fast");
                assert_eq!(value, "f2");
            }
            other => panic!("Expected fast stream second, got: {:#?}", other),
        }

        // Third poll: With noop_waker, slow stream hasn't been re-polled
        let result = Pin::new(&mut fair_queue).poll_next(&mut cx);
        match result {
            Poll::Pending => {} // Expected with noop_waker
            other @ Poll::Ready(_) => panic!("Expected Pending, got: {:#?}", other),
        }
    }

    #[test]
    fn test_fair_queue_multiple_clients_fairness() {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        let mut fair_queue: FairQueue<UnifiedStream, &str> = FairQueue::new(false);
        {
            let inner = fair_queue.inner();
            let mut lock = inner.lock();
            lock.insert(
                "fast",
                UnifiedStream::Test(TestStream::ready(&["f1", "f2", "f3"])),
            );
            lock.insert("slow", UnifiedStream::Test(TestStream::new(2, &["s1"])));
            lock.insert(
                "mid",
                UnifiedStream::Test(TestStream::new(1, &["m1", "m2"])),
            );
        }

        let mut messages = Vec::new();
        const MAX_ITERATIONS: usize = 20; // Upper bound - 3 for fast, 2 for mid, 1 for slow.

        for _ in 0..MAX_ITERATIONS {
            match Pin::new(&mut fair_queue).poll_next(&mut cx) {
                Poll::Ready(Some((key, value))) => {
                    messages.push(format!("{}:{}", key, value));

                    let has_slow = messages.iter().any(|m| m.starts_with("slow:"));
                    let fast_count = messages.iter().filter(|m| m.starts_with("fast:")).count();
                    let mid_count = messages.iter().filter(|m| m.starts_with("mid:")).count();

                    if has_slow && fast_count == 3 && mid_count == 2 {
                        break;
                    }
                }
                Poll::Ready(None) => break,
                Poll::Pending => {}
            }
        }

        // Ensure fast stream isn't starved by pending streams
        let fast_messages = messages.iter().filter(|m| m.starts_with("fast:")).count();
        assert!(
            fast_messages >= 1,
            "Fast stream was starved: {:?}",
            messages
        );
    }
}
