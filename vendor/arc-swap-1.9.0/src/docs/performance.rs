//! Performance characteristics.
//!
//! There are several performance advantages of [`ArcSwap`] over [`RwLock`].
//!
//! ## Lock-free readers
//!
//! All the read operations are always [lock-free]. Most of the time, they are actually
//! [wait-free]. They may wait from time to time, with at least `usize::MAX / 4` [wait-free]
//! accesses in between waits.
//!
//! Writers are [lock-free].
//!
//! Whenever the documentation talks about *contention* in the context of [`ArcSwap`], it talks
//! about contention on the CPU level ‒ multiple cores having to deal with accessing the same cache
//! line. This slows things down (compared to each one accessing its own cache line), but an
//! eventual progress is still guaranteed and the cost is significantly lower than parking threads
//! as with mutex-style contention.
//!
//! ## Speeds
//!
//! The base line speed of read operations is similar to using an *uncontended* [`Mutex`].
//! However, [`load`] suffers no contention from any other read operations and only slight
//! ones during updates. The [`load_full`] operation is additionally contended only on
//! the reference count of the [`Arc`] inside ‒ so, in general, while [`Mutex`] rapidly
//! loses its performance when being in active use by multiple threads at once and
//! [`RwLock`] is slow to start with, [`ArcSwap`] mostly keeps its performance even when read by
//! many threads in parallel.
//!
//! Write operations are considered expensive. A write operation is more expensive than access to
//! an *uncontended* [`Mutex`] and on some architectures even slower than uncontended
//! [`RwLock`]. However, it is faster than either under contention.
//!
//! There are some (very unscientific) [benchmarks] within the source code of the library, and the
//! [`DefaultStrategy`][crate::DefaultStrategy] has some numbers measured on my computer.
//!
//! The exact numbers are highly dependent on the machine used (both absolute numbers and relative
//! between different data structures). Not only architectures have a huge impact (eg. x86 vs ARM),
//! but even AMD vs. Intel or two different Intel processors. Therefore, if what matters is more
//! the speed than the wait-free guarantees, you're advised to do your own measurements.
//!
//! Further speed improvements may be gained by the use of the [`Cache`].
//!
//! ## Consistency
//!
//! The combination of [wait-free] guarantees of readers and no contention between concurrent
//! [`load`]s provides *consistent* performance characteristics of the synchronization mechanism.
//! This might be important for soft-realtime applications (the CPU-level contention caused by a
//! recent update/write operation might be problematic for some hard-realtime cases, though).
//!
//! ## Choosing the right reading operation
//!
//! There are several load operations available. While the general go-to one should be
//! [`load`], there may be situations in which the others are a better match.
//!
//! The [`load`] usually only borrows the instance from the shared [`ArcSwap`]. This makes
//! it faster, because different threads don't contend on the reference count. There are two
//! situations when this borrow isn't possible. If the content gets changed, all existing
//! [`Guard`]s are promoted to contain an owned instance. The promotion is done by the
//! writer, but the readers still need to decrement the reference counts of the old instance when
//! they no longer use it, contending on the count.
//!
//! The other situation derives from internal implementation. The number of borrows each thread can
//! have at each time (across all [`Guard`]s) is limited. If this limit is exceeded, an owned
//! instance is created instead.
//!
//! Therefore, if you intend to hold onto the loaded value for extended time span, you may prefer
//! [`load_full`]. It loads the pointer instance ([`Arc`]) without borrowing, which is
//! slower (because of the possible contention on the reference count), but doesn't consume one of
//! the borrow slots, which will make it more likely for following [`load`]s to have a slot
//! available. Similarly, if some API needs an owned `Arc`, [`load_full`] is more convenient and
//! potentially faster then first [`load`]ing and then cloning that [`Arc`].
//!
//! Additionally, it is possible to use a [`Cache`] to get further speed improvement at the
//! cost of less comfortable API and possibly keeping the older values alive for longer than
//! necessary.
//!
//! [`ArcSwap`]: crate::ArcSwap
//! [`Cache`]: crate::cache::Cache
//! [`Guard`]: crate::Guard
//! [`load`]: crate::ArcSwapAny::load
//! [`load_full`]: crate::ArcSwapAny::load_full
//! [`Arc`]: std::sync::Arc
//! [`Mutex`]: std::sync::Mutex
//! [`RwLock`]: std::sync::RwLock
//! [benchmarks]: https://github.com/vorner/arc-swap/tree/master/benches
//! [lock-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Lock-freedom
//! [wait-free]: https://en.wikipedia.org/wiki/Non-blocking_algorithm#Wait-freedom
