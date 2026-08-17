use super::*;
use core::mem::transmute_copy;
use core::mem::zeroed;

/// Provides automatic parameter conversion in cases where the Windows API expects implicit conversion support.
///
/// There is no need to implement this trait. Blanket implementations are provided for all applicable Windows types.
pub trait Param<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    #[doc(hidden)]
    unsafe fn param(self) -> ParamValue<T>;
}

impl<T> Param<T> for Option<&T>
where
    T: Type<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        unsafe {
            ParamValue::Borrowed(match self {
                Some(item) => transmute_copy(item),
                None => zeroed(),
            })
        }
    }
}

impl<T> Param<T> for InterfaceRef<'_, T>
where
    T: Type<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        unsafe { ParamValue::Borrowed(transmute_copy(&self)) }
    }
}

impl<T, U> Param<T, InterfaceType> for &U
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
    T: Interface,
    U: Interface,
    U: imp::CanInto<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        unsafe {
            if U::QUERY {
                self.cast()
                    .map_or(ParamValue::Borrowed(zeroed()), |ok| ParamValue::Owned(ok))
            } else {
                ParamValue::Borrowed(transmute_copy(self))
            }
        }
    }
}

impl<T> Param<T, CloneType> for &T
where
    T: TypeKind<TypeKind = CloneType> + Clone,
{
    unsafe fn param(self) -> ParamValue<T> {
        unsafe { ParamValue::Borrowed(transmute_copy(self)) }
    }
}

impl<T, U> Param<T, CopyType> for U
where
    T: TypeKind<TypeKind = CopyType> + Clone,
    U: TypeKind<TypeKind = CopyType> + Clone,
    U: imp::CanInto<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        unsafe { ParamValue::Owned(transmute_copy(&self)) }
    }
}
