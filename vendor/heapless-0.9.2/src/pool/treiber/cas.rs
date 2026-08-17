use core::{marker::PhantomData, ptr::NonNull};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;

use atomic::Ordering;

use super::{Node, Stack};

#[cfg(target_pointer_width = "32")]
mod types {
    use super::atomic;

    pub type Inner = u64;
    pub type InnerAtomic = atomic::AtomicU64;
    pub type InnerNonZero = core::num::NonZeroU64;

    pub type Tag = core::num::NonZeroU32;
    pub type Address = u32;
}

#[cfg(target_pointer_width = "64")]
mod types {
    use super::atomic;

    pub type Inner = u128;
    pub type InnerAtomic = atomic::AtomicU128;
    pub type InnerNonZero = core::num::NonZeroU128;

    pub type Tag = core::num::NonZeroU64;
    pub type Address = u64;
}

use types::*;

pub struct AtomicPtr<N>
where
    N: Node,
{
    inner: InnerAtomic,
    _marker: PhantomData<*mut N>,
}

impl<N> AtomicPtr<N>
where
    N: Node,
{
    #[inline]
    pub const fn null() -> Self {
        Self {
            inner: InnerAtomic::new(0),
            _marker: PhantomData,
        }
    }

    fn compare_and_exchange_weak(
        &self,
        current: Option<NonNullPtr<N>>,
        new: Option<NonNullPtr<N>>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), Option<NonNullPtr<N>>> {
        self.inner
            .compare_exchange_weak(
                current.map(NonNullPtr::into_inner).unwrap_or_default(),
                new.map(NonNullPtr::into_inner).unwrap_or_default(),
                success,
                failure,
            )
            .map(drop)
            .map_err(|value| {
                // SAFETY: `value` cam from a `NonNullPtr::into_inner` call.
                unsafe { NonNullPtr::from_inner(value) }
            })
    }

    #[inline]
    fn load(&self, order: Ordering) -> Option<NonNullPtr<N>> {
        Some(NonNullPtr {
            inner: InnerNonZero::new(self.inner.load(order))?,
            _marker: PhantomData,
        })
    }

    #[inline]
    fn store(&self, value: Option<NonNullPtr<N>>, order: Ordering) {
        self.inner
            .store(value.map(NonNullPtr::into_inner).unwrap_or_default(), order);
    }
}

pub struct NonNullPtr<N>
where
    N: Node,
{
    inner: InnerNonZero,
    _marker: PhantomData<*mut N>,
}

impl<N> Clone for NonNullPtr<N>
where
    N: Node,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<N> Copy for NonNullPtr<N> where N: Node {}

impl<N> NonNullPtr<N>
where
    N: Node,
{
    #[inline]
    pub fn as_ptr(&self) -> *mut N {
        self.inner.get() as *mut N
    }

    #[inline]
    pub fn from_static_mut_ref(reference: &'static mut N) -> Self {
        // SAFETY: `reference` is a static mutable reference, i.e. a valid pointer.
        unsafe { Self::new_unchecked(initial_tag(), NonNull::from(reference)) }
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked(ptr: *mut N) -> Self {
        debug_assert!(!ptr.is_null(), "Pointer must be non-null");
        Self::new_unchecked(initial_tag(), NonNull::new_unchecked(ptr))
    }

    /// # Safety
    ///
    /// - `ptr` must be a valid pointer.
    #[inline]
    unsafe fn new_unchecked(tag: Tag, ptr: NonNull<N>) -> Self {
        let value =
            (Inner::from(tag.get()) << Address::BITS) | Inner::from(ptr.as_ptr() as Address);

        Self {
            // SAFETY: `value` is constructed from a `Tag` which is non-zero and half the
            //         size of the `InnerNonZero` type, and a `NonNull<N>` pointer.
            inner: unsafe { InnerNonZero::new_unchecked(value) },
            _marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// - `value` must come from a `Self::into_inner` call.
    #[inline]
    unsafe fn from_inner(value: Inner) -> Option<Self> {
        Some(Self {
            inner: InnerNonZero::new(value)?,
            _marker: PhantomData,
        })
    }

    #[inline]
    fn non_null(&self) -> NonNull<N> {
        // SAFETY: `Self` can only be constructed using a `NonNull<N>`.
        unsafe { NonNull::new_unchecked(self.as_ptr()) }
    }

    #[inline]
    fn into_inner(self) -> Inner {
        self.inner.get()
    }

    #[inline]
    fn tag(&self) -> Tag {
        // SAFETY: `self.inner` was constructed from a non-zero `Tag`.
        unsafe { Tag::new_unchecked((self.inner.get() >> Address::BITS) as Address) }
    }

    fn increment_tag(&mut self) {
        let new_tag = self.tag().checked_add(1).unwrap_or_else(initial_tag);

        // SAFETY: `self.non_null()` is a valid pointer.
        *self = unsafe { Self::new_unchecked(new_tag, self.non_null()) };
    }
}

#[inline]
const fn initial_tag() -> Tag {
    Tag::MIN
}

/// Pushes the given node on top of the stack
///
/// # Safety
///
/// - `new_top` must point to a node that is properly initialized for linking, i.e.
///   `new_top.as_ref().next()` must be valid to call (see [`Node::next`])
/// - `new_top` must be convertible to a reference (see [`NonNull::as_ref`])
pub unsafe fn push<N>(stack: &Stack<N>, new_top: NonNullPtr<N>)
where
    N: Node,
{
    let mut top = stack.top.load(Ordering::Relaxed);

    loop {
        new_top
            .non_null()
            .as_ref()
            // SAFETY: Caller ensures that it is safe to call `next`
            .next()
            .store(top, Ordering::Relaxed);

        if let Err(p) = stack.top.compare_and_exchange_weak(
            top,
            Some(new_top),
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            top = p;
        } else {
            return;
        }
    }
}

pub fn try_pop<N>(stack: &Stack<N>) -> Option<NonNullPtr<N>>
where
    N: Node,
{
    loop {
        if let Some(mut top) = stack.top.load(Ordering::Acquire) {
            let next = unsafe { top.non_null().as_ref().next().load(Ordering::Relaxed) };

            if stack
                .top
                .compare_and_exchange_weak(Some(top), next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // Prevent the ABA problem (https://en.wikipedia.org/wiki/Treiber_stack#Correctness).
                //
                // Without this, the following would be possible:
                //
                // | Thread 1                      | Thread 2                | Stack                        |
                // |-------------------------------|-------------------------|------------------------------|
                // | push((1, 1))                  |                         | (1, 1)                       |
                // | push((1, 2))                  |                         | (1, 2) -> (1, 1)             |
                // | p = try_pop()::load // (1, 2) |                         | (1, 2) -> (1, 1)             |
                // |                               | p = try_pop() // (1, 2) | (1, 1)                       |
                // |                               | push((1, 3))            | (1, 3) -> (1, 1)             |
                // |                               | push(p)                 | (1, 2) -> (1, 3) -> (1, 1)   |
                // | try_pop()::cas(p, p.next)     |                         | (1, 1)                       |
                //
                // As can be seen, the `cas` operation succeeds, wrongly removing pointer `3` from
                // the stack.
                //
                // By incrementing the tag before returning the pointer, it cannot be pushed again
                // with the, same tag, preventing the `try_pop()::cas(p, p.next)`
                // operation from succeeding.
                //
                // With this fix, `try_pop()` in thread 2 returns `(2, 2)` and the comparison
                // between `(1, 2)` and `(2, 2)` fails, restarting the loop and
                // correctly removing the new top:
                //
                // | Thread 1                      | Thread 2                | Stack                        |
                // |-------------------------------|-------------------------|------------------------------|
                // | push((1, 1))                  |                         | (1, 1)                       |
                // | push((1, 2))                  |                         | (1, 2) -> (1, 1)             |
                // | p = try_pop()::load // (1, 2) |                         | (1, 2) -> (1, 1)             |
                // |                               | p = try_pop() // (2, 2) | (1, 1)                       |
                // |                               | push((1, 3))            | (1, 3) -> (1, 1)             |
                // |                               | push(p)                 | (2, 2) -> (1, 3) -> (1, 1)   |
                // | try_pop()::cas(p, p.next)     |                         | (2, 2) -> (1, 3) -> (1, 1)   |
                // | p = try_pop()::load // (2, 2) |                         | (2, 2) -> (1, 3) -> (1, 1)   |
                // | try_pop()::cas(p, p.next)     |                         | (1, 3) -> (1, 1)             |
                top.increment_tag();

                return Some(top);
            }
        } else {
            // stack observed as empty
            return None;
        }
    }
}
