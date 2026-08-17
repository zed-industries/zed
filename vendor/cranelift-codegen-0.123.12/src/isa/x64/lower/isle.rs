//! ISLE integration glue code for x64 lowering.

// Pull in the ISLE generated code.
pub(crate) mod generated_code;
use crate::{ir::AtomicRmwOp, ir::types};
use generated_code::{AssemblerOutputs, Context, MInst, RegisterClass};

// Types that the generated ISLE code uses via `use super::*`.
use super::external::{CraneliftRegisters, PairedGpr, PairedXmm, isle_assembler_methods};
use super::{MergeableLoadSize, is_int_or_ref_ty, is_mergeable_load, lower_to_amode};
use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir::immediates::*;
use crate::ir::types::*;
use crate::ir::{
    BlockCall, Inst, InstructionData, LibCall, MemFlags, Opcode, TrapCode, Value, ValueList,
};
use crate::isa::x64::X64Backend;
use crate::isa::x64::inst::{ReturnCallInfo, args::*, regs};
use crate::isa::x64::lower::{InsnInput, emit_vm_call};
use crate::machinst::isle::*;
use crate::machinst::{
    ArgPair, CallArgList, CallInfo, CallRetList, InstOutput, MachInst, VCodeConstant,
    VCodeConstantData,
};
use alloc::vec::Vec;
use cranelift_assembler_x64 as asm;
use cranelift_entity::{Signed, Unsigned};
use regalloc2::PReg;
use std::boxed::Box;

/// Type representing out-of-line data for calls. This type optional because the
/// call instruction is also used by Winch to emit calls, but the
/// `Box<CallInfo>` field is not used, it's only used by Cranelift. By making it
/// optional, we reduce the number of heap allocations in Winch.
type BoxCallInfo = Box<CallInfo<ExternalName>>;
type BoxCallIndInfo = Box<CallInfo<RegMem>>;
type BoxReturnCallInfo = Box<ReturnCallInfo<ExternalName>>;
type BoxReturnCallIndInfo = Box<ReturnCallInfo<Reg>>;
type VecArgPair = Vec<ArgPair>;
type BoxSyntheticAmode = Box<SyntheticAmode>;

/// When interacting with the external assembler (see `external.rs`), we
/// need to fix the types we'll use.
type AssemblerInst = asm::Inst<CraneliftRegisters>;

pub struct SinkableLoad {
    inst: Inst,
    addr_input: InsnInput,
    offset: i32,
}

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &X64Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &X64Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, &targets)
}

impl Context for IsleContext<'_, '_, MInst, X64Backend> {
    isle_lower_prelude_methods!();
    isle_assembler_methods!();

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
        dest: &RegMem,
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
                .gen_call_info(sig, dest.clone(), uses, defs, try_call_info),
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

        Box::new(ReturnCallInfo {
            dest,
            uses,
            tmp: self.lower_ctx.temp_writable_gpr(),
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

        Box::new(ReturnCallInfo {
            dest,
            uses,
            tmp: self.lower_ctx.temp_writable_gpr(),
            new_stack_arg_size,
        })
    }

    #[inline]
    fn operand_size_of_type_32_64(&mut self, ty: Type) -> OperandSize {
        if ty.bits() == 64 {
            OperandSize::Size64
        } else {
            OperandSize::Size32
        }
    }

    #[inline]
    fn raw_operand_size_of_type(&mut self, ty: Type) -> OperandSize {
        OperandSize::from_ty(ty)
    }

    fn put_in_reg_mem_imm(&mut self, val: Value) -> RegMemImm {
        if let Some(imm) = self.i64_from_iconst(val) {
            if let Ok(imm) = i32::try_from(imm) {
                return RegMemImm::Imm {
                    simm32: imm.unsigned(),
                };
            }
        }

        self.put_in_reg_mem(val).into()
    }

    fn put_in_xmm_mem_imm(&mut self, val: Value) -> XmmMemImm {
        if let Some(imm) = self.i64_from_iconst(val) {
            if let Ok(imm) = i32::try_from(imm) {
                return XmmMemImm::unwrap_new(RegMemImm::Imm {
                    simm32: imm.unsigned(),
                });
            }
        }

        let res = match self.put_in_xmm_mem(val).to_reg_mem() {
            RegMem::Reg { reg } => RegMemImm::Reg { reg },
            RegMem::Mem { addr } => RegMemImm::Mem { addr },
        };

        XmmMemImm::unwrap_new(res)
    }

    fn put_in_xmm_mem(&mut self, val: Value) -> XmmMem {
        let inputs = self.lower_ctx.get_value_as_source_or_const(val);

        if let Some(c) = inputs.constant {
            // A load from the constant pool is better than a rematerialization into a register,
            // because it reduces register pressure.
            //
            // NOTE: this is where behavior differs from `put_in_reg_mem`, as we always force
            // constants to be 16 bytes when a constant will be used in place of an xmm register.
            let vcode_constant = self.emit_u128_le_const(c as u128);
            return XmmMem::unwrap_new(RegMem::mem(SyntheticAmode::ConstantOffset(vcode_constant)));
        }

        XmmMem::unwrap_new(self.put_in_reg_mem(val))
    }

    fn put_in_reg_mem(&mut self, val: Value) -> RegMem {
        let inputs = self.lower_ctx.get_value_as_source_or_const(val);

        if let Some(c) = inputs.constant {
            // A load from the constant pool is better than a
            // rematerialization into a register, because it reduces
            // register pressure.
            let vcode_constant = self.emit_u64_le_const(c);
            return RegMem::mem(SyntheticAmode::ConstantOffset(vcode_constant));
        }

        if let Some(load) = self.sinkable_load(val) {
            return RegMem::Mem {
                addr: self.sink_load(&load),
            };
        }

        RegMem::reg(self.put_in_reg(val))
    }

    #[inline]
    fn encode_fcmp_imm(&mut self, imm: &FcmpImm) -> u8 {
        imm.encode()
    }

    #[inline]
    fn encode_round_imm(&mut self, imm: &RoundImm) -> u8 {
        imm.encode()
    }

    #[inline]
    fn use_avx(&mut self) -> bool {
        self.backend.x64_flags.use_avx()
    }

    #[inline]
    fn use_avx2(&mut self) -> bool {
        self.backend.x64_flags.use_avx2()
    }

    #[inline]
    fn use_avx512vl(&mut self) -> bool {
        self.backend.x64_flags.use_avx512vl()
    }

    #[inline]
    fn use_avx512dq(&mut self) -> bool {
        self.backend.x64_flags.use_avx512dq()
    }

    #[inline]
    fn use_avx512f(&mut self) -> bool {
        self.backend.x64_flags.use_avx512f()
    }

    #[inline]
    fn use_avx512bitalg(&mut self) -> bool {
        self.backend.x64_flags.use_avx512bitalg()
    }

    #[inline]
    fn use_avx512vbmi(&mut self) -> bool {
        self.backend.x64_flags.use_avx512vbmi()
    }

    #[inline]
    fn use_lzcnt(&mut self) -> bool {
        self.backend.x64_flags.use_lzcnt()
    }

    #[inline]
    fn use_bmi1(&mut self) -> bool {
        self.backend.x64_flags.use_bmi1()
    }

    #[inline]
    fn use_bmi2(&mut self) -> bool {
        self.backend.x64_flags.use_bmi2()
    }

    #[inline]
    fn use_popcnt(&mut self) -> bool {
        self.backend.x64_flags.use_popcnt()
    }

    #[inline]
    fn use_fma(&mut self) -> bool {
        self.backend.x64_flags.use_fma()
    }

    #[inline]
    fn use_sse3(&mut self) -> bool {
        self.backend.x64_flags.use_sse3()
    }

    #[inline]
    fn use_ssse3(&mut self) -> bool {
        self.backend.x64_flags.use_ssse3()
    }

    #[inline]
    fn use_sse41(&mut self) -> bool {
        self.backend.x64_flags.use_sse41()
    }

    #[inline]
    fn use_sse42(&mut self) -> bool {
        self.backend.x64_flags.use_sse42()
    }

    #[inline]
    fn use_cmpxchg16b(&mut self) -> bool {
        self.backend.x64_flags.use_cmpxchg16b()
    }

    #[inline]
    fn shift_mask(&mut self, ty: Type) -> u8 {
        debug_assert!(ty.lane_bits().is_power_of_two());

        (ty.lane_bits() - 1) as u8
    }

    fn shift_amount_masked(&mut self, ty: Type, val: Imm64) -> u8 {
        (val.bits() as u8) & self.shift_mask(ty)
    }

    #[inline]
    fn simm32_from_value(&mut self, val: Value) -> Option<GprMemImm> {
        let imm = self.i64_from_iconst(val)?;
        Some(GprMemImm::unwrap_new(RegMemImm::Imm {
            simm32: i32::try_from(imm).ok()?.unsigned(),
        }))
    }

    fn sinkable_load(&mut self, val: Value) -> Option<SinkableLoad> {
        if let Some(inst) = self.is_sinkable_inst(val) {
            if let Some((addr_input, offset)) =
                is_mergeable_load(self.lower_ctx, inst, MergeableLoadSize::Min32)
            {
                return Some(SinkableLoad {
                    inst,
                    addr_input,
                    offset,
                });
            }
        }
        None
    }

    fn sinkable_load_exact(&mut self, val: Value) -> Option<SinkableLoad> {
        if let Some(inst) = self.is_sinkable_inst(val) {
            if let Some((addr_input, offset)) =
                is_mergeable_load(self.lower_ctx, inst, MergeableLoadSize::Exact)
            {
                return Some(SinkableLoad {
                    inst,
                    addr_input,
                    offset,
                });
            }
        }
        None
    }

    fn sink_load(&mut self, load: &SinkableLoad) -> SyntheticAmode {
        self.lower_ctx.sink_inst(load.inst);
        let addr = lower_to_amode(self.lower_ctx, load.addr_input, load.offset);
        SyntheticAmode::Real(addr)
    }

    #[inline]
    fn ext_mode(&mut self, from_bits: u16, to_bits: u16) -> ExtMode {
        ExtMode::new(from_bits, to_bits).unwrap()
    }

    fn emit(&mut self, inst: &MInst) -> Unit {
        self.lower_ctx.emit(inst.clone());
    }

    #[inline]
    fn sse_insertps_lane_imm(&mut self, lane: u8) -> u8 {
        // Insert 32-bits from replacement (at index 00, bits 7:8) to vector (lane
        // shifted into bits 5:6).
        0b00_00_00_00 | lane << 4
    }

    #[inline]
    fn synthetic_amode_to_reg_mem(&mut self, addr: &SyntheticAmode) -> RegMem {
        RegMem::mem(addr.clone())
    }

    #[inline]
    fn amode_to_synthetic_amode(&mut self, amode: &Amode) -> SyntheticAmode {
        amode.clone().into()
    }

    #[inline]
    fn const_to_synthetic_amode(&mut self, c: VCodeConstant) -> SyntheticAmode {
        SyntheticAmode::ConstantOffset(c)
    }

    #[inline]
    fn writable_gpr_to_reg(&mut self, r: WritableGpr) -> WritableReg {
        r.to_writable_reg()
    }

    #[inline]
    fn writable_xmm_to_reg(&mut self, r: WritableXmm) -> WritableReg {
        r.to_writable_reg()
    }

    fn ishl_i8x16_mask_for_const(&mut self, amt: u32) -> SyntheticAmode {
        // When the shift amount is known, we can statically (i.e. at compile
        // time) determine the mask to use and only emit that.
        debug_assert!(amt < 8);
        let mask_offset = amt as usize * 16;
        let mask_constant = self.lower_ctx.use_constant(VCodeConstantData::WellKnown(
            &I8X16_ISHL_MASKS[mask_offset..mask_offset + 16],
        ));
        SyntheticAmode::ConstantOffset(mask_constant)
    }

    fn ishl_i8x16_mask_table(&mut self) -> SyntheticAmode {
        let mask_table = self
            .lower_ctx
            .use_constant(VCodeConstantData::WellKnown(&I8X16_ISHL_MASKS));
        SyntheticAmode::ConstantOffset(mask_table)
    }

    fn ushr_i8x16_mask_for_const(&mut self, amt: u32) -> SyntheticAmode {
        // When the shift amount is known, we can statically (i.e. at compile
        // time) determine the mask to use and only emit that.
        debug_assert!(amt < 8);
        let mask_offset = amt as usize * 16;
        let mask_constant = self.lower_ctx.use_constant(VCodeConstantData::WellKnown(
            &I8X16_USHR_MASKS[mask_offset..mask_offset + 16],
        ));
        SyntheticAmode::ConstantOffset(mask_constant)
    }

    fn ushr_i8x16_mask_table(&mut self) -> SyntheticAmode {
        let mask_table = self
            .lower_ctx
            .use_constant(VCodeConstantData::WellKnown(&I8X16_USHR_MASKS));
        SyntheticAmode::ConstantOffset(mask_table)
    }

    #[inline]
    fn writable_reg_to_xmm(&mut self, r: WritableReg) -> WritableXmm {
        Writable::from_reg(Xmm::unwrap_new(r.to_reg()))
    }

    #[inline]
    fn writable_xmm_to_xmm(&mut self, r: WritableXmm) -> Xmm {
        r.to_reg()
    }

    #[inline]
    fn writable_gpr_to_gpr(&mut self, r: WritableGpr) -> Gpr {
        r.to_reg()
    }

    #[inline]
    fn gpr_to_reg(&mut self, r: Gpr) -> Reg {
        r.into()
    }

    #[inline]
    fn xmm_to_reg(&mut self, r: Xmm) -> Reg {
        r.into()
    }

    #[inline]
    fn xmm_to_xmm_mem_imm(&mut self, r: Xmm) -> XmmMemImm {
        r.into()
    }

    #[inline]
    fn xmm_mem_to_xmm_mem_imm(&mut self, r: &XmmMem) -> XmmMemImm {
        XmmMemImm::unwrap_new(r.clone().to_reg_mem().into())
    }

    #[inline]
    fn temp_writable_gpr(&mut self) -> WritableGpr {
        self.lower_ctx.temp_writable_gpr()
    }

    #[inline]
    fn temp_writable_xmm(&mut self) -> WritableXmm {
        self.lower_ctx.temp_writable_xmm()
    }

    #[inline]
    fn reg_to_reg_mem_imm(&mut self, reg: Reg) -> RegMemImm {
        RegMemImm::Reg { reg }
    }

    #[inline]
    fn reg_mem_to_xmm_mem(&mut self, rm: &RegMem) -> XmmMem {
        XmmMem::unwrap_new(rm.clone())
    }

    #[inline]
    fn gpr_mem_imm_new(&mut self, rmi: &RegMemImm) -> GprMemImm {
        GprMemImm::unwrap_new(rmi.clone())
    }

    #[inline]
    fn xmm_mem_imm_new(&mut self, rmi: &RegMemImm) -> XmmMemImm {
        XmmMemImm::unwrap_new(rmi.clone())
    }

    #[inline]
    fn xmm_to_xmm_mem(&mut self, r: Xmm) -> XmmMem {
        r.into()
    }

    #[inline]
    fn xmm_mem_to_reg_mem(&mut self, xm: &XmmMem) -> RegMem {
        xm.clone().into()
    }

    #[inline]
    fn gpr_mem_to_reg_mem(&mut self, gm: &GprMem) -> RegMem {
        gm.clone().into()
    }

    #[inline]
    fn xmm_new(&mut self, r: Reg) -> Xmm {
        Xmm::unwrap_new(r)
    }

    #[inline]
    fn gpr_new(&mut self, r: Reg) -> Gpr {
        Gpr::unwrap_new(r)
    }

    #[inline]
    fn reg_mem_to_gpr_mem(&mut self, rm: &RegMem) -> GprMem {
        GprMem::unwrap_new(rm.clone())
    }

    #[inline]
    fn reg_to_gpr_mem(&mut self, r: Reg) -> GprMem {
        GprMem::unwrap_new(RegMem::reg(r))
    }

    #[inline]
    fn gpr_to_gpr_mem(&mut self, gpr: Gpr) -> GprMem {
        GprMem::from(gpr)
    }

    #[inline]
    fn gpr_to_gpr_mem_imm(&mut self, gpr: Gpr) -> GprMemImm {
        GprMemImm::from(gpr)
    }

    #[inline]
    fn type_register_class(&mut self, ty: Type) -> Option<RegisterClass> {
        if is_int_or_ref_ty(ty) || ty == I128 {
            Some(RegisterClass::Gpr {
                single_register: ty != I128,
            })
        } else if ty.is_float() || (ty.is_vector() && ty.bits() <= 128) {
            Some(RegisterClass::Xmm)
        } else {
            None
        }
    }

    #[inline]
    fn ty_int_bool_or_ref(&mut self, ty: Type) -> Option<()> {
        match ty {
            types::I8 | types::I16 | types::I32 | types::I64 => Some(()),
            _ => None,
        }
    }

    #[inline]
    fn intcc_to_cc(&mut self, intcc: &IntCC) -> CC {
        CC::from_intcc(*intcc)
    }

    #[inline]
    fn cc_invert(&mut self, cc: &CC) -> CC {
        cc.invert()
    }

    #[inline]
    fn cc_nz_or_z(&mut self, cc: &CC) -> Option<CC> {
        match cc {
            CC::Z => Some(*cc),
            CC::NZ => Some(*cc),
            _ => None,
        }
    }

    #[inline]
    fn sum_extend_fits_in_32_bits(
        &mut self,
        extend_from_ty: Type,
        constant_value: Imm64,
        offset: Offset32,
    ) -> Option<u32> {
        let offset: i64 = offset.into();
        let constant_value: u64 = constant_value.bits() as u64;
        // If necessary, zero extend `constant_value` up to 64 bits.
        let shift = 64 - extend_from_ty.bits();
        let zero_extended_constant_value = (constant_value << shift) >> shift;
        // Sum up the two operands.
        let sum = offset.wrapping_add(zero_extended_constant_value as i64);
        // Check that the sum will fit in 32-bits.
        if sum == ((sum << 32) >> 32) {
            Some(sum as u32)
        } else {
            None
        }
    }

    #[inline]
    fn amode_offset(&mut self, addr: &Amode, offset: i32) -> Amode {
        addr.offset(offset)
    }

    #[inline]
    fn zero_offset(&mut self) -> Offset32 {
        Offset32::new(0)
    }

    #[inline]
    fn preg_rbp(&mut self) -> PReg {
        regs::rbp().to_real_reg().unwrap().into()
    }

    #[inline]
    fn preg_rsp(&mut self) -> PReg {
        regs::rsp().to_real_reg().unwrap().into()
    }

    #[inline]
    fn preg_pinned(&mut self) -> PReg {
        regs::pinned_reg().to_real_reg().unwrap().into()
    }

    fn libcall_1(&mut self, libcall: &LibCall, a: Reg) -> Reg {
        let outputs = emit_vm_call(
            self.lower_ctx,
            &self.backend.flags,
            &self.backend.triple,
            *libcall,
            &[ValueRegs::one(a)],
        )
        .expect("Failed to emit LibCall");

        debug_assert_eq!(outputs.len(), 1);

        outputs[0].only_reg().unwrap()
    }

    fn libcall_2(&mut self, libcall: &LibCall, a: Reg, b: Reg) -> Reg {
        let outputs = emit_vm_call(
            self.lower_ctx,
            &self.backend.flags,
            &self.backend.triple,
            *libcall,
            &[ValueRegs::one(a), ValueRegs::one(b)],
        )
        .expect("Failed to emit LibCall");

        debug_assert_eq!(outputs.len(), 1);

        outputs[0].only_reg().unwrap()
    }

    fn libcall_3(&mut self, libcall: &LibCall, a: Reg, b: Reg, c: Reg) -> Reg {
        let outputs = emit_vm_call(
            self.lower_ctx,
            &self.backend.flags,
            &self.backend.triple,
            *libcall,
            &[ValueRegs::one(a), ValueRegs::one(b), ValueRegs::one(c)],
        )
        .expect("Failed to emit LibCall");

        debug_assert_eq!(outputs.len(), 1);

        outputs[0].only_reg().unwrap()
    }

    #[inline]
    fn vconst_all_ones_or_all_zeros(&mut self, constant: Constant) -> Option<()> {
        let const_data = self.lower_ctx.get_constant_data(constant);
        if const_data.iter().all(|&b| b == 0 || b == 0xFF) {
            return Some(());
        }
        None
    }

    #[inline]
    fn shuffle_0_31_mask(&mut self, mask: &VecMask) -> VCodeConstant {
        let mask = mask
            .iter()
            .map(|&b| if b > 15 { b.wrapping_sub(16) } else { b })
            .map(|b| if b > 15 { 0b10000000 } else { b })
            .collect();
        self.lower_ctx
            .use_constant(VCodeConstantData::Generated(mask))
    }

    #[inline]
    fn shuffle_0_15_mask(&mut self, mask: &VecMask) -> VCodeConstant {
        let mask = mask
            .iter()
            .map(|&b| if b > 15 { 0b10000000 } else { b })
            .collect();
        self.lower_ctx
            .use_constant(VCodeConstantData::Generated(mask))
    }

    #[inline]
    fn shuffle_16_31_mask(&mut self, mask: &VecMask) -> VCodeConstant {
        let mask = mask
            .iter()
            .map(|&b| b.wrapping_sub(16))
            .map(|b| if b > 15 { 0b10000000 } else { b })
            .collect();
        self.lower_ctx
            .use_constant(VCodeConstantData::Generated(mask))
    }

    #[inline]
    fn perm_from_mask_with_zeros(
        &mut self,
        mask: &VecMask,
    ) -> Option<(VCodeConstant, VCodeConstant)> {
        if !mask.iter().any(|&b| b > 31) {
            return None;
        }

        let zeros = mask
            .iter()
            .map(|&b| if b > 31 { 0x00 } else { 0xff })
            .collect();

        Some((
            self.perm_from_mask(mask),
            self.lower_ctx
                .use_constant(VCodeConstantData::Generated(zeros)),
        ))
    }

    #[inline]
    fn perm_from_mask(&mut self, mask: &VecMask) -> VCodeConstant {
        let mask = mask.iter().cloned().collect();
        self.lower_ctx
            .use_constant(VCodeConstantData::Generated(mask))
    }

    fn xmm_mem_to_xmm_mem_aligned(&mut self, arg: &XmmMem) -> XmmMemAligned {
        match XmmMemAligned::new(arg.clone().into()) {
            Some(aligned) => aligned,
            None => match arg.clone().into() {
                RegMem::Mem { addr } => self.load_xmm_unaligned(addr).into(),
                _ => unreachable!(),
            },
        }
    }

    fn xmm_mem_imm_to_xmm_mem_aligned_imm(&mut self, arg: &XmmMemImm) -> XmmMemAlignedImm {
        match XmmMemAlignedImm::new(arg.clone().into()) {
            Some(aligned) => aligned,
            None => match arg.clone().into() {
                RegMemImm::Mem { addr } => self.load_xmm_unaligned(addr).into(),
                _ => unreachable!(),
            },
        }
    }

    fn pshufd_lhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b, c, d) = self.shuffle32_from_imm(imm)?;
        if a < 4 && b < 4 && c < 4 && d < 4 {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn pshufd_rhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b, c, d) = self.shuffle32_from_imm(imm)?;
        // When selecting from the right-hand-side, subtract these all by 4
        // which will bail out if anything is less than 4. Afterwards the check
        // is the same as `pshufd_lhs_imm` above.
        let a = a.checked_sub(4)?;
        let b = b.checked_sub(4)?;
        let c = c.checked_sub(4)?;
        let d = d.checked_sub(4)?;
        if a < 4 && b < 4 && c < 4 && d < 4 {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn shufps_imm(&mut self, imm: Immediate) -> Option<u8> {
        // The `shufps` instruction selects the first two elements from the
        // first vector and the second two elements from the second vector, so
        // offset the third/fourth selectors by 4 and then make sure everything
        // fits in 32-bits.
        let (a, b, c, d) = self.shuffle32_from_imm(imm)?;
        let c = c.checked_sub(4)?;
        let d = d.checked_sub(4)?;
        if a < 4 && b < 4 && c < 4 && d < 4 {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn shufps_rev_imm(&mut self, imm: Immediate) -> Option<u8> {
        // This is almost the same as `shufps_imm` except the elements that are
        // subtracted are reversed. This handles the case that `shufps`
        // instruction can be emitted if the order of the operands are swapped.
        let (a, b, c, d) = self.shuffle32_from_imm(imm)?;
        let a = a.checked_sub(4)?;
        let b = b.checked_sub(4)?;
        if a < 4 && b < 4 && c < 4 && d < 4 {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn pshuflw_lhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        // Similar to `shufps` except this operates over 16-bit values so four
        // of them must be fixed and the other four must be in-range to encode
        // in the immediate.
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;
        if a < 4 && b < 4 && c < 4 && d < 4 && [e, f, g, h] == [4, 5, 6, 7] {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn pshuflw_rhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;
        let a = a.checked_sub(8)?;
        let b = b.checked_sub(8)?;
        let c = c.checked_sub(8)?;
        let d = d.checked_sub(8)?;
        let e = e.checked_sub(8)?;
        let f = f.checked_sub(8)?;
        let g = g.checked_sub(8)?;
        let h = h.checked_sub(8)?;
        if a < 4 && b < 4 && c < 4 && d < 4 && [e, f, g, h] == [4, 5, 6, 7] {
            Some(a | (b << 2) | (c << 4) | (d << 6))
        } else {
            None
        }
    }

    fn pshufhw_lhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        // Similar to `pshuflw` except that the first four operands must be
        // fixed and the second four are offset by an extra 4 and tested to
        // make sure they're all in the range [4, 8).
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;
        let e = e.checked_sub(4)?;
        let f = f.checked_sub(4)?;
        let g = g.checked_sub(4)?;
        let h = h.checked_sub(4)?;
        if e < 4 && f < 4 && g < 4 && h < 4 && [a, b, c, d] == [0, 1, 2, 3] {
            Some(e | (f << 2) | (g << 4) | (h << 6))
        } else {
            None
        }
    }

    fn pshufhw_rhs_imm(&mut self, imm: Immediate) -> Option<u8> {
        // Note that everything here is offset by at least 8 and the upper
        // bits are offset by 12 to test they're in the range of [12, 16).
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;
        let a = a.checked_sub(8)?;
        let b = b.checked_sub(8)?;
        let c = c.checked_sub(8)?;
        let d = d.checked_sub(8)?;
        let e = e.checked_sub(12)?;
        let f = f.checked_sub(12)?;
        let g = g.checked_sub(12)?;
        let h = h.checked_sub(12)?;
        if e < 4 && f < 4 && g < 4 && h < 4 && [a, b, c, d] == [0, 1, 2, 3] {
            Some(e | (f << 2) | (g << 4) | (h << 6))
        } else {
            None
        }
    }

    fn palignr_imm_from_immediate(&mut self, imm: Immediate) -> Option<u8> {
        let bytes = self.lower_ctx.get_immediate_data(imm).as_slice();

        if bytes.windows(2).all(|a| a[0] + 1 == a[1]) {
            Some(bytes[0])
        } else {
            None
        }
    }

    fn pblendw_imm(&mut self, imm: Immediate) -> Option<u8> {
        // First make sure that the shuffle immediate is selecting 16-bit lanes.
        let (a, b, c, d, e, f, g, h) = self.shuffle16_from_imm(imm)?;

        // Next build up an 8-bit mask from each of the bits of the selected
        // lanes above. This instruction can only be used when each lane
        // selector chooses from the corresponding lane in either of the two
        // operands, meaning the Nth lane selection must satisfy `lane % 8 ==
        // N`.
        //
        // This helper closure is used to calculate the value of the
        // corresponding bit.
        let bit = |x: u8, c: u8| {
            if x % 8 == c {
                if x < 8 { Some(0) } else { Some(1 << c) }
            } else {
                None
            }
        };
        Some(
            bit(a, 0)?
                | bit(b, 1)?
                | bit(c, 2)?
                | bit(d, 3)?
                | bit(e, 4)?
                | bit(f, 5)?
                | bit(g, 6)?
                | bit(h, 7)?,
        )
    }

    fn xmi_imm(&mut self, imm: u32) -> XmmMemImm {
        XmmMemImm::unwrap_new(RegMemImm::imm(imm))
    }

    fn insert_i8x16_lane_hole(&mut self, hole_idx: u8) -> VCodeConstant {
        let mask = -1i128 as u128;
        self.emit_u128_le_const(mask ^ (0xff << (hole_idx * 8)))
    }

    fn writable_invalid_gpr(&mut self) -> WritableGpr {
        let reg = Gpr::new(self.invalid_reg()).unwrap();
        WritableGpr::from_reg(reg)
    }

    fn box_synthetic_amode(&mut self, amode: &SyntheticAmode) -> BoxSyntheticAmode {
        Box::new(amode.clone())
    }

    ////////////////////////////////////////////////////////////////////////////
    ///// External assembler methods.
    ////////////////////////////////////////////////////////////////////////////

    fn is_imm8(&mut self, src: &GprMemImm) -> Option<u8> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(i8::try_from(simm32.signed()).ok()?.unsigned()),
            _ => None,
        }
    }

    fn is_imm8_xmm(&mut self, src: &XmmMemImm) -> Option<u8> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(i8::try_from(simm32.signed()).ok()?.unsigned()),
            _ => None,
        }
    }

    fn is_simm8(&mut self, src: &GprMemImm) -> Option<i8> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(i8::try_from(simm32.signed()).ok()?),
            _ => None,
        }
    }

    fn is_imm16(&mut self, src: &GprMemImm) -> Option<u16> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(i16::try_from(simm32.signed()).ok()?.unsigned()),
            _ => None,
        }
    }

    fn is_simm16(&mut self, src: &GprMemImm) -> Option<i16> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(i16::try_from(simm32.signed()).ok()?),
            _ => None,
        }
    }

    fn is_imm32(&mut self, src: &GprMemImm) -> Option<u32> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(simm32),
            _ => None,
        }
    }

    fn is_simm32(&mut self, src: &GprMemImm) -> Option<i32> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Imm { simm32 } => Some(simm32 as i32),
            _ => None,
        }
    }

    fn is_gpr(&mut self, src: &GprMemImm) -> Option<Gpr> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Reg { reg } => Gpr::new(reg),
            _ => None,
        }
    }

    fn is_xmm(&mut self, src: &XmmMem) -> Option<Xmm> {
        match src.clone().to_reg_mem() {
            RegMem::Reg { reg } => Xmm::new(reg),
            _ => None,
        }
    }

    fn is_gpr_mem(&mut self, src: &GprMemImm) -> Option<GprMem> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Reg { reg } => GprMem::new(RegMem::Reg { reg }),
            RegMemImm::Mem { addr } => GprMem::new(RegMem::Mem { addr }),
            _ => None,
        }
    }

    fn is_xmm_mem(&mut self, src: &XmmMemImm) -> Option<XmmMem> {
        match src.clone().to_reg_mem_imm() {
            RegMemImm::Reg { reg } => XmmMem::new(RegMem::Reg { reg }),
            RegMemImm::Mem { addr } => XmmMem::new(RegMem::Mem { addr }),
            _ => None,
        }
    }

    fn is_mem(&mut self, src: &XmmMem) -> Option<SyntheticAmode> {
        match src.clone().to_reg_mem() {
            RegMem::Reg { .. } => None,
            RegMem::Mem { addr } => Some(addr),
        }
    }

    // Custom constructors for `mulx` which only calculates the high half of the
    // result meaning that the same output operand is used in both destination
    // registers. This is in contrast to the assembler-generated version of this
    // instruction which generates two distinct temporary registers for output
    // which calculates both the high and low halves of the result.

    fn x64_mulxl_rvm_hi(&mut self, src1: &GprMem, src2: Gpr) -> Gpr {
        let ret = self.temp_writable_gpr();
        let src1 = self.convert_gpr_mem_to_assembler_read_gpr_mem(src1);
        let inst = asm::inst::mulxl_rvm::new(ret, ret, src1, src2);
        self.emit(&MInst::External { inst: inst.into() });
        ret.to_reg()
    }

    fn x64_mulxq_rvm_hi(&mut self, src1: &GprMem, src2: Gpr) -> Gpr {
        let ret = self.temp_writable_gpr();
        let src1 = self.convert_gpr_mem_to_assembler_read_gpr_mem(src1);
        let inst = asm::inst::mulxq_rvm::new(ret, ret, src1, src2);
        self.emit(&MInst::External { inst: inst.into() });
        ret.to_reg()
    }

    fn bt_imm(&mut self, val: u64) -> Option<u8> {
        if val.count_ones() == 1 {
            Some(u8::try_from(val.trailing_zeros()).unwrap())
        } else {
            None
        }
    }
}

impl IsleContext<'_, '_, MInst, X64Backend> {
    fn load_xmm_unaligned(&mut self, addr: SyntheticAmode) -> Xmm {
        let tmp = self.lower_ctx.alloc_tmp(types::F32X4).only_reg().unwrap();
        self.lower_ctx.emit(MInst::External {
            inst: asm::inst::movdqu_a::new(
                Writable::from_reg(Xmm::unwrap_new(tmp.to_reg())),
                asm::XmmMem::Mem(addr.into()),
            )
            .into(),
        });
        Xmm::unwrap_new(tmp.to_reg())
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_gpr_to_assembler_read_write_gpr(&mut self, read: Gpr) -> asm::Gpr<PairedGpr> {
        let write = self.lower_ctx.alloc_tmp(types::I64).only_reg().unwrap();
        let write = WritableGpr::from_writable_reg(write).unwrap();
        asm::Gpr::new(PairedGpr { read, write })
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_gpr_to_assembler_fixed_read_write_gpr<const E: u8>(
        &mut self,
        read: Gpr,
    ) -> asm::Fixed<PairedGpr, E> {
        let write = self.lower_ctx.alloc_tmp(types::I64).only_reg().unwrap();
        let write = WritableGpr::from_writable_reg(write).unwrap();
        asm::Fixed(PairedGpr { read, write })
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_xmm_to_assembler_read_write_xmm(&mut self, read: Xmm) -> asm::Xmm<PairedXmm> {
        let write = self.lower_ctx.alloc_tmp(types::F32X4).only_reg().unwrap();
        let write = WritableXmm::from_writable_reg(write).unwrap();
        asm::Xmm::new(PairedXmm { read, write })
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_gpr_mem_to_assembler_read_gpr_mem(&self, read: &GprMem) -> asm::GprMem<Gpr, Gpr> {
        match read.clone().into() {
            RegMem::Reg { reg } => asm::GprMem::Gpr(Gpr::new(reg).unwrap()),
            RegMem::Mem { addr } => asm::GprMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_xmm_mem_to_assembler_read_xmm_mem_aligned(
        &self,
        read: &XmmMemAligned,
    ) -> asm::XmmMem<Xmm, Gpr> {
        match read.clone().into() {
            RegMem::Reg { reg } => asm::XmmMem::Xmm(Xmm::new(reg).unwrap()),
            RegMem::Mem { addr } => asm::XmmMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_xmm_mem_to_assembler_read_xmm_mem(&self, read: &XmmMem) -> asm::XmmMem<Xmm, Gpr> {
        match read.clone().into() {
            RegMem::Reg { reg } => asm::XmmMem::Xmm(Xmm::new(reg).unwrap()),
            RegMem::Mem { addr } => asm::XmmMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_xmm_mem_to_assembler_write_xmm_mem(
        &self,
        write: &XmmMem,
    ) -> asm::XmmMem<Writable<Xmm>, Gpr> {
        match write.clone().into() {
            RegMem::Reg { reg } => asm::XmmMem::Xmm(Writable::from_reg(Xmm::new(reg).unwrap())),
            RegMem::Mem { addr } => asm::XmmMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_xmm_mem_to_assembler_write_xmm_mem_aligned(
        &self,
        write: &XmmMemAligned,
    ) -> asm::XmmMem<Writable<Xmm>, Gpr> {
        match write.clone().into() {
            RegMem::Reg { reg } => asm::XmmMem::Xmm(Writable::from_reg(Xmm::new(reg).unwrap())),
            RegMem::Mem { addr } => asm::XmmMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_gpr_mem_to_assembler_read_write_gpr_mem(
        &mut self,
        read: &GprMem,
    ) -> asm::GprMem<PairedGpr, Gpr> {
        match read.clone().into() {
            RegMem::Reg { reg } => asm::GprMem::Gpr(
                *self
                    .convert_gpr_to_assembler_read_write_gpr(Gpr::new(reg).unwrap())
                    .as_ref(),
            ),
            RegMem::Mem { addr } => asm::GprMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_gpr_mem_to_assembler_write_gpr_mem(
        &mut self,
        read: &GprMem,
    ) -> asm::GprMem<WritableGpr, Gpr> {
        match read.clone().into() {
            RegMem::Reg { reg } => asm::GprMem::Gpr(WritableGpr::from_reg(Gpr::new(reg).unwrap())),
            RegMem::Mem { addr } => asm::GprMem::Mem(addr.into()),
        }
    }

    /// Helper used by code generated by the `cranelift-assembler-x64` crate.
    fn convert_amode_to_assembler_amode(&mut self, amode: &SyntheticAmode) -> asm::Amode<Gpr> {
        amode.clone().into()
    }
}

// Since x64 doesn't have 8x16 shifts and we must use a 16x8 shift instead, we
// need to fix up the bits that migrate from one half of the lane to the
// other. Each 16-byte mask is indexed by the shift amount: e.g. if we shift
// right by 0 (no movement), we want to retain all the bits so we mask with
// `0xff`; if we shift right by 1, we want to retain all bits except the MSB so
// we mask with `0x7f`; etc.

#[rustfmt::skip] // Preserve 16 bytes (i.e. one mask) per row.
const I8X16_ISHL_MASKS: [u8; 128] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
    0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc,
    0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8,
    0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
    0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
    0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
];

#[rustfmt::skip] // Preserve 16 bytes (i.e. one mask) per row.
const I8X16_USHR_MASKS: [u8; 128] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f,
    0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
    0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f,
    0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
];
