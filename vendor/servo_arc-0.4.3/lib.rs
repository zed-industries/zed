// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Fork of Arc for Servo. This has the following advantages over std::sync::Arc:
//!
//! * We don't waste storage on the weak reference count.
//! * We don't do extra RMU operations to handle the possibility of weak references.
//! * We can experiment with arena allocation (todo).
//! * We can add methods to support our custom use cases [1].
//! * We have support for dynamically-sized types (see from_header_and_iter).
//! * We have support for thin arcs to unsized types (see ThinArc).
//! * We have support for references to static data, which don't do any
//!   refcounting.
//!
//! [1]: https://bugzilla.mozilla.org/show_bug.cgi?id=1360883

// The semantics of `Arc` are already documented in the Rust docs, so we don't
// duplicate those here.
#![allow(missing_docs)]

#[cfg(feature = "servo")]
use serde::{Deserialize, Serialize};
use stable_deref_trait::{CloneStableDeref, StableDeref};
use std::alloc::{self, Layout};
use std::borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::{self, align_of, size_of};
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::process;
use std::ptr;
use std::sync::atomic;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{isize, usize};

/// A soft limit on the amount of references that may be made to an `Arc`.
///
/// Going above this limit will abort your program (although not
/// necessarily) at _exactly_ `MAX_REFCOUNT + 1` references.
const MAX_REFCOUNT: usize = (isize::MAX) as usize;

/// Special refcount value that means the data is not reference counted,
/// and that the `Arc` is really acting as a read-only static reference.
const STATIC_REFCOUNT: usize = usize::MAX;

/// An atomically reference counted shared pointer
///
/// See the documentation for [`Arc`] in the standard library. Unlike the
/// standard library `Arc`, this `Arc` does not support weak reference counting.
///
/// See the discussion in https://github.com/rust-lang/rust/pull/60594 for the
/// usage of PhantomData.
///
/// [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
///
/// cbindgen:derive-eq=false
/// cbindgen:derive-neq=false
#[repr(C)]
pub struct Arc<T: ?Sized> {
    p: ptr::NonNull<ArcInner<T>>,
    phantom: PhantomData<T>,
}

/// An `Arc` that is known to be uniquely owned
///
/// When `Arc`s are constructed, they are known to be
/// uniquely owned. In such a case it is safe to mutate
/// the contents of the `Arc`. Normally, one would just handle
/// this by mutating the data on the stack before allocating the
/// `Arc`, however it's possible the data is large or unsized
/// and you need to heap-allocate it earlier in such a way
/// that it can be freely converted into a regular `Arc` once you're
/// done.
///
/// `UniqueArc` exists for this purpose, when constructed it performs
/// the same allocations necessary for an `Arc`, however it allows mutable access.
/// Once the mutation is finished, you can call `.shareable()` and get a regular `Arc`
/// out of it.
///
/// Ignore the doctest below there's no way to skip building with refcount
/// logging during doc tests (see rust-lang/rust#45599).
///
/// ```rust,ignore
/// # use servo_arc::UniqueArc;
/// let data = [1, 2, 3, 4, 5];
/// let mut x = UniqueArc::new(data);
/// x[4] = 7; // mutate!
/// let y = x.shareable(); // y is an Arc<T>
/// ```
pub struct UniqueArc<T: ?Sized>(Arc<T>);

impl<T> UniqueArc<T> {
    #[inline]
    /// Construct a new UniqueArc
    pub fn new(data: T) -> Self {
        UniqueArc(Arc::new(data))
    }

    /// Construct an uninitialized arc
    #[inline]
    pub fn new_uninit() -> UniqueArc<mem::MaybeUninit<T>> {
        unsafe {
            let layout = Layout::new::<ArcInner<mem::MaybeUninit<T>>>();
            let ptr = alloc::alloc(layout);
            let mut p = ptr::NonNull::new(ptr)
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
                .cast::<ArcInner<mem::MaybeUninit<T>>>();
            ptr::write(&mut p.as_mut().count, atomic::AtomicUsize::new(1));
            #[cfg(feature = "track_alloc_size")]
            ptr::write(&mut p.as_mut().alloc_size, layout.size());

            #[cfg(feature = "gecko_refcount_logging")]
            {
                NS_LogCtor(p.as_ptr() as *mut _, b"ServoArc\0".as_ptr() as *const _, 8)
            }

            UniqueArc(Arc {
                p,
                phantom: PhantomData,
            })
        }
    }

    #[inline]
    /// Convert to a shareable Arc<T> once we're done mutating it
    pub fn shareable(self) -> Arc<T> {
        self.0
    }
}

impl<T> UniqueArc<mem::MaybeUninit<T>> {
    /// Convert to an initialized Arc.
    #[inline]
    pub unsafe fn assume_init(this: Self) -> UniqueArc<T> {
        UniqueArc(Arc {
            p: mem::ManuallyDrop::new(this).0.p.cast(),
            phantom: PhantomData,
        })
    }
}

impl<T> Deref for UniqueArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T> DerefMut for UniqueArc<T> {
    fn deref_mut(&mut self) -> &mut T {
        // We know this to be uniquely owned
        unsafe { &mut (*self.0.ptr()).data }
    }
}

unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}

/// The object allocated by an Arc<T>
///
/// See https://github.com/mozilla/cbindgen/issues/937 for the derive-{eq,neq}=false. But we don't
/// use those anyways so we can just disable them.
/// cbindgen:derive-eq=false
/// cbindgen:derive-neq=false
#[repr(C)]
struct ArcInner<T: ?Sized> {
    count: atomic::AtomicUsize,
    // NOTE(emilio): This needs to be here so that HeaderSlice<> is deallocated properly if the
    // allocator relies on getting the right Layout. We don't need to track the right alignment,
    // since we know that statically.
    //
    // This member could be completely avoided once min_specialization feature is stable (by
    // implementing a trait for HeaderSlice that gives you the right layout). For now, servo-only
    // since Gecko doesn't need it (its allocator doesn't need the size for the alignments we care
    // about). See https://github.com/rust-lang/rust/issues/31844.
    #[cfg(feature = "track_alloc_size")]
    alloc_size: usize,
    data: T,
}

unsafe impl<T: ?Sized + Sync + Send> Send for ArcInner<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for ArcInner<T> {}

/// Computes the offset of the data field within ArcInner.
fn data_offset<T>() -> usize {
    let size = size_of::<ArcInner<()>>();
    let align = align_of::<T>();
    // https://github.com/rust-lang/rust/blob/1.36.0/src/libcore/alloc.rs#L187-L207
    size.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}

impl<T> Arc<T> {
    /// Construct an `Arc<T>`
    #[inline]
    pub fn new(data: T) -> Self {
        let layout = Layout::new::<ArcInner<T>>();
        let p = unsafe {
            let ptr = ptr::NonNull::new(alloc::alloc(layout))
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
                .cast::<ArcInner<T>>();
            ptr::write(
                ptr.as_ptr(),
                ArcInner {
                    count: atomic::AtomicUsize::new(1),
                    #[cfg(feature = "track_alloc_size")]
                    alloc_size: layout.size(),
                    data,
                },
            );
            ptr
        };

        #[cfg(feature = "gecko_refcount_logging")]
        unsafe {
            // FIXME(emilio): Would be so amazing to have
            // std::intrinsics::type_name() around, so that we could also report
            // a real size.
            NS_LogCtor(p.as_ptr() as *mut _, b"ServoArc\0".as_ptr() as *const _, 8);
        }

        Arc {
            p,
            phantom: PhantomData,
        }
    }

    /// Construct an intentionally-leaked arc.
    #[inline]
    pub fn new_leaked(data: T) -> Self {
        let arc = Self::new(data);
        arc.mark_as_intentionally_leaked();
        arc
    }

    /// Convert the Arc<T> to a raw pointer, suitable for use across FFI
    ///
    /// Note: This returns a pointer to the data T, which is offset in the allocation.
    #[inline]
    pub fn into_raw(this: Self) -> *const T {
        let ptr = unsafe { &((*this.ptr()).data) as *const _ };
        mem::forget(this);
        ptr
    }

    /// Reconstruct the Arc<T> from a raw pointer obtained from into_raw()
    ///
    /// Note: This raw pointer will be offset in the allocation and must be preceded
    /// by the atomic count.
    #[inline]
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        // To find the corresponding pointer to the `ArcInner` we need
        // to subtract the offset of the `data` field from the pointer.
        let ptr = (ptr as *const u8).sub(data_offset::<T>());
        Arc {
            p: ptr::NonNull::new_unchecked(ptr as *mut ArcInner<T>),
            phantom: PhantomData,
        }
    }

    /// Like from_raw, but returns an addrefed arc instead.
    #[inline]
    pub unsafe fn from_raw_addrefed(ptr: *const T) -> Self {
        let arc = Self::from_raw(ptr);
        mem::forget(arc.clone());
        arc
    }

    /// Create a new static Arc<T> (one that won't reference count the object)
    /// and place it in the allocation provided by the specified `alloc`
    /// function.
    ///
    /// `alloc` must return a pointer into a static allocation suitable for
    /// storing data with the `Layout` passed into it. The pointer returned by
    /// `alloc` will not be freed.
    #[inline]
    pub unsafe fn new_static<F>(alloc: F, data: T) -> Arc<T>
    where
        F: FnOnce(Layout) -> *mut u8,
    {
        let layout = Layout::new::<ArcInner<T>>();
        let ptr = alloc(layout) as *mut ArcInner<T>;

        let x = ArcInner {
            count: atomic::AtomicUsize::new(STATIC_REFCOUNT),
            #[cfg(feature = "track_alloc_size")]
            alloc_size: layout.size(),
            data,
        };

        ptr::write(ptr, x);

        Arc {
            p: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }

    /// Produce a pointer to the data that can be converted back
    /// to an Arc. This is basically an `&Arc<T>`, without the extra indirection.
    /// It has the benefits of an `&T` but also knows about the underlying refcount
    /// and can be converted into more `Arc<T>`s if necessary.
    #[inline]
    pub fn borrow_arc<'a>(&'a self) -> ArcBorrow<'a, T> {
        ArcBorrow(&**self)
    }

    /// Returns the address on the heap of the Arc itself -- not the T within it -- for memory
    /// reporting.
    ///
    /// If this is a static reference, this returns null.
    pub fn heap_ptr(&self) -> *const c_void {
        if self.inner().count.load(Relaxed) == STATIC_REFCOUNT {
            ptr::null()
        } else {
            self.p.as_ptr() as *const ArcInner<T> as *const c_void
        }
    }
}

impl<T: ?Sized> Arc<T> {
    #[inline]
    fn inner(&self) -> &ArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { &*self.ptr() }
    }

    #[inline(always)]
    fn record_drop(&self) {
        #[cfg(feature = "gecko_refcount_logging")]
        unsafe {
            NS_LogDtor(self.ptr() as *mut _, b"ServoArc\0".as_ptr() as *const _, 8);
        }
    }

    /// Marks this `Arc` as intentionally leaked for the purposes of refcount
    /// logging.
    ///
    /// It's a logic error to call this more than once, but it's not unsafe, as
    /// it'd just report negative leaks.
    #[inline(always)]
    pub fn mark_as_intentionally_leaked(&self) {
        self.record_drop();
    }

    // Non-inlined part of `drop`. Just invokes the destructor and calls the
    // refcount logging machinery if enabled.
    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        self.record_drop();
        let inner = self.ptr();

        let layout = Layout::for_value(&*inner);
        #[cfg(feature = "track_alloc_size")]
        let layout = Layout::from_size_align_unchecked((*inner).alloc_size, layout.align());

        std::ptr::drop_in_place(inner);
        alloc::dealloc(inner as *mut _, layout);
    }

    /// Test pointer equality between the two Arcs, i.e. they must be the _same_
    /// allocation
    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.raw_ptr() == other.raw_ptr()
    }

    fn ptr(&self) -> *mut ArcInner<T> {
        self.p.as_ptr()
    }

    /// Returns a raw ptr to the underlying allocation.
    pub fn raw_ptr(&self) -> ptr::NonNull<()> {
        self.p.cast()
    }
}

#[cfg(feature = "gecko_refcount_logging")]
extern "C" {
    fn NS_LogCtor(
        aPtr: *mut std::os::raw::c_void,
        aTypeName: *const std::os::raw::c_char,
        aSize: u32,
    );
    fn NS_LogDtor(
        aPtr: *mut std::os::raw::c_void,
        aTypeName: *const std::os::raw::c_char,
        aSize: u32,
    );
}

impl<T: ?Sized> Clone for Arc<T> {
    #[inline]
    fn clone(&self) -> Self {
        // NOTE(emilio): If you change anything here, make sure that the
        // implementation in layout/style/ServoStyleConstsInlines.h matches!
        //
        // Using a relaxed ordering to check for STATIC_REFCOUNT is safe, since
        // `count` never changes between STATIC_REFCOUNT and other values.
        if self.inner().count.load(Relaxed) != STATIC_REFCOUNT {
            // Using a relaxed ordering is alright here, as knowledge of the
            // original reference prevents other threads from erroneously deleting
            // the object.
            //
            // As explained in the [Boost documentation][1], Increasing the
            // reference counter can always be done with memory_order_relaxed: New
            // references to an object can only be formed from an existing
            // reference, and passing an existing reference from one thread to
            // another must already provide any required synchronization.
            //
            // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
            let old_size = self.inner().count.fetch_add(1, Relaxed);

            // However we need to guard against massive refcounts in case someone
            // is `mem::forget`ing Arcs. If we don't do this the count can overflow
            // and users will use-after free. We racily saturate to `isize::MAX` on
            // the assumption that there aren't ~2 billion threads incrementing
            // the reference count at once. This branch will never be taken in
            // any realistic program.
            //
            // We abort because such a program is incredibly degenerate, and we
            // don't care to support it.
            if old_size > MAX_REFCOUNT {
                process::abort();
            }
        }

        unsafe {
            Arc {
                p: ptr::NonNull::new_unchecked(self.ptr()),
                phantom: PhantomData,
            }
        }
    }
}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner().data
    }
}

impl<T: Clone> Arc<T> {
    /// Makes a mutable reference to the `Arc`, cloning if necessary
    ///
    /// This is functionally equivalent to [`Arc::make_mut`][mm] from the standard library.
    ///
    /// If this `Arc` is uniquely owned, `make_mut()` will provide a mutable
    /// reference to the contents. If not, `make_mut()` will create a _new_ `Arc`
    /// with a copy of the contents, update `this` to point to it, and provide
    /// a mutable reference to its contents.
    ///
    /// This is useful for implementing copy-on-write schemes where you wish to
    /// avoid copying things if your `Arc` is not shared.
    ///
    /// [mm]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#method.make_mut
    #[inline]
    pub fn make_mut(this: &mut Self) -> &mut T {
        if !this.is_unique() {
            // Another pointer exists; clone
            *this = Arc::new((**this).clone());
        }

        unsafe {
            // This unsafety is ok because we're guaranteed that the pointer
            // returned is the *only* pointer that will ever be returned to T. Our
            // reference count is guaranteed to be 1 at this point, and we required
            // the Arc itself to be `mut`, so we're returning the only possible
            // reference to the inner data.
            &mut (*this.ptr()).data
        }
    }
}

impl<T: ?Sized> Arc<T> {
    /// Provides mutable access to the contents _if_ the `Arc` is uniquely owned.
    #[inline]
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.is_unique() {
            unsafe {
                // See make_mut() for documentation of the threadsafety here.
                Some(&mut (*this.ptr()).data)
            }
        } else {
            None
        }
    }

    /// Whether or not the `Arc` is a static reference.
    #[inline]
    pub fn is_static(&self) -> bool {
        // Using a relaxed ordering to check for STATIC_REFCOUNT is safe, since
        // `count` never changes between STATIC_REFCOUNT and other values.
        self.inner().count.load(Relaxed) == STATIC_REFCOUNT
    }

    /// Whether or not the `Arc` is uniquely owned (is the refcount 1?) and not
    /// a static reference.
    #[inline]
    pub fn is_unique(&self) -> bool {
        // See the extensive discussion in [1] for why this needs to be Acquire.
        //
        // [1] https://github.com/servo/servo/issues/21186
        self.inner().count.load(Acquire) == 1
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    #[inline]
    fn drop(&mut self) {
        // NOTE(emilio): If you change anything here, make sure that the
        // implementation in layout/style/ServoStyleConstsInlines.h matches!
        if self.is_static() {
            return;
        }

        // Because `fetch_sub` is already atomic, we do not need to synchronize
        // with other threads unless we are going to delete the object.
        if self.inner().count.fetch_sub(1, Release) != 1 {
            return;
        }

        // FIXME(bholley): Use the updated comment when [2] is merged.
        //
        // This load is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` load. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this load, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: https://github.com/rust-lang/rust/pull/41714
        self.inner().count.load(Acquire);

        unsafe {
            self.drop_slow();
        }
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Arc<T> {
    fn eq(&self, other: &Arc<T>) -> bool {
        Self::ptr_eq(self, other) || *(*self) == *(*other)
    }

    fn ne(&self, other: &Arc<T>) -> bool {
        !Self::ptr_eq(self, other) && *(*self) != *(*other)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Arc<T> {
    fn partial_cmp(&self, other: &Arc<T>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    fn lt(&self, other: &Arc<T>) -> bool {
        *(*self) < *(*other)
    }

    fn le(&self, other: &Arc<T>) -> bool {
        *(*self) <= *(*other)
    }

    fn gt(&self, other: &Arc<T>) -> bool {
        *(*self) > *(*other)
    }

    fn ge(&self, other: &Arc<T>) -> bool {
        *(*self) >= *(*other)
    }
}
impl<T: ?Sized + Ord> Ord for Arc<T> {
    fn cmp(&self, other: &Arc<T>) -> Ordering {
        (**self).cmp(&**other)
    }
}
impl<T: ?Sized + Eq> Eq for Arc<T> {}

impl<T: ?Sized + fmt::Display> fmt::Display for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr(), f)
    }
}

impl<T: Default> Default for Arc<T> {
    fn default() -> Arc<T> {
        Arc::new(Default::default())
    }
}

impl<T: ?Sized + Hash> Hash for Arc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T> From<T> for Arc<T> {
    #[inline]
    fn from(t: T) -> Self {
        Arc::new(t)
    }
}

impl<T: ?Sized> borrow::Borrow<T> for Arc<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &**self
    }
}

unsafe impl<T: ?Sized> StableDeref for Arc<T> {}
unsafe impl<T: ?Sized> CloneStableDeref for Arc<T> {}

#[cfg(feature = "servo")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Arc<T> {
    fn deserialize<D>(deserializer: D) -> Result<Arc<T>, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Arc::new)
    }
}

#[cfg(feature = "servo")]
impl<T: Serialize> Serialize for Arc<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        (**self).serialize(serializer)
    }
}

/// Structure to allow Arc-managing some fixed-sized data and a variably-sized
/// slice in a single allocation.
///
/// cbindgen:derive-eq=false
/// cbindgen:derive-neq=false
#[derive(Eq)]
#[repr(C)]
pub struct HeaderSlice<H, T> {
    /// The fixed-sized data.
    pub header: H,

    /// The length of the slice at our end.
    len: usize,

    /// The dynamically-sized data.
    data: [T; 0],
}

impl<H: PartialEq, T: PartialEq> PartialEq for HeaderSlice<H, T> {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.slice() == other.slice()
    }
}

impl<H, T> Drop for HeaderSlice<H, T> {
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.data_mut();
            for _ in 0..self.len {
                std::ptr::drop_in_place(ptr);
                ptr = ptr.offset(1);
            }
        }
    }
}

impl<H: fmt::Debug, T: fmt::Debug> fmt::Debug for HeaderSlice<H, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HeaderSlice")
            .field("header", &self.header)
            .field("slice", &self.slice())
            .finish()
    }
}

impl<H, T> HeaderSlice<H, T> {
    /// Returns the dynamically sized slice in this HeaderSlice.
    #[inline(always)]
    pub fn slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data(), self.len) }
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        std::ptr::addr_of!(self.data) as *const _
    }

    #[inline(always)]
    fn data_mut(&mut self) -> *mut T {
        std::ptr::addr_of_mut!(self.data) as *mut _
    }

    /// Returns the dynamically sized slice in this HeaderSlice.
    #[inline(always)]
    pub fn slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_mut(), self.len) }
    }

    /// Returns the len of the slice.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<H, T> Arc<HeaderSlice<H, T>> {
    /// Creates an Arc for a HeaderSlice using the given header struct and
    /// iterator to generate the slice.
    ///
    /// `is_static` indicates whether to create a static Arc.
    ///
    /// `alloc` is used to get a pointer to the memory into which the
    /// dynamically sized ArcInner<HeaderSlice<H, T>> value will be
    /// written.  If `is_static` is true, then `alloc` must return a
    /// pointer into some static memory allocation.  If it is false,
    /// then `alloc` must return an allocation that can be dellocated
    /// by calling Box::from_raw::<ArcInner<HeaderSlice<H, T>>> on it.
    #[inline]
    pub fn from_header_and_iter_alloc<F, I>(
        alloc: F,
        header: H,
        mut items: I,
        num_items: usize,
        is_static: bool,
    ) -> Self
    where
        F: FnOnce(Layout) -> *mut u8,
        I: Iterator<Item = T>,
    {
        assert_ne!(size_of::<T>(), 0, "Need to think about ZST");

        let layout = Layout::new::<ArcInner<HeaderSlice<H, T>>>();
        debug_assert!(layout.align() >= align_of::<T>());
        debug_assert!(layout.align() >= align_of::<usize>());
        let array_layout = Layout::array::<T>(num_items).expect("Overflow");
        let (layout, _offset) = layout.extend(array_layout).expect("Overflow");
        let p = unsafe {
            // Allocate the buffer.
            let buffer = alloc(layout);
            let mut p = ptr::NonNull::new(buffer)
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
                .cast::<ArcInner<HeaderSlice<H, T>>>();

            // Write the data.
            //
            // Note that any panics here (i.e. from the iterator) are safe, since
            // we'll just leak the uninitialized memory.
            let count = if is_static {
                atomic::AtomicUsize::new(STATIC_REFCOUNT)
            } else {
                atomic::AtomicUsize::new(1)
            };
            ptr::write(&mut p.as_mut().count, count);
            #[cfg(feature = "track_alloc_size")]
            ptr::write(&mut p.as_mut().alloc_size, layout.size());
            ptr::write(&mut p.as_mut().data.header, header);
            ptr::write(&mut p.as_mut().data.len, num_items);
            if num_items != 0 {
                let mut current = std::ptr::addr_of_mut!(p.as_mut().data.data) as *mut T;
                for _ in 0..num_items {
                    ptr::write(
                        current,
                        items
                            .next()
                            .expect("ExactSizeIterator over-reported length"),
                    );
                    current = current.offset(1);
                }
                // We should have consumed the buffer exactly, maybe accounting
                // for some padding from the alignment.
                debug_assert!(
                    (buffer.add(layout.size()) as usize - current as *mut u8 as usize)
                        < layout.align()
                );
            }
            assert!(
                items.next().is_none(),
                "ExactSizeIterator under-reported length"
            );
            p
        };
        #[cfg(feature = "gecko_refcount_logging")]
        unsafe {
            if !is_static {
                // FIXME(emilio): Would be so amazing to have
                // std::intrinsics::type_name() around.
                NS_LogCtor(p.as_ptr() as *mut _, b"ServoArc\0".as_ptr() as *const _, 8)
            }
        }

        // Return the fat Arc.
        assert_eq!(
            size_of::<Self>(),
            size_of::<usize>(),
            "The Arc should be thin"
        );

        Arc {
            p,
            phantom: PhantomData,
        }
    }

    /// Creates an Arc for a HeaderSlice using the given header struct and iterator to generate the
    /// slice. Panics if num_items doesn't match the number of items.
    #[inline]
    pub fn from_header_and_iter_with_size<I>(header: H, items: I, num_items: usize) -> Self
    where
        I: Iterator<Item = T>,
    {
        Arc::from_header_and_iter_alloc(
            |layout| unsafe { alloc::alloc(layout) },
            header,
            items,
            num_items,
            /* is_static = */ false,
        )
    }

    /// Creates an Arc for a HeaderSlice using the given header struct and
    /// iterator to generate the slice. The resulting Arc will be fat.
    #[inline]
    pub fn from_header_and_iter<I>(header: H, items: I) -> Self
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        let len = items.len();
        Self::from_header_and_iter_with_size(header, items, len)
    }
}

/// This is functionally equivalent to Arc<(H, [T])>
///
/// When you create an `Arc` containing a dynamically sized type like a slice, the `Arc` is
/// represented on the stack as a "fat pointer", where the length of the slice is stored alongside
/// the `Arc`'s pointer. In some situations you may wish to have a thin pointer instead, perhaps
/// for FFI compatibility or space efficiency. `ThinArc` solves this by storing the length in the
/// allocation itself, via `HeaderSlice`.
pub type ThinArc<H, T> = Arc<HeaderSlice<H, T>>;

/// See `ArcUnion`. This is a version that works for `ThinArc`s.
pub type ThinArcUnion<H1, T1, H2, T2> = ArcUnion<HeaderSlice<H1, T1>, HeaderSlice<H2, T2>>;

impl<H, T> UniqueArc<HeaderSlice<H, T>> {
    #[inline]
    pub fn from_header_and_iter<I>(header: H, items: I) -> Self
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        Self(Arc::from_header_and_iter(header, items))
    }

    #[inline]
    pub fn from_header_and_iter_with_size<I>(header: H, items: I, num_items: usize) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(Arc::from_header_and_iter_with_size(
            header, items, num_items,
        ))
    }

    /// Returns a mutable reference to the header.
    pub fn header_mut(&mut self) -> &mut H {
        // We know this to be uniquely owned
        unsafe { &mut (*self.0.ptr()).data.header }
    }

    /// Returns a mutable reference to the slice.
    pub fn data_mut(&mut self) -> &mut [T] {
        // We know this to be uniquely owned
        unsafe { (*self.0.ptr()).data.slice_mut() }
    }
}

/// A "borrowed `Arc`". This is a pointer to
/// a T that is known to have been allocated within an
/// `Arc`.
///
/// This is equivalent in guarantees to `&Arc<T>`, however it is
/// a bit more flexible. To obtain an `&Arc<T>` you must have
/// an `Arc<T>` instance somewhere pinned down until we're done with it.
/// It's also a direct pointer to `T`, so using this involves less pointer-chasing
///
/// However, C++ code may hand us refcounted things as pointers to T directly,
/// so we have to conjure up a temporary `Arc` on the stack each time.
///
/// `ArcBorrow` lets us deal with borrows of known-refcounted objects
/// without needing to worry about where the `Arc<T>` is.
#[derive(Debug, Eq, PartialEq)]
pub struct ArcBorrow<'a, T: 'a>(&'a T);

impl<'a, T> Copy for ArcBorrow<'a, T> {}
impl<'a, T> Clone for ArcBorrow<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> ArcBorrow<'a, T> {
    /// Clone this as an `Arc<T>`. This bumps the refcount.
    #[inline]
    pub fn clone_arc(&self) -> Arc<T> {
        let arc = unsafe { Arc::from_raw(self.0) };
        // addref it!
        mem::forget(arc.clone());
        arc
    }

    /// For constructing from a reference known to be Arc-backed,
    /// e.g. if we obtain such a reference over FFI
    #[inline]
    pub unsafe fn from_ref(r: &'a T) -> Self {
        ArcBorrow(r)
    }

    /// Compare two `ArcBorrow`s via pointer equality. Will only return
    /// true if they come from the same allocation
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.0 as *const T == other.0 as *const T
    }

    /// Temporarily converts |self| into a bonafide Arc and exposes it to the
    /// provided callback. The refcount is not modified.
    #[inline]
    pub fn with_arc<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&Arc<T>) -> U,
        T: 'static,
    {
        // Synthesize transient Arc, which never touches the refcount.
        let transient = unsafe { mem::ManuallyDrop::new(Arc::from_raw(self.0)) };

        // Expose the transient Arc to the callback, which may clone it if it wants.
        let result = f(&transient);

        // Forward the result.
        result
    }

    /// Similar to deref, but uses the lifetime |a| rather than the lifetime of
    /// self, which is incompatible with the signature of the Deref trait.
    #[inline]
    pub fn get(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> Deref for ArcBorrow<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0
    }
}

/// A tagged union that can represent `Arc<A>` or `Arc<B>` while only consuming a
/// single word. The type is also `NonNull`, and thus can be stored in an Option
/// without increasing size.
///
/// This is functionally equivalent to
/// `enum ArcUnion<A, B> { First(Arc<A>), Second(Arc<B>)` but only takes up
/// up a single word of stack space.
///
/// This could probably be extended to support four types if necessary.
pub struct ArcUnion<A, B> {
    p: ptr::NonNull<()>,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

unsafe impl<A: Sync + Send, B: Send + Sync> Send for ArcUnion<A, B> {}
unsafe impl<A: Sync + Send, B: Send + Sync> Sync for ArcUnion<A, B> {}

impl<A: PartialEq, B: PartialEq> PartialEq for ArcUnion<A, B> {
    fn eq(&self, other: &Self) -> bool {
        use crate::ArcUnionBorrow::*;
        match (self.borrow(), other.borrow()) {
            (First(x), First(y)) => x == y,
            (Second(x), Second(y)) => x == y,
            (_, _) => false,
        }
    }
}

impl<A: Eq, B: Eq> Eq for ArcUnion<A, B> {}

/// This represents a borrow of an `ArcUnion`.
#[derive(Debug)]
pub enum ArcUnionBorrow<'a, A: 'a, B: 'a> {
    First(ArcBorrow<'a, A>),
    Second(ArcBorrow<'a, B>),
}

impl<A, B> ArcUnion<A, B> {
    unsafe fn new(ptr: *mut ()) -> Self {
        ArcUnion {
            p: ptr::NonNull::new_unchecked(ptr),
            phantom_a: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Returns true if the two values are pointer-equal.
    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.p == other.p
    }

    #[inline]
    pub fn ptr(&self) -> ptr::NonNull<()> {
        self.p
    }

    /// Returns an enum representing a borrow of either A or B.
    #[inline]
    pub fn borrow(&self) -> ArcUnionBorrow<'_, A, B> {
        if self.is_first() {
            let ptr = self.p.as_ptr() as *const ArcInner<A>;
            let borrow = unsafe { ArcBorrow::from_ref(&(*ptr).data) };
            ArcUnionBorrow::First(borrow)
        } else {
            let ptr = ((self.p.as_ptr() as usize) & !0x1) as *const ArcInner<B>;
            let borrow = unsafe { ArcBorrow::from_ref(&(*ptr).data) };
            ArcUnionBorrow::Second(borrow)
        }
    }

    /// Creates an `ArcUnion` from an instance of the first type.
    pub fn from_first(other: Arc<A>) -> Self {
        let union = unsafe { Self::new(other.ptr() as *mut _) };
        mem::forget(other);
        union
    }

    /// Creates an `ArcUnion` from an instance of the second type.
    pub fn from_second(other: Arc<B>) -> Self {
        let union = unsafe { Self::new(((other.ptr() as usize) | 0x1) as *mut _) };
        mem::forget(other);
        union
    }

    /// Returns true if this `ArcUnion` contains the first type.
    pub fn is_first(&self) -> bool {
        self.p.as_ptr() as usize & 0x1 == 0
    }

    /// Returns true if this `ArcUnion` contains the second type.
    pub fn is_second(&self) -> bool {
        !self.is_first()
    }

    /// Returns a borrow of the first type if applicable, otherwise `None`.
    pub fn as_first(&self) -> Option<ArcBorrow<'_, A>> {
        match self.borrow() {
            ArcUnionBorrow::First(x) => Some(x),
            ArcUnionBorrow::Second(_) => None,
        }
    }

    /// Returns a borrow of the second type if applicable, otherwise None.
    pub fn as_second(&self) -> Option<ArcBorrow<'_, B>> {
        match self.borrow() {
            ArcUnionBorrow::First(_) => None,
            ArcUnionBorrow::Second(x) => Some(x),
        }
    }
}

impl<A, B> Clone for ArcUnion<A, B> {
    fn clone(&self) -> Self {
        match self.borrow() {
            ArcUnionBorrow::First(x) => ArcUnion::from_first(x.clone_arc()),
            ArcUnionBorrow::Second(x) => ArcUnion::from_second(x.clone_arc()),
        }
    }
}

impl<A, B> Drop for ArcUnion<A, B> {
    fn drop(&mut self) {
        match self.borrow() {
            ArcUnionBorrow::First(x) => unsafe {
                let _ = Arc::from_raw(&*x);
            },
            ArcUnionBorrow::Second(x) => unsafe {
                let _ = Arc::from_raw(&*x);
            },
        }
    }
}

impl<A: fmt::Debug, B: fmt::Debug> fmt::Debug for ArcUnion<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.borrow(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::{Arc, ThinArc};
    use std::clone::Clone;
    use std::ops::Drop;
    use std::sync::atomic;
    use std::sync::atomic::Ordering::{Acquire, SeqCst};

    #[derive(PartialEq)]
    struct Canary(*mut atomic::AtomicUsize);

    impl Drop for Canary {
        fn drop(&mut self) {
            unsafe {
                (*self.0).fetch_add(1, SeqCst);
            }
        }
    }

    #[test]
    fn empty_thin() {
        let x = Arc::from_header_and_iter(100u32, std::iter::empty::<i32>());
        assert_eq!(x.header, 100);
        assert!(x.slice().is_empty());
    }

    #[test]
    fn thin_assert_padding() {
        #[derive(Clone, Default)]
        #[repr(C)]
        struct Padded {
            i: u16,
        }

        // The header will have more alignment than `Padded`
        let items = vec![Padded { i: 0xdead }, Padded { i: 0xbeef }];
        let a = ThinArc::from_header_and_iter(0i32, items.into_iter());
        assert_eq!(a.len(), 2);
        assert_eq!(a.slice()[0].i, 0xdead);
        assert_eq!(a.slice()[1].i, 0xbeef);
    }

    #[test]
    fn slices_and_thin() {
        let mut canary = atomic::AtomicUsize::new(0);
        let c = Canary(&mut canary as *mut atomic::AtomicUsize);
        let v = vec![5, 6];
        {
            let x = Arc::from_header_and_iter(c, v.into_iter());
            let _ = x.clone();
            let _ = x == x;
        }
        assert_eq!(canary.load(Acquire), 1);
    }
}
