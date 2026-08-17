//! Wrapper that allows changing the generic type of a `PhantomData<NT>`
//!
//! Copied from <https://github.com/rkyv/rkyv_contrib> (MIT-Apache2 licences) which isn’t published yet.

use rkyv::{
    Fallible,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
};
use std::marker::PhantomData;

/// A wrapper that allows for changing the generic type of a `PhantomData<NT>`.
pub struct CustomPhantom<NT: ?Sized> {
    _data: PhantomData<*const NT>,
}

impl<OT: ?Sized, NT: ?Sized> ArchiveWith<PhantomData<OT>> for CustomPhantom<NT> {
    type Archived = PhantomData<NT>;
    type Resolver = ();

    #[inline]
    unsafe fn resolve_with(
        _: &PhantomData<OT>,
        _: usize,
        _: Self::Resolver,
        _: *mut Self::Archived,
    ) {
    }
}

impl<OT: ?Sized, NT: ?Sized, S: Fallible + ?Sized> SerializeWith<PhantomData<OT>, S>
    for CustomPhantom<NT>
{
    #[inline]
    fn serialize_with(_: &PhantomData<OT>, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<OT: ?Sized, NT: ?Sized, D: Fallible + ?Sized>
    DeserializeWith<PhantomData<NT>, PhantomData<OT>, D> for CustomPhantom<NT>
{
    #[inline]
    fn deserialize_with(_: &PhantomData<NT>, _: &mut D) -> Result<PhantomData<OT>, D::Error> {
        Ok(PhantomData)
    }
}
