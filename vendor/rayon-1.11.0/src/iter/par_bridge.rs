#[cfg(not(feature = "web_spin_lock"))]
use std::sync::Mutex;

#[cfg(feature = "web_spin_lock")]
use wasm_sync::Mutex;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::iter::plumbing::{bridge_unindexed, Folder, UnindexedConsumer, UnindexedProducer};
use crate::iter::ParallelIterator;
use crate::{current_num_threads, current_thread_index};

/// Conversion trait to convert an `Iterator` to a `ParallelIterator`.
///
/// This creates a "bridge" from a sequential iterator to a parallel one, by distributing its items
/// across the Rayon thread pool. This has the advantage of being able to parallelize just about
/// anything, but the resulting `ParallelIterator` can be less efficient than if you started with
/// `par_iter` instead. However, it can still be useful for iterators that are difficult to
/// parallelize by other means, like channels or file or network I/O.
///
/// Iterator items are pulled by `next()` one at a time, synchronized from each thread that is
/// ready for work, so this may become a bottleneck if the serial iterator can't keep up with the
/// parallel demand. The items are not buffered by `IterBridge`, so it's fine to use this with
/// large or even unbounded iterators.
///
/// The resulting iterator is not guaranteed to keep the order of the original iterator.
///
/// # Examples
///
/// To use this trait, take an existing `Iterator` and call `par_bridge` on it. After that, you can
/// use any of the `ParallelIterator` methods:
///
/// ```
/// use rayon::iter::ParallelBridge;
/// use rayon::prelude::ParallelIterator;
/// use std::sync::mpsc::channel;
///
/// let rx = {
///     let (tx, rx) = channel();
///
///     tx.send("one!");
///     tx.send("two!");
///     tx.send("three!");
///
///     rx
/// };
///
/// let mut output: Vec<&'static str> = rx.into_iter().par_bridge().collect();
/// output.sort_unstable();
///
/// assert_eq!(&*output, &["one!", "three!", "two!"]);
/// ```
pub trait ParallelBridge: Sized {
    /// Creates a bridge from this type to a `ParallelIterator`.
    fn par_bridge(self) -> IterBridge<Self>;
}

impl<T> ParallelBridge for T
where
    T: Iterator<Item: Send> + Send,
{
    fn par_bridge(self) -> IterBridge<Self> {
        IterBridge { iter: self }
    }
}

/// `IterBridge` is a parallel iterator that wraps a sequential iterator.
///
/// This type is created when using the `par_bridge` method on `ParallelBridge`. See the
/// [`ParallelBridge`] documentation for details.
#[derive(Debug, Clone)]
pub struct IterBridge<Iter> {
    iter: Iter,
}

impl<Iter> ParallelIterator for IterBridge<Iter>
where
    Iter: Iterator<Item: Send> + Send,
{
    type Item = Iter::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let num_threads = current_num_threads();
        let threads_started: Vec<_> = (0..num_threads).map(|_| AtomicBool::new(false)).collect();

        bridge_unindexed(
            &IterParallelProducer {
                split_count: AtomicUsize::new(num_threads),
                iter: Mutex::new(self.iter.fuse()),
                threads_started: &threads_started,
            },
            consumer,
        )
    }
}

struct IterParallelProducer<'a, Iter> {
    split_count: AtomicUsize,
    iter: Mutex<std::iter::Fuse<Iter>>,
    threads_started: &'a [AtomicBool],
}

impl<Iter: Iterator + Send> UnindexedProducer for &IterParallelProducer<'_, Iter> {
    type Item = Iter::Item;

    fn split(self) -> (Self, Option<Self>) {
        // Check if the iterator is exhausted
        let update = self
            .split_count
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |c| c.checked_sub(1));
        (self, update.is_ok().then_some(self))
    }

    fn fold_with<F>(self, mut folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        // Guard against work-stealing-induced recursion, in case `Iter::next()`
        // calls rayon internally, so we don't deadlock our mutex. We might also
        // be recursing via `folder` methods, which doesn't present a mutex hazard,
        // but it's lower overhead for us to just check this once, rather than
        // updating additional shared state on every mutex lock/unlock.
        // (If this isn't a rayon thread, then there's no work-stealing anyway...)
        if let Some(i) = current_thread_index() {
            // Note: If the number of threads in the pool ever grows dynamically, then
            // we'll end up sharing flags and may falsely detect recursion -- that's
            // still fine for overall correctness, just not optimal for parallelism.
            let thread_started = &self.threads_started[i % self.threads_started.len()];
            if thread_started.swap(true, Ordering::Relaxed) {
                // We can't make progress with a nested mutex, so just return and let
                // the outermost loop continue with the rest of the iterator items.
                return folder;
            }
        }

        loop {
            if let Ok(mut iter) = self.iter.lock() {
                if let Some(it) = iter.next() {
                    drop(iter);
                    folder = folder.consume(it);
                    if folder.full() {
                        return folder;
                    }
                } else {
                    return folder;
                }
            } else {
                // any panics from other threads will have been caught by the pool,
                // and will be re-thrown when joined - just exit
                return folder;
            }
        }
    }
}
