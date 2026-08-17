use crate::{Tunables, WasmResult, wasm_unsupported};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::{fmt, ops::Range};
use cranelift_entity::entity_impl;
use serde_derive::{Deserialize, Serialize};
use smallvec::SmallVec;

/// A trait for things that can trace all type-to-type edges, aka all type
/// indices within this thing.
pub trait TypeTrace {
    /// Visit each edge.
    ///
    /// The function can break out of tracing by returning `Err(E)`.
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>;

    /// Visit each edge, mutably.
    ///
    /// Allows updating edges.
    ///
    /// The function can break out of tracing by returning `Err(E)`.
    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>;

    /// Trace all `VMSharedTypeIndex` edges, ignoring other edges.
    fn trace_engine_indices<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(VMSharedTypeIndex) -> Result<(), E>,
    {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(idx) => func(idx),
            EngineOrModuleTypeIndex::Module(_) | EngineOrModuleTypeIndex::RecGroup(_) => Ok(()),
        })
    }

    /// Canonicalize `self` by rewriting all type references inside `self` from
    /// module-level interned type indices to engine-level interned type
    /// indices.
    ///
    /// This produces types that are suitable for usage by the runtime (only
    /// contains `VMSharedTypeIndex` type references).
    ///
    /// This does not produce types that are suitable for hash consing types
    /// (must have recgroup-relative indices for type indices referencing other
    /// types in the same recgroup).
    fn canonicalize_for_runtime_usage<F>(&mut self, module_to_engine: &mut F)
    where
        F: FnMut(ModuleInternedTypeIndex) -> VMSharedTypeIndex,
    {
        self.trace_mut::<_, ()>(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(module_index) => {
                let engine_index = module_to_engine(*module_index);
                *idx = EngineOrModuleTypeIndex::Engine(engine_index);
                Ok(())
            }
            EngineOrModuleTypeIndex::RecGroup(_) => {
                panic!("should not already be canonicalized for hash consing")
            }
        })
        .unwrap()
    }

    /// Is this type canonicalized for runtime usage?
    fn is_canonicalized_for_runtime_usage(&self) -> bool {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(_) | EngineOrModuleTypeIndex::RecGroup(_) => Err(()),
        })
        .is_ok()
    }

    /// Canonicalize `self` by rewriting all type references inside `self` from
    /// module-level interned type indices to either engine-level interned type
    /// indices or recgroup-relative indices.
    ///
    /// This produces types that are suitable for hash consing and deduplicating
    /// recgroups (types may have recgroup-relative indices for references to
    /// other types within the same recgroup).
    ///
    /// This does *not* produce types that are suitable for usage by the runtime
    /// (only contain `VMSharedTypeIndex` type references).
    fn canonicalize_for_hash_consing<F>(
        &mut self,
        rec_group_range: Range<ModuleInternedTypeIndex>,
        module_to_engine: &mut F,
    ) where
        F: FnMut(ModuleInternedTypeIndex) -> VMSharedTypeIndex,
    {
        self.trace_mut::<_, ()>(&mut |idx| match *idx {
            EngineOrModuleTypeIndex::Engine(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(module_index) => {
                *idx = if rec_group_range.start <= module_index {
                    // Any module index within the recursion group gets
                    // translated into a recgroup-relative index.
                    debug_assert!(module_index < rec_group_range.end);
                    let relative = module_index.as_u32() - rec_group_range.start.as_u32();
                    let relative = RecGroupRelativeTypeIndex::from_u32(relative);
                    EngineOrModuleTypeIndex::RecGroup(relative)
                } else {
                    // Cross-group indices are translated directly into
                    // `VMSharedTypeIndex`es.
                    debug_assert!(module_index < rec_group_range.start);
                    EngineOrModuleTypeIndex::Engine(module_to_engine(module_index))
                };
                Ok(())
            }
            EngineOrModuleTypeIndex::RecGroup(_) => {
                panic!("should not already be canonicalized for hash consing")
            }
        })
        .unwrap()
    }

    /// Is this type canonicalized for hash consing?
    fn is_canonicalized_for_hash_consing(&self) -> bool {
        self.trace(&mut |idx| match idx {
            EngineOrModuleTypeIndex::Engine(_) | EngineOrModuleTypeIndex::RecGroup(_) => Ok(()),
            EngineOrModuleTypeIndex::Module(_) => Err(()),
        })
        .is_ok()
    }
}

/// WebAssembly value type -- equivalent of `wasmparser::ValType`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasmValType {
    /// I32 type
    I32,
    /// I64 type
    I64,
    /// F32 type
    F32,
    /// F64 type
    F64,
    /// V128 type
    V128,
    /// Reference type
    Ref(WasmRefType),
}

impl fmt::Display for WasmValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WasmValType::I32 => write!(f, "i32"),
            WasmValType::I64 => write!(f, "i64"),
            WasmValType::F32 => write!(f, "f32"),
            WasmValType::F64 => write!(f, "f64"),
            WasmValType::V128 => write!(f, "v128"),
            WasmValType::Ref(rt) => write!(f, "{rt}"),
        }
    }
}

impl TypeTrace for WasmValType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmValType::Ref(r) => r.trace(func),
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::F32
            | WasmValType::F64
            | WasmValType::V128 => Ok(()),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmValType::Ref(r) => r.trace_mut(func),
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::F32
            | WasmValType::F64
            | WasmValType::V128 => Ok(()),
        }
    }
}

impl WasmValType {
    /// Is this a type that is represented as a `VMGcRef`?
    #[inline]
    pub fn is_vmgcref_type(&self) -> bool {
        match self {
            WasmValType::Ref(r) => r.is_vmgcref_type(),
            _ => false,
        }
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a type that actually refers to an object allocated in a
    /// GC heap?
    #[inline]
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        match self {
            WasmValType::Ref(r) => r.is_vmgcref_type_and_not_i31(),
            _ => false,
        }
    }

    fn trampoline_type(&self) -> Self {
        match self {
            WasmValType::Ref(r) => WasmValType::Ref(WasmRefType {
                nullable: true,
                heap_type: r.heap_type.top().into(),
            }),
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::F32
            | WasmValType::F64
            | WasmValType::V128 => *self,
        }
    }

    /// Attempt to build a `WasmValType` with the passed number of bits.
    ///
    /// Panics if the number of bits doesn't map to a WASM int type.
    pub fn int_from_bits(bits: u8) -> Self {
        match bits {
            32 => Self::I32,
            64 => Self::I64,
            size => panic!("invalid int bits for WasmValType: {size}"),
        }
    }

    /// Returns the contained reference type.
    ///
    /// Panics if the value type is not a vmgcref
    pub fn unwrap_ref_type(&self) -> WasmRefType {
        match self {
            WasmValType::Ref(ref_type) => *ref_type,
            _ => panic!("Called WasmValType::unwrap_ref_type on non-reference type"),
        }
    }
}

/// WebAssembly reference type -- equivalent of `wasmparser`'s RefType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WasmRefType {
    /// Whether or not this reference is nullable.
    pub nullable: bool,
    /// The heap type that this reference contains.
    pub heap_type: WasmHeapType,
}

impl TypeTrace for WasmRefType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.heap_type.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.heap_type.trace_mut(func)
    }
}

impl WasmRefType {
    /// Shorthand for `externref`
    pub const EXTERNREF: WasmRefType = WasmRefType {
        nullable: true,
        heap_type: WasmHeapType::Extern,
    };
    /// Shorthand for `funcref`
    pub const FUNCREF: WasmRefType = WasmRefType {
        nullable: true,
        heap_type: WasmHeapType::Func,
    };

    /// Is this a type that is represented as a `VMGcRef`?
    #[inline]
    pub fn is_vmgcref_type(&self) -> bool {
        self.heap_type.is_vmgcref_type()
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a type that actually refers to an object allocated in a
    /// GC heap?
    #[inline]
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        self.heap_type.is_vmgcref_type_and_not_i31()
    }
}

impl fmt::Display for WasmRefType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::FUNCREF => write!(f, "funcref"),
            Self::EXTERNREF => write!(f, "externref"),
            _ => {
                if self.nullable {
                    write!(f, "(ref null {})", self.heap_type)
                } else {
                    write!(f, "(ref {})", self.heap_type)
                }
            }
        }
    }
}

/// An interned type index, either at the module or engine level.
///
/// Roughly equivalent to `wasmparser::UnpackedIndex`, although doesn't have to
/// concern itself with recursion-group-local indices.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineOrModuleTypeIndex {
    /// An index within an engine, canonicalized among all modules that can
    /// interact with each other.
    Engine(VMSharedTypeIndex),

    /// An index within the current Wasm module, canonicalized within just this
    /// current module.
    Module(ModuleInternedTypeIndex),

    /// An index within the containing type's rec group. This is only used when
    /// hashing and canonicalizing rec groups, and should never appear outside
    /// of the engine's type registry.
    RecGroup(RecGroupRelativeTypeIndex),
}

impl From<ModuleInternedTypeIndex> for EngineOrModuleTypeIndex {
    #[inline]
    fn from(i: ModuleInternedTypeIndex) -> Self {
        Self::Module(i)
    }
}

impl From<VMSharedTypeIndex> for EngineOrModuleTypeIndex {
    #[inline]
    fn from(i: VMSharedTypeIndex) -> Self {
        Self::Engine(i)
    }
}

impl From<RecGroupRelativeTypeIndex> for EngineOrModuleTypeIndex {
    #[inline]
    fn from(i: RecGroupRelativeTypeIndex) -> Self {
        Self::RecGroup(i)
    }
}

impl fmt::Display for EngineOrModuleTypeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(i) => write!(f, "(engine {})", i.bits()),
            Self::Module(i) => write!(f, "(module {})", i.as_u32()),
            Self::RecGroup(i) => write!(f, "(recgroup {})", i.as_u32()),
        }
    }
}

impl EngineOrModuleTypeIndex {
    /// Is this an engine-level type index?
    pub fn is_engine_type_index(self) -> bool {
        matches!(self, Self::Engine(_))
    }

    /// Get the underlying engine-level type index, if any.
    pub fn as_engine_type_index(self) -> Option<VMSharedTypeIndex> {
        match self {
            Self::Engine(e) => Some(e),
            Self::RecGroup(_) | Self::Module(_) => None,
        }
    }

    /// Get the underlying engine-level type index, or panic.
    #[track_caller]
    pub fn unwrap_engine_type_index(self) -> VMSharedTypeIndex {
        match self.as_engine_type_index() {
            Some(x) => x,
            None => panic!("`unwrap_engine_type_index` on {self:?}"),
        }
    }

    /// Is this an module-level type index?
    pub fn is_module_type_index(self) -> bool {
        matches!(self, Self::Module(_))
    }

    /// Get the underlying module-level type index, if any.
    pub fn as_module_type_index(self) -> Option<ModuleInternedTypeIndex> {
        match self {
            Self::Module(e) => Some(e),
            Self::RecGroup(_) | Self::Engine(_) => None,
        }
    }

    /// Get the underlying module-level type index, or panic.
    #[track_caller]
    pub fn unwrap_module_type_index(self) -> ModuleInternedTypeIndex {
        match self.as_module_type_index() {
            Some(x) => x,
            None => panic!("`unwrap_module_type_index` on {self:?}"),
        }
    }

    /// Is this an recgroup-level type index?
    pub fn is_rec_group_type_index(self) -> bool {
        matches!(self, Self::RecGroup(_))
    }

    /// Get the underlying recgroup-level type index, if any.
    pub fn as_rec_group_type_index(self) -> Option<RecGroupRelativeTypeIndex> {
        match self {
            Self::RecGroup(r) => Some(r),
            Self::Module(_) | Self::Engine(_) => None,
        }
    }

    /// Get the underlying module-level type index, or panic.
    #[track_caller]
    pub fn unwrap_rec_group_type_index(self) -> RecGroupRelativeTypeIndex {
        match self.as_rec_group_type_index() {
            Some(x) => x,
            None => panic!("`unwrap_rec_group_type_index` on {self:?}"),
        }
    }
}

/// WebAssembly heap type -- equivalent of `wasmparser`'s HeapType
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum WasmHeapType {
    // External types.
    Extern,
    NoExtern,

    // Function types.
    Func,
    ConcreteFunc(EngineOrModuleTypeIndex),
    NoFunc,

    // Exception types.
    Exn,
    ConcreteExn(EngineOrModuleTypeIndex),
    NoExn,

    // Continuation types.
    Cont,
    ConcreteCont(EngineOrModuleTypeIndex),
    NoCont,

    // Internal types.
    Any,
    Eq,
    I31,
    Array,
    ConcreteArray(EngineOrModuleTypeIndex),
    Struct,
    ConcreteStruct(EngineOrModuleTypeIndex),
    None,
}

impl From<WasmHeapTopType> for WasmHeapType {
    #[inline]
    fn from(value: WasmHeapTopType) -> Self {
        match value {
            WasmHeapTopType::Extern => Self::Extern,
            WasmHeapTopType::Any => Self::Any,
            WasmHeapTopType::Func => Self::Func,
            WasmHeapTopType::Cont => Self::Cont,
            WasmHeapTopType::Exn => Self::Exn,
        }
    }
}

impl From<WasmHeapBottomType> for WasmHeapType {
    #[inline]
    fn from(value: WasmHeapBottomType) -> Self {
        match value {
            WasmHeapBottomType::NoExtern => Self::NoExtern,
            WasmHeapBottomType::None => Self::None,
            WasmHeapBottomType::NoFunc => Self::NoFunc,
            WasmHeapBottomType::NoCont => Self::NoCont,
            WasmHeapBottomType::NoExn => Self::NoExn,
        }
    }
}

impl fmt::Display for WasmHeapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Extern => write!(f, "extern"),
            Self::NoExtern => write!(f, "noextern"),
            Self::Func => write!(f, "func"),
            Self::ConcreteFunc(i) => write!(f, "func {i}"),
            Self::NoFunc => write!(f, "nofunc"),
            Self::Cont => write!(f, "cont"),
            Self::ConcreteCont(i) => write!(f, "cont {i}"),
            Self::NoCont => write!(f, "nocont"),
            Self::Any => write!(f, "any"),
            Self::Eq => write!(f, "eq"),
            Self::I31 => write!(f, "i31"),
            Self::Array => write!(f, "array"),
            Self::ConcreteArray(i) => write!(f, "array {i}"),
            Self::Struct => write!(f, "struct"),
            Self::ConcreteStruct(i) => write!(f, "struct {i}"),
            Self::Exn => write!(f, "exn"),
            Self::ConcreteExn(i) => write!(f, "exn {i}"),
            Self::NoExn => write!(f, "noexn"),
            Self::None => write!(f, "none"),
        }
    }
}

impl TypeTrace for WasmHeapType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match *self {
            Self::ConcreteArray(i) => func(i),
            Self::ConcreteFunc(i) => func(i),
            Self::ConcreteStruct(i) => func(i),
            Self::ConcreteCont(i) => func(i),
            _ => Ok(()),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::ConcreteArray(i) => func(i),
            Self::ConcreteFunc(i) => func(i),
            Self::ConcreteStruct(i) => func(i),
            Self::ConcreteCont(i) => func(i),
            _ => Ok(()),
        }
    }
}

impl WasmHeapType {
    /// Is this a type that is represented as a `VMGcRef`?
    #[inline]
    pub fn is_vmgcref_type(&self) -> bool {
        match self.top() {
            // All `t <: (ref null any)`, `t <: (ref null extern)`,
            // and `t <: (ref null exn)` are represented as
            // `VMGcRef`s.
            WasmHeapTopType::Any | WasmHeapTopType::Extern | WasmHeapTopType::Exn => true,

            // All `t <: (ref null func)` are not.
            WasmHeapTopType::Func => false,
            WasmHeapTopType::Cont => false,
        }
    }

    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a type that actually refers to an object allocated in a
    /// GC heap?
    #[inline]
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        self.is_vmgcref_type() && *self != Self::I31
    }

    /// Is this heap type the top of its type hierarchy?
    #[inline]
    pub fn is_top(&self) -> bool {
        *self == Self::from(self.top())
    }

    /// Get this type's top type.
    #[inline]
    pub fn top(&self) -> WasmHeapTopType {
        match self {
            WasmHeapType::Extern | WasmHeapType::NoExtern => WasmHeapTopType::Extern,

            WasmHeapType::Func | WasmHeapType::ConcreteFunc(_) | WasmHeapType::NoFunc => {
                WasmHeapTopType::Func
            }

            WasmHeapType::Cont | WasmHeapType::ConcreteCont(_) | WasmHeapType::NoCont => {
                WasmHeapTopType::Cont
            }

            WasmHeapType::Exn | WasmHeapType::ConcreteExn(_) | WasmHeapType::NoExn => {
                WasmHeapTopType::Exn
            }

            WasmHeapType::Any
            | WasmHeapType::Eq
            | WasmHeapType::I31
            | WasmHeapType::Array
            | WasmHeapType::ConcreteArray(_)
            | WasmHeapType::Struct
            | WasmHeapType::ConcreteStruct(_)
            | WasmHeapType::None => WasmHeapTopType::Any,
        }
    }

    /// Is this heap type the bottom of its type hierarchy?
    #[inline]
    pub fn is_bottom(&self) -> bool {
        *self == Self::from(self.bottom())
    }

    /// Get this type's bottom type.
    #[inline]
    pub fn bottom(&self) -> WasmHeapBottomType {
        match self {
            WasmHeapType::Extern | WasmHeapType::NoExtern => WasmHeapBottomType::NoExtern,

            WasmHeapType::Func | WasmHeapType::ConcreteFunc(_) | WasmHeapType::NoFunc => {
                WasmHeapBottomType::NoFunc
            }

            WasmHeapType::Cont | WasmHeapType::ConcreteCont(_) | WasmHeapType::NoCont => {
                WasmHeapBottomType::NoCont
            }

            WasmHeapType::Exn | WasmHeapType::ConcreteExn(_) | WasmHeapType::NoExn => {
                WasmHeapBottomType::NoExn
            }

            WasmHeapType::Any
            | WasmHeapType::Eq
            | WasmHeapType::I31
            | WasmHeapType::Array
            | WasmHeapType::ConcreteArray(_)
            | WasmHeapType::Struct
            | WasmHeapType::ConcreteStruct(_)
            | WasmHeapType::None => WasmHeapBottomType::None,
        }
    }
}

/// A top heap type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WasmHeapTopType {
    /// The common supertype of all external references.
    Extern,
    /// The common supertype of all internal references.
    Any,
    /// The common supertype of all function references.
    Func,
    /// The common supertype of all exception references.
    Exn,
    /// The common supertype of all continuation references.
    Cont,
}

/// A bottom heap type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WasmHeapBottomType {
    /// The common subtype of all external references.
    NoExtern,
    /// The common subtype of all internal references.
    None,
    /// The common subtype of all function references.
    NoFunc,
    /// The common subtype of all exception references.
    NoExn,
    /// The common subtype of all continuation references.
    NoCont,
}

/// WebAssembly function type -- equivalent of `wasmparser`'s FuncType.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmFuncType {
    params: Box<[WasmValType]>,
    non_i31_gc_ref_params_count: usize,
    returns: Box<[WasmValType]>,
    non_i31_gc_ref_returns_count: usize,
}

impl fmt::Display for WasmFuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(func")?;
        if !self.params.is_empty() {
            write!(f, " (param")?;
            for p in self.params.iter() {
                write!(f, " {p}")?;
            }
            write!(f, ")")?;
        }
        if !self.returns.is_empty() {
            write!(f, " (result")?;
            for r in self.returns.iter() {
                write!(f, " {r}")?;
            }
            write!(f, ")")?;
        }
        write!(f, ")")
    }
}

impl TypeTrace for WasmFuncType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for p in self.params.iter() {
            p.trace(func)?;
        }
        for r in self.returns.iter() {
            r.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for p in self.params.iter_mut() {
            p.trace_mut(func)?;
        }
        for r in self.returns.iter_mut() {
            r.trace_mut(func)?;
        }
        Ok(())
    }
}

impl WasmFuncType {
    /// Creates a new function type from the provided `params` and `returns`.
    #[inline]
    pub fn new(params: Box<[WasmValType]>, returns: Box<[WasmValType]>) -> Self {
        let non_i31_gc_ref_params_count = params
            .iter()
            .filter(|p| p.is_vmgcref_type_and_not_i31())
            .count();
        let non_i31_gc_ref_returns_count = returns
            .iter()
            .filter(|r| r.is_vmgcref_type_and_not_i31())
            .count();
        WasmFuncType {
            params,
            non_i31_gc_ref_params_count,
            returns,
            non_i31_gc_ref_returns_count,
        }
    }

    /// Function params types.
    #[inline]
    pub fn params(&self) -> &[WasmValType] {
        &self.params
    }

    /// How many `externref`s are in this function's params?
    #[inline]
    pub fn non_i31_gc_ref_params_count(&self) -> usize {
        self.non_i31_gc_ref_params_count
    }

    /// Returns params types.
    #[inline]
    pub fn returns(&self) -> &[WasmValType] {
        &self.returns
    }

    /// How many `externref`s are in this function's returns?
    #[inline]
    pub fn non_i31_gc_ref_returns_count(&self) -> usize {
        self.non_i31_gc_ref_returns_count
    }

    /// Is this function type compatible with trampoline usage in Wasmtime?
    pub fn is_trampoline_type(&self) -> bool {
        self.params().iter().all(|p| *p == p.trampoline_type())
            && self.returns().iter().all(|r| *r == r.trampoline_type())
    }

    /// Get the version of this function type that is suitable for usage as a
    /// trampoline in Wasmtime.
    ///
    /// If this function is suitable for trampoline usage as-is, then a borrowed
    /// `Cow` is returned. If it must be tweaked for trampoline usage, then an
    /// owned `Cow` is returned.
    ///
    /// ## What is a trampoline type?
    ///
    /// All reference types in parameters and results are mapped to their
    /// nullable top type, e.g. `(ref $my_struct_type)` becomes `(ref null
    /// any)`.
    ///
    /// This allows us to share trampolines between functions whose signatures
    /// both map to the same trampoline type. It also allows the host to satisfy
    /// a Wasm module's function import of type `S` with a function of type `T`
    /// where `T <: S`, even when the Wasm module never defines the type `T`
    /// (and might never even be able to!)
    ///
    /// The flip side is that this adds a constraint to our trampolines: they
    /// can only pass references around (e.g. move a reference from one calling
    /// convention's location to another's) and may not actually inspect the
    /// references themselves (unless the trampolines start doing explicit,
    /// fallible downcasts, but if we ever need that, then we might want to
    /// redesign this stuff).
    pub fn trampoline_type(&self) -> Cow<'_, Self> {
        if self.is_trampoline_type() {
            return Cow::Borrowed(self);
        }

        Cow::Owned(Self::new(
            self.params().iter().map(|p| p.trampoline_type()).collect(),
            self.returns().iter().map(|r| r.trampoline_type()).collect(),
        ))
    }
}

/// WebAssembly continuation type -- equivalent of `wasmparser`'s ContType.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmContType(EngineOrModuleTypeIndex);

impl fmt::Display for WasmContType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(cont {})", self.0)
    }
}

impl WasmContType {
    /// Constructs a new continuation type.
    pub fn new(idx: EngineOrModuleTypeIndex) -> Self {
        WasmContType(idx)
    }

    /// Returns the (module interned) index to the underlying function type.
    pub fn unwrap_module_type_index(self) -> ModuleInternedTypeIndex {
        match self.0 {
            EngineOrModuleTypeIndex::Engine(_) => panic!("not module interned"),
            EngineOrModuleTypeIndex::Module(idx) => idx,
            EngineOrModuleTypeIndex::RecGroup(_) => todo!(),
        }
    }
}

impl TypeTrace for WasmContType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(self.0)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(&mut self.0)
    }
}

/// WebAssembly exception type.
///
/// This "exception type" is not a Wasm language-level
/// concept. Instead, it denotes an *exception object signature* --
/// the types of the payload values.
///
/// In contrast, at the Wasm language level, exception objects are
/// associated with specific tags, and these tags refer to their
/// signatures (function types). However, tags are *nominal*: like
/// memories and tables, a separate instance of a tag exists for every
/// instance of the defining module, and these tag instances can be
/// imported and exported. At runtime we handle tags like we do
/// memories and tables, but these runtime instances do not exist in
/// the type system here.
///
/// Because the Wasm type system does not have concrete `exn` types
/// (i.e., the heap-type lattice has only top `exn` and bottom
/// `noexn`), we are free to decide what we mean by "concrete type"
/// here. Thus, we define an "exception type" to refer to the
/// type-level *signature*. When a particular *exception object* is
/// created in a store, it can be associated with a particular *tag
/// instance* also in that store, and the compatibility is checked
/// (the tag's function type must match the function type in the
/// associated WasmExnType).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmExnType {
    /// The function type from which we get our signature. We hold
    /// this directly so that we can efficiently derive a FuncType
    /// without re-interning the field types.
    pub func_ty: EngineOrModuleTypeIndex,
    /// The fields (payload values) that make up this exception type.
    ///
    /// While we could obtain these by looking up the `func_ty` above,
    /// we also need to be able to derive a GC object layout from this
    /// type descriptor without referencing other type descriptors; so
    /// we directly inline the information here.
    pub fields: Box<[WasmFieldType]>,
}

impl fmt::Display for WasmExnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(exn ({})", self.func_ty)?;
        for ty in self.fields.iter() {
            write!(f, " {ty}")?;
        }
        write!(f, ")")
    }
}

impl TypeTrace for WasmExnType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(self.func_ty)?;
        for f in self.fields.iter() {
            f.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(&mut self.func_ty)?;
        for f in self.fields.iter_mut() {
            f.trace_mut(func)?;
        }
        Ok(())
    }
}

/// Represents storage types introduced in the GC spec for array and struct fields.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum WasmStorageType {
    /// The storage type is i8.
    I8,
    /// The storage type is i16.
    I16,
    /// The storage type is a value type.
    Val(WasmValType),
}

impl fmt::Display for WasmStorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmStorageType::I8 => write!(f, "i8"),
            WasmStorageType::I16 => write!(f, "i16"),
            WasmStorageType::Val(v) => fmt::Display::fmt(v, f),
        }
    }
}

impl TypeTrace for WasmStorageType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmStorageType::I8 | WasmStorageType::I16 => Ok(()),
            WasmStorageType::Val(v) => v.trace(func),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            WasmStorageType::I8 | WasmStorageType::I16 => Ok(()),
            WasmStorageType::Val(v) => v.trace_mut(func),
        }
    }
}

impl WasmStorageType {
    /// Is this a type that is represented as a `VMGcRef` and is additionally
    /// not an `i31`?
    ///
    /// That is, is this a type that actually refers to an object allocated in a
    /// GC heap?
    pub fn is_vmgcref_type_and_not_i31(&self) -> bool {
        match self {
            WasmStorageType::I8 | WasmStorageType::I16 => false,
            WasmStorageType::Val(v) => v.is_vmgcref_type_and_not_i31(),
        }
    }
}

/// The type of a struct field or array element.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmFieldType {
    /// The field's element type.
    pub element_type: WasmStorageType,

    /// Whether this field can be mutated or not.
    pub mutable: bool,
}

impl fmt::Display for WasmFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mutable {
            write!(f, "(mut {})", self.element_type)
        } else {
            fmt::Display::fmt(&self.element_type, f)
        }
    }
}

impl TypeTrace for WasmFieldType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.element_type.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.element_type.trace_mut(func)
    }
}

/// A concrete array type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmArrayType(pub WasmFieldType);

impl fmt::Display for WasmArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(array {})", self.0)
    }
}

impl TypeTrace for WasmArrayType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.0.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        self.0.trace_mut(func)
    }
}

/// A concrete struct type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmStructType {
    /// The fields that make up this struct type.
    pub fields: Box<[WasmFieldType]>,
}

impl fmt::Display for WasmStructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(struct")?;
        for ty in self.fields.iter() {
            write!(f, " {ty}")?;
        }
        write!(f, ")")
    }
}

impl TypeTrace for WasmStructType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for f in self.fields.iter() {
            f.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for f in self.fields.iter_mut() {
            f.trace_mut(func)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self-describing type")]
pub struct WasmCompositeType {
    /// The type defined inside the composite type.
    pub inner: WasmCompositeInnerType,
    /// Is the composite type shared? This is part of the
    /// shared-everything-threads proposal.
    pub shared: bool,
}

impl fmt::Display for WasmCompositeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shared {
            write!(f, "(shared ")?;
        }
        fmt::Display::fmt(&self.inner, f)?;
        if self.shared {
            write!(f, ")")?;
        }
        Ok(())
    }
}

/// A function, array, or struct type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum WasmCompositeInnerType {
    Array(WasmArrayType),
    Func(WasmFuncType),
    Struct(WasmStructType),
    Cont(WasmContType),
    Exn(WasmExnType),
}

impl fmt::Display for WasmCompositeInnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array(ty) => fmt::Display::fmt(ty, f),
            Self::Func(ty) => fmt::Display::fmt(ty, f),
            Self::Struct(ty) => fmt::Display::fmt(ty, f),
            Self::Cont(ty) => fmt::Display::fmt(ty, f),
            Self::Exn(ty) => fmt::Display::fmt(ty, f),
        }
    }
}

#[expect(missing_docs, reason = "self-describing functions")]
impl WasmCompositeInnerType {
    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    #[inline]
    pub fn as_array(&self) -> Option<&WasmArrayType> {
        match self {
            Self::Array(f) => Some(f),
            _ => None,
        }
    }

    #[inline]
    pub fn unwrap_array(&self) -> &WasmArrayType {
        self.as_array().unwrap()
    }

    #[inline]
    pub fn is_func(&self) -> bool {
        matches!(self, Self::Func(_))
    }

    #[inline]
    pub fn as_func(&self) -> Option<&WasmFuncType> {
        match self {
            Self::Func(f) => Some(f),
            _ => None,
        }
    }

    #[inline]
    pub fn unwrap_func(&self) -> &WasmFuncType {
        self.as_func().unwrap()
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    #[inline]
    pub fn as_struct(&self) -> Option<&WasmStructType> {
        match self {
            Self::Struct(f) => Some(f),
            _ => None,
        }
    }

    #[inline]
    pub fn unwrap_struct(&self) -> &WasmStructType {
        self.as_struct().unwrap()
    }

    #[inline]
    pub fn is_cont(&self) -> bool {
        matches!(self, Self::Cont(_))
    }

    #[inline]
    pub fn as_cont(&self) -> Option<&WasmContType> {
        match self {
            Self::Cont(f) => Some(f),
            _ => None,
        }
    }

    #[inline]
    pub fn unwrap_cont(&self) -> &WasmContType {
        self.as_cont().unwrap()
    }

    #[inline]
    pub fn is_exn(&self) -> bool {
        matches!(self, Self::Exn(_))
    }

    #[inline]
    pub fn as_exn(&self) -> Option<&WasmExnType> {
        match self {
            Self::Exn(f) => Some(f),
            _ => None,
        }
    }

    #[inline]
    pub fn unwrap_exn(&self) -> &WasmExnType {
        self.as_exn().unwrap()
    }
}

impl TypeTrace for WasmCompositeType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match &self.inner {
            WasmCompositeInnerType::Array(a) => a.trace(func),
            WasmCompositeInnerType::Func(f) => f.trace(func),
            WasmCompositeInnerType::Struct(a) => a.trace(func),
            WasmCompositeInnerType::Cont(c) => c.trace(func),
            WasmCompositeInnerType::Exn(e) => e.trace(func),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match &mut self.inner {
            WasmCompositeInnerType::Array(a) => a.trace_mut(func),
            WasmCompositeInnerType::Func(f) => f.trace_mut(func),
            WasmCompositeInnerType::Struct(a) => a.trace_mut(func),
            WasmCompositeInnerType::Cont(c) => c.trace_mut(func),
            WasmCompositeInnerType::Exn(e) => e.trace_mut(func),
        }
    }
}

/// A concrete, user-defined (or host-defined) Wasm type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmSubType {
    /// Whether this type is forbidden from being the supertype of any other
    /// type.
    pub is_final: bool,

    /// This type's supertype, if any.
    pub supertype: Option<EngineOrModuleTypeIndex>,

    /// The array, function, or struct that is defined.
    pub composite_type: WasmCompositeType,
}

impl fmt::Display for WasmSubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_final && self.supertype.is_none() {
            fmt::Display::fmt(&self.composite_type, f)
        } else {
            write!(f, "(sub")?;
            if self.is_final {
                write!(f, " final")?;
            }
            if let Some(sup) = self.supertype {
                write!(f, " {sup}")?;
            }
            write!(f, " {})", self.composite_type)
        }
    }
}

/// Implicitly define all of these helper functions to handle only unshared
/// types; essentially, these act like `is_unshared_*` functions until shared
/// support is implemented.
#[expect(missing_docs, reason = "self-describing functions")]
impl WasmSubType {
    #[inline]
    pub fn is_func(&self) -> bool {
        self.composite_type.inner.is_func() && !self.composite_type.shared
    }

    #[inline]
    pub fn as_func(&self) -> Option<&WasmFuncType> {
        if self.composite_type.shared {
            None
        } else {
            self.composite_type.inner.as_func()
        }
    }

    #[inline]
    pub fn unwrap_func(&self) -> &WasmFuncType {
        assert!(!self.composite_type.shared);
        self.composite_type.inner.unwrap_func()
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        self.composite_type.inner.is_array() && !self.composite_type.shared
    }

    #[inline]
    pub fn as_array(&self) -> Option<&WasmArrayType> {
        if self.composite_type.shared {
            None
        } else {
            self.composite_type.inner.as_array()
        }
    }

    #[inline]
    pub fn unwrap_array(&self) -> &WasmArrayType {
        assert!(!self.composite_type.shared);
        self.composite_type.inner.unwrap_array()
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        self.composite_type.inner.is_struct() && !self.composite_type.shared
    }

    #[inline]
    pub fn as_struct(&self) -> Option<&WasmStructType> {
        if self.composite_type.shared {
            None
        } else {
            self.composite_type.inner.as_struct()
        }
    }

    #[inline]
    pub fn unwrap_struct(&self) -> &WasmStructType {
        assert!(!self.composite_type.shared);
        self.composite_type.inner.unwrap_struct()
    }

    #[inline]
    pub fn is_cont(&self) -> bool {
        self.composite_type.inner.is_cont() && !self.composite_type.shared
    }

    #[inline]
    pub fn as_cont(&self) -> Option<&WasmContType> {
        if self.composite_type.shared {
            None
        } else {
            self.composite_type.inner.as_cont()
        }
    }

    #[inline]
    pub fn unwrap_cont(&self) -> &WasmContType {
        assert!(!self.composite_type.shared);
        self.composite_type.inner.unwrap_cont()
    }

    #[inline]
    pub fn is_exn(&self) -> bool {
        self.composite_type.inner.is_exn() && !self.composite_type.shared
    }

    #[inline]
    pub fn as_exn(&self) -> Option<&WasmExnType> {
        if self.composite_type.shared {
            None
        } else {
            self.composite_type.inner.as_exn()
        }
    }

    #[inline]
    pub fn unwrap_exn(&self) -> &WasmExnType {
        assert!(!self.composite_type.shared);
        self.composite_type.inner.unwrap_exn()
    }
}

impl TypeTrace for WasmSubType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        if let Some(sup) = self.supertype {
            func(sup)?;
        }
        self.composite_type.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        if let Some(sup) = self.supertype.as_mut() {
            func(sup)?;
        }
        self.composite_type.trace_mut(func)
    }
}

/// A recursive type group.
///
/// Types within a recgroup can have forward references to each other, which
/// allows for cyclic types, for example a function `$f` that returns a
/// reference to a function `$g` which returns a reference to a function `$f`:
///
/// ```ignore
/// (rec (type (func $f (result (ref null $g))))
///      (type (func $g (result (ref null $f)))))
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WasmRecGroup {
    /// The types inside of this recgroup.
    pub types: Box<[WasmSubType]>,
}

impl TypeTrace for WasmRecGroup {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.types.iter() {
            ty.trace(func)?;
        }
        Ok(())
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        for ty in self.types.iter_mut() {
            ty.trace_mut(func)?;
        }
        Ok(())
    }
}

/// Index type of a function (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct FuncIndex(u32);
entity_impl!(FuncIndex);

/// Index type of a defined function inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedFuncIndex(u32);
entity_impl!(DefinedFuncIndex);

/// Index type of a defined table inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedTableIndex(u32);
entity_impl!(DefinedTableIndex);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedMemoryIndex(u32);
entity_impl!(DefinedMemoryIndex);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct OwnedMemoryIndex(u32);
entity_impl!(OwnedMemoryIndex);

/// Index type of a defined global inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedGlobalIndex(u32);
entity_impl!(DefinedGlobalIndex);

/// Index type of a table (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TableIndex(u32);
entity_impl!(TableIndex);

/// Index type of a global variable (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct GlobalIndex(u32);
entity_impl!(GlobalIndex);

/// Index type of a linear memory (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct MemoryIndex(u32);
entity_impl!(MemoryIndex);

/// Index type of a canonicalized recursive type group inside a WebAssembly
/// module (as opposed to canonicalized within the whole engine).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ModuleInternedRecGroupIndex(u32);
entity_impl!(ModuleInternedRecGroupIndex);

/// Index type of a canonicalized recursive type group inside the whole engine
/// (as opposed to canonicalized within just a single Wasm module).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct EngineInternedRecGroupIndex(u32);
entity_impl!(EngineInternedRecGroupIndex);

/// Index type of a type (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TypeIndex(u32);
entity_impl!(TypeIndex);

/// A canonicalized type index referencing a type within a single recursion
/// group from another type within that same recursion group.
///
/// This is only suitable for use when hash consing and deduplicating rec
/// groups.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct RecGroupRelativeTypeIndex(u32);
entity_impl!(RecGroupRelativeTypeIndex);

/// A canonicalized type index for a type within a single WebAssembly module.
///
/// Note that this is deduplicated only at the level of a single WebAssembly
/// module, not at the level of a whole store or engine. This means that these
/// indices are only unique within the context of a single Wasm module, and
/// therefore are not suitable for runtime type checks (which, in general, may
/// involve entities defined in different modules).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ModuleInternedTypeIndex(u32);
entity_impl!(ModuleInternedTypeIndex);

/// A canonicalized type index into an engine's shared type registry.
///
/// This is canonicalized/deduped at the level of a whole engine, across all the
/// modules loaded into that engine, not just at the level of a single
/// particular module. This means that `VMSharedTypeIndex` is usable for
/// e.g. checking that function signatures match during an indirect call
/// (potentially to a function defined in a different module) at runtime.
#[repr(transparent)] // Used directly by JIT code.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct VMSharedTypeIndex(u32);
entity_impl!(VMSharedTypeIndex);

impl VMSharedTypeIndex {
    /// Create a new `VMSharedTypeIndex`.
    #[inline]
    pub fn new(value: u32) -> Self {
        assert_ne!(
            value,
            u32::MAX,
            "u32::MAX is reserved for the default value"
        );
        Self(value)
    }

    /// Returns the underlying bits of the index.
    #[inline]
    pub fn bits(&self) -> u32 {
        self.0
    }
}

impl Default for VMSharedTypeIndex {
    #[inline]
    fn default() -> Self {
        Self(u32::MAX)
    }
}

/// Index type of a passive data segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DataIndex(u32);
entity_impl!(DataIndex);

/// Index type of a passive element segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ElemIndex(u32);
entity_impl!(ElemIndex);

/// Index type of a defined tag inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DefinedTagIndex(u32);
entity_impl!(DefinedTagIndex);

/// Index type of an event inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct TagIndex(u32);
entity_impl!(TagIndex);

/// Index into the global list of modules found within an entire component.
///
/// Module translations are saved on the side to get fully compiled after
/// the original component has finished being translated.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct StaticModuleIndex(u32);
entity_impl!(StaticModuleIndex);

/// An index of an entity.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum EntityIndex {
    /// Function index.
    Function(FuncIndex),
    /// Table index.
    Table(TableIndex),
    /// Memory index.
    Memory(MemoryIndex),
    /// Global index.
    Global(GlobalIndex),
    /// Tag index.
    Tag(TagIndex),
}

impl From<FuncIndex> for EntityIndex {
    fn from(idx: FuncIndex) -> EntityIndex {
        EntityIndex::Function(idx)
    }
}

impl From<TableIndex> for EntityIndex {
    fn from(idx: TableIndex) -> EntityIndex {
        EntityIndex::Table(idx)
    }
}

impl From<MemoryIndex> for EntityIndex {
    fn from(idx: MemoryIndex) -> EntityIndex {
        EntityIndex::Memory(idx)
    }
}

impl From<GlobalIndex> for EntityIndex {
    fn from(idx: GlobalIndex) -> EntityIndex {
        EntityIndex::Global(idx)
    }
}

impl From<TagIndex> for EntityIndex {
    fn from(idx: TagIndex) -> EntityIndex {
        EntityIndex::Tag(idx)
    }
}

/// A type of an item in a wasm module where an item is typically something that
/// can be exported.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    /// A global variable with the specified content type
    Global(Global),
    /// A linear memory with the specified limits
    Memory(Memory),
    /// An exception and control tag definition.
    Tag(Tag),
    /// A table with the specified element type and limits
    Table(Table),
    /// A function type where the index points to the type section and records a
    /// function signature.
    Function(EngineOrModuleTypeIndex),
}

impl TypeTrace for EntityType {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::Global(g) => g.trace(func),
            Self::Table(t) => t.trace(func),
            Self::Function(idx) => func(*idx),
            Self::Memory(_) => Ok(()),
            Self::Tag(t) => t.trace(func),
        }
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        match self {
            Self::Global(g) => g.trace_mut(func),
            Self::Table(t) => t.trace_mut(func),
            Self::Function(idx) => func(idx),
            Self::Memory(_) => Ok(()),
            Self::Tag(t) => t.trace_mut(func),
        }
    }
}

impl EntityType {
    /// Assert that this entity is a global
    pub fn unwrap_global(&self) -> &Global {
        match self {
            EntityType::Global(g) => g,
            _ => panic!("not a global"),
        }
    }

    /// Assert that this entity is a memory
    pub fn unwrap_memory(&self) -> &Memory {
        match self {
            EntityType::Memory(g) => g,
            _ => panic!("not a memory"),
        }
    }

    /// Assert that this entity is a tag
    pub fn unwrap_tag(&self) -> &Tag {
        match self {
            EntityType::Tag(g) => g,
            _ => panic!("not a tag"),
        }
    }

    /// Assert that this entity is a table
    pub fn unwrap_table(&self) -> &Table {
        match self {
            EntityType::Table(g) => g,
            _ => panic!("not a table"),
        }
    }

    /// Assert that this entity is a function
    pub fn unwrap_func(&self) -> EngineOrModuleTypeIndex {
        match self {
            EntityType::Function(g) => *g,
            _ => panic!("not a func"),
        }
    }
}

/// A WebAssembly global.
///
/// Note that we record both the original Wasm type and the Cranelift IR type
/// used to represent it. This is because multiple different kinds of Wasm types
/// might be represented with the same Cranelift IR type. For example, both a
/// Wasm `i64` and a `funcref` might be represented with a Cranelift `i64` on
/// 64-bit architectures, and when GC is not required for func refs.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Global {
    /// The Wasm type of the value stored in the global.
    pub wasm_ty: crate::WasmValType,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
}

impl TypeTrace for Global {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Global {
            wasm_ty,
            mutability: _,
        } = self;
        wasm_ty.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Global {
            wasm_ty,
            mutability: _,
        } = self;
        wasm_ty.trace_mut(func)
    }
}

/// A constant expression.
///
/// These are used to initialize globals, table elements, etc...
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConstExpr {
    ops: SmallVec<[ConstOp; 2]>,
}

impl ConstExpr {
    /// Create a new const expression from the given opcodes.
    ///
    /// Does not do any validation that the const expression is well-typed.
    ///
    /// Panics if given zero opcodes.
    pub fn new(ops: impl IntoIterator<Item = ConstOp>) -> Self {
        let ops = ops.into_iter().collect::<SmallVec<[ConstOp; 2]>>();
        assert!(!ops.is_empty());
        ConstExpr { ops }
    }

    /// Create a new const expression from a `wasmparser` const expression.
    ///
    /// Returns the new const expression as well as the escaping function
    /// indices that appeared in `ref.func` instructions, if any.
    pub fn from_wasmparser(
        expr: wasmparser::ConstExpr<'_>,
    ) -> WasmResult<(Self, SmallVec<[FuncIndex; 1]>)> {
        let mut iter = expr
            .get_operators_reader()
            .into_iter_with_offsets()
            .peekable();

        let mut ops = SmallVec::<[ConstOp; 2]>::new();
        let mut escaped = SmallVec::<[FuncIndex; 1]>::new();
        while let Some(res) = iter.next() {
            let (op, offset) = res?;

            // If we reach an `end` instruction, and there are no more
            // instructions after that, then we are done reading this const
            // expression.
            if matches!(op, wasmparser::Operator::End) && iter.peek().is_none() {
                break;
            }

            // Track any functions that appear in `ref.func` so that callers can
            // make sure to flag them as escaping.
            if let wasmparser::Operator::RefFunc { function_index } = &op {
                escaped.push(FuncIndex::from_u32(*function_index));
            }

            ops.push(ConstOp::from_wasmparser(op, offset)?);
        }
        Ok((Self { ops }, escaped))
    }

    /// Get the opcodes that make up this const expression.
    pub fn ops(&self) -> &[ConstOp] {
        &self.ops
    }

    /// Is this ConstExpr a provably nonzero integer value?
    ///
    /// This must be conservative: if the expression *might* be zero,
    /// it must return `false`. It is always allowed to return `false`
    /// for some expression kind that we don't support. However, if it
    /// returns `true`, the expression must be actually nonzero.
    ///
    /// We use this for certain table optimizations that rely on
    /// knowing for sure that index 0 is not referenced.
    pub fn provably_nonzero_i32(&self) -> bool {
        assert!(self.ops.len() > 0);
        if self.ops.len() > 1 {
            // Compound expressions not yet supported: conservatively
            // return `false` (we can't prove nonzero).
            return false;
        }
        // Exactly one op at this point.
        match self.ops[0] {
            // An actual zero value -- definitely not nonzero!
            ConstOp::I32Const(0) => false,
            // Any other constant value -- provably nonzero, if above
            // did not match.
            ConstOp::I32Const(_) => true,
            // Anything else: we can't prove anything.
            _ => false,
        }
    }
}

/// The subset of Wasm opcodes that are constant.
#[expect(missing_docs, reason = "self-describing variants")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ConstOp {
    I32Const(i32),
    I64Const(i64),
    F32Const(u32),
    F64Const(u64),
    V128Const(u128),
    GlobalGet(GlobalIndex),
    RefI31,
    RefNull,
    RefFunc(FuncIndex),
    I32Add,
    I32Sub,
    I32Mul,
    I64Add,
    I64Sub,
    I64Mul,
    StructNew {
        struct_type_index: TypeIndex,
    },
    StructNewDefault {
        struct_type_index: TypeIndex,
    },
    ArrayNew {
        array_type_index: TypeIndex,
    },
    ArrayNewDefault {
        array_type_index: TypeIndex,
    },
    ArrayNewFixed {
        array_type_index: TypeIndex,
        array_size: u32,
    },
    ExternConvertAny,
    AnyConvertExtern,
}

impl ConstOp {
    /// Convert a `wasmparser::Operator` to a `ConstOp`.
    pub fn from_wasmparser(op: wasmparser::Operator<'_>, offset: usize) -> WasmResult<Self> {
        use wasmparser::Operator as O;
        Ok(match op {
            O::I32Const { value } => Self::I32Const(value),
            O::I64Const { value } => Self::I64Const(value),
            O::F32Const { value } => Self::F32Const(value.bits()),
            O::F64Const { value } => Self::F64Const(value.bits()),
            O::V128Const { value } => Self::V128Const(u128::from_le_bytes(*value.bytes())),
            O::RefNull { hty: _ } => Self::RefNull,
            O::RefFunc { function_index } => Self::RefFunc(FuncIndex::from_u32(function_index)),
            O::GlobalGet { global_index } => Self::GlobalGet(GlobalIndex::from_u32(global_index)),
            O::RefI31 => Self::RefI31,
            O::I32Add => Self::I32Add,
            O::I32Sub => Self::I32Sub,
            O::I32Mul => Self::I32Mul,
            O::I64Add => Self::I64Add,
            O::I64Sub => Self::I64Sub,
            O::I64Mul => Self::I64Mul,
            O::StructNew { struct_type_index } => Self::StructNew {
                struct_type_index: TypeIndex::from_u32(struct_type_index),
            },
            O::StructNewDefault { struct_type_index } => Self::StructNewDefault {
                struct_type_index: TypeIndex::from_u32(struct_type_index),
            },
            O::ArrayNew { array_type_index } => Self::ArrayNew {
                array_type_index: TypeIndex::from_u32(array_type_index),
            },
            O::ArrayNewDefault { array_type_index } => Self::ArrayNewDefault {
                array_type_index: TypeIndex::from_u32(array_type_index),
            },
            O::ArrayNewFixed {
                array_type_index,
                array_size,
            } => Self::ArrayNewFixed {
                array_type_index: TypeIndex::from_u32(array_type_index),
                array_size,
            },
            O::ExternConvertAny => Self::ExternConvertAny,
            O::AnyConvertExtern => Self::AnyConvertExtern,
            op => {
                return Err(wasm_unsupported!(
                    "unsupported opcode in const expression at offset {offset:#x}: {op:?}",
                ));
            }
        })
    }
}

/// The type that can be used to index into [Memory] and [Table].
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum IndexType {
    I32,
    I64,
}

/// The size range of resizeable storage associated with [Memory] types and [Table] types.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[expect(missing_docs, reason = "self-describing fields")]
pub struct Limits {
    pub min: u64,
    pub max: Option<u64>,
}

/// WebAssembly table.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Table {
    /// The type of the index used to access the table.
    pub idx_type: IndexType,
    /// Tables are constrained by limits for their minimum and optionally maximum size.
    /// The limits are given in numbers of entries.
    pub limits: Limits,
    /// The table elements' Wasm type.
    pub ref_type: WasmRefType,
}

impl TypeTrace for Table {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Table {
            ref_type: wasm_ty,
            idx_type: _,
            limits: _,
        } = self;
        wasm_ty.trace(func)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        let Table {
            ref_type: wasm_ty,
            idx_type: _,
            limits: _,
        } = self;
        wasm_ty.trace_mut(func)
    }
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Memory {
    /// The type of the index used to access the memory.
    pub idx_type: IndexType,
    /// The limits constrain the minimum and optionally the maximum size of a memory.
    /// The limits are given in units of page size.
    pub limits: Limits,
    /// Whether the memory may be shared between multiple threads.
    pub shared: bool,
    /// The log2 of this memory's page size, in bytes.
    ///
    /// By default the page size is 64KiB (0x10000; 2**16; 1<<16; 65536) but the
    /// custom-page-sizes proposal allows opting into a page size of `1`.
    pub page_size_log2: u8,
}

/// Maximum size, in bytes, of 32-bit memories (4G)
pub const WASM32_MAX_SIZE: u64 = 1 << 32;

impl Memory {
    /// WebAssembly page sizes are 64KiB by default.
    pub const DEFAULT_PAGE_SIZE: u32 = 0x10000;

    /// WebAssembly page sizes are 64KiB (or `2**16`) by default.
    pub const DEFAULT_PAGE_SIZE_LOG2: u8 = {
        let log2 = 16;
        assert!(1 << log2 == Memory::DEFAULT_PAGE_SIZE);
        log2
    };

    /// Returns the minimum size, in bytes, that this memory must be.
    ///
    /// # Errors
    ///
    /// Returns an error if the calculation of the minimum size overflows the
    /// `u64` return type. This means that the memory can't be allocated but
    /// it's deferred to the caller to how to deal with that.
    pub fn minimum_byte_size(&self) -> Result<u64, SizeOverflow> {
        self.limits
            .min
            .checked_mul(self.page_size())
            .ok_or(SizeOverflow)
    }

    /// Returns the maximum size, in bytes, that this memory is allowed to be.
    ///
    /// Note that the return value here is not an `Option` despite the maximum
    /// size of a linear memory being optional in wasm. If a maximum size
    /// is not present in the memory's type then a maximum size is selected for
    /// it. For example the maximum size of a 32-bit memory is `1<<32`. The
    /// maximum size of a 64-bit linear memory is chosen to be a value that
    /// won't ever be allowed at runtime.
    ///
    /// # Errors
    ///
    /// Returns an error if the calculation of the maximum size overflows the
    /// `u64` return type. This means that the memory can't be allocated but
    /// it's deferred to the caller to how to deal with that.
    pub fn maximum_byte_size(&self) -> Result<u64, SizeOverflow> {
        match self.limits.max {
            Some(max) => max.checked_mul(self.page_size()).ok_or(SizeOverflow),
            None => {
                let min = self.minimum_byte_size()?;
                Ok(min.max(self.max_size_based_on_index_type()))
            }
        }
    }

    /// Get the size of this memory's pages, in bytes.
    pub fn page_size(&self) -> u64 {
        debug_assert!(
            self.page_size_log2 == 16 || self.page_size_log2 == 0,
            "invalid page_size_log2: {}; must be 16 or 0",
            self.page_size_log2
        );
        1 << self.page_size_log2
    }

    /// Returns the maximum size memory is allowed to be only based on the
    /// index type used by this memory.
    ///
    /// For example 32-bit linear memories return `1<<32` from this method.
    pub fn max_size_based_on_index_type(&self) -> u64 {
        match self.idx_type {
            IndexType::I64 =>
            // Note that the true maximum size of a 64-bit linear memory, in
            // bytes, cannot be represented in a `u64`. That would require a u65
            // to store `1<<64`. Despite that no system can actually allocate a
            // full 64-bit linear memory so this is instead emulated as "what if
            // the kernel fit in a single Wasm page of linear memory". Shouldn't
            // ever actually be possible but it provides a number to serve as an
            // effective maximum.
            {
                0_u64.wrapping_sub(self.page_size())
            }
            IndexType::I32 => WASM32_MAX_SIZE,
        }
    }

    /// Returns whether this memory can be implemented with virtual memory on
    /// a host with `host_page_size_log2`.
    ///
    /// When this function returns `true` then it means that signals such as
    /// SIGSEGV on the host are compatible with wasm and can be used to
    /// represent out-of-bounds memory accesses.
    ///
    /// When this function returns `false` then it means that this memory must,
    /// for example, have explicit bounds checks. This additionally means that
    /// virtual memory traps (e.g. SIGSEGV) cannot be relied on to implement
    /// linear memory semantics.
    pub fn can_use_virtual_memory(&self, tunables: &Tunables, host_page_size_log2: u8) -> bool {
        tunables.signals_based_traps && self.page_size_log2 >= host_page_size_log2
    }

    /// Returns whether this memory is a candidate for bounds check elision
    /// given the configuration and host page size.
    ///
    /// This function determines whether the given compilation configuration and
    /// hos enables possible bounds check elision for this memory. Bounds checks
    /// can only be elided if [`Memory::can_use_virtual_memory`] returns `true`
    /// for example but there are additionally requirements on the index size of
    /// this memory and the memory reservation in `tunables`.
    ///
    /// Currently the only case that supports bounds check elision is when all
    /// of these apply:
    ///
    /// * When [`Memory::can_use_virtual_memory`] returns `true`.
    /// * This is a 32-bit linear memory (e.g. not 64-bit)
    /// * `tunables.memory_reservation` is in excess of 4GiB
    ///
    /// In this situation all computable addresses fall within the reserved
    /// space (modulo static offsets factoring in guard pages) so bounds checks
    /// may be elidable.
    pub fn can_elide_bounds_check(&self, tunables: &Tunables, host_page_size_log2: u8) -> bool {
        self.can_use_virtual_memory(tunables, host_page_size_log2)
            && self.idx_type == IndexType::I32
            && tunables.memory_reservation >= (1 << 32)
    }

    /// Returns the static size of this heap in bytes at runtime, if available.
    ///
    /// This is only computable when the minimum size equals the maximum size.
    pub fn static_heap_size(&self) -> Option<u64> {
        let min = self.minimum_byte_size().ok()?;
        let max = self.maximum_byte_size().ok()?;
        if min == max { Some(min) } else { None }
    }

    /// Returns whether or not the base pointer of this memory is allowed to be
    /// relocated at runtime.
    ///
    /// When this function returns `false` then it means that after the initial
    /// allocation the base pointer is constant for the entire lifetime of a
    /// memory. This can enable compiler optimizations, for example.
    pub fn memory_may_move(&self, tunables: &Tunables) -> bool {
        // Shared memories cannot ever relocate their base pointer so the
        // settings configured in the engine must be appropriate for them ahead
        // of time.
        if self.shared {
            return false;
        }

        // If movement is disallowed in engine configuration, then the answer is
        // "no".
        if !tunables.memory_may_move {
            return false;
        }

        // If its minimum and maximum are the same, then the memory will never
        // be resized, and therefore will never move.
        if self.limits.max.is_some_and(|max| self.limits.min == max) {
            return false;
        }

        // If the maximum size of this memory is above the threshold of the
        // initial memory reservation then the memory may move.
        let max = self.maximum_byte_size().unwrap_or(u64::MAX);
        max > tunables.memory_reservation
    }
}

#[derive(Copy, Clone, Debug)]
#[expect(missing_docs, reason = "self-describing error struct")]
pub struct SizeOverflow;

impl fmt::Display for SizeOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("size overflow calculating memory size")
    }
}

impl core::error::Error for SizeOverflow {}

impl From<wasmparser::MemoryType> for Memory {
    fn from(ty: wasmparser::MemoryType) -> Memory {
        let idx_type = match ty.memory64 {
            false => IndexType::I32,
            true => IndexType::I64,
        };
        let limits = Limits {
            min: ty.initial,
            max: ty.maximum,
        };
        let page_size_log2 = u8::try_from(ty.page_size_log2.unwrap_or(16)).unwrap();
        debug_assert!(
            page_size_log2 == 16 || page_size_log2 == 0,
            "invalid page_size_log2: {page_size_log2}; must be 16 or 0"
        );
        Memory {
            idx_type,
            limits,
            shared: ty.shared,
            page_size_log2,
        }
    }
}

/// WebAssembly exception and control tag.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The tag signature type.
    pub signature: EngineOrModuleTypeIndex,
}

impl TypeTrace for Tag {
    fn trace<F, E>(&self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(self.signature)
    }

    fn trace_mut<F, E>(&mut self, func: &mut F) -> Result<(), E>
    where
        F: FnMut(&mut EngineOrModuleTypeIndex) -> Result<(), E>,
    {
        func(&mut self.signature)
    }
}

/// Helpers used to convert a `wasmparser` type to a type in this crate.
#[expect(missing_docs, reason = "self-describing functions")]
pub trait TypeConvert {
    /// Converts a wasmparser table type into a wasmtime type
    fn convert_global_type(&self, ty: &wasmparser::GlobalType) -> WasmResult<Global> {
        Ok(Global {
            wasm_ty: self.convert_valtype(ty.content_type)?,
            mutability: ty.mutable,
        })
    }

    /// Converts a wasmparser table type into a wasmtime type
    fn convert_table_type(&self, ty: &wasmparser::TableType) -> WasmResult<Table> {
        let idx_type = match ty.table64 {
            false => IndexType::I32,
            true => IndexType::I64,
        };
        let limits = Limits {
            min: ty.initial,
            max: ty.maximum,
        };
        Ok(Table {
            idx_type,
            limits,
            ref_type: self.convert_ref_type(ty.element_type)?,
        })
    }

    fn convert_sub_type(&self, ty: &wasmparser::SubType) -> WasmResult<WasmSubType> {
        Ok(WasmSubType {
            is_final: ty.is_final,
            supertype: ty.supertype_idx.map(|i| self.lookup_type_index(i.unpack())),
            composite_type: self.convert_composite_type(&ty.composite_type)?,
        })
    }

    fn convert_composite_type(
        &self,
        ty: &wasmparser::CompositeType,
    ) -> WasmResult<WasmCompositeType> {
        let inner = match &ty.inner {
            wasmparser::CompositeInnerType::Func(f) => {
                WasmCompositeInnerType::Func(self.convert_func_type(f)?)
            }
            wasmparser::CompositeInnerType::Array(a) => {
                WasmCompositeInnerType::Array(self.convert_array_type(a)?)
            }
            wasmparser::CompositeInnerType::Struct(s) => {
                WasmCompositeInnerType::Struct(self.convert_struct_type(s)?)
            }
            wasmparser::CompositeInnerType::Cont(c) => {
                WasmCompositeInnerType::Cont(self.convert_cont_type(c))
            }
        };
        Ok(WasmCompositeType {
            inner,
            shared: ty.shared,
        })
    }

    /// Converts a wasmparser continuation type to a wasmtime type
    fn convert_cont_type(&self, ty: &wasmparser::ContType) -> WasmContType {
        if let WasmHeapType::ConcreteFunc(sigidx) = self.lookup_heap_type(ty.0.unpack()) {
            WasmContType::new(sigidx)
        } else {
            panic!("Failed to extract signature index for continuation type.")
        }
    }

    fn convert_struct_type(&self, ty: &wasmparser::StructType) -> WasmResult<WasmStructType> {
        Ok(WasmStructType {
            fields: ty
                .fields
                .iter()
                .map(|f| self.convert_field_type(f))
                .collect::<WasmResult<_>>()?,
        })
    }

    fn convert_array_type(&self, ty: &wasmparser::ArrayType) -> WasmResult<WasmArrayType> {
        Ok(WasmArrayType(self.convert_field_type(&ty.0)?))
    }

    fn convert_field_type(&self, ty: &wasmparser::FieldType) -> WasmResult<WasmFieldType> {
        Ok(WasmFieldType {
            element_type: self.convert_storage_type(&ty.element_type)?,
            mutable: ty.mutable,
        })
    }

    fn convert_storage_type(&self, ty: &wasmparser::StorageType) -> WasmResult<WasmStorageType> {
        Ok(match ty {
            wasmparser::StorageType::I8 => WasmStorageType::I8,
            wasmparser::StorageType::I16 => WasmStorageType::I16,
            wasmparser::StorageType::Val(v) => WasmStorageType::Val(self.convert_valtype(*v)?),
        })
    }

    /// Converts a wasmparser function type to a wasmtime type
    fn convert_func_type(&self, ty: &wasmparser::FuncType) -> WasmResult<WasmFuncType> {
        let params = ty
            .params()
            .iter()
            .map(|t| self.convert_valtype(*t))
            .collect::<WasmResult<_>>()?;
        let results = ty
            .results()
            .iter()
            .map(|t| self.convert_valtype(*t))
            .collect::<WasmResult<_>>()?;
        Ok(WasmFuncType::new(params, results))
    }

    /// Converts a wasmparser value type to a wasmtime type
    fn convert_valtype(&self, ty: wasmparser::ValType) -> WasmResult<WasmValType> {
        Ok(match ty {
            wasmparser::ValType::I32 => WasmValType::I32,
            wasmparser::ValType::I64 => WasmValType::I64,
            wasmparser::ValType::F32 => WasmValType::F32,
            wasmparser::ValType::F64 => WasmValType::F64,
            wasmparser::ValType::V128 => WasmValType::V128,
            wasmparser::ValType::Ref(t) => WasmValType::Ref(self.convert_ref_type(t)?),
        })
    }

    /// Converts a wasmparser reference type to a wasmtime type
    fn convert_ref_type(&self, ty: wasmparser::RefType) -> WasmResult<WasmRefType> {
        Ok(WasmRefType {
            nullable: ty.is_nullable(),
            heap_type: self.convert_heap_type(ty.heap_type())?,
        })
    }

    /// Converts a wasmparser heap type to a wasmtime type
    fn convert_heap_type(&self, ty: wasmparser::HeapType) -> WasmResult<WasmHeapType> {
        Ok(match ty {
            wasmparser::HeapType::Concrete(i) => self.lookup_heap_type(i),
            wasmparser::HeapType::Abstract { ty, shared: false } => match ty {
                wasmparser::AbstractHeapType::Extern => WasmHeapType::Extern,
                wasmparser::AbstractHeapType::NoExtern => WasmHeapType::NoExtern,
                wasmparser::AbstractHeapType::Func => WasmHeapType::Func,
                wasmparser::AbstractHeapType::NoFunc => WasmHeapType::NoFunc,
                wasmparser::AbstractHeapType::Any => WasmHeapType::Any,
                wasmparser::AbstractHeapType::Eq => WasmHeapType::Eq,
                wasmparser::AbstractHeapType::I31 => WasmHeapType::I31,
                wasmparser::AbstractHeapType::Array => WasmHeapType::Array,
                wasmparser::AbstractHeapType::Struct => WasmHeapType::Struct,
                wasmparser::AbstractHeapType::None => WasmHeapType::None,
                wasmparser::AbstractHeapType::Cont => WasmHeapType::Cont,
                wasmparser::AbstractHeapType::NoCont => WasmHeapType::NoCont,
                wasmparser::AbstractHeapType::Exn => WasmHeapType::Exn,
                wasmparser::AbstractHeapType::NoExn => WasmHeapType::NoExn,
            },
            _ => return Err(wasm_unsupported!("unsupported heap type {ty:?}")),
        })
    }

    /// Converts the specified type index from a heap type into a canonicalized
    /// heap type.
    fn lookup_heap_type(&self, index: wasmparser::UnpackedIndex) -> WasmHeapType;

    /// Converts the specified type index from a heap type into a canonicalized
    /// heap type.
    fn lookup_type_index(&self, index: wasmparser::UnpackedIndex) -> EngineOrModuleTypeIndex;
}
