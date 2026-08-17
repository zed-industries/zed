//! Implementation of the standard x64 ABI.

use crate::CodegenResult;
use crate::ir::{self, LibCall, MemFlags, Signature, TrapCode, types};
use crate::ir::{ExternalName, types::*};
use crate::isa;
use crate::isa::winch;
use crate::isa::{CallConv, unwind::UnwindInst, x64::inst::*, x64::settings as x64_settings};
use crate::machinst::abi::*;
use crate::machinst::*;
use crate::settings;
use alloc::boxed::Box;
use alloc::vec::Vec;
use args::*;
use cranelift_assembler_x64 as asm;
use regalloc2::{MachineEnv, PReg, PRegSet};
use smallvec::{SmallVec, smallvec};
use std::borrow::ToOwned;
use std::sync::OnceLock;

/// Support for the x64 ABI from the callee side (within a function body).
pub(crate) type X64Callee = Callee<X64ABIMachineSpec>;

/// Implementation of ABI primitives for x64.
pub struct X64ABIMachineSpec;

impl X64ABIMachineSpec {
    fn gen_probestack_unroll(insts: &mut SmallInstVec<Inst>, guard_size: u32, probe_count: u32) {
        insts.reserve(probe_count as usize);
        for _ in 0..probe_count {
            // "Allocate" stack space for the probe by decrementing the stack pointer before
            // the write. This is required to make valgrind happy.
            // See: https://github.com/bytecodealliance/wasmtime/issues/7454
            insts.extend(Self::gen_sp_reg_adjust(-(guard_size as i32)));

            // TODO: It would be nice if we could store the imm 0, but we don't have insts for those
            // so store the stack pointer. Any register will do, since the stack is undefined at this point
            insts.push(Inst::store(
                I32,
                regs::rsp(),
                Amode::imm_reg(0, regs::rsp()),
            ));
        }

        // Restore the stack pointer to its original value
        insts.extend(Self::gen_sp_reg_adjust((guard_size * probe_count) as i32));
    }

    fn gen_probestack_loop(
        insts: &mut SmallInstVec<Inst>,
        _call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        // We have to use a caller-saved register since clobbering only
        // happens after stack probing.
        // `r11` is caller saved on both Fastcall and SystemV, and not used
        // for argument passing, so it's pretty much free. It is also not
        // used by the stacklimit mechanism.
        let tmp = regs::r11();
        debug_assert!({
            let real_reg = tmp.to_real_reg().unwrap();
            !is_callee_save_systemv(real_reg, false) && !is_callee_save_fastcall(real_reg, false)
        });

        insts.push(Inst::StackProbeLoop {
            tmp: Writable::from_reg(tmp),
            frame_size,
            guard_size,
        });
    }
}

impl IsaFlags for x64_settings::Flags {}

impl ABIMachineSpec for X64ABIMachineSpec {
    type I = Inst;

    type F = x64_settings::Flags;

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
        let is_fastcall = call_conv == CallConv::WindowsFastcall;

        let mut next_gpr = 0;
        let mut next_vreg = 0;
        let mut next_stack: u32 = 0;
        let mut next_param_idx = 0; // Fastcall cares about overall param index

        if args_or_rets == ArgsOrRets::Args && is_fastcall {
            // Fastcall always reserves 32 bytes of shadow space corresponding to
            // the four initial in-arg parameters.
            //
            // (See:
            // https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-170)
            next_stack = 32;
        }

        let ret_area_ptr = if add_ret_area_ptr {
            debug_assert_eq!(args_or_rets, ArgsOrRets::Args);
            next_gpr += 1;
            next_param_idx += 1;
            // In the SystemV and WindowsFastcall ABIs, the return area pointer is the first
            // argument. For the Tail and Winch ABIs we do the same for simplicity sake.
            Some(ABIArg::reg(
                get_intreg_for_arg(call_conv, 0, 0)
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

        // If any param uses extension, the winch calling convention will not pack its results
        // on the stack and will instead align them to 8-byte boundaries the same way that all the
        // other calling conventions do. This isn't consistent with Winch itself, but is fine as
        // Winch only uses this calling convention via trampolines, and those trampolines don't add
        // extension annotations. Additionally, handling extension attributes this way allows clif
        // functions that use them with the Winch calling convention to interact successfully with
        // testing infrastructure.
        // The results are also not packed if any of the types are `f16`. This is to simplify the
        // implementation of `Inst::load`/`Inst::store` (which would otherwise require multiple
        // instructions), and doesn't affect Winch itself as Winch doesn't support `f16` at all.
        let uses_extension = params.iter().any(|p| {
            p.extension != ir::ArgumentExtension::None
                || p.value_type == types::F16
                || p.value_type == types::I8X2
        });

        for (ix, param) in params.iter().enumerate() {
            let last_param = ix == params.len() - 1;

            if let ir::ArgumentPurpose::StructArgument(size) = param.purpose {
                let offset = next_stack as i64;
                let size = size;
                assert!(size % 8 == 0, "StructArgument size is not properly aligned");
                next_stack += size;
                args.push(ABIArg::StructArg {
                    offset,
                    size: size as u64,
                    purpose: param.purpose,
                });
                continue;
            }

            // Find regclass(es) of the register(s) used to store a value of this type.
            let (rcs, reg_tys) = Inst::rc_for_type(param.value_type)?;

            // Now assign ABIArgSlots for each register-sized part.
            //
            // Note that the handling of `i128` values is unique here:
            //
            // - If `enable_llvm_abi_extensions` is set in the flags, each
            //   `i128` is split into two `i64`s and assigned exactly as if it
            //   were two consecutive 64-bit args, except that if one of the
            //   two halves is forced onto the stack, the other half is too.
            //   This is consistent with LLVM's behavior, and is needed for
            //   some uses of Cranelift (e.g., the rustc backend).
            //
            // - Otherwise, both SysV and Fastcall specify behavior (use of
            //   vector register, a register pair, or passing by reference
            //   depending on the case), but for simplicity, we will just panic if
            //   an i128 type appears in a signature and the LLVM extensions flag
            //   is not set.
            //
            // For examples of how rustc compiles i128 args and return values on
            // both SysV and Fastcall platforms, see:
            // https://godbolt.org/z/PhG3ob

            if param.value_type.bits() > 64
                && !(param.value_type.is_vector() || param.value_type.is_float())
                && !flags.enable_llvm_abi_extensions()
            {
                panic!(
                    "i128 args/return values not supported unless LLVM ABI extensions are enabled"
                );
            }
            // As MSVC doesn't support f16/f128 there is no standard way to pass/return them with
            // the Windows ABI. LLVM passes/returns them in XMM registers.
            if matches!(param.value_type, types::F16 | types::F128)
                && is_fastcall
                && !flags.enable_llvm_abi_extensions()
            {
                panic!(
                    "f16/f128 args/return values not supported for windows_fastcall unless LLVM ABI extensions are enabled"
                );
            }

            // Windows fastcall dictates that `__m128i` and `f128` parameters to
            // a function are passed indirectly as pointers, so handle that as a
            // special case before the loop below.
            if (param.value_type.is_vector() || param.value_type.is_float())
                && param.value_type.bits() >= 128
                && args_or_rets == ArgsOrRets::Args
                && is_fastcall
            {
                let pointer = match get_intreg_for_arg(call_conv, next_gpr, next_param_idx) {
                    Some(reg) => {
                        next_gpr += 1;
                        ABIArgSlot::Reg {
                            reg: reg.to_real_reg().unwrap(),
                            ty: ir::types::I64,
                            extension: ir::ArgumentExtension::None,
                        }
                    }

                    None => {
                        next_stack = align_to(next_stack, 8) + 8;
                        ABIArgSlot::Stack {
                            offset: (next_stack - 8) as i64,
                            ty: ir::types::I64,
                            extension: param.extension,
                        }
                    }
                };
                next_param_idx += 1;
                args.push(ABIArg::ImplicitPtrArg {
                    // NB: this is filled in after this loop
                    offset: 0,
                    pointer,
                    ty: param.value_type,
                    purpose: param.purpose,
                });
                continue;
            }

            // SystemV dictates that 128bit int parameters are always either
            // passed in two registers or on the stack, so handle that as a
            // special case before the loop below.
            if param.value_type == types::I128
                && args_or_rets == ArgsOrRets::Args
                && call_conv == CallConv::SystemV
            {
                let mut slots = ABIArgSlotVec::new();
                match (
                    get_intreg_for_arg(CallConv::SystemV, next_gpr, next_param_idx),
                    get_intreg_for_arg(CallConv::SystemV, next_gpr + 1, next_param_idx + 1),
                ) {
                    (Some(reg1), Some(reg2)) => {
                        slots.push(ABIArgSlot::Reg {
                            reg: reg1.to_real_reg().unwrap(),
                            ty: ir::types::I64,
                            extension: ir::ArgumentExtension::None,
                        });
                        slots.push(ABIArgSlot::Reg {
                            reg: reg2.to_real_reg().unwrap(),
                            ty: ir::types::I64,
                            extension: ir::ArgumentExtension::None,
                        });
                    }
                    _ => {
                        let size = 16;

                        // Align.
                        next_stack = align_to(next_stack, size);

                        slots.push(ABIArgSlot::Stack {
                            offset: next_stack as i64,
                            ty: ir::types::I64,
                            extension: param.extension,
                        });
                        slots.push(ABIArgSlot::Stack {
                            offset: next_stack as i64 + 8,
                            ty: ir::types::I64,
                            extension: param.extension,
                        });
                        next_stack += size;
                    }
                };
                // Unconditionally increment next_gpr even when storing the
                // argument on the stack to prevent reusing a possibly
                // remaining register for the next argument.
                next_gpr += 2;
                next_param_idx += 2;

                args.push(ABIArg::Slots {
                    slots,
                    purpose: param.purpose,
                });
                continue;
            }

            let mut slots = ABIArgSlotVec::new();
            for (ix, (rc, reg_ty)) in rcs.iter().zip(reg_tys.iter()).enumerate() {
                let last_slot = last_param && ix == rcs.len() - 1;

                let intreg = *rc == RegClass::Int;
                let nextreg = if intreg {
                    match args_or_rets {
                        ArgsOrRets::Args => get_intreg_for_arg(call_conv, next_gpr, next_param_idx),
                        ArgsOrRets::Rets => {
                            get_intreg_for_retval(call_conv, flags, next_gpr, last_slot)
                        }
                    }
                } else {
                    match args_or_rets {
                        ArgsOrRets::Args => {
                            get_fltreg_for_arg(call_conv, next_vreg, next_param_idx)
                        }
                        ArgsOrRets::Rets => get_fltreg_for_retval(call_conv, next_vreg, last_slot),
                    }
                };
                next_param_idx += 1;
                if let Some(reg) = nextreg {
                    if intreg {
                        next_gpr += 1;
                    } else {
                        next_vreg += 1;
                    }
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

                    let size = reg_ty.bytes();
                    let size = if call_conv == CallConv::Winch
                        && args_or_rets == ArgsOrRets::Rets
                        && !uses_extension
                    {
                        size
                    } else {
                        let size = std::cmp::max(size, 8);

                        // Align.
                        debug_assert!(size.is_power_of_two());
                        next_stack = align_to(next_stack, size);
                        size
                    };

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

        // Fastcall's indirect 128+ bit vector arguments are all located on the
        // stack, and stack space is reserved after all parameters are passed,
        // so allocate from the space now.
        if args_or_rets == ArgsOrRets::Args && is_fastcall {
            for arg in args.args_mut() {
                if let ABIArg::ImplicitPtrArg { offset, .. } = arg {
                    assert_eq!(*offset, 0);
                    next_stack = align_to(next_stack, 16);
                    *offset = next_stack as i64;
                    next_stack += 16;
                }
            }
        }
        let extra_arg_idx = if let Some(ret_area_ptr) = ret_area_ptr {
            args.push_non_formal(ret_area_ptr);
            Some(args.args().len() - 1)
        } else {
            None
        };

        // Winch writes the first result to the highest offset, so we need to iterate through the
        // args and adjust the offsets down.
        if call_conv == CallConv::Winch && args_or_rets == ArgsOrRets::Rets {
            winch::reverse_stack(args, next_stack, uses_extension);
        }

        next_stack = align_to(next_stack, 16);

        Ok((next_stack, extra_arg_idx))
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Self::I {
        // For integer-typed values, we always load a full 64 bits (and we always spill a full 64
        // bits as well -- see `Inst::store()`).
        let ty = match ty {
            types::I8 | types::I16 | types::I32 => types::I64,
            // Stack slots are always at least 8 bytes, so it's fine to load 4 bytes instead of only
            // two.
            types::F16 | types::I8X2 => types::F32,
            _ => ty,
        };
        Inst::load(ty, mem, into_reg, ExtKind::None)
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Self::I {
        let ty = match ty {
            // See `gen_load_stack`.
            types::F16 | types::I8X2 => types::F32,
            _ => ty,
        };
        Inst::store(ty, from_reg, mem)
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Self::I {
        Inst::gen_move(to_reg, from_reg, ty)
    }

    /// Generate an integer-extend operation.
    fn gen_extend(
        to_reg: Writable<Reg>,
        from_reg: Reg,
        is_signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Self::I {
        let ext_mode = ExtMode::new(from_bits as u16, to_bits as u16)
            .unwrap_or_else(|| panic!("invalid extension: {from_bits} -> {to_bits}"));
        if is_signed {
            Inst::movsx_rm_r(ext_mode, RegMem::reg(from_reg), to_reg)
        } else {
            Inst::movzx_rm_r(ext_mode, RegMem::reg(from_reg), to_reg)
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
    ) -> SmallInstVec<Self::I> {
        let mut ret = SmallVec::new();
        if from_reg != into_reg.to_reg() {
            ret.push(Inst::gen_move(into_reg, from_reg, I64));
        }
        let imm = i32::try_from(imm).expect("`imm` is too large to fit in a 32-bit immediate");
        ret.push(Inst::addq_mi(into_reg, imm));
        ret
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Self::I> {
        smallvec![
            Inst::External {
                inst: asm::inst::cmpq_rm::new(Gpr::unwrap_new(limit_reg), Gpr::RSP,).into(),
            },
            Inst::TrapIf {
                // NBE == "> unsigned"; args above are reversed; this tests limit_reg > rsp.
                cc: CC::NBE,
                trap_code: TrapCode::STACK_OVERFLOW,
            },
        ]
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Self::I {
        let mem: SyntheticAmode = mem.into();
        Inst::External {
            inst: asm::inst::leaq_rm::new(into_reg, mem).into(),
        }
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        // As per comment on trait definition, we must return a caller-save
        // register that is not used as an argument here.
        debug_assert!(!is_callee_save_systemv(
            regs::r10().to_real_reg().unwrap(),
            false
        ));
        regs::r10()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Self::I {
        // Only ever used for I64s, F128s and vectors; if that changes, see if
        // the ExtKind below needs to be changed.
        assert!(ty == I64 || ty.is_vector() || ty == F128);
        let mem = Amode::imm_reg(offset, base);
        Inst::load(ty, mem, into_reg, ExtKind::None)
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Self::I {
        let ty = match ty {
            // See `gen_load_stack`.
            types::F16 | types::I8X2 => types::F32,
            _ => ty,
        };
        let mem = Amode::imm_reg(offset, base);
        Inst::store(ty, from_reg, mem)
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Self::I> {
        let rsp = Writable::from_reg(regs::rsp());
        let inst = if amount >= 0 {
            Inst::addq_mi(rsp, amount)
        } else {
            Inst::subq_mi(rsp, -amount)
        };
        smallvec![inst]
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        _isa_flags: &x64_settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        let r_rsp = Gpr::RSP;
        let r_rbp = Gpr::RBP;
        let w_rbp = Writable::from_reg(r_rbp);
        let mut insts = SmallVec::new();
        // `push %rbp`
        // RSP before the call will be 0 % 16.  So here, it is 8 % 16.
        insts.push(Inst::External {
            inst: asm::inst::pushq_o::new(r_rbp).into(),
        });

        if flags.unwind_info() {
            insts.push(Inst::Unwind {
                inst: UnwindInst::PushFrameRegs {
                    offset_upward_to_caller_sp: frame_layout.setup_area_size,
                },
            });
        }

        // `mov %rsp, %rbp`
        // RSP is now 0 % 16
        insts.push(Inst::External {
            inst: asm::inst::movq_mr::new(w_rbp, r_rsp).into(),
        });

        insts
    }

    fn gen_epilogue_frame_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &x64_settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        let rbp = Gpr::RBP;
        let rsp = Gpr::RSP;

        let mut insts = SmallVec::new();
        // `mov %rbp, %rsp`
        insts.push(Inst::External {
            inst: asm::inst::movq_mr::new(Writable::from_reg(rsp), rbp).into(),
        });
        // `pop %rbp`
        insts.push(Inst::External {
            inst: asm::inst::popq_o::new(Writable::from_reg(rbp)).into(),
        });
        insts
    }

    fn gen_return(
        call_conv: CallConv,
        _isa_flags: &x64_settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        // Emit return instruction.
        let stack_bytes_to_pop = if call_conv == CallConv::Tail {
            frame_layout.tail_args_size
        } else {
            0
        };
        let inst = if stack_bytes_to_pop == 0 {
            asm::inst::retq_zo::new().into()
        } else {
            let stack_bytes_to_pop = u16::try_from(stack_bytes_to_pop).unwrap();
            asm::inst::retq_i::new(stack_bytes_to_pop).into()
        };
        smallvec![Inst::External { inst }]
    }

    fn gen_probestack(insts: &mut SmallInstVec<Self::I>, frame_size: u32) {
        insts.push(Inst::imm(
            OperandSize::Size32,
            frame_size as u64,
            Writable::from_reg(regs::rax()),
        ));
        insts.push(Inst::CallKnown {
            // No need to include arg here: we are post-regalloc
            // so no constraints will be seen anyway.
            info: Box::new(CallInfo::empty(
                ExternalName::LibCall(LibCall::Probestack),
                CallConv::Probestack,
            )),
        });
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        // Unroll at most n consecutive probes, before falling back to using a loop
        //
        // This was number was picked because the loop version is 38 bytes long. We can fit
        // 4 inline probes in that space, so unroll if its beneficial in terms of code size.
        const PROBE_MAX_UNROLL: u32 = 4;

        // Calculate how many probes we need to perform. Round down, as we only
        // need to probe whole guard_size regions we'd otherwise skip over.
        let probe_count = frame_size / guard_size;
        if probe_count == 0 {
            // No probe necessary
        } else if probe_count <= PROBE_MAX_UNROLL {
            Self::gen_probestack_unroll(insts, guard_size, probe_count)
        } else {
            Self::gen_probestack_loop(insts, call_conv, frame_size, guard_size)
        }
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        let mut insts = SmallVec::new();

        // When a return_call within this function required more stack arguments than we have
        // present, resize the incoming argument area of the frame to accommodate those arguments.
        let incoming_args_diff = frame_layout.tail_args_size - frame_layout.incoming_args_size;
        if incoming_args_diff > 0 {
            // Decrement the stack pointer to make space for the new arguments.
            let rsp = Writable::from_reg(regs::rsp());
            insts.push(Inst::subq_mi(
                rsp,
                i32::try_from(incoming_args_diff)
                    .expect("`incoming_args_diff` is too large to fit in a 32-bit immediate"),
            ));

            // Make sure to keep the frame pointer and stack pointer in sync at
            // this point.
            let rbp = Gpr::RBP;
            let rsp = Gpr::RSP;
            insts.push(Inst::External {
                inst: asm::inst::movq_mr::new(Writable::from_reg(rbp), rsp).into(),
            });

            let incoming_args_diff = i32::try_from(incoming_args_diff).unwrap();

            // Move the saved frame pointer down by `incoming_args_diff`.
            let addr = Amode::imm_reg(incoming_args_diff, regs::rsp());
            let r11 = Writable::from_reg(Gpr::R11);
            let inst = asm::inst::movq_rm::new(r11, addr).into();
            insts.push(Inst::External { inst });
            let inst = asm::inst::movq_mr::new(Amode::imm_reg(0, regs::rsp()), r11.to_reg()).into();
            insts.push(Inst::External { inst });

            // Move the saved return address down by `incoming_args_diff`.
            let addr = Amode::imm_reg(incoming_args_diff + 8, regs::rsp());
            let inst = asm::inst::movq_rm::new(r11, addr).into();
            insts.push(Inst::External { inst });
            let inst = asm::inst::movq_mr::new(Amode::imm_reg(8, regs::rsp()), r11.to_reg()).into();
            insts.push(Inst::External { inst });
        }

        // We need to factor `incoming_args_diff` into the offset upward here, as we have grown
        // the argument area -- `setup_area_size` alone will not be the correct offset up to the
        // original caller's SP.
        let offset_upward_to_caller_sp = frame_layout.setup_area_size + incoming_args_diff;
        if flags.unwind_info() && offset_upward_to_caller_sp > 0 {
            // Emit unwind info: start the frame. The frame (from unwind
            // consumers' point of view) starts at clobbbers, just below
            // the FP and return address. Spill slots and stack slots are
            // part of our actual frame but do not concern the unwinder.
            insts.push(Inst::Unwind {
                inst: UnwindInst::DefineNewFrame {
                    offset_downward_to_clobbers: frame_layout.clobber_size,
                    offset_upward_to_caller_sp,
                },
            });
        }

        // Adjust the stack pointer downward for clobbers and the function fixed
        // frame (spillslots, storage slots, and argument area).
        let stack_size = frame_layout.fixed_frame_storage_size
            + frame_layout.clobber_size
            + frame_layout.outgoing_args_size;
        if stack_size > 0 {
            let rsp = Writable::from_reg(regs::rsp());
            let stack_size = i32::try_from(stack_size)
                .expect("`stack_size` is too large to fit in a 32-bit immediate");
            insts.push(Inst::subq_mi(rsp, stack_size));
        }

        // Store each clobbered register in order at offsets from RSP,
        // placing them above the fixed frame slots.
        let clobber_offset =
            frame_layout.fixed_frame_storage_size + frame_layout.outgoing_args_size;
        let mut cur_offset = 0;
        for reg in &frame_layout.clobbered_callee_saves {
            let r_reg = reg.to_reg();
            let ty = match r_reg.class() {
                RegClass::Int => types::I64,
                RegClass::Float => types::I8X16,
                RegClass::Vector => unreachable!(),
            };

            // Align to 8 or 16 bytes as required by the storage type of the clobber.
            cur_offset = align_to(cur_offset, ty.bytes());
            let off = cur_offset;
            cur_offset += ty.bytes();

            insts.push(Inst::store(
                ty,
                r_reg.into(),
                Amode::imm_reg(i32::try_from(off + clobber_offset).unwrap(), regs::rsp()),
            ));

            if flags.unwind_info() {
                insts.push(Inst::Unwind {
                    inst: UnwindInst::SaveReg {
                        clobber_offset: off,
                        reg: r_reg,
                    },
                });
            }
        }

        insts
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        let mut insts = SmallVec::new();

        // Restore regs by loading from offsets of RSP. We compute the offset from
        // the same base as above in clobber_save, as RSP won't change between the
        // prologue and epilogue.
        let mut cur_offset =
            frame_layout.fixed_frame_storage_size + frame_layout.outgoing_args_size;
        for reg in &frame_layout.clobbered_callee_saves {
            let rreg = reg.to_reg();
            let ty = match rreg.class() {
                RegClass::Int => types::I64,
                RegClass::Float => types::I8X16,
                RegClass::Vector => unreachable!(),
            };

            // Align to 8 or 16 bytes as required by the storage type of the clobber.
            cur_offset = align_to(cur_offset, ty.bytes());

            insts.push(Inst::load(
                ty,
                Amode::imm_reg(cur_offset.try_into().unwrap(), regs::rsp()),
                Writable::from_reg(rreg.into()),
                ExtKind::None,
            ));

            cur_offset += ty.bytes();
        }

        let stack_size = frame_layout.fixed_frame_storage_size
            + frame_layout.clobber_size
            + frame_layout.outgoing_args_size;

        // Adjust RSP back upward.
        if stack_size > 0 {
            let rsp = Writable::from_reg(regs::rsp());
            let stack_size = i32::try_from(stack_size)
                .expect("`stack_size` is too large to fit in a 32-bit immediate");
            insts.push(Inst::addq_mi(rsp, stack_size));
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
        let arg0 = get_intreg_for_arg(call_conv, 0, 0).unwrap();
        let arg1 = get_intreg_for_arg(call_conv, 1, 1).unwrap();
        let arg2 = get_intreg_for_arg(call_conv, 2, 2).unwrap();
        let temp = alloc_tmp(Self::word_type());
        let temp2 = alloc_tmp(Self::word_type());
        insts.push(Inst::imm(OperandSize::Size64, size as u64, temp));
        // We use an indirect call and a full LoadExtName because we do not have
        // information about the libcall `RelocDistance` here, so we
        // conservatively use the more flexible calling sequence.
        insts.push(Inst::LoadExtName {
            dst: temp2.map(Gpr::unwrap_new),
            name: Box::new(ExternalName::LibCall(LibCall::Memcpy)),
            offset: 0,
            distance: RelocDistance::Far,
        });
        let callee_pop_size = 0;
        insts.push(Inst::call_unknown(Box::new(CallInfo {
            dest: RegMem::reg(temp2.to_reg()),
            uses: smallvec![
                CallArgPair {
                    vreg: dst,
                    preg: arg0
                },
                CallArgPair {
                    vreg: src,
                    preg: arg1
                },
                CallArgPair {
                    vreg: temp.to_reg(),
                    preg: arg2
                },
            ],
            defs: smallvec![],
            clobbers: Self::get_regs_clobbered_by_call(call_conv, false),
            callee_pop_size,
            callee_conv: call_conv,
            caller_conv: call_conv,
            try_call_info: None,
        })));
        insts
    }

    fn get_number_of_spillslots_for_value(
        rc: RegClass,
        vector_scale: u32,
        _isa_flags: &Self::F,
    ) -> u32 {
        // We allocate in terms of 8-byte slots.
        match rc {
            RegClass::Int => 1,
            RegClass::Float => vector_scale / 8,
            RegClass::Vector => unreachable!(),
        }
    }

    fn get_machine_env(flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        if flags.enable_pinned_reg() {
            static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
            MACHINE_ENV.get_or_init(|| create_reg_env_systemv(true))
        } else {
            static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
            MACHINE_ENV.get_or_init(|| create_reg_env_systemv(false))
        }
    }

    fn get_regs_clobbered_by_call(
        call_conv_of_callee: isa::CallConv,
        is_exception: bool,
    ) -> PRegSet {
        match call_conv_of_callee {
            CallConv::Winch => ALL_CLOBBERS,
            CallConv::WindowsFastcall => WINDOWS_CLOBBERS,
            CallConv::Tail if is_exception => ALL_CLOBBERS,
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
        call_conv: CallConv,
        flags: &settings::Flags,
        _sig: &Signature,
        regs: &[Writable<RealReg>],
        _is_leaf: bool,
        incoming_args_size: u32,
        tail_args_size: u32,
        stackslots_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> FrameLayout {
        debug_assert!(tail_args_size >= incoming_args_size);

        let mut regs: Vec<Writable<RealReg>> = match call_conv {
            // The `winch` calling convention doesn't have any callee-save
            // registers.
            CallConv::Winch => vec![],
            CallConv::Fast | CallConv::Cold | CallConv::SystemV | CallConv::Tail => regs
                .iter()
                .cloned()
                .filter(|r| is_callee_save_systemv(r.to_reg(), flags.enable_pinned_reg()))
                .collect(),
            CallConv::WindowsFastcall => regs
                .iter()
                .cloned()
                .filter(|r| is_callee_save_fastcall(r.to_reg(), flags.enable_pinned_reg()))
                .collect(),
            CallConv::Probestack => todo!("probestack?"),
            CallConv::AppleAarch64 => unreachable!(),
        };
        // Sort registers for deterministic code output. We can do an unstable sort because the
        // registers will be unique (there are no dups).
        regs.sort_unstable();

        // Compute clobber size.
        let clobber_size = compute_clobber_size(&regs);

        // Compute setup area size.
        let setup_area_size = 16; // RBP, return address

        // Return FrameLayout structure.
        FrameLayout {
            word_bytes: 8,
            incoming_args_size,
            tail_args_size: align_to(tail_args_size, 16),
            setup_area_size,
            clobber_size,
            fixed_frame_storage_size,
            stackslots_size,
            outgoing_args_size,
            clobbered_callee_saves: regs,
        }
    }

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        // Use r11 as a temp: clobbered anyway, and
        // not otherwise used as a return value in any of our
        // supported calling conventions.
        Writable::from_reg(regs::r11())
    }

    fn exception_payload_regs(call_conv: isa::CallConv) -> &'static [Reg] {
        const PAYLOAD_REGS: &'static [Reg] = &[regs::rax(), regs::rdx()];
        match call_conv {
            isa::CallConv::SystemV | isa::CallConv::Tail => PAYLOAD_REGS,
            _ => &[],
        }
    }
}

impl From<StackAMode> for SyntheticAmode {
    fn from(amode: StackAMode) -> Self {
        // We enforce a 128 MB stack-frame size limit above, so these
        // `expect()`s should never fail.
        match amode {
            StackAMode::IncomingArg(off, stack_args_size) => {
                let offset = u32::try_from(off).expect(
                    "Offset in IncomingArg is greater than 4GB; should hit impl limit first",
                );
                SyntheticAmode::IncomingArg {
                    offset: stack_args_size - offset,
                }
            }
            StackAMode::Slot(off) => {
                let off = i32::try_from(off)
                    .expect("Offset in Slot is greater than 2GB; should hit impl limit first");
                SyntheticAmode::slot_offset(off)
            }
            StackAMode::OutgoingArg(off) => {
                let off = i32::try_from(off).expect(
                    "Offset in OutgoingArg is greater than 2GB; should hit impl limit first",
                );
                SyntheticAmode::Real(Amode::ImmReg {
                    simm32: off,
                    base: regs::rsp(),
                    flags: MemFlags::trusted(),
                })
            }
        }
    }
}

fn get_intreg_for_arg(call_conv: CallConv, idx: usize, arg_idx: usize) -> Option<Reg> {
    let is_fastcall = call_conv == CallConv::WindowsFastcall;

    // Fastcall counts by absolute argument number; SysV counts by argument of
    // this (integer) class.
    let i = if is_fastcall { arg_idx } else { idx };
    match (i, is_fastcall) {
        (0, false) => Some(regs::rdi()),
        (1, false) => Some(regs::rsi()),
        (2, false) => Some(regs::rdx()),
        (3, false) => Some(regs::rcx()),
        (4, false) => Some(regs::r8()),
        (5, false) => Some(regs::r9()),
        (0, true) => Some(regs::rcx()),
        (1, true) => Some(regs::rdx()),
        (2, true) => Some(regs::r8()),
        (3, true) => Some(regs::r9()),
        _ => None,
    }
}

fn get_fltreg_for_arg(call_conv: CallConv, idx: usize, arg_idx: usize) -> Option<Reg> {
    let is_fastcall = call_conv == CallConv::WindowsFastcall;

    // Fastcall counts by absolute argument number; SysV counts by argument of
    // this (floating-point) class.
    let i = if is_fastcall { arg_idx } else { idx };
    match (i, is_fastcall) {
        (0, false) => Some(regs::xmm0()),
        (1, false) => Some(regs::xmm1()),
        (2, false) => Some(regs::xmm2()),
        (3, false) => Some(regs::xmm3()),
        (4, false) => Some(regs::xmm4()),
        (5, false) => Some(regs::xmm5()),
        (6, false) => Some(regs::xmm6()),
        (7, false) => Some(regs::xmm7()),
        (0, true) => Some(regs::xmm0()),
        (1, true) => Some(regs::xmm1()),
        (2, true) => Some(regs::xmm2()),
        (3, true) => Some(regs::xmm3()),
        _ => None,
    }
}

fn get_intreg_for_retval(
    call_conv: CallConv,
    flags: &settings::Flags,
    intreg_idx: usize,
    is_last: bool,
) -> Option<Reg> {
    match call_conv {
        CallConv::Tail => match intreg_idx {
            0 => Some(regs::rax()),
            1 => Some(regs::rcx()),
            2 => Some(regs::rdx()),
            3 => Some(regs::rsi()),
            4 => Some(regs::rdi()),
            5 => Some(regs::r8()),
            6 => Some(regs::r9()),
            7 => Some(regs::r10()),
            // NB: `r11` is reserved as a scratch register that is
            // also part of the clobber set.
            // NB: `r15` is reserved as a scratch register.
            _ => None,
        },
        CallConv::Fast | CallConv::Cold | CallConv::SystemV => match intreg_idx {
            0 => Some(regs::rax()),
            1 => Some(regs::rdx()),
            2 if flags.enable_llvm_abi_extensions() => Some(regs::rcx()),
            _ => None,
        },
        CallConv::WindowsFastcall => match intreg_idx {
            0 => Some(regs::rax()),
            1 => Some(regs::rdx()), // The Rust ABI for i128s needs this.
            _ => None,
        },

        CallConv::Winch => is_last.then(|| regs::rax()),
        CallConv::Probestack => todo!(),
        CallConv::AppleAarch64 => unreachable!(),
    }
}

fn get_fltreg_for_retval(call_conv: CallConv, fltreg_idx: usize, is_last: bool) -> Option<Reg> {
    match call_conv {
        CallConv::Tail => match fltreg_idx {
            0 => Some(regs::xmm0()),
            1 => Some(regs::xmm1()),
            2 => Some(regs::xmm2()),
            3 => Some(regs::xmm3()),
            4 => Some(regs::xmm4()),
            5 => Some(regs::xmm5()),
            6 => Some(regs::xmm6()),
            7 => Some(regs::xmm7()),
            _ => None,
        },
        CallConv::Fast | CallConv::Cold | CallConv::SystemV => match fltreg_idx {
            0 => Some(regs::xmm0()),
            1 => Some(regs::xmm1()),
            _ => None,
        },
        CallConv::WindowsFastcall => match fltreg_idx {
            0 => Some(regs::xmm0()),
            _ => None,
        },
        CallConv::Winch => is_last.then(|| regs::xmm0()),
        CallConv::Probestack => todo!(),
        CallConv::AppleAarch64 => unreachable!(),
    }
}

fn is_callee_save_systemv(r: RealReg, enable_pinned_reg: bool) -> bool {
    use regs::*;
    match r.class() {
        RegClass::Int => match r.hw_enc() {
            ENC_RBX | ENC_RBP | ENC_R12 | ENC_R13 | ENC_R14 => true,
            // R15 is the pinned register; if we're using it that way,
            // it is effectively globally-allocated, and is not
            // callee-saved.
            ENC_R15 => !enable_pinned_reg,
            _ => false,
        },
        RegClass::Float => false,
        RegClass::Vector => unreachable!(),
    }
}

fn is_callee_save_fastcall(r: RealReg, enable_pinned_reg: bool) -> bool {
    use regs::*;
    match r.class() {
        RegClass::Int => match r.hw_enc() {
            ENC_RBX | ENC_RBP | ENC_RSI | ENC_RDI | ENC_R12 | ENC_R13 | ENC_R14 => true,
            // See above for SysV: we must treat the pinned reg specially.
            ENC_R15 => !enable_pinned_reg,
            _ => false,
        },
        RegClass::Float => match r.hw_enc() {
            6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => true,
            _ => false,
        },
        RegClass::Vector => unreachable!(),
    }
}

fn compute_clobber_size(clobbers: &[Writable<RealReg>]) -> u32 {
    let mut clobbered_size = 0;
    for reg in clobbers {
        match reg.to_reg().class() {
            RegClass::Int => {
                clobbered_size += 8;
            }
            RegClass::Float => {
                clobbered_size = align_to(clobbered_size, 16);
                clobbered_size += 16;
            }
            RegClass::Vector => unreachable!(),
        }
    }
    align_to(clobbered_size, 16)
}

const WINDOWS_CLOBBERS: PRegSet = windows_clobbers();
const SYSV_CLOBBERS: PRegSet = sysv_clobbers();
pub(crate) const ALL_CLOBBERS: PRegSet = all_clobbers();

const fn windows_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(regs::gpr_preg(regs::ENC_RAX))
        .with(regs::gpr_preg(regs::ENC_RCX))
        .with(regs::gpr_preg(regs::ENC_RDX))
        .with(regs::gpr_preg(regs::ENC_R8))
        .with(regs::gpr_preg(regs::ENC_R9))
        .with(regs::gpr_preg(regs::ENC_R10))
        .with(regs::gpr_preg(regs::ENC_R11))
        .with(regs::fpr_preg(0))
        .with(regs::fpr_preg(1))
        .with(regs::fpr_preg(2))
        .with(regs::fpr_preg(3))
        .with(regs::fpr_preg(4))
        .with(regs::fpr_preg(5))
}

const fn sysv_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(regs::gpr_preg(regs::ENC_RAX))
        .with(regs::gpr_preg(regs::ENC_RCX))
        .with(regs::gpr_preg(regs::ENC_RDX))
        .with(regs::gpr_preg(regs::ENC_RSI))
        .with(regs::gpr_preg(regs::ENC_RDI))
        .with(regs::gpr_preg(regs::ENC_R8))
        .with(regs::gpr_preg(regs::ENC_R9))
        .with(regs::gpr_preg(regs::ENC_R10))
        .with(regs::gpr_preg(regs::ENC_R11))
        .with(regs::fpr_preg(0))
        .with(regs::fpr_preg(1))
        .with(regs::fpr_preg(2))
        .with(regs::fpr_preg(3))
        .with(regs::fpr_preg(4))
        .with(regs::fpr_preg(5))
        .with(regs::fpr_preg(6))
        .with(regs::fpr_preg(7))
        .with(regs::fpr_preg(8))
        .with(regs::fpr_preg(9))
        .with(regs::fpr_preg(10))
        .with(regs::fpr_preg(11))
        .with(regs::fpr_preg(12))
        .with(regs::fpr_preg(13))
        .with(regs::fpr_preg(14))
        .with(regs::fpr_preg(15))
}

/// For calling conventions that clobber all registers.
const fn all_clobbers() -> PRegSet {
    PRegSet::empty()
        .with(regs::gpr_preg(regs::ENC_RAX))
        .with(regs::gpr_preg(regs::ENC_RCX))
        .with(regs::gpr_preg(regs::ENC_RDX))
        .with(regs::gpr_preg(regs::ENC_RBX))
        .with(regs::gpr_preg(regs::ENC_RSI))
        .with(regs::gpr_preg(regs::ENC_RDI))
        .with(regs::gpr_preg(regs::ENC_R8))
        .with(regs::gpr_preg(regs::ENC_R9))
        .with(regs::gpr_preg(regs::ENC_R10))
        .with(regs::gpr_preg(regs::ENC_R11))
        .with(regs::gpr_preg(regs::ENC_R12))
        .with(regs::gpr_preg(regs::ENC_R13))
        .with(regs::gpr_preg(regs::ENC_R14))
        .with(regs::gpr_preg(regs::ENC_R15))
        .with(regs::fpr_preg(0))
        .with(regs::fpr_preg(1))
        .with(regs::fpr_preg(2))
        .with(regs::fpr_preg(3))
        .with(regs::fpr_preg(4))
        .with(regs::fpr_preg(5))
        .with(regs::fpr_preg(6))
        .with(regs::fpr_preg(7))
        .with(regs::fpr_preg(8))
        .with(regs::fpr_preg(9))
        .with(regs::fpr_preg(10))
        .with(regs::fpr_preg(11))
        .with(regs::fpr_preg(12))
        .with(regs::fpr_preg(13))
        .with(regs::fpr_preg(14))
        .with(regs::fpr_preg(15))
}

fn create_reg_env_systemv(enable_pinned_reg: bool) -> MachineEnv {
    fn preg(r: Reg) -> PReg {
        r.to_real_reg().unwrap().into()
    }

    let mut env = MachineEnv {
        preferred_regs_by_class: [
            // Preferred GPRs: caller-saved in the SysV ABI.
            vec![
                preg(regs::rsi()),
                preg(regs::rdi()),
                preg(regs::rax()),
                preg(regs::rcx()),
                preg(regs::rdx()),
                preg(regs::r8()),
                preg(regs::r9()),
                preg(regs::r10()),
                preg(regs::r11()),
            ],
            // Preferred XMMs: the first 8, which can have smaller encodings
            // with AVX instructions.
            vec![
                preg(regs::xmm0()),
                preg(regs::xmm1()),
                preg(regs::xmm2()),
                preg(regs::xmm3()),
                preg(regs::xmm4()),
                preg(regs::xmm5()),
                preg(regs::xmm6()),
                preg(regs::xmm7()),
            ],
            // The Vector Regclass is unused
            vec![],
        ],
        non_preferred_regs_by_class: [
            // Non-preferred GPRs: callee-saved in the SysV ABI.
            vec![
                preg(regs::rbx()),
                preg(regs::r12()),
                preg(regs::r13()),
                preg(regs::r14()),
            ],
            // Non-preferred XMMs: the last 8 registers, which can have larger
            // encodings with AVX instructions.
            vec![
                preg(regs::xmm8()),
                preg(regs::xmm9()),
                preg(regs::xmm10()),
                preg(regs::xmm11()),
                preg(regs::xmm12()),
                preg(regs::xmm13()),
                preg(regs::xmm14()),
                preg(regs::xmm15()),
            ],
            // The Vector Regclass is unused
            vec![],
        ],
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    };

    debug_assert_eq!(regs::r15(), regs::pinned_reg());
    if !enable_pinned_reg {
        env.non_preferred_regs_by_class[0].push(preg(regs::r15()));
    }

    env
}
