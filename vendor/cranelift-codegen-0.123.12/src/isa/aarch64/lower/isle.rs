//! ISLE integration glue code for aarch64 lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
use generated_code::{Context, ImmExtend};

// Types that the generated ISLE code uses via `use super::*`.
use super::{
    ASIMDFPModImm, ASIMDMovModImm, BranchTarget, CallInfo, Cond, CondBrKind, ExtendOp, FPUOpRI,
    FPUOpRIMod, FloatCC, Imm12, ImmLogic, ImmShift, Inst as MInst, IntCC, MachLabel, MemLabel,
    MoveWideConst, MoveWideOp, NZCV, Opcode, OperandSize, Reg, SImm9, ScalarSize, ShiftOpAndAmt,
    UImm5, UImm12Scaled, VecMisc2, VectorSize, fp_reg, lower_condcode, lower_fp_condcode,
    stack_reg, writable_link_reg, writable_zero_reg, zero_reg,
};
use crate::ir::{ArgumentExtension, condcodes};
use crate::isa;
use crate::isa::aarch64::AArch64Backend;
use crate::isa::aarch64::inst::{FPULeftShiftImm, FPURightShiftImm, ReturnCallInfo};
use crate::machinst::isle::*;
use crate::{
    binemit::CodeOffset,
    ir::{
        AtomicRmwOp, BlockCall, ExternalName, Inst, InstructionData, MemFlags, TrapCode, Value,
        ValueList, immediates::*, types::*,
    },
    isa::aarch64::abi::AArch64MachineDeps,
    isa::aarch64::inst::SImm7Scaled,
    isa::aarch64::inst::args::{ShiftOp, ShiftOpShiftImm},
    machinst::{
        CallArgList, CallRetList, InstOutput, MachInst, VCodeConstant, VCodeConstantData,
        abi::ArgPair, ty_bits,
    },
};
use core::u32;
use regalloc2::PReg;
use std::boxed::Box;
use std::vec::Vec;

type BoxCallInfo = Box<CallInfo<ExternalName>>;
type BoxCallIndInfo = Box<CallInfo<Reg>>;
type BoxReturnCallInfo = Box<ReturnCallInfo<ExternalName>>;
type BoxReturnCallIndInfo = Box<ReturnCallInfo<Reg>>;
type VecMachLabel = Vec<MachLabel>;
type BoxExternalName = Box<ExternalName>;
type VecArgPair = Vec<ArgPair>;

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &AArch64Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &AArch64Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}

pub struct ExtendedValue {
    val: Value,
    extend: ExtendOp,
}

impl Context for IsleContext<'_, '_, MInst, AArch64Backend> {
    isle_lower_prelude_methods!();

    fn gen_call_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info),
        )
    }

    fn gen_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallIndInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info),
        )
    }

    fn gen_return_call_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
    ) -> BoxReturnCallInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        let key =
            AArch64MachineDeps::select_api_key(&self.backend.isa_flags, isa::CallConv::Tail, true);

        Box::new(ReturnCallInfo {
            dest,
            uses,
            key,
            new_stack_arg_size,
        })
    }

    fn gen_return_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
    ) -> BoxReturnCallIndInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        let key =
            AArch64MachineDeps::select_api_key(&self.backend.isa_flags, isa::CallConv::Tail, true);

        Box::new(ReturnCallInfo {
            dest,
            uses,
            key,
            new_stack_arg_size,
        })
    }

    fn sign_return_address_disabled(&mut self) -> Option<()> {
        if self.backend.isa_flags.sign_return_address() {
            None
        } else {
            Some(())
        }
    }

    fn use_lse(&mut self, _: Inst) -> Option<()> {
        if self.backend.isa_flags.has_lse() {
            Some(())
        } else {
            None
        }
    }

    fn use_fp16(&mut self) -> bool {
        self.backend.isa_flags.has_fp16()
    }

    fn move_wide_const_from_u64(&mut self, ty: Type, n: u64) -> Option<MoveWideConst> {
        let bits = ty.bits();
        let n = if bits < 64 {
            n & !(u64::MAX << bits)
        } else {
            n
        };
        MoveWideConst::maybe_from_u64(n)
    }

    fn move_wide_const_from_inverted_u64(&mut self, ty: Type, n: u64) -> Option<MoveWideConst> {
        self.move_wide_const_from_u64(ty, !n)
    }

    fn imm_logic_from_u64(&mut self, ty: Type, n: u64) -> Option<ImmLogic> {
        ImmLogic::maybe_from_u64(n, ty)
    }

    fn imm_size_from_type(&mut self, ty: Type) -> Option<u16> {
        match ty {
            I32 => Some(32),
            I64 => Some(64),
            _ => None,
        }
    }

    fn imm_logic_from_imm64(&mut self, ty: Type, n: Imm64) -> Option<ImmLogic> {
        let ty = if ty.bits() < 32 { I32 } else { ty };
        self.imm_logic_from_u64(ty, n.bits() as u64)
    }

    fn imm12_from_u64(&mut self, n: u64) -> Option<Imm12> {
        Imm12::maybe_from_u64(n)
    }

    fn imm_shift_from_u8(&mut self, n: u8) -> ImmShift {
        ImmShift::maybe_from_u64(n.into()).unwrap()
    }

    fn lshr_from_u64(&mut self, ty: Type, n: u64) -> Option<ShiftOpAndAmt> {
        let shiftimm = ShiftOpShiftImm::maybe_from_shift(n)?;
        if let Ok(bits) = u8::try_from(ty_bits(ty)) {
            let shiftimm = shiftimm.mask(bits);
            Some(ShiftOpAndAmt::new(ShiftOp::LSR, shiftimm))
        } else {
            None
        }
    }

    fn lshl_from_imm64(&mut self, ty: Type, n: Imm64) -> Option<ShiftOpAndAmt> {
        self.lshl_from_u64(ty, n.bits() as u64)
    }

    fn lshl_from_u64(&mut self, ty: Type, n: u64) -> Option<ShiftOpAndAmt> {
        let shiftimm = ShiftOpShiftImm::maybe_from_shift(n)?;
        let shiftee_bits = ty_bits(ty);
        if shiftee_bits <= std::u8::MAX as usize {
            let shiftimm = shiftimm.mask(shiftee_bits as u8);
            Some(ShiftOpAndAmt::new(ShiftOp::LSL, shiftimm))
        } else {
            None
        }
    }

    fn ashr_from_u64(&mut self, ty: Type, n: u64) -> Option<ShiftOpAndAmt> {
        let shiftimm = ShiftOpShiftImm::maybe_from_shift(n)?;
        let shiftee_bits = ty_bits(ty);
        if shiftee_bits <= std::u8::MAX as usize {
            let shiftimm = shiftimm.mask(shiftee_bits as u8);
            Some(ShiftOpAndAmt::new(ShiftOp::ASR, shiftimm))
        } else {
            None
        }
    }

    fn integral_ty(&mut self, ty: Type) -> Option<Type> {
        match ty {
            I8 | I16 | I32 | I64 => Some(ty),
            _ => None,
        }
    }

    fn is_zero_simm9(&mut self, imm: &SImm9) -> Option<()> {
        if imm.value() == 0 { Some(()) } else { None }
    }

    fn is_zero_uimm12(&mut self, imm: &UImm12Scaled) -> Option<()> {
        if imm.value() == 0 { Some(()) } else { None }
    }

    /// This is target-word-size dependent.  And it excludes booleans and reftypes.
    fn valid_atomic_transaction(&mut self, ty: Type) -> Option<Type> {
        match ty {
            I8 | I16 | I32 | I64 => Some(ty),
            _ => None,
        }
    }

    /// This is the fallback case for loading a 64-bit integral constant into a
    /// register.
    ///
    /// The logic here is nontrivial enough that it's not really worth porting
    /// this over to ISLE.
    fn load_constant_full(
        &mut self,
        ty: Type,
        extend: &ImmExtend,
        extend_to: &OperandSize,
        value: u64,
    ) -> Reg {
        let bits = ty.bits();

        let value = match (extend_to, *extend) {
            (OperandSize::Size32, ImmExtend::Sign) if bits < 32 => {
                let shift = 32 - bits;
                let value = value as i32;

                // we cast first to a u32 and then to a u64, to ensure that we are representing a
                // i32 in a u64, and not a i64. This is important, otherwise value will not fit in
                // 32 bits
                ((value << shift) >> shift) as u32 as u64
            }
            (OperandSize::Size32, ImmExtend::Zero) if bits < 32 => {
                value & !((u32::MAX as u64) << bits)
            }
            (OperandSize::Size64, ImmExtend::Sign) if bits < 64 => {
                let shift = 64 - bits;
                let value = value as i64;

                ((value << shift) >> shift) as u64
            }
            (OperandSize::Size64, ImmExtend::Zero) if bits < 64 => value & !(u64::MAX << bits),
            _ => value,
        };

        // Divide the value into 16-bit slices that we can manipulate using
        // `movz`, `movn`, and `movk`.
        fn get(value: u64, shift: u8) -> u16 {
            (value >> (shift * 16)) as u16
        }
        fn replace(mut old: u64, new: u16, shift: u8) -> u64 {
            let offset = shift * 16;
            old &= !(0xffff << offset);
            old |= u64::from(new) << offset;
            old
        }

        // The 32-bit versions of `movz`/`movn`/`movk` will clear the upper 32
        // bits, so if that's the outcome we want we might as well use them. For
        // simplicity and ease of reading the disassembly, we use the same size
        // for all instructions in the sequence.
        let size = if value >> 32 == 0 {
            OperandSize::Size32
        } else {
            OperandSize::Size64
        };

        // The `movz` instruction initially sets all bits to zero, while `movn`
        // initially sets all bits to one. A good choice of initial value can
        // reduce the number of `movk` instructions we need afterward, so we
        // check both variants to determine which is closest to the constant
        // we actually wanted. In case they're equally good, we prefer `movz`
        // because the assembly listings are generally harder to read when the
        // operands are negated.
        let (mut running_value, op, first) =
            [(MoveWideOp::MovZ, 0), (MoveWideOp::MovN, size.max_value())]
                .into_iter()
                .map(|(op, base)| {
                    // Both `movz` and `movn` can overwrite one slice after setting
                    // the initial value; we get to pick which one. 32-bit variants
                    // can only modify the lower two slices.
                    let first = (0..(size.bits() / 16))
                        // Pick one slice that's different from the initial value
                        .find(|&i| get(base ^ value, i) != 0)
                        // If none are different, we still have to pick one
                        .unwrap_or(0);
                    // Compute the value we'll get from this `movz`/`movn`
                    (replace(base, get(value, first), first), op, first)
                })
                // Count how many `movk` instructions we'll need.
                .min_by_key(|(base, ..)| (0..4).filter(|&i| get(base ^ value, i) != 0).count())
                // `variants` isn't empty so `min_by_key` always returns something.
                .unwrap();

        // Build the initial instruction we chose above, putting the result
        // into a new temporary virtual register. Note that the encoding for the
        // immediate operand is bitwise-inverted for `movn`.
        let mut rd = self.temp_writable_reg(I64);
        self.lower_ctx.emit(MInst::MovWide {
            op,
            rd,
            imm: MoveWideConst {
                bits: match op {
                    MoveWideOp::MovZ => get(value, first),
                    MoveWideOp::MovN => !get(value, first),
                },
                shift: first,
            },
            size,
        });
        if self.backend.flags.enable_pcc() {
            self.lower_ctx
                .add_range_fact(rd.to_reg(), 64, running_value, running_value);
        }

        // Emit a `movk` instruction for each remaining slice of the desired
        // constant that does not match the initial value constructed above.
        for shift in (first + 1)..(size.bits() / 16) {
            let bits = get(value, shift);
            if bits != get(running_value, shift) {
                let rn = rd.to_reg();
                rd = self.temp_writable_reg(I64);
                self.lower_ctx.emit(MInst::MovK {
                    rd,
                    rn,
                    imm: MoveWideConst { bits, shift },
                    size,
                });
                running_value = replace(running_value, bits, shift);
                if self.backend.flags.enable_pcc() {
                    self.lower_ctx
                        .add_range_fact(rd.to_reg(), 64, running_value, running_value);
                }
            }
        }

        debug_assert_eq!(value, running_value);
        return rd.to_reg();
    }

    fn zero_reg(&mut self) -> Reg {
        zero_reg()
    }

    fn stack_reg(&mut self) -> Reg {
        stack_reg()
    }

    fn fp_reg(&mut self) -> Reg {
        fp_reg()
    }

    fn writable_link_reg(&mut self) -> WritableReg {
        writable_link_reg()
    }

    fn extended_value_from_value(&mut self, val: Value) -> Option<ExtendedValue> {
        let (val, extend) = super::get_as_extended_value(self.lower_ctx, val)?;
        Some(ExtendedValue { val, extend })
    }

    fn put_extended_in_reg(&mut self, reg: &ExtendedValue) -> Reg {
        self.put_in_reg(reg.val)
    }

    fn get_extended_op(&mut self, reg: &ExtendedValue) -> ExtendOp {
        reg.extend
    }

    fn emit(&mut self, inst: &MInst) -> Unit {
        self.lower_ctx.emit(inst.clone());
    }

    fn cond_br_zero(&mut self, reg: Reg, size: &OperandSize) -> CondBrKind {
        CondBrKind::Zero(reg, *size)
    }

    fn cond_br_not_zero(&mut self, reg: Reg, size: &OperandSize) -> CondBrKind {
        CondBrKind::NotZero(reg, *size)
    }

    fn cond_br_cond(&mut self, cond: &Cond) -> CondBrKind {
        CondBrKind::Cond(*cond)
    }

    fn nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) -> NZCV {
        NZCV::new(n, z, c, v)
    }

    fn u8_into_uimm5(&mut self, x: u8) -> UImm5 {
        UImm5::maybe_from_u8(x).unwrap()
    }

    fn u8_into_imm12(&mut self, x: u8) -> Imm12 {
        Imm12::maybe_from_u64(x.into()).unwrap()
    }

    fn writable_zero_reg(&mut self) -> WritableReg {
        writable_zero_reg()
    }

    fn shift_mask(&mut self, ty: Type) -> ImmLogic {
        debug_assert!(ty.lane_bits().is_power_of_two());

        let mask = (ty.lane_bits() - 1) as u64;
        ImmLogic::maybe_from_u64(mask, I32).unwrap()
    }

    fn imm_shift_from_imm64(&mut self, ty: Type, val: Imm64) -> Option<ImmShift> {
        let imm_value = (val.bits() as u64) & ((ty.bits() - 1) as u64);
        ImmShift::maybe_from_u64(imm_value)
    }

    fn u64_into_imm_logic(&mut self, ty: Type, val: u64) -> ImmLogic {
        ImmLogic::maybe_from_u64(val, ty).unwrap()
    }

    fn negate_imm_shift(&mut self, ty: Type, mut imm: ImmShift) -> ImmShift {
        let size = u8::try_from(ty.bits()).unwrap();
        imm.imm = size.wrapping_sub(imm.value());
        imm.imm &= size - 1;
        imm
    }

    fn rotr_mask(&mut self, ty: Type) -> ImmLogic {
        ImmLogic::maybe_from_u64((ty.bits() - 1) as u64, I32).unwrap()
    }

    fn rotr_opposite_amount(&mut self, ty: Type, val: ImmShift) -> ImmShift {
        let amount = val.value() & u8::try_from(ty.bits() - 1).unwrap();
        ImmShift::maybe_from_u64(u64::from(ty.bits()) - u64::from(amount)).unwrap()
    }

    fn icmp_zero_cond(&mut self, cond: &IntCC) -> Option<IntCC> {
        match cond {
            &IntCC::Equal
            | &IntCC::SignedGreaterThanOrEqual
            | &IntCC::SignedGreaterThan
            | &IntCC::SignedLessThanOrEqual
            | &IntCC::SignedLessThan => Some(*cond),
            _ => None,
        }
    }

    fn fcmp_zero_cond(&mut self, cond: &FloatCC) -> Option<FloatCC> {
        match cond {
            &FloatCC::Equal
            | &FloatCC::GreaterThanOrEqual
            | &FloatCC::GreaterThan
            | &FloatCC::LessThanOrEqual
            | &FloatCC::LessThan => Some(*cond),
            _ => None,
        }
    }

    fn fcmp_zero_cond_not_eq(&mut self, cond: &FloatCC) -> Option<FloatCC> {
        match cond {
            &FloatCC::NotEqual => Some(FloatCC::NotEqual),
            _ => None,
        }
    }

    fn icmp_zero_cond_not_eq(&mut self, cond: &IntCC) -> Option<IntCC> {
        match cond {
            &IntCC::NotEqual => Some(IntCC::NotEqual),
            _ => None,
        }
    }

    fn float_cc_cmp_zero_to_vec_misc_op(&mut self, cond: &FloatCC) -> VecMisc2 {
        match cond {
            &FloatCC::Equal => VecMisc2::Fcmeq0,
            &FloatCC::GreaterThanOrEqual => VecMisc2::Fcmge0,
            &FloatCC::LessThanOrEqual => VecMisc2::Fcmle0,
            &FloatCC::GreaterThan => VecMisc2::Fcmgt0,
            &FloatCC::LessThan => VecMisc2::Fcmlt0,
            _ => panic!(),
        }
    }

    fn int_cc_cmp_zero_to_vec_misc_op(&mut self, cond: &IntCC) -> VecMisc2 {
        match cond {
            &IntCC::Equal => VecMisc2::Cmeq0,
            &IntCC::SignedGreaterThanOrEqual => VecMisc2::Cmge0,
            &IntCC::SignedLessThanOrEqual => VecMisc2::Cmle0,
            &IntCC::SignedGreaterThan => VecMisc2::Cmgt0,
            &IntCC::SignedLessThan => VecMisc2::Cmlt0,
            _ => panic!(),
        }
    }

    fn float_cc_cmp_zero_to_vec_misc_op_swap(&mut self, cond: &FloatCC) -> VecMisc2 {
        match cond {
            &FloatCC::Equal => VecMisc2::Fcmeq0,
            &FloatCC::GreaterThanOrEqual => VecMisc2::Fcmle0,
            &FloatCC::LessThanOrEqual => VecMisc2::Fcmge0,
            &FloatCC::GreaterThan => VecMisc2::Fcmlt0,
            &FloatCC::LessThan => VecMisc2::Fcmgt0,
            _ => panic!(),
        }
    }

    fn int_cc_cmp_zero_to_vec_misc_op_swap(&mut self, cond: &IntCC) -> VecMisc2 {
        match cond {
            &IntCC::Equal => VecMisc2::Cmeq0,
            &IntCC::SignedGreaterThanOrEqual => VecMisc2::Cmle0,
            &IntCC::SignedLessThanOrEqual => VecMisc2::Cmge0,
            &IntCC::SignedGreaterThan => VecMisc2::Cmlt0,
            &IntCC::SignedLessThan => VecMisc2::Cmgt0,
            _ => panic!(),
        }
    }

    fn fp_cond_code(&mut self, cc: &condcodes::FloatCC) -> Cond {
        lower_fp_condcode(*cc)
    }

    fn cond_code(&mut self, cc: &condcodes::IntCC) -> Cond {
        lower_condcode(*cc)
    }

    fn invert_cond(&mut self, cond: &Cond) -> Cond {
        (*cond).invert()
    }
    fn preg_sp(&mut self) -> PReg {
        super::regs::stack_reg().to_real_reg().unwrap().into()
    }

    fn preg_fp(&mut self) -> PReg {
        super::regs::fp_reg().to_real_reg().unwrap().into()
    }

    fn preg_link(&mut self) -> PReg {
        super::regs::link_reg().to_real_reg().unwrap().into()
    }

    fn preg_pinned(&mut self) -> PReg {
        super::regs::pinned_reg().to_real_reg().unwrap().into()
    }

    fn branch_target(&mut self, label: MachLabel) -> BranchTarget {
        BranchTarget::Label(label)
    }

    fn targets_jt_space(&mut self, elements: &BoxVecMachLabel) -> CodeOffset {
        // calculate the number of bytes needed for the jumptable sequence:
        // 4 bytes per instruction, with 8 instructions base + the size of
        // the jumptable more.
        (4 * (8 + elements.len())).try_into().unwrap()
    }

    fn min_fp_value(&mut self, signed: bool, in_bits: u8, out_bits: u8) -> Reg {
        if in_bits == 32 {
            // From float32.
            let min = match (signed, out_bits) {
                (true, 8) => i8::MIN as f32 - 1.,
                (true, 16) => i16::MIN as f32 - 1.,
                (true, 32) => i32::MIN as f32, // I32_MIN - 1 isn't precisely representable as a f32.
                (true, 64) => i64::MIN as f32, // I64_MIN - 1 isn't precisely representable as a f32.

                (false, _) => -1.,
                _ => unimplemented!(
                    "unexpected {} output size of {} bits for 32-bit input",
                    if signed { "signed" } else { "unsigned" },
                    out_bits
                ),
            };

            generated_code::constructor_constant_f32(self, min.to_bits())
        } else if in_bits == 64 {
            // From float64.
            let min = match (signed, out_bits) {
                (true, 8) => i8::MIN as f64 - 1.,
                (true, 16) => i16::MIN as f64 - 1.,
                (true, 32) => i32::MIN as f64 - 1.,
                (true, 64) => i64::MIN as f64,

                (false, _) => -1.,
                _ => unimplemented!(
                    "unexpected {} output size of {} bits for 64-bit input",
                    if signed { "signed" } else { "unsigned" },
                    out_bits
                ),
            };

            generated_code::constructor_constant_f64(self, min.to_bits())
        } else {
            unimplemented!(
                "unexpected input size for min_fp_value: {} (signed: {}, output size: {})",
                in_bits,
                signed,
                out_bits
            );
        }
    }

    fn max_fp_value(&mut self, signed: bool, in_bits: u8, out_bits: u8) -> Reg {
        if in_bits == 32 {
            // From float32.
            let max = match (signed, out_bits) {
                (true, 8) => i8::MAX as f32 + 1.,
                (true, 16) => i16::MAX as f32 + 1.,
                (true, 32) => (i32::MAX as u64 + 1) as f32,
                (true, 64) => (i64::MAX as u64 + 1) as f32,

                (false, 8) => u8::MAX as f32 + 1.,
                (false, 16) => u16::MAX as f32 + 1.,
                (false, 32) => (u32::MAX as u64 + 1) as f32,
                (false, 64) => (u64::MAX as u128 + 1) as f32,
                _ => unimplemented!(
                    "unexpected {} output size of {} bits for 32-bit input",
                    if signed { "signed" } else { "unsigned" },
                    out_bits
                ),
            };

            generated_code::constructor_constant_f32(self, max.to_bits())
        } else if in_bits == 64 {
            // From float64.
            let max = match (signed, out_bits) {
                (true, 8) => i8::MAX as f64 + 1.,
                (true, 16) => i16::MAX as f64 + 1.,
                (true, 32) => i32::MAX as f64 + 1.,
                (true, 64) => (i64::MAX as u64 + 1) as f64,

                (false, 8) => u8::MAX as f64 + 1.,
                (false, 16) => u16::MAX as f64 + 1.,
                (false, 32) => u32::MAX as f64 + 1.,
                (false, 64) => (u64::MAX as u128 + 1) as f64,
                _ => unimplemented!(
                    "unexpected {} output size of {} bits for 64-bit input",
                    if signed { "signed" } else { "unsigned" },
                    out_bits
                ),
            };

            generated_code::constructor_constant_f64(self, max.to_bits())
        } else {
            unimplemented!(
                "unexpected input size for max_fp_value: {} (signed: {}, output size: {})",
                in_bits,
                signed,
                out_bits
            );
        }
    }

    fn fpu_op_ri_ushr(&mut self, ty_bits: u8, shift: u8) -> FPUOpRI {
        if ty_bits == 32 {
            FPUOpRI::UShr32(FPURightShiftImm::maybe_from_u8(shift, ty_bits).unwrap())
        } else if ty_bits == 64 {
            FPUOpRI::UShr64(FPURightShiftImm::maybe_from_u8(shift, ty_bits).unwrap())
        } else {
            unimplemented!(
                "unexpected input size for fpu_op_ri_ushr: {} (shift: {})",
                ty_bits,
                shift
            );
        }
    }

    fn fpu_op_ri_sli(&mut self, ty_bits: u8, shift: u8) -> FPUOpRIMod {
        if ty_bits == 32 {
            FPUOpRIMod::Sli32(FPULeftShiftImm::maybe_from_u8(shift, ty_bits).unwrap())
        } else if ty_bits == 64 {
            FPUOpRIMod::Sli64(FPULeftShiftImm::maybe_from_u8(shift, ty_bits).unwrap())
        } else {
            unimplemented!(
                "unexpected input size for fpu_op_ri_sli: {} (shift: {})",
                ty_bits,
                shift
            );
        }
    }

    fn vec_extract_imm4_from_immediate(&mut self, imm: Immediate) -> Option<u8> {
        let bytes = self.lower_ctx.get_immediate_data(imm).as_slice();

        if bytes.windows(2).all(|a| a[0] + 1 == a[1]) && bytes[0] < 16 {
            Some(bytes[0])
        } else {
            None
        }
    }

    fn shuffle_dup8_from_imm(&mut self, imm: Immediate) -> Option<u8> {
        let bytes = self.lower_ctx.get_immediate_data(imm).as_slice();
        if bytes.iter().all(|b| *b == bytes[0]) && bytes[0] < 16 {
            Some(bytes[0])
        } else {
            None
        }
    }
    fn shuffle_dup16_from_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;
        if a == b && b == c && c == d && d == e && e == f && f == g && g == h && a < 8 {
            Some(a)
        } else {
            None
        }
    }
    fn shuffle_dup32_from_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b, c, d) = self.shuffle32_from_imm(imm)?;
        if a == b && b == c && c == d && a < 4 {
            Some(a)
        } else {
            None
        }
    }
    fn shuffle_dup64_from_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b) = self.shuffle64_from_imm(imm)?;
        if a == b && a < 2 { Some(a) } else { None }
    }

    fn asimd_mov_mod_imm_zero(&mut self, size: &ScalarSize) -> ASIMDMovModImm {
        ASIMDMovModImm::zero(*size)
    }

    fn asimd_mov_mod_imm_from_u64(
        &mut self,
        val: u64,
        size: &ScalarSize,
    ) -> Option<ASIMDMovModImm> {
        ASIMDMovModImm::maybe_from_u64(val, *size)
    }

    fn asimd_fp_mod_imm_from_u64(&mut self, val: u64, size: &ScalarSize) -> Option<ASIMDFPModImm> {
        ASIMDFPModImm::maybe_from_u64(val, *size)
    }

    fn u64_low32_bits_unset(&mut self, val: u64) -> Option<u64> {
        if val & 0xffffffff == 0 {
            Some(val)
        } else {
            None
        }
    }

    fn shift_masked_imm(&mut self, ty: Type, imm: u64) -> u8 {
        (imm as u8) & ((ty.lane_bits() - 1) as u8)
    }

    fn simm7_scaled_from_i64(&mut self, val: i64, ty: Type) -> Option<SImm7Scaled> {
        SImm7Scaled::maybe_from_i64(val, ty)
    }

    fn simm9_from_i64(&mut self, val: i64) -> Option<SImm9> {
        SImm9::maybe_from_i64(val)
    }

    fn uimm12_scaled_from_i64(&mut self, val: i64, ty: Type) -> Option<UImm12Scaled> {
        UImm12Scaled::maybe_from_i64(val, ty)
    }

    fn test_and_compare_bit_const(&mut self, ty: Type, n: u64) -> Option<u8> {
        if n.count_ones() != 1 {
            return None;
        }
        let bit = n.trailing_zeros();
        if bit >= ty.bits() {
            return None;
        }
        Some(bit as u8)
    }

    /// Use as a helper when generating `AluRRRShift` for `extr` instructions.
    fn a64_extr_imm(&mut self, ty: Type, shift: ImmShift) -> ShiftOpAndAmt {
        // The `ShiftOpAndAmt` immediate is used with `AluRRRShift` shape which
        // requires `ShiftOpAndAmt` so the shift of `ty` and `shift` are
        // translated into `ShiftOpAndAmt` here. The `ShiftOp` value here is
        // only used for its encoding, not its logical meaning.
        let (op, expected) = match ty {
            types::I32 => (ShiftOp::LSL, 0b00),
            types::I64 => (ShiftOp::LSR, 0b01),
            _ => unreachable!(),
        };
        assert_eq!(op.bits(), expected);
        ShiftOpAndAmt::new(
            op,
            ShiftOpShiftImm::maybe_from_shift(shift.value().into()).unwrap(),
        )
    }
}
