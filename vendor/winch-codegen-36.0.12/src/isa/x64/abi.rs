use super::regs;
use crate::{
    RegIndexEnv,
    abi::{ABI, ABIOperand, ABIParams, ABIResults, ABISig, ParamsOrReturns, align_to},
    codegen::CodeGenError,
    isa::{CallingConvention, reg::Reg},
};
use anyhow::{Result, bail};
use wasmtime_environ::{WasmHeapType, WasmValType};

#[derive(Default)]
pub(crate) struct X64ABI;

impl ABI for X64ABI {
    fn stack_align() -> u8 {
        16
    }

    fn call_stack_align() -> u8 {
        16
    }

    fn arg_base_offset() -> u8 {
        // Two 8-byte slots, one for the return address and another
        // one for the frame pointer.
        // ┌──────────┬───────── Argument base
        // │   Ret    │
        // │   Addr   │
        // ├──────────┼
        // │          │
        // │   FP     │
        // └──────────┴
        16
    }

    fn initial_frame_size() -> u8 {
        // The initial frame size is equal to the space allocated to save the
        // return address and the frame pointer.
        Self::arg_base_offset()
    }

    fn word_bits() -> u8 {
        64
    }

    fn sig_from(
        params: &[WasmValType],
        returns: &[WasmValType],
        call_conv: &CallingConvention,
    ) -> Result<ABISig> {
        assert!(call_conv.is_fastcall() || call_conv.is_systemv() || call_conv.is_default());
        let is_fastcall = call_conv.is_fastcall();
        // In the fastcall calling convention, the callee gets a contiguous
        // stack area of 32 bytes (4 register arguments) just before its frame.
        // See
        // https://learn.microsoft.com/en-us/cpp/build/stack-usage?view=msvc-170#stack-allocation
        let (params_stack_offset, mut params_index_env) = if is_fastcall {
            (32, RegIndexEnv::with_absolute_limit(4))
        } else {
            (0, RegIndexEnv::with_limits_per_class(6, 8))
        };

        let results = Self::abi_results(returns, call_conv)?;
        let params = ABIParams::from::<_, Self>(
            params,
            params_stack_offset,
            results.on_stack(),
            |ty, stack_offset| {
                Self::to_abi_operand(
                    ty,
                    stack_offset,
                    &mut params_index_env,
                    call_conv,
                    ParamsOrReturns::Params,
                )
            },
        )?;

        Ok(ABISig::new(*call_conv, params, results))
    }

    fn abi_results(returns: &[WasmValType], call_conv: &CallingConvention) -> Result<ABIResults> {
        assert!(call_conv.is_default() || call_conv.is_fastcall() || call_conv.is_systemv());
        // Use absolute count for results given that for Winch's
        // default CallingConvention only one register is used for results
        // independent of the register class.
        // In the case of 2+ results, the rest are passed in the stack,
        // similar to how Wasmtime handles multi-value returns.
        let mut results_index_env = RegIndexEnv::with_absolute_limit(1);
        ABIResults::from(returns, call_conv, |ty, offset| {
            Self::to_abi_operand(
                ty,
                offset,
                &mut results_index_env,
                call_conv,
                ParamsOrReturns::Returns,
            )
        })
    }

    fn vmctx_reg() -> Reg {
        regs::vmctx()
    }

    fn stack_slot_size() -> u8 {
        // Winch default calling convention follows SysV calling convention so
        // we use one 8 byte slot for values that are smaller or equal to 8
        // bytes in size and 2 8 byte slots for values that are 128 bits.
        // See Section 3.2.3 in
        // https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf for further
        // details.
        Self::word_bytes()
    }

    fn sizeof(ty: &WasmValType) -> u8 {
        match ty {
            WasmValType::Ref(rt) => match rt.heap_type {
                WasmHeapType::Func | WasmHeapType::Extern => Self::word_bytes(),
                ht => unimplemented!("Support for WasmHeapType: {ht}"),
            },
            WasmValType::F64 | WasmValType::I64 => Self::word_bytes(),
            WasmValType::F32 | WasmValType::I32 => Self::word_bytes() / 2,
            WasmValType::V128 => Self::word_bytes() * 2,
        }
    }
}

impl X64ABI {
    fn to_abi_operand(
        wasm_arg: &WasmValType,
        stack_offset: u32,
        index_env: &mut RegIndexEnv,
        call_conv: &CallingConvention,
        params_or_returns: ParamsOrReturns,
    ) -> Result<(ABIOperand, u32)> {
        let (reg, ty) = match wasm_arg {
            ty @ WasmValType::Ref(rt) => match rt.heap_type {
                WasmHeapType::Func | WasmHeapType::Extern => (
                    Self::int_reg_for(index_env.next_gpr(), call_conv, params_or_returns),
                    ty,
                ),
                _ => bail!(CodeGenError::unsupported_wasm_type()),
            },

            ty @ (WasmValType::I32 | WasmValType::I64) => (
                Self::int_reg_for(index_env.next_gpr(), call_conv, params_or_returns),
                ty,
            ),

            // v128 also uses an XMM register (that is, an fpr).
            ty @ (WasmValType::F32 | WasmValType::F64 | WasmValType::V128) => (
                Self::float_reg_for(index_env.next_fpr(), call_conv, params_or_returns),
                ty,
            ),
        };

        let ty_size = <Self as ABI>::sizeof(wasm_arg);
        let default = || {
            let slot_size = Self::stack_slot_size();
            if params_or_returns == ParamsOrReturns::Params {
                // Stack slots for parameters are aligned to a fixed slot size,
                // 8 bytes if the type size is 8 or less and type-sized aligned
                // if the type size is greater than 8 bytes.
                let alignment = std::cmp::max(ty_size, slot_size);
                let offset = align_to(stack_offset, u32::from(alignment));
                let arg = ABIOperand::stack_offset(offset, *ty, u32::from(ty_size));
                (arg, offset + u32::from(alignment))
            } else {
                // For the default calling convention, we don't type-size align,
                // given that results on the stack must match spills generated
                // from within the compiler, which are not type-size aligned.
                // In all other cases the results are type-sized aligned.
                if call_conv.is_default() {
                    let arg = ABIOperand::stack_offset(stack_offset, *ty, u32::from(ty_size));
                    (arg, stack_offset + (ty_size as u32))
                } else {
                    let offset = align_to(stack_offset, u32::from(ty_size));
                    (
                        ABIOperand::stack_offset(offset, *ty, u32::from(ty_size)),
                        offset + u32::from(ty_size),
                    )
                }
            }
        };

        Ok(reg.map_or_else(default, |reg| {
            (ABIOperand::reg(reg, *ty, ty_size as u32), stack_offset)
        }))
    }

    fn int_reg_for(
        index: Option<u8>,
        call_conv: &CallingConvention,
        params_or_returns: ParamsOrReturns,
    ) -> Option<Reg> {
        use ParamsOrReturns::*;

        let index = match index {
            None => return None,
            Some(index) => index,
        };

        if call_conv.is_fastcall() {
            return match (index, params_or_returns) {
                (0, Params) => Some(regs::rcx()),
                (1, Params) => Some(regs::rdx()),
                (2, Params) => Some(regs::r8()),
                (3, Params) => Some(regs::r9()),
                (0, Returns) => Some(regs::rax()),
                _ => None,
            };
        }

        if call_conv.is_systemv() || call_conv.is_default() {
            return match (index, params_or_returns) {
                (0, Params) => Some(regs::rdi()),
                (1, Params) => Some(regs::rsi()),
                (2, Params) => Some(regs::rdx()),
                (3, Params) => Some(regs::rcx()),
                (4, Params) => Some(regs::r8()),
                (5, Params) => Some(regs::r9()),
                (0, Returns) => Some(regs::rax()),
                _ => None,
            };
        }

        None
    }

    fn float_reg_for(
        index: Option<u8>,
        call_conv: &CallingConvention,
        params_or_returns: ParamsOrReturns,
    ) -> Option<Reg> {
        use ParamsOrReturns::*;

        let index = match index {
            None => return None,
            Some(index) => index,
        };

        if call_conv.is_fastcall() {
            return match (index, params_or_returns) {
                (0, Params) => Some(regs::xmm0()),
                (1, Params) => Some(regs::xmm1()),
                (2, Params) => Some(regs::xmm2()),
                (3, Params) => Some(regs::xmm3()),
                (0, Returns) => Some(regs::xmm0()),
                _ => None,
            };
        }

        if call_conv.is_systemv() || call_conv.is_default() {
            return match (index, params_or_returns) {
                (0, Params) => Some(regs::xmm0()),
                (1, Params) => Some(regs::xmm1()),
                (2, Params) => Some(regs::xmm2()),
                (3, Params) => Some(regs::xmm3()),
                (4, Params) => Some(regs::xmm4()),
                (5, Params) => Some(regs::xmm5()),
                (6, Params) => Some(regs::xmm6()),
                (7, Params) => Some(regs::xmm7()),
                (0, Returns) => Some(regs::xmm0()),
                _ => None,
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::X64ABI;
    use crate::{
        abi::{ABI, ABIOperand},
        isa::{CallingConvention, reg::Reg, x64::regs},
    };

    use anyhow::Result;

    use wasmtime_environ::{
        WasmFuncType,
        WasmValType::{self, *},
    };

    #[test]
    fn int_abi_sig() -> Result<()> {
        let wasm_sig =
            WasmFuncType::new([I32, I64, I32, I64, I32, I32, I64, I32].into(), [].into());

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), I32, regs::rdi());
        match_reg_arg(params.get(1).unwrap(), I64, regs::rsi());
        match_reg_arg(params.get(2).unwrap(), I32, regs::rdx());
        match_reg_arg(params.get(3).unwrap(), I64, regs::rcx());
        match_reg_arg(params.get(4).unwrap(), I32, regs::r8());
        match_reg_arg(params.get(5).unwrap(), I32, regs::r9());
        match_stack_arg(params.get(6).unwrap(), I64, 0);
        match_stack_arg(params.get(7).unwrap(), I32, 8);
        Ok(())
    }

    #[test]
    fn int_abi_sig_multi_returns() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [I32, I64, I32, I64, I32, I32, I64, I32].into(),
            [I32, I32, I32].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;
        let results = sig.results;

        match_reg_arg(params.get(0).unwrap(), I32, regs::rsi());
        match_reg_arg(params.get(1).unwrap(), I64, regs::rdx());
        match_reg_arg(params.get(2).unwrap(), I32, regs::rcx());
        match_reg_arg(params.get(3).unwrap(), I64, regs::r8());
        match_reg_arg(params.get(4).unwrap(), I32, regs::r9());
        match_stack_arg(params.get(5).unwrap(), I32, 0);
        match_stack_arg(params.get(6).unwrap(), I64, 8);
        match_stack_arg(params.get(7).unwrap(), I32, 16);

        match_stack_arg(results.get(0).unwrap(), I32, 4);
        match_stack_arg(results.get(1).unwrap(), I32, 0);
        match_reg_arg(results.get(2).unwrap(), I32, regs::rax());
        Ok(())
    }

    #[test]
    fn float_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, F64, F32, F64, F32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::xmm0());
        match_reg_arg(params.get(1).unwrap(), F64, regs::xmm1());
        match_reg_arg(params.get(2).unwrap(), F32, regs::xmm2());
        match_reg_arg(params.get(3).unwrap(), F64, regs::xmm3());
        match_reg_arg(params.get(4).unwrap(), F32, regs::xmm4());
        match_reg_arg(params.get(5).unwrap(), F32, regs::xmm5());
        match_reg_arg(params.get(6).unwrap(), F64, regs::xmm6());
        match_reg_arg(params.get(7).unwrap(), F32, regs::xmm7());
        match_stack_arg(params.get(8).unwrap(), F64, 0);
        Ok(())
    }

    #[test]
    fn vector_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [V128, V128, V128, V128, V128, V128, V128, V128, V128, V128].into(),
            [].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), V128, regs::xmm0());
        match_reg_arg(params.get(1).unwrap(), V128, regs::xmm1());
        match_reg_arg(params.get(2).unwrap(), V128, regs::xmm2());
        match_reg_arg(params.get(3).unwrap(), V128, regs::xmm3());
        match_reg_arg(params.get(4).unwrap(), V128, regs::xmm4());
        match_reg_arg(params.get(5).unwrap(), V128, regs::xmm5());
        match_reg_arg(params.get(6).unwrap(), V128, regs::xmm6());
        match_reg_arg(params.get(7).unwrap(), V128, regs::xmm7());
        match_stack_arg(params.get(8).unwrap(), V128, 0);
        match_stack_arg(params.get(9).unwrap(), V128, 16);
        Ok(())
    }

    #[test]
    fn vector_abi_sig_multi_returns() -> Result<()> {
        let wasm_sig = WasmFuncType::new([].into(), [V128, V128, V128].into());

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let results = sig.results;

        match_stack_arg(results.get(0).unwrap(), V128, 16);
        match_stack_arg(results.get(1).unwrap(), V128, 0);
        match_reg_arg(results.get(2).unwrap(), V128, regs::xmm0());
        Ok(())
    }

    #[test]
    fn mixed_abi_sig() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::xmm0());
        match_reg_arg(params.get(1).unwrap(), I32, regs::rdi());
        match_reg_arg(params.get(2).unwrap(), I64, regs::rsi());
        match_reg_arg(params.get(3).unwrap(), F64, regs::xmm1());
        match_reg_arg(params.get(4).unwrap(), I32, regs::rdx());
        match_reg_arg(params.get(5).unwrap(), F32, regs::xmm2());
        match_reg_arg(params.get(6).unwrap(), F64, regs::xmm3());
        match_reg_arg(params.get(7).unwrap(), F32, regs::xmm4());
        match_reg_arg(params.get(8).unwrap(), F64, regs::xmm5());

        let wasm_sig = WasmFuncType::new(
            [F32, F32, F32, F32, F32, F32, F32, F32, F32, V128].into(),
            [V128].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::Default)?;
        let params = sig.params;

        match_stack_arg(params.get(8).unwrap(), F32, 0);
        match_stack_arg(params.get(9).unwrap(), V128, 16);
        Ok(())
    }

    #[test]
    fn system_v_call_conv() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::SystemV)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::xmm0());
        match_reg_arg(params.get(1).unwrap(), I32, regs::rdi());
        match_reg_arg(params.get(2).unwrap(), I64, regs::rsi());
        match_reg_arg(params.get(3).unwrap(), F64, regs::xmm1());
        match_reg_arg(params.get(4).unwrap(), I32, regs::rdx());
        match_reg_arg(params.get(5).unwrap(), F32, regs::xmm2());
        match_reg_arg(params.get(6).unwrap(), F64, regs::xmm3());
        match_reg_arg(params.get(7).unwrap(), F32, regs::xmm4());
        match_reg_arg(params.get(8).unwrap(), F64, regs::xmm5());
        Ok(())
    }

    #[test]
    fn fastcall_call_conv() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32, F32, F64, F32, F64].into(),
            [].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::WindowsFastcall)?;
        let params = sig.params;

        match_reg_arg(params.get(0).unwrap(), F32, regs::xmm0());
        match_reg_arg(params.get(1).unwrap(), I32, regs::rdx());
        match_reg_arg(params.get(2).unwrap(), I64, regs::r8());
        match_reg_arg(params.get(3).unwrap(), F64, regs::xmm3());
        match_stack_arg(params.get(4).unwrap(), I32, 32);
        match_stack_arg(params.get(5).unwrap(), F32, 40);
        Ok(())
    }

    #[test]
    fn fastcall_call_conv_multi_returns() -> Result<()> {
        let wasm_sig = WasmFuncType::new(
            [F32, I32, I64, F64, I32, F32, F64, F32, F64].into(),
            [I32, F32, I32, F32, I64].into(),
        );

        let sig = X64ABI::sig(&wasm_sig, &CallingConvention::WindowsFastcall)?;
        let params = sig.params;
        let results = sig.results;

        match_reg_arg(params.get(0).unwrap(), F32, regs::xmm1());
        match_reg_arg(params.get(1).unwrap(), I32, regs::r8());
        match_reg_arg(params.get(2).unwrap(), I64, regs::r9());
        // Each argument stack slot is 8 bytes.
        match_stack_arg(params.get(3).unwrap(), F64, 32);
        match_stack_arg(params.get(4).unwrap(), I32, 40);
        match_stack_arg(params.get(5).unwrap(), F32, 48);

        match_reg_arg(results.get(0).unwrap(), I32, regs::rax());

        match_stack_arg(results.get(1).unwrap(), F32, 0);
        match_stack_arg(results.get(2).unwrap(), I32, 4);
        match_stack_arg(results.get(3).unwrap(), F32, 8);
        match_stack_arg(results.get(4).unwrap(), I64, 16);
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
