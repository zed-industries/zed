/*!
`nucleo` is a high level crate that provides a high level matcher API that
provides a highly effective (parallel) matcher worker. It's designed to allow
quickly plugging a fully featured (and faster) fzf/skim like fuzzy matcher into
your TUI application.

It's designed to run matching on a background threadpool while providing a
snapshot of the last complete match. That means the matcher can update the
results live while the user is typing while never blocking the main UI thread
(beyond a user provided timeout). Nucleo also supports fully concurrent lock-free
(and wait-free) streaming of input items.

The [`Nucleo`] struct servers as the main API entrypoint for this crate.

# Status

Nucleo is used in the helix-editor and therefore has a large user base with lots
or real world testing. The core matcher implementation is considered complete
and is unlikely to see major changes. The `nucleo-matcher` crate is finished and
ready for widespread use, breaking changes should be very rare (a 1.0 release
should not be far away).

While the high level `nucleo` crate also works well (and is also used in helix),
there are still additional features that will be added in the future. The high
level crate also need better documentation and will likely see a few minor API
changes in the future.

*/
use std::ops::{Bound, RangeBounds};
use std::sync::atomic::{self, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use rayon::ThreadPool;

use crate::pattern::MultiPattern;
use crate::worker::Worker;
pub use nucleo_matcher::{chars, Config, Matcher, Utf32Str, Utf32String};

mod boxcar;
mod par_sort;
pub mod pattern;
mod worker;

#[cfg(test)]
mod tests;

/// A match candidate stored in a [`Nucleo`] worker.
pub struct Item<'a, T> {
    pub data: &'a T,
    pub matcher_columns: &'a [Utf32String],
}

/// A handle that allows adding new items to a [`Nucleo`] worker.
///
/// It's internally reference counted and can be cheaply cloned
/// and sent across threads.
pub struct Injector<T> {
    items: Arc<boxcar::Vec<T>>,
    notify: Arc<(dyn Fn() + Sync + Send)>,
}

impl<T> Clone for Injector<T> {
    fn clone(&self) -> Self {
        Injector {
            items: self.items.clone(),
            notify: self.notify.clone(),
        }
    }
}

impl<T> Injector<T> {
    /// Appends an element to the list of matched items.
    /// This function is lock-free and wait-free.
    pub fn push(&self, value: T, fill_columns: impl FnOnce(&T, &mut [Utf32String])) -> u32 {
        let idx = self.items.push(value, fill_columns);
        (self.notify)();
        idx
    }

    /// Returns the total number of items injected in the matcher. This might
    /// not match the number of items in the match snapshot (if the matcher
    /// is still running)
    pub fn injected_items(&self) -> u32 {
        self.items.count()
    }

    /// Returns a reference to the item at the given index.
    ///
    /// # Safety
    ///
    /// Item at `index` must be initialized. That means you must have observed
    /// `push` returning this value or `get` retunring `Some` for this value.
    /// Just because a later index is initialized doesn't mean that this index
    /// is initialized
    pub unsafe fn get_unchecked(&self, index: u32) -> Item<'_, T> {
        self.items.get_unchecked(index)
    }

    /// Returns a reference to the element at the given index.
    pub fn get(&self, index: u32) -> Option<Item<'_, T>> {
        self.items.get(index)
    }
}

/// An [item](crate::Item) that was successfully matched by a [`Nucleo`] worker.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Match {
    pub score: u32,
    pub idx: u32,
}

/// That status of a [`Nucleo`] worker after a match.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Status {
    /// Whether the current snapshot has changed.
    pub changed: bool,
    /// Whether the matcher is still processing in the background.
    pub running: bool,
}

/// A snapshot represent the results of a [`Nucleo`] worker after
/// finishing a [`tick`](Nucleo::tick).
pub struct Snapshot<T: Sync + Send + 'static> {
    item_count: u32,
    matches: Vec<Match>,
    pattern: MultiPattern,
    items: Arc<boxcar::Vec<T>>,
}

impl<T: Sync + Send + 'static> Snapshot<T> {
    fn clear(&mut self, new_items: Arc<boxcar::Vec<T>>) {
        self.item_count = 0;
        self.matches.clear();
        self.items = new_items
    }

    fn update(&mut self, worker: &Worker<T>) {
        self.item_count = worker.item_count();
        self.pattern.clone_from(&worker.pattern);
        self.matches.clone_from(&worker.matches);
        if !Arc::ptr_eq(&worker.items, &self.items) {
            self.items = worker.items.clone()
        }
    }

    /// Returns that total number of items
    pub fn item_count(&self) -> u32 {
        self.item_count
    }

    /// Returns the pattern which items were matched against
    pub fn pattern(&self) -> &MultiPattern {
        &self.pattern
    }

    /// Returns that number of items that matched the pattern
    pub fn matched_item_count(&self) -> u32 {
        self.matches.len() as u32
    }

    /// Returns an iteror over the items that correspond to a subrange of
    /// all the matches in this snapshot.
    ///
    /// # Panics
    /// Panics if `range` has a range bound that is larger than
    /// the matched item count
    pub fn matched_items(
        &self,
        range: impl RangeBounds<u32>,
    ) -> impl ExactSizeIterator<Item = Item<'_, T>> + DoubleEndedIterator + '_ {
        // TODO: use TAIT
        let start = match range.start_bound() {
            Bound::Included(&start) => start as usize,
            Bound::Excluded(&start) => start as usize + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end as usize + 1,
            Bound::Excluded(&end) => end as usize,
            Bound::Unbounded => self.matches.len(),
        };
        self.matches[start..end]
            .iter()
            .map(|&m| unsafe { self.items.get_unchecked(m.idx) })
    }

    /// Returns a reference to the item at the given index.
    ///
    /// # Safety
    ///
    /// Item at `index` must be initialized. That means you must have observed
    /// match with the corresponding index in this exact snapshot. Observing
    /// a higher index is not enough as item indices can be non-contigously
    /// initialized
    #[inline]
    pub unsafe fn get_item_unchecked(&self, index: u32) -> Item<'_, T> {
        self.items.get_unchecked(index)
    }

    /// Returns a reference to the item at the given index.
    ///
    /// Returns `None` if the given `index` is not initialized. This function
    /// is only guarteed to return `Some` for item indices that can be found in
    /// the `matches` of this struct. Both small and larger indices may returns
    /// `None`
    #[inline]
    pub fn get_item(&self, index: u32) -> Option<Item<'_, T>> {
        self.items.get(index)
    }

    /// Returns a reference to the nth match.
    ///
    /// Returns `None` if the given `index` is not initialized. This function
    /// is only guarteed to return `Some` for item indices that can be found in
    /// the `matches` of this struct. Both small and larger indices may returns
    /// `None`
    #[inline]
    pub fn get_matched_item(&self, n: u32) -> Option<Item<'_, T>> {
        self.get_item(self.matches.get(n as usize)?.idx)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Init,
    /// items have been cleared but snapshot and items are still outdated
    Cleared,
    /// items are fresh
    Fresh,
}

impl State {
    fn matcher_item_refs(self) -> usize {
        match self {
            State::Cleared => 1,
            State::Init | State::Fresh => 2,
        }
    }

    fn canceled(self) -> bool {
        self != State::Fresh
    }

    fn cleared(self) -> bool {
        self != State::Fresh
    }
}

/// A high level matcher worker that quickly computes matches in a background
/// threadpool.
pub struct Nucleo<T: Sync + Send + 'static> {
    // the way the API is build we totally don't actually need these to be Arcs
    // but this lets us avoid some unsafe
    canceled: Arc<AtomicBool>,
    should_notify: Arc<AtomicBool>,
    worker: Arc<Mutex<Worker<T>>>,
    pool: ThreadPool,
    state: State,
    items: Arc<boxcar::Vec<T>>,
    notify: Arc<(dyn Fn() + Sync + Send)>,
    snapshot: Snapshot<T>,
    /// The pattern matched by this matcher. To update the match pattern
    /// [`MultiPattern::reparse`](`pattern::MultiPattern::reparse`) should be used.
    /// Note that the matcher worker will only become aware of the new pattern
    /// after a call to [`tick`](Nucleo::tick).
    pub pattern: MultiPattern,
}

impl<T: Sync + Send + 'static> Nucleo<T> {
    /// Constructs a new `nucleo` worker threadpool with the provided `config`.
    ///
    /// `notify` is called everytime new information is available and
    /// [`tick`](Nucleo::tick) should be called. Note that `notify` is not
    /// debounced, that should be handled by the downstream crate (for example
    /// debouncing to only redraw at most every 1/60 seconds).
    ///
    /// If `None` is passed for the number of worker threads, nucleo will use
    /// one thread per hardware thread.
    ///
    /// Nucleo can match items with multiple orthogonal properties. `columns`
    /// indicates how many matching columns each item (and the pattern) has. The
    /// number of columns can not be changed after construction.
    pub fn new(
        config: Config,
        notify: Arc<(dyn Fn() + Sync + Send)>,
        num_threads: Option<usize>,
        columns: u32,
    ) -> Self {
        let (pool, worker) = Worker::new(num_threads, config, notify.clone(), columns);
        Self {
            canceled: worker.canceled.clone(),
            should_notify: worker.should_notify.clone(),
            items: worker.items.clone(),
            pool,
            pattern: MultiPattern::new(columns as usize),
            snapshot: Snapshot {
                matches: Vec::with_capacity(2 * 1024),
                pattern: MultiPattern::new(columns as usize),
                item_count: 0,
                items: worker.items.clone(),
            },
            worker: Arc::new(Mutex::new(worker)),
            state: State::Init,
            notify,
        }
    }

    /// Returns the total number of active injectors
    pub fn active_injectors(&self) -> usize {
        Arc::strong_count(&self.items)
            - self.state.matcher_item_refs()
            - (Arc::ptr_eq(&self.snapshot.items, &self.items)) as usize
    }

    /// Returns a snapshot of the current matcher state.
    pub fn snapshot(&self) -> &Snapshot<T> {
        &self.snapshot
    }

    /// Returns an injector that can be used for adding candidates to the matcher.
    pub fn injector(&self) -> Injector<T> {
        Injector {
            items: self.items.clone(),
            notify: self.notify.clone(),
        }
    }

    /// Restart the the item stream. Removes all items and disconnects all
    /// previously created injectors from this instance. If `clear_snapshot`
    /// is `true` then all items and matched are removed from the [`Snapshot`]
    /// (crate::Snapshot) immediately. Otherwise the snapshot will keep the
    /// current matches until the matcher has run again.
    ///
    /// # Note
    ///
    /// The injectors will continue to function but they will not affect this
    /// instance anymore. The old items will only be dropped when all injectors
    /// were dropped.
    pub fn restart(&mut self, clear_snapshot: bool) {
        self.canceled.store(true, Ordering::Relaxed);
        self.items = Arc::new(boxcar::Vec::with_capacity(1024, self.items.columns()));
        self.state = State::Cleared;
        if clear_snapshot {
            self.snapshot.clear(self.items.clone());
        }
    }

    pub fn update_config(&mut self, config: Config) {
        self.worker.lock().update_config(config)
    }

    /// The main way to interact with the matcher, this should be called
    /// regularly (for example each time a frame is rendered). To avoid
    /// excessive redraws this method will wait `timeout` milliseconds for the
    /// worker therad to finish. It is recommend to set the timeout to 10ms.
    pub fn tick(&mut self, timeout: u64) -> Status {
        self.should_notify.store(false, atomic::Ordering::Relaxed);
        let status = self.pattern.status();
        let canceled = status != pattern::Status::Unchanged || self.state.canceled();
        let mut res = self.tick_inner(timeout, canceled, status);
        if !canceled {
            return res;
        }
        self.state = State::Fresh;
        let status2 = self.tick_inner(timeout, false, pattern::Status::Unchanged);
        res.changed |= status2.changed;
        res.running = status2.running;
        res
    }

    fn tick_inner(&mut self, timeout: u64, canceled: bool, status: pattern::Status) -> Status {
        let mut inner = if canceled {
            self.pattern.reset_status();
            self.canceled.store(true, atomic::Ordering::Relaxed);
            self.worker.lock_arc()
        } else {
            let Some(worker) = self.worker.try_lock_arc_for(Duration::from_millis(timeout)) else {
                self.should_notify.store(true, Ordering::Release);
                return Status {
                    changed: false,
                    running: true,
                };
            };
            worker
        };

        let changed = inner.running;

        let running = canceled || self.items.count() > inner.item_count();
        if inner.running {
            inner.running = false;
            if !inner.was_canceled && !self.state.canceled() {
                self.snapshot.update(&inner)
            }
        }
        if running {
            inner.pattern.clone_from(&self.pattern);
            self.canceled.store(false, atomic::Ordering::Relaxed);
            if !canceled {
                self.should_notify.store(true, atomic::Ordering::Release);
            }
            let cleared = self.state.cleared();
            if cleared {
                inner.items = self.items.clone();
            }
            self.pool
                .spawn(move || unsafe { inner.run(status, cleared) })
        }
        Status { changed, running }
    }
}

impl<T: Sync + Send> Drop for Nucleo<T> {
    fn drop(&mut self) {
        // we ensure the worker quits before dropping items to ensure that
        // the worker can always assume the items outlive it
        self.canceled.store(true, atomic::Ordering::Relaxed);
        let lock = self.worker.try_lock_for(Duration::from_secs(1));
        if lock.is_none() {
            unreachable!("thread pool failed to shutdown properly")
        }
    }
}
