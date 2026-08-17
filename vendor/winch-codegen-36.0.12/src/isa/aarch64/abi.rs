use super::regs;
use crate::RegIndexEnv;
use crate::abi::{ABI, ABIOperand, ABIParams, ABIResults, ABISig, ParamsOrReturns, align_to};
use crate::codegen::CodeGenError;
use crate::isa::{CallingConvention, reg::Reg};
use anyhow::{Result, bail};
use wasmtime_environ::{WasmHeapType, WasmValType};

#[derive(Default)]
pub(crate) struct Aarch64ABI;

/// The x28 register serves as the shadow stack pointer. For further details,
/// please refer to [`crate::isa::aarch64::regs::shadow_sp`].
///
/// This register is designated as callee-saved to prevent corruption during
/// function calls. This is especially important for Wasm-to-Wasm calls in
/// Winch-generated code, as Winch's default calling convention does not define
/// any callee-saved registers.
///
/// Note that 16 bytes are used to save the shadow stack pointer register even
/// though only 8 are needed. 16 is used for simplicity to ensure that the
/// 16-byte alignment requirement for memory addressing is met at the function's
/// prologue and epilogue.
pub const SHADOW_STACK_POINTER_SLOT_SIZE: u8 = 16;

impl ABI for Aarch64ABI {
    // TODO change to 16 once SIMD is supported
    fn stack_align() -> u8 {
        8
    }

    fn call_stack_align() -> u8 {
        16
    }

    fn arg_base_offset() -> u8 {
        // Two 8-byte slots:
        // * One for link register
        // * One for the frame pointer
        //
        // ┌──────────┬───────── Argument base
        // │   LR     │
        // │          │
        // ├──────────┼
        // │          │
        // │   FP     │
        // └──────────┴ -> 16
        16
    }

    fn initial_frame_size() -> u8 {
        // The initial frame size is composed of 4 8-byte slots:
        // * Two slots for the link register and the frame pointer. See
        //   [`Self::arg_base_offset`]
        // * Two for the shadow stack pointer register. See
        //   [`SHADOW_STACK_POINTER_SIZE`]
        //
        // ┌──────────┬───────── Argument base
        // │   LR     │
        // │          │
        // ├──────────┼
        // │          │
        // │   FP     │
        // ┌──────────┬
        // │          │
        // │          │
        // │          │
        // │   x28    │
        // └──────────┴ -> 32
        Self::arg_base_offset() + SHADOW_STACK_POINTER_SLOT_SIZE
    }

    fn word_bits() -> u8 {
        64
    }

    fn sig_from(
        params: &[WasmValType],
        returns: &[WasmValType],
        call_conv: &CallingConvention,
    ) -> Result<ABISig> {
        assert!(call_conv.is_apple_aarch64() || call_conv.is_default());
        // The first element tracks the general purpose register index, capped at 7 (x0-x7).
        // The second element tracks the floating point register index, capped at 7 (v0-v7).
        // Follows
        // https://github.com/ARM-software/abi-aa/blob/2021Q1/aapcs64/aapcs64.rst#64parameter-passing
        let mut params_index_env = RegIndexEnv::with_limits_per_class(8, 8);
        let results = Self::abi_results(returns, call_conv)?;
        let params =
            ABIParams::from::<_, Self>(params, 0, results.on_stack(), |ty, stack_offset| {
                Self::to_abi_operand(
                    ty,
                    stack_offset,
                    &mut params_index_env,
                    call_conv,
                    ParamsOrReturns::Params,
                )
            })?;

        Ok(ABISig::new(*call_conv, params, results))
    }

    fn abi_results(returns: &[WasmValType], call_conv: &CallingConvention) -> Result<ABIResults> {
        assert!(call_conv.is_apple_aarch64() || call_conv.is_default());
        // Use absolute count for results given that for Winch's
        // default CallingConvention only one register is used for results
        // independent of the register class.
        // In the case of 2+ results, the rest are passed in the stack,
        // similar to how Wasmtime handles multi-value returns.
        let mut returns_index_env = RegIndexEnv::with_absolute_limit(1);

        ABIResults::from(returns, call_conv, |ty, stack_offset| {
            Self::to_abi_operand(
                ty,
                stack_offset,
                &mut returns_index_env,
                call_conv,
                ParamsOrReturns::Returns,
            )
        })
    }

    fn vmctx_reg() -> Reg {
        regs::xreg(9)
    }

    fn stack_slot_size() -> u8 {
        Self::word_bytes()
    }

    fn sizeof(ty: &WasmValType) -> u8 {
        match ty {
            WasmValType::Ref(rt) => match rt.heap_type {
                WasmHeapType::Func => Self::word_bytes(),
                ht => unimplemented!("Support for WasmHeapType: {ht}"),
            },
            WasmValType::F64 | WasmValType::I64 => Self::word_bytes(),
            WasmValType::F32 | WasmValType::I32 => Self::word_bytes() / 2,
            WasmValType::V128 => Self::word_bytes() * 2,
        }
    }
}

impl Aarch64ABI {
    fn to_abi_operand(
        wasm_arg: &WasmValType,
        stack_offset: u32,
        index_env: &mut RegIndexEnv,
        call_conv: &CallingConvention,
        params_or_returns: ParamsOrReturns,
    ) -> Result<(ABIOperand, u32)> {
        let (reg, ty) = match wasm_arg {
            ty @ (WasmValType::I32 | WasmValType::I64) => {
                (index_env.next_gpr().map(regs::xreg), ty)
            }

            ty @ (WasmValType::F32 | WasmValType::F64) => {
                (index_env.next_fpr().map(regs::vreg), ty)
            }

            ty @ WasmValType::Ref(rt) => match rt.heap_type {
                WasmHeapType::Func | WasmHeapType::Extern => {
                    (index_env.next_gpr().map(regs::xreg), ty)
                }
                _ => bail!(CodeGenError::unsupported_wasm_type()),
            },

            _ => bail!(CodeGenError::unsupported_wasm_type()),
        };

        let ty_size = <Self as ABI>::sizeof(wasm_arg);
        let default = || {
            let arg = ABIOperand::stack_offset(stack_offset, *ty, ty_size as u32);
            let slot_size = Self::stack_slot_size();
            // Stack slots for parameters are aligned to a fixed slot size,
            // in the case of Aarch64, 8 bytes.
            // For the non-default calling convention, stack slots for
            // return values are type-sized aligned.
            // For the default calling convention, we don't type-size align,
            // given that results on the stack must match spills generated
            // from within the compiler, which are not type-size aligned.
            let next_stack = if params_or_returns == ParamsOrReturns::Params {
                align_to(stack_offset, slot_size as u32) + (slot_size as u32)
            } else if call_conv.is_default() {
                stack_offset + (ty_size as u32)
            } else {
                align_to(stack_offset, ty_size as u32) + (ty_size as u32)
            };
            (arg, next_stack)
        };
        Ok(reg.map_or_else(default, |reg| {
            (ABIOperand::reg(reg, *ty, ty_size as u32), stack_offset)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Aarch64ABI;
    use crate::{
        abi::{ABI, ABIOperand},
        isa::CallingConvention,
        isa::aarch64::regs,
        isa::reg::Reg,
    };
    use wasmtime_environ::{
        WasmFuncType,
        WasmValType::{self, *},
    };

    use anyhow::Result;

    #[test]
    fn xreg_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [I32, I64, I32, I64, I32, I32, I64, I32, I64].into(),
            [].into(),
        );

        let sig = Aarch64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), I32, regs::xreg(0));
        match_reg_arg(params.get(1).unwrap(), I64, regs::xreg(1));
        match_reg_arg(params.get(2).unwrap(), I32, regs::xreg(2));
        match_reg_arg(params.get(3).unwrap(), I64, regs::xreg(3));
        match_reg_arg(params.get(4).unwrap(), I32, regs::xreg(4));
        match_reg_arg(params.get(5).unwrap(), I32, regs::xreg(5));
        match_reg_arg(params.get(6).unwrap(), I64, regs::xreg(6));
        match_reg_arg(params.get(7).unwrap(), I32, regs::xreg(7));
        match_stack_arg(params.get(8).unwrap(), I64, 0);
        Ok(())
    }

    #[test]
    fn vreg_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, F64, F32, F64, F32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = Aarch64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::vreg(0));
        match_reg_arg(params.get(1).unwrap(), F64, regs::vreg(1));
        match_reg_arg(params.get(2).unwrap(), F32, regs::vreg(2));
        match_reg_arg(params.get(3).unwrap(), F64, regs::vreg(3));
        match_reg_arg(params.get(4).unwrap(), F32, regs::vreg(4));
        match_reg_arg(params.get(5).unwrap(), F32, regs::vreg(5));
        match_reg_arg(params.get(6).unwrap(), F64, regs::vreg(6));
        match_reg_arg(params.get(7).unwrap(), F32, regs::vreg(7));
        match_stack_arg(params.get(8).unwrap(), F64, 0);
        Ok(())
    }

    #[test]
    fn mixed_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = Aarch64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::vreg(0));
        match_reg_arg(params.get(1).unwrap(), I32, regs::xreg(0));
        match_reg_arg(params.get(2).unwrap(), I64, regs::xreg(1));
        match_reg_arg(params.get(3).unwrap(), F64, regs::vreg(1));
        match_reg_arg(params.get(4).unwrap(), I32, regs::xreg(2));
        match_reg_arg(params.get(5).unwrap(), F32, regs::vreg(2));
        match_reg_arg(params.get(6).unwrap(), F64, regs::vreg(3));
        match_reg_arg(params.get(7).unwrap(), F32, regs::vreg(4));
        match_reg_arg(params.get(8).unwrap(), F64, regs::vreg(5));
        Ok(())
    }

    #[test]
    fn int_abi_sig_multi_returns() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [I32, I64, I32, I64, I32, I32].into(),
            [I32, I32, I32].into(),
        );

        let sig = Aarch64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;
        let results = sig.results;

        match_reg_arg(params.get(0).unwrap(), I32, regs::xreg(1));
        match_reg_arg(params.get(1).unwrap(), I64, regs::xreg(2));
        match_reg_arg(params.get(2).unwrap(), I32, regs::xreg(3));
        match_reg_arg(params.get(3).unwrap(), I64, regs::xreg(4));
        match_reg_arg(params.get(4).unwrap(), I32, regs::xreg(5));
        match_reg_arg(params.get(5).unwrap(), I32, regs::xreg(6));

        match_stack_arg(results.get(0).unwrap(), I32, 4);
        match_stack_arg(results.get(1).unwrap(), I32, 0);
        match_reg_arg(results.get(2).unwrap(), I32, regs::xreg(0));
        Ok(())
    }

    #[test]
    fn mixed_abi_sig_multi_returns() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32].into(),
            [I32, F32, I32, F32, I64].into(),
        );

        let sig = Aarch64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;
        let results = sig.results;

        match_reg_arg(params.get(0).unwrap(), F32, regs::vreg(0));
        match_reg_arg(params.get(1).unwrap(), I32, regs::xreg(1));
        match_reg_arg(params.get(2).unwrap(), I64, regs::xreg(2));
        match_reg_arg(params.get(3).unwrap(), F64, regs::vreg(1));
        match_reg_arg(params.get(4).unwrap(), I32, regs::xreg(3));

        match_stack_arg(results.get(0).unwrap(), I32, 12);
        match_stack_arg(results.get(1).unwrap(), F32, 8);
        match_stack_arg(results.get(2).unwrap(), I32, 4);
        match_stack_arg(results.get(3).unwrap(), F32, 0);
        match_reg_arg(results.get(4).unwrap(), I64, regs::xreg(0));
        Ok(())
    }

    #[track_caller]
    fn match_reg_arg(abi_arg: &ABIOperand, expected_ty: WasmValType, expected_reg: Reg) {
        match abi_arg {
            &ABIOperand::Reg { reg, ty, .. } => {
                assert_eq!(reg, expected_reg);
                assert_eq!(ty, expected_ty);
            }
            stack => panic!("Expected reg argument, got {stack:?}"),
        }
    }

    #[track_caller]
    fn match_stack_arg(abi_arg: &ABIOperand, expected_ty: WasmValType, expected_offset: u32) {
        match abi_arg {
            &ABIOperand::Stack { offset, ty, .. } => {
                assert_eq!(offset, expected_offset);
                assert_eq!(ty, expected_ty);
            }
            reg => panic!("Expected stack argument, got {reg:?}"),
        }
    }
}
