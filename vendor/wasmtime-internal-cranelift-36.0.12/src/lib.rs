//! Support for compiling with Cranelift.
//!
//! This crate provides an implementation of the `wasmtime_environ::Compiler`
//! and `wasmtime_environ::CompilerBuilder` traits.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

// See documentation in crates/wasmtime/src/runtime.rs for why this is
// selectively enabled here.
#![warn(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use cranelift_codegen::{
    FinalizedMachReloc, FinalizedRelocTarget, MachTrap, binemit,
    cursor::FuncCursor,
    ir::{self, AbiParam, ArgumentPurpose, ExternalName, InstBuilder, Signature, TrapCode},
    isa::{CallConv, TargetIsa},
    settings,
};
use cranelift_entity::PrimaryMap;

use target_lexicon::Architecture;
use wasmtime_environ::{
    BuiltinFunctionIndex, FlagValue, FuncIndex, RelocationTarget, Trap, TrapInformation, Tunables,
    WasmFuncType, WasmHeapTopType, WasmHeapType, WasmValType,
};

pub use builder::builder;

pub mod isa_builder;
mod obj;
pub use obj::*;
mod compiled_function;
pub use compiled_function::*;

mod bounds_checks;
mod builder;
mod compiler;
mod debug;
mod func_environ;
mod translate;

use self::compiler::Compiler;

const TRAP_INTERNAL_ASSERT: TrapCode = TrapCode::unwrap_user(1);
const TRAP_OFFSET: u8 = 2;
pub const TRAP_ALWAYS: TrapCode =
    TrapCode::unwrap_user(Trap::AlwaysTrapAdapter as u8 + TRAP_OFFSET);
pub const TRAP_CANNOT_ENTER: TrapCode =
    TrapCode::unwrap_user(Trap::CannotEnterComponent as u8 + TRAP_OFFSET);
pub const TRAP_INDIRECT_CALL_TO_NULL: TrapCode =
    TrapCode::unwrap_user(Trap::IndirectCallToNull as u8 + TRAP_OFFSET);
pub const TRAP_BAD_SIGNATURE: TrapCode =
    TrapCode::unwrap_user(Trap::BadSignature as u8 + TRAP_OFFSET);
pub const TRAP_NULL_REFERENCE: TrapCode =
    TrapCode::unwrap_user(Trap::NullReference as u8 + TRAP_OFFSET);
pub const TRAP_ALLOCATION_TOO_LARGE: TrapCode =
    TrapCode::unwrap_user(Trap::AllocationTooLarge as u8 + TRAP_OFFSET);
pub const TRAP_ARRAY_OUT_OF_BOUNDS: TrapCode =
    TrapCode::unwrap_user(Trap::ArrayOutOfBounds as u8 + TRAP_OFFSET);
pub const TRAP_UNREACHABLE: TrapCode =
    TrapCode::unwrap_user(Trap::UnreachableCodeReached as u8 + TRAP_OFFSET);
pub const TRAP_HEAP_MISALIGNED: TrapCode =
    TrapCode::unwrap_user(Trap::HeapMisaligned as u8 + TRAP_OFFSET);
pub const TRAP_TABLE_OUT_OF_BOUNDS: TrapCode =
    TrapCode::unwrap_user(Trap::TableOutOfBounds as u8 + TRAP_OFFSET);
pub const TRAP_UNHANDLED_TAG: TrapCode =
    TrapCode::unwrap_user(Trap::UnhandledTag as u8 + TRAP_OFFSET);
pub const TRAP_CONTINUATION_ALREADY_CONSUMED: TrapCode =
    TrapCode::unwrap_user(Trap::ContinuationAlreadyConsumed as u8 + TRAP_OFFSET);
pub const TRAP_CAST_FAILURE: TrapCode =
    TrapCode::unwrap_user(Trap::CastFailure as u8 + TRAP_OFFSET);

/// Creates a new cranelift `Signature` with no wasm params/results for the
/// given calling convention.
///
/// This will add the default vmctx/etc parameters to the signature returned.
fn blank_sig(isa: &dyn TargetIsa, call_conv: CallConv) -> ir::Signature {
    let pointer_type = isa.pointer_type();
    let mut sig = ir::Signature::new(call_conv);
    // Add the caller/callee `vmctx` parameters.
    sig.params.push(ir::AbiParam::special(
        pointer_type,
        ir::ArgumentPurpose::VMContext,
    ));
    sig.params.push(ir::AbiParam::new(pointer_type));
    return sig;
}

/// Emit code for the following unbarriered memory write of the given type:
///
/// ```ignore
/// *(base + offset) = value
/// ```
///
/// This is intended to be used with things like `ValRaw` and the array calling
/// convention.
fn unbarriered_store_type_at_offset(
    pos: &mut FuncCursor,
    flags: ir::MemFlags,
    base: ir::Value,
    offset: i32,
    value: ir::Value,
) {
    pos.ins().store(flags, value, base, offset);
}

/// Emit code to do the following unbarriered memory read of the given type and
/// with the given flags:
///
/// ```ignore
/// result = *(base + offset)
/// ```
///
/// This is intended to be used with things like `ValRaw` and the array calling
/// convention.
fn unbarriered_load_type_at_offset(
    isa: &dyn TargetIsa,
    pos: &mut FuncCursor,
    ty: WasmValType,
    flags: ir::MemFlags,
    base: ir::Value,
    offset: i32,
) -> ir::Value {
    let ir_ty = value_type(isa, ty);
    pos.ins().load(ir_ty, flags, base, offset)
}

/// Returns the corresponding cranelift type for the provided wasm type.
fn value_type(isa: &dyn TargetIsa, ty: WasmValType) -> ir::types::Type {
    match ty {
        WasmValType::I32 => ir::types::I32,
        WasmValType::I64 => ir::types::I64,
        WasmValType::F32 => ir::types::F32,
        WasmValType::F64 => ir::types::F64,
        WasmValType::V128 => ir::types::I8X16,
        WasmValType::Ref(rt) => reference_type(rt.heap_type, isa.pointer_type()),
    }
}

/// Get the Cranelift signature for all array-call functions, that is:
///
/// ```ignore
/// unsafe extern "C" fn(
///     callee_vmctx: *mut VMOpaqueContext,
///     caller_vmctx: *mut VMOpaqueContext,
///     values_ptr: *mut ValRaw,
///     values_len: usize,
/// )
/// ```
///
/// This signature uses the target's default calling convention.
///
/// Note that regardless of the Wasm function type, the array-call calling
/// convention always uses that same signature.
fn array_call_signature(isa: &dyn TargetIsa) -> ir::Signature {
    let mut sig = blank_sig(isa, CallConv::triple_default(isa.triple()));
    // The array-call signature has an added parameter for the `values_vec`
    // input/output buffer in addition to the size of the buffer, in units
    // of `ValRaw`.
    sig.params.push(ir::AbiParam::new(isa.pointer_type()));
    sig.params.push(ir::AbiParam::new(isa.pointer_type()));
    // boolean return value of whether this function trapped
    sig.returns.push(ir::AbiParam::new(ir::types::I8));
    sig
}

/// Get the internal Wasm calling convention for the target/tunables combo
fn wasm_call_conv(isa: &dyn TargetIsa, tunables: &Tunables) -> CallConv {
    // The default calling convention is `CallConv::Tail` to enable the use of
    // tail calls in modules when needed. Note that this is used even if the
    // tail call proposal is disabled in wasm. This is not interacted with on
    // the host so it's purely an internal detail of wasm itself.
    //
    // The Winch calling convention is used instead when generating trampolines
    // which call Winch-generated functions. The winch calling convention is
    // only implemented for x64 and aarch64, so assert that here and panic on
    // other architectures.
    if tunables.winch_callable {
        assert!(
            matches!(
                isa.triple().architecture,
                Architecture::X86_64 | Architecture::Aarch64(_)
            ),
            "The Winch calling convention is only implemented for x86_64 and aarch64"
        );
        CallConv::Winch
    } else {
        CallConv::Tail
    }
}

/// Get the internal Wasm calling convention signature for the given type.
fn wasm_call_signature(
    isa: &dyn TargetIsa,
    wasm_func_ty: &WasmFuncType,
    tunables: &Tunables,
) -> ir::Signature {
    let call_conv = wasm_call_conv(isa, tunables);
    let mut sig = blank_sig(isa, call_conv);
    let cvt = |ty: &WasmValType| ir::AbiParam::new(value_type(isa, *ty));
    sig.params.extend(wasm_func_ty.params().iter().map(&cvt));
    sig.returns.extend(wasm_func_ty.returns().iter().map(&cvt));
    sig
}

/// Returns the reference type to use for the provided wasm type.
fn reference_type(wasm_ht: WasmHeapType, pointer_type: ir::Type) -> ir::Type {
    match wasm_ht.top() {
        WasmHeapTopType::Func => pointer_type,
        WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => ir::types::I32,
        WasmHeapTopType::Cont =>
        // TODO(10248) This is added in a follow-up PR
        {
            unimplemented!("codegen for stack switching types not implemented, yet")
        }
    }
}

// List of namespaces which are processed in `mach_reloc_to_reloc` below.

/// Namespace corresponding to wasm functions, the index is the index of the
/// defined function that's being referenced.
pub const NS_WASM_FUNC: u32 = 0;

/// Namespace for builtin function trampolines. The index is the index of the
/// builtin that's being referenced. These trampolines invoke the real host
/// function through an indirect function call loaded by the `VMContext`.
pub const NS_WASMTIME_BUILTIN: u32 = 1;

/// Namespace used to when a call from Pulley to the host is being made. This is
/// used with a `colocated: false` name to trigger codegen for a special opcode
/// for pulley-to-host communication. The index of the functions used in this
/// namespace correspond to the function signature of `for_each_host_signature!`
/// in the pulley_interpreter crate.
pub const NS_PULLEY_HOSTCALL: u32 = 2;

/// A record of a relocation to perform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relocation {
    /// The relocation code.
    pub reloc: binemit::Reloc,
    /// Relocation target.
    pub reloc_target: RelocationTarget,
    /// The offset where to apply the relocation.
    pub offset: binemit::CodeOffset,
    /// The addend to add to the relocation value.
    pub addend: binemit::Addend,
}

/// Converts cranelift_codegen settings to the wasmtime_environ equivalent.
pub fn clif_flags_to_wasmtime(
    flags: impl IntoIterator<Item = settings::Value>,
) -> Vec<(&'static str, FlagValue<'static>)> {
    flags
        .into_iter()
        .map(|val| (val.name, to_flag_value(&val)))
        .collect()
}

fn to_flag_value(v: &settings::Value) -> FlagValue<'static> {
    match v.kind() {
        settings::SettingKind::Enum => FlagValue::Enum(v.as_enum().unwrap()),
        settings::SettingKind::Num => FlagValue::Num(v.as_num().unwrap()),
        settings::SettingKind::Bool => FlagValue::Bool(v.as_bool().unwrap()),
        settings::SettingKind::Preset => unreachable!(),
    }
}

/// Converts machine traps to trap information.
pub fn mach_trap_to_trap(trap: &MachTrap) -> Option<TrapInformation> {
    let &MachTrap { offset, code } = trap;
    Some(TrapInformation {
        code_offset: offset,
        trap_code: clif_trap_to_env_trap(code)?,
    })
}

fn clif_trap_to_env_trap(trap: ir::TrapCode) -> Option<Trap> {
    Some(match trap {
        ir::TrapCode::STACK_OVERFLOW => Trap::StackOverflow,
        ir::TrapCode::HEAP_OUT_OF_BOUNDS => Trap::MemoryOutOfBounds,
        ir::TrapCode::INTEGER_OVERFLOW => Trap::IntegerOverflow,
        ir::TrapCode::INTEGER_DIVISION_BY_ZERO => Trap::IntegerDivisionByZero,
        ir::TrapCode::BAD_CONVERSION_TO_INTEGER => Trap::BadConversionToInteger,

        // These do not get converted to wasmtime traps, since they
        // shouldn't ever be hit in theory. Instead of catching and handling
        // these, we let the signal crash the process.
        TRAP_INTERNAL_ASSERT => return None,

        other => Trap::from_u8(other.as_raw().get() - TRAP_OFFSET).unwrap(),
    })
}

/// Converts machine relocations to relocation information
/// to perform.
fn mach_reloc_to_reloc(
    reloc: &FinalizedMachReloc,
    name_map: &PrimaryMap<ir::UserExternalNameRef, ir::UserExternalName>,
) -> Relocation {
    let &FinalizedMachReloc {
        offset,
        kind,
        ref target,
        addend,
    } = reloc;
    let reloc_target = match *target {
        FinalizedRelocTarget::ExternalName(ExternalName::User(user_func_ref)) => {
            let name = &name_map[user_func_ref];
            match name.namespace {
                NS_WASM_FUNC => RelocationTarget::Wasm(FuncIndex::from_u32(name.index)),
                NS_WASMTIME_BUILTIN => {
                    RelocationTarget::Builtin(BuiltinFunctionIndex::from_u32(name.index))
                }
                NS_PULLEY_HOSTCALL => RelocationTarget::PulleyHostcall(name.index),
                _ => panic!("unknown namespace {}", name.namespace),
            }
        }
        FinalizedRelocTarget::ExternalName(ExternalName::LibCall(libcall)) => {
            // We should have avoided any code that needs this style of libcalls
            // in the Wasm-to-Cranelift translator.
            panic!("unexpected libcall {libcall:?}");
        }
        _ => panic!("unrecognized external name"),
    };
    Relocation {
        reloc: kind,
        reloc_target,
        offset,
        addend,
    }
}

/// Helper structure for creating a `Signature` for all builtins.
struct BuiltinFunctionSignatures {
    pointer_type: ir::Type,

    host_call_conv: CallConv,
    wasm_call_conv: CallConv,
    argument_extension: ir::ArgumentExtension,
}

impl BuiltinFunctionSignatures {
    fn new(compiler: &Compiler) -> Self {
        Self {
            pointer_type: compiler.isa().pointer_type(),
            host_call_conv: CallConv::triple_default(compiler.isa().triple()),
            wasm_call_conv: wasm_call_conv(compiler.isa(), compiler.tunables()),
            argument_extension: compiler.isa().default_argument_extension(),
        }
    }

    fn vmctx(&self) -> AbiParam {
        AbiParam::special(self.pointer_type, ArgumentPurpose::VMContext)
    }

    fn pointer(&self) -> AbiParam {
        AbiParam::new(self.pointer_type)
    }

    fn u32(&self) -> AbiParam {
        AbiParam::new(ir::types::I32)
    }

    fn u64(&self) -> AbiParam {
        AbiParam::new(ir::types::I64)
    }

    fn f32(&self) -> AbiParam {
        AbiParam::new(ir::types::F32)
    }

    fn f64(&self) -> AbiParam {
        AbiParam::new(ir::types::F64)
    }

    fn u8(&self) -> AbiParam {
        AbiParam::new(ir::types::I8)
    }

    fn i8x16(&self) -> AbiParam {
        AbiParam::new(ir::types::I8X16)
    }

    fn f32x4(&self) -> AbiParam {
        AbiParam::new(ir::types::F32X4)
    }

    fn f64x2(&self) -> AbiParam {
        AbiParam::new(ir::types::F64X2)
    }

    fn bool(&self) -> AbiParam {
        AbiParam::new(ir::types::I8)
    }

    fn wasm_signature(&self, builtin: BuiltinFunctionIndex) -> Signature {
        let mut _cur = 0;
        macro_rules! iter {
            (
                $(
                    $( #[$attr:meta] )*
                    $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
                )*
            ) => {
                $(
                    $( #[$attr] )*
                    if _cur == builtin.index() {
                        return Signature {
                            params: vec![ $( self.$param() ),* ],
                            returns: vec![ $( self.$result() )? ],
                            call_conv: self.wasm_call_conv,
                        };
                    }
                    _cur += 1;
                )*
            };
        }

        wasmtime_environ::foreach_builtin_function!(iter);

        unreachable!();
    }

    fn host_signature(&self, builtin: BuiltinFunctionIndex) -> Signature {
        let mut sig = self.wasm_signature(builtin);
        sig.call_conv = self.host_call_conv;

        // Once we're declaring the signature of a host function we must
        // respect the default ABI of the platform which is where argument
        // extension of params/results may come into play.
        for arg in sig.params.iter_mut().chain(sig.returns.iter_mut()) {
            if arg.value_type.is_int() {
                arg.extension = self.argument_extension;
            }
        }

        sig
    }
}

/// If this bit is set on a GC reference, then the GC reference is actually an
/// unboxed `i31`.
///
/// Must be kept in sync with
/// `crate::runtime::vm::gc::VMGcRef::I31_REF_DISCRIMINANT`.
const I31_REF_DISCRIMINANT: u32 = 1;

/// Like `Option<T>` but specifically for passing information about transitions
/// from reachable to unreachable state and the like from callees to callers.
///
/// Marked `must_use` to force callers to update
/// `FuncTranslationStacks::reachable` as necessary.
#[derive(PartialEq, Eq)]
#[must_use]
enum Reachability<T> {
    /// The Wasm execution state is reachable, here is a `T`.
    Reachable(T),
    /// The Wasm execution state has been determined to be statically
    /// unreachable. It is the receiver of this value's responsibility to update
    /// `FuncTranslationStacks::reachable` as necessary.
    Unreachable,
}
