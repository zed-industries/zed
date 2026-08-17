//! Provides [`Encode`] for encoding values for the database.

use std::mem;

use crate::database::Database;
use crate::error::BoxDynError;

/// The return type of [Encode::encode].
#[must_use]
pub enum IsNull {
    /// The value is null; no data was written.
    Yes,

    /// The value is not null.
    ///
    /// This does not mean that data was written.
    No,
}

impl IsNull {
    pub fn is_null(&self) -> bool {
        matches!(self, IsNull::Yes)
    }
}

/// Encode a single value to be sent to the database.
pub trait Encode<'q, DB: Database> {
    /// Writes the value of `self` into `buf` in the expected format for the database.
    fn encode(self, buf: &mut <DB as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError>
    where
        Self: Sized,
    {
        self.encode_by_ref(buf)
    }

    /// Writes the value of `self` into `buf` without moving `self`.
    ///
    /// Where possible, make use of `encode` instead as it can take advantage of re-using
    /// memory.
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError>;

    fn produces(&self) -> Option<DB::TypeInfo> {
        // `produces` is inherently a hook to allow database drivers to produce value-dependent
        // type information; if the driver doesn't need this, it can leave this as `None`
        None
    }

    #[inline]
    fn size_hint(&self) -> usize {
        mem::size_of_val(self)
    }
}

impl<'q, T, DB: Database> Encode<'q, DB> for &'_ T
where
    T: Encode<'q, DB>,
{
    #[inline]
    fn encode(self, buf: &mut <DB as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        <T as Encode<DB>>::encode_by_ref(self, buf)
    }

    #[inline]
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <&T as Encode<DB>>::encode(self, buf)
    }

    #[inline]
    fn produces(&self) -> Option<DB::TypeInfo> {
        (**self).produces()
    }

    #[inline]
    fn size_hint(&self) -> usize {
        (**self).size_hint()
    }
}

#[macro_export]
macro_rules! impl_encode_for_option {
    ($DB:ident) => {
        impl<'q, T> $crate::encode::Encode<'q, $DB> for Option<T>
        where
            T: $crate::encode::Encode<'q, $DB> + $crate::types::Type<$DB> + 'q,
        {
            #[inline]
            fn produces(&self) -> Option<<$DB as $crate::database::Database>::TypeInfo> {
                if let Some(v) = self {
                    v.produces()
                } else {
                    T::type_info().into()
                }
            }

            #[inline]
            fn encode(
                self,
                buf: &mut <$DB as $crate::database::Database>::ArgumentBuffer<'q>,
            ) -> Result<$crate::encode::IsNull, $crate::error::BoxDynError> {
                if let Some(v) = self {
                    v.encode(buf)
                } else {
                    Ok($crate::encode::IsNull::Yes)
                }
            }

            #[inline]
            fn encode_by_ref(
                &self,
                buf: &mut <$DB as $crate::database::Database>::ArgumentBuffer<'q>,
            ) -> Result<$crate::encode::IsNull, $crate::error::BoxDynError> {
                if let Some(v) = self {
                    v.encode_by_ref(buf)
                } else {
                    Ok($crate::encode::IsNull::Yes)
                }
            }

            #[inline]
            fn size_hint(&self) -> usize {
                self.as_ref().map_or(0, $crate::encode::Encode::size_hint)
            }
        }
    };
}
