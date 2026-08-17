//! Implementation of a standard Pulley ABI.

use super::{PulleyFlags, PulleyTargetKind, inst::*};
use crate::isa::pulley_shared::PointerWidth;
use crate::{
    CodegenResult,
    ir::{self, MemFlags, Signature, types::*},
    isa,
    machinst::*,
    settings,
};
use alloc::vec::Vec;
use core::marker::PhantomData;
use cranelift_bitset::ScalarBitSet;
use regalloc2::{MachineEnv, PReg, PRegSet};
use smallvec::{SmallVec, smallvec};
use std::borrow::ToOwned;
use std::sync::OnceLock;

/// Support for the Pulley ABI from the callee side (within a function body).
pub(crate) type PulleyCallee<P> = Callee<PulleyMachineDeps<P>>;

/// Pulley-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct PulleyMachineDeps<P>
where
    P: PulleyTargetKind,
{
    _phantom: PhantomData<P>,
}

impl<P> ABIMachineSpec for PulleyMachineDeps<P>
where
    P: PulleyTargetKind,
{
    type I = InstAndKind<P>;
    type F = PulleyFlags;

    /// This is the limit for the size of argument and return-value areas on the
    /// stack. We place a reasonable limit here to avoid integer overflow issues
    /// with 32-bit arithmetic: for now, 128 MB.
    const STACK_ARG_RET_SIZE_LIMIT: u32 = 128 * 1024 * 1024;

    fn word_bits() -> u32 {
        P::pointer_width().bits().into()
    }

    /// Return required stack alignment in bytes.
    fn stack_align(_call_conv: isa::CallConv) -> u32 {
        16
    }

    fn compute_arg_locs(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        params: &[ir::AbiParam],
        args_or_rets: ArgsOrRets,
        add_ret_area_ptr: bool,
        mut args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        // NB: make sure this method stays in sync with
        // `cranelift_pulley::interp::Vm::call`.
        //
        // In general we use the first half of all register banks as argument
        // passing registers because, well, why not for now. Currently the only
        // exception is x15 which is reserved as a single caller-saved register
        // not used for arguments. This is used in `ReturnCallIndirect` to hold
        // the location of where we're jumping to.

        let x_end = 14;
        let f_end = 15;
        let v_end = 15;

        let mut next_x_reg = 0;
        let mut next_f_reg = 0;
        let mut next_v_reg = 0;
        let mut next_stack: u32 = 0;

        let ret_area_ptr = if add_ret_area_ptr {
            debug_assert_eq!(args_or_rets, ArgsOrRets::Args);
            next_x_reg += 1;
            Some(ABIArg::reg(
                x_reg(next_x_reg - 1).to_real_reg().unwrap(),
                I64,
                ir::ArgumentExtension::None,
                ir::ArgumentPurpose::Normal,
            ))
        } else {
            None
        };

        for param in params {
            // Find the regclass(es) of the register(s) used to store a value of
            // this type.
            let (rcs, reg_tys) = Self::I::rc_for_type(param.value_type)?;

            let mut slots = ABIArgSlotVec::new();
            for (rc, reg_ty) in rcs.iter().zip(reg_tys.iter()) {
                let next_reg = if (next_x_reg <= x_end) && *rc == RegClass::Int {
                    let x = Some(x_reg(next_x_reg));
                    next_x_reg += 1;
                    x
                } else if (next_f_reg <= f_end) && *rc == RegClass::Float {
                    let f = Some(f_reg(next_f_reg));
                    next_f_reg += 1;
                    f
                } else if (next_v_reg <= v_end) && *rc == RegClass::Vector {
                    let v = Some(v_reg(next_v_reg));
                    next_v_reg += 1;
                    v
                } else {
                    None
                };

                if let Some(reg) = next_reg {
                    slots.push(ABIArgSlot::Reg {
                        reg: reg.to_real_reg().unwrap(),
                        ty: *reg_ty,
                        extension: param.extension,
                    });
                } else {
                    if args_or_rets == ArgsOrRets::Rets && !flags.enable_multi_ret_implicit_sret() {
                        return Err(crate::CodegenError::Unsupported(
                            "Too many return values to fit in registers. \
                            Use a StructReturn argument instead. (#9510)"
                                .to_owned(),
                        ));
                    }

                    // Compute size and 16-byte stack alignment happens
                    // separately after all args.
                    let size = reg_ty.bits() / 8;
                    let size = std::cmp::max(size, 8);

                    // Align.
                    debug_assert!(size.is_power_of_two());
                    next_stack = align_to(next_stack, size);

                    slots.push(ABIArgSlot::Stack {
                        offset: i64::from(next_stack),
                        ty: *reg_ty,
                        extension: param.extension,
                    });

                    next_stack += size;
                }
            }

            args.push(ABIArg::Slots {
                slots,
                purpose: param.purpose,
            });
        }

        let pos = if let Some(ret_area_ptr) = ret_area_ptr {
            args.push_non_formal(ret_area_ptr);
            Some(args.args().len() - 1)
        } else {
            None
        };

        next_stack = align_to(next_stack, Self::stack_align(call_conv));

        Ok((next_stack, pos))
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Self::I {
        let mut flags = MemFlags::trusted();
        // Stack loads/stores of vectors always use little-endianess to avoid
        // implementing a byte-swap of vectors on big-endian platforms.
        if ty.is_vector() {
            flags.set_endianness(ir::Endianness::Little);
        }
        Inst::gen_load(into_reg, mem.into(), ty, flags).into()
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Self::I {
        let mut flags = MemFlags::trusted();
        // Stack loads/stores of vectors always use little-endianess to avoid
        // implementing a byte-swap of vectors on big-endian platforms.
        if ty.is_vector() {
            flags.set_endianness(ir::Endianness::Little);
        }
        Inst::gen_store(mem.into(), from_reg, ty, flags).into()
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Self::I {
        Self::I::gen_move(to_reg, from_reg, ty)
    }

    fn gen_extend(
        dst: Writable<Reg>,
        src: Reg,
        signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Self::I {
        assert!(from_bits < to_bits);
        let src = XReg::new(src).unwrap();
        let dst = dst.try_into().unwrap();
        match (signed, from_bits) {
            (true, 8) => RawInst::Sext8 { dst, src }.into(),
            (true, 16) => RawInst::Sext16 { dst, src }.into(),
            (true, 32) => RawInst::Sext32 { dst, src }.into(),
            (false, 8) => RawInst::Zext8 { dst, src }.into(),
            (false, 16) => RawInst::Zext16 { dst, src }.into(),
            (false, 32) => RawInst::Zext32 { dst, src }.into(),
            _ => unimplemented!("extend {from_bits} to {to_bits} as signed? {signed}"),
        }
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        specified
    }

    fn gen_args(args: Vec<ArgPair>) -> Self::I {
        Inst::Args { args }.into()
    }

    fn gen_rets(rets: Vec<RetPair>) -> Self::I {
        Inst::Rets { rets }.into()
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        spilltmp_reg()
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Self::I> {
        let dst = into_reg.try_into().unwrap();
        let imm = imm as i32;
        smallvec![
            RawInst::Xconst32 { dst, imm }.into(),
            RawInst::Xadd32 {
                dst,
                src1: from_reg.try_into().unwrap(),
                src2: dst.to_reg(),
            }
            .into()
        ]
    }

    fn gen_stack_lower_bound_trap(_limit_reg: Reg) -> SmallInstVec<Self::I> {
        unimplemented!("pulley shouldn't need stack bound checks")
    }

    fn gen_get_stack_addr(mem: StackAMode, dst: Writable<Reg>) -> Self::I {
        let dst = dst.to_reg();
        let dst = XReg::new(dst).unwrap();
        let dst = WritableXReg::from_reg(dst);
        let mem = mem.into();
        Inst::LoadAddr { dst, mem }.into()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Self::I {
        let base = XReg::try_from(base).unwrap();
        let mem = Amode::RegOffset { base, offset };
        Inst::gen_load(into_reg, mem, ty, MemFlags::trusted()).into()
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Self::I {
        let base = XReg::try_from(base).unwrap();
        let mem = Amode::RegOffset { base, offset };
        Inst::gen_store(mem, from_reg, ty, MemFlags::trusted()).into()
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Self::I> {
        if amount == 0 {
            return smallvec![];
        }

        let inst = if amount < 0 {
            let amount = amount.checked_neg().unwrap();
            if let Ok(amt) = u32::try_from(amount) {
                RawInst::StackAlloc32 { amt }
            } else {
                unreachable!()
            }
        } else {
            if let Ok(amt) = u32::try_from(amount) {
                RawInst::StackFree32 { amt }
            } else {
                unreachable!()
            }
        };
        smallvec![inst.into()]
    }

    /// Generates the entire prologue for the function.
    ///
    /// Note that this is different from other backends where it's not spread
    /// out among a few individual functions. That's because the goal here is to
    /// generate a single macro-instruction for the entire prologue in the most
    /// common cases and we don't want to spread the logic over multiple
    /// functions.
    ///
    /// The general machinst methods are split to accommodate stack checks and
    /// things like stack probes, all of which are empty on Pulley because
    /// Pulley has its own stack check mechanism.
    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &PulleyFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        let mut insts = SmallVec::new();

        let incoming_args_diff = frame_layout.tail_args_size - frame_layout.incoming_args_size;
        if incoming_args_diff > 0 {
            // Decrement SP by the amount of additional incoming argument space
            // we need
            insts.extend(Self::gen_sp_reg_adjust(-(incoming_args_diff as i32)));
        }

        let style = frame_layout.pulley_frame_style();

        match &style {
            FrameStyle::None => {}
            FrameStyle::PulleyBasicSetup { frame_size } => {
                insts.push(RawInst::PushFrame.into());
                insts.extend(Self::gen_sp_reg_adjust(
                    -i32::try_from(*frame_size).unwrap(),
                ));
            }
            FrameStyle::PulleySetupAndSaveClobbers {
                frame_size,
                saved_by_pulley,
            } => insts.push(
                RawInst::PushFrameSave {
                    amt: *frame_size,
                    regs: pulley_interpreter::UpperRegSet::from_bitset(*saved_by_pulley),
                }
                .into(),
            ),
            FrameStyle::Manual { frame_size } => insts.extend(Self::gen_sp_reg_adjust(
                -i32::try_from(*frame_size).unwrap(),
            )),
        }

        for (offset, ty, reg) in frame_layout.manually_managed_clobbers(&style) {
            insts.push(
                Inst::gen_store(Amode::SpOffset { offset }, reg, ty, MemFlags::trusted()).into(),
            );
        }

        insts
    }

    /// Reverse of `gen_prologue_frame_setup`.
    fn gen_epilogue_frame_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &PulleyFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        let mut insts = SmallVec::new();

        let style = frame_layout.pulley_frame_style();

        // Restore clobbered registers that are manually managed in Cranelift.
        for (offset, ty, reg) in frame_layout.manually_managed_clobbers(&style) {
            insts.push(
                Inst::gen_load(
                    Writable::from_reg(reg),
                    Amode::SpOffset { offset },
                    ty,
                    MemFlags::trusted(),
                )
                .into(),
            );
        }

        // Perform the inverse of `gen_prologue_frame_setup`.
        match &style {
            FrameStyle::None => {}
            FrameStyle::PulleyBasicSetup { frame_size } => {
                insts.extend(Self::gen_sp_reg_adjust(i32::try_from(*frame_size).unwrap()));
                insts.push(RawInst::PopFrame.into());
            }
            FrameStyle::PulleySetupAndSaveClobbers {
                frame_size,
                saved_by_pulley,
            } => insts.push(
                RawInst::PopFrameRestore {
                    amt: *frame_size,
                    regs: pulley_interpreter::UpperRegSet::from_bitset(*saved_by_pulley),
                }
                .into(),
            ),
            FrameStyle::Manual { frame_size } => {
                insts.extend(Self::gen_sp_reg_adjust(i32::try_from(*frame_size).unwrap()))
            }
        }

        insts
    }

    fn gen_return(
        call_conv: isa::CallConv,
        _isa_flags: &PulleyFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        let mut insts = SmallVec::new();

        // Handle final stack adjustments for the tail-call ABI.
        if call_conv == isa::CallConv::Tail && frame_layout.tail_args_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(
                frame_layout.tail_args_size.try_into().unwrap(),
            ));
        }
        insts.push(RawInst::Ret {}.into());

        insts
    }

    fn gen_probestack(_insts: &mut SmallInstVec<Self::I>, _frame_size: u32) {
        // Pulley doesn't implement stack probes since all stack pointer
        // decrements are checked already.
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        // Note that this is intentionally empty because everything necessary
        // was already done in `gen_prologue_frame_setup`.
        SmallVec::new()
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        // Intentionally empty as restores happen for Pulley in `gen_return`.
        SmallVec::new()
    }

    fn gen_memcpy<F: FnMut(Type) -> Writable<Reg>>(
        _call_conv: isa::CallConv,
        _dst: Reg,
        _src: Reg,
        _size: usize,
        _alloc_tmp: F,
    ) -> SmallVec<[Self::I; 8]> {
        todo!()
    }

    fn get_number_of_spillslots_for_value(
        rc: RegClass,
        _target_vector_bytes: u32,
        _isa_flags: &PulleyFlags,
    ) -> u32 {
        // Spill slots are the size of a "word" or a pointer, but Pulley
        // registers are 8-byte for integers/floats regardless of pointer size.
        // Calculate the number of slots necessary to store 8 bytes.
        let slots_for_8bytes = match P::pointer_width() {
            PointerWidth::PointerWidth32 => 2,
            PointerWidth::PointerWidth64 => 1,
        };
        match rc {
            // Int/float registers are 8-bytes
            RegClass::Int | RegClass::Float => slots_for_8bytes,
            // Vector registers are 16 bytes
            RegClass::Vector => 2 * slots_for_8bytes,
        }
    }

    fn get_machine_env(_flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
        MACHINE_ENV.get_or_init(create_reg_environment)
    }

    fn get_regs_clobbered_by_call(
        _call_conv_of_callee: isa::CallConv,
        is_exception: bool,
    ) -> PRegSet {
        if is_exception {
            ALL_CLOBBERS
        } else {
            DEFAULT_CLOBBERS
        }
    }

    fn compute_frame_layout(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        _sig: &Signature,
        regs: &[Writable<RealReg>],
        is_leaf: bool,
        incoming_args_size: u32,
        tail_args_size: u32,
        stackslots_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> FrameLayout {
        let mut regs: Vec<Writable<RealReg>> = regs
            .iter()
            .cloned()
            .filter(|r| DEFAULT_CALLEE_SAVES.contains(r.to_reg().into()))
            .collect();

        regs.sort_unstable();

        // Compute clobber size.
        let clobber_size = compute_clobber_size(&regs);

        // Compute linkage frame size.
        let setup_area_size = if flags.preserve_frame_pointers()
            || !is_leaf
            // The function arguments that are passed on the stack are addressed
            // relative to the Frame Pointer.
            || incoming_args_size > 0
            || clobber_size > 0
            || fixed_frame_storage_size > 0
        {
            P::pointer_width().bytes() * 2 // FP, LR
        } else {
            0
        };

        FrameLayout {
            word_bytes: u32::from(P::pointer_width().bytes()),
            incoming_args_size,
            tail_args_size,
            setup_area_size: setup_area_size.into(),
            clobber_size,
            fixed_frame_storage_size,
            stackslots_size,
            outgoing_args_size,
            clobbered_callee_saves: regs,
        }
    }

    fn gen_inline_probestack(
        _insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        _frame_size: u32,
        _guard_size: u32,
    ) {
        // Pulley doesn't need inline probestacks because it always checks stack
        // decrements.
    }

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        // Use x15 as a temp if needed: clobbered, not a
        // retval.
        Writable::from_reg(regs::x_reg(15))
    }

    fn exception_payload_regs(_call_conv: isa::CallConv) -> &'static [Reg] {
        const PAYLOAD_REGS: &'static [Reg] = &[
            Reg::from_real_reg(regs::px_reg(0)),
            Reg::from_real_reg(regs::px_reg(1)),
        ];
        PAYLOAD_REGS
    }
}

/// Different styles of management of fp/lr and clobbered registers.
///
/// This helps decide, depending on Cranelift settings and frame layout, what
/// macro instruction is used to setup the pulley frame.
enum FrameStyle {
    /// No management is happening, fp/lr aren't saved by Pulley or Cranelift.
    /// No stack is being allocated either.
    None,

    /// Pulley saves the fp/lr combo and then stack adjustments/clobbers are
    /// handled manually.
    PulleyBasicSetup { frame_size: u32 },

    /// Pulley is managing the fp/lr combo, the stack size, and clobbered
    /// X-class registers.
    ///
    /// Note that `saved_by_pulley` is not the exhaustive set of clobbered
    /// registers. It's only those that are part of the `PushFrameSave`
    /// instruction.
    PulleySetupAndSaveClobbers {
        /// The size of the frame, including clobbers, that's being allocated.
        frame_size: u16,
        /// Registers that pulley is saving/restoring.
        saved_by_pulley: ScalarBitSet<u16>,
    },

    /// Cranelift is manually managing everything, both clobbers and stack
    /// increments/decrements.
    ///
    /// Note that fp/lr are not saved in this mode.
    Manual {
        /// The size of the stack being allocated.
        frame_size: u32,
    },
}

/// Pulley-specific helpers when dealing with ABI code.
impl FrameLayout {
    /// Whether or not this frame saves fp/lr.
    fn setup_frame(&self) -> bool {
        self.setup_area_size > 0
    }

    /// Returns the stack size allocated by this function, excluding incoming
    /// tail args or the optional "setup area" of fp/lr.
    fn stack_size(&self) -> u32 {
        self.clobber_size + self.fixed_frame_storage_size + self.outgoing_args_size
    }

    /// Returns the style of frame being used for this function.
    ///
    /// See `FrameStyle` for more information.
    fn pulley_frame_style(&self) -> FrameStyle {
        let saved_by_pulley = self.clobbered_xregs_saved_by_pulley();
        match (
            self.stack_size(),
            self.setup_frame(),
            saved_by_pulley.is_empty(),
        ) {
            // No stack allocated, not saving fp/lr, no clobbers, nothing to do
            (0, false, true) => FrameStyle::None,

            // No stack allocated, saving fp/lr, no clobbers, so this is
            // pulley-managed via push/pop_frame.
            (0, true, true) => FrameStyle::PulleyBasicSetup { frame_size: 0 },

            // Some stack is being allocated and pulley is managing fp/lr. Let
            // pulley manage clobbered registers as well, regardless if they're
            // present or not.
            //
            // If the stack is too large then `PulleyBasicSetup` is used
            // otherwise we'll be pushing `PushFrameSave` and `PopFrameRestore`.
            (frame_size, true, _) => match frame_size.try_into() {
                Ok(frame_size) => FrameStyle::PulleySetupAndSaveClobbers {
                    frame_size,
                    saved_by_pulley,
                },
                Err(_) => FrameStyle::PulleyBasicSetup { frame_size },
            },

            // Some stack is being allocated, but pulley isn't managing fp/lr,
            // so we're manually doing everything.
            (frame_size, false, true) => FrameStyle::Manual { frame_size },

            // If there's no frame setup and there's clobbered registers this
            // technically should have already hit a case above, so panic here.
            (_, false, false) => unreachable!(),
        }
    }

    /// Returns the set of clobbered registers that Pulley is managing via its
    /// macro instructions rather than the generated code.
    fn clobbered_xregs_saved_by_pulley(&self) -> ScalarBitSet<u16> {
        let mut clobbered: ScalarBitSet<u16> = ScalarBitSet::new();
        // Pulley only manages clobbers if it's also managing fp/lr.
        if !self.setup_frame() {
            return clobbered;
        }
        let mut found_manual_clobber = false;
        for reg in self.clobbered_callee_saves.iter() {
            let r_reg = reg.to_reg();
            // Pulley can only manage clobbers of integer registers at this
            // time, float registers are managed manually.
            //
            // Also assert that all pulley-managed clobbers come first,
            // otherwise the loop below in `manually_managed_clobbers` is
            // incorrect.
            if r_reg.class() == RegClass::Int {
                assert!(!found_manual_clobber);
                if let Some(offset) = r_reg.hw_enc().checked_sub(16) {
                    clobbered.insert(offset);
                }
            } else {
                found_manual_clobber = true;
            }
        }
        clobbered
    }

    /// Returns an iterator over the clobbers that Cranelift is managing, not
    /// Pulley.
    ///
    /// If this frame has clobbers then they're either saved by Pulley with
    /// `FrameStyle::PulleySetupAndSaveClobbers`. Cranelift might need to manage
    /// these registers depending on Cranelift settings. Cranelift also always
    /// manages floating-point registers.
    fn manually_managed_clobbers<'a>(
        &'a self,
        style: &'a FrameStyle,
    ) -> impl Iterator<Item = (i32, Type, Reg)> + 'a {
        let mut offset = self.stack_size();
        self.clobbered_callee_saves.iter().filter_map(move |reg| {
            // Allocate space for this clobber no matter what. If pulley is
            // managing this then we're just accounting for the pulley-saved
            // registers as well. Note that all pulley-managed registers come
            // first in the list here.
            offset -= 8;
            let r_reg = reg.to_reg();
            let ty = match r_reg.class() {
                RegClass::Int => {
                    // If this register is saved by pulley, skip this clobber.
                    if let FrameStyle::PulleySetupAndSaveClobbers {
                        saved_by_pulley, ..
                    } = style
                    {
                        if let Some(reg) = r_reg.hw_enc().checked_sub(16) {
                            if saved_by_pulley.contains(reg) {
                                return None;
                            }
                        }
                    }
                    I64
                }
                RegClass::Float => F64,
                RegClass::Vector => unreachable!("no vector registers are callee-save"),
            };
            let offset = i32::try_from(offset).unwrap();
            Some((offset, ty, Reg::from(reg.to_reg())))
        })
    }
}

const DEFAULT_CALLEE_SAVES: PRegSet = PRegSet::empty()
    // Integer registers.
    .with(px_reg(16))
    .with(px_reg(17))
    .with(px_reg(18))
    .with(px_reg(19))
    .with(px_reg(20))
    .with(px_reg(21))
    .with(px_reg(22))
    .with(px_reg(23))
    .with(px_reg(24))
    .with(px_reg(25))
    .with(px_reg(26))
    .with(px_reg(27))
    .with(px_reg(28))
    .with(px_reg(29))
    .with(px_reg(30))
    .with(px_reg(31))
    // Float registers.
    .with(pf_reg(16))
    .with(pf_reg(17))
    .with(pf_reg(18))
    .with(pf_reg(19))
    .with(pf_reg(20))
    .with(pf_reg(21))
    .with(pf_reg(22))
    .with(pf_reg(23))
    .with(pf_reg(24))
    .with(pf_reg(25))
    .with(pf_reg(26))
    .with(pf_reg(27))
    .with(pf_reg(28))
    .with(pf_reg(29))
    .with(pf_reg(30))
    .with(pf_reg(31))
    // Note: no vector registers are callee-saved.
;

fn compute_clobber_size(clobbers: &[Writable<RealReg>]) -> u32 {
    let mut clobbered_size = 0;
    for reg in clobbers {
        match reg.to_reg().class() {
            RegClass::Int => {
                clobbered_size += 8;
            }
            RegClass::Float => {
                clobbered_size += 8;
            }
            RegClass::Vector => unimplemented!("Vector Size Clobbered"),
        }
    }
    align_to(clobbered_size, 16)
}

const DEFAULT_CLOBBERS: PRegSet = PRegSet::empty()
    // Integer registers: the first 16 get clobbered.
    .with(px_reg(0))
    .with(px_reg(1))
    .with(px_reg(2))
    .with(px_reg(3))
    .with(px_reg(4))
    .with(px_reg(5))
    .with(px_reg(6))
    .with(px_reg(7))
    .with(px_reg(8))
    .with(px_reg(9))
    .with(px_reg(10))
    .with(px_reg(11))
    .with(px_reg(12))
    .with(px_reg(13))
    .with(px_reg(14))
    .with(px_reg(15))
    // Float registers: the first 16 get clobbered.
    .with(pf_reg(0))
    .with(pf_reg(1))
    .with(pf_reg(2))
    .with(pf_reg(3))
    .with(pf_reg(4))
    .with(pf_reg(5))
    .with(pf_reg(6))
    .with(pf_reg(7))
    .with(pf_reg(8))
    .with(pf_reg(9))
    .with(pf_reg(10))
    .with(pf_reg(11))
    .with(pf_reg(12))
    .with(pf_reg(13))
    .with(pf_reg(14))
    .with(pf_reg(15))
    // All vector registers get clobbered.
    .with(pv_reg(0))
    .with(pv_reg(1))
    .with(pv_reg(2))
    .with(pv_reg(3))
    .with(pv_reg(4))
    .with(pv_reg(5))
    .with(pv_reg(6))
    .with(pv_reg(7))
    .with(pv_reg(8))
    .with(pv_reg(9))
    .with(pv_reg(10))
    .with(pv_reg(11))
    .with(pv_reg(12))
    .with(pv_reg(13))
    .with(pv_reg(14))
    .with(pv_reg(15))
    .with(pv_reg(16))
    .with(pv_reg(17))
    .with(pv_reg(18))
    .with(pv_reg(19))
    .with(pv_reg(20))
    .with(pv_reg(21))
    .with(pv_reg(22))
    .with(pv_reg(23))
    .with(pv_reg(24))
    .with(pv_reg(25))
    .with(pv_reg(26))
    .with(pv_reg(27))
    .with(pv_reg(28))
    .with(pv_reg(29))
    .with(pv_reg(30))
    .with(pv_reg(31));

const ALL_CLOBBERS: PRegSet = PRegSet::empty()
    .with(px_reg(0))
    .with(px_reg(1))
    .with(px_reg(2))
    .with(px_reg(3))
    .with(px_reg(4))
    .with(px_reg(5))
    .with(px_reg(6))
    .with(px_reg(7))
    .with(px_reg(8))
    .with(px_reg(9))
    .with(px_reg(10))
    .with(px_reg(11))
    .with(px_reg(12))
    .with(px_reg(13))
    .with(px_reg(14))
    .with(px_reg(15))
    .with(px_reg(16))
    .with(px_reg(17))
    .with(px_reg(18))
    .with(px_reg(19))
    .with(px_reg(20))
    .with(px_reg(21))
    .with(px_reg(22))
    .with(px_reg(23))
    .with(px_reg(24))
    .with(px_reg(25))
    .with(px_reg(26))
    .with(px_reg(27))
    .with(px_reg(28))
    .with(px_reg(29))
    .with(px_reg(30))
    .with(px_reg(31))
    .with(pf_reg(0))
    .with(pf_reg(1))
    .with(pf_reg(2))
    .with(pf_reg(3))
    .with(pf_reg(4))
    .with(pf_reg(5))
    .with(pf_reg(6))
    .with(pf_reg(7))
    .with(pf_reg(8))
    .with(pf_reg(9))
    .with(pf_reg(10))
    .with(pf_reg(11))
    .with(pf_reg(12))
    .with(pf_reg(13))
    .with(pf_reg(14))
    .with(pf_reg(15))
    .with(pf_reg(16))
    .with(pf_reg(17))
    .with(pf_reg(18))
    .with(pf_reg(19))
    .with(pf_reg(20))
    .with(pf_reg(21))
    .with(pf_reg(22))
    .with(pf_reg(23))
    .with(pf_reg(24))
    .with(pf_reg(25))
    .with(pf_reg(26))
    .with(pf_reg(27))
    .with(pf_reg(28))
    .with(pf_reg(29))
    .with(pf_reg(30))
    .with(pf_reg(31))
    .with(pv_reg(0))
    .with(pv_reg(1))
    .with(pv_reg(2))
    .with(pv_reg(3))
    .with(pv_reg(4))
    .with(pv_reg(5))
    .with(pv_reg(6))
    .with(pv_reg(7))
    .with(pv_reg(8))
    .with(pv_reg(9))
    .with(pv_reg(10))
    .with(pv_reg(11))
    .with(pv_reg(12))
    .with(pv_reg(13))
    .with(pv_reg(14))
    .with(pv_reg(15))
    .with(pv_reg(16))
    .with(pv_reg(17))
    .with(pv_reg(18))
    .with(pv_reg(19))
    .with(pv_reg(20))
    .with(pv_reg(21))
    .with(pv_reg(22))
    .with(pv_reg(23))
    .with(pv_reg(24))
    .with(pv_reg(25))
    .with(pv_reg(26))
    .with(pv_reg(27))
    .with(pv_reg(28))
    .with(pv_reg(29))
    .with(pv_reg(30))
    .with(pv_reg(31));

fn create_reg_environment() -> MachineEnv {
    // Prefer caller-saved registers over callee-saved registers, because that
    // way we don't need to emit code to save and restore them if we don't
    // mutate them.

    let preferred_regs_by_class: [Vec<PReg>; 3] = {
        let x_registers: Vec<PReg> = (0..16).map(|x| px_reg(x)).collect();
        let f_registers: Vec<PReg> = (0..16).map(|x| pf_reg(x)).collect();
        let v_registers: Vec<PReg> = (0..32).map(|x| pv_reg(x)).collect();
        [x_registers, f_registers, v_registers]
    };

    let non_preferred_regs_by_class: [Vec<PReg>; 3] = {
        let x_registers: Vec<PReg> = (16..XReg::SPECIAL_START)
            .map(|x| px_reg(x.into()))
            .collect();
        let f_registers: Vec<PReg> = (16..32).map(|x| pf_reg(x)).collect();
        let v_registers: Vec<PReg> = vec![];
        [x_registers, f_registers, v_registers]
    };

    MachineEnv {
        preferred_regs_by_class,
        non_preferred_regs_by_class,
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    }
}
