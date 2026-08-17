//! Builtin function handling.

use crate::{
    CallingConvention,
    abi::{ABI, ABISig},
    codegen::env::ptr_type_from_ptr_size,
};
use anyhow::Result;
use std::sync::Arc;
use wasmtime_environ::{BuiltinFunctionIndex, PtrSize, VMOffsets, WasmValType};

#[derive(Copy, Clone)]
pub(crate) enum BuiltinType {
    /// Dynamic built-in function, derived from the VMContext.
    Builtin(BuiltinFunctionIndex),
}

impl BuiltinType {
    /// Creates a new builtin from a Wasmtime-defined builtin function
    /// enumerated with a [`BuiltinFunctionIndex`].
    pub fn builtin(idx: BuiltinFunctionIndex) -> Self {
        Self::Builtin(idx)
    }
}

#[derive(Clone)]
pub struct BuiltinFunction {
    inner: Arc<BuiltinFunctionInner>,
}

impl BuiltinFunction {
    pub(crate) fn sig(&self) -> &ABISig {
        &self.inner.sig
    }

    pub(crate) fn ty(&self) -> BuiltinType {
        self.inner.ty
    }
}

/// Metadata about a builtin function.
pub struct BuiltinFunctionInner {
    /// The ABI specific signature of the function.
    sig: ABISig,
    /// The built-in function type.
    ty: BuiltinType,
}

macro_rules! declare_function_sig {
    (
        $(
            $( #[$attr:meta] )*
            $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
         )*
    ) => {
        /// Provides the ABI signatures for each builtin function
        /// signature.
        pub struct BuiltinFunctions {
            /// The target calling convention for host intrinsics.
            host_call_conv: CallingConvention,
            /// The target calling convention for wasm builtins.
            wasm_call_conv: CallingConvention,
            /// The target pointer type, as a WebAssembly type.
            ptr_type: WasmValType,
            $(
                $( #[ $attr ] )*
                $name: Option<BuiltinFunction>,
            )*
        }

        #[expect(dead_code, reason = "not all functions used yet")]
        impl BuiltinFunctions {
            pub fn new<P: PtrSize>(
                vmoffsets: &VMOffsets<P>,
                host_call_conv: CallingConvention,
                wasm_call_conv: CallingConvention,
            ) -> Self {
                let size = vmoffsets.ptr.size();
                Self {
                    host_call_conv,
                    wasm_call_conv,
                    ptr_type: ptr_type_from_ptr_size(size),
                    $(
                        $( #[ $attr ] )*
                        $name: None,
                    )*
                }
            }

            fn pointer(&self) -> WasmValType {
                self.ptr_type
            }

            fn size(&self) -> WasmValType {
                self.ptr_type
            }

            fn vmctx(&self) -> WasmValType {
                self.pointer()
            }

            fn u32(&self) -> WasmValType {
                WasmValType::I32
            }

            fn u8(&self) -> WasmValType {
                WasmValType::I32
            }

            fn f32(&self) -> WasmValType {
                WasmValType::F32
            }

            fn f64(&self) -> WasmValType {
                WasmValType::F64
            }

            fn u64(&self) -> WasmValType {
                WasmValType::I64
            }

            fn i8x16(&self) -> WasmValType {
                WasmValType::V128
            }

            fn f32x4(&self) -> WasmValType {
                WasmValType::V128
            }

            fn f64x2(&self) -> WasmValType {
                WasmValType::V128
            }

            fn bool(&self) -> WasmValType {
                WasmValType::I32
            }

            fn over_f64<A: ABI>(&self) -> Result<ABISig> {
                A::sig_from(&[self.f64()], &[self.f64()], &self.host_call_conv)
            }

            fn over_f32<A: ABI>(&self) -> Result<ABISig> {
                A::sig_from(&[self.f64()], &[self.f64()], &self.host_call_conv)
            }

            $(
                $( #[ $attr ] )*
                pub(crate) fn $name<A: ABI, P: PtrSize>(&mut self) -> Result<BuiltinFunction> {
                    if self.$name.is_none() {
                        let params = vec![ $(self.$param() ),* ];
                        let result = vec![ $(self.$result() )?];
                        let sig = A::sig_from(&params, &result, &self.wasm_call_conv)?;
                        let index = BuiltinFunctionIndex::$name();
                        let inner = Arc::new(BuiltinFunctionInner { sig, ty: BuiltinType::builtin(index) });
                        self.$name = Some(BuiltinFunction {
                            inner,
                        });
                    }

                    Ok(self.$name.as_ref().unwrap().clone())
                }
             )*
        }
    }
}

wasmtime_environ::foreach_builtin_function!(declare_function_sig);
