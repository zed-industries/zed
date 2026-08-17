//! ISLE integration glue code for riscv64 lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
use generated_code::MInst;

// Types that the generated ISLE code uses via `use super::*`.
use self::generated_code::{FpuOPWidth, VecAluOpRR, VecLmul};
use crate::isa::riscv64::Riscv64Backend;
use crate::isa::riscv64::lower::args::{
    FReg, VReg, WritableFReg, WritableVReg, WritableXReg, XReg,
};
use crate::machinst::Reg;
use crate::machinst::{CallInfo, MachInst, isle::*};
use crate::machinst::{VCodeConstant, VCodeConstantData};
use crate::{
    ir::{
        AtomicRmwOp, BlockCall, ExternalName, Inst, InstructionData, MemFlags, Opcode, TrapCode,
        Value, ValueList, immediates::*, types::*,
    },
    isa::riscv64::inst::*,
    machinst::{ArgPair, CallArgList, CallRetList, InstOutput},
};
use regalloc2::PReg;
use std::boxed::Box;
use std::vec::Vec;
use wasmtime_math::{f32_cvt_to_int_bounds, f64_cvt_to_int_bounds};

type BoxCallInfo = Box<CallInfo<ExternalName>>;
type BoxCallIndInfo = Box<CallInfo<Reg>>;
type BoxReturnCallInfo = Box<ReturnCallInfo<ExternalName>>;
type BoxReturnCallIndInfo = Box<ReturnCallInfo<Reg>>;
type BoxExternalName = Box<ExternalName>;
type VecMachLabel = Vec<MachLabel>;
type VecArgPair = Vec<ArgPair>;

pub(crate) struct RV64IsleContext<'a, 'b, I, B>
where
    I: VCodeInst,
    B: LowerBackend,
{
    pub lower_ctx: &'a mut Lower<'b, I>,
    pub backend: &'a B,
    /// Precalucated value for the minimum vector register size. Will be 0 if
    /// vectors are not supported.
    min_vec_reg_size: u64,
}

impl<'a, 'b> RV64IsleContext<'a, 'b, MInst, Riscv64Backend> {
    fn new(lower_ctx: &'a mut Lower<'b, MInst>, backend: &'a Riscv64Backend) -> Self {
        Self {
            lower_ctx,
            backend,
            min_vec_reg_size: backend.isa_flags.min_vec_reg_size(),
        }
    }

    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.lower_ctx.f.dfg
    }
}

impl generated_code::Context for RV64IsleContext<'_, '_, MInst, Riscv64Backend> {
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

        Box::new(ReturnCallInfo {
            dest,
            uses,
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
            new_stack_arg_size,
        })
    }

    fn fpu_op_width_from_ty(&mut self, ty: Type) -> FpuOPWidth {
        match ty {
            F16 => FpuOPWidth::H,
            F32 => FpuOPWidth::S,
            F64 => FpuOPWidth::D,
            F128 => FpuOPWidth::Q,
            _ => unimplemented!("Unimplemented FPU Op Width: {ty}"),
        }
    }

    fn vreg_new(&mut self, r: Reg) -> VReg {
        VReg::new(r).unwrap()
    }
    fn writable_vreg_new(&mut self, r: WritableReg) -> WritableVReg {
        r.map(|wr| VReg::new(wr).unwrap())
    }
    fn writable_vreg_to_vreg(&mut self, arg0: WritableVReg) -> VReg {
        arg0.to_reg()
    }
    fn writable_vreg_to_writable_reg(&mut self, arg0: WritableVReg) -> WritableReg {
        arg0.map(|vr| vr.to_reg())
    }
    fn vreg_to_reg(&mut self, arg0: VReg) -> Reg {
        *arg0
    }
    fn xreg_new(&mut self, r: Reg) -> XReg {
        XReg::new(r).unwrap()
    }
    fn writable_xreg_new(&mut self, r: WritableReg) -> WritableXReg {
        r.map(|wr| XReg::new(wr).unwrap())
    }
    fn writable_xreg_to_xreg(&mut self, arg0: WritableXReg) -> XReg {
        arg0.to_reg()
    }
    fn writable_xreg_to_writable_reg(&mut self, arg0: WritableXReg) -> WritableReg {
        arg0.map(|xr| xr.to_reg())
    }
    fn xreg_to_reg(&mut self, arg0: XReg) -> Reg {
        *arg0
    }
    fn freg_new(&mut self, r: Reg) -> FReg {
        FReg::new(r).unwrap()
    }
    fn writable_freg_new(&mut self, r: WritableReg) -> WritableFReg {
        r.map(|wr| FReg::new(wr).unwrap())
    }
    fn writable_freg_to_freg(&mut self, arg0: WritableFReg) -> FReg {
        arg0.to_reg()
    }
    fn writable_freg_to_writable_reg(&mut self, arg0: WritableFReg) -> WritableReg {
        arg0.map(|fr| fr.to_reg())
    }
    fn freg_to_reg(&mut self, arg0: FReg) -> Reg {
        *arg0
    }

    fn min_vec_reg_size(&mut self) -> u64 {
        self.min_vec_reg_size
    }

    #[inline]
    fn ty_vec_fits_in_register(&mut self, ty: Type) -> Option<Type> {
        if ty.is_vector() && (ty.bits() as u64) <= self.min_vec_reg_size() {
            Some(ty)
        } else {
            None
        }
    }

    fn ty_supported(&mut self, ty: Type) -> Option<Type> {
        let lane_type = ty.lane_type();
        let supported = match ty {
            // Scalar integers are always supported
            ty if ty.is_int() => true,
            // Floating point types depend on certain extensions
            // F32 depends on the F extension
            // If F32 is supported, then the registers are also large enough for F16
            F16 | F32 => self.backend.isa_flags.has_f(),
            // F64 depends on the D extension
            F64 => self.backend.isa_flags.has_d(),
            // F128 is currently stored in a pair of integer registers
            F128 => true,

            // The base vector extension supports all integer types, up to 64 bits
            // as long as they fit in a register
            ty if self.ty_vec_fits_in_register(ty).is_some()
                && lane_type.is_int()
                && lane_type.bits() <= 64 =>
            {
                true
            }

            // If the vector type has floating point lanes then the spec states:
            //
            // Vector instructions where any floating-point vector operand’s EEW is not a
            // supported floating-point type width (which includes when FLEN < SEW) are reserved.
            //
            // So we also have to check if we support the scalar version of the type.
            ty if self.ty_vec_fits_in_register(ty).is_some()
                && lane_type.is_float()
                && self.ty_supported(lane_type).is_some()
                // Additionally the base V spec only supports 32 and 64 bit floating point types.
                && (lane_type.bits() == 32 || lane_type.bits() == 64 || (lane_type.bits() == 16 && self.backend.isa_flags.has_zvfh())) =>
            {
                true
            }

            // Otherwise do not match
            _ => false,
        };

        if supported { Some(ty) } else { None }
    }

    fn ty_supported_float_size(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported(ty)
            .filter(|&ty| ty.is_float() && ty != F128)
    }

    fn ty_supported_float_min(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported_float_size(ty)
            .filter(|&ty| ty != F16 || self.backend.isa_flags.has_zfhmin())
    }

    fn ty_supported_float_full(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported_float_min(ty)
            .filter(|&ty| ty != F16 || self.backend.isa_flags.has_zfh())
    }

    fn ty_supported_vec(&mut self, ty: Type) -> Option<Type> {
        self.ty_supported(ty).filter(|ty| ty.is_vector())
    }

    fn ty_reg_pair(&mut self, ty: Type) -> Option<Type> {
        match ty {
            I128 | F128 => Some(ty),
            _ => None,
        }
    }

    fn load_ra(&mut self) -> Reg {
        if self.backend.flags.preserve_frame_pointers() {
            let tmp = self.temp_writable_reg(I64);
            self.emit(&MInst::Load {
                rd: tmp,
                op: LoadOP::Ld,
                flags: MemFlags::trusted(),
                from: AMode::FPOffset(8),
            });
            tmp.to_reg()
        } else {
            link_reg()
        }
    }

    fn label_to_br_target(&mut self, label: MachLabel) -> CondBrTarget {
        CondBrTarget::Label(label)
    }

    fn imm12_and(&mut self, imm: Imm12, x: u64) -> Imm12 {
        Imm12::from_i16(imm.as_i16() & (x as i16))
    }

    fn fli_constant_from_u64(&mut self, ty: Type, imm: u64) -> Option<FliConstant> {
        FliConstant::maybe_from_u64(ty, imm)
    }

    fn fli_constant_from_negated_u64(&mut self, ty: Type, imm: u64) -> Option<FliConstant> {
        let negated_imm = match ty {
            F64 => imm ^ 0x8000_0000_0000_0000,
            F32 => imm ^ 0x8000_0000,
            F16 => imm ^ 0x8000,
            _ => unimplemented!(),
        };

        FliConstant::maybe_from_u64(ty, negated_imm)
    }

    fn i64_generate_imm(&mut self, imm: i64) -> Option<(Imm20, Imm12)> {
        MInst::generate_imm(imm as u64)
    }

    fn i64_shift_for_lui(&mut self, imm: i64) -> Option<(u64, Imm12)> {
        let trailing = imm.trailing_zeros();
        if trailing < 12 {
            return None;
        }

        let shift = Imm12::from_i16(trailing as i16 - 12);
        let base = (imm as u64) >> trailing;
        Some((base, shift))
    }

    fn i64_shift(&mut self, imm: i64) -> Option<(i64, Imm12)> {
        let trailing = imm.trailing_zeros();
        // We can do without this condition but in this case there is no need to go further
        if trailing == 0 {
            return None;
        }

        let shift = Imm12::from_i16(trailing as i16);
        let base = imm >> trailing;
        Some((base, shift))
    }

    #[inline]
    fn emit(&mut self, arg0: &MInst) -> Unit {
        self.lower_ctx.emit(arg0.clone());
    }
    #[inline]
    fn imm12_from_u64(&mut self, arg0: u64) -> Option<Imm12> {
        Imm12::maybe_from_u64(arg0)
    }
    #[inline]
    fn imm12_from_i64(&mut self, arg0: i64) -> Option<Imm12> {
        Imm12::maybe_from_i64(arg0)
    }
    #[inline]
    fn imm12_is_zero(&mut self, imm: Imm12) -> Option<()> {
        if imm.as_i16() == 0 { Some(()) } else { None }
    }

    #[inline]
    fn imm20_from_u64(&mut self, arg0: u64) -> Option<Imm20> {
        Imm20::maybe_from_u64(arg0)
    }
    #[inline]
    fn imm20_from_i64(&mut self, arg0: i64) -> Option<Imm20> {
        Imm20::maybe_from_i64(arg0)
    }
    #[inline]
    fn imm20_is_zero(&mut self, imm: Imm20) -> Option<()> {
        if imm.as_i32() == 0 { Some(()) } else { None }
    }

    #[inline]
    fn imm5_from_u64(&mut self, arg0: u64) -> Option<Imm5> {
        Imm5::maybe_from_i8(i8::try_from(arg0 as i64).ok()?)
    }
    #[inline]
    fn imm5_from_i64(&mut self, arg0: i64) -> Option<Imm5> {
        Imm5::maybe_from_i8(i8::try_from(arg0).ok()?)
    }
    #[inline]
    fn i8_to_imm5(&mut self, arg0: i8) -> Option<Imm5> {
        Imm5::maybe_from_i8(arg0)
    }
    #[inline]
    fn uimm5_bitcast_to_imm5(&mut self, arg0: UImm5) -> Imm5 {
        Imm5::from_bits(arg0.bits() as u8)
    }
    #[inline]
    fn uimm5_from_u8(&mut self, arg0: u8) -> Option<UImm5> {
        UImm5::maybe_from_u8(arg0)
    }
    #[inline]
    fn uimm5_from_u64(&mut self, arg0: u64) -> Option<UImm5> {
        arg0.try_into().ok().and_then(UImm5::maybe_from_u8)
    }
    #[inline]
    fn writable_zero_reg(&mut self) -> WritableReg {
        writable_zero_reg()
    }
    #[inline]
    fn zero_reg(&mut self) -> XReg {
        XReg::new(zero_reg()).unwrap()
    }
    fn is_non_zero_reg(&mut self, reg: XReg) -> Option<()> {
        if reg != self.zero_reg() {
            Some(())
        } else {
            None
        }
    }
    fn is_zero_reg(&mut self, reg: XReg) -> Option<()> {
        if reg == self.zero_reg() {
            Some(())
        } else {
            None
        }
    }
    #[inline]
    fn imm_from_bits(&mut self, val: u64) -> Imm12 {
        Imm12::maybe_from_u64(val).unwrap()
    }
    #[inline]
    fn imm_from_neg_bits(&mut self, val: i64) -> Imm12 {
        Imm12::maybe_from_i64(val).unwrap()
    }

    fn frm_bits(&mut self, frm: &FRM) -> UImm5 {
        UImm5::maybe_from_u8(frm.bits()).unwrap()
    }

    fn imm12_const(&mut self, val: i32) -> Imm12 {
        if let Some(res) = Imm12::maybe_from_i64(val as i64) {
            res
        } else {
            panic!("Unable to make an Imm12 value from {val}")
        }
    }
    fn imm12_const_add(&mut self, val: i32, add: i32) -> Imm12 {
        Imm12::maybe_from_i64((val + add) as i64).unwrap()
    }
    fn imm12_add(&mut self, val: Imm12, add: i32) -> Option<Imm12> {
        Imm12::maybe_from_i64((i32::from(val.as_i16()) + add).into())
    }

    //
    fn gen_shamt(&mut self, ty: Type, shamt: XReg) -> ValueRegs {
        let ty_bits = if ty.bits() > 64 { 64 } else { ty.bits() };
        let ty_bits = i16::try_from(ty_bits).unwrap();
        let shamt = {
            let tmp = self.temp_writable_reg(I64);
            self.emit(&MInst::AluRRImm12 {
                alu_op: AluOPRRI::Andi,
                rd: tmp,
                rs: shamt.to_reg(),
                imm12: Imm12::from_i16(ty_bits - 1),
            });
            tmp.to_reg()
        };
        let len_sub_shamt = {
            let tmp = self.temp_writable_reg(I64);
            self.emit(&MInst::load_imm12(tmp, Imm12::from_i16(ty_bits)));
            let len_sub_shamt = self.temp_writable_reg(I64);
            self.emit(&MInst::AluRRR {
                alu_op: AluOPRRR::Sub,
                rd: len_sub_shamt,
                rs1: tmp.to_reg(),
                rs2: shamt,
            });
            len_sub_shamt.to_reg()
        };
        ValueRegs::two(shamt, len_sub_shamt)
    }

    fn has_v(&mut self) -> bool {
        self.backend.isa_flags.has_v()
    }

    fn has_m(&mut self) -> bool {
        self.backend.isa_flags.has_m()
    }

    fn has_zfa(&mut self) -> bool {
        self.backend.isa_flags.has_zfa()
    }

    fn has_zfhmin(&mut self) -> bool {
        self.backend.isa_flags.has_zfhmin()
    }

    fn has_zfh(&mut self) -> bool {
        self.backend.isa_flags.has_zfh()
    }

    fn has_zvfh(&mut self) -> bool {
        self.backend.isa_flags.has_zvfh()
    }

    fn has_zbkb(&mut self) -> bool {
        self.backend.isa_flags.has_zbkb()
    }

    fn has_zba(&mut self) -> bool {
        self.backend.isa_flags.has_zba()
    }

    fn has_zbb(&mut self) -> bool {
        self.backend.isa_flags.has_zbb()
    }

    fn has_zbc(&mut self) -> bool {
        self.backend.isa_flags.has_zbc()
    }

    fn has_zbs(&mut self) -> bool {
        self.backend.isa_flags.has_zbs()
    }

    fn has_zicond(&mut self) -> bool {
        self.backend.isa_flags.has_zicond()
    }

    fn gen_reg_offset_amode(&mut self, base: Reg, offset: i64) -> AMode {
        AMode::RegOffset(base, offset)
    }

    fn gen_sp_offset_amode(&mut self, offset: i64) -> AMode {
        AMode::SPOffset(offset)
    }

    fn gen_fp_offset_amode(&mut self, offset: i64) -> AMode {
        AMode::FPOffset(offset)
    }

    fn gen_stack_slot_amode(&mut self, ss: StackSlot, offset: i64) -> AMode {
        // Offset from beginning of stackslot area.
        let stack_off = self.lower_ctx.abi().sized_stackslot_offsets()[ss] as i64;
        let sp_off: i64 = stack_off + offset;
        AMode::SlotOffset(sp_off)
    }

    fn gen_const_amode(&mut self, c: VCodeConstant) -> AMode {
        AMode::Const(c)
    }

    fn valid_atomic_transaction(&mut self, ty: Type) -> Option<Type> {
        if ty.is_int() && ty.bits() <= 64 {
            Some(ty)
        } else {
            None
        }
    }
    fn is_atomic_rmw_max_etc(&mut self, op: &AtomicRmwOp) -> Option<(AtomicRmwOp, bool)> {
        let op = *op;
        match op {
            crate::ir::AtomicRmwOp::Umin => Some((op, false)),
            crate::ir::AtomicRmwOp::Umax => Some((op, false)),
            crate::ir::AtomicRmwOp::Smin => Some((op, true)),
            crate::ir::AtomicRmwOp::Smax => Some((op, true)),
            _ => None,
        }
    }

    fn sinkable_inst(&mut self, val: Value) -> Option<Inst> {
        self.is_sinkable_inst(val)
    }

    fn load_op(&mut self, ty: Type) -> LoadOP {
        LoadOP::from_type(ty)
    }
    fn store_op(&mut self, ty: Type) -> StoreOP {
        StoreOP::from_type(ty)
    }
    fn load_ext_name(&mut self, name: ExternalName, offset: i64) -> Reg {
        let tmp = self.temp_writable_reg(I64);
        self.emit(&MInst::LoadExtName {
            rd: tmp,
            name: Box::new(name),
            offset,
        });
        tmp.to_reg()
    }

    fn gen_stack_addr(&mut self, slot: StackSlot, offset: Offset32) -> Reg {
        let result = self.temp_writable_reg(I64);
        let i = self
            .lower_ctx
            .abi()
            .sized_stackslot_addr(slot, i64::from(offset) as u32, result);
        self.emit(&i);
        result.to_reg()
    }
    fn atomic_amo(&mut self) -> AMO {
        AMO::SeqCst
    }

    fn lower_br_table(&mut self, index: Reg, targets: &[MachLabel]) -> Unit {
        let tmp1 = self.temp_writable_reg(I64);
        let tmp2 = self.temp_writable_reg(I64);
        self.emit(&MInst::BrTable {
            index,
            tmp1,
            tmp2,
            targets: targets.to_vec(),
        });
    }

    fn fp_reg(&mut self) -> PReg {
        px_reg(8)
    }

    fn sp_reg(&mut self) -> PReg {
        px_reg(2)
    }

    #[inline]
    fn int_compare(&mut self, kind: &IntCC, rs1: XReg, rs2: XReg) -> IntegerCompare {
        IntegerCompare {
            kind: *kind,
            rs1: rs1.to_reg(),
            rs2: rs2.to_reg(),
        }
    }

    #[inline]
    fn int_compare_decompose(&mut self, cmp: IntegerCompare) -> (IntCC, XReg, XReg) {
        (cmp.kind, self.xreg_new(cmp.rs1), self.xreg_new(cmp.rs2))
    }

    #[inline]
    fn vstate_from_type(&mut self, ty: Type) -> VState {
        VState::from_type(ty)
    }

    #[inline]
    fn vstate_mf2(&mut self, vs: VState) -> VState {
        VState {
            vtype: VType {
                lmul: VecLmul::LmulF2,
                ..vs.vtype
            },
            ..vs
        }
    }

    fn vec_alu_rr_dst_type(&mut self, op: &VecAluOpRR) -> Type {
        MInst::canonical_type_for_rc(op.dst_regclass())
    }

    fn bclr_imm(&mut self, ty: Type, i: u64) -> Option<Imm12> {
        // Only consider those bits in the immediate which are up to the width
        // of `ty`.
        let neg = !i & (u64::MAX >> (64 - ty.bits()));
        if neg.count_ones() != 1 {
            return None;
        }
        Imm12::maybe_from_u64(neg.trailing_zeros().into())
    }

    fn binvi_imm(&mut self, i: u64) -> Option<Imm12> {
        if i.count_ones() != 1 {
            return None;
        }
        Imm12::maybe_from_u64(i.trailing_zeros().into())
    }
    fn bseti_imm(&mut self, i: u64) -> Option<Imm12> {
        self.binvi_imm(i)
    }

    fn fcvt_smin_bound(&mut self, float: Type, int: Type, saturating: bool) -> u64 {
        match (int, float) {
            // Saturating cases for larger integers are handled using the
            // `fcvt.{w,d}.{s,d}` instruction directly, that automatically
            // saturates up/down to the correct limit.
            //
            // NB: i32/i64 don't use this function because the native RISC-V
            // instruction does everything we already need, so only cases for
            // i8/i16 are listed here.
            (I8, F32) if saturating => f32::from(i8::MIN).to_bits().into(),
            (I8, F64) if saturating => f64::from(i8::MIN).to_bits(),
            (I16, F32) if saturating => f32::from(i16::MIN).to_bits().into(),
            (I16, F64) if saturating => f64::from(i16::MIN).to_bits(),

            (_, F32) if !saturating => f32_cvt_to_int_bounds(true, int.bits()).0.to_bits().into(),
            (_, F64) if !saturating => f64_cvt_to_int_bounds(true, int.bits()).0.to_bits(),
            _ => unimplemented!(),
        }
    }

    fn fcvt_smax_bound(&mut self, float: Type, int: Type, saturating: bool) -> u64 {
        // NB: see `fcvt_smin_bound` for some more comments
        match (int, float) {
            (I8, F32) if saturating => f32::from(i8::MAX).to_bits().into(),
            (I8, F64) if saturating => f64::from(i8::MAX).to_bits(),
            (I16, F32) if saturating => f32::from(i16::MAX).to_bits().into(),
            (I16, F64) if saturating => f64::from(i16::MAX).to_bits(),

            (_, F32) if !saturating => f32_cvt_to_int_bounds(true, int.bits()).1.to_bits().into(),
            (_, F64) if !saturating => f64_cvt_to_int_bounds(true, int.bits()).1.to_bits(),
            _ => unimplemented!(),
        }
    }

    fn fcvt_umax_bound(&mut self, float: Type, int: Type, saturating: bool) -> u64 {
        // NB: see `fcvt_smin_bound` for some more comments
        match (int, float) {
            (I8, F32) if saturating => f32::from(u8::MAX).to_bits().into(),
            (I8, F64) if saturating => f64::from(u8::MAX).to_bits(),
            (I16, F32) if saturating => f32::from(u16::MAX).to_bits().into(),
            (I16, F64) if saturating => f64::from(u16::MAX).to_bits(),

            (_, F32) if !saturating => f32_cvt_to_int_bounds(false, int.bits()).1.to_bits().into(),
            (_, F64) if !saturating => f64_cvt_to_int_bounds(false, int.bits()).1.to_bits(),
            _ => unimplemented!(),
        }
    }

    fn fcvt_umin_bound(&mut self, float: Type, saturating: bool) -> u64 {
        assert!(!saturating);
        match float {
            F32 => (-1.0f32).to_bits().into(),
            F64 => (-1.0f64).to_bits(),
            _ => unimplemented!(),
        }
    }
}

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &Riscv64Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = RV64IsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

/// The main entry point for branch lowering with ISLE.
pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &Riscv64Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = RV64IsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}
