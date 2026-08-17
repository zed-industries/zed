//! Object pool API
//!
//! # Example usage
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{object_pool, pool::object::{Object, ObjectBlock}};
//!
//! object_pool!(MyObjectPool: [u8; 128]);
//!
//! // cannot request objects without first giving object blocks to the pool
//! assert!(MyObjectPool.request().is_none());
//!
//! // (some `no_std` runtimes have safe APIs to create `&'static mut` references)
//! let block: &'static mut ObjectBlock<[u8; 128]> = unsafe {
//!     // unlike the memory pool APIs, an initial value must be specified here
//!     static mut BLOCK: ObjectBlock<[u8; 128]>= ObjectBlock::new([0; 128]);
//!     addr_of_mut!(BLOCK).as_mut().unwrap()
//! };
//!
//! // give object block to the pool
//! MyObjectPool.manage(block);
//!
//! // it's now possible to request objects
//! // unlike the memory pool APIs, no initial value is required here
//! let mut object = MyObjectPool.request().unwrap();
//!
//! // mutation is possible
//! object.iter_mut().for_each(|byte| *byte = byte.wrapping_add(1));
//!
//! // the number of live objects is limited to the number of blocks managed by the pool
//! let res = MyObjectPool.request();
//! assert!(res.is_none());
//!
//! // `object`'s destructor returns the object to the pool
//! drop(object);
//!
//! // it's possible to request an `Object` again
//! let res = MyObjectPool.request();
//!
//! assert!(res.is_some());
//! ```
//!
//! # Array block initialization
//!
//! You can create a static variable that contains an array of memory blocks and give all the blocks
//! to the `ObjectPool`. This requires an intermediate `const` value as shown below:
//!
//! ```
//! use core::ptr::addr_of_mut;
//! use heapless::{object_pool, pool::object::ObjectBlock};
//!
//! object_pool!(MyObjectPool: [u8; 128]);
//!
//! const POOL_CAPACITY: usize = 8;
//!
//! let blocks: &'static mut [ObjectBlock<[u8; 128]>] = {
//!     const BLOCK: ObjectBlock<[u8; 128]> = ObjectBlock::new([0; 128]); // <=
//!     static mut BLOCKS: [ObjectBlock<[u8; 128]>; POOL_CAPACITY] = [BLOCK; POOL_CAPACITY];
//!     unsafe { addr_of_mut!(BLOCKS).as_mut().unwrap() }
//! };
//!
//! for block in blocks {
//!     MyObjectPool.manage(block);
//! }
//! ```

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    mem::ManuallyDrop,
    ops, ptr,
};

use stable_deref_trait::StableDeref;

use super::treiber::{AtomicPtr, NonNullPtr, Stack, StructNode};

/// Creates a new `ObjectPool` singleton with the given `$name` that manages the specified
/// `$data_type`
///
/// For more extensive documentation see the [module level documentation](crate::pool::object)
#[macro_export]
macro_rules! object_pool {
    ($name:ident: $data_type:ty) => {
        pub struct $name;

        impl $crate::pool::object::ObjectPool for $name {
            type Data = $data_type;

            fn singleton() -> &'static $crate::pool::object::ObjectPoolImpl<$data_type> {
                // Even though the static variable is not exposed to user code, it is
                // still useful to have a descriptive symbol name for debugging.
                #[allow(non_upper_case_globals)]
                static $name: $crate::pool::object::ObjectPoolImpl<$data_type> =
                    $crate::pool::object::ObjectPoolImpl::new();

                &$name
            }
        }

        impl $name {
            /// Inherent method version of `ObjectPool::request`
            #[allow(dead_code)]
            pub fn request(&self) -> Option<$crate::pool::object::Object<$name>> {
                <$name as $crate::pool::object::ObjectPool>::request()
            }

            /// Inherent method version of `ObjectPool::manage`
            #[allow(dead_code)]
            pub fn manage(
                &self,
                block: &'static mut $crate::pool::object::ObjectBlock<$data_type>,
            ) {
                <$name as $crate::pool::object::ObjectPool>::manage(block)
            }
        }
    };
}

/// A singleton that manages `pool::object::Object`s
pub trait ObjectPool: Sized {
    /// The data type of the objects managed by the object pool
    type Data: 'static;

    /// `object_pool!` implementation detail
    #[doc(hidden)]
    fn singleton() -> &'static ObjectPoolImpl<Self::Data>;

    /// Request a new object from the pool
    fn request() -> Option<Object<Self>> {
        Self::singleton()
            .request()
            .map(|node_ptr| Object { node_ptr })
    }

    /// Adds a statically allocate object to the pool
    fn manage(block: &'static mut ObjectBlock<Self::Data>) {
        Self::singleton().manage(block);
    }
}

/// `object_pool!` implementation detail
#[doc(hidden)]
pub struct ObjectPoolImpl<T> {
    stack: Stack<StructNode<T>>,
}

impl<T> ObjectPoolImpl<T> {
    /// `object_pool!` implementation detail
    #[doc(hidden)]
    pub const fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    fn request(&self) -> Option<NonNullPtr<StructNode<T>>> {
        self.stack.try_pop()
    }

    fn manage(&self, block: &'static mut ObjectBlock<T>) {
        let node: &'static mut _ = &mut block.node;

        unsafe { self.stack.push(NonNullPtr::from_static_mut_ref(node)) }
    }
}

// `T needs` to be Send because returning an object from a thread and then
// requesting it from another is effectively a cross-thread 'send' operation
unsafe impl<T> Sync for ObjectPoolImpl<T> where T: Send {}

/// An object managed by object pool `P`
pub struct Object<P>
where
    P: ObjectPool,
{
    node_ptr: NonNullPtr<StructNode<P::Data>>,
}

impl<A, T, const N: usize> AsMut<[T]> for Object<A>
where
    A: ObjectPool<Data = [T; N]>,
{
    fn as_mut(&mut self) -> &mut [T] {
        &mut **self
    }
}

impl<A, T, const N: usize> AsRef<[T]> for Object<A>
where
    A: ObjectPool<Data = [T; N]>,
{
    fn as_ref(&self) -> &[T] {
        &**self
    }
}

impl<A> fmt::Debug for Object<A>
where
    A: ObjectPool,
    A::Data: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<A> ops::Deref for Object<A>
where
    A: ObjectPool,
{
    type Target = A::Data;

    fn deref(&self) -> &Self::Target {
        unsafe { &*ptr::addr_of!((*self.node_ptr.as_ptr()).data) }
    }
}

impl<A> ops::DerefMut for Object<A>
where
    A: ObjectPool,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *ptr::addr_of_mut!((*self.node_ptr.as_ptr()).data) }
    }
}

unsafe impl<A> StableDeref for Object<A> where A: ObjectPool {}

impl<A> fmt::Display for Object<A>
where
    A: ObjectPool,
    A::Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        A::Data::fmt(self, f)
    }
}

impl<P> Drop for Object<P>
where
    P: ObjectPool,
{
    fn drop(&mut self) {
        unsafe { P::singleton().stack.push(self.node_ptr) }
    }
}

impl<A> Eq for Object<A>
where
    A: ObjectPool,
    A::Data: Eq,
{
}

impl<A> Hash for Object<A>
where
    A: ObjectPool,
    A::Data: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(state);
    }
}

impl<A> Ord for Object<A>
where
    A: ObjectPool,
    A::Data: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        A::Data::cmp(self, other)
    }
}

impl<A, B> PartialEq<Object<B>> for Object<A>
where
    A: ObjectPool,
    B: ObjectPool,
    A::Data: PartialEq<B::Data>,
{
    fn eq(&self, other: &Object<B>) -> bool {
        A::Data::eq(self, other)
    }
}

impl<A, B> PartialOrd<Object<B>> for Object<A>
where
    A: ObjectPool,
    B: ObjectPool,
    A::Data: PartialOrd<B::Data>,
{
    fn partial_cmp(&self, other: &Object<B>) -> Option<Ordering> {
        A::Data::partial_cmp(self, other)
    }
}

unsafe impl<P> Send for Object<P>
where
    P: ObjectPool,
    P::Data: Send,
{
}

unsafe impl<P> Sync for Object<P>
where
    P: ObjectPool,
    P::Data: Sync,
{
}

/// An object "block" of data type `T` that has not yet been associated to an `ObjectPool`
pub struct ObjectBlock<T> {
    node: StructNode<T>,
}

impl<T> ObjectBlock<T> {
    /// Creates a new object block with the given `initial_value`
    pub const fn new(initial_value: T) -> Self {
        Self {
            node: StructNode {
                next: ManuallyDrop::new(AtomicPtr::null()),
                data: ManuallyDrop::new(initial_value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{self, AtomicUsize};
    use std::ptr::addr_of_mut;

    use super::*;

    #[test]
    fn cannot_request_if_empty() {
        object_pool!(MyObjectPool: i32);

        assert_eq!(None, MyObjectPool.request());
    }

    #[test]
    fn can_request_if_manages_one_block() {
        object_pool!(MyObjectPool: i32);

        let block = unsafe {
            static mut BLOCK: ObjectBlock<i32> = ObjectBlock::new(1);
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyObjectPool.manage(block);

        assert_eq!(1, *MyObjectPool.request().unwrap());
    }

    #[test]
    fn request_drop_request() {
        object_pool!(MyObjectPool: i32);

        let block = unsafe {
            static mut BLOCK: ObjectBlock<i32> = ObjectBlock::new(1);
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyObjectPool.manage(block);

        let mut object = MyObjectPool.request().unwrap();

        *object = 2;
        drop(object);

        assert_eq!(2, *MyObjectPool.request().unwrap());
    }

    #[test]
    fn destructor_does_not_run_on_drop() {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        pub struct MyStruct;

        impl Drop for MyStruct {
            fn drop(&mut self) {
                COUNT.fetch_add(1, atomic::Ordering::Relaxed);
            }
        }

        object_pool!(MyObjectPool: MyStruct);

        let block = unsafe {
            static mut BLOCK: ObjectBlock<MyStruct> = ObjectBlock::new(MyStruct);
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyObjectPool.manage(block);

        let object = MyObjectPool.request().unwrap();

        assert_eq!(0, COUNT.load(atomic::Ordering::Relaxed));

        drop(object);

        assert_eq!(0, COUNT.load(atomic::Ordering::Relaxed));
    }

    #[test]
    fn zst_is_well_aligned() {
        #[repr(align(4096))]
        pub struct Zst4096;

        object_pool!(MyObjectPool: Zst4096);

        let block = unsafe {
            static mut BLOCK: ObjectBlock<Zst4096> = ObjectBlock::new(Zst4096);
            addr_of_mut!(BLOCK).as_mut().unwrap()
        };
        MyObjectPool.manage(block);

        let object = MyObjectPool.request().unwrap();

        let raw = &*object as *const Zst4096;
        assert_eq!(0, raw as usize % 4096);
    }
}
