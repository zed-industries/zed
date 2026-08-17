//! Validation implementations for relative pointers

use crate::{
    rel_ptr::{Offset, RawRelPtr, RelPtr},
    ArchivePointee, Fallible,
};
use bytecheck::CheckBytes;
use core::{
    convert::Infallible,
    marker::{PhantomData, PhantomPinned},
    ptr,
};

impl<O: Offset> RawRelPtr<O> {
    /// Checks the bytes of the given raw relative pointer.
    ///
    /// This is done rather than implementing `CheckBytes` to force users to manually write their
    /// `CheckBytes` implementation since they need to also provide the ownership model of their
    /// memory.
    ///
    /// # Safety
    ///
    /// The given pointer must be aligned and point to enough bytes to represent a `RawRelPtr`.
    #[inline]
    pub unsafe fn manual_check_bytes<'a, C: Fallible + ?Sized>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Infallible>
    where
        O: CheckBytes<C>,
    {
        O::check_bytes(ptr::addr_of!((*value).offset), context).unwrap();
        PhantomPinned::check_bytes(ptr::addr_of!((*value)._phantom), context).unwrap();
        Ok(&*value)
    }
}

impl<T: ArchivePointee + ?Sized, O: Offset> RelPtr<T, O> {
    /// Checks the bytes of the given relative pointer.
    ///
    /// This is done rather than implementing `CheckBytes` to force users to manually write their
    /// `CheckBytes` implementation since they need to also provide the ownership model of their
    /// memory.
    ///
    /// # Safety
    ///
    /// The given pointer must be aligned and point to enough bytes to represent a `RelPtr<T>`.
    #[inline]
    pub unsafe fn manual_check_bytes<'a, C: Fallible + ?Sized>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, <T::ArchivedMetadata as CheckBytes<C>>::Error>
    where
        O: CheckBytes<C>,
        T::ArchivedMetadata: CheckBytes<C>,
    {
        RawRelPtr::manual_check_bytes(ptr::addr_of!((*value).raw_ptr), context).unwrap();
        T::ArchivedMetadata::check_bytes(ptr::addr_of!((*value).metadata), context)?;
        PhantomData::<T>::check_bytes(ptr::addr_of!((*value)._phantom), context).unwrap();
        Ok(&*value)
    }
}
