//! Implementation of a standard Riscv64 ABI.

use crate::ir;
use crate::ir::types::*;

use crate::isa;

use crate::isa::CallConv;
use crate::isa::riscv64::inst::*;
use crate::machinst::*;

use crate::CodegenResult;
use crate::ir::LibCall;
use crate::ir::Signature;
use crate::isa::riscv64::settings::Flags as RiscvFlags;
use crate::isa::unwind::UnwindInst;
use crate::settings;
use alloc::boxed::Box;
use alloc::vec::Vec;
use regalloc2::{MachineEnv, PReg, PRegSet};

use smallvec::{SmallVec, smallvec};
use std::borrow::ToOwned;
use std::sync::OnceLock;

/// Support for the Riscv64 ABI from the callee side (within a function body).
pub(crate) type Riscv64Callee = Callee<Riscv64MachineDeps>;

/// Riscv64-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct Riscv64MachineDeps;

impl IsaFlags for RiscvFlags {}

impl RiscvFlags {
    pub(crate) fn min_vec_reg_size(&self) -> u64 {
        let entries = [
            (self.has_zvl65536b(), 65536),
            (self.has_zvl32768b(), 32768),
            (self.has_zvl16384b(), 16384),
            (self.has_zvl8192b(), 8192),
            (self.has_zvl4096b(), 4096),
            (self.has_zvl2048b(), 2048),
            (self.has_zvl1024b(), 1024),
            (self.has_zvl512b(), 512),
            (self.has_zvl256b(), 256),
            // In order to claim the Application Profile V extension, a minimum
            // register size of 128 is required. i.e. V implies Zvl128b.
            (self.has_v(), 128),
            (self.has_zvl128b(), 128),
            (self.has_zvl64b(), 64),
            (self.has_zvl32b(), 32),
        ];

        for (has_flag, size) in entries.into_iter() {
            if !has_flag {
                continue;
            }

            // Due to a limitation in regalloc2, we can't support types
            // larger than 1024 bytes. So limit that here.
            return std::cmp::min(size, 1024);
        }

        return 0;
    }
}

impl ABIMachineSpec for Riscv64MachineDeps {
    type I = Inst;
    type F = RiscvFlags;

    /// This is the limit for the size of argument and return-value areas on the
    /// stack. We place a reasonable limit here to avoid integer overflow issues
    /// with 32-bit arithmetic: for now, 128 MB.
    const STACK_ARG_RET_SIZE_LIMIT: u32 = 128 * 1024 * 1024;

    fn word_bits() -> u32 {
        64
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
        // This implements the LP64D RISC-V ABI.

        assert_ne!(
            call_conv,
            isa::CallConv::Winch,
            "riscv64 does not support the 'winch' calling convention yet"
        );

        // All registers that can be used as parameters or rets.
        // both start and end are included.
        let (x_start, x_end, f_start, f_end) = match args_or_rets {
            ArgsOrRets::Args => (10, 17, 10, 17),
            ArgsOrRets::Rets => (10, 11, 10, 11),
        };
        let mut next_x_reg = x_start;
        let mut next_f_reg = f_start;
        // Stack space.
        let mut next_stack: u32 = 0;

        let ret_area_ptr = if add_ret_area_ptr {
            assert!(ArgsOrRets::Args == args_or_rets);
            next_x_reg += 1;
            Some(ABIArg::reg(
                x_reg(x_start).to_real_reg().unwrap(),
                I64,
                ir::ArgumentExtension::None,
                ir::ArgumentPurpose::Normal,
            ))
        } else {
            None
        };

        for param in params {
            if let ir::ArgumentPurpose::StructArgument(_) = param.purpose {
                panic!(
                    "StructArgument parameters are not supported on riscv64. \
                    Use regular pointer arguments instead."
                );
            }

            // Find regclass(es) of the register(s) used to store a value of this type.
            let (rcs, reg_tys) = Inst::rc_for_type(param.value_type)?;
            let mut slots = ABIArgSlotVec::new();
            for (rc, reg_ty) in rcs.iter().zip(reg_tys.iter()) {
                let next_reg = if (next_x_reg <= x_end) && *rc == RegClass::Int {
                    let x = Some(x_reg(next_x_reg));
                    next_x_reg += 1;
                    x
                } else if (next_f_reg <= f_end) && *rc == RegClass::Float {
                    let x = Some(f_reg(next_f_reg));
                    next_f_reg += 1;
                    x
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
                        offset: next_stack as i64,
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

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Inst {
        Inst::gen_load(into_reg, mem.into(), ty, MemFlags::trusted())
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Inst {
        Inst::gen_store(mem.into(), from_reg, ty, MemFlags::trusted())
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Inst {
        Inst::gen_move(to_reg, from_reg, ty)
    }

    fn gen_extend(
        to_reg: Writable<Reg>,
        from_reg: Reg,
        signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Inst {
        assert!(from_bits < to_bits);
        Inst::Extend {
            rd: to_reg,
            rn: from_reg,
            signed,
            from_bits,
            to_bits,
        }
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        specified
    }

    fn gen_args(args: Vec<ArgPair>) -> Inst {
        Inst::Args { args }
    }

    fn gen_rets(rets: Vec<RetPair>) -> Inst {
        Inst::Rets { rets }
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        spilltmp_reg()
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Inst> {
        let mut insts = SmallInstVec::new();
        if let Some(imm12) = Imm12::maybe_from_u64(imm as u64) {
            insts.push(Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd: into_reg,
                rs: from_reg,
                imm12,
            });
        } else {
            insts.extend(Inst::load_constant_u32(
                writable_spilltmp_reg2(),
                imm as u64,
            ));
            insts.push(Inst::AluRRR {
                alu_op: AluOPRRR::Add,
                rd: into_reg,
                rs1: spilltmp_reg2(),
                rs2: from_reg,
            });
        }
        insts
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();
        insts.push(Inst::TrapIf {
            cc: IntCC::UnsignedLessThan,
            rs1: stack_reg(),
            rs2: limit_reg,
            trap_code: ir::TrapCode::STACK_OVERFLOW,
        });
        insts
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Inst {
        Inst::LoadAddr {
            rd: into_reg,
            mem: mem.into(),
        }
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Inst {
        let mem = AMode::RegOffset(base, offset as i64);
        Inst::gen_load(into_reg, mem, ty, MemFlags::trusted())
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Inst {
        let mem = AMode::RegOffset(base, offset as i64);
        Inst::gen_store(mem, from_reg, ty, MemFlags::trusted())
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();

        if amount == 0 {
            return insts;
        }

        if let Some(imm) = Imm12::maybe_from_i64(amount as i64) {
            insts.push(Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd: writable_stack_reg(),
                rs: stack_reg(),
                imm12: imm,
            })
        } else {
            let tmp = writable_spilltmp_reg();
            insts.extend(Inst::load_constant_u64(tmp, amount as i64 as u64));
            insts.push(Inst::AluRRR {
                alu_op: AluOPRRR::Add,
                rd: writable_stack_reg(),
                rs1: stack_reg(),
                rs2: tmp.to_reg(),
            });
        }

        insts
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        _isa_flags: &RiscvFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();

        if frame_layout.setup_area_size > 0 {
            // add  sp,sp,-16    ;; alloc stack space for fp.
            // sd   ra,8(sp)     ;; save ra.
            // sd   fp,0(sp)     ;; store old fp.
            // mv   fp,sp        ;; set fp to sp.
            insts.extend(Self::gen_sp_reg_adjust(-16));
            insts.push(Inst::gen_store(
                AMode::SPOffset(8),
                link_reg(),
                I64,
                MemFlags::trusted(),
            ));
            insts.push(Inst::gen_store(
                AMode::SPOffset(0),
                fp_reg(),
                I64,
                MemFlags::trusted(),
            ));

            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::PushFrameRegs {
                        offset_upward_to_caller_sp: frame_layout.setup_area_size,
                    },
                });
            }
            insts.push(Inst::Mov {
                rd: writable_fp_reg(),
                rm: stack_reg(),
                ty: I64,
            });
        }

        insts
    }
    /// reverse of gen_prologue_frame_setup.
    fn gen_epilogue_frame_restore(
        call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &RiscvFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();

        if frame_layout.setup_area_size > 0 {
            insts.push(Inst::gen_load(
                writable_link_reg(),
                AMode::SPOffset(8),
                I64,
                MemFlags::trusted(),
            ));
            insts.push(Inst::gen_load(
                writable_fp_reg(),
                AMode::SPOffset(0),
                I64,
                MemFlags::trusted(),
            ));
            insts.extend(Self::gen_sp_reg_adjust(16));
        }

        if call_conv == isa::CallConv::Tail && frame_layout.tail_args_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(
                frame_layout.tail_args_size.try_into().unwrap(),
            ));
        }

        insts
    }

    fn gen_return(
        _call_conv: isa::CallConv,
        _isa_flags: &RiscvFlags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        smallvec![Inst::Ret {}]
    }

    fn gen_probestack(insts: &mut SmallInstVec<Self::I>, frame_size: u32) {
        insts.extend(Inst::load_constant_u32(writable_a0(), frame_size as u64));
        let mut info = CallInfo::empty(
            ExternalName::LibCall(LibCall::Probestack),
            CallConv::SystemV,
        );
        info.uses.push(CallArgPair {
            vreg: a0(),
            preg: a0(),
        });
        insts.push(Inst::Call {
            info: Box::new(info),
        });
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        let mut insts = SmallVec::new();
        let setup_frame = frame_layout.setup_area_size > 0;

        let incoming_args_diff = frame_layout.tail_args_size - frame_layout.incoming_args_size;
        if incoming_args_diff > 0 {
            // Decrement SP by the amount of additional incoming argument space we need
            insts.extend(Self::gen_sp_reg_adjust(-(incoming_args_diff as i32)));

            if setup_frame {
                // Write the lr position on the stack again, as it hasn't changed since it was
                // pushed in `gen_prologue_frame_setup`
                insts.push(Inst::gen_store(
                    AMode::SPOffset(8),
                    link_reg(),
                    I64,
                    MemFlags::trusted(),
                ));
                insts.push(Inst::gen_load(
                    writable_fp_reg(),
                    AMode::SPOffset(i64::from(incoming_args_diff)),
                    I64,
                    MemFlags::trusted(),
                ));
                insts.push(Inst::gen_store(
                    AMode::SPOffset(0),
                    fp_reg(),
                    I64,
                    MemFlags::trusted(),
                ));

                // Finally, sync the frame pointer with SP
                insts.push(Inst::gen_move(writable_fp_reg(), stack_reg(), I64));
            }
        }

        if flags.unwind_info() && setup_frame {
            // The *unwind* frame (but not the actual frame) starts at the
            // clobbers, just below the saved FP/LR pair.
            insts.push(Inst::Unwind {
                inst: UnwindInst::DefineNewFrame {
                    offset_downward_to_clobbers: frame_layout.clobber_size,
                    offset_upward_to_caller_sp: frame_layout.setup_area_size,
                },
            });
        }

        // Adjust the stack pointer downward for clobbers, the function fixed
        // frame (spillslots and storage slots), and outgoing arguments.
        let stack_size = frame_layout.clobber_size
            + frame_layout.fixed_frame_storage_size
            + frame_layout.outgoing_args_size;

        // Store each clobbered register in order at offsets from SP,
        // placing them above the fixed frame slots.
        if stack_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(-(stack_size as i32)));

            let mut cur_offset = 8;
            for reg in &frame_layout.clobbered_callee_saves {
                let r_reg = reg.to_reg();
                let ty = match r_reg.class() {
                    RegClass::Int => I64,
                    RegClass::Float => F64,
                    RegClass::Vector => unimplemented!("Vector Clobber Saves"),
                };
                insts.push(Inst::gen_store(
                    AMode::SPOffset((stack_size - cur_offset) as i64),
                    Reg::from(reg.to_reg()),
                    ty,
                    MemFlags::trusted(),
                ));

                if flags.unwind_info() {
                    insts.push(Inst::Unwind {
                        inst: UnwindInst::SaveReg {
                            clobber_offset: frame_layout.clobber_size - cur_offset,
                            reg: r_reg,
                        },
                    });
                }

                cur_offset += 8
            }
        }
        insts
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        let mut insts = SmallVec::new();

        let stack_size = frame_layout.clobber_size
            + frame_layout.fixed_frame_storage_size
            + frame_layout.outgoing_args_size;

        let mut cur_offset = 8;
        for reg in &frame_layout.clobbered_callee_saves {
            let rreg = reg.to_reg();
            let ty = match rreg.class() {
                RegClass::Int => I64,
                RegClass::Float => F64,
                RegClass::Vector => unimplemented!("Vector Clobber Restores"),
            };
            insts.push(Inst::gen_load(
                reg.map(Reg::from),
                AMode::SPOffset(i64::from(stack_size - cur_offset)),
                ty,
                MemFlags::trusted(),
            ));
            cur_offset += 8
        }

        if stack_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(stack_size as i32));
        }

        insts
    }

    fn gen_memcpy<F: FnMut(Type) -> Writable<Reg>>(
        call_conv: isa::CallConv,
        dst: Reg,
        src: Reg,
        size: usize,
        mut alloc_tmp: F,
    ) -> SmallVec<[Self::I; 8]> {
        let mut insts = SmallVec::new();
        let arg0 = Writable::from_reg(x_reg(10));
        let arg1 = Writable::from_reg(x_reg(11));
        let arg2 = Writable::from_reg(x_reg(12));
        let tmp = alloc_tmp(Self::word_type());
        insts.extend(Inst::load_constant_u64(tmp, size as u64));
        insts.push(Inst::Call {
            info: Box::new(CallInfo {
                dest: ExternalName::LibCall(LibCall::Memcpy),
                uses: smallvec![
                    CallArgPair {
                        vreg: dst,
                        preg: arg0.to_reg()
                    },
                    CallArgPair {
                        vreg: src,
                        preg: arg1.to_reg()
                    },
                    CallArgPair {
                        vreg: tmp.to_reg(),
                        preg: arg2.to_reg()
                    }
                ],
                defs: smallvec![],
                clobbers: Self::get_regs_clobbered_by_call(call_conv, false),
                caller_conv: call_conv,
                callee_conv: call_conv,
                callee_pop_size: 0,
                try_call_info: None,
            }),
        });
        insts
    }

    fn get_number_of_spillslots_for_value(
        rc: RegClass,
        _target_vector_bytes: u32,
        isa_flags: &RiscvFlags,
    ) -> u32 {
        // We allocate in terms of 8-byte slots.
        match rc {
            RegClass::Int => 1,
            RegClass::Float => 1,
            RegClass::Vector => (isa_flags.min_vec_reg_size() / 8) as u32,
        }
    }

    fn get_machine_env(_flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
        MACHINE_ENV.get_or_init(create_reg_environment)
    }

    fn get_regs_clobbered_by_call(
        call_conv_of_callee: isa::CallConv,
        is_exception: bool,
    ) -> PRegSet {
        match call_conv_of_callee {
            isa::CallConv::Tail if is_exception => ALL_CLOBBERS,
            _ => DEFAULT_CLOBBERS,
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
            16 // FP, LR
        } else {
            0
        };

        // Return FrameLayout structure.
        FrameLayout {
            word_bytes: 8,
            incoming_args_size,
            tail_args_size,
            setup_area_size,
            clobber_size,
            fixed_frame_storage_size,
            stackslots_size,
            outgoing_args_size,
            clobbered_callee_saves: regs,
        }
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        // Unroll at most n consecutive probes, before falling back to using a loop
        const PROBE_MAX_UNROLL: u32 = 3;

        // Calculate how many probes we need to perform. Round down, as we only
        // need to probe whole guard_size regions we'd otherwise skip over.
        let probe_count = frame_size / guard_size;
        if probe_count == 0 {
            // No probe necessary
            return;
        }

        // Must be a caller-saved register that is not an argument.
        let tmp = Writable::from_reg(x_reg(28)); // t3

        if probe_count <= PROBE_MAX_UNROLL {
            Self::gen_probestack_unroll(insts, tmp, guard_size, probe_count)
        } else {
            insts.push(Inst::StackProbeLoop {
                guard_size,
                probe_count,
                tmp,
            });
        }
    }

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        // Use x12 as a temp if needed: clobbered, not a
        // retval.
        Writable::from_reg(regs::x_reg(12))
    }

    fn exception_payload_regs(call_conv: isa::CallConv) -> &'static [Reg] {
        const PAYLOAD_REGS: &'static [Reg] = &[regs::a0(), regs::a1()];
        match call_conv {
            isa::CallConv::SystemV | isa::CallConv::Tail => PAYLOAD_REGS,
            _ => &[],
        }
    }
}

// NOTE: no V regs are callee save.
const DEFAULT_CALLEE_SAVES: PRegSet = PRegSet::empty()
    // X Regs
    .with(px_reg(2))
    .with(px_reg(8))
    .with(px_reg(9))
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
    // F Regs
    .with(pf_reg(8))
    .with(pf_reg(18))
    .with(pf_reg(19))
    .with(pf_reg(20))
    .with(pf_reg(21))
    .with(pf_reg(22))
    .with(pf_reg(23))
    .with(pf_reg(24))
    .with(pf_reg(25))
    .with(pf_reg(26))
    .with(pf_reg(27));

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
    .with(px_reg(1))
    .with(px_reg(5))
    .with(px_reg(6))
    .with(px_reg(7))
    .with(px_reg(10))
    .with(px_reg(11))
    .with(px_reg(12))
    .with(px_reg(13))
    .with(px_reg(14))
    .with(px_reg(15))
    .with(px_reg(16))
    .with(px_reg(17))
    .with(px_reg(28))
    .with(px_reg(29))
    .with(px_reg(30))
    .with(px_reg(31))
    // F Regs
    .with(pf_reg(0))
    .with(pf_reg(1))
    .with(pf_reg(2))
    .with(pf_reg(3))
    .with(pf_reg(4))
    .with(pf_reg(5))
    .with(pf_reg(6))
    .with(pf_reg(7))
    .with(pf_reg(9))
    .with(pf_reg(10))
    .with(pf_reg(11))
    .with(pf_reg(12))
    .with(pf_reg(13))
    .with(pf_reg(14))
    .with(pf_reg(15))
    .with(pf_reg(16))
    .with(pf_reg(17))
    .with(pf_reg(28))
    .with(pf_reg(29))
    .with(pf_reg(30))
    .with(pf_reg(31))
    // V Regs - All vector regs get clobbered
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
    // Specials: x0 is the zero register; x1 is the return address; x2 is SP.
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
    // F Regs
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
    // V Regs
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
    // Some C Extension instructions can only use a subset of the registers.
    // x8 - x15, f8 - f15, v8 - v15 so we should prefer to use those since
    // they allow us to emit C instructions more often.
    //
    // In general the order of preference is:
    //   1. Compressible Caller Saved registers.
    //   2. Non-Compressible Caller Saved registers.
    //   3. Compressible Callee Saved registers.
    //   4. Non-Compressible Callee Saved registers.

    let preferred_regs_by_class: [Vec<PReg>; 3] = {
        let x_registers: Vec<PReg> = (10..=15).map(px_reg).collect();
        let f_registers: Vec<PReg> = (10..=15).map(pf_reg).collect();
        let v_registers: Vec<PReg> = (8..=15).map(pv_reg).collect();

        [x_registers, f_registers, v_registers]
    };

    let non_preferred_regs_by_class: [Vec<PReg>; 3] = {
        // x0 - x4 are special registers, so we don't want to use them.
        // Omit x30 and x31 since they are the spilltmp registers.

        // Start with the Non-Compressible Caller Saved registers.
        let x_registers: Vec<PReg> = (5..=7)
            .chain(16..=17)
            .chain(28..=29)
            // The first Callee Saved register is x9 since its Compressible
            // Omit x8 since it's the frame pointer.
            .chain(9..=9)
            // The rest of the Callee Saved registers are Non-Compressible
            .chain(18..=27)
            .map(px_reg)
            .collect();

        // Prefer Caller Saved registers.
        let f_registers: Vec<PReg> = (0..=7)
            .chain(16..=17)
            .chain(28..=31)
            // Once those are exhausted, we should prefer f8 and f9 since they are
            // callee saved, but compressible.
            .chain(8..=9)
            .chain(18..=27)
            .map(pf_reg)
            .collect();

        let v_registers = (0..=7).chain(16..=31).map(pv_reg).collect();

        [x_registers, f_registers, v_registers]
    };

    MachineEnv {
        preferred_regs_by_class,
        non_preferred_regs_by_class,
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    }
}

impl Riscv64MachineDeps {
    fn gen_probestack_unroll(
        insts: &mut SmallInstVec<Inst>,
        tmp: Writable<Reg>,
        guard_size: u32,
        probe_count: u32,
    ) {
        // When manually unrolling adjust the stack pointer and then write a zero
        // to the stack at that offset.
        //
        // We do this because valgrind expects us to never write beyond the stack
        // pointer and associated redzone.
        // See: https://github.com/bytecodealliance/wasmtime/issues/7454

        // Store the adjust amount in a register upfront, so we don't have to
        // reload it for each probe. It's worth loading this as a negative and
        // using an `add` instruction since we have compressed versions of `add`
        // but not the `sub` instruction.
        insts.extend(Inst::load_constant_u64(tmp, (-(guard_size as i64)) as u64));

        for _ in 0..probe_count {
            insts.push(Inst::AluRRR {
                alu_op: AluOPRRR::Add,
                rd: writable_stack_reg(),
                rs1: stack_reg(),
                rs2: tmp.to_reg(),
            });

            insts.push(Inst::gen_store(
                AMode::SPOffset(0),
                zero_reg(),
                I32,
                MemFlags::trusted(),
            ));
        }

        // Restore the stack pointer to its original value
        insts.extend(Self::gen_sp_reg_adjust((guard_size * probe_count) as i32));
    }
}
