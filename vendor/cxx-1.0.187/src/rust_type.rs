#![allow(missing_docs)]

use crate::extern_type::ExternType;
use crate::kind::Trivial;
use core::marker::{PhantomData, Unpin};
use core::ops::Deref;

pub unsafe trait RustType {}
pub unsafe trait ImplBox {}
pub unsafe trait ImplVec {}

// Opaque Rust types are required to be Unpin.
pub fn require_unpin<T: ?Sized + Unpin>() {}

pub fn require_box<T: ImplBox>() {}
pub fn require_vec<T: ImplVec>() {}

pub struct With<T: ?Sized>(PhantomData<T>);
pub struct Without;

pub const fn with<T: ?Sized>() -> With<T> {
    With(PhantomData)
}

impl<T: ?Sized + RustType> With<T> {
    #[allow(clippy::unused_self)]
    pub const fn check_slice<U>(&self) {}
}

impl<T: ?Sized> Deref for With<T> {
    type Target = Without;
    fn deref(&self) -> &Self::Target {
        &Without
    }
}

pub trait SliceOfExternType {
    type Kind;
}
impl<T: ExternType> SliceOfExternType for &[T] {
    type Kind = T::Kind;
}
impl<T: ExternType> SliceOfExternType for &mut [T] {
    type Kind = T::Kind;
}

impl Without {
    #[allow(clippy::unused_self)]
    pub const fn check_slice<U: SliceOfExternType<Kind = Trivial>>(&self) {}
}
