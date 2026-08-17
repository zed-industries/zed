use crate::{niche::option_box::ArchivedOptionBox, ArchivePointee};
#[cfg(not(feature = "std"))]
use ::alloc::boxed::Box;

impl<T, U> PartialEq<Option<Box<T>>> for ArchivedOptionBox<U>
where
    T: ?Sized,
    U: ArchivePointee + PartialEq<T> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &Option<Box<T>>) -> bool {
        if let Some(self_value) = self.as_deref() {
            if let Some(other_value) = other.as_deref() {
                self_value.eq(other_value)
            } else {
                false
            }
        } else {
            other.is_none()
        }
    }
}

impl<T, U> PartialEq<ArchivedOptionBox<T>> for Option<Box<U>>
where
    T: ArchivePointee + PartialEq<U> + ?Sized,
    U: ?Sized,
{
    #[inline]
    fn eq(&self, other: &ArchivedOptionBox<T>) -> bool {
        other.eq(self)
    }
}
