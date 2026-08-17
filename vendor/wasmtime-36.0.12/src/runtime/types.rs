use crate::prelude::*;
use crate::runtime::Memory as RuntimeMemory;
use crate::runtime::externals::Global as RuntimeGlobal;
use crate::runtime::externals::Table as RuntimeTable;
use crate::{AsContextMut, Extern, Func, Val};
use crate::{Engine, type_registry::RegisteredType};
use core::fmt::{self, Display, Write};
use wasmtime_environ::WasmExnType;
use wasmtime_environ::{
    EngineOrModuleTypeIndex, EntityType, Global, IndexType, Limits, Memory, ModuleTypes, Table,
    Tag, TypeTrace, VMSharedTypeIndex, WasmArrayType, WasmCompositeInnerType, WasmCompositeType,
    WasmFieldType, WasmFuncType, WasmHeapType, WasmRefType, WasmStorageType, WasmStructType,
    WasmSubType, WasmValType,
};

pub(crate) mod matching;

// Type Representations

// Type attributes

/// Indicator of whether a global value, struct's field, or array type's
/// elements are mutable or not.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Mutability {
    /// The global value, struct field, or array elements are constant and the
    /// value does not change.
    Const,
    /// The value of the global, struct field, or array elements can change over
    /// time.
    Var,
}

impl Mutability {
    /// Is this constant?
    #[inline]
    pub fn is_const(&self) -> bool {
        *self == Self::Const
    }

    /// Is this variable?
    #[inline]
    pub fn is_var(&self) -> bool {
        *self == Self::Var
    }
}

/// Indicator of whether a type is final or not.
///
/// Final types may not be the supertype of other types.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Finality {
    /// The associated type is final.
    Final,
    /// The associated type is not final.
    NonFinal,
}

impl Finality {
    /// Is this final?
    #[inline]
    pub fn is_final(&self) -> bool {
        *self == Self::Final
    }

    /// Is this non-final?
    #[inline]
    pub fn is_non_final(&self) -> bool {
        *self == Self::NonFinal
    }
}

// Value Types

/// A list of all possible value types in WebAssembly.
///
/// # Subtyping and Equality
///
/// `ValType` does not implement `Eq`, because reference types have a subtyping
/// relationship, and so 99.99% of the time you actually want to check whether
/// one type matches (i.e. is a subtype of) another type. You can use the
/// [`ValType::matches`] and [`Val::matches_ty`][crate::Val::matches_ty] methods
/// to perform these types of checks. If, however, you are in that 0.01%
/// scenario where you need to check precise equality between types, you can use
/// the [`ValType::eq`] method.
#[derive(Clone, Hash)]
pub enum ValType {
    // NB: the ordering of variants here is intended to match the ordering in
    // `wasmtime_environ::WasmType` to help improve codegen when converting.
    //
    /// Signed 32 bit integer.
    I32,
    /// Signed 64 bit integer.
    I64,
    /// Floating point 32 bit integer.
    F32,
    /// Floating point 64 bit integer.
    F64,
    /// A 128 bit number.
    V128,
    /// An opaque reference to some type on the heap.
    Ref(RefType),
}

impl fmt::Debug for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(r) => Display::fmt(r, f),
        }
    }
}

impl From<RefType> for ValType {
    #[inline]
    fn from(r: RefType) -> Self {
        ValType::Ref(r)
    }
}

impl ValType {
    /// The `externref` type, aka `(ref null extern)`.
    pub const EXTERNREF: Self = ValType::Ref(RefType::EXTERNREF);

    /// The `nullexternref` type, aka `(ref null noextern)`.
    pub const NULLEXTERNREF: Self = ValType::Ref(RefType::NULLEXTERNREF);

    /// The `funcref` type, aka `(ref null func)`.
    pub const FUNCREF: Self = ValType::Ref(RefType::FUNCREF);

    /// The `nullfuncref` type, aka `(ref null nofunc)`.
    pub const NULLFUNCREF: Self = ValType::Ref(RefType::NULLFUNCREF);

    /// The `anyref` type, aka `(ref null any)`.
    pub const ANYREF: Self = ValType::Ref(RefType::ANYREF);

    /// The `eqref` type, aka `(ref null eq)`.
    pub const EQREF: Self = ValType::Ref(RefType::EQREF);

    /// The `i31ref` type, aka `(ref null i31)`.
    pub const I31REF: Self = ValType::Ref(RefType::I31REF);

    /// The `arrayref` type, aka `(ref null array)`.
    pub const ARRAYREF: Self = ValType::Ref(RefType::ARRAYREF);

    /// The `structref` type, aka `(ref null struct)`.
    pub const STRUCTREF: Self = ValType::Ref(RefType::STRUCTREF);

    /// The `nullref` type, aka `(ref null none)`.
    pub const NULLREF: Self = ValType::Ref(RefType::NULLREF);

    /// The `contref` type, aka `(ref null cont)`.
    pub const CONTREF: Self = ValType::Ref(RefType::CONTREF);

    /// The `nullcontref` type, aka. `(ref null nocont)`.
    pub const NULLCONTREF: Self = ValType::Ref(RefType::NULLCONTREF);

    /// The `exnref` type, aka `(ref null exn)`.
    pub const EXNREF: Self = ValType::Ref(RefType::EXNREF);

    /// The `nullexnref` type, aka `(ref null noexn)`.
    pub const NULLEXNREF: Self = ValType::Ref(RefType::NULLEXNREF);

    /// Returns true if `ValType` matches any of the numeric types. (e.g. `I32`,
    /// `I64`, `F32`, `F64`).
    #[inline]
    pub fn is_num(&self) -> bool {
        match self {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 => true,
            _ => false,
        }
    }

    /// Is this the `i32` type?
    #[inline]
    pub fn is_i32(&self) -> bool {
        matches!(self, ValType::I32)
    }

    /// Is this the `i64` type?
    #[inline]
    pub fn is_i64(&self) -> bool {
        matches!(self, ValType::I64)
    }

    /// Is this the `f32` type?
    #[inline]
    pub fn is_f32(&self) -> bool {
        matches!(self, ValType::F32)
    }

    /// Is this the `f64` type?
    #[inline]
    pub fn is_f64(&self) -> bool {
        matches!(self, ValType::F64)
    }

    /// Is this the `v128` type?
    #[inline]
    pub fn is_v128(&self) -> bool {
        matches!(self, ValType::V128)
    }

    /// Returns true if `ValType` is any kind of reference type.
    #[inline]
    pub fn is_ref(&self) -> bool {
        matches!(self, ValType::Ref(_))
    }

    /// Is this the `funcref` (aka `(ref null func)`) type?
    #[inline]
    pub fn is_funcref(&self) -> bool {
        matches!(
            self,
            ValType::Ref(RefType {
                is_nullable: true,
                heap_type: HeapType::Func
            })
        )
    }

    /// Is this the `externref` (aka `(ref null extern)`) type?
    #[inline]
    pub fn is_externref(&self) -> bool {
        matches!(
            self,
            ValType::Ref(RefType {
                is_nullable: true,
                heap_type: HeapType::Extern
            })
        )
    }

    /// Is this the `anyref` (aka `(ref null any)`) type?
    #[inline]
    pub fn is_anyref(&self) -> bool {
        matches!(
            self,
            ValType::Ref(RefType {
                is_nullable: true,
                heap_type: HeapType::Any
            })
        )
    }

    /// Is this the `contref` (aka `(ref null cont)`) type?
    #[inline]
    pub fn is_contref(&self) -> bool {
        matches!(
            self,
            ValType::Ref(RefType {
                is_nullable: true,
                heap_type: HeapType::Cont
            })
        )
    }

    /// Get the underlying reference type, if this value type is a reference
    /// type.
    #[inline]
    pub fn as_ref(&self) -> Option<&RefType> {
        match self {
            ValType::Ref(r) => Some(r),
            _ => None,
        }
    }

    /// Get the underlying reference type, panicking if this value type is not a
    /// reference type.
    #[inline]
    pub fn unwrap_ref(&self) -> &RefType {
        self.as_ref()
            .expect("ValType::unwrap_ref on a non-reference type")
    }

    /// Does this value type match the other type?
    ///
    /// That is, is this value type a subtype of the other?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &ValType) -> bool {
        match (self, other) {
            (Self::I32, Self::I32) => true,
            (Self::I64, Self::I64) => true,
            (Self::F32, Self::F32) => true,
            (Self::F64, Self::F64) => true,
            (Self::V128, Self::V128) => true,
            (Self::Ref(a), Self::Ref(b)) => a.matches(b),
            (Self::I32, _)
            | (Self::I64, _)
            | (Self::F32, _)
            | (Self::F64, _)
            | (Self::V128, _)
            | (Self::Ref(_), _) => false,
        }
    }

    /// Is value type `a` precisely equal to value type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same value type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine.
    pub fn eq(a: &Self, b: &Self) -> bool {
        a.matches(b) && b.matches(a)
    }

    /// Is this a `VMGcRef` type that is not i31 and is not an uninhabited
    /// bottom type?
    #[inline]
    pub(crate) fn is_vmgcref_type_and_points_to_object(&self) -> bool {
        match self {
            ValType::Ref(r) => r.is_vmgcref_type_and_points_to_object(),
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => false,
        }
    }

    pub(crate) fn ensure_matches(&self, engine: &Engine, other: &ValType) -> Result<()> {
        if !self.comes_from_same_engine(engine) || !other.comes_from_same_engine(engine) {
            bail!("type used with wrong engine");
        }
        if self.matches(other) {
            Ok(())
        } else {
            bail!("type mismatch: expected {other}, found {self}")
        }
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        match self {
            Self::I32 | Self::I64 | Self::F32 | Self::F64 | Self::V128 => true,
            Self::Ref(r) => r.comes_from_same_engine(engine),
        }
    }

    pub(crate) fn to_wasm_type(&self) -> WasmValType {
        match self {
            Self::I32 => WasmValType::I32,
            Self::I64 => WasmValType::I64,
            Self::F32 => WasmValType::F32,
            Self::F64 => WasmValType::F64,
            Self::V128 => WasmValType::V128,
            Self::Ref(r) => WasmValType::Ref(r.to_wasm_type()),
        }
    }

    #[inline]
    pub(crate) fn from_wasm_type(engine: &Engine, ty: &WasmValType) -> Self {
        match ty {
            WasmValType::I32 => Self::I32,
            WasmValType::I64 => Self::I64,
            WasmValType::F32 => Self::F32,
            WasmValType::F64 => Self::F64,
            WasmValType::V128 => Self::V128,
            WasmValType::Ref(r) => Self::Ref(RefType::from_wasm_type(engine, r)),
        }
    }
    /// Construct a default value. Returns None for non-nullable Ref types, which have no default.
    pub fn default_value(&self) -> Option<Val> {
        match self {
            ValType::I32 => Some(Val::I32(0)),
            ValType::I64 => Some(Val::I64(0)),
            ValType::F32 => Some(Val::F32(0)),
            ValType::F64 => Some(Val::F64(0)),
            ValType::V128 => Some(Val::V128(0.into())),
            ValType::Ref(r) => {
                if r.is_nullable() {
                    Some(Val::null_ref(r.heap_type()))
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn into_registered_type(self) -> Option<RegisteredType> {
        match self {
            ValType::Ref(ty) => ty.into_registered_type(),
            _ => None,
        }
    }
}

/// Opaque references to data in the Wasm heap or to host data.
///
/// # Subtyping and Equality
///
/// `RefType` does not implement `Eq`, because reference types have a subtyping
/// relationship, and so 99.99% of the time you actually want to check whether
/// one type matches (i.e. is a subtype of) another type. You can use the
/// [`RefType::matches`] and [`Ref::matches_ty`][crate::Ref::matches_ty] methods
/// to perform these types of checks. If, however, you are in that 0.01%
/// scenario where you need to check precise equality between types, you can use
/// the [`RefType::eq`] method.
#[derive(Clone, Hash)]
pub struct RefType {
    is_nullable: bool,
    heap_type: HeapType,
}

impl fmt::Debug for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl fmt::Display for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(ref ")?;
        if self.is_nullable() {
            write!(f, "null ")?;
        }
        write!(f, "{})", self.heap_type())
    }
}

impl RefType {
    /// The `externref` type, aka `(ref null extern)`.
    pub const EXTERNREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Extern,
    };

    /// The `nullexternref` type, aka `(ref null noextern)`.
    pub const NULLEXTERNREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::NoExtern,
    };

    /// The `funcref` type, aka `(ref null func)`.
    pub const FUNCREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Func,
    };

    /// The `nullfuncref` type, aka `(ref null nofunc)`.
    pub const NULLFUNCREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::NoFunc,
    };

    /// The `anyref` type, aka `(ref null any)`.
    pub const ANYREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Any,
    };

    /// The `eqref` type, aka `(ref null eq)`.
    pub const EQREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Eq,
    };

    /// The `i31ref` type, aka `(ref null i31)`.
    pub const I31REF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::I31,
    };

    /// The `arrayref` type, aka `(ref null array)`.
    pub const ARRAYREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Array,
    };

    /// The `structref` type, aka `(ref null struct)`.
    pub const STRUCTREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Struct,
    };

    /// The `nullref` type, aka `(ref null none)`.
    pub const NULLREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::None,
    };

    /// The `contref` type, aka `(ref null cont)`.
    pub const CONTREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Cont,
    };

    /// The `nullcontref` type, aka `(ref null nocont)`.
    pub const NULLCONTREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::NoCont,
    };

    /// The `exnref` type, aka `(ref null exn)`.
    pub const EXNREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::Exn,
    };

    /// The `nullexnref` type, aka `(ref null noexn)`.
    pub const NULLEXNREF: Self = RefType {
        is_nullable: true,
        heap_type: HeapType::NoExn,
    };

    /// Construct a new reference type.
    pub fn new(is_nullable: bool, heap_type: HeapType) -> RefType {
        RefType {
            is_nullable,
            heap_type,
        }
    }

    /// Can this type of reference be null?
    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    /// The heap type that this is a reference to.
    #[inline]
    pub fn heap_type(&self) -> &HeapType {
        &self.heap_type
    }

    /// Does this reference type match the other?
    ///
    /// That is, is this reference type a subtype of the other?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &RefType) -> bool {
        if self.is_nullable() && !other.is_nullable() {
            return false;
        }
        self.heap_type().matches(other.heap_type())
    }

    /// Is reference type `a` precisely equal to reference type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same reference type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine.
    pub fn eq(a: &RefType, b: &RefType) -> bool {
        a.matches(b) && b.matches(a)
    }

    pub(crate) fn ensure_matches(&self, engine: &Engine, other: &RefType) -> Result<()> {
        if !self.comes_from_same_engine(engine) || !other.comes_from_same_engine(engine) {
            bail!("type used with wrong engine");
        }
        if self.matches(other) {
            Ok(())
        } else {
            bail!("type mismatch: expected {other}, found {self}")
        }
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        self.heap_type().comes_from_same_engine(engine)
    }

    pub(crate) fn to_wasm_type(&self) -> WasmRefType {
        WasmRefType {
            nullable: self.is_nullable(),
            heap_type: self.heap_type().to_wasm_type(),
        }
    }

    pub(crate) fn from_wasm_type(engine: &Engine, ty: &WasmRefType) -> RefType {
        RefType {
            is_nullable: ty.nullable,
            heap_type: HeapType::from_wasm_type(engine, &ty.heap_type),
        }
    }

    pub(crate) fn is_vmgcref_type_and_points_to_object(&self) -> bool {
        self.heap_type().is_vmgcref_type_and_points_to_object()
    }

    pub(crate) fn into_registered_type(self) -> Option<RegisteredType> {
        self.heap_type.into_registered_type()
    }
}

/// The heap types that can Wasm can have references to.
///
/// # Subtyping Hierarchy
///
/// Wasm has three different heap type hierarchies:
///
/// 1. Function types
/// 2. External types
/// 3. Internal (struct and array) types
/// 4. Exception types
///
/// Each hierarchy has a top type (the common supertype of which everything else
/// in its hierarchy is a subtype of) and a bottom type (the common subtype of
/// which everything else in its hierarchy is supertype of).
///
/// ## Function Types Hierarchy
///
/// The top of the function types hierarchy is `func`; the bottom is
/// `nofunc`. In between are all the concrete function types.
///
/// ```text
///                          func
///                       /  /  \  \
///      ,----------------  /    \  -------------------------.
///     /                  /      \                           \
///    |              ,----        -----------.                |
///    |              |                       |                |
///    |              |                       |                |
/// (func)    (func (param i32))    (func (param i32 i32))    ...
///    |              |                       |                |
///    |              |                       |                |
///    |              `---.        ,----------'                |
///     \                  \      /                           /
///      `---------------.  \    /  ,------------------------'
///                       \  \  /  /
///                         nofunc
/// ```
///
/// Additionally, some concrete function types are sub- or supertypes of other
/// concrete function types, if that was declared in their definition. For
/// simplicity, this isn't depicted in the diagram above.
///
/// ## External
///
/// The top of the external types hierarchy is `extern`; the bottom is
/// `noextern`. There are no concrete types in this hierarchy.
///
/// ```text
///  extern
///    |
/// noextern
/// ```
///
/// ## Internal
///
/// The top of the internal types hierarchy is `any`; the bottom is `none`. The
/// `eq` type is the common supertype of all types that can be compared for
/// equality. The `struct` and `array` types are the common supertypes of all
/// concrete struct and array types respectively. The `i31` type represents
/// unboxed 31-bit integers.
///
/// ```text
///                                   any
///                                  / | \
///    ,----------------------------'  |  `--------------------------.
///   /                                |                              \
///  |                        .--------'                               |
///  |                        |                                        |
///  |                      struct                                   array
///  |                     /  |   \                                 /  |   \
/// i31             ,-----'   |    '-----.                   ,-----'   |    `-----.
///  |             /          |           \                 /          |           \
///  |            |           |            |               |           |            |
///  |        (struct)    (struct i32)    ...        (array i32)    (array i64)    ...
///  |            |           |            |               |           |            |
///  |             \          |           /                 \          |           /
///   \             `-----.   |    ,-----'                   `-----.   |    ,-----'
///    \                   \  |   /                                 \  |   /
///     \                   \ |  /                                   \ |  /
///      \                   \| /                                     \| /
///       \                   |/                                       |/
///        \                  |                                        |
///         \                 |                                       /
///          \                '--------.                             /
///           \                        |                            /
///            `--------------------.  |   ,-----------------------'
///                                  \ |  /
///                                   none
/// ```
///
/// Additionally, concrete struct and array types can be subtypes of other
/// concrete struct and array types respectively, if that was declared in their
/// definitions. Once again, this is omitted from the above diagram for
/// simplicity.
///
/// ## Exceptions
///
/// The top of the exception types hierarchy is `exn`; the bottom is
/// `noexn`. At the WebAssembly level, there are no concrete types in
/// this hierachy. However, internally we do reify a heap type for
/// each tag, similar to how continuation objects work.
///
/// ```text
///   exn
///  / | \
/// (exn $t) ...
///  \ | /
/// noexn
/// ```
///
/// # Subtyping and Equality
///
/// `HeapType` does not implement `Eq`, because heap types have a subtyping
/// relationship, and so 99.99% of the time you actually want to check whether
/// one type matches (i.e. is a subtype of) another type. You can use the
/// [`HeapType::matches`] method to perform these types of checks. If, however,
/// you are in that 0.01% scenario where you need to check precise equality
/// between types, you can use the [`HeapType::eq`] method.
#[derive(Debug, Clone, Hash)]
pub enum HeapType {
    /// The abstract `extern` heap type represents external host data.
    ///
    /// This is the top type for the external type hierarchy, and therefore is
    /// the common supertype of all external reference types.
    Extern,

    /// The abstract `noextern` heap type represents the null external
    /// reference.
    ///
    /// This is the bottom type for the external type hierarchy, and therefore
    /// is the common subtype of all external reference types.
    NoExtern,

    /// The abstract `func` heap type represents a reference to any kind of
    /// function.
    ///
    /// This is the top type for the function references type hierarchy, and is
    /// therefore a supertype of every function reference.
    Func,

    /// A reference to a function of a specific, concrete type.
    ///
    /// These are subtypes of `func` and supertypes of `nofunc`.
    ConcreteFunc(FuncType),

    /// The abstract `nofunc` heap type represents the null function reference.
    ///
    /// This is the bottom type for the function references type hierarchy, and
    /// therefore `nofunc` is a subtype of all function reference types.
    NoFunc,

    /// The abstract `any` heap type represents all internal Wasm data.
    ///
    /// This is the top type of the internal type hierarchy, and is therefore a
    /// supertype of all internal types (such as `eq`, `i31`, `struct`s, and
    /// `array`s).
    Any,

    /// The abstract `eq` heap type represenets all internal Wasm references
    /// that can be compared for equality.
    ///
    /// This is a subtype of `any` and a supertype of `i31`, `array`, `struct`,
    /// and `none` heap types.
    Eq,

    /// The `i31` heap type represents unboxed 31-bit integers.
    ///
    /// This is a subtype of `any` and `eq`, and a supertype of `none`.
    I31,

    /// The abstract `array` heap type represents a reference to any kind of
    /// array.
    ///
    /// This is a subtype of `any` and `eq`, and a supertype of all concrete
    /// array types, as well as a supertype of the abstract `none` heap type.
    Array,

    /// A reference to an array of a specific, concrete type.
    ///
    /// These are subtypes of the `array` heap type (therefore also a subtype of
    /// `any` and `eq`) and supertypes of the `none` heap type.
    ConcreteArray(ArrayType),

    /// The abstract `struct` heap type represents a reference to any kind of
    /// struct.
    ///
    /// This is a subtype of `any` and `eq`, and a supertype of all concrete
    /// struct types, as well as a supertype of the abstract `none` heap type.
    Struct,

    /// A reference to an struct of a specific, concrete type.
    ///
    /// These are subtypes of the `struct` heap type (therefore also a subtype
    /// of `any` and `eq`) and supertypes of the `none` heap type.
    ConcreteStruct(StructType),

    /// The abstract `exn` heap type represents a reference to any
    /// kind of exception.
    ///
    /// This is a supertype of the internal concrete exception heap
    /// types and the `noexn` heap type.
    Exn,

    /// A concrete exception object with a specific tag.
    ///
    /// These are internal, not exposed at the Wasm level, but useful
    /// in our implementation and host API. These are subtypes of
    /// `exn` and supertypes of `noexn`.
    ConcreteExn(ExnType),

    /// A reference to a continuation of a specific, concrete type.
    ///
    /// These are subtypes of `cont` and supertypes of `nocont`.
    ConcreteCont(ContType),

    /// The `cont` heap type represents a reference to any kind of continuation.
    ///
    /// This is the top type for the continuation objects type hierarchy, and is
    /// therefore a supertype of every continuation object.
    Cont,

    /// The `nocont` heap type represents the null continuation object.
    ///
    /// This is the bottom type for the continuation objects type hierarchy, and
    /// therefore `nocont` is a subtype of all continuation object types.
    NoCont,

    /// The abstract `none` heap type represents the null internal reference.
    ///
    /// This is the bottom type for the internal type hierarchy, and therefore
    /// `none` is a subtype of internal types.
    None,

    /// The `noexn` heap type represents the null exception object.
    ///
    /// This is the bottom type for the exception objects type hierarchy.
    NoExn,
}

impl Display for HeapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeapType::Extern => write!(f, "extern"),
            HeapType::NoExtern => write!(f, "noextern"),
            HeapType::Func => write!(f, "func"),
            HeapType::NoFunc => write!(f, "nofunc"),
            HeapType::Any => write!(f, "any"),
            HeapType::Eq => write!(f, "eq"),
            HeapType::I31 => write!(f, "i31"),
            HeapType::Array => write!(f, "array"),
            HeapType::Struct => write!(f, "struct"),
            HeapType::None => write!(f, "none"),
            HeapType::ConcreteFunc(ty) => write!(f, "(concrete func {:?})", ty.type_index()),
            HeapType::ConcreteArray(ty) => write!(f, "(concrete array {:?})", ty.type_index()),
            HeapType::ConcreteStruct(ty) => write!(f, "(concrete struct {:?})", ty.type_index()),
            HeapType::ConcreteCont(ty) => write!(f, "(concrete cont {:?})", ty.type_index()),
            HeapType::ConcreteExn(ty) => write!(f, "(concrete exn {:?})", ty.type_index()),
            HeapType::Cont => write!(f, "cont"),
            HeapType::NoCont => write!(f, "nocont"),
            HeapType::Exn => write!(f, "exn"),
            HeapType::NoExn => write!(f, "noexn"),
        }
    }
}

impl From<FuncType> for HeapType {
    #[inline]
    fn from(f: FuncType) -> Self {
        HeapType::ConcreteFunc(f)
    }
}

impl From<ArrayType> for HeapType {
    #[inline]
    fn from(a: ArrayType) -> Self {
        HeapType::ConcreteArray(a)
    }
}

impl From<StructType> for HeapType {
    #[inline]
    fn from(s: StructType) -> Self {
        HeapType::ConcreteStruct(s)
    }
}

impl From<ContType> for HeapType {
    #[inline]
    fn from(f: ContType) -> Self {
        HeapType::ConcreteCont(f)
    }
}

impl From<ExnType> for HeapType {
    #[inline]
    fn from(e: ExnType) -> Self {
        HeapType::ConcreteExn(e)
    }
}

impl HeapType {
    /// Is this the abstract `extern` heap type?
    pub fn is_extern(&self) -> bool {
        matches!(self, HeapType::Extern)
    }

    /// Is this the abstract `func` heap type?
    pub fn is_func(&self) -> bool {
        matches!(self, HeapType::Func)
    }

    /// Is this the abstract `nofunc` heap type?
    pub fn is_no_func(&self) -> bool {
        matches!(self, HeapType::NoFunc)
    }

    /// Is this the abstract `any` heap type?
    pub fn is_any(&self) -> bool {
        matches!(self, HeapType::Any)
    }

    /// Is this the abstract `i31` heap type?
    pub fn is_i31(&self) -> bool {
        matches!(self, HeapType::I31)
    }

    /// Is this the abstract `none` heap type?
    pub fn is_none(&self) -> bool {
        matches!(self, HeapType::None)
    }

    /// Is this the abstract `cont` heap type?
    pub fn is_cont(&self) -> bool {
        matches!(self, HeapType::Cont)
    }

    /// Is this the abstract `exn` heap type?
    pub fn is_exn(&self) -> bool {
        matches!(self, HeapType::Exn)
    }

    /// Is this the abstract `noexn` heap type?
    pub fn is_no_exn(&self) -> bool {
        matches!(self, HeapType::NoExn)
    }

    /// Is this an abstract type?
    ///
    /// Types that are not abstract are concrete, user-defined types.
    pub fn is_abstract(&self) -> bool {
        !self.is_concrete()
    }

    /// Is this a concrete, user-defined heap type?
    ///
    /// Types that are not concrete, user-defined types are abstract types.
    #[inline]
    pub fn is_concrete(&self) -> bool {
        matches!(
            self,
            HeapType::ConcreteFunc(_)
                | HeapType::ConcreteArray(_)
                | HeapType::ConcreteStruct(_)
                | HeapType::ConcreteCont(_)
                | HeapType::ConcreteExn(_)
        )
    }

    /// Is this a concrete, user-defined function type?
    pub fn is_concrete_func(&self) -> bool {
        matches!(self, HeapType::ConcreteFunc(_))
    }

    /// Get the underlying concrete, user-defined function type, if any.
    ///
    /// Returns `None` if this is not a concrete function type.
    pub fn as_concrete_func(&self) -> Option<&FuncType> {
        match self {
            HeapType::ConcreteFunc(f) => Some(f),
            _ => None,
        }
    }

    /// Get the underlying concrete, user-defined type, panicking if this is not
    /// a concrete function type.
    pub fn unwrap_concrete_func(&self) -> &FuncType {
        self.as_concrete_func().unwrap()
    }

    /// Is this a concrete, user-defined array type?
    pub fn is_concrete_array(&self) -> bool {
        matches!(self, HeapType::ConcreteArray(_))
    }

    /// Get the underlying concrete, user-defined array type, if any.
    ///
    /// Returns `None` for if this is not a concrete array type.
    pub fn as_concrete_array(&self) -> Option<&ArrayType> {
        match self {
            HeapType::ConcreteArray(f) => Some(f),
            _ => None,
        }
    }

    /// Get the underlying concrete, user-defined type, panicking if this is not
    /// a concrete array type.
    pub fn unwrap_concrete_array(&self) -> &ArrayType {
        self.as_concrete_array().unwrap()
    }

    /// Is this a concrete, user-defined continuation type?
    pub fn is_concrete_cont(&self) -> bool {
        matches!(self, HeapType::ConcreteCont(_))
    }

    /// Get the underlying concrete, user-defined continuation type, if any.
    ///
    /// Returns `None` if this is not a concrete continuation type.
    pub fn as_concrete_cont(&self) -> Option<&ContType> {
        match self {
            HeapType::ConcreteCont(f) => Some(f),
            _ => None,
        }
    }

    /// Is this a concrete, user-defined struct type?
    pub fn is_concrete_struct(&self) -> bool {
        matches!(self, HeapType::ConcreteStruct(_))
    }

    /// Get the underlying concrete, user-defined struct type, if any.
    ///
    /// Returns `None` for if this is not a concrete struct type.
    pub fn as_concrete_struct(&self) -> Option<&StructType> {
        match self {
            HeapType::ConcreteStruct(f) => Some(f),
            _ => None,
        }
    }

    /// Get the underlying concrete, user-defined type, panicking if this is not
    /// a concrete continuation type.
    pub fn unwrap_concrete_cont(&self) -> &ContType {
        self.as_concrete_cont().unwrap()
    }

    /// Get the underlying concrete, user-defined type, panicking if this is not
    /// a concrete struct type.
    pub fn unwrap_concrete_struct(&self) -> &StructType {
        self.as_concrete_struct().unwrap()
    }

    /// Is this a concrete, user-defined exception type?
    pub fn is_concrete_exn(&self) -> bool {
        matches!(self, HeapType::ConcreteExn(_))
    }

    /// Get the underlying concrete, user-defined exception type, if any.
    ///
    /// Returns `None` if this is not a concrete exception type.
    pub fn as_concrete_exn(&self) -> Option<&ExnType> {
        match self {
            HeapType::ConcreteExn(e) => Some(e),
            _ => None,
        }
    }

    /// Get the top type of this heap type's type hierarchy.
    ///
    /// The returned heap type is a supertype of all types in this heap type's
    /// type hierarchy.
    #[inline]
    pub fn top(&self) -> HeapType {
        match self {
            HeapType::Func | HeapType::ConcreteFunc(_) | HeapType::NoFunc => HeapType::Func,

            HeapType::Extern | HeapType::NoExtern => HeapType::Extern,

            HeapType::Any
            | HeapType::Eq
            | HeapType::I31
            | HeapType::Array
            | HeapType::ConcreteArray(_)
            | HeapType::Struct
            | HeapType::ConcreteStruct(_)
            | HeapType::None => HeapType::Any,

            HeapType::Cont | HeapType::ConcreteCont(_) | HeapType::NoCont => HeapType::Cont,

            HeapType::Exn | HeapType::ConcreteExn(_) | HeapType::NoExn => HeapType::Exn,
        }
    }

    /// Is this the top type within its type hierarchy?
    #[inline]
    pub fn is_top(&self) -> bool {
        match self {
            HeapType::Any | HeapType::Extern | HeapType::Func | HeapType::Cont | HeapType::Exn => {
                true
            }
            _ => false,
        }
    }

    /// Get the bottom type of this heap type's type hierarchy.
    ///
    /// The returned heap type is a subtype of all types in this heap type's
    /// type hierarchy.
    #[inline]
    pub fn bottom(&self) -> HeapType {
        match self {
            HeapType::Extern | HeapType::NoExtern => HeapType::NoExtern,

            HeapType::Func | HeapType::ConcreteFunc(_) | HeapType::NoFunc => HeapType::NoFunc,

            HeapType::Any
            | HeapType::Eq
            | HeapType::I31
            | HeapType::Array
            | HeapType::ConcreteArray(_)
            | HeapType::Struct
            | HeapType::ConcreteStruct(_)
            | HeapType::None => HeapType::None,

            HeapType::Cont | HeapType::ConcreteCont(_) | HeapType::NoCont => HeapType::NoCont,

            HeapType::Exn | HeapType::ConcreteExn(_) | HeapType::NoExn => HeapType::NoExn,
        }
    }

    /// Is this the bottom type within its type hierarchy?
    #[inline]
    pub fn is_bottom(&self) -> bool {
        match self {
            HeapType::None
            | HeapType::NoExtern
            | HeapType::NoFunc
            | HeapType::NoCont
            | HeapType::NoExn => true,
            _ => false,
        }
    }

    /// Does this heap type match the other heap type?
    ///
    /// That is, is this heap type a subtype of the other?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &HeapType) -> bool {
        match (self, other) {
            (HeapType::Extern, HeapType::Extern) => true,
            (HeapType::Extern, _) => false,

            (HeapType::NoExtern, HeapType::NoExtern | HeapType::Extern) => true,
            (HeapType::NoExtern, _) => false,

            (HeapType::NoFunc, HeapType::NoFunc | HeapType::ConcreteFunc(_) | HeapType::Func) => {
                true
            }
            (HeapType::NoFunc, _) => false,

            (HeapType::ConcreteFunc(_), HeapType::Func) => true,
            (HeapType::ConcreteFunc(a), HeapType::ConcreteFunc(b)) => {
                assert!(a.comes_from_same_engine(b.engine()));
                a.engine()
                    .signatures()
                    .is_subtype(a.type_index(), b.type_index())
            }
            (HeapType::ConcreteFunc(_), _) => false,

            (HeapType::Func, HeapType::Func) => true,
            (HeapType::Func, _) => false,

            (HeapType::Cont, HeapType::Cont) => true,
            (HeapType::Cont, _) => false,

            (HeapType::NoCont, HeapType::NoCont | HeapType::ConcreteCont(_) | HeapType::Cont) => {
                true
            }
            (HeapType::NoCont, _) => false,

            (HeapType::ConcreteCont(_), HeapType::Cont) => true,
            (HeapType::ConcreteCont(a), HeapType::ConcreteCont(b)) => a.matches(b),
            (HeapType::ConcreteCont(_), _) => false,

            (
                HeapType::None,
                HeapType::None
                | HeapType::ConcreteArray(_)
                | HeapType::Array
                | HeapType::ConcreteStruct(_)
                | HeapType::Struct
                | HeapType::I31
                | HeapType::Eq
                | HeapType::Any,
            ) => true,
            (HeapType::None, _) => false,

            (HeapType::ConcreteArray(_), HeapType::Array | HeapType::Eq | HeapType::Any) => true,
            (HeapType::ConcreteArray(a), HeapType::ConcreteArray(b)) => {
                assert!(a.comes_from_same_engine(b.engine()));
                a.engine()
                    .signatures()
                    .is_subtype(a.type_index(), b.type_index())
            }
            (HeapType::ConcreteArray(_), _) => false,

            (HeapType::Array, HeapType::Array | HeapType::Eq | HeapType::Any) => true,
            (HeapType::Array, _) => false,

            (HeapType::ConcreteStruct(_), HeapType::Struct | HeapType::Eq | HeapType::Any) => true,
            (HeapType::ConcreteStruct(a), HeapType::ConcreteStruct(b)) => {
                assert!(a.comes_from_same_engine(b.engine()));
                a.engine()
                    .signatures()
                    .is_subtype(a.type_index(), b.type_index())
            }
            (HeapType::ConcreteStruct(_), _) => false,

            (HeapType::Struct, HeapType::Struct | HeapType::Eq | HeapType::Any) => true,
            (HeapType::Struct, _) => false,

            (HeapType::I31, HeapType::I31 | HeapType::Eq | HeapType::Any) => true,
            (HeapType::I31, _) => false,

            (HeapType::Eq, HeapType::Eq | HeapType::Any) => true,
            (HeapType::Eq, _) => false,

            (HeapType::Any, HeapType::Any) => true,
            (HeapType::Any, _) => false,

            (HeapType::NoExn, HeapType::Exn | HeapType::ConcreteExn(_) | HeapType::NoExn) => true,
            (HeapType::NoExn, _) => true,

            (HeapType::ConcreteExn(_), HeapType::Exn) => true,
            (HeapType::ConcreteExn(a), HeapType::ConcreteExn(b)) => a.matches(b),
            (HeapType::ConcreteExn(_), _) => false,

            (HeapType::Exn, HeapType::Exn) => true,
            (HeapType::Exn, _) => false,
        }
    }

    /// Is heap type `a` precisely equal to heap type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same heap type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &HeapType, b: &HeapType) -> bool {
        a.matches(b) && b.matches(a)
    }

    pub(crate) fn ensure_matches(&self, engine: &Engine, other: &HeapType) -> Result<()> {
        if !self.comes_from_same_engine(engine) || !other.comes_from_same_engine(engine) {
            bail!("type used with wrong engine");
        }
        if self.matches(other) {
            Ok(())
        } else {
            bail!("type mismatch: expected {other}, found {self}");
        }
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        match self {
            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::NoFunc
            | HeapType::Any
            | HeapType::Eq
            | HeapType::I31
            | HeapType::Array
            | HeapType::Struct
            | HeapType::Cont
            | HeapType::NoCont
            | HeapType::Exn
            | HeapType::NoExn
            | HeapType::None => true,
            HeapType::ConcreteFunc(ty) => ty.comes_from_same_engine(engine),
            HeapType::ConcreteArray(ty) => ty.comes_from_same_engine(engine),
            HeapType::ConcreteStruct(ty) => ty.comes_from_same_engine(engine),
            HeapType::ConcreteCont(ty) => ty.comes_from_same_engine(engine),
            HeapType::ConcreteExn(ty) => ty.comes_from_same_engine(engine),
        }
    }

    pub(crate) fn to_wasm_type(&self) -> WasmHeapType {
        match self {
            HeapType::Extern => WasmHeapType::Extern,
            HeapType::NoExtern => WasmHeapType::NoExtern,
            HeapType::Func => WasmHeapType::Func,
            HeapType::NoFunc => WasmHeapType::NoFunc,
            HeapType::Any => WasmHeapType::Any,
            HeapType::Eq => WasmHeapType::Eq,
            HeapType::I31 => WasmHeapType::I31,
            HeapType::Array => WasmHeapType::Array,
            HeapType::Struct => WasmHeapType::Struct,
            HeapType::None => WasmHeapType::None,
            HeapType::ConcreteFunc(f) => {
                WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::Engine(f.type_index()))
            }
            HeapType::ConcreteArray(a) => {
                WasmHeapType::ConcreteArray(EngineOrModuleTypeIndex::Engine(a.type_index()))
            }
            HeapType::ConcreteStruct(a) => {
                WasmHeapType::ConcreteStruct(EngineOrModuleTypeIndex::Engine(a.type_index()))
            }
            HeapType::Cont => WasmHeapType::Cont,
            HeapType::NoCont => WasmHeapType::NoCont,
            HeapType::ConcreteCont(c) => {
                WasmHeapType::ConcreteCont(EngineOrModuleTypeIndex::Engine(c.type_index()))
            }
            HeapType::Exn => WasmHeapType::Exn,
            HeapType::NoExn => WasmHeapType::NoExn,
            HeapType::ConcreteExn(e) => {
                WasmHeapType::ConcreteExn(EngineOrModuleTypeIndex::Engine(e.type_index()))
            }
        }
    }

    pub(crate) fn from_wasm_type(engine: &Engine, ty: &WasmHeapType) -> HeapType {
        match ty {
            WasmHeapType::Extern => HeapType::Extern,
            WasmHeapType::NoExtern => HeapType::NoExtern,
            WasmHeapType::Func => HeapType::Func,
            WasmHeapType::NoFunc => HeapType::NoFunc,
            WasmHeapType::Any => HeapType::Any,
            WasmHeapType::Eq => HeapType::Eq,
            WasmHeapType::I31 => HeapType::I31,
            WasmHeapType::Array => HeapType::Array,
            WasmHeapType::Struct => HeapType::Struct,
            WasmHeapType::None => HeapType::None,
            WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::Engine(idx)) => {
                HeapType::ConcreteFunc(FuncType::from_shared_type_index(engine, *idx))
            }
            WasmHeapType::ConcreteArray(EngineOrModuleTypeIndex::Engine(idx)) => {
                HeapType::ConcreteArray(ArrayType::from_shared_type_index(engine, *idx))
            }
            WasmHeapType::ConcreteStruct(EngineOrModuleTypeIndex::Engine(idx)) => {
                HeapType::ConcreteStruct(StructType::from_shared_type_index(engine, *idx))
            }

            WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::Module(_))
            | WasmHeapType::ConcreteFunc(EngineOrModuleTypeIndex::RecGroup(_))
            | WasmHeapType::ConcreteArray(EngineOrModuleTypeIndex::Module(_))
            | WasmHeapType::ConcreteArray(EngineOrModuleTypeIndex::RecGroup(_))
            | WasmHeapType::ConcreteStruct(EngineOrModuleTypeIndex::Module(_))
            | WasmHeapType::ConcreteStruct(EngineOrModuleTypeIndex::RecGroup(_))
            | WasmHeapType::ConcreteCont(EngineOrModuleTypeIndex::Module(_))
            | WasmHeapType::ConcreteCont(EngineOrModuleTypeIndex::RecGroup(_))
            | WasmHeapType::ConcreteExn(EngineOrModuleTypeIndex::Module(_))
            | WasmHeapType::ConcreteExn(EngineOrModuleTypeIndex::RecGroup(_)) => {
                panic!("HeapType::from_wasm_type on non-canonicalized-for-runtime-usage heap type")
            }
            WasmHeapType::Cont => HeapType::Cont,
            WasmHeapType::NoCont => HeapType::NoCont,
            WasmHeapType::ConcreteCont(EngineOrModuleTypeIndex::Engine(idx)) => {
                HeapType::ConcreteCont(ContType::from_shared_type_index(engine, *idx))
            }
            WasmHeapType::Exn => HeapType::Exn,
            WasmHeapType::NoExn => HeapType::NoExn,
            WasmHeapType::ConcreteExn(EngineOrModuleTypeIndex::Engine(idx)) => {
                HeapType::ConcreteExn(ExnType::from_shared_type_index(engine, *idx))
            }
        }
    }

    pub(crate) fn as_registered_type(&self) -> Option<&RegisteredType> {
        match self {
            HeapType::ConcreteCont(c) => Some(&c.registered_type),
            HeapType::ConcreteFunc(f) => Some(&f.registered_type),
            HeapType::ConcreteArray(a) => Some(&a.registered_type),
            HeapType::ConcreteStruct(a) => Some(&a.registered_type),
            HeapType::ConcreteExn(e) => Some(&e.registered_type),

            HeapType::Extern
            | HeapType::NoExtern
            | HeapType::Func
            | HeapType::NoFunc
            | HeapType::Any
            | HeapType::Eq
            | HeapType::I31
            | HeapType::Array
            | HeapType::Struct
            | HeapType::Cont
            | HeapType::NoCont
            | HeapType::Exn
            | HeapType::NoExn
            | HeapType::None => None,
        }
    }

    #[inline]
    pub(crate) fn is_vmgcref_type(&self) -> bool {
        match self.top() {
            Self::Any | Self::Extern => true,
            Self::Func => false,
            Self::Cont => false,
            ty => unreachable!("not a top type: {ty:?}"),
        }
    }

    /// Is this a `VMGcRef` type that is not i31 and is not an uninhabited
    /// bottom type?
    #[inline]
    pub(crate) fn is_vmgcref_type_and_points_to_object(&self) -> bool {
        self.is_vmgcref_type()
            && !matches!(
                self,
                HeapType::I31 | HeapType::NoExtern | HeapType::NoFunc | HeapType::None
            )
    }

    pub(crate) fn into_registered_type(self) -> Option<RegisteredType> {
        use HeapType::*;
        match self {
            ConcreteFunc(ty) => Some(ty.registered_type),
            ConcreteArray(ty) => Some(ty.registered_type),
            ConcreteStruct(ty) => Some(ty.registered_type),
            ConcreteCont(ty) => Some(ty.registered_type),
            ConcreteExn(ty) => Some(ty.registered_type),
            Extern | NoExtern | Func | NoFunc | Any | Eq | I31 | Array | Struct | Cont | NoCont
            | Exn | NoExn | None => Option::None,
        }
    }
}

// External Types

/// A list of all possible types which can be externally referenced from a
/// WebAssembly module.
///
/// This list can be found in [`ImportType`] or [`ExportType`], so these types
/// can either be imported or exported.
#[derive(Debug, Clone)]
pub enum ExternType {
    /// This external type is the type of a WebAssembly function.
    Func(FuncType),
    /// This external type is the type of a WebAssembly global.
    Global(GlobalType),
    /// This external type is the type of a WebAssembly table.
    Table(TableType),
    /// This external type is the type of a WebAssembly memory.
    Memory(MemoryType),
    /// This external type is the type of a WebAssembly tag.
    Tag(TagType),
}

macro_rules! extern_type_accessors {
    ($(($variant:ident($ty:ty) $get:ident $unwrap:ident))*) => ($(
        /// Attempt to return the underlying type of this external type,
        /// returning `None` if it is a different type.
        pub fn $get(&self) -> Option<&$ty> {
            if let ExternType::$variant(e) = self {
                Some(e)
            } else {
                None
            }
        }

        /// Returns the underlying descriptor of this [`ExternType`], panicking
        /// if it is a different type.
        ///
        /// # Panics
        ///
        /// Panics if `self` is not of the right type.
        pub fn $unwrap(&self) -> &$ty {
            self.$get().expect(concat!("expected ", stringify!($ty)))
        }
    )*)
}

impl ExternType {
    extern_type_accessors! {
        (Func(FuncType) func unwrap_func)
        (Global(GlobalType) global unwrap_global)
        (Table(TableType) table unwrap_table)
        (Memory(MemoryType) memory unwrap_memory)
        (Tag(TagType) tag unwrap_tag)
    }

    pub(crate) fn from_wasmtime(
        engine: &Engine,
        types: &ModuleTypes,
        ty: &EntityType,
    ) -> ExternType {
        match ty {
            EntityType::Function(idx) => match idx {
                EngineOrModuleTypeIndex::Engine(e) => {
                    FuncType::from_shared_type_index(engine, *e).into()
                }
                EngineOrModuleTypeIndex::Module(m) => {
                    let subty = &types[*m];
                    debug_assert!(subty.is_canonicalized_for_runtime_usage());
                    // subty.canonicalize_for_runtime_usage(&mut |idx| {
                    //     signatures.shared_type(idx).unwrap()
                    // });
                    FuncType::from_wasm_func_type(
                        engine,
                        subty.is_final,
                        subty.supertype,
                        subty.unwrap_func().clone(),
                    )
                    .into()
                }
                EngineOrModuleTypeIndex::RecGroup(_) => unreachable!(),
            },
            EntityType::Global(ty) => GlobalType::from_wasmtime_global(engine, ty).into(),
            EntityType::Memory(ty) => MemoryType::from_wasmtime_memory(ty).into(),
            EntityType::Table(ty) => TableType::from_wasmtime_table(engine, ty).into(),
            EntityType::Tag(ty) => TagType::from_wasmtime_tag(engine, ty).into(),
        }
    }
    /// Construct a default value, if possible for the underlying type. Tags do not have a default value.
    pub fn default_value(&self, store: impl AsContextMut) -> Result<Extern> {
        match self {
            ExternType::Func(func_ty) => func_ty.default_value(store).map(Extern::Func),
            ExternType::Global(global_ty) => global_ty.default_value(store).map(Extern::Global),
            ExternType::Table(table_ty) => table_ty.default_value(store).map(Extern::Table),
            ExternType::Memory(mem_ty) => mem_ty.default_value(store).map(Extern::Memory),
            ExternType::Tag(_) => bail!("default tags not supported yet"), // FIXME: #10252
        }
    }
}

impl From<FuncType> for ExternType {
    fn from(ty: FuncType) -> ExternType {
        ExternType::Func(ty)
    }
}

impl From<GlobalType> for ExternType {
    fn from(ty: GlobalType) -> ExternType {
        ExternType::Global(ty)
    }
}

impl From<MemoryType> for ExternType {
    fn from(ty: MemoryType) -> ExternType {
        ExternType::Memory(ty)
    }
}

impl From<TableType> for ExternType {
    fn from(ty: TableType) -> ExternType {
        ExternType::Table(ty)
    }
}

impl From<TagType> for ExternType {
    fn from(ty: TagType) -> ExternType {
        ExternType::Tag(ty)
    }
}

/// The storage type of a `struct` field or `array` element.
///
/// This is either a packed 8- or -16 bit integer, or else it is some unpacked
/// Wasm value type.
#[derive(Clone, Hash)]
pub enum StorageType {
    /// `i8`, an 8-bit integer.
    I8,
    /// `i16`, a 16-bit integer.
    I16,
    /// A value type.
    ValType(ValType),
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageType::I8 => write!(f, "i8"),
            StorageType::I16 => write!(f, "i16"),
            StorageType::ValType(ty) => fmt::Display::fmt(ty, f),
        }
    }
}

impl From<ValType> for StorageType {
    #[inline]
    fn from(v: ValType) -> Self {
        StorageType::ValType(v)
    }
}

impl From<RefType> for StorageType {
    #[inline]
    fn from(r: RefType) -> Self {
        StorageType::ValType(r.into())
    }
}

impl StorageType {
    /// Is this an `i8`?
    #[inline]
    pub fn is_i8(&self) -> bool {
        matches!(self, Self::I8)
    }

    /// Is this an `i16`?
    #[inline]
    pub fn is_i16(&self) -> bool {
        matches!(self, Self::I16)
    }

    /// Is this a Wasm value type?
    #[inline]
    pub fn is_val_type(&self) -> bool {
        matches!(self, Self::I16)
    }

    /// Get this storage type's underlying value type, if any.
    ///
    /// Returns `None` if this storage type is not a value type.
    #[inline]
    pub fn as_val_type(&self) -> Option<&ValType> {
        match self {
            Self::ValType(v) => Some(v),
            _ => None,
        }
    }

    /// Get this storage type's underlying value type, panicking if it is not a
    /// value type.
    pub fn unwrap_val_type(&self) -> &ValType {
        self.as_val_type().unwrap()
    }

    /// Unpack this (possibly packed) storage type into a full `ValType`.
    ///
    /// If this is a `StorageType::ValType`, then the inner `ValType` is
    /// returned as-is.
    ///
    /// If this is a packed `StorageType::I8` or `StorageType::I16, then a
    /// `ValType::I32` is returned.
    pub fn unpack(&self) -> &ValType {
        match self {
            StorageType::I8 | StorageType::I16 => &ValType::I32,
            StorageType::ValType(ty) => ty,
        }
    }

    /// Does this field type match the other field type?
    ///
    /// That is, is this field type a subtype of the other field type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (StorageType::I8, StorageType::I8) => true,
            (StorageType::I8, _) => false,
            (StorageType::I16, StorageType::I16) => true,
            (StorageType::I16, _) => false,
            (StorageType::ValType(a), StorageType::ValType(b)) => a.matches(b),
            (StorageType::ValType(_), _) => false,
        }
    }

    /// Is field type `a` precisely equal to field type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same field type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &Self, b: &Self) -> bool {
        match (a, b) {
            (StorageType::I8, StorageType::I8) => true,
            (StorageType::I8, _) => false,
            (StorageType::I16, StorageType::I16) => true,
            (StorageType::I16, _) => false,
            (StorageType::ValType(a), StorageType::ValType(b)) => ValType::eq(a, b),
            (StorageType::ValType(_), _) => false,
        }
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        match self {
            StorageType::I8 | StorageType::I16 => true,
            StorageType::ValType(v) => v.comes_from_same_engine(engine),
        }
    }

    pub(crate) fn from_wasm_storage_type(engine: &Engine, ty: &WasmStorageType) -> Self {
        match ty {
            WasmStorageType::I8 => Self::I8,
            WasmStorageType::I16 => Self::I16,
            WasmStorageType::Val(v) => ValType::from_wasm_type(engine, &v).into(),
        }
    }

    pub(crate) fn to_wasm_storage_type(&self) -> WasmStorageType {
        match self {
            Self::I8 => WasmStorageType::I8,
            Self::I16 => WasmStorageType::I16,
            Self::ValType(v) => WasmStorageType::Val(v.to_wasm_type()),
        }
    }

    /// The byte size of this type, if it has a defined size in the spec.
    ///
    /// See
    /// https://webassembly.github.io/gc/core/syntax/types.html#bitwidth-fieldtype
    /// and
    /// https://webassembly.github.io/gc/core/syntax/types.html#bitwidth-valtype
    #[cfg(feature = "gc")]
    pub(crate) fn data_byte_size(&self) -> Option<u32> {
        match self {
            StorageType::I8 => Some(1),
            StorageType::I16 => Some(2),
            StorageType::ValType(ValType::I32 | ValType::F32) => Some(4),
            StorageType::ValType(ValType::I64 | ValType::F64) => Some(8),
            StorageType::ValType(ValType::V128) => Some(16),
            StorageType::ValType(ValType::Ref(_)) => None,
        }
    }
}

/// The type of a `struct` field or an `array`'s elements.
///
/// This is a pair of both the field's storage type and its mutability
/// (i.e. whether the field can be updated or not).
#[derive(Clone, Hash)]
pub struct FieldType {
    mutability: Mutability,
    element_type: StorageType,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mutability.is_var() {
            write!(f, "(mut {})", self.element_type)
        } else {
            fmt::Display::fmt(&self.element_type, f)
        }
    }
}

impl FieldType {
    /// Construct a new field type from the given parts.
    #[inline]
    pub fn new(mutability: Mutability, element_type: StorageType) -> Self {
        Self {
            mutability,
            element_type,
        }
    }

    /// Get whether or not this field type is mutable.
    #[inline]
    pub fn mutability(&self) -> Mutability {
        self.mutability
    }

    /// Get this field type's storage type.
    #[inline]
    pub fn element_type(&self) -> &StorageType {
        &self.element_type
    }

    /// Does this field type match the other field type?
    ///
    /// That is, is this field type a subtype of the other field type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &Self) -> bool {
        // Our storage type must match `other`'s storage type and either
        //
        // 1. Both field types are immutable, or
        //
        // 2. Both field types are mutable and `other`'s storage type must match
        //    ours, i.e. the storage types are exactly the same.
        use Mutability as M;
        match (self.mutability, other.mutability) {
            // Case 1
            (M::Const, M::Const) => self.element_type.matches(&other.element_type),
            // Case 2
            (M::Var, M::Var) => StorageType::eq(&self.element_type, &other.element_type),
            // Does not match.
            _ => false,
        }
    }

    /// Is field type `a` precisely equal to field type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same field type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &Self, b: &Self) -> bool {
        a.matches(b) && b.matches(a)
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        self.element_type.comes_from_same_engine(engine)
    }

    pub(crate) fn from_wasm_field_type(engine: &Engine, ty: &WasmFieldType) -> Self {
        Self {
            mutability: if ty.mutable {
                Mutability::Var
            } else {
                Mutability::Const
            },
            element_type: StorageType::from_wasm_storage_type(engine, &ty.element_type),
        }
    }

    pub(crate) fn to_wasm_field_type(&self) -> WasmFieldType {
        WasmFieldType {
            element_type: self.element_type.to_wasm_storage_type(),
            mutable: matches!(self.mutability, Mutability::Var),
        }
    }
}

/// The type of a WebAssembly struct.
///
/// WebAssembly structs are a static, fixed-length, ordered sequence of
/// fields. Fields are named by index, not an identifier. Each field is mutable
/// or constant and stores unpacked [`Val`][crate::Val]s or packed 8-/16-bit
/// integers.
///
/// # Subtyping and Equality
///
/// `StructType` does not implement `Eq`, because reference types have a
/// subtyping relationship, and so 99.99% of the time you actually want to check
/// whether one type matches (i.e. is a subtype of) another type. You can use
/// the [`StructType::matches`] method to perform these types of checks. If,
/// however, you are in that 0.01% scenario where you need to check precise
/// equality between types, you can use the [`StructType::eq`] method.
//
// TODO: Once we have struct values, update above docs with a reference to the
// future `Struct::matches_ty` method
#[derive(Debug, Clone, Hash)]
pub struct StructType {
    registered_type: RegisteredType,
}

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(struct")?;
        for field in self.fields() {
            write!(f, " (field {field})")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl StructType {
    /// Construct a new `StructType` with the given field types.
    ///
    /// This `StructType` will be final and without a supertype.
    ///
    /// The result will be associated with the given engine, and attempts to use
    /// it with other engines will panic (for example, checking whether it is a
    /// subtype of another struct type that is associated with a different
    /// engine).
    ///
    /// Returns an error if the number of fields exceeds the implementation
    /// limit.
    ///
    /// # Panics
    ///
    /// Panics if any given field type is not associated with the given engine.
    pub fn new(engine: &Engine, fields: impl IntoIterator<Item = FieldType>) -> Result<Self> {
        Self::with_finality_and_supertype(engine, Finality::Final, None, fields)
    }

    /// Construct a new `StructType` with the given finality, supertype, and
    /// fields.
    ///
    /// The result will be associated with the given engine, and attempts to use
    /// it with other engines will panic (for example, checking whether it is a
    /// subtype of another struct type that is associated with a different
    /// engine).
    ///
    /// Returns an error if the number of fields exceeds the implementation
    /// limit, if the supertype is final, or if this type does not match the
    /// supertype.
    ///
    /// # Panics
    ///
    /// Panics if any given field type is not associated with the given engine.
    pub fn with_finality_and_supertype(
        engine: &Engine,
        finality: Finality,
        supertype: Option<&Self>,
        fields: impl IntoIterator<Item = FieldType>,
    ) -> Result<Self> {
        let fields = fields.into_iter();

        let mut wasmtime_fields = Vec::with_capacity({
            let size_hint = fields.size_hint();
            let cap = size_hint.1.unwrap_or(size_hint.0);
            // Only reserve space if we have a supertype, as that is the only time
            // that this vec is used.
            supertype.is_some() as usize * cap
        });

        // Same as in `FuncType::new`: we must prevent any `RegisteredType`s
        // from being reclaimed while constructing this struct type.
        let mut registrations = smallvec::SmallVec::<[_; 4]>::new();

        let fields = fields
            .map(|ty: FieldType| {
                assert!(ty.comes_from_same_engine(engine));

                if supertype.is_some() {
                    wasmtime_fields.push(ty.clone());
                }

                if let Some(r) = ty.element_type.as_val_type().and_then(|v| v.as_ref()) {
                    if let Some(r) = r.heap_type().as_registered_type() {
                        registrations.push(r.clone());
                    }
                }

                ty.to_wasm_field_type()
            })
            .collect();

        if let Some(supertype) = supertype {
            ensure!(
                supertype.finality().is_non_final(),
                "cannot create a subtype of a final supertype"
            );
            ensure!(
                Self::fields_match(wasmtime_fields.into_iter(), supertype.fields()),
                "struct fields must match their supertype's fields"
            );
        }

        Self::from_wasm_struct_type(
            engine,
            finality.is_final(),
            false,
            supertype.map(|ty| ty.type_index().into()),
            WasmStructType { fields },
        )
    }

    /// Get the engine that this struct type is associated with.
    pub fn engine(&self) -> &Engine {
        self.registered_type.engine()
    }

    /// Get the finality of this struct type.
    pub fn finality(&self) -> Finality {
        match self.registered_type.is_final {
            true => Finality::Final,
            false => Finality::NonFinal,
        }
    }

    /// Get the supertype of this struct type, if any.
    pub fn supertype(&self) -> Option<Self> {
        self.registered_type
            .supertype
            .map(|ty| Self::from_shared_type_index(self.engine(), ty.unwrap_engine_type_index()))
    }

    /// Get the `i`th field type.
    ///
    /// Returns `None` if `i` is out of bounds.
    pub fn field(&self, i: usize) -> Option<FieldType> {
        let engine = self.engine();
        self.as_wasm_struct_type()
            .fields
            .get(i)
            .map(|ty| FieldType::from_wasm_field_type(engine, ty))
    }

    /// Returns the list of field types for this function.
    #[inline]
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FieldType> + '_ {
        let engine = self.engine();
        self.as_wasm_struct_type()
            .fields
            .iter()
            .map(|ty| FieldType::from_wasm_field_type(engine, ty))
    }

    /// Does this struct type match the other struct type?
    ///
    /// That is, is this function type a subtype of the other struct type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &StructType) -> bool {
        assert!(self.comes_from_same_engine(other.engine()));

        self.engine()
            .signatures()
            .is_subtype(self.type_index(), other.type_index())
    }

    fn fields_match(
        a: impl ExactSizeIterator<Item = FieldType>,
        b: impl ExactSizeIterator<Item = FieldType>,
    ) -> bool {
        a.len() >= b.len() && a.zip(b).all(|(a, b)| a.matches(&b))
    }

    /// Is struct type `a` precisely equal to struct type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same struct type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &StructType, b: &StructType) -> bool {
        assert!(a.comes_from_same_engine(b.engine()));
        a.type_index() == b.type_index()
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        Engine::same(self.registered_type().engine(), engine)
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.registered_type().index()
    }

    pub(crate) fn as_wasm_struct_type(&self) -> &WasmStructType {
        self.registered_type().unwrap_struct()
    }

    pub(crate) fn registered_type(&self) -> &RegisteredType {
        &self.registered_type
    }

    /// Construct a `StructType` from a `WasmStructType`.
    ///
    /// This method should only be used when something has already registered --
    /// and is *keeping registered* -- any other concrete Wasm types referenced
    /// by the given `WasmStructType`.
    ///
    /// For example, this method may be called to convert an struct type from
    /// within a Wasm module's `ModuleTypes` since the Wasm module itself is
    /// holding a strong reference to all of its types, including any `(ref null
    /// <index>)` types used as the element type for this struct type.
    pub(crate) fn from_wasm_struct_type(
        engine: &Engine,
        is_final: bool,
        is_shared: bool,
        supertype: Option<EngineOrModuleTypeIndex>,
        ty: WasmStructType,
    ) -> Result<StructType> {
        const MAX_FIELDS: usize = 10_000;
        let fields_len = ty.fields.len();
        ensure!(
            fields_len <= MAX_FIELDS,
            "attempted to define a struct type with {fields_len} fields, but \
             that is more than the maximum supported number of fields \
             ({MAX_FIELDS})",
        );

        let ty = RegisteredType::new(
            engine,
            WasmSubType {
                is_final,
                supertype,
                composite_type: WasmCompositeType {
                    shared: is_shared,
                    inner: WasmCompositeInnerType::Struct(ty),
                },
            },
        );
        Ok(Self {
            registered_type: ty,
        })
    }

    pub(crate) fn from_shared_type_index(engine: &Engine, index: VMSharedTypeIndex) -> StructType {
        let ty = RegisteredType::root(engine, index);
        Self::from_registered_type(ty)
    }

    pub(crate) fn from_registered_type(registered_type: RegisteredType) -> Self {
        debug_assert!(registered_type.is_struct());
        Self { registered_type }
    }
}

/// The type of a WebAssembly array.
///
/// WebAssembly arrays are dynamically-sized, but not resizable. They contain
/// either unpacked [`Val`][crate::Val]s or packed 8-/16-bit integers.
///
/// # Subtyping and Equality
///
/// `ArrayType` does not implement `Eq`, because reference types have a
/// subtyping relationship, and so 99.99% of the time you actually want to check
/// whether one type matches (i.e. is a subtype of) another type. You can use
/// the [`ArrayType::matches`] method to perform these types of checks. If,
/// however, you are in that 0.01% scenario where you need to check precise
/// equality between types, you can use the [`ArrayType::eq`] method.
//
// TODO: Once we have array values, update above docs with a reference to the
// future `Array::matches_ty` method
#[derive(Debug, Clone, Hash)]
pub struct ArrayType {
    registered_type: RegisteredType,
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_ty = self.field_type();
        write!(f, "(array (field {field_ty}))")?;
        Ok(())
    }
}

impl ArrayType {
    /// Construct a new `ArrayType` with the given field type's mutability and
    /// storage type.
    ///
    /// The new `ArrayType` will be final and without a supertype.
    ///
    /// The result will be associated with the given engine, and attempts to use
    /// it with other engines will panic (for example, checking whether it is a
    /// subtype of another array type that is associated with a different
    /// engine).
    ///
    /// # Panics
    ///
    /// Panics if the given field type is not associated with the given engine.
    pub fn new(engine: &Engine, field_type: FieldType) -> Self {
        Self::with_finality_and_supertype(engine, Finality::Final, None, field_type)
            .expect("cannot fail without a supertype")
    }

    /// Construct a new `StructType` with the given finality, supertype, and
    /// fields.
    ///
    /// The result will be associated with the given engine, and attempts to use
    /// it with other engines will panic (for example, checking whether it is a
    /// subtype of another struct type that is associated with a different
    /// engine).
    ///
    /// Returns an error if the supertype is final, or if this type does not
    /// match the supertype.
    ///
    /// # Panics
    ///
    /// Panics if the given field type is not associated with the given engine.
    pub fn with_finality_and_supertype(
        engine: &Engine,
        finality: Finality,
        supertype: Option<&Self>,
        field_type: FieldType,
    ) -> Result<Self> {
        if let Some(supertype) = supertype {
            assert!(supertype.comes_from_same_engine(engine));
            ensure!(
                supertype.finality().is_non_final(),
                "cannot create a subtype of a final supertype"
            );
            ensure!(
                field_type.matches(&supertype.field_type()),
                "array field type must match its supertype's field type"
            );
        }

        // Same as in `FuncType::new`: we must prevent any `RegisteredType` in
        // `field_type` from being reclaimed while constructing this array type.
        let _registration = field_type
            .element_type
            .as_val_type()
            .and_then(|v| v.as_ref())
            .and_then(|r| r.heap_type().as_registered_type());

        assert!(field_type.comes_from_same_engine(engine));
        let wasm_ty = WasmArrayType(field_type.to_wasm_field_type());

        Ok(Self::from_wasm_array_type(
            engine,
            finality.is_final(),
            supertype.map(|ty| ty.type_index().into()),
            wasm_ty,
        ))
    }

    /// Get the engine that this array type is associated with.
    pub fn engine(&self) -> &Engine {
        self.registered_type.engine()
    }

    /// Get the finality of this array type.
    pub fn finality(&self) -> Finality {
        match self.registered_type.is_final {
            true => Finality::Final,
            false => Finality::NonFinal,
        }
    }

    /// Get the supertype of this array type, if any.
    pub fn supertype(&self) -> Option<Self> {
        self.registered_type
            .supertype
            .map(|ty| Self::from_shared_type_index(self.engine(), ty.unwrap_engine_type_index()))
    }

    /// Get this array's underlying field type.
    ///
    /// The field type contains information about both this array type's
    /// mutability and the storage type used for its elements.
    pub fn field_type(&self) -> FieldType {
        FieldType::from_wasm_field_type(self.engine(), &self.as_wasm_array_type().0)
    }

    /// Get this array type's mutability and whether its instances' elements can
    /// be updated or not.
    ///
    /// This is a convenience method providing a short-hand for
    /// `my_array_type.field_type().mutability()`.
    pub fn mutability(&self) -> Mutability {
        if self.as_wasm_array_type().0.mutable {
            Mutability::Var
        } else {
            Mutability::Const
        }
    }

    /// Get the storage type used for this array type's elements.
    ///
    /// This is a convenience method providing a short-hand for
    /// `my_array_type.field_type().element_type()`.
    pub fn element_type(&self) -> StorageType {
        StorageType::from_wasm_storage_type(
            self.engine(),
            &self.registered_type.unwrap_array().0.element_type,
        )
    }

    /// Does this array type match the other array type?
    ///
    /// That is, is this function type a subtype of the other array type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &ArrayType) -> bool {
        assert!(self.comes_from_same_engine(other.engine()));

        self.engine()
            .signatures()
            .is_subtype(self.type_index(), other.type_index())
    }

    /// Is array type `a` precisely equal to array type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same array type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &ArrayType, b: &ArrayType) -> bool {
        assert!(a.comes_from_same_engine(b.engine()));
        a.type_index() == b.type_index()
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        Engine::same(self.registered_type.engine(), engine)
    }

    #[cfg(feature = "gc")]
    pub(crate) fn registered_type(&self) -> &RegisteredType {
        &self.registered_type
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.registered_type.index()
    }

    pub(crate) fn as_wasm_array_type(&self) -> &WasmArrayType {
        self.registered_type.unwrap_array()
    }

    /// Construct a `ArrayType` from a `WasmArrayType`.
    ///
    /// This method should only be used when something has already registered --
    /// and is *keeping registered* -- any other concrete Wasm types referenced
    /// by the given `WasmArrayType`.
    ///
    /// For example, this method may be called to convert an array type from
    /// within a Wasm module's `ModuleTypes` since the Wasm module itself is
    /// holding a strong reference to all of its types, including any `(ref null
    /// <index>)` types used as the element type for this array type.
    pub(crate) fn from_wasm_array_type(
        engine: &Engine,
        is_final: bool,
        supertype: Option<EngineOrModuleTypeIndex>,
        ty: WasmArrayType,
    ) -> ArrayType {
        let ty = RegisteredType::new(
            engine,
            WasmSubType {
                is_final,
                supertype,
                composite_type: WasmCompositeType {
                    shared: false,
                    inner: WasmCompositeInnerType::Array(ty),
                },
            },
        );
        Self {
            registered_type: ty,
        }
    }

    pub(crate) fn from_shared_type_index(engine: &Engine, index: VMSharedTypeIndex) -> ArrayType {
        let ty = RegisteredType::root(engine, index);
        Self::from_registered_type(ty)
    }

    pub(crate) fn from_registered_type(registered_type: RegisteredType) -> Self {
        debug_assert!(registered_type.is_array());
        Self { registered_type }
    }
}

/// The type of a WebAssembly function.
///
/// WebAssembly functions can have 0 or more parameters and results.
///
/// # Subtyping and Equality
///
/// `FuncType` does not implement `Eq`, because reference types have a subtyping
/// relationship, and so 99.99% of the time you actually want to check whether
/// one type matches (i.e. is a subtype of) another type. You can use the
/// [`FuncType::matches`] and [`Func::matches_ty`][crate::Func::matches_ty]
/// methods to perform these types of checks. If, however, you are in that 0.01%
/// scenario where you need to check precise equality between types, you can use
/// the [`FuncType::eq`] method.
#[derive(Debug, Clone, Hash)]
pub struct FuncType {
    registered_type: RegisteredType,
}

impl Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(type (func")?;
        if self.params().len() > 0 {
            write!(f, " (param")?;
            for p in self.params() {
                write!(f, " {p}")?;
            }
            write!(f, ")")?;
        }
        if self.results().len() > 0 {
            write!(f, " (result")?;
            for r in self.results() {
                write!(f, " {r}")?;
            }
            write!(f, ")")?;
        }
        write!(f, "))")
    }
}

impl FuncType {
    /// Creates a new function type from the given parameters and results.
    ///
    /// The function type returned will represent a function which takes
    /// `params` as arguments and returns `results` when it is finished.
    ///
    /// The resulting function type will be final and without a supertype.
    ///
    /// # Panics
    ///
    /// Panics if any parameter or value type is not associated with the given
    /// engine.
    pub fn new(
        engine: &Engine,
        params: impl IntoIterator<Item = ValType>,
        results: impl IntoIterator<Item = ValType>,
    ) -> FuncType {
        Self::with_finality_and_supertype(engine, Finality::Final, None, params, results)
            .expect("cannot fail without a supertype")
    }

    /// Create a new function type with the given finality, supertype, parameter
    /// types, and result types.
    ///
    /// Returns an error if the supertype is final, or if this function type
    /// does not match the supertype.
    ///
    /// # Panics
    ///
    /// Panics if any parameter or value type is not associated with the given
    /// engine.
    pub fn with_finality_and_supertype(
        engine: &Engine,
        finality: Finality,
        supertype: Option<&Self>,
        params: impl IntoIterator<Item = ValType>,
        results: impl IntoIterator<Item = ValType>,
    ) -> Result<Self> {
        let params = params.into_iter();
        let results = results.into_iter();

        let mut wasmtime_params = Vec::with_capacity({
            let size_hint = params.size_hint();
            let cap = size_hint.1.unwrap_or(size_hint.0);
            // Only reserve space if we have a supertype, as that is the only time
            // that this vec is used.
            supertype.is_some() as usize * cap
        });

        let mut wasmtime_results = Vec::with_capacity({
            let size_hint = results.size_hint();
            let cap = size_hint.1.unwrap_or(size_hint.0);
            // Same as above.
            supertype.is_some() as usize * cap
        });

        // Keep any of our parameters' and results' `RegisteredType`s alive
        // across `Self::from_wasm_func_type`. If one of our given `ValType`s is
        // the only thing keeping a type in the registry, we don't want to
        // unregister it when we convert the `ValType` into a `WasmValType` just
        // before we register our new `WasmFuncType` that will reference it.
        let mut registrations = smallvec::SmallVec::<[_; 4]>::new();

        let mut to_wasm_type = |ty: ValType, vec: &mut Vec<_>| {
            assert!(ty.comes_from_same_engine(engine));

            if supertype.is_some() {
                vec.push(ty.clone());
            }

            if let Some(r) = ty.as_ref() {
                if let Some(r) = r.heap_type().as_registered_type() {
                    registrations.push(r.clone());
                }
            }

            ty.to_wasm_type()
        };

        let wasm_func_ty = WasmFuncType::new(
            params
                .map(|p| to_wasm_type(p, &mut wasmtime_params))
                .collect(),
            results
                .map(|r| to_wasm_type(r, &mut wasmtime_results))
                .collect(),
        );

        if let Some(supertype) = supertype {
            assert!(supertype.comes_from_same_engine(engine));
            ensure!(
                supertype.finality().is_non_final(),
                "cannot create a subtype of a final supertype"
            );
            ensure!(
                Self::matches_impl(
                    wasmtime_params.iter().cloned(),
                    supertype.params(),
                    wasmtime_results.iter().cloned(),
                    supertype.results()
                ),
                "function type must match its supertype: found (func{params}{results}), expected \
                 {supertype}",
                params = if wasmtime_params.is_empty() {
                    String::new()
                } else {
                    let mut s = format!(" (params");
                    for p in &wasmtime_params {
                        write!(&mut s, " {p}").unwrap();
                    }
                    s.push(')');
                    s
                },
                results = if wasmtime_results.is_empty() {
                    String::new()
                } else {
                    let mut s = format!(" (results");
                    for r in &wasmtime_results {
                        write!(&mut s, " {r}").unwrap();
                    }
                    s.push(')');
                    s
                },
            );
        }

        Ok(Self::from_wasm_func_type(
            engine,
            finality.is_final(),
            supertype.map(|ty| ty.type_index().into()),
            wasm_func_ty,
        ))
    }

    /// Get the engine that this function type is associated with.
    pub fn engine(&self) -> &Engine {
        self.registered_type.engine()
    }

    /// Get the finality of this function type.
    pub fn finality(&self) -> Finality {
        match self.registered_type.is_final {
            true => Finality::Final,
            false => Finality::NonFinal,
        }
    }

    /// Get the supertype of this function type, if any.
    pub fn supertype(&self) -> Option<Self> {
        self.registered_type
            .supertype
            .map(|ty| Self::from_shared_type_index(self.engine(), ty.unwrap_engine_type_index()))
    }

    /// Get the `i`th parameter type.
    ///
    /// Returns `None` if `i` is out of bounds.
    pub fn param(&self, i: usize) -> Option<ValType> {
        let engine = self.engine();
        self.registered_type
            .unwrap_func()
            .params()
            .get(i)
            .map(|ty| ValType::from_wasm_type(engine, ty))
    }

    /// Returns the list of parameter types for this function.
    #[inline]
    pub fn params(&self) -> impl ExactSizeIterator<Item = ValType> + '_ {
        let engine = self.engine();
        self.registered_type
            .unwrap_func()
            .params()
            .iter()
            .map(|ty| ValType::from_wasm_type(engine, ty))
    }

    /// Get the `i`th result type.
    ///
    /// Returns `None` if `i` is out of bounds.
    pub fn result(&self, i: usize) -> Option<ValType> {
        let engine = self.engine();
        self.registered_type
            .unwrap_func()
            .returns()
            .get(i)
            .map(|ty| ValType::from_wasm_type(engine, ty))
    }

    /// Returns the list of result types for this function.
    #[inline]
    pub fn results(&self) -> impl ExactSizeIterator<Item = ValType> + '_ {
        let engine = self.engine();
        self.registered_type
            .unwrap_func()
            .returns()
            .iter()
            .map(|ty| ValType::from_wasm_type(engine, ty))
    }

    /// Does this function type match the other function type?
    ///
    /// That is, is this function type a subtype of the other function type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &FuncType) -> bool {
        assert!(self.comes_from_same_engine(other.engine()));

        // Avoid matching on structure for subtyping checks when we have
        // precisely the same type.
        if self.type_index() == other.type_index() {
            return true;
        }

        Self::matches_impl(
            self.params(),
            other.params(),
            self.results(),
            other.results(),
        )
    }

    fn matches_impl(
        a_params: impl ExactSizeIterator<Item = ValType>,
        b_params: impl ExactSizeIterator<Item = ValType>,
        a_results: impl ExactSizeIterator<Item = ValType>,
        b_results: impl ExactSizeIterator<Item = ValType>,
    ) -> bool {
        a_params.len() == b_params.len()
            && a_results.len() == b_results.len()
            // Params are contravariant and results are covariant. For more
            // details and a refresher on variance, read
            // https://github.com/bytecodealliance/wasm-tools/blob/f1d89a4/crates/wasmparser/src/readers/core/types/matches.rs#L137-L174
            && a_params
                .zip(b_params)
                .all(|(a, b)| b.matches(&a))
            && a_results
                .zip(b_results)
                .all(|(a, b)| a.matches(&b))
    }

    /// Is function type `a` precisely equal to function type `b`?
    ///
    /// Returns `false` even if `a` is a subtype of `b` or vice versa, if they
    /// are not exactly the same function type.
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn eq(a: &FuncType, b: &FuncType) -> bool {
        assert!(a.comes_from_same_engine(b.engine()));
        a.type_index() == b.type_index()
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        Engine::same(self.registered_type.engine(), engine)
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.registered_type.index()
    }

    pub(crate) fn into_registered_type(self) -> RegisteredType {
        self.registered_type
    }

    /// Construct a `FuncType` from a `WasmFuncType`.
    ///
    /// This method should only be used when something has already registered --
    /// and is *keeping registered* -- any other concrete Wasm types referenced
    /// by the given `WasmFuncType`.
    ///
    /// For example, this method may be called to convert a function type from
    /// within a Wasm module's `ModuleTypes` since the Wasm module itself is
    /// holding a strong reference to all of its types, including any `(ref null
    /// <index>)` types used in the function's parameters and results.
    pub(crate) fn from_wasm_func_type(
        engine: &Engine,
        is_final: bool,
        supertype: Option<EngineOrModuleTypeIndex>,
        ty: WasmFuncType,
    ) -> FuncType {
        let ty = RegisteredType::new(
            engine,
            WasmSubType {
                is_final,
                supertype,
                composite_type: WasmCompositeType {
                    shared: false,
                    inner: WasmCompositeInnerType::Func(ty),
                },
            },
        );
        Self {
            registered_type: ty,
        }
    }

    pub(crate) fn from_shared_type_index(engine: &Engine, index: VMSharedTypeIndex) -> FuncType {
        let ty = RegisteredType::root(engine, index);
        Self::from_registered_type(ty)
    }

    pub(crate) fn from_registered_type(registered_type: RegisteredType) -> Self {
        debug_assert!(registered_type.is_func());
        Self { registered_type }
    }
    /// Construct a func which returns results of default value, if each result type has a default value.
    pub fn default_value(&self, mut store: impl AsContextMut) -> Result<Func> {
        let dummy_results = self
            .results()
            .map(|ty| ty.default_value())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("function results do not have a default value"))?;
        Ok(Func::new(&mut store, self.clone(), move |_, _, results| {
            for (slot, dummy) in results.iter_mut().zip(dummy_results.iter()) {
                *slot = *dummy;
            }
            Ok(())
        }))
    }
}

// Continuation types
/// A WebAssembly continuation descriptor.
#[derive(Debug, Clone, Hash)]
pub struct ContType {
    registered_type: RegisteredType,
}

impl ContType {
    /// Get the engine that this function type is associated with.
    pub fn engine(&self) -> &Engine {
        self.registered_type.engine()
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        Engine::same(self.registered_type.engine(), engine)
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.registered_type.index()
    }

    /// Does this continuation type match the other continuation type?
    ///
    /// That is, is this continuation type a subtype of the other continuation type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &ContType) -> bool {
        assert!(self.comes_from_same_engine(other.engine()));

        // Avoid matching on structure for subtyping checks when we have
        // precisely the same type.
        // TODO(dhil): Implement subtype check later.
        self.type_index() == other.type_index()
    }

    pub(crate) fn from_shared_type_index(engine: &Engine, index: VMSharedTypeIndex) -> ContType {
        let ty = RegisteredType::root(engine, index);
        assert!(ty.is_cont());
        Self {
            registered_type: ty,
        }
    }
}

// Exception types

/// A WebAssembly exception-object signature type.
///
/// This type captures the *signature* of an exception object. Note
/// that the WebAssembly standard does not define concrete types in
/// the heap-type lattice between `exn` (any exception object -- the
/// top type) and `noexn` (the uninhabited bottom type). Wasmtime
/// defines concrete types based on the *signature* -- that is, the
/// function type that describes the signature of the exception
/// payload values -- rather than the tag. The tag is a per-instance
/// nominal entity (similar to a memory or a table) and is associated
/// only with particular exception *objects*.
#[derive(Debug, Clone, Hash)]
pub struct ExnType {
    func_ty: FuncType,
    registered_type: RegisteredType,
}

impl fmt::Display for ExnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(exn {}", self.func_ty)?;
        for field in self.fields() {
            write!(f, " (field {field})")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl ExnType {
    /// Create a new `ExnType`.
    ///
    /// This function creates a new exception object type with the
    /// given signature, i.e., list of payload value types. This
    /// signature implies a tag type, and when instantiated at
    /// runtime, it must be associated with a tag of that type.
    pub fn new(engine: &Engine, fields: impl IntoIterator<Item = ValType>) -> Result<ExnType> {
        let fields = fields.into_iter().collect::<Vec<_>>();

        // First, construct/intern a FuncType: we need this to exist
        // so we can hand out a TagType, and it also roots any nested registrations.
        let func_ty = FuncType::new(engine, fields.clone(), []);

        Ok(Self::_new(engine, fields, func_ty))
    }

    /// Create a new `ExnType` from an existing `TagType`.
    ///
    /// This function creates a new exception object type with the
    /// signature represented by the tag. The signature must have no
    /// result values, i.e., must be of the form `(T1, T2, ...) ->
    /// ()`.
    pub fn from_tag_type(tag: &TagType) -> Result<ExnType> {
        let func_ty = tag.ty();

        // Check that the tag's signature type has no results.
        ensure!(
            func_ty.results().len() == 0,
            "Cannot create an exception type from a tag type with results in the signature"
        );

        Ok(Self::_new(
            tag.ty.engine(),
            func_ty.params(),
            func_ty.clone(),
        ))
    }

    fn _new(
        engine: &Engine,
        fields: impl IntoIterator<Item = ValType>,
        func_ty: FuncType,
    ) -> ExnType {
        let fields = fields
            .into_iter()
            .map(|ty| {
                assert!(ty.comes_from_same_engine(engine));
                WasmFieldType {
                    element_type: WasmStorageType::Val(ty.to_wasm_type()),
                    mutable: false,
                }
            })
            .collect();

        let ty = RegisteredType::new(
            engine,
            WasmSubType {
                is_final: true,
                supertype: None,
                composite_type: WasmCompositeType {
                    shared: false,
                    inner: WasmCompositeInnerType::Exn(WasmExnType {
                        func_ty: EngineOrModuleTypeIndex::Engine(func_ty.type_index()),
                        fields,
                    }),
                },
            },
        );

        Self {
            func_ty,
            registered_type: ty,
        }
    }

    /// Get the tag type that this exception type is associated with.
    pub fn tag_type(&self) -> TagType {
        TagType {
            ty: self.func_ty.clone(),
        }
    }

    /// Get the `i`th field type.
    ///
    /// Returns `None` if `i` is out of bounds.
    pub fn field(&self, i: usize) -> Option<FieldType> {
        let engine = self.engine();
        self.as_wasm_exn_type()
            .fields
            .get(i)
            .map(|ty| FieldType::from_wasm_field_type(engine, ty))
    }

    /// Returns the list of field types for this function.
    #[inline]
    pub fn fields(&self) -> impl ExactSizeIterator<Item = FieldType> + '_ {
        let engine = self.engine();
        self.as_wasm_exn_type()
            .fields
            .iter()
            .map(|ty| FieldType::from_wasm_field_type(engine, ty))
    }

    /// Get the engine that this exception type is associated with.
    pub fn engine(&self) -> &Engine {
        self.registered_type.engine()
    }

    pub(crate) fn comes_from_same_engine(&self, engine: &Engine) -> bool {
        Engine::same(self.registered_type.engine(), engine)
    }

    pub(crate) fn as_wasm_exn_type(&self) -> &WasmExnType {
        self.registered_type().unwrap_exn()
    }

    pub(crate) fn type_index(&self) -> VMSharedTypeIndex {
        self.registered_type.index()
    }

    /// Does this exception type match the other exception type?
    ///
    /// That is, is this exception type a subtype of the other exception type?
    ///
    /// # Panics
    ///
    /// Panics if either type is associated with a different engine from the
    /// other.
    pub fn matches(&self, other: &ExnType) -> bool {
        assert!(self.comes_from_same_engine(other.engine()));

        // We have no concrete-exception-type subtyping; concrete
        // exception types are only (mutually, trivially) subtypes if
        // they are exactly equal.
        self.type_index() == other.type_index()
    }

    pub(crate) fn registered_type(&self) -> &RegisteredType {
        &self.registered_type
    }

    pub(crate) fn from_shared_type_index(engine: &Engine, index: VMSharedTypeIndex) -> ExnType {
        let ty = RegisteredType::root(engine, index);
        assert!(ty.is_exn());
        let func_ty = FuncType::from_shared_type_index(
            engine,
            ty.unwrap_exn().func_ty.unwrap_engine_type_index(),
        );
        Self {
            func_ty,
            registered_type: ty,
        }
    }
}

// Global Types

/// A WebAssembly global descriptor.
///
/// This type describes an instance of a global in a WebAssembly module. Globals
/// are local to an [`Instance`](crate::Instance) and are either immutable or
/// mutable.
#[derive(Debug, Clone, Hash)]
pub struct GlobalType {
    content: ValType,
    mutability: Mutability,
}

impl GlobalType {
    /// Creates a new global descriptor of the specified `content` type and
    /// whether or not it's mutable.
    pub fn new(content: ValType, mutability: Mutability) -> GlobalType {
        GlobalType {
            content,
            mutability,
        }
    }

    /// Returns the value type of this global descriptor.
    pub fn content(&self) -> &ValType {
        &self.content
    }

    /// Returns whether or not this global is mutable.
    pub fn mutability(&self) -> Mutability {
        self.mutability
    }

    /// Returns `None` if the wasmtime global has a type that we can't
    /// represent, but that should only very rarely happen and indicate a bug.
    pub(crate) fn from_wasmtime_global(engine: &Engine, global: &Global) -> GlobalType {
        let ty = ValType::from_wasm_type(engine, &global.wasm_ty);
        let mutability = if global.mutability {
            Mutability::Var
        } else {
            Mutability::Const
        };
        GlobalType::new(ty, mutability)
    }
    /// Construct a new global import with this type’s default value.
    ///
    /// This creates a host `Global` in the given store initialized to the
    /// type’s zero/null default (e.g. `0` for numeric globals, `null_ref` for refs).
    pub fn default_value(&self, store: impl AsContextMut) -> Result<RuntimeGlobal> {
        let val = self
            .content()
            .default_value()
            .ok_or_else(|| anyhow!("global type has no default value"))?;
        RuntimeGlobal::new(store, self.clone(), val)
    }

    pub(crate) fn into_registered_type(self) -> Option<RegisteredType> {
        self.content.into_registered_type()
    }
}

// Tag Types

/// A descriptor for a tag in a WebAssembly module.
///
/// Note that tags are local to an [`Instance`](crate::Instance),
/// i.e., are a runtime entity. However, a tag is associated with a
/// function type, and so has a kind of static type. This descriptor
/// is a thin wrapper around a `FuncType` representing the function
/// type of a tag.
#[derive(Debug, Clone, Hash)]
pub struct TagType {
    ty: FuncType,
}

impl TagType {
    /// Creates a new global descriptor of the specified type.
    pub fn new(ty: FuncType) -> TagType {
        TagType { ty }
    }

    /// Returns the underlying function type of this tag descriptor.
    pub fn ty(&self) -> &FuncType {
        &self.ty
    }

    pub(crate) fn from_wasmtime_tag(engine: &Engine, tag: &Tag) -> TagType {
        let ty = FuncType::from_shared_type_index(engine, tag.signature.unwrap_engine_type_index());
        TagType { ty }
    }
}

// Table Types

/// A descriptor for a table in a WebAssembly module.
///
/// Tables are contiguous chunks of a specific element, typically a `funcref` or
/// an `externref`. The most common use for tables is a function table through
/// which `call_indirect` can invoke other functions.
#[derive(Debug, Clone, Hash)]
pub struct TableType {
    // Keep a `wasmtime::RefType` so that `TableType::element` doesn't need to
    // take an `&Engine`.
    element: RefType,
    ty: Table,
}

impl TableType {
    /// Creates a new table descriptor which will contain the specified
    /// `element` and have the `limits` applied to its length.
    pub fn new(element: RefType, min: u32, max: Option<u32>) -> TableType {
        let ref_type = element.to_wasm_type();

        debug_assert!(
            ref_type.is_canonicalized_for_runtime_usage(),
            "should be canonicalized for runtime usage: {ref_type:?}"
        );

        let limits = Limits {
            min: u64::from(min),
            max: max.map(|x| u64::from(x)),
        };

        TableType {
            element,
            ty: Table {
                idx_type: IndexType::I32,
                limits,
                ref_type,
            },
        }
    }

    /// Crates a new descriptor for a 64-bit table.
    ///
    /// Note that 64-bit tables are part of the memory64 proposal for
    /// WebAssembly which is not standardized yet.
    pub fn new64(element: RefType, min: u64, max: Option<u64>) -> TableType {
        let ref_type = element.to_wasm_type();

        debug_assert!(
            ref_type.is_canonicalized_for_runtime_usage(),
            "should be canonicalized for runtime usage: {ref_type:?}"
        );

        TableType {
            element,
            ty: Table {
                ref_type,
                idx_type: IndexType::I64,
                limits: Limits { min, max },
            },
        }
    }

    /// Returns whether or not this table is a 64-bit table.
    ///
    /// Note that 64-bit tables are part of the memory64 proposal for
    /// WebAssembly which is not standardized yet.
    pub fn is_64(&self) -> bool {
        matches!(self.ty.idx_type, IndexType::I64)
    }

    /// Returns the element value type of this table.
    pub fn element(&self) -> &RefType {
        &self.element
    }

    /// Returns minimum number of elements this table must have
    pub fn minimum(&self) -> u64 {
        self.ty.limits.min
    }

    /// Returns the optionally-specified maximum number of elements this table
    /// can have.
    ///
    /// If this returns `None` then the table is not limited in size.
    pub fn maximum(&self) -> Option<u64> {
        self.ty.limits.max
    }

    pub(crate) fn from_wasmtime_table(engine: &Engine, table: &Table) -> TableType {
        let element = RefType::from_wasm_type(engine, &table.ref_type);
        TableType {
            element,
            ty: *table,
        }
    }

    pub(crate) fn wasmtime_table(&self) -> &Table {
        &self.ty
    }
    /// Construct a new table import whose entries are filled with this type’s default.
    ///
    /// Creates a host `Table` in the store with its initial size and element
    /// type’s default (e.g. `null_ref` for nullable refs).
    pub fn default_value(&self, store: impl AsContextMut) -> Result<RuntimeTable> {
        let val: ValType = self.element().clone().into();
        let init_val = val
            .default_value()
            .context("table element type does not have a default value")?
            .ref_()
            .unwrap();
        RuntimeTable::new(store, self.clone(), init_val)
    }
}

// Memory Types

/// A builder for [`MemoryType`][crate::MemoryType]s.
///
/// A new builder can be constructed via its `Default` implementation.
///
/// When you're done configuring, get the underlying
/// [`MemoryType`][crate::MemoryType] by calling the
/// [`build`][crate::MemoryTypeBuilder::build] method.
///
/// # Example
///
/// ```
/// # fn foo() -> wasmtime::Result<()> {
/// use wasmtime::MemoryTypeBuilder;
///
/// let memory_type = MemoryTypeBuilder::new()
///     // Set the minimum size, in pages.
///     .min(4096)
///     // Set the maximum size, in pages.
///     .max(Some(4096))
///     // Set the page size to 1 byte (aka 2**0).
///     .page_size_log2(0)
///     // Get the underlying memory type.
///     .build()?;
/// #   Ok(())
/// # }
/// ```
pub struct MemoryTypeBuilder {
    ty: Memory,
}

impl Default for MemoryTypeBuilder {
    fn default() -> Self {
        MemoryTypeBuilder {
            ty: Memory {
                idx_type: IndexType::I32,
                limits: Limits { min: 0, max: None },
                shared: false,
                page_size_log2: Memory::DEFAULT_PAGE_SIZE_LOG2,
            },
        }
    }
}

impl MemoryTypeBuilder {
    /// Create a new builder for a [`MemoryType`] with the default settings.
    ///
    /// By default memory types have the following properties:
    ///
    /// * The minimum memory size is 0 pages.
    /// * The maximum memory size is unspecified.
    /// * Memories use 32-bit indexes.
    /// * The page size is 64KiB.
    ///
    /// Each option can be configured through the methods on the returned
    /// builder.
    pub fn new() -> MemoryTypeBuilder {
        MemoryTypeBuilder::default()
    }

    fn validate(&self) -> Result<()> {
        if self
            .ty
            .limits
            .max
            .map_or(false, |max| max < self.ty.limits.min)
        {
            bail!("maximum page size cannot be smaller than the minimum page size");
        }

        match self.ty.page_size_log2 {
            0 | Memory::DEFAULT_PAGE_SIZE_LOG2 => {}
            x => bail!(
                "page size must be 2**16 or 2**0, but was given 2**{x}; note \
                 that future Wasm extensions might allow any power of two page \
                 size, but only 2**16 and 2**0 are currently valid",
            ),
        }

        if self.ty.shared && self.ty.limits.max.is_none() {
            bail!("shared memories must have a maximum size");
        }

        let absolute_max = self.ty.max_size_based_on_index_type();
        let min = self
            .ty
            .minimum_byte_size()
            .context("memory's minimum byte size must fit in a u64")?;
        if min > absolute_max {
            bail!("minimum size is too large for this memory type's index type");
        }
        if self
            .ty
            .maximum_byte_size()
            .map_or(false, |max| max > absolute_max)
        {
            bail!("maximum size is too large for this memory type's index type");
        }

        Ok(())
    }

    /// Set the minimum size, in units of pages, for the memory type being
    /// built.
    ///
    /// The default minimum is `0`.
    pub fn min(&mut self, minimum: u64) -> &mut Self {
        self.ty.limits.min = minimum;
        self
    }

    /// Set the maximum size, in units of pages, for the memory type being
    /// built.
    ///
    /// The default maximum is `None`.
    pub fn max(&mut self, maximum: Option<u64>) -> &mut Self {
        self.ty.limits.max = maximum;
        self
    }

    /// Set whether this is a 64-bit memory or not.
    ///
    /// If a memory is not a 64-bit memory, then it is a 32-bit memory.
    ///
    /// The default is `false`, aka 32-bit memories.
    ///
    /// Note that 64-bit memories are part of [the memory64
    /// proposal](https://github.com/WebAssembly/memory64) for WebAssembly which
    /// is not fully standardized yet.
    pub fn memory64(&mut self, memory64: bool) -> &mut Self {
        self.ty.idx_type = match memory64 {
            true => IndexType::I64,
            false => IndexType::I32,
        };
        self
    }

    /// Set the sharedness for the memory type being built.
    ///
    /// The default is `false`, aka unshared.
    ///
    /// Note that shared memories are part of [the threads
    /// proposal](https://github.com/WebAssembly/threads) for WebAssembly which
    /// is not fully standardized yet.
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        self.ty.shared = shared;
        self
    }

    /// Set the log base 2 of the page size, in bytes, for the memory type being
    /// built.
    ///
    /// The default value is `16`, which results in the default Wasm page size
    /// of 64KiB (aka 2<sup>16</sup> or 65536).
    ///
    /// Other than `16`, the only valid value is `0`, which results in a page
    /// size of one byte (aka 2<sup>0</sup>). Single-byte page sizes can be used
    /// to get fine-grained control over a Wasm memory's resource consumption
    /// and run Wasm in embedded environments with less than 64KiB of RAM, for
    /// example.
    ///
    /// Future extensions to the core WebAssembly language might relax these
    /// constraints and introduce more valid page sizes, such as any power of
    /// two between 1 and 65536 inclusive.
    ///
    /// Note that non-default page sizes are part of [the custom-page-sizes
    /// proposal](https://github.com/WebAssembly/custom-page-sizes) for
    /// WebAssembly which is not fully standardized yet.
    pub fn page_size_log2(&mut self, page_size_log2: u8) -> &mut Self {
        self.ty.page_size_log2 = page_size_log2;
        self
    }

    /// Get the underlying memory type that this builder has been building.
    ///
    /// # Errors
    ///
    /// Returns an error if the configured memory type is invalid, for example
    /// if the maximum size is smaller than the minimum size.
    pub fn build(&self) -> Result<MemoryType> {
        self.validate()?;
        Ok(MemoryType { ty: self.ty })
    }
}

/// A descriptor for a WebAssembly memory type.
///
/// Memories are described in units of pages (64KB) and represent contiguous
/// chunks of addressable memory.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MemoryType {
    ty: Memory,
}

impl MemoryType {
    /// Creates a new descriptor for a 32-bit WebAssembly memory given the
    /// specified limits of the memory.
    ///
    /// The `minimum` and `maximum` values here are specified in units of
    /// WebAssembly pages, which are 64KiB by default. Use
    /// [`MemoryTypeBuilder`][crate::MemoryTypeBuilder] if you want a
    /// non-default page size.
    ///
    /// # Panics
    ///
    /// Panics if the minimum is greater than the maximum or if the minimum or
    /// maximum number of pages can result in a byte size that is not
    /// addressable with a 32-bit integer.
    pub fn new(minimum: u32, maximum: Option<u32>) -> MemoryType {
        MemoryTypeBuilder::default()
            .min(minimum.into())
            .max(maximum.map(Into::into))
            .build()
            .unwrap()
    }

    /// Creates a new descriptor for a 64-bit WebAssembly memory given the
    /// specified limits of the memory.
    ///
    /// The `minimum` and `maximum` values here are specified in units of
    /// WebAssembly pages, which are 64KiB by default. Use
    /// [`MemoryTypeBuilder`][crate::MemoryTypeBuilder] if you want a
    /// non-default page size.
    ///
    /// Note that 64-bit memories are part of [the memory64
    /// proposal](https://github.com/WebAssembly/memory64) for WebAssembly which
    /// is not fully standardized yet.
    ///
    /// # Panics
    ///
    /// Panics if the minimum is greater than the maximum or if the minimum or
    /// maximum number of pages can result in a byte size that is not
    /// addressable with a 64-bit integer.
    pub fn new64(minimum: u64, maximum: Option<u64>) -> MemoryType {
        MemoryTypeBuilder::default()
            .memory64(true)
            .min(minimum)
            .max(maximum)
            .build()
            .unwrap()
    }

    /// Creates a new descriptor for shared WebAssembly memory given the
    /// specified limits of the memory.
    ///
    /// The `minimum` and `maximum` values here are specified in units of
    /// WebAssembly pages, which are 64KiB by default. Use
    /// [`MemoryTypeBuilder`][crate::MemoryTypeBuilder] if you want a
    /// non-default page size.
    ///
    /// Note that shared memories are part of [the threads
    /// proposal](https://github.com/WebAssembly/threads) for WebAssembly which
    /// is not fully standardized yet.
    ///
    /// # Panics
    ///
    /// Panics if the minimum is greater than the maximum or if the minimum or
    /// maximum number of pages can result in a byte size that is not
    /// addressable with a 32-bit integer.
    pub fn shared(minimum: u32, maximum: u32) -> MemoryType {
        MemoryTypeBuilder::default()
            .shared(true)
            .min(minimum.into())
            .max(Some(maximum.into()))
            .build()
            .unwrap()
    }

    /// Creates a new [`MemoryTypeBuilder`] to configure all the various knobs
    /// of the final memory type being created.
    ///
    /// This is a convenience function for [`MemoryTypeBuilder::new`].
    pub fn builder() -> MemoryTypeBuilder {
        MemoryTypeBuilder::new()
    }

    /// Returns whether this is a 64-bit memory or not.
    ///
    /// Note that 64-bit memories are part of the memory64 proposal for
    /// WebAssembly which is not standardized yet.
    pub fn is_64(&self) -> bool {
        matches!(self.ty.idx_type, IndexType::I64)
    }

    /// Returns whether this is a shared memory or not.
    ///
    /// Note that shared memories are part of the threads proposal for
    /// WebAssembly which is not standardized yet.
    pub fn is_shared(&self) -> bool {
        self.ty.shared
    }

    /// Returns minimum number of WebAssembly pages this memory must have.
    ///
    /// Note that the return value, while a `u64`, will always fit into a `u32`
    /// for 32-bit memories.
    pub fn minimum(&self) -> u64 {
        self.ty.limits.min
    }

    /// Returns the optionally-specified maximum number of pages this memory
    /// can have.
    ///
    /// If this returns `None` then the memory is not limited in size.
    ///
    /// Note that the return value, while a `u64`, will always fit into a `u32`
    /// for 32-bit memories.
    pub fn maximum(&self) -> Option<u64> {
        self.ty.limits.max
    }

    /// This memory's page size, in bytes.
    pub fn page_size(&self) -> u64 {
        self.ty.page_size()
    }

    /// The log2 of this memory's page size, in bytes.
    pub fn page_size_log2(&self) -> u8 {
        self.ty.page_size_log2
    }

    pub(crate) fn from_wasmtime_memory(memory: &Memory) -> MemoryType {
        MemoryType { ty: *memory }
    }

    pub(crate) fn wasmtime_memory(&self) -> &Memory {
        &self.ty
    }
    /// Construct a new memory import initialized to this memory type’s default state
    ///
    /// Returns a host `Memory` in the given store with the configured initial
    /// page size and zeroed contents.
    pub fn default_value(&self, store: impl AsContextMut) -> Result<RuntimeMemory> {
        RuntimeMemory::new(store, self.clone())
    }
}

// Import Types

/// A descriptor for an imported value into a wasm module.
///
/// This type is primarily accessed from the
/// [`Module::imports`](crate::Module::imports) API. Each [`ImportType`]
/// describes an import into the wasm module with the module/name that it's
/// imported from as well as the type of item that's being imported.
#[derive(Clone)]
pub struct ImportType<'module> {
    /// The module of the import.
    module: &'module str,

    /// The field of the import.
    name: &'module str,

    /// The type of the import.
    ty: EntityType,
    types: &'module ModuleTypes,
    engine: &'module Engine,
}

impl<'module> ImportType<'module> {
    /// Creates a new import descriptor which comes from `module` and `name` and
    /// is of type `ty`.
    pub(crate) fn new(
        module: &'module str,
        name: &'module str,
        ty: EntityType,
        types: &'module ModuleTypes,
        engine: &'module Engine,
    ) -> ImportType<'module> {
        assert!(ty.is_canonicalized_for_runtime_usage());
        ImportType {
            module,
            name,
            ty,
            types,
            engine,
        }
    }

    /// Returns the module name that this import is expected to come from.
    pub fn module(&self) -> &'module str {
        self.module
    }

    /// Returns the field name of the module that this import is expected to
    /// come from.
    pub fn name(&self) -> &'module str {
        self.name
    }

    /// Returns the expected type of this import.
    pub fn ty(&self) -> ExternType {
        ExternType::from_wasmtime(self.engine, self.types, &self.ty)
    }
}

impl<'module> fmt::Debug for ImportType<'module> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportType")
            .field("module", &self.module())
            .field("name", &self.name())
            .field("ty", &self.ty())
            .finish()
    }
}

// Export Types

/// A descriptor for an exported WebAssembly value.
///
/// This type is primarily accessed from the
/// [`Module::exports`](crate::Module::exports) accessor and describes what
/// names are exported from a wasm module and the type of the item that is
/// exported.
#[derive(Clone)]
pub struct ExportType<'module> {
    /// The name of the export.
    name: &'module str,

    /// The type of the export.
    ty: EntityType,
    types: &'module ModuleTypes,
    engine: &'module Engine,
}

impl<'module> ExportType<'module> {
    /// Creates a new export which is exported with the given `name` and has the
    /// given `ty`.
    pub(crate) fn new(
        name: &'module str,
        ty: EntityType,
        types: &'module ModuleTypes,
        engine: &'module Engine,
    ) -> ExportType<'module> {
        ExportType {
            name,
            ty,
            types,
            engine,
        }
    }

    /// Returns the name by which this export is known.
    pub fn name(&self) -> &'module str {
        self.name
    }

    /// Returns the type of this export.
    pub fn ty(&self) -> ExternType {
        ExternType::from_wasmtime(self.engine, self.types, &self.ty)
    }
}

impl<'module> fmt::Debug for ExportType<'module> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExportType")
            .field("name", &self.name().to_owned())
            .field("ty", &self.ty())
            .finish()
    }
}
