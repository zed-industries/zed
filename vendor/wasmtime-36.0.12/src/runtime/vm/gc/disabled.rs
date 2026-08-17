//! Dummy GC types for when the `gc` cargo feature is disabled.

pub enum VMExternRef {}

pub enum VMStructRef {}

pub enum VMArrayRef {}

pub enum VMExnRef {}

pub struct VMGcObjectData {
    _inner: VMStructRef,
    _phantom: core::marker::PhantomData<[u8]>,
}

impl<'a> From<&'a [u8]> for &'a VMGcObjectData {
    fn from(_: &'a [u8]) -> Self {
        unreachable!()
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut VMGcObjectData {
    fn from(_: &'a mut [u8]) -> Self {
        unreachable!()
    }
}
