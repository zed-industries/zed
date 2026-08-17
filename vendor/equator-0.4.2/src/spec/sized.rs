use crate::{spec::Wrapper, Cmp, CmpDisplay, CmpError};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SizedWrapper<T>(pub T);
#[repr(transparent)]
pub struct NoSizedWrapper<'a, T: ?Sized>(pub &'a T);

impl<T: ?Sized> Copy for NoSizedWrapper<'_, T> {}
impl<T: ?Sized> Clone for NoSizedWrapper<'_, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: fmt::Debug> fmt::Debug for SizedWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for NoSizedWrapper<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> SizedWrapper<T> {
    #[inline(always)]
    pub fn get(&self) -> &Self {
        self
    }
}
impl<T: ?Sized> NoSizedWrapper<'_, T> {
    #[inline(always)]
    pub fn get(&self) -> &Self {
        self
    }
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: Cmp<Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > Cmp<Wrapper<Lhs>, Wrapper<Rhs>> for CmpSizedWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &Wrapper<Lhs>, rhs: &Wrapper<Rhs>) -> Result<(), Self::Error> {
        self.0
            .test(
                unsafe { &*((&*lhs.0) as *const Lhs::Target as *const Wrapper<Lhs::Target>) },
                unsafe { &*((&*rhs.0) as *const Rhs::Target as *const Wrapper<Rhs::Target>) },
            )
            .map_err(CmpSizedWrapper)
    }
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: CmpError<C, Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > CmpError<CmpSizedWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpSizedWrapper<C>
{
    type Error = CmpSizedWrapper<C::Error>;
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C,
        E: CmpDisplay<C, Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > CmpDisplay<CmpSizedWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpSizedWrapper<E>
{
    fn fmt(
        &self,
        cmp: &CmpSizedWrapper<C>,
        lhs: &Wrapper<Lhs>,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Wrapper<Rhs>,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        self.0.fmt(
            &cmp.0,
            unsafe { &*((&*lhs.0) as *const Lhs::Target as *const Wrapper<Lhs::Target>) },
            lhs_source,
            lhs_debug,
            unsafe { &*((&*rhs.0) as *const Rhs::Target as *const Wrapper<Rhs::Target>) },
            rhs_source,
            rhs_debug,
            f,
        )
    }
}

impl<T> core::ops::Deref for SizedWrapper<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> core::ops::Deref for NoSizedWrapper<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl SizedWrap {
    #[inline(always)]
    pub fn do_wrap<T>(self, value: &T) -> &SizedWrapper<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}
impl NoSizedWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> NoSizedWrapper<'_, T> {
        NoSizedWrapper(value)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpSizedWrapper<T>(pub T);

pub struct SizedWrap;
pub struct NoSizedWrap;

pub trait TrySizedWrap {
    type Wrap;
    fn wrap_sized(&self) -> Self::Wrap;
}

impl<T: Sized> TrySizedWrap for &Wrapper<&T> {
    type Wrap = SizedWrap;

    #[inline]
    fn wrap_sized(&self) -> Self::Wrap {
        SizedWrap
    }
}
impl<T: ?Sized> TrySizedWrap for Wrapper<&T> {
    type Wrap = NoSizedWrap;

    #[inline]
    fn wrap_sized(&self) -> Self::Wrap {
        NoSizedWrap
    }
}
