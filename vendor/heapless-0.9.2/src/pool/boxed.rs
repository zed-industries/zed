//! `std::boxed::Box`-like API on top of a lock-free memory pool
//!
//! # Example usage
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{box_pool, pool::boxed::{Box, BoxBlock}};
//!
//! box_pool!(MyBoxPool: u128);
//!
//! // cannot allocate without first giving memory blocks to the pool
//! assert!(MyBoxPool.alloc(42).is_err());
//!
//! // (some `no_std` runtimes have safe APIs to create `&'static mut` references)
//! let block: &'static mut BoxBlock<u128> = unsafe {
//!     static mut BLOCK: BoxBlock <u128>= BoxBlock::new();
//!     addr_of_mut!(BLOCK).as_mut().unwrap()
//! };
//!
//! // give block of memory to the pool
//! MyBoxPool.manage(block);
//!
//! // it's now possible to allocate
//! let mut boxed = MyBoxPool.alloc(1).unwrap();
//!
//! // mutation is possible
//! *boxed += 1;
//! assert_eq!(2, *boxed);
//!
//! // number of boxes is limited to the number of blocks managed by the pool
//! let res = MyBoxPool.alloc(3);
//! assert!(res.is_err());
//!
//! // give another memory block to the pool
//! MyBoxPool.manage(unsafe {
//!     static mut BLOCK: BoxBlock<u128> = BoxBlock::new();
//!     addr_of_mut!(BLOCK).as_mut().unwrap()
//! });
//!
//! // cloning also consumes a memory block from the pool
//! let mut separate_box = boxed.clone();
//! *separate_box += 1;
//! assert_eq!(3, *separate_box);
//!
//! // after the clone it's not possible to allocate again
//! let res = MyBoxPool.alloc(4);
//! assert!(res.is_err());
//!
//! // `boxed`'s destructor returns the memory block to the pool
//! drop(boxed);
//!
//! // it's possible to allocate again
//! let res = MyBoxPool.alloc(5);
//!
//! assert!(res.is_ok());
//! ```
//!
//! # Array block initialization
//!
//! You can create a static variable that contains an array of memory blocks and give all the blocks
//! to the `BoxPool`. This requires an intermediate `const` value as shown below:
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{box_pool, pool::boxed::BoxBlock};
//!
//! box_pool!(MyBoxPool: u128);
//!
//! const POOL_CAPACITY: usize = 8;
//!
//! let blocks: &'static mut [BoxBlock<u128>] = {
//!     #[allow(clippy::declare_interior_mutable_const)]
//!     const BLOCK: BoxBlock<u128> = BoxBlock::new(); // <=
//!     static mut BLOCKS: [BoxBlock<u128>; POOL_CAPACITY] = [BLOCK; POOL_CAPACITY];
//!     unsafe { addr_of_mut!(BLOCKS).as_mut().unwrap() }
//! };
//!
//! for block in blocks {
//!     MyBoxPool.manage(block);
//! }
//! ```

use core::{
    fmt,
    hash::{Hash, Hasher},
    mem::{ManuallyDrop, MaybeUninit},
    ops, ptr,
    ptr::{addr_of, addr_of_mut},
};
use stable_deref_trait::StableDeref;

use super::treiber::{NonNullPtr, Stack, UnionNode};

/// Creates a new `BoxPool` singleton with the given `$name` that manages the specified `$data_type`
///
/// For more extensive documentation see the [module level documentation](crate::pool::boxed)
#[macro_export]
macro_rules! box_pool {
    ($name:ident: $data_type:ty) => {
        pub struct $name;

        impl $crate::pool::boxed::BoxPool for $name {
            type Data = $data_type;

            fn singleton() -> &'static $crate::pool::boxed::BoxPoolImpl<$data_type> {
                // Even though the static variable is not exposed to user code, it is
                // still useful to have a descriptive symbol name for debugging.
                #[allow(non_upper_case_globals)]
                static $name: $crate::pool::boxed::BoxPoolImpl<$data_type> =
                    $crate::pool::boxed::BoxPoolImpl::new();

                &$name
            }
        }

        impl $name {
            /// Inherent method version of `BoxPool::alloc`
            #[allow(dead_code)]
            pub fn alloc(
                &self,
                value: $data_type,
            ) -> Result<$crate::pool::boxed::Box<$name>, $data_type> {
                <$name as $crate::pool::boxed::BoxPool>::alloc(value)
            }

            /// Inherent method version of `BoxPool::manage`
            #[allow(dead_code)]
            pub fn manage(&self, block: &'static mut $crate::pool::boxed::BoxBlock<$data_type>) {
                <$name as $crate::pool::boxed::BoxPool>::manage(block)
            }
        }
    };
}

/// A singleton that manages `pool::boxed::Box`-es
///
/// # Usage
///
/// Do not implement this trait yourself; instead use the `box_pool!` macro to create a type that
/// implements this trait.
///
/// # Semver guarantees
///
/// *Implementing* this trait is exempt from semver guarantees.
/// i.e. a new patch release is allowed to break downstream `BoxPool` implementations.
///
/// *Using* the trait, e.g. in generic code, does fall under semver guarantees.
pub trait BoxPool: Sized {
    /// The data type managed by the memory pool
    type Data: 'static;

    /// `box_pool!` implementation detail
    #[doc(hidden)]
    fn singleton() -> &'static BoxPoolImpl<Self::Data>;

    /// Allocate a new `Box` initialized to the given `value`
    ///
    /// `manage` should be called at least once before calling `alloc`
    ///
    /// # Errors
    ///
    /// The `Err`or variant is returned when the memory pool has run out of memory blocks
    fn alloc(value: Self::Data) -> Result<Box<Self>, Self::Data> {
        Ok(Box {
            node_ptr: Self::singleton().alloc(value)?,
        })
    }

    /// Add a statically allocated memory block to the memory pool
    fn manage(block: &'static mut BoxBlock<Self::Data>) {
        Self::singleton().manage(block);
    }
}

/// Like `std::boxed::Box` but managed by memory pool `P` rather than `#[global_allocator]`
pub struct Box<P>
where
    P: BoxPool,
{
    node_ptr: NonNullPtr<UnionNode<MaybeUninit<P::Data>>>,
}

impl<P> Box<P>
where
    P: BoxPool,
{
    /// Consumes the `Box`, returning the wrapped pointer
    pub fn into_raw(b: Self) -> *mut P::Data {
        let mut b = ManuallyDrop::new(b);
        // SAFETY: `b` is not dropped, so the pointer remains valid however caller must ensure that
        // eventually it is returned to the pool
        addr_of_mut!(**b)
    }

    /// Constructs a `Box<P>` from a raw pointer
    ///
    /// # Safety
    ///
    /// The `ptr` must have been allocated by the same `BoxPool` `P` that this `Box<P>` uses.
    pub unsafe fn from_raw(ptr: *mut P::Data) -> Self {
        // SAFETY: caller must guarantee that `ptr` is valid and was allocated by `P`
        debug_assert!(!ptr.is_null(), "Pointer must be non-null");

        // Calculate the offset of the `data` field within `UnionNode` (with the current layout
        // tha data is at the beginning so offset is zero, but this may change in the future)
        let uninit_union_node = MaybeUninit::<UnionNode<MaybeUninit<P::Data>>>::uninit();
        let data_ptr = unsafe { addr_of!((*uninit_union_node.as_ptr()).data) };
        let data_offset = (data_ptr as usize) - (uninit_union_node.as_ptr() as usize);
        let union_node_ptr = ptr
            .cast::<u8>()
            .sub(data_offset)
            .cast::<UnionNode<MaybeUninit<P::Data>>>();

        Self {
            node_ptr: NonNullPtr::from_ptr_unchecked(union_node_ptr),
        }
    }
}

impl<A> Clone for Box<A>
where
    A: BoxPool,
    A::Data: Clone,
{
    fn clone(&self) -> Self {
        A::alloc((**self).clone()).ok().expect("OOM")
    }
}

impl<A> fmt::Debug for Box<A>
where
    A: BoxPool,
    A::Data: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<P> ops::Deref for Box<P>
where
    P: BoxPool,
{
    type Target = P::Data;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.node_ptr.as_ptr().cast::<P::Data>() }
    }
}

impl<P> ops::DerefMut for Box<P>
where
    P: BoxPool,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.node_ptr.as_ptr().cast::<P::Data>() }
    }
}

unsafe impl<P> StableDeref for Box<P> where P: BoxPool {}

impl<A> fmt::Display for Box<A>
where
    A: BoxPool,
    A::Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<P> Drop for Box<P>
where
    P: BoxPool,
{
    fn drop(&mut self) {
        let node = self.node_ptr;

        unsafe { ptr::drop_in_place(node.as_ptr().cast::<P::Data>()) }

        unsafe { P::singleton().stack.push(node) }
    }
}

impl<A> Eq for Box<A>
where
    A: BoxPool,
    A::Data: Eq,
{
}

impl<A> Hash for Box<A>
where
    A: BoxPool,
    A::Data: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(state);
    }
}

impl<A> Ord for Box<A>
where
    A: BoxPool,
    A::Data: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        A::Data::cmp(self, other)
    }
}

impl<A, B> PartialEq<Box<B>> for Box<A>
where
    A: BoxPool,
    B: BoxPool,
    A::Data: PartialEq<B::Data>,
{
    fn eq(&self, other: &Box<B>) -> bool {
        A::Data::eq(self, other)
    }
}

impl<A, B> PartialOrd<Box<B>> for Box<A>
where
    A: BoxPool,
    B: BoxPool,
    A::Data: PartialOrd<B::Data>,
{
    fn partial_cmp(&self, other: &Box<B>) -> Option<core::cmp::Ordering> {
        A::Data::partial_cmp(self, other)
    }
}

unsafe impl<P> Send for Box<P>
where
    P: BoxPool,
    P::Data: Send,
{
}

unsafe impl<P> Sync for Box<P>
where
    P: BoxPool,
    P::Data: Sync,
{
}

/// `box_pool!` implementation detail
// newtype to avoid having to make field types public
#[doc(hidden)]
pub struct BoxPoolImpl<T> {
    stack: Stack<UnionNode<MaybeUninit<T>>>,
}

impl<T> BoxPoolImpl<T> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    fn alloc(&self, value: T) -> Result<NonNullPtr<UnionNode<MaybeUninit<T>>>, T> {
        if let Some(node_ptr) = self.stack.try_pop() {
            unsafe { node_ptr.as_ptr().cast::<T>().write(value) }

            Ok(node_ptr)
        } else {
            Err(value)
        }
    }

    fn manage(&self, block: &'static mut BoxBlock<T>) {
        let node: &'static mut _ = &mut block.node;

        // SAFETY: The node within a `BoxBlock` is always properly initialized for linking because
        // the only way for         client code to construct a `BoxBlock` is through
        // `BoxBlock::new`. The `NonNullPtr` comes from a         reference, so it is
        // guaranteed to be dereferencable. It is also unique because the `BoxBlock` itself
        //         is passed as a `&mut`
        unsafe { self.stack.push(NonNullPtr::from_static_mut_ref(node)) }
    }
}

unsafe impl<T> Sync for BoxPoolImpl<T> {}

/// A chunk of memory that a `BoxPool` singleton can manage
pub struct BoxBlock<T> {
    node: UnionNode<MaybeUninit<T>>,
}

impl<T> BoxBlock<T> {
    /// Creates a new memory block
    pub const fn new() -> Self {
        Self {
            node: UnionNode::unlinked(),
        }
    }
}

impl<T> Default for BoxBlock<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::{ptr::addr_of_mut, thread};

    use super::*;

    #[test]
    fn cannot_alloc_if_empty() {
        box_pool!(MyBoxPool: i32);

        assert_eq!(Err(42), MyBoxPool.alloc(42));
    }

    #[test]
    fn can_alloc_if_pool_manages_one_block() {
        box_pool!(MyBoxPool: i32);

        let block = unsafe {
            static mut BLOCK: BoxBlock<i32> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyBoxPool.manage(block);

        assert_eq!(42, *MyBoxPool.alloc(42).unwrap());
    }

    #[test]
    fn alloc_drop_alloc() {
        box_pool!(MyBoxPool: i32);

        let block = unsafe {
            static mut BLOCK: BoxBlock<i32> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyBoxPool.manage(block);

        let boxed = MyBoxPool.alloc(1).unwrap();

        drop(boxed);

        assert_eq!(2, *MyBoxPool.alloc(2).unwrap());
    }

    #[test]
    fn runs_destructor_exactly_once_on_drop() {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        pub struct MyStruct;

        impl Drop for MyStruct {
            fn drop(&mut self) {
                COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }

        box_pool!(MyBoxPool: MyStruct);

        let block = unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyBoxPool.manage(block);

        let boxed = MyBoxPool.alloc(MyStruct).ok().unwrap();

        assert_eq!(0, COUNT.load(Ordering::Relaxed));

        drop(boxed);

        assert_eq!(1, COUNT.load(Ordering::Relaxed));
    }

    #[test]
    fn zst_is_well_aligned() {
        #[repr(align(4096))]
        pub struct Zst4096;

        box_pool!(MyBoxPool: Zst4096);

        let block = unsafe {
            static mut BLOCK: BoxBlock<Zst4096> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyBoxPool.manage(block);

        let boxed = MyBoxPool.alloc(Zst4096).ok().unwrap();

        let raw = &*boxed as *const Zst4096;
        assert_eq!(0, raw as usize % 4096);
    }

    #[test]
    fn can_clone_if_pool_is_not_exhausted() {
        static STRUCT_CLONE_WAS_CALLED: AtomicBool = AtomicBool::new(false);

        pub struct MyStruct;

        impl Clone for MyStruct {
            fn clone(&self) -> Self {
                STRUCT_CLONE_WAS_CALLED.store(true, Ordering::Relaxed);
                Self
            }
        }

        box_pool!(MyBoxPool: MyStruct);

        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });
        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });

        let first = MyBoxPool.alloc(MyStruct).ok().unwrap();
        let _second = first.clone();

        assert!(STRUCT_CLONE_WAS_CALLED.load(Ordering::Relaxed));

        let is_oom = MyBoxPool.alloc(MyStruct).is_err();
        assert!(is_oom);
    }

    #[test]
    fn clone_panics_if_pool_exhausted() {
        static STRUCT_CLONE_WAS_CALLED: AtomicBool = AtomicBool::new(false);

        pub struct MyStruct;

        impl Clone for MyStruct {
            fn clone(&self) -> Self {
                STRUCT_CLONE_WAS_CALLED.store(true, Ordering::Relaxed);
                Self
            }
        }

        box_pool!(MyBoxPool: MyStruct);

        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });

        let first = MyBoxPool.alloc(MyStruct).ok().unwrap();

        let thread = thread::spawn(move || {
            let _second = first.clone();
        });

        let thread_panicked = thread.join().is_err();
        assert!(thread_panicked);

        // we diverge from `alloc::Box<T>` in that we call `T::clone` first and then request
        // memory from the allocator whereas `alloc::Box<T>` does it the other way around
        // assert!(!STRUCT_CLONE_WAS_CALLED.load(Ordering::Relaxed));
    }

    #[test]
    fn panicking_clone_does_not_leak_memory() {
        static STRUCT_CLONE_WAS_CALLED: AtomicBool = AtomicBool::new(false);

        pub struct MyStruct;

        impl Clone for MyStruct {
            fn clone(&self) -> Self {
                STRUCT_CLONE_WAS_CALLED.store(true, Ordering::Relaxed);
                panic!()
            }
        }

        box_pool!(MyBoxPool: MyStruct);

        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });
        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });

        let boxed = MyBoxPool.alloc(MyStruct).ok().unwrap();

        let thread = thread::spawn(move || {
            let _boxed = boxed.clone();
        });

        let thread_panicked = thread.join().is_err();
        assert!(thread_panicked);

        assert!(STRUCT_CLONE_WAS_CALLED.load(Ordering::Relaxed));

        let once = MyBoxPool.alloc(MyStruct);
        let twice = MyBoxPool.alloc(MyStruct);

        assert!(once.is_ok());
        assert!(twice.is_ok());
    }

    #[test]
    fn into_raw_from_raw() {
        pub struct MyStruct {
            value: [u8; 64],
        }

        static NUM_DROP_CALLS: AtomicUsize = AtomicUsize::new(0);
        impl Drop for MyStruct {
            fn drop(&mut self) {
                NUM_DROP_CALLS.fetch_add(1, Ordering::AcqRel);
            }
        }

        box_pool!(MyBoxPool: MyStruct);

        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });

        MyBoxPool.manage(unsafe {
            static mut BLOCK: BoxBlock<MyStruct> = BoxBlock::new();
            addr_of_mut!(BLOCK).as_mut().unwrap()
        });

        let raw = {
            let boxed = MyBoxPool
                .alloc(MyStruct { value: [0xA5; 64] })
                .ok()
                .unwrap();
            Box::into_raw(boxed)
        };
        assert_eq!(0, NUM_DROP_CALLS.load(Ordering::Acquire));
        let addr_1 = raw as usize;

        {
            let boxed_again: Box<MyBoxPool> = unsafe { Box::from_raw(raw) };
            let addr_2 = boxed_again.node_ptr.as_ptr() as usize;
            assert_eq!([0xA5; 64], boxed_again.value);
            assert_eq!(addr_1, addr_2);
        }
        assert_eq!(1, NUM_DROP_CALLS.load(Ordering::Acquire));

        // Allocate again to ensure memory was returned to the pool
        let boxed_2 = MyBoxPool
            .alloc(MyStruct { value: [0xEF; 64] })
            .ok()
            .unwrap();
        let addr_2 = boxed_2.node_ptr.as_ptr() as usize;
        assert_eq!([0xEF; 64], boxed_2.value);
        assert_eq!(addr_1, addr_2);
    }
}
