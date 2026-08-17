//! Validation implementation for utility types.

use crate::{collections::util::Entry, validation::ArchiveContext};
use ::bytecheck::CheckBytes;
use ::core::{fmt, ptr};

/// Errors that can occur while checking an archived hash map entry.
#[derive(Debug)]
pub enum ArchivedEntryError<K, V> {
    /// An error occurred while checking the bytes of a key
    KeyCheckError(K),
    /// An error occurred while checking the bytes of a value
    ValueCheckError(V),
}

impl<K: fmt::Display, V: fmt::Display> fmt::Display for ArchivedEntryError<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchivedEntryError::KeyCheckError(e) => write!(f, "key check error: {}", e),
            ArchivedEntryError::ValueCheckError(e) => write!(f, "value check error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl<K: Error + 'static, V: Error + 'static> Error for ArchivedEntryError<K, V> {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                ArchivedEntryError::KeyCheckError(e) => Some(e as &dyn Error),
                ArchivedEntryError::ValueCheckError(e) => Some(e as &dyn Error),
            }
        }
    }
};

impl<K, V, C> CheckBytes<C> for Entry<K, V>
where
    K: CheckBytes<C>,
    V: CheckBytes<C>,
    C: ArchiveContext + ?Sized,
{
    type Error = ArchivedEntryError<K::Error, V::Error>;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        K::check_bytes(ptr::addr_of!((*value).key), context)
            .map_err(ArchivedEntryError::KeyCheckError)?;
        V::check_bytes(ptr::addr_of!((*value).value), context)
            .map_err(ArchivedEntryError::ValueCheckError)?;
        Ok(&*value)
    }
}
