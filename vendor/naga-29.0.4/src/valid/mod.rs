/*!
Shader validator.
*/

mod analyzer;
mod compose;
mod expression;
mod function;
mod handles;
mod interface;
mod r#type;

use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::ops;

use bit_set::BitSet;

use crate::{
    arena::{Handle, HandleSet},
    proc::{ExpressionKindTracker, LayoutError, Layouter, TypeResolution},
    FastHashSet,
};

//TODO: analyze the model at the same time as we validate it,
// merge the corresponding matches over expressions and statements.

use crate::span::{AddSpan as _, WithSpan};
pub use analyzer::{ExpressionInfo, FunctionInfo, GlobalUse, Uniformity, UniformityRequirements};
pub use compose::ComposeError;
pub use expression::{check_literal_value, LiteralError};
pub use expression::{ConstExpressionError, ExpressionError};
pub use function::{CallError, FunctionError, LocalVariableError, SubgroupError};
pub use interface::{EntryPointError, GlobalVariableError, VaryingError};
pub use r#type::{Disalignment, ImmediateError, TypeError, TypeFlags, WidthError};

use self::handles::InvalidHandleError;

/// Maximum size of a type, in bytes.
pub const MAX_TYPE_SIZE: u32 = 0x4000_0000; // 1GB

bitflags::bitflags! {
    /// Validation flags.
    ///
    /// If you are working with trusted shaders, then you may be able
    /// to save some time by skipping validation.
    ///
    /// If you do not perform full validation, invalid shaders may
    /// cause Naga to panic. If you do perform full validation and
    /// [`Validator::validate`] returns `Ok`, then Naga promises that
    /// code generation will either succeed or return an error; it
    /// should never panic.
    ///
    /// The default value for `ValidationFlags` is
    /// `ValidationFlags::all()`.
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ValidationFlags: u8 {
        /// Expressions.
        const EXPRESSIONS = 0x1;
        /// Statements and blocks of them.
        const BLOCKS = 0x2;
        /// Uniformity of control flow for operations that require it.
        const CONTROL_FLOW_UNIFORMITY = 0x4;
        /// Host-shareable structure layouts.
        const STRUCT_LAYOUTS = 0x8;
        /// Constants.
        const CONSTANTS = 0x10;
        /// Group, binding, and location attributes.
        const BINDINGS = 0x20;
    }
}

impl Default for ValidationFlags {
    fn default() -> Self {
        Self::all()
    }
}

bitflags::bitflags! {
    /// Allowed IR capabilities.
    #[must_use]
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Capabilities: u64 {
        /// Support for [`AddressSpace::Immediate`][1].
        ///
        /// [1]: crate::AddressSpace::Immediate
        const IMMEDIATES = 1 << 0;
        /// Float values with width = 8.
        const FLOAT64 = 1 << 1;
        /// Support for [`BuiltIn::PrimitiveIndex`][1].
        ///
        /// [1]: crate::BuiltIn::PrimitiveIndex
        const PRIMITIVE_INDEX = 1 << 2;
        /// Support for binding arrays of sampled textures and samplers.
        const TEXTURE_AND_SAMPLER_BINDING_ARRAY = 1 << 3;
        /// Support for binding arrays of uniform buffers.
        const BUFFER_BINDING_ARRAY = 1 << 4;
        /// Support for binding arrays of storage textures.
        const STORAGE_TEXTURE_BINDING_ARRAY = 1 << 5;
        /// Support for binding arrays of storage buffers.
        const STORAGE_BUFFER_BINDING_ARRAY = 1 << 6;
        /// Support for [`BuiltIn::ClipDistance`].
        ///
        /// [`BuiltIn::ClipDistance`]: crate::BuiltIn::ClipDistance
        const CLIP_DISTANCE = 1 << 7;
        /// Support for [`BuiltIn::CullDistance`].
        ///
        /// [`BuiltIn::CullDistance`]: crate::BuiltIn::CullDistance
        const CULL_DISTANCE = 1 << 8;
        /// Support for 16-bit normalized storage texture formats.
        const STORAGE_TEXTURE_16BIT_NORM_FORMATS = 1 << 9;
        /// Support for [`BuiltIn::ViewIndex`].
        ///
        /// [`BuiltIn::ViewIndex`]: crate::BuiltIn::ViewIndex
        const MULTIVIEW = 1 << 10;
        /// Support for `early_depth_test`.
        const EARLY_DEPTH_TEST = 1 << 11;
        /// Support for [`BuiltIn::SampleIndex`] and [`Sampling::Sample`].
        ///
        /// [`BuiltIn::SampleIndex`]: crate::BuiltIn::SampleIndex
        /// [`Sampling::Sample`]: crate::Sampling::Sample
        const MULTISAMPLED_SHADING = 1 << 12;
        /// Support for ray queries and acceleration structures.
        const RAY_QUERY = 1 << 13;
        /// Support for generating two sources for blending from fragment shaders.
        const DUAL_SOURCE_BLENDING = 1 << 14;
        /// Support for arrayed cube textures.
        const CUBE_ARRAY_TEXTURES = 1 << 15;
        /// Support for 64-bit signed and unsigned integers.
        const SHADER_INT64 = 1 << 16;
        /// Support for subgroup operations (except barriers) in fragment and compute shaders.
        ///
        /// Subgroup operations in the vertex stage require
        /// [`Capabilities::SUBGROUP_VERTEX_STAGE`] in addition to `Capabilities::SUBGROUP`.
        /// (But note that `create_validator` automatically sets
        /// `Capabilities::SUBGROUP` whenever `Features::SUBGROUP_VERTEX` is
        /// available.)
        ///
        /// Subgroup barriers require [`Capabilities::SUBGROUP_BARRIER`] in addition to
        /// `Capabilities::SUBGROUP`.
        const SUBGROUP = 1 << 17;
        /// Support for subgroup barriers in compute shaders.
        ///
        /// Requires [`Capabilities::SUBGROUP`]. Without it, enables nothing.
        const SUBGROUP_BARRIER = 1 << 18;
        /// Support for subgroup operations (not including barriers) in the vertex stage.
        ///
        /// Without [`Capabilities::SUBGROUP`], enables nothing. (But note that
        /// `create_validator` automatically sets `Capabilities::SUBGROUP`
        /// whenever `Features::SUBGROUP_VERTEX` is available.)
        const SUBGROUP_VERTEX_STAGE = 1 << 19;
        /// Support for [`AtomicFunction::Min`] and [`AtomicFunction::Max`] on
        /// 64-bit integers in the [`Storage`] address space, when the return
        /// value is not used.
        ///
        /// This is the only 64-bit atomic functionality available on Metal 3.1.
        ///
        /// [`AtomicFunction::Min`]: crate::AtomicFunction::Min
        /// [`AtomicFunction::Max`]: crate::AtomicFunction::Max
        /// [`Storage`]: crate::AddressSpace::Storage
        const SHADER_INT64_ATOMIC_MIN_MAX = 1 << 20;
        /// Support for all atomic operations on 64-bit integers.
        const SHADER_INT64_ATOMIC_ALL_OPS = 1 << 21;
        /// Support for [`AtomicFunction::Add`], [`AtomicFunction::Sub`],
        /// and [`AtomicFunction::Exchange { compare: None }`] on 32-bit floating-point numbers
        /// in the [`Storage`] address space.
        ///
        /// [`AtomicFunction::Add`]: crate::AtomicFunction::Add
        /// [`AtomicFunction::Sub`]: crate::AtomicFunction::Sub
        /// [`AtomicFunction::Exchange { compare: None }`]: crate::AtomicFunction::Exchange
        /// [`Storage`]: crate::AddressSpace::Storage
        const SHADER_FLOAT32_ATOMIC = 1 << 22;
        /// Support for atomic operations on images.
        const TEXTURE_ATOMIC = 1 << 23;
        /// Support for atomic operations on 64-bit images.
        const TEXTURE_INT64_ATOMIC = 1 << 24;
        /// Support for ray queries returning vertex position
        const RAY_HIT_VERTEX_POSITION = 1 << 25;
        /// Support for 16-bit floating-point types.
        const SHADER_FLOAT16 = 1 << 26;
        /// Support for [`ImageClass::External`]
        const TEXTURE_EXTERNAL = 1 << 27;
        /// Support for `quantizeToF16`, `pack2x16float`, and `unpack2x16float`, which store
        /// `f16`-precision values in `f32`s.
        const SHADER_FLOAT16_IN_FLOAT32 = 1 << 28;
        /// Support for fragment shader barycentric coordinates.
        const SHADER_BARYCENTRICS = 1 << 29;
        /// Support for task shaders, mesh shaders, and per-primitive fragment inputs
        const MESH_SHADER = 1 << 30;
        /// Support for mesh shaders which output points.
        const MESH_SHADER_POINT_TOPOLOGY = 1 << 31;
        /// Support for non-uniform indexing of binding arrays of sampled textures and samplers.
        const TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 1 << 32;
        /// Support for non-uniform indexing of binding arrays of uniform buffers.
        const BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 1 << 33;
        /// Support for non-uniform indexing of binding arrays of storage textures.
        const STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING = 1 << 34;
        /// Support for non-uniform indexing of binding arrays of storage buffers.
        const STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING = 1 << 35;
        /// Support for cooperative matrix types and operations
        const COOPERATIVE_MATRIX = 1 << 36;
        /// Support for per-vertex fragment input.
        const PER_VERTEX = 1 << 37;
        /// Support for ray generation, any hit, closest hit, and miss shaders.
        const RAY_TRACING_PIPELINE = 1 << 38;
        /// Support for draw index builtin
        const DRAW_INDEX = 1 << 39;
        /// Support for binding arrays of acceleration structures.
        const ACCELERATION_STRUCTURE_BINDING_ARRAY = 1 << 40;
        /// Support for the `@coherent` memory decoration on storage buffers.
        const MEMORY_DECORATION_COHERENT = 1 << 41;
        /// Support for the `@volatile` memory decoration on storage buffers.
        const MEMORY_DECORATION_VOLATILE = 1 << 42;
    }
}

impl Capabilities {
    /// Returns the extension corresponding to this capability, if there is one.
    ///
    /// This is used by integration tests.
    #[cfg(feature = "wgsl-in")]
    #[doc(hidden)]
    pub const fn extension(&self) -> Option<crate::front::wgsl::ImplementedEnableExtension> {
        use crate::front::wgsl::ImplementedEnableExtension as Ext;
        match *self {
            Self::DUAL_SOURCE_BLENDING => Some(Ext::DualSourceBlending),
            // NOTE: `SHADER_FLOAT16_IN_FLOAT32` _does not_ require the `f16` extension
            Self::SHADER_FLOAT16 => Some(Ext::F16),
            Self::CLIP_DISTANCE => Some(Ext::ClipDistances),
            Self::MESH_SHADER => Some(Ext::WgpuMeshShader),
            Self::RAY_QUERY => Some(Ext::WgpuRayQuery),
            Self::RAY_HIT_VERTEX_POSITION => Some(Ext::WgpuRayQueryVertexReturn),
            Self::COOPERATIVE_MATRIX => Some(Ext::WgpuCooperativeMatrix),
            Self::RAY_TRACING_PIPELINE => Some(Ext::WgpuRayTracingPipeline),
            _ => None,
        }
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::MULTISAMPLED_SHADING | Self::CUBE_ARRAY_TEXTURES
    }
}

bitflags::bitflags! {
    /// Supported subgroup operations
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct SubgroupOperationSet: u8 {
        /// Barriers
        // Possibly elections, when that is supported.
        // https://github.com/gfx-rs/wgpu/issues/6042#issuecomment-3272603431
        // Contrary to what the name "basic" suggests, HLSL/DX12 support the
        // other subgroup operations, but do not support subgroup barriers.
        const BASIC = 1 << 0;
        /// Any, All
        const VOTE = 1 << 1;
        /// reductions, scans
        const ARITHMETIC = 1 << 2;
        /// ballot, broadcast
        const BALLOT = 1 << 3;
        /// shuffle, shuffle xor
        const SHUFFLE = 1 << 4;
        /// shuffle up, down
        const SHUFFLE_RELATIVE = 1 << 5;
        // We don't support these operations yet
        // /// Clustered
        // const CLUSTERED = 1 << 6;
        /// Quad supported
        const QUAD_FRAGMENT_COMPUTE = 1 << 7;
        // /// Quad supported in all stages
        // const QUAD_ALL_STAGES = 1 << 8;
    }
}

impl super::SubgroupOperation {
    const fn required_operations(&self) -> SubgroupOperationSet {
        use SubgroupOperationSet as S;
        match *self {
            Self::All | Self::Any => S::VOTE,
            Self::Add | Self::Mul | Self::Min | Self::Max | Self::And | Self::Or | Self::Xor => {
                S::ARITHMETIC
            }
        }
    }
}

impl super::GatherMode {
    const fn required_operations(&self) -> SubgroupOperationSet {
        use SubgroupOperationSet as S;
        match *self {
            Self::BroadcastFirst | Self::Broadcast(_) => S::BALLOT,
            Self::Shuffle(_) | Self::ShuffleXor(_) => S::SHUFFLE,
            Self::ShuffleUp(_) | Self::ShuffleDown(_) => S::SHUFFLE_RELATIVE,
            Self::QuadBroadcast(_) | Self::QuadSwap(_) => S::QUAD_FRAGMENT_COMPUTE,
        }
    }
}

bitflags::bitflags! {
    /// Validation flags.
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ShaderStages: u16 {
        const VERTEX = 0x1;
        const FRAGMENT = 0x2;
        const COMPUTE = 0x4;
        const MESH = 0x8;
        const TASK = 0x10;
        const RAY_GENERATION = 0x20;
        const ANY_HIT = 0x40;
        const CLOSEST_HIT = 0x80;
        const MISS = 0x100;
        const COMPUTE_LIKE = Self::COMPUTE.bits() | Self::TASK.bits() | Self::MESH.bits();
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ModuleInfo {
    type_flags: Vec<TypeFlags>,
    functions: Vec<FunctionInfo>,
    entry_points: Vec<FunctionInfo>,
    const_expression_types: Box<[TypeResolution]>,
}

impl ops::Index<Handle<crate::Type>> for ModuleInfo {
    type Output = TypeFlags;
    fn index(&self, handle: Handle<crate::Type>) -> &Self::Output {
        &self.type_flags[handle.index()]
    }
}

impl ops::Index<Handle<crate::Function>> for ModuleInfo {
    type Output = FunctionInfo;
    fn index(&self, handle: Handle<crate::Function>) -> &Self::Output {
        &self.functions[handle.index()]
    }
}

impl ops::Index<Handle<crate::Expression>> for ModuleInfo {
    type Output = TypeResolution;
    fn index(&self, handle: Handle<crate::Expression>) -> &Self::Output {
        &self.const_expression_types[handle.index()]
    }
}

#[derive(Debug)]
pub struct Validator {
    flags: ValidationFlags,
    capabilities: Capabilities,
    subgroup_stages: ShaderStages,
    subgroup_operations: SubgroupOperationSet,
    types: Vec<r#type::TypeInfo>,
    layouter: Layouter,
    location_mask: BitSet,
    ep_resource_bindings: FastHashSet<crate::ResourceBinding>,
    switch_values: FastHashSet<crate::SwitchValue>,
    valid_expression_list: Vec<Handle<crate::Expression>>,
    valid_expression_set: HandleSet<crate::Expression>,
    override_ids: FastHashSet<u16>,

    /// Treat overrides whose initializers are not fully-evaluated
    /// constant expressions as errors.
    overrides_resolved: bool,

    /// A checklist of expressions that must be visited by a specific kind of
    /// statement.
    ///
    /// For example:
    ///
    /// - [`CallResult`] expressions must be visited by a [`Call`] statement.
    /// - [`AtomicResult`] expressions must be visited by an [`Atomic`] statement.
    ///
    /// Be sure not to remove any [`Expression`] handle from this set unless
    /// you've explicitly checked that it is the right kind of expression for
    /// the visiting [`Statement`].
    ///
    /// [`CallResult`]: crate::Expression::CallResult
    /// [`Call`]: crate::Statement::Call
    /// [`AtomicResult`]: crate::Expression::AtomicResult
    /// [`Atomic`]: crate::Statement::Atomic
    /// [`Expression`]: crate::Expression
    /// [`Statement`]: crate::Statement
    needs_visit: HandleSet<crate::Expression>,

    /// Whether any trace rays call is called, and whether all have vertex return.
    /// If one call doesn't use vertex ruturn, builtins for triangle vertex positions
    /// (not yet implemented) are not allowed.
    trace_rays_vertex_return: TraceRayVertexReturnState,

    /// The type of the ray payload, this must always be the same type in a particular
    /// entrypoint
    trace_rays_payload_type: Option<Handle<crate::Type>>,
}

#[derive(Debug)]
enum TraceRayVertexReturnState {
    /// No trace ray calls yet have been found.
    NoTraceRays,
    /// Trace ray calls have been found, at least
    /// one uses an acceleration structure that
    /// does not have the flag enabling vertex return.
    // Don't yet have vertex return builtins to return.
    // this error for.
    #[expect(unused)]
    NoVertexReturn(crate::Span),
    /// Trace ray calls have been found, all
    /// acceleration structures have the flag enabling
    /// vertex return.
    VertexReturn,
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ConstantError {
    #[error("Initializer must be a const-expression")]
    InitializerExprType,
    #[error("The type doesn't match the constant")]
    InvalidType,
    #[error("The type is not constructible")]
    NonConstructibleType,
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum OverrideError {
    #[error("Override name and ID are missing")]
    MissingNameAndID,
    #[error("Override ID must be unique")]
    DuplicateID,
    #[error("Initializer must be a const-expression or override-expression")]
    InitializerExprType,
    #[error("The type doesn't match the override")]
    InvalidType,
    #[error("The type is not constructible")]
    NonConstructibleType,
    #[error("The type is not a scalar")]
    TypeNotScalar,
    #[error("Override declarations are not allowed")]
    NotAllowed,
    #[error("Override is uninitialized")]
    UninitializedOverride,
    #[error("Constant expression {handle:?} is invalid")]
    ConstExpression {
        handle: Handle<crate::Expression>,
        source: ConstExpressionError,
    },
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ValidationError {
    #[error(transparent)]
    InvalidHandle(#[from] InvalidHandleError),
    #[error(transparent)]
    Layouter(#[from] LayoutError),
    #[error("Type {handle:?} '{name}' is invalid")]
    Type {
        handle: Handle<crate::Type>,
        name: String,
        source: TypeError,
    },
    #[error("Constant expression {handle:?} is invalid")]
    ConstExpression {
        handle: Handle<crate::Expression>,
        source: ConstExpressionError,
    },
    #[error("Array size expression {handle:?} is not strictly positive")]
    ArraySizeError { handle: Handle<crate::Expression> },
    #[error("Constant {handle:?} '{name}' is invalid")]
    Constant {
        handle: Handle<crate::Constant>,
        name: String,
        source: ConstantError,
    },
    #[error("Override {handle:?} '{name}' is invalid")]
    Override {
        handle: Handle<crate::Override>,
        name: String,
        source: OverrideError,
    },
    #[error("Global variable {handle:?} '{name}' is invalid")]
    GlobalVariable {
        handle: Handle<crate::GlobalVariable>,
        name: String,
        source: GlobalVariableError,
    },
    #[error("Function {handle:?} '{name}' is invalid")]
    Function {
        handle: Handle<crate::Function>,
        name: String,
        source: FunctionError,
    },
    #[error("Entry point {name} at {stage:?} is invalid")]
    EntryPoint {
        stage: crate::ShaderStage,
        name: String,
        source: EntryPointError,
    },
    #[error("Module is corrupted")]
    Corrupted,
}

impl crate::TypeInner {
    const fn is_sized(&self) -> bool {
        match *self {
            Self::Scalar { .. }
            | Self::Vector { .. }
            | Self::Matrix { .. }
            | Self::CooperativeMatrix { .. }
            | Self::Array {
                size: crate::ArraySize::Constant(_),
                ..
            }
            | Self::Atomic { .. }
            | Self::Pointer { .. }
            | Self::ValuePointer { .. }
            | Self::Struct { .. } => true,
            Self::Array { .. }
            | Self::Image { .. }
            | Self::Sampler { .. }
            | Self::AccelerationStructure { .. }
            | Self::RayQuery { .. }
            | Self::BindingArray { .. } => false,
        }
    }

    /// Return the `ImageDimension` for which `self` is an appropriate coordinate.
    const fn image_storage_coordinates(&self) -> Option<crate::ImageDimension> {
        match *self {
            Self::Scalar(crate::Scalar {
                kind: crate::ScalarKind::Sint | crate::ScalarKind::Uint,
                ..
            }) => Some(crate::ImageDimension::D1),
            Self::Vector {
                size: crate::VectorSize::Bi,
                scalar:
                    crate::Scalar {
                        kind: crate::ScalarKind::Sint | crate::ScalarKind::Uint,
                        ..
                    },
            } => Some(crate::ImageDimension::D2),
            Self::Vector {
                size: crate::VectorSize::Tri,
                scalar:
                    crate::Scalar {
                        kind: crate::ScalarKind::Sint | crate::ScalarKind::Uint,
                        ..
                    },
            } => Some(crate::ImageDimension::D3),
            _ => None,
        }
    }
}

impl Validator {
    /// Create a validator for Naga [`Module`]s.
    ///
    /// The `flags` argument indicates which stages of validation the
    /// returned `Validator` should perform. Skipping stages can make
    /// validation somewhat faster, but the validator may not reject some
    /// invalid modules. Regardless of `flags`, validation always returns
    /// a usable [`ModuleInfo`] value on success.
    ///
    /// If `flags` contains everything in `ValidationFlags::default()`,
    /// then the returned Naga [`Validator`] will reject any [`Module`]
    /// that would use capabilities not included in `capabilities`.
    ///
    /// [`Module`]: crate::Module
    pub fn new(flags: ValidationFlags, capabilities: Capabilities) -> Self {
        let subgroup_operations = if capabilities.contains(Capabilities::SUBGROUP) {
            use SubgroupOperationSet as S;
            S::BASIC
                | S::VOTE
                | S::ARITHMETIC
                | S::BALLOT
                | S::SHUFFLE
                | S::SHUFFLE_RELATIVE
                | S::QUAD_FRAGMENT_COMPUTE
        } else {
            SubgroupOperationSet::empty()
        };
        let subgroup_stages = {
            let mut stages = ShaderStages::empty();
            if capabilities.contains(Capabilities::SUBGROUP_VERTEX_STAGE) {
                stages |= ShaderStages::VERTEX;
            }
            if capabilities.contains(Capabilities::SUBGROUP) {
                stages |= ShaderStages::FRAGMENT | ShaderStages::COMPUTE_LIKE;
            }
            stages
        };

        Validator {
            flags,
            capabilities,
            subgroup_stages,
            subgroup_operations,
            types: Vec::new(),
            layouter: Layouter::default(),
            location_mask: BitSet::new(),
            ep_resource_bindings: FastHashSet::default(),
            switch_values: FastHashSet::default(),
            valid_expression_list: Vec::new(),
            valid_expression_set: HandleSet::new(),
            override_ids: FastHashSet::default(),
            overrides_resolved: false,
            needs_visit: HandleSet::new(),
            trace_rays_vertex_return: TraceRayVertexReturnState::NoTraceRays,
            trace_rays_payload_type: None,
        }
    }

    // TODO(https://github.com/gfx-rs/wgpu/issues/8207): Consider removing this
    pub const fn subgroup_stages(&mut self, stages: ShaderStages) -> &mut Self {
        self.subgroup_stages = stages;
        self
    }

    // TODO(https://github.com/gfx-rs/wgpu/issues/8207): Consider removing this
    pub const fn subgroup_operations(&mut self, operations: SubgroupOperationSet) -> &mut Self {
        self.subgroup_operations = operations;
        self
    }

    /// Reset the validator internals
    pub fn reset(&mut self) {
        self.types.clear();
        self.layouter.clear();
        self.location_mask.make_empty();
        self.ep_resource_bindings.clear();
        self.switch_values.clear();
        self.valid_expression_list.clear();
        self.valid_expression_set.clear();
        self.override_ids.clear();
    }

    fn validate_constant(
        &self,
        handle: Handle<crate::Constant>,
        gctx: crate::proc::GlobalCtx,
        mod_info: &ModuleInfo,
        global_expr_kind: &ExpressionKindTracker,
    ) -> Result<(), ConstantError> {
        let con = &gctx.constants[handle];

        let type_info = &self.types[con.ty.index()];
        if !type_info.flags.contains(TypeFlags::CONSTRUCTIBLE) {
            return Err(ConstantError::NonConstructibleType);
        }

        if !global_expr_kind.is_const(con.init) {
            return Err(ConstantError::InitializerExprType);
        }

        if !gctx.compare_types(&TypeResolution::Handle(con.ty), &mod_info[con.init]) {
            return Err(ConstantError::InvalidType);
        }

        Ok(())
    }

    fn validate_override(
        &mut self,
        handle: Handle<crate::Override>,
        gctx: crate::proc::GlobalCtx,
        mod_info: &ModuleInfo,
    ) -> Result<(), OverrideError> {
        let o = &gctx.overrides[handle];

        if let Some(id) = o.id {
            if !self.override_ids.insert(id) {
                return Err(OverrideError::DuplicateID);
            }
        }

        let type_info = &self.types[o.ty.index()];
        if !type_info.flags.contains(TypeFlags::CONSTRUCTIBLE) {
            return Err(OverrideError::NonConstructibleType);
        }

        match gctx.types[o.ty].inner {
            crate::TypeInner::Scalar(
                crate::Scalar::BOOL
                | crate::Scalar::I32
                | crate::Scalar::U32
                | crate::Scalar::F16
                | crate::Scalar::F32
                | crate::Scalar::F64,
            ) => {}
            _ => return Err(OverrideError::TypeNotScalar),
        }

        if let Some(init) = o.init {
            if !gctx.compare_types(&TypeResolution::Handle(o.ty), &mod_info[init]) {
                return Err(OverrideError::InvalidType);
            }
        } else if self.overrides_resolved {
            return Err(OverrideError::UninitializedOverride);
        }

        Ok(())
    }

    /// Check the given module to be valid.
    pub fn validate(
        &mut self,
        module: &crate::Module,
    ) -> Result<ModuleInfo, WithSpan<ValidationError>> {
        self.overrides_resolved = false;
        self.validate_impl(module)
    }

    /// Check the given module to be valid, requiring overrides to be resolved.
    ///
    /// This is the same as [`validate`], except that any override
    /// whose value is not a fully-evaluated constant expression is
    /// treated as an error.
    ///
    /// [`validate`]: Validator::validate
    pub fn validate_resolved_overrides(
        &mut self,
        module: &crate::Module,
    ) -> Result<ModuleInfo, WithSpan<ValidationError>> {
        self.overrides_resolved = true;
        self.validate_impl(module)
    }

    fn validate_impl(
        &mut self,
        module: &crate::Module,
    ) -> Result<ModuleInfo, WithSpan<ValidationError>> {
        self.reset();
        self.reset_types(module.types.len());

        Self::validate_module_handles(module).map_err(|e| e.with_span())?;

        self.layouter.update(module.to_ctx()).map_err(|e| {
            let handle = e.ty;
            ValidationError::from(e).with_span_handle(handle, &module.types)
        })?;

        // These should all get overwritten.
        let placeholder = TypeResolution::Value(crate::TypeInner::Scalar(crate::Scalar {
            kind: crate::ScalarKind::Bool,
            width: 0,
        }));

        let mut mod_info = ModuleInfo {
            type_flags: Vec::with_capacity(module.types.len()),
            functions: Vec::with_capacity(module.functions.len()),
            entry_points: Vec::with_capacity(module.entry_points.len()),
            const_expression_types: vec![placeholder; module.global_expressions.len()]
                .into_boxed_slice(),
        };

        for (handle, ty) in module.types.iter() {
            let ty_info = self
                .validate_type(handle, module.to_ctx())
                .map_err(|source| {
                    ValidationError::Type {
                        handle,
                        name: ty.name.clone().unwrap_or_default(),
                        source,
                    }
                    .with_span_handle(handle, &module.types)
                })?;
            debug_assert!(
                ty_info.flags.contains(TypeFlags::CONSTRUCTIBLE)
                    == module.types[handle].inner.is_constructible(&module.types)
            );
            mod_info.type_flags.push(ty_info.flags);
            self.types[handle.index()] = ty_info;
        }

        {
            let t = crate::Arena::new();
            let resolve_context = crate::proc::ResolveContext::with_locals(module, &t, &[]);
            for (handle, _) in module.global_expressions.iter() {
                mod_info
                    .process_const_expression(handle, &resolve_context, module.to_ctx())
                    .map_err(|source| {
                        ValidationError::ConstExpression { handle, source }
                            .with_span_handle(handle, &module.global_expressions)
                    })?
            }
        }

        let global_expr_kind = ExpressionKindTracker::from_arena(&module.global_expressions);

        if self.flags.contains(ValidationFlags::CONSTANTS) {
            for (handle, _) in module.global_expressions.iter() {
                self.validate_const_expression(
                    handle,
                    module.to_ctx(),
                    &mod_info,
                    &global_expr_kind,
                )
                .map_err(|source| {
                    ValidationError::ConstExpression { handle, source }
                        .with_span_handle(handle, &module.global_expressions)
                })?
            }

            for (handle, constant) in module.constants.iter() {
                self.validate_constant(handle, module.to_ctx(), &mod_info, &global_expr_kind)
                    .map_err(|source| {
                        ValidationError::Constant {
                            handle,
                            name: constant.name.clone().unwrap_or_default(),
                            source,
                        }
                        .with_span_handle(handle, &module.constants)
                    })?
            }

            for (handle, r#override) in module.overrides.iter() {
                self.validate_override(handle, module.to_ctx(), &mod_info)
                    .map_err(|source| {
                        ValidationError::Override {
                            handle,
                            name: r#override.name.clone().unwrap_or_default(),
                            source,
                        }
                        .with_span_handle(handle, &module.overrides)
                    })?;
            }
        }

        for (var_handle, var) in module.global_variables.iter() {
            self.validate_global_var(var, module.to_ctx(), &mod_info, &global_expr_kind)
                .map_err(|source| {
                    ValidationError::GlobalVariable {
                        handle: var_handle,
                        name: var.name.clone().unwrap_or_default(),
                        source,
                    }
                    .with_span_handle(var_handle, &module.global_variables)
                })?;
        }

        for (handle, fun) in module.functions.iter() {
            match self.validate_function(fun, module, &mod_info, false) {
                Ok(info) => mod_info.functions.push(info),
                Err(error) => {
                    return Err(error.and_then(|source| {
                        ValidationError::Function {
                            handle,
                            name: fun.name.clone().unwrap_or_default(),
                            source,
                        }
                        .with_span_handle(handle, &module.functions)
                    }))
                }
            }
        }

        let mut ep_map = FastHashSet::default();
        for ep in module.entry_points.iter() {
            if !ep_map.insert((ep.stage, &ep.name)) {
                return Err(ValidationError::EntryPoint {
                    stage: ep.stage,
                    name: ep.name.clone(),
                    source: EntryPointError::Conflict,
                }
                .with_span()); // TODO: keep some EP span information?
            }

            match self.validate_entry_point(ep, module, &mod_info) {
                Ok(info) => mod_info.entry_points.push(info),
                Err(error) => {
                    return Err(error.and_then(|source| {
                        ValidationError::EntryPoint {
                            stage: ep.stage,
                            name: ep.name.clone(),
                            source,
                        }
                        .with_span()
                    }));
                }
            }
        }

        Ok(mod_info)
    }
}

fn validate_atomic_compare_exchange_struct(
    types: &crate::UniqueArena<crate::Type>,
    members: &[crate::StructMember],
    scalar_predicate: impl FnOnce(&crate::TypeInner) -> bool,
) -> bool {
    members.len() == 2
        && members[0].name.as_deref() == Some("old_value")
        && scalar_predicate(&types[members[0].ty].inner)
        && members[1].name.as_deref() == Some("exchanged")
        && types[members[1].ty].inner == crate::TypeInner::Scalar(crate::Scalar::BOOL)
}
