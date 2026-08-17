//! Optimization driver using ISLE rewrite rules on an egraph.

mod div_const;

use crate::egraph::{NewOrExistingInst, OptimizeCtx};
pub use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir::dfg::ValueDef;
pub use crate::ir::immediates::{Ieee16, Ieee32, Ieee64, Ieee128, Imm64, Offset32, Uimm8, V128Imm};
use crate::ir::instructions::InstructionFormat;
pub use crate::ir::types::*;
pub use crate::ir::{
    AtomicRmwOp, BlockCall, Constant, DynamicStackSlot, FuncRef, GlobalValue, Immediate,
    InstructionData, MemFlags, Opcode, StackSlot, TrapCode, Type, Value,
};
use crate::isle_common_prelude_methods;
use crate::machinst::isle::*;
use crate::trace;
use cranelift_entity::packed_option::ReservedValue;
use smallvec::{SmallVec, smallvec};
use std::marker::PhantomData;

pub type Unit = ();
pub type ValueArray2 = [Value; 2];
pub type ValueArray3 = [Value; 3];

const MAX_ISLE_RETURNS: usize = 8;

pub type ConstructorVec<T> = SmallVec<[T; MAX_ISLE_RETURNS]>;

type TypeAndInstructionData = (Type, InstructionData);

impl<T: smallvec::Array> generated_code::Length for SmallVec<T> {
    #[inline]
    fn len(&self) -> usize {
        SmallVec::len(self)
    }
}

pub(crate) mod generated_code;
use generated_code::{ContextIter, IntoContextIter};

pub(crate) struct IsleContext<'a, 'b, 'c> {
    pub(crate) ctx: &'a mut OptimizeCtx<'b, 'c>,
}

impl IsleContext<'_, '_, '_> {
    #[allow(dead_code, reason = "dead code, only on nightly rust at this time")]
    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.ctx.func.dfg
    }
}

pub(crate) struct InstDataEtorIter<'a, 'b, 'c> {
    stack: SmallVec<[Value; 8]>,
    _phantom1: PhantomData<&'a ()>,
    _phantom2: PhantomData<&'b ()>,
    _phantom3: PhantomData<&'c ()>,
}

impl Default for InstDataEtorIter<'_, '_, '_> {
    fn default() -> Self {
        InstDataEtorIter {
            stack: SmallVec::default(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
        }
    }
}

impl<'a, 'b, 'c> InstDataEtorIter<'a, 'b, 'c> {
    fn new(root: Value) -> Self {
        debug_assert_ne!(root, Value::reserved_value());
        trace!("new iter from root {root}");
        Self {
            stack: smallvec![root],
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
        }
    }
}

impl<'a, 'b, 'c> ContextIter for InstDataEtorIter<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    type Context = IsleContext<'a, 'b, 'c>;
    type Output = (Type, InstructionData);

    fn next(&mut self, ctx: &mut IsleContext<'a, 'b, 'c>) -> Option<Self::Output> {
        while let Some(value) = self.stack.pop() {
            debug_assert!(ctx.ctx.func.dfg.value_is_real(value));
            trace!("iter: value {:?}", value);
            match ctx.ctx.func.dfg.value_def(value) {
                ValueDef::Union(x, y) => {
                    debug_assert_ne!(x, Value::reserved_value());
                    debug_assert_ne!(y, Value::reserved_value());
                    trace!(" -> {}, {}", x, y);
                    self.stack.push(x);
                    self.stack.push(y);
                    continue;
                }
                ValueDef::Result(inst, _) if ctx.ctx.func.dfg.inst_results(inst).len() == 1 => {
                    let ty = ctx.ctx.func.dfg.value_type(value);
                    trace!(" -> value of type {}", ty);
                    return Some((ty, ctx.ctx.func.dfg.insts[inst]));
                }
                _ => {}
            }
        }
        None
    }
}

impl<'a, 'b, 'c> IntoContextIter for InstDataEtorIter<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    type Context = IsleContext<'a, 'b, 'c>;
    type Output = (Type, InstructionData);
    type IntoIter = Self;

    fn into_context_iter(self) -> Self {
        self
    }
}

#[derive(Default)]
pub(crate) struct MaybeUnaryEtorIter<'a, 'b, 'c> {
    opcode: Option<Opcode>,
    inner: InstDataEtorIter<'a, 'b, 'c>,
    fallback: Option<Value>,
}

impl MaybeUnaryEtorIter<'_, '_, '_> {
    fn new(opcode: Opcode, value: Value) -> Self {
        debug_assert_eq!(opcode.format(), InstructionFormat::Unary);
        Self {
            opcode: Some(opcode),
            inner: InstDataEtorIter::new(value),
            fallback: Some(value),
        }
    }
}

impl<'a, 'b, 'c> ContextIter for MaybeUnaryEtorIter<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    type Context = IsleContext<'a, 'b, 'c>;
    type Output = (Type, Value);

    fn next(&mut self, ctx: &mut IsleContext<'a, 'b, 'c>) -> Option<Self::Output> {
        debug_assert_ne!(self.opcode, None);
        while let Some((ty, inst_def)) = self.inner.next(ctx) {
            let InstructionData::Unary { opcode, arg } = inst_def else {
                continue;
            };
            if Some(opcode) == self.opcode {
                self.fallback = None;
                return Some((ty, arg));
            }
        }

        self.fallback.take().map(|value| {
            let ty = generated_code::Context::value_type(ctx, value);
            (ty, value)
        })
    }
}

impl<'a, 'b, 'c> IntoContextIter for MaybeUnaryEtorIter<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    type Context = IsleContext<'a, 'b, 'c>;
    type Output = (Type, Value);
    type IntoIter = Self;

    fn into_context_iter(self) -> Self {
        self
    }
}

impl<'a, 'b, 'c> generated_code::Context for IsleContext<'a, 'b, 'c> {
    isle_common_prelude_methods!();

    type inst_data_value_etor_returns = InstDataEtorIter<'a, 'b, 'c>;

    fn inst_data_value_etor(&mut self, eclass: Value, returns: &mut InstDataEtorIter<'a, 'b, 'c>) {
        *returns = InstDataEtorIter::new(eclass);
    }

    type inst_data_value_tupled_etor_returns = InstDataEtorIter<'a, 'b, 'c>;

    fn inst_data_value_tupled_etor(
        &mut self,
        eclass: Value,
        returns: &mut InstDataEtorIter<'a, 'b, 'c>,
    ) {
        // Literally identical to `inst_data_value_etor`, just a different nominal type in ISLE
        self.inst_data_value_etor(eclass, returns);
    }

    fn make_inst_ctor(&mut self, ty: Type, op: &InstructionData) -> Value {
        trace!("make_inst_ctor: creating {:?}", op);
        let value = self.ctx.insert_pure_enode(NewOrExistingInst::New(*op, ty));
        trace!("make_inst_ctor: {:?} -> {}", op, value);
        value
    }

    fn make_skeleton_inst_ctor(&mut self, data: &InstructionData) -> Inst {
        let inst = self.ctx.func.dfg.make_inst(*data);
        self.ctx
            .func
            .dfg
            .make_inst_results(inst, Default::default());
        inst
    }

    fn inst_data_etor(&mut self, inst: Inst) -> Option<InstructionData> {
        Some(self.ctx.func.dfg.insts[inst])
    }

    fn value_array_2_ctor(&mut self, arg0: Value, arg1: Value) -> ValueArray2 {
        [arg0, arg1]
    }

    fn value_array_3_ctor(&mut self, arg0: Value, arg1: Value, arg2: Value) -> ValueArray3 {
        [arg0, arg1, arg2]
    }

    #[inline]
    fn value_type(&mut self, val: Value) -> Type {
        self.ctx.func.dfg.value_type(val)
    }

    fn iconst_sextend_etor(
        &mut self,
        (ty, inst_data): (Type, InstructionData),
    ) -> Option<(Type, i64)> {
        if let InstructionData::UnaryImm {
            opcode: Opcode::Iconst,
            imm,
        } = inst_data
        {
            Some((ty, self.i64_sextend_imm64(ty, imm)))
        } else {
            None
        }
    }

    fn remat(&mut self, value: Value) -> Value {
        trace!("remat: {}", value);
        self.ctx.remat_values.insert(value);
        self.ctx.stats.remat += 1;
        value
    }

    fn subsume(&mut self, value: Value) -> Value {
        trace!("subsume: {}", value);
        self.ctx.subsume_values.insert(value);
        self.ctx.stats.subsume += 1;
        value
    }

    fn splat64(&mut self, val: u64) -> Constant {
        let val = u128::from(val);
        let val = val | (val << 64);
        let imm = V128Imm(val.to_le_bytes());
        self.ctx.func.dfg.constants.insert(imm.into())
    }

    type sextend_maybe_etor_returns = MaybeUnaryEtorIter<'a, 'b, 'c>;
    fn sextend_maybe_etor(&mut self, value: Value, returns: &mut Self::sextend_maybe_etor_returns) {
        *returns = MaybeUnaryEtorIter::new(Opcode::Sextend, value);
    }

    type uextend_maybe_etor_returns = MaybeUnaryEtorIter<'a, 'b, 'c>;
    fn uextend_maybe_etor(&mut self, value: Value, returns: &mut Self::uextend_maybe_etor_returns) {
        *returns = MaybeUnaryEtorIter::new(Opcode::Uextend, value);
    }

    // NB: Cranelift's defined semantics for `fcvt_from_{s,u}int` match Rust's
    // own semantics for converting an integer to a float, so these are all
    // implemented with `as` conversions in Rust.
    fn f32_from_uint(&mut self, n: u64) -> Ieee32 {
        Ieee32::with_float(n as f32)
    }

    fn f64_from_uint(&mut self, n: u64) -> Ieee64 {
        Ieee64::with_float(n as f64)
    }

    fn f32_from_sint(&mut self, n: i64) -> Ieee32 {
        Ieee32::with_float(n as f32)
    }

    fn f64_from_sint(&mut self, n: i64) -> Ieee64 {
        Ieee64::with_float(n as f64)
    }

    fn u64_bswap16(&mut self, n: u64) -> u64 {
        (n as u16).swap_bytes() as u64
    }

    fn u64_bswap32(&mut self, n: u64) -> u64 {
        (n as u32).swap_bytes() as u64
    }

    fn u64_bswap64(&mut self, n: u64) -> u64 {
        n.swap_bytes()
    }

    fn ieee128_constant_extractor(&mut self, n: Constant) -> Option<Ieee128> {
        self.ctx.func.dfg.constants.get(n).try_into().ok()
    }

    fn ieee128_constant(&mut self, n: Ieee128) -> Constant {
        self.ctx.func.dfg.constants.insert(n.into())
    }

    fn div_const_magic_u32(&mut self, d: u32) -> generated_code::DivConstMagicU32 {
        let div_const::MU32 {
            mul_by,
            do_add,
            shift_by,
        } = div_const::magic_u32(d);
        generated_code::DivConstMagicU32::U32 {
            mul_by,
            do_add,
            shift_by: shift_by.try_into().unwrap(),
        }
    }

    fn div_const_magic_u64(&mut self, d: u64) -> generated_code::DivConstMagicU64 {
        let div_const::MU64 {
            mul_by,
            do_add,
            shift_by,
        } = div_const::magic_u64(d);
        generated_code::DivConstMagicU64::U64 {
            mul_by,
            do_add,
            shift_by: shift_by.try_into().unwrap(),
        }
    }

    fn div_const_magic_s32(&mut self, d: i32) -> generated_code::DivConstMagicS32 {
        let div_const::MS32 { mul_by, shift_by } = div_const::magic_s32(d);
        generated_code::DivConstMagicS32::S32 {
            mul_by,
            shift_by: shift_by.try_into().unwrap(),
        }
    }

    fn div_const_magic_s64(&mut self, d: i64) -> generated_code::DivConstMagicS64 {
        let div_const::MS64 { mul_by, shift_by } = div_const::magic_s64(d);
        generated_code::DivConstMagicS64::S64 {
            mul_by,
            shift_by: shift_by.try_into().unwrap(),
        }
    }
}
