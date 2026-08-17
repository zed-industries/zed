//! Compilation support for the component model.

use crate::{
    TRAP_ALWAYS, TRAP_CANNOT_ENTER, TRAP_INTERNAL_ASSERT,
    compiler::{Abi, Compiler},
};
use anyhow::Result;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{self, InstBuilder, MemFlags, Value};
use cranelift_codegen::isa::{CallConv, TargetIsa};
use cranelift_frontend::FunctionBuilder;
use wasmtime_environ::fact::PREPARE_CALL_FIXED_PARAMS;
use wasmtime_environ::{CompiledFunctionBody, component::*};
use wasmtime_environ::{
    EntityRef, HostCall, ModuleInternedTypeIndex, PtrSize, TrapSentinel, Tunables, WasmFuncType,
    WasmValType,
};

struct TrampolineCompiler<'a> {
    compiler: &'a Compiler,
    isa: &'a (dyn TargetIsa + 'static),
    builder: FunctionBuilder<'a>,
    component: &'a Component,
    types: &'a ComponentTypesBuilder,
    offsets: VMComponentOffsets<u8>,
    abi: Abi,
    block0: ir::Block,
    signature: ModuleInternedTypeIndex,
    tunables: &'a Tunables,
}

/// What host functions can be called, used in `translate_hostcall` below.
enum HostCallee {
    /// Call a host-lowered function specified by this index.
    Lowering(LoweredIndex),
    /// Call a host libcall, specified by this accessor.
    Libcall(GetLibcallFn),
}

type GetLibcallFn =
    fn(&dyn TargetIsa, &mut ir::Function) -> (ir::SigRef, ComponentBuiltinFunctionIndex);

impl From<LoweredIndex> for HostCallee {
    fn from(index: LoweredIndex) -> HostCallee {
        HostCallee::Lowering(index)
    }
}

impl From<GetLibcallFn> for HostCallee {
    fn from(f: GetLibcallFn) -> HostCallee {
        HostCallee::Libcall(f)
    }
}

/// How to interpret the results of a host function.
enum HostResult {
    /// The host function has no results.
    None,

    /// The host function returns the sentinel specified which is interpreted
    /// and translated to the real return value.
    Sentinel(TrapSentinel),

    /// The host function returns a `bool` indicating whether it succeeded or
    /// not.
    ///
    /// After the return value is interpreted the host function also filled in
    /// `ptr` and `len` with wasm return values which need to be returned.
    ///
    /// If `ptr` and `len` are not specified then this must be used with
    /// `WasmArgs::ValRawList` and that ptr/len is used.
    MultiValue {
        /// The base pointer of the `ValRaw` list on the stack.
        ptr: Option<ir::Value>,
        /// The length of the `ValRaw` list on the stack.
        len: Option<ir::Value>,
    },
}

impl From<TrapSentinel> for HostResult {
    fn from(sentinel: TrapSentinel) -> HostResult {
        HostResult::Sentinel(sentinel)
    }
}

/// Different means of passing WebAssembly arguments to host calls.
#[derive(Debug, Copy, Clone)]
enum WasmArgs {
    /// All wasm arguments to the host are passed directly as values, typically
    /// through registers.
    InRegisters,

    /// All wasm arguments to the host are passed indirectly by spilling them
    /// to the stack as a sequence of contiguous `ValRaw`s.
    ValRawList,

    /// The first `n` arguments are passed in registers, but everything after
    /// that is spilled to the stack.
    InRegistersUpTo(usize),
}

impl<'a> TrampolineCompiler<'a> {
    fn new(
        compiler: &'a Compiler,
        func_compiler: &'a mut super::FunctionCompiler<'_>,
        component: &'a Component,
        types: &'a ComponentTypesBuilder,
        index: TrampolineIndex,
        abi: Abi,
        tunables: &'a Tunables,
    ) -> TrampolineCompiler<'a> {
        let isa = &*compiler.isa;
        let signature = component.trampolines[index];
        let ty = types[signature].unwrap_func();
        let func = ir::Function::with_name_signature(
            ir::UserFuncName::user(0, 0),
            match abi {
                Abi::Wasm => crate::wasm_call_signature(isa, ty, &compiler.tunables),
                Abi::Array => crate::array_call_signature(isa),
            },
        );
        let (builder, block0) = func_compiler.builder(func);
        TrampolineCompiler {
            compiler,
            isa,
            builder,
            component,
            types,
            offsets: VMComponentOffsets::new(isa.pointer_bytes(), component),
            abi,
            block0,
            signature,
            tunables,
        }
    }

    fn translate(&mut self, trampoline: &Trampoline) {
        match trampoline {
            Trampoline::Transcoder {
                op,
                from,
                from64,
                to,
                to64,
            } => {
                match self.abi {
                    Abi::Wasm => {
                        self.translate_transcode(*op, *from, *from64, *to, *to64);
                    }
                    // Transcoders can only actually be called by Wasm, so let's assert
                    // that here.
                    Abi::Array => {
                        self.builder.ins().trap(TRAP_INTERNAL_ASSERT);
                    }
                }
            }
            Trampoline::LowerImport {
                index,
                options,
                lower_ty,
            } => {
                let pointer_type = self.isa.pointer_type();
                self.translate_hostcall(
                    HostCallee::Lowering(*index),
                    HostResult::MultiValue {
                        ptr: None,
                        len: None,
                    },
                    WasmArgs::ValRawList,
                    |me, params| {
                        let vmctx = params[0];
                        params.extend([
                            me.builder.ins().load(
                                pointer_type,
                                MemFlags::trusted(),
                                vmctx,
                                i32::try_from(me.offsets.lowering_data(*index)).unwrap(),
                            ),
                            me.index_value(*lower_ty),
                            me.index_value(*options),
                        ]);
                    },
                );
            }
            Trampoline::AlwaysTrap => {
                if self.tunables.signals_based_traps {
                    self.builder.ins().trap(TRAP_ALWAYS);
                    return;
                }
                self.translate_libcall(
                    host::trap,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        let code = wasmtime_environ::Trap::AlwaysTrapAdapter as u8;
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(code)));
                    },
                );
            }
            Trampoline::ResourceNew(ty) => {
                // Currently this only supports resources represented by `i32`
                assert_eq!(
                    self.types[self.signature].unwrap_func().params()[0],
                    WasmValType::I32
                );
                self.translate_libcall(
                    host::resource_new32,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::ResourceRep(ty) => {
                // Currently this only supports resources represented by `i32`
                assert_eq!(
                    self.types[self.signature].unwrap_func().returns()[0],
                    WasmValType::I32
                );
                self.translate_libcall(
                    host::resource_rep32,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::ResourceDrop(ty) => {
                self.translate_resource_drop(*ty);
            }
            Trampoline::BackpressureSet { instance } => {
                self.translate_libcall(
                    host::backpressure_set,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::TaskReturn { results, options } => {
                self.translate_libcall(
                    host::task_return,
                    TrapSentinel::Falsy,
                    WasmArgs::ValRawList,
                    |me, params| {
                        params.push(me.index_value(*results));
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::TaskCancel { instance } => {
                self.translate_libcall(
                    host::task_cancel,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::WaitableSetNew { instance } => {
                self.translate_libcall(
                    host::waitable_set_new,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::WaitableSetWait { options } => {
                self.translate_libcall(
                    host::waitable_set_wait,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::WaitableSetPoll { options } => {
                self.translate_libcall(
                    host::waitable_set_poll,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::WaitableSetDrop { instance } => {
                self.translate_libcall(
                    host::waitable_set_drop,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::WaitableJoin { instance } => {
                self.translate_libcall(
                    host::waitable_join,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::Yield { async_ } => {
                self.translate_libcall(
                    host::yield_,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::SubtaskDrop { instance } => {
                self.translate_libcall(
                    host::subtask_drop,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                    },
                );
            }
            Trampoline::SubtaskCancel { instance, async_ } => {
                self.translate_libcall(
                    host::subtask_cancel,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*instance));
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::StreamNew { ty } => {
                self.translate_libcall(
                    host::stream_new,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::StreamRead { ty, options } => {
                if let Some(info) = self.flat_stream_element_info(*ty).cloned() {
                    self.translate_libcall(
                        host::flat_stream_read,
                        TrapSentinel::NegativeOne,
                        WasmArgs::InRegisters,
                        |me, params| {
                            params.extend([
                                me.index_value(*ty),
                                me.index_value(*options),
                                me.builder
                                    .ins()
                                    .iconst(ir::types::I32, i64::from(info.size32)),
                                me.builder
                                    .ins()
                                    .iconst(ir::types::I32, i64::from(info.align32)),
                            ]);
                        },
                    );
                } else {
                    self.translate_libcall(
                        host::stream_read,
                        TrapSentinel::NegativeOne,
                        WasmArgs::InRegisters,
                        |me, params| {
                            params.push(me.index_value(*ty));
                            params.push(me.index_value(*options));
                        },
                    );
                }
            }
            Trampoline::StreamWrite { ty, options } => {
                if let Some(info) = self.flat_stream_element_info(*ty).cloned() {
                    self.translate_libcall(
                        host::flat_stream_write,
                        TrapSentinel::NegativeOne,
                        WasmArgs::InRegisters,
                        |me, params| {
                            params.extend([
                                me.index_value(*ty),
                                me.index_value(*options),
                                me.builder
                                    .ins()
                                    .iconst(ir::types::I32, i64::from(info.size32)),
                                me.builder
                                    .ins()
                                    .iconst(ir::types::I32, i64::from(info.align32)),
                            ]);
                        },
                    );
                } else {
                    self.translate_libcall(
                        host::stream_write,
                        TrapSentinel::NegativeOne,
                        WasmArgs::InRegisters,
                        |me, params| {
                            params.push(me.index_value(*ty));
                            params.push(me.index_value(*options));
                        },
                    );
                }
            }
            Trampoline::StreamCancelRead { ty, async_ } => {
                self.translate_libcall(
                    host::stream_cancel_read,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::StreamCancelWrite { ty, async_ } => {
                self.translate_libcall(
                    host::stream_cancel_write,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::StreamDropReadable { ty } => {
                self.translate_libcall(
                    host::stream_drop_readable,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::StreamDropWritable { ty } => {
                self.translate_libcall(
                    host::stream_drop_writable,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::FutureNew { ty } => {
                self.translate_libcall(
                    host::future_new,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::FutureRead { ty, options } => {
                self.translate_libcall(
                    host::future_read,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::FutureWrite { ty, options } => {
                self.translate_libcall(
                    host::future_write,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::FutureCancelRead { ty, async_ } => {
                self.translate_libcall(
                    host::future_cancel_read,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::FutureCancelWrite { ty, async_ } => {
                self.translate_libcall(
                    host::future_cancel_write,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.builder.ins().iconst(ir::types::I8, i64::from(*async_)));
                    },
                );
            }
            Trampoline::FutureDropReadable { ty } => {
                self.translate_libcall(
                    host::future_drop_readable,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::FutureDropWritable { ty } => {
                self.translate_libcall(
                    host::future_drop_writable,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::ErrorContextNew { ty, options } => {
                self.translate_libcall(
                    host::error_context_new,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::ErrorContextDebugMessage { ty, options } => {
                self.translate_libcall(
                    host::error_context_debug_message,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                        params.push(me.index_value(*options));
                    },
                );
            }
            Trampoline::ErrorContextDrop { ty } => {
                self.translate_libcall(
                    host::error_context_drop,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.index_value(*ty));
                    },
                );
            }
            Trampoline::ResourceTransferOwn => {
                self.translate_libcall(
                    host::resource_transfer_own,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::ResourceTransferBorrow => {
                self.translate_libcall(
                    host::resource_transfer_borrow,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::ResourceEnterCall => {
                self.translate_libcall(
                    host::resource_enter_call,
                    HostResult::None,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::ResourceExitCall => {
                self.translate_libcall(
                    host::resource_exit_call,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::PrepareCall { memory } => {
                self.translate_libcall(
                    host::prepare_call,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegistersUpTo(PREPARE_CALL_FIXED_PARAMS.len()),
                    |me, params| {
                        let vmctx = params[0];
                        params.push(me.load_optional_memory(vmctx, *memory));
                    },
                );
            }
            Trampoline::SyncStartCall { callback } => {
                let pointer_type = self.isa.pointer_type();
                let wasm_func_ty = &self.types[self.signature].unwrap_func();
                let (values_vec_ptr, len) = self.compiler.allocate_stack_array_and_spill_args(
                    &WasmFuncType::new(
                        Box::new([]),
                        wasm_func_ty.returns().iter().copied().collect(),
                    ),
                    &mut self.builder,
                    &[],
                );
                let values_vec_len = self.builder.ins().iconst(pointer_type, i64::from(len));
                self.translate_libcall(
                    host::sync_start,
                    HostResult::MultiValue {
                        ptr: Some(values_vec_ptr),
                        len: Some(values_vec_len),
                    },
                    WasmArgs::InRegisters,
                    |me, params| {
                        let vmctx = params[0];
                        params.push(me.load_callback(vmctx, *callback));
                        params.push(values_vec_ptr);
                        params.push(values_vec_len);
                    },
                );
            }
            Trampoline::AsyncStartCall {
                callback,
                post_return,
            } => {
                self.translate_libcall(
                    host::async_start,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        let vmctx = params[0];
                        params.extend([
                            me.load_callback(vmctx, *callback),
                            me.load_post_return(vmctx, *post_return),
                        ]);
                    },
                );
            }
            Trampoline::FutureTransfer => {
                self.translate_libcall(
                    host::future_transfer,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::StreamTransfer => {
                self.translate_libcall(
                    host::stream_transfer,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::ErrorContextTransfer => {
                self.translate_libcall(
                    host::error_context_transfer,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |_, _| {},
                );
            }
            Trampoline::ContextGet(i) => {
                self.translate_libcall(
                    host::context_get,
                    TrapSentinel::NegativeOne,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.builder.ins().iconst(ir::types::I32, i64::from(*i)));
                    },
                );
            }
            Trampoline::ContextSet(i) => {
                self.translate_libcall(
                    host::context_set,
                    TrapSentinel::Falsy,
                    WasmArgs::InRegisters,
                    |me, params| {
                        params.push(me.builder.ins().iconst(ir::types::I32, i64::from(*i)));
                    },
                );
            }
        }
    }

    /// Determine whether the specified type can be optimized as a stream
    /// payload by lifting and lowering with a simple `memcpy`.
    ///
    /// Any type containing only "flat", primitive data (i.e. no pointers or
    /// handles) should qualify for this optimization, but it's also okay to
    /// conservatively return `None` here; the fallback slow path will always
    /// work -- it just won't be as efficient.
    fn flat_stream_element_info(&self, ty: TypeStreamTableIndex) -> Option<&CanonicalAbiInfo> {
        let payload = self.types[self.types[ty].ty].payload;
        match payload {
            None => Some(&CanonicalAbiInfo::ZERO),
            Some(
                payload @ (InterfaceType::Bool
                | InterfaceType::S8
                | InterfaceType::U8
                | InterfaceType::S16
                | InterfaceType::U16
                | InterfaceType::S32
                | InterfaceType::U32
                | InterfaceType::S64
                | InterfaceType::U64
                | InterfaceType::Float32
                | InterfaceType::Float64
                | InterfaceType::Char),
            ) => Some(self.types.canonical_abi(&payload)),
            // TODO: Recursively check for other "flat" types (i.e. those without pointers or handles),
            // e.g. `record`s, `variant`s, etc. which contain only flat types.
            _ => None,
        }
    }

    /// Helper function to spill the wasm arguments `args` to this function into
    /// a stack-allocated array.
    fn store_wasm_arguments(&mut self, args: &[Value]) -> (Value, Value) {
        let pointer_type = self.isa.pointer_type();
        let wasm_func_ty = &self.types[self.signature].unwrap_func();

        match self.abi {
            // For the wasm ABI a stack needs to be allocated and these
            // arguments are stored onto the stack.
            Abi::Wasm => {
                let (ptr, len) = self.compiler.allocate_stack_array_and_spill_args(
                    wasm_func_ty,
                    &mut self.builder,
                    args,
                );
                let len = self.builder.ins().iconst(pointer_type, i64::from(len));
                (ptr, len)
            }

            // For the array ABI all arguments were already in a stack, so
            // forward along that pointer/len.
            Abi::Array => {
                let params = self.builder.func.dfg.block_params(self.block0);
                (params[2], params[3])
            }
        }
    }

    /// Convenience wrapper around `translate_hostcall` to enable type inference
    /// on the `get_libcall` parameter here.
    fn translate_libcall(
        &mut self,
        get_libcall: GetLibcallFn,
        host_result: impl Into<HostResult>,
        wasm_args: WasmArgs,
        extra_host_args: impl FnOnce(&mut Self, &mut Vec<ir::Value>),
    ) {
        self.translate_hostcall(
            HostCallee::Libcall(get_libcall),
            host_result.into(),
            wasm_args,
            extra_host_args,
        )
    }

    /// Translates an invokation of a host function and interpret the result.
    ///
    /// This is intended to be a relatively narrow waist which most intrinsics
    /// go through. The configuration supported here is:
    ///
    /// * `host_callee` - what's being called, either a libcall or a lowered
    ///   function
    /// * `host_result` - how to interpret the return value to see if it's a
    ///   trap
    /// * `wasm_args` - how to pass wasm args to the host, either in registers
    ///   or on the stack
    /// * `extra_host_args` - a closure used to push extra arguments just before
    ///   the wasm arguments are forwarded.
    fn translate_hostcall(
        &mut self,
        host_callee: HostCallee,
        host_result: impl Into<HostResult>,
        wasm_args: WasmArgs,
        extra_host_args: impl FnOnce(&mut Self, &mut Vec<ir::Value>),
    ) {
        let pointer_type = self.isa.pointer_type();
        let wasm_func_ty = self.types[self.signature].unwrap_func();

        // Load all parameters in an ABI-agnostic fashion, of which the
        // `VMComponentContext` will be the first.
        let params = self.abi_load_params();
        let vmctx = params[0];
        let wasm_params = &params[2..];

        // Start building up arguments to the host. The first is always the
        // vmctx. After is whatever `extra_host_args` appends, and then finally
        // is what `WasmArgs` specifies.
        let mut host_args = vec![vmctx];
        extra_host_args(self, &mut host_args);
        let mut val_raw_ptr = None;
        let mut val_raw_len = None;
        match wasm_args {
            // Wasm params are passed through as values themselves.
            WasmArgs::InRegisters => host_args.extend(wasm_params.iter().copied()),

            // Wasm params are spilled and then the ptr/len is passed.
            WasmArgs::ValRawList => {
                let (ptr, len) = self.store_wasm_arguments(wasm_params);
                val_raw_ptr = Some(ptr);
                val_raw_len = Some(len);
                host_args.push(ptr);
                host_args.push(len);
            }

            // A mixture of the above two.
            WasmArgs::InRegistersUpTo(n) => {
                let (values_vec_ptr, len) = self.compiler.allocate_stack_array_and_spill_args(
                    &WasmFuncType::new(
                        wasm_func_ty.params().iter().skip(n).copied().collect(),
                        Box::new([]),
                    ),
                    &mut self.builder,
                    &wasm_params[n..],
                );
                let values_vec_len = self.builder.ins().iconst(pointer_type, i64::from(len));

                host_args.extend(wasm_params[..n].iter().copied());
                host_args.push(values_vec_ptr);
                host_args.push(values_vec_len);
            }
        }

        // Next perform the actual invocation of the host with `host_args`.
        let call = match host_callee {
            HostCallee::Libcall(get_libcall) => self.call_libcall(vmctx, get_libcall, &host_args),
            HostCallee::Lowering(index) => {
                // Load host function pointer from the vmcontext and then call that
                // indirect function pointer with the list of arguments.
                let host_fn = self.builder.ins().load(
                    pointer_type,
                    MemFlags::trusted(),
                    vmctx,
                    i32::try_from(self.offsets.lowering_callee(index)).unwrap(),
                );
                let host_sig = {
                    let mut sig = ir::Signature::new(CallConv::triple_default(self.isa.triple()));
                    for param in host_args.iter() {
                        let ty = self.builder.func.dfg.value_type(*param);
                        sig.params.push(ir::AbiParam::new(ty));
                    }
                    // return value is a bool whether a trap was raised or not
                    sig.returns.push(ir::AbiParam::new(ir::types::I8));
                    self.builder.import_signature(sig)
                };
                self.compiler.call_indirect_host(
                    &mut self.builder,
                    HostCall::ComponentLowerImport,
                    host_sig,
                    host_fn,
                    &host_args,
                )
            }
        };

        // Acquire the result of this function (if any) and interpret it
        // according to `host_result`.
        //
        // NOte that all match arms here end with `abi_store_results` which
        // accounts for the ABI of this function when storing results.
        let result = self.builder.func.dfg.inst_results(call).get(0).copied();
        let result_ty = result.map(|v| self.builder.func.dfg.value_type(v));
        let expected = wasm_func_ty.returns();
        match host_result.into() {
            HostResult::Sentinel(TrapSentinel::NegativeOne) => {
                assert_eq!(expected.len(), 1);
                let (result, result_ty) = (result.unwrap(), result_ty.unwrap());
                let result = match (result_ty, expected[0]) {
                    (ir::types::I64, WasmValType::I32) => {
                        self.raise_if_negative_one_and_truncate(result)
                    }
                    (ir::types::I64, WasmValType::I64) | (ir::types::I32, WasmValType::I32) => {
                        self.raise_if_negative_one(result)
                    }
                    other => panic!("unsupported NegativeOne combo {other:?}"),
                };
                self.abi_store_results(&[result]);
            }
            HostResult::Sentinel(TrapSentinel::Falsy) => {
                assert_eq!(expected.len(), 0);
                self.raise_if_host_trapped(result.unwrap());
                self.abi_store_results(&[]);
            }
            HostResult::Sentinel(_) => todo!("support additional return types if/when necessary"),
            HostResult::None => {
                assert!(result.is_none());
                self.abi_store_results(&[]);
            }

            HostResult::MultiValue { ptr, len } => {
                let ptr = ptr.or(val_raw_ptr).unwrap();
                let len = len.or(val_raw_len).unwrap();
                self.raise_if_host_trapped(result.unwrap());
                let results = self.compiler.load_values_from_array(
                    wasm_func_ty.returns(),
                    &mut self.builder,
                    ptr,
                    len,
                );
                self.abi_store_results(&results);
            }
        }
    }

    fn index_value(&mut self, index: impl EntityRef) -> ir::Value {
        self.builder
            .ins()
            .iconst(ir::types::I32, i64::try_from(index.index()).unwrap())
    }

    fn translate_resource_drop(&mut self, resource: TypeResourceTableIndex) {
        let args = self.abi_load_params();
        let vmctx = args[0];
        let caller_vmctx = args[1];
        let pointer_type = self.isa.pointer_type();

        // The arguments this shim passes along to the libcall are:
        //
        //   * the vmctx
        //   * a constant value for this `ResourceDrop` intrinsic
        //   * the wasm handle index to drop
        let mut host_args = Vec::new();
        host_args.push(vmctx);
        host_args.push(
            self.builder
                .ins()
                .iconst(ir::types::I32, i64::from(resource.as_u32())),
        );
        host_args.push(args[2]);

        let call = self.call_libcall(vmctx, host::resource_drop, &host_args);

        // Immediately raise a trap if requested by the host
        let should_run_destructor =
            self.raise_if_negative_one(self.builder.func.dfg.inst_results(call)[0]);

        let resource_ty = self.types[resource].ty;
        let resource_def = self
            .component
            .defined_resource_index(resource_ty)
            .map(|idx| {
                self.component
                    .initializers
                    .iter()
                    .filter_map(|i| match i {
                        GlobalInitializer::Resource(r) if r.index == idx => Some(r),
                        _ => None,
                    })
                    .next()
                    .unwrap()
            });
        let has_destructor = match resource_def {
            Some(def) => def.dtor.is_some(),
            None => true,
        };
        // Synthesize the following:
        //
        //      ...
        //      brif should_run_destructor, run_destructor_block, return_block
        //
        //    run_destructor_block:
        //      ;; test may_enter, but only if the component instances
        //      ;; differ
        //      flags = load.i32 vmctx+$offset
        //      masked = band flags, $FLAG_MAY_ENTER
        //      trapz masked, CANNOT_ENTER_CODE
        //
        //      ;; ============================================================
        //      ;; this is conditionally emitted based on whether the resource
        //      ;; has a destructor or not, and can be statically omitted
        //      ;; because that information is known at compile time here.
        //      rep = ushr.i64 rep, 1
        //      rep = ireduce.i32 rep
        //      dtor = load.ptr vmctx+$offset
        //      func_addr = load.ptr dtor+$offset
        //      callee_vmctx = load.ptr dtor+$offset
        //      call_indirect func_addr, callee_vmctx, vmctx, rep
        //      ;; ============================================================
        //
        //      jump return_block
        //
        //    return_block:
        //      return
        //
        // This will decode `should_run_destructor` and run the destructor
        // funcref if one is specified for this resource. Note that not all
        // resources have destructors, hence the null check.
        self.builder.ensure_inserted_block();
        let current_block = self.builder.current_block().unwrap();
        let run_destructor_block = self.builder.create_block();
        self.builder
            .insert_block_after(run_destructor_block, current_block);
        let return_block = self.builder.create_block();
        self.builder
            .insert_block_after(return_block, run_destructor_block);

        self.builder.ins().brif(
            should_run_destructor,
            run_destructor_block,
            &[],
            return_block,
            &[],
        );

        let trusted = ir::MemFlags::trusted().with_readonly();

        self.builder.switch_to_block(run_destructor_block);

        // If this is a defined resource within the component itself then a
        // check needs to be emitted for the `may_enter` flag. Note though
        // that this check can be elided if the resource table resides in
        // the same component instance that defined the resource as the
        // component is calling itself.
        if let Some(def) = resource_def {
            if self.types[resource].instance != def.instance {
                let flags = self.builder.ins().load(
                    ir::types::I32,
                    trusted,
                    vmctx,
                    i32::try_from(self.offsets.instance_flags(def.instance)).unwrap(),
                );
                let masked = self
                    .builder
                    .ins()
                    .band_imm(flags, i64::from(FLAG_MAY_ENTER));
                self.builder.ins().trapz(masked, TRAP_CANNOT_ENTER);
            }
        }

        // Conditionally emit destructor-execution code based on whether we
        // statically know that a destructor exists or not.
        if has_destructor {
            let rep = self.builder.ins().ushr_imm(should_run_destructor, 1);
            let rep = self.builder.ins().ireduce(ir::types::I32, rep);
            let index = self.types[resource].ty;
            // NB: despite the vmcontext storing nullable funcrefs for function
            // pointers we know this is statically never null due to the
            // `has_destructor` check above.
            let dtor_func_ref = self.builder.ins().load(
                pointer_type,
                trusted,
                vmctx,
                i32::try_from(self.offsets.resource_destructor(index)).unwrap(),
            );
            if cfg!(debug_assertions) {
                self.builder
                    .ins()
                    .trapz(dtor_func_ref, TRAP_INTERNAL_ASSERT);
            }
            let func_addr = self.builder.ins().load(
                pointer_type,
                trusted,
                dtor_func_ref,
                i32::from(self.offsets.ptr.vm_func_ref_wasm_call()),
            );
            let callee_vmctx = self.builder.ins().load(
                pointer_type,
                trusted,
                dtor_func_ref,
                i32::from(self.offsets.ptr.vm_func_ref_vmctx()),
            );

            let sig = crate::wasm_call_signature(
                self.isa,
                &self.types[self.signature].unwrap_func(),
                &self.compiler.tunables,
            );
            let sig_ref = self.builder.import_signature(sig);

            // NB: note that the "caller" vmctx here is the caller of this
            // intrinsic itself, not the `VMComponentContext`. This effectively
            // takes ourselves out of the chain here but that's ok since the
            // caller is only used for store/limits and that same info is
            // stored, but elsewhere, in the component context.
            self.builder.ins().call_indirect(
                sig_ref,
                func_addr,
                &[callee_vmctx, caller_vmctx, rep],
            );
        }
        self.builder.ins().jump(return_block, &[]);
        self.builder.seal_block(run_destructor_block);

        self.builder.switch_to_block(return_block);
        self.builder.seal_block(return_block);
        self.abi_store_results(&[]);
    }

    fn load_optional_memory(
        &mut self,
        vmctx: ir::Value,
        memory: Option<RuntimeMemoryIndex>,
    ) -> ir::Value {
        match memory {
            Some(idx) => self.load_memory(vmctx, idx),
            None => self.builder.ins().iconst(self.isa.pointer_type(), 0),
        }
    }

    fn load_memory(&mut self, vmctx: ir::Value, memory: RuntimeMemoryIndex) -> ir::Value {
        self.builder.ins().load(
            self.isa.pointer_type(),
            MemFlags::trusted(),
            vmctx,
            i32::try_from(self.offsets.runtime_memory(memory)).unwrap(),
        )
    }

    fn load_callback(
        &mut self,
        vmctx: ir::Value,
        callback: Option<RuntimeCallbackIndex>,
    ) -> ir::Value {
        let pointer_type = self.isa.pointer_type();
        match callback {
            Some(idx) => self.builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                vmctx,
                i32::try_from(self.offsets.runtime_callback(idx)).unwrap(),
            ),
            None => self.builder.ins().iconst(pointer_type, 0),
        }
    }

    fn load_post_return(
        &mut self,
        vmctx: ir::Value,
        post_return: Option<RuntimePostReturnIndex>,
    ) -> ir::Value {
        let pointer_type = self.isa.pointer_type();
        match post_return {
            Some(idx) => self.builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                vmctx,
                i32::try_from(self.offsets.runtime_post_return(idx)).unwrap(),
            ),
            None => self.builder.ins().iconst(pointer_type, 0),
        }
    }

    /// Loads a host function pointer for a libcall stored at the `offset`
    /// provided in the libcalls array.
    ///
    /// The offset is calculated in the `host` module below.
    fn load_libcall(
        &mut self,
        vmctx: ir::Value,
        index: ComponentBuiltinFunctionIndex,
    ) -> ir::Value {
        let pointer_type = self.isa.pointer_type();
        // First load the pointer to the builtins structure which is static
        // per-process.
        let builtins_array = self.builder.ins().load(
            pointer_type,
            MemFlags::trusted().with_readonly(),
            vmctx,
            i32::try_from(self.offsets.builtins()).unwrap(),
        );
        // Next load the function pointer at `offset` and return that.
        self.builder.ins().load(
            pointer_type,
            MemFlags::trusted().with_readonly(),
            builtins_array,
            i32::try_from(index.index() * u32::from(self.offsets.ptr.size())).unwrap(),
        )
    }

    fn abi_load_params(&mut self) -> Vec<ir::Value> {
        let mut block0_params = self.builder.func.dfg.block_params(self.block0).to_vec();
        match self.abi {
            // Wasm and native ABIs pass parameters as normal function
            // parameters.
            Abi::Wasm => block0_params,

            // The array ABI passes a pointer/length as the 3rd/4th arguments
            // and those are used to load the actual wasm parameters.
            Abi::Array => {
                let results = self.compiler.load_values_from_array(
                    self.types[self.signature].unwrap_func().params(),
                    &mut self.builder,
                    block0_params[2],
                    block0_params[3],
                );
                block0_params.truncate(2);
                block0_params.extend(results);
                block0_params
            }
        }
    }

    fn abi_store_results(&mut self, results: &[ir::Value]) {
        match self.abi {
            // Wasm/native ABIs return values as usual.
            Abi::Wasm => {
                self.builder.ins().return_(results);
            }

            // The array ABI stores all results in the pointer/length passed
            // as arguments to this function, which contractually are required
            // to have enough space for the results.
            Abi::Array => {
                let block0_params = self.builder.func.dfg.block_params(self.block0);
                let (ptr, len) = (block0_params[2], block0_params[3]);
                self.compiler.store_values_to_array(
                    &mut self.builder,
                    self.types[self.signature].unwrap_func().returns(),
                    results,
                    ptr,
                    len,
                );
                let true_value = self.builder.ins().iconst(ir::types::I8, 1);
                self.builder.ins().return_(&[true_value]);
            }
        }
    }

    fn raise_if_host_trapped(&mut self, succeeded: ir::Value) {
        let caller_vmctx = self.builder.func.dfg.block_params(self.block0)[1];
        self.compiler
            .raise_if_host_trapped(&mut self.builder, caller_vmctx, succeeded);
    }

    fn raise_if_transcode_trapped(&mut self, amount_copied: ir::Value) {
        let pointer_type = self.isa.pointer_type();
        let minus_one = self.builder.ins().iconst(pointer_type, -1);
        let succeeded = self
            .builder
            .ins()
            .icmp(IntCC::NotEqual, amount_copied, minus_one);
        self.raise_if_host_trapped(succeeded);
    }

    fn raise_if_negative_one_and_truncate(&mut self, ret: ir::Value) -> ir::Value {
        let ret = self.raise_if_negative_one(ret);
        self.builder.ins().ireduce(ir::types::I32, ret)
    }

    fn raise_if_negative_one(&mut self, ret: ir::Value) -> ir::Value {
        let result_ty = self.builder.func.dfg.value_type(ret);
        let minus_one = self.builder.ins().iconst(result_ty, -1);
        let succeeded = self.builder.ins().icmp(IntCC::NotEqual, ret, minus_one);
        self.raise_if_host_trapped(succeeded);
        ret
    }

    fn call_libcall(
        &mut self,
        vmctx: ir::Value,
        get_libcall: GetLibcallFn,
        args: &[ir::Value],
    ) -> ir::Inst {
        let (host_sig, index) = get_libcall(self.isa, &mut self.builder.func);
        let host_fn = self.load_libcall(vmctx, index);
        self.compiler
            .call_indirect_host(&mut self.builder, index, host_sig, host_fn, args)
    }
}

impl ComponentCompiler for Compiler {
    fn compile_trampoline(
        &self,
        component: &ComponentTranslation,
        types: &ComponentTypesBuilder,
        index: TrampolineIndex,
        tunables: &Tunables,
        _symbol: &str,
    ) -> Result<AllCallFunc<CompiledFunctionBody>> {
        let compile = |abi: Abi| -> Result<_> {
            let mut compiler = self.function_compiler();
            let mut c = TrampolineCompiler::new(
                self,
                &mut compiler,
                &component.component,
                types,
                index,
                abi,
                tunables,
            );

            // If we are crossing the Wasm-to-native boundary, we need to save the
            // exit FP and return address for stack walking purposes. However, we
            // always debug assert that our vmctx is a component context, regardless
            // whether we are actually crossing that boundary because it should
            // always hold.
            let vmctx = c.builder.block_params(c.block0)[0];
            let pointer_type = self.isa.pointer_type();
            super::debug_assert_vmctx_kind(
                &*self.isa,
                &mut c.builder,
                vmctx,
                wasmtime_environ::component::VMCOMPONENT_MAGIC,
            );
            if let Abi::Wasm = abi {
                let vm_store_context = c.builder.ins().load(
                    pointer_type,
                    MemFlags::trusted(),
                    vmctx,
                    i32::try_from(c.offsets.vm_store_context()).unwrap(),
                );
                super::save_last_wasm_exit_fp_and_pc(
                    &mut c.builder,
                    pointer_type,
                    &c.offsets.ptr,
                    vm_store_context,
                );
            }

            c.translate(&component.trampolines[index]);
            c.builder.finalize();
            compiler.cx.abi = Some(abi);

            Ok(CompiledFunctionBody {
                code: Box::new(Some(compiler.cx)),
                needs_gc_heap: false,
            })
        };

        Ok(AllCallFunc {
            wasm_call: compile(Abi::Wasm)?,
            array_call: compile(Abi::Array)?,
        })
    }
}

impl TrampolineCompiler<'_> {
    fn translate_transcode(
        &mut self,
        op: Transcode,
        from: RuntimeMemoryIndex,
        from64: bool,
        to: RuntimeMemoryIndex,
        to64: bool,
    ) {
        let pointer_type = self.isa.pointer_type();
        let vmctx = self.builder.func.dfg.block_params(self.block0)[0];

        // Determine the static signature of the host libcall for this transcode
        // operation and additionally calculate the static offset within the
        // transode libcalls array.
        let get_libcall = match op {
            Transcode::Copy(FixedEncoding::Utf8) => host::utf8_to_utf8,
            Transcode::Copy(FixedEncoding::Utf16) => host::utf16_to_utf16,
            Transcode::Copy(FixedEncoding::Latin1) => host::latin1_to_latin1,
            Transcode::Latin1ToUtf16 => host::latin1_to_utf16,
            Transcode::Latin1ToUtf8 => host::latin1_to_utf8,
            Transcode::Utf16ToCompactProbablyUtf16 => host::utf16_to_compact_probably_utf16,
            Transcode::Utf16ToCompactUtf16 => host::utf16_to_compact_utf16,
            Transcode::Utf16ToLatin1 => host::utf16_to_latin1,
            Transcode::Utf16ToUtf8 => host::utf16_to_utf8,
            Transcode::Utf8ToCompactUtf16 => host::utf8_to_compact_utf16,
            Transcode::Utf8ToLatin1 => host::utf8_to_latin1,
            Transcode::Utf8ToUtf16 => host::utf8_to_utf16,
        };

        // Load the base pointers for the from/to linear memories.
        let from_base = self.load_runtime_memory_base(vmctx, from);
        let to_base = self.load_runtime_memory_base(vmctx, to);

        let mut args = Vec::new();
        args.push(vmctx);

        let uses_retptr = match op {
            Transcode::Utf16ToUtf8
            | Transcode::Latin1ToUtf8
            | Transcode::Utf8ToLatin1
            | Transcode::Utf16ToLatin1 => true,
            _ => false,
        };

        // Most transcoders share roughly the same signature despite doing very
        // different things internally, so most libcalls are lumped together
        // here.
        match op {
            Transcode::Copy(_)
            | Transcode::Latin1ToUtf16
            | Transcode::Utf16ToCompactProbablyUtf16
            | Transcode::Utf8ToLatin1
            | Transcode::Utf16ToLatin1
            | Transcode::Utf8ToUtf16 => {
                args.push(self.ptr_param(0, from64, from_base));
                args.push(self.len_param(1, from64));
                args.push(self.ptr_param(2, to64, to_base));
            }

            Transcode::Utf16ToUtf8 | Transcode::Latin1ToUtf8 => {
                args.push(self.ptr_param(0, from64, from_base));
                args.push(self.len_param(1, from64));
                args.push(self.ptr_param(2, to64, to_base));
                args.push(self.len_param(3, to64));
            }

            Transcode::Utf8ToCompactUtf16 | Transcode::Utf16ToCompactUtf16 => {
                args.push(self.ptr_param(0, from64, from_base));
                args.push(self.len_param(1, from64));
                args.push(self.ptr_param(2, to64, to_base));
                args.push(self.len_param(3, to64));
                args.push(self.len_param(4, to64));
            }
        };
        if uses_retptr {
            let slot = self
                .builder
                .func
                .create_sized_stack_slot(ir::StackSlotData::new(
                    ir::StackSlotKind::ExplicitSlot,
                    pointer_type.bytes(),
                    0,
                ));
            args.push(self.builder.ins().stack_addr(pointer_type, slot, 0));
        }
        let call = self.call_libcall(vmctx, get_libcall, &args);
        let mut results = self.builder.func.dfg.inst_results(call).to_vec();
        if uses_retptr {
            results.push(self.builder.ins().load(
                pointer_type,
                ir::MemFlags::trusted(),
                *args.last().unwrap(),
                0,
            ));
        }
        let mut raw_results = Vec::new();

        // Like the arguments the results are fairly similar across libcalls, so
        // they're lumped into various buckets here.
        match op {
            Transcode::Copy(_) | Transcode::Latin1ToUtf16 => {
                self.raise_if_host_trapped(results[0]);
            }

            Transcode::Utf8ToUtf16
            | Transcode::Utf16ToCompactProbablyUtf16
            | Transcode::Utf8ToCompactUtf16
            | Transcode::Utf16ToCompactUtf16 => {
                self.raise_if_transcode_trapped(results[0]);
                raw_results.push(self.cast_from_pointer(results[0], to64));
            }

            Transcode::Latin1ToUtf8
            | Transcode::Utf16ToUtf8
            | Transcode::Utf8ToLatin1
            | Transcode::Utf16ToLatin1 => {
                self.raise_if_transcode_trapped(results[0]);
                raw_results.push(self.cast_from_pointer(results[0], from64));
                raw_results.push(self.cast_from_pointer(results[1], to64));
            }
        };

        self.builder.ins().return_(&raw_results);
    }

    // Helper function to cast an input parameter to the host pointer type.
    fn len_param(&mut self, param: usize, is64: bool) -> ir::Value {
        let val = self.builder.func.dfg.block_params(self.block0)[2 + param];
        self.cast_to_pointer(val, is64)
    }

    // Helper function to interpret an input parameter as a pointer into
    // linear memory. This will cast the input parameter to the host integer
    // type and then add that value to the base.
    //
    // Note that bounds-checking happens in adapter modules, and this
    // trampoline is simply calling the host libcall.
    fn ptr_param(&mut self, param: usize, is64: bool, base: ir::Value) -> ir::Value {
        let val = self.len_param(param, is64);
        self.builder.ins().iadd(base, val)
    }

    // Helper function to cast a core wasm input to a host pointer type
    // which will go into the host libcall.
    fn cast_to_pointer(&mut self, val: ir::Value, is64: bool) -> ir::Value {
        let pointer_type = self.isa.pointer_type();
        let host64 = pointer_type == ir::types::I64;
        if is64 == host64 {
            val
        } else if !is64 {
            assert!(host64);
            self.builder.ins().uextend(pointer_type, val)
        } else {
            assert!(!host64);
            self.builder.ins().ireduce(pointer_type, val)
        }
    }

    // Helper to cast a host pointer integer type to the destination type.
    fn cast_from_pointer(&mut self, val: ir::Value, is64: bool) -> ir::Value {
        let host64 = self.isa.pointer_type() == ir::types::I64;
        if is64 == host64 {
            val
        } else if !is64 {
            assert!(host64);
            self.builder.ins().ireduce(ir::types::I32, val)
        } else {
            assert!(!host64);
            self.builder.ins().uextend(ir::types::I64, val)
        }
    }

    fn load_runtime_memory_base(&mut self, vmctx: ir::Value, mem: RuntimeMemoryIndex) -> ir::Value {
        let pointer_type = self.isa.pointer_type();
        let from_vmmemory_definition = self.load_memory(vmctx, mem);
        self.builder.ins().load(
            pointer_type,
            MemFlags::trusted(),
            from_vmmemory_definition,
            i32::from(self.offsets.ptr.vmmemory_definition_base()),
        )
    }
}

/// Module with macro-generated contents that will return the signature and
/// offset for each of the host transcoder functions.
///
/// Note that a macro is used here to keep this in sync with the actual
/// transcoder functions themselves which are also defined via a macro.
mod host {
    use cranelift_codegen::ir::{self, AbiParam};
    use cranelift_codegen::isa::{CallConv, TargetIsa};
    use wasmtime_environ::component::ComponentBuiltinFunctionIndex;

    macro_rules! define {
        (
            $(
                $( #[$attr:meta] )*
                $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
            )*
        ) => {
            $(
                pub(super) fn $name(isa: &dyn TargetIsa, func: &mut ir::Function) -> (ir::SigRef, ComponentBuiltinFunctionIndex) {
                    let pointer_type = isa.pointer_type();
                    let sig = build_sig(
                        isa,
                        func,
                        &[$( define!(@ty pointer_type $param) ),*],
                        &[$( define!(@ty pointer_type $result) ),*],
                    );

                    return (sig, ComponentBuiltinFunctionIndex::$name())
                }
            )*
        };

        (@ty $ptr:ident size) => ($ptr);
        (@ty $ptr:ident ptr_u8) => ($ptr);
        (@ty $ptr:ident ptr_u16) => ($ptr);
        (@ty $ptr:ident ptr_size) => ($ptr);
        (@ty $ptr:ident bool) => (ir::types::I8);
        (@ty $ptr:ident u8) => (ir::types::I8);
        (@ty $ptr:ident u32) => (ir::types::I32);
        (@ty $ptr:ident u64) => (ir::types::I64);
        (@ty $ptr:ident vmctx) => ($ptr);
    }

    wasmtime_environ::foreach_builtin_component_function!(define);

    fn build_sig(
        isa: &dyn TargetIsa,
        func: &mut ir::Function,
        params: &[ir::Type],
        returns: &[ir::Type],
    ) -> ir::SigRef {
        let mut sig = ir::Signature {
            params: params.iter().map(|ty| AbiParam::new(*ty)).collect(),
            returns: returns.iter().map(|ty| AbiParam::new(*ty)).collect(),
            call_conv: CallConv::triple_default(isa.triple()),
        };

        // Once we're declaring the signature of a host function we must respect
        // the default ABI of the platform which is where argument extension of
        // params/results may come into play.
        let extension = isa.default_argument_extension();
        for arg in sig.params.iter_mut().chain(sig.returns.iter_mut()) {
            if arg.value_type.is_int() {
                arg.extension = extension;
            }
        }
        func.import_signature(sig)
    }
}
