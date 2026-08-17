use crate::{Encode, Section, SectionId, encode_section};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Represents a subtype of possible other types in a WebAssembly module.
#[derive(Debug, Clone)]
pub struct SubType {
    /// Is the subtype final.
    pub is_final: bool,
    /// The list of supertype indexes. As of GC MVP, there can be at most one
    /// supertype.
    pub supertype_idx: Option<u32>,
    /// The composite type of the subtype.
    pub composite_type: CompositeType,
}

/// Represents a composite type in a WebAssembly module.
#[derive(Debug, Clone)]
pub struct CompositeType {
    /// The type defined inside the composite type.
    pub inner: CompositeInnerType,
    /// Whether the type is shared. This is part of the
    /// shared-everything-threads proposal.
    pub shared: bool,
    /// Optional descriptor attribute.
    pub descriptor: Option<u32>,
    /// Optional describes attribute.
    pub describes: Option<u32>,
}

/// A [`CompositeType`] can contain one of these types.
#[derive(Debug, Clone)]
pub enum CompositeInnerType {
    /// The type is for a function.
    Func(FuncType),
    /// The type is for an array.
    Array(ArrayType),
    /// The type is for a struct.
    Struct(StructType),
    /// The type is for a continuation.
    Cont(ContType),
}

/// Represents a type of a function in a WebAssembly module.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FuncType {
    /// The combined parameters and result types.
    params_results: Box<[ValType]>,
    /// The number of parameter types.
    len_params: usize,
}

/// Represents a type of an array in a WebAssembly module.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ArrayType(pub FieldType);

/// Represents a type of a struct in a WebAssembly module.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StructType {
    /// Struct fields.
    pub fields: Box<[FieldType]>,
}

/// Field type in composite types (structs, arrays).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FieldType {
    /// Storage type of the field.
    pub element_type: StorageType,
    /// Is the field mutable.
    pub mutable: bool,
}

/// Storage type for composite type fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum StorageType {
    /// The `i8` type.
    I8,
    /// The `i16` type.
    I16,
    /// A value type.
    Val(ValType),
}

impl StorageType {
    /// Is this storage type defaultable?
    pub fn is_defaultable(&self) -> bool {
        self.unpack().is_defaultable()
    }

    /// Unpack this storage type into a value type.
    pub fn unpack(&self) -> ValType {
        match self {
            StorageType::I8 | StorageType::I16 => ValType::I32,
            StorageType::Val(v) => *v,
        }
    }
}

/// Represents a type of a continuation in a WebAssembly module.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ContType(pub u32);

/// The type of a core WebAssembly value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ValType {
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `v128` type.
    ///
    /// Part of the SIMD proposal.
    V128,
    /// A reference type.
    ///
    /// The `funcref` and `externref` type fall into this category and the full
    /// generalization here is due to the implementation of the
    /// function-references proposal.
    Ref(RefType),
}

impl ValType {
    /// Is this a numeric value type?
    pub fn is_numeric(&self) -> bool {
        match self {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 => true,
            ValType::V128 | ValType::Ref(_) => false,
        }
    }

    /// Is this a vector type?
    pub fn is_vector(&self) -> bool {
        match self {
            ValType::V128 => true,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::Ref(_) => false,
        }
    }

    /// Is this a reference type?
    pub fn is_reference(&self) -> bool {
        match self {
            ValType::Ref(_) => true,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => false,
        }
    }
}

impl FuncType {
    /// Creates a new [`FuncType`] from the given `params` and `results`.
    pub fn new<P, R>(params: P, results: R) -> Self
    where
        P: IntoIterator<Item = ValType>,
        R: IntoIterator<Item = ValType>,
    {
        let mut buffer = params.into_iter().collect::<Vec<_>>();
        let len_params = buffer.len();
        buffer.extend(results);
        Self::from_parts(buffer.into(), len_params)
    }

    #[inline]
    pub(crate) fn from_parts(params_results: Box<[ValType]>, len_params: usize) -> Self {
        Self {
            params_results,
            len_params,
        }
    }

    /// Returns a shared slice to the parameter types of the [`FuncType`].
    #[inline]
    pub fn params(&self) -> &[ValType] {
        &self.params_results[..self.len_params]
    }

    /// Returns a shared slice to the result types of the [`FuncType`].
    #[inline]
    pub fn results(&self) -> &[ValType] {
        &self.params_results[self.len_params..]
    }
}

impl ValType {
    /// Alias for the `funcref` type in WebAssembly
    pub const FUNCREF: ValType = ValType::Ref(RefType::FUNCREF);
    /// Alias for the `externref` type in WebAssembly
    pub const EXTERNREF: ValType = ValType::Ref(RefType::EXTERNREF);
    /// Alias for the `exnref` type in WebAssembly
    pub const EXNREF: ValType = ValType::Ref(RefType::EXNREF);

    /// Is this value defaultable?
    pub fn is_defaultable(&self) -> bool {
        match self {
            ValType::Ref(r) => r.nullable,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => true,
        }
    }
}

impl Encode for StorageType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            StorageType::I8 => sink.push(0x78),
            StorageType::I16 => sink.push(0x77),
            StorageType::Val(vt) => vt.encode(sink),
        }
    }
}

impl Encode for ValType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            ValType::I32 => sink.push(0x7F),
            ValType::I64 => sink.push(0x7E),
            ValType::F32 => sink.push(0x7D),
            ValType::F64 => sink.push(0x7C),
            ValType::V128 => sink.push(0x7B),
            ValType::Ref(rt) => rt.encode(sink),
        }
    }
}

/// A reference type.
///
/// This is largely part of the function references proposal for WebAssembly but
/// additionally is used by the `funcref` and `externref` types. The full
/// generality of this type is only exercised with function-references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(missing_docs)]
pub struct RefType {
    pub nullable: bool,
    pub heap_type: HeapType,
}

impl RefType {
    /// Alias for the `anyref` type in WebAssembly.
    pub const ANYREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        },
    };

    /// Alias for the `anyref` type in WebAssembly.
    pub const EQREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Eq,
        },
    };

    /// Alias for the `funcref` type in WebAssembly.
    pub const FUNCREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Func,
        },
    };

    /// Alias for the `externref` type in WebAssembly.
    pub const EXTERNREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Extern,
        },
    };

    /// Alias for the `i31ref` type in WebAssembly.
    pub const I31REF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::I31,
        },
    };

    /// Alias for the `arrayref` type in WebAssembly.
    pub const ARRAYREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Array,
        },
    };

    /// Alias for the `exnref` type in WebAssembly.
    pub const EXNREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Exn,
        },
    };

    /// Create a new abstract reference type.
    pub fn new_abstract(ty: AbstractHeapType, nullable: bool, shared: bool) -> Self {
        Self {
            nullable,
            heap_type: HeapType::Abstract { shared, ty },
        }
    }

    /// Set the nullability of this reference type.
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
}

impl Encode for RefType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            // Binary abbreviations (i.e., short form), for when the ref is
            // nullable.
            RefType {
                nullable: true,
                heap_type: heap @ HeapType::Abstract { .. },
            } => {
                heap.encode(sink);
            }

            // Generic 'ref null <heaptype>' encoding (i.e., long form).
            RefType {
                nullable: true,
                heap_type,
            } => {
                sink.push(0x63);
                heap_type.encode(sink);
            }

            // Generic 'ref <heaptype>' encoding.
            RefType {
                nullable: false,
                heap_type,
            } => {
                sink.push(0x64);
                heap_type.encode(sink);
            }
        }
    }
}

impl From<RefType> for ValType {
    fn from(ty: RefType) -> ValType {
        ValType::Ref(ty)
    }
}

/// Part of the function references proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HeapType {
    /// An abstract heap type; e.g., `anyref`.
    Abstract {
        /// Whether the type is shared.
        shared: bool,
        /// The actual heap type.
        ty: AbstractHeapType,
    },

    /// A concrete Wasm-defined type at the given index.
    Concrete(u32),
    /// An exact type.
    Exact(u32),
}

impl HeapType {
    /// Alias for the unshared `any` heap type.
    pub const ANY: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::Any,
    };

    /// Alias for the unshared `func` heap type.
    pub const FUNC: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::Func,
    };

    /// Alias for the unshared `extern` heap type.
    pub const EXTERN: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::Extern,
    };

    /// Alias for the unshared `i31` heap type.
    pub const I31: Self = Self::Abstract {
        shared: false,
        ty: AbstractHeapType::I31,
    };
}

impl Encode for HeapType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            HeapType::Abstract { shared, ty } => {
                if *shared {
                    sink.push(0x65);
                }
                ty.encode(sink);
            }
            // Note that this is encoded as a signed type rather than unsigned
            // as it's decoded as an s33
            HeapType::Concrete(i) => i64::from(*i).encode(sink),
            // Exact type is u32
            HeapType::Exact(i) => {
                sink.push(0x62);
                u32::from(*i).encode(sink)
            }
        }
    }
}

/// An abstract heap type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AbstractHeapType {
    /// Untyped (any) function.
    Func,

    /// The abstract external heap type.
    Extern,

    /// The abstract `any` heap type.
    ///
    /// The common supertype (a.k.a. top) of all internal types.
    Any,

    /// The abstract `none` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all internal types.
    None,

    /// The abstract `noextern` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all external types.
    NoExtern,

    /// The abstract `nofunc` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all function types.
    NoFunc,

    /// The abstract `eq` heap type.
    ///
    /// The common supertype of all referenceable types on which comparison
    /// (ref.eq) is allowed.
    Eq,

    /// The abstract `struct` heap type.
    ///
    /// The common supertype of all struct types.
    Struct,

    /// The abstract `array` heap type.
    ///
    /// The common supertype of all array types.
    Array,

    /// The unboxed `i31` heap type.
    I31,

    /// The abstract `exception` heap type.
    Exn,

    /// The abstract `noexn` heap type.
    NoExn,

    /// The abstract `cont` heap type.
    Cont,

    /// The abstract `nocont` heap type.
    NoCont,
}

impl Encode for AbstractHeapType {
    fn encode(&self, sink: &mut Vec<u8>) {
        use AbstractHeapType::*;
        match self {
            Func => sink.push(0x70),
            Extern => sink.push(0x6F),
            Any => sink.push(0x6E),
            None => sink.push(0x71),
            NoExtern => sink.push(0x72),
            NoFunc => sink.push(0x73),
            Eq => sink.push(0x6D),
            Struct => sink.push(0x6B),
            Array => sink.push(0x6A),
            I31 => sink.push(0x6C),
            Exn => sink.push(0x69),
            NoExn => sink.push(0x74),
            Cont => sink.push(0x68),
            NoCont => sink.push(0x75),
        }
    }
}

/// An encoder for the type section of WebAssembly modules.
///
/// # Example
///
/// ```rust
/// use wasm_encoder::{Module, TypeSection, ValType};
///
/// let mut types = TypeSection::new();
///
/// types.ty().function([ValType::I32, ValType::I32], [ValType::I64]);
///
/// let mut module = Module::new();
/// module.section(&types);
///
/// let bytes = module.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct TypeSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl TypeSection {
    /// Create a new module type section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of types in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Encode a function type in this type section.
    #[must_use = "the encoder must be used to encode the type"]
    pub fn ty(&mut self) -> CoreTypeEncoder<'_> {
        self.num_added += 1;
        CoreTypeEncoder {
            bytes: &mut self.bytes,
            push_prefix_if_component_core_type: false,
        }
    }
}

impl Encode for TypeSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for TypeSection {
    fn id(&self) -> u8 {
        SectionId::Type.into()
    }
}

/// A single-use encoder for encoding a type; this forces all encoding for a
/// type to be done in a single shot.
#[derive(Debug)]
pub struct CoreTypeEncoder<'a> {
    pub(crate) bytes: &'a mut Vec<u8>,
    // For the time being, this flag handles an ambiguous encoding in the
    // component model: the `0x50` opcode represents both a core module type as
    // well as a GC non-final `sub` type. To avoid this, the component model
    // specification requires us to prefix a non-final `sub` type with `0x00`
    // when it is used as a top-level core type of a component. Eventually
    // (prior to the component model's v1.0 release), a module type will get a
    // new opcode and this special logic can go away.
    pub(crate) push_prefix_if_component_core_type: bool,
}
impl<'a> CoreTypeEncoder<'a> {
    /// Define a function type in this type section.
    pub fn function<P, R>(mut self, params: P, results: R)
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        self.encode_function(params, results);
    }

    /// Define a function type in this type section.
    pub fn func_type(mut self, ty: &FuncType) {
        self.encode_function(ty.params().iter().cloned(), ty.results().iter().cloned());
    }

    fn encode_function<P, R>(&mut self, params: P, results: R)
    where
        P: IntoIterator<Item = ValType>,
        P::IntoIter: ExactSizeIterator,
        R: IntoIterator<Item = ValType>,
        R::IntoIter: ExactSizeIterator,
    {
        let params = params.into_iter();
        let results = results.into_iter();

        self.bytes.push(0x60);
        params.len().encode(self.bytes);
        params.for_each(|p| p.encode(self.bytes));
        results.len().encode(self.bytes);
        results.for_each(|p| p.encode(self.bytes));
    }

    /// Define an array type in this type section.
    pub fn array(mut self, ty: &StorageType, mutable: bool) {
        self.encode_array(ty, mutable);
    }

    fn encode_array(&mut self, ty: &StorageType, mutable: bool) {
        self.bytes.push(0x5e);
        self.encode_field(ty, mutable);
    }

    fn encode_field(&mut self, ty: &StorageType, mutable: bool) {
        ty.encode(self.bytes);
        self.bytes.push(mutable as u8);
    }

    /// Define a struct type in this type section.
    pub fn struct_<F>(mut self, fields: F)
    where
        F: IntoIterator<Item = FieldType>,
        F::IntoIter: ExactSizeIterator,
    {
        self.encode_struct(fields);
    }

    fn encode_struct<F>(&mut self, fields: F)
    where
        F: IntoIterator<Item = FieldType>,
        F::IntoIter: ExactSizeIterator,
    {
        let fields = fields.into_iter();
        self.bytes.push(0x5f);
        fields.len().encode(self.bytes);
        for f in fields {
            self.encode_field(&f.element_type, f.mutable);
        }
    }

    /// Define a continuation type in this subsection
    pub fn cont(mut self, ty: &ContType) {
        self.encode_cont(ty)
    }

    fn encode_cont(&mut self, ty: &ContType) {
        self.bytes.push(0x5d);
        i64::from(ty.0).encode(self.bytes);
    }

    /// Define an explicit subtype in this type section.
    pub fn subtype(mut self, ty: &SubType) {
        self.encode_subtype(ty)
    }

    /// Define an explicit subtype in this type section.
    fn encode_subtype(&mut self, ty: &SubType) {
        // We only need to emit a prefix byte before the actual composite type
        // when either the `sub` type is not final or it has a declared super
        // type (see notes on `push_prefix_if_component_core_type`).
        if ty.supertype_idx.is_some() || !ty.is_final {
            if ty.is_final {
                self.bytes.push(0x4f);
            } else {
                if self.push_prefix_if_component_core_type {
                    self.bytes.push(0x00);
                }
                self.bytes.push(0x50);
            }
            ty.supertype_idx.encode(self.bytes);
        }
        if ty.composite_type.shared {
            self.bytes.push(0x65);
        }
        if let Some(index) = ty.composite_type.describes {
            self.bytes.push(0x4c);
            index.encode(self.bytes);
        }
        if let Some(index) = ty.composite_type.descriptor {
            self.bytes.push(0x4d);
            index.encode(self.bytes);
        }
        match &ty.composite_type.inner {
            CompositeInnerType::Func(ty) => {
                self.encode_function(ty.params().iter().copied(), ty.results().iter().copied())
            }
            CompositeInnerType::Array(ArrayType(ty)) => {
                self.encode_array(&ty.element_type, ty.mutable)
            }
            CompositeInnerType::Struct(ty) => self.encode_struct(ty.fields.iter().cloned()),
            CompositeInnerType::Cont(ty) => self.encode_cont(ty),
        }
    }

    /// Define an explicit recursion group in this type section.
    pub fn rec<T>(mut self, types: T)
    where
        T: IntoIterator<Item = SubType>,
        T::IntoIter: ExactSizeIterator,
    {
        // When emitting a `rec` group, we will never emit `sub`'s special
        // `0x00` prefix; that is only necessary when `sub` is not wrapped by
        // `rec` (see notes on `push_prefix_if_component_core_type`).
        self.push_prefix_if_component_core_type = false;
        let types = types.into_iter();
        self.bytes.push(0x4e);
        types.len().encode(self.bytes);
        types.for_each(|t| {
            self.encode_subtype(&t);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Module;
    use wasmparser::WasmFeatures;

    #[test]
    fn func_types_dont_require_wasm_gc() {
        let mut types = TypeSection::new();
        types.ty().subtype(&SubType {
            is_final: true,
            supertype_idx: None,
            composite_type: CompositeType {
                inner: CompositeInnerType::Func(FuncType::new([], [])),
                shared: false,
                descriptor: None,
                describes: None,
            },
        });

        let mut module = Module::new();
        module.section(&types);
        let wasm_bytes = module.finish();

        let mut validator =
            wasmparser::Validator::new_with_features(WasmFeatures::default() & !WasmFeatures::GC);

        validator.validate_all(&wasm_bytes).expect(
            "Encoding pre Wasm GC type should not accidentally use Wasm GC specific encoding",
        );
    }
}
