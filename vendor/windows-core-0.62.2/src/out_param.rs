use super::*;
use core::mem::{take, transmute_copy, zeroed};

/// Provides automatic parameter conversion in cases where the Windows API expects implicit conversion support.
///
/// This is a mutable version of [Param] meant to support out parameters.
/// There is no need to implement this trait. Blanket implementations are provided for all applicable Windows types.
pub trait OutParam<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    #[doc(hidden)]
    unsafe fn borrow_mut(&self) -> OutRef<'_, T>;
}

impl<T> OutParam<T, CloneType> for &mut T
where
    T: TypeKind<TypeKind = CloneType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            let this: &mut T = transmute_copy(self);
            take(this);
            transmute_copy(self)
        }
    }
}

impl<T> OutParam<T, CopyType> for &mut T
where
    T: TypeKind<TypeKind = CopyType> + Clone + Default,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe { transmute_copy(self) }
    }
}

impl<T> OutParam<T, InterfaceType> for &mut Option<T>
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            let this: &mut Option<T> = transmute_copy(self);
            take(this);
            transmute_copy(self)
        }
    }
}

impl<T> OutParam<T> for Option<&mut T>
where
    T: Type<T>,
{
    unsafe fn borrow_mut(&self) -> OutRef<'_, T> {
        unsafe {
            match self {
                Some(this) => transmute_copy(this),
                None => zeroed(),
            }
        }
    }
}
