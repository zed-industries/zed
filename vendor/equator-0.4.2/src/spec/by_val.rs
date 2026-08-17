use crate::{
    decompose::PtrToDeref,
    spec::{debug::CmpDebugWrapper, sized::CmpSizedWrapper, Wrapper},
    Cmp, CmpDisplay, CmpError,
};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ByVal<T>(pub T);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ByRef<T>(pub T);

impl<T: fmt::Debug> fmt::Debug for ByVal<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Debug> fmt::Debug for ByRef<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> ByRef<T> {
    #[inline(always)]
    pub fn get_ptr(&self) -> *const () {
        self as *const _ as *const ()
    }
}

impl<T: Copy> ByVal<T> {
    const FIT_IN_PTR: bool = core::mem::size_of::<T>() <= core::mem::size_of::<*const ()>()
        && core::mem::align_of::<T>() <= core::mem::align_of::<*const ()>();

    #[inline(always)]
    pub fn get_ptr(&self) -> *const () {
        if Self::FIT_IN_PTR {
            let mut out = core::ptr::null::<()>();
            unsafe {
                *((&mut out) as *mut *const () as *mut T) = self.0;
            };
            out
        } else {
            self as *const _ as *const ()
        }
    }
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: Cmp<Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > Cmp<Wrapper<Lhs>, Wrapper<Rhs>> for CmpByValWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &Wrapper<Lhs>, rhs: &Wrapper<Rhs>) -> Result<(), Self::Error> {
        self.0
            .test(
                unsafe { &*((&*lhs.0) as *const Lhs::Target as *const Wrapper<Lhs::Target>) },
                unsafe { &*((&*rhs.0) as *const Rhs::Target as *const Wrapper<Rhs::Target>) },
            )
            .map_err(CmpByValWrapper)
    }
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C: CmpError<C, Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > CmpError<CmpByValWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpByValWrapper<C>
{
    type Error = CmpByValWrapper<C::Error>;
}

impl<
        Lhs: ?Sized + core::ops::Deref,
        Rhs: ?Sized + core::ops::Deref,
        C,
        E: CmpDisplay<C, Wrapper<Lhs::Target>, Wrapper<Rhs::Target>>,
    > CmpDisplay<CmpByValWrapper<C>, Wrapper<Lhs>, Wrapper<Rhs>> for CmpByValWrapper<E>
{
    fn fmt(
        &self,
        cmp: &CmpByValWrapper<C>,
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

impl<T> core::ops::Deref for ByVal<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::Deref for ByRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ByValWrap {
    #[inline(always)]
    pub fn do_wrap<T: Copy>(self, value: &T) -> &ByVal<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}
impl ByRefWrap {
    #[inline(always)]
    pub fn do_wrap<T>(self, value: &T) -> &ByRef<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}

impl<'a, C> CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<&'a C>>> {
    #[inline(always)]
    pub fn __wrap_ref(self) -> &'a CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<C>>> {
        unsafe {
            &*(self.0 .0 .0 as *const C
                as *const CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<C>>>)
        }
    }
}
impl<T: Copy> TryByValWrap for &Wrapper<&T> {
    type Wrap = ByValWrap;

    #[inline]
    fn wrap_by_val(&self) -> Self::Wrap {
        ByValWrap
    }
}
impl<T> TryByValWrap for Wrapper<T> {
    type Wrap = ByRefWrap;

    #[inline]
    fn wrap_by_val(&self) -> Self::Wrap {
        ByRefWrap
    }
}

pub struct ByValWrap;
pub struct ByRefWrap;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpByValWrapper<T: ?Sized>(pub T);

pub trait TryByValWrap {
    type Wrap;
    fn wrap_by_val(&self) -> Self::Wrap;
}

pub(crate) trait DerefVTable {
    const VTABLE: unsafe fn(*const *const ()) -> *const ();
}

unsafe fn no_deref(ptr: *const *const ()) -> *const () {
    ptr as *const ()
}
unsafe fn deref(ptr: *const *const ()) -> *const () {
    *ptr
}

impl<T: Copy> DerefVTable for ByVal<T> {
    const VTABLE: PtrToDeref = {
        if ByVal::<T>::FIT_IN_PTR {
            no_deref
        } else {
            deref
        }
    };
}
impl<T> DerefVTable for ByRef<T> {
    const VTABLE: PtrToDeref = { deref };
}
