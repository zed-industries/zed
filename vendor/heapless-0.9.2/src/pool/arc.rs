//! `std::sync::Arc`-like API on top of a lock-free memory pool
//!
//! # Example usage
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{arc_pool, pool::arc::{Arc, ArcBlock}};
//!
//! arc_pool!(MyArcPool: u128);
//!
//! // cannot allocate without first giving memory blocks to the pool
//! assert!(MyArcPool.alloc(42).is_err());
//!
//! // (some `no_std` runtimes have safe APIs to create `&'static mut` references)
//! let block: &'static mut ArcBlock<u128> = unsafe {
//!     static mut BLOCK: ArcBlock<u128> = ArcBlock::new();
//!     addr_of_mut!(BLOCK).as_mut().unwrap()
//! };
//!
//! MyArcPool.manage(block);
//!
//! let arc = MyArcPool.alloc(1).unwrap();
//!
//! // number of smart pointers is limited to the number of blocks managed by the pool
//! let res = MyArcPool.alloc(2);
//! assert!(res.is_err());
//!
//! // but cloning does not consume an `ArcBlock`
//! let arc2 = arc.clone();
//!
//! assert_eq!(1, *arc2);
//!
//! // `arc`'s destructor returns the memory block to the pool
//! drop(arc2); // decrease reference counter
//! drop(arc); // release memory
//!
//! // it's now possible to allocate a new `Arc` smart pointer
//! let res = MyArcPool.alloc(3);
//!
//! assert!(res.is_ok());
//! ```
//!
//! # Array block initialization
//!
//! You can create a static variable that contains an array of memory blocks and give all the blocks
//! to the `ArcPool`. This requires an intermediate `const` value as shown below:
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{arc_pool, pool::arc::ArcBlock};
//!
//! arc_pool!(MyArcPool: u128);
//!
//! const POOL_CAPACITY: usize = 8;
//!
//! let blocks: &'static mut [ArcBlock<u128>] = {
//!     const BLOCK: ArcBlock<u128> = ArcBlock::new(); // <=
//!     static mut BLOCKS: [ArcBlock<u128>; POOL_CAPACITY] = [BLOCK; POOL_CAPACITY];
//!     unsafe { addr_of_mut!(BLOCKS).as_mut().unwrap() }
//! };
//!
//! for block in blocks {
//!     MyArcPool.manage(block);
//! }
//! ```

// reference counting logic is based on version 1.63.0 of the Rust standard library (`alloc`  crate)
// which is licensed under 'MIT or APACHE-2.0'
// https://github.com/rust-lang/rust/blob/1.63.0/library/alloc/src/sync.rs#L235 (last visited
// 2022-09-05)

use core::{
    fmt,
    hash::{Hash, Hasher},
    mem::MaybeUninit,
    ops, ptr,
};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;

use atomic::{AtomicUsize, Ordering};

use super::treiber::{NonNullPtr, Stack, UnionNode};

/// Creates a new `ArcPool` singleton with the given `$name` that manages the specified `$data_type`
///
/// For more extensive documentation see the [module level documentation](crate::pool::arc)
#[macro_export]
macro_rules! arc_pool {
    ($name:ident: $data_type:ty) => {
        pub struct $name;

        impl $crate::pool::arc::ArcPool for $name {
            type Data = $data_type;

            fn singleton() -> &'static $crate::pool::arc::ArcPoolImpl<$data_type> {
                // Even though the static variable is not exposed to user code, it is
                // still useful to have a descriptive symbol name for debugging.
                #[allow(non_upper_case_globals)]
                static $name: $crate::pool::arc::ArcPoolImpl<$data_type> =
                    $crate::pool::arc::ArcPoolImpl::new();

                &$name
            }
        }

        impl $name {
            /// Inherent method version of `ArcPool::alloc`
            #[allow(dead_code)]
            pub fn alloc(
                &self,
                value: $data_type,
            ) -> Result<$crate::pool::arc::Arc<$name>, $data_type> {
                <$name as $crate::pool::arc::ArcPool>::alloc(value)
            }

            /// Inherent method version of `ArcPool::manage`
            #[allow(dead_code)]
            pub fn manage(&self, block: &'static mut $crate::pool::arc::ArcBlock<$data_type>) {
                <$name as $crate::pool::arc::ArcPool>::manage(block)
            }
        }
    };
}

/// A singleton that manages `pool::arc::Arc` smart pointers
pub trait ArcPool: Sized {
    /// The data type managed by the memory pool
    type Data: 'static;

    /// `arc_pool!` implementation detail
    #[doc(hidden)]
    fn singleton() -> &'static ArcPoolImpl<Self::Data>;

    /// Allocate a new `Arc` smart pointer initialized to the given `value`
    ///
    /// `manage` should be called at least once before calling `alloc`
    ///
    /// # Errors
    ///
    /// The `Err`or variant is returned when the memory pool has run out of memory blocks
    fn alloc(value: Self::Data) -> Result<Arc<Self>, Self::Data> {
        Ok(Arc {
            node_ptr: Self::singleton().alloc(value)?,
        })
    }

    /// Add a statically allocated memory block to the memory pool
    fn manage(block: &'static mut ArcBlock<Self::Data>) {
        Self::singleton().manage(block);
    }
}

/// `arc_pool!` implementation detail
// newtype to avoid having to make field types public
#[doc(hidden)]
pub struct ArcPoolImpl<T> {
    stack: Stack<UnionNode<MaybeUninit<ArcInner<T>>>>,
}

impl<T> ArcPoolImpl<T> {
    /// `arc_pool!` implementation detail
    #[doc(hidden)]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    fn alloc(&self, value: T) -> Result<NonNullPtr<UnionNode<MaybeUninit<ArcInner<T>>>>, T> {
        if let Some(node_ptr) = self.stack.try_pop() {
            let inner = ArcInner {
                data: value,
                strong: AtomicUsize::new(1),
            };
            unsafe { node_ptr.as_ptr().cast::<ArcInner<T>>().write(inner) }

            Ok(node_ptr)
        } else {
            Err(value)
        }
    }

    fn manage(&self, block: &'static mut ArcBlock<T>) {
        let node: &'static mut _ = &mut block.node;

        // SAFETY: The node within an `ArcBlock` is always properly initialized for linking because
        // the only way for         client code to construct an `ArcBlock` is through
        // `ArcBlock::new`. The `NonNullPtr` comes from a         reference, so it is
        // guaranteed to be dereferencable. It is also unique because the `ArcBlock` itself
        //         is passed as a `&mut`
        unsafe { self.stack.push(NonNullPtr::from_static_mut_ref(node)) }
    }
}

unsafe impl<T> Sync for ArcPoolImpl<T> {}

/// Like `std::sync::Arc` but managed by memory pool `P`
pub struct Arc<P>
where
    P: ArcPool,
{
    node_ptr: NonNullPtr<UnionNode<MaybeUninit<ArcInner<P::Data>>>>,
}

impl<P> Arc<P>
where
    P: ArcPool,
{
    fn inner(&self) -> &ArcInner<P::Data> {
        unsafe { &*self.node_ptr.as_ptr().cast::<ArcInner<P::Data>>() }
    }

    fn from_inner(node_ptr: NonNullPtr<UnionNode<MaybeUninit<ArcInner<P::Data>>>>) -> Self {
        Self { node_ptr }
    }

    unsafe fn get_mut_unchecked(this: &mut Self) -> &mut P::Data {
        &mut *ptr::addr_of_mut!((*this.node_ptr.as_ptr().cast::<ArcInner<P::Data>>()).data)
    }

    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        // run `P::Data`'s destructor
        ptr::drop_in_place(Self::get_mut_unchecked(self));

        // return memory to pool
        P::singleton().stack.push(self.node_ptr);
    }
}

impl<P> AsRef<P::Data> for Arc<P>
where
    P: ArcPool,
{
    fn as_ref(&self) -> &P::Data {
        self
    }
}

const MAX_REFCOUNT: usize = (isize::MAX) as usize;

impl<P> Clone for Arc<P>
where
    P: ArcPool,
{
    fn clone(&self) -> Self {
        let old_size = self.inner().strong.fetch_add(1, Ordering::Relaxed);

        if old_size > MAX_REFCOUNT {
            // XXX original code calls `intrinsics::abort` which is unstable API
            panic!();
        }

        Self::from_inner(self.node_ptr)
    }
}

impl<A> fmt::Debug for Arc<A>
where
    A: ArcPool,
    A::Data: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<P> ops::Deref for Arc<P>
where
    P: ArcPool,
{
    type Target = P::Data;

    fn deref(&self) -> &Self::Target {
        unsafe { &*ptr::addr_of!((*self.node_ptr.as_ptr().cast::<ArcInner<P::Data>>()).data) }
    }
}

impl<A> fmt::Display for Arc<A>
where
    A: ArcPool,
    A::Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<A> Drop for Arc<A>
where
    A: ArcPool,
{
    fn drop(&mut self) {
        if self.inner().strong.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        atomic::fence(Ordering::Acquire);

        unsafe { self.drop_slow() }
    }
}

impl<A> Eq for Arc<A>
where
    A: ArcPool,
    A::Data: Eq,
{
}

impl<A> Hash for Arc<A>
where
    A: ArcPool,
    A::Data: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(state);
    }
}

impl<A> Ord for Arc<A>
where
    A: ArcPool,
    A::Data: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        A::Data::cmp(self, other)
    }
}

impl<A, B> PartialEq<Arc<B>> for Arc<A>
where
    A: ArcPool,
    B: ArcPool,
    A::Data: PartialEq<B::Data>,
{
    fn eq(&self, other: &Arc<B>) -> bool {
        A::Data::eq(self, &**other)
    }
}

impl<A, B> PartialOrd<Arc<B>> for Arc<A>
where
    A: ArcPool,
    B: ArcPool,
    A::Data: PartialOrd<B::Data>,
{
    fn partial_cmp(&self, other: &Arc<B>) -> Option<core::cmp::Ordering> {
        A::Data::partial_cmp(self, &**other)
    }
}

unsafe impl<A> Send for Arc<A>
where
    A: ArcPool,
    A::Data: Sync + Send,
{
}

unsafe impl<A> Sync for Arc<A>
where
    A: ArcPool,
    A::Data: Sync + Send,
{
}

impl<A> Unpin for Arc<A> where A: ArcPool {}

struct ArcInner<T> {
    data: T,
    strong: AtomicUsize,
}

/// A chunk of memory that an `ArcPool` can manage
pub struct ArcBlock<T> {
    node: UnionNode<MaybeUninit<ArcInner<T>>>,
}

impl<T> ArcBlock<T> {
    /// Creates a new memory block
    pub const fn new() -> Self {
        Self {
            node: UnionNode::unlinked(),
        }
    }
}

impl<T> Default for ArcBlock<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::addr_of_mut;

    #[test]
    fn cannot_alloc_if_empty() {
        arc_pool!(MyArcPool: i32);

        assert_eq!(Err(42), MyArcPool.alloc(42),);
    }

    #[test]
    fn can_alloc_if_manages_one_block() {
        arc_pool!(MyArcPool: i32);

        let block = unsafe {
            static mut BLOCK: ArcBlock<i32> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        assert_eq!(42, *MyArcPool.alloc(42).unwrap());
    }

    #[test]
    fn alloc_drop_alloc() {
        arc_pool!(MyArcPool: i32);

        let block = unsafe {
            static mut BLOCK: ArcBlock<i32> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(1).unwrap();

        drop(arc);

        assert_eq!(2, *MyArcPool.alloc(2).unwrap());
    }

    #[test]
    fn strong_count_starts_at_one() {
        arc_pool!(MyArcPool: i32);

        let block = unsafe {
            static mut BLOCK: ArcBlock<i32> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(1).ok().unwrap();

        assert_eq!(1, arc.inner().strong.load(Ordering::Relaxed));
    }

    #[test]
    fn clone_increases_strong_count() {
        arc_pool!(MyArcPool: i32);

        let block = unsafe {
            static mut BLOCK: ArcBlock<i32> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(1).ok().unwrap();

        let before = arc.inner().strong.load(Ordering::Relaxed);

        let arc2 = arc.clone();

        let expected = before + 1;
        assert_eq!(expected, arc.inner().strong.load(Ordering::Relaxed));
        assert_eq!(expected, arc2.inner().strong.load(Ordering::Relaxed));
    }

    #[test]
    fn drop_decreases_strong_count() {
        arc_pool!(MyArcPool: i32);

        let block = unsafe {
            static mut BLOCK: ArcBlock<i32> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(1).ok().unwrap();
        let arc2 = arc.clone();

        let before = arc.inner().strong.load(Ordering::Relaxed);

        drop(arc);

        let expected = before - 1;
        assert_eq!(expected, arc2.inner().strong.load(Ordering::Relaxed));
    }

    #[test]
    fn runs_destructor_exactly_once_when_strong_count_reaches_zero() {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        pub struct MyStruct;

        impl Drop for MyStruct {
            fn drop(&mut self) {
                COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }

        arc_pool!(MyArcPool: MyStruct);

        let block = unsafe {
            static mut BLOCK: ArcBlock<MyStruct> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(MyStruct).ok().unwrap();

        assert_eq!(0, COUNT.load(Ordering::Relaxed));

        drop(arc);

        assert_eq!(1, COUNT.load(Ordering::Relaxed));
    }

    #[test]
    fn zst_is_well_aligned() {
        #[repr(align(4096))]
        pub struct Zst4096;

        arc_pool!(MyArcPool: Zst4096);

        let block = unsafe {
            static mut BLOCK: ArcBlock<Zst4096> = ArcBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyArcPool.manage(block);

        let arc = MyArcPool.alloc(Zst4096).ok().unwrap();

        let raw = &*arc as *const Zst4096;
        assert_eq!(0, raw as usize % 4096);
    }
}
