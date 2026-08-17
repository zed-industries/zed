//!
//! Trait object based compile-time dispatch.
//!

use crate::traits::*;

#[derive(Copy, Clone, Default)]
pub struct Empty;

pub type CloneFn = unsafe fn(src: *const u8, dst: *mut u8, len: usize);
unsafe fn clone_fn<T: Clone>(src: *const u8, dst: *mut u8, len: usize){
    let src = src as *const T;
    let dst = dst as *mut T;
    for i in 0..len {
        let dst = dst.add(i);
        let src = src.add(i);
        dst.write((*src).clone());
    }
}
fn nop_fn(_: *const u8, _: *mut u8, _: usize){}


pub trait CloneFnTrait<Traits: ?Sized>{
    const CLONE_FN: CloneFn = nop_fn;
}
impl<T: Clone> CloneFnTrait<dyn Cloneable> for T{
    const CLONE_FN: CloneFn = clone_fn::<T>;
}
impl<T: Clone> CloneFnTrait<dyn Cloneable+Send> for T{
    const CLONE_FN: CloneFn = clone_fn::<T>;
}
impl<T: Clone> CloneFnTrait<dyn Cloneable+Sync> for T{
    const CLONE_FN: CloneFn = clone_fn::<T>;
}
impl<T: Clone> CloneFnTrait<dyn Cloneable+Send+Sync> for T{
    const CLONE_FN: CloneFn = clone_fn::<T>;
}
impl<T> CloneFnTrait<dyn None> for T{}
impl<T> CloneFnTrait<dyn Send> for T{}
impl<T> CloneFnTrait<dyn Sync> for T{}
impl<T> CloneFnTrait<dyn Send+Sync> for T{}


/// This all just to replace AnyVec's clone function pointer with ZST,
/// when non-Cloneable.
pub trait CloneType{
    type Type: Copy;
    fn new(f: CloneFn) -> Self::Type;
    fn get(f: Self::Type) -> CloneFn;
}
macro_rules! impl_clone_type_empty {
    ($t:ty) => {
        impl CloneType for $t {
            type Type = Empty;
            fn new(_: CloneFn) -> Self::Type{ Empty }
            fn get(_: Self::Type) -> CloneFn{ nop_fn }
        }
    }
}
macro_rules! impl_clone_type_fn {
    ($t:ty) => {
        impl CloneType for $t {
            type Type = CloneFn;
            fn new(f: CloneFn) -> Self::Type{ f }
            fn get(f: Self::Type) -> CloneFn{ f as CloneFn }
        }
    }
}
impl_clone_type_empty!(dyn None);
impl_clone_type_empty!(dyn Sync);
impl_clone_type_empty!(dyn Send);
impl_clone_type_empty!(dyn Send + Sync);
impl_clone_type_fn!(dyn Cloneable);
impl_clone_type_fn!(dyn Cloneable + Send);
impl_clone_type_fn!(dyn Cloneable + Sync);
impl_clone_type_fn!(dyn Cloneable + Send + Sync);
