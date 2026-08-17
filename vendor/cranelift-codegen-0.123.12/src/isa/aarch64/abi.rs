//! Implementation of a standard AArch64 ABI.

use crate::CodegenResult;
use crate::ir;
use crate::ir::MemFlags;
use crate::ir::types;
use crate::ir::types::*;
use crate::ir::{ExternalName, LibCall, Signature, dynamic_to_fixed};
use crate::isa;
use crate::isa::aarch64::{inst::*, settings as aarch64_settings};
use crate::isa::unwind::UnwindInst;
use crate::isa::winch;
use crate::machinst::*;
use crate::settings;
use alloc::boxed::Box;
use alloc::vec::Vec;
use regalloc2::{MachineEnv, PReg, PRegSet};
use smallvec::{SmallVec, smallvec};
use std::borrow::ToOwned;
use std::sync::OnceLock;

// We use a generic implementation that factors out AArch64 and x64 ABI commonalities, because
// these ABIs are very similar.

/// Support for the AArch64 ABI from the callee side (within a function body).
pub(crate) type AArch64Callee = Callee<AArch64MachineDeps>;

impl From<StackAMode> for AMode {
    fn from(stack: StackAMode) -> AMode {
        match stack {
            StackAMode::IncomingArg(off, stack_args_size) => AMode::IncomingArg {
                off: i64::from(stack_args_size) - off,
            },
            StackAMode::Slot(off) => AMode::SlotOffset { off },
            StackAMode::OutgoingArg(off) => AMode::SPOffset { off },
        }
    }
}

// Returns the size of stack space needed to store the
// `clobbered_callee_saved` registers.
fn compute_clobber_size(clobbered_callee_saves: &[Writable<RealReg>]) -> u32 {
    let mut int_regs = 0;
    let mut vec_regs = 0;
    for &reg in clobbered_callee_saves {
        match reg.to_reg().class() {
            RegClass::Int => {
                int_regs += 1;
            }
            RegClass::Float => {
                vec_regs += 1;
            }
            RegClass::Vector => unreachable!(),
        }
    }

    // Round up to multiple of 2, to keep 16-byte stack alignment.
    let int_save_bytes = (int_regs + (int_regs & 1)) * 8;
    // The Procedure Call Standard for the Arm 64-bit Architecture
    // (AAPCS64, including several related ABIs such as the one used by
    // Windows) mandates saving only the bottom 8 bytes of the vector
    // registers, so we round up the number of registers to ensure
    // proper stack alignment (similarly to the situation with
    // `int_reg`).
    let vec_reg_size = 8;
    let vec_save_padding = vec_regs & 1;
    // FIXME: SVE: ABI is different to Neon, so do we treat all vec regs as Z-regs?
    let vec_save_bytes = (vec_regs + vec_save_padding) * vec_reg_size;

    int_save_bytes + vec_save_bytes
}

/// AArch64-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct AArch64MachineDeps;

impl IsaFlags for aarch64_settings::Flags {
    fn is_forward_edge_cfi_enabled(&self) -> bool {
        self.use_bti()
    }
}

impl ABIMachineSpec for AArch64MachineDeps {
    type I = Inst;

    type F = aarch64_settings::Flags;

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
        let is_apple_cc = call_conv == isa::CallConv::AppleAarch64;
        let is_winch_return = call_conv == isa::CallConv::Winch && args_or_rets == ArgsOrRets::Rets;

        // See AArch64 ABI (https://github.com/ARM-software/abi-aa/blob/2021Q1/aapcs64/aapcs64.rst#64parameter-passing), sections 6.4.
        //
        // MacOS aarch64 is slightly different, see also
        // https://developer.apple.com/documentation/xcode/writing_arm64_code_for_apple_platforms.
        // We are diverging from the MacOS aarch64 implementation in the
        // following ways:
        // - sign- and zero- extensions of data types less than 32 bits are not
        // implemented yet.
        // - we align the arguments stack space to a 16-bytes boundary, while
        // the MacOS allows aligning only on 8 bytes. In practice it means we're
        // slightly overallocating when calling, which is fine, and doesn't
        // break our other invariants that the stack is always allocated in
        // 16-bytes chunks.

        let mut next_xreg = if call_conv == isa::CallConv::Tail {
            // We reserve `x0` for the return area pointer. For simplicity, we
            // reserve it even when there is no return area pointer needed. This
            // also means that identity functions don't have to shuffle arguments to
            // different return registers because we shifted all argument register
            // numbers down by one to make space for the return area pointer.
            //
            // Also, we cannot use all allocatable GPRs as arguments because we need
            // at least one allocatable register for holding the callee address in
            // indirect calls. So skip `x1` also, reserving it for that role.
            2
        } else {
            0
        };
        let mut next_vreg = 0;
        let mut next_stack: u32 = 0;

        // Note on return values: on the regular ABI, we may return values
        // in 8 registers for V128 and I64 registers independently of the
        // number of register values returned in the other class. That is,
        // we can return values in up to 8 integer and
        // 8 vector registers at once.
        let max_per_class_reg_vals = 8; // x0-x7 and v0-v7
        let mut remaining_reg_vals = 16;

        let ret_area_ptr = if add_ret_area_ptr {
            debug_assert_eq!(args_or_rets, ArgsOrRets::Args);
            if call_conv != isa::CallConv::Winch {
                // In the AAPCS64 calling convention the return area pointer is
                // stored in x8.
                Some(ABIArg::reg(
                    xreg(8).to_real_reg().unwrap(),
                    I64,
                    ir::ArgumentExtension::None,
                    ir::ArgumentPurpose::Normal,
                ))
            } else {
                // Use x0 for the return area pointer in the Winch calling convention
                // to simplify the ABI handling code in Winch by avoiding an AArch64
                // special case to assign it to x8.
                next_xreg += 1;
                Some(ABIArg::reg(
                    xreg(0).to_real_reg().unwrap(),
                    I64,
                    ir::ArgumentExtension::None,
                    ir::ArgumentPurpose::Normal,
                ))
            }
        } else {
            None
        };

        for (i, param) in params.into_iter().enumerate() {
            if is_apple_cc && param.value_type == types::F128 && !flags.enable_llvm_abi_extensions()
            {
                panic!(
                    "f128 args/return values not supported for apple_aarch64 unless LLVM ABI extensions are enabled"
                );
            }

            let (rcs, reg_types) = Inst::rc_for_type(param.value_type)?;

            if let ir::ArgumentPurpose::StructReturn = param.purpose {
                assert!(
                    call_conv != isa::CallConv::Tail,
                    "support for StructReturn parameters is not implemented for the `tail` \
                    calling convention yet",
                );
            }

            if let ir::ArgumentPurpose::StructArgument(_) = param.purpose {
                panic!(
                    "StructArgument parameters are not supported on arm64. \
                    Use regular pointer arguments instead."
                );
            }

            if let ir::ArgumentPurpose::StructReturn = param.purpose {
                // FIXME add assert_eq!(args_or_rets, ArgsOrRets::Args); once
                // ensure_struct_return_ptr_is_returned is gone.
                assert!(
                    param.value_type == types::I64,
                    "StructReturn must be a pointer sized integer"
                );
                args.push(ABIArg::Slots {
                    slots: smallvec![ABIArgSlot::Reg {
                        reg: xreg(8).to_real_reg().unwrap(),
                        ty: types::I64,
                        extension: param.extension,
                    },],
                    purpose: ir::ArgumentPurpose::StructReturn,
                });
                continue;
            }

            // Handle multi register params
            //
            // See AArch64 ABI (https://github.com/ARM-software/abi-aa/blob/2021Q1/aapcs64/aapcs64.rst#642parameter-passing-rules), (Section 6.4.2 Stage C).
            //
            // For arguments with alignment of 16 we round up the register number
            // to the next even value. So we can never allocate for example an i128
            // to X1 and X2, we have to skip one register and do X2, X3
            // (Stage C.8)
            // Note: The Apple ABI deviates a bit here. They don't respect Stage C.8
            // and will happily allocate a i128 to X1 and X2
            //
            // For integer types with alignment of 16 we also have the additional
            // restriction of passing the lower half in Xn and the upper half in Xn+1
            // (Stage C.9)
            //
            // For examples of how LLVM handles this: https://godbolt.org/z/bhd3vvEfh
            //
            // On the Apple ABI it is unspecified if we can spill half the value into the stack
            // i.e load the lower half into x7 and the upper half into the stack
            // LLVM does not seem to do this, so we are going to replicate that behaviour
            let is_multi_reg = rcs.len() >= 2;
            if is_multi_reg {
                assert!(
                    rcs.len() == 2,
                    "Unable to handle multi reg params with more than 2 regs"
                );
                assert!(
                    rcs == &[RegClass::Int, RegClass::Int],
                    "Unable to handle non i64 regs"
                );

                let reg_class_space = max_per_class_reg_vals - next_xreg;
                let reg_space = remaining_reg_vals;

                if reg_space >= 2 && reg_class_space >= 2 {
                    // The aarch64 ABI does not allow us to start a split argument
                    // at an odd numbered register. So we need to skip one register
                    //
                    // TODO: The Fast ABI should probably not skip the register
                    if !is_apple_cc && next_xreg % 2 != 0 {
                        next_xreg += 1;
                    }

                    let lower_reg = xreg(next_xreg);
                    let upper_reg = xreg(next_xreg + 1);

                    args.push(ABIArg::Slots {
                        slots: smallvec![
                            ABIArgSlot::Reg {
                                reg: lower_reg.to_real_reg().unwrap(),
                                ty: reg_types[0],
                                extension: param.extension,
                            },
                            ABIArgSlot::Reg {
                                reg: upper_reg.to_real_reg().unwrap(),
                                ty: reg_types[1],
                                extension: param.extension,
                            },
                        ],
                        purpose: param.purpose,
                    });

                    next_xreg += 2;
                    remaining_reg_vals -= 2;
                    continue;
                }
            } else {
                // Single Register parameters
                let rc = rcs[0];
                let next_reg = match rc {
                    RegClass::Int => &mut next_xreg,
                    RegClass::Float => &mut next_vreg,
                    RegClass::Vector => unreachable!(),
                };

                let push_to_reg = if is_winch_return {
                    // Winch uses the first register to return the last result
                    i == params.len() - 1
                } else {
                    // Use max_per_class_reg_vals & remaining_reg_vals otherwise
                    *next_reg < max_per_class_reg_vals && remaining_reg_vals > 0
                };

                if push_to_reg {
                    let reg = match rc {
                        RegClass::Int => xreg(*next_reg),
                        RegClass::Float => vreg(*next_reg),
                        RegClass::Vector => unreachable!(),
                    };
                    // Overlay Z-regs on V-regs for parameter passing.
                    let ty = if param.value_type.is_dynamic_vector() {
                        dynamic_to_fixed(param.value_type)
                    } else {
                        param.value_type
                    };
                    args.push(ABIArg::reg(
                        reg.to_real_reg().unwrap(),
                        ty,
                        param.extension,
                        param.purpose,
                    ));
                    *next_reg += 1;
                    remaining_reg_vals -= 1;
                    continue;
                }
            }

            // Spill to the stack

            if args_or_rets == ArgsOrRets::Rets && !flags.enable_multi_ret_implicit_sret() {
                return Err(crate::CodegenError::Unsupported(
                    "Too many return values to fit in registers. \
                    Use a StructReturn argument instead. (#9510)"
                        .to_owned(),
                ));
            }

            // Compute the stack slot's size.
            let size = (ty_bits(param.value_type) / 8) as u32;

            let size = if is_apple_cc || is_winch_return {
                // MacOS and Winch aarch64 allows stack slots with
                // sizes less than 8 bytes. They still need to be
                // properly aligned on their natural data alignment,
                // though.
                size
            } else {
                // Every arg takes a minimum slot of 8 bytes. (16-byte stack
                // alignment happens separately after all args.)
                std::cmp::max(size, 8)
            };

            if !is_winch_return {
                // Align the stack slot.
                debug_assert!(size.is_power_of_two());
                next_stack = align_to(next_stack, size);
            }

            let slots = reg_types
                .iter()
                .copied()
                // Build the stack locations from each slot
                .scan(next_stack, |next_stack, ty| {
                    let slot_offset = *next_stack as i64;
                    *next_stack += (ty_bits(ty) / 8) as u32;

                    Some((ty, slot_offset))
                })
                .map(|(ty, offset)| ABIArgSlot::Stack {
                    offset,
                    ty,
                    extension: param.extension,
                })
                .collect();

            args.push(ABIArg::Slots {
                slots,
                purpose: param.purpose,
            });

            next_stack += size;
        }

        let extra_arg = if let Some(ret_area_ptr) = ret_area_ptr {
            args.push_non_formal(ret_area_ptr);
            Some(args.args().len() - 1)
        } else {
            None
        };

        if is_winch_return {
            winch::reverse_stack(args, next_stack, false);
        }

        next_stack = align_to(next_stack, 16);

        Ok((next_stack, extra_arg))
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
        let imm = imm as u64;
        let mut insts = SmallVec::new();
        if let Some(imm12) = Imm12::maybe_from_u64(imm) {
            insts.push(Inst::AluRRImm12 {
                alu_op: ALUOp::Add,
                size: OperandSize::Size64,
                rd: into_reg,
                rn: from_reg,
                imm12,
            });
        } else {
            let scratch2 = writable_tmp2_reg();
            assert_ne!(scratch2.to_reg(), from_reg);
            // `gen_add_imm` is only ever called after register allocation has taken place, and as a
            // result it's ok to reuse the scratch2 register here. If that changes, we'll need to
            // plumb through a way to allocate temporary virtual registers
            insts.extend(Inst::load_constant(scratch2, imm));
            insts.push(Inst::AluRRRExtend {
                alu_op: ALUOp::Add,
                size: OperandSize::Size64,
                rd: into_reg,
                rn: from_reg,
                rm: scratch2.to_reg(),
                extendop: ExtendOp::UXTX,
            });
        }
        insts
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Inst> {
        let mut insts = SmallVec::new();
        insts.push(Inst::AluRRRExtend {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size64,
            rd: writable_zero_reg(),
            rn: stack_reg(),
            rm: limit_reg,
            extendop: ExtendOp::UXTX,
        });
        insts.push(Inst::TrapIf {
            trap_code: ir::TrapCode::STACK_OVERFLOW,
            // Here `Lo` == "less than" when interpreting the two
            // operands as unsigned integers.
            kind: CondBrKind::Cond(Cond::Lo),
        });
        insts
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Inst {
        // FIXME: Do something different for dynamic types?
        let mem = mem.into();
        Inst::LoadAddr { rd: into_reg, mem }
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        spilltmp_reg()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Inst {
        let mem = AMode::RegOffset {
            rn: base,
            off: offset as i64,
        };
        Inst::gen_load(into_reg, mem, ty, MemFlags::trusted())
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Inst {
        let mem = AMode::RegOffset {
            rn: base,
            off: offset as i64,
        };
        Inst::gen_store(mem, from_reg, ty, MemFlags::trusted())
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Inst> {
        if amount == 0 {
            return SmallVec::new();
        }

        let (amount, is_sub) = if amount > 0 {
            (amount as u64, false)
        } else {
            (-amount as u64, true)
        };

        let alu_op = if is_sub { ALUOp::Sub } else { ALUOp::Add };

        let mut ret = SmallVec::new();
        if let Some(imm12) = Imm12::maybe_from_u64(amount) {
            let adj_inst = Inst::AluRRImm12 {
                alu_op,
                size: OperandSize::Size64,
                rd: writable_stack_reg(),
                rn: stack_reg(),
                imm12,
            };
            ret.push(adj_inst);
        } else {
            let tmp = writable_spilltmp_reg();
            // `gen_sp_reg_adjust` is called after regalloc2, so it's acceptable to reuse `tmp` for
            // intermediates in `load_constant`.
            let const_inst = Inst::load_constant(tmp, amount);
            let adj_inst = Inst::AluRRRExtend {
                alu_op,
                size: OperandSize::Size64,
                rd: writable_stack_reg(),
                rn: stack_reg(),
                rm: tmp.to_reg(),
                extendop: ExtendOp::UXTX,
            };
            ret.extend(const_inst);
            ret.push(adj_inst);
        }
        ret
    }

    fn gen_prologue_frame_setup(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        isa_flags: &aarch64_settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        let setup_frame = frame_layout.setup_area_size > 0;
        let mut insts = SmallVec::new();

        match Self::select_api_key(isa_flags, call_conv, setup_frame) {
            Some(key) => {
                insts.push(Inst::Paci { key });
                if flags.unwind_info() {
                    insts.push(Inst::Unwind {
                        inst: UnwindInst::Aarch64SetPointerAuth {
                            return_addresses: true,
                        },
                    });
                }
            }
            None => {
                if isa_flags.use_bti() {
                    insts.push(Inst::Bti {
                        targets: BranchTargetType::C,
                    });
                }

                if flags.unwind_info() && call_conv == isa::CallConv::AppleAarch64 {
                    // The macOS unwinder seems to require this.
                    insts.push(Inst::Unwind {
                        inst: UnwindInst::Aarch64SetPointerAuth {
                            return_addresses: false,
                        },
                    });
                }
            }
        }

        if setup_frame {
            // stp fp (x29), lr (x30), [sp, #-16]!
            insts.push(Inst::StoreP64 {
                rt: fp_reg(),
                rt2: link_reg(),
                mem: PairAMode::SPPreIndexed {
                    simm7: SImm7Scaled::maybe_from_i64(-16, types::I64).unwrap(),
                },
                flags: MemFlags::trusted(),
            });

            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::PushFrameRegs {
                        offset_upward_to_caller_sp: frame_layout.setup_area_size,
                    },
                });
            }

            // mov fp (x29), sp. This uses the ADDI rd, rs, 0 form of `MOV` because
            // the usual encoding (`ORR`) does not work with SP.
            insts.push(Inst::AluRRImm12 {
                alu_op: ALUOp::Add,
                size: OperandSize::Size64,
                rd: writable_fp_reg(),
                rn: stack_reg(),
                imm12: Imm12 {
                    bits: 0,
                    shift12: false,
                },
            });
        }

        insts
    }

    fn gen_epilogue_frame_restore(
        call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &aarch64_settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        let setup_frame = frame_layout.setup_area_size > 0;
        let mut insts = SmallVec::new();

        if setup_frame {
            // N.B.: sp is already adjusted to the appropriate place by the
            // clobber-restore code (which also frees the fixed frame). Hence, there
            // is no need for the usual `mov sp, fp` here.

            // `ldp fp, lr, [sp], #16`
            insts.push(Inst::LoadP64 {
                rt: writable_fp_reg(),
                rt2: writable_link_reg(),
                mem: PairAMode::SPPostIndexed {
                    simm7: SImm7Scaled::maybe_from_i64(16, types::I64).unwrap(),
                },
                flags: MemFlags::trusted(),
            });
        }

        if call_conv == isa::CallConv::Tail && frame_layout.tail_args_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(
                frame_layout.tail_args_size.try_into().unwrap(),
            ));
        }

        insts
    }

    fn gen_return(
        call_conv: isa::CallConv,
        isa_flags: &aarch64_settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        let setup_frame = frame_layout.setup_area_size > 0;

        match Self::select_api_key(isa_flags, call_conv, setup_frame) {
            Some(key) => {
                smallvec![Inst::AuthenticatedRet {
                    key,
                    is_hint: !isa_flags.has_pauth(),
                }]
            }
            None => {
                smallvec![Inst::Ret {}]
            }
        }
    }

    fn gen_probestack(_insts: &mut SmallInstVec<Self::I>, _: u32) {
        // TODO: implement if we ever require stack probes on an AArch64 host
        // (unlikely unless Lucet is ported)
        unimplemented!("Stack probing is unimplemented on AArch64");
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        // The stack probe loop currently takes 6 instructions and each inline
        // probe takes 2 (ish, these numbers sort of depend on the constants).
        // Set this to 3 to keep the max size of the probe to 6 instructions.
        const PROBE_MAX_UNROLL: u32 = 3;

        // Calculate how many probes we need to perform. Round down, as we only
        // need to probe whole guard_size regions we'd otherwise skip over.
        let probe_count = frame_size / guard_size;
        if probe_count == 0 {
            // No probe necessary
        } else if probe_count <= PROBE_MAX_UNROLL {
            Self::gen_probestack_unroll(insts, guard_size, probe_count)
        } else {
            Self::gen_probestack_loop(insts, frame_size, guard_size)
        }
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        let (clobbered_int, clobbered_vec) = frame_layout.clobbered_callee_saves_by_class();

        let mut insts = SmallVec::new();
        let setup_frame = frame_layout.setup_area_size > 0;

        // When a return_call within this function required more stack arguments than we have
        // present, resize the incoming argument area of the frame to accommodate those arguments.
        let incoming_args_diff = frame_layout.tail_args_size - frame_layout.incoming_args_size;
        if incoming_args_diff > 0 {
            // Decrement SP to account for the additional space required by a tail call.
            insts.extend(Self::gen_sp_reg_adjust(-(incoming_args_diff as i32)));
            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::StackAlloc {
                        size: incoming_args_diff,
                    },
                });
            }

            // Move fp and lr down.
            if setup_frame {
                // Reload the frame pointer from the stack.
                insts.push(Inst::ULoad64 {
                    rd: regs::writable_fp_reg(),
                    mem: AMode::SPOffset {
                        off: i64::from(incoming_args_diff),
                    },
                    flags: MemFlags::trusted(),
                });

                // Store the frame pointer and link register again at the new SP
                insts.push(Inst::StoreP64 {
                    rt: fp_reg(),
                    rt2: link_reg(),
                    mem: PairAMode::SignedOffset {
                        reg: regs::stack_reg(),
                        simm7: SImm7Scaled::maybe_from_i64(0, types::I64).unwrap(),
                    },
                    flags: MemFlags::trusted(),
                });

                // Keep the frame pointer in sync
                insts.push(Self::gen_move(
                    regs::writable_fp_reg(),
                    regs::stack_reg(),
                    types::I64,
                ));
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

        // We use pre-indexed addressing modes here, rather than the possibly
        // more efficient "subtract sp once then used fixed offsets" scheme,
        // because (i) we cannot necessarily guarantee that the offset of a
        // clobber-save slot will be within a SImm7Scaled (+504-byte) offset
        // range of the whole frame including other slots, it is more complex to
        // conditionally generate a two-stage SP adjustment (clobbers then fixed
        // frame) otherwise, and generally we just want to maintain simplicity
        // here for maintainability.  Because clobbers are at the top of the
        // frame, just below FP, all that is necessary is to use the pre-indexed
        // "push" `[sp, #-16]!` addressing mode.
        //
        // `frame_offset` tracks offset above start-of-clobbers for unwind-info
        // purposes.
        let mut clobber_offset = frame_layout.clobber_size;
        let clobber_offset_change = 16;
        let iter = clobbered_int.chunks_exact(2);

        if let [rd] = iter.remainder() {
            let rd: Reg = rd.to_reg().into();

            debug_assert_eq!(rd.class(), RegClass::Int);
            // str rd, [sp, #-16]!
            insts.push(Inst::Store64 {
                rd,
                mem: AMode::SPPreIndexed {
                    simm9: SImm9::maybe_from_i64(-clobber_offset_change).unwrap(),
                },
                flags: MemFlags::trusted(),
            });

            if flags.unwind_info() {
                clobber_offset -= clobber_offset_change as u32;
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset,
                        reg: rd.to_real_reg().unwrap(),
                    },
                });
            }
        }

        let mut iter = iter.rev();

        while let Some([rt, rt2]) = iter.next() {
            // .to_reg().into(): Writable<RealReg> --> RealReg --> Reg
            let rt: Reg = rt.to_reg().into();
            let rt2: Reg = rt2.to_reg().into();

            debug_assert!(rt.class() == RegClass::Int);
            debug_assert!(rt2.class() == RegClass::Int);

            // stp rt, rt2, [sp, #-16]!
            insts.push(Inst::StoreP64 {
                rt,
                rt2,
                mem: PairAMode::SPPreIndexed {
                    simm7: SImm7Scaled::maybe_from_i64(-clobber_offset_change, types::I64).unwrap(),
                },
                flags: MemFlags::trusted(),
            });

            if flags.unwind_info() {
                clobber_offset -= clobber_offset_change as u32;
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset,
                        reg: rt.to_real_reg().unwrap(),
                    },
                });
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset: clobber_offset + (clobber_offset_change / 2) as u32,
                        reg: rt2.to_real_reg().unwrap(),
                    },
                });
            }
        }

        let store_vec_reg = |rd| Inst::FpuStore64 {
            rd,
            mem: AMode::SPPreIndexed {
                simm9: SImm9::maybe_from_i64(-clobber_offset_change).unwrap(),
            },
            flags: MemFlags::trusted(),
        };
        let iter = clobbered_vec.chunks_exact(2);

        if let [rd] = iter.remainder() {
            let rd: Reg = rd.to_reg().into();

            debug_assert_eq!(rd.class(), RegClass::Float);
            insts.push(store_vec_reg(rd));

            if flags.unwind_info() {
                clobber_offset -= clobber_offset_change as u32;
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset,
                        reg: rd.to_real_reg().unwrap(),
                    },
                });
            }
        }

        let store_vec_reg_pair = |rt, rt2| {
            let clobber_offset_change = 16;

            (
                Inst::FpuStoreP64 {
                    rt,
                    rt2,
                    mem: PairAMode::SPPreIndexed {
                        simm7: SImm7Scaled::maybe_from_i64(-clobber_offset_change, F64).unwrap(),
                    },
                    flags: MemFlags::trusted(),
                },
                clobber_offset_change as u32,
            )
        };
        let mut iter = iter.rev();

        while let Some([rt, rt2]) = iter.next() {
            let rt: Reg = rt.to_reg().into();
            let rt2: Reg = rt2.to_reg().into();

            debug_assert_eq!(rt.class(), RegClass::Float);
            debug_assert_eq!(rt2.class(), RegClass::Float);

            let (inst, clobber_offset_change) = store_vec_reg_pair(rt, rt2);

            insts.push(inst);

            if flags.unwind_info() {
                clobber_offset -= clobber_offset_change;
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset,
                        reg: rt.to_real_reg().unwrap(),
                    },
                });
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset: clobber_offset + clobber_offset_change / 2,
                        reg: rt2.to_real_reg().unwrap(),
                    },
                });
            }
        }

        // Allocate the fixed frame below the clobbers if necessary.
        let stack_size = frame_layout.fixed_frame_storage_size + frame_layout.outgoing_args_size;
        if stack_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(-(stack_size as i32)));
            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::StackAlloc { size: stack_size },
                });
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
        let (clobbered_int, clobbered_vec) = frame_layout.clobbered_callee_saves_by_class();

        // Free the fixed frame if necessary.
        let stack_size = frame_layout.fixed_frame_storage_size + frame_layout.outgoing_args_size;
        if stack_size > 0 {
            insts.extend(Self::gen_sp_reg_adjust(stack_size as i32));
        }

        let load_vec_reg = |rd| Inst::FpuLoad64 {
            rd,
            mem: AMode::SPPostIndexed {
                simm9: SImm9::maybe_from_i64(16).unwrap(),
            },
            flags: MemFlags::trusted(),
        };
        let load_vec_reg_pair = |rt, rt2| Inst::FpuLoadP64 {
            rt,
            rt2,
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(16, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        };

        let mut iter = clobbered_vec.chunks_exact(2);

        while let Some([rt, rt2]) = iter.next() {
            let rt: Writable<Reg> = rt.map(|r| r.into());
            let rt2: Writable<Reg> = rt2.map(|r| r.into());

            debug_assert_eq!(rt.to_reg().class(), RegClass::Float);
            debug_assert_eq!(rt2.to_reg().class(), RegClass::Float);
            insts.push(load_vec_reg_pair(rt, rt2));
        }

        debug_assert!(iter.remainder().len() <= 1);

        if let [rd] = iter.remainder() {
            let rd: Writable<Reg> = rd.map(|r| r.into());

            debug_assert_eq!(rd.to_reg().class(), RegClass::Float);
            insts.push(load_vec_reg(rd));
        }

        let mut iter = clobbered_int.chunks_exact(2);

        while let Some([rt, rt2]) = iter.next() {
            let rt: Writable<Reg> = rt.map(|r| r.into());
            let rt2: Writable<Reg> = rt2.map(|r| r.into());

            debug_assert_eq!(rt.to_reg().class(), RegClass::Int);
            debug_assert_eq!(rt2.to_reg().class(), RegClass::Int);
            // ldp rt, rt2, [sp], #16
            insts.push(Inst::LoadP64 {
                rt,
                rt2,
                mem: PairAMode::SPPostIndexed {
                    simm7: SImm7Scaled::maybe_from_i64(16, I64).unwrap(),
                },
                flags: MemFlags::trusted(),
            });
        }

        debug_assert!(iter.remainder().len() <= 1);

        if let [rd] = iter.remainder() {
            let rd: Writable<Reg> = rd.map(|r| r.into());

            debug_assert_eq!(rd.to_reg().class(), RegClass::Int);
            // ldr rd, [sp], #16
            insts.push(Inst::ULoad64 {
                rd,
                mem: AMode::SPPostIndexed {
                    simm9: SImm9::maybe_from_i64(16).unwrap(),
                },
                flags: MemFlags::trusted(),
            });
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
        let arg0 = writable_xreg(0);
        let arg1 = writable_xreg(1);
        let arg2 = writable_xreg(2);
        let tmp = alloc_tmp(Self::word_type());
        insts.extend(Inst::load_constant(tmp, size as u64));
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
        vector_size: u32,
        _isa_flags: &Self::F,
    ) -> u32 {
        assert_eq!(vector_size % 8, 0);
        // We allocate in terms of 8-byte slots.
        match rc {
            RegClass::Int => 1,
            RegClass::Float => vector_size / 8,
            RegClass::Vector => unreachable!(),
        }
    }

    fn get_machine_env(flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        if flags.enable_pinned_reg() {
            static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
            MACHINE_ENV.get_or_init(|| create_reg_env(true))
        } else {
            static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
            MACHINE_ENV.get_or_init(|| create_reg_env(false))
        }
    }

    fn get_regs_clobbered_by_call(call_conv: isa::CallConv, is_exception: bool) -> PRegSet {
        match call_conv {
            isa::CallConv::Winch => WINCH_CLOBBERS,
            isa::CallConv::Tail if is_exception => ALL_CLOBBERS,
            _ => DEFAULT_AAPCS_CLOBBERS,
        }
    }

    fn get_ext_mode(
        call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        if call_conv == isa::CallConv::AppleAarch64 {
            specified
        } else {
            ir::ArgumentExtension::None
        }
    }

    fn compute_frame_layout(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        sig: &Signature,
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
            .filter(|r| {
                is_reg_saved_in_prologue(call_conv, flags.enable_pinned_reg(), sig, r.to_reg())
            })
            .collect();

        // Sort registers for deterministic code output. We can do an unstable
        // sort because the registers will be unique (there are no dups).
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

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        // Use x9 as a temp if needed: clobbered, not a
        // retval.
        regs::writable_xreg(9)
    }

    fn exception_payload_regs(call_conv: isa::CallConv) -> &'static [Reg] {
        const PAYLOAD_REGS: &'static [Reg] = &[regs::xreg(0), regs::xreg(1)];
        match call_conv {
            isa::CallConv::SystemV | isa::CallConv::Tail => PAYLOAD_REGS,
            _ => &[],
        }
    }
}

impl AArch64MachineDeps {
    fn gen_probestack_unroll(insts: &mut SmallInstVec<Inst>, guard_size: u32, probe_count: u32) {
        // When manually unrolling adjust the stack pointer and then write a zero
        // to the stack at that offset. This generates something like
        // `sub sp, sp, #1, lsl #12` followed by `stur wzr, [sp]`.
        //
        // We do this because valgrind expects us to never write beyond the stack
        // pointer and associated redzone.
        // See: https://github.com/bytecodealliance/wasmtime/issues/7454
        for _ in 0..probe_count {
            insts.extend(Self::gen_sp_reg_adjust(-(guard_size as i32)));

            insts.push(Inst::gen_store(
                AMode::SPOffset { off: 0 },
                zero_reg(),
                I32,
                MemFlags::trusted(),
            ));
        }

        // Restore the stack pointer to its original value
        insts.extend(Self::gen_sp_reg_adjust((guard_size * probe_count) as i32));
    }

    fn gen_probestack_loop(insts: &mut SmallInstVec<Inst>, frame_size: u32, guard_size: u32) {
        // The non-unrolled version uses two temporary registers. The
        // `start` contains the current offset from sp and counts downwards
        // during the loop by increments of `guard_size`. The `end` is
        // the size of the frame and where we stop.
        //
        // Note that this emission is all post-regalloc so it should be ok
        // to use the temporary registers here as input/output as the loop
        // itself is not allowed to use the registers.
        let start = writable_spilltmp_reg();
        let end = writable_tmp2_reg();
        // `gen_inline_probestack` is called after regalloc2, so it's acceptable to reuse
        // `start` and `end` as temporaries in load_constant.
        insts.extend(Inst::load_constant(start, 0));
        insts.extend(Inst::load_constant(end, frame_size.into()));
        insts.push(Inst::StackProbeLoop {
            start,
            end: end.to_reg(),
            step: Imm12::maybe_from_u64(guard_size.into()).unwrap(),
        });
    }

    pub fn select_api_key(
        isa_flags: &aarch64_settings::Flags,
        call_conv: isa::CallConv,
        setup_frame: bool,
    ) -> Option<APIKey> {
        if isa_flags.sign_return_address() && (setup_frame || isa_flags.sign_return_address_all()) {
            // The `tail` calling convention uses a zero modifier rather than SP
            // because tail calls may happen with a different stack pointer than
            // when the function was entered, meaning that it won't be the same when
            // the return address is decrypted.
            Some(if isa_flags.sign_return_address_with_bkey() {
                match call_conv {
                    isa::CallConv::Tail => APIKey::BZ,
                    _ => APIKey::BSP,
                }
            } else {
                match call_conv {
                    isa::CallConv::Tail => APIKey::AZ,
                    _ => APIKey::ASP,
                }
            })
        } else {
            None
        }
    }
}

/// Is the given register saved in the prologue if clobbered, i.e., is it a
/// callee-save?
fn is_reg_saved_in_prologue(
    _call_conv: isa::CallConv,
    enable_pinned_reg: bool,
    sig: &Signature,
    r: RealReg,
) -> bool {
    // FIXME: We need to inspect whether a function is returning Z or P regs too.
    let save_z_regs = sig
        .params
        .iter()
        .filter(|p| p.value_type.is_dynamic_vector())
        .count()
        != 0;

    match r.class() {
        RegClass::Int => {
            // x19 - x28 inclusive are callee-saves.
            // However, x21 is the pinned reg if `enable_pinned_reg`
            // is set, and is implicitly globally-allocated, hence not
            // callee-saved in prologues.
            if enable_pinned_reg && r.hw_enc() == PINNED_REG {
                false
            } else {
                r.hw_enc() >= 19 && r.hw_enc() <= 28
            }
        }
        RegClass::Float => {
            // If a subroutine takes at least one argument in scalable vector registers
            // or scalable predicate registers, or if it is a function that returns
            // results in such registers, it must ensure that the entire contents of
            // z8-z23 are preserved across the call. In other cases it need only
            // preserve the low 64 bits of z8-z15.
            if save_z_regs {
                r.hw_enc() >= 8 && r.hw_enc() <= 23
            } else {
                // v8 - v15 inclusive are callee-saves.
                r.hw_enc() >= 8 && r.hw_enc() <= 15
            }
        }
        RegClass::Vector => unreachable!(),
    }
}

const fn default_aapcs_clobbers() -> PRegSet {
    PRegSet::empty()
        // x0 - x17 inclusive are caller-saves.
        .with(xreg_preg(0))
        .with(xreg_preg(1))
        .with(xreg_preg(2))
        .with(xreg_preg(3))
        .with(xreg_preg(4))
        .with(xreg_preg(5))
        .with(xreg_preg(6))
        .with(xreg_preg(7))
        .with(xreg_preg(8))
        .with(xreg_preg(9))
        .with(xreg_preg(10))
        .with(xreg_preg(11))
        .with(xreg_preg(12))
        .with(xreg_preg(13))
        .with(xreg_preg(14))
        .with(xreg_preg(15))
        .with(xreg_preg(16))
        .with(xreg_preg(17))
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
        .with(vreg_preg(0))
        .with(vreg_preg(1))
        .with(vreg_preg(2))
        .with(vreg_preg(3))
        .with(vreg_preg(4))
        .with(vreg_preg(5))
        .with(vreg_preg(6))
        .with(vreg_preg(7))
        .with(vreg_preg(8))
        .with(vreg_preg(9))
        .with(vreg_preg(10))
        .with(vreg_preg(11))
        .with(vreg_preg(12))
        .with(vreg_preg(13))
        .with(vreg_preg(14))
        .with(vreg_preg(15))
        .with(vreg_preg(16))
        .with(vreg_preg(17))
        .with(vreg_preg(18))
        .with(vreg_preg(19))
        .with(vreg_preg(20))
        .with(vreg_preg(21))
        .with(vreg_preg(22))
        .with(vreg_preg(23))
        .with(vreg_preg(24))
        .with(vreg_preg(25))
        .with(vreg_preg(26))
        .with(vreg_preg(27))
        .with(vreg_preg(28))
        .with(vreg_preg(29))
        .with(vreg_preg(30))
        .with(vreg_preg(31))
}

const fn winch_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(xreg_preg(0))
        .with(xreg_preg(1))
        .with(xreg_preg(2))
        .with(xreg_preg(3))
        .with(xreg_preg(4))
        .with(xreg_preg(5))
        .with(xreg_preg(6))
        .with(xreg_preg(7))
        .with(xreg_preg(8))
        .with(xreg_preg(9))
        .with(xreg_preg(10))
        .with(xreg_preg(11))
        .with(xreg_preg(12))
        .with(xreg_preg(13))
        .with(xreg_preg(14))
        .with(xreg_preg(15))
        .with(xreg_preg(16))
        .with(xreg_preg(17))
        // x18 is used to carry platform state and is not allocatable by Winch.
        //
        // x19 - x27 are considered caller-saved in Winch's calling convention.
        .with(xreg_preg(19))
        .with(xreg_preg(20))
        .with(xreg_preg(21))
        .with(xreg_preg(22))
        .with(xreg_preg(23))
        .with(xreg_preg(24))
        .with(xreg_preg(25))
        .with(xreg_preg(26))
        .with(xreg_preg(27))
        // x28 is used as the shadow stack pointer and is considered
        // callee-saved.
        //
        // All vregs are considered caller-saved.
        .with(vreg_preg(0))
        .with(vreg_preg(1))
        .with(vreg_preg(2))
        .with(vreg_preg(3))
        .with(vreg_preg(4))
        .with(vreg_preg(5))
        .with(vreg_preg(6))
        .with(vreg_preg(7))
        .with(vreg_preg(8))
        .with(vreg_preg(9))
        .with(vreg_preg(10))
        .with(vreg_preg(11))
        .with(vreg_preg(12))
        .with(vreg_preg(13))
        .with(vreg_preg(14))
        .with(vreg_preg(15))
        .with(vreg_preg(16))
        .with(vreg_preg(17))
        .with(vreg_preg(18))
        .with(vreg_preg(19))
        .with(vreg_preg(20))
        .with(vreg_preg(21))
        .with(vreg_preg(22))
        .with(vreg_preg(23))
        .with(vreg_preg(24))
        .with(vreg_preg(25))
        .with(vreg_preg(26))
        .with(vreg_preg(27))
        .with(vreg_preg(28))
        .with(vreg_preg(29))
        .with(vreg_preg(30))
        .with(vreg_preg(31))
}

const fn all_clobbers() -> PRegSet {
    PRegSet::empty()
        // integer registers: x0 to x28 inclusive. (x29 is FP, x30 is
        // LR, x31 is SP/ZR.)
        .with(xreg_preg(0))
        .with(xreg_preg(1))
        .with(xreg_preg(2))
        .with(xreg_preg(3))
        .with(xreg_preg(4))
        .with(xreg_preg(5))
        .with(xreg_preg(6))
        .with(xreg_preg(7))
        .with(xreg_preg(8))
        .with(xreg_preg(9))
        .with(xreg_preg(10))
        .with(xreg_preg(11))
        .with(xreg_preg(12))
        .with(xreg_preg(13))
        .with(xreg_preg(14))
        .with(xreg_preg(15))
        .with(xreg_preg(16))
        .with(xreg_preg(17))
        .with(xreg_preg(18))
        .with(xreg_preg(19))
        .with(xreg_preg(20))
        .with(xreg_preg(21))
        .with(xreg_preg(22))
        .with(xreg_preg(23))
        .with(xreg_preg(24))
        .with(xreg_preg(25))
        .with(xreg_preg(26))
        .with(xreg_preg(27))
        .with(xreg_preg(28))
        // vector registers: v0 to v31 inclusive.
        .with(vreg_preg(0))
        .with(vreg_preg(1))
        .with(vreg_preg(2))
        .with(vreg_preg(3))
        .with(vreg_preg(4))
        .with(vreg_preg(5))
        .with(vreg_preg(6))
        .with(vreg_preg(7))
        .with(vreg_preg(8))
        .with(vreg_preg(9))
        .with(vreg_preg(10))
        .with(vreg_preg(11))
        .with(vreg_preg(12))
        .with(vreg_preg(13))
        .with(vreg_preg(14))
        .with(vreg_preg(15))
        .with(vreg_preg(16))
        .with(vreg_preg(17))
        .with(vreg_preg(18))
        .with(vreg_preg(19))
        .with(vreg_preg(20))
        .with(vreg_preg(21))
        .with(vreg_preg(22))
        .with(vreg_preg(23))
        .with(vreg_preg(24))
        .with(vreg_preg(25))
        .with(vreg_preg(26))
        .with(vreg_preg(27))
        .with(vreg_preg(28))
        .with(vreg_preg(29))
        .with(vreg_preg(30))
        .with(vreg_preg(31))
}

const DEFAULT_AAPCS_CLOBBERS: PRegSet = default_aapcs_clobbers();
const WINCH_CLOBBERS: PRegSet = winch_clobbers();
const ALL_CLOBBERS: PRegSet = all_clobbers();

fn create_reg_env(enable_pinned_reg: bool) -> MachineEnv {
    fn preg(r: Reg) -> PReg {
        r.to_real_reg().unwrap().into()
    }

    let mut env = MachineEnv {
        preferred_regs_by_class: [
            vec![
                preg(xreg(0)),
                preg(xreg(1)),
                preg(xreg(2)),
                preg(xreg(3)),
                preg(xreg(4)),
                preg(xreg(5)),
                preg(xreg(6)),
                preg(xreg(7)),
                preg(xreg(8)),
                preg(xreg(9)),
                preg(xreg(10)),
                preg(xreg(11)),
                preg(xreg(12)),
                preg(xreg(13)),
                preg(xreg(14)),
                preg(xreg(15)),
                // x16 and x17 are spilltmp and tmp2 (see above).
                // x18 could be used by the platform to carry inter-procedural state;
                // conservatively assume so and make it not allocatable.
                // x19-28 are callee-saved and so not preferred.
                // x21 is the pinned register (if enabled) and not allocatable if so.
                // x29 is FP, x30 is LR, x31 is SP/ZR.
            ],
            vec![
                preg(vreg(0)),
                preg(vreg(1)),
                preg(vreg(2)),
                preg(vreg(3)),
                preg(vreg(4)),
                preg(vreg(5)),
                preg(vreg(6)),
                preg(vreg(7)),
                // v8-15 are callee-saved and so not preferred.
                preg(vreg(16)),
                preg(vreg(17)),
                preg(vreg(18)),
                preg(vreg(19)),
                preg(vreg(20)),
                preg(vreg(21)),
                preg(vreg(22)),
                preg(vreg(23)),
                preg(vreg(24)),
                preg(vreg(25)),
                preg(vreg(26)),
                preg(vreg(27)),
                preg(vreg(28)),
                preg(vreg(29)),
                preg(vreg(30)),
                preg(vreg(31)),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        non_preferred_regs_by_class: [
            vec![
                preg(xreg(19)),
                preg(xreg(20)),
                // x21 is pinned reg if enabled; we add to this list below if not.
                preg(xreg(22)),
                preg(xreg(23)),
                preg(xreg(24)),
                preg(xreg(25)),
                preg(xreg(26)),
                preg(xreg(27)),
                preg(xreg(28)),
            ],
            vec![
                preg(vreg(8)),
                preg(vreg(9)),
                preg(vreg(10)),
                preg(vreg(11)),
                preg(vreg(12)),
                preg(vreg(13)),
                preg(vreg(14)),
                preg(vreg(15)),
            ],
            // Vector Regclass is unused
            vec![],
        ],
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    };

    if !enable_pinned_reg {
        debug_assert_eq!(PINNED_REG, 21); // We assumed this above in hardcoded reg list.
        env.non_preferred_regs_by_class[0].push(preg(xreg(PINNED_REG)));
    }

    env
}
