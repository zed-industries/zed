use crate::{
    decompose,
    traits::{DynInfo, DynInfoType, Expr},
};
use core::fmt;

pub struct DebugMessageImpl<'a, D: decompose::Recompose> {
    pub result: &'a D::Result,
    pub source: &'a D::Source,
    pub debug_lhs: &'a D::DebugLhs,
    pub debug_rhs: &'a D::DebugRhs,
    pub debug_cmp: D::DebugCmp,
    pub vtable: &'a D::VTable,
}
pub struct DebugMessage<'a, D: decompose::Recompose> {
    pub source: &'a WithSource<D::Source, &'static D::VTable>,
    pub debug_lhs: D::DebugLhs,
    pub debug_rhs: D::DebugRhs,
    pub debug_cmp: D::DebugCmp,
    pub message: fmt::Arguments<'a>,
}
impl<D: decompose::Recompose> fmt::Debug for DebugMessage<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        D::debug_final(self, f)
    }
}

impl<D: decompose::Recompose> Copy for DebugMessage<'_, D> {}
impl<D: decompose::Recompose> Clone for DebugMessage<'_, D> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct WithSource<S, V> {
    pub source: S,
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub vtable: V,
}

#[derive(Copy, Clone)]
pub struct Finalize<E> {
    pub inner: E,
}

impl<E: DynInfoType> DynInfoType for Finalize<E> {
    type VTable = E::VTable;
    const NULL_VTABLE: &'static Self::VTable = E::NULL_VTABLE;
}

impl<E: DynInfo> DynInfo for Finalize<E> {
    const VTABLE: &'static Self::VTable = E::VTABLE;
}

impl<E> Expr for &Finalize<E> {
    type Result = Result<(), ()>;
    type Marker = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        core::unreachable!()
    }
}

impl<E: Expr> Expr for &&Finalize<E> {
    type Result = E::Result;
    type Marker = E::Marker;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.inner.eval_expr()
    }
}
