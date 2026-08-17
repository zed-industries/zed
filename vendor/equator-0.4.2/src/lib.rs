#![no_std]

use core::fmt;

#[doc(hidden)]
pub use equator_macro as imp;

#[macro_export]
macro_rules! assert {
    ($($tokens: tt)*) => {
        $crate::imp::assert!($crate, $($tokens)*)
    };
}

#[macro_export]
macro_rules! debug_assert {
    ($($tokens: tt)*) => {
        if cfg!(debug_assertions) {
            $crate::imp::assert!($crate, $($tokens)*)
        }
    };
}

#[doc(hidden)]
pub mod decompose;
#[doc(hidden)]
pub mod spec;
#[doc(hidden)]
pub mod structures;
#[doc(hidden)]
pub mod traits;

#[doc(hidden)]
pub mod expr {
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct CmpExpr<Cmp, Lhs, Rhs> {
        pub cmp: Cmp,
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct CustomCmpExpr<Cmp, Lhs, Rhs> {
        pub cmp: Cmp,
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct AndExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct OrExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }
}

pub trait CmpError<C, Lhs: ?Sized, Rhs: ?Sized>: Sized {
    type Error;
}

pub trait CmpDisplay<C, Lhs: ?Sized, Rhs: ?Sized> {
    fn fmt(
        &self,
        cmp: &C,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result;
}

pub trait Cmp<Lhs: ?Sized, Rhs: ?Sized>: CmpError<Self, Lhs, Rhs> {
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), Self::Error>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Eq;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ne;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Le;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ge;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lt;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EqError;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NeError;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeError;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeError;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LtError;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GtError;

fn display_cmp_impl(
    cmp: &str,
    lhs: &dyn fmt::Debug,
    lhs_source: &str,
    rhs: &dyn fmt::Debug,
    rhs_source: &str,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    write!(f, "Assertion failed: {lhs_source} {cmp} {rhs_source}\n")?;
    write!(f, "- {lhs_source} = {lhs:#?}\n")?;
    write!(f, "- {rhs_source} = {rhs:#?}")
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Eq, Lhs, Rhs> for Eq {
    type Error = EqError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Eq, Lhs, Rhs> for EqError {
    fn fmt(
        &self,
        cmp: &Eq,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl("==", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Ne, Lhs, Rhs> for Ne {
    type Error = NeError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Ne, Lhs, Rhs> for NeError {
    fn fmt(
        &self,
        cmp: &Ne,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl("!=", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Lt, Lhs, Rhs> for Lt {
    type Error = LtError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Lt, Lhs, Rhs> for LtError {
    fn fmt(
        &self,
        cmp: &Lt,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl("<", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Gt, Lhs, Rhs> for Gt {
    type Error = GtError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Gt, Lhs, Rhs> for GtError {
    fn fmt(
        &self,
        cmp: &Gt,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl(">", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Le, Lhs, Rhs> for Le {
    type Error = LeError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Le, Lhs, Rhs> for LeError {
    fn fmt(
        &self,
        cmp: &Le,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl("<=", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized> CmpError<Ge, Lhs, Rhs> for Ge {
    type Error = GeError;
}
impl<Lhs: ?Sized, Rhs: ?Sized> CmpDisplay<Ge, Lhs, Rhs> for GeError {
    fn fmt(
        &self,
        cmp: &Ge,
        lhs: &Lhs,
        lhs_source: &str,
        lhs_debug: &dyn fmt::Debug,
        rhs: &Rhs,
        rhs_source: &str,
        rhs_debug: &dyn fmt::Debug,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        _ = (lhs, rhs, cmp);
        display_cmp_impl(">=", lhs_debug, lhs_source, rhs_debug, rhs_source, f)
    }
}

impl<Rhs: ?Sized, Lhs: ?Sized + PartialEq<Rhs>> Cmp<Lhs, Rhs> for Eq {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), EqError> {
        if *lhs == *rhs {
            Ok(())
        } else {
            Err(EqError)
        }
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialEq<Rhs>> Cmp<Lhs, Rhs> for Ne {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), NeError> {
        if *lhs != *rhs {
            Ok(())
        } else {
            Err(NeError)
        }
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Le {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), LeError> {
        if *lhs <= *rhs {
            Ok(())
        } else {
            Err(LeError)
        }
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Ge {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), GeError> {
        if *lhs >= *rhs {
            Ok(())
        } else {
            Err(GeError)
        }
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Lt {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), LtError> {
        if *lhs < *rhs {
            Ok(())
        } else {
            Err(LtError)
        }
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Gt {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> Result<(), GtError> {
        if *lhs > *rhs {
            Ok(())
        } else {
            Err(GtError)
        }
    }
}

#[doc(hidden)]
pub struct CmpExpr;
#[doc(hidden)]
pub struct CustomCmpExpr<E>(pub core::marker::PhantomData<E>);
#[doc(hidden)]
pub struct AndExpr<L, R>(pub L, pub R);
#[doc(hidden)]
pub struct OrExpr<L, R>(pub L, pub R);

#[doc(hidden)]
pub struct Message<'a>(pub core::fmt::Arguments<'a>);
#[doc(hidden)]
pub struct NoMessage;

impl From<NoMessage> for core::fmt::Arguments<'_> {
    fn from(_: NoMessage) -> Self {
        core::format_args!("")
    }
}

impl<'a> From<Message<'a>> for core::fmt::Arguments<'a> {
    fn from(t: Message<'a>) -> Self {
        t.0
    }
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[track_caller]
pub fn panic_failed_assert<'a, M: Into<core::fmt::Arguments<'a>>, D: decompose::Recompose>(
    __marker: core::marker::PhantomData<D>,
    debug_lhs: D::DebugLhs,
    debug_rhs: D::DebugRhs,
    debug_cmp: D::DebugCmp,
    source: &'static structures::WithSource<D::Source, &'static D::VTable>,
    message: M,
) -> ! {
    panic!(
        "{:#?}",
        structures::DebugMessage::<D> {
            source,
            debug_lhs,
            debug_rhs,
            debug_cmp,
            message: message.into(),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_assert() {
        assert!(false);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_debug_assert() {
        debug_assert!(false);
    }
}
