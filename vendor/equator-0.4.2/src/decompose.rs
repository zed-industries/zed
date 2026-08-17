use crate::{
    expr,
    structures::{DebugMessage, DebugMessageImpl},
    traits::Eval,
    CmpDisplay,
};
use core::fmt;

pub type PtrToDeref = unsafe fn(*const *const ()) -> *const ();
pub type PtrToCmp = unsafe fn(out: *mut (), cmp: *const (), lhs: *const (), rhs: *const ());
pub type PtrToDebug = unsafe fn(*const ()) -> &'static dyn fmt::Debug;
pub type PtrToDisplay =
    unsafe fn(*const ()) -> &'static dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>;

pub trait Decompose {
    type Decomposed: Recompose;
}

pub trait Recompose: Sized {
    type Result: Eval;
    type Source;
    type VTable: 'static;
    type DebugLhs: Copy + fmt::Debug;
    type DebugRhs: Copy + fmt::Debug;
    type DebugCmp: Copy + fmt::Debug;

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result;
    fn eval_impl(
        debug_lhs: &Self::DebugLhs,
        debug_rhs: &Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result;

    fn debug_final(full: &DebugMessage<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let result = &Self::eval_impl(
            &full.debug_lhs,
            &full.debug_rhs,
            full.debug_cmp,
            &full.source.vtable,
        );

        let message = full.message;
        let inner = DebugMessageImpl::<'_, Self> {
            result,
            source: &full.source.source,
            debug_lhs: &full.debug_lhs,
            debug_rhs: &full.debug_rhs,
            debug_cmp: full.debug_cmp,
            vtable: full.source.vtable,
        };
        write!(
            f,
            "Assertion failed at {}:{}:{}\n",
            full.source.file, full.source.line, full.source.col
        )?;
        if message.as_str() != Some("") {
            write!(f, "{message:#?}\n")?;
        }
        Self::debug_impl(&inner, f)
    }
}

impl Recompose for bool {
    type Result = Result<(), ()>;
    type Source = &'static str;
    type VTable = ();
    type DebugLhs = ();
    type DebugRhs = ();
    type DebugCmp = bool;

    fn eval_impl(
        _: &Self::DebugLhs,
        _: &Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        _: &Self::VTable,
    ) -> Self::Result {
        if debug_cmp {
            Ok(())
        } else {
            Err(())
        }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let source = *message.source;
        let result = message.result.is_ok();
        write!(f, "Assertion failed: {source}\n")?;
        write!(f, "- {source} = {result:#?}")
    }
}

impl Recompose for crate::CmpExpr {
    type Result = Result<(), ()>;
    type Source = expr::CmpExpr<(), &'static str, &'static str>;
    type VTable =
        expr::CmpExpr<(PtrToDisplay, PtrToCmp), (PtrToDebug, PtrToDeref), (PtrToDebug, PtrToDeref)>;
    type DebugLhs = *const ();
    type DebugRhs = *const ();
    type DebugCmp = ();

    fn eval_impl(
        debug_lhs: &Self::DebugLhs,
        debug_rhs: &Self::DebugRhs,
        _: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let debug_lhs = unsafe { (vtable.lhs.1)(debug_lhs) };
        let debug_rhs = unsafe { (vtable.rhs.1)(debug_rhs) };
        let mut result = core::mem::MaybeUninit::<Self::Result>::uninit();
        unsafe {
            (vtable.cmp.1)(
                (&mut result) as *mut core::mem::MaybeUninit<Self::Result> as *mut (),
                core::ptr::NonNull::<()>::dangling().as_ptr(),
                debug_lhs,
                debug_rhs,
            )
        }
        unsafe { result.assume_init() }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs_source = message.source.lhs;
        let rhs_source = message.source.rhs;
        let debug_lhs = unsafe { (message.vtable.lhs.1)(message.debug_lhs) };
        let debug_rhs = unsafe { (message.vtable.rhs.1)(message.debug_rhs) };

        let lhs = unsafe { (message.vtable.lhs.0)(debug_lhs) };
        let rhs = unsafe { (message.vtable.rhs.0)(debug_rhs) };

        let err =
            unsafe { (message.vtable.cmp.0)(message.result.as_ref().unwrap_err() as *const ()) };
        err.fmt(
            &(core::ptr::NonNull::<()>::dangling().as_ptr() as *const ()),
            lhs,
            lhs_source,
            lhs,
            rhs,
            rhs_source,
            rhs,
            f,
        )
    }
}

impl<E> Recompose for crate::CustomCmpExpr<E> {
    type Result = Result<(), E>;
    type Source = expr::CustomCmpExpr<(), &'static str, &'static str>;
    type VTable = expr::CustomCmpExpr<
        (PtrToDisplay, PtrToCmp),
        (PtrToDebug, PtrToDeref),
        (PtrToDebug, PtrToDeref),
    >;
    type DebugLhs = *const ();
    type DebugRhs = *const ();
    type DebugCmp = *const ();

    fn eval_impl(
        debug_lhs: &Self::DebugLhs,
        debug_rhs: &Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let debug_lhs = unsafe { (vtable.lhs.1)(debug_lhs) };
        let debug_rhs = unsafe { (vtable.rhs.1)(debug_rhs) };

        let mut result = core::mem::MaybeUninit::<Self::Result>::uninit();
        unsafe {
            (vtable.cmp.1)(
                (&mut result) as *mut core::mem::MaybeUninit<Self::Result> as *mut (),
                debug_cmp,
                debug_lhs,
                debug_rhs,
            )
        }
        unsafe { result.assume_init() }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs_source = message.source.lhs;
        let rhs_source = message.source.rhs;
        let debug_lhs = unsafe { (message.vtable.lhs.1)(message.debug_lhs) };
        let debug_rhs = unsafe { (message.vtable.rhs.1)(message.debug_rhs) };

        let lhs = unsafe { (message.vtable.lhs.0)(debug_lhs) };
        let rhs = unsafe { (message.vtable.rhs.0)(debug_rhs) };

        let err = unsafe {
            (message.vtable.cmp.0)(message.result.as_ref().unwrap_err() as *const E as *const ())
        };
        err.fmt(
            &message.debug_cmp,
            lhs,
            lhs_source,
            lhs,
            rhs,
            rhs_source,
            rhs,
            f,
        )
    }
}

impl<L: Recompose, R: Recompose> Recompose for crate::AndExpr<L, R> {
    type Result = expr::AndExpr<L::Result, R::Result>;
    type Source = expr::AndExpr<L::Source, R::Source>;
    type VTable = expr::AndExpr<&'static L::VTable, &'static R::VTable>;
    type DebugCmp = expr::AndExpr<L::DebugCmp, R::DebugCmp>;
    type DebugLhs = expr::AndExpr<L::DebugLhs, R::DebugLhs>;
    type DebugRhs = expr::AndExpr<L::DebugRhs, R::DebugRhs>;

    fn eval_impl(
        debug_lhs: &Self::DebugLhs,
        debug_rhs: &Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let lhs = L::eval_impl(&debug_lhs.lhs, &debug_rhs.lhs, debug_cmp.lhs, vtable.lhs);
        let rhs = R::eval_impl(&debug_lhs.rhs, &debug_rhs.rhs, debug_cmp.rhs, vtable.rhs);
        expr::AndExpr { lhs, rhs }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = DebugMessageImpl::<'_, L> {
            result: &message.result.lhs,
            source: &message.source.lhs,
            vtable: message.vtable.lhs,
            debug_lhs: &message.debug_lhs.lhs,
            debug_rhs: &message.debug_rhs.lhs,
            debug_cmp: message.debug_cmp.lhs,
        };
        let rhs = DebugMessageImpl::<'_, R> {
            result: &message.result.rhs,
            source: &message.source.rhs,
            vtable: message.vtable.rhs,
            debug_lhs: &message.debug_lhs.rhs,
            debug_rhs: &message.debug_rhs.rhs,
            debug_cmp: message.debug_cmp.rhs,
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval && rhs_eval) {
            if !lhs_eval {
                L::debug_impl(&lhs, f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                R::debug_impl(&rhs, f)?;
            }
        }
        Ok(())
    }
}

impl<L: Recompose, R: Recompose> Recompose for crate::OrExpr<L, R> {
    type Result = expr::OrExpr<L::Result, R::Result>;
    type Source = expr::OrExpr<L::Source, R::Source>;
    type VTable = expr::OrExpr<&'static L::VTable, &'static R::VTable>;
    type DebugCmp = expr::OrExpr<L::DebugCmp, R::DebugCmp>;
    type DebugLhs = expr::OrExpr<L::DebugLhs, R::DebugLhs>;
    type DebugRhs = expr::OrExpr<L::DebugRhs, R::DebugRhs>;

    fn eval_impl(
        debug_lhs: &Self::DebugLhs,
        debug_rhs: &Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let lhs = L::eval_impl(&debug_lhs.lhs, &debug_rhs.lhs, debug_cmp.lhs, vtable.lhs);
        let rhs = R::eval_impl(&debug_lhs.rhs, &debug_rhs.rhs, debug_cmp.rhs, vtable.rhs);
        expr::OrExpr { lhs, rhs }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = DebugMessageImpl::<'_, L> {
            result: &message.result.lhs,
            source: &message.source.lhs,
            vtable: message.vtable.lhs,
            debug_lhs: &message.debug_lhs.lhs,
            debug_rhs: &message.debug_rhs.lhs,
            debug_cmp: message.debug_cmp.lhs,
        };
        let rhs = DebugMessageImpl::<'_, R> {
            result: &message.result.rhs,
            source: &message.source.rhs,
            vtable: message.vtable.rhs,
            debug_lhs: &message.debug_lhs.rhs,
            debug_rhs: &message.debug_rhs.rhs,
            debug_cmp: message.debug_cmp.rhs,
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval || rhs_eval) {
            if !lhs_eval {
                L::debug_impl(&lhs, f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                R::debug_impl(&rhs, f)?;
            }
        }
        Ok(())
    }
}

impl Decompose for &'static str {
    type Decomposed = bool;
}
impl Decompose for expr::CmpExpr<(), &'static str, &'static str> {
    type Decomposed = crate::CmpExpr;
}
impl Decompose for expr::CustomCmpExpr<(), &'static str, &'static str> {
    type Decomposed = crate::CustomCmpExpr<()>;
}
impl<L: Decompose, R: Decompose> Decompose for expr::AndExpr<L, R> {
    type Decomposed = crate::AndExpr<L::Decomposed, R::Decomposed>;
}
impl<L: Decompose, R: Decompose> Decompose for expr::OrExpr<L, R> {
    type Decomposed = crate::OrExpr<L::Decomposed, R::Decomposed>;
}
