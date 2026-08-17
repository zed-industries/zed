//! Implementation of a standard S390x ABI.
//!
//! This machine uses the "vanilla" ABI implementation from abi.rs,
//! however a few details are different from the description there:
//!
//! - On s390x, the caller must provide a "register save area" of 160
//!   bytes to any function it calls.  The called function is free to use
//!   this space for any purpose; usually to save callee-saved GPRs.
//!   (Note that while this area is allocated by the caller, it is counted
//!   as part of the callee's stack frame; in particular, the callee's CFA
//!   is the top of the register save area, not the incoming SP value.)
//!
//! - Overflow arguments are passed on the stack starting immediately
//!   above the register save area.  On s390x, this space is allocated
//!   only once directly in the prologue, using a size large enough to
//!   hold overflow arguments for every call in the function.
//!
//! - On s390x we do not use a frame pointer register; instead, every
//!   element of the stack frame is addressed via (constant) offsets
//!   from the stack pointer.  Note that due to the above (and because
//!   there are no variable-sized stack allocations in cranelift), the
//!   value of the stack pointer register never changes after the
//!   initial allocation in the function prologue.
//!
//! - If we are asked to "preserve frame pointers" to enable stack
//!   unwinding, we use the stack backchain feature instead, which
//!   is documented by the s390x ELF ABI, but marked as optional.
//!   This ensures that at all times during execution of a function,
//!   the lowest word on the stack (part of the register save area)
//!   holds a copy of the stack pointer at function entry.
//!
//! Overall, the stack frame layout on s390x is as follows:
//!
//! ```plain
//!   (high address)
//!
//!                              +---------------------------+
//!                              |          ...              |
//! CFA                  ----->  | stack args                |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | 160 bytes reg save area   |
//!                              | (used to save GPRs)       |
//! SP at function entry ----->  | (incl. caller's backchain)|
//!                              +---------------------------+
//!                              |          ...              |
//!                              | clobbered callee-saves    |
//!                              | (used to save FPRs)       |
//! unwind-frame base     ---->  | (alloc'd by prologue)     |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | spill slots               |
//!                              | (accessed via SP)         |
//!                              |          ...              |
//!                              | stack slots               |
//!                              | (accessed via SP)         |
//!                              | (alloc'd by prologue)     |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | args for call             |
//!                              | outgoing reg save area    |
//!                              | (alloc'd by prologue)     |
//! SP during function  ------>  | (incl. callee's backchain)|
//!                              +---------------------------+
//!
//!   (low address)
//! ```
//!
//!
//! The tail-call ABI has the following changes to the system ABI:
//!
//! - %r6 and %r7 are both non-callee-saved argument registers.
//!
//! - The argument save area for outgoing (non-tail) calls to
//!   a tail-call ABI function is placed *below* the caller's
//!   stack frame.  This means the caller temporarily allocates
//!   a part of the callee's frame, including temporary space
//!   for a register save area holding a copy of the backchain.
//!
//! - For tail calls, the caller puts outgoing arguments at the
//!   very top of its stack frame, overlapping the incoming
//!   argument area.  This is extended by the prolog if needed.
//!
//! Overall, the tail-call stack frame layout on s390x is as follows:
//!
//! ```plain
//!   (high address)
//!
//!                              +---------------------------+
//!                              |          ...              |
//! CFA                  ----->  | (caller's frame)          |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | 160 bytes reg save area   |
//!                              | (used to save GPRs)       |
//! SP at function return----->  | (incl. caller's backchain)|
//!                              +---------------------------+
//!                              |          ...              |
//!                              | incoming stack args       |
//! SP at function entry ----->  | (incl. backchain copy)    |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | outgoing tail call args   |
//!                              | (overlaps incoming args)  |
//!                              | (incl. backchain copy)    |
//! SP at tail cail       ---->  | (alloc'd by prologue)     |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | clobbered callee-saves    |
//!                              | (used to save FPRs)       |
//! unwind-frame base     ---->  | (alloc'd by prologue)     |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | spill slots               |
//!                              | (accessed via SP)         |
//!                              |          ...              |
//!                              | stack slots               |
//!                              | (accessed via SP)         |
//!                              | (alloc'd by prologue)     |
//!                              +---------------------------+
//!                              |          ...              |
//!                              | outgoing calls return buf |
//!                              | outgoing reg save area    |
//!                              | (alloc'd by prologue)     |
//! SP during function  ------>  | (incl. callee's backchain)|
//!                              +---------------------------+
//!                              |          ...              |
//!                              | outgoing stack args       |
//!                              | (alloc'd by call sequence)|
//! SP at non-tail call  ----->  | (incl. backchain copy)    |
//!                              +---------------------------+
//!   (low address)
//! ```

use crate::CodegenResult;
use crate::ir;
use crate::ir::MemFlags;
use crate::ir::Signature;
use crate::ir::Type;
use crate::ir::condcodes::IntCC;
use crate::ir::types;
use crate::isa;
use crate::isa::s390x::{inst::*, settings as s390x_settings};
use crate::isa::unwind::UnwindInst;
use crate::machinst::*;
use crate::settings;
use alloc::vec::Vec;
use regalloc2::{MachineEnv, PRegSet};
use smallvec::{SmallVec, smallvec};
use std::borrow::ToOwned;
use std::sync::OnceLock;

// We use a generic implementation that factors out ABI commonalities.

/// Support for the S390x ABI from the callee side (within a function body).
pub type S390xCallee = Callee<S390xMachineDeps>;

/// ABI Register usage

fn in_int_reg(ty: Type) -> bool {
    match ty {
        types::I8 | types::I16 | types::I32 | types::I64 => true,
        _ => false,
    }
}

fn in_flt_reg(ty: Type) -> bool {
    match ty {
        types::F16 | types::F32 | types::F64 => true,
        _ => false,
    }
}

fn in_vec_reg(ty: Type) -> bool {
    ty.is_vector() && ty.bits() == 128
}

fn get_intreg_for_arg(call_conv: isa::CallConv, idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::gpr(2)),
        1 => Some(regs::gpr(3)),
        2 => Some(regs::gpr(4)),
        3 => Some(regs::gpr(5)),
        4 => Some(regs::gpr(6)),
        5 if call_conv == isa::CallConv::Tail => Some(regs::gpr(7)),
        _ => None,
    }
}

fn get_fltreg_for_arg(idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::vr(0)),
        1 => Some(regs::vr(2)),
        2 => Some(regs::vr(4)),
        3 => Some(regs::vr(6)),
        _ => None,
    }
}

fn get_vecreg_for_arg(idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::vr(24)),
        1 => Some(regs::vr(25)),
        2 => Some(regs::vr(26)),
        3 => Some(regs::vr(27)),
        4 => Some(regs::vr(28)),
        5 => Some(regs::vr(29)),
        6 => Some(regs::vr(30)),
        7 => Some(regs::vr(31)),
        _ => None,
    }
}

fn get_intreg_for_ret(call_conv: isa::CallConv, idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::gpr(2)),
        // ABI extension to support multi-value returns:
        1 => Some(regs::gpr(3)),
        2 => Some(regs::gpr(4)),
        3 => Some(regs::gpr(5)),
        4 if call_conv == isa::CallConv::Tail => Some(regs::gpr(6)),
        5 if call_conv == isa::CallConv::Tail => Some(regs::gpr(7)),
        _ => None,
    }
}

fn get_fltreg_for_ret(idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::vr(0)),
        // ABI extension to support multi-value returns:
        1 => Some(regs::vr(2)),
        2 => Some(regs::vr(4)),
        3 => Some(regs::vr(6)),
        _ => None,
    }
}

fn get_vecreg_for_ret(idx: usize) -> Option<Reg> {
    match idx {
        0 => Some(regs::vr(24)),
        // ABI extension to support multi-value returns:
        1 => Some(regs::vr(25)),
        2 => Some(regs::vr(26)),
        3 => Some(regs::vr(27)),
        4 => Some(regs::vr(28)),
        5 => Some(regs::vr(29)),
        6 => Some(regs::vr(30)),
        7 => Some(regs::vr(31)),
        _ => None,
    }
}

/// The size of the register save area
pub static REG_SAVE_AREA_SIZE: u32 = 160;

impl From<StackAMode> for MemArg {
    fn from(stack: StackAMode) -> MemArg {
        match stack {
            StackAMode::IncomingArg(off, stack_args_size) => MemArg::IncomingArgOffset {
                off: off - stack_args_size as i64,
            },
            StackAMode::Slot(off) => MemArg::SlotOffset { off },
            StackAMode::OutgoingArg(off) => MemArg::OutgoingArgOffset { off },
        }
    }
}

/// Lane order to be used for a given calling convention.
impl From<isa::CallConv> for LaneOrder {
    fn from(call_conv: isa::CallConv) -> Self {
        match call_conv {
            isa::CallConv::Tail => LaneOrder::LittleEndian,
            _ => LaneOrder::BigEndian,
        }
    }
}

/// S390x-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct S390xMachineDeps;

impl IsaFlags for s390x_settings::Flags {}

impl ABIMachineSpec for S390xMachineDeps {
    type I = Inst;

    type F = s390x_settings::Flags;

    /// This is the limit for the size of argument and return-value areas on the
    /// stack. We place a reasonable limit here to avoid integer overflow issues
    /// with 32-bit arithmetic: for now, 128 MB.
    const STACK_ARG_RET_SIZE_LIMIT: u32 = 128 * 1024 * 1024;

    fn word_bits() -> u32 {
        64
    }

    /// Return required stack alignment in bytes.
    fn stack_align(_call_conv: isa::CallConv) -> u32 {
        8
    }

    fn compute_arg_locs(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        params: &[ir::AbiParam],
        args_or_rets: ArgsOrRets,
        add_ret_area_ptr: bool,
        mut args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        assert_ne!(
            call_conv,
            isa::CallConv::Winch,
            "s390x does not support the 'winch' calling convention yet"
        );

        let mut next_gpr = 0;
        let mut next_fpr = 0;
        let mut next_vr = 0;
        let mut next_stack: u32 = 0;

        let ret_area_ptr = if add_ret_area_ptr {
            debug_assert_eq!(args_or_rets, ArgsOrRets::Args);
            next_gpr += 1;
            Some(ABIArg::reg(
                get_intreg_for_arg(call_conv, 0)
                    .unwrap()
                    .to_real_reg()
                    .unwrap(),
                types::I64,
                ir::ArgumentExtension::None,
                ir::ArgumentPurpose::Normal,
            ))
        } else {
            None
        };

        for mut param in params.into_iter().copied() {
            if let ir::ArgumentPurpose::StructArgument(_) = param.purpose {
                panic!(
                    "StructArgument parameters are not supported on s390x. \
                    Use regular pointer arguments instead."
                );
            }

            let intreg = in_int_reg(param.value_type);
            let fltreg = in_flt_reg(param.value_type);
            let vecreg = in_vec_reg(param.value_type);
            debug_assert!(intreg as i32 + fltreg as i32 + vecreg as i32 <= 1);

            let (next_reg, candidate, implicit_ref) = if intreg {
                let candidate = match args_or_rets {
                    ArgsOrRets::Args => get_intreg_for_arg(call_conv, next_gpr),
                    ArgsOrRets::Rets => get_intreg_for_ret(call_conv, next_gpr),
                };
                (&mut next_gpr, candidate, None)
            } else if fltreg {
                let candidate = match args_or_rets {
                    ArgsOrRets::Args => get_fltreg_for_arg(next_fpr),
                    ArgsOrRets::Rets => get_fltreg_for_ret(next_fpr),
                };
                (&mut next_fpr, candidate, None)
            } else if vecreg {
                let candidate = match args_or_rets {
                    ArgsOrRets::Args => get_vecreg_for_arg(next_vr),
                    ArgsOrRets::Rets => get_vecreg_for_ret(next_vr),
                };
                (&mut next_vr, candidate, None)
            } else {
                // We must pass this by implicit reference.
                if args_or_rets == ArgsOrRets::Rets {
                    // For return values, just force them to memory.
                    (&mut next_gpr, None, None)
                } else {
                    // For arguments, implicitly convert to pointer type.
                    let implicit_ref = Some(param.value_type);
                    param = ir::AbiParam::new(types::I64);
                    let candidate = get_intreg_for_arg(call_conv, next_gpr);
                    (&mut next_gpr, candidate, implicit_ref)
                }
            };

            let slot = if let Some(reg) = candidate {
                *next_reg += 1;
                ABIArgSlot::Reg {
                    reg: reg.to_real_reg().unwrap(),
                    ty: param.value_type,
                    extension: param.extension,
                }
            } else {
                if args_or_rets == ArgsOrRets::Rets && !flags.enable_multi_ret_implicit_sret() {
                    return Err(crate::CodegenError::Unsupported(
                        "Too many return values to fit in registers. \
                        Use a StructReturn argument instead. (#9510)"
                            .to_owned(),
                    ));
                }

                // Compute size. Every argument or return value takes a slot of
                // at least 8 bytes.
                let size = (ty_bits(param.value_type) / 8) as u32;
                let slot_size = std::cmp::max(size, 8);

                // Align the stack slot.
                debug_assert!(slot_size.is_power_of_two());
                let slot_align = std::cmp::min(slot_size, 8);
                next_stack = align_to(next_stack, slot_align);

                // If the type is actually of smaller size (and the argument
                // was not extended), it is passed right-aligned.
                let offset = if size < slot_size && param.extension == ir::ArgumentExtension::None {
                    slot_size - size
                } else {
                    0
                };
                let offset = (next_stack + offset) as i64;
                next_stack += slot_size;
                ABIArgSlot::Stack {
                    offset,
                    ty: param.value_type,
                    extension: param.extension,
                }
            };

            if let Some(ty) = implicit_ref {
                assert!(
                    (ty_bits(ty) / 8) % 8 == 0,
                    "implicit argument size is not properly aligned"
                );
                args.push(ABIArg::ImplicitPtrArg {
                    pointer: slot,
                    offset: 0, // Will be filled in later
                    ty,
                    purpose: param.purpose,
                });
            } else {
                args.push(ABIArg::Slots {
                    slots: smallvec![slot],
                    purpose: param.purpose,
                });
            }
        }

        next_stack = align_to(next_stack, 8);

        let extra_arg = if let Some(ret_area_ptr) = ret_area_ptr {
            args.push_non_formal(ret_area_ptr);
            Some(args.args().len() - 1)
        } else {
            None
        };

        // After all arguments are in their well-defined location,
        // allocate buffers for all ImplicitPtrArg arguments.
        for arg in args.args_mut() {
            match arg {
                ABIArg::StructArg { .. } => unreachable!(),
                ABIArg::ImplicitPtrArg { offset, ty, .. } => {
                    *offset = next_stack as i64;
                    next_stack += (ty_bits(*ty) / 8) as u32;
                }
                _ => {}
            }
        }

        // With the tail-call convention, arguments are passed in the *callee*'s
        // frame instead of the caller's frame.  This means that the register save
        // area will lie between the incoming arguments and the return buffer.
        // Include the size of the register area in the argument area size to
        // match common code expectation that the return buffer resides immediately
        // above the argument area.
        if call_conv == isa::CallConv::Tail && args_or_rets == ArgsOrRets::Args && next_stack != 0 {
            next_stack += REG_SAVE_AREA_SIZE;
        }

        Ok((next_stack, extra_arg))
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Inst {
        Inst::gen_load(into_reg, mem.into(), ty)
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Inst {
        Inst::gen_store(mem.into(), from_reg, ty)
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

    fn gen_args(args: Vec<ArgPair>) -> Inst {
        Inst::Args { args }
    }

    fn gen_rets(rets: Vec<RetPair>) -> Inst {
        Inst::Rets { rets }
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();
        if let Some(imm) = UImm12::maybe_from_u64(imm as u64) {
            insts.push(Inst::LoadAddr {
                rd: into_reg,
                mem: MemArg::BXD12 {
                    base: from_reg,
                    index: zero_reg(),
                    disp: imm,
                    flags: MemFlags::trusted(),
                },
            });
        } else if let Some(imm) = SImm20::maybe_from_i64(imm as i64) {
            insts.push(Inst::LoadAddr {
                rd: into_reg,
                mem: MemArg::BXD20 {
                    base: from_reg,
                    index: zero_reg(),
                    disp: imm,
                    flags: MemFlags::trusted(),
                },
            });
        } else {
            if from_reg != into_reg.to_reg() {
                insts.push(Inst::mov64(into_reg, from_reg));
            }
            insts.push(Inst::AluRUImm32 {
                alu_op: ALUOp::AddLogical64,
                rd: into_reg,
                ri: into_reg.to_reg(),
                imm,
            });
        }
        insts
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();
        insts.push(Inst::CmpTrapRR {
            op: CmpOp::CmpL64,
            rn: stack_reg(),
            rm: limit_reg,
            cond: Cond::from_intcc(IntCC::UnsignedLessThanOrEqual),
            trap_code: ir::TrapCode::STACK_OVERFLOW,
        });
        insts
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Inst {
        let mem = mem.into();
        Inst::LoadAddr { rd: into_reg, mem }
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        spilltmp_reg()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Inst {
        let mem = MemArg::reg_plus_off(base, offset.into(), MemFlags::trusted());
        Inst::gen_load(into_reg, mem, ty)
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Inst {
        let mem = MemArg::reg_plus_off(base, offset.into(), MemFlags::trusted());
        Inst::gen_store(mem, from_reg, ty)
    }

    fn gen_sp_reg_adjust(imm: i32) -> SmallInstVec<Inst> {
        if imm == 0 {
            return SmallVec::new();
        }

        let mut insts = SmallVec::new();
        if let Ok(imm) = i16::try_from(imm) {
            insts.push(Inst::AluRSImm16 {
                alu_op: ALUOp::Add64,
                rd: writable_stack_reg(),
                ri: stack_reg(),
                imm,
            });
        } else {
            insts.push(Inst::AluRSImm32 {
                alu_op: ALUOp::Add64,
                rd: writable_stack_reg(),
                ri: stack_reg(),
                imm,
            });
        }
        insts
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &s390x_settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        SmallVec::new()
    }

    fn gen_epilogue_frame_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &s390x_settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        SmallVec::new()
    }

    fn gen_return(
        _call_conv: isa::CallConv,
        _isa_flags: &s390x_settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        smallvec![Inst::Ret { link: gpr(14) }]
    }

    fn gen_probestack(_insts: &mut SmallInstVec<Self::I>, _: u32) {
        // TODO: implement if we ever require stack probes on an s390x host
        // (unlikely unless Lucet is ported)
        unimplemented!("Stack probing is unimplemented on S390x");
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        // The stack probe loop currently takes 4 instructions and each unrolled
        // probe takes 2.  Set this to 2 to keep the max size to 4 instructions.
        const PROBE_MAX_UNROLL: u32 = 2;

        // Calculate how many probes we need to perform. Round down, as we only
        // need to probe whole guard_size regions we'd otherwise skip over.
        let probe_count = frame_size / guard_size;
        if probe_count == 0 {
            // No probe necessary
        } else if probe_count <= PROBE_MAX_UNROLL {
            // Unrolled probe loop.
            for _ in 0..probe_count {
                insts.extend(Self::gen_sp_reg_adjust(-(guard_size as i32)));

                insts.push(Inst::StoreImm8 {
                    imm: 0,
                    mem: MemArg::reg(stack_reg(), MemFlags::trusted()),
                });
            }
        } else {
            // Explicit probe loop.

            // Load the number of probes into a register used as loop counter.
            // `gen_inline_probestack` is called after regalloc2, so we can
            // use the nonallocatable spilltmp register for this purpose.
            let probe_count_reg = writable_spilltmp_reg();
            if let Ok(probe_count) = i16::try_from(probe_count) {
                insts.push(Inst::Mov32SImm16 {
                    rd: probe_count_reg,
                    imm: probe_count,
                });
            } else {
                insts.push(Inst::Mov32Imm {
                    rd: probe_count_reg,
                    imm: probe_count,
                });
            }

            // Emit probe loop.  The guard size is assumed to fit in 16 bits.
            insts.push(Inst::StackProbeLoop {
                probe_count: probe_count_reg,
                guard_size: i16::try_from(guard_size).unwrap(),
            });
        }

        // Restore the stack pointer to its original position.
        insts.extend(Self::gen_sp_reg_adjust((probe_count * guard_size) as i32));
    }

    fn gen_clobber_save(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        let mut insts = SmallVec::new();

        // With the tail call convention, the caller already allocated the
        // part of our stack frame that contains incoming arguments.
        let incoming_tail_args_size = if call_conv == isa::CallConv::Tail {
            frame_layout.incoming_args_size
        } else {
            0
        };

        // Define unwind stack frame.
        if flags.unwind_info() {
            insts.push(Inst::Unwind {
                inst: UnwindInst::DefineNewFrame {
                    offset_upward_to_caller_sp: REG_SAVE_AREA_SIZE + incoming_tail_args_size,
                    offset_downward_to_clobbers: frame_layout.clobber_size
                        - incoming_tail_args_size,
                },
            });
        }

        // Use STMG to save clobbered GPRs into save area.
        // Note that we always save SP (%r15) here if anything is saved.
        if let Some((first_clobbered_gpr, _)) = get_clobbered_gprs(frame_layout) {
            let mut last_clobbered_gpr = 15;
            let offset = 8 * first_clobbered_gpr as i64 + incoming_tail_args_size as i64;
            insts.push(Inst::StoreMultiple64 {
                rt: gpr(first_clobbered_gpr),
                rt2: gpr(last_clobbered_gpr),
                mem: MemArg::reg_plus_off(stack_reg(), offset, MemFlags::trusted()),
            });
            if flags.unwind_info() {
                // Normally, we instruct the unwinder to restore the stack pointer
                // from its slot in the save area.  However, if we have incoming
                // tail-call arguments, the value saved in that slot is incorrect.
                // In that case, we instead instruct the unwinder to compute the
                // unwound SP relative to the current CFA, as CFA == SP + 160.
                if incoming_tail_args_size != 0 {
                    insts.push(Inst::Unwind {
                        inst: UnwindInst::RegStackOffset {
                            clobber_offset: frame_layout.clobber_size,
                            reg: gpr(last_clobbered_gpr).to_real_reg().unwrap(),
                        },
                    });
                    last_clobbered_gpr = last_clobbered_gpr - 1;
                }
                for i in first_clobbered_gpr..(last_clobbered_gpr + 1) {
                    insts.push(Inst::Unwind {
                        inst: UnwindInst::SaveReg {
                            clobber_offset: frame_layout.clobber_size + (i * 8) as u32,
                            reg: gpr(i).to_real_reg().unwrap(),
                        },
                    });
                }
            }
        }

        // Save current stack pointer value if we need to write the backchain.
        if flags.preserve_frame_pointers() {
            if incoming_tail_args_size == 0 {
                insts.push(Inst::mov64(writable_gpr(1), stack_reg()));
            } else {
                insts.extend(Self::gen_add_imm(
                    call_conv,
                    writable_gpr(1),
                    stack_reg(),
                    incoming_tail_args_size,
                ));
            }
        }

        // Decrement stack pointer.
        let stack_size = frame_layout.outgoing_args_size as i32
            + frame_layout.clobber_size as i32
            + frame_layout.fixed_frame_storage_size as i32
            - incoming_tail_args_size as i32;
        insts.extend(Self::gen_sp_reg_adjust(-stack_size));
        if flags.unwind_info() {
            insts.push(Inst::Unwind {
                inst: UnwindInst::StackAlloc {
                    size: stack_size as u32,
                },
            });
        }

        // Write the stack backchain if requested, using the value saved above.
        if flags.preserve_frame_pointers() {
            insts.push(Inst::Store64 {
                rd: gpr(1),
                mem: MemArg::reg_plus_off(stack_reg(), 0, MemFlags::trusted()),
            });
        }

        // Save FPRs.
        for (i, reg) in get_clobbered_fprs(frame_layout).iter().enumerate() {
            insts.push(Inst::VecStoreLane {
                size: 64,
                rd: reg.to_reg().into(),
                mem: MemArg::reg_plus_off(
                    stack_reg(),
                    (i * 8) as i64
                        + frame_layout.outgoing_args_size as i64
                        + frame_layout.fixed_frame_storage_size as i64,
                    MemFlags::trusted(),
                ),
                lane_imm: 0,
            });
            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset: (i * 8) as u32,
                        reg: reg.to_reg(),
                    },
                });
            }
        }

        insts
    }

    fn gen_clobber_restore(
        call_conv: isa::CallConv,
        _flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        let mut insts = SmallVec::new();

        // Restore FPRs.
        insts.extend(gen_restore_fprs(frame_layout));

        // Restore GPRs (including SP).
        insts.extend(gen_restore_gprs(call_conv, frame_layout, 0));

        insts
    }

    fn gen_memcpy<F: FnMut(Type) -> Writable<Reg>>(
        _call_conv: isa::CallConv,
        _dst: Reg,
        _src: Reg,
        _size: usize,
        _alloc: F,
    ) -> SmallVec<[Self::I; 8]> {
        unimplemented!("StructArgs not implemented for S390X yet");
    }

    fn get_number_of_spillslots_for_value(
        rc: RegClass,
        _vector_scale: u32,
        _isa_flags: &Self::F,
    ) -> u32 {
        // We allocate in terms of 8-byte slots.
        match rc {
            RegClass::Int => 1,
            RegClass::Float => 2,
            RegClass::Vector => unreachable!(),
        }
    }

    fn get_machine_env(_flags: &settings::Flags, call_conv: isa::CallConv) -> &MachineEnv {
        match call_conv {
            isa::CallConv::Tail => {
                static TAIL_MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
                TAIL_MACHINE_ENV.get_or_init(tail_create_machine_env)
            }
            _ => {
                static SYSV_MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
                SYSV_MACHINE_ENV.get_or_init(sysv_create_machine_env)
            }
        }
    }

    fn get_regs_clobbered_by_call(
        call_conv_of_callee: isa::CallConv,
        is_exception: bool,
    ) -> PRegSet {
        match call_conv_of_callee {
            isa::CallConv::Tail if is_exception => ALL_CLOBBERS,
            isa::CallConv::Tail => TAIL_CLOBBERS,
            _ => SYSV_CLOBBERS,
        }
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        specified
    }

    fn compute_frame_layout(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        _sig: &Signature,
        regs: &[Writable<RealReg>],
        _is_leaf: bool,
        incoming_args_size: u32,
        tail_args_size: u32,
        stackslots_size: u32,
        fixed_frame_storage_size: u32,
        mut outgoing_args_size: u32,
    ) -> FrameLayout {
        assert!(
            !flags.enable_pinned_reg(),
            "Pinned register not supported on s390x"
        );

        let mut regs: Vec<Writable<RealReg>> = regs
            .iter()
            .cloned()
            .filter(|r| is_reg_saved_in_prologue(call_conv, r.to_reg()))
            .collect();

        // If the front end asks to preserve frame pointers (which we do not
        // really have in the s390x ABI), we use the stack backchain instead.
        // For this to work in all cases, we must allocate a stack frame with
        // at least the outgoing register save area even in leaf functions.
        // Update our caller's outgoing_args_size to reflect this.
        if flags.preserve_frame_pointers() {
            if outgoing_args_size < REG_SAVE_AREA_SIZE {
                outgoing_args_size = REG_SAVE_AREA_SIZE;
            }
        }

        // We need to save/restore the link register in non-leaf functions.
        // This is not included in the clobber list because we have excluded
        // call instructions via the is_included_in_clobbers callback.
        // We also want to enforce saving the link register in leaf functions
        // for stack unwinding, if we're asked to preserve frame pointers.
        if outgoing_args_size > 0 {
            let link_reg = Writable::from_reg(RealReg::from(gpr_preg(14)));
            if !regs.contains(&link_reg) {
                regs.push(link_reg);
            }
        }

        // Sort registers for deterministic code output. We can do an unstable
        // sort because the registers will be unique (there are no dups).
        regs.sort_unstable();

        // Compute clobber size.  We only need to count FPR save slots.
        let mut clobber_size = 0;
        for reg in &regs {
            match reg.to_reg().class() {
                RegClass::Int => {}
                RegClass::Float => {
                    clobber_size += 8;
                }
                RegClass::Vector => unreachable!(),
            }
        }

        // Common code assumes that tail-call arguments are part of the caller's
        // frame.  This is not correct for our tail-call convention.  To ensure
        // common code still gets the total size of this stack frame correct,
        // we add the (incoming and outgoing) taill-call argument size to the
        // "clobber" size.
        if call_conv == isa::CallConv::Tail {
            clobber_size += tail_args_size;
        }

        // Return FrameLayout structure.
        FrameLayout {
            word_bytes: 8,
            incoming_args_size,
            // We already accounted for tail-call arguments above, so reset
            // this value to its default.
            tail_args_size: incoming_args_size,
            setup_area_size: 0,
            clobber_size,
            fixed_frame_storage_size,
            stackslots_size,
            outgoing_args_size,
            clobbered_callee_saves: regs,
        }
    }

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        panic!("Should not be called");
    }

    fn exception_payload_regs(call_conv: isa::CallConv) -> &'static [Reg] {
        const PAYLOAD_REGS: &'static [Reg] = &[gpr(6), gpr(7)];
        match call_conv {
            isa::CallConv::SystemV | isa::CallConv::Tail => PAYLOAD_REGS,
            _ => &[],
        }
    }
}

impl S390xMachineDeps {
    pub fn gen_tail_epilogue(
        frame_layout: &FrameLayout,
        callee_pop_size: u32,
        dest: &CallInstDest,
    ) -> (SmallVec<[Inst; 16]>, Option<Reg>) {
        let mut insts = SmallVec::new();
        let call_conv = isa::CallConv::Tail;

        // Restore FPRs.
        insts.extend(gen_restore_fprs(frame_layout));

        // If the tail call target is in a callee-saved GPR, we need to move it
        // to %r1 (as the only available temp register) before restoring GPRs
        // (but after restoring FPRs, which might clobber %r1).
        let temp_dest = match dest {
            CallInstDest::Indirect { reg }
                if reg.to_real_reg().is_some()
                    && is_reg_saved_in_prologue(call_conv, reg.to_real_reg().unwrap()) =>
            {
                insts.push(Inst::Mov64 {
                    rd: writable_gpr(1),
                    rm: *reg,
                });
                Some(gpr(1))
            }
            _ => None,
        };

        // Restore GPRs (including SP).
        insts.extend(gen_restore_gprs(call_conv, frame_layout, callee_pop_size));

        (insts, temp_dest)
    }

    /// Emit loads for any stack-carried return values using the call
    /// info and allocations.  In addition, emit lane swaps for all
    /// vector-types return values if needed.
    pub fn gen_retval_loads(info: &CallInfo<CallInstDest>) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();

        // Helper routine to lane-swap a register if needed.
        let lane_swap_if_needed = |insts: &mut SmallInstVec<Inst>, vreg, ty: Type| {
            if LaneOrder::from(info.caller_conv) != LaneOrder::from(info.callee_conv) {
                if ty.is_vector() && ty.lane_count() >= 2 {
                    insts.push(Inst::VecEltRev {
                        lane_count: ty.lane_count(),
                        rd: vreg,
                        rn: vreg.to_reg(),
                    });
                }
            }
        };

        // Helper routine to allocate a temp register for ty.
        let temp_reg = |ty| match Inst::rc_for_type(ty).unwrap() {
            (&[RegClass::Int], _) => writable_gpr(0),
            (&[RegClass::Float], _) => writable_vr(1),
            _ => unreachable!(),
        };

        // Do a first pass over the return locations to handle copies that
        // need temp registers.  These need to be done before regular stack
        // loads in case the destination of a load happens to be our temp
        // register.  (The temp registers by choice are distinct from all
        // real return registers, which we verify here again.)
        for CallRetPair { vreg, location } in &info.defs {
            match location {
                RetLocation::Reg(preg, ty) => {
                    debug_assert!(*preg != temp_reg(*ty).to_reg());
                }
                RetLocation::Stack(amode, ty) => {
                    if let Some(spillslot) = vreg.to_reg().to_spillslot() {
                        let temp = temp_reg(*ty);
                        insts.push(Inst::gen_load(temp, (*amode).into(), *ty));
                        lane_swap_if_needed(&mut insts, temp, *ty);
                        insts.push(Inst::gen_store(
                            MemArg::SpillOffset {
                                off: 8 * (spillslot.index() as i64),
                            },
                            temp.to_reg(),
                            Inst::canonical_type_for_rc(temp.to_reg().class()),
                        ));
                    }
                }
            }
        }
        // Now handle all remaining return locations.
        for CallRetPair { vreg, location } in &info.defs {
            match location {
                RetLocation::Reg(preg, ty) => {
                    lane_swap_if_needed(&mut insts, Writable::from_reg(*preg), *ty);
                }
                RetLocation::Stack(amode, ty) => {
                    if vreg.to_reg().to_spillslot().is_none() {
                        insts.push(Inst::gen_load(*vreg, (*amode).into(), *ty));
                        lane_swap_if_needed(&mut insts, *vreg, *ty);
                    }
                }
            }
        }
        insts
    }
}

fn is_reg_saved_in_prologue(call_conv: isa::CallConv, r: RealReg) -> bool {
    match (call_conv, r.class()) {
        (isa::CallConv::Tail, RegClass::Int) => {
            // r8 - r15 inclusive are callee-saves.
            r.hw_enc() >= 8 && r.hw_enc() <= 15
        }
        (_, RegClass::Int) => {
            // r6 - r15 inclusive are callee-saves.
            r.hw_enc() >= 6 && r.hw_enc() <= 15
        }
        (_, RegClass::Float) => {
            // f8 - f15 inclusive are callee-saves.
            r.hw_enc() >= 8 && r.hw_enc() <= 15
        }
        (_, RegClass::Vector) => unreachable!(),
    }
}

fn get_clobbered_gprs(frame_layout: &FrameLayout) -> Option<(u8, u8)> {
    // Collect clobbered GPRs.  Note we save/restore GPR always as
    // a block of registers using LOAD MULTIPLE / STORE MULTIPLE, starting
    // with the clobbered GPR with the lowest number up to the clobbered GPR
    // with the highest number.
    let (clobbered_gpr, _) = frame_layout.clobbered_callee_saves_by_class();
    if clobbered_gpr.is_empty() {
        return None;
    }

    let first = clobbered_gpr.first().unwrap().to_reg().hw_enc();
    let last = clobbered_gpr.last().unwrap().to_reg().hw_enc();
    debug_assert!(clobbered_gpr.iter().all(|r| r.to_reg().hw_enc() >= first));
    debug_assert!(clobbered_gpr.iter().all(|r| r.to_reg().hw_enc() <= last));
    Some((first, last))
}

fn get_clobbered_fprs(frame_layout: &FrameLayout) -> &[Writable<RealReg>] {
    // Collect clobbered floating-point registers.
    let (_, clobbered_fpr) = frame_layout.clobbered_callee_saves_by_class();
    clobbered_fpr
}

// Restore GPRs (including SP) from the register save area.
// This must not clobber any register, specifically including %r1.
fn gen_restore_gprs(
    call_conv: isa::CallConv,
    frame_layout: &FrameLayout,
    callee_pop_size: u32,
) -> SmallVec<[Inst; 16]> {
    let mut insts = SmallVec::new();

    // Determine GPRs to be restored.
    let clobbered_gpr = get_clobbered_gprs(frame_layout);

    // Increment stack pointer unless it will be restored implicitly.
    // Note that implicit stack pointer restoration cannot be done in the
    // presence of either incoming or outgoing tail call arguments.
    let stack_size = frame_layout.outgoing_args_size as i32
        + frame_layout.clobber_size as i32
        + frame_layout.fixed_frame_storage_size as i32;
    let implicit_sp_restore = callee_pop_size == 0
        && (call_conv != isa::CallConv::Tail || frame_layout.incoming_args_size == 0)
        && clobbered_gpr.map_or(false, |(first, _)| {
            SImm20::maybe_from_i64(8 * first as i64 + stack_size as i64).is_some()
        });
    if !implicit_sp_restore {
        insts.extend(S390xMachineDeps::gen_sp_reg_adjust(
            stack_size - callee_pop_size as i32,
        ));
    }

    // Use LMG to restore clobbered GPRs from save area.
    if let Some((first, mut last)) = clobbered_gpr {
        // Attempt to restore via SP, taking implicit restoration into account.
        let mut reg = stack_reg();
        let mut offset = callee_pop_size as i64 + 8 * first as i64;
        if implicit_sp_restore {
            offset += stack_size as i64 - callee_pop_size as i64;
            last = 15;
        }
        // If the offset still overflows, use the first restored GPR
        // as temporary holding the address, as we cannot use %r1.
        if SImm20::maybe_from_i64(offset).is_none() {
            insts.extend(S390xMachineDeps::gen_add_imm(
                call_conv,
                writable_gpr(first),
                stack_reg(),
                offset as u32,
            ));
            reg = gpr(first);
            offset = 0;
        }
        // Now this LMG will always have an in-range offset.
        insts.push(Inst::LoadMultiple64 {
            rt: writable_gpr(first),
            rt2: writable_gpr(last),
            mem: MemArg::reg_plus_off(reg, offset, MemFlags::trusted()),
        });
    }

    insts
}

// Restore FPRs from the clobber area.
fn gen_restore_fprs(frame_layout: &FrameLayout) -> SmallVec<[Inst; 16]> {
    let mut insts = SmallVec::new();

    // Determine FPRs to be restored.
    let clobbered_fpr = get_clobbered_fprs(frame_layout);

    // Restore FPRs.
    for (i, reg) in clobbered_fpr.iter().enumerate() {
        insts.push(Inst::VecLoadLaneUndef {
            size: 64,
            rd: Writable::from_reg(reg.to_reg().into()),
            mem: MemArg::reg_plus_off(
                stack_reg(),
                (i * 8) as i64
                    + frame_layout.outgoing_args_size as i64
                    + frame_layout.fixed_frame_storage_size as i64,
                MemFlags::trusted(),
            ),
            lane_imm: 0,
        });
    }

    insts
}

const fn sysv_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(gpr_preg(0))
        .with(gpr_preg(1))
        .with(gpr_preg(2))
        .with(gpr_preg(3))
        .with(gpr_preg(4))
        .with(gpr_preg(5))
        // v0 - v7 inclusive and v16 - v31 inclusive are
        // caller-saves. The upper 64 bits of v8 - v15 inclusive are
        // also caller-saves.  However, because we cannot currently
        // represent partial registers to regalloc2, we indicate here
        // that every vector register is caller-save. Because this
        // function is used at *callsites*, approximating in this
        // direction (save more than necessary) is conservative and
        // thus safe.
        //
        // Note that we exclude clobbers from a call instruction when
        // a call instruction's callee has the same ABI as the caller
        // (the current function body); this is safe (anything
        // clobbered by callee can be clobbered by caller as well) and
        // avoids unnecessary saves of v8-v15 in the prologue even
        // though we include them as defs here.
        .with(vr_preg(0))
        .with(vr_preg(1))
        .with(vr_preg(2))
        .with(vr_preg(3))
        .with(vr_preg(4))
        .with(vr_preg(5))
        .with(vr_preg(6))
        .with(vr_preg(7))
        .with(vr_preg(8))
        .with(vr_preg(9))
        .with(vr_preg(10))
        .with(vr_preg(11))
        .with(vr_preg(12))
        .with(vr_preg(13))
        .with(vr_preg(14))
        .with(vr_preg(15))
        .with(vr_preg(16))
        .with(vr_preg(17))
        .with(vr_preg(18))
        .with(vr_preg(19))
        .with(vr_preg(20))
        .with(vr_preg(21))
        .with(vr_preg(22))
        .with(vr_preg(23))
        .with(vr_preg(24))
        .with(vr_preg(25))
        .with(vr_preg(26))
        .with(vr_preg(27))
        .with(vr_preg(28))
        .with(vr_preg(29))
        .with(vr_preg(30))
        .with(vr_preg(31))
}
const SYSV_CLOBBERS: PRegSet = sysv_clobbers();

const fn tail_clobbers() -> PRegSet {
    // Same as the SystemV ABI, except that %r6 and %r7 are clobbered.
    PRegSet::empty()
        .with(gpr_preg(0))
        .with(gpr_preg(1))
        .with(gpr_preg(2))
        .with(gpr_preg(3))
        .with(gpr_preg(4))
        .with(gpr_preg(5))
        .with(gpr_preg(6))
        .with(gpr_preg(7))
        .with(vr_preg(0))
        .with(vr_preg(1))
        .with(vr_preg(2))
        .with(vr_preg(3))
        .with(vr_preg(4))
        .with(vr_preg(5))
        .with(vr_preg(6))
        .with(vr_preg(7))
        .with(vr_preg(8))
        .with(vr_preg(9))
        .with(vr_preg(10))
        .with(vr_preg(11))
        .with(vr_preg(12))
        .with(vr_preg(13))
        .with(vr_preg(14))
        .with(vr_preg(15))
        .with(vr_preg(16))
        .with(vr_preg(17))
        .with(vr_preg(18))
        .with(vr_preg(19))
        .with(vr_preg(20))
        .with(vr_preg(21))
        .with(vr_preg(22))
        .with(vr_preg(23))
        .with(vr_preg(24))
        .with(vr_preg(25))
        .with(vr_preg(26))
        .with(vr_preg(27))
        .with(vr_preg(28))
        .with(vr_preg(29))
        .with(vr_preg(30))
        .with(vr_preg(31))
}
const TAIL_CLOBBERS: PRegSet = tail_clobbers();

const fn all_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(gpr_preg(0))
        .with(gpr_preg(1))
        .with(gpr_preg(2))
        .with(gpr_preg(3))
        .with(gpr_preg(4))
        .with(gpr_preg(5))
        .with(gpr_preg(6))
        .with(gpr_preg(7))
        .with(gpr_preg(8))
        .with(gpr_preg(9))
        .with(gpr_preg(10))
        .with(gpr_preg(11))
        .with(gpr_preg(12))
        .with(gpr_preg(13))
        .with(gpr_preg(14))
        .with(gpr_preg(15))
        .with(vr_preg(0))
        .with(vr_preg(1))
        .with(vr_preg(2))
        .with(vr_preg(3))
        .with(vr_preg(4))
        .with(vr_preg(5))
        .with(vr_preg(6))
        .with(vr_preg(7))
        .with(vr_preg(8))
        .with(vr_preg(9))
        .with(vr_preg(10))
        .with(vr_preg(11))
        .with(vr_preg(12))
        .with(vr_preg(13))
        .with(vr_preg(14))
        .with(vr_preg(15))
        .with(vr_preg(16))
        .with(vr_preg(17))
        .with(vr_preg(18))
        .with(vr_preg(19))
        .with(vr_preg(20))
        .with(vr_preg(21))
        .with(vr_preg(22))
        .with(vr_preg(23))
        .with(vr_preg(24))
        .with(vr_preg(25))
        .with(vr_preg(26))
        .with(vr_preg(27))
        .with(vr_preg(28))
        .with(vr_preg(29))
        .with(vr_preg(30))
        .with(vr_preg(31))
}
const ALL_CLOBBERS: PRegSet = all_clobbers();

fn sysv_create_machine_env() -> MachineEnv {
    MachineEnv {
        preferred_regs_by_class: [
            vec![
                // no r0; can't use for addressing?
                // no r1; it is our spilltmp.
                gpr_preg(2),
                gpr_preg(3),
                gpr_preg(4),
                gpr_preg(5),
            ],
            vec![
                vr_preg(0),
                vr_preg(1),
                vr_preg(2),
                vr_preg(3),
                vr_preg(4),
                vr_preg(5),
                vr_preg(6),
                vr_preg(7),
                vr_preg(16),
                vr_preg(17),
                vr_preg(18),
                vr_preg(19),
                vr_preg(20),
                vr_preg(21),
                vr_preg(22),
                vr_preg(23),
                vr_preg(24),
                vr_preg(25),
                vr_preg(26),
                vr_preg(27),
                vr_preg(28),
                vr_preg(29),
                vr_preg(30),
                vr_preg(31),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        non_preferred_regs_by_class: [
            vec![
                gpr_preg(6),
                gpr_preg(7),
                gpr_preg(8),
                gpr_preg(9),
                gpr_preg(10),
                gpr_preg(11),
                gpr_preg(12),
                gpr_preg(13),
                gpr_preg(14),
                // no r15; it is the stack pointer.
            ],
            vec![
                vr_preg(8),
                vr_preg(9),
                vr_preg(10),
                vr_preg(11),
                vr_preg(12),
                vr_preg(13),
                vr_preg(14),
                vr_preg(15),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    }
}

fn tail_create_machine_env() -> MachineEnv {
    // Same as the SystemV ABI, except that %r6 and %r7 are preferred.
    MachineEnv {
        preferred_regs_by_class: [
            vec![
                // no r0; can't use for addressing?
                // no r1; it is our spilltmp.
                gpr_preg(2),
                gpr_preg(3),
                gpr_preg(4),
                gpr_preg(5),
                gpr_preg(6),
                gpr_preg(7),
            ],
            vec![
                vr_preg(0),
                vr_preg(1),
                vr_preg(2),
                vr_preg(3),
                vr_preg(4),
                vr_preg(5),
                vr_preg(6),
                vr_preg(7),
                vr_preg(16),
                vr_preg(17),
                vr_preg(18),
                vr_preg(19),
                vr_preg(20),
                vr_preg(21),
                vr_preg(22),
                vr_preg(23),
                vr_preg(24),
                vr_preg(25),
                vr_preg(26),
                vr_preg(27),
                vr_preg(28),
                vr_preg(29),
                vr_preg(30),
                vr_preg(31),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        non_preferred_regs_by_class: [
            vec![
                gpr_preg(8),
                gpr_preg(9),
                gpr_preg(10),
                gpr_preg(11),
                gpr_preg(12),
                gpr_preg(13),
                gpr_preg(14),
                // no r15; it is the stack pointer.
            ],
            vec![
                vr_preg(8),
                vr_preg(9),
                vr_preg(10),
                vr_preg(11),
                vr_preg(12),
                vr_preg(13),
                vr_preg(14),
                vr_preg(15),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    }
}
