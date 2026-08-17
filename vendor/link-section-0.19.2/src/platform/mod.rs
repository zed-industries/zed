pub mod apple;
pub mod standard;
pub mod wasm;
pub mod windows;

#[cfg(target_vendor = "apple")]
pub use apple::{get_section, section_name};
#[cfg(all(
    not(target_family = "wasm"),
    not(target_os = "windows"),
    not(target_vendor = "apple")
))]
pub use standard::{get_section, section_name};
#[cfg(target_family = "wasm")]
pub use wasm::{get_section, section_name};
#[cfg(target_os = "windows")]
pub use windows::{get_section, section_name};

// Select the appropriate bounds and reference-storage types for the platform.
#[cfg(target_family = "wasm")]
pub use wasm::{Bounds, MovableBounds, MovableRefStorage, RefStorage};
#[cfg(not(target_family = "wasm"))]
pub use {PtrBounds as Bounds, PtrMovableBounds as MovableBounds};

/// Rejects section names that cannot be represented on the current target.
pub const fn validate_section_name(name: &str) {
    if cfg!(target_vendor = "apple") {
        apple::validate_apple_section_name(name);
    }
    if cfg!(all(
        not(target_family = "wasm"),
        not(target_os = "windows"),
        not(target_vendor = "apple")
    )) {
        standard::is_valid_section_name(name);
    }
}

/// Launder a pointer's provenance so it appears as an "exposed" pointer.
pub fn launder_pointer_provenance<T>(ptr: *const T) -> *const T {
    #[cfg(not(windows))]
    {
        core::ptr::with_exposed_provenance(ptr.expose_provenance())
    }

    // Windows requires a stronger hint to avoid mis-optimization. On Windows, section bounds
    // come from marker sections. LLVM may fold ptrtoint/inttoptr under LTO, which takes our
    // exposed provenance and reverts it, which it then traces to the marker allocation which
    // it then believes all slice loads come from.
    //
    // Treating this provenance round-trip as a no-op is arguably an LLVM optimization issue
    // somewhere between Rust and LLVM.
    #[cfg(windows)]
    {
        core::hint::black_box(core::ptr::with_exposed_provenance(ptr.expose_provenance()))
    }
}

/// An immutable snapshot of a section's `[start, end)` range.
///
/// Callers resolve a section's bounds once (via [`Bounds::range`] and friends)
/// and then compute length, slices, and offsets from the snapshot without
/// re-locking. `start` and `end` share provenance.
#[derive(Clone, Copy)]
pub struct SectionRange {
    start: *const (),
    end: *const (),
}

impl SectionRange {
    /// A range covering `[start, end)`. `start` and `end` must share provenance.
    #[inline(always)]
    pub const fn new(start: *const (), end: *const ()) -> Self {
        Self { start, end }
    }

    /// Section start address.
    #[inline(always)]
    pub const fn start_ptr(&self) -> *const () {
        self.start
    }

    /// One byte past the last section byte.
    #[inline(always)]
    pub const fn end_ptr(&self) -> *const () {
        self.end
    }

    /// Length in bytes (`end - start`).
    #[inline(always)]
    pub fn byte_len(&self) -> usize {
        // Provenance-insensitive difference.
        self.end.addr() - self.start.addr()
    }

    /// The range as a typed slice. `stride` must divide `byte_len()`. This is
    /// the shared body of every `as_slice` accessor (see `sections.rs`), kept
    /// here so the empty-range fast path and the `from_raw_parts` cast have one
    /// source of truth.
    ///
    /// The returned slice borrows the underlying section memory, not this
    /// snapshot (which is just two pointers and may be a temporary). The output
    /// lifetime is unbound and the caller must tie it to the real borrow.
    ///
    /// # Safety
    ///
    /// `self` must denote a valid, aligned, readable range of `len = byte_len()
    /// / stride` consecutive `T`s, and the returned slice's lifetime must not
    /// outlive that memory. Callers holding `&section` tie the lifetime to that
    /// borrow.
    #[inline]
    pub unsafe fn slice_of<'a, T>(self, stride: usize) -> &'a [T] {
        let len = self.byte_len() / stride;
        if len == 0 {
            &[]
        } else {
            unsafe { ::core::slice::from_raw_parts(self.start as *const T, len) }
        }
    }

    /// The mutable counterpart to [`Self::slice_of`].
    ///
    /// # Safety
    ///
    /// As [`Self::slice_of`], plus: the caller must uphold exclusivity — no
    /// other reference (shared or mutable) to the same memory may exist for the
    /// returned slice's lifetime. Calling this twice on the same range without
    /// an intervening reborrow is UB.
    #[inline]
    pub unsafe fn slice_of_mut<'a, T>(self, stride: usize) -> &'a mut [T] {
        let len = self.byte_len() / stride;
        if len == 0 {
            &mut []
        } else {
            unsafe { ::core::slice::from_raw_parts_mut(self.start as *mut T, len) }
        }
    }
}

/// Constant bounds for a pointer-based section.
pub struct PtrBounds {
    /// Section start address.
    pub start: *const (),
    /// One byte past the last section byte.
    pub end: *const (),
}

impl PtrBounds {
    /// Bounds covering `[start, end)`.
    pub const fn new(start: *const (), end: *const ()) -> Self {
        Self { start, end }
    }

    /// Resolve the section into an immutable [`SectionRange`].
    #[inline(always)]
    pub fn range(&self) -> SectionRange {
        // Launder the start's provenance and derive the end from it, so both
        // pointers share provenance while the length stays provenance-free.
        let start = launder_pointer_provenance(self.start);
        let byte_len = self.end.addr() - self.start.addr();
        let end = unsafe { (start as *const u8).add(byte_len) as *const () };
        SectionRange::new(start, end)
    }
}

/// Bounds for a movable section and its associated backref section.
pub struct PtrMovableBounds {
    /// Bounds for the submitted values.
    values: PtrBounds,
    /// Bounds for the submitted backrefs.
    refs: PtrBounds,
}

impl PtrMovableBounds {
    /// Create movable-section bounds.
    pub const fn new(values: PtrBounds, refs: PtrBounds) -> Self {
        Self { values, refs }
    }

    /// Resolve the movable item section into an immutable [`SectionRange`].
    #[inline(always)]
    pub fn range(&self) -> SectionRange {
        self.values.range()
    }

    /// Resolve the movable backref section into an immutable [`SectionRange`].
    #[inline(always)]
    pub fn backrefs_range(&self) -> SectionRange {
        self.refs.range()
    }
}

/// `UnsafeCell` that is `Sync` and `Send`.
#[repr(transparent)]
pub struct SyncUnsafeCell<T> {
    #[allow(unused)]
    cell: ::core::cell::UnsafeCell<T>,
}

impl<T> SyncUnsafeCell<T> {
    /// Create a new `SyncUnsafeCell`.
    pub const fn new(value: T) -> Self {
        Self {
            cell: ::core::cell::UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the contained value.
    #[inline]
    pub const fn get(&self) -> *mut T {
        self.cell.get()
    }
}

unsafe impl<T> Sync for SyncUnsafeCell<T> {}
unsafe impl<T> Send for SyncUnsafeCell<T> {}

/// Platform storage backing a [`crate::Ref`] off WASM: the value lives inline
/// (and is placed directly in the linker section), so the handle is layout-
/// compatible with `T`.
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct RefStorage<T: 'static> {
    t: T,
}

#[cfg(not(target_family = "wasm"))]
impl<T> RefStorage<T> {
    /// Storage holding `t` inline.
    pub const fn new(t: T) -> Self {
        Self { t }
    }

    /// Pointer to the inline value.
    pub fn as_ptr(&self) -> *const T {
        &self.t as *const T
    }

    /// The inline value.
    pub fn get(&self) -> &T {
        &self.t
    }
}

/// Platform storage backing a [`crate::MovableRef`] off WASM: a stable pointer
/// slot updated in place when the section is reordered. `slot` is the sole
/// field, so [`crate::MovableRef::slot_ptr`] is an offset-0 cast.
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct MovableRefStorage<T: 'static> {
    slot: SyncUnsafeCell<*const T>,
}

#[cfg(not(target_family = "wasm"))]
impl<T> MovableRefStorage<T> {
    /// Storage whose slot initially points at `ptr`.
    pub const fn new(ptr: *const T) -> Self {
        Self {
            slot: SyncUnsafeCell::new(ptr),
        }
    }

    /// The current slot pointer.
    pub const fn as_ptr(&self) -> *const T {
        unsafe { *self.slot.get() }
    }

    /// The item currently referenced by the slot.
    pub fn get(&self) -> &T {
        unsafe { self.as_ptr().as_ref().expect("MovableRef not initialized") }
    }
}

/// A non-zero-sized type that is used to align the start and end of the
/// section.
#[repr(C)]
pub struct Alignment<T> {
    _align: [T; 0],
    _padding: u8,
}

#[allow(clippy::new_without_default)]
impl<T> Alignment<T> {
    /// Zero-sized alignment anchor.
    pub const fn new() -> Self {
        Self {
            _align: [],
            _padding: 0,
        }
    }
}

/// Declares the section_name macro.
#[macro_export]
#[doc(hidden)]
macro_rules! __def_section_name {
    (
        $__name:ident,
        {$(
            $__section:ident $__type:ident => $__prefix:tt __ $__suffix:tt;
        )*}
        AUXILIARY = $__aux_sep:literal;
        REFS = $__refs_sep:literal;
        MAX_LENGTH = $__max_length:literal;
        HASH_LENGTH = $__hash_length:literal;
        VALID_SECTION_CHARS = $__valid_section_chars:literal;
    ) => {
        mod $__name {
            /// Internal macro for generating a section name.
            #[macro_export]
            #[doc(hidden)]
            macro_rules! $__name {
                $(
                    (string item $__section $__type ($name:tt () $unsafe:tt)) => {
                        $crate::__support::hash!($unsafe ($__prefix) ($name) ($__suffix) $__hash_length $__max_length $__valid_section_chars)
                    };
                    (string item $__section $__type ($aux:tt $name:tt $unsafe:tt)) => {
                        $crate::__support::hash!($unsafe ($__prefix) ($name $__aux_sep $aux) ($__suffix) $__hash_length $__max_length $__valid_section_chars)
                    };
                    (string backref $__section $__type ($name:tt () $unsafe:tt)) => {
                        $crate::__support::hash!($unsafe ($__prefix) ($name $__refs_sep) ($__suffix) $__hash_length $__max_length $__valid_section_chars)
                    };
                    (string backref $__section $__type ($aux:tt $name:tt $unsafe:tt)) => {
                        $crate::__support::hash!($unsafe ($__prefix) ($name $__aux_sep $aux $__refs_sep) ($__suffix) $__hash_length $__max_length $__valid_section_chars)
                    };
                )*
                ($pattern:tt $unknown_ref_or_item:ident $unknown_section:ident $unknown_type:ident $name:ident) => {
                    const _: () = {
                        compile_error!(concat!("Unknown section type: `", stringify!($unknown_ref_or_item), "/", stringify!($unknown_section), "/", stringify!($unknown_type), "`"));
                    }
                };
            }

            pub use $__name as section_name;
        }

        pub use $__name::section_name;

        pub(crate) const MAX_LENGTH: usize = $__max_length;
        pub(crate) const VALID_SECTION_CHARS: &[u8] = $__valid_section_chars.as_bytes();

        #[allow(unused)]
        pub(crate) const fn is_valid_section_char(b: u8) -> bool {
            let mut i = 0;
            while i < VALID_SECTION_CHARS.len() {
                if VALID_SECTION_CHARS[i] == b {
                    return true;
                }
                i += 1;
            }
            false
        }

        #[allow(unused)]
        pub(crate) const fn is_valid_section_name(name: &str) -> bool {
            let bytes = name.as_bytes();
            if bytes.is_empty() || bytes.len() > MAX_LENGTH {
                return false;
            }
            let mut i = 0;
            while i < bytes.len() {
                let b = bytes[i];
                if !is_valid_section_char(b) {
                    return false;
                }
                i += 1;
            }
            true
        }
    };
}
