use super::*;

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
        ParamValue::Borrowed(match self {
            Some(item) => std::mem::transmute_copy(item),
            None => std::mem::zeroed(),
        })
    }
}

impl<T, U> Param<T, InterfaceType> for &U
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
    T: Interface,
    U: Interface,
    U: CanInto<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        if U::QUERY {
            self.cast().map_or(ParamValue::Borrowed(std::mem::zeroed()), |ok| ParamValue::Owned(ok))
        } else {
            ParamValue::Borrowed(std::mem::transmute_copy(self))
        }
    }
}

impl<T> Param<T, CloneType> for &T
where
    T: TypeKind<TypeKind = CloneType> + Clone,
{
    unsafe fn param(self) -> ParamValue<T> {
        ParamValue::Borrowed(std::mem::transmute_copy(self))
    }
}

impl<T, U> Param<T, CopyType> for U
where
    T: TypeKind<TypeKind = CopyType> + Clone,
    U: TypeKind<TypeKind = CopyType> + Clone,
    U: CanInto<T>,
{
    unsafe fn param(self) -> ParamValue<T> {
        ParamValue::Owned(std::mem::transmute_copy(&self))
    }
}

impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl Param<PCWSTR> for &HSTRING {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl Param<PCWSTR> for PWSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.0))
    }
}

impl Param<PCSTR> for PSTR {
    unsafe fn param(self) -> ParamValue<PCSTR> {
        ParamValue::Owned(PCSTR(self.0))
    }
}

#[doc(hidden)]
pub enum ParamValue<T: Type<T>> {
    Owned(T),
    Borrowed(T::Abi),
}

impl<T: Type<T>> ParamValue<T> {
    pub fn abi(&self) -> T::Abi {
        unsafe {
            match self {
                Self::Owned(item) => std::mem::transmute_copy(item),
                Self::Borrowed(borrowed) => std::mem::transmute_copy(borrowed),
            }
        }
    }
}

#[doc(hidden)]
pub trait CanInto<T>: Sized {
    const QUERY: bool = false;
}

impl<T> CanInto<T> for T where T: Clone {}
