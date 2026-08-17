//! Strategies for protecting the reference counts.
//!
//! There are multiple algorithms how to protect the reference counts while they're being updated
//! by multiple threads, each with its own set of pros and cons. The [`DefaultStrategy`] is used by
//! default and should generally be the least surprising option. It is possible to pick a different
//! strategy.
//!
//! For now, the traits in here are sealed and don't expose any methods to the users of the crate.
//! This is because we are not confident about the details just yet. In the future it may be
//! possible for downstream users to implement their own, but for now it is only so users can
//! choose one of the provided.
//!
//! It is expected that future strategies would come with different capabilities and limitations.
//! In particular, some that are not "tight" in the cleanup (delay the cleanup) or not support the
//! compare and swap operations.
//!
//! Currently, we have these strategies:
//!
//! * [`DefaultStrategy`] (this one is used implicitly)
//! * [`RwLock<()>`][std::sync::RwLock]
//!
//! # Testing
//!
//! Formally, the [`RwLock<()>`][std::sync::RwLock] may be used as a strategy too. It doesn't have
//! the performance characteristics or lock-free guarantees of the others, but it is much simpler
//! and contains less `unsafe` code (actually, less code altogether). Therefore, it can be used for
//! testing purposes and cross-checking.
//!
//! Note that generally, using [`RwLock<Arc<T>>`][std::sync::RwLock] is likely to be better
//! performance wise. So if the goal is to not use third-party unsafe code, only the one in
//! [`std`], that is the better option. This is provided mostly for investigation and testing of
//! [`ArcSwap`] itself or algorithms written to use [`ArcSwap`].
//!
//! *This is not meant to be used in production code*.
//!
//! [`ArcSwap`]: crate::ArcSwap
//! [`load`]: crate::ArcSwapAny::load

use core::borrow::Borrow;
use core::sync::atomic::AtomicPtr;

use crate::ref_cnt::RefCnt;

pub(crate) mod hybrid;

#[cfg(all(
    feature = "internal-test-strategies",
    feature = "experimental-thread-local"
))]
compile_error!("experimental-thread-local is incompatible with internal-test-strategies as it enables #[no_std]");

#[cfg(feature = "internal-test-strategies")]
mod rw_lock;
// Do not use from outside of the crate.
#[cfg(feature = "internal-test-strategies")]
#[doc(hidden)]
pub mod test_strategies;

use self::hybrid::{DefaultConfig, HybridStrategy};

/// The default strategy.
///
/// It is used by the type aliases [`ArcSwap`][crate::ArcSwap] and
/// [`ArcSwapOption`][crate::ArcSwapOption]. Only the other strategies need to be used explicitly.
///
/// # Performance characteristics
///
/// * It is optimized for read-heavy situations, with possibly many concurrent read accesses from
///   multiple threads. Readers don't contend each other at all.
/// * Readers are wait-free (with the exception of at most once in `usize::MAX / 4` accesses, which
///   is only lock-free).
/// * Writers are lock-free.
/// * Reclamation is exact ‒ the resource is released as soon as possible (works like RAII, not
///   like a traditional garbage collector; can contain non-`'static` data).
///
/// Each thread has a limited number of fast slots (currently 8, but the exact number is not
/// guaranteed). If it holds at most that many [`Guard`]s at once, acquiring them is fast. Once
/// these slots are used up (by holding to these many [`Guard`]s), acquiring more of them will be
/// slightly slower, but still wait-free.
///
/// If you expect to hold a lot of "handles" to the data around, or hold onto it for a long time,
/// you may want to prefer the [`load_full`][crate::ArcSwapAny::load_full] method.
///
/// The speed of the fast slots is in the ballpark of locking an *uncontented* mutex. The advantage
/// over the mutex is the stability of speed in the face of contention from other threads ‒ while
/// the performance of mutex goes rapidly down, the slowdown of running out of held slots or heavy
/// concurrent writer thread in the area of single-digit multiples.
///
/// The ballpark benchmark figures (my older computer) are around these, but you're welcome to run
/// the benchmarks in the git repository or write your own.
///
/// * Load (both uncontented and contented by other loads): ~30ns
/// * `load_full`: ~50ns uncontented, goes up a bit with other `load_full` in other threads on the
///   same `Arc` value (~80-100ns).
/// * Loads after running out of the slots ‒ about 10-20ns slower than `load_full`.
/// * Stores: Dependent on number of threads, but generally low microseconds.
/// * Loads with heavy concurrent writer (to the same `ArcSwap`): ~250ns.
///
/// [`load`]: crate::ArcSwapAny::load
/// [`Guard`]: crate::Guard
pub type DefaultStrategy = HybridStrategy<DefaultConfig>;

/// Strategy for isolating instances.
///
/// It is similar to [`DefaultStrategy`], however the spin lock is not sharded (therefore multiple
/// concurrent threads might get bigger hit when multiple threads have to fall back). Nevertheless,
/// each instance has a private spin lock, not influencing the other instances. That also makes
/// them bigger in memory.
///
/// The hazard pointers are still shared between all instances.
///
/// The purpose of this strategy is meant for cases where a single instance is going to be
/// "tortured" a lot, so it should not overflow to other instances.
///
/// This too may be changed for something else (but with at least as good guarantees, primarily
/// that other instances won't get influenced by the "torture").
// Testing if the DefaultStrategy is good enough to replace it fully and then deprecate.
#[doc(hidden)]
pub type IndependentStrategy = DefaultStrategy;

// TODO: When we are ready to un-seal, should these traits become unsafe?

pub(crate) mod sealed {
    use super::*;
    use crate::as_raw::AsRaw;

    pub trait Protected<T>: Borrow<T> {
        fn into_inner(self) -> T;
        fn from_inner(ptr: T) -> Self;
    }

    pub trait InnerStrategy<T: RefCnt> {
        // Drop „unlocks“
        type Protected: Protected<T>;
        unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> Self::Protected;
        unsafe fn wait_for_readers(&self, old: *const T::Base, storage: &AtomicPtr<T::Base>);
    }

    pub trait CaS<T: RefCnt>: InnerStrategy<T> {
        unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
            &self,
            storage: &AtomicPtr<T::Base>,
            current: C,
            new: T,
        ) -> Self::Protected;
    }
}

/// A strategy for protecting the reference counted pointer `T`.
///
/// This chooses the algorithm for how the reference counts are protected. Note that the user of
/// the crate can't implement the trait and can't access any method; this is hopefully temporary
/// measure to make sure the interface is not part of the stability guarantees of the crate. Once
/// enough experience is gained with implementing various strategies, it will be un-sealed and
/// users will be able to provide their own implementation.
///
/// For now, the trait works only as a bound to talk about the types that represent strategies.
pub trait Strategy<T: RefCnt>: sealed::InnerStrategy<T> {}
impl<T: RefCnt, S: sealed::InnerStrategy<T>> Strategy<T> for S {}

/// An extension of the [`Strategy`], allowing for compare and swap operation.
///
/// The compare and swap operation is "advanced" and not all strategies need to support them.
/// Therefore, it is a separate trait.
///
/// Similarly, it is not yet made publicly usable or implementable and works only as a bound.
pub trait CaS<T: RefCnt>: sealed::CaS<T> {}
impl<T: RefCnt, S: sealed::CaS<T>> CaS<T> for S {}
