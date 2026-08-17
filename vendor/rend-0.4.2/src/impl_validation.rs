macro_rules! impl_validation {
    (@always $endian:ident<$ne:ty>) => {
        impl<C: ?Sized> CheckBytes<C> for $endian<$ne> {
            type Error = Infallible;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                _: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                Ok(&*value)
            }
        }
    };
    (@signed_int $endian:ident<$ne:ty>) => {
        impl_validation!(@always $endian<$ne>);
    };
    (@unsigned_int $endian:ident<$ne:ty>) => {
        impl_validation!(@always $endian<$ne>);
    };
    (@float $endian:ident<$ne:ty>) => {
        impl_validation!(@always $endian<$ne>);
    };
    (@char $endian:ident<$ne:ty>) => {
        impl<C: ?Sized> CheckBytes<C> for $endian<$ne> {
            type Error = CharCheckError;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                context: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                let as_u32 = &*$endian::<u32>::check_bytes(value.cast(), context)?;
                let c = as_u32.value();
                <$ne>::from_u32(c).ok_or_else(|| CharCheckError { invalid_value: c })?;
                Ok(&*value)
            }
        }
    };
    (@nonzero $endian:ident<$ne:ty> = $prim:ty) => {
        impl<C: ?Sized> CheckBytes<C> for $endian<$ne> {
            type Error = NonZeroCheckError;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                context: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                if $endian::<$prim>::check_bytes(value.cast(), context)?.value() == 0 {
                    Err(NonZeroCheckError::IsZero)
                } else {
                    Ok(&*value)
                }
            }
        }
    };
    (@atomic $endian:ident<$ne:ty> = $prim:ty) => {
        impl_validation!(@always $endian<$ne>);
    };
}
