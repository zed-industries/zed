/*!
Frontend for [SPIR-V][spv] (Standard Portable Intermediate Representation).

## ID lookups

Our IR links to everything with `Handle`, while SPIR-V uses IDs.
In order to keep track of the associations, the parser has many lookup tables.
There map `spv::Word` into a specific IR handle, plus potentially a bit of
extra info, such as the related SPIR-V type ID.
TODO: would be nice to find ways that avoid looking up as much

## Inputs/Outputs

We create a private variable for each input/output. The relevant inputs are
populated at the start of an entry point. The outputs are saved at the end.

The function associated with an entry point is wrapped in another function,
such that we can handle any `Return` statements without problems.

## Row-major matrices

We don't handle them natively, since the IR only expects column majority.
Instead, we detect when such matrix is accessed in the `OpAccessChain`,
and we generate a parallel expression that loads the value, but transposed.
This value then gets used instead of `OpLoad` result later on.

[spv]: https://www.khronos.org/registry/SPIR-V/
*/

mod convert;
mod error;
mod function;
mod image;
mod next_block;
mod null;

pub use error::Error;

use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
use core::{convert::TryInto, mem, num::NonZeroU32};

use half::f16;
use petgraph::graphmap::GraphMap;

use super::atomic_upgrade::Upgrades;
use crate::{
    arena::{Arena, Handle, UniqueArena},
    proc::{Alignment, Layouter},
    FastHashMap, FastHashSet, FastIndexMap,
};
use convert::*;
use function::*;

pub const SUPPORTED_CAPABILITIES: &[spirv::Capability] = &[
    spirv::Capability::Shader,
    spirv::Capability::VulkanMemoryModel,
    spirv::Capability::ClipDistance,
    spirv::Capability::CullDistance,
    spirv::Capability::SampleRateShading,
    spirv::Capability::DerivativeControl,
    spirv::Capability::Matrix,
    spirv::Capability::ImageQuery,
    spirv::Capability::Sampled1D,
    spirv::Capability::Image1D,
    spirv::Capability::SampledCubeArray,
    spirv::Capability::ImageCubeArray,
    spirv::Capability::StorageImageExtendedFormats,
    spirv::Capability::Int8,
    spirv::Capability::Int16,
    spirv::Capability::Int64,
    spirv::Capability::Int64Atomics,
    spirv::Capability::Float16,
    spirv::Capability::AtomicFloat32AddEXT,
    spirv::Capability::Float64,
    spirv::Capability::Geometry,
    spirv::Capability::MultiView,
    spirv::Capability::StorageBuffer16BitAccess,
    spirv::Capability::UniformAndStorageBuffer16BitAccess,
    spirv::Capability::GroupNonUniform,
    spirv::Capability::GroupNonUniformVote,
    spirv::Capability::GroupNonUniformArithmetic,
    spirv::Capability::GroupNonUniformBallot,
    spirv::Capability::GroupNonUniformShuffle,
    spirv::Capability::GroupNonUniformShuffleRelative,
    spirv::Capability::RuntimeDescriptorArray,
    spirv::Capability::StorageImageMultisample,
    spirv::Capability::FragmentBarycentricKHR,
    // tricky ones
    spirv::Capability::UniformBufferArrayDynamicIndexing,
    spirv::Capability::StorageBufferArrayDynamicIndexing,
];
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "SPV_KHR_storage_buffer_storage_class",
    "SPV_KHR_vulkan_memory_model",
    "SPV_KHR_multiview",
    "SPV_EXT_descriptor_indexing",
    "SPV_EXT_shader_atomic_float_add",
    "SPV_KHR_16bit_storage",
    "SPV_KHR_non_semantic_info",
    "SPV_KHR_fragment_shader_barycentric",
];

#[derive(Copy, Clone)]
pub struct Instruction {
    op: spirv::Op,
    wc: u16,
}

impl Instruction {
    const fn expect(self, count: u16) -> Result<(), Error> {
        if self.wc == count {
            Ok(())
        } else {
            Err(Error::InvalidOperandCount(self.op, self.wc))
        }
    }

    fn expect_at_least(self, count: u16) -> Result<u16, Error> {
        self.wc
            .checked_sub(count)
            .ok_or(Error::InvalidOperandCount(self.op, self.wc))
    }
}

impl crate::TypeInner {
    fn can_comparison_sample(&self, module: &crate::Module) -> bool {
        match *self {
            crate::TypeInner::Image {
                class:
                    crate::ImageClass::Sampled {
                        kind: crate::ScalarKind::Float,
                        multi: false,
                    },
                ..
            } => true,
            crate::TypeInner::Sampler { .. } => true,
            crate::TypeInner::BindingArray { base, .. } => {
                module.types[base].inner.can_comparison_sample(module)
            }
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ModuleState {
    Empty,
    Capability,
    Extension,
    ExtInstImport,
    MemoryModel,
    EntryPoint,
    ExecutionMode,
    Source,
    Name,
    ModuleProcessed,
    Annotation,
    Type,
    Function,
}

trait LookupHelper {
    type Target;
    fn lookup(&self, key: spirv::Word) -> Result<&Self::Target, Error>;
}

impl<T> LookupHelper for FastHashMap<spirv::Word, T> {
    type Target = T;
    fn lookup(&self, key: spirv::Word) -> Result<&T, Error> {
        self.get(&key).ok_or(Error::InvalidId(key))
    }
}

impl crate::ImageDimension {
    const fn required_coordinate_size(&self) -> Option<crate::VectorSize> {
        match *self {
            crate::ImageDimension::D1 => None,
            crate::ImageDimension::D2 => Some(crate::VectorSize::Bi),
            crate::ImageDimension::D3 => Some(crate::VectorSize::Tri),
            crate::ImageDimension::Cube => Some(crate::VectorSize::Tri),
        }
    }
}

type MemberIndex = u32;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    struct DecorationFlags: u32 {
        const NON_READABLE = 0x1;
        const NON_WRITABLE = 0x2;
        const COHERENT = 0x4;
        const VOLATILE = 0x8;
    }
}

impl DecorationFlags {
    fn to_storage_access(self) -> crate::StorageAccess {
        let mut access = crate::StorageAccess::LOAD | crate::StorageAccess::STORE;
        if self.contains(DecorationFlags::NON_READABLE) {
            access &= !crate::StorageAccess::LOAD;
        }
        if self.contains(DecorationFlags::NON_WRITABLE) {
            access &= !crate::StorageAccess::STORE;
        }
        access
    }

    fn to_memory_decorations(self) -> crate::MemoryDecorations {
        let mut decorations = crate::MemoryDecorations::empty();
        if self.contains(DecorationFlags::COHERENT) {
            decorations |= crate::MemoryDecorations::COHERENT;
        }
        if self.contains(DecorationFlags::VOLATILE) {
            decorations |= crate::MemoryDecorations::VOLATILE;
        }
        decorations
    }
}

#[derive(Debug, PartialEq)]
enum Majority {
    Column,
    Row,
}

#[derive(Debug, Default)]
struct Decoration {
    name: Option<String>,
    built_in: Option<spirv::Word>,
    location: Option<spirv::Word>,
    index: Option<spirv::Word>,
    desc_set: Option<spirv::Word>,
    desc_index: Option<spirv::Word>,
    specialization_constant_id: Option<spirv::Word>,
    storage_buffer: bool,
    offset: Option<spirv::Word>,
    array_stride: Option<NonZeroU32>,
    matrix_stride: Option<NonZeroU32>,
    matrix_major: Option<Majority>,
    invariant: bool,
    interpolation: Option<crate::Interpolation>,
    sampling: Option<crate::Sampling>,
    flags: DecorationFlags,
}

impl Decoration {
    const fn debug_name(&self) -> &str {
        match self.name {
            Some(ref name) => name.as_str(),
            None => "?",
        }
    }

    const fn resource_binding(&self) -> Option<crate::ResourceBinding> {
        match *self {
            Decoration {
                desc_set: Some(group),
                desc_index: Some(binding),
                ..
            } => Some(crate::ResourceBinding { group, binding }),
            _ => None,
        }
    }

    fn io_binding(&self) -> Result<crate::Binding, Error> {
        match *self {
            Decoration {
                built_in: Some(built_in),
                location: None,
                invariant,
                ..
            } => Ok(crate::Binding::BuiltIn(map_builtin(built_in, invariant)?)),
            Decoration {
                built_in: None,
                location: Some(location),
                index: Some(index),
                ..
            } => Ok(crate::Binding::Location {
                location,
                interpolation: None,
                sampling: None,
                blend_src: Some(index),
                per_primitive: false,
            }),
            Decoration {
                built_in: None,
                location: Some(location),
                interpolation,
                sampling,
                ..
            } => Ok(crate::Binding::Location {
                location,
                interpolation,
                sampling,
                blend_src: None,
                per_primitive: false,
            }),
            _ => Err(Error::MissingDecoration(spirv::Decoration::Location)),
        }
    }
}

#[derive(Debug)]
struct LookupFunctionType {
    parameter_type_ids: Vec<spirv::Word>,
    return_type_id: spirv::Word,
}

struct LookupFunction {
    handle: Handle<crate::Function>,
    parameters_sampling: Vec<image::SamplingFlags>,
}

#[derive(Debug)]
struct EntryPoint {
    stage: crate::ShaderStage,
    name: String,
    early_depth_test: Option<crate::EarlyDepthTest>,
    workgroup_size: [u32; 3],
    variable_ids: Vec<spirv::Word>,
}

#[derive(Clone, Debug)]
struct LookupType {
    handle: Handle<crate::Type>,
    base_id: Option<spirv::Word>,
}

#[derive(Debug)]
enum Constant {
    Constant(Handle<crate::Constant>),
    Override(Handle<crate::Override>),
}

impl Constant {
    const fn to_expr(&self) -> crate::Expression {
        match *self {
            Self::Constant(c) => crate::Expression::Constant(c),
            Self::Override(o) => crate::Expression::Override(o),
        }
    }
}

#[derive(Debug)]
struct LookupConstant {
    inner: Constant,
    type_id: spirv::Word,
}

#[derive(Debug)]
enum Variable {
    Global,
    Input(crate::FunctionArgument),
    Output(crate::FunctionResult),
}

#[derive(Debug)]
struct LookupVariable {
    inner: Variable,
    handle: Handle<crate::GlobalVariable>,
    type_id: spirv::Word,
}

/// Information about SPIR-V result ids, stored in `Frontend::lookup_expression`.
#[derive(Clone, Debug)]
struct LookupExpression {
    /// The `Expression` constructed for this result.
    ///
    /// Note that, while a SPIR-V result id can be used in any block dominated
    /// by its definition, a Naga `Expression` is only in scope for the rest of
    /// its subtree. `Frontend::get_expr_handle` takes care of spilling the result
    /// to a `LocalVariable` which can then be used anywhere.
    handle: Handle<crate::Expression>,

    /// The SPIR-V type of this result.
    type_id: spirv::Word,

    /// The label id of the block that defines this expression.
    ///
    /// This is zero for globals, constants, and function parameters, since they
    /// originate outside any function's block.
    block_id: spirv::Word,
}

#[derive(Debug)]
struct LookupMember {
    type_id: spirv::Word,
    // This is true for either matrices, or arrays of matrices (yikes).
    row_major: bool,
}

#[derive(Clone, Debug)]
enum LookupLoadOverride {
    /// For arrays of matrices, we track them but not loading yet.
    Pending,
    /// For matrices, vectors, and scalars, we pre-load the data.
    Loaded(Handle<crate::Expression>),
}

#[derive(PartialEq)]
enum ExtendedClass {
    Global(crate::AddressSpace),
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub struct Options {
    /// The IR coordinate space matches all the APIs except SPIR-V,
    /// so by default we flip the Y coordinate of the `BuiltIn::Position`.
    /// This flag can be used to avoid this.
    pub adjust_coordinate_space: bool,
    /// Only allow shaders with the known set of capabilities.
    pub strict_capabilities: bool,
    pub block_ctx_dump_prefix: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            adjust_coordinate_space: true,
            strict_capabilities: true,
            block_ctx_dump_prefix: None,
        }
    }
}

/// An index into the `BlockContext::bodies` table.
type BodyIndex = usize;

/// An intermediate representation of a Naga [`Statement`].
///
/// `Body` and `BodyFragment` values form a tree: the `BodyIndex` fields of the
/// variants are indices of the child `Body` values in [`BlockContext::bodies`].
/// The `lower` function assembles the final `Statement` tree from this `Body`
/// tree. See [`BlockContext`] for details.
///
/// [`Statement`]: crate::Statement
#[derive(Debug)]
enum BodyFragment {
    BlockId(spirv::Word),
    If {
        condition: Handle<crate::Expression>,
        accept: BodyIndex,
        reject: BodyIndex,
    },
    Loop {
        /// The body of the loop. Its [`Body::parent`] is the block containing
        /// this `Loop` fragment.
        body: BodyIndex,

        /// The loop's continuing block. This is a grandchild: its
        /// [`Body::parent`] is the loop body block, whose index is above.
        continuing: BodyIndex,

        /// If the SPIR-V loop's back-edge branch is conditional, this is the
        /// expression that must be `false` for the back-edge to be taken, with
        /// `true` being for the "loop merge" (which breaks out of the loop).
        break_if: Option<Handle<crate::Expression>>,
    },
    Switch {
        selector: Handle<crate::Expression>,
        cases: Vec<(i32, BodyIndex)>,
        default: BodyIndex,
    },
    Break,
    Continue,
}

/// An intermediate representation of a Naga [`Block`].
///
/// This will be assembled into a `Block` once we've added spills for phi nodes
/// and out-of-scope expressions. See [`BlockContext`] for details.
///
/// [`Block`]: crate::Block
#[derive(Debug)]
struct Body {
    /// The index of the direct parent of this body
    parent: usize,
    data: Vec<BodyFragment>,
}

impl Body {
    /// Creates a new empty `Body` with the specified `parent`
    pub const fn with_parent(parent: usize) -> Self {
        Body {
            parent,
            data: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct PhiExpression {
    /// The local variable used for the phi node
    local: Handle<crate::LocalVariable>,
    /// List of (expression, block)
    expressions: Vec<(spirv::Word, spirv::Word)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MergeBlockInformation {
    LoopMerge,
    LoopContinue,
    SelectionMerge,
    SwitchMerge,
}

/// Fragments of Naga IR, to be assembled into `Statements` once data flow is
/// resolved.
///
/// We can't build a Naga `Statement` tree directly from SPIR-V blocks for three
/// main reasons:
///
/// - We parse a function's SPIR-V blocks in the order they appear in the file.
///   Within a function, SPIR-V requires that a block must precede any blocks it
///   structurally dominates, but doesn't say much else about the order in which
///   they must appear. So while we know we'll see control flow header blocks
///   before their child constructs and merge blocks, those children and the
///   merge blocks may appear in any order - perhaps even intermingled with
///   children of other constructs.
///
/// - A SPIR-V expression can be used in any SPIR-V block dominated by its
///   definition, whereas Naga expressions are scoped to the rest of their
///   subtree. This means that discovering an expression use later in the
///   function retroactively requires us to have spilled that expression into a
///   local variable back before we left its scope. (The docs for
///   [`Frontend::get_expr_handle`] explain this in more detail.)
///
/// - We translate SPIR-V OpPhi expressions as Naga local variables in which we
///   store the appropriate value before jumping to the OpPhi's block.
///
/// All these cases require us to go back and amend previously generated Naga IR
/// based on things we discover later. But modifying old blocks in arbitrary
/// spots in a `Statement` tree is awkward.
///
/// Instead, as we iterate through the function's body, we accumulate
/// control-flow-free fragments of Naga IR in the [`blocks`] table, while
/// building a skeleton of the Naga `Statement` tree in [`bodies`]. We note any
/// spills and temporaries we must introduce in [`phis`].
///
/// Finally, once we've processed the entire function, we add temporaries and
/// spills to the fragmentary `Blocks` as directed by `phis`, and assemble them
/// into the final Naga `Statement` tree as directed by `bodies`.
///
/// [`blocks`]: BlockContext::blocks
/// [`bodies`]: BlockContext::bodies
/// [`phis`]: BlockContext::phis
#[derive(Debug)]
struct BlockContext<'function> {
    /// Phi nodes encountered when parsing the function, used to generate spills
    /// to local variables.
    phis: Vec<PhiExpression>,

    /// Fragments of control-flow-free Naga IR.
    ///
    /// These will be stitched together into a proper [`Statement`] tree according
    /// to `bodies`, once parsing is complete.
    ///
    /// [`Statement`]: crate::Statement
    blocks: FastHashMap<spirv::Word, crate::Block>,

    /// Map from each SPIR-V block's label id to the index of the [`Body`] in
    /// [`bodies`] the block should append its contents to.
    ///
    /// Since each statement in a Naga [`Block`] dominates the next, we are sure
    /// to encounter their SPIR-V blocks in order. Thus, by having this table
    /// map a SPIR-V structured control flow construct's merge block to the same
    /// body index as its header block, when we encounter the merge block, we
    /// will simply pick up building the [`Body`] where the header left off.
    ///
    /// A function's first block is special: it is the only block we encounter
    /// without having seen its label mentioned in advance. (It's simply the
    /// first `OpLabel` after the `OpFunction`.) We thus assume that any block
    /// missing an entry here must be the first block, which always has body
    /// index zero.
    ///
    /// [`bodies`]: BlockContext::bodies
    /// [`Block`]: crate::Block
    body_for_label: FastHashMap<spirv::Word, BodyIndex>,

    /// SPIR-V metadata about merge/continue blocks.
    mergers: FastHashMap<spirv::Word, MergeBlockInformation>,

    /// A table of `Body` values, each representing a block in the final IR.
    ///
    /// The first element is always the function's top-level block.
    bodies: Vec<Body>,

    /// The module we're building.
    module: &'function mut crate::Module,

    /// Id of the function currently being processed
    function_id: spirv::Word,
    /// Expression arena of the function currently being processed
    expressions: &'function mut Arena<crate::Expression>,
    /// Local variables arena of the function currently being processed
    local_arena: &'function mut Arena<crate::LocalVariable>,
    /// Arguments of the function currently being processed
    arguments: &'function [crate::FunctionArgument],
    /// Metadata about the usage of function parameters as sampling objects
    parameter_sampling: &'function mut [image::SamplingFlags],
}

enum SignAnchor {
    Result,
    Operand,
}

pub struct Frontend<I> {
    data: I,
    data_offset: usize,
    state: ModuleState,
    layouter: Layouter,
    temp_bytes: Vec<u8>,
    ext_glsl_id: Option<spirv::Word>,
    ext_non_semantic_id: Option<spirv::Word>,
    future_decor: FastHashMap<spirv::Word, Decoration>,
    future_member_decor: FastHashMap<(spirv::Word, MemberIndex), Decoration>,
    lookup_member: FastHashMap<(Handle<crate::Type>, MemberIndex), LookupMember>,
    handle_sampling: FastHashMap<Handle<crate::GlobalVariable>, image::SamplingFlags>,

    /// A record of what is accessed by [`Atomic`] statements we've
    /// generated, so we can upgrade the types of their operands.
    ///
    /// [`Atomic`]: crate::Statement::Atomic
    upgrade_atomics: Upgrades,

    lookup_type: FastHashMap<spirv::Word, LookupType>,
    lookup_void_type: Option<spirv::Word>,
    lookup_storage_buffer_types: FastHashMap<Handle<crate::Type>, crate::StorageAccess>,
    lookup_constant: FastHashMap<spirv::Word, LookupConstant>,
    lookup_variable: FastHashMap<spirv::Word, LookupVariable>,
    lookup_expression: FastHashMap<spirv::Word, LookupExpression>,
    // Load overrides are used to work around row-major matrices
    lookup_load_override: FastHashMap<spirv::Word, LookupLoadOverride>,
    lookup_sampled_image: FastHashMap<spirv::Word, image::LookupSampledImage>,
    lookup_function_type: FastHashMap<spirv::Word, LookupFunctionType>,
    lookup_function: FastHashMap<spirv::Word, LookupFunction>,
    lookup_entry_point: FastHashMap<spirv::Word, EntryPoint>,
    // When parsing functions, each entry point function gets an entry here so that additional
    // processing for them can be performed after all function parsing.
    deferred_entry_points: Vec<(EntryPoint, spirv::Word)>,
    //Note: each `OpFunctionCall` gets a single entry here, indexed by the
    // dummy `Handle<crate::Function>` of the call site.
    deferred_function_calls: Vec<spirv::Word>,
    dummy_functions: Arena<crate::Function>,
    // Graph of all function calls through the module.
    // It's used to sort the functions (as nodes) topologically,
    // so that in the IR any called function is already known.
    function_call_graph: GraphMap<
        spirv::Word,
        (),
        petgraph::Directed,
        core::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    >,
    options: Options,

    /// Maps for a switch from a case target to the respective body and associated literals that
    /// use that target block id.
    ///
    /// Used to preserve allocations between instruction parsing.
    switch_cases: FastIndexMap<spirv::Word, (BodyIndex, Vec<i32>)>,

    /// Tracks access to gl_PerVertex's builtins, it is used to cull unused builtins since initializing those can
    /// affect performance and the mere presence of some of these builtins might cause backends to error since they
    /// might be unsupported.
    ///
    /// The problematic builtins are: PointSize, ClipDistance and CullDistance.
    ///
    /// glslang declares those by default even though they are never written to
    /// (see <https://github.com/KhronosGroup/glslang/issues/1868>)
    gl_per_vertex_builtin_access: FastHashSet<crate::BuiltIn>,
}

impl<I: Iterator<Item = u32>> Frontend<I> {
    pub fn new(data: I, options: &Options) -> Self {
        Frontend {
            data,
            data_offset: 0,
            state: ModuleState::Empty,
            layouter: Layouter::default(),
            temp_bytes: Vec::new(),
            ext_glsl_id: None,
            ext_non_semantic_id: None,
            future_decor: FastHashMap::default(),
            future_member_decor: FastHashMap::default(),
            handle_sampling: FastHashMap::default(),
            lookup_member: FastHashMap::default(),
            upgrade_atomics: Default::default(),
            lookup_type: FastHashMap::default(),
            lookup_void_type: None,
            lookup_storage_buffer_types: FastHashMap::default(),
            lookup_constant: FastHashMap::default(),
            lookup_variable: FastHashMap::default(),
            lookup_expression: FastHashMap::default(),
            lookup_load_override: FastHashMap::default(),
            lookup_sampled_image: FastHashMap::default(),
            lookup_function_type: FastHashMap::default(),
            lookup_function: FastHashMap::default(),
            lookup_entry_point: FastHashMap::default(),
            deferred_entry_points: Vec::default(),
            deferred_function_calls: Vec::default(),
            dummy_functions: Arena::new(),
            function_call_graph: GraphMap::new(),
            options: options.clone(),
            switch_cases: FastIndexMap::default(),
            gl_per_vertex_builtin_access: FastHashSet::default(),
        }
    }

    fn span_from(&self, from: usize) -> crate::Span {
        crate::Span::from(from..self.data_offset)
    }

    fn span_from_with_op(&self, from: usize) -> crate::Span {
        crate::Span::from((from - 4)..self.data_offset)
    }

    fn next(&mut self) -> Result<u32, Error> {
        if let Some(res) = self.data.next() {
            self.data_offset += 4;
            Ok(res)
        } else {
            Err(Error::IncompleteData)
        }
    }

    fn next_inst(&mut self) -> Result<Instruction, Error> {
        let word = self.next()?;
        let (wc, opcode) = ((word >> 16) as u16, (word & 0xffff) as u16);
        if wc == 0 {
            return Err(Error::InvalidWordCount);
        }
        let op = spirv::Op::from_u32(opcode as u32).ok_or(Error::UnknownInstruction(opcode))?;

        Ok(Instruction { op, wc })
    }

    fn next_string(&mut self, mut count: u16) -> Result<(String, u16), Error> {
        self.temp_bytes.clear();
        loop {
            if count == 0 {
                return Err(Error::BadString);
            }
            count -= 1;
            let chars = self.next()?.to_le_bytes();
            let pos = chars.iter().position(|&c| c == 0).unwrap_or(4);
            self.temp_bytes.extend_from_slice(&chars[..pos]);
            if pos < 4 {
                break;
            }
        }
        core::str::from_utf8(&self.temp_bytes)
            .map(|s| (s.to_owned(), count))
            .map_err(|_| Error::BadString)
    }

    fn next_decoration(
        &mut self,
        inst: Instruction,
        base_words: u16,
        dec: &mut Decoration,
    ) -> Result<(), Error> {
        let raw = self.next()?;
        let dec_typed = spirv::Decoration::from_u32(raw).ok_or(Error::InvalidDecoration(raw))?;
        log::trace!("\t\t{}: {:?}", dec.debug_name(), dec_typed);
        match dec_typed {
            spirv::Decoration::BuiltIn => {
                inst.expect(base_words + 2)?;
                dec.built_in = Some(self.next()?);
            }
            spirv::Decoration::Location => {
                inst.expect(base_words + 2)?;
                dec.location = Some(self.next()?);
            }
            spirv::Decoration::Index => {
                inst.expect(base_words + 2)?;
                dec.index = Some(self.next()?);
            }
            spirv::Decoration::DescriptorSet => {
                inst.expect(base_words + 2)?;
                dec.desc_set = Some(self.next()?);
            }
            spirv::Decoration::Binding => {
                inst.expect(base_words + 2)?;
                dec.desc_index = Some(self.next()?);
            }
            spirv::Decoration::BufferBlock => {
                dec.storage_buffer = true;
            }
            spirv::Decoration::Offset => {
                inst.expect(base_words + 2)?;
                dec.offset = Some(self.next()?);
            }
            spirv::Decoration::ArrayStride => {
                inst.expect(base_words + 2)?;
                dec.array_stride = NonZeroU32::new(self.next()?);
            }
            spirv::Decoration::MatrixStride => {
                inst.expect(base_words + 2)?;
                dec.matrix_stride = NonZeroU32::new(self.next()?);
            }
            spirv::Decoration::Invariant => {
                dec.invariant = true;
            }
            spirv::Decoration::NoPerspective => {
                dec.interpolation = Some(crate::Interpolation::Linear);
            }
            spirv::Decoration::Flat => {
                dec.interpolation = Some(crate::Interpolation::Flat);
            }
            spirv::Decoration::PerVertexKHR => {
                dec.interpolation = Some(crate::Interpolation::PerVertex);
            }
            spirv::Decoration::Centroid => {
                dec.sampling = Some(crate::Sampling::Centroid);
            }
            spirv::Decoration::Sample => {
                dec.sampling = Some(crate::Sampling::Sample);
            }
            spirv::Decoration::NonReadable => {
                dec.flags |= DecorationFlags::NON_READABLE;
            }
            spirv::Decoration::NonWritable => {
                dec.flags |= DecorationFlags::NON_WRITABLE;
            }
            spirv::Decoration::Coherent => {
                dec.flags |= DecorationFlags::COHERENT;
            }
            spirv::Decoration::Volatile => {
                dec.flags |= DecorationFlags::VOLATILE;
            }
            spirv::Decoration::ColMajor => {
                dec.matrix_major = Some(Majority::Column);
            }
            spirv::Decoration::RowMajor => {
                dec.matrix_major = Some(Majority::Row);
            }
            spirv::Decoration::SpecId => {
                dec.specialization_constant_id = Some(self.next()?);
            }
            other => {
                let level = match other {
                    // Block decorations show up everywhere and we don't
                    // really care about them, so to prevent log spam
                    // we demote them to debug level.
                    spirv::Decoration::Block => log::Level::Debug,
                    _ => log::Level::Warn,
                };

                log::log!(level, "Unknown decoration {other:?}");
                for _ in base_words + 1..inst.wc {
                    let _var = self.next()?;
                }
            }
        }
        Ok(())
    }

    /// Return the Naga [`Expression`] to use in `body_idx` to refer to the SPIR-V result `id`.
    ///
    /// Ideally, we would just have a map from each SPIR-V instruction id to the
    /// [`Handle`] for the Naga [`Expression`] we generated for it.
    /// Unfortunately, SPIR-V and Naga IR are different enough that such a
    /// straightforward relationship isn't possible.
    ///
    /// In SPIR-V, an instruction's result id can be used by any instruction
    /// dominated by that instruction. In Naga, an [`Expression`] is only in
    /// scope for the remainder of its [`Block`]. In pseudocode:
    ///
    /// ```ignore
    ///     loop {
    ///         a = f();
    ///         g(a);
    ///         break;
    ///     }
    ///     h(a);
    /// ```
    ///
    /// Suppose the calls to `f`, `g`, and `h` are SPIR-V instructions. In
    /// SPIR-V, both the `g` and `h` instructions are allowed to refer to `a`,
    /// because the loop body, including `f`, dominates both of them.
    ///
    /// But if `a` is a Naga [`Expression`], its scope ends at the end of the
    /// block it's evaluated in: the loop body. Thus, while the [`Expression`]
    /// we generate for `g` can refer to `a`, the one we generate for `h`
    /// cannot.
    ///
    /// Instead, the SPIR-V front end must generate Naga IR like this:
    ///
    /// ```ignore
    ///     var temp; // INTRODUCED
    ///     loop {
    ///         a = f();
    ///         g(a);
    ///         temp = a; // INTRODUCED
    ///     }
    ///     h(temp); // ADJUSTED
    /// ```
    ///
    /// In other words, where `a` is in scope, [`Expression`]s can refer to it
    /// directly; but once it is out of scope, we need to spill it to a
    /// temporary and refer to that instead.
    ///
    /// Given a SPIR-V expression `id` and the index `body_idx` of the [body]
    /// that wants to refer to it:
    ///
    /// - If the Naga [`Expression`] we generated for `id` is in scope in
    ///   `body_idx`, then we simply return its `Handle<Expression>`.
    ///
    /// - Otherwise, introduce a new [`LocalVariable`], and add an entry to
    ///   [`BlockContext::phis`] to arrange for `id`'s value to be spilled to
    ///   it. Then emit a fresh [`Load`] of that temporary variable for use in
    ///   `body_idx`'s block, and return its `Handle`.
    ///
    /// The SPIR-V domination rule ensures that the introduced [`LocalVariable`]
    /// will always have been initialized before it is used.
    ///
    /// `lookup` must be the [`LookupExpression`] for `id`.
    ///
    /// `body_idx` argument must be the index of the [`Body`] that hopes to use
    /// `id`'s [`Expression`].
    ///
    /// [`Expression`]: crate::Expression
    /// [`Handle`]: crate::Handle
    /// [`Block`]: crate::Block
    /// [body]: BlockContext::bodies
    /// [`LocalVariable`]: crate::LocalVariable
    /// [`Load`]: crate::Expression::Load
    fn get_expr_handle(
        &self,
        id: spirv::Word,
        lookup: &LookupExpression,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        body_idx: BodyIndex,
    ) -> Handle<crate::Expression> {
        // What `Body` was `id` defined in?
        let expr_body_idx = ctx
            .body_for_label
            .get(&lookup.block_id)
            .copied()
            .unwrap_or(0);

        // Don't need to do a load/store if the expression is in the main body
        // or if the expression is in the same body as where the query was
        // requested. The body_idx might actually not be the final one if a loop
        // or conditional occurs but in those cases we know that the new body
        // will be a subscope of the body that was passed so we can still reuse
        // the handle and not issue a load/store.
        if is_parent(body_idx, expr_body_idx, ctx) {
            lookup.handle
        } else {
            // Add a temporary variable of the same type which will be used to
            // store the original expression and used in the current block
            let ty = self.lookup_type[&lookup.type_id].handle;
            let local = ctx.local_arena.append(
                crate::LocalVariable {
                    name: None,
                    ty,
                    init: None,
                },
                crate::Span::default(),
            );

            block.extend(emitter.finish(ctx.expressions));
            let pointer = ctx.expressions.append(
                crate::Expression::LocalVariable(local),
                crate::Span::default(),
            );
            emitter.start(ctx.expressions);
            let expr = ctx
                .expressions
                .append(crate::Expression::Load { pointer }, crate::Span::default());

            // Add a slightly odd entry to the phi table, so that while `id`'s
            // `Expression` is still in scope, the usual phi processing will
            // spill its value to `local`, where we can find it later.
            //
            // This pretends that the block in which `id` is defined is the
            // predecessor of some other block with a phi in it that cites id as
            // one of its sources, and uses `local` as its variable. There is no
            // such phi, but nobody needs to know that.
            ctx.phis.push(PhiExpression {
                local,
                expressions: vec![(id, lookup.block_id)],
            });

            expr
        }
    }

    fn parse_expr_unary_op(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::UnaryOperator,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p_id = self.next()?;

        let p_lexp = self.lookup_expression.lookup(p_id)?;
        let handle = self.get_expr_handle(p_id, p_lexp, ctx, emitter, block, body_idx);

        let expr = crate::Expression::Unary { op, expr: handle };
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, self.span_from_with_op(start)),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    fn parse_expr_binary_op(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::BinaryOperator,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p1_id = self.next()?;
        let p2_id = self.next()?;

        let p1_lexp = self.lookup_expression.lookup(p1_id)?;
        let left = self.get_expr_handle(p1_id, p1_lexp, ctx, emitter, block, body_idx);
        let p2_lexp = self.lookup_expression.lookup(p2_id)?;
        let right = self.get_expr_handle(p2_id, p2_lexp, ctx, emitter, block, body_idx);

        let expr = crate::Expression::Binary { op, left, right };
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, self.span_from_with_op(start)),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    /// A more complicated version of the unary op,
    /// where we force the operand to have the same type as the result.
    fn parse_expr_unary_op_sign_adjusted(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::UnaryOperator,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p1_id = self.next()?;
        let span = self.span_from_with_op(start);

        let p1_lexp = self.lookup_expression.lookup(p1_id)?;
        let left = self.get_expr_handle(p1_id, p1_lexp, ctx, emitter, block, body_idx);

        let result_lookup_ty = self.lookup_type.lookup(result_type_id)?;
        let kind = ctx.module.types[result_lookup_ty.handle]
            .inner
            .scalar_kind()
            .unwrap();

        let expr = crate::Expression::Unary {
            op,
            expr: if p1_lexp.type_id == result_type_id {
                left
            } else {
                ctx.expressions.append(
                    crate::Expression::As {
                        expr: left,
                        kind,
                        convert: None,
                    },
                    span,
                )
            },
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, span),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    /// A more complicated version of the binary op,
    /// where we force the operand to have the same type as the result.
    /// This is mostly needed for "i++" and "i--" coming from GLSL.
    #[allow(clippy::too_many_arguments)]
    fn parse_expr_binary_op_sign_adjusted(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::BinaryOperator,
        // For arithmetic operations, we need the sign of operands to match the result.
        // For boolean operations, however, the operands need to match the signs, but
        // result is always different - a boolean.
        anchor: SignAnchor,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p1_id = self.next()?;
        let p2_id = self.next()?;
        let span = self.span_from_with_op(start);

        let p1_lexp = self.lookup_expression.lookup(p1_id)?;
        let left = self.get_expr_handle(p1_id, p1_lexp, ctx, emitter, block, body_idx);
        let p2_lexp = self.lookup_expression.lookup(p2_id)?;
        let right = self.get_expr_handle(p2_id, p2_lexp, ctx, emitter, block, body_idx);

        let expected_type_id = match anchor {
            SignAnchor::Result => result_type_id,
            SignAnchor::Operand => p1_lexp.type_id,
        };
        let expected_lookup_ty = self.lookup_type.lookup(expected_type_id)?;
        let kind = ctx.module.types[expected_lookup_ty.handle]
            .inner
            .scalar_kind()
            .unwrap();

        let expr = crate::Expression::Binary {
            op,
            left: if p1_lexp.type_id == expected_type_id {
                left
            } else {
                ctx.expressions.append(
                    crate::Expression::As {
                        expr: left,
                        kind,
                        convert: None,
                    },
                    span,
                )
            },
            right: if p2_lexp.type_id == expected_type_id {
                right
            } else {
                ctx.expressions.append(
                    crate::Expression::As {
                        expr: right,
                        kind,
                        convert: None,
                    },
                    span,
                )
            },
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, span),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    /// A version of the binary op where one or both of the arguments might need to be casted to a
    /// specific integer kind (unsigned or signed), used for operations like OpINotEqual or
    /// OpUGreaterThan.
    #[allow(clippy::too_many_arguments)]
    fn parse_expr_int_comparison(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::BinaryOperator,
        kind: crate::ScalarKind,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p1_id = self.next()?;
        let p2_id = self.next()?;
        let span = self.span_from_with_op(start);

        let p1_lexp = self.lookup_expression.lookup(p1_id)?;
        let left = self.get_expr_handle(p1_id, p1_lexp, ctx, emitter, block, body_idx);
        let p1_lookup_ty = self.lookup_type.lookup(p1_lexp.type_id)?;
        let p1_kind = ctx.module.types[p1_lookup_ty.handle]
            .inner
            .scalar_kind()
            .unwrap();
        let p2_lexp = self.lookup_expression.lookup(p2_id)?;
        let right = self.get_expr_handle(p2_id, p2_lexp, ctx, emitter, block, body_idx);
        let p2_lookup_ty = self.lookup_type.lookup(p2_lexp.type_id)?;
        let p2_kind = ctx.module.types[p2_lookup_ty.handle]
            .inner
            .scalar_kind()
            .unwrap();

        let expr = crate::Expression::Binary {
            op,
            left: if p1_kind == kind {
                left
            } else {
                ctx.expressions.append(
                    crate::Expression::As {
                        expr: left,
                        kind,
                        convert: None,
                    },
                    span,
                )
            },
            right: if p2_kind == kind {
                right
            } else {
                ctx.expressions.append(
                    crate::Expression::As {
                        expr: right,
                        kind,
                        convert: None,
                    },
                    span,
                )
            },
        };

        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, span),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    fn parse_expr_shift_op(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        op: crate::BinaryOperator,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let p1_id = self.next()?;
        let p2_id = self.next()?;

        let span = self.span_from_with_op(start);

        let p1_lexp = self.lookup_expression.lookup(p1_id)?;
        let left = self.get_expr_handle(p1_id, p1_lexp, ctx, emitter, block, body_idx);
        let p2_lexp = self.lookup_expression.lookup(p2_id)?;
        let p2_handle = self.get_expr_handle(p2_id, p2_lexp, ctx, emitter, block, body_idx);
        // convert the shift to Uint
        let right = ctx.expressions.append(
            crate::Expression::As {
                expr: p2_handle,
                kind: crate::ScalarKind::Uint,
                convert: None,
            },
            span,
        );

        let expr = crate::Expression::Binary { op, left, right };
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, span),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    fn parse_expr_derivative(
        &mut self,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        (axis, ctrl): (crate::DerivativeAxis, crate::DerivativeControl),
    ) -> Result<(), Error> {
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let arg_id = self.next()?;

        let arg_lexp = self.lookup_expression.lookup(arg_id)?;
        let arg_handle = self.get_expr_handle(arg_id, arg_lexp, ctx, emitter, block, body_idx);

        let expr = crate::Expression::Derivative {
            axis,
            ctrl,
            expr: arg_handle,
        };
        self.lookup_expression.insert(
            result_id,
            LookupExpression {
                handle: ctx.expressions.append(expr, self.span_from_with_op(start)),
                type_id: result_type_id,
                block_id,
            },
        );
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_composite(
        &self,
        root_expr: Handle<crate::Expression>,
        root_type_id: spirv::Word,
        object_expr: Handle<crate::Expression>,
        selections: &[spirv::Word],
        type_arena: &UniqueArena<crate::Type>,
        expressions: &mut Arena<crate::Expression>,
        span: crate::Span,
    ) -> Result<Handle<crate::Expression>, Error> {
        let selection = match selections.first() {
            Some(&index) => index,
            None => return Ok(object_expr),
        };
        let root_span = expressions.get_span(root_expr);
        let root_lookup = self.lookup_type.lookup(root_type_id)?;

        let (count, child_type_id) = match type_arena[root_lookup.handle].inner {
            crate::TypeInner::Struct { ref members, .. } => {
                let child_member = self
                    .lookup_member
                    .get(&(root_lookup.handle, selection))
                    .ok_or(Error::InvalidAccessType(root_type_id))?;
                (members.len(), child_member.type_id)
            }
            crate::TypeInner::Array { size, .. } => {
                let size = match size {
                    crate::ArraySize::Constant(size) => size.get(),
                    crate::ArraySize::Pending(_) => {
                        unreachable!();
                    }
                    // A runtime sized array is not a composite type
                    crate::ArraySize::Dynamic => {
                        return Err(Error::InvalidAccessType(root_type_id))
                    }
                };

                let child_type_id = root_lookup
                    .base_id
                    .ok_or(Error::InvalidAccessType(root_type_id))?;

                (size as usize, child_type_id)
            }
            crate::TypeInner::Vector { size, .. }
            | crate::TypeInner::Matrix { columns: size, .. } => {
                let child_type_id = root_lookup
                    .base_id
                    .ok_or(Error::InvalidAccessType(root_type_id))?;
                (size as usize, child_type_id)
            }
            _ => return Err(Error::InvalidAccessType(root_type_id)),
        };

        let mut components = Vec::with_capacity(count);
        for index in 0..count as u32 {
            let expr = expressions.append(
                crate::Expression::AccessIndex {
                    base: root_expr,
                    index,
                },
                if index == selection { span } else { root_span },
            );
            components.push(expr);
        }
        components[selection as usize] = self.insert_composite(
            components[selection as usize],
            child_type_id,
            object_expr,
            &selections[1..],
            type_arena,
            expressions,
            span,
        )?;

        Ok(expressions.append(
            crate::Expression::Compose {
                ty: root_lookup.handle,
                components,
            },
            span,
        ))
    }

    /// Return the Naga [`Expression`] for `pointer_id`, and its referent [`Type`].
    ///
    /// Return a [`Handle`] for a Naga [`Expression`] that holds the value of
    /// the SPIR-V instruction `pointer_id`, along with the [`Type`] to which it
    /// is a pointer.
    ///
    /// This may entail spilling `pointer_id`'s value to a temporary:
    /// see [`get_expr_handle`]'s documentation.
    ///
    /// [`Expression`]: crate::Expression
    /// [`Type`]: crate::Type
    /// [`Handle`]: crate::Handle
    /// [`get_expr_handle`]: Frontend::get_expr_handle
    fn get_exp_and_base_ty_handles(
        &self,
        pointer_id: spirv::Word,
        ctx: &mut BlockContext,
        emitter: &mut crate::proc::Emitter,
        block: &mut crate::Block,
        body_idx: usize,
    ) -> Result<(Handle<crate::Expression>, Handle<crate::Type>), Error> {
        log::trace!("\t\t\tlooking up pointer expr {pointer_id:?}");
        let p_lexp_handle;
        let p_lexp_ty_id;
        {
            let lexp = self.lookup_expression.lookup(pointer_id)?;
            p_lexp_handle = self.get_expr_handle(pointer_id, lexp, ctx, emitter, block, body_idx);
            p_lexp_ty_id = lexp.type_id;
        };

        log::trace!("\t\t\tlooking up pointer type {pointer_id:?}");
        let p_ty = self.lookup_type.lookup(p_lexp_ty_id)?;
        let p_ty_base_id = p_ty.base_id.ok_or(Error::InvalidAccessType(p_lexp_ty_id))?;

        log::trace!("\t\t\tlooking up pointer base type {p_ty_base_id:?} of {p_ty:?}");
        let p_base_ty = self.lookup_type.lookup(p_ty_base_id)?;

        Ok((p_lexp_handle, p_base_ty.handle))
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_atomic_expr_with_value(
        &mut self,
        inst: Instruction,
        emitter: &mut crate::proc::Emitter,
        ctx: &mut BlockContext,
        block: &mut crate::Block,
        block_id: spirv::Word,
        body_idx: usize,
        atomic_function: crate::AtomicFunction,
    ) -> Result<(), Error> {
        inst.expect(7)?;
        let start = self.data_offset;
        let result_type_id = self.next()?;
        let result_id = self.next()?;
        let pointer_id = self.next()?;
        let _scope_id = self.next()?;
        let _memory_semantics_id = self.next()?;
        let value_id = self.next()?;
        let span = self.span_from_with_op(start);

        let (p_lexp_handle, p_base_ty_handle) =
            self.get_exp_and_base_ty_handles(pointer_id, ctx, emitter, block, body_idx)?;

        log::trace!("\t\t\tlooking up value expr {value_id:?}");
        let v_lexp_handle = self.lookup_expression.lookup(value_id)?.handle;

        block.extend(emitter.finish(ctx.expressions));
        // Create an expression for our result
        let r_lexp_handle = {
            let expr = crate::Expression::AtomicResult {
                ty: p_base_ty_handle,
                comparison: false,
            };
            let handle = ctx.expressions.append(expr, span);
            self.lookup_expression.insert(
                result_id,
                LookupExpression {
                    handle,
                    type_id: result_type_id,
                    block_id,
                },
            );
            handle
        };
        emitter.start(ctx.expressions);

        // Create a statement for the op itself
        let stmt = crate::Statement::Atomic {
            pointer: p_lexp_handle,
            fun: atomic_function,
            value: v_lexp_handle,
            result: Some(r_lexp_handle),
        };
        block.push(stmt, span);

        // Store any associated global variables so we can upgrade their types later
        self.record_atomic_access(ctx, p_lexp_handle)?;

        Ok(())
    }

    fn make_expression_storage(
        &mut self,
        globals: &Arena<crate::GlobalVariable>,
        constants: &Arena<crate::Constant>,
        overrides: &Arena<crate::Override>,
    ) -> Arena<crate::Expression> {
        let mut expressions = Arena::new();
        assert!(self.lookup_expression.is_empty());
        // register global variables
        for (&id, var) in self.lookup_variable.iter() {
            let span = globals.get_span(var.handle);
            let handle = expressions.append(crate::Expression::GlobalVariable(var.handle), span);
            self.lookup_expression.insert(
                id,
                LookupExpression {
                    type_id: var.type_id,
                    handle,
                    // Setting this to an invalid id will cause get_expr_handle
                    // to default to the main body making sure no load/stores
                    // are added.
                    block_id: 0,
                },
            );
        }
        // register constants
        for (&id, con) in self.lookup_constant.iter() {
            let (expr, span) = match con.inner {
                Constant::Constant(c) => (crate::Expression::Constant(c), constants.get_span(c)),
                Constant::Override(o) => (crate::Expression::Override(o), overrides.get_span(o)),
            };
            let handle = expressions.append(expr, span);
            self.lookup_expression.insert(
                id,
                LookupExpression {
                    type_id: con.type_id,
                    handle,
                    // Setting this to an invalid id will cause get_expr_handle
                    // to default to the main body making sure no load/stores
                    // are added.
                    block_id: 0,
                },
            );
        }
        // done
        expressions
    }

    fn switch(&mut self, state: ModuleState, op: spirv::Op) -> Result<(), Error> {
        if state < self.state {
            Err(Error::UnsupportedInstruction(self.state, op))
        } else {
            self.state = state;
            Ok(())
        }
    }

    /// Walk the statement tree and patch it in the following cases:
    /// 1. Function call targets are replaced by `deferred_function_calls` map
    fn patch_statements(
        &mut self,
        statements: &mut crate::Block,
        expressions: &mut Arena<crate::Expression>,
        fun_parameter_sampling: &mut [image::SamplingFlags],
    ) -> Result<(), Error> {
        use crate::Statement as S;
        let mut i = 0usize;
        while i < statements.len() {
            match statements[i] {
                S::Emit(_) => {}
                S::Block(ref mut block) => {
                    self.patch_statements(block, expressions, fun_parameter_sampling)?;
                }
                S::If {
                    condition: _,
                    ref mut accept,
                    ref mut reject,
                } => {
                    self.patch_statements(reject, expressions, fun_parameter_sampling)?;
                    self.patch_statements(accept, expressions, fun_parameter_sampling)?;
                }
                S::Switch {
                    selector: _,
                    ref mut cases,
                } => {
                    for case in cases.iter_mut() {
                        self.patch_statements(&mut case.body, expressions, fun_parameter_sampling)?;
                    }
                }
                S::Loop {
                    ref mut body,
                    ref mut continuing,
                    break_if: _,
                } => {
                    self.patch_statements(body, expressions, fun_parameter_sampling)?;
                    self.patch_statements(continuing, expressions, fun_parameter_sampling)?;
                }
                S::Break
                | S::Continue
                | S::Return { .. }
                | S::Kill
                | S::ControlBarrier(_)
                | S::MemoryBarrier(_)
                | S::Store { .. }
                | S::ImageStore { .. }
                | S::Atomic { .. }
                | S::ImageAtomic { .. }
                | S::RayQuery { .. }
                | S::SubgroupBallot { .. }
                | S::SubgroupCollectiveOperation { .. }
                | S::SubgroupGather { .. }
                | S::RayPipelineFunction(..) => {}
                S::Call {
                    function: ref mut callee,
                    ref arguments,
                    ..
                } => {
                    let fun_id = self.deferred_function_calls[callee.index()];
                    let fun_lookup = self.lookup_function.lookup(fun_id)?;
                    *callee = fun_lookup.handle;

                    // Patch sampling flags
                    for (arg_index, arg) in arguments.iter().enumerate() {
                        let flags = match fun_lookup.parameters_sampling.get(arg_index) {
                            Some(&flags) if !flags.is_empty() => flags,
                            _ => continue,
                        };

                        match expressions[*arg] {
                            crate::Expression::GlobalVariable(handle) => {
                                if let Some(sampling) = self.handle_sampling.get_mut(&handle) {
                                    *sampling |= flags
                                }
                            }
                            crate::Expression::FunctionArgument(i) => {
                                fun_parameter_sampling[i as usize] |= flags;
                            }
                            ref other => return Err(Error::InvalidGlobalVar(other.clone())),
                        }
                    }
                }
                S::WorkGroupUniformLoad { .. } => unreachable!(),
                S::CooperativeStore { .. } => unreachable!(),
            }
            i += 1;
        }
        Ok(())
    }

    fn patch_function(
        &mut self,
        handle: Option<Handle<crate::Function>>,
        fun: &mut crate::Function,
    ) -> Result<(), Error> {
        // Note: this search is a bit unfortunate
        let (fun_id, mut parameters_sampling) = match handle {
            Some(h) => {
                let (&fun_id, lookup) = self
                    .lookup_function
                    .iter_mut()
                    .find(|&(_, ref lookup)| lookup.handle == h)
                    .unwrap();
                (fun_id, mem::take(&mut lookup.parameters_sampling))
            }
            None => (0, Vec::new()),
        };

        for (_, expr) in fun.expressions.iter_mut() {
            if let crate::Expression::CallResult(ref mut function) = *expr {
                let fun_id = self.deferred_function_calls[function.index()];
                *function = self.lookup_function.lookup(fun_id)?.handle;
            }
        }

        self.patch_statements(
            &mut fun.body,
            &mut fun.expressions,
            &mut parameters_sampling,
        )?;

        if let Some(lookup) = self.lookup_function.get_mut(&fun_id) {
            lookup.parameters_sampling = parameters_sampling;
        }
        Ok(())
    }

    pub fn parse(mut self) -> Result<crate::Module, Error> {
        let mut module = {
            if self.next()? != spirv::MAGIC_NUMBER {
                return Err(Error::InvalidHeader);
            }
            let version_raw = self.next()?;
            let generator = self.next()?;
            let _bound = self.next()?;
            let _schema = self.next()?;
            log::debug!("Generated by {generator} version {version_raw:x}");
            crate::Module::default()
        };

        self.layouter.clear();
        self.dummy_functions = Arena::new();
        self.lookup_function.clear();
        self.function_call_graph.clear();

        loop {
            use spirv::Op;

            let inst = match self.next_inst() {
                Ok(inst) => inst,
                Err(Error::IncompleteData) => break,
                Err(other) => return Err(other),
            };
            log::debug!("\t{:?} [{}]", inst.op, inst.wc);

            match inst.op {
                Op::Capability => self.parse_capability(inst),
                Op::Extension => self.parse_extension(inst),
                Op::ExtInstImport => self.parse_ext_inst_import(inst),
                Op::MemoryModel => self.parse_memory_model(inst),
                Op::EntryPoint => self.parse_entry_point(inst),
                Op::ExecutionMode => self.parse_execution_mode(inst),
                Op::String => self.parse_string(inst),
                Op::Source => self.parse_source(inst),
                Op::SourceExtension => self.parse_source_extension(inst),
                Op::Name => self.parse_name(inst),
                Op::MemberName => self.parse_member_name(inst),
                Op::ModuleProcessed => self.parse_module_processed(inst),
                Op::Decorate => self.parse_decorate(inst),
                Op::MemberDecorate => self.parse_member_decorate(inst),
                Op::TypeVoid => self.parse_type_void(inst),
                Op::TypeBool => self.parse_type_bool(inst, &mut module),
                Op::TypeInt => self.parse_type_int(inst, &mut module),
                Op::TypeFloat => self.parse_type_float(inst, &mut module),
                Op::TypeVector => self.parse_type_vector(inst, &mut module),
                Op::TypeMatrix => self.parse_type_matrix(inst, &mut module),
                Op::TypeFunction => self.parse_type_function(inst),
                Op::TypePointer => self.parse_type_pointer(inst, &mut module),
                Op::TypeArray => self.parse_type_array(inst, &mut module),
                Op::TypeRuntimeArray => self.parse_type_runtime_array(inst, &mut module),
                Op::TypeStruct => self.parse_type_struct(inst, &mut module),
                Op::TypeImage => self.parse_type_image(inst, &mut module),
                Op::TypeSampledImage => self.parse_type_sampled_image(inst),
                Op::TypeSampler => self.parse_type_sampler(inst, &mut module),
                Op::Constant | Op::SpecConstant => self.parse_constant(inst, &mut module),
                Op::ConstantComposite | Op::SpecConstantComposite => {
                    self.parse_composite_constant(inst, &mut module)
                }
                Op::ConstantNull | Op::Undef => self.parse_null_constant(inst, &mut module),
                Op::ConstantTrue | Op::SpecConstantTrue => {
                    self.parse_bool_constant(inst, true, &mut module)
                }
                Op::ConstantFalse | Op::SpecConstantFalse => {
                    self.parse_bool_constant(inst, false, &mut module)
                }
                Op::Variable => self.parse_global_variable(inst, &mut module),
                Op::Function => {
                    self.switch(ModuleState::Function, inst.op)?;
                    inst.expect(5)?;
                    self.parse_function(&mut module)
                }
                Op::ExtInst => {
                    // Ignore the result type and result id
                    let _ = self.next()?;
                    let _ = self.next()?;
                    let set_id = self.next()?;
                    if Some(set_id) == self.ext_non_semantic_id {
                        // We've already skipped the instruction byte, result type, result id, and instruction set id
                        for _ in 0..inst.wc - 4 {
                            self.next()?;
                        }
                        Ok(())
                    } else {
                        return Err(Error::UnsupportedInstruction(self.state, inst.op));
                    }
                }
                _ => Err(Error::UnsupportedInstruction(self.state, inst.op)), //TODO
            }?;
        }

        if !self.upgrade_atomics.is_empty() {
            log::debug!("Upgrading atomic pointers...");
            module.upgrade_atomics(&self.upgrade_atomics)?;
        }

        // Do entry point specific processing after all functions are parsed so that we can
        // cull unused problematic builtins of gl_PerVertex.
        for (ep, fun_id) in mem::take(&mut self.deferred_entry_points) {
            self.process_entry_point(&mut module, ep, fun_id)?;
        }

        log::debug!("Patching...");
        {
            let mut nodes = petgraph::algo::toposort(&self.function_call_graph, None)
                .map_err(|cycle| Error::FunctionCallCycle(cycle.node_id()))?;
            nodes.reverse(); // we need dominated first
            let mut functions = mem::take(&mut module.functions);
            for fun_id in nodes {
                if fun_id > !(functions.len() as u32) {
                    // skip all the fake IDs registered for the entry points
                    continue;
                }
                let lookup = self.lookup_function.get_mut(&fun_id).unwrap();
                // take out the function from the old array
                let fun = mem::take(&mut functions[lookup.handle]);
                // add it to the newly formed arena, and adjust the lookup
                lookup.handle = module
                    .functions
                    .append(fun, functions.get_span(lookup.handle));
            }
        }
        // patch all the functions
        for (handle, fun) in module.functions.iter_mut() {
            self.patch_function(Some(handle), fun)?;
        }
        for ep in module.entry_points.iter_mut() {
            self.patch_function(None, &mut ep.function)?;
        }

        // Check all the images and samplers to have consistent comparison property.
        for (handle, flags) in self.handle_sampling.drain() {
            if !image::patch_comparison_type(
                flags,
                module.global_variables.get_mut(handle),
                &mut module.types,
            ) {
                return Err(Error::InconsistentComparisonSampling(handle));
            }
        }

        if !self.future_decor.is_empty() {
            log::debug!("Unused item decorations: {:?}", self.future_decor);
            self.future_decor.clear();
        }
        if !self.future_member_decor.is_empty() {
            log::debug!("Unused member decorations: {:?}", self.future_member_decor);
            self.future_member_decor.clear();
        }

        Ok(module)
    }

    fn parse_capability(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Capability, inst.op)?;
        inst.expect(2)?;
        let capability = self.next()?;
        let cap =
            spirv::Capability::from_u32(capability).ok_or(Error::UnknownCapability(capability))?;
        if !SUPPORTED_CAPABILITIES.contains(&cap) {
            if self.options.strict_capabilities {
                return Err(Error::UnsupportedCapability(cap));
            } else {
                log::warn!("Unknown capability {cap:?}");
            }
        }
        Ok(())
    }

    fn parse_extension(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Extension, inst.op)?;
        inst.expect_at_least(2)?;
        let (name, left) = self.next_string(inst.wc - 1)?;
        if left != 0 {
            return Err(Error::InvalidOperand);
        }
        if !SUPPORTED_EXTENSIONS.contains(&name.as_str()) {
            return Err(Error::UnsupportedExtension(name));
        }
        Ok(())
    }

    fn parse_ext_inst_import(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Extension, inst.op)?;
        inst.expect_at_least(3)?;
        let result_id = self.next()?;
        let (name, left) = self.next_string(inst.wc - 2)?;
        if left != 0 {
            return Err(Error::InvalidOperand);
        }
        if &name == "GLSL.std.450" {
            self.ext_glsl_id = Some(result_id);
        } else if &name == "NonSemantic.Shader.DebugInfo.100" {
            // We completely ignore this extension. All related instructions are
            // non-semantic and only for debug purposes, and the spec says they
            // are ignorable. Many compilers (dxc, slang, etc) will emit these
            // instructions depending on configuration.
            self.ext_non_semantic_id = Some(result_id);
        } else {
            return Err(Error::UnsupportedExtSet(name));
        }
        Ok(())
    }

    fn parse_memory_model(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::MemoryModel, inst.op)?;
        inst.expect(3)?;
        let _addressing_model = self.next()?;
        let _memory_model = self.next()?;
        Ok(())
    }

    fn parse_entry_point(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::EntryPoint, inst.op)?;
        inst.expect_at_least(4)?;
        let exec_model = self.next()?;
        let exec_model = spirv::ExecutionModel::from_u32(exec_model)
            .ok_or(Error::UnsupportedExecutionModel(exec_model))?;
        let function_id = self.next()?;
        let (name, left) = self.next_string(inst.wc - 3)?;
        let ep = EntryPoint {
            stage: match exec_model {
                spirv::ExecutionModel::Vertex => crate::ShaderStage::Vertex,
                spirv::ExecutionModel::Fragment => crate::ShaderStage::Fragment,
                spirv::ExecutionModel::GLCompute => crate::ShaderStage::Compute,
                spirv::ExecutionModel::TaskEXT => crate::ShaderStage::Task,
                spirv::ExecutionModel::MeshEXT => crate::ShaderStage::Mesh,
                _ => return Err(Error::UnsupportedExecutionModel(exec_model as u32)),
            },
            name,
            early_depth_test: None,
            workgroup_size: [0; 3],
            variable_ids: self.data.by_ref().take(left as usize).collect(),
        };
        self.lookup_entry_point.insert(function_id, ep);
        Ok(())
    }

    fn parse_execution_mode(&mut self, inst: Instruction) -> Result<(), Error> {
        use spirv::ExecutionMode;

        self.switch(ModuleState::ExecutionMode, inst.op)?;
        inst.expect_at_least(3)?;

        let ep_id = self.next()?;
        let mode_id = self.next()?;
        let args: Vec<spirv::Word> = self.data.by_ref().take(inst.wc as usize - 3).collect();

        let ep = self
            .lookup_entry_point
            .get_mut(&ep_id)
            .ok_or(Error::InvalidId(ep_id))?;
        let mode =
            ExecutionMode::from_u32(mode_id).ok_or(Error::UnsupportedExecutionMode(mode_id))?;

        match mode {
            ExecutionMode::EarlyFragmentTests => {
                ep.early_depth_test = Some(crate::EarlyDepthTest::Force);
            }
            ExecutionMode::DepthUnchanged => {
                if let &mut Some(ref mut early_depth_test) = &mut ep.early_depth_test {
                    if let &mut crate::EarlyDepthTest::Allow {
                        ref mut conservative,
                    } = early_depth_test
                    {
                        *conservative = crate::ConservativeDepth::Unchanged;
                    }
                } else {
                    ep.early_depth_test = Some(crate::EarlyDepthTest::Allow {
                        conservative: crate::ConservativeDepth::Unchanged,
                    });
                }
            }
            ExecutionMode::DepthGreater => {
                if let &mut Some(ref mut early_depth_test) = &mut ep.early_depth_test {
                    if let &mut crate::EarlyDepthTest::Allow {
                        ref mut conservative,
                    } = early_depth_test
                    {
                        *conservative = crate::ConservativeDepth::GreaterEqual;
                    }
                } else {
                    ep.early_depth_test = Some(crate::EarlyDepthTest::Allow {
                        conservative: crate::ConservativeDepth::GreaterEqual,
                    });
                }
            }
            ExecutionMode::DepthLess => {
                if let &mut Some(ref mut early_depth_test) = &mut ep.early_depth_test {
                    if let &mut crate::EarlyDepthTest::Allow {
                        ref mut conservative,
                    } = early_depth_test
                    {
                        *conservative = crate::ConservativeDepth::LessEqual;
                    }
                } else {
                    ep.early_depth_test = Some(crate::EarlyDepthTest::Allow {
                        conservative: crate::ConservativeDepth::LessEqual,
                    });
                }
            }
            ExecutionMode::DepthReplacing => {
                // Ignored because it can be deduced from the IR.
            }
            ExecutionMode::OriginUpperLeft => {
                // Ignored because the other option (OriginLowerLeft) is not valid in Vulkan mode.
            }
            ExecutionMode::LocalSize => {
                ep.workgroup_size = [args[0], args[1], args[2]];
            }
            _ => {
                return Err(Error::UnsupportedExecutionMode(mode_id));
            }
        }

        Ok(())
    }

    fn parse_string(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Source, inst.op)?;
        inst.expect_at_least(3)?;
        let _id = self.next()?;
        let (_name, _) = self.next_string(inst.wc - 2)?;
        Ok(())
    }

    fn parse_source(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Source, inst.op)?;
        for _ in 1..inst.wc {
            let _ = self.next()?;
        }
        Ok(())
    }

    fn parse_source_extension(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Source, inst.op)?;
        inst.expect_at_least(2)?;
        let (_name, _) = self.next_string(inst.wc - 1)?;
        Ok(())
    }

    fn parse_name(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Name, inst.op)?;
        inst.expect_at_least(3)?;
        let id = self.next()?;
        let (name, left) = self.next_string(inst.wc - 2)?;
        if left != 0 {
            return Err(Error::InvalidOperand);
        }
        self.future_decor.entry(id).or_default().name = Some(name);
        Ok(())
    }

    fn parse_member_name(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Name, inst.op)?;
        inst.expect_at_least(4)?;
        let id = self.next()?;
        let member = self.next()?;
        let (name, left) = self.next_string(inst.wc - 3)?;
        if left != 0 {
            return Err(Error::InvalidOperand);
        }

        self.future_member_decor
            .entry((id, member))
            .or_default()
            .name = Some(name);
        Ok(())
    }

    fn parse_module_processed(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Name, inst.op)?;
        inst.expect_at_least(2)?;
        let (_info, left) = self.next_string(inst.wc - 1)?;
        //Note: string is ignored
        if left != 0 {
            return Err(Error::InvalidOperand);
        }
        Ok(())
    }

    fn parse_decorate(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Annotation, inst.op)?;
        inst.expect_at_least(3)?;
        let id = self.next()?;
        let mut dec = self.future_decor.remove(&id).unwrap_or_default();
        self.next_decoration(inst, 2, &mut dec)?;
        self.future_decor.insert(id, dec);
        Ok(())
    }

    fn parse_member_decorate(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Annotation, inst.op)?;
        inst.expect_at_least(4)?;
        let id = self.next()?;
        let member = self.next()?;

        let mut dec = self
            .future_member_decor
            .remove(&(id, member))
            .unwrap_or_default();
        self.next_decoration(inst, 3, &mut dec)?;
        self.future_member_decor.insert((id, member), dec);
        Ok(())
    }

    fn parse_type_void(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(2)?;
        let id = self.next()?;
        self.lookup_void_type = Some(id);
        Ok(())
    }

    fn parse_type_bool(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(2)?;
        let id = self.next()?;
        let inner = crate::TypeInner::Scalar(crate::Scalar::BOOL);
        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: self.future_decor.remove(&id).and_then(|dec| dec.name),
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: None,
            },
        );
        Ok(())
    }

    fn parse_type_int(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(4)?;
        let id = self.next()?;
        let width = self.next()?;
        let sign = self.next()?;
        let inner = crate::TypeInner::Scalar(crate::Scalar {
            kind: match sign {
                0 => crate::ScalarKind::Uint,
                1 => crate::ScalarKind::Sint,
                _ => return Err(Error::InvalidSign(sign)),
            },
            width: map_width(width)?,
        });
        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: self.future_decor.remove(&id).and_then(|dec| dec.name),
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: None,
            },
        );
        Ok(())
    }

    fn parse_type_float(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(3)?;
        let id = self.next()?;
        let width = self.next()?;
        let inner = crate::TypeInner::Scalar(crate::Scalar::float(map_width(width)?));
        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: self.future_decor.remove(&id).and_then(|dec| dec.name),
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: None,
            },
        );
        Ok(())
    }

    fn parse_type_vector(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(4)?;
        let id = self.next()?;
        let type_id = self.next()?;
        let type_lookup = self.lookup_type.lookup(type_id)?;
        let scalar = match module.types[type_lookup.handle].inner {
            crate::TypeInner::Scalar(scalar) => scalar,
            _ => return Err(Error::InvalidInnerType(type_id)),
        };
        let component_count = self.next()?;
        let inner = crate::TypeInner::Vector {
            size: map_vector_size(component_count)?,
            scalar,
        };
        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: self.future_decor.remove(&id).and_then(|dec| dec.name),
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: Some(type_id),
            },
        );
        Ok(())
    }

    fn parse_type_matrix(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(4)?;
        let id = self.next()?;
        let vector_type_id = self.next()?;
        let num_columns = self.next()?;
        let decor = self.future_decor.remove(&id);

        let vector_type_lookup = self.lookup_type.lookup(vector_type_id)?;
        let inner = match module.types[vector_type_lookup.handle].inner {
            crate::TypeInner::Vector { size, scalar } => crate::TypeInner::Matrix {
                columns: map_vector_size(num_columns)?,
                rows: size,
                scalar,
            },
            _ => return Err(Error::InvalidInnerType(vector_type_id)),
        };

        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: decor.and_then(|dec| dec.name),
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: Some(vector_type_id),
            },
        );
        Ok(())
    }

    fn parse_type_function(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect_at_least(3)?;
        let id = self.next()?;
        let return_type_id = self.next()?;
        let parameter_type_ids = self.data.by_ref().take(inst.wc as usize - 3).collect();
        self.lookup_function_type.insert(
            id,
            LookupFunctionType {
                parameter_type_ids,
                return_type_id,
            },
        );
        Ok(())
    }

    fn parse_type_pointer(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(4)?;
        let id = self.next()?;
        let storage_class = self.next()?;
        let type_id = self.next()?;

        let decor = self.future_decor.remove(&id);
        let base_lookup_ty = self.lookup_type.lookup(type_id)?;
        let base_inner = &module.types[base_lookup_ty.handle].inner;

        let space = if let Some(space) = base_inner.pointer_space() {
            space
        } else if self
            .lookup_storage_buffer_types
            .contains_key(&base_lookup_ty.handle)
        {
            crate::AddressSpace::Storage {
                access: crate::StorageAccess::default(),
            }
        } else {
            match map_storage_class(storage_class)? {
                ExtendedClass::Global(space) => space,
                ExtendedClass::Input | ExtendedClass::Output => crate::AddressSpace::Private,
            }
        };

        // We don't support pointers to runtime-sized arrays in the `Uniform`
        // storage class with the `BufferBlock` decoration. Runtime-sized arrays
        // should be in the StorageBuffer class.
        if let crate::TypeInner::Array {
            size: crate::ArraySize::Dynamic,
            ..
        } = *base_inner
        {
            match space {
                crate::AddressSpace::Storage { .. } => {}
                _ => {
                    return Err(Error::UnsupportedRuntimeArrayStorageClass);
                }
            }
        }

        // Don't bother with pointer stuff for `Handle` types.
        let lookup_ty = if space == crate::AddressSpace::Handle {
            base_lookup_ty.clone()
        } else {
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: decor.and_then(|dec| dec.name),
                        inner: crate::TypeInner::Pointer {
                            base: base_lookup_ty.handle,
                            space,
                        },
                    },
                    self.span_from_with_op(start),
                ),
                base_id: Some(type_id),
            }
        };
        self.lookup_type.insert(id, lookup_ty);
        Ok(())
    }

    fn parse_type_array(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(4)?;
        let id = self.next()?;
        let type_id = self.next()?;
        let length_id = self.next()?;
        let length_const = self.lookup_constant.lookup(length_id)?;

        let size = resolve_constant(module.to_ctx(), &length_const.inner)
            .and_then(NonZeroU32::new)
            .ok_or(Error::InvalidArraySize(length_id))?;

        let decor = self.future_decor.remove(&id).unwrap_or_default();
        let base = self.lookup_type.lookup(type_id)?.handle;

        self.layouter.update(module.to_ctx()).unwrap();

        // HACK if the underlying type is an image or a sampler, let's assume
        //      that we're dealing with a binding-array
        //
        // Note that it's not a strictly correct assumption, but rather a trade
        // off caused by an impedance mismatch between SPIR-V's and Naga's type
        // systems - Naga distinguishes between arrays and binding-arrays via
        // types (i.e. both kinds of arrays are just different types), while
        // SPIR-V distinguishes between them through usage - e.g. given:
        //
        // ```
        // %image = OpTypeImage %float 2D 2 0 0 2 Rgba16f
        // %uint_256 = OpConstant %uint 256
        // %image_array = OpTypeArray %image %uint_256
        // ```
        //
        // ```
        // %image = OpTypeImage %float 2D 2 0 0 2 Rgba16f
        // %uint_256 = OpConstant %uint 256
        // %image_array = OpTypeArray %image %uint_256
        // %image_array_ptr = OpTypePointer UniformConstant %image_array
        // ```
        //
        // ... in the first case, `%image_array` should technically correspond
        // to `TypeInner::Array`, while in the second case it should say
        // `TypeInner::BindingArray` (kinda, depending on whether `%image_array`
        // is ever used as a freestanding type or rather always through the
        // pointer-indirection).
        //
        // Anyway, at the moment we don't support other kinds of image / sampler
        // arrays than those binding-based, so this assumption is pretty safe
        // for now.
        let inner = if let crate::TypeInner::Image { .. } | crate::TypeInner::Sampler { .. } =
            module.types[base].inner
        {
            crate::TypeInner::BindingArray {
                base,
                size: crate::ArraySize::Constant(size),
            }
        } else {
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                stride: match decor.array_stride {
                    Some(stride) => stride.get(),
                    None => self.layouter[base].to_stride(),
                },
            }
        };

        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: decor.name,
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: Some(type_id),
            },
        );
        Ok(())
    }

    fn parse_type_runtime_array(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(3)?;
        let id = self.next()?;
        let type_id = self.next()?;

        let decor = self.future_decor.remove(&id).unwrap_or_default();
        let base = self.lookup_type.lookup(type_id)?.handle;

        self.layouter.update(module.to_ctx()).unwrap();

        // HACK same case as in `parse_type_array()`
        let inner = if let crate::TypeInner::Image { .. } | crate::TypeInner::Sampler { .. } =
            module.types[base].inner
        {
            crate::TypeInner::BindingArray {
                base: self.lookup_type.lookup(type_id)?.handle,
                size: crate::ArraySize::Dynamic,
            }
        } else {
            crate::TypeInner::Array {
                base: self.lookup_type.lookup(type_id)?.handle,
                size: crate::ArraySize::Dynamic,
                stride: match decor.array_stride {
                    Some(stride) => stride.get(),
                    None => self.layouter[base].to_stride(),
                },
            }
        };

        self.lookup_type.insert(
            id,
            LookupType {
                handle: module.types.insert(
                    crate::Type {
                        name: decor.name,
                        inner,
                    },
                    self.span_from_with_op(start),
                ),
                base_id: Some(type_id),
            },
        );
        Ok(())
    }

    fn parse_type_struct(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect_at_least(2)?;
        let id = self.next()?;
        let parent_decor = self.future_decor.remove(&id);
        let is_storage_buffer = parent_decor
            .as_ref()
            .is_some_and(|decor| decor.storage_buffer);

        self.layouter.update(module.to_ctx()).unwrap();

        let mut members = Vec::<crate::StructMember>::with_capacity(inst.wc as usize - 2);
        let mut member_lookups = Vec::with_capacity(members.capacity());
        let mut storage_access = crate::StorageAccess::empty();
        let mut span = 0;
        let mut alignment = Alignment::ONE;
        for i in 0..u32::from(inst.wc) - 2 {
            let type_id = self.next()?;
            let ty = self.lookup_type.lookup(type_id)?.handle;
            let decor = self
                .future_member_decor
                .remove(&(id, i))
                .unwrap_or_default();

            storage_access |= decor.flags.to_storage_access();

            member_lookups.push(LookupMember {
                type_id,
                row_major: decor.matrix_major == Some(Majority::Row),
            });

            let member_alignment = self.layouter[ty].alignment;
            span = member_alignment.round_up(span);
            alignment = member_alignment.max(alignment);

            let binding = decor.io_binding().ok();
            if let Some(offset) = decor.offset {
                span = offset;
            }
            let offset = span;

            span += self.layouter[ty].size;

            let inner = &module.types[ty].inner;
            if let crate::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } = *inner
            {
                if let Some(stride) = decor.matrix_stride {
                    let expected_stride = Alignment::from(rows) * scalar.width as u32;
                    if stride.get() != expected_stride {
                        return Err(Error::UnsupportedMatrixStride {
                            stride: stride.get(),
                            columns: columns as u8,
                            rows: rows as u8,
                            width: scalar.width,
                        });
                    }
                }
            }

            members.push(crate::StructMember {
                name: decor.name,
                ty,
                binding,
                offset,
            });
        }

        span = alignment.round_up(span);

        let inner = crate::TypeInner::Struct { span, members };

        let ty_handle = module.types.insert(
            crate::Type {
                name: parent_decor.and_then(|dec| dec.name),
                inner,
            },
            self.span_from_with_op(start),
        );

        if is_storage_buffer {
            self.lookup_storage_buffer_types
                .insert(ty_handle, storage_access);
        }
        for (i, member_lookup) in member_lookups.into_iter().enumerate() {
            self.lookup_member
                .insert((ty_handle, i as u32), member_lookup);
        }
        self.lookup_type.insert(
            id,
            LookupType {
                handle: ty_handle,
                base_id: None,
            },
        );
        Ok(())
    }

    fn parse_type_image(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(9)?;

        let id = self.next()?;
        let sample_type_id = self.next()?;
        let dim = self.next()?;
        let is_depth = self.next()?;
        let is_array = self.next()? != 0;
        let is_msaa = self.next()? != 0;
        let is_sampled = self.next()?;
        let format = self.next()?;

        let dim = map_image_dim(dim)?;
        let decor = self.future_decor.remove(&id).unwrap_or_default();

        // ensure there is a type for texture coordinate without extra components
        module.types.insert(
            crate::Type {
                name: None,
                inner: {
                    let scalar = crate::Scalar::F32;
                    match dim.required_coordinate_size() {
                        None => crate::TypeInner::Scalar(scalar),
                        Some(size) => crate::TypeInner::Vector { size, scalar },
                    }
                },
            },
            Default::default(),
        );

        let base_handle = self.lookup_type.lookup(sample_type_id)?.handle;
        let kind = module.types[base_handle]
            .inner
            .scalar_kind()
            .ok_or(Error::InvalidImageBaseType(base_handle))?;

        let inner = crate::TypeInner::Image {
            class: if is_depth == 1 {
                if is_sampled == 2 {
                    return Err(Error::InvalidImageDepthStorage);
                }

                crate::ImageClass::Depth { multi: is_msaa }
            }
            // If we have an unknown format and storage texture, this is
            // StorageRead/WriteWithoutFormat. We don't currently support
            // this.
            else if is_sampled == 2 && format == 0 {
                return Err(Error::InvalidStorageImageWithoutFormat);
            }
            // If we have explicit class information (is_sampled = 2 = Storage), use it.
            //
            // If we have unknown class information (is_sampled = 0 = Unknown), infer the
            // class from the presence of an explicit format.
            else if format != 0 && (is_sampled == 0 || is_sampled == 2) {
                crate::ImageClass::Storage {
                    format: map_image_format(format)?,
                    access: crate::StorageAccess::default(),
                }
            }
            // We will hit this case either when sampled is 1, or if we have unknown
            // sampling information or when sampled is 0 and we have no explicit format.
            else {
                crate::ImageClass::Sampled {
                    kind,
                    multi: is_msaa,
                }
            },
            dim,
            arrayed: is_array,
        };

        let handle = module.types.insert(
            crate::Type {
                name: decor.name,
                inner,
            },
            self.span_from_with_op(start),
        );

        self.lookup_type.insert(
            id,
            LookupType {
                handle,
                base_id: Some(sample_type_id),
            },
        );
        Ok(())
    }

    fn parse_type_sampled_image(&mut self, inst: Instruction) -> Result<(), Error> {
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(3)?;
        let id = self.next()?;
        let image_id = self.next()?;
        self.lookup_type.insert(
            id,
            LookupType {
                handle: self.lookup_type.lookup(image_id)?.handle,
                base_id: Some(image_id),
            },
        );
        Ok(())
    }

    fn parse_type_sampler(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(2)?;
        let id = self.next()?;
        let decor = self.future_decor.remove(&id).unwrap_or_default();
        let handle = module.types.insert(
            crate::Type {
                name: decor.name,
                inner: crate::TypeInner::Sampler { comparison: false },
            },
            self.span_from_with_op(start),
        );
        self.lookup_type.insert(
            id,
            LookupType {
                handle,
                base_id: None,
            },
        );
        Ok(())
    }

    fn parse_constant(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect_at_least(4)?;
        let type_id = self.next()?;
        let id = self.next()?;
        let type_lookup = self.lookup_type.lookup(type_id)?;
        let ty = type_lookup.handle;

        let literal = match module.types[ty].inner {
            crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Uint,
                width,
            }) => {
                let low = self.next()?;
                match width {
                    4 => crate::Literal::U32(low),
                    8 => {
                        inst.expect(5)?;
                        let high = self.next()?;
                        crate::Literal::U64((u64::from(high) << 32) | u64::from(low))
                    }
                    _ => return Err(Error::InvalidTypeWidth(width as u32)),
                }
            }
            crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Sint,
                width,
            }) => {
                let low = self.next()?;
                match width {
                    4 => crate::Literal::I32(low as i32),
                    8 => {
                        inst.expect(5)?;
                        let high = self.next()?;
                        crate::Literal::I64(((u64::from(high) << 32) | u64::from(low)) as i64)
                    }
                    _ => return Err(Error::InvalidTypeWidth(width as u32)),
                }
            }
            crate::TypeInner::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Float,
                width,
            }) => {
                let low = self.next()?;
                match width {
                    // https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Literal
                    // If a numeric type’s bit width is less than 32-bits, the value appears in the low-order bits of the word.
                    2 => crate::Literal::F16(f16::from_bits(low as u16)),
                    4 => crate::Literal::F32(f32::from_bits(low)),
                    8 => {
                        inst.expect(5)?;
                        let high = self.next()?;
                        crate::Literal::F64(f64::from_bits(
                            (u64::from(high) << 32) | u64::from(low),
                        ))
                    }
                    _ => return Err(Error::InvalidTypeWidth(width as u32)),
                }
            }
            _ => return Err(Error::UnsupportedType(type_lookup.handle)),
        };

        let span = self.span_from_with_op(start);

        let init = module
            .global_expressions
            .append(crate::Expression::Literal(literal), span);

        self.insert_parsed_constant(module, id, type_id, ty, init, span)
    }

    fn parse_composite_constant(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect_at_least(3)?;
        let type_id = self.next()?;
        let id = self.next()?;

        let type_lookup = self.lookup_type.lookup(type_id)?;
        let ty = type_lookup.handle;

        let mut components = Vec::with_capacity(inst.wc as usize - 3);
        for _ in 0..components.capacity() {
            let start = self.data_offset;
            let component_id = self.next()?;
            let span = self.span_from_with_op(start);
            let constant = self.lookup_constant.lookup(component_id)?;
            let expr = module
                .global_expressions
                .append(constant.inner.to_expr(), span);
            components.push(expr);
        }

        let span = self.span_from_with_op(start);

        let init = module
            .global_expressions
            .append(crate::Expression::Compose { ty, components }, span);

        self.insert_parsed_constant(module, id, type_id, ty, init, span)
    }

    fn parse_null_constant(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(3)?;
        let type_id = self.next()?;
        let id = self.next()?;
        let span = self.span_from_with_op(start);

        let type_lookup = self.lookup_type.lookup(type_id)?;
        let ty = type_lookup.handle;

        let init = module
            .global_expressions
            .append(crate::Expression::ZeroValue(ty), span);

        self.insert_parsed_constant(module, id, type_id, ty, init, span)
    }

    fn parse_bool_constant(
        &mut self,
        inst: Instruction,
        value: bool,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect(3)?;
        let type_id = self.next()?;
        let id = self.next()?;
        let span = self.span_from_with_op(start);

        let type_lookup = self.lookup_type.lookup(type_id)?;
        let ty = type_lookup.handle;

        let init = module.global_expressions.append(
            crate::Expression::Literal(crate::Literal::Bool(value)),
            span,
        );

        self.insert_parsed_constant(module, id, type_id, ty, init, span)
    }

    fn insert_parsed_constant(
        &mut self,
        module: &mut crate::Module,
        id: u32,
        type_id: u32,
        ty: Handle<crate::Type>,
        init: Handle<crate::Expression>,
        span: crate::Span,
    ) -> Result<(), Error> {
        let decor = self.future_decor.remove(&id).unwrap_or_default();

        let inner = if let Some(id) = decor.specialization_constant_id {
            let o = crate::Override {
                name: decor.name,
                id: Some(id.try_into().map_err(|_| Error::SpecIdTooHigh(id))?),
                ty,
                init: Some(init),
            };
            Constant::Override(module.overrides.append(o, span))
        } else {
            let c = crate::Constant {
                name: decor.name,
                ty,
                init,
            };
            Constant::Constant(module.constants.append(c, span))
        };

        self.lookup_constant
            .insert(id, LookupConstant { inner, type_id });
        Ok(())
    }

    fn parse_global_variable(
        &mut self,
        inst: Instruction,
        module: &mut crate::Module,
    ) -> Result<(), Error> {
        let start = self.data_offset;
        self.switch(ModuleState::Type, inst.op)?;
        inst.expect_at_least(4)?;
        let type_id = self.next()?;
        let id = self.next()?;
        let storage_class = self.next()?;
        let init = if inst.wc > 4 {
            inst.expect(5)?;
            let start = self.data_offset;
            let init_id = self.next()?;
            let span = self.span_from_with_op(start);
            let lconst = self.lookup_constant.lookup(init_id)?;
            let expr = module
                .global_expressions
                .append(lconst.inner.to_expr(), span);
            Some(expr)
        } else {
            None
        };
        let span = self.span_from_with_op(start);
        let dec = self.future_decor.remove(&id).unwrap_or_default();

        let original_ty = self.lookup_type.lookup(type_id)?.handle;
        let mut ty = original_ty;

        if let crate::TypeInner::Pointer { base, space: _ } = module.types[original_ty].inner {
            ty = base;
        }

        if let crate::TypeInner::BindingArray { .. } = module.types[original_ty].inner {
            // Inside `parse_type_array()` we guess that an array of images or
            // samplers must be a binding array, and here we validate that guess
            if dec.desc_set.is_none() || dec.desc_index.is_none() {
                return Err(Error::NonBindingArrayOfImageOrSamplers);
            }
        }

        if let crate::TypeInner::Image {
            dim,
            arrayed,
            class: crate::ImageClass::Storage { format, access: _ },
        } = module.types[ty].inner
        {
            // Storage image types in IR have to contain the access, but not in the SPIR-V.
            // The same image type in SPIR-V can be used (and has to be used) for multiple images.
            // So we copy the type out and apply the variable access decorations.
            let access = dec.flags.to_storage_access();

            ty = module.types.insert(
                crate::Type {
                    name: None,
                    inner: crate::TypeInner::Image {
                        dim,
                        arrayed,
                        class: crate::ImageClass::Storage { format, access },
                    },
                },
                Default::default(),
            );
        }

        let ext_class = match self.lookup_storage_buffer_types.get(&ty) {
            Some(&access) => ExtendedClass::Global(crate::AddressSpace::Storage { access }),
            None => map_storage_class(storage_class)?,
        };

        let (inner, var) = match ext_class {
            ExtendedClass::Global(mut space) => {
                if let crate::AddressSpace::Storage { ref mut access } = space {
                    *access &= dec.flags.to_storage_access();
                }
                let var = crate::GlobalVariable {
                    binding: dec.resource_binding(),
                    name: dec.name,
                    space,
                    ty,
                    init,
                    memory_decorations: dec.flags.to_memory_decorations(),
                };
                (Variable::Global, var)
            }
            ExtendedClass::Input => {
                let binding = dec.io_binding()?;
                let mut unsigned_ty = ty;
                if let crate::Binding::BuiltIn(built_in) = binding {
                    let needs_inner_uint = match built_in {
                        crate::BuiltIn::BaseInstance
                        | crate::BuiltIn::BaseVertex
                        | crate::BuiltIn::InstanceIndex
                        | crate::BuiltIn::SampleIndex
                        | crate::BuiltIn::VertexIndex
                        | crate::BuiltIn::PrimitiveIndex
                        | crate::BuiltIn::LocalInvocationIndex => {
                            Some(crate::TypeInner::Scalar(crate::Scalar::U32))
                        }
                        crate::BuiltIn::GlobalInvocationId
                        | crate::BuiltIn::LocalInvocationId
                        | crate::BuiltIn::WorkGroupId
                        | crate::BuiltIn::WorkGroupSize => Some(crate::TypeInner::Vector {
                            size: crate::VectorSize::Tri,
                            scalar: crate::Scalar::U32,
                        }),
                        crate::BuiltIn::Barycentric { perspective: false } => {
                            Some(crate::TypeInner::Vector {
                                size: crate::VectorSize::Tri,
                                scalar: crate::Scalar::F32,
                            })
                        }
                        _ => None,
                    };
                    if let (Some(inner), Some(crate::ScalarKind::Sint)) =
                        (needs_inner_uint, module.types[ty].inner.scalar_kind())
                    {
                        unsigned_ty = module
                            .types
                            .insert(crate::Type { name: None, inner }, Default::default());
                    }
                }

                let var = crate::GlobalVariable {
                    name: dec.name.clone(),
                    space: crate::AddressSpace::Private,
                    binding: None,
                    ty,
                    init: None,
                    memory_decorations: crate::MemoryDecorations::empty(),
                };

                let inner = Variable::Input(crate::FunctionArgument {
                    name: dec.name,
                    ty: unsigned_ty,
                    binding: Some(binding),
                });
                (inner, var)
            }
            ExtendedClass::Output => {
                // For output interface blocks, this would be a structure.
                let binding = dec.io_binding().ok();
                let init = match binding {
                    Some(crate::Binding::BuiltIn(built_in)) => {
                        match null::generate_default_built_in(
                            Some(built_in),
                            ty,
                            &mut module.global_expressions,
                            span,
                        ) {
                            Ok(handle) => Some(handle),
                            Err(e) => {
                                log::warn!("Failed to initialize output built-in: {e}");
                                None
                            }
                        }
                    }
                    Some(crate::Binding::Location { .. }) => None,
                    None => match module.types[ty].inner {
                        crate::TypeInner::Struct { ref members, .. } => {
                            let mut components = Vec::with_capacity(members.len());
                            for member in members.iter() {
                                let built_in = match member.binding {
                                    Some(crate::Binding::BuiltIn(built_in)) => Some(built_in),
                                    _ => None,
                                };
                                let handle = null::generate_default_built_in(
                                    built_in,
                                    member.ty,
                                    &mut module.global_expressions,
                                    span,
                                )?;
                                components.push(handle);
                            }
                            Some(
                                module
                                    .global_expressions
                                    .append(crate::Expression::Compose { ty, components }, span),
                            )
                        }
                        _ => None,
                    },
                };

                let var = crate::GlobalVariable {
                    name: dec.name,
                    space: crate::AddressSpace::Private,
                    binding: None,
                    ty,
                    init,
                    memory_decorations: crate::MemoryDecorations::empty(),
                };
                let inner = Variable::Output(crate::FunctionResult { ty, binding });
                (inner, var)
            }
        };

        let handle = module.global_variables.append(var, span);

        if module.types[ty].inner.can_comparison_sample(module) {
            log::debug!("\t\ttracking {handle:?} for sampling properties");

            self.handle_sampling
                .insert(handle, image::SamplingFlags::empty());
        }

        self.lookup_variable.insert(
            id,
            LookupVariable {
                inner,
                handle,
                type_id,
            },
        );
        Ok(())
    }

    /// Record an atomic access to some component of a global variable.
    ///
    /// Given `handle`, an expression referring to a scalar that has had an
    /// atomic operation applied to it, descend into the expression, noting
    /// which global variable it ultimately refers to, and which struct fields
    /// of that global's value it accesses.
    ///
    /// Return the handle of the type of the expression.
    ///
    /// If the expression doesn't actually refer to something in a global
    /// variable, we can't upgrade its type in a way that Naga validation would
    /// pass, so reject the input instead.
    fn record_atomic_access(
        &mut self,
        ctx: &BlockContext,
        handle: Handle<crate::Expression>,
    ) -> Result<Handle<crate::Type>, Error> {
        log::debug!("\t\tlocating global variable in {handle:?}");
        match ctx.expressions[handle] {
            crate::Expression::Access { base, index } => {
                log::debug!("\t\t  access {handle:?} {index:?}");
                let ty = self.record_atomic_access(ctx, base)?;
                let crate::TypeInner::Array { base, .. } = ctx.module.types[ty].inner else {
                    unreachable!("Atomic operations on Access expressions only work for arrays");
                };
                Ok(base)
            }
            crate::Expression::AccessIndex { base, index } => {
                log::debug!("\t\t  access index {handle:?} {index:?}");
                let ty = self.record_atomic_access(ctx, base)?;
                match ctx.module.types[ty].inner {
                    crate::TypeInner::Struct { ref members, .. } => {
                        let index = index as usize;
                        self.upgrade_atomics.insert_field(ty, index);
                        Ok(members[index].ty)
                    }
                    crate::TypeInner::Array { base, .. } => {
                        Ok(base)
                    }
                    _ => unreachable!("Atomic operations on AccessIndex expressions only work for structs and arrays"),
                }
            }
            crate::Expression::GlobalVariable(h) => {
                log::debug!("\t\t  found {h:?}");
                self.upgrade_atomics.insert_global(h);
                Ok(ctx.module.global_variables[h].ty)
            }
            _ => Err(Error::AtomicUpgradeError(
                crate::front::atomic_upgrade::Error::GlobalVariableMissing,
            )),
        }
    }
}

fn resolve_constant(gctx: crate::proc::GlobalCtx, constant: &Constant) -> Option<u32> {
    let constant = match *constant {
        Constant::Constant(constant) => constant,
        Constant::Override(_) => return None,
    };
    match gctx.global_expressions[gctx.constants[constant].init] {
        crate::Expression::Literal(crate::Literal::U32(id)) => Some(id),
        crate::Expression::Literal(crate::Literal::I32(id)) => Some(id as u32),
        _ => None,
    }
}

pub fn parse_u8_slice(data: &[u8], options: &Options) -> Result<crate::Module, Error> {
    if !data.len().is_multiple_of(4) {
        return Err(Error::IncompleteData);
    }

    let words = data
        .chunks(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()));
    Frontend::new(words, options).parse()
}

/// Helper function to check if `child` is in the scope of `parent`
fn is_parent(mut child: usize, parent: usize, block_ctx: &BlockContext) -> bool {
    loop {
        if child == parent {
            // The child is in the scope parent
            break true;
        } else if child == 0 {
            // Searched finished at the root the child isn't in the parent's body
            break false;
        }

        child = block_ctx.bodies[child].parent;
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    #[test]
    fn parse() {
        let bin = vec![
            // Magic number.           Version number: 1.0.
            0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00,
            // Generator number: 0.    Bound: 0.
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Reserved word: 0.
            0x00, 0x00, 0x00, 0x00, // OpMemoryModel.          Logical.
            0x0e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // GLSL450.
            0x01, 0x00, 0x00, 0x00,
        ];
        let _ = super::parse_u8_slice(&bin, &Default::default()).unwrap();
    }
}
