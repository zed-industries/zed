//! Archived versions of FFI types.

use crate::{ser::Serializer, ArchiveUnsized, MetadataResolver, RelPtr, SerializeUnsized};
use core::{
    borrow::Borrow,
    cmp, fmt, hash,
    ops::{Deref, Index, RangeFull},
    pin::Pin,
};
use std::ffi::CStr;

/// An archived [`CString`](std::ffi::CString).
///
/// Uses a [`RelPtr`] to a `CStr` under the hood.
#[repr(transparent)]
pub struct ArchivedCString(RelPtr<CStr>);

impl ArchivedCString {
    /// Returns the contents of this CString as a slice of bytes.
    ///
    /// The returned slice does **not** contain the trailing nul terminator, and it is guaranteed to
    /// not have any interior nul bytes. If you need the nul terminator, use
    /// [`as_bytes_with_nul`][ArchivedCString::as_bytes_with_nul()] instead.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    /// Equivalent to [`as_bytes`][ArchivedCString::as_bytes()] except that the returned slice
    /// includes the trailing nul terminator.
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_c_str().to_bytes_with_nul()
    }

    /// Extracts a `CStr` slice containing the entire string.
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { &*self.0.as_ptr() }
    }

    /// Extracts a pinned mutable `CStr` slice containing the entire string.
    #[inline]
    pub fn pin_mut_c_str(self: Pin<&mut Self>) -> Pin<&mut CStr> {
        unsafe { self.map_unchecked_mut(|s| &mut *s.0.as_mut_ptr()) }
    }

    /// Resolves an archived C string from the given C string and parameters.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing a C string
    #[inline]
    pub unsafe fn resolve_from_c_str(
        c_str: &CStr,
        pos: usize,
        resolver: CStringResolver,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.0);
        // metadata_resolver is guaranteed to be (), but it's better to be explicit about it
        #[allow(clippy::unit_arg)]
        c_str.resolve_unsized(pos + fp, resolver.pos, resolver.metadata_resolver, fo);
    }

    /// Serializes a C string.
    #[inline]
    pub fn serialize_from_c_str<S: Serializer + ?Sized>(
        c_str: &CStr,
        serializer: &mut S,
    ) -> Result<CStringResolver, S::Error> {
        Ok(CStringResolver {
            pos: c_str.serialize_unsized(serializer)?,
            metadata_resolver: c_str.serialize_metadata(serializer)?,
        })
    }
}

impl AsRef<CStr> for ArchivedCString {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Borrow<CStr> for ArchivedCString {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

impl fmt::Debug for ArchivedCString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_c_str().fmt(f)
    }
}

impl Deref for ArchivedCString {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_c_str()
    }
}

impl Eq for ArchivedCString {}

impl hash::Hash for ArchivedCString {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes_with_nul().hash(state);
    }
}

impl Index<RangeFull> for ArchivedCString {
    type Output = CStr;

    #[inline]
    fn index(&self, _: RangeFull) -> &Self::Output {
        self.as_c_str()
    }
}

impl Ord for ArchivedCString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialEq for ArchivedCString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<&CStr> for ArchivedCString {
    #[inline]
    fn eq(&self, other: &&CStr) -> bool {
        PartialEq::eq(self.as_c_str(), other)
    }
}

impl PartialEq<ArchivedCString> for &CStr {
    #[inline]
    fn eq(&self, other: &ArchivedCString) -> bool {
        PartialEq::eq(other.as_c_str(), self)
    }
}

impl PartialOrd for ArchivedCString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The resolver for `CString`.
pub struct CStringResolver {
    pos: usize,
    metadata_resolver: MetadataResolver<CStr>,
}

#[cfg(feature = "validation")]
const _: () = {
    use crate::validation::{
        owned::{CheckOwnedPointerError, OwnedPointerError},
        ArchiveContext,
    };
    use bytecheck::{CheckBytes, Error};

    impl<C: ArchiveContext + ?Sized> CheckBytes<C> for ArchivedCString
    where
        C::Error: Error,
    {
        type Error = CheckOwnedPointerError<CStr, C>;

        #[inline]
        unsafe fn check_bytes<'a>(
            value: *const Self,
            context: &mut C,
        ) -> Result<&'a Self, Self::Error> {
            let rel_ptr = RelPtr::<CStr>::manual_check_bytes(value.cast(), context)
                .map_err(OwnedPointerError::PointerCheckBytesError)?;
            let ptr = context
                .check_subtree_rel_ptr(rel_ptr)
                .map_err(OwnedPointerError::ContextError)?;

            let range = context
                .push_prefix_subtree(ptr)
                .map_err(OwnedPointerError::ContextError)?;
            CStr::check_bytes(ptr, context).map_err(OwnedPointerError::ValueCheckBytesError)?;
            context
                .pop_prefix_range(range)
                .map_err(OwnedPointerError::ContextError)?;

            Ok(&*value)
        }
    }
};
