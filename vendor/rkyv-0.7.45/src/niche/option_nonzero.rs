//! Niched archived `Option<NonZero>` integers that use less space.

use crate::Archived;
use core::{
    cmp, fmt, hash,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    pin::Pin,
};

macro_rules! impl_archived_option_nonzero {
    ($ar:ident, $nz:ty, $ne:ty) => {
        #[doc = concat!("A niched archived `Option<", stringify!($nz), ">`")]
        #[repr(transparent)]
        pub struct $ar {
            inner: Archived<$ne>,
        }

        impl $ar {
            /// Returns `true` if the option is a `None` value.
            #[inline]
            pub fn is_none(&self) -> bool {
                self.inner == 0
            }

            /// Returns `true` if the option is a `Some` value.
            #[inline]
            pub fn is_some(&self) -> bool {
                self.inner != 0
            }

            #[doc = concat!("Converts to an `Option<&Archived<", stringify!($nz), ">>`")]
            pub fn as_ref(&self) -> Option<&Archived<$nz>> {
                if self.inner != 0 {
                    let as_nonzero = unsafe {
                        // SAFETY: NonZero types have the same memory layout and bit patterns as
                        // their integer counterparts, regardless of endianness
                        &*(&self.inner as *const _ as *const Archived<$nz>)
                    };
                    Some(as_nonzero)
                } else {
                    None
                }
            }

            #[doc = concat!("Converts to an `Option<&mut Archived<", stringify!($nz), ">>`")]
            pub fn as_mut(&mut self) -> Option<&mut Archived<$nz>> {
                if self.inner != 0 {
                    let as_nonzero = unsafe {
                        // SAFETY: NonZero types have the same memory layout and bit patterns as
                        // their integer counterparts, regardless of endianness
                        &mut *(&mut self.inner as *mut _ as *mut Archived<$nz>)
                    };
                    Some(as_nonzero)
                } else {
                    None
                }
            }

            #[doc = concat!("Converts from `Pin<&ArchivedOption", stringify!($nz), ">` to `Option<Pin<&Archived<", stringify!($nz), ">>>`.")]
            #[inline]
            pub fn as_pin_ref(self: Pin<&Self>) -> Option<Pin<&Archived<$nz>>> {
                unsafe { Pin::get_ref(self).as_ref().map(|x| Pin::new_unchecked(x)) }
            }

            #[doc = concat!("Converts from `Pin<&mut ArchivedOption", stringify!($nz), ">` to `Option<Pin<&mut Archived<", stringify!($nz), ">>>`.")]
            #[inline]
            pub fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut Archived<$nz>>> {
                unsafe {
                    Pin::get_unchecked_mut(self)
                        .as_mut()
                        .map(|x| Pin::new_unchecked(x))
                }
            }

            /// Returns an iterator over the possibly contained value.
            #[inline]
            pub fn iter(&self) -> Iter<'_, Archived<$nz>> {
                Iter {
                    inner: self.as_ref(),
                }
            }

            /// Returns a mutable iterator over the possibly contained value.
            #[inline]
            pub fn iter_mut(&mut self) -> IterMut<'_, Archived<$nz>> {
                IterMut {
                    inner: self.as_mut(),
                }
            }

            /// Inserts `v` into the option if it is `None`, then returns a mutable
            /// reference to the contained value.
            #[inline]
            pub fn get_or_insert(&mut self, v: $nz) -> &mut Archived<$nz> {
                self.get_or_insert_with(move || v)
            }

            /// Inserts a value computed from `f` into the option if it is `None`, then
            /// returns a mutable reference to the contained value.
            #[inline]
            pub fn get_or_insert_with<F: FnOnce() -> $nz>(&mut self, f: F) -> &mut Archived<$nz> {
                if self.inner == 0 {
                    self.inner = f().get().into();
                }
                unsafe {
                    // SAFETY: self.inner is nonzero
                    &mut *(&mut self.inner as *mut _ as *mut Archived<$nz>)
                }
            }

            /// Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.
            ///
            /// # Safety
            ///
            /// - `pos` must be the position of `out` within the archive
            #[inline]
            pub unsafe fn resolve_from_option(field: Option<$nz>, out: *mut Self) {
                let (_, fo) = out_field!(out.inner);
                if let Some(nz) = field {
                    fo.write(nz.get().into());
                } else {
                    fo.write((0 as $ne).into());
                }
            }
        }

        impl fmt::Debug for $ar {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.as_ref() {
                    Some(inner) => inner.fmt(f),
                    None => f.debug_tuple("None").finish(),
                }
            }
        }

        impl Eq for $ar {}

        impl hash::Hash for $ar {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.as_ref().hash(state)
            }
        }

        impl Ord for $ar {
            #[inline]
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                self.as_ref().cmp(&other.as_ref())
            }
        }

        impl PartialEq for $ar {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.as_ref().eq(&other.as_ref())
            }
        }

        impl PartialOrd for $ar {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

impl_archived_option_nonzero!(ArchivedOptionNonZeroI8, NonZeroI8, i8);
impl_archived_option_nonzero!(ArchivedOptionNonZeroI16, NonZeroI16, i16);
impl_archived_option_nonzero!(ArchivedOptionNonZeroI32, NonZeroI32, i32);
impl_archived_option_nonzero!(ArchivedOptionNonZeroI64, NonZeroI64, i64);
impl_archived_option_nonzero!(ArchivedOptionNonZeroI128, NonZeroI128, i128);

impl_archived_option_nonzero!(ArchivedOptionNonZeroU8, NonZeroU8, u8);
impl_archived_option_nonzero!(ArchivedOptionNonZeroU16, NonZeroU16, u16);
impl_archived_option_nonzero!(ArchivedOptionNonZeroU32, NonZeroU32, u32);
impl_archived_option_nonzero!(ArchivedOptionNonZeroU64, NonZeroU64, u64);
impl_archived_option_nonzero!(ArchivedOptionNonZeroU128, NonZeroU128, u128);

/// An iterator over a reference to the `Some` variant of an `ArchivedOptionNonZero` integer.
///
/// This iterator yields one value if the `ArchivedOptionNonZero` integer is a `Some`, otherwise
/// none.
pub type Iter<'a, T> = crate::option::Iter<'a, T>;

/// An iterator over a mutable reference to the `Some` variant of an `ArchivedOptionNonZero`
/// integer.
///
/// This iterator yields one value if the `ArchivedOptionNonZero` integer is a `Some`, otherwise
/// none.
pub type IterMut<'a, T> = crate::option::IterMut<'a, T>;
