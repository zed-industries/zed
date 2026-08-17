//! Validators that can check archived types.

mod archive;
mod shared;
mod util;

use crate::{
    validation::{
        check_archived_root_with_context, check_archived_value_with_context, ArchiveContext,
        CheckTypeError, SharedContext,
    },
    Archive, Fallible,
};
pub use archive::*;
use bytecheck::CheckBytes;
use core::{
    alloc::{Layout, LayoutError},
    any::TypeId,
    fmt,
};
pub use shared::*;
pub use util::*;

/// The default validator error.
#[derive(Debug)]
pub enum DefaultValidatorError {
    /// An archive validator error occurred.
    ArchiveError(ArchiveError),
    /// A shared validator error occurred.
    SharedError(SharedError),
}

impl fmt::Display for DefaultValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArchiveError(e) => write!(f, "{}", e),
            Self::SharedError(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl Error for DefaultValidatorError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::ArchiveError(e) => Some(e as &dyn Error),
                Self::SharedError(e) => Some(e as &dyn Error),
            }
        }
    }
};

/// The default validator.
#[derive(Debug)]
pub struct DefaultValidator<'a> {
    archive: ArchiveValidator<'a>,
    shared: SharedValidator,
}

impl<'a> DefaultValidator<'a> {
    /// Creates a new validator from a byte range.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            archive: ArchiveValidator::new(bytes),
            shared: SharedValidator::new(),
        }
    }

    /// Create a new validator from a byte range with specific capacity.
    #[inline]
    pub fn with_capacity(bytes: &'a [u8], capacity: usize) -> Self {
        Self {
            archive: ArchiveValidator::new(bytes),
            shared: SharedValidator::with_capacity(capacity),
        }
    }
}

impl<'a> Fallible for DefaultValidator<'a> {
    type Error = DefaultValidatorError;
}

impl<'a> ArchiveContext for DefaultValidator<'a> {
    type PrefixRange = <ArchiveValidator<'a> as ArchiveContext>::PrefixRange;
    type SuffixRange = <ArchiveValidator<'a> as ArchiveContext>::SuffixRange;

    #[inline]
    unsafe fn bounds_check_ptr(
        &mut self,
        base: *const u8,
        offset: isize,
    ) -> Result<*const u8, Self::Error> {
        self.archive
            .bounds_check_ptr(base, offset)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    unsafe fn bounds_check_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error> {
        self.archive
            .bounds_check_layout(data_address, layout)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    unsafe fn bounds_check_subtree_ptr_layout(
        &mut self,
        data_address: *const u8,
        layout: &Layout,
    ) -> Result<(), Self::Error> {
        self.archive
            .bounds_check_subtree_ptr_layout(data_address, layout)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    unsafe fn push_prefix_subtree_range(
        &mut self,
        root: *const u8,
        end: *const u8,
    ) -> Result<PrefixRange, Self::Error> {
        self.archive
            .push_prefix_subtree_range(root, end)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    fn pop_prefix_range(&mut self, range: PrefixRange) -> Result<(), Self::Error> {
        self.archive
            .pop_prefix_range(range)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    unsafe fn push_suffix_subtree_range(
        &mut self,
        start: *const u8,
        root: *const u8,
    ) -> Result<SuffixRange, Self::Error> {
        self.archive
            .push_suffix_subtree_range(start, root)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    fn pop_suffix_range(&mut self, range: SuffixRange) -> Result<(), Self::Error> {
        self.archive
            .pop_suffix_range(range)
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    fn finish(&mut self) -> Result<(), Self::Error> {
        self.archive
            .finish()
            .map_err(DefaultValidatorError::ArchiveError)
    }

    #[inline]
    fn wrap_layout_error(error: LayoutError) -> Self::Error {
        DefaultValidatorError::ArchiveError(ArchiveValidator::wrap_layout_error(error))
    }
}

impl<'a> SharedContext for DefaultValidator<'a> {
    #[inline]
    fn register_shared_ptr(
        &mut self,
        ptr: *const u8,
        type_id: TypeId,
    ) -> Result<bool, Self::Error> {
        self.shared
            .register_shared_ptr(ptr, type_id)
            .map_err(DefaultValidatorError::SharedError)
    }
}

/// Checks the given archive at the given position for an archived version of the given type.
///
/// This is a safe alternative to [`archived_value`](crate::archived_value) for types that implement
/// `CheckBytes`.
///
/// # Examples
/// ```
/// use rkyv::{
///     check_archived_value,
///     ser::{Serializer, serializers::AlignedSerializer},
///     AlignedVec,
///     Archive,
///     Serialize,
/// };
/// use bytecheck::CheckBytes;
///
/// #[derive(Archive, Serialize)]
/// #[archive_attr(derive(CheckBytes))]
/// struct Example {
///     name: String,
///     value: i32,
/// }
///
/// let value = Example {
///     name: "pi".to_string(),
///     value: 31415926,
/// };
///
/// let mut serializer = AlignedSerializer::new(AlignedVec::new());
/// let pos = serializer.serialize_value(&value)
///     .expect("failed to archive test");
/// let buf = serializer.into_inner();
/// let archived = check_archived_value::<Example>(buf.as_ref(), pos).unwrap();
/// ```
#[inline]
pub fn check_archived_value<'a, T: Archive>(
    bytes: &'a [u8],
    pos: usize,
) -> Result<&T::Archived, CheckTypeError<T::Archived, DefaultValidator<'a>>>
where
    T::Archived: CheckBytes<DefaultValidator<'a>>,
{
    let mut validator = DefaultValidator::new(bytes);
    check_archived_value_with_context::<T, DefaultValidator>(bytes, pos, &mut validator)
}

/// Checks the given archive at the given position for an archived version of the given type.
///
/// This is a safe alternative to [`archived_value`](crate::archived_value) for types that implement
/// `CheckBytes`.
///
/// See [`check_archived_value`] for more details.
#[inline]
pub fn check_archived_root<'a, T: Archive>(
    bytes: &'a [u8],
) -> Result<&'a T::Archived, CheckTypeError<T::Archived, DefaultValidator<'a>>>
where
    T::Archived: CheckBytes<DefaultValidator<'a>>,
{
    let mut validator = DefaultValidator::new(bytes);
    check_archived_root_with_context::<T, DefaultValidator>(bytes, &mut validator)
}
