//! Low-level compilation of an fused adapter function.
//!
//! This module is tasked with the top-level `compile` function which creates a
//! single WebAssembly function which will perform the steps of the fused
//! adapter for an `AdapterData` provided. This is the "meat" of compilation
//! where the validation of the canonical ABI or similar all happens to
//! translate arguments from one module to another.
//!
//! ## Traps and their ordering
//!
//! Currently this compiler is pretty "loose" about the ordering of precisely
//! what trap happens where. The main reason for this is that to core wasm all
//! traps are the same and for fused adapters if a trap happens no intermediate
//! side effects are visible (as designed by the canonical ABI itself). For this
//! it's important to note that some of the precise choices of control flow here
//! can be somewhat arbitrary, an intentional decision.

use crate::component::{
    CanonicalAbiInfo, ComponentTypesBuilder, FLAG_MAY_ENTER, FLAG_MAY_LEAVE, FixedEncoding as FE,
    FlatType, InterfaceType, MAX_FLAT_ASYNC_PARAMS, MAX_FLAT_PARAMS, PREPARE_ASYNC_NO_RESULT,
    PREPARE_ASYNC_WITH_RESULT, START_FLAG_ASYNC_CALLEE, StringEncoding, Transcode,
    TypeComponentLocalErrorContextTableIndex, TypeEnumIndex, TypeFlagsIndex, TypeFutureTableIndex,
    TypeListIndex, TypeOptionIndex, TypeRecordIndex, TypeResourceTableIndex, TypeResultIndex,
    TypeStreamTableIndex, TypeTupleIndex, TypeVariantIndex, VariantInfo,
};
use crate::fact::signature::Signature;
use crate::fact::transcode::Transcoder;
use crate::fact::traps::Trap;
use crate::fact::{
    AdapterData, Body, Function, FunctionId, Helper, HelperLocation, HelperType,
    LinearMemoryOptions, Module, Options,
};
use crate::prelude::*;
use crate::{FuncIndex, GlobalIndex};
use cranelift_entity::Signed;
use std::collections::HashMap;
use std::mem;
use std::ops::Range;
use wasm_encoder::{BlockType, Encode, Instruction, Instruction::*, MemArg, ValType};
use wasmtime_component_util::{DiscriminantSize, FlagsSize};

use super::DataModel;

const MAX_STRING_BYTE_LENGTH: u32 = 1 << 31;
const UTF16_TAG: u32 = 1 << 31;

/// This value is arbitrarily chosen and should be fine to change at any time,
/// it just seemed like a halfway reasonable starting point.
const INITIAL_FUEL: usize = 1_000;

struct Compiler<'a, 'b> {
    types: &'a ComponentTypesBuilder,
    module: &'b mut Module<'a>,
    result: FunctionId,

    /// The encoded WebAssembly function body so far, not including locals.
    code: Vec<u8>,

    /// Total number of locals generated so far.
    nlocals: u32,

    /// Locals partitioned by type which are not currently in use.
    free_locals: HashMap<ValType, Vec<u32>>,

    /// Metadata about all `unreachable` trap instructions in this function and
    /// what the trap represents. The offset within `self.code` is recorded as
    /// well.
    traps: Vec<(usize, Trap)>,

    /// A heuristic which is intended to limit the size of a generated function
    /// to a certain maximum to avoid generating arbitrarily large functions.
    ///
    /// This fuel counter is decremented each time `translate` is called and
    /// when fuel is entirely consumed further translations, if necessary, will
    /// be done through calls to other functions in the module. This is intended
    /// to be a heuristic to split up the main function into theoretically
    /// reusable portions.
    fuel: usize,

    /// Indicates whether an "enter call" should be emitted in the generated
    /// function with a call to `Resource{Enter,Exit}Call` at the beginning and
    /// end of the function for tracking of information related to borrowed
    /// resources.
    emit_resource_call: bool,
}

pub(super) fn compile(module: &mut Module<'_>, adapter: &AdapterData) {
    fn compiler<'a, 'b>(
        module: &'b mut Module<'a>,
        adapter: &AdapterData,
    ) -> (Compiler<'a, 'b>, Signature, Signature) {
        let lower_sig = module.types.signature(&adapter.lower);
        let lift_sig = module.types.signature(&adapter.lift);
        let ty = module
            .core_types
            .function(&lower_sig.params, &lower_sig.results);
        let result = module
            .funcs
            .push(Function::new(Some(adapter.name.clone()), ty));

        // If this type signature contains any borrowed resources then invocations
        // of enter/exit call for resource-related metadata tracking must be used.
        // It shouldn't matter whether the lower/lift signature is used here as both
        // should return the same answer.
        let emit_resource_call = module.types.contains_borrow_resource(&adapter.lower);
        assert_eq!(
            emit_resource_call,
            module.types.contains_borrow_resource(&adapter.lift)
        );

        (
            Compiler::new(
                module,
                result,
                lower_sig.params.len() as u32,
                emit_resource_call,
            ),
            lower_sig,
            lift_sig,
        )
    }

    // This closure compiles a function to be exported to the host which host to
    // lift the parameters from the caller and lower them to the callee.
    //
    // This allows the host to delay copying the parameters until the callee
    // signals readiness by clearing its backpressure flag.
    let async_start_adapter = |module: &mut Module| {
        let sig = module
            .types
            .async_start_signature(&adapter.lower, &adapter.lift);
        let ty = module.core_types.function(&sig.params, &sig.results);
        let result = module.funcs.push(Function::new(
            Some(format!("[async-start]{}", adapter.name)),
            ty,
        ));

        Compiler::new(module, result, sig.params.len() as u32, false)
            .compile_async_start_adapter(adapter, &sig);

        result
    };

    // This closure compiles a function to be exported by the adapter module and
    // called by the host to lift the results from the callee and lower them to
    // the caller.
    //
    // Given that async-lifted exports return their results via the
    // `task.return` intrinsic, the host will need to copy the results from
    // callee to caller when that intrinsic is called rather than when the
    // callee task fully completes (which may happen much later).
    let async_return_adapter = |module: &mut Module| {
        let sig = module
            .types
            .async_return_signature(&adapter.lower, &adapter.lift);
        let ty = module.core_types.function(&sig.params, &sig.results);
        let result = module.funcs.push(Function::new(
            Some(format!("[async-return]{}", adapter.name)),
            ty,
        ));

        Compiler::new(module, result, sig.params.len() as u32, false)
            .compile_async_return_adapter(adapter, &sig);

        result
    };

    match (adapter.lower.options.async_, adapter.lift.options.async_) {
        (false, false) => {
            // We can adapt sync->sync case with only minimal use of intrinsics,
            // e.g. resource enter and exit calls as needed.
            let (compiler, lower_sig, lift_sig) = compiler(module, adapter);
            compiler.compile_sync_to_sync_adapter(adapter, &lower_sig, &lift_sig)
        }
        (true, true) => {
            // In the async->async case, we must compile a couple of helper functions:
            //
            // - `async-start`: copies the parameters from the caller to the callee
            // - `async-return`: copies the result from the callee to the caller
            //
            // Unlike synchronous calls, the above operations are asynchronous
            // and subject to backpressure.  If the callee is not yet ready to
            // handle a new call, the `async-start` function will not be called
            // immediately.  Instead, control will return to the caller,
            // allowing it to do other work while waiting for this call to make
            // progress.  Once the callee indicates it is ready, `async-start`
            // will be called, and sometime later (possibly after various task
            // switch events), when the callee has produced a result, it will
            // call `async-return` via the `task.return` intrinsic, at which
            // point a `STATUS_RETURNED` event will be delivered to the caller.
            let start = async_start_adapter(module);
            let return_ = async_return_adapter(module);
            let (compiler, lower_sig, lift_sig) = compiler(module, adapter);
            compiler.compile_async_to_async_adapter(
                adapter,
                start,
                return_,
                i32::try_from(lift_sig.params.len()).unwrap(),
                &lower_sig,
            );
        }
        (false, true) => {
            // Like the async->async case above, for the sync->async case we
            // also need `async-start` and `async-return` helper functions to
            // allow the callee to asynchronously "pull" the parameters and
            // "push" the results when it is ready.
            //
            // However, since the caller is using the synchronous ABI, the
            // parameters may have been passed via the stack rather than linear
            // memory.  In that case, we pass them to the host to store in a
            // task-local location temporarily in the case of backpressure.
            // Similarly, the host will also temporarily store the results that
            // the callee provides to `async-return` until it is ready to resume
            // the caller.
            let start = async_start_adapter(module);
            let return_ = async_return_adapter(module);
            let (compiler, lower_sig, lift_sig) = compiler(module, adapter);
            compiler.compile_sync_to_async_adapter(
                adapter,
                start,
                return_,
                i32::try_from(lift_sig.params.len()).unwrap(),
                &lower_sig,
            );
        }
        (true, false) => {
            // As with the async->async and sync->async cases above, for the
            // async->sync case we use `async-start` and `async-return` helper
            // functions.  Here, those functions allow the host to enforce
            // backpressure in the case where the callee instance already has
            // another synchronous call in progress, in which case we can't
            // start a new one until the current one (and any others already
            // waiting in line behind it) has completed.
            //
            // In the case of backpressure, we'll return control to the caller
            // immediately so it can do other work.  Later, once the callee is
            // ready, the host will call the `async-start` function to retrieve
            // the parameters and pass them to the callee.  At that point, the
            // callee may block on a host call, at which point the host will
            // suspend the fiber it is running on and allow the caller (or any
            // other ready instance) to run concurrently with the blocked
            // callee.  Once the callee finally returns, the host will call the
            // `async-return` function to write the result to the caller's
            // linear memory and deliver a `STATUS_RETURNED` event to the
            // caller.
            let lift_sig = module.types.signature(&adapter.lift);
            let start = async_start_adapter(module);
            let return_ = async_return_adapter(module);
            let (compiler, lower_sig, ..) = compiler(module, adapter);
            compiler.compile_async_to_sync_adapter(
                adapter,
                start,
                return_,
                i32::try_from(lift_sig.params.len()).unwrap(),
                i32::try_from(lift_sig.results.len()).unwrap(),
                &lower_sig,
            );
        }
    }
}

/// Compiles a helper function as specified by the `Helper` configuration.
///
/// This function is invoked when the translation process runs out of fuel for
/// some prior function which enqueues a helper to get translated later. This
/// translation function will perform one type translation as specified by
/// `Helper` which can either be in the stack or memory for each side.
pub(super) fn compile_helper(module: &mut Module<'_>, result: FunctionId, helper: Helper) {
    let mut nlocals = 0;
    let src_flat;
    let src = match helper.src.loc {
        // If the source is on the stack then it's specified in the parameters
        // to the function, so this creates the flattened representation and
        // then lists those as the locals with appropriate types for the source
        // values.
        HelperLocation::Stack => {
            src_flat = module
                .types
                .flatten_types(&helper.src.opts, usize::MAX, [helper.src.ty])
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, ty)| (i as u32, *ty))
                .collect::<Vec<_>>();
            nlocals += src_flat.len() as u32;
            Source::Stack(Stack {
                locals: &src_flat,
                opts: &helper.src.opts,
            })
        }
        // If the source is in memory then that's just propagated here as the
        // first local is the pointer to the source.
        HelperLocation::Memory => {
            nlocals += 1;
            Source::Memory(Memory {
                opts: &helper.src.opts,
                addr: TempLocal::new(0, helper.src.opts.data_model.unwrap_memory().ptr()),
                offset: 0,
            })
        }
        HelperLocation::StructField | HelperLocation::ArrayElement => todo!("CM+GC"),
    };
    let dst_flat;
    let dst = match helper.dst.loc {
        // This is the same as the stack-based source although `Destination` is
        // configured slightly differently.
        HelperLocation::Stack => {
            dst_flat = module
                .types
                .flatten_types(&helper.dst.opts, usize::MAX, [helper.dst.ty])
                .unwrap();
            Destination::Stack(&dst_flat, &helper.dst.opts)
        }
        // This is the same as a memory-based source but note that the address
        // of the destination is passed as the final parameter to the function.
        HelperLocation::Memory => {
            nlocals += 1;
            Destination::Memory(Memory {
                opts: &helper.dst.opts,
                addr: TempLocal::new(
                    nlocals - 1,
                    helper.dst.opts.data_model.unwrap_memory().ptr(),
                ),
                offset: 0,
            })
        }
        HelperLocation::StructField | HelperLocation::ArrayElement => todo!("CM+GC"),
    };
    let mut compiler = Compiler {
        types: module.types,
        module,
        code: Vec::new(),
        nlocals,
        free_locals: HashMap::new(),
        traps: Vec::new(),
        result,
        fuel: INITIAL_FUEL,
        // This is a helper function and only the top-level function is
        // responsible for emitting these intrinsic calls.
        emit_resource_call: false,
    };
    compiler.translate(&helper.src.ty, &src, &helper.dst.ty, &dst);
    compiler.finish();
}

/// Possible ways that a interface value is represented in the core wasm
/// canonical ABI.
enum Source<'a> {
    /// This value is stored on the "stack" in wasm locals.
    ///
    /// This could mean that it's inline from the parameters to the function or
    /// that after a function call the results were stored in locals and the
    /// locals are the inline results.
    Stack(Stack<'a>),

    /// This value is stored in linear memory described by the `Memory`
    /// structure.
    Memory(Memory<'a>),

    /// This value is stored in a GC struct field described by the `GcStruct`
    /// structure.
    #[allow(dead_code, reason = "CM+GC is still WIP")]
    Struct(GcStruct<'a>),

    /// This value is stored in a GC array element described by the `GcArray`
    /// structure.
    #[allow(dead_code, reason = "CM+GC is still WIP")]
    Array(GcArray<'a>),
}

/// Same as `Source` but for where values are translated into.
enum Destination<'a> {
    /// This value is destined for the WebAssembly stack which means that
    /// results are simply pushed as we go along.
    ///
    /// The types listed are the types that are expected to be on the stack at
    /// the end of translation.
    Stack(&'a [ValType], &'a Options),

    /// This value is to be placed in linear memory described by `Memory`.
    Memory(Memory<'a>),

    /// This value is to be placed in a GC struct field described by the
    /// `GcStruct` structure.
    #[allow(dead_code, reason = "CM+GC is still WIP")]
    Struct(GcStruct<'a>),

    /// This value is to be placed in a GC array element described by the
    /// `GcArray` structure.
    #[allow(dead_code, reason = "CM+GC is still WIP")]
    Array(GcArray<'a>),
}

struct Stack<'a> {
    /// The locals that comprise a particular value.
    ///
    /// The length of this list represents the flattened list of types that make
    /// up the component value. Each list has the index of the local being
    /// accessed as well as the type of the local itself.
    locals: &'a [(u32, ValType)],
    /// The lifting/lowering options for where this stack of values comes from
    opts: &'a Options,
}

/// Representation of where a value is going to be stored in linear memory.
struct Memory<'a> {
    /// The lifting/lowering options with memory configuration
    opts: &'a Options,
    /// The index of the local that contains the base address of where the
    /// storage is happening.
    addr: TempLocal,
    /// A "static" offset that will be baked into wasm instructions for where
    /// memory loads/stores happen.
    offset: u32,
}

impl<'a> Memory<'a> {
    fn mem_opts(&self) -> &'a LinearMemoryOptions {
        self.opts.data_model.unwrap_memory()
    }
}

/// Representation of where a value is coming from or going to in a GC struct.
struct GcStruct<'a> {
    opts: &'a Options,
    // TODO: more fields to come in the future.
}

/// Representation of where a value is coming from or going to in a GC array.
struct GcArray<'a> {
    opts: &'a Options,
    // TODO: more fields to come in the future.
}

impl<'a, 'b> Compiler<'a, 'b> {
    fn new(
        module: &'b mut Module<'a>,
        result: FunctionId,
        nlocals: u32,
        emit_resource_call: bool,
    ) -> Self {
        Self {
            types: module.types,
            module,
            result,
            code: Vec::new(),
            nlocals,
            free_locals: HashMap::new(),
            traps: Vec::new(),
            fuel: INITIAL_FUEL,
            emit_resource_call,
        }
    }

    /// Compile an adapter function supporting an async-lowered import to an
    /// async-lifted export.
    ///
    /// This uses a pair of `async-prepare` and `async-start` built-in functions
    /// to set up and start a subtask, respectively.  `async-prepare` accepts
    /// `start` and `return_` functions which copy the parameters and results,
    /// respectively; the host will call the former when the callee has cleared
    /// its backpressure flag and the latter when the callee has called
    /// `task.return`.
    fn compile_async_to_async_adapter(
        mut self,
        adapter: &AdapterData,
        start: FunctionId,
        return_: FunctionId,
        param_count: i32,
        lower_sig: &Signature,
    ) {
        let start_call =
            self.module
                .import_async_start_call(&adapter.name, adapter.lift.options.callback, None);

        self.call_prepare(adapter, start, return_, lower_sig, false);

        // TODO: As an optimization, consider checking the backpressure flag on
        // the callee instance and, if it's unset _and_ the callee uses a
        // callback, translate the params and call the callee function directly
        // here (and make sure `start_call` knows _not_ to call it in that case).

        // We export this function so we can pass a funcref to the host.
        //
        // TODO: Use a declarative element segment instead of exporting this.
        self.module.exports.push((
            adapter.callee.as_u32(),
            format!("[adapter-callee]{}", adapter.name),
        ));

        self.instruction(RefFunc(adapter.callee.as_u32()));
        self.instruction(I32Const(param_count));
        // The result count for an async callee is either one (if there's a
        // callback) or zero (if there's no callback).  We conservatively use
        // one here to ensure the host provides room for the result, if any.
        self.instruction(I32Const(1));
        self.instruction(I32Const(START_FLAG_ASYNC_CALLEE));
        self.instruction(Call(start_call.as_u32()));

        self.finish()
    }

    /// Invokes the `prepare_call` builtin with the provided parameters for this
    /// adapter.
    ///
    /// This is part of a async lower and/or async lift adapter. This is not
    /// used for a sync->sync function call. This is done to create the task on
    /// the host side of the runtime and such. This will notably invoke a
    /// Cranelift builtin which will spill all wasm-level parameters to the
    /// stack to handle variadic signatures.
    ///
    /// Note that the `prepare_sync` parameter here configures the
    /// `result_count_or_max_if_async` parameter to indicate whether this is a
    /// sync or async prepare.
    fn call_prepare(
        &mut self,
        adapter: &AdapterData,
        start: FunctionId,
        return_: FunctionId,
        lower_sig: &Signature,
        prepare_sync: bool,
    ) {
        let prepare = self.module.import_prepare_call(
            &adapter.name,
            &lower_sig.params,
            match adapter.lift.options.data_model {
                DataModel::Gc {} => todo!("CM+GC"),
                DataModel::LinearMemory(LinearMemoryOptions { memory, .. }) => memory,
            },
        );

        self.flush_code();
        self.module.funcs[self.result]
            .body
            .push(Body::RefFunc(start));
        self.module.funcs[self.result]
            .body
            .push(Body::RefFunc(return_));
        self.instruction(I32Const(
            i32::try_from(adapter.lower.instance.as_u32()).unwrap(),
        ));
        self.instruction(I32Const(
            i32::try_from(adapter.lift.instance.as_u32()).unwrap(),
        ));
        self.instruction(I32Const(
            i32::try_from(self.types[adapter.lift.ty].results.as_u32()).unwrap(),
        ));
        self.instruction(I32Const(i32::from(
            adapter.lift.options.string_encoding as u8,
        )));

        // flag this as a preparation for either an async call or sync call,
        // depending on `prepare_sync`
        let result_types = &self.types[self.types[adapter.lower.ty].results].types;
        if prepare_sync {
            self.instruction(I32Const(
                i32::try_from(
                    self.types
                        .flatten_types(
                            &adapter.lower.options,
                            usize::MAX,
                            result_types.iter().copied(),
                        )
                        .map(|v| v.len())
                        .unwrap_or(usize::try_from(i32::MAX).unwrap()),
                )
                .unwrap(),
            ));
        } else {
            if result_types.len() > 0 {
                self.instruction(I32Const(PREPARE_ASYNC_WITH_RESULT.signed()));
            } else {
                self.instruction(I32Const(PREPARE_ASYNC_NO_RESULT.signed()));
            }
        }

        // forward all our own arguments on to the host stub
        for index in 0..lower_sig.params.len() {
            self.instruction(LocalGet(u32::try_from(index).unwrap()));
        }
        self.instruction(Call(prepare.as_u32()));
    }

    /// Compile an adapter function supporting a sync-lowered import to an
    /// async-lifted export.
    ///
    /// This uses a pair of `sync-prepare` and `sync-start` built-in functions
    /// to set up and start a subtask, respectively.  `sync-prepare` accepts
    /// `start` and `return_` functions which copy the parameters and results,
    /// respectively; the host will call the former when the callee has cleared
    /// its backpressure flag and the latter when the callee has called
    /// `task.return`.
    fn compile_sync_to_async_adapter(
        mut self,
        adapter: &AdapterData,
        start: FunctionId,
        return_: FunctionId,
        lift_param_count: i32,
        lower_sig: &Signature,
    ) {
        let start_call = self.module.import_sync_start_call(
            &adapter.name,
            adapter.lift.options.callback,
            &lower_sig.results,
        );

        self.call_prepare(adapter, start, return_, lower_sig, true);

        // TODO: As an optimization, consider checking the backpressure flag on
        // the callee instance and, if it's unset _and_ the callee uses a
        // callback, translate the params and call the callee function directly
        // here (and make sure `start_call` knows _not_ to call it in that case).

        // We export this function so we can pass a funcref to the host.
        //
        // TODO: Use a declarative element segment instead of exporting this.
        self.module.exports.push((
            adapter.callee.as_u32(),
            format!("[adapter-callee]{}", adapter.name),
        ));

        self.instruction(RefFunc(adapter.callee.as_u32()));
        self.instruction(I32Const(lift_param_count));
        self.instruction(Call(start_call.as_u32()));

        self.finish()
    }

    /// Compile an adapter function supporting an async-lowered import to a
    /// sync-lifted export.
    ///
    /// This uses a pair of `async-prepare` and `async-start` built-in functions
    /// to set up and start a subtask, respectively.  `async-prepare` accepts
    /// `start` and `return_` functions which copy the parameters and results,
    /// respectively; the host will call the former when the callee has cleared
    /// its backpressure flag and the latter when the callee has returned its
    /// result(s).
    fn compile_async_to_sync_adapter(
        mut self,
        adapter: &AdapterData,
        start: FunctionId,
        return_: FunctionId,
        param_count: i32,
        result_count: i32,
        lower_sig: &Signature,
    ) {
        let start_call =
            self.module
                .import_async_start_call(&adapter.name, None, adapter.lift.post_return);

        self.call_prepare(adapter, start, return_, lower_sig, false);

        // We export this function so we can pass a funcref to the host.
        //
        // TODO: Use a declarative element segment instead of exporting this.
        self.module.exports.push((
            adapter.callee.as_u32(),
            format!("[adapter-callee]{}", adapter.name),
        ));

        self.instruction(RefFunc(adapter.callee.as_u32()));
        self.instruction(I32Const(param_count));
        self.instruction(I32Const(result_count));
        self.instruction(I32Const(0));
        self.instruction(Call(start_call.as_u32()));

        self.finish()
    }

    /// Compiles a function to be exported to the host which host to lift the
    /// parameters from the caller and lower them to the callee.
    ///
    /// This allows the host to delay copying the parameters until the callee
    /// signals readiness by clearing its backpressure flag.
    fn compile_async_start_adapter(mut self, adapter: &AdapterData, sig: &Signature) {
        let param_locals = sig
            .params
            .iter()
            .enumerate()
            .map(|(i, ty)| (i as u32, *ty))
            .collect::<Vec<_>>();

        self.set_flag(adapter.lift.flags, FLAG_MAY_LEAVE, false);
        self.translate_params(adapter, &param_locals);
        self.set_flag(adapter.lift.flags, FLAG_MAY_LEAVE, true);

        self.finish();
    }

    /// Compiles a function to be exported by the adapter module and called by
    /// the host to lift the results from the callee and lower them to the
    /// caller.
    ///
    /// Given that async-lifted exports return their results via the
    /// `task.return` intrinsic, the host will need to copy the results from
    /// callee to caller when that intrinsic is called rather than when the
    /// callee task fully completes (which may happen much later).
    fn compile_async_return_adapter(mut self, adapter: &AdapterData, sig: &Signature) {
        let param_locals = sig
            .params
            .iter()
            .enumerate()
            .map(|(i, ty)| (i as u32, *ty))
            .collect::<Vec<_>>();

        self.set_flag(adapter.lower.flags, FLAG_MAY_LEAVE, false);
        // Note that we pass `param_locals` as _both_ the `param_locals` and
        // `result_locals` parameters to `translate_results`.  That's because
        // the _parameters_ to `task.return` are actually the _results_ that the
        // caller is waiting for.
        //
        // Additionally, the host will append a return
        // pointer to the end of that list before calling this adapter's
        // `async-return` function if the results exceed `MAX_FLAT_RESULTS` or
        // the import is lowered async, in which case `translate_results` will
        // use that pointer to store the results.
        self.translate_results(adapter, &param_locals, &param_locals);
        self.set_flag(adapter.lower.flags, FLAG_MAY_LEAVE, true);

        self.finish()
    }

    /// Compile an adapter function supporting a sync-lowered import to a
    /// sync-lifted export.
    ///
    /// Unlike calls involving async-lowered imports or async-lifted exports,
    /// this adapter need not involve host built-ins except possibly for
    /// resource bookkeeping.
    fn compile_sync_to_sync_adapter(
        mut self,
        adapter: &AdapterData,
        lower_sig: &Signature,
        lift_sig: &Signature,
    ) {
        // Check the instance flags required for this trampoline.
        //
        // This inserts the initial check required by `canon_lower` that the
        // caller instance can be left and additionally checks the
        // flags on the callee if necessary whether it can be entered.
        self.trap_if_not_flag(adapter.lower.flags, FLAG_MAY_LEAVE, Trap::CannotLeave);
        if adapter.called_as_export {
            self.trap_if_not_flag(adapter.lift.flags, FLAG_MAY_ENTER, Trap::CannotEnter);
            self.set_flag(adapter.lift.flags, FLAG_MAY_ENTER, false);
        } else if self.module.debug {
            self.assert_not_flag(
                adapter.lift.flags,
                FLAG_MAY_ENTER,
                "may_enter should be unset",
            );
        }

        if self.emit_resource_call {
            let enter = self.module.import_resource_enter_call();
            self.instruction(Call(enter.as_u32()));
        }

        // Perform the translation of arguments. Note that `FLAG_MAY_LEAVE` is
        // cleared around this invocation for the callee as per the
        // `canon_lift` definition in the spec. Additionally note that the
        // precise ordering of traps here is not required since internal state
        // is not visible to either instance and a trap will "lock down" both
        // instances to no longer be visible. This means that we're free to
        // reorder lifts/lowers and flags and such as is necessary and
        // convenient here.
        //
        // TODO: if translation doesn't actually call any functions in either
        // instance then there's no need to set/clear the flag here and that can
        // be optimized away.
        self.set_flag(adapter.lift.flags, FLAG_MAY_LEAVE, false);
        let param_locals = lower_sig
            .params
            .iter()
            .enumerate()
            .map(|(i, ty)| (i as u32, *ty))
            .collect::<Vec<_>>();
        self.translate_params(adapter, &param_locals);
        self.set_flag(adapter.lift.flags, FLAG_MAY_LEAVE, true);

        // With all the arguments on the stack the actual target function is
        // now invoked. The core wasm results of the function are then placed
        // into locals for result translation afterwards.
        self.instruction(Call(adapter.callee.as_u32()));
        let mut result_locals = Vec::with_capacity(lift_sig.results.len());
        let mut temps = Vec::new();
        for ty in lift_sig.results.iter().rev() {
            let local = self.local_set_new_tmp(*ty);
            result_locals.push((local.idx, *ty));
            temps.push(local);
        }
        result_locals.reverse();

        // Like above during the translation of results the caller cannot be
        // left (as we might invoke things like `realloc`). Again the precise
        // order of everything doesn't matter since intermediate states cannot
        // be witnessed, hence the setting of flags here to encapsulate both
        // liftings and lowerings.
        //
        // TODO: like above the management of the `MAY_LEAVE` flag can probably
        // be elided here for "simple" results.
        self.set_flag(adapter.lower.flags, FLAG_MAY_LEAVE, false);
        self.translate_results(adapter, &param_locals, &result_locals);
        self.set_flag(adapter.lower.flags, FLAG_MAY_LEAVE, true);

        // And finally post-return state is handled here once all results/etc
        // are all translated.
        if let Some(func) = adapter.lift.post_return {
            for (result, _) in result_locals.iter() {
                self.instruction(LocalGet(*result));
            }
            self.instruction(Call(func.as_u32()));
        }
        if adapter.called_as_export {
            self.set_flag(adapter.lift.flags, FLAG_MAY_ENTER, true);
        }

        for tmp in temps {
            self.free_temp_local(tmp);
        }

        if self.emit_resource_call {
            let exit = self.module.import_resource_exit_call();
            self.instruction(Call(exit.as_u32()));
        }

        self.finish()
    }

    fn translate_params(&mut self, adapter: &AdapterData, param_locals: &[(u32, ValType)]) {
        let src_tys = self.types[adapter.lower.ty].params;
        let src_tys = self.types[src_tys]
            .types
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let dst_tys = self.types[adapter.lift.ty].params;
        let dst_tys = self.types[dst_tys]
            .types
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let lift_opts = &adapter.lift.options;
        let lower_opts = &adapter.lower.options;

        // TODO: handle subtyping
        assert_eq!(src_tys.len(), dst_tys.len());

        // Async lowered functions have a smaller limit on flat parameters, but
        // their destination, a lifted function, does not have a different limit
        // than sync functions.
        let max_flat_params = if adapter.lower.options.async_ {
            MAX_FLAT_ASYNC_PARAMS
        } else {
            MAX_FLAT_PARAMS
        };
        let src_flat =
            self.types
                .flatten_types(lower_opts, max_flat_params, src_tys.iter().copied());
        let dst_flat =
            self.types
                .flatten_types(lift_opts, MAX_FLAT_PARAMS, dst_tys.iter().copied());

        let src = if let Some(flat) = &src_flat {
            Source::Stack(Stack {
                locals: &param_locals[..flat.len()],
                opts: lower_opts,
            })
        } else {
            // If there are too many parameters then that means the parameters
            // are actually a tuple stored in linear memory addressed by the
            // first parameter local.
            let lower_mem_opts = lower_opts.data_model.unwrap_memory();
            let (addr, ty) = param_locals[0];
            assert_eq!(ty, lower_mem_opts.ptr());
            let align = src_tys
                .iter()
                .map(|t| self.types.align(lower_mem_opts, t))
                .max()
                .unwrap_or(1);
            Source::Memory(self.memory_operand(lower_opts, TempLocal::new(addr, ty), align))
        };

        let dst = if let Some(flat) = &dst_flat {
            Destination::Stack(flat, lift_opts)
        } else {
            let abi = CanonicalAbiInfo::record(dst_tys.iter().map(|t| self.types.canonical_abi(t)));
            match lift_opts.data_model {
                DataModel::Gc {} => todo!("CM+GC"),
                DataModel::LinearMemory(LinearMemoryOptions { memory64, .. }) => {
                    let (size, align) = if memory64 {
                        (abi.size64, abi.align64)
                    } else {
                        (abi.size32, abi.align32)
                    };

                    // If there are too many parameters then space is allocated in the
                    // destination module for the parameters via its `realloc` function.
                    let size = MallocSize::Const(size);
                    Destination::Memory(self.malloc(lift_opts, size, align))
                }
            }
        };

        let srcs = src
            .record_field_srcs(self.types, src_tys.iter().copied())
            .zip(src_tys.iter());
        let dsts = dst
            .record_field_dsts(self.types, dst_tys.iter().copied())
            .zip(dst_tys.iter());
        for ((src, src_ty), (dst, dst_ty)) in srcs.zip(dsts) {
            self.translate(&src_ty, &src, &dst_ty, &dst);
        }

        // If the destination was linear memory instead of the stack then the
        // actual parameter that we're passing is the address of the values
        // stored, so ensure that's happening in the wasm body here.
        if let Destination::Memory(mem) = dst {
            self.instruction(LocalGet(mem.addr.idx));
            self.free_temp_local(mem.addr);
        }
    }

    fn translate_results(
        &mut self,
        adapter: &AdapterData,
        param_locals: &[(u32, ValType)],
        result_locals: &[(u32, ValType)],
    ) {
        let src_tys = self.types[adapter.lift.ty].results;
        let src_tys = self.types[src_tys]
            .types
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let dst_tys = self.types[adapter.lower.ty].results;
        let dst_tys = self.types[dst_tys]
            .types
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let lift_opts = &adapter.lift.options;
        let lower_opts = &adapter.lower.options;

        let src_flat = self
            .types
            .flatten_lifting_types(lift_opts, src_tys.iter().copied());
        let dst_flat = self
            .types
            .flatten_lowering_types(lower_opts, dst_tys.iter().copied());

        let src = if src_flat.is_some() {
            Source::Stack(Stack {
                locals: result_locals,
                opts: lift_opts,
            })
        } else {
            // The original results to read from in this case come from the
            // return value of the function itself. The imported function will
            // return a linear memory address at which the values can be read
            // from.
            let lift_mem_opts = lift_opts.data_model.unwrap_memory();
            let align = src_tys
                .iter()
                .map(|t| self.types.align(lift_mem_opts, t))
                .max()
                .unwrap_or(1);
            assert_eq!(
                result_locals.len(),
                if lower_opts.async_ || lift_opts.async_ {
                    2
                } else {
                    1
                }
            );
            let (addr, ty) = result_locals[0];
            assert_eq!(ty, lift_opts.data_model.unwrap_memory().ptr());
            Source::Memory(self.memory_operand(lift_opts, TempLocal::new(addr, ty), align))
        };

        let dst = if let Some(flat) = &dst_flat {
            Destination::Stack(flat, lower_opts)
        } else {
            // This is slightly different than `translate_params` where the
            // return pointer was provided by the caller of this function
            // meaning the last parameter local is a pointer into linear memory.
            let lower_mem_opts = lower_opts.data_model.unwrap_memory();
            let align = dst_tys
                .iter()
                .map(|t| self.types.align(lower_mem_opts, t))
                .max()
                .unwrap_or(1);
            let (addr, ty) = *param_locals.last().expect("no retptr");
            assert_eq!(ty, lower_opts.data_model.unwrap_memory().ptr());
            Destination::Memory(self.memory_operand(lower_opts, TempLocal::new(addr, ty), align))
        };

        let srcs = src
            .record_field_srcs(self.types, src_tys.iter().copied())
            .zip(src_tys.iter());
        let dsts = dst
            .record_field_dsts(self.types, dst_tys.iter().copied())
            .zip(dst_tys.iter());
        for ((src, src_ty), (dst, dst_ty)) in srcs.zip(dsts) {
            self.translate(&src_ty, &src, &dst_ty, &dst);
        }
    }

    fn translate(
        &mut self,
        src_ty: &InterfaceType,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        if let Source::Memory(mem) = src {
            self.assert_aligned(src_ty, mem);
        }
        if let Destination::Memory(mem) = dst {
            self.assert_aligned(dst_ty, mem);
        }

        // Calculate a cost heuristic for what the translation of this specific
        // layer of the type is going to incur. The purpose of this cost is that
        // we'll deduct it from `self.fuel` and if no fuel is remaining then
        // translation is outlined into a separate function rather than being
        // translated into this function.
        //
        // The general goal is to avoid creating an exponentially sized function
        // for a linearly sized input (the type section). By outlining helper
        // functions there will ideally be a constant set of helper functions
        // per type (to accommodate in-memory or on-stack transfers as well as
        // src/dst options) which means that each function is at most a certain
        // size and we have a linear number of functions which should guarantee
        // an overall linear size of the output.
        //
        // To implement this the current heuristic is that each layer of
        // translating a type has a cost associated with it and this cost is
        // accounted for in `self.fuel`. Some conversions are considered free as
        // they generate basically as much code as the `call` to the translation
        // function while other are considered proportionally expensive to the
        // size of the type. The hope is that some upper layers are of a type's
        // translation are all inlined into one function but bottom layers end
        // up getting outlined to separate functions. Theoretically, again this
        // is built on hopes and dreams, the outlining can be shared amongst
        // tightly-intertwined type hierarchies which will reduce the size of
        // the output module due to the helpers being used.
        //
        // This heuristic of how to split functions has changed a few times in
        // the past and this isn't necessarily guaranteed to be the final
        // iteration.
        let cost = match src_ty {
            // These types are all quite simple to load/store and equate to
            // basically the same cost of the `call` instruction to call an
            // out-of-line translation function, so give them 0 cost.
            InterfaceType::Bool
            | InterfaceType::U8
            | InterfaceType::S8
            | InterfaceType::U16
            | InterfaceType::S16
            | InterfaceType::U32
            | InterfaceType::S32
            | InterfaceType::U64
            | InterfaceType::S64
            | InterfaceType::Float32
            | InterfaceType::Float64 => 0,

            // This has a small amount of validation associated with it, so
            // give it a cost of 1.
            InterfaceType::Char => 1,

            // This has a fair bit of code behind it depending on the
            // strings/encodings in play, so arbitrarily assign it this cost.
            InterfaceType::String => 40,

            // Iteration of a loop is along the lines of the cost of a string
            // so give it the same cost
            InterfaceType::List(_) => 40,

            InterfaceType::Flags(i) => {
                let count = self.module.types[*i].names.len();
                match FlagsSize::from_count(count) {
                    FlagsSize::Size0 => 0,
                    FlagsSize::Size1 | FlagsSize::Size2 => 1,
                    FlagsSize::Size4Plus(n) => n.into(),
                }
            }

            InterfaceType::Record(i) => self.types[*i].fields.len(),
            InterfaceType::Tuple(i) => self.types[*i].types.len(),
            InterfaceType::Variant(i) => self.types[*i].cases.len(),
            InterfaceType::Enum(i) => self.types[*i].names.len(),

            // 2 cases to consider for each of these variants.
            InterfaceType::Option(_) | InterfaceType::Result(_) => 2,

            // TODO(#6696) - something nonzero, is 1 right?
            InterfaceType::Own(_)
            | InterfaceType::Borrow(_)
            | InterfaceType::Future(_)
            | InterfaceType::Stream(_)
            | InterfaceType::ErrorContext(_) => 1,
        };

        match self.fuel.checked_sub(cost) {
            // This function has enough fuel to perform the layer of translation
            // necessary for this type, so the fuel is updated in-place and
            // translation continues. Note that the recursion here is bounded by
            // the static recursion limit for all interface types as imposed
            // during the translation phase.
            Some(n) => {
                self.fuel = n;
                match src_ty {
                    InterfaceType::Bool => self.translate_bool(src, dst_ty, dst),
                    InterfaceType::U8 => self.translate_u8(src, dst_ty, dst),
                    InterfaceType::S8 => self.translate_s8(src, dst_ty, dst),
                    InterfaceType::U16 => self.translate_u16(src, dst_ty, dst),
                    InterfaceType::S16 => self.translate_s16(src, dst_ty, dst),
                    InterfaceType::U32 => self.translate_u32(src, dst_ty, dst),
                    InterfaceType::S32 => self.translate_s32(src, dst_ty, dst),
                    InterfaceType::U64 => self.translate_u64(src, dst_ty, dst),
                    InterfaceType::S64 => self.translate_s64(src, dst_ty, dst),
                    InterfaceType::Float32 => self.translate_f32(src, dst_ty, dst),
                    InterfaceType::Float64 => self.translate_f64(src, dst_ty, dst),
                    InterfaceType::Char => self.translate_char(src, dst_ty, dst),
                    InterfaceType::String => self.translate_string(src, dst_ty, dst),
                    InterfaceType::List(t) => self.translate_list(*t, src, dst_ty, dst),
                    InterfaceType::Record(t) => self.translate_record(*t, src, dst_ty, dst),
                    InterfaceType::Flags(f) => self.translate_flags(*f, src, dst_ty, dst),
                    InterfaceType::Tuple(t) => self.translate_tuple(*t, src, dst_ty, dst),
                    InterfaceType::Variant(v) => self.translate_variant(*v, src, dst_ty, dst),
                    InterfaceType::Enum(t) => self.translate_enum(*t, src, dst_ty, dst),
                    InterfaceType::Option(t) => self.translate_option(*t, src, dst_ty, dst),
                    InterfaceType::Result(t) => self.translate_result(*t, src, dst_ty, dst),
                    InterfaceType::Own(t) => self.translate_own(*t, src, dst_ty, dst),
                    InterfaceType::Borrow(t) => self.translate_borrow(*t, src, dst_ty, dst),
                    InterfaceType::Future(t) => self.translate_future(*t, src, dst_ty, dst),
                    InterfaceType::Stream(t) => self.translate_stream(*t, src, dst_ty, dst),
                    InterfaceType::ErrorContext(t) => {
                        self.translate_error_context(*t, src, dst_ty, dst)
                    }
                }
            }

            // This function does not have enough fuel left to perform this
            // layer of translation so the translation is deferred to a helper
            // function. The actual translation here is then done by marshalling
            // the src/dst into the function we're calling and then processing
            // the results.
            None => {
                let src_loc = match src {
                    // If the source is on the stack then `stack_get` is used to
                    // convert everything to the appropriate flat representation
                    // for the source type.
                    Source::Stack(stack) => {
                        for (i, ty) in stack
                            .opts
                            .flat_types(src_ty, self.types)
                            .unwrap()
                            .iter()
                            .enumerate()
                        {
                            let stack = stack.slice(i..i + 1);
                            self.stack_get(&stack, (*ty).into());
                        }
                        HelperLocation::Stack
                    }
                    // If the source is in memory then the pointer is passed
                    // through, but note that the offset must be factored in
                    // here since the translation function will start from
                    // offset 0.
                    Source::Memory(mem) => {
                        self.push_mem_addr(mem);
                        HelperLocation::Memory
                    }
                    Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
                };
                let dst_loc = match dst {
                    Destination::Stack(..) => HelperLocation::Stack,
                    Destination::Memory(mem) => {
                        self.push_mem_addr(mem);
                        HelperLocation::Memory
                    }
                    Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
                };
                // Generate a `FunctionId` corresponding to the `Helper`
                // configuration that is necessary here. This will ideally be a
                // "cache hit" and use a preexisting helper which represents
                // outlining what would otherwise be duplicate code within a
                // function to one function.
                let helper = self.module.translate_helper(Helper {
                    src: HelperType {
                        ty: *src_ty,
                        opts: *src.opts(),
                        loc: src_loc,
                    },
                    dst: HelperType {
                        ty: *dst_ty,
                        opts: *dst.opts(),
                        loc: dst_loc,
                    },
                });
                // Emit a `call` instruction which will get "relocated" to a
                // function index once translation has completely finished.
                self.flush_code();
                self.module.funcs[self.result].body.push(Body::Call(helper));

                // If the destination of the translation was on the stack then
                // the types on the stack need to be optionally converted to
                // different types (e.g. if the result here is part of a variant
                // somewhere else).
                //
                // This translation happens inline here by popping the results
                // into new locals and then using those locals to do a
                // `stack_set`.
                if let Destination::Stack(tys, opts) = dst {
                    let flat = self
                        .types
                        .flatten_types(opts, usize::MAX, [*dst_ty])
                        .unwrap();
                    assert_eq!(flat.len(), tys.len());
                    let locals = flat
                        .iter()
                        .rev()
                        .map(|ty| self.local_set_new_tmp(*ty))
                        .collect::<Vec<_>>();
                    for (ty, local) in tys.iter().zip(locals.into_iter().rev()) {
                        self.instruction(LocalGet(local.idx));
                        self.stack_set(std::slice::from_ref(ty), local.ty);
                        self.free_temp_local(local);
                    }
                }
            }
        }
    }

    fn push_mem_addr(&mut self, mem: &Memory<'_>) {
        self.instruction(LocalGet(mem.addr.idx));
        if mem.offset != 0 {
            self.ptr_uconst(mem.mem_opts(), mem.offset);
            self.ptr_add(mem.mem_opts());
        }
    }

    fn translate_bool(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::Bool));
        self.push_dst_addr(dst);

        // Booleans are canonicalized to 0 or 1 as they pass through the
        // component boundary, so use a `select` instruction to do so.
        self.instruction(I32Const(1));
        self.instruction(I32Const(0));
        match src {
            Source::Memory(mem) => self.i32_load8u(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        self.instruction(Select);

        match dst {
            Destination::Memory(mem) => self.i32_store8(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_u8(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::U8));
        self.convert_u8_mask(src, dst, 0xff);
    }

    fn convert_u8_mask(&mut self, src: &Source<'_>, dst: &Destination<'_>, mask: u8) {
        self.push_dst_addr(dst);
        let mut needs_mask = true;
        match src {
            Source::Memory(mem) => {
                self.i32_load8u(mem);
                needs_mask = mask != 0xff;
            }
            Source::Stack(stack) => {
                self.stack_get(stack, ValType::I32);
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        if needs_mask {
            self.instruction(I32Const(i32::from(mask)));
            self.instruction(I32And);
        }
        match dst {
            Destination::Memory(mem) => self.i32_store8(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_s8(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::S8));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i32_load8s(mem),
            Source::Stack(stack) => {
                self.stack_get(stack, ValType::I32);
                self.instruction(I32Extend8S);
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.i32_store8(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_u16(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::U16));
        self.convert_u16_mask(src, dst, 0xffff);
    }

    fn convert_u16_mask(&mut self, src: &Source<'_>, dst: &Destination<'_>, mask: u16) {
        self.push_dst_addr(dst);
        let mut needs_mask = true;
        match src {
            Source::Memory(mem) => {
                self.i32_load16u(mem);
                needs_mask = mask != 0xffff;
            }
            Source::Stack(stack) => {
                self.stack_get(stack, ValType::I32);
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        if needs_mask {
            self.instruction(I32Const(i32::from(mask)));
            self.instruction(I32And);
        }
        match dst {
            Destination::Memory(mem) => self.i32_store16(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_s16(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::S16));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i32_load16s(mem),
            Source::Stack(stack) => {
                self.stack_get(stack, ValType::I32);
                self.instruction(I32Extend16S);
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.i32_store16(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_u32(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::U32));
        self.convert_u32_mask(src, dst, 0xffffffff)
    }

    fn convert_u32_mask(&mut self, src: &Source<'_>, dst: &Destination<'_>, mask: u32) {
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i32_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        if mask != 0xffffffff {
            self.instruction(I32Const(mask as i32));
            self.instruction(I32And);
        }
        match dst {
            Destination::Memory(mem) => self.i32_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_s32(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::S32));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i32_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.i32_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_u64(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::U64));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i64_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I64),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.i64_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I64),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_s64(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::S64));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i64_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I64),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.i64_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I64),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_f32(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::Float32));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.f32_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::F32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.f32_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::F32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_f64(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        // TODO: subtyping
        assert!(matches!(dst_ty, InterfaceType::Float64));
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.f64_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::F64),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        match dst {
            Destination::Memory(mem) => self.f64_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::F64),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn translate_char(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        assert!(matches!(dst_ty, InterfaceType::Char));
        match src {
            Source::Memory(mem) => self.i32_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        let local = self.local_set_new_tmp(ValType::I32);

        // This sequence is copied from the output of LLVM for:
        //
        //      pub extern "C" fn foo(x: u32) -> char {
        //          char::try_from(x)
        //              .unwrap_or_else(|_| std::arch::wasm32::unreachable())
        //      }
        //
        // Apparently this does what's required by the canonical ABI:
        //
        //    def i32_to_char(opts, i):
        //      trap_if(i >= 0x110000)
        //      trap_if(0xD800 <= i <= 0xDFFF)
        //      return chr(i)
        //
        // ... but I don't know how it works other than "well I trust LLVM"
        self.instruction(Block(BlockType::Empty));
        self.instruction(Block(BlockType::Empty));
        self.instruction(LocalGet(local.idx));
        self.instruction(I32Const(0xd800));
        self.instruction(I32Xor);
        self.instruction(I32Const(-0x110000));
        self.instruction(I32Add);
        self.instruction(I32Const(-0x10f800));
        self.instruction(I32LtU);
        self.instruction(BrIf(0));
        self.instruction(LocalGet(local.idx));
        self.instruction(I32Const(0x110000));
        self.instruction(I32Ne);
        self.instruction(BrIf(1));
        self.instruction(End);
        self.trap(Trap::InvalidChar);
        self.instruction(End);

        self.push_dst_addr(dst);
        self.instruction(LocalGet(local.idx));
        match dst {
            Destination::Memory(mem) => {
                self.i32_store(mem);
            }
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }

        self.free_temp_local(local);
    }

    fn translate_string(&mut self, src: &Source<'_>, dst_ty: &InterfaceType, dst: &Destination) {
        assert!(matches!(dst_ty, InterfaceType::String));
        let src_opts = src.opts();
        let dst_opts = dst.opts();

        let src_mem_opts = match &src_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        // Load the pointer/length of this string into temporary locals. These
        // will be referenced a good deal so this just makes it easier to deal
        // with them consistently below rather than trying to reload from memory
        // for example.
        match src {
            Source::Stack(s) => {
                assert_eq!(s.locals.len(), 2);
                self.stack_get(&s.slice(0..1), src_mem_opts.ptr());
                self.stack_get(&s.slice(1..2), src_mem_opts.ptr());
            }
            Source::Memory(mem) => {
                self.ptr_load(mem);
                self.ptr_load(&mem.bump(src_mem_opts.ptr_size().into()));
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        let src_len = self.local_set_new_tmp(src_mem_opts.ptr());
        let src_ptr = self.local_set_new_tmp(src_mem_opts.ptr());
        let src_str = WasmString {
            ptr: src_ptr,
            len: src_len,
            opts: src_opts,
        };

        let dst_str = match src_opts.string_encoding {
            StringEncoding::Utf8 => match dst_opts.string_encoding {
                StringEncoding::Utf8 => self.string_copy(&src_str, FE::Utf8, dst_opts, FE::Utf8),
                StringEncoding::Utf16 => self.string_utf8_to_utf16(&src_str, dst_opts),
                StringEncoding::CompactUtf16 => {
                    self.string_to_compact(&src_str, FE::Utf8, dst_opts)
                }
            },

            StringEncoding::Utf16 => {
                self.verify_aligned(src_mem_opts, src_str.ptr.idx, 2);
                match dst_opts.string_encoding {
                    StringEncoding::Utf8 => {
                        self.string_deflate_to_utf8(&src_str, FE::Utf16, dst_opts)
                    }
                    StringEncoding::Utf16 => {
                        self.string_copy(&src_str, FE::Utf16, dst_opts, FE::Utf16)
                    }
                    StringEncoding::CompactUtf16 => {
                        self.string_to_compact(&src_str, FE::Utf16, dst_opts)
                    }
                }
            }

            StringEncoding::CompactUtf16 => {
                self.verify_aligned(src_mem_opts, src_str.ptr.idx, 2);

                // Test the tag big to see if this is a utf16 or a latin1 string
                // at runtime...
                self.instruction(LocalGet(src_str.len.idx));
                self.ptr_uconst(src_mem_opts, UTF16_TAG);
                self.ptr_and(src_mem_opts);
                self.ptr_if(src_mem_opts, BlockType::Empty);

                // In the utf16 block unset the upper bit from the length local
                // so further calculations have the right value. Afterwards the
                // string transcode proceeds assuming utf16.
                self.instruction(LocalGet(src_str.len.idx));
                self.ptr_uconst(src_mem_opts, UTF16_TAG);
                self.ptr_xor(src_mem_opts);
                self.instruction(LocalSet(src_str.len.idx));
                let s1 = match dst_opts.string_encoding {
                    StringEncoding::Utf8 => {
                        self.string_deflate_to_utf8(&src_str, FE::Utf16, dst_opts)
                    }
                    StringEncoding::Utf16 => {
                        self.string_copy(&src_str, FE::Utf16, dst_opts, FE::Utf16)
                    }
                    StringEncoding::CompactUtf16 => {
                        self.string_compact_utf16_to_compact(&src_str, dst_opts)
                    }
                };

                self.instruction(Else);

                // In the latin1 block the `src_len` local is already the number
                // of code units, so the string transcoding is all that needs to
                // happen.
                let s2 = match dst_opts.string_encoding {
                    StringEncoding::Utf16 => {
                        self.string_copy(&src_str, FE::Latin1, dst_opts, FE::Utf16)
                    }
                    StringEncoding::Utf8 => {
                        self.string_deflate_to_utf8(&src_str, FE::Latin1, dst_opts)
                    }
                    StringEncoding::CompactUtf16 => {
                        self.string_copy(&src_str, FE::Latin1, dst_opts, FE::Latin1)
                    }
                };
                // Set our `s2` generated locals to the `s2` generated locals
                // as the resulting pointer of this transcode.
                self.instruction(LocalGet(s2.ptr.idx));
                self.instruction(LocalSet(s1.ptr.idx));
                self.instruction(LocalGet(s2.len.idx));
                self.instruction(LocalSet(s1.len.idx));
                self.instruction(End);
                self.free_temp_local(s2.ptr);
                self.free_temp_local(s2.len);
                s1
            }
        };

        // Store the ptr/length in the desired destination
        match dst {
            Destination::Stack(s, _) => {
                self.instruction(LocalGet(dst_str.ptr.idx));
                self.stack_set(&s[..1], dst_mem_opts.ptr());
                self.instruction(LocalGet(dst_str.len.idx));
                self.stack_set(&s[1..], dst_mem_opts.ptr());
            }
            Destination::Memory(mem) => {
                self.instruction(LocalGet(mem.addr.idx));
                self.instruction(LocalGet(dst_str.ptr.idx));
                self.ptr_store(mem);
                self.instruction(LocalGet(mem.addr.idx));
                self.instruction(LocalGet(dst_str.len.idx));
                self.ptr_store(&mem.bump(dst_mem_opts.ptr_size().into()));
            }
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }

        self.free_temp_local(src_str.ptr);
        self.free_temp_local(src_str.len);
        self.free_temp_local(dst_str.ptr);
        self.free_temp_local(dst_str.len);
    }

    // Corresponding function for `store_string_copy` in the spec.
    //
    // This performs a transcoding of the string with a one-pass copy from
    // the `src` encoding to the `dst` encoding. This is only possible for
    // fixed encodings where the first allocation is guaranteed to be an
    // appropriate fit so it's not suitable for all encodings.
    //
    // Imported host transcoding functions here take the src/dst pointers as
    // well as the number of code units in the source (which always matches
    // the number of code units in the destination). There is no return
    // value from the transcode function since the encoding should always
    // work on the first pass.
    fn string_copy<'c>(
        &mut self,
        src: &WasmString<'_>,
        src_enc: FE,
        dst_opts: &'c Options,
        dst_enc: FE,
    ) -> WasmString<'c> {
        assert!(dst_enc.width() >= src_enc.width());

        let src_mem_opts = {
            match &src.opts.data_model {
                DataModel::Gc {} => todo!("CM+GC"),
                DataModel::LinearMemory(opts) => opts,
            }
        };
        let dst_mem_opts = {
            match &dst_opts.data_model {
                DataModel::Gc {} => todo!("CM+GC"),
                DataModel::LinearMemory(opts) => opts,
            }
        };

        let (src_byte_len_tmp, src_byte_len) =
            self.source_string_byte_len(src, src_enc, src_mem_opts);

        // Convert the source code units length to the destination byte
        // length type.
        self.convert_src_len_to_dst(
            src.len.idx,
            src.opts.data_model.unwrap_memory().ptr(),
            dst_opts.data_model.unwrap_memory().ptr(),
        );
        let dst_len = self.local_tee_new_tmp(dst_opts.data_model.unwrap_memory().ptr());
        if dst_enc.width() > 1 {
            assert_eq!(dst_enc.width(), 2);
            self.ptr_uconst(dst_mem_opts, 1);
            self.ptr_shl(dst_mem_opts);
        }
        let dst_byte_len = self.local_set_new_tmp(dst_opts.data_model.unwrap_memory().ptr());

        // Allocate space in the destination using the calculated byte
        // length.
        let dst = {
            let dst_mem = self.malloc(
                dst_opts,
                MallocSize::Local(dst_byte_len.idx),
                dst_enc.width().into(),
            );
            WasmString {
                ptr: dst_mem.addr,
                len: dst_len,
                opts: dst_opts,
            }
        };

        // Validate that `src_len + src_ptr` and
        // `dst_mem.addr_local + dst_byte_len` are both in-bounds. This
        // is done by loading the last byte of the string and if that
        // doesn't trap then it's known valid.
        self.validate_string_inbounds(src, src_byte_len);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        // If the validations pass then the host `transcode` intrinsic
        // is invoked. This will either raise a trap or otherwise succeed
        // in which case we're done.
        let op = if src_enc == dst_enc {
            Transcode::Copy(src_enc)
        } else {
            assert_eq!(src_enc, FE::Latin1);
            assert_eq!(dst_enc, FE::Utf16);
            Transcode::Latin1ToUtf16
        };
        let transcode = self.transcoder(src, &dst, op);
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(Call(transcode.as_u32()));

        self.free_temp_local(dst_byte_len);
        if let Some(tmp) = src_byte_len_tmp {
            self.free_temp_local(tmp);
        }

        dst
    }

    /// Calculate the source byte length given the size of each code
    /// unit.
    ///
    /// Returns an optional temporary local if it was needed, which the caller
    /// needs to deallocate with `free_temp_local`. Additionally returns the
    /// index of the local which contains the byte length of the string, which
    /// may point to the temporary local passed in.
    fn source_string_byte_len(
        &mut self,
        src: &WasmString<'_>,
        src_enc: FE,
        src_mem_opts: &LinearMemoryOptions,
    ) -> (Option<TempLocal>, u32) {
        self.validate_string_length(src, src_enc);

        if src_enc.width() == 1 {
            (None, src.len.idx)
        } else {
            assert_eq!(src_enc.width(), 2);

            // Note that this shouldn't overflow given `validate_string_length`
            // above.
            self.instruction(LocalGet(src.len.idx));
            self.ptr_uconst(src_mem_opts, 1);
            self.ptr_shl(src_mem_opts);
            let tmp = self.local_set_new_tmp(src.opts.data_model.unwrap_memory().ptr());

            let idx = tmp.idx;
            (Some(tmp), idx)
        }
    }

    // Corresponding function for `store_string_to_utf8` in the spec.
    //
    // This translation works by possibly performing a number of
    // reallocations. First a buffer of size input-code-units is used to try
    // to get the transcoding correct on the first try. If that fails the
    // maximum worst-case size is used and then that is resized down if it's
    // too large.
    //
    // The host transcoding function imported here will receive src ptr/len
    // and dst ptr/len and return how many code units were consumed on both
    // sides. The amount of code units consumed in the source dictates which
    // branches are taken in this conversion.
    fn string_deflate_to_utf8<'c>(
        &mut self,
        src: &WasmString<'_>,
        src_enc: FE,
        dst_opts: &'c Options,
    ) -> WasmString<'c> {
        let src_mem_opts = match &src.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        self.validate_string_length(src, src_enc);

        // Optimistically assume that the code unit length of the source is
        // all that's needed in the destination. Perform that allocation
        // here and proceed to transcoding below.
        self.convert_src_len_to_dst(
            src.len.idx,
            src.opts.data_model.unwrap_memory().ptr(),
            dst_opts.data_model.unwrap_memory().ptr(),
        );
        let dst_len = self.local_tee_new_tmp(dst_opts.data_model.unwrap_memory().ptr());
        let dst_byte_len = self.local_set_new_tmp(dst_opts.data_model.unwrap_memory().ptr());

        let dst = {
            let dst_mem = self.malloc(dst_opts, MallocSize::Local(dst_byte_len.idx), 1);
            WasmString {
                ptr: dst_mem.addr,
                len: dst_len,
                opts: dst_opts,
            }
        };

        // Ensure buffers are all in-bounds
        let mut src_byte_len_tmp = None;
        let src_byte_len = match src_enc {
            FE::Latin1 => src.len.idx,
            FE::Utf16 => {
                self.instruction(LocalGet(src.len.idx));
                self.ptr_uconst(src_mem_opts, 1);
                self.ptr_shl(src_mem_opts);
                let tmp = self.local_set_new_tmp(src.opts.data_model.unwrap_memory().ptr());
                let ret = tmp.idx;
                src_byte_len_tmp = Some(tmp);
                ret
            }
            FE::Utf8 => unreachable!(),
        };
        self.validate_string_inbounds(src, src_byte_len);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        // Perform the initial transcode
        let op = match src_enc {
            FE::Latin1 => Transcode::Latin1ToUtf8,
            FE::Utf16 => Transcode::Utf16ToUtf8,
            FE::Utf8 => unreachable!(),
        };
        let transcode = self.transcoder(src, &dst, op);
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(LocalGet(dst_byte_len.idx));
        self.instruction(Call(transcode.as_u32()));
        self.instruction(LocalSet(dst.len.idx));
        let src_len_tmp = self.local_set_new_tmp(src.opts.data_model.unwrap_memory().ptr());

        // Test if the source was entirely transcoded by comparing
        // `src_len_tmp`, the number of code units transcoded from the
        // source, with `src_len`, the original number of code units.
        self.instruction(LocalGet(src_len_tmp.idx));
        self.instruction(LocalGet(src.len.idx));
        self.ptr_ne(src_mem_opts);
        self.instruction(If(BlockType::Empty));

        // Here a worst-case reallocation is performed to grow `dst_mem`.
        // In-line a check is also performed that the worst-case byte size
        // fits within the maximum size of strings.
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 1); // align
        let factor = match src_enc {
            FE::Latin1 => 2,
            FE::Utf16 => 3,
            _ => unreachable!(),
        };
        self.validate_string_length_u8(src, factor);
        self.convert_src_len_to_dst(
            src.len.idx,
            src.opts.data_model.unwrap_memory().ptr(),
            dst_opts.data_model.unwrap_memory().ptr(),
        );
        self.ptr_uconst(dst_mem_opts, factor.into());
        self.ptr_mul(dst_mem_opts);
        self.instruction(LocalTee(dst_byte_len.idx));
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));

        // Verify that the destination is still in-bounds
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        // Perform another round of transcoding that should be guaranteed
        // to succeed. Note that all the parameters here are offset by the
        // results of the first transcoding to only perform the remaining
        // transcode on the final units.
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src_len_tmp.idx));
        if let FE::Utf16 = src_enc {
            self.ptr_uconst(src_mem_opts, 1);
            self.ptr_shl(src_mem_opts);
        }
        self.ptr_add(src_mem_opts);
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(src_len_tmp.idx));
        self.ptr_sub(src_mem_opts);
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_add(dst_mem_opts);
        self.instruction(LocalGet(dst_byte_len.idx));
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_sub(dst_mem_opts);
        self.instruction(Call(transcode.as_u32()));

        // Add the second result, the amount of destination units encoded,
        // to `dst_len` so it's an accurate reflection of the final size of
        // the destination buffer.
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_add(dst_mem_opts);
        self.instruction(LocalSet(dst.len.idx));

        // In debug mode verify the first result consumed the entire string,
        // otherwise simply discard it.
        if self.module.debug {
            self.instruction(LocalGet(src.len.idx));
            self.instruction(LocalGet(src_len_tmp.idx));
            self.ptr_sub(src_mem_opts);
            self.ptr_ne(src_mem_opts);
            self.instruction(If(BlockType::Empty));
            self.trap(Trap::AssertFailed("should have finished encoding"));
            self.instruction(End);
        } else {
            self.instruction(Drop);
        }

        // Perform a downsizing if the worst-case size was too large
        self.instruction(LocalGet(dst.len.idx));
        self.instruction(LocalGet(dst_byte_len.idx));
        self.ptr_ne(dst_mem_opts);
        self.instruction(If(BlockType::Empty));
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 1); // align
        self.instruction(LocalGet(dst.len.idx)); // new_size
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));
        self.instruction(End);

        // If the first transcode was enough then assert that the returned
        // amount of destination items written equals the byte size.
        if self.module.debug {
            self.instruction(Else);

            self.instruction(LocalGet(dst.len.idx));
            self.instruction(LocalGet(dst_byte_len.idx));
            self.ptr_ne(dst_mem_opts);
            self.instruction(If(BlockType::Empty));
            self.trap(Trap::AssertFailed("should have finished encoding"));
            self.instruction(End);
        }

        self.instruction(End); // end of "first transcode not enough"

        self.free_temp_local(src_len_tmp);
        self.free_temp_local(dst_byte_len);
        if let Some(tmp) = src_byte_len_tmp {
            self.free_temp_local(tmp);
        }

        dst
    }

    // Corresponds to the `store_utf8_to_utf16` function in the spec.
    //
    // When converting utf-8 to utf-16 a pessimistic allocation is
    // done which is twice the byte length of the utf-8 string.
    // The host then transcodes and returns how many code units were
    // actually used during the transcoding and if it's beneath the
    // pessimistic maximum then the buffer is reallocated down to
    // a smaller amount.
    //
    // The host-imported transcoding function takes the src/dst pointer as
    // well as the code unit size of both the source and destination. The
    // destination should always be big enough to hold the result of the
    // transcode and so the result of the host function is how many code
    // units were written to the destination.
    fn string_utf8_to_utf16<'c>(
        &mut self,
        src: &WasmString<'_>,
        dst_opts: &'c Options,
    ) -> WasmString<'c> {
        let src_mem_opts = match &src.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        self.validate_string_length(src, FE::Utf16);
        self.convert_src_len_to_dst(
            src.len.idx,
            src_mem_opts.ptr(),
            dst_opts.data_model.unwrap_memory().ptr(),
        );
        let dst_len = self.local_tee_new_tmp(dst_opts.data_model.unwrap_memory().ptr());
        self.ptr_uconst(dst_mem_opts, 1);
        self.ptr_shl(dst_mem_opts);
        let dst_byte_len = self.local_set_new_tmp(dst_opts.data_model.unwrap_memory().ptr());
        let dst = {
            let dst_mem = self.malloc(dst_opts, MallocSize::Local(dst_byte_len.idx), 2);
            WasmString {
                ptr: dst_mem.addr,
                len: dst_len,
                opts: dst_opts,
            }
        };

        self.validate_string_inbounds(src, src.len.idx);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        let transcode = self.transcoder(src, &dst, Transcode::Utf8ToUtf16);
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(Call(transcode.as_u32()));
        self.instruction(LocalSet(dst.len.idx));

        // If the number of code units returned by transcode is not
        // equal to the original number of code units then
        // the buffer must be shrunk.
        //
        // Note that the byte length of the final allocation we
        // want is twice the code unit length returned by the
        // transcoding function.
        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_ne(dst_mem_opts);
        self.instruction(If(BlockType::Empty));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(LocalGet(dst_byte_len.idx));
        self.ptr_uconst(dst_mem_opts, 2);
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_uconst(dst_mem_opts, 1);
        self.ptr_shl(dst_mem_opts);
        self.instruction(Call(match dst.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(LinearMemoryOptions { realloc, .. }) => {
                realloc.unwrap().as_u32()
            }
        }));
        self.instruction(LocalSet(dst.ptr.idx));
        self.verify_aligned(dst_opts.data_model.unwrap_memory(), dst.ptr.idx, 2);
        self.instruction(End); // end of shrink-to-fit

        self.free_temp_local(dst_byte_len);

        dst
    }

    // Corresponds to `store_probably_utf16_to_latin1_or_utf16` in the spec.
    //
    // This will try to transcode the input utf16 string to utf16 in the
    // destination. If utf16 isn't needed though and latin1 could be used
    // then that's used instead and a reallocation to downsize occurs
    // afterwards.
    //
    // The host transcode function here will take the src/dst pointers as
    // well as src length. The destination byte length is twice the src code
    // unit length. The return value is the tagged length of the returned
    // string. If the upper bit is set then utf16 was used and the
    // conversion is done. If the upper bit is not set then latin1 was used
    // and a downsizing needs to happen.
    fn string_compact_utf16_to_compact<'c>(
        &mut self,
        src: &WasmString<'_>,
        dst_opts: &'c Options,
    ) -> WasmString<'c> {
        let src_mem_opts = match &src.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        self.validate_string_length(src, FE::Utf16);
        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        let dst_len = self.local_tee_new_tmp(dst_mem_opts.ptr());
        self.ptr_uconst(dst_mem_opts, 1);
        self.ptr_shl(dst_mem_opts);
        let dst_byte_len = self.local_set_new_tmp(dst_mem_opts.ptr());
        let dst = {
            let dst_mem = self.malloc(dst_opts, MallocSize::Local(dst_byte_len.idx), 2);
            WasmString {
                ptr: dst_mem.addr,
                len: dst_len,
                opts: dst_opts,
            }
        };

        self.convert_src_len_to_dst(
            dst_byte_len.idx,
            dst.opts.data_model.unwrap_memory().ptr(),
            src_mem_opts.ptr(),
        );
        let src_byte_len = self.local_set_new_tmp(src_mem_opts.ptr());

        self.validate_string_inbounds(src, src_byte_len.idx);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        let transcode = self.transcoder(src, &dst, Transcode::Utf16ToCompactProbablyUtf16);
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(Call(transcode.as_u32()));
        self.instruction(LocalSet(dst.len.idx));

        // Assert that the untagged code unit length is the same as the
        // source code unit length.
        if self.module.debug {
            self.instruction(LocalGet(dst.len.idx));
            self.ptr_uconst(dst_mem_opts, !UTF16_TAG);
            self.ptr_and(dst_mem_opts);
            self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
            self.ptr_ne(dst_mem_opts);
            self.instruction(If(BlockType::Empty));
            self.trap(Trap::AssertFailed("expected equal code units"));
            self.instruction(End);
        }

        // If the UTF16_TAG is set then utf16 was used and the destination
        // should be appropriately sized. Bail out of the "is this string
        // empty" block and fall through otherwise to resizing.
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_uconst(dst_mem_opts, UTF16_TAG);
        self.ptr_and(dst_mem_opts);
        self.ptr_br_if(dst_mem_opts, 0);

        // Here `realloc` is used to downsize the string
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 2); // align
        self.instruction(LocalGet(dst.len.idx)); // new_size
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));
        self.verify_aligned(dst_opts.data_model.unwrap_memory(), dst.ptr.idx, 2);

        self.free_temp_local(dst_byte_len);
        self.free_temp_local(src_byte_len);

        dst
    }

    // Corresponds to `store_string_to_latin1_or_utf16` in the spec.
    //
    // This will attempt a first pass of transcoding to latin1 and on
    // failure a larger buffer is allocated for utf16 and then utf16 is
    // encoded in-place into the buffer. After either latin1 or utf16 the
    // buffer is then resized to fit the final string allocation.
    fn string_to_compact<'c>(
        &mut self,
        src: &WasmString<'_>,
        src_enc: FE,
        dst_opts: &'c Options,
    ) -> WasmString<'c> {
        let src_mem_opts = match &src.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst_opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        let (src_byte_len_tmp, src_byte_len) =
            self.source_string_byte_len(src, src_enc, src_mem_opts);

        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        let dst_len = self.local_tee_new_tmp(dst_mem_opts.ptr());
        let dst_byte_len = self.local_set_new_tmp(dst_mem_opts.ptr());
        let dst = {
            let dst_mem = self.malloc(dst_opts, MallocSize::Local(dst_byte_len.idx), 2);
            WasmString {
                ptr: dst_mem.addr,
                len: dst_len,
                opts: dst_opts,
            }
        };

        self.validate_string_inbounds(src, src_byte_len);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        // Perform the initial latin1 transcode. This returns the number of
        // source code units consumed and the number of destination code
        // units (bytes) written.
        let (latin1, utf16) = match src_enc {
            FE::Utf8 => (Transcode::Utf8ToLatin1, Transcode::Utf8ToCompactUtf16),
            FE::Utf16 => (Transcode::Utf16ToLatin1, Transcode::Utf16ToCompactUtf16),
            FE::Latin1 => unreachable!(),
        };
        let transcode_latin1 = self.transcoder(src, &dst, latin1);
        let transcode_utf16 = self.transcoder(src, &dst, utf16);
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(dst.ptr.idx));
        self.instruction(Call(transcode_latin1.as_u32()));
        self.instruction(LocalSet(dst.len.idx));
        let src_len_tmp = self.local_set_new_tmp(src_mem_opts.ptr());

        // If the source was entirely consumed then the transcode completed
        // and all that's necessary is to optionally shrink the buffer.
        self.instruction(LocalGet(src_len_tmp.idx));
        self.instruction(LocalGet(src.len.idx));
        self.ptr_eq(src_mem_opts);
        self.instruction(If(BlockType::Empty)); // if latin1-or-utf16 block

        // Test if the original byte length of the allocation is the same as
        // the number of written bytes, and if not then shrink the buffer
        // with a call to `realloc`.
        self.instruction(LocalGet(dst_byte_len.idx));
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_ne(dst_mem_opts);
        self.instruction(If(BlockType::Empty));
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 2); // align
        self.instruction(LocalGet(dst.len.idx)); // new_size
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));
        self.verify_aligned(dst_opts.data_model.unwrap_memory(), dst.ptr.idx, 2);
        self.instruction(End);

        // In this block the latin1 encoding failed. The host transcode
        // returned how many units were consumed from the source and how
        // many bytes were written to the destination. Here the buffer is
        // inflated and sized and the second utf16 intrinsic is invoked to
        // perform the final inflation.
        self.instruction(Else); // else latin1-or-utf16 block

        // For utf8 validate that the inflated size is still within bounds.
        if src_enc.width() == 1 {
            self.validate_string_length_u8(src, 2);
        }

        // Reallocate the buffer with twice the source code units in byte
        // size.
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 2); // align
        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        self.ptr_uconst(dst_mem_opts, 1);
        self.ptr_shl(dst_mem_opts);
        self.instruction(LocalTee(dst_byte_len.idx));
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));
        self.verify_aligned(dst_opts.data_model.unwrap_memory(), dst.ptr.idx, 2);
        self.validate_string_inbounds(&dst, dst_byte_len.idx);

        // Call the host utf16 transcoding function. This will inflate the
        // prior latin1 bytes and then encode the rest of the source string
        // as utf16 into the remaining space in the destination buffer.
        self.instruction(LocalGet(src.ptr.idx));
        self.instruction(LocalGet(src_len_tmp.idx));
        if let FE::Utf16 = src_enc {
            self.ptr_uconst(src_mem_opts, 1);
            self.ptr_shl(src_mem_opts);
        }
        self.ptr_add(src_mem_opts);
        self.instruction(LocalGet(src.len.idx));
        self.instruction(LocalGet(src_len_tmp.idx));
        self.ptr_sub(src_mem_opts);
        self.instruction(LocalGet(dst.ptr.idx));
        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        self.instruction(LocalGet(dst.len.idx));
        self.instruction(Call(transcode_utf16.as_u32()));
        self.instruction(LocalSet(dst.len.idx));

        // If the returned number of code units written to the destination
        // is not equal to the size of the allocation then the allocation is
        // resized down to the appropriate size.
        //
        // Note that the byte size desired is `2*dst_len` and the current
        // byte buffer size is `2*src_len` so the `2` factor isn't checked
        // here, just the lengths.
        self.instruction(LocalGet(dst.len.idx));
        self.convert_src_len_to_dst(src.len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
        self.ptr_ne(dst_mem_opts);
        self.instruction(If(BlockType::Empty));
        self.instruction(LocalGet(dst.ptr.idx)); // old_ptr
        self.instruction(LocalGet(dst_byte_len.idx)); // old_size
        self.ptr_uconst(dst_mem_opts, 2); // align
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_uconst(dst_mem_opts, 1);
        self.ptr_shl(dst_mem_opts);
        self.instruction(Call(dst_mem_opts.realloc.unwrap().as_u32()));
        self.instruction(LocalSet(dst.ptr.idx));
        self.verify_aligned(dst_opts.data_model.unwrap_memory(), dst.ptr.idx, 2);
        self.instruction(End);

        // Tag the returned pointer as utf16
        self.instruction(LocalGet(dst.len.idx));
        self.ptr_uconst(dst_mem_opts, UTF16_TAG);
        self.ptr_or(dst_mem_opts);
        self.instruction(LocalSet(dst.len.idx));

        self.instruction(End); // end latin1-or-utf16 block

        self.free_temp_local(src_len_tmp);
        self.free_temp_local(dst_byte_len);
        if let Some(tmp) = src_byte_len_tmp {
            self.free_temp_local(tmp);
        }

        dst
    }

    fn validate_string_length(&mut self, src: &WasmString<'_>, dst: FE) {
        self.validate_string_length_u8(src, dst.width())
    }

    fn validate_string_length_u8(&mut self, s: &WasmString<'_>, dst: u8) {
        let mem_opts = match &s.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        // Check to see if the source byte length is out of bounds in
        // which case a trap is generated.
        self.instruction(LocalGet(s.len.idx));
        let max = MAX_STRING_BYTE_LENGTH / u32::from(dst);
        self.ptr_uconst(mem_opts, max);
        self.ptr_ge_u(mem_opts);
        self.instruction(If(BlockType::Empty));
        self.trap(Trap::StringLengthTooBig);
        self.instruction(End);
    }

    fn transcoder(
        &mut self,
        src: &WasmString<'_>,
        dst: &WasmString<'_>,
        op: Transcode,
    ) -> FuncIndex {
        match (src.opts.data_model, dst.opts.data_model) {
            (DataModel::Gc {}, _) | (_, DataModel::Gc {}) => {
                todo!("CM+GC")
            }
            (
                DataModel::LinearMemory(LinearMemoryOptions {
                    memory64: src64,
                    memory: src_mem,
                    realloc: _,
                }),
                DataModel::LinearMemory(LinearMemoryOptions {
                    memory64: dst64,
                    memory: dst_mem,
                    realloc: _,
                }),
            ) => self.module.import_transcoder(Transcoder {
                from_memory: src_mem.unwrap(),
                from_memory64: src64,
                to_memory: dst_mem.unwrap(),
                to_memory64: dst64,
                op,
            }),
        }
    }

    fn validate_string_inbounds(&mut self, s: &WasmString<'_>, byte_len: u32) {
        match &s.opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => {
                self.validate_memory_inbounds(opts, s.ptr.idx, byte_len, Trap::StringLengthOverflow)
            }
        }
    }

    fn validate_memory_inbounds(
        &mut self,
        opts: &LinearMemoryOptions,
        ptr_local: u32,
        byte_len_local: u32,
        trap: Trap,
    ) {
        let extend_to_64 = |me: &mut Self| {
            if !opts.memory64 {
                me.instruction(I64ExtendI32U);
            }
        };

        self.instruction(Block(BlockType::Empty));
        self.instruction(Block(BlockType::Empty));

        // Calculate the full byte size of memory with `memory.size`. Note that
        // arithmetic here is done always in 64-bits to accommodate 4G memories.
        // Additionally it's assumed that 64-bit memories never fill up
        // entirely.
        self.instruction(MemorySize(opts.memory.unwrap().as_u32()));
        extend_to_64(self);
        self.instruction(I64Const(16));
        self.instruction(I64Shl);

        // Calculate the end address of the string. This is done by adding the
        // base pointer to the byte length. For 32-bit memories there's no need
        // to check for overflow since everything is extended to 64-bit, but for
        // 64-bit memories overflow is checked.
        self.instruction(LocalGet(ptr_local));
        extend_to_64(self);
        self.instruction(LocalGet(byte_len_local));
        extend_to_64(self);
        self.instruction(I64Add);
        if opts.memory64 {
            let tmp = self.local_tee_new_tmp(ValType::I64);
            self.instruction(LocalGet(ptr_local));
            self.ptr_lt_u(opts);
            self.instruction(BrIf(0));
            self.instruction(LocalGet(tmp.idx));
            self.free_temp_local(tmp);
        }

        // If the byte size of memory is greater than the final address of the
        // string then the string is invalid. Note that if it's precisely equal
        // then that's ok.
        self.instruction(I64GeU);
        self.instruction(BrIf(1));

        self.instruction(End);
        self.trap(trap);
        self.instruction(End);
    }

    fn translate_list(
        &mut self,
        src_ty: TypeListIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_mem_opts = match &src.opts().data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };
        let dst_mem_opts = match &dst.opts().data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(opts) => opts,
        };

        let src_element_ty = &self.types[src_ty].element;
        let dst_element_ty = match dst_ty {
            InterfaceType::List(r) => &self.types[*r].element,
            _ => panic!("expected a list"),
        };
        let src_opts = src.opts();
        let dst_opts = dst.opts();
        let (src_size, src_align) = self.types.size_align(src_mem_opts, src_element_ty);
        let (dst_size, dst_align) = self.types.size_align(dst_mem_opts, dst_element_ty);

        // Load the pointer/length of this list into temporary locals. These
        // will be referenced a good deal so this just makes it easier to deal
        // with them consistently below rather than trying to reload from memory
        // for example.
        match src {
            Source::Stack(s) => {
                assert_eq!(s.locals.len(), 2);
                self.stack_get(&s.slice(0..1), src_mem_opts.ptr());
                self.stack_get(&s.slice(1..2), src_mem_opts.ptr());
            }
            Source::Memory(mem) => {
                self.ptr_load(mem);
                self.ptr_load(&mem.bump(src_mem_opts.ptr_size().into()));
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        let src_len = self.local_set_new_tmp(src_mem_opts.ptr());
        let src_ptr = self.local_set_new_tmp(src_mem_opts.ptr());

        // Create a `Memory` operand which will internally assert that the
        // `src_ptr` value is properly aligned.
        let src_mem = self.memory_operand(src_opts, src_ptr, src_align);

        // Calculate the source/destination byte lengths into unique locals.
        let src_byte_len = self.calculate_list_byte_len(src_mem_opts, src_len.idx, src_size);
        let dst_byte_len = if src_size == dst_size {
            self.convert_src_len_to_dst(src_byte_len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
            self.local_set_new_tmp(dst_mem_opts.ptr())
        } else if src_mem_opts.ptr() == dst_mem_opts.ptr() {
            self.calculate_list_byte_len(dst_mem_opts, src_len.idx, dst_size)
        } else {
            self.convert_src_len_to_dst(src_byte_len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
            let tmp = self.local_set_new_tmp(dst_mem_opts.ptr());
            let ret = self.calculate_list_byte_len(dst_mem_opts, tmp.idx, dst_size);
            self.free_temp_local(tmp);
            ret
        };

        // Here `realloc` is invoked (in a `malloc`-like fashion) to allocate
        // space for the list in the destination memory. This will also
        // internally insert checks that the returned pointer is aligned
        // correctly for the destination.
        let dst_mem = self.malloc(dst_opts, MallocSize::Local(dst_byte_len.idx), dst_align);

        // With all the pointers and byte lengths verity that both the source
        // and the destination buffers are in-bounds.
        self.validate_memory_inbounds(
            src_mem_opts,
            src_mem.addr.idx,
            src_byte_len.idx,
            Trap::ListByteLengthOverflow,
        );
        self.validate_memory_inbounds(
            dst_mem_opts,
            dst_mem.addr.idx,
            dst_byte_len.idx,
            Trap::ListByteLengthOverflow,
        );

        self.free_temp_local(src_byte_len);
        self.free_temp_local(dst_byte_len);

        // This is the main body of the loop to actually translate list types.
        // Note that if both element sizes are 0 then this won't actually do
        // anything so the loop is removed entirely.
        if src_size > 0 || dst_size > 0 {
            // This block encompasses the entire loop and is use to exit before even
            // entering the loop if the list size is zero.
            self.instruction(Block(BlockType::Empty));

            // Set the `remaining` local and only continue if it's > 0
            self.instruction(LocalGet(src_len.idx));
            let remaining = self.local_tee_new_tmp(src_mem_opts.ptr());
            self.ptr_eqz(src_mem_opts);
            self.instruction(BrIf(0));

            // Initialize the two destination pointers to their initial values
            self.instruction(LocalGet(src_mem.addr.idx));
            let cur_src_ptr = self.local_set_new_tmp(src_mem_opts.ptr());
            self.instruction(LocalGet(dst_mem.addr.idx));
            let cur_dst_ptr = self.local_set_new_tmp(dst_mem_opts.ptr());

            self.instruction(Loop(BlockType::Empty));

            // Translate the next element in the list
            let element_src = Source::Memory(Memory {
                opts: src_opts,
                offset: 0,
                addr: TempLocal::new(cur_src_ptr.idx, cur_src_ptr.ty),
            });
            let element_dst = Destination::Memory(Memory {
                opts: dst_opts,
                offset: 0,
                addr: TempLocal::new(cur_dst_ptr.idx, cur_dst_ptr.ty),
            });
            self.translate(src_element_ty, &element_src, dst_element_ty, &element_dst);

            // Update the two loop pointers
            if src_size > 0 {
                self.instruction(LocalGet(cur_src_ptr.idx));
                self.ptr_uconst(src_mem_opts, src_size);
                self.ptr_add(src_mem_opts);
                self.instruction(LocalSet(cur_src_ptr.idx));
            }
            if dst_size > 0 {
                self.instruction(LocalGet(cur_dst_ptr.idx));
                self.ptr_uconst(dst_mem_opts, dst_size);
                self.ptr_add(dst_mem_opts);
                self.instruction(LocalSet(cur_dst_ptr.idx));
            }

            // Update the remaining count, falling through to break out if it's zero
            // now.
            self.instruction(LocalGet(remaining.idx));
            self.ptr_iconst(src_mem_opts, -1);
            self.ptr_add(src_mem_opts);
            self.instruction(LocalTee(remaining.idx));
            self.ptr_br_if(src_mem_opts, 0);
            self.instruction(End); // end of loop
            self.instruction(End); // end of block

            self.free_temp_local(cur_dst_ptr);
            self.free_temp_local(cur_src_ptr);
            self.free_temp_local(remaining);
        }

        // Store the ptr/length in the desired destination
        match dst {
            Destination::Stack(s, _) => {
                self.instruction(LocalGet(dst_mem.addr.idx));
                self.stack_set(&s[..1], dst_mem_opts.ptr());
                self.convert_src_len_to_dst(src_len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
                self.stack_set(&s[1..], dst_mem_opts.ptr());
            }
            Destination::Memory(mem) => {
                self.instruction(LocalGet(mem.addr.idx));
                self.instruction(LocalGet(dst_mem.addr.idx));
                self.ptr_store(mem);
                self.instruction(LocalGet(mem.addr.idx));
                self.convert_src_len_to_dst(src_len.idx, src_mem_opts.ptr(), dst_mem_opts.ptr());
                self.ptr_store(&mem.bump(dst_mem_opts.ptr_size().into()));
            }
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }

        self.free_temp_local(src_len);
        self.free_temp_local(src_mem.addr);
        self.free_temp_local(dst_mem.addr);
    }

    fn calculate_list_byte_len(
        &mut self,
        opts: &LinearMemoryOptions,
        len_local: u32,
        elt_size: u32,
    ) -> TempLocal {
        // Zero-size types are easy to handle here because the byte size of the
        // destination is always zero.
        if elt_size == 0 {
            self.ptr_uconst(opts, 0);
            return self.local_set_new_tmp(opts.ptr());
        }

        // For one-byte elements in the destination the check here can be a bit
        // more optimal than the general case below. In these situations if the
        // source pointer type is 32-bit then we're guaranteed to not overflow,
        // so the source length is simply casted to the destination's type.
        //
        // If the source is 64-bit then all that needs to be checked is to
        // ensure that it does not have the upper 32-bits set.
        if elt_size == 1 {
            if let ValType::I64 = opts.ptr() {
                self.instruction(LocalGet(len_local));
                self.instruction(I64Const(32));
                self.instruction(I64ShrU);
                self.instruction(I32WrapI64);
                self.instruction(If(BlockType::Empty));
                self.trap(Trap::ListByteLengthOverflow);
                self.instruction(End);
            }
            self.instruction(LocalGet(len_local));
            return self.local_set_new_tmp(opts.ptr());
        }

        // The main check implemented by this function is to verify that
        // `src_len_local` does not exceed the 32-bit range. Byte sizes for
        // lists must always fit in 32-bits to get transferred to 32-bit
        // memories.
        self.instruction(Block(BlockType::Empty));
        self.instruction(Block(BlockType::Empty));
        self.instruction(LocalGet(len_local));
        match opts.ptr() {
            // The source's list length is guaranteed to be less than 32-bits
            // so simply extend it up to a 64-bit type for the multiplication
            // below.
            ValType::I32 => self.instruction(I64ExtendI32U),

            // If the source is a 64-bit memory then if the item length doesn't
            // fit in 32-bits the byte length definitely won't, so generate a
            // branch to our overflow trap here if any of the upper 32-bits are set.
            ValType::I64 => {
                self.instruction(I64Const(32));
                self.instruction(I64ShrU);
                self.instruction(I32WrapI64);
                self.instruction(BrIf(0));
                self.instruction(LocalGet(len_local));
            }

            _ => unreachable!(),
        }

        // Next perform a 64-bit multiplication with the element byte size that
        // is itself guaranteed to fit in 32-bits. The result is then checked
        // to see if we overflowed the 32-bit space. The two input operands to
        // the multiplication are guaranteed to be 32-bits at most which means
        // that this multiplication shouldn't overflow.
        //
        // The result of the multiplication is saved into a local as well to
        // get the result afterwards.
        self.instruction(I64Const(elt_size.into()));
        self.instruction(I64Mul);
        let tmp = self.local_tee_new_tmp(ValType::I64);
        // Branch to success if the upper 32-bits are zero, otherwise
        // fall-through to the trap.
        self.instruction(I64Const(32));
        self.instruction(I64ShrU);
        self.instruction(I64Eqz);
        self.instruction(BrIf(1));
        self.instruction(End);
        self.trap(Trap::ListByteLengthOverflow);
        self.instruction(End);

        // If a fresh local was used to store the result of the multiplication
        // then convert it down to 32-bits which should be guaranteed to not
        // lose information at this point.
        if opts.ptr() == ValType::I64 {
            tmp
        } else {
            self.instruction(LocalGet(tmp.idx));
            self.instruction(I32WrapI64);
            self.free_temp_local(tmp);
            self.local_set_new_tmp(ValType::I32)
        }
    }

    fn convert_src_len_to_dst(
        &mut self,
        src_len_local: u32,
        src_ptr_ty: ValType,
        dst_ptr_ty: ValType,
    ) {
        self.instruction(LocalGet(src_len_local));
        match (src_ptr_ty, dst_ptr_ty) {
            (ValType::I32, ValType::I64) => self.instruction(I64ExtendI32U),
            (ValType::I64, ValType::I32) => self.instruction(I32WrapI64),
            (src, dst) => assert_eq!(src, dst),
        }
    }

    fn translate_record(
        &mut self,
        src_ty: TypeRecordIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Record(r) => &self.types[*r],
            _ => panic!("expected a record"),
        };

        // TODO: subtyping
        assert_eq!(src_ty.fields.len(), dst_ty.fields.len());

        // First a map is made of the source fields to where they're coming
        // from (e.g. which offset or which locals). This map is keyed by the
        // fields' names
        let mut src_fields = HashMap::new();
        for (i, src) in src
            .record_field_srcs(self.types, src_ty.fields.iter().map(|f| f.ty))
            .enumerate()
        {
            let field = &src_ty.fields[i];
            src_fields.insert(&field.name, (src, &field.ty));
        }

        // .. and next translation is performed in the order of the destination
        // fields in case the destination is the stack to ensure that the stack
        // has the fields all in the right order.
        //
        // Note that the lookup in `src_fields` is an infallible lookup which
        // will panic if the field isn't found.
        //
        // TODO: should that lookup be fallible with subtyping?
        for (i, dst) in dst
            .record_field_dsts(self.types, dst_ty.fields.iter().map(|f| f.ty))
            .enumerate()
        {
            let field = &dst_ty.fields[i];
            let (src, src_ty) = &src_fields[&field.name];
            self.translate(src_ty, src, &field.ty, &dst);
        }
    }

    fn translate_flags(
        &mut self,
        src_ty: TypeFlagsIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Flags(r) => &self.types[*r],
            _ => panic!("expected a record"),
        };

        // TODO: subtyping
        //
        // Notably this implementation does not support reordering flags from
        // the source to the destination nor having more flags in the
        // destination. Currently this is a copy from source to destination
        // in-bulk. Otherwise reordering indices would have to have some sort of
        // fancy bit twiddling tricks or something like that.
        assert_eq!(src_ty.names, dst_ty.names);
        let cnt = src_ty.names.len();
        match FlagsSize::from_count(cnt) {
            FlagsSize::Size0 => {}
            FlagsSize::Size1 => {
                let mask = if cnt == 8 { 0xff } else { (1 << cnt) - 1 };
                self.convert_u8_mask(src, dst, mask);
            }
            FlagsSize::Size2 => {
                let mask = if cnt == 16 { 0xffff } else { (1 << cnt) - 1 };
                self.convert_u16_mask(src, dst, mask);
            }
            FlagsSize::Size4Plus(n) => {
                let srcs = src.record_field_srcs(self.types, (0..n).map(|_| InterfaceType::U32));
                let dsts = dst.record_field_dsts(self.types, (0..n).map(|_| InterfaceType::U32));
                let n = usize::from(n);
                for (i, (src, dst)) in srcs.zip(dsts).enumerate() {
                    let mask = if i == n - 1 && (cnt % 32 != 0) {
                        (1 << (cnt % 32)) - 1
                    } else {
                        0xffffffff
                    };
                    self.convert_u32_mask(&src, &dst, mask);
                }
            }
        }
    }

    fn translate_tuple(
        &mut self,
        src_ty: TypeTupleIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Tuple(t) => &self.types[*t],
            _ => panic!("expected a tuple"),
        };

        // TODO: subtyping
        assert_eq!(src_ty.types.len(), dst_ty.types.len());

        let srcs = src
            .record_field_srcs(self.types, src_ty.types.iter().copied())
            .zip(src_ty.types.iter());
        let dsts = dst
            .record_field_dsts(self.types, dst_ty.types.iter().copied())
            .zip(dst_ty.types.iter());
        for ((src, src_ty), (dst, dst_ty)) in srcs.zip(dsts) {
            self.translate(src_ty, &src, dst_ty, &dst);
        }
    }

    fn translate_variant(
        &mut self,
        src_ty: TypeVariantIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Variant(t) => &self.types[*t],
            _ => panic!("expected a variant"),
        };

        let src_info = variant_info(self.types, src_ty.cases.iter().map(|(_, c)| c.as_ref()));
        let dst_info = variant_info(self.types, dst_ty.cases.iter().map(|(_, c)| c.as_ref()));

        let iter = src_ty
            .cases
            .iter()
            .enumerate()
            .map(|(src_i, (src_case, src_case_ty))| {
                let dst_i = dst_ty
                    .cases
                    .iter()
                    .position(|(c, _)| c == src_case)
                    .unwrap();
                let dst_case_ty = &dst_ty.cases[dst_i];
                let src_i = u32::try_from(src_i).unwrap();
                let dst_i = u32::try_from(dst_i).unwrap();
                VariantCase {
                    src_i,
                    src_ty: src_case_ty.as_ref(),
                    dst_i,
                    dst_ty: dst_case_ty.as_ref(),
                }
            });
        self.convert_variant(src, &src_info, dst, &dst_info, iter);
    }

    fn translate_enum(
        &mut self,
        src_ty: TypeEnumIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Enum(t) => &self.types[*t],
            _ => panic!("expected an option"),
        };

        debug_assert_eq!(src_ty.info.size, dst_ty.info.size);
        debug_assert_eq!(src_ty.names.len(), dst_ty.names.len());
        debug_assert!(
            src_ty
                .names
                .iter()
                .zip(dst_ty.names.iter())
                .all(|(a, b)| a == b)
        );

        // Get the discriminant.
        match src {
            Source::Stack(s) => self.stack_get(&s.slice(0..1), ValType::I32),
            Source::Memory(mem) => match src_ty.info.size {
                DiscriminantSize::Size1 => self.i32_load8u(mem),
                DiscriminantSize::Size2 => self.i32_load16u(mem),
                DiscriminantSize::Size4 => self.i32_load(mem),
            },
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        let tmp = self.local_tee_new_tmp(ValType::I32);

        // Assert that the discriminant is valid.
        self.instruction(I32Const(i32::try_from(src_ty.names.len()).unwrap()));
        self.instruction(I32GtU);
        self.instruction(If(BlockType::Empty));
        self.trap(Trap::InvalidDiscriminant);
        self.instruction(End);

        // Save the discriminant to the destination.
        match dst {
            Destination::Stack(stack, _) => {
                self.local_get_tmp(&tmp);
                self.stack_set(&stack[..1], ValType::I32)
            }
            Destination::Memory(mem) => {
                self.push_dst_addr(dst);
                self.local_get_tmp(&tmp);
                match dst_ty.info.size {
                    DiscriminantSize::Size1 => self.i32_store8(mem),
                    DiscriminantSize::Size2 => self.i32_store16(mem),
                    DiscriminantSize::Size4 => self.i32_store(mem),
                }
            }
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
        self.free_temp_local(tmp);
    }

    fn translate_option(
        &mut self,
        src_ty: TypeOptionIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty].ty;
        let dst_ty = match dst_ty {
            InterfaceType::Option(t) => &self.types[*t].ty,
            _ => panic!("expected an option"),
        };
        let src_ty = Some(src_ty);
        let dst_ty = Some(dst_ty);

        let src_info = variant_info(self.types, [None, src_ty]);
        let dst_info = variant_info(self.types, [None, dst_ty]);

        self.convert_variant(
            src,
            &src_info,
            dst,
            &dst_info,
            [
                VariantCase {
                    src_i: 0,
                    dst_i: 0,
                    src_ty: None,
                    dst_ty: None,
                },
                VariantCase {
                    src_i: 1,
                    dst_i: 1,
                    src_ty,
                    dst_ty,
                },
            ]
            .into_iter(),
        );
    }

    fn translate_result(
        &mut self,
        src_ty: TypeResultIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let src_ty = &self.types[src_ty];
        let dst_ty = match dst_ty {
            InterfaceType::Result(t) => &self.types[*t],
            _ => panic!("expected a result"),
        };

        let src_info = variant_info(self.types, [src_ty.ok.as_ref(), src_ty.err.as_ref()]);
        let dst_info = variant_info(self.types, [dst_ty.ok.as_ref(), dst_ty.err.as_ref()]);

        self.convert_variant(
            src,
            &src_info,
            dst,
            &dst_info,
            [
                VariantCase {
                    src_i: 0,
                    dst_i: 0,
                    src_ty: src_ty.ok.as_ref(),
                    dst_ty: dst_ty.ok.as_ref(),
                },
                VariantCase {
                    src_i: 1,
                    dst_i: 1,
                    src_ty: src_ty.err.as_ref(),
                    dst_ty: dst_ty.err.as_ref(),
                },
            ]
            .into_iter(),
        );
    }

    fn convert_variant<'c>(
        &mut self,
        src: &Source<'_>,
        src_info: &VariantInfo,
        dst: &Destination,
        dst_info: &VariantInfo,
        src_cases: impl ExactSizeIterator<Item = VariantCase<'c>>,
    ) {
        // The outermost block is special since it has the result type of the
        // translation here. That will depend on the `dst`.
        let outer_block_ty = match dst {
            Destination::Stack(dst_flat, _) => match dst_flat.len() {
                0 => BlockType::Empty,
                1 => BlockType::Result(dst_flat[0]),
                _ => {
                    let ty = self.module.core_types.function(&[], &dst_flat);
                    BlockType::FunctionType(ty)
                }
            },
            Destination::Memory(_) => BlockType::Empty,
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        };
        self.instruction(Block(outer_block_ty));

        // After the outermost block generate a new block for each of the
        // remaining cases.
        let src_cases_len = src_cases.len();
        for _ in 0..src_cases_len - 1 {
            self.instruction(Block(BlockType::Empty));
        }

        // Generate a block for an invalid variant discriminant
        self.instruction(Block(BlockType::Empty));

        // And generate one final block that we'll be jumping out of with the
        // `br_table`
        self.instruction(Block(BlockType::Empty));

        // Load the discriminant
        match src {
            Source::Stack(s) => self.stack_get(&s.slice(0..1), ValType::I32),
            Source::Memory(mem) => match src_info.size {
                DiscriminantSize::Size1 => self.i32_load8u(mem),
                DiscriminantSize::Size2 => self.i32_load16u(mem),
                DiscriminantSize::Size4 => self.i32_load(mem),
            },
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }

        // Generate the `br_table` for the discriminant. Each case has an
        // offset of 1 to skip the trapping block.
        let mut targets = Vec::new();
        for i in 0..src_cases_len {
            targets.push((i + 1) as u32);
        }
        self.instruction(BrTable(targets[..].into(), 0));
        self.instruction(End); // end the `br_table` block

        self.trap(Trap::InvalidDiscriminant);
        self.instruction(End); // end the "invalid discriminant" block

        // Translate each case individually within its own block. Note that the
        // iteration order here places the first case in the innermost block
        // and the last case in the outermost block. This matches the order
        // of the jump targets in the `br_table` instruction.
        let src_cases_len = u32::try_from(src_cases_len).unwrap();
        for case in src_cases {
            let VariantCase {
                src_i,
                src_ty,
                dst_i,
                dst_ty,
            } = case;

            // Translate the discriminant here, noting that `dst_i` may be
            // different than `src_i`.
            self.push_dst_addr(dst);
            self.instruction(I32Const(dst_i as i32));
            match dst {
                Destination::Stack(stack, _) => self.stack_set(&stack[..1], ValType::I32),
                Destination::Memory(mem) => match dst_info.size {
                    DiscriminantSize::Size1 => self.i32_store8(mem),
                    DiscriminantSize::Size2 => self.i32_store16(mem),
                    DiscriminantSize::Size4 => self.i32_store(mem),
                },
                Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
            }

            let src_payload = src.payload_src(self.types, src_info, src_ty);
            let dst_payload = dst.payload_dst(self.types, dst_info, dst_ty);

            // Translate the payload of this case using the various types from
            // the dst/src.
            match (src_ty, dst_ty) {
                (Some(src_ty), Some(dst_ty)) => {
                    self.translate(src_ty, &src_payload, dst_ty, &dst_payload);
                }
                (None, None) => {}
                _ => unimplemented!(),
            }

            // If the results of this translation were placed on the stack then
            // the stack values may need to be padded with more zeros due to
            // this particular case being possibly smaller than the entire
            // variant. That's handled here by pushing remaining zeros after
            // accounting for the discriminant pushed as well as the results of
            // this individual payload.
            if let Destination::Stack(payload_results, _) = dst_payload {
                if let Destination::Stack(dst_results, _) = dst {
                    let remaining = &dst_results[1..][payload_results.len()..];
                    for ty in remaining {
                        match ty {
                            ValType::I32 => self.instruction(I32Const(0)),
                            ValType::I64 => self.instruction(I64Const(0)),
                            ValType::F32 => self.instruction(F32Const(0.0.into())),
                            ValType::F64 => self.instruction(F64Const(0.0.into())),
                            _ => unreachable!(),
                        }
                    }
                }
            }

            // Branch to the outermost block. Note that this isn't needed for
            // the outermost case since it simply falls through.
            if src_i != src_cases_len - 1 {
                self.instruction(Br(src_cases_len - src_i - 1));
            }
            self.instruction(End); // end this case's block
        }
    }

    fn translate_future(
        &mut self,
        src_ty: TypeFutureTableIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let dst_ty = match dst_ty {
            InterfaceType::Future(t) => *t,
            _ => panic!("expected a `Future`"),
        };
        let transfer = self.module.import_future_transfer();
        self.translate_handle(src_ty.as_u32(), src, dst_ty.as_u32(), dst, transfer);
    }

    fn translate_stream(
        &mut self,
        src_ty: TypeStreamTableIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let dst_ty = match dst_ty {
            InterfaceType::Stream(t) => *t,
            _ => panic!("expected a `Stream`"),
        };
        let transfer = self.module.import_stream_transfer();
        self.translate_handle(src_ty.as_u32(), src, dst_ty.as_u32(), dst, transfer);
    }

    fn translate_error_context(
        &mut self,
        src_ty: TypeComponentLocalErrorContextTableIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let dst_ty = match dst_ty {
            InterfaceType::ErrorContext(t) => *t,
            _ => panic!("expected an `ErrorContext`"),
        };
        let transfer = self.module.import_error_context_transfer();
        self.translate_handle(src_ty.as_u32(), src, dst_ty.as_u32(), dst, transfer);
    }

    fn translate_own(
        &mut self,
        src_ty: TypeResourceTableIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let dst_ty = match dst_ty {
            InterfaceType::Own(t) => *t,
            _ => panic!("expected an `Own`"),
        };
        let transfer = self.module.import_resource_transfer_own();
        self.translate_handle(src_ty.as_u32(), src, dst_ty.as_u32(), dst, transfer);
    }

    fn translate_borrow(
        &mut self,
        src_ty: TypeResourceTableIndex,
        src: &Source<'_>,
        dst_ty: &InterfaceType,
        dst: &Destination,
    ) {
        let dst_ty = match dst_ty {
            InterfaceType::Borrow(t) => *t,
            _ => panic!("expected an `Borrow`"),
        };

        let transfer = self.module.import_resource_transfer_borrow();
        self.translate_handle(src_ty.as_u32(), src, dst_ty.as_u32(), dst, transfer);
    }

    /// Translates the index `src`, which resides in the table `src_ty`, into
    /// and index within `dst_ty` and is stored at `dst`.
    ///
    /// Actual translation of the index happens in a wasmtime libcall, which a
    /// cranelift-generated trampoline to satisfy this import will call. The
    /// `transfer` function is an imported function which takes the src, src_ty,
    /// and dst_ty, and returns the dst index.
    fn translate_handle(
        &mut self,
        src_ty: u32,
        src: &Source<'_>,
        dst_ty: u32,
        dst: &Destination,
        transfer: FuncIndex,
    ) {
        self.push_dst_addr(dst);
        match src {
            Source::Memory(mem) => self.i32_load(mem),
            Source::Stack(stack) => self.stack_get(stack, ValType::I32),
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
        self.instruction(I32Const(src_ty as i32));
        self.instruction(I32Const(dst_ty as i32));
        self.instruction(Call(transfer.as_u32()));
        match dst {
            Destination::Memory(mem) => self.i32_store(mem),
            Destination::Stack(stack, _) => self.stack_set(stack, ValType::I32),
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn trap_if_not_flag(&mut self, flags_global: GlobalIndex, flag_to_test: i32, trap: Trap) {
        self.instruction(GlobalGet(flags_global.as_u32()));
        self.instruction(I32Const(flag_to_test));
        self.instruction(I32And);
        self.instruction(I32Eqz);
        self.instruction(If(BlockType::Empty));
        self.trap(trap);
        self.instruction(End);
    }

    fn assert_not_flag(&mut self, flags_global: GlobalIndex, flag_to_test: i32, msg: &'static str) {
        self.instruction(GlobalGet(flags_global.as_u32()));
        self.instruction(I32Const(flag_to_test));
        self.instruction(I32And);
        self.instruction(If(BlockType::Empty));
        self.trap(Trap::AssertFailed(msg));
        self.instruction(End);
    }

    fn set_flag(&mut self, flags_global: GlobalIndex, flag_to_set: i32, value: bool) {
        self.instruction(GlobalGet(flags_global.as_u32()));
        if value {
            self.instruction(I32Const(flag_to_set));
            self.instruction(I32Or);
        } else {
            self.instruction(I32Const(!flag_to_set));
            self.instruction(I32And);
        }
        self.instruction(GlobalSet(flags_global.as_u32()));
    }

    fn verify_aligned(&mut self, opts: &LinearMemoryOptions, addr_local: u32, align: u32) {
        // If the alignment is 1 then everything is trivially aligned and the
        // check can be omitted.
        if align == 1 {
            return;
        }
        self.instruction(LocalGet(addr_local));
        assert!(align.is_power_of_two());
        self.ptr_uconst(opts, align - 1);
        self.ptr_and(opts);
        self.ptr_if(opts, BlockType::Empty);
        self.trap(Trap::UnalignedPointer);
        self.instruction(End);
    }

    fn assert_aligned(&mut self, ty: &InterfaceType, mem: &Memory) {
        let mem_opts = mem.mem_opts();
        if !self.module.debug {
            return;
        }
        let align = self.types.align(mem_opts, ty);
        if align == 1 {
            return;
        }
        assert!(align.is_power_of_two());
        self.instruction(LocalGet(mem.addr.idx));
        self.ptr_uconst(mem_opts, mem.offset);
        self.ptr_add(mem_opts);
        self.ptr_uconst(mem_opts, align - 1);
        self.ptr_and(mem_opts);
        self.ptr_if(mem_opts, BlockType::Empty);
        self.trap(Trap::AssertFailed("pointer not aligned"));
        self.instruction(End);
    }

    fn malloc<'c>(&mut self, opts: &'c Options, size: MallocSize, align: u32) -> Memory<'c> {
        match &opts.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(mem_opts) => {
                let realloc = mem_opts.realloc.unwrap();
                self.ptr_uconst(mem_opts, 0);
                self.ptr_uconst(mem_opts, 0);
                self.ptr_uconst(mem_opts, align);
                match size {
                    MallocSize::Const(size) => self.ptr_uconst(mem_opts, size),
                    MallocSize::Local(idx) => self.instruction(LocalGet(idx)),
                }
                self.instruction(Call(realloc.as_u32()));
                let addr = self.local_set_new_tmp(mem_opts.ptr());
                self.memory_operand(opts, addr, align)
            }
        }
    }

    fn memory_operand<'c>(&mut self, opts: &'c Options, addr: TempLocal, align: u32) -> Memory<'c> {
        let ret = Memory {
            addr,
            offset: 0,
            opts,
        };
        self.verify_aligned(opts.data_model.unwrap_memory(), ret.addr.idx, align);
        ret
    }

    /// Generates a new local in this function of the `ty` specified,
    /// initializing it with the top value on the current wasm stack.
    ///
    /// The returned `TempLocal` must be freed after it is finished with
    /// `free_temp_local`.
    fn local_tee_new_tmp(&mut self, ty: ValType) -> TempLocal {
        self.gen_temp_local(ty, LocalTee)
    }

    /// Same as `local_tee_new_tmp` but initializes the local with `LocalSet`
    /// instead of `LocalTee`.
    fn local_set_new_tmp(&mut self, ty: ValType) -> TempLocal {
        self.gen_temp_local(ty, LocalSet)
    }

    fn local_get_tmp(&mut self, local: &TempLocal) {
        self.instruction(LocalGet(local.idx));
    }

    fn gen_temp_local(&mut self, ty: ValType, insn: fn(u32) -> Instruction<'static>) -> TempLocal {
        // First check to see if any locals are available in this function which
        // were previously generated but are no longer in use.
        if let Some(idx) = self.free_locals.get_mut(&ty).and_then(|v| v.pop()) {
            self.instruction(insn(idx));
            return TempLocal {
                ty,
                idx,
                needs_free: true,
            };
        }

        // Failing that generate a fresh new local.
        let locals = &mut self.module.funcs[self.result].locals;
        match locals.last_mut() {
            Some((cnt, prev_ty)) if ty == *prev_ty => *cnt += 1,
            _ => locals.push((1, ty)),
        }
        self.nlocals += 1;
        let idx = self.nlocals - 1;
        self.instruction(insn(idx));
        TempLocal {
            ty,
            idx,
            needs_free: true,
        }
    }

    /// Used to release a `TempLocal` from a particular lexical scope to allow
    /// its possible reuse in later scopes.
    fn free_temp_local(&mut self, mut local: TempLocal) {
        assert!(local.needs_free);
        self.free_locals
            .entry(local.ty)
            .or_insert(Vec::new())
            .push(local.idx);
        local.needs_free = false;
    }

    fn instruction(&mut self, instr: Instruction) {
        instr.encode(&mut self.code);
    }

    fn trap(&mut self, trap: Trap) {
        self.traps.push((self.code.len(), trap));
        self.instruction(Unreachable);
    }

    /// Flushes out the current `code` instructions (and `traps` if there are
    /// any) into the destination function.
    ///
    /// This is a noop if no instructions have been encoded yet.
    fn flush_code(&mut self) {
        if self.code.is_empty() {
            return;
        }
        self.module.funcs[self.result].body.push(Body::Raw(
            mem::take(&mut self.code),
            mem::take(&mut self.traps),
        ));
    }

    fn finish(mut self) {
        // Append the final `end` instruction which all functions require, and
        // then empty out the temporary buffer in `Compiler`.
        self.instruction(End);
        self.flush_code();

        // Flag the function as "done" which helps with an assert later on in
        // emission that everything was eventually finished.
        self.module.funcs[self.result].filled_in = true;
    }

    /// Fetches the value contained with the local specified by `stack` and
    /// converts it to `dst_ty`.
    ///
    /// This is only intended for use in primitive operations where `stack` is
    /// guaranteed to have only one local. The type of the local on the stack is
    /// then converted to `dst_ty` appropriately. Note that the types may be
    /// different due to the "flattening" of variant types.
    fn stack_get(&mut self, stack: &Stack<'_>, dst_ty: ValType) {
        assert_eq!(stack.locals.len(), 1);
        let (idx, src_ty) = stack.locals[0];
        self.instruction(LocalGet(idx));
        match (src_ty, dst_ty) {
            (ValType::I32, ValType::I32)
            | (ValType::I64, ValType::I64)
            | (ValType::F32, ValType::F32)
            | (ValType::F64, ValType::F64) => {}

            (ValType::I32, ValType::F32) => self.instruction(F32ReinterpretI32),
            (ValType::I64, ValType::I32) => {
                self.assert_i64_upper_bits_not_set(idx);
                self.instruction(I32WrapI64);
            }
            (ValType::I64, ValType::F64) => self.instruction(F64ReinterpretI64),
            (ValType::I64, ValType::F32) => {
                self.assert_i64_upper_bits_not_set(idx);
                self.instruction(I32WrapI64);
                self.instruction(F32ReinterpretI32);
            }

            // should not be possible given the `join` function for variants
            (ValType::I32, ValType::I64)
            | (ValType::I32, ValType::F64)
            | (ValType::F32, ValType::I32)
            | (ValType::F32, ValType::I64)
            | (ValType::F32, ValType::F64)
            | (ValType::F64, ValType::I32)
            | (ValType::F64, ValType::I64)
            | (ValType::F64, ValType::F32)

            // not used in the component model
            | (ValType::Ref(_), _)
            | (_, ValType::Ref(_))
            | (ValType::V128, _)
            | (_, ValType::V128) => {
                panic!("cannot get {dst_ty:?} from {src_ty:?} local");
            }
        }
    }

    fn assert_i64_upper_bits_not_set(&mut self, local: u32) {
        if !self.module.debug {
            return;
        }
        self.instruction(LocalGet(local));
        self.instruction(I64Const(32));
        self.instruction(I64ShrU);
        self.instruction(I32WrapI64);
        self.instruction(If(BlockType::Empty));
        self.trap(Trap::AssertFailed("upper bits are unexpectedly set"));
        self.instruction(End);
    }

    /// Converts the top value on the WebAssembly stack which has type
    /// `src_ty` to `dst_tys[0]`.
    ///
    /// This is only intended for conversion of primitives where the `dst_tys`
    /// list is known to be of length 1.
    fn stack_set(&mut self, dst_tys: &[ValType], src_ty: ValType) {
        assert_eq!(dst_tys.len(), 1);
        let dst_ty = dst_tys[0];
        match (src_ty, dst_ty) {
            (ValType::I32, ValType::I32)
            | (ValType::I64, ValType::I64)
            | (ValType::F32, ValType::F32)
            | (ValType::F64, ValType::F64) => {}

            (ValType::F32, ValType::I32) => self.instruction(I32ReinterpretF32),
            (ValType::I32, ValType::I64) => self.instruction(I64ExtendI32U),
            (ValType::F64, ValType::I64) => self.instruction(I64ReinterpretF64),
            (ValType::F32, ValType::I64) => {
                self.instruction(I32ReinterpretF32);
                self.instruction(I64ExtendI32U);
            }

            // should not be possible given the `join` function for variants
            (ValType::I64, ValType::I32)
            | (ValType::F64, ValType::I32)
            | (ValType::I32, ValType::F32)
            | (ValType::I64, ValType::F32)
            | (ValType::F64, ValType::F32)
            | (ValType::I32, ValType::F64)
            | (ValType::I64, ValType::F64)
            | (ValType::F32, ValType::F64)

            // not used in the component model
            | (ValType::Ref(_), _)
            | (_, ValType::Ref(_))
            | (ValType::V128, _)
            | (_, ValType::V128) => {
                panic!("cannot get {dst_ty:?} from {src_ty:?} local");
            }
        }
    }

    fn i32_load8u(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I32Load8U(mem.memarg(0)));
    }

    fn i32_load8s(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I32Load8S(mem.memarg(0)));
    }

    fn i32_load16u(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I32Load16U(mem.memarg(1)));
    }

    fn i32_load16s(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I32Load16S(mem.memarg(1)));
    }

    fn i32_load(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I32Load(mem.memarg(2)));
    }

    fn i64_load(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(I64Load(mem.memarg(3)));
    }

    fn ptr_load(&mut self, mem: &Memory) {
        if mem.mem_opts().memory64 {
            self.i64_load(mem);
        } else {
            self.i32_load(mem);
        }
    }

    fn ptr_add(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Add);
        } else {
            self.instruction(I32Add);
        }
    }

    fn ptr_sub(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Sub);
        } else {
            self.instruction(I32Sub);
        }
    }

    fn ptr_mul(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Mul);
        } else {
            self.instruction(I32Mul);
        }
    }

    fn ptr_ge_u(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64GeU);
        } else {
            self.instruction(I32GeU);
        }
    }

    fn ptr_lt_u(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64LtU);
        } else {
            self.instruction(I32LtU);
        }
    }

    fn ptr_shl(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Shl);
        } else {
            self.instruction(I32Shl);
        }
    }

    fn ptr_eqz(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Eqz);
        } else {
            self.instruction(I32Eqz);
        }
    }

    fn ptr_uconst(&mut self, opts: &LinearMemoryOptions, val: u32) {
        if opts.memory64 {
            self.instruction(I64Const(val.into()));
        } else {
            self.instruction(I32Const(val as i32));
        }
    }

    fn ptr_iconst(&mut self, opts: &LinearMemoryOptions, val: i32) {
        if opts.memory64 {
            self.instruction(I64Const(val.into()));
        } else {
            self.instruction(I32Const(val));
        }
    }

    fn ptr_eq(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Eq);
        } else {
            self.instruction(I32Eq);
        }
    }

    fn ptr_ne(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Ne);
        } else {
            self.instruction(I32Ne);
        }
    }

    fn ptr_and(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64And);
        } else {
            self.instruction(I32And);
        }
    }

    fn ptr_or(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Or);
        } else {
            self.instruction(I32Or);
        }
    }

    fn ptr_xor(&mut self, opts: &LinearMemoryOptions) {
        if opts.memory64 {
            self.instruction(I64Xor);
        } else {
            self.instruction(I32Xor);
        }
    }

    fn ptr_if(&mut self, opts: &LinearMemoryOptions, ty: BlockType) {
        if opts.memory64 {
            self.instruction(I64Const(0));
            self.instruction(I64Ne);
        }
        self.instruction(If(ty));
    }

    fn ptr_br_if(&mut self, opts: &LinearMemoryOptions, depth: u32) {
        if opts.memory64 {
            self.instruction(I64Const(0));
            self.instruction(I64Ne);
        }
        self.instruction(BrIf(depth));
    }

    fn f32_load(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(F32Load(mem.memarg(2)));
    }

    fn f64_load(&mut self, mem: &Memory) {
        self.instruction(LocalGet(mem.addr.idx));
        self.instruction(F64Load(mem.memarg(3)));
    }

    fn push_dst_addr(&mut self, dst: &Destination) {
        if let Destination::Memory(mem) = dst {
            self.instruction(LocalGet(mem.addr.idx));
        }
    }

    fn i32_store8(&mut self, mem: &Memory) {
        self.instruction(I32Store8(mem.memarg(0)));
    }

    fn i32_store16(&mut self, mem: &Memory) {
        self.instruction(I32Store16(mem.memarg(1)));
    }

    fn i32_store(&mut self, mem: &Memory) {
        self.instruction(I32Store(mem.memarg(2)));
    }

    fn i64_store(&mut self, mem: &Memory) {
        self.instruction(I64Store(mem.memarg(3)));
    }

    fn ptr_store(&mut self, mem: &Memory) {
        if mem.mem_opts().memory64 {
            self.i64_store(mem);
        } else {
            self.i32_store(mem);
        }
    }

    fn f32_store(&mut self, mem: &Memory) {
        self.instruction(F32Store(mem.memarg(2)));
    }

    fn f64_store(&mut self, mem: &Memory) {
        self.instruction(F64Store(mem.memarg(3)));
    }
}

impl<'a> Source<'a> {
    /// Given this `Source` returns an iterator over the `Source` for each of
    /// the component `fields` specified.
    ///
    /// This will automatically slice stack-based locals to the appropriate
    /// width for each component type and additionally calculate the appropriate
    /// offset for each memory-based type.
    fn record_field_srcs<'b>(
        &'b self,
        types: &'b ComponentTypesBuilder,
        fields: impl IntoIterator<Item = InterfaceType> + 'b,
    ) -> impl Iterator<Item = Source<'a>> + 'b
    where
        'a: 'b,
    {
        let mut offset = 0;
        fields.into_iter().map(move |ty| match self {
            Source::Memory(mem) => {
                let mem = next_field_offset(&mut offset, types, &ty, mem);
                Source::Memory(mem)
            }
            Source::Stack(stack) => {
                let cnt = types.flat_types(&ty).unwrap().len() as u32;
                offset += cnt;
                Source::Stack(stack.slice((offset - cnt) as usize..offset as usize))
            }
            Source::Struct(_) => todo!(),
            Source::Array(_) => todo!(),
        })
    }

    /// Returns the corresponding discriminant source and payload source f
    fn payload_src(
        &self,
        types: &ComponentTypesBuilder,
        info: &VariantInfo,
        case: Option<&InterfaceType>,
    ) -> Source<'a> {
        match self {
            Source::Stack(s) => {
                let flat_len = match case {
                    Some(case) => types.flat_types(case).unwrap().len(),
                    None => 0,
                };
                Source::Stack(s.slice(1..s.locals.len()).slice(0..flat_len))
            }
            Source::Memory(mem) => {
                let mem = if mem.mem_opts().memory64 {
                    mem.bump(info.payload_offset64)
                } else {
                    mem.bump(info.payload_offset32)
                };
                Source::Memory(mem)
            }
            Source::Struct(_) | Source::Array(_) => todo!("CM+GC"),
        }
    }

    fn opts(&self) -> &'a Options {
        match self {
            Source::Stack(s) => s.opts,
            Source::Memory(mem) => mem.opts,
            Source::Struct(s) => s.opts,
            Source::Array(a) => a.opts,
        }
    }
}

impl<'a> Destination<'a> {
    /// Same as `Source::record_field_srcs` but for destinations.
    fn record_field_dsts<'b, I>(
        &'b self,
        types: &'b ComponentTypesBuilder,
        fields: I,
    ) -> impl Iterator<Item = Destination<'b>> + use<'b, I>
    where
        'a: 'b,
        I: IntoIterator<Item = InterfaceType> + 'b,
    {
        let mut offset = 0;
        fields.into_iter().map(move |ty| match self {
            Destination::Memory(mem) => {
                let mem = next_field_offset(&mut offset, types, &ty, mem);
                Destination::Memory(mem)
            }
            Destination::Stack(s, opts) => {
                let cnt = types.flat_types(&ty).unwrap().len() as u32;
                offset += cnt;
                Destination::Stack(&s[(offset - cnt) as usize..offset as usize], opts)
            }
            Destination::Struct(_) => todo!(),
            Destination::Array(_) => todo!(),
        })
    }

    /// Returns the corresponding discriminant source and payload source f
    fn payload_dst(
        &self,
        types: &ComponentTypesBuilder,
        info: &VariantInfo,
        case: Option<&InterfaceType>,
    ) -> Destination<'_> {
        match self {
            Destination::Stack(s, opts) => {
                let flat_len = match case {
                    Some(case) => types.flat_types(case).unwrap().len(),
                    None => 0,
                };
                Destination::Stack(&s[1..][..flat_len], opts)
            }
            Destination::Memory(mem) => {
                let mem = if mem.mem_opts().memory64 {
                    mem.bump(info.payload_offset64)
                } else {
                    mem.bump(info.payload_offset32)
                };
                Destination::Memory(mem)
            }
            Destination::Struct(_) | Destination::Array(_) => todo!("CM+GC"),
        }
    }

    fn opts(&self) -> &'a Options {
        match self {
            Destination::Stack(_, opts) => opts,
            Destination::Memory(mem) => mem.opts,
            Destination::Struct(s) => s.opts,
            Destination::Array(a) => a.opts,
        }
    }
}

fn next_field_offset<'a>(
    offset: &mut u32,
    types: &ComponentTypesBuilder,
    field: &InterfaceType,
    mem: &Memory<'a>,
) -> Memory<'a> {
    let abi = types.canonical_abi(field);
    let offset = if mem.mem_opts().memory64 {
        abi.next_field64(offset)
    } else {
        abi.next_field32(offset)
    };
    mem.bump(offset)
}

impl<'a> Memory<'a> {
    fn memarg(&self, align: u32) -> MemArg {
        MemArg {
            offset: u64::from(self.offset),
            align,
            memory_index: self.mem_opts().memory.unwrap().as_u32(),
        }
    }

    fn bump(&self, offset: u32) -> Memory<'a> {
        Memory {
            opts: self.opts,
            addr: TempLocal::new(self.addr.idx, self.addr.ty),
            offset: self.offset + offset,
        }
    }
}

impl<'a> Stack<'a> {
    fn slice(&self, range: Range<usize>) -> Stack<'a> {
        Stack {
            locals: &self.locals[range],
            opts: self.opts,
        }
    }
}

struct VariantCase<'a> {
    src_i: u32,
    src_ty: Option<&'a InterfaceType>,
    dst_i: u32,
    dst_ty: Option<&'a InterfaceType>,
}

fn variant_info<'a, I>(types: &ComponentTypesBuilder, cases: I) -> VariantInfo
where
    I: IntoIterator<Item = Option<&'a InterfaceType>>,
    I::IntoIter: ExactSizeIterator,
{
    VariantInfo::new(
        cases
            .into_iter()
            .map(|ty| ty.map(|ty| types.canonical_abi(ty))),
    )
    .0
}

enum MallocSize {
    Const(u32),
    Local(u32),
}

struct WasmString<'a> {
    ptr: TempLocal,
    len: TempLocal,
    opts: &'a Options,
}

struct TempLocal {
    idx: u32,
    ty: ValType,
    needs_free: bool,
}

impl TempLocal {
    fn new(idx: u32, ty: ValType) -> TempLocal {
        TempLocal {
            idx,
            ty,
            needs_free: false,
        }
    }
}

impl std::ops::Drop for TempLocal {
    fn drop(&mut self) {
        if self.needs_free {
            panic!("temporary local not free'd");
        }
    }
}

impl From<FlatType> for ValType {
    fn from(ty: FlatType) -> ValType {
        match ty {
            FlatType::I32 => ValType::I32,
            FlatType::I64 => ValType::I64,
            FlatType::F32 => ValType::F32,
            FlatType::F64 => ValType::F64,
        }
    }
}
