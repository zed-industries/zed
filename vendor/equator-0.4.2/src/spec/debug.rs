use crate::{spec::Wrapper, Cmp, CmpDisplay, CmpError};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DebugWrapper<T: ?Sized>(pub T);
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NoDebugWrapper<T: ?Sized>(pub T);

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: CmpError<C, Lhs::Target, Rhs::Target>,
    > CmpError<CmpDebugWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpDebugWrapper<C>
{
    type Error = CmpDebugWrapper<C::Error>;
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C,
        E: CmpDisplay<C, Lhs::Target, Rhs::Target>,
    > CmpDisplay<CmpDebugWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpDebugWrapper<E>
{
    fn fmt(
        &self,
        cmp: &CmpDebugWrapper<C>,
        lhs: &Wrapper<Lhs>,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Wrapper<Rhs>,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        self.0.fmt(
            &cmp.0, &*lhs.0, lhs_source, lhs_debug, &*rhs.0, rhs_source, rhs_debug, f,
        )
    }
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: Cmp<Lhs::Target, Rhs::Target>,
    > Cmp<Wrapper<Lhs>, Wrapper<Rhs>> for CmpDebugWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &Wrapper<Lhs>, rhs: &Wrapper<Rhs>) -> Result<(), Self::Error> {
        self.0.test(&*lhs.0, &*rhs.0).map_err(CmpDebugWrapper)
    }
}

impl<T: ?Sized> core::ops::Deref for DebugWrapper<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> core::ops::Deref for NoDebugWrapper<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for DebugWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: ?Sized> fmt::Debug for NoDebugWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<object of type \"{}\" at address {:?}>",
            core::any::type_name::<T>(),
            self as *const _ as *const ()
        )
    }
}

pub struct DebugWrap;
pub struct NoDebugWrap;

impl DebugWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> &DebugWrapper<T> {
        unsafe { &*(value as *const T as *const DebugWrapper<T>) }
    }
}
impl NoDebugWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> &NoDebugWrapper<T> {
        unsafe { &*(value as *const T as *const NoDebugWrapper<T>) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpDebugWrapper<T>(pub T);

pub trait TryDebugWrap {
    type Wrap;
    fn wrap_debug(&self) -> Self::Wrap;
}

impl<T: fmt::Debug + ?Sized> TryDebugWrap for &Wrapper<T> {
    type Wrap = DebugWrap;

    #[inline]
    fn wrap_debug(&self) -> Self::Wrap {
        DebugWrap
    }
}
impl<T: ?Sized> TryDebugWrap for Wrapper<T> {
    type Wrap = NoDebugWrap;

    #[inline]
    fn wrap_debug(&self) -> Self::Wrap {
        NoDebugWrap
    }
}
