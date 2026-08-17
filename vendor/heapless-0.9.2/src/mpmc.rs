//! A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue.
//!
//! # Deprecation
//!
//! <div class="warning">
//! The current implementation of `mpmc` is marked as deprecated due to not being truly lock-free
//! </div>
//!
//! If a thread is parked, or pre-empted for a long time by an higher-priority task
//! during an `enqueue` or `dequeue` operation, it is possible that the queue ends-up
//! in a state were no other task can successfully enqueue or dequeue items from it
//! until the pre-empted task can finish its operation.
//!
//! In that case, [`enqueue`](QueueInner::dequeue) and [`dequeue`](QueueInner::enqueue)
//! will return an error, but will not panic or reach undefined behaviour
//!
//! This makes `mpmc` unsuitable for some use cases such as using it as a pool of objects.
//!
//! ## When can this queue be used?
//!
//! This queue should be used for cross-task communication only when items sent over the queue
//! can be dropped in case of concurrent operations, or when it is possible to retry
//! the dequeue/enqueue operation after other tasks have had the opportunity to make progress.
//!
//! In that case you can safely ignore the warnings using `#[expect(deprecated)]`
//! when `new` is called
//!
//! For more information, and possible alternative, please see
//! <https://github.com/rust-embedded/heapless/issues/583>
//!
//!
//! **Note:** This module requires atomic compare-and-swap (CAS) instructions. On
//! targets where they're not natively available, they are emulated by the
//! [`portable-atomic`](https://crates.io/crates/portable-atomic) crate.
//!
//! # Example
//!
//! This queue can be constructed in `const` context. Placing it in a `static` variable lets *all*
//! contexts (interrupts/threads/`main`) safely enqueue and dequeue items.
//!
//! ```
//! use core::sync::atomic::{AtomicU8, Ordering};
//!
//! use heapless::mpmc::Queue;
//!
//! static Q: Queue<u8, 2> = Queue::new();
//!
//! fn main() {
//!     // Configure systick interrupt.
//!
//!     loop {
//!         if let Some(x) = Q.dequeue() {
//!             println!("{}", x);
//!         } else {
//!             // Wait for interrupt.
//!         }
//! #       break
//!     }
//! }
//!
//! fn systick() {
//!     static COUNT: AtomicU8 = AtomicU8::new(0);
//!     let count = COUNT.fetch_add(1, Ordering::SeqCst);
//!
//! #   let _ =
//!     Q.enqueue(count);
//! }
//! ```
//!
//! # Benchmark
//!
//! Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled
//! with `-C opt-level=z`:
//!
//! | Method                      | Time | N  |
//! |:----------------------------|-----:|---:|
//! | `Queue::<u8, 8>::enqueue()` |   34 |  0 |
//! | `Queue::<u8, 8>::enqueue()` |   52 |  1 |
//! | `Queue::<u8, 8>::enqueue()` |   69 |  2 |
//! | `Queue::<u8, 8>::dequeue()` |   35 |  0 |
//! | `Queue::<u8, 8>::dequeue()` |   53 |  1 |
//! | `Queue::<u8, 8>::dequeue()` |   71 |  2 |
//!
//! - N denotes the number of interruptions. On Cortex-M, an interruption consists of an interrupt
//!   handler preempting the would-be atomic section of the `enqueue`/`dequeue` operation. Note that
//!   it does *not* matter if the higher priority handler uses the queue or not.
//! - All execution times are in clock cycles (1 clock cycle = 125 ns).
//! - Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
//!   `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
//! - The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some` and
//!   `enqueue` returning `Ok`.
//!
//! # References
//!
//! This is an implementation of Dmitry Vyukov's [bounded MPMC queue], minus the
//! cache padding.
//!
//! [bounded MPMC queue]: http://www.1024cores.net/home/lock-free-algorithms/queues/bounded-mpmc-queue

use core::{cell::UnsafeCell, mem::MaybeUninit};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;

use atomic::Ordering;

use crate::storage::{OwnedStorage, Storage, ViewStorage};

#[cfg(feature = "mpmc_large")]
type AtomicTargetSize = atomic::AtomicUsize;
#[cfg(not(feature = "mpmc_large"))]
type AtomicTargetSize = atomic::AtomicU8;

#[cfg(feature = "mpmc_large")]
type UintSize = usize;
#[cfg(not(feature = "mpmc_large"))]
type UintSize = u8;

#[cfg(feature = "mpmc_large")]
type IntSize = isize;
#[cfg(not(feature = "mpmc_large"))]
type IntSize = i8;

/// Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].
///
/// In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this
/// struct if you want to write code that's generic over both.
pub struct QueueInner<T, S: Storage> {
    dequeue_pos: AtomicTargetSize,
    enqueue_pos: AtomicTargetSize,
    buffer: UnsafeCell<S::Buffer<Cell<T>>>,
}

/// A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements.
///
/// <div class="warning">
///
/// `N` must be a power of 2.
///
/// </div>
///
/// The maximum value of `N` is 128 if the `mpmc_large` feature is not enabled.
pub type Queue<T, const N: usize> = QueueInner<T, OwnedStorage<N>>;

/// A [`Queue`] with dynamic capacity.
///
/// [`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by
/// reference.
pub type QueueView<T> = QueueInner<T, ViewStorage>;

impl<T, const N: usize> Queue<T, N> {
    #[deprecated(
        note = "See the documentation of Queue::new() for more information: https://docs.rs/heapless/latest/heapless/mpmc/type.Queue.html#method.new"
    )]
    /// Creates an empty queue.
    ///
    /// # Deprecation
    ///
    /// <div class="warning">
    /// The current implementation of `mpmc` is marked as deprecated due to not being truly
    /// lock-free </div>
    ///
    /// If a thread is parked, or pre-empted for a long time by an higher-priority task
    /// during an `enqueue` or `dequeue` operation, it is possible that the queue ends-up
    /// in a state were no other task can successfully enqueue or dequeue items from it
    /// until the pre-empted task can finish its operation.
    ///
    /// In that case, [`enqueue`](QueueInner::dequeue) and [`dequeue`](QueueInner::enqueue)
    /// will return an error, but will not panic or reach undefined behaviour
    ///
    /// This makes `mpmc` unsuitable for some use cases such as using it as a pool of objects.
    ///
    /// ## When can this queue be used?
    ///
    /// This queue should be used for cross-task communication only when items sent over the queue
    /// can be dropped in case of concurrent operations, or when it is possible to retry
    /// the dequeue/enqueue operation after other tasks have had the opportunity to make progress.
    ///
    /// In that case you can safely ignore the warnings using `#[expect(deprecated)]`
    /// when `new` is called
    ///
    /// For more information, and possible alternative, please see
    /// <https://github.com/rust-embedded/heapless/issues/583>
    pub const fn new() -> Self {
        const {
            assert!(N > 1);
            assert!(N.is_power_of_two());
            assert!(N < UintSize::MAX as usize);
        }

        let mut cell_count = 0;

        let mut result_cells: [Cell<T>; N] = [const { Cell::new(0) }; N];
        while cell_count != N {
            result_cells[cell_count] = Cell::new(cell_count);
            cell_count += 1;
        }

        Self {
            buffer: UnsafeCell::new(result_cells),
            dequeue_pos: AtomicTargetSize::new(0),
            enqueue_pos: AtomicTargetSize::new(0),
        }
    }

    /// Used in `Storage` implementation.
    pub(crate) fn as_view_private(&self) -> &QueueView<T> {
        self
    }
    /// Used in `Storage` implementation.
    pub(crate) fn as_view_mut_private(&mut self) -> &mut QueueView<T> {
        self
    }
}

impl<T, S: Storage> QueueInner<T, S> {
    /// Returns the maximum number of elements the queue can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        S::len(self.buffer.get())
    }

    /// Get a reference to the `Queue`, erasing the `N` const-generic.
    ///
    ///
    /// ```rust
    /// # use heapless::mpmc::{Queue, QueueView};
    /// let queue: Queue<u8, 2> = Queue::new();
    /// let view: &QueueView<u8> = queue.as_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `Queue<T, N>` implements
    /// `Unsize<QueueView<T>>`:
    ///
    /// ```rust
    /// # use heapless::mpmc::{Queue, QueueView};
    /// let queue: Queue<u8, 2> = Queue::new();
    /// let view: &QueueView<u8> = &queue;
    /// ```
    #[inline]
    pub fn as_view(&self) -> &QueueView<T> {
        S::as_mpmc_view(self)
    }

    /// Get a mutable reference to the `Queue`, erasing the `N` const-generic.
    ///
    /// ```rust
    /// # use heapless::mpmc::{Queue, QueueView};
    /// ##[expect(deprecated)]
    /// let mut queue: Queue<u8, 2> = Queue::new();
    /// let view: &mut QueueView<u8> = queue.as_mut_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `Queue<T, N>` implements
    /// `Unsize<QueueView<T>>`:
    ///
    /// ```rust
    /// # use heapless::mpmc::{Queue, QueueView};
    /// ##[expect(deprecated)]
    /// let mut queue: Queue<u8, 2> = Queue::new();
    /// let view: &mut QueueView<u8> = &mut queue;
    /// ```
    #[inline]
    pub fn as_mut_view(&mut self) -> &mut QueueView<T> {
        S::as_mpmc_mut_view(self)
    }

    fn mask(&self) -> UintSize {
        (S::len(self.buffer.get()) - 1) as _
    }

    /// Returns the item in the front of the queue, or `None` if the queue is empty.
    pub fn dequeue(&self) -> Option<T> {
        unsafe { dequeue(S::as_ptr(self.buffer.get()), &self.dequeue_pos, self.mask()) }
    }

    /// Adds an `item` to the end of the queue.
    ///
    /// Returns back the `item` if the queue is full.
    pub fn enqueue(&self, item: T) -> Result<(), T> {
        unsafe {
            enqueue(
                S::as_ptr(self.buffer.get()),
                &self.enqueue_pos,
                self.mask(),
                item,
            )
        }
    }
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        #[allow(deprecated)]
        Self::new()
    }
}

impl<T, S: Storage> Drop for QueueInner<T, S> {
    fn drop(&mut self) {
        // Drop all elements currently in the queue.
        while self.dequeue().is_some() {}
    }
}

unsafe impl<T, S: Storage> Sync for QueueInner<T, S> where T: Send {}

struct Cell<T> {
    data: MaybeUninit<T>,
    sequence: AtomicTargetSize,
}

impl<T> Cell<T> {
    const fn new(seq: usize) -> Self {
        Self {
            data: MaybeUninit::uninit(),
            sequence: AtomicTargetSize::new(seq as UintSize),
        }
    }
}

unsafe fn dequeue<T>(
    buffer: *mut Cell<T>,
    dequeue_pos: &AtomicTargetSize,
    mask: UintSize,
) -> Option<T> {
    let mut pos = dequeue_pos.load(Ordering::Relaxed);

    let mut cell;
    loop {
        cell = buffer.add(usize::from(pos & mask));
        let seq = (*cell).sequence.load(Ordering::Acquire);
        let dif = (seq as IntSize).wrapping_sub((pos.wrapping_add(1)) as IntSize);

        match dif.cmp(&0) {
            core::cmp::Ordering::Equal => {
                if dequeue_pos
                    .compare_exchange_weak(
                        pos,
                        pos.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    break;
                }
            }
            core::cmp::Ordering::Less => {
                return None;
            }
            core::cmp::Ordering::Greater => {
                pos = dequeue_pos.load(Ordering::Relaxed);
            }
        }
    }

    let data = (*cell).data.as_ptr().read();
    (*cell)
        .sequence
        .store(pos.wrapping_add(mask).wrapping_add(1), Ordering::Release);
    Some(data)
}

unsafe fn enqueue<T>(
    buffer: *mut Cell<T>,
    enqueue_pos: &AtomicTargetSize,
    mask: UintSize,
    item: T,
) -> Result<(), T> {
    let mut pos = enqueue_pos.load(Ordering::Relaxed);

    let mut cell;
    loop {
        cell = buffer.add(usize::from(pos & mask));
        let seq = (*cell).sequence.load(Ordering::Acquire);
        let dif = (seq as IntSize).wrapping_sub(pos as IntSize);

        match dif.cmp(&0) {
            core::cmp::Ordering::Equal => {
                if enqueue_pos
                    .compare_exchange_weak(
                        pos,
                        pos.wrapping_add(1),
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    break;
                }
            }
            core::cmp::Ordering::Less => {
                return Err(item);
            }
            core::cmp::Ordering::Greater => {
                pos = enqueue_pos.load(Ordering::Relaxed);
            }
        }
    }

    (*cell).data.as_mut_ptr().write(item);
    (*cell)
        .sequence
        .store(pos.wrapping_add(1), Ordering::Release);
    Ok(())
}

#[cfg(test)]
mod tests {
    use static_assertions::assert_not_impl_any;

    use super::Queue;

    // Ensure a `Queue` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Queue<*const (), 4>: Send);

    #[test]
    fn memory_leak() {
        droppable!();

        #[expect(deprecated)]
        let q = Queue::<_, 2>::new();
        q.enqueue(Droppable::new()).unwrap_or_else(|_| panic!());
        q.enqueue(Droppable::new()).unwrap_or_else(|_| panic!());
        drop(q);

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn sanity() {
        #[expect(deprecated)]
        let q = Queue::<_, 2>::new();
        q.enqueue(0).unwrap();
        q.enqueue(1).unwrap();
        assert!(q.enqueue(2).is_err());

        assert_eq!(q.dequeue(), Some(0));
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn drain_at_pos255() {
        #[expect(deprecated)]
        let q = Queue::<_, 2>::new();
        for _ in 0..255 {
            assert!(q.enqueue(0).is_ok());
            assert_eq!(q.dequeue(), Some(0));
        }

        // Queue is empty, this should not block forever.
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn full_at_wrapped_pos0() {
        #[expect(deprecated)]
        let q = Queue::<_, 2>::new();
        for _ in 0..254 {
            assert!(q.enqueue(0).is_ok());
            assert_eq!(q.dequeue(), Some(0));
        }
        assert!(q.enqueue(0).is_ok());
        assert!(q.enqueue(0).is_ok());
        // this should not block forever
        assert!(q.enqueue(0).is_err());
    }

    #[test]
    fn enqueue_full() {
        #[cfg(not(feature = "mpmc_large"))]
        const CAPACITY: usize = 128;

        #[cfg(feature = "mpmc_large")]
        const CAPACITY: usize = 256;

        #[expect(deprecated)]
        let q: Queue<u8, CAPACITY> = Queue::new();

        assert_eq!(q.capacity(), CAPACITY);

        for _ in 0..CAPACITY {
            q.enqueue(0xAA).unwrap();
        }

        // Queue is full, this should not block forever.
        q.enqueue(0x55).unwrap_err();
    }
}
