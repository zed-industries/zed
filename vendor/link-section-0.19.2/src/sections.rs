#[cfg(not(target_family = "wasm"))]
use core::sync::atomic::{AtomicU8, Ordering};
use core::{cell::UnsafeCell, ptr};

use crate::__support::{Bounds, MovableBounds};

/// An untyped link section that can be used to store any type. The underlying
/// data is not enumerable.
#[repr(C)]
pub struct Section {
    name: &'static str,
    bounds: Bounds,
}

impl Section {
    /// # Safety
    ///
    /// For macro-generated use only. `bounds` must match the linker-resolved
    /// (or WASM-initialized) section range for `name`.
    #[doc(hidden)]
    pub const unsafe fn new(name: &'static str, bounds: Bounds) -> Self {
        Self { name, bounds }
    }

    /// The byte length of the section.
    #[inline]
    pub fn byte_len(&self) -> usize {
        self.bounds.range().byte_len()
    }

    /// The start address of the section.
    #[inline]
    pub fn start_ptr(&self) -> *const () {
        self.bounds.range().start_ptr()
    }
    /// The end address of the section.
    #[inline]
    pub fn end_ptr(&self) -> *const () {
        self.bounds.range().end_ptr()
    }

    /// Ensures that a section exists at the given path.
    #[doc(hidden)]
    pub const fn __validate<T: IsUntypedSection>(_section: &T) {}
}

impl ::core::fmt::Debug for Section {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Section")
            .field("name", &self.name)
            .field("start", &self.start_ptr())
            .field("end", &self.end_ptr())
            .field("byte_len", &self.byte_len())
            .finish()
    }
}

unsafe impl Sync for Section {}
unsafe impl Send for Section {}

// Waiting on Rust 1.78
// #[diagnostic::on_unimplemented(message = "This is not an untyped section")]
/// Marker: untyped [`Section`] handle.
pub trait IsUntypedSection {}

macro_rules! impl_section_new {
    ($generic:ident) => {
        /// # Safety
        ///
        /// For macro-generated use only. `bounds` must match the linker-resolved
        /// (or WASM-initialized) section range for `name`.
        #[doc(hidden)]
        pub const unsafe fn new(name: &'static str, bounds: Bounds) -> Self {
            assert!(
                ::core::mem::size_of::<$generic>() > 0,
                "Zero-sized types are not supported"
            );
            Self {
                name,
                bounds,
                _phantom: ::core::marker::PhantomData,
            }
        }
    };
}

macro_rules! impl_bounds_fns {
    ($generic:ident) => {
        /// The start address of the section.
        #[inline(always)]
        pub fn start_ptr(&self) -> *const T {
            self.bounds.range().start_ptr() as *const T
        }

        /// The end address of the section.
        #[inline(always)]
        pub fn end_ptr(&self) -> *const T {
            self.bounds.range().end_ptr() as *const T
        }

        /// The stride of the typed section.
        #[inline(always)]
        pub const fn stride(&self) -> usize {
            assert!(
                ::core::mem::size_of::<T>() > 0
                    && ::core::mem::size_of::<T>() * 2 == ::core::mem::size_of::<[T; 2]>()
            );
            ::core::mem::size_of::<T>()
        }

        /// The byte length of the section.
        #[inline]
        pub fn byte_len(&self) -> usize {
            self.bounds.range().byte_len()
        }

        /// The number of elements in the section.
        #[inline]
        pub fn len(&self) -> usize {
            self.byte_len() / self.stride()
        }

        /// True if the section is empty.
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// The section as a slice.
        #[inline]
        pub fn as_slice(&self) -> &[T] {
            // SAFETY: `bounds.range()` is the section's valid, aligned,
            // readable range of `T`s (linker-resolved or WASM-materialised),
            // and the returned slice is borrowed from `&self`, so it cannot
            // outlive the section.
            unsafe { self.bounds.range().slice_of::<T>(self.stride()) }
        }

        /// The offset of the item in the section, if it is in the section.
        ///
        /// This is O(1), as it performs direct pointer arithmetic.
        #[inline]
        pub fn offset_of(&self, item: impl $crate::SectionItemLocation<T>) -> Option<usize> {
            // Resolve the bounds up-front (complex operation on some platforms)
            let range = self.bounds.range();
            let start = range.start_ptr() as *const T;
            let end = range.end_ptr() as *const T;
            let ptr = item.item_ptr();
            if ptr < start || ptr >= end {
                None
            } else {
                Some(unsafe { ptr.offset_from(start) as usize })
            }
        }
    };
}

macro_rules! impl_bounds_traits {
    ($name:ident < $generic:ident >) => {
        impl<'a, $generic> ::core::iter::IntoIterator for &'a $name<$generic> {
            type Item = &'a $generic;
            type IntoIter = ::core::slice::Iter<'a, $generic>;
            fn into_iter(self) -> Self::IntoIter {
                self.as_slice().iter()
            }
        }

        impl<T> ::core::ops::Deref for $name<$generic> {
            type Target = [$generic];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl<T> ::core::fmt::Debug for $name<$generic> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("name", &self.name)
                    .field("start", &self.start_ptr())
                    .field("end", &self.end_ptr())
                    .field("len", &self.len())
                    .field("stride", &self.stride())
                    .finish()
            }
        }

        impl<T> $crate::__support::SectionItemType for $name<$generic> {
            type Item = $generic;
        }

        impl<T> $crate::__support::SectionItemTyped<$generic> for $name<$generic> {
            type Item = $generic;
        }

        unsafe impl<$generic> Sync for $name<$generic> where $generic: Sync {}
        unsafe impl<$generic> Send for $name<$generic> where $generic: Send {}
    };
}

/// A typed link section that can be used to store any sized type. The
/// underlying data is immutable and enumerable. `static` and `const` items are
/// stored directly in the section.
///
/// `static` items are guaranteed to have a valid return from
/// [`TypedSection::offset_of`] if they are in the section.
///
/// Platform note: WASM platforms require `const` items. Use
/// [`TypedReferenceSection`] for cross-platform support for `static` items.
#[repr(C)]
pub struct TypedSection<T: 'static> {
    name: &'static str,
    bounds: Bounds,
    _phantom: ::core::marker::PhantomData<T>,
}

impl<T: 'static> TypedSection<T> {
    impl_section_new!(T);
    impl_bounds_fns!(T);
}

impl_bounds_traits!(TypedSection<T>);

/// A mutable typed link section that can be used to store any sized type. The
/// underlying data is (unsafely) mutable and enumerable.
///
/// Only `const` items may be submitted to a [`TypedMutableSection`].
///
/// Mutating the section (for example via [`TypedMutableSection::as_mut_slice`])
/// requires exclusive access. See [Exclusive access](crate#exclusive-access) for
/// more information.
#[repr(C)]
pub struct TypedMutableSection<T: 'static> {
    name: &'static str,
    bounds: Bounds,
    _phantom: ::core::marker::PhantomData<T>,
}

impl<T: 'static> TypedMutableSection<T> {
    impl_section_new!(T);
    impl_bounds_fns!(T);

    /// The start address of the section.
    #[inline]
    pub fn start_ptr_mut(&self) -> *mut T {
        self.bounds.range().start_ptr() as *mut T
    }

    /// The start address of the section.
    #[inline]
    pub fn end_ptr_mut(&self) -> *mut T {
        self.bounds.range().end_ptr() as *mut T
    }

    /// The section as a mutable slice.
    ///
    /// # Safety
    ///
    /// Mutating the section requires exclusive access. See
    /// [Exclusive access](crate#exclusive-access) for more information.
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        // SAFETY: caller upholds exclusivity (see `# Safety` above); the range
        // is the section's valid, aligned `T` range.
        unsafe { self.bounds.range().slice_of_mut::<T>(self.stride()) }
    }
}

impl_bounds_traits!(TypedMutableSection<T>);

/// A movable typed link section that can be used to store any sized type. The
/// underlying data is (unsafely) mutable, enumerable, and expected to be
/// reordered during startup initialization. Each item is paired with a
/// [`MovableBackref`] that updates a stable [`MovableRef`] slot when the
/// section is sorted.
///
/// Only `static` items may be submitted to a [`TypedMovableSection`].
///
/// Mutating or reordering the section requires exclusive access. See
/// [Exclusive access](crate#exclusive-access) for more information.
/// [`TypedMovableSection::sort_unstable`] also updates every [`MovableRef`]; any
/// `&T` obtained before sorting may be stale afterward.
#[repr(C)]
pub struct TypedMovableSection<T: 'static> {
    name: &'static str,
    bounds: MovableBounds,
    #[cfg(not(target_family = "wasm"))]
    backref_state: AtomicU8,
    _phantom: ::core::marker::PhantomData<T>,
}

impl<T: 'static> TypedMovableSection<T> {
    /// # Safety
    ///
    /// For macro-generated use only. `bounds` must describe the final layout of
    /// the linker (or WASM runtime) section after all items are registered.
    #[doc(hidden)]
    pub const unsafe fn new(name: &'static str, bounds: MovableBounds) -> Self {
        assert!(
            ::core::mem::size_of::<T>() > 0,
            "Zero-sized types are not supported"
        );
        Self {
            name,
            bounds,
            #[cfg(not(target_family = "wasm"))]
            backref_state: AtomicU8::new(0),
            _phantom: ::core::marker::PhantomData,
        }
    }

    impl_bounds_fns!(T);

    /// The section as a mutable slice.
    ///
    /// # Safety
    ///
    /// Mutating the section requires exclusive access. See
    /// [Exclusive access](crate#exclusive-access) for more information.
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        // SAFETY: caller upholds exclusivity (see `# Safety` above); the range
        // is the section's valid, aligned `T` range.
        unsafe { self.bounds.range().slice_of_mut::<T>(self.stride()) }
    }

    /// The backrefs as a mutable slice, ordered to match the current value
    /// section addresses.
    ///
    /// # Safety
    ///
    /// Mutating the backref section requires exclusive access. See
    /// [Exclusive access](crate#exclusive-access) for more information. Do not
    /// call while any other slice into either the value or backref linker
    /// sections is live.
    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub unsafe fn as_mut_backrefs(&self) -> &mut [MovableBackref<T>] {
        let range = self.bounds.backrefs_range();
        // SAFETY: caller upholds exclusivity (see `# Safety` above); the range
        // is the backref section's valid, aligned `MovableBackref<T>` range.
        let backrefs = unsafe {
            range.slice_of_mut::<MovableBackref<T>>(::core::mem::size_of::<MovableBackref<T>>())
        };
        #[cfg(not(target_family = "wasm"))]
        unsafe {
            self.fixup_backrefs(backrefs)
        };
        backrefs
    }

    /// As we cannot guarantee that the linker placed the items and backrefs in
    /// the same order, we need to sort the backrefs to match the items.
    ///
    /// The exception, of course, is WASM where we placed both of them
    /// ourselves.
    #[cfg(not(target_family = "wasm"))]
    unsafe fn fixup_backrefs(&self, backrefs: &mut [MovableBackref<T>]) {
        match self
            .backref_state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
        {
            Ok(_) => {}
            Err(2) => return,
            Err(_) => panic!("movable section backrefs already being initialized"),
        }

        if backrefs.len() != self.len() {
            panic!(
                "movable section backref count ({}) does not match item count ({})",
                backrefs.len(),
                self.len()
            );
        }

        backrefs.sort_unstable_by_key(|backref| backref.current_ptr());
        self.backref_state.store(2, Ordering::Release);
    }

    /// Sort the section and backrefs in place.
    ///
    /// This algorithm is currently implemented as a quicksort.
    ///
    /// # Safety
    ///
    /// Reordering the section requires exclusive access. See
    /// [Exclusive access](crate#exclusive-access) for more information. After
    /// this returns, every [`MovableRef`] slot points at the new location of its
    /// item; any `&T` obtained through [`MovableRef`] before the sort must not
    /// be used.
    #[allow(unsafe_code)]
    pub unsafe fn sort_unstable(&self)
    where
        T: Ord,
    {
        // Trivial case.
        let main = unsafe { self.as_mut_slice() };
        if main.len() <= 1 {
            return;
        }

        let refs = unsafe { self.as_mut_backrefs() };
        debug_assert_eq!(main.len(), refs.len());

        fn partition<T: Ord, R>(main: &mut [T], refs: &mut [R]) -> usize {
            let n = main.len();
            if n == 0 {
                return 0;
            }
            let pivot = n - 1;
            let mut i = 0;
            for j in 0..pivot {
                if main[j] <= main[pivot] {
                    main.swap(i, j);
                    refs.swap(i, j);
                    i += 1;
                }
            }
            main.swap(i, pivot);
            refs.swap(i, pivot);
            i
        }

        fn recurse<T: Ord, R>(main: &mut [T], refs: &mut [R]) {
            let n = main.len();
            if n <= 1 {
                return;
            }
            let p = partition(main, refs);
            let (ml, mr) = main.split_at_mut(p);
            let (rl, rr) = refs.split_at_mut(p);
            recurse(ml, rl);
            if mr.len() > 1 {
                recurse(&mut mr[1..], &mut rr[1..]);
            }
        }

        recurse(main, refs);

        // TODO: could we avoid the fixup if no changes are made?
        for (item, backref) in main.iter().zip(refs.iter()) {
            unsafe {
                backref.set_current_ptr(item as *const T);
            }
        }
    }
}

impl_bounds_traits!(TypedMovableSection<T>);

/// A reference to a movable item through a stable pointer slot.
///
/// The slot is updated when a [`TypedMovableSection`] is reordered (for example
/// by [`TypedMovableSection::sort_unstable`]). Do not keep an `&T` from
/// dereferencing this handle across such an update.
///
/// The platform-specific representation lives in [`MovableRefStorage`], whose
/// `slot` is its first field so [`MovableRef::slot_ptr`] is an offset-0 cast.
///
/// [`MovableRefStorage`]: crate::__support::MovableRefStorage
#[repr(C)]
pub struct MovableRef<T: 'static> {
    storage: crate::__support::MovableRefStorage<T>,
}

impl<T> MovableRef<T> {
    #[doc(hidden)]
    pub const fn new(storage: crate::__support::MovableRefStorage<T>) -> Self {
        Self { storage }
    }

    /// Get a raw pointer to the stable pointer slot inside this handle. `slot`
    /// is the first field of the storage, so this is an offset-0 cast.
    #[doc(hidden)]
    pub const fn slot_ptr(this: *const Self) -> *const UnsafeCell<*const T> {
        this.cast::<UnsafeCell<*const T>>()
    }

    /// Raw pointer to the value currently referenced by this slot, read without
    /// flattening.
    pub(crate) const fn as_ptr(&self) -> *const T {
        self.storage.as_ptr()
    }
}

impl<T> ::core::ops::Deref for MovableRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.storage.get()
    }
}

impl<T> ::core::fmt::Debug for MovableRef<T>
where
    T: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> ::core::fmt::Display for MovableRef<T>
where
    T: ::core::fmt::Display,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

unsafe impl<T> Send for MovableRef<T> where T: Send {}
unsafe impl<T> Sync for MovableRef<T> where T: Sync {}

/// A backref record submitted alongside each item in a [`TypedMovableSection`].
/// This points to a [`MovableRef`] that lives outside of the section.
#[repr(C)]
pub struct MovableBackref<T: 'static> {
    slot: *const UnsafeCell<*const T>,
}

impl<T> MovableBackref<T> {
    #[doc(hidden)]
    pub const fn new(slot: *const UnsafeCell<*const T>) -> Self {
        Self { slot }
    }

    /// Current value of the stable pointer slot as a pointer.
    pub fn current_ptr(&self) -> *const T {
        unsafe { ptr::read(UnsafeCell::raw_get(self.slot)) }
    }

    /// Update the stable pointer slot.
    ///
    /// # Safety
    ///
    /// Updating the slot requires exclusive access. See
    /// [Exclusive access](crate#exclusive-access) for more information. Any live
    /// `&T` or [`MovableRef`] dereference may alias the old or new target.
    pub unsafe fn set_current_ptr(&self, ptr: *const T) {
        unsafe {
            ptr::write(UnsafeCell::raw_get(self.slot), ptr);
        }
    }
}

unsafe impl<T> Send for MovableBackref<T> where T: Send {}
unsafe impl<T> Sync for MovableBackref<T> where T: Sync {}

/// A typed link section that can be used to store any sized type. The
/// underlying data is enumerable.
#[repr(C)]
pub struct TypedReferenceSection<T: 'static> {
    name: &'static str,
    bounds: Bounds,
    _phantom: ::core::marker::PhantomData<T>,
}

impl<T: 'static> TypedReferenceSection<T> {
    impl_section_new!(T);
    impl_bounds_fns!(T);
}

impl_bounds_traits!(TypedReferenceSection<T>);

/// A reference to a value in a link section. This allows platforms like WASM
/// to reference the value, even though the final location is not known until
/// after initialization.
#[repr(C)]
pub struct Ref<T: 'static> {
    storage: crate::__support::RefStorage<T>,
}

impl<T> Ref<T> {
    #[doc(hidden)]
    pub const fn new(storage: crate::__support::RefStorage<T>) -> Self {
        Self { storage }
    }

    /// Raw pointer to the value, read without flattening on WASM. Internal
    /// plumbing for [`SectionItemLocation`](crate::SectionItemLocation):
    /// `offset_of` flattens via the section bounds first. Use `Deref` otherwise.
    pub(crate) fn as_ptr(&self) -> *const T {
        self.storage.as_ptr()
    }
}

impl<T> ::core::ops::Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.storage.get()
    }
}

impl<T> ::core::fmt::Debug for Ref<T>
where
    T: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> ::core::fmt::Display for Ref<T>
where
    T: ::core::fmt::Display,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

unsafe impl<T> Send for Ref<T> where T: Send {}
unsafe impl<T> Sync for Ref<T> where T: Sync {}
