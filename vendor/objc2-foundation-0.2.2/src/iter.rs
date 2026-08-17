#![allow(dead_code)]
use core::ptr::{self, NonNull};
#[cfg(debug_assertions)]
use std::os::raw::c_ulong;

use objc2::mutability::IsIdCloneable;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{mutability::IsMutable, runtime::ProtocolObject};
use objc2::{ClassType, Message};

use super::util;
use crate::Foundation::{NSFastEnumeration, NSFastEnumerationState};

/// Swift and Objective-C both have a stack buffer size of 16, so we do that
/// as well.
///
/// TODO: Consider lowering this a bit, since the most common type of
/// enumeration (e.g. NSArray) doesn't use the buffer:
/// [CFArray's NSFastEnumeration implementation](https://github.com/apple-oss-distributions/CF/blob/dc54c6bb1c1e5e0b9486c1d26dd5bef110b20bf3/CFArray.c#L618-L642)
const BUF_SIZE: usize = 16;

/// Helper type for doing fast enumeration.
///
/// See the following other implementations of this:
/// - [Swift](https://github.com/apple/swift-corelibs-foundation/blob/2d23cf3dc07951ed2b988608d08d7a54cc53b26e/Darwin/Foundation-swiftoverlay/NSFastEnumeration.swift#L23)
/// - [Clang](https://github.com/llvm/llvm-project/blob/28d85d207fc37b5593c17a25f687c91b7afda5b4/clang/lib/Frontend/Rewrite/RewriteModernObjC.cpp#L1653-L1850)
#[derive(Debug, PartialEq)]
struct FastEnumeratorHelper {
    state: NSFastEnumerationState,
    buf: [*mut AnyObject; BUF_SIZE],
    // TODO: We could possibly optimize things a bit by doing
    // `itemsPtr.add(1); items_count -= 1;` on every loop, instead of storing
    // `current_item` - but it's not really defined whether we're allowed to
    // mutate `itemsPtr`? All of the GNUStep code and the open sourced Apple
    // code I've found would work under this scheme, and from a cursory GitHub
    // search I only found the following that assume that `itemsPtr` is stable
    // across invocations:
    // https://github.com/iodefog/VipVideo/blob/52243b893f0004a6880f13c33104e2394b56b7f1/VipVideo/Helper/JSONKit.m#L759-L769
    //
    // Though they also assume that the given stack buffer has a stable
    // address, which is something that would be prohibitively expensive to
    // ensure, so don't think we should consider that one.
    current_item: usize,
    items_count: usize,
    /// Track mutation mistakes when debug assertions are enabled (they should
    /// be made impossible by Rust at compile-time, so hence we don't need to
    /// track them in release mode).
    ///
    /// This is set to `None` initially, but later loaded to `Some(_)` after
    /// the first enumeration.
    #[cfg(debug_assertions)]
    mutations_state: Option<c_ulong>,
}

// SAFETY: Neither `FastEnumeratorHelper`, nor inner the enumeration state,
// need to be bound to any specific thread (at least, we can generally assume
// this for the enumerable types in Foundation - but `NSEnumerator` won't be
// `Send`, and neither will its iterator types).
unsafe impl Send for FastEnumeratorHelper {}

// SAFETY: `FastEnumeratorHelper` is only mutable behind `&mut`, and as such
// is safe to share with other threads.
unsafe impl Sync for FastEnumeratorHelper {}

impl FastEnumeratorHelper {
    #[inline]
    fn new() -> Self {
        Self {
            state: NSFastEnumerationState {
                state: 0,
                itemsPtr: ptr::null_mut(),
                mutationsPtr: ptr::null_mut(),
                extra: [0; 5],
            },
            buf: [ptr::null_mut(); BUF_SIZE],
            current_item: 0,
            items_count: 0,
            #[cfg(debug_assertions)]
            mutations_state: None,
        }
    }

    #[inline]
    const fn remaining_items_at_least(&self) -> usize {
        self.items_count - self.current_item
    }

    /// Load the next array of items into the enumeration state.
    ///
    ///
    /// # Safety
    ///
    /// The collection must be the same on each call.
    #[inline]
    #[track_caller]
    unsafe fn load_next_items(&mut self, collection: &ProtocolObject<dyn NSFastEnumeration>) {
        // SAFETY: The ptr comes from a slice, which is always non-null.
        //
        // The callee may choose to use this or not, see:
        // <https://www.mikeash.com/pyblog/friday-qa-2010-04-16-implementing-fast-enumeration.html>
        let buf_ptr = unsafe { NonNull::new_unchecked(self.buf.as_mut_ptr()) };
        // SAFETY:
        // - The state is mutable, and we don't touch the `state` and
        //   `extra` fields.
        // - The buffer is mutable and its length is correctly set.
        // - The collection and state are guaranteed by the caller to match.
        self.items_count = unsafe {
            collection.countByEnumeratingWithState_objects_count(
                NonNull::from(&mut self.state),
                buf_ptr,
                self.buf.len(),
            )
        };

        #[cfg(debug_assertions)]
        {
            if self.items_count > 0 && self.state.itemsPtr.is_null() {
                panic!("`itemsPtr` was NULL");
            }
        }

        // For unwind safety, we only set this _after_ the message send has
        // completed, otherwise, if it unwinded, upon iterating again we might
        // invalidly assume that the buffer was ready.
        self.current_item = 0;
    }

    /// Get the next item from the given collection.
    ///
    /// We use a `ProtocolObject` instead of a generic, so that there is only
    /// one instance of this function in the compilation unit (should improve
    /// compilation speed).
    ///
    ///
    /// # Safety
    ///
    /// The collection must be the same on each call.
    #[inline]
    #[track_caller]
    unsafe fn next_from(
        &mut self,
        collection: &ProtocolObject<dyn NSFastEnumeration>,
    ) -> Option<NonNull<AnyObject>> {
        // If we've exhausted the current array of items.
        if self.current_item >= self.items_count {
            // Get the next array of items.
            //
            // SAFETY: Upheld by caller.
            unsafe { self.load_next_items(collection) };

            // If the next array was also empty.
            if self.items_count == 0 {
                // We are done enumerating.
                return None;
            }
        }

        #[cfg(debug_assertions)]
        {
            // If the mutation ptr is not set, we do nothing.
            if let Some(ptr) = NonNull::new(self.state.mutationsPtr) {
                // SAFETY:
                // - The pointer is not NULL.
                //
                // - The enumerator is expected to give back a dereferenceable
                //   pointer, that is alive for as long as the collection is
                //   alive.
                //
                //   Note that iterating past the first returned `None` is not
                //   tested by most Objective-C implementations, so it may
                //   deallocate the mutations ptr in that case?
                //
                // - The enumeration should not be modifiable across threads,
                //   so neither will this pointer be accessed from different
                //   threads.
                //
                //   Note that this assumption is relatively likely to be
                //   violated, but if that is the case, the program already
                //   has UB, so then it is better that we detect it.
                //
                // - The value is an integer, so is always initialized.
                //
                //
                // We do an unaligned read here since we have no guarantees
                // about this pointer, and efficiency doesn't really matter.
                let new_state = unsafe { ptr.as_ptr().read_unaligned() };
                match self.mutations_state {
                    // On the first iteration, initialize the mutation state
                    None => {
                        self.mutations_state = Some(new_state);
                    }
                    // On subsequent iterations, verify that the state hasn't
                    // changed.
                    Some(current_state) => {
                        if current_state != new_state {
                            panic!("mutation detected during enumeration. This is undefined behaviour, and must be avoided");
                        }
                    }
                }
            }
        }

        // Compute a pointer to the current item.
        //
        // SAFETY: The index is checked above to be in bounds of the returned
        // array.
        let ptr = unsafe { self.state.itemsPtr.add(self.current_item) };
        // Move to the next item.
        self.current_item += 1;
        // And read the current item.
        //
        // SAFETY:
        // - The pointer is not NULL, it is guaranteed to have been set
        //   by `countByEnumeratingWithState:objects:count:`.
        // - The pointer is within the bounds of the returned array.
        //
        // Note: GNUStep may sometimes return unaligned pointers. TODO: Should
        // we go back to using `read` on Apple systems for better performance?
        let obj = unsafe { ptr.read_unaligned() };
        // SAFETY: The returned array contains no NULL pointers.
        Some(unsafe { NonNull::new_unchecked(obj) })
    }
}

// Unfortunately, `NSFastEnumeration` doesn't provide a way for enumerated
// objects to clean up their work after having being enumerated over.
//
// impl Drop for FastEnumeratorHelper { ... }

/// Internal helper trait to figure out the output type of something that
/// implements `NSFastEnumeration`.
///
///
/// # Safety
///
/// The associated `Item` type must be the type that is used in Objective-C's
/// `for (type elem in collection) { stmts; }` enumeration, and the collection
/// itself must not contain any lifetime parameter.
pub(crate) unsafe trait FastEnumerationHelper: Message + NSFastEnumeration {
    type Item: Message;

    fn maybe_len(&self) -> Option<usize>;
}

// Iterator implementations we _can't_ do:
//
//
// # `ExactSizeIterator`
//
// This could probably be implemented correctly for the concrete class
// `NSArray`, but not for arbitary `NSArray` subclasses, which are always be
// valid to convert to `NSArray`.
//
//
// # `FusedIterator`
//
// Although most fast enumerators are probably fused, this is not guaranteed
// by the documentation (actually, some may even use the last iteration as a
// cleanup step, and do it twice, which we should perhaps protect against?).
//
//
// # `DoubleEndedIterator`
//
// `NSArray` has `reverseObjectEnumerator`, but this is a separate object from
// `objectEnumerator`, and as such it is not possible to enumerate it in the
// backwards direction.

// Explicit lifetime bound on `C` to future-proof against possible soundness
// mistakes in the future.
//
// TODO: Consider adding `#[repr(C)]`, to allow LLVM to perform better
// zero-initialization.
#[derive(Debug, PartialEq)]
pub(crate) struct Iter<'a, C: ?Sized + 'a> {
    helper: FastEnumeratorHelper,
    /// 'a and C are covariant.
    collection: &'a C,
}

impl<'a, C: ?Sized + FastEnumerationHelper> Iter<'a, C> {
    pub(crate) fn new(collection: &'a C) -> Self {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
        }
    }
}

impl<'a, C: FastEnumerationHelper> Iterator for Iter<'a, C> {
    type Item = &'a C::Item;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<&'a C::Item> {
        // SAFETY: The collection is the same on each iteration.
        let obj = unsafe {
            self.helper
                .next_from(ProtocolObject::from_ref(self.collection))?
        };
        // SAFETY: The lifetime is bound to the collection, and the type is
        // correct.
        //
        // [Enumeration variables are externally retained][enum-retained], so
        // it is okay to return a reference instead of retaining here.
        //
        // [enum-retained]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#fast-enumeration-iteration-variables
        Some(unsafe { obj.cast::<C::Item>().as_ref() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            self.collection.maybe_len(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IterMut<'a, C: ?Sized + 'a> {
    helper: FastEnumeratorHelper,
    /// 'a and C are covariant.
    collection: &'a mut C,
}

impl<'a, C: ?Sized + FastEnumerationHelper> IterMut<'a, C>
where
    C::Item: IsMutable,
{
    pub(crate) fn new(collection: &'a mut C) -> Self {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
        }
    }
}

impl<'a, C: FastEnumerationHelper> Iterator for IterMut<'a, C>
where
    C::Item: IsMutable,
{
    type Item = &'a mut C::Item;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<&'a mut C::Item> {
        // SAFETY: The collection is the same on each iteration.
        //
        // Passing the collection as immutable here is safe since it isn't
        // mutated itself, and it is `UnsafeCell` anyhow.
        let obj = unsafe {
            self.helper
                .next_from(ProtocolObject::from_mut(self.collection))?
        };
        // SAFETY: That we take the collection by `&mut`, along with the
        // `C::Item: IsMutable` bound, ensure that the items are safe to
        // return as mutable.
        //
        // Rest is same as `Iter<'a, C>`.
        Some(unsafe { obj.cast::<C::Item>().as_mut() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            self.collection.maybe_len(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IterRetained<'a, C: ?Sized + 'a> {
    inner: Iter<'a, C>,
}

impl<'a, C: ?Sized + FastEnumerationHelper> IterRetained<'a, C>
where
    C::Item: IsIdCloneable,
{
    pub(crate) fn new(collection: &'a C) -> Self {
        Self {
            inner: Iter::new(collection),
        }
    }
}

impl<'a, C: FastEnumerationHelper> Iterator for IterRetained<'a, C>
where
    C::Item: IsIdCloneable,
{
    type Item = Retained<C::Item>;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<Retained<C::Item>> {
        let obj = self.inner.next()?;
        // SAFETY: The object is stored inside the enumerated collection.
        Some(unsafe { util::collection_retain(obj) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// Note: I first considered having `IntoIter<T>` instead of `IntoIter<C>`, but
// that would be unsound for `NSDictionary<K, V>`, since if the `V` has a
// lifetime, that lifetime would have been erased (so we'd have to add an
// `V: 'static` bound, which is easy to forget).
//
// Also, this design allows us to have proper `Send + Sync` implementations,
// as well as allowing us to implement `FusedIterator` and `ExactSizeIterator`
// when possible.
#[derive(Debug, PartialEq)]
pub(crate) struct IntoIter<C: ?Sized> {
    helper: FastEnumeratorHelper,
    /// C is covariant.
    collection: Retained<C>,
}

impl<C: FastEnumerationHelper> IntoIter<C> {
    pub(crate) fn new_immutable(collection: Retained<C>) -> Self
    where
        C: IsIdCloneable,
        C::Item: IsIdCloneable,
    {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
        }
    }

    pub(crate) fn new_mutable<T: ClassType<Super = C> + IsMutable>(collection: Retained<T>) -> Self
    where
        C: IsIdCloneable,
    {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection: unsafe { Retained::cast(collection) },
        }
    }

    pub(crate) fn new(collection: Retained<C>) -> Self
    where
        C: IsMutable,
    {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
        }
    }
}

impl<C: FastEnumerationHelper> Iterator for IntoIter<C> {
    type Item = Retained<C::Item>;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<Retained<C::Item>> {
        let collection = ProtocolObject::from_ref(&*self.collection);
        // SAFETY: The collection is the same on each iteration.
        let obj = unsafe { self.helper.next_from(collection)? };
        // SAFETY: TODO
        let obj = unsafe { Retained::retain(obj.cast::<C::Item>().as_ptr()) };
        // SAFETY: The object was `NonNull`, and `retain` returns the same
        // pointer.
        Some(unsafe { obj.unwrap_unchecked() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            self.collection.maybe_len(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IterWithBackingEnum<'a, C: ?Sized + 'a, E: ?Sized + 'a> {
    helper: FastEnumeratorHelper,
    /// 'a and C are covariant.
    collection: &'a C,
    /// E is covariant.
    enumerator: Retained<E>,
}

impl<'a, C, E> IterWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: ?Sized + FastEnumerationHelper,
{
    pub(crate) unsafe fn new(collection: &'a C, enumerator: Retained<E>) -> Self {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
            enumerator,
        }
    }
}

impl<'a, C, E> Iterator for IterWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: FastEnumerationHelper,
{
    type Item = &'a E::Item;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<&'a E::Item> {
        // SAFETY: The enumerator is the same on each iteration.
        let obj = unsafe {
            self.helper
                .next_from(ProtocolObject::from_ref(&*self.enumerator))?
        };
        // SAFETY: TODO
        Some(unsafe { obj.cast::<E::Item>().as_ref() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            // Assume that the length of the collection matches the length of
            // the enumerator.
            self.collection.maybe_len(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IterMutWithBackingEnum<'a, C: ?Sized + 'a, E: ?Sized + 'a> {
    helper: FastEnumeratorHelper,
    /// 'a and C are covariant.
    collection: &'a mut C,
    /// E is covariant.
    enumerator: Retained<E>,
}

impl<'a, C, E> IterMutWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: ?Sized + FastEnumerationHelper + IsMutable,
    E::Item: IsMutable,
{
    pub(crate) unsafe fn new(collection: &'a mut C, enumerator: Retained<E>) -> Self {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
            enumerator,
        }
    }
}

impl<'a, C, E> Iterator for IterMutWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: FastEnumerationHelper + IsMutable,
    E::Item: IsMutable,
{
    type Item = &'a mut E::Item;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<&'a mut E::Item> {
        // SAFETY: The enumerator is the same on each iteration.
        let obj = unsafe {
            self.helper
                .next_from(ProtocolObject::from_mut(&mut *self.enumerator))?
        };
        // SAFETY: TODO
        Some(unsafe { obj.cast::<E::Item>().as_mut() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            // Assume that the length of the collection matches the length of
            // the enumerator.
            self.collection.maybe_len(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IterRetainedWithBackingEnum<'a, C: ?Sized + 'a, E: ?Sized + 'a> {
    inner: IterWithBackingEnum<'a, C, E>,
}

impl<'a, C, E> IterRetainedWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: ?Sized + FastEnumerationHelper,
    E::Item: IsIdCloneable,
{
    pub(crate) unsafe fn new(collection: &'a C, enumerator: Retained<E>) -> Self {
        // SAFETY: Upheld by caller.
        let inner = unsafe { IterWithBackingEnum::new(collection, enumerator) };
        Self { inner }
    }
}

impl<'a, C, E> Iterator for IterRetainedWithBackingEnum<'a, C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: FastEnumerationHelper,
    E::Item: IsIdCloneable,
{
    type Item = Retained<E::Item>;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<Retained<E::Item>> {
        let obj = self.inner.next()?;
        // SAFETY: The object is stored inside the enumerated collection.
        Some(unsafe { util::collection_retain(obj) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IntoIterWithBackingEnum<C: ?Sized, E: ?Sized> {
    helper: FastEnumeratorHelper,
    /// C is covariant.
    collection: Retained<C>,
    /// E is covariant.
    enumerator: Retained<E>,
}

impl<C, E> IntoIterWithBackingEnum<C, E>
where
    C: FastEnumerationHelper,
    E: ?Sized + FastEnumerationHelper,
{
    pub(crate) unsafe fn new_immutable(collection: Retained<C>, enumerator: Retained<E>) -> Self
    where
        C: IsIdCloneable,
        E::Item: IsIdCloneable,
    {
        Self {
            helper: FastEnumeratorHelper::new(),
            collection,
            enumerator,
        }
    }

    pub(crate) unsafe fn new_mutable<T: ClassType<Super = C> + IsMutable>(
        collection: Retained<T>,
        enumerator: Retained<E>,
    ) -> Self
    where
        C: IsIdCloneable,
    {
        Self {
            collection: unsafe { Retained::cast(collection) },
            enumerator,
            helper: FastEnumeratorHelper::new(),
        }
    }
}

impl<C, E> Iterator for IntoIterWithBackingEnum<C, E>
where
    C: ?Sized + FastEnumerationHelper,
    E: FastEnumerationHelper,
{
    type Item = Retained<E::Item>;

    #[inline]
    #[track_caller]
    fn next(&mut self) -> Option<Retained<E::Item>> {
        let enumerator = ProtocolObject::from_ref(&*self.enumerator);
        // SAFETY: The enumerator is the same on each iteration.
        let obj = unsafe { self.helper.next_from(enumerator)? };
        // SAFETY: TODO
        let obj = unsafe { Retained::retain(obj.cast::<E::Item>().as_ptr()) };
        // SAFETY: TODO
        Some(unsafe { obj.unwrap_unchecked() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.helper.remaining_items_at_least(),
            // Assume that the length of the collection matches the length of
            // the enumerator.
            self.collection.maybe_len(),
        )
    }
}

#[doc(hidden)]
macro_rules! __impl_iter {
    (
        impl<$($lifetime:lifetime, )? $t1:ident: $bound1:ident $(+ $bound1_b:ident)? $(, $t2:ident: $bound2:ident $(+ $bound2_b:ident)?)?> Iterator<Item = $item:ty> for $for:ty { ... }
    ) => {
        impl<$($lifetime, )? $t1: $bound1 $(+ $bound1_b)? $(, $t2: $bound2 $(+ $bound2_b)?)?> Iterator for $for {
            type Item = $item;

            #[inline]
            #[track_caller]
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }
    }
}

#[doc(hidden)]
macro_rules! __impl_into_iter {
    () => {};
    (
        $(#[$m:meta])*
        impl<T: Message> IntoIterator for &$ty:ident<T> {
            type IntoIter = $iter:ident<'_, T>;
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl<'a, T: Message> IntoIterator for &'a $ty<T> {
            type Item = &'a T;
            type IntoIter = $iter<'a, T>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                $iter(super::iter::Iter::new(&self))
            }
        }

        __impl_into_iter! {
            $($rest)*
        }
    };
    (
        $(#[$m:meta])*
        impl<T: Message + IsMutable> IntoIterator for &mut $ty:ident<T> {
            type IntoIter = $iter_mut:ident<'_, T>;
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl<'a, T: Message + IsMutable> IntoIterator for &'a mut $ty<T> {
            type Item = &'a mut T;
            type IntoIter = $iter_mut<'a, T>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                $iter_mut(super::iter::IterMut::new(&mut *self))
            }
        }

        __impl_into_iter! {
            $($rest)*
        }
    };
    (
        $(#[$m:meta])*
        impl<T: Message + IsIdCloneable> IntoIterator for Retained<$ty:ident<T>> {
            type IntoIter = $into_iter:ident<T>;
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl<T: Message + IsIdCloneable> objc2::rc::RetainedIntoIterator for $ty<T> {
            type Item = Retained<T>;
            type IntoIter = $into_iter<T>;

            #[inline]
            fn id_into_iter(this: Retained<Self>) -> Self::IntoIter {
                $into_iter(super::iter::IntoIter::new_immutable(this))
            }
        }

        __impl_into_iter! {
            $($rest)*
        }
    };
    (
        $(#[$m:meta])*
        impl<T: Message> IntoIterator for Retained<$ty:ident<T>> {
            type IntoIter = $into_iter:ident<T>;
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl<T: Message> objc2::rc::RetainedIntoIterator for $ty<T> {
            type Item = Retained<T>;
            type IntoIter = $into_iter<T>;

            #[inline]
            fn id_into_iter(this: Retained<Self>) -> Self::IntoIter {
                $into_iter(super::iter::IntoIter::new_mutable(this))
            }
        }

        __impl_into_iter! {
            $($rest)*
        }
    };
}

#[cfg(test)]
#[cfg(feature = "NSArray")]
#[cfg(feature = "NSValue")]
mod tests {
    use super::*;
    use core::mem::size_of;

    use crate::Foundation::{NSArray, NSNumber};

    #[test]
    #[cfg_attr(
        any(not(target_pointer_width = "64"), debug_assertions),
        ignore = "assertions assume pointer-width of 64, and the size only really matter in release mode"
    )]
    fn test_enumerator_helper() {
        // We should attempt to reduce these if possible
        assert_eq!(size_of::<NSFastEnumerationState>(), 64);
        assert_eq!(size_of::<FastEnumeratorHelper>(), 208);
        assert_eq!(size_of::<Iter<'_, NSArray<NSNumber>>>(), 216);
    }

    #[test]
    fn test_enumerator() {
        let vec = (0..4).map(NSNumber::new_usize).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.iter();
        assert_eq!(enumerator.count(), 4);

        let enumerator = array.iter();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.as_usize() == i));
    }

    #[test]
    fn test_into_enumerator() {
        let vec = (0..4).map(NSNumber::new_usize).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.into_iter();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.as_usize() == i));
    }
}
