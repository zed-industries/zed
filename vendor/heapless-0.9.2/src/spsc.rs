//! A fixed capacity single-producer, single-consumer (SPSC) lock-free queue.
//!
//! *Note:* This module requires atomic load and store instructions. On
//! targets where they're not natively available, they are emulated by the
//! [`portable-atomic`](https://crates.io/crates/portable-atomic) crate.
//!
//! # Examples
//!
//! [`Queue`] can be used as a plain queue:
//!
//! ```
//! use heapless::spsc::Queue;
//!
//! let mut queue: Queue<u8, 4> = Queue::new();
//!
//! assert_eq!(queue.enqueue(0), Ok(()));
//! assert_eq!(queue.enqueue(1), Ok(()));
//! assert_eq!(queue.enqueue(2), Ok(()));
//! assert_eq!(queue.enqueue(3), Err(3)); // Queue is full.
//!
//! assert_eq!(queue.dequeue(), Some(0));
//! ```
//!
//! [`Queue::split`] can be used to split the queue into a [`Producer`]/[`Consumer`] pair.
//!
//! After splitting a `&'static mut Queue`, the resulting [`Producer`] and [`Consumer`]
//! can be moved into different execution contexts, e.g. threads, interrupt handlers, etc.
//!
//!
//! ```
//! use heapless::spsc::{Producer, Queue};
//!
//! #[derive(Debug)]
//! enum Event {
//!     A,
//!     B,
//! }
//!
//! fn main() {
//!     let queue: &'static mut Queue<Event, 4> = {
//!         static mut Q: Queue<Event, 4> = Queue::new();
//!         // SAFETY: `Q` is only accessible in this scope
//!         // and `main` is only called once.
//!         unsafe { &mut Q }
//!     };
//!
//!     let (producer, mut consumer) = queue.split();
//!
//!     // `producer` can be moved into `interrupt_handler` using a static mutex or the mechanism
//!     // provided by the concurrency framework you are using, e.g. a resource in RTIC.
//! #   let mut producer = producer;
//! #   interrupt_handler(&mut producer);
//!
//!     loop {
//!         match consumer.dequeue() {
//!             Some(Event::A) => { /* .. */ }
//!             Some(Event::B) => { /* .. */ }
//!             None => { /* Sleep. */ }
//!         }
//! #       break
//!     }
//! }
//!
//! // This is a different execution context that can preempt `main`.
//! fn interrupt_handler(producer: &mut Producer<'static, Event>) {
//! #   let condition = true;
//!
//!     // ..
//!
//!     if condition {
//!         producer.enqueue(Event::A).unwrap();
//!     } else {
//!         producer.enqueue(Event::B).unwrap();
//!     }
//!
//!     // ..
//! }
//! ```
//!
//! # Benchmarks
//!
//! Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled
//! with `-C opt-level=3`:
//!
//! | Method                         | Time |
//! |:-------------------------------|-----:|
//! | `Producer::<u8, _>::enqueue()` |   16 |
//! | `Queue::<u8, _>::enqueue()`    |   14 |
//! | `Consumer::<u8, _>::dequeue()` |   15 |
//! | `Queue::<u8, _>::dequeue()`    |   12 |
//!
//! - All execution times are in clock cycles (1 clock cycle = 125 ns).
//! - Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
//!   `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
//! - The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some` and
//!   `enqueue` returning `Ok`.
//!
//! # References
//!
//! This is an implementation based on [https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular](
//!   https://web.archive.org/web/20250117082625/https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular
//! ).

use core::{borrow::Borrow, cell::UnsafeCell, fmt, hash, mem::MaybeUninit, ptr};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;

use atomic::{AtomicUsize, Ordering};

use crate::storage::{OwnedStorage, Storage, ViewStorage};

/// Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].
///
/// In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this
/// struct if you want to write code that's generic over both.
pub struct QueueInner<T, S: Storage> {
    // this is from where we dequeue items
    pub(crate) head: AtomicUsize,

    // this is where we enqueue new items
    pub(crate) tail: AtomicUsize,

    pub(crate) buffer: S::Buffer<UnsafeCell<MaybeUninit<T>>>,
}

/// A statically allocated single-producer, single-consumer queue with a capacity of `N - 1`
/// elements.
///
/// <div class="warning">
///
/// To get better performance, use a value for `N` that is a power of 2.
///
/// </div>
///
/// You will likely want to use [`split`](QueueInner::split) to create a producer-consumer pair.
pub type Queue<T, const N: usize> = QueueInner<T, OwnedStorage<N>>;

/// A [`Queue`] with dynamic capacity.
///
/// [`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by
/// reference.
pub type QueueView<T> = QueueInner<T, ViewStorage>;

impl<T, const N: usize> Queue<T, N> {
    /// Creates an empty queue.
    pub const fn new() -> Self {
        const {
            assert!(N > 1);
        }

        Queue {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buffer: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
        }
    }

    /// Used in `Storage` implementation
    pub(crate) fn as_view_private(&self) -> &QueueView<T> {
        self
    }

    /// Used in `Storage` implementation
    pub(crate) fn as_mut_view_private(&mut self) -> &mut QueueView<T> {
        self
    }
}

impl<T, S: Storage> QueueInner<T, S> {
    /// Get a reference to the `Queue`, erasing the `N` const-generic.
    pub fn as_view(&self) -> &QueueView<T> {
        S::as_queue_view(self)
    }

    /// Get a mutable reference to the `Queue`, erasing the `N` const-generic.
    pub fn as_mut_view(&mut self) -> &mut QueueView<T> {
        S::as_mut_queue_view(self)
    }

    #[inline]
    fn increment(&self, val: usize) -> usize {
        (val + 1) % self.n()
    }

    #[inline]
    fn n(&self) -> usize {
        self.buffer.borrow().len()
    }

    /// Returns the maximum number of elements the queue can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.n() - 1
    }

    /// Returns the number of elements in the queue.
    #[inline]
    pub fn len(&self) -> usize {
        let current_head = self.head.load(Ordering::Relaxed);
        let current_tail = self.tail.load(Ordering::Relaxed);

        current_tail
            .wrapping_sub(current_head)
            .wrapping_add(self.n())
            % self.n()
    }

    /// Returns whether the queue is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// Returns whether the queue is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.increment(self.tail.load(Ordering::Relaxed)) == self.head.load(Ordering::Relaxed)
    }

    /// Iterates from the front of the queue to the back.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            rb: self.as_view(),
            index: 0,
            len: self.len(),
        }
    }

    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let len = self.len();
        IterMut {
            rb: self.as_view(),
            index: 0,
            len,
        }
    }

    /// Adds an `item` to the end of the queue.
    ///
    /// Returns back the `item` if the queue is full.
    #[inline]
    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        unsafe { self.inner_enqueue(item) }
    }

    /// Returns the item in the front of the queue, or `None` if the queue is empty.
    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        unsafe { self.inner_dequeue() }
    }

    /// Returns a reference to the item in the front of the queue without dequeuing it, or
    /// `None` if the queue is empty.
    ///
    /// # Examples
    /// ```
    /// use heapless::spsc::Queue;
    ///
    /// let mut queue: Queue<u8, 235> = Queue::new();
    /// let (mut producer, mut consumer) = queue.split();
    /// assert_eq!(None, consumer.peek());
    /// producer.enqueue(1);
    /// assert_eq!(Some(&1), consumer.peek());
    /// assert_eq!(Some(1), consumer.dequeue());
    /// assert_eq!(None, consumer.peek());
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let head = self.head.load(Ordering::Relaxed);
            Some(unsafe { &*(self.buffer.borrow().get_unchecked(head).get() as *const T) })
        }
    }

    // The memory for enqueueing is "owned" by the tail pointer.
    //
    // NOTE: This internal function uses internal mutability to allow the [`Producer`] to enqueue
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_enqueue(&self, val: T) -> Result<(), T> {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = self.increment(current_tail);

        if next_tail == self.head.load(Ordering::Acquire) {
            Err(val)
        } else {
            (self.buffer.borrow().get_unchecked(current_tail).get()).write(MaybeUninit::new(val));
            self.tail.store(next_tail, Ordering::Release);

            Ok(())
        }
    }

    // The memory for enqueueing is "owned" by the tail pointer.
    //
    // NOTE: This internal function uses internal mutability to allow the [`Producer`] to enqueue
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_enqueue_unchecked(&self, val: T) {
        let current_tail = self.tail.load(Ordering::Relaxed);

        (self.buffer.borrow().get_unchecked(current_tail).get()).write(MaybeUninit::new(val));
        self.tail
            .store(self.increment(current_tail), Ordering::Release);
    }

    /// Adds an `item` to the end of the queue, without checking if it's full.
    ///
    /// # Safety
    ///
    /// If the queue is full, this operation will leak a value (`T`'s destructor won't run on
    /// the value that got overwritten by `item`), *and* will allow the `dequeue` operation
    /// to create a copy of `item`, which could result in `T`'s destructor running on `item`
    /// twice.
    pub unsafe fn enqueue_unchecked(&mut self, item: T) {
        self.inner_enqueue_unchecked(item);
    }

    // The memory for dequeuing is "owned" by the head pointer.
    //
    // NOTE: This internal function uses internal mutability to allow the [`Consumer`] to dequeue
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_dequeue(&self) -> Option<T> {
        let current_head = self.head.load(Ordering::Relaxed);

        if current_head == self.tail.load(Ordering::Acquire) {
            None
        } else {
            let v = (self.buffer.borrow().get_unchecked(current_head).get() as *const T).read();

            self.head
                .store(self.increment(current_head), Ordering::Release);

            Some(v)
        }
    }

    // The memory for dequeuing is "owned" by the head pointer.
    //
    // NOTE: This internal function uses internal mutability to allow the [`Consumer`] to dequeue
    // items without doing pointer arithmetic and accessing internal fields of this type.
    unsafe fn inner_dequeue_unchecked(&self) -> T {
        let current_head = self.head.load(Ordering::Relaxed);
        let v = (self.buffer.borrow().get_unchecked(current_head).get() as *const T).read();

        self.head
            .store(self.increment(current_head), Ordering::Release);

        v
    }

    /// Returns the item in the front of the queue, without checking if there is something in the
    /// queue.
    ///
    /// # Safety
    ///
    /// The queue must not be empty. Calling this on an empty queue causes [undefined behavior].
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn dequeue_unchecked(&mut self) -> T {
        self.inner_dequeue_unchecked()
    }

    /// Splits a queue into producer and consumer endpoints.
    ///
    /// If you need this function in a `const` context,
    /// check out [`Queue::split_const`] and [`QueueView::split_const`].
    ///
    /// # Examples
    ///
    /// Create a queue and split it at runtime
    ///
    /// ```
    /// # use heapless::spsc::Queue;
    /// let mut queue: Queue<(), 4> = Queue::new();
    /// let (mut producer, mut consumer) = queue.split();
    /// producer.enqueue(()).unwrap();
    /// assert_eq!(consumer.dequeue(), Some(()));
    /// ```
    ///
    /// Create a queue at compile time, split it at runtime,
    /// and pass it to an interrupt handler via a mutex.
    ///
    /// ```
    /// use core::cell::RefCell;
    /// use critical_section::Mutex;
    /// use heapless::spsc::{Producer, Queue};
    ///
    /// static PRODUCER: Mutex<RefCell<Option<Producer<'static, ()>>>> =
    ///     { Mutex::new(RefCell::new(None)) };
    ///
    /// fn interrupt() {
    ///     let mut producer = {
    ///         static mut P: Option<Producer<'static, ()>> = None;
    ///         // SAFETY: Mutable access to `P` is allowed exclusively in this scope
    ///         // and `interrupt` cannot be called directly or preempt itself.
    ///         unsafe { &mut P }
    ///     }
    ///     .get_or_insert_with(|| {
    ///         critical_section::with(|cs| PRODUCER.borrow_ref_mut(cs).take().unwrap())
    ///     });
    ///
    ///     producer.enqueue(()).unwrap();
    /// }
    ///
    /// fn main() {
    ///     let mut consumer = {
    ///         let (p, c) = {
    ///             static mut Q: Queue<(), 4> = Queue::new();
    ///             // SAFETY: `Q` is only accessible in this scope
    ///             // and `main` is only called once.
    ///             #[allow(static_mut_refs)]
    ///             unsafe {
    ///                 Q.split()
    ///             }
    ///         };
    ///
    ///         critical_section::with(move |cs| {
    ///             let mut producer = PRODUCER.borrow_ref_mut(cs);
    ///             *producer = Some(p);
    ///         });
    ///
    ///         c
    ///     };
    ///
    ///     // Interrupt occurs.
    /// #   interrupt();
    ///
    ///     consumer.dequeue().unwrap();
    /// }
    /// ```
    pub fn split(&mut self) -> (Producer<'_, T>, Consumer<'_, T>) {
        (
            Producer { rb: self.as_view() },
            Consumer { rb: self.as_view() },
        )
    }
}

impl<T, const N: usize> Queue<T, N> {
    /// Splits a queue into producer and consumer endpoints.
    ///
    /// Unlike [`Queue::split`](), this method can be used in a `const` context
    ///
    /// # Example
    ///
    /// Create and split a queue at compile time, and pass it to the main
    /// function and an interrupt handler via a mutex at runtime.
    ///
    /// ```
    /// use core::cell::RefCell;
    ///
    /// use critical_section::Mutex;
    /// use heapless::spsc::{Consumer, Producer, Queue};
    ///
    /// static PC: (
    ///     Mutex<RefCell<Option<Producer<'_, ()>>>>,
    ///     Mutex<RefCell<Option<Consumer<'_, ()>>>>,
    /// ) = {
    ///     static mut Q: Queue<(), 4> = Queue::new();
    ///     // SAFETY: `Q` is only accessible in this scope.
    ///     #[allow(static_mut_refs)]
    ///     let (p, c) = unsafe { Q.split_const() };
    ///
    ///     (
    ///         Mutex::new(RefCell::new(Some(p))),
    ///         Mutex::new(RefCell::new(Some(c))),
    ///     )
    /// };
    ///
    /// fn interrupt() {
    ///     let mut producer = {
    ///         static mut P: Option<Producer<'_, ()>> = None;
    ///         // SAFETY: Mutable access to `P` is allowed exclusively in this scope
    ///         // and `interrupt` cannot be called directly or preempt itself.
    ///         unsafe { &mut P }
    ///     }
    ///     .get_or_insert_with(|| {
    ///         critical_section::with(|cs| PC.0.borrow_ref_mut(cs).take().unwrap())
    ///     });
    ///
    ///     producer.enqueue(()).unwrap();
    /// }
    ///
    /// fn main() {
    ///     let mut consumer = critical_section::with(|cs| PC.1.borrow_ref_mut(cs).take().unwrap());
    ///
    ///     // Interrupt occurs.
    /// #   interrupt();
    ///
    ///     consumer.dequeue().unwrap();
    /// }
    /// ```
    pub const fn split_const(&mut self) -> (Producer<'_, T>, Consumer<'_, T>) {
        (Producer { rb: self }, Consumer { rb: self })
    }
}

impl<T> QueueView<T> {
    /// Splits a queue into producer and consumer endpoints.
    ///
    /// Unlike [`Queue::split`](), this method can be used in a `const` context
    ///
    /// # Example
    ///
    /// Create and split a queue at compile time, and pass it to the main
    /// function and an interrupt handler via a mutex at runtime.
    ///
    /// ```
    /// use core::cell::RefCell;
    ///
    /// use critical_section::Mutex;
    /// use heapless::spsc::{Consumer, Producer, Queue, QueueView};
    ///
    /// static PC: (
    ///     Mutex<RefCell<Option<Producer<'_, ()>>>>,
    ///     Mutex<RefCell<Option<Consumer<'_, ()>>>>,
    /// ) = {
    ///     static mut Q: &mut QueueView<()> = &mut Queue::<(), 4>::new();
    ///     // SAFETY: `Q` is only accessible in this scope.
    ///     #[allow(static_mut_refs)]
    ///     let (p, c) = unsafe { Q.split_const() };
    ///
    ///     (
    ///         Mutex::new(RefCell::new(Some(p))),
    ///         Mutex::new(RefCell::new(Some(c))),
    ///     )
    /// };
    ///
    /// fn interrupt() {
    ///     let mut producer = {
    ///         static mut P: Option<Producer<'_, ()>> = None;
    ///         // SAFETY: Mutable access to `P` is allowed exclusively in this scope
    ///         // and `interrupt` cannot be called directly or preempt itself.
    ///         unsafe { &mut P }
    ///     }
    ///     .get_or_insert_with(|| {
    ///         critical_section::with(|cs| PC.0.borrow_ref_mut(cs).take().unwrap())
    ///     });
    ///
    ///     producer.enqueue(()).unwrap();
    /// }
    ///
    /// fn main() {
    ///     let mut consumer = critical_section::with(|cs| PC.1.borrow_ref_mut(cs).take().unwrap());
    ///
    ///     // Interrupt occurs.
    /// #   interrupt();
    ///
    ///     consumer.dequeue().unwrap();
    /// }
    /// ```
    pub const fn split_const(&mut self) -> (Producer<'_, T>, Consumer<'_, T>) {
        (Producer { rb: self }, Consumer { rb: self })
    }
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Clone for Queue<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut new: Self = Self::new();

        for s in self.iter() {
            // SAFETY: `new.capacity() == self.capacity() >= self.len()`,
            // so no overflow is possible.
            unsafe {
                new.enqueue_unchecked(s.clone());
            }
        }

        new
    }
}

impl<T, S, S2> PartialEq<QueueInner<T, S2>> for QueueInner<T, S>
where
    T: PartialEq,
    S: Storage,
    S2: Storage,
{
    fn eq(&self, other: &QueueInner<T, S2>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(v1, v2)| v1 == v2)
    }
}

impl<T, S: Storage> Eq for QueueInner<T, S> where T: Eq {}

/// An iterator over the items of a queue.
pub struct Iter<'a, T> {
    rb: &'a QueueView<T>,
    index: usize,
    len: usize,
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            rb: self.rb,
            index: self.index,
            len: self.len,
        }
    }
}

/// An iterator over the items of a queue.
pub struct IterMut<'a, T> {
    rb: &'a QueueView<T>,
    index: usize,
    len: usize,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let head = self.rb.head.load(Ordering::Relaxed);

            let i = (head + self.index) % self.rb.n();
            self.index += 1;

            Some(unsafe { &*(self.rb.buffer.borrow().get_unchecked(i).get() as *const T) })
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let head = self.rb.head.load(Ordering::Relaxed);

            let i = (head + self.index) % self.rb.n();
            self.index += 1;

            Some(unsafe { &mut *self.rb.buffer.borrow().get_unchecked(i).get().cast::<T>() })
        } else {
            None
        }
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let head = self.rb.head.load(Ordering::Relaxed);

            // self.len > 0, since it's larger than self.index > 0
            let i = (head + self.len - 1) % self.rb.n();
            self.len -= 1;
            Some(unsafe { &*(self.rb.buffer.borrow().get_unchecked(i).get() as *const T) })
        } else {
            None
        }
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let head = self.rb.head.load(Ordering::Relaxed);

            // self.len > 0, since it's larger than self.index > 0
            let i = (head + self.len - 1) % self.rb.n();
            self.len -= 1;
            Some(unsafe { &mut *self.rb.buffer.borrow().get_unchecked(i).get().cast::<T>() })
        } else {
            None
        }
    }
}

impl<T, S: Storage> Drop for QueueInner<T, S> {
    fn drop(&mut self) {
        for item in self {
            unsafe {
                ptr::drop_in_place(item);
            }
        }
    }
}

impl<T, S> fmt::Debug for QueueInner<T, S>
where
    T: fmt::Debug,
    S: Storage,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, S> hash::Hash for QueueInner<T, S>
where
    T: hash::Hash,
    S: Storage,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // iterate over self in order
        for t in self.iter() {
            hash::Hash::hash(t, state);
        }
    }
}

impl<'a, T, S: Storage> IntoIterator for &'a QueueInner<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, S: Storage> IntoIterator for &'a mut QueueInner<T, S> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A consumer; it can dequeue items from the queue.
///
/// **Note:** The consumer semantically owns the `head` pointer of the queue.
pub struct Consumer<'a, T> {
    rb: &'a QueueView<T>,
}

unsafe impl<T> Send for Consumer<'_, T> where T: Send {}

/// A producer; it can enqueue items into the queue.
///
/// **Note:** The producer semantically owns the `tail` pointer of the queue.
pub struct Producer<'a, T> {
    rb: &'a QueueView<T>,
}

unsafe impl<T> Send for Producer<'_, T> where T: Send {}

impl<T> Consumer<'_, T> {
    /// Returns the item in the front of the queue, or `None` if the queue is empty.
    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        unsafe { self.rb.inner_dequeue() }
    }

    /// Returns the item in the front of the queue, without checking if there are elements in the
    /// queue.
    ///
    /// # Safety
    ///
    /// See [`Queue::dequeue_unchecked`].
    #[inline]
    pub unsafe fn dequeue_unchecked(&mut self) -> T {
        self.rb.inner_dequeue_unchecked()
    }

    /// Returns if there are any items to dequeue. When this returns `true`, at least the
    /// first subsequent dequeue will succeed.
    #[inline]
    pub fn ready(&self) -> bool {
        !self.rb.is_empty()
    }

    /// Returns the number of elements in the queue.
    #[inline]
    pub fn len(&self) -> usize {
        self.rb.len()
    }

    /// Returns whether the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::spsc::Queue;
    ///
    /// let mut queue: Queue<u8, 235> = Queue::new();
    /// let (mut producer, mut consumer) = queue.split();
    /// assert!(consumer.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the maximum number of elements the queue can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.rb.capacity()
    }

    /// Returns the item in the front of the queue without dequeuing, or `None` if the queue is
    /// empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::spsc::Queue;
    ///
    /// let mut queue: Queue<u8, 235> = Queue::new();
    /// let (mut producer, mut consumer) = queue.split();
    /// assert_eq!(None, consumer.peek());
    /// producer.enqueue(1);
    /// assert_eq!(Some(&1), consumer.peek());
    /// assert_eq!(Some(1), consumer.dequeue());
    /// assert_eq!(None, consumer.peek());
    /// ```
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.rb.peek()
    }
}

impl<T> Producer<'_, T> {
    /// Adds an `item` to the end of the queue, returns back the `item` if the queue is full.
    #[inline]
    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        unsafe { self.rb.inner_enqueue(item) }
    }

    /// Adds an `item` to the end of the queue, without checking if the queue is full.
    ///
    /// # Safety
    ///
    /// See [`Queue::enqueue_unchecked`].
    #[inline]
    pub unsafe fn enqueue_unchecked(&mut self, item: T) {
        self.rb.inner_enqueue_unchecked(item);
    }

    /// Returns if there is any space to enqueue a new item. When this returns true, at
    /// least the first subsequent enqueue will succeed.
    #[inline]
    pub fn ready(&self) -> bool {
        !self.rb.is_full()
    }

    /// Returns the number of elements in the queue.
    #[inline]
    pub fn len(&self) -> usize {
        self.rb.len()
    }

    /// Returns whether the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::spsc::Queue;
    ///
    /// let mut queue: Queue<u8, 235> = Queue::new();
    /// let (mut producer, mut consumer) = queue.split();
    /// assert!(producer.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the maximum number of elements the queue can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.rb.capacity()
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use super::{Consumer, Producer, Queue};

    use static_assertions::assert_not_impl_any;

    // Ensure a `Queue` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Queue<*const (), 4>: Send);

    // Ensure a `Producer` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Producer<*const ()>: Send);

    // Ensure a `Consumer` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Consumer<*const ()>: Send);

    #[test]
    fn const_split() {
        use critical_section::Mutex;
        use std::cell::RefCell;

        use super::{Consumer, Producer};

        #[allow(clippy::type_complexity)]
        static PC: (
            Mutex<RefCell<Option<Producer<'_, ()>>>>,
            Mutex<RefCell<Option<Consumer<'_, ()>>>>,
        ) = {
            static mut Q: Queue<(), 4> = Queue::new();
            // SAFETY: `Q` is only accessible in this scope.
            #[allow(static_mut_refs)]
            let (p, c) = unsafe { Q.split_const() };

            (
                Mutex::new(RefCell::new(Some(p))),
                Mutex::new(RefCell::new(Some(c))),
            )
        };
        let producer = critical_section::with(|cs| PC.0.borrow_ref_mut(cs).take().unwrap());
        let consumer = critical_section::with(|cs| PC.1.borrow_ref_mut(cs).take().unwrap());

        let mut producer: Producer<'static, ()> = producer;
        let mut consumer: Consumer<'static, ()> = consumer;

        assert_eq!(producer.enqueue(()), Ok(()));
        assert_eq!(consumer.dequeue(), Some(()));
    }

    #[test]
    fn full() {
        let mut rb: Queue<i32, 3> = Queue::new();

        assert!(!rb.is_full());

        rb.enqueue(1).unwrap();
        assert!(!rb.is_full());

        rb.enqueue(2).unwrap();
        assert!(rb.is_full());
    }

    #[test]
    fn empty() {
        let mut rb: Queue<i32, 3> = Queue::new();

        assert!(rb.is_empty());

        rb.enqueue(1).unwrap();
        assert!(!rb.is_empty());

        rb.enqueue(2).unwrap();
        assert!(!rb.is_empty());
    }

    #[test]
    #[cfg_attr(miri, ignore)] // too slow
    fn len() {
        let mut rb: Queue<i32, 3> = Queue::new();

        assert_eq!(rb.len(), 0);

        rb.enqueue(1).unwrap();
        assert_eq!(rb.len(), 1);

        rb.enqueue(2).unwrap();
        assert_eq!(rb.len(), 2);

        for _ in 0..1_000_000 {
            let v = rb.dequeue().unwrap();
            println!("{v}");
            rb.enqueue(v).unwrap();
            assert_eq!(rb.len(), 2);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)] // too slow
    fn try_overflow() {
        const N: usize = 23;
        let mut rb: Queue<i32, N> = Queue::new();

        for i in 0..N as i32 - 1 {
            rb.enqueue(i).unwrap();
        }

        for _ in 0..1_000_000 {
            for i in 0..N as i32 - 1 {
                let d = rb.dequeue().unwrap();
                assert_eq!(d, i);
                rb.enqueue(i).unwrap();
            }
        }
    }

    #[test]
    fn sanity() {
        let mut rb: Queue<i32, 10> = Queue::new();

        let (mut p, mut c) = rb.split();

        assert!(p.ready());

        assert!(!c.ready());

        assert_eq!(c.dequeue(), None);

        p.enqueue(0).unwrap();

        assert_eq!(c.dequeue(), Some(0));
    }

    #[test]
    fn static_new() {
        static mut _Q: Queue<i32, 4> = Queue::new();
    }

    #[test]
    fn drop() {
        struct Droppable;
        impl Droppable {
            fn new() -> Self {
                unsafe {
                    COUNT += 1;
                }
                Self
            }
        }

        impl Drop for Droppable {
            fn drop(&mut self) {
                unsafe {
                    COUNT -= 1;
                }
            }
        }

        static mut COUNT: i32 = 0;

        {
            let mut v: Queue<Droppable, 4> = Queue::new();
            v.enqueue(Droppable::new()).ok().unwrap();
            v.enqueue(Droppable::new()).ok().unwrap();
            v.dequeue().unwrap();
        }

        assert_eq!(unsafe { COUNT }, 0);

        {
            let mut v: Queue<Droppable, 4> = Queue::new();
            v.enqueue(Droppable::new()).ok().unwrap();
            v.enqueue(Droppable::new()).ok().unwrap();
        }

        assert_eq!(unsafe { COUNT }, 0);
    }

    #[test]
    fn iter() {
        let mut rb: Queue<i32, 4> = Queue::new();

        rb.enqueue(0).unwrap();
        rb.dequeue().unwrap();
        rb.enqueue(1).unwrap();
        rb.enqueue(2).unwrap();
        rb.enqueue(3).unwrap();

        let mut items = rb.iter();

        // assert_eq!(items.next(), Some(&0));
        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_double_ended() {
        let mut rb: Queue<i32, 4> = Queue::new();

        rb.enqueue(0).unwrap();
        rb.enqueue(1).unwrap();
        rb.enqueue(2).unwrap();

        let mut items = rb.iter();

        assert_eq!(items.next(), Some(&0));
        assert_eq!(items.next_back(), Some(&2));
        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), None);
        assert_eq!(items.next_back(), None);
    }

    #[test]
    fn iter_mut() {
        let mut rb: Queue<i32, 4> = Queue::new();

        rb.enqueue(0).unwrap();
        rb.enqueue(1).unwrap();
        rb.enqueue(2).unwrap();

        let mut items = rb.iter_mut();

        assert_eq!(items.next(), Some(&mut 0));
        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_mut_double_ended() {
        let mut rb: Queue<i32, 4> = Queue::new();

        rb.enqueue(0).unwrap();
        rb.enqueue(1).unwrap();
        rb.enqueue(2).unwrap();

        let mut items = rb.iter_mut();

        assert_eq!(items.next(), Some(&mut 0));
        assert_eq!(items.next_back(), Some(&mut 2));
        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), None);
        assert_eq!(items.next_back(), None);
    }

    #[test]
    fn wrap_around() {
        let mut rb: Queue<i32, 4> = Queue::new();

        rb.enqueue(0).unwrap();
        rb.enqueue(1).unwrap();
        rb.enqueue(2).unwrap();
        rb.dequeue().unwrap();
        rb.dequeue().unwrap();
        rb.dequeue().unwrap();
        rb.enqueue(3).unwrap();
        rb.enqueue(4).unwrap();

        assert_eq!(rb.len(), 2);
    }

    #[test]
    fn ready_flag() {
        let mut rb: Queue<i32, 3> = Queue::new();
        let (mut p, mut c) = rb.split();
        assert!(!c.ready());
        assert!(p.ready());

        p.enqueue(0).unwrap();

        assert!(c.ready());
        assert!(p.ready());

        p.enqueue(1).unwrap();

        assert!(c.ready());
        assert!(!p.ready());

        c.dequeue().unwrap();

        assert!(c.ready());
        assert!(p.ready());

        c.dequeue().unwrap();

        assert!(!c.ready());
        assert!(p.ready());
    }

    #[test]
    fn clone() {
        let mut rb1: Queue<i32, 4> = Queue::new();
        rb1.enqueue(0).unwrap();
        rb1.enqueue(0).unwrap();
        rb1.dequeue().unwrap();
        rb1.enqueue(0).unwrap();
        let rb2 = rb1.clone();
        assert_eq!(rb1.capacity(), rb2.capacity());
        assert_eq!(rb1.len(), rb2.len());
        assert!(rb1.iter().zip(rb2.iter()).all(|(v1, v2)| v1 == v2));
    }

    #[test]
    fn eq() {
        // generate two queues with same content
        // but different buffer alignment
        let mut rb1: Queue<i32, 4> = Queue::new();
        rb1.enqueue(0).unwrap();
        rb1.enqueue(0).unwrap();
        rb1.dequeue().unwrap();
        rb1.enqueue(0).unwrap();
        let mut rb2: Queue<i32, 4> = Queue::new();
        rb2.enqueue(0).unwrap();
        rb2.enqueue(0).unwrap();
        assert!(rb1 == rb2);
        // test for symmetry
        assert!(rb2 == rb1);
        // test for changes in content
        rb1.enqueue(0).unwrap();
        assert!(rb1 != rb2);
        rb2.enqueue(1).unwrap();
        assert!(rb1 != rb2);
        // test for refexive relation
        assert!(rb1 == rb1);
        assert!(rb2 == rb2);
    }

    #[test]
    fn hash_equality() {
        // generate two queues with same content
        // but different buffer alignment
        let rb1 = {
            let mut rb1: Queue<i32, 4> = Queue::new();
            rb1.enqueue(0).unwrap();
            rb1.enqueue(0).unwrap();
            rb1.dequeue().unwrap();
            rb1.enqueue(0).unwrap();
            rb1
        };
        let rb2 = {
            let mut rb2: Queue<i32, 4> = Queue::new();
            rb2.enqueue(0).unwrap();
            rb2.enqueue(0).unwrap();
            rb2
        };
        let hash1 = {
            let mut hasher1 = hash32::FnvHasher::default();
            rb1.hash(&mut hasher1);
            hasher1.finish()
        };
        let hash2 = {
            let mut hasher2 = hash32::FnvHasher::default();
            rb2.hash(&mut hasher2);
            hasher2.finish()
        };
        assert_eq!(hash1, hash2);
    }
}
