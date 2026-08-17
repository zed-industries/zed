/*!
Backend for [SPIR-V][spv] (Standard Portable Intermediate Representation).

# Layout of values in `uniform` buffers

WGSL's ["Internal Layout of Values"][ilov] rules specify the memory layout of
each WGSL type. The memory layout is important for data stored in `uniform` and
`storage` buffers, especially when exchanging data with CPU code.

Both WGSL and Vulkan specify some conditions that a type's memory layout
must satisfy in order to use that type in a `uniform` or `storage` buffer.
For `storage` buffers, the WGSL and Vulkan restrictions are compatible, but
for `uniform` buffers, WGSL allows some types that Vulkan does not, requiring
adjustments when emitting SPIR-V for `uniform` buffers.

## Padding in two-row matrices

SPIR-V provides detailed control over the layout of matrix types, and is
capable of describing the WGSL memory layout. However, Vulkan imposes
additional restrictions.

Vulkan's ["extended layout"][extended-layout] (also known as std140) rules
apply to types used in `uniform` buffers. Under these rules, matrices are
defined in terms of arrays of their vector type, and arrays are defined to have
an alignment equal to the alignment of their element type rounded up to a
multiple of 16. This means that each column of the matrix has a minimum
alignment of 16. WGSL, and consequently Naga IR, on the other hand specifies
column alignment equal to the alignment of the vector type, without being
rounded up to 16.

To compensate for this, for any `struct` used as a `uniform` buffer which
contains a two-row matrix, we declare an additional "std140 compatible" type
in which each column of the matrix has been decomposed into the containing
struct. For example, the following WGSL struct type:

```ignore
struct Baz {
    m: mat3x2<f32>,
}
```

is rendered as the SPIR-V struct type:

```ignore
OpTypeStruct %v2float %v2float %v2float
```

This has the effect that struct indices in Naga IR for such types do not
correspond to the struct indices used in SPIR-V. A mapping of struct indices
for these types is maintained in [`Std140CompatTypeInfo`].

Additionally, any two-row matrices that are declared directly as uniform
buffers without being wrapped in a struct are declared as a struct containing a
vector member for each column. Any array of a two-row matrix in a uniform
buffer is declared as an array of a struct containing a vector member for each
column. Any struct or array within a uniform buffer which contains a member or
whose base type requires a std140 compatible type declaration, itself requires a
std140 compatible type declaration.

Whenever a value of such a type is [`loaded`] we insert code to convert the
loaded value from the std140 compatible type to the regular type. This occurs
in `BlockContext::write_checked_load`, making use of the wrapper function
defined by `Writer::write_wrapped_convert_from_std140_compat_type`. For matrices
that have been decomposed as separate columns in the containing struct, we load
each column separately then composite the matrix type in
`BlockContext::maybe_write_load_uniform_matcx2_struct_member`.

Whenever a column of a matrix that has been decomposed into its containing
struct is [`accessed`] with a constant index we adjust the emitted access chain
to access from the containing struct instead, in `BlockContext::write_access_chain`.

Whenever a column of a uniform buffer two-row matrix is [`dynamically accessed`]
we must first load the matrix type, converting it from its std140 compatible
type as described above, then access the column using the wrapper function
defined by `Writer::write_wrapped_matcx2_get_column`. This is handled by
`BlockContext::maybe_write_uniform_matcx2_dynamic_access`.

Note that this approach differs somewhat from the equivalent code in the HLSL
backend. For HLSL all structs containing two-row matrices (or arrays of such)
have their declarations modified, not just those used as uniform buffers.
Two-row matrices and arrays of such only use modified type declarations when
used as uniform buffers, or additionally when used as struct member in any
context. This avoids the need to convert struct values when loading from uniform
buffers, but when loading arrays and matrices from uniform buffers or from any
struct the conversion is still required. In contrast, the approach used here
always requires converting *any* affected type when loading from a uniform
buffer, but consistently *only* when loading from a uniform buffer. As a result
this also means we only have to handle loads and not stores, as uniform buffers
are read-only.

[spv]: https://www.khronos.org/registry/SPIR-V/
[ilov]: https://gpuweb.github.io/gpuweb/wgsl/#internal-value-layout
[extended-layout]: https://docs.vulkan.org/spec/latest/chapters/interfaces.html#interfaces-resources-layout
[`loaded`]: crate::Expression::Load
[`accessed`]: crate::Expression::AccessIndex
[`dynamically accessed`]: crate::Expression::Access
*/

mod block;
mod f16_polyfill;
mod helpers;
mod image;
mod index;
mod instructions;
mod layout;
mod mesh_shader;
mod ray;
mod reclaimable;
mod selection;
mod subgroup;
mod writer;

pub use mesh_shader::{MeshReturnInfo, MeshReturnMember};
pub use spirv::{Capability, SourceLanguage};

use alloc::{string::String, vec::Vec};
use core::ops;

use spirv::Word;
use thiserror::Error;

use crate::arena::{Handle, HandleVec};
use crate::back::TaskDispatchLimits;
use crate::proc::{BoundsCheckPolicies, TypeResolution};

#[derive(Clone)]
struct PhysicalLayout {
    magic_number: Word,
    version: Word,
    generator: Word,
    bound: Word,
    instruction_schema: Word,
}

#[derive(Default)]
struct LogicalLayout {
    capabilities: Vec<Word>,
    extensions: Vec<Word>,
    ext_inst_imports: Vec<Word>,
    memory_model: Vec<Word>,
    entry_points: Vec<Word>,
    execution_modes: Vec<Word>,
    debugs: Vec<Word>,
    annotations: Vec<Word>,
    declarations: Vec<Word>,
    function_declarations: Vec<Word>,
    function_definitions: Vec<Word>,
}

#[derive(Clone)]
struct Instruction {
    op: spirv::Op,
    wc: u32,
    type_id: Option<Word>,
    result_id: Option<Word>,
    operands: Vec<Word>,
}

const BITS_PER_BYTE: crate::Bytes = 8;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("The requested entry point couldn't be found")]
    EntryPointNotFound,
    #[error("target SPIRV-{0}.{1} is not supported")]
    UnsupportedVersion(u8, u8),
    #[error("using {0} requires at least one of the capabilities {1:?}, but none are available")]
    MissingCapabilities(&'static str, Vec<Capability>),
    #[error("unimplemented {0}")]
    FeatureNotImplemented(&'static str),
    #[error("module is not validated properly: {0}")]
    Validation(&'static str),
    #[error("overrides should not be present at this stage")]
    Override,
    #[error(transparent)]
    ResolveArraySizeError(#[from] crate::proc::ResolveArraySizeError),
    #[error("module requires SPIRV-{0}.{1}, which isn't supported")]
    SpirvVersionTooLow(u8, u8),
    #[error("mapping of {0:?} is missing")]
    MissingBinding(crate::ResourceBinding),
}

#[derive(Default)]
struct IdGenerator(Word);

impl IdGenerator {
    const fn next(&mut self) -> Word {
        self.0 += 1;
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct DebugInfo<'a> {
    pub source_code: &'a str,
    pub file_name: &'a str,
    pub language: SourceLanguage,
}

/// A SPIR-V block to which we are still adding instructions.
///
/// A `Block` represents a SPIR-V block that does not yet have a termination
/// instruction like `OpBranch` or `OpReturn`.
///
/// The `OpLabel` that starts the block is implicit. It will be emitted based on
/// `label_id` when we write the block to a `LogicalLayout`.
///
/// To terminate a `Block`, pass the block and the termination instruction to
/// `Function::consume`. This takes ownership of the `Block` and transforms it
/// into a `TerminatedBlock`.
struct Block {
    label_id: Word,
    body: Vec<Instruction>,
}

/// A SPIR-V block that ends with a termination instruction.
struct TerminatedBlock {
    label_id: Word,
    body: Vec<Instruction>,
}

impl Block {
    const fn new(label_id: Word) -> Self {
        Block {
            label_id,
            body: Vec::new(),
        }
    }
}

struct LocalVariable {
    id: Word,
    instruction: Instruction,
}

struct ResultMember {
    id: Word,
    type_id: Word,
    built_in: Option<crate::BuiltIn>,
}

struct EntryPointContext {
    argument_ids: Vec<Word>,
    results: Vec<ResultMember>,
    task_payload_variable_id: Option<Word>,
    mesh_state: Option<MeshReturnInfo>,
}

#[derive(Default)]
struct Function {
    signature: Option<Instruction>,
    parameters: Vec<FunctionArgument>,
    variables: crate::FastHashMap<Handle<crate::LocalVariable>, LocalVariable>,
    /// Map from a local variable that is a ray query to its u32 tracker.
    ray_query_initialization_tracker_variables:
        crate::FastHashMap<Handle<crate::LocalVariable>, LocalVariable>,
    /// Map from a local variable that is a ray query to its tracker for the t max.
    ray_query_t_max_tracker_variables:
        crate::FastHashMap<Handle<crate::LocalVariable>, LocalVariable>,
    /// List of local variables used as a counters to ensure that all loops are bounded.
    force_loop_bounding_vars: Vec<LocalVariable>,

    /// A map from a Naga expression to the temporary SPIR-V variable we have
    /// spilled its value to, if any.
    ///
    /// Naga IR lets us apply [`Access`] expressions to expressions whose value
    /// is an array or matrix---not a pointer to such---but SPIR-V doesn't have
    /// instructions that can do the same. So when we encounter such code, we
    /// spill the expression's value to a generated temporary variable. That, we
    /// can obtain a pointer to, and then use an `OpAccessChain` instruction to
    /// do whatever series of [`Access`] and [`AccessIndex`] operations we need
    /// (with bounds checks). Finally, we generate an `OpLoad` to get the final
    /// value.
    ///
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    spilled_composites: crate::FastIndexMap<Handle<crate::Expression>, LocalVariable>,

    /// A set of expressions that are either in [`spilled_composites`] or refer
    /// to some component/element of such.
    ///
    /// [`spilled_composites`]: Function::spilled_composites
    spilled_accesses: crate::arena::HandleSet<crate::Expression>,

    /// A map taking each expression to the number of [`Access`] and
    /// [`AccessIndex`] expressions that uses it as a base value. If an
    /// expression has no entry, its count is zero: it is never used as a
    /// [`Access`] or [`AccessIndex`] base.
    ///
    /// We use this, together with [`ExpressionInfo::ref_count`], to recognize
    /// the tips of chains of [`Access`] and [`AccessIndex`] expressions that
    /// access spilled values --- expressions in [`spilled_composites`]. We
    /// defer generating code for the chain until we reach its tip, so we can
    /// handle it with a single instruction.
    ///
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    /// [`ExpressionInfo::ref_count`]: crate::valid::ExpressionInfo
    /// [`spilled_composites`]: Function::spilled_composites
    access_uses: crate::FastHashMap<Handle<crate::Expression>, usize>,

    blocks: Vec<TerminatedBlock>,
    entry_point_context: Option<EntryPointContext>,
}

impl Function {
    fn consume(&mut self, mut block: Block, termination: Instruction) {
        block.body.push(termination);
        self.blocks.push(TerminatedBlock {
            label_id: block.label_id,
            body: block.body,
        })
    }

    fn parameter_id(&self, index: u32) -> Word {
        match self.entry_point_context {
            Some(ref context) => context.argument_ids[index as usize],
            None => self.parameters[index as usize]
                .instruction
                .result_id
                .unwrap(),
        }
    }
}

/// Characteristics of a SPIR-V `OpTypeImage` type.
///
/// SPIR-V requires non-composite types to be unique, including images. Since we
/// use `LocalType` for this deduplication, it's essential that `LocalImageType`
/// be equal whenever the corresponding `OpTypeImage`s would be. To reduce the
/// likelihood of mistakes, we use fields that correspond exactly to the
/// operands of an `OpTypeImage` instruction, using the actual SPIR-V types
/// where practical.
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct LocalImageType {
    sampled_type: crate::Scalar,
    dim: spirv::Dim,
    flags: ImageTypeFlags,
    image_format: spirv::ImageFormat,
}

bitflags::bitflags! {
    /// Flags corresponding to the boolean(-ish) parameters to OpTypeImage.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ImageTypeFlags: u8 {
        const DEPTH = 0x1;
        const ARRAYED = 0x2;
        const MULTISAMPLED = 0x4;
        const SAMPLED = 0x8;
    }
}

impl LocalImageType {
    /// Construct a `LocalImageType` from the fields of a `TypeInner::Image`.
    fn from_inner(dim: crate::ImageDimension, arrayed: bool, class: crate::ImageClass) -> Self {
        let make_flags = |multi: bool, other: ImageTypeFlags| -> ImageTypeFlags {
            let mut flags = other;
            flags.set(ImageTypeFlags::ARRAYED, arrayed);
            flags.set(ImageTypeFlags::MULTISAMPLED, multi);
            flags
        };

        let dim = spirv::Dim::from(dim);

        match class {
            crate::ImageClass::Sampled { kind, multi } => LocalImageType {
                sampled_type: crate::Scalar { kind, width: 4 },
                dim,
                flags: make_flags(multi, ImageTypeFlags::SAMPLED),
                image_format: spirv::ImageFormat::Unknown,
            },
            crate::ImageClass::Depth { multi } => LocalImageType {
                sampled_type: crate::Scalar {
                    kind: crate::ScalarKind::Float,
                    width: 4,
                },
                dim,
                flags: make_flags(multi, ImageTypeFlags::DEPTH | ImageTypeFlags::SAMPLED),
                image_format: spirv::ImageFormat::Unknown,
            },
            crate::ImageClass::Storage { format, access: _ } => LocalImageType {
                sampled_type: format.into(),
                dim,
                flags: make_flags(false, ImageTypeFlags::empty()),
                image_format: format.into(),
            },
            crate::ImageClass::External => unimplemented!(),
        }
    }
}

/// A numeric type, for use in [`LocalType`].
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum NumericType {
    Scalar(crate::Scalar),
    Vector {
        size: crate::VectorSize,
        scalar: crate::Scalar,
    },
    Matrix {
        columns: crate::VectorSize,
        rows: crate::VectorSize,
        scalar: crate::Scalar,
    },
}

impl NumericType {
    const fn from_inner(inner: &crate::TypeInner) -> Option<Self> {
        match *inner {
            crate::TypeInner::Scalar(scalar) | crate::TypeInner::Atomic(scalar) => {
                Some(NumericType::Scalar(scalar))
            }
            crate::TypeInner::Vector { size, scalar } => Some(NumericType::Vector { size, scalar }),
            crate::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => Some(NumericType::Matrix {
                columns,
                rows,
                scalar,
            }),
            _ => None,
        }
    }

    const fn scalar(self) -> crate::Scalar {
        match self {
            NumericType::Scalar(scalar)
            | NumericType::Vector { scalar, .. }
            | NumericType::Matrix { scalar, .. } => scalar,
        }
    }

    const fn with_scalar(self, scalar: crate::Scalar) -> Self {
        match self {
            NumericType::Scalar(_) => NumericType::Scalar(scalar),
            NumericType::Vector { size, .. } => NumericType::Vector { size, scalar },
            NumericType::Matrix { columns, rows, .. } => NumericType::Matrix {
                columns,
                rows,
                scalar,
            },
        }
    }
}

/// A cooperative type, for use in [`LocalType`].
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum CooperativeType {
    Matrix {
        columns: crate::CooperativeSize,
        rows: crate::CooperativeSize,
        scalar: crate::Scalar,
        role: crate::CooperativeRole,
    },
}

impl CooperativeType {
    const fn from_inner(inner: &crate::TypeInner) -> Option<Self> {
        match *inner {
            crate::TypeInner::CooperativeMatrix {
                columns,
                rows,
                scalar,
                role,
            } => Some(Self::Matrix {
                columns,
                rows,
                scalar,
                role,
            }),
            _ => None,
        }
    }
}

/// A SPIR-V type constructed during code generation.
///
/// This is the variant of [`LookupType`] used to represent types that might not
/// be available in the arena. Variants are present here for one of two reasons:
///
/// -   They represent types synthesized during code generation, as explained
///     in the documentation for [`LookupType`].
///
/// -   They represent types for which SPIR-V forbids duplicate `OpType...`
///     instructions, requiring deduplication.
///
/// This is not a complete copy of [`TypeInner`]: for example, SPIR-V generation
/// never synthesizes new struct types, so `LocalType` has nothing for that.
///
/// Each `LocalType` variant should be handled identically to its analogous
/// `TypeInner` variant. You can use the [`Writer::localtype_from_inner`]
/// function to help with this, by converting everything possible to a
/// `LocalType` before inspecting it.
///
/// ## `LocalType` equality and SPIR-V `OpType` uniqueness
///
/// The definition of `Eq` on `LocalType` is carefully chosen to help us follow
/// certain SPIR-V rules. SPIR-V §2.8 requires some classes of `OpType...`
/// instructions to be unique; for example, you can't have two `OpTypeInt 32 1`
/// instructions in the same module. All 32-bit signed integers must use the
/// same type id.
///
/// All SPIR-V types that must be unique can be represented as a `LocalType`,
/// and two `LocalType`s are always `Eq` if SPIR-V would require them to use the
/// same `OpType...` instruction. This lets us avoid duplicates by recording the
/// ids of the type instructions we've already generated in a hash table,
/// [`Writer::lookup_type`], keyed by `LocalType`.
///
/// As another example, [`LocalImageType`], stored in the `LocalType::Image`
/// variant, is designed to help us deduplicate `OpTypeImage` instructions. See
/// its documentation for details.
///
/// SPIR-V does not require pointer types to be unique - but different
/// SPIR-V ids are considered to be distinct pointer types. Since Naga
/// uses structural type equality, we need to represent each Naga
/// equivalence class with a single SPIR-V `OpTypePointer`.
///
/// As it always must, the `Hash` implementation respects the `Eq` relation.
///
/// [`TypeInner`]: crate::TypeInner
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum LocalType {
    /// A numeric type.
    Numeric(NumericType),
    Cooperative(CooperativeType),
    Pointer {
        base: Word,
        class: spirv::StorageClass,
    },
    Image(LocalImageType),
    SampledImage {
        image_type_id: Word,
    },
    Sampler,
    BindingArray {
        base: Handle<crate::Type>,
        size: u32,
    },
    AccelerationStructure,
    RayQuery,
}

/// A type encountered during SPIR-V generation.
///
/// In the process of writing SPIR-V, we need to synthesize various types for
/// intermediate results and such: pointer types, vector/matrix component types,
/// or even booleans, which usually appear in SPIR-V code even when they're not
/// used by the module source.
///
/// However, we can't use `crate::Type` or `crate::TypeInner` for these, as the
/// type arena may not contain what we need (it only contains types used
/// directly by other parts of the IR), and the IR module is immutable, so we
/// can't add anything to it.
///
/// So for local use in the SPIR-V writer, we use this type, which holds either
/// a handle into the arena, or a [`LocalType`] containing something synthesized
/// locally.
///
/// This is very similar to the [`proc::TypeResolution`] enum, with `LocalType`
/// playing the role of `TypeInner`. However, `LocalType` also has other
/// properties needed for SPIR-V generation; see the description of
/// [`LocalType`] for details.
///
/// [`proc::TypeResolution`]: crate::proc::TypeResolution
#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum LookupType {
    Handle(Handle<crate::Type>),
    Local(LocalType),
}

impl From<LocalType> for LookupType {
    fn from(local: LocalType) -> Self {
        Self::Local(local)
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct LookupFunctionType {
    parameter_type_ids: Vec<Word>,
    return_type_id: Word,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum LookupRayQueryFunction {
    Initialize,
    Proceed,
    GenerateIntersection,
    ConfirmIntersection,
    GetVertexPositions { committed: bool },
    GetIntersection { committed: bool },
    Terminate,
}

#[derive(Debug)]
enum Dimension {
    Scalar,
    Vector,
    Matrix,
    CooperativeMatrix,
}

/// Key used to look up an operation which we have wrapped in a helper
/// function, which should be called instead of directly emitting code
/// for the expression. See [`Writer::wrapped_functions`].
#[derive(Debug, Eq, PartialEq, Hash)]
enum WrappedFunction {
    BinaryOp {
        op: crate::BinaryOperator,
        left_type_id: Word,
        right_type_id: Word,
    },
    ConvertFromStd140CompatType {
        r#type: Handle<crate::Type>,
    },
    MatCx2GetColumn {
        r#type: Handle<crate::Type>,
    },
}

/// A map from evaluated [`Expression`](crate::Expression)s to their SPIR-V ids.
///
/// When we emit code to evaluate a given `Expression`, we record the
/// SPIR-V id of its value here, under its `Handle<Expression>` index.
///
/// A `CachedExpressions` value can be indexed by a `Handle<Expression>` value.
///
/// [emit]: index.html#expression-evaluation-time-and-scope
#[derive(Default)]
struct CachedExpressions {
    ids: HandleVec<crate::Expression, Word>,
}
impl CachedExpressions {
    fn reset(&mut self, length: usize) {
        self.ids.clear();
        self.ids.resize(length, 0);
    }
}
impl ops::Index<Handle<crate::Expression>> for CachedExpressions {
    type Output = Word;
    fn index(&self, h: Handle<crate::Expression>) -> &Word {
        let id = &self.ids[h];
        if *id == 0 {
            unreachable!("Expression {:?} is not cached!", h);
        }
        id
    }
}
impl ops::IndexMut<Handle<crate::Expression>> for CachedExpressions {
    fn index_mut(&mut self, h: Handle<crate::Expression>) -> &mut Word {
        let id = &mut self.ids[h];
        if *id != 0 {
            unreachable!("Expression {:?} is already cached!", h);
        }
        id
    }
}
impl reclaimable::Reclaimable for CachedExpressions {
    fn reclaim(self) -> Self {
        CachedExpressions {
            ids: self.ids.reclaim(),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum CachedConstant {
    Literal(crate::proc::HashableLiteral),
    Composite {
        ty: LookupType,
        constituent_ids: Vec<Word>,
    },
    ZeroValue(Word),
}

/// The SPIR-V representation of a [`crate::GlobalVariable`].
///
/// In the Vulkan spec 1.3.296, the section [Descriptor Set Interface][dsi] says:
///
/// > Variables identified with the `Uniform` storage class are used to access
/// > transparent buffer backed resources. Such variables *must* be:
/// >
/// > -   typed as `OpTypeStruct`, or an array of this type,
/// >
/// > -   identified with a `Block` or `BufferBlock` decoration, and
/// >
/// > -   laid out explicitly using the `Offset`, `ArrayStride`, and `MatrixStride`
/// >     decorations as specified in "Offset and Stride Assignment".
///
/// This is followed by identical language for the `StorageBuffer`,
/// except that a `BufferBlock` decoration is not allowed.
///
/// When we encounter a global variable in the [`Storage`] or [`Uniform`]
/// address spaces whose type is not already [`Struct`], this backend implicitly
/// wraps the global variable in a struct: we generate a SPIR-V global variable
/// holding an `OpTypeStruct` with a single member, whose type is what the Naga
/// global's type would suggest, decorated as required above.
///
/// The [`helpers::global_needs_wrapper`] function determines whether a given
/// [`crate::GlobalVariable`] needs to be wrapped.
///
/// [dsi]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-descset
/// [`Storage`]: crate::AddressSpace::Storage
/// [`Uniform`]: crate::AddressSpace::Uniform
/// [`Struct`]: crate::TypeInner::Struct
#[derive(Clone)]
struct GlobalVariable {
    /// The SPIR-V id of the `OpVariable` that declares the global.
    ///
    /// If this global has been implicitly wrapped in an `OpTypeStruct`, this id
    /// refers to the wrapper, not the original Naga value it contains. If you
    /// need the Naga value, use [`access_id`] instead of this field.
    ///
    /// If this global is not implicitly wrapped, this is the same as
    /// [`access_id`].
    ///
    /// This is used to compute the `access_id` pointer in function prologues,
    /// and used for `ArrayLength` expressions, which need to pass the wrapper
    /// struct.
    ///
    /// [`access_id`]: GlobalVariable::access_id
    var_id: Word,

    /// The loaded value of a `AddressSpace::Handle` global variable.
    ///
    /// If the current function uses this global variable, this is the id of an
    /// `OpLoad` instruction in the function's prologue that loads its value.
    /// (This value is assigned as we write the prologue code of each function.)
    /// It is then used for all operations on the global, such as `OpImageSample`.
    handle_id: Word,

    /// The SPIR-V id of a pointer to this variable's Naga IR value.
    ///
    /// If the current function uses this global variable, and it has been
    /// implicitly wrapped in an `OpTypeStruct`, this is the id of an
    /// `OpAccessChain` instruction in the function's prologue that refers to
    /// the wrapped value inside the struct. (This value is assigned as we write
    /// the prologue code of each function.) If you need the wrapper struct
    /// itself, use [`var_id`] instead of this field.
    ///
    /// If this global is not implicitly wrapped, this is the same as
    /// [`var_id`].
    ///
    /// [`var_id`]: GlobalVariable::var_id
    access_id: Word,
}

impl GlobalVariable {
    const fn dummy() -> Self {
        Self {
            var_id: 0,
            handle_id: 0,
            access_id: 0,
        }
    }

    const fn new(id: Word) -> Self {
        Self {
            var_id: id,
            handle_id: 0,
            access_id: 0,
        }
    }

    /// Prepare `self` for use within a single function.
    const fn reset_for_function(&mut self) {
        self.handle_id = 0;
        self.access_id = 0;
    }
}

struct FunctionArgument {
    /// Actual instruction of the argument.
    instruction: Instruction,
    handle_id: Word,
}

/// Tracks the expressions for which the backend emits the following instructions:
/// - OpConstantTrue
/// - OpConstantFalse
/// - OpConstant
/// - OpConstantComposite
/// - OpConstantNull
struct ExpressionConstnessTracker {
    inner: crate::arena::HandleSet<crate::Expression>,
}

impl ExpressionConstnessTracker {
    fn from_arena(arena: &crate::Arena<crate::Expression>) -> Self {
        let mut inner = crate::arena::HandleSet::for_arena(arena);
        for (handle, expr) in arena.iter() {
            let insert = match *expr {
                crate::Expression::Literal(_)
                | crate::Expression::ZeroValue(_)
                | crate::Expression::Constant(_) => true,
                crate::Expression::Compose { ref components, .. } => {
                    components.iter().all(|&h| inner.contains(h))
                }
                crate::Expression::Splat { value, .. } => inner.contains(value),
                _ => false,
            };
            if insert {
                inner.insert(handle);
            }
        }
        Self { inner }
    }

    fn is_const(&self, value: Handle<crate::Expression>) -> bool {
        self.inner.contains(value)
    }
}

/// General information needed to emit SPIR-V for Naga statements.
struct BlockContext<'w> {
    /// The writer handling the module to which this code belongs.
    writer: &'w mut Writer,

    /// The [`Module`](crate::Module) for which we're generating code.
    ir_module: &'w crate::Module,

    /// The [`Function`](crate::Function) for which we're generating code.
    ir_function: &'w crate::Function,

    /// Information module validation produced about
    /// [`ir_function`](BlockContext::ir_function).
    fun_info: &'w crate::valid::FunctionInfo,

    /// The [`spv::Function`](Function) to which we are contributing SPIR-V instructions.
    function: &'w mut Function,

    /// SPIR-V ids for expressions we've evaluated.
    cached: CachedExpressions,

    /// The `Writer`'s temporary vector, for convenience.
    temp_list: Vec<Word>,

    /// Tracks the constness of `Expression`s residing in `self.ir_function.expressions`
    expression_constness: ExpressionConstnessTracker,

    force_loop_bounding: bool,

    /// Hash from an expression whose type is a ray query / pointer to a ray query to its tracker.
    /// Note: this is sparse, so can't be a handle vec
    ray_query_tracker_expr: crate::FastHashMap<Handle<crate::Expression>, RayQueryTrackers>,
}

#[derive(Clone, Copy)]
struct RayQueryTrackers {
    // Initialization tracker
    initialized_tracker: Word,
    // Tracks the t max from ray query initialize.
    // Unlike HLSL, spir-v's equivalent getter for the current committed t has UB (instead of just
    // returning t_max) if there was no previous hit (though in some places it treats the behaviour as
    // defined), therefore we must track the tmax inputted into ray query initialize.
    t_max_tracker: Word,
}

impl BlockContext<'_> {
    const fn gen_id(&mut self) -> Word {
        self.writer.id_gen.next()
    }

    fn get_type_id(&mut self, lookup_type: LookupType) -> Word {
        self.writer.get_type_id(lookup_type)
    }

    fn get_handle_type_id(&mut self, handle: Handle<crate::Type>) -> Word {
        self.writer.get_handle_type_id(handle)
    }

    fn get_expression_type_id(&mut self, tr: &TypeResolution) -> Word {
        self.writer.get_expression_type_id(tr)
    }

    fn get_index_constant(&mut self, index: Word) -> Word {
        self.writer.get_constant_scalar(crate::Literal::U32(index))
    }

    fn get_scope_constant(&mut self, scope: Word) -> Word {
        self.writer
            .get_constant_scalar(crate::Literal::I32(scope as _))
    }

    fn get_pointer_type_id(&mut self, base: Word, class: spirv::StorageClass) -> Word {
        self.writer.get_pointer_type_id(base, class)
    }

    fn get_numeric_type_id(&mut self, numeric: NumericType) -> Word {
        self.writer.get_numeric_type_id(numeric)
    }
}

/// Information about a type for which we have declared a std140 layout
/// compatible variant, because the type is used in a uniform but does not
/// adhere to std140 requirements. The uniform will be declared using the
/// type `type_id`, and the result of any `Load` will be immediately converted
/// to the base type. This is used for matrices with 2 rows, as well as any
/// arrays or structs containing such matrices.
pub struct Std140CompatTypeInfo {
    /// ID of the std140 compatible type declaration.
    type_id: Word,
    /// For structs, a mapping of Naga IR struct member indices to the indices
    /// used in the generated SPIR-V. For non-struct types this will be empty.
    member_indices: Vec<u32>,
}

pub struct Writer {
    physical_layout: PhysicalLayout,
    logical_layout: LogicalLayout,
    id_gen: IdGenerator,

    /// The set of capabilities modules are permitted to use.
    ///
    /// This is initialized from `Options::capabilities`.
    capabilities_available: Option<crate::FastHashSet<Capability>>,

    /// The set of capabilities used by this module.
    ///
    /// If `capabilities_available` is `Some`, then this is always a subset of
    /// that.
    capabilities_used: crate::FastIndexSet<Capability>,

    /// The set of spirv extensions used.
    extensions_used: crate::FastIndexSet<&'static str>,

    debug_strings: Vec<Instruction>,
    debugs: Vec<Instruction>,
    annotations: Vec<Instruction>,
    flags: WriterFlags,
    bounds_check_policies: BoundsCheckPolicies,
    zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode,
    force_loop_bounding: bool,
    use_storage_input_output_16: bool,
    void_type: Word,
    tuple_of_u32s_ty_id: Option<Word>,
    //TODO: convert most of these into vectors, addressable by handle indices
    lookup_type: crate::FastHashMap<LookupType, Word>,
    lookup_function: crate::FastHashMap<Handle<crate::Function>, Word>,
    lookup_function_type: crate::FastHashMap<LookupFunctionType, Word>,
    /// Operations which have been wrapped in a helper function. The value is
    /// the ID of the function, which should be called instead of emitting code
    /// for the operation directly.
    wrapped_functions: crate::FastHashMap<WrappedFunction, Word>,
    /// Indexed by const-expression handle indexes
    constant_ids: HandleVec<crate::Expression, Word>,
    cached_constants: crate::FastHashMap<CachedConstant, Word>,
    global_variables: HandleVec<crate::GlobalVariable, GlobalVariable>,
    std140_compat_uniform_types: crate::FastHashMap<Handle<crate::Type>, Std140CompatTypeInfo>,
    fake_missing_bindings: bool,
    binding_map: BindingMap,

    // Cached expressions are only meaningful within a BlockContext, but we
    // retain the table here between functions to save heap allocations.
    saved_cached: CachedExpressions,

    gl450_ext_inst_id: Word,

    // Just a temporary list of SPIR-V ids
    temp_list: Vec<Word>,

    ray_query_functions: crate::FastHashMap<LookupRayQueryFunction, Word>,

    /// F16 I/O polyfill manager for handling `f16` input/output variables
    /// when `StorageInputOutput16` capability is not available.
    io_f16_polyfills: f16_polyfill::F16IoPolyfill,

    /// Non semantic debug printf extension `OpExtInstImport`
    debug_printf: Option<Word>,
    pub(crate) ray_query_initialization_tracking: bool,

    /// Limits to the mesh shader dispatch group a task workgroup can dispatch.
    ///
    /// Metal for example limits to 1024 workgroups per task shader dispatch. Dispatching more is
    /// undefined behavior, so this would validate that to dispatch zero workgroups.
    task_dispatch_limits: Option<TaskDispatchLimits>,
    /// If true, naga may generate checks that the primitive indices are valid in the output.
    ///
    /// Currently this validation is unimplemented.
    mesh_shader_primitive_indices_clamp: bool,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct WriterFlags: u32 {
        /// Include debug labels for everything.
        const DEBUG = 0x1;

        /// Flip Y coordinate of [`BuiltIn::Position`] output.
        ///
        /// [`BuiltIn::Position`]: crate::BuiltIn::Position
        const ADJUST_COORDINATE_SPACE = 0x2;

        /// Emit [`OpName`][op] for input/output locations.
        ///
        /// Contrary to spec, some drivers treat it as semantic, not allowing
        /// any conflicts.
        ///
        /// [op]: https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpName
        const LABEL_VARYINGS = 0x4;

        /// Emit [`PointSize`] output builtin to vertex shaders, which is
        /// required for drawing with `PointList` topology.
        ///
        /// [`PointSize`]: crate::BuiltIn::PointSize
        const FORCE_POINT_SIZE = 0x8;

        /// Clamp [`BuiltIn::FragDepth`] output between 0 and 1.
        ///
        /// [`BuiltIn::FragDepth`]: crate::BuiltIn::FragDepth
        const CLAMP_FRAG_DEPTH = 0x10;

        /// Instead of silently failing if the arguments to generate a ray query are
        /// invalid, uses debug printf extension to print to the command line
        ///
        /// Note: VK_KHR_shader_non_semantic_info must be enabled. This will have no
        /// effect if `options.ray_query_initialization_tracking` is set to false.
        const PRINT_ON_RAY_QUERY_INITIALIZATION_FAIL = 0x20;
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BindingInfo {
    pub descriptor_set: u32,
    pub binding: u32,
    /// If the binding is an unsized binding array, this overrides the size.
    pub binding_array_size: Option<u32>,
}

// Using `BTreeMap` instead of `HashMap` so that we can hash itself.
pub type BindingMap = alloc::collections::BTreeMap<crate::ResourceBinding, BindingInfo>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZeroInitializeWorkgroupMemoryMode {
    /// Via `VK_KHR_zero_initialize_workgroup_memory` or Vulkan 1.3
    Native,
    /// Via assignments + barrier
    Polyfill,
    None,
}

#[derive(Debug, Clone)]
pub struct Options<'a> {
    /// (Major, Minor) target version of the SPIR-V.
    pub lang_version: (u8, u8),

    /// Configuration flags for the writer.
    pub flags: WriterFlags,

    /// Don't panic on missing bindings. Instead use fake values for `Binding`
    /// and `DescriptorSet` decorations. This may result in invalid SPIR-V.
    pub fake_missing_bindings: bool,

    /// Map of resources to information about the binding.
    pub binding_map: BindingMap,

    /// If given, the set of capabilities modules are allowed to use. Code that
    /// requires capabilities beyond these is rejected with an error.
    ///
    /// If this is `None`, all capabilities are permitted.
    pub capabilities: Option<crate::FastHashSet<Capability>>,

    /// How should generate code handle array, vector, matrix, or image texel
    /// indices that are out of range?
    pub bounds_check_policies: BoundsCheckPolicies,

    /// Dictates the way workgroup variables should be zero initialized
    pub zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode,

    /// If set, loops will have code injected into them, forcing the compiler
    /// to think the number of iterations is bounded.
    pub force_loop_bounding: bool,

    /// if set, ray queries will get a variable to track their state to prevent
    /// misuse.
    pub ray_query_initialization_tracking: bool,

    /// Whether to use the `StorageInputOutput16` capability for `f16` shader I/O.
    /// When false, `f16` I/O is polyfilled using `f32` types with conversions.
    pub use_storage_input_output_16: bool,

    pub debug_info: Option<DebugInfo<'a>>,

    pub task_dispatch_limits: Option<TaskDispatchLimits>,

    pub mesh_shader_primitive_indices_clamp: bool,
}

impl Default for Options<'_> {
    fn default() -> Self {
        let mut flags = WriterFlags::ADJUST_COORDINATE_SPACE
            | WriterFlags::LABEL_VARYINGS
            | WriterFlags::CLAMP_FRAG_DEPTH;
        if cfg!(debug_assertions) {
            flags |= WriterFlags::DEBUG;
        }
        Options {
            lang_version: (1, 0),
            flags,
            fake_missing_bindings: true,
            binding_map: BindingMap::default(),
            capabilities: None,
            bounds_check_policies: BoundsCheckPolicies::default(),
            zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode::Polyfill,
            force_loop_bounding: true,
            ray_query_initialization_tracking: true,
            use_storage_input_output_16: true,
            debug_info: None,
            task_dispatch_limits: None,
            mesh_shader_primitive_indices_clamp: true,
        }
    }
}

// A subset of options meant to be changed per pipeline.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PipelineOptions {
    /// The stage of the entry point.
    pub shader_stage: crate::ShaderStage,
    /// The name of the entry point.
    ///
    /// If no entry point that matches is found while creating a [`Writer`], a error will be thrown.
    pub entry_point: String,
}

pub fn write_vec(
    module: &crate::Module,
    info: &crate::valid::ModuleInfo,
    options: &Options,
    pipeline_options: Option<&PipelineOptions>,
) -> Result<Vec<u32>, Error> {
    let mut words: Vec<u32> = Vec::new();
    let mut w = Writer::new(options)?;

    w.write(
        module,
        info,
        pipeline_options,
        &options.debug_info,
        &mut words,
    )?;
    Ok(words)
}

pub fn supported_capabilities() -> crate::valid::Capabilities {
    use crate::valid::Capabilities as Caps;

    Caps::IMMEDIATES
        | Caps::FLOAT64
        | Caps::PRIMITIVE_INDEX
        | Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY
        | Caps::BUFFER_BINDING_ARRAY
        | Caps::STORAGE_TEXTURE_BINDING_ARRAY
        | Caps::STORAGE_BUFFER_BINDING_ARRAY
        | Caps::ACCELERATION_STRUCTURE_BINDING_ARRAY
        | Caps::CLIP_DISTANCE
        // No cull distance
        | Caps::STORAGE_TEXTURE_16BIT_NORM_FORMATS
        | Caps::MULTIVIEW
        | Caps::EARLY_DEPTH_TEST
        | Caps::MULTISAMPLED_SHADING
        | Caps::RAY_QUERY
        | Caps::DUAL_SOURCE_BLENDING
        | Caps::CUBE_ARRAY_TEXTURES
        | Caps::SHADER_INT64
        | Caps::SUBGROUP
        | Caps::SUBGROUP_BARRIER
        | Caps::SUBGROUP_VERTEX_STAGE
        | Caps::SHADER_INT64_ATOMIC_MIN_MAX
        | Caps::SHADER_INT64_ATOMIC_ALL_OPS
        | Caps::SHADER_FLOAT32_ATOMIC
        | Caps::TEXTURE_ATOMIC
        | Caps::TEXTURE_INT64_ATOMIC
        | Caps::RAY_HIT_VERTEX_POSITION
        | Caps::SHADER_FLOAT16
        // No TEXTURE_EXTERNAL
        | Caps::SHADER_FLOAT16_IN_FLOAT32
        | Caps::SHADER_BARYCENTRICS
        | Caps::MESH_SHADER
        | Caps::MESH_SHADER_POINT_TOPOLOGY
        | Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        // No BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        | Caps::COOPERATIVE_MATRIX
        | Caps::PER_VERTEX
        // No RAY_TRACING_PIPELINE
        | Caps::DRAW_INDEX
        | Caps::MEMORY_DECORATION_COHERENT
        | Caps::MEMORY_DECORATION_VOLATILE
}
