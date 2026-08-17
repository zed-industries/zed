// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

pub type Word = u32;
pub const MAGIC_NUMBER: u32 = 0x07230203;
pub const MAJOR_VERSION: u8 = 1u8;
pub const MINOR_VERSION: u8 = 6u8;
pub const REVISION: u8 = 4u8;
bitflags! { # [doc = "SPIR-V operand kind: [ImageOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_operands_a_image_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct ImageOperands : u32 { const NONE = 0u32 ; const BIAS = 1u32 ; const LOD = 2u32 ; const GRAD = 4u32 ; const CONST_OFFSET = 8u32 ; const OFFSET = 16u32 ; const CONST_OFFSETS = 32u32 ; const SAMPLE = 64u32 ; const MIN_LOD = 128u32 ; const MAKE_TEXEL_AVAILABLE = 256u32 ; const MAKE_TEXEL_VISIBLE = 512u32 ; const NON_PRIVATE_TEXEL = 1024u32 ; const VOLATILE_TEXEL = 2048u32 ; const SIGN_EXTEND = 4096u32 ; const ZERO_EXTEND = 8192u32 ; const NONTEMPORAL = 16384u32 ; const OFFSETS = 65536u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [FPFastMathMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_fast_math_mode_a_fp_fast_math_mode)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct FPFastMathMode : u32 { const NONE = 0u32 ; const NOT_NAN = 1u32 ; const NOT_INF = 2u32 ; const NSZ = 4u32 ; const ALLOW_RECIP = 8u32 ; const FAST = 16u32 ; const ALLOW_CONTRACT = 65536u32 ; const ALLOW_REASSOC = 131072u32 ; const ALLOW_TRANSFORM = 262144u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [SelectionControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_selection_control_a_selection_control)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct SelectionControl : u32 { const NONE = 0u32 ; const FLATTEN = 1u32 ; const DONT_FLATTEN = 2u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [LoopControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_loop_control_a_loop_control)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct LoopControl : u32 { const NONE = 0u32 ; const UNROLL = 1u32 ; const DONT_UNROLL = 2u32 ; const DEPENDENCY_INFINITE = 4u32 ; const DEPENDENCY_LENGTH = 8u32 ; const MIN_ITERATIONS = 16u32 ; const MAX_ITERATIONS = 32u32 ; const ITERATION_MULTIPLE = 64u32 ; const PEEL_COUNT = 128u32 ; const PARTIAL_COUNT = 256u32 ; const INITIATION_INTERVAL_ALTERA = 65536u32 ; const MAX_CONCURRENCY_ALTERA = 131072u32 ; const DEPENDENCY_ARRAY_ALTERA = 262144u32 ; const PIPELINE_ENABLE_ALTERA = 524288u32 ; const LOOP_COALESCE_ALTERA = 1048576u32 ; const MAX_INTERLEAVING_ALTERA = 2097152u32 ; const SPECULATED_ITERATIONS_ALTERA = 4194304u32 ; const NO_FUSION_ALTERA = 8388608u32 ; const LOOP_COUNT_ALTERA = 16777216u32 ; const MAX_REINVOCATION_DELAY_ALTERA = 33554432u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [FunctionControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_function_control_a_function_control)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct FunctionControl : u32 { const NONE = 0u32 ; const INLINE = 1u32 ; const DONT_INLINE = 2u32 ; const PURE = 4u32 ; const CONST = 8u32 ; const OPT_NONE_EXT = 65536u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [MemorySemantics](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_semantics_a_memory_semantics)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct MemorySemantics : u32 { const RELAXED = 0u32 ; const ACQUIRE = 2u32 ; const RELEASE = 4u32 ; const ACQUIRE_RELEASE = 8u32 ; const SEQUENTIALLY_CONSISTENT = 16u32 ; const UNIFORM_MEMORY = 64u32 ; const SUBGROUP_MEMORY = 128u32 ; const WORKGROUP_MEMORY = 256u32 ; const CROSS_WORKGROUP_MEMORY = 512u32 ; const ATOMIC_COUNTER_MEMORY = 1024u32 ; const IMAGE_MEMORY = 2048u32 ; const OUTPUT_MEMORY = 4096u32 ; const MAKE_AVAILABLE = 8192u32 ; const MAKE_VISIBLE = 16384u32 ; const VOLATILE = 32768u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [MemoryAccess](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_access_a_memory_access)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct MemoryAccess : u32 { const NONE = 0u32 ; const VOLATILE = 1u32 ; const ALIGNED = 2u32 ; const NONTEMPORAL = 4u32 ; const MAKE_POINTER_AVAILABLE = 8u32 ; const MAKE_POINTER_VISIBLE = 16u32 ; const NON_PRIVATE_POINTER = 32u32 ; const ALIAS_SCOPE_INTEL_MASK = 65536u32 ; const NO_ALIAS_INTEL_MASK = 131072u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [KernelProfilingInfo](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_kernel_profiling_info_a_kernel_profiling_info)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct KernelProfilingInfo : u32 { const NONE = 0u32 ; const CMD_EXEC_TIME = 1u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [RayFlags](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_ray_flags_a_ray_flags)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct RayFlags : u32 { const NONE_KHR = 0u32 ; const OPAQUE_KHR = 1u32 ; const NO_OPAQUE_KHR = 2u32 ; const TERMINATE_ON_FIRST_HIT_KHR = 4u32 ; const SKIP_CLOSEST_HIT_SHADER_KHR = 8u32 ; const CULL_BACK_FACING_TRIANGLES_KHR = 16u32 ; const CULL_FRONT_FACING_TRIANGLES_KHR = 32u32 ; const CULL_OPAQUE_KHR = 64u32 ; const CULL_NO_OPAQUE_KHR = 128u32 ; const SKIP_TRIANGLES_KHR = 256u32 ; const SKIP_AAB_BS_KHR = 512u32 ; const FORCE_OPACITY_MICROMAP2_STATE_EXT = 1024u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [FragmentShadingRate](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fragment_shading_rate_a_fragment_shading_rate)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct FragmentShadingRate : u32 { const VERTICAL2_PIXELS = 1u32 ; const VERTICAL4_PIXELS = 2u32 ; const HORIZONTAL2_PIXELS = 4u32 ; const HORIZONTAL4_PIXELS = 8u32 ; } }
bitflags! { # [doc = "SPIR-V operand kind: [RawAccessChainOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_raw_access_chain_operands_a_raw_access_chain_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct RawAccessChainOperands : u32 { const NONE = 0u32 ; const ROBUSTNESS_PER_COMPONENT_NV = 1u32 ; const ROBUSTNESS_PER_ELEMENT_NV = 2u32 ; } }
#[doc = "SPIR-V operand kind: [SourceLanguage](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_source_language_a_source_language)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SourceLanguage {
    Unknown = 0u32,
    ESSL = 1u32,
    GLSL = 2u32,
    OpenCL_C = 3u32,
    OpenCL_CPP = 4u32,
    HLSL = 5u32,
    CPP_for_OpenCL = 6u32,
    SYCL = 7u32,
    HERO_C = 8u32,
    NZSL = 9u32,
    WGSL = 10u32,
    Slang = 11u32,
    Zig = 12u32,
    Rust = 13u32,
}
impl SourceLanguage {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=13u32 => unsafe { core::mem::transmute::<u32, SourceLanguage>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl SourceLanguage {}
impl core::str::FromStr for SourceLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Unknown" => Self::Unknown,
            "ESSL" => Self::ESSL,
            "GLSL" => Self::GLSL,
            "OpenCL_C" => Self::OpenCL_C,
            "OpenCL_CPP" => Self::OpenCL_CPP,
            "HLSL" => Self::HLSL,
            "CPP_for_OpenCL" => Self::CPP_for_OpenCL,
            "SYCL" => Self::SYCL,
            "HERO_C" => Self::HERO_C,
            "NZSL" => Self::NZSL,
            "WGSL" => Self::WGSL,
            "Slang" => Self::Slang,
            "Zig" => Self::Zig,
            "Rust" => Self::Rust,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ExecutionModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_execution_model_a_execution_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ExecutionModel {
    Vertex = 0u32,
    TessellationControl = 1u32,
    TessellationEvaluation = 2u32,
    Geometry = 3u32,
    Fragment = 4u32,
    GLCompute = 5u32,
    Kernel = 6u32,
    TaskNV = 5267u32,
    MeshNV = 5268u32,
    RayGenerationKHR = 5313u32,
    IntersectionKHR = 5314u32,
    AnyHitKHR = 5315u32,
    ClosestHitKHR = 5316u32,
    MissKHR = 5317u32,
    CallableKHR = 5318u32,
    TaskEXT = 5364u32,
    MeshEXT = 5365u32,
}
impl ExecutionModel {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=6u32 => unsafe { core::mem::transmute::<u32, ExecutionModel>(n) },
            5267u32..=5268u32 => unsafe { core::mem::transmute::<u32, ExecutionModel>(n) },
            5313u32..=5318u32 => unsafe { core::mem::transmute::<u32, ExecutionModel>(n) },
            5364u32..=5365u32 => unsafe { core::mem::transmute::<u32, ExecutionModel>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ExecutionModel {
    pub const RayGenerationNV: Self = Self::RayGenerationKHR;
    pub const IntersectionNV: Self = Self::IntersectionKHR;
    pub const AnyHitNV: Self = Self::AnyHitKHR;
    pub const ClosestHitNV: Self = Self::ClosestHitKHR;
    pub const MissNV: Self = Self::MissKHR;
    pub const CallableNV: Self = Self::CallableKHR;
}
impl core::str::FromStr for ExecutionModel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Vertex" => Self::Vertex,
            "TessellationControl" => Self::TessellationControl,
            "TessellationEvaluation" => Self::TessellationEvaluation,
            "Geometry" => Self::Geometry,
            "Fragment" => Self::Fragment,
            "GLCompute" => Self::GLCompute,
            "Kernel" => Self::Kernel,
            "TaskNV" => Self::TaskNV,
            "MeshNV" => Self::MeshNV,
            "RayGenerationKHR" => Self::RayGenerationKHR,
            "RayGenerationNV" => Self::RayGenerationKHR,
            "IntersectionKHR" => Self::IntersectionKHR,
            "IntersectionNV" => Self::IntersectionKHR,
            "AnyHitKHR" => Self::AnyHitKHR,
            "AnyHitNV" => Self::AnyHitKHR,
            "ClosestHitKHR" => Self::ClosestHitKHR,
            "ClosestHitNV" => Self::ClosestHitKHR,
            "MissKHR" => Self::MissKHR,
            "MissNV" => Self::MissKHR,
            "CallableKHR" => Self::CallableKHR,
            "CallableNV" => Self::CallableKHR,
            "TaskEXT" => Self::TaskEXT,
            "MeshEXT" => Self::MeshEXT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [AddressingModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_addressing_model_a_addressing_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum AddressingModel {
    Logical = 0u32,
    Physical32 = 1u32,
    Physical64 = 2u32,
    PhysicalStorageBuffer64 = 5348u32,
}
impl AddressingModel {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, AddressingModel>(n) },
            5348u32 => unsafe { core::mem::transmute::<u32, AddressingModel>(5348u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl AddressingModel {
    pub const PhysicalStorageBuffer64EXT: Self = Self::PhysicalStorageBuffer64;
}
impl core::str::FromStr for AddressingModel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Logical" => Self::Logical,
            "Physical32" => Self::Physical32,
            "Physical64" => Self::Physical64,
            "PhysicalStorageBuffer64" => Self::PhysicalStorageBuffer64,
            "PhysicalStorageBuffer64EXT" => Self::PhysicalStorageBuffer64,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [MemoryModel](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_memory_model_a_memory_model)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum MemoryModel {
    Simple = 0u32,
    GLSL450 = 1u32,
    OpenCL = 2u32,
    Vulkan = 3u32,
}
impl MemoryModel {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, MemoryModel>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl MemoryModel {
    pub const VulkanKHR: Self = Self::Vulkan;
}
impl core::str::FromStr for MemoryModel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Simple" => Self::Simple,
            "GLSL450" => Self::GLSL450,
            "OpenCL" => Self::OpenCL,
            "Vulkan" => Self::Vulkan,
            "VulkanKHR" => Self::Vulkan,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ExecutionMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_execution_mode_a_execution_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ExecutionMode {
    Invocations = 0u32,
    SpacingEqual = 1u32,
    SpacingFractionalEven = 2u32,
    SpacingFractionalOdd = 3u32,
    VertexOrderCw = 4u32,
    VertexOrderCcw = 5u32,
    PixelCenterInteger = 6u32,
    OriginUpperLeft = 7u32,
    OriginLowerLeft = 8u32,
    EarlyFragmentTests = 9u32,
    PointMode = 10u32,
    Xfb = 11u32,
    DepthReplacing = 12u32,
    DepthGreater = 14u32,
    DepthLess = 15u32,
    DepthUnchanged = 16u32,
    LocalSize = 17u32,
    LocalSizeHint = 18u32,
    InputPoints = 19u32,
    InputLines = 20u32,
    InputLinesAdjacency = 21u32,
    Triangles = 22u32,
    InputTrianglesAdjacency = 23u32,
    Quads = 24u32,
    Isolines = 25u32,
    OutputVertices = 26u32,
    OutputPoints = 27u32,
    OutputLineStrip = 28u32,
    OutputTriangleStrip = 29u32,
    VecTypeHint = 30u32,
    ContractionOff = 31u32,
    Initializer = 33u32,
    Finalizer = 34u32,
    SubgroupSize = 35u32,
    SubgroupsPerWorkgroup = 36u32,
    SubgroupsPerWorkgroupId = 37u32,
    LocalSizeId = 38u32,
    LocalSizeHintId = 39u32,
    NonCoherentColorAttachmentReadEXT = 4169u32,
    NonCoherentDepthAttachmentReadEXT = 4170u32,
    NonCoherentStencilAttachmentReadEXT = 4171u32,
    SubgroupUniformControlFlowKHR = 4421u32,
    PostDepthCoverage = 4446u32,
    DenormPreserve = 4459u32,
    DenormFlushToZero = 4460u32,
    SignedZeroInfNanPreserve = 4461u32,
    RoundingModeRTE = 4462u32,
    RoundingModeRTZ = 4463u32,
    NonCoherentTileAttachmentReadQCOM = 4489u32,
    TileShadingRateQCOM = 4490u32,
    EarlyAndLateFragmentTestsAMD = 5017u32,
    StencilRefReplacingEXT = 5027u32,
    CoalescingAMDX = 5069u32,
    IsApiEntryAMDX = 5070u32,
    MaxNodeRecursionAMDX = 5071u32,
    StaticNumWorkgroupsAMDX = 5072u32,
    ShaderIndexAMDX = 5073u32,
    MaxNumWorkgroupsAMDX = 5077u32,
    StencilRefUnchangedFrontAMD = 5079u32,
    StencilRefGreaterFrontAMD = 5080u32,
    StencilRefLessFrontAMD = 5081u32,
    StencilRefUnchangedBackAMD = 5082u32,
    StencilRefGreaterBackAMD = 5083u32,
    StencilRefLessBackAMD = 5084u32,
    QuadDerivativesKHR = 5088u32,
    RequireFullQuadsKHR = 5089u32,
    SharesInputWithAMDX = 5102u32,
    OutputLinesEXT = 5269u32,
    OutputPrimitivesEXT = 5270u32,
    DerivativeGroupQuadsKHR = 5289u32,
    DerivativeGroupLinearKHR = 5290u32,
    OutputTrianglesEXT = 5298u32,
    PixelInterlockOrderedEXT = 5366u32,
    PixelInterlockUnorderedEXT = 5367u32,
    SampleInterlockOrderedEXT = 5368u32,
    SampleInterlockUnorderedEXT = 5369u32,
    ShadingRateInterlockOrderedEXT = 5370u32,
    ShadingRateInterlockUnorderedEXT = 5371u32,
    Shader64BitIndexingEXT = 5427u32,
    SharedLocalMemorySizeINTEL = 5618u32,
    RoundingModeRTPINTEL = 5620u32,
    RoundingModeRTNINTEL = 5621u32,
    FloatingPointModeALTINTEL = 5622u32,
    FloatingPointModeIEEEINTEL = 5623u32,
    MaxWorkgroupSizeINTEL = 5893u32,
    MaxWorkDimINTEL = 5894u32,
    NoGlobalOffsetINTEL = 5895u32,
    NumSIMDWorkitemsINTEL = 5896u32,
    SchedulerTargetFmaxMhzINTEL = 5903u32,
    MaximallyReconvergesKHR = 6023u32,
    FPFastMathDefault = 6028u32,
    StreamingInterfaceINTEL = 6154u32,
    RegisterMapInterfaceINTEL = 6160u32,
    NamedBarrierCountINTEL = 6417u32,
    MaximumRegistersINTEL = 6461u32,
    MaximumRegistersIdINTEL = 6462u32,
    NamedMaximumRegistersINTEL = 6463u32,
}
impl ExecutionMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=12u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            14u32..=31u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            33u32..=39u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            4169u32..=4171u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            4421u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(4421u32) },
            4446u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(4446u32) },
            4459u32..=4463u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            4489u32..=4490u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5017u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5017u32) },
            5027u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5027u32) },
            5069u32..=5073u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5077u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5077u32) },
            5079u32..=5084u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5088u32..=5089u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5102u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5102u32) },
            5269u32..=5270u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5289u32..=5290u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5298u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5298u32) },
            5366u32..=5371u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5427u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5427u32) },
            5618u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5618u32) },
            5620u32..=5623u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5893u32..=5896u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            5903u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(5903u32) },
            6023u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(6023u32) },
            6028u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(6028u32) },
            6154u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(6154u32) },
            6160u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(6160u32) },
            6417u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(6417u32) },
            6461u32..=6463u32 => unsafe { core::mem::transmute::<u32, ExecutionMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ExecutionMode {
    pub const OutputLinesNV: Self = Self::OutputLinesEXT;
    pub const OutputPrimitivesNV: Self = Self::OutputPrimitivesEXT;
    pub const DerivativeGroupQuadsNV: Self = Self::DerivativeGroupQuadsKHR;
    pub const DerivativeGroupLinearNV: Self = Self::DerivativeGroupLinearKHR;
    pub const OutputTrianglesNV: Self = Self::OutputTrianglesEXT;
}
impl core::str::FromStr for ExecutionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Invocations" => Self::Invocations,
            "SpacingEqual" => Self::SpacingEqual,
            "SpacingFractionalEven" => Self::SpacingFractionalEven,
            "SpacingFractionalOdd" => Self::SpacingFractionalOdd,
            "VertexOrderCw" => Self::VertexOrderCw,
            "VertexOrderCcw" => Self::VertexOrderCcw,
            "PixelCenterInteger" => Self::PixelCenterInteger,
            "OriginUpperLeft" => Self::OriginUpperLeft,
            "OriginLowerLeft" => Self::OriginLowerLeft,
            "EarlyFragmentTests" => Self::EarlyFragmentTests,
            "PointMode" => Self::PointMode,
            "Xfb" => Self::Xfb,
            "DepthReplacing" => Self::DepthReplacing,
            "DepthGreater" => Self::DepthGreater,
            "DepthLess" => Self::DepthLess,
            "DepthUnchanged" => Self::DepthUnchanged,
            "LocalSize" => Self::LocalSize,
            "LocalSizeHint" => Self::LocalSizeHint,
            "InputPoints" => Self::InputPoints,
            "InputLines" => Self::InputLines,
            "InputLinesAdjacency" => Self::InputLinesAdjacency,
            "Triangles" => Self::Triangles,
            "InputTrianglesAdjacency" => Self::InputTrianglesAdjacency,
            "Quads" => Self::Quads,
            "Isolines" => Self::Isolines,
            "OutputVertices" => Self::OutputVertices,
            "OutputPoints" => Self::OutputPoints,
            "OutputLineStrip" => Self::OutputLineStrip,
            "OutputTriangleStrip" => Self::OutputTriangleStrip,
            "VecTypeHint" => Self::VecTypeHint,
            "ContractionOff" => Self::ContractionOff,
            "Initializer" => Self::Initializer,
            "Finalizer" => Self::Finalizer,
            "SubgroupSize" => Self::SubgroupSize,
            "SubgroupsPerWorkgroup" => Self::SubgroupsPerWorkgroup,
            "SubgroupsPerWorkgroupId" => Self::SubgroupsPerWorkgroupId,
            "LocalSizeId" => Self::LocalSizeId,
            "LocalSizeHintId" => Self::LocalSizeHintId,
            "NonCoherentColorAttachmentReadEXT" => Self::NonCoherentColorAttachmentReadEXT,
            "NonCoherentDepthAttachmentReadEXT" => Self::NonCoherentDepthAttachmentReadEXT,
            "NonCoherentStencilAttachmentReadEXT" => Self::NonCoherentStencilAttachmentReadEXT,
            "SubgroupUniformControlFlowKHR" => Self::SubgroupUniformControlFlowKHR,
            "PostDepthCoverage" => Self::PostDepthCoverage,
            "DenormPreserve" => Self::DenormPreserve,
            "DenormFlushToZero" => Self::DenormFlushToZero,
            "SignedZeroInfNanPreserve" => Self::SignedZeroInfNanPreserve,
            "RoundingModeRTE" => Self::RoundingModeRTE,
            "RoundingModeRTZ" => Self::RoundingModeRTZ,
            "NonCoherentTileAttachmentReadQCOM" => Self::NonCoherentTileAttachmentReadQCOM,
            "TileShadingRateQCOM" => Self::TileShadingRateQCOM,
            "EarlyAndLateFragmentTestsAMD" => Self::EarlyAndLateFragmentTestsAMD,
            "StencilRefReplacingEXT" => Self::StencilRefReplacingEXT,
            "CoalescingAMDX" => Self::CoalescingAMDX,
            "IsApiEntryAMDX" => Self::IsApiEntryAMDX,
            "MaxNodeRecursionAMDX" => Self::MaxNodeRecursionAMDX,
            "StaticNumWorkgroupsAMDX" => Self::StaticNumWorkgroupsAMDX,
            "ShaderIndexAMDX" => Self::ShaderIndexAMDX,
            "MaxNumWorkgroupsAMDX" => Self::MaxNumWorkgroupsAMDX,
            "StencilRefUnchangedFrontAMD" => Self::StencilRefUnchangedFrontAMD,
            "StencilRefGreaterFrontAMD" => Self::StencilRefGreaterFrontAMD,
            "StencilRefLessFrontAMD" => Self::StencilRefLessFrontAMD,
            "StencilRefUnchangedBackAMD" => Self::StencilRefUnchangedBackAMD,
            "StencilRefGreaterBackAMD" => Self::StencilRefGreaterBackAMD,
            "StencilRefLessBackAMD" => Self::StencilRefLessBackAMD,
            "QuadDerivativesKHR" => Self::QuadDerivativesKHR,
            "RequireFullQuadsKHR" => Self::RequireFullQuadsKHR,
            "SharesInputWithAMDX" => Self::SharesInputWithAMDX,
            "OutputLinesEXT" => Self::OutputLinesEXT,
            "OutputLinesNV" => Self::OutputLinesEXT,
            "OutputPrimitivesEXT" => Self::OutputPrimitivesEXT,
            "OutputPrimitivesNV" => Self::OutputPrimitivesEXT,
            "DerivativeGroupQuadsKHR" => Self::DerivativeGroupQuadsKHR,
            "DerivativeGroupQuadsNV" => Self::DerivativeGroupQuadsKHR,
            "DerivativeGroupLinearKHR" => Self::DerivativeGroupLinearKHR,
            "DerivativeGroupLinearNV" => Self::DerivativeGroupLinearKHR,
            "OutputTrianglesEXT" => Self::OutputTrianglesEXT,
            "OutputTrianglesNV" => Self::OutputTrianglesEXT,
            "PixelInterlockOrderedEXT" => Self::PixelInterlockOrderedEXT,
            "PixelInterlockUnorderedEXT" => Self::PixelInterlockUnorderedEXT,
            "SampleInterlockOrderedEXT" => Self::SampleInterlockOrderedEXT,
            "SampleInterlockUnorderedEXT" => Self::SampleInterlockUnorderedEXT,
            "ShadingRateInterlockOrderedEXT" => Self::ShadingRateInterlockOrderedEXT,
            "ShadingRateInterlockUnorderedEXT" => Self::ShadingRateInterlockUnorderedEXT,
            "Shader64BitIndexingEXT" => Self::Shader64BitIndexingEXT,
            "SharedLocalMemorySizeINTEL" => Self::SharedLocalMemorySizeINTEL,
            "RoundingModeRTPINTEL" => Self::RoundingModeRTPINTEL,
            "RoundingModeRTNINTEL" => Self::RoundingModeRTNINTEL,
            "FloatingPointModeALTINTEL" => Self::FloatingPointModeALTINTEL,
            "FloatingPointModeIEEEINTEL" => Self::FloatingPointModeIEEEINTEL,
            "MaxWorkgroupSizeINTEL" => Self::MaxWorkgroupSizeINTEL,
            "MaxWorkDimINTEL" => Self::MaxWorkDimINTEL,
            "NoGlobalOffsetINTEL" => Self::NoGlobalOffsetINTEL,
            "NumSIMDWorkitemsINTEL" => Self::NumSIMDWorkitemsINTEL,
            "SchedulerTargetFmaxMhzINTEL" => Self::SchedulerTargetFmaxMhzINTEL,
            "MaximallyReconvergesKHR" => Self::MaximallyReconvergesKHR,
            "FPFastMathDefault" => Self::FPFastMathDefault,
            "StreamingInterfaceINTEL" => Self::StreamingInterfaceINTEL,
            "RegisterMapInterfaceINTEL" => Self::RegisterMapInterfaceINTEL,
            "NamedBarrierCountINTEL" => Self::NamedBarrierCountINTEL,
            "MaximumRegistersINTEL" => Self::MaximumRegistersINTEL,
            "MaximumRegistersIdINTEL" => Self::MaximumRegistersIdINTEL,
            "NamedMaximumRegistersINTEL" => Self::NamedMaximumRegistersINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [StorageClass](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_storage_class_a_storage_class)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum StorageClass {
    UniformConstant = 0u32,
    Input = 1u32,
    Uniform = 2u32,
    Output = 3u32,
    Workgroup = 4u32,
    CrossWorkgroup = 5u32,
    Private = 6u32,
    Function = 7u32,
    Generic = 8u32,
    PushConstant = 9u32,
    AtomicCounter = 10u32,
    Image = 11u32,
    StorageBuffer = 12u32,
    TileImageEXT = 4172u32,
    TileAttachmentQCOM = 4491u32,
    NodePayloadAMDX = 5068u32,
    CallableDataKHR = 5328u32,
    IncomingCallableDataKHR = 5329u32,
    RayPayloadKHR = 5338u32,
    HitAttributeKHR = 5339u32,
    IncomingRayPayloadKHR = 5342u32,
    ShaderRecordBufferKHR = 5343u32,
    PhysicalStorageBuffer = 5349u32,
    HitObjectAttributeNV = 5385u32,
    TaskPayloadWorkgroupEXT = 5402u32,
    HitObjectAttributeEXT = 5411u32,
    CodeSectionINTEL = 5605u32,
    DeviceOnlyALTERA = 5936u32,
    HostOnlyALTERA = 5937u32,
}
impl StorageClass {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=12u32 => unsafe { core::mem::transmute::<u32, StorageClass>(n) },
            4172u32 => unsafe { core::mem::transmute::<u32, StorageClass>(4172u32) },
            4491u32 => unsafe { core::mem::transmute::<u32, StorageClass>(4491u32) },
            5068u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5068u32) },
            5328u32..=5329u32 => unsafe { core::mem::transmute::<u32, StorageClass>(n) },
            5338u32..=5339u32 => unsafe { core::mem::transmute::<u32, StorageClass>(n) },
            5342u32..=5343u32 => unsafe { core::mem::transmute::<u32, StorageClass>(n) },
            5349u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5349u32) },
            5385u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5385u32) },
            5402u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5402u32) },
            5411u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5411u32) },
            5605u32 => unsafe { core::mem::transmute::<u32, StorageClass>(5605u32) },
            5936u32..=5937u32 => unsafe { core::mem::transmute::<u32, StorageClass>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl StorageClass {
    pub const CallableDataNV: Self = Self::CallableDataKHR;
    pub const IncomingCallableDataNV: Self = Self::IncomingCallableDataKHR;
    pub const RayPayloadNV: Self = Self::RayPayloadKHR;
    pub const HitAttributeNV: Self = Self::HitAttributeKHR;
    pub const IncomingRayPayloadNV: Self = Self::IncomingRayPayloadKHR;
    pub const ShaderRecordBufferNV: Self = Self::ShaderRecordBufferKHR;
    pub const PhysicalStorageBufferEXT: Self = Self::PhysicalStorageBuffer;
    pub const DeviceOnlyINTEL: Self = Self::DeviceOnlyALTERA;
    pub const HostOnlyINTEL: Self = Self::HostOnlyALTERA;
}
impl core::str::FromStr for StorageClass {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UniformConstant" => Self::UniformConstant,
            "Input" => Self::Input,
            "Uniform" => Self::Uniform,
            "Output" => Self::Output,
            "Workgroup" => Self::Workgroup,
            "CrossWorkgroup" => Self::CrossWorkgroup,
            "Private" => Self::Private,
            "Function" => Self::Function,
            "Generic" => Self::Generic,
            "PushConstant" => Self::PushConstant,
            "AtomicCounter" => Self::AtomicCounter,
            "Image" => Self::Image,
            "StorageBuffer" => Self::StorageBuffer,
            "TileImageEXT" => Self::TileImageEXT,
            "TileAttachmentQCOM" => Self::TileAttachmentQCOM,
            "NodePayloadAMDX" => Self::NodePayloadAMDX,
            "CallableDataKHR" => Self::CallableDataKHR,
            "CallableDataNV" => Self::CallableDataKHR,
            "IncomingCallableDataKHR" => Self::IncomingCallableDataKHR,
            "IncomingCallableDataNV" => Self::IncomingCallableDataKHR,
            "RayPayloadKHR" => Self::RayPayloadKHR,
            "RayPayloadNV" => Self::RayPayloadKHR,
            "HitAttributeKHR" => Self::HitAttributeKHR,
            "HitAttributeNV" => Self::HitAttributeKHR,
            "IncomingRayPayloadKHR" => Self::IncomingRayPayloadKHR,
            "IncomingRayPayloadNV" => Self::IncomingRayPayloadKHR,
            "ShaderRecordBufferKHR" => Self::ShaderRecordBufferKHR,
            "ShaderRecordBufferNV" => Self::ShaderRecordBufferKHR,
            "PhysicalStorageBuffer" => Self::PhysicalStorageBuffer,
            "PhysicalStorageBufferEXT" => Self::PhysicalStorageBuffer,
            "HitObjectAttributeNV" => Self::HitObjectAttributeNV,
            "TaskPayloadWorkgroupEXT" => Self::TaskPayloadWorkgroupEXT,
            "HitObjectAttributeEXT" => Self::HitObjectAttributeEXT,
            "CodeSectionINTEL" => Self::CodeSectionINTEL,
            "DeviceOnlyALTERA" => Self::DeviceOnlyALTERA,
            "DeviceOnlyINTEL" => Self::DeviceOnlyALTERA,
            "HostOnlyALTERA" => Self::HostOnlyALTERA,
            "HostOnlyINTEL" => Self::HostOnlyALTERA,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [Dim](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_dim_a_dim)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Dim {
    Dim1D = 0u32,
    Dim2D = 1u32,
    Dim3D = 2u32,
    DimCube = 3u32,
    DimRect = 4u32,
    DimBuffer = 5u32,
    DimSubpassData = 6u32,
    DimTileImageDataEXT = 4173u32,
}
impl Dim {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=6u32 => unsafe { core::mem::transmute::<u32, Dim>(n) },
            4173u32 => unsafe { core::mem::transmute::<u32, Dim>(4173u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl Dim {}
impl core::str::FromStr for Dim {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Dim1D" => Self::Dim1D,
            "Dim2D" => Self::Dim2D,
            "Dim3D" => Self::Dim3D,
            "DimCube" => Self::DimCube,
            "DimRect" => Self::DimRect,
            "DimBuffer" => Self::DimBuffer,
            "DimSubpassData" => Self::DimSubpassData,
            "DimTileImageDataEXT" => Self::DimTileImageDataEXT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [SamplerAddressingMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_sampler_addressing_mode_a_sampler_addressing_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SamplerAddressingMode {
    None = 0u32,
    ClampToEdge = 1u32,
    Clamp = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
impl SamplerAddressingMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=4u32 => unsafe { core::mem::transmute::<u32, SamplerAddressingMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl SamplerAddressingMode {}
impl core::str::FromStr for SamplerAddressingMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "None" => Self::None,
            "ClampToEdge" => Self::ClampToEdge,
            "Clamp" => Self::Clamp,
            "Repeat" => Self::Repeat,
            "RepeatMirrored" => Self::RepeatMirrored,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [SamplerFilterMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_sampler_filter_mode_a_sampler_filter_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SamplerFilterMode {
    Nearest = 0u32,
    Linear = 1u32,
}
impl SamplerFilterMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, SamplerFilterMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl SamplerFilterMode {}
impl core::str::FromStr for SamplerFilterMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Nearest" => Self::Nearest,
            "Linear" => Self::Linear,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ImageFormat](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_format_a_image_format)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ImageFormat {
    Unknown = 0u32,
    Rgba32f = 1u32,
    Rgba16f = 2u32,
    R32f = 3u32,
    Rgba8 = 4u32,
    Rgba8Snorm = 5u32,
    Rg32f = 6u32,
    Rg16f = 7u32,
    R11fG11fB10f = 8u32,
    R16f = 9u32,
    Rgba16 = 10u32,
    Rgb10A2 = 11u32,
    Rg16 = 12u32,
    Rg8 = 13u32,
    R16 = 14u32,
    R8 = 15u32,
    Rgba16Snorm = 16u32,
    Rg16Snorm = 17u32,
    Rg8Snorm = 18u32,
    R16Snorm = 19u32,
    R8Snorm = 20u32,
    Rgba32i = 21u32,
    Rgba16i = 22u32,
    Rgba8i = 23u32,
    R32i = 24u32,
    Rg32i = 25u32,
    Rg16i = 26u32,
    Rg8i = 27u32,
    R16i = 28u32,
    R8i = 29u32,
    Rgba32ui = 30u32,
    Rgba16ui = 31u32,
    Rgba8ui = 32u32,
    R32ui = 33u32,
    Rgb10a2ui = 34u32,
    Rg32ui = 35u32,
    Rg16ui = 36u32,
    Rg8ui = 37u32,
    R16ui = 38u32,
    R8ui = 39u32,
    R64ui = 40u32,
    R64i = 41u32,
}
impl ImageFormat {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=41u32 => unsafe { core::mem::transmute::<u32, ImageFormat>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ImageFormat {}
impl core::str::FromStr for ImageFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Unknown" => Self::Unknown,
            "Rgba32f" => Self::Rgba32f,
            "Rgba16f" => Self::Rgba16f,
            "R32f" => Self::R32f,
            "Rgba8" => Self::Rgba8,
            "Rgba8Snorm" => Self::Rgba8Snorm,
            "Rg32f" => Self::Rg32f,
            "Rg16f" => Self::Rg16f,
            "R11fG11fB10f" => Self::R11fG11fB10f,
            "R16f" => Self::R16f,
            "Rgba16" => Self::Rgba16,
            "Rgb10A2" => Self::Rgb10A2,
            "Rg16" => Self::Rg16,
            "Rg8" => Self::Rg8,
            "R16" => Self::R16,
            "R8" => Self::R8,
            "Rgba16Snorm" => Self::Rgba16Snorm,
            "Rg16Snorm" => Self::Rg16Snorm,
            "Rg8Snorm" => Self::Rg8Snorm,
            "R16Snorm" => Self::R16Snorm,
            "R8Snorm" => Self::R8Snorm,
            "Rgba32i" => Self::Rgba32i,
            "Rgba16i" => Self::Rgba16i,
            "Rgba8i" => Self::Rgba8i,
            "R32i" => Self::R32i,
            "Rg32i" => Self::Rg32i,
            "Rg16i" => Self::Rg16i,
            "Rg8i" => Self::Rg8i,
            "R16i" => Self::R16i,
            "R8i" => Self::R8i,
            "Rgba32ui" => Self::Rgba32ui,
            "Rgba16ui" => Self::Rgba16ui,
            "Rgba8ui" => Self::Rgba8ui,
            "R32ui" => Self::R32ui,
            "Rgb10a2ui" => Self::Rgb10a2ui,
            "Rg32ui" => Self::Rg32ui,
            "Rg16ui" => Self::Rg16ui,
            "Rg8ui" => Self::Rg8ui,
            "R16ui" => Self::R16ui,
            "R8ui" => Self::R8ui,
            "R64ui" => Self::R64ui,
            "R64i" => Self::R64i,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ImageChannelOrder](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_channel_order_a_image_channel_order)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ImageChannelOrder {
    R = 0u32,
    A = 1u32,
    RG = 2u32,
    RA = 3u32,
    RGB = 4u32,
    RGBA = 5u32,
    BGRA = 6u32,
    ARGB = 7u32,
    Intensity = 8u32,
    Luminance = 9u32,
    Rx = 10u32,
    RGx = 11u32,
    RGBx = 12u32,
    Depth = 13u32,
    DepthStencil = 14u32,
    sRGB = 15u32,
    sRGBx = 16u32,
    sRGBA = 17u32,
    sBGRA = 18u32,
    ABGR = 19u32,
}
impl ImageChannelOrder {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=19u32 => unsafe { core::mem::transmute::<u32, ImageChannelOrder>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ImageChannelOrder {}
impl core::str::FromStr for ImageChannelOrder {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::R,
            "A" => Self::A,
            "RG" => Self::RG,
            "RA" => Self::RA,
            "RGB" => Self::RGB,
            "RGBA" => Self::RGBA,
            "BGRA" => Self::BGRA,
            "ARGB" => Self::ARGB,
            "Intensity" => Self::Intensity,
            "Luminance" => Self::Luminance,
            "Rx" => Self::Rx,
            "RGx" => Self::RGx,
            "RGBx" => Self::RGBx,
            "Depth" => Self::Depth,
            "DepthStencil" => Self::DepthStencil,
            "sRGB" => Self::sRGB,
            "sRGBx" => Self::sRGBx,
            "sRGBA" => Self::sRGBA,
            "sBGRA" => Self::sBGRA,
            "ABGR" => Self::ABGR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ImageChannelDataType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_image_channel_data_type_a_image_channel_data_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ImageChannelDataType {
    SnormInt8 = 0u32,
    SnormInt16 = 1u32,
    UnormInt8 = 2u32,
    UnormInt16 = 3u32,
    UnormShort565 = 4u32,
    UnormShort555 = 5u32,
    UnormInt101010 = 6u32,
    SignedInt8 = 7u32,
    SignedInt16 = 8u32,
    SignedInt32 = 9u32,
    UnsignedInt8 = 10u32,
    UnsignedInt16 = 11u32,
    UnsignedInt32 = 12u32,
    HalfFloat = 13u32,
    Float = 14u32,
    UnormInt24 = 15u32,
    UnormInt101010_2 = 16u32,
    UnormInt10X6EXT = 17u32,
    UnsignedIntRaw10EXT = 19u32,
    UnsignedIntRaw12EXT = 20u32,
    UnormInt2_101010EXT = 21u32,
    UnsignedInt10X6EXT = 22u32,
    UnsignedInt12X4EXT = 23u32,
    UnsignedInt14X2EXT = 24u32,
    UnormInt12X4EXT = 25u32,
    UnormInt14X2EXT = 26u32,
}
impl ImageChannelDataType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=17u32 => unsafe { core::mem::transmute::<u32, ImageChannelDataType>(n) },
            19u32..=26u32 => unsafe { core::mem::transmute::<u32, ImageChannelDataType>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ImageChannelDataType {}
impl core::str::FromStr for ImageChannelDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "SnormInt8" => Self::SnormInt8,
            "SnormInt16" => Self::SnormInt16,
            "UnormInt8" => Self::UnormInt8,
            "UnormInt16" => Self::UnormInt16,
            "UnormShort565" => Self::UnormShort565,
            "UnormShort555" => Self::UnormShort555,
            "UnormInt101010" => Self::UnormInt101010,
            "SignedInt8" => Self::SignedInt8,
            "SignedInt16" => Self::SignedInt16,
            "SignedInt32" => Self::SignedInt32,
            "UnsignedInt8" => Self::UnsignedInt8,
            "UnsignedInt16" => Self::UnsignedInt16,
            "UnsignedInt32" => Self::UnsignedInt32,
            "HalfFloat" => Self::HalfFloat,
            "Float" => Self::Float,
            "UnormInt24" => Self::UnormInt24,
            "UnormInt101010_2" => Self::UnormInt101010_2,
            "UnormInt10X6EXT" => Self::UnormInt10X6EXT,
            "UnsignedIntRaw10EXT" => Self::UnsignedIntRaw10EXT,
            "UnsignedIntRaw12EXT" => Self::UnsignedIntRaw12EXT,
            "UnormInt2_101010EXT" => Self::UnormInt2_101010EXT,
            "UnsignedInt10X6EXT" => Self::UnsignedInt10X6EXT,
            "UnsignedInt12X4EXT" => Self::UnsignedInt12X4EXT,
            "UnsignedInt14X2EXT" => Self::UnsignedInt14X2EXT,
            "UnormInt12X4EXT" => Self::UnormInt12X4EXT,
            "UnormInt14X2EXT" => Self::UnormInt14X2EXT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [FPRoundingMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_rounding_mode_a_fp_rounding_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FPRoundingMode {
    RTE = 0u32,
    RTZ = 1u32,
    RTP = 2u32,
    RTN = 3u32,
}
impl FPRoundingMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, FPRoundingMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl FPRoundingMode {}
impl core::str::FromStr for FPRoundingMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RTE" => Self::RTE,
            "RTZ" => Self::RTZ,
            "RTP" => Self::RTP,
            "RTN" => Self::RTN,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [FPDenormMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_denorm_mode_a_fp_denorm_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FPDenormMode {
    Preserve = 0u32,
    FlushToZero = 1u32,
}
impl FPDenormMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, FPDenormMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl FPDenormMode {}
impl core::str::FromStr for FPDenormMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Preserve" => Self::Preserve,
            "FlushToZero" => Self::FlushToZero,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [QuantizationModes](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_quantization_modes_a_quantization_modes)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum QuantizationModes {
    TRN = 0u32,
    TRN_ZERO = 1u32,
    RND = 2u32,
    RND_ZERO = 3u32,
    RND_INF = 4u32,
    RND_MIN_INF = 5u32,
    RND_CONV = 6u32,
    RND_CONV_ODD = 7u32,
}
impl QuantizationModes {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=7u32 => unsafe { core::mem::transmute::<u32, QuantizationModes>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl QuantizationModes {}
impl core::str::FromStr for QuantizationModes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TRN" => Self::TRN,
            "TRN_ZERO" => Self::TRN_ZERO,
            "RND" => Self::RND,
            "RND_ZERO" => Self::RND_ZERO,
            "RND_INF" => Self::RND_INF,
            "RND_MIN_INF" => Self::RND_MIN_INF,
            "RND_CONV" => Self::RND_CONV,
            "RND_CONV_ODD" => Self::RND_CONV_ODD,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [FPOperationMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_operation_mode_a_fp_operation_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FPOperationMode {
    IEEE = 0u32,
    ALT = 1u32,
}
impl FPOperationMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, FPOperationMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl FPOperationMode {}
impl core::str::FromStr for FPOperationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "IEEE" => Self::IEEE,
            "ALT" => Self::ALT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [OverflowModes](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_overflow_modes_a_overflow_modes)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum OverflowModes {
    WRAP = 0u32,
    SAT = 1u32,
    SAT_ZERO = 2u32,
    SAT_SYM = 3u32,
}
impl OverflowModes {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, OverflowModes>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl OverflowModes {}
impl core::str::FromStr for OverflowModes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "WRAP" => Self::WRAP,
            "SAT" => Self::SAT,
            "SAT_ZERO" => Self::SAT_ZERO,
            "SAT_SYM" => Self::SAT_SYM,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [LinkageType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_linkage_type_a_linkage_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum LinkageType {
    Export = 0u32,
    Import = 1u32,
    LinkOnceODR = 2u32,
}
impl LinkageType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, LinkageType>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl LinkageType {}
impl core::str::FromStr for LinkageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Export" => Self::Export,
            "Import" => Self::Import,
            "LinkOnceODR" => Self::LinkOnceODR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [AccessQualifier](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_access_qualifier_a_access_qualifier)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum AccessQualifier {
    ReadOnly = 0u32,
    WriteOnly = 1u32,
    ReadWrite = 2u32,
}
impl AccessQualifier {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, AccessQualifier>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl AccessQualifier {}
impl core::str::FromStr for AccessQualifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ReadOnly" => Self::ReadOnly,
            "WriteOnly" => Self::WriteOnly,
            "ReadWrite" => Self::ReadWrite,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [HostAccessQualifier](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_host_access_qualifier_a_host_access_qualifier)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum HostAccessQualifier {
    NoneINTEL = 0u32,
    ReadINTEL = 1u32,
    WriteINTEL = 2u32,
    ReadWriteINTEL = 3u32,
}
impl HostAccessQualifier {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, HostAccessQualifier>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl HostAccessQualifier {}
impl core::str::FromStr for HostAccessQualifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "NoneINTEL" => Self::NoneINTEL,
            "ReadINTEL" => Self::ReadINTEL,
            "WriteINTEL" => Self::WriteINTEL,
            "ReadWriteINTEL" => Self::ReadWriteINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [FunctionParameterAttribute](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_function_parameter_attribute_a_function_parameter_attribute)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FunctionParameterAttribute {
    Zext = 0u32,
    Sext = 1u32,
    ByVal = 2u32,
    Sret = 3u32,
    NoAlias = 4u32,
    NoCapture = 5u32,
    NoWrite = 6u32,
    NoReadWrite = 7u32,
    RuntimeAlignedALTERA = 5940u32,
}
impl FunctionParameterAttribute {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=7u32 => unsafe { core::mem::transmute::<u32, FunctionParameterAttribute>(n) },
            5940u32 => unsafe { core::mem::transmute::<u32, FunctionParameterAttribute>(5940u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl FunctionParameterAttribute {
    pub const RuntimeAlignedINTEL: Self = Self::RuntimeAlignedALTERA;
}
impl core::str::FromStr for FunctionParameterAttribute {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Zext" => Self::Zext,
            "Sext" => Self::Sext,
            "ByVal" => Self::ByVal,
            "Sret" => Self::Sret,
            "NoAlias" => Self::NoAlias,
            "NoCapture" => Self::NoCapture,
            "NoWrite" => Self::NoWrite,
            "NoReadWrite" => Self::NoReadWrite,
            "RuntimeAlignedALTERA" => Self::RuntimeAlignedALTERA,
            "RuntimeAlignedINTEL" => Self::RuntimeAlignedALTERA,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [Decoration](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_decoration_a_decoration)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Decoration {
    RelaxedPrecision = 0u32,
    SpecId = 1u32,
    Block = 2u32,
    BufferBlock = 3u32,
    RowMajor = 4u32,
    ColMajor = 5u32,
    ArrayStride = 6u32,
    MatrixStride = 7u32,
    GLSLShared = 8u32,
    GLSLPacked = 9u32,
    CPacked = 10u32,
    BuiltIn = 11u32,
    NoPerspective = 13u32,
    Flat = 14u32,
    Patch = 15u32,
    Centroid = 16u32,
    Sample = 17u32,
    Invariant = 18u32,
    Restrict = 19u32,
    Aliased = 20u32,
    Volatile = 21u32,
    Constant = 22u32,
    Coherent = 23u32,
    NonWritable = 24u32,
    NonReadable = 25u32,
    Uniform = 26u32,
    UniformId = 27u32,
    SaturatedConversion = 28u32,
    Stream = 29u32,
    Location = 30u32,
    Component = 31u32,
    Index = 32u32,
    Binding = 33u32,
    DescriptorSet = 34u32,
    Offset = 35u32,
    XfbBuffer = 36u32,
    XfbStride = 37u32,
    FuncParamAttr = 38u32,
    FPRoundingMode = 39u32,
    FPFastMathMode = 40u32,
    LinkageAttributes = 41u32,
    NoContraction = 42u32,
    InputAttachmentIndex = 43u32,
    Alignment = 44u32,
    MaxByteOffset = 45u32,
    AlignmentId = 46u32,
    MaxByteOffsetId = 47u32,
    SaturatedToLargestFloat8NormalConversionEXT = 4216u32,
    NoSignedWrap = 4469u32,
    NoUnsignedWrap = 4470u32,
    WeightTextureQCOM = 4487u32,
    BlockMatchTextureQCOM = 4488u32,
    BlockMatchSamplerQCOM = 4499u32,
    ExplicitInterpAMD = 4999u32,
    NodeSharesPayloadLimitsWithAMDX = 5019u32,
    NodeMaxPayloadsAMDX = 5020u32,
    TrackFinishWritingAMDX = 5078u32,
    PayloadNodeNameAMDX = 5091u32,
    PayloadNodeBaseIndexAMDX = 5098u32,
    PayloadNodeSparseArrayAMDX = 5099u32,
    PayloadNodeArraySizeAMDX = 5100u32,
    PayloadDispatchIndirectAMDX = 5105u32,
    ArrayStrideIdEXT = 5124u32,
    OffsetIdEXT = 5125u32,
    OverrideCoverageNV = 5248u32,
    PassthroughNV = 5250u32,
    ViewportRelativeNV = 5252u32,
    SecondaryViewportRelativeNV = 5256u32,
    PerPrimitiveEXT = 5271u32,
    PerViewNV = 5272u32,
    PerTaskNV = 5273u32,
    PerVertexKHR = 5285u32,
    NonUniform = 5300u32,
    RestrictPointer = 5355u32,
    AliasedPointer = 5356u32,
    MemberOffsetNV = 5358u32,
    HitObjectShaderRecordBufferNV = 5386u32,
    HitObjectShaderRecordBufferEXT = 5389u32,
    BankNV = 5397u32,
    BindlessSamplerNV = 5398u32,
    BindlessImageNV = 5399u32,
    BoundSamplerNV = 5400u32,
    BoundImageNV = 5401u32,
    SIMTCallINTEL = 5599u32,
    ReferencedIndirectlyINTEL = 5602u32,
    ClobberINTEL = 5607u32,
    SideEffectsINTEL = 5608u32,
    VectorComputeVariableINTEL = 5624u32,
    FuncParamIOKindINTEL = 5625u32,
    VectorComputeFunctionINTEL = 5626u32,
    StackCallINTEL = 5627u32,
    GlobalVariableOffsetINTEL = 5628u32,
    CounterBuffer = 5634u32,
    UserSemantic = 5635u32,
    UserTypeGOOGLE = 5636u32,
    FunctionRoundingModeINTEL = 5822u32,
    FunctionDenormModeINTEL = 5823u32,
    RegisterALTERA = 5825u32,
    MemoryALTERA = 5826u32,
    NumbanksALTERA = 5827u32,
    BankwidthALTERA = 5828u32,
    MaxPrivateCopiesALTERA = 5829u32,
    SinglepumpALTERA = 5830u32,
    DoublepumpALTERA = 5831u32,
    MaxReplicatesALTERA = 5832u32,
    SimpleDualPortALTERA = 5833u32,
    MergeALTERA = 5834u32,
    BankBitsALTERA = 5835u32,
    ForcePow2DepthALTERA = 5836u32,
    StridesizeALTERA = 5883u32,
    WordsizeALTERA = 5884u32,
    TrueDualPortALTERA = 5885u32,
    BurstCoalesceALTERA = 5899u32,
    CacheSizeALTERA = 5900u32,
    DontStaticallyCoalesceALTERA = 5901u32,
    PrefetchALTERA = 5902u32,
    StallEnableALTERA = 5905u32,
    FuseLoopsInFunctionALTERA = 5907u32,
    MathOpDSPModeALTERA = 5909u32,
    AliasScopeINTEL = 5914u32,
    NoAliasINTEL = 5915u32,
    InitiationIntervalALTERA = 5917u32,
    MaxConcurrencyALTERA = 5918u32,
    PipelineEnableALTERA = 5919u32,
    BufferLocationALTERA = 5921u32,
    IOPipeStorageALTERA = 5944u32,
    FunctionFloatingPointModeINTEL = 6080u32,
    SingleElementVectorINTEL = 6085u32,
    VectorComputeCallableFunctionINTEL = 6087u32,
    MediaBlockIOINTEL = 6140u32,
    StallFreeALTERA = 6151u32,
    FPMaxErrorDecorationINTEL = 6170u32,
    LatencyControlLabelALTERA = 6172u32,
    LatencyControlConstraintALTERA = 6173u32,
    ConduitKernelArgumentALTERA = 6175u32,
    RegisterMapKernelArgumentALTERA = 6176u32,
    MMHostInterfaceAddressWidthALTERA = 6177u32,
    MMHostInterfaceDataWidthALTERA = 6178u32,
    MMHostInterfaceLatencyALTERA = 6179u32,
    MMHostInterfaceReadWriteModeALTERA = 6180u32,
    MMHostInterfaceMaxBurstALTERA = 6181u32,
    MMHostInterfaceWaitRequestALTERA = 6182u32,
    StableKernelArgumentALTERA = 6183u32,
    HostAccessINTEL = 6188u32,
    InitModeALTERA = 6190u32,
    ImplementInRegisterMapALTERA = 6191u32,
    ConditionalINTEL = 6247u32,
    CacheControlLoadINTEL = 6442u32,
    CacheControlStoreINTEL = 6443u32,
}
impl Decoration {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=11u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            13u32..=47u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            4216u32 => unsafe { core::mem::transmute::<u32, Decoration>(4216u32) },
            4469u32..=4470u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            4487u32..=4488u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            4499u32 => unsafe { core::mem::transmute::<u32, Decoration>(4499u32) },
            4999u32 => unsafe { core::mem::transmute::<u32, Decoration>(4999u32) },
            5019u32..=5020u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5078u32 => unsafe { core::mem::transmute::<u32, Decoration>(5078u32) },
            5091u32 => unsafe { core::mem::transmute::<u32, Decoration>(5091u32) },
            5098u32..=5100u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5105u32 => unsafe { core::mem::transmute::<u32, Decoration>(5105u32) },
            5124u32..=5125u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5248u32 => unsafe { core::mem::transmute::<u32, Decoration>(5248u32) },
            5250u32 => unsafe { core::mem::transmute::<u32, Decoration>(5250u32) },
            5252u32 => unsafe { core::mem::transmute::<u32, Decoration>(5252u32) },
            5256u32 => unsafe { core::mem::transmute::<u32, Decoration>(5256u32) },
            5271u32..=5273u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5285u32 => unsafe { core::mem::transmute::<u32, Decoration>(5285u32) },
            5300u32 => unsafe { core::mem::transmute::<u32, Decoration>(5300u32) },
            5355u32..=5356u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5358u32 => unsafe { core::mem::transmute::<u32, Decoration>(5358u32) },
            5386u32 => unsafe { core::mem::transmute::<u32, Decoration>(5386u32) },
            5389u32 => unsafe { core::mem::transmute::<u32, Decoration>(5389u32) },
            5397u32..=5401u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5599u32 => unsafe { core::mem::transmute::<u32, Decoration>(5599u32) },
            5602u32 => unsafe { core::mem::transmute::<u32, Decoration>(5602u32) },
            5607u32..=5608u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5624u32..=5628u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5634u32..=5636u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5822u32..=5823u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5825u32..=5836u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5883u32..=5885u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5899u32..=5902u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5905u32 => unsafe { core::mem::transmute::<u32, Decoration>(5905u32) },
            5907u32 => unsafe { core::mem::transmute::<u32, Decoration>(5907u32) },
            5909u32 => unsafe { core::mem::transmute::<u32, Decoration>(5909u32) },
            5914u32..=5915u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5917u32..=5919u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            5921u32 => unsafe { core::mem::transmute::<u32, Decoration>(5921u32) },
            5944u32 => unsafe { core::mem::transmute::<u32, Decoration>(5944u32) },
            6080u32 => unsafe { core::mem::transmute::<u32, Decoration>(6080u32) },
            6085u32 => unsafe { core::mem::transmute::<u32, Decoration>(6085u32) },
            6087u32 => unsafe { core::mem::transmute::<u32, Decoration>(6087u32) },
            6140u32 => unsafe { core::mem::transmute::<u32, Decoration>(6140u32) },
            6151u32 => unsafe { core::mem::transmute::<u32, Decoration>(6151u32) },
            6170u32 => unsafe { core::mem::transmute::<u32, Decoration>(6170u32) },
            6172u32..=6173u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            6175u32..=6183u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            6188u32 => unsafe { core::mem::transmute::<u32, Decoration>(6188u32) },
            6190u32..=6191u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            6247u32 => unsafe { core::mem::transmute::<u32, Decoration>(6247u32) },
            6442u32..=6443u32 => unsafe { core::mem::transmute::<u32, Decoration>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl Decoration {
    pub const PerPrimitiveNV: Self = Self::PerPrimitiveEXT;
    pub const PerVertexNV: Self = Self::PerVertexKHR;
    pub const NonUniformEXT: Self = Self::NonUniform;
    pub const RestrictPointerEXT: Self = Self::RestrictPointer;
    pub const AliasedPointerEXT: Self = Self::AliasedPointer;
    pub const HlslCounterBufferGOOGLE: Self = Self::CounterBuffer;
    pub const HlslSemanticGOOGLE: Self = Self::UserSemantic;
    pub const RegisterINTEL: Self = Self::RegisterALTERA;
    pub const MemoryINTEL: Self = Self::MemoryALTERA;
    pub const NumbanksINTEL: Self = Self::NumbanksALTERA;
    pub const BankwidthINTEL: Self = Self::BankwidthALTERA;
    pub const MaxPrivateCopiesINTEL: Self = Self::MaxPrivateCopiesALTERA;
    pub const SinglepumpINTEL: Self = Self::SinglepumpALTERA;
    pub const DoublepumpINTEL: Self = Self::DoublepumpALTERA;
    pub const MaxReplicatesINTEL: Self = Self::MaxReplicatesALTERA;
    pub const SimpleDualPortINTEL: Self = Self::SimpleDualPortALTERA;
    pub const MergeINTEL: Self = Self::MergeALTERA;
    pub const BankBitsINTEL: Self = Self::BankBitsALTERA;
    pub const ForcePow2DepthINTEL: Self = Self::ForcePow2DepthALTERA;
    pub const StridesizeINTEL: Self = Self::StridesizeALTERA;
    pub const WordsizeINTEL: Self = Self::WordsizeALTERA;
    pub const TrueDualPortINTEL: Self = Self::TrueDualPortALTERA;
    pub const BurstCoalesceINTEL: Self = Self::BurstCoalesceALTERA;
    pub const CacheSizeINTEL: Self = Self::CacheSizeALTERA;
    pub const DontStaticallyCoalesceINTEL: Self = Self::DontStaticallyCoalesceALTERA;
    pub const PrefetchINTEL: Self = Self::PrefetchALTERA;
    pub const StallEnableINTEL: Self = Self::StallEnableALTERA;
    pub const FuseLoopsInFunctionINTEL: Self = Self::FuseLoopsInFunctionALTERA;
    pub const MathOpDSPModeINTEL: Self = Self::MathOpDSPModeALTERA;
    pub const InitiationIntervalINTEL: Self = Self::InitiationIntervalALTERA;
    pub const MaxConcurrencyINTEL: Self = Self::MaxConcurrencyALTERA;
    pub const PipelineEnableINTEL: Self = Self::PipelineEnableALTERA;
    pub const BufferLocationINTEL: Self = Self::BufferLocationALTERA;
    pub const IOPipeStorageINTEL: Self = Self::IOPipeStorageALTERA;
    pub const StallFreeINTEL: Self = Self::StallFreeALTERA;
    pub const LatencyControlLabelINTEL: Self = Self::LatencyControlLabelALTERA;
    pub const LatencyControlConstraintINTEL: Self = Self::LatencyControlConstraintALTERA;
    pub const ConduitKernelArgumentINTEL: Self = Self::ConduitKernelArgumentALTERA;
    pub const RegisterMapKernelArgumentINTEL: Self = Self::RegisterMapKernelArgumentALTERA;
    pub const MMHostInterfaceAddressWidthINTEL: Self = Self::MMHostInterfaceAddressWidthALTERA;
    pub const MMHostInterfaceDataWidthINTEL: Self = Self::MMHostInterfaceDataWidthALTERA;
    pub const MMHostInterfaceLatencyINTEL: Self = Self::MMHostInterfaceLatencyALTERA;
    pub const MMHostInterfaceReadWriteModeINTEL: Self = Self::MMHostInterfaceReadWriteModeALTERA;
    pub const MMHostInterfaceMaxBurstINTEL: Self = Self::MMHostInterfaceMaxBurstALTERA;
    pub const MMHostInterfaceWaitRequestINTEL: Self = Self::MMHostInterfaceWaitRequestALTERA;
    pub const StableKernelArgumentINTEL: Self = Self::StableKernelArgumentALTERA;
    pub const InitModeINTEL: Self = Self::InitModeALTERA;
    pub const ImplementInRegisterMapINTEL: Self = Self::ImplementInRegisterMapALTERA;
}
impl core::str::FromStr for Decoration {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RelaxedPrecision" => Self::RelaxedPrecision,
            "SpecId" => Self::SpecId,
            "Block" => Self::Block,
            "BufferBlock" => Self::BufferBlock,
            "RowMajor" => Self::RowMajor,
            "ColMajor" => Self::ColMajor,
            "ArrayStride" => Self::ArrayStride,
            "MatrixStride" => Self::MatrixStride,
            "GLSLShared" => Self::GLSLShared,
            "GLSLPacked" => Self::GLSLPacked,
            "CPacked" => Self::CPacked,
            "BuiltIn" => Self::BuiltIn,
            "NoPerspective" => Self::NoPerspective,
            "Flat" => Self::Flat,
            "Patch" => Self::Patch,
            "Centroid" => Self::Centroid,
            "Sample" => Self::Sample,
            "Invariant" => Self::Invariant,
            "Restrict" => Self::Restrict,
            "Aliased" => Self::Aliased,
            "Volatile" => Self::Volatile,
            "Constant" => Self::Constant,
            "Coherent" => Self::Coherent,
            "NonWritable" => Self::NonWritable,
            "NonReadable" => Self::NonReadable,
            "Uniform" => Self::Uniform,
            "UniformId" => Self::UniformId,
            "SaturatedConversion" => Self::SaturatedConversion,
            "Stream" => Self::Stream,
            "Location" => Self::Location,
            "Component" => Self::Component,
            "Index" => Self::Index,
            "Binding" => Self::Binding,
            "DescriptorSet" => Self::DescriptorSet,
            "Offset" => Self::Offset,
            "XfbBuffer" => Self::XfbBuffer,
            "XfbStride" => Self::XfbStride,
            "FuncParamAttr" => Self::FuncParamAttr,
            "FPRoundingMode" => Self::FPRoundingMode,
            "FPFastMathMode" => Self::FPFastMathMode,
            "LinkageAttributes" => Self::LinkageAttributes,
            "NoContraction" => Self::NoContraction,
            "InputAttachmentIndex" => Self::InputAttachmentIndex,
            "Alignment" => Self::Alignment,
            "MaxByteOffset" => Self::MaxByteOffset,
            "AlignmentId" => Self::AlignmentId,
            "MaxByteOffsetId" => Self::MaxByteOffsetId,
            "SaturatedToLargestFloat8NormalConversionEXT" => {
                Self::SaturatedToLargestFloat8NormalConversionEXT
            }
            "NoSignedWrap" => Self::NoSignedWrap,
            "NoUnsignedWrap" => Self::NoUnsignedWrap,
            "WeightTextureQCOM" => Self::WeightTextureQCOM,
            "BlockMatchTextureQCOM" => Self::BlockMatchTextureQCOM,
            "BlockMatchSamplerQCOM" => Self::BlockMatchSamplerQCOM,
            "ExplicitInterpAMD" => Self::ExplicitInterpAMD,
            "NodeSharesPayloadLimitsWithAMDX" => Self::NodeSharesPayloadLimitsWithAMDX,
            "NodeMaxPayloadsAMDX" => Self::NodeMaxPayloadsAMDX,
            "TrackFinishWritingAMDX" => Self::TrackFinishWritingAMDX,
            "PayloadNodeNameAMDX" => Self::PayloadNodeNameAMDX,
            "PayloadNodeBaseIndexAMDX" => Self::PayloadNodeBaseIndexAMDX,
            "PayloadNodeSparseArrayAMDX" => Self::PayloadNodeSparseArrayAMDX,
            "PayloadNodeArraySizeAMDX" => Self::PayloadNodeArraySizeAMDX,
            "PayloadDispatchIndirectAMDX" => Self::PayloadDispatchIndirectAMDX,
            "ArrayStrideIdEXT" => Self::ArrayStrideIdEXT,
            "OffsetIdEXT" => Self::OffsetIdEXT,
            "OverrideCoverageNV" => Self::OverrideCoverageNV,
            "PassthroughNV" => Self::PassthroughNV,
            "ViewportRelativeNV" => Self::ViewportRelativeNV,
            "SecondaryViewportRelativeNV" => Self::SecondaryViewportRelativeNV,
            "PerPrimitiveEXT" => Self::PerPrimitiveEXT,
            "PerPrimitiveNV" => Self::PerPrimitiveEXT,
            "PerViewNV" => Self::PerViewNV,
            "PerTaskNV" => Self::PerTaskNV,
            "PerVertexKHR" => Self::PerVertexKHR,
            "PerVertexNV" => Self::PerVertexKHR,
            "NonUniform" => Self::NonUniform,
            "NonUniformEXT" => Self::NonUniform,
            "RestrictPointer" => Self::RestrictPointer,
            "RestrictPointerEXT" => Self::RestrictPointer,
            "AliasedPointer" => Self::AliasedPointer,
            "AliasedPointerEXT" => Self::AliasedPointer,
            "MemberOffsetNV" => Self::MemberOffsetNV,
            "HitObjectShaderRecordBufferNV" => Self::HitObjectShaderRecordBufferNV,
            "HitObjectShaderRecordBufferEXT" => Self::HitObjectShaderRecordBufferEXT,
            "BankNV" => Self::BankNV,
            "BindlessSamplerNV" => Self::BindlessSamplerNV,
            "BindlessImageNV" => Self::BindlessImageNV,
            "BoundSamplerNV" => Self::BoundSamplerNV,
            "BoundImageNV" => Self::BoundImageNV,
            "SIMTCallINTEL" => Self::SIMTCallINTEL,
            "ReferencedIndirectlyINTEL" => Self::ReferencedIndirectlyINTEL,
            "ClobberINTEL" => Self::ClobberINTEL,
            "SideEffectsINTEL" => Self::SideEffectsINTEL,
            "VectorComputeVariableINTEL" => Self::VectorComputeVariableINTEL,
            "FuncParamIOKindINTEL" => Self::FuncParamIOKindINTEL,
            "VectorComputeFunctionINTEL" => Self::VectorComputeFunctionINTEL,
            "StackCallINTEL" => Self::StackCallINTEL,
            "GlobalVariableOffsetINTEL" => Self::GlobalVariableOffsetINTEL,
            "CounterBuffer" => Self::CounterBuffer,
            "HlslCounterBufferGOOGLE" => Self::CounterBuffer,
            "UserSemantic" => Self::UserSemantic,
            "HlslSemanticGOOGLE" => Self::UserSemantic,
            "UserTypeGOOGLE" => Self::UserTypeGOOGLE,
            "FunctionRoundingModeINTEL" => Self::FunctionRoundingModeINTEL,
            "FunctionDenormModeINTEL" => Self::FunctionDenormModeINTEL,
            "RegisterALTERA" => Self::RegisterALTERA,
            "RegisterINTEL" => Self::RegisterALTERA,
            "MemoryALTERA" => Self::MemoryALTERA,
            "MemoryINTEL" => Self::MemoryALTERA,
            "NumbanksALTERA" => Self::NumbanksALTERA,
            "NumbanksINTEL" => Self::NumbanksALTERA,
            "BankwidthALTERA" => Self::BankwidthALTERA,
            "BankwidthINTEL" => Self::BankwidthALTERA,
            "MaxPrivateCopiesALTERA" => Self::MaxPrivateCopiesALTERA,
            "MaxPrivateCopiesINTEL" => Self::MaxPrivateCopiesALTERA,
            "SinglepumpALTERA" => Self::SinglepumpALTERA,
            "SinglepumpINTEL" => Self::SinglepumpALTERA,
            "DoublepumpALTERA" => Self::DoublepumpALTERA,
            "DoublepumpINTEL" => Self::DoublepumpALTERA,
            "MaxReplicatesALTERA" => Self::MaxReplicatesALTERA,
            "MaxReplicatesINTEL" => Self::MaxReplicatesALTERA,
            "SimpleDualPortALTERA" => Self::SimpleDualPortALTERA,
            "SimpleDualPortINTEL" => Self::SimpleDualPortALTERA,
            "MergeALTERA" => Self::MergeALTERA,
            "MergeINTEL" => Self::MergeALTERA,
            "BankBitsALTERA" => Self::BankBitsALTERA,
            "BankBitsINTEL" => Self::BankBitsALTERA,
            "ForcePow2DepthALTERA" => Self::ForcePow2DepthALTERA,
            "ForcePow2DepthINTEL" => Self::ForcePow2DepthALTERA,
            "StridesizeALTERA" => Self::StridesizeALTERA,
            "StridesizeINTEL" => Self::StridesizeALTERA,
            "WordsizeALTERA" => Self::WordsizeALTERA,
            "WordsizeINTEL" => Self::WordsizeALTERA,
            "TrueDualPortALTERA" => Self::TrueDualPortALTERA,
            "TrueDualPortINTEL" => Self::TrueDualPortALTERA,
            "BurstCoalesceALTERA" => Self::BurstCoalesceALTERA,
            "BurstCoalesceINTEL" => Self::BurstCoalesceALTERA,
            "CacheSizeALTERA" => Self::CacheSizeALTERA,
            "CacheSizeINTEL" => Self::CacheSizeALTERA,
            "DontStaticallyCoalesceALTERA" => Self::DontStaticallyCoalesceALTERA,
            "DontStaticallyCoalesceINTEL" => Self::DontStaticallyCoalesceALTERA,
            "PrefetchALTERA" => Self::PrefetchALTERA,
            "PrefetchINTEL" => Self::PrefetchALTERA,
            "StallEnableALTERA" => Self::StallEnableALTERA,
            "StallEnableINTEL" => Self::StallEnableALTERA,
            "FuseLoopsInFunctionALTERA" => Self::FuseLoopsInFunctionALTERA,
            "FuseLoopsInFunctionINTEL" => Self::FuseLoopsInFunctionALTERA,
            "MathOpDSPModeALTERA" => Self::MathOpDSPModeALTERA,
            "MathOpDSPModeINTEL" => Self::MathOpDSPModeALTERA,
            "AliasScopeINTEL" => Self::AliasScopeINTEL,
            "NoAliasINTEL" => Self::NoAliasINTEL,
            "InitiationIntervalALTERA" => Self::InitiationIntervalALTERA,
            "InitiationIntervalINTEL" => Self::InitiationIntervalALTERA,
            "MaxConcurrencyALTERA" => Self::MaxConcurrencyALTERA,
            "MaxConcurrencyINTEL" => Self::MaxConcurrencyALTERA,
            "PipelineEnableALTERA" => Self::PipelineEnableALTERA,
            "PipelineEnableINTEL" => Self::PipelineEnableALTERA,
            "BufferLocationALTERA" => Self::BufferLocationALTERA,
            "BufferLocationINTEL" => Self::BufferLocationALTERA,
            "IOPipeStorageALTERA" => Self::IOPipeStorageALTERA,
            "IOPipeStorageINTEL" => Self::IOPipeStorageALTERA,
            "FunctionFloatingPointModeINTEL" => Self::FunctionFloatingPointModeINTEL,
            "SingleElementVectorINTEL" => Self::SingleElementVectorINTEL,
            "VectorComputeCallableFunctionINTEL" => Self::VectorComputeCallableFunctionINTEL,
            "MediaBlockIOINTEL" => Self::MediaBlockIOINTEL,
            "StallFreeALTERA" => Self::StallFreeALTERA,
            "StallFreeINTEL" => Self::StallFreeALTERA,
            "FPMaxErrorDecorationINTEL" => Self::FPMaxErrorDecorationINTEL,
            "LatencyControlLabelALTERA" => Self::LatencyControlLabelALTERA,
            "LatencyControlLabelINTEL" => Self::LatencyControlLabelALTERA,
            "LatencyControlConstraintALTERA" => Self::LatencyControlConstraintALTERA,
            "LatencyControlConstraintINTEL" => Self::LatencyControlConstraintALTERA,
            "ConduitKernelArgumentALTERA" => Self::ConduitKernelArgumentALTERA,
            "ConduitKernelArgumentINTEL" => Self::ConduitKernelArgumentALTERA,
            "RegisterMapKernelArgumentALTERA" => Self::RegisterMapKernelArgumentALTERA,
            "RegisterMapKernelArgumentINTEL" => Self::RegisterMapKernelArgumentALTERA,
            "MMHostInterfaceAddressWidthALTERA" => Self::MMHostInterfaceAddressWidthALTERA,
            "MMHostInterfaceAddressWidthINTEL" => Self::MMHostInterfaceAddressWidthALTERA,
            "MMHostInterfaceDataWidthALTERA" => Self::MMHostInterfaceDataWidthALTERA,
            "MMHostInterfaceDataWidthINTEL" => Self::MMHostInterfaceDataWidthALTERA,
            "MMHostInterfaceLatencyALTERA" => Self::MMHostInterfaceLatencyALTERA,
            "MMHostInterfaceLatencyINTEL" => Self::MMHostInterfaceLatencyALTERA,
            "MMHostInterfaceReadWriteModeALTERA" => Self::MMHostInterfaceReadWriteModeALTERA,
            "MMHostInterfaceReadWriteModeINTEL" => Self::MMHostInterfaceReadWriteModeALTERA,
            "MMHostInterfaceMaxBurstALTERA" => Self::MMHostInterfaceMaxBurstALTERA,
            "MMHostInterfaceMaxBurstINTEL" => Self::MMHostInterfaceMaxBurstALTERA,
            "MMHostInterfaceWaitRequestALTERA" => Self::MMHostInterfaceWaitRequestALTERA,
            "MMHostInterfaceWaitRequestINTEL" => Self::MMHostInterfaceWaitRequestALTERA,
            "StableKernelArgumentALTERA" => Self::StableKernelArgumentALTERA,
            "StableKernelArgumentINTEL" => Self::StableKernelArgumentALTERA,
            "HostAccessINTEL" => Self::HostAccessINTEL,
            "InitModeALTERA" => Self::InitModeALTERA,
            "InitModeINTEL" => Self::InitModeALTERA,
            "ImplementInRegisterMapALTERA" => Self::ImplementInRegisterMapALTERA,
            "ImplementInRegisterMapINTEL" => Self::ImplementInRegisterMapALTERA,
            "ConditionalINTEL" => Self::ConditionalINTEL,
            "CacheControlLoadINTEL" => Self::CacheControlLoadINTEL,
            "CacheControlStoreINTEL" => Self::CacheControlStoreINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [BuiltIn](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_built_in_a_built_in)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum BuiltIn {
    Position = 0u32,
    PointSize = 1u32,
    ClipDistance = 3u32,
    CullDistance = 4u32,
    VertexId = 5u32,
    InstanceId = 6u32,
    PrimitiveId = 7u32,
    InvocationId = 8u32,
    Layer = 9u32,
    ViewportIndex = 10u32,
    TessLevelOuter = 11u32,
    TessLevelInner = 12u32,
    TessCoord = 13u32,
    PatchVertices = 14u32,
    FragCoord = 15u32,
    PointCoord = 16u32,
    FrontFacing = 17u32,
    SampleId = 18u32,
    SamplePosition = 19u32,
    SampleMask = 20u32,
    FragDepth = 22u32,
    HelperInvocation = 23u32,
    NumWorkgroups = 24u32,
    WorkgroupSize = 25u32,
    WorkgroupId = 26u32,
    LocalInvocationId = 27u32,
    GlobalInvocationId = 28u32,
    LocalInvocationIndex = 29u32,
    WorkDim = 30u32,
    GlobalSize = 31u32,
    EnqueuedWorkgroupSize = 32u32,
    GlobalOffset = 33u32,
    GlobalLinearId = 34u32,
    SubgroupSize = 36u32,
    SubgroupMaxSize = 37u32,
    NumSubgroups = 38u32,
    NumEnqueuedSubgroups = 39u32,
    SubgroupId = 40u32,
    SubgroupLocalInvocationId = 41u32,
    VertexIndex = 42u32,
    InstanceIndex = 43u32,
    CoreIDARM = 4160u32,
    CoreCountARM = 4161u32,
    CoreMaxIDARM = 4162u32,
    WarpIDARM = 4163u32,
    WarpMaxIDARM = 4164u32,
    SubgroupEqMask = 4416u32,
    SubgroupGeMask = 4417u32,
    SubgroupGtMask = 4418u32,
    SubgroupLeMask = 4419u32,
    SubgroupLtMask = 4420u32,
    BaseVertex = 4424u32,
    BaseInstance = 4425u32,
    DrawIndex = 4426u32,
    PrimitiveShadingRateKHR = 4432u32,
    DeviceIndex = 4438u32,
    ViewIndex = 4440u32,
    ShadingRateKHR = 4444u32,
    TileOffsetQCOM = 4492u32,
    TileDimensionQCOM = 4493u32,
    TileApronSizeQCOM = 4494u32,
    BaryCoordNoPerspAMD = 4992u32,
    BaryCoordNoPerspCentroidAMD = 4993u32,
    BaryCoordNoPerspSampleAMD = 4994u32,
    BaryCoordSmoothAMD = 4995u32,
    BaryCoordSmoothCentroidAMD = 4996u32,
    BaryCoordSmoothSampleAMD = 4997u32,
    BaryCoordPullModelAMD = 4998u32,
    FragStencilRefEXT = 5014u32,
    RemainingRecursionLevelsAMDX = 5021u32,
    ShaderIndexAMDX = 5073u32,
    SamplerHeapEXT = 5122u32,
    ResourceHeapEXT = 5123u32,
    ViewportMaskNV = 5253u32,
    SecondaryPositionNV = 5257u32,
    SecondaryViewportMaskNV = 5258u32,
    PositionPerViewNV = 5261u32,
    ViewportMaskPerViewNV = 5262u32,
    FullyCoveredEXT = 5264u32,
    TaskCountNV = 5274u32,
    PrimitiveCountNV = 5275u32,
    PrimitiveIndicesNV = 5276u32,
    ClipDistancePerViewNV = 5277u32,
    CullDistancePerViewNV = 5278u32,
    LayerPerViewNV = 5279u32,
    MeshViewCountNV = 5280u32,
    MeshViewIndicesNV = 5281u32,
    BaryCoordKHR = 5286u32,
    BaryCoordNoPerspKHR = 5287u32,
    FragSizeEXT = 5292u32,
    FragInvocationCountEXT = 5293u32,
    PrimitivePointIndicesEXT = 5294u32,
    PrimitiveLineIndicesEXT = 5295u32,
    PrimitiveTriangleIndicesEXT = 5296u32,
    CullPrimitiveEXT = 5299u32,
    LaunchIdKHR = 5319u32,
    LaunchSizeKHR = 5320u32,
    WorldRayOriginKHR = 5321u32,
    WorldRayDirectionKHR = 5322u32,
    ObjectRayOriginKHR = 5323u32,
    ObjectRayDirectionKHR = 5324u32,
    RayTminKHR = 5325u32,
    RayTmaxKHR = 5326u32,
    InstanceCustomIndexKHR = 5327u32,
    ObjectToWorldKHR = 5330u32,
    WorldToObjectKHR = 5331u32,
    HitTNV = 5332u32,
    HitKindKHR = 5333u32,
    CurrentRayTimeNV = 5334u32,
    HitTriangleVertexPositionsKHR = 5335u32,
    HitMicroTriangleVertexPositionsNV = 5337u32,
    HitMicroTriangleVertexBarycentricsNV = 5344u32,
    IncomingRayFlagsKHR = 5351u32,
    RayGeometryIndexKHR = 5352u32,
    HitIsSphereNV = 5359u32,
    HitIsLSSNV = 5360u32,
    HitSpherePositionNV = 5361u32,
    WarpsPerSMNV = 5374u32,
    SMCountNV = 5375u32,
    WarpIDNV = 5376u32,
    SMIDNV = 5377u32,
    HitLSSPositionsNV = 5396u32,
    HitKindFrontFacingMicroTriangleNV = 5405u32,
    HitKindBackFacingMicroTriangleNV = 5406u32,
    HitSphereRadiusNV = 5420u32,
    HitLSSRadiiNV = 5421u32,
    ClusterIDNV = 5436u32,
    CullMaskKHR = 6021u32,
}
impl BuiltIn {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            3u32..=20u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            22u32..=34u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            36u32..=43u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            4160u32..=4164u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            4416u32..=4420u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            4424u32..=4426u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            4432u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(4432u32) },
            4438u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(4438u32) },
            4440u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(4440u32) },
            4444u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(4444u32) },
            4492u32..=4494u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            4992u32..=4998u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5014u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5014u32) },
            5021u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5021u32) },
            5073u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5073u32) },
            5122u32..=5123u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5253u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5253u32) },
            5257u32..=5258u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5261u32..=5262u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5264u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5264u32) },
            5274u32..=5281u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5286u32..=5287u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5292u32..=5296u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5299u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5299u32) },
            5319u32..=5327u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5330u32..=5335u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5337u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5337u32) },
            5344u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5344u32) },
            5351u32..=5352u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5359u32..=5361u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5374u32..=5377u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5396u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5396u32) },
            5405u32..=5406u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5420u32..=5421u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(n) },
            5436u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(5436u32) },
            6021u32 => unsafe { core::mem::transmute::<u32, BuiltIn>(6021u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl BuiltIn {
    pub const SubgroupEqMaskKHR: Self = Self::SubgroupEqMask;
    pub const SubgroupGeMaskKHR: Self = Self::SubgroupGeMask;
    pub const SubgroupGtMaskKHR: Self = Self::SubgroupGtMask;
    pub const SubgroupLeMaskKHR: Self = Self::SubgroupLeMask;
    pub const SubgroupLtMaskKHR: Self = Self::SubgroupLtMask;
    pub const BaryCoordNV: Self = Self::BaryCoordKHR;
    pub const BaryCoordNoPerspNV: Self = Self::BaryCoordNoPerspKHR;
    pub const FragmentSizeNV: Self = Self::FragSizeEXT;
    pub const InvocationsPerPixelNV: Self = Self::FragInvocationCountEXT;
    pub const LaunchIdNV: Self = Self::LaunchIdKHR;
    pub const LaunchSizeNV: Self = Self::LaunchSizeKHR;
    pub const WorldRayOriginNV: Self = Self::WorldRayOriginKHR;
    pub const WorldRayDirectionNV: Self = Self::WorldRayDirectionKHR;
    pub const ObjectRayOriginNV: Self = Self::ObjectRayOriginKHR;
    pub const ObjectRayDirectionNV: Self = Self::ObjectRayDirectionKHR;
    pub const RayTminNV: Self = Self::RayTminKHR;
    pub const RayTmaxNV: Self = Self::RayTmaxKHR;
    pub const InstanceCustomIndexNV: Self = Self::InstanceCustomIndexKHR;
    pub const ObjectToWorldNV: Self = Self::ObjectToWorldKHR;
    pub const WorldToObjectNV: Self = Self::WorldToObjectKHR;
    pub const HitKindNV: Self = Self::HitKindKHR;
    pub const IncomingRayFlagsNV: Self = Self::IncomingRayFlagsKHR;
}
impl core::str::FromStr for BuiltIn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Position" => Self::Position,
            "PointSize" => Self::PointSize,
            "ClipDistance" => Self::ClipDistance,
            "CullDistance" => Self::CullDistance,
            "VertexId" => Self::VertexId,
            "InstanceId" => Self::InstanceId,
            "PrimitiveId" => Self::PrimitiveId,
            "InvocationId" => Self::InvocationId,
            "Layer" => Self::Layer,
            "ViewportIndex" => Self::ViewportIndex,
            "TessLevelOuter" => Self::TessLevelOuter,
            "TessLevelInner" => Self::TessLevelInner,
            "TessCoord" => Self::TessCoord,
            "PatchVertices" => Self::PatchVertices,
            "FragCoord" => Self::FragCoord,
            "PointCoord" => Self::PointCoord,
            "FrontFacing" => Self::FrontFacing,
            "SampleId" => Self::SampleId,
            "SamplePosition" => Self::SamplePosition,
            "SampleMask" => Self::SampleMask,
            "FragDepth" => Self::FragDepth,
            "HelperInvocation" => Self::HelperInvocation,
            "NumWorkgroups" => Self::NumWorkgroups,
            "WorkgroupSize" => Self::WorkgroupSize,
            "WorkgroupId" => Self::WorkgroupId,
            "LocalInvocationId" => Self::LocalInvocationId,
            "GlobalInvocationId" => Self::GlobalInvocationId,
            "LocalInvocationIndex" => Self::LocalInvocationIndex,
            "WorkDim" => Self::WorkDim,
            "GlobalSize" => Self::GlobalSize,
            "EnqueuedWorkgroupSize" => Self::EnqueuedWorkgroupSize,
            "GlobalOffset" => Self::GlobalOffset,
            "GlobalLinearId" => Self::GlobalLinearId,
            "SubgroupSize" => Self::SubgroupSize,
            "SubgroupMaxSize" => Self::SubgroupMaxSize,
            "NumSubgroups" => Self::NumSubgroups,
            "NumEnqueuedSubgroups" => Self::NumEnqueuedSubgroups,
            "SubgroupId" => Self::SubgroupId,
            "SubgroupLocalInvocationId" => Self::SubgroupLocalInvocationId,
            "VertexIndex" => Self::VertexIndex,
            "InstanceIndex" => Self::InstanceIndex,
            "CoreIDARM" => Self::CoreIDARM,
            "CoreCountARM" => Self::CoreCountARM,
            "CoreMaxIDARM" => Self::CoreMaxIDARM,
            "WarpIDARM" => Self::WarpIDARM,
            "WarpMaxIDARM" => Self::WarpMaxIDARM,
            "SubgroupEqMask" => Self::SubgroupEqMask,
            "SubgroupEqMaskKHR" => Self::SubgroupEqMask,
            "SubgroupGeMask" => Self::SubgroupGeMask,
            "SubgroupGeMaskKHR" => Self::SubgroupGeMask,
            "SubgroupGtMask" => Self::SubgroupGtMask,
            "SubgroupGtMaskKHR" => Self::SubgroupGtMask,
            "SubgroupLeMask" => Self::SubgroupLeMask,
            "SubgroupLeMaskKHR" => Self::SubgroupLeMask,
            "SubgroupLtMask" => Self::SubgroupLtMask,
            "SubgroupLtMaskKHR" => Self::SubgroupLtMask,
            "BaseVertex" => Self::BaseVertex,
            "BaseInstance" => Self::BaseInstance,
            "DrawIndex" => Self::DrawIndex,
            "PrimitiveShadingRateKHR" => Self::PrimitiveShadingRateKHR,
            "DeviceIndex" => Self::DeviceIndex,
            "ViewIndex" => Self::ViewIndex,
            "ShadingRateKHR" => Self::ShadingRateKHR,
            "TileOffsetQCOM" => Self::TileOffsetQCOM,
            "TileDimensionQCOM" => Self::TileDimensionQCOM,
            "TileApronSizeQCOM" => Self::TileApronSizeQCOM,
            "BaryCoordNoPerspAMD" => Self::BaryCoordNoPerspAMD,
            "BaryCoordNoPerspCentroidAMD" => Self::BaryCoordNoPerspCentroidAMD,
            "BaryCoordNoPerspSampleAMD" => Self::BaryCoordNoPerspSampleAMD,
            "BaryCoordSmoothAMD" => Self::BaryCoordSmoothAMD,
            "BaryCoordSmoothCentroidAMD" => Self::BaryCoordSmoothCentroidAMD,
            "BaryCoordSmoothSampleAMD" => Self::BaryCoordSmoothSampleAMD,
            "BaryCoordPullModelAMD" => Self::BaryCoordPullModelAMD,
            "FragStencilRefEXT" => Self::FragStencilRefEXT,
            "RemainingRecursionLevelsAMDX" => Self::RemainingRecursionLevelsAMDX,
            "ShaderIndexAMDX" => Self::ShaderIndexAMDX,
            "SamplerHeapEXT" => Self::SamplerHeapEXT,
            "ResourceHeapEXT" => Self::ResourceHeapEXT,
            "ViewportMaskNV" => Self::ViewportMaskNV,
            "SecondaryPositionNV" => Self::SecondaryPositionNV,
            "SecondaryViewportMaskNV" => Self::SecondaryViewportMaskNV,
            "PositionPerViewNV" => Self::PositionPerViewNV,
            "ViewportMaskPerViewNV" => Self::ViewportMaskPerViewNV,
            "FullyCoveredEXT" => Self::FullyCoveredEXT,
            "TaskCountNV" => Self::TaskCountNV,
            "PrimitiveCountNV" => Self::PrimitiveCountNV,
            "PrimitiveIndicesNV" => Self::PrimitiveIndicesNV,
            "ClipDistancePerViewNV" => Self::ClipDistancePerViewNV,
            "CullDistancePerViewNV" => Self::CullDistancePerViewNV,
            "LayerPerViewNV" => Self::LayerPerViewNV,
            "MeshViewCountNV" => Self::MeshViewCountNV,
            "MeshViewIndicesNV" => Self::MeshViewIndicesNV,
            "BaryCoordKHR" => Self::BaryCoordKHR,
            "BaryCoordNV" => Self::BaryCoordKHR,
            "BaryCoordNoPerspKHR" => Self::BaryCoordNoPerspKHR,
            "BaryCoordNoPerspNV" => Self::BaryCoordNoPerspKHR,
            "FragSizeEXT" => Self::FragSizeEXT,
            "FragmentSizeNV" => Self::FragSizeEXT,
            "FragInvocationCountEXT" => Self::FragInvocationCountEXT,
            "InvocationsPerPixelNV" => Self::FragInvocationCountEXT,
            "PrimitivePointIndicesEXT" => Self::PrimitivePointIndicesEXT,
            "PrimitiveLineIndicesEXT" => Self::PrimitiveLineIndicesEXT,
            "PrimitiveTriangleIndicesEXT" => Self::PrimitiveTriangleIndicesEXT,
            "CullPrimitiveEXT" => Self::CullPrimitiveEXT,
            "LaunchIdKHR" => Self::LaunchIdKHR,
            "LaunchIdNV" => Self::LaunchIdKHR,
            "LaunchSizeKHR" => Self::LaunchSizeKHR,
            "LaunchSizeNV" => Self::LaunchSizeKHR,
            "WorldRayOriginKHR" => Self::WorldRayOriginKHR,
            "WorldRayOriginNV" => Self::WorldRayOriginKHR,
            "WorldRayDirectionKHR" => Self::WorldRayDirectionKHR,
            "WorldRayDirectionNV" => Self::WorldRayDirectionKHR,
            "ObjectRayOriginKHR" => Self::ObjectRayOriginKHR,
            "ObjectRayOriginNV" => Self::ObjectRayOriginKHR,
            "ObjectRayDirectionKHR" => Self::ObjectRayDirectionKHR,
            "ObjectRayDirectionNV" => Self::ObjectRayDirectionKHR,
            "RayTminKHR" => Self::RayTminKHR,
            "RayTminNV" => Self::RayTminKHR,
            "RayTmaxKHR" => Self::RayTmaxKHR,
            "RayTmaxNV" => Self::RayTmaxKHR,
            "InstanceCustomIndexKHR" => Self::InstanceCustomIndexKHR,
            "InstanceCustomIndexNV" => Self::InstanceCustomIndexKHR,
            "ObjectToWorldKHR" => Self::ObjectToWorldKHR,
            "ObjectToWorldNV" => Self::ObjectToWorldKHR,
            "WorldToObjectKHR" => Self::WorldToObjectKHR,
            "WorldToObjectNV" => Self::WorldToObjectKHR,
            "HitTNV" => Self::HitTNV,
            "HitKindKHR" => Self::HitKindKHR,
            "HitKindNV" => Self::HitKindKHR,
            "CurrentRayTimeNV" => Self::CurrentRayTimeNV,
            "HitTriangleVertexPositionsKHR" => Self::HitTriangleVertexPositionsKHR,
            "HitMicroTriangleVertexPositionsNV" => Self::HitMicroTriangleVertexPositionsNV,
            "HitMicroTriangleVertexBarycentricsNV" => Self::HitMicroTriangleVertexBarycentricsNV,
            "IncomingRayFlagsKHR" => Self::IncomingRayFlagsKHR,
            "IncomingRayFlagsNV" => Self::IncomingRayFlagsKHR,
            "RayGeometryIndexKHR" => Self::RayGeometryIndexKHR,
            "HitIsSphereNV" => Self::HitIsSphereNV,
            "HitIsLSSNV" => Self::HitIsLSSNV,
            "HitSpherePositionNV" => Self::HitSpherePositionNV,
            "WarpsPerSMNV" => Self::WarpsPerSMNV,
            "SMCountNV" => Self::SMCountNV,
            "WarpIDNV" => Self::WarpIDNV,
            "SMIDNV" => Self::SMIDNV,
            "HitLSSPositionsNV" => Self::HitLSSPositionsNV,
            "HitKindFrontFacingMicroTriangleNV" => Self::HitKindFrontFacingMicroTriangleNV,
            "HitKindBackFacingMicroTriangleNV" => Self::HitKindBackFacingMicroTriangleNV,
            "HitSphereRadiusNV" => Self::HitSphereRadiusNV,
            "HitLSSRadiiNV" => Self::HitLSSRadiiNV,
            "ClusterIDNV" => Self::ClusterIDNV,
            "CullMaskKHR" => Self::CullMaskKHR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [Scope](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_scope_a_scope)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Scope {
    CrossDevice = 0u32,
    Device = 1u32,
    Workgroup = 2u32,
    Subgroup = 3u32,
    Invocation = 4u32,
    QueueFamily = 5u32,
    ShaderCallKHR = 6u32,
}
impl Scope {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=6u32 => unsafe { core::mem::transmute::<u32, Scope>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl Scope {
    pub const QueueFamilyKHR: Self = Self::QueueFamily;
}
impl core::str::FromStr for Scope {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CrossDevice" => Self::CrossDevice,
            "Device" => Self::Device,
            "Workgroup" => Self::Workgroup,
            "Subgroup" => Self::Subgroup,
            "Invocation" => Self::Invocation,
            "QueueFamily" => Self::QueueFamily,
            "QueueFamilyKHR" => Self::QueueFamily,
            "ShaderCallKHR" => Self::ShaderCallKHR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [GroupOperation](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_group_operation_a_group_operation)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum GroupOperation {
    Reduce = 0u32,
    InclusiveScan = 1u32,
    ExclusiveScan = 2u32,
    ClusteredReduce = 3u32,
    PartitionedReduceEXT = 6u32,
    PartitionedInclusiveScanEXT = 7u32,
    PartitionedExclusiveScanEXT = 8u32,
}
impl GroupOperation {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, GroupOperation>(n) },
            6u32..=8u32 => unsafe { core::mem::transmute::<u32, GroupOperation>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl GroupOperation {
    pub const PartitionedReduceNV: Self = Self::PartitionedReduceEXT;
    pub const PartitionedInclusiveScanNV: Self = Self::PartitionedInclusiveScanEXT;
    pub const PartitionedExclusiveScanNV: Self = Self::PartitionedExclusiveScanEXT;
}
impl core::str::FromStr for GroupOperation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Reduce" => Self::Reduce,
            "InclusiveScan" => Self::InclusiveScan,
            "ExclusiveScan" => Self::ExclusiveScan,
            "ClusteredReduce" => Self::ClusteredReduce,
            "PartitionedReduceEXT" => Self::PartitionedReduceEXT,
            "PartitionedReduceNV" => Self::PartitionedReduceEXT,
            "PartitionedInclusiveScanEXT" => Self::PartitionedInclusiveScanEXT,
            "PartitionedInclusiveScanNV" => Self::PartitionedInclusiveScanEXT,
            "PartitionedExclusiveScanEXT" => Self::PartitionedExclusiveScanEXT,
            "PartitionedExclusiveScanNV" => Self::PartitionedExclusiveScanEXT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [KernelEnqueueFlags](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_kernel_enqueue_flags_a_kernel_enqueue_flags)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum KernelEnqueueFlags {
    NoWait = 0u32,
    WaitKernel = 1u32,
    WaitWorkGroup = 2u32,
}
impl KernelEnqueueFlags {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, KernelEnqueueFlags>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl KernelEnqueueFlags {}
impl core::str::FromStr for KernelEnqueueFlags {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "NoWait" => Self::NoWait,
            "WaitKernel" => Self::WaitKernel,
            "WaitWorkGroup" => Self::WaitWorkGroup,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [Capability](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_capability_a_capability)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Capability {
    Matrix = 0u32,
    Shader = 1u32,
    Geometry = 2u32,
    Tessellation = 3u32,
    Addresses = 4u32,
    Linkage = 5u32,
    Kernel = 6u32,
    Vector16 = 7u32,
    Float16Buffer = 8u32,
    Float16 = 9u32,
    Float64 = 10u32,
    Int64 = 11u32,
    Int64Atomics = 12u32,
    ImageBasic = 13u32,
    ImageReadWrite = 14u32,
    ImageMipmap = 15u32,
    Pipes = 17u32,
    Groups = 18u32,
    DeviceEnqueue = 19u32,
    LiteralSampler = 20u32,
    AtomicStorage = 21u32,
    Int16 = 22u32,
    TessellationPointSize = 23u32,
    GeometryPointSize = 24u32,
    ImageGatherExtended = 25u32,
    StorageImageMultisample = 27u32,
    UniformBufferArrayDynamicIndexing = 28u32,
    SampledImageArrayDynamicIndexing = 29u32,
    StorageBufferArrayDynamicIndexing = 30u32,
    StorageImageArrayDynamicIndexing = 31u32,
    ClipDistance = 32u32,
    CullDistance = 33u32,
    ImageCubeArray = 34u32,
    SampleRateShading = 35u32,
    ImageRect = 36u32,
    SampledRect = 37u32,
    GenericPointer = 38u32,
    Int8 = 39u32,
    InputAttachment = 40u32,
    SparseResidency = 41u32,
    MinLod = 42u32,
    Sampled1D = 43u32,
    Image1D = 44u32,
    SampledCubeArray = 45u32,
    SampledBuffer = 46u32,
    ImageBuffer = 47u32,
    ImageMSArray = 48u32,
    StorageImageExtendedFormats = 49u32,
    ImageQuery = 50u32,
    DerivativeControl = 51u32,
    InterpolationFunction = 52u32,
    TransformFeedback = 53u32,
    GeometryStreams = 54u32,
    StorageImageReadWithoutFormat = 55u32,
    StorageImageWriteWithoutFormat = 56u32,
    MultiViewport = 57u32,
    SubgroupDispatch = 58u32,
    NamedBarrier = 59u32,
    PipeStorage = 60u32,
    GroupNonUniform = 61u32,
    GroupNonUniformVote = 62u32,
    GroupNonUniformArithmetic = 63u32,
    GroupNonUniformBallot = 64u32,
    GroupNonUniformShuffle = 65u32,
    GroupNonUniformShuffleRelative = 66u32,
    GroupNonUniformClustered = 67u32,
    GroupNonUniformQuad = 68u32,
    ShaderLayer = 69u32,
    ShaderViewportIndex = 70u32,
    UniformDecoration = 71u32,
    CoreBuiltinsARM = 4165u32,
    TileImageColorReadAccessEXT = 4166u32,
    TileImageDepthReadAccessEXT = 4167u32,
    TileImageStencilReadAccessEXT = 4168u32,
    TensorsARM = 4174u32,
    StorageTensorArrayDynamicIndexingARM = 4175u32,
    StorageTensorArrayNonUniformIndexingARM = 4176u32,
    GraphARM = 4191u32,
    CooperativeMatrixLayoutsARM = 4201u32,
    Float8EXT = 4212u32,
    Float8CooperativeMatrixEXT = 4213u32,
    FragmentShadingRateKHR = 4422u32,
    SubgroupBallotKHR = 4423u32,
    DrawParameters = 4427u32,
    WorkgroupMemoryExplicitLayoutKHR = 4428u32,
    WorkgroupMemoryExplicitLayout8BitAccessKHR = 4429u32,
    WorkgroupMemoryExplicitLayout16BitAccessKHR = 4430u32,
    SubgroupVoteKHR = 4431u32,
    StorageBuffer16BitAccess = 4433u32,
    UniformAndStorageBuffer16BitAccess = 4434u32,
    StoragePushConstant16 = 4435u32,
    StorageInputOutput16 = 4436u32,
    DeviceGroup = 4437u32,
    MultiView = 4439u32,
    VariablePointersStorageBuffer = 4441u32,
    VariablePointers = 4442u32,
    AtomicStorageOps = 4445u32,
    SampleMaskPostDepthCoverage = 4447u32,
    StorageBuffer8BitAccess = 4448u32,
    UniformAndStorageBuffer8BitAccess = 4449u32,
    StoragePushConstant8 = 4450u32,
    DenormPreserve = 4464u32,
    DenormFlushToZero = 4465u32,
    SignedZeroInfNanPreserve = 4466u32,
    RoundingModeRTE = 4467u32,
    RoundingModeRTZ = 4468u32,
    RayQueryProvisionalKHR = 4471u32,
    RayQueryKHR = 4472u32,
    UntypedPointersKHR = 4473u32,
    RayTraversalPrimitiveCullingKHR = 4478u32,
    RayTracingKHR = 4479u32,
    TextureSampleWeightedQCOM = 4484u32,
    TextureBoxFilterQCOM = 4485u32,
    TextureBlockMatchQCOM = 4486u32,
    TileShadingQCOM = 4495u32,
    CooperativeMatrixConversionQCOM = 4496u32,
    TextureBlockMatch2QCOM = 4498u32,
    Float16ImageAMD = 5008u32,
    ImageGatherBiasLodAMD = 5009u32,
    FragmentMaskAMD = 5010u32,
    StencilExportEXT = 5013u32,
    ImageReadWriteLodAMD = 5015u32,
    Int64ImageEXT = 5016u32,
    ShaderClockKHR = 5055u32,
    ShaderEnqueueAMDX = 5067u32,
    QuadControlKHR = 5087u32,
    Int4TypeINTEL = 5112u32,
    Int4CooperativeMatrixINTEL = 5114u32,
    BFloat16TypeKHR = 5116u32,
    BFloat16DotProductKHR = 5117u32,
    BFloat16CooperativeMatrixKHR = 5118u32,
    DescriptorHeapEXT = 5128u32,
    SampleMaskOverrideCoverageNV = 5249u32,
    GeometryShaderPassthroughNV = 5251u32,
    ShaderViewportIndexLayerEXT = 5254u32,
    ShaderViewportMaskNV = 5255u32,
    ShaderStereoViewNV = 5259u32,
    PerViewAttributesNV = 5260u32,
    FragmentFullyCoveredEXT = 5265u32,
    MeshShadingNV = 5266u32,
    ImageFootprintNV = 5282u32,
    MeshShadingEXT = 5283u32,
    FragmentBarycentricKHR = 5284u32,
    ComputeDerivativeGroupQuadsKHR = 5288u32,
    FragmentDensityEXT = 5291u32,
    GroupNonUniformPartitionedEXT = 5297u32,
    ShaderNonUniform = 5301u32,
    RuntimeDescriptorArray = 5302u32,
    InputAttachmentArrayDynamicIndexing = 5303u32,
    UniformTexelBufferArrayDynamicIndexing = 5304u32,
    StorageTexelBufferArrayDynamicIndexing = 5305u32,
    UniformBufferArrayNonUniformIndexing = 5306u32,
    SampledImageArrayNonUniformIndexing = 5307u32,
    StorageBufferArrayNonUniformIndexing = 5308u32,
    StorageImageArrayNonUniformIndexing = 5309u32,
    InputAttachmentArrayNonUniformIndexing = 5310u32,
    UniformTexelBufferArrayNonUniformIndexing = 5311u32,
    StorageTexelBufferArrayNonUniformIndexing = 5312u32,
    RayTracingPositionFetchKHR = 5336u32,
    RayTracingNV = 5340u32,
    RayTracingMotionBlurNV = 5341u32,
    VulkanMemoryModel = 5345u32,
    VulkanMemoryModelDeviceScope = 5346u32,
    PhysicalStorageBufferAddresses = 5347u32,
    ComputeDerivativeGroupLinearKHR = 5350u32,
    RayTracingProvisionalKHR = 5353u32,
    CooperativeMatrixNV = 5357u32,
    FragmentShaderSampleInterlockEXT = 5363u32,
    FragmentShaderShadingRateInterlockEXT = 5372u32,
    ShaderSMBuiltinsNV = 5373u32,
    FragmentShaderPixelInterlockEXT = 5378u32,
    DemoteToHelperInvocation = 5379u32,
    DisplacementMicromapNV = 5380u32,
    RayTracingOpacityMicromapEXT = 5381u32,
    ShaderInvocationReorderNV = 5383u32,
    ShaderInvocationReorderEXT = 5388u32,
    BindlessTextureNV = 5390u32,
    RayQueryPositionFetchKHR = 5391u32,
    CooperativeVectorNV = 5394u32,
    AtomicFloat16VectorNV = 5404u32,
    RayTracingDisplacementMicromapNV = 5409u32,
    RawAccessChainsNV = 5414u32,
    RayTracingSpheresGeometryNV = 5418u32,
    RayTracingLinearSweptSpheresGeometryNV = 5419u32,
    PushConstantBanksNV = 5423u32,
    LongVectorEXT = 5425u32,
    Shader64BitIndexingEXT = 5426u32,
    CooperativeMatrixReductionsNV = 5430u32,
    CooperativeMatrixConversionsNV = 5431u32,
    CooperativeMatrixPerElementOperationsNV = 5432u32,
    CooperativeMatrixTensorAddressingNV = 5433u32,
    CooperativeMatrixBlockLoadsNV = 5434u32,
    CooperativeVectorTrainingNV = 5435u32,
    RayTracingClusterAccelerationStructureNV = 5437u32,
    TensorAddressingNV = 5439u32,
    SubgroupShuffleINTEL = 5568u32,
    SubgroupBufferBlockIOINTEL = 5569u32,
    SubgroupImageBlockIOINTEL = 5570u32,
    SubgroupImageMediaBlockIOINTEL = 5579u32,
    RoundToInfinityINTEL = 5582u32,
    FloatingPointModeINTEL = 5583u32,
    IntegerFunctions2INTEL = 5584u32,
    FunctionPointersINTEL = 5603u32,
    IndirectReferencesINTEL = 5604u32,
    AsmINTEL = 5606u32,
    AtomicFloat32MinMaxEXT = 5612u32,
    AtomicFloat64MinMaxEXT = 5613u32,
    AtomicFloat16MinMaxEXT = 5616u32,
    VectorComputeINTEL = 5617u32,
    VectorAnyINTEL = 5619u32,
    ExpectAssumeKHR = 5629u32,
    SubgroupAvcMotionEstimationINTEL = 5696u32,
    SubgroupAvcMotionEstimationIntraINTEL = 5697u32,
    SubgroupAvcMotionEstimationChromaINTEL = 5698u32,
    VariableLengthArrayINTEL = 5817u32,
    FunctionFloatControlINTEL = 5821u32,
    FPGAMemoryAttributesALTERA = 5824u32,
    FPFastMathModeINTEL = 5837u32,
    ArbitraryPrecisionIntegersALTERA = 5844u32,
    ArbitraryPrecisionFloatingPointALTERA = 5845u32,
    UnstructuredLoopControlsINTEL = 5886u32,
    FPGALoopControlsALTERA = 5888u32,
    KernelAttributesINTEL = 5892u32,
    FPGAKernelAttributesINTEL = 5897u32,
    FPGAMemoryAccessesALTERA = 5898u32,
    FPGAClusterAttributesALTERA = 5904u32,
    LoopFuseALTERA = 5906u32,
    FPGADSPControlALTERA = 5908u32,
    MemoryAccessAliasingINTEL = 5910u32,
    FPGAInvocationPipeliningAttributesALTERA = 5916u32,
    FPGABufferLocationALTERA = 5920u32,
    ArbitraryPrecisionFixedPointALTERA = 5922u32,
    USMStorageClassesALTERA = 5935u32,
    RuntimeAlignedAttributeALTERA = 5939u32,
    IOPipesALTERA = 5943u32,
    BlockingPipesALTERA = 5945u32,
    FPGARegALTERA = 5948u32,
    DotProductInputAll = 6016u32,
    DotProductInput4x8Bit = 6017u32,
    DotProductInput4x8BitPacked = 6018u32,
    DotProduct = 6019u32,
    RayCullMaskKHR = 6020u32,
    CooperativeMatrixKHR = 6022u32,
    ReplicatedCompositesEXT = 6024u32,
    BitInstructions = 6025u32,
    GroupNonUniformRotateKHR = 6026u32,
    FloatControls2 = 6029u32,
    FMAKHR = 6030u32,
    AtomicFloat32AddEXT = 6033u32,
    AtomicFloat64AddEXT = 6034u32,
    LongCompositesINTEL = 6089u32,
    OptNoneEXT = 6094u32,
    AtomicFloat16AddEXT = 6095u32,
    DebugInfoModuleINTEL = 6114u32,
    BFloat16ConversionINTEL = 6115u32,
    SplitBarrierINTEL = 6141u32,
    ArithmeticFenceEXT = 6144u32,
    FPGAClusterAttributesV2ALTERA = 6150u32,
    FPGAKernelAttributesv2INTEL = 6161u32,
    TaskSequenceALTERA = 6162u32,
    FPMaxErrorINTEL = 6169u32,
    FPGALatencyControlALTERA = 6171u32,
    FPGAArgumentInterfacesALTERA = 6174u32,
    GlobalVariableHostAccessINTEL = 6187u32,
    GlobalVariableFPGADecorationsALTERA = 6189u32,
    SubgroupBufferPrefetchINTEL = 6220u32,
    Subgroup2DBlockIOINTEL = 6228u32,
    Subgroup2DBlockTransformINTEL = 6229u32,
    Subgroup2DBlockTransposeINTEL = 6230u32,
    SubgroupMatrixMultiplyAccumulateINTEL = 6236u32,
    TernaryBitwiseFunctionINTEL = 6241u32,
    UntypedVariableLengthArrayINTEL = 6243u32,
    SpecConditionalINTEL = 6245u32,
    FunctionVariantsINTEL = 6246u32,
    GroupUniformArithmeticKHR = 6400u32,
    TensorFloat32RoundingINTEL = 6425u32,
    MaskedGatherScatterINTEL = 6427u32,
    CacheControlsINTEL = 6441u32,
    RegisterLimitsINTEL = 6460u32,
    BindlessImagesINTEL = 6528u32,
}
impl Capability {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=15u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            17u32..=25u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            27u32..=71u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4165u32..=4168u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4174u32..=4176u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4191u32 => unsafe { core::mem::transmute::<u32, Capability>(4191u32) },
            4201u32 => unsafe { core::mem::transmute::<u32, Capability>(4201u32) },
            4212u32..=4213u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4422u32..=4423u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4427u32..=4431u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4433u32..=4437u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4439u32 => unsafe { core::mem::transmute::<u32, Capability>(4439u32) },
            4441u32..=4442u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4445u32 => unsafe { core::mem::transmute::<u32, Capability>(4445u32) },
            4447u32..=4450u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4464u32..=4468u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4471u32..=4473u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4478u32..=4479u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4484u32..=4486u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4495u32..=4496u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            4498u32 => unsafe { core::mem::transmute::<u32, Capability>(4498u32) },
            5008u32..=5010u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5013u32 => unsafe { core::mem::transmute::<u32, Capability>(5013u32) },
            5015u32..=5016u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5055u32 => unsafe { core::mem::transmute::<u32, Capability>(5055u32) },
            5067u32 => unsafe { core::mem::transmute::<u32, Capability>(5067u32) },
            5087u32 => unsafe { core::mem::transmute::<u32, Capability>(5087u32) },
            5112u32 => unsafe { core::mem::transmute::<u32, Capability>(5112u32) },
            5114u32 => unsafe { core::mem::transmute::<u32, Capability>(5114u32) },
            5116u32..=5118u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5128u32 => unsafe { core::mem::transmute::<u32, Capability>(5128u32) },
            5249u32 => unsafe { core::mem::transmute::<u32, Capability>(5249u32) },
            5251u32 => unsafe { core::mem::transmute::<u32, Capability>(5251u32) },
            5254u32..=5255u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5259u32..=5260u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5265u32..=5266u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5282u32..=5284u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5288u32 => unsafe { core::mem::transmute::<u32, Capability>(5288u32) },
            5291u32 => unsafe { core::mem::transmute::<u32, Capability>(5291u32) },
            5297u32 => unsafe { core::mem::transmute::<u32, Capability>(5297u32) },
            5301u32..=5312u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5336u32 => unsafe { core::mem::transmute::<u32, Capability>(5336u32) },
            5340u32..=5341u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5345u32..=5347u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5350u32 => unsafe { core::mem::transmute::<u32, Capability>(5350u32) },
            5353u32 => unsafe { core::mem::transmute::<u32, Capability>(5353u32) },
            5357u32 => unsafe { core::mem::transmute::<u32, Capability>(5357u32) },
            5363u32 => unsafe { core::mem::transmute::<u32, Capability>(5363u32) },
            5372u32..=5373u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5378u32..=5381u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5383u32 => unsafe { core::mem::transmute::<u32, Capability>(5383u32) },
            5388u32 => unsafe { core::mem::transmute::<u32, Capability>(5388u32) },
            5390u32..=5391u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5394u32 => unsafe { core::mem::transmute::<u32, Capability>(5394u32) },
            5404u32 => unsafe { core::mem::transmute::<u32, Capability>(5404u32) },
            5409u32 => unsafe { core::mem::transmute::<u32, Capability>(5409u32) },
            5414u32 => unsafe { core::mem::transmute::<u32, Capability>(5414u32) },
            5418u32..=5419u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5423u32 => unsafe { core::mem::transmute::<u32, Capability>(5423u32) },
            5425u32..=5426u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5430u32..=5435u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5437u32 => unsafe { core::mem::transmute::<u32, Capability>(5437u32) },
            5439u32 => unsafe { core::mem::transmute::<u32, Capability>(5439u32) },
            5568u32..=5570u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5579u32 => unsafe { core::mem::transmute::<u32, Capability>(5579u32) },
            5582u32..=5584u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5603u32..=5604u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5606u32 => unsafe { core::mem::transmute::<u32, Capability>(5606u32) },
            5612u32..=5613u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5616u32..=5617u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5619u32 => unsafe { core::mem::transmute::<u32, Capability>(5619u32) },
            5629u32 => unsafe { core::mem::transmute::<u32, Capability>(5629u32) },
            5696u32..=5698u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5817u32 => unsafe { core::mem::transmute::<u32, Capability>(5817u32) },
            5821u32 => unsafe { core::mem::transmute::<u32, Capability>(5821u32) },
            5824u32 => unsafe { core::mem::transmute::<u32, Capability>(5824u32) },
            5837u32 => unsafe { core::mem::transmute::<u32, Capability>(5837u32) },
            5844u32..=5845u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5886u32 => unsafe { core::mem::transmute::<u32, Capability>(5886u32) },
            5888u32 => unsafe { core::mem::transmute::<u32, Capability>(5888u32) },
            5892u32 => unsafe { core::mem::transmute::<u32, Capability>(5892u32) },
            5897u32..=5898u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            5904u32 => unsafe { core::mem::transmute::<u32, Capability>(5904u32) },
            5906u32 => unsafe { core::mem::transmute::<u32, Capability>(5906u32) },
            5908u32 => unsafe { core::mem::transmute::<u32, Capability>(5908u32) },
            5910u32 => unsafe { core::mem::transmute::<u32, Capability>(5910u32) },
            5916u32 => unsafe { core::mem::transmute::<u32, Capability>(5916u32) },
            5920u32 => unsafe { core::mem::transmute::<u32, Capability>(5920u32) },
            5922u32 => unsafe { core::mem::transmute::<u32, Capability>(5922u32) },
            5935u32 => unsafe { core::mem::transmute::<u32, Capability>(5935u32) },
            5939u32 => unsafe { core::mem::transmute::<u32, Capability>(5939u32) },
            5943u32 => unsafe { core::mem::transmute::<u32, Capability>(5943u32) },
            5945u32 => unsafe { core::mem::transmute::<u32, Capability>(5945u32) },
            5948u32 => unsafe { core::mem::transmute::<u32, Capability>(5948u32) },
            6016u32..=6020u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6022u32 => unsafe { core::mem::transmute::<u32, Capability>(6022u32) },
            6024u32..=6026u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6029u32..=6030u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6033u32..=6034u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6089u32 => unsafe { core::mem::transmute::<u32, Capability>(6089u32) },
            6094u32..=6095u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6114u32..=6115u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6141u32 => unsafe { core::mem::transmute::<u32, Capability>(6141u32) },
            6144u32 => unsafe { core::mem::transmute::<u32, Capability>(6144u32) },
            6150u32 => unsafe { core::mem::transmute::<u32, Capability>(6150u32) },
            6161u32..=6162u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6169u32 => unsafe { core::mem::transmute::<u32, Capability>(6169u32) },
            6171u32 => unsafe { core::mem::transmute::<u32, Capability>(6171u32) },
            6174u32 => unsafe { core::mem::transmute::<u32, Capability>(6174u32) },
            6187u32 => unsafe { core::mem::transmute::<u32, Capability>(6187u32) },
            6189u32 => unsafe { core::mem::transmute::<u32, Capability>(6189u32) },
            6220u32 => unsafe { core::mem::transmute::<u32, Capability>(6220u32) },
            6228u32..=6230u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6236u32 => unsafe { core::mem::transmute::<u32, Capability>(6236u32) },
            6241u32 => unsafe { core::mem::transmute::<u32, Capability>(6241u32) },
            6243u32 => unsafe { core::mem::transmute::<u32, Capability>(6243u32) },
            6245u32..=6246u32 => unsafe { core::mem::transmute::<u32, Capability>(n) },
            6400u32 => unsafe { core::mem::transmute::<u32, Capability>(6400u32) },
            6425u32 => unsafe { core::mem::transmute::<u32, Capability>(6425u32) },
            6427u32 => unsafe { core::mem::transmute::<u32, Capability>(6427u32) },
            6441u32 => unsafe { core::mem::transmute::<u32, Capability>(6441u32) },
            6460u32 => unsafe { core::mem::transmute::<u32, Capability>(6460u32) },
            6528u32 => unsafe { core::mem::transmute::<u32, Capability>(6528u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl Capability {
    pub const StorageUniformBufferBlock16: Self = Self::StorageBuffer16BitAccess;
    pub const StorageUniform16: Self = Self::UniformAndStorageBuffer16BitAccess;
    pub const ShaderViewportIndexLayerNV: Self = Self::ShaderViewportIndexLayerEXT;
    pub const FragmentBarycentricNV: Self = Self::FragmentBarycentricKHR;
    pub const ComputeDerivativeGroupQuadsNV: Self = Self::ComputeDerivativeGroupQuadsKHR;
    pub const ShadingRateNV: Self = Self::FragmentDensityEXT;
    pub const GroupNonUniformPartitionedNV: Self = Self::GroupNonUniformPartitionedEXT;
    pub const ShaderNonUniformEXT: Self = Self::ShaderNonUniform;
    pub const RuntimeDescriptorArrayEXT: Self = Self::RuntimeDescriptorArray;
    pub const InputAttachmentArrayDynamicIndexingEXT: Self =
        Self::InputAttachmentArrayDynamicIndexing;
    pub const UniformTexelBufferArrayDynamicIndexingEXT: Self =
        Self::UniformTexelBufferArrayDynamicIndexing;
    pub const StorageTexelBufferArrayDynamicIndexingEXT: Self =
        Self::StorageTexelBufferArrayDynamicIndexing;
    pub const UniformBufferArrayNonUniformIndexingEXT: Self =
        Self::UniformBufferArrayNonUniformIndexing;
    pub const SampledImageArrayNonUniformIndexingEXT: Self =
        Self::SampledImageArrayNonUniformIndexing;
    pub const StorageBufferArrayNonUniformIndexingEXT: Self =
        Self::StorageBufferArrayNonUniformIndexing;
    pub const StorageImageArrayNonUniformIndexingEXT: Self =
        Self::StorageImageArrayNonUniformIndexing;
    pub const InputAttachmentArrayNonUniformIndexingEXT: Self =
        Self::InputAttachmentArrayNonUniformIndexing;
    pub const UniformTexelBufferArrayNonUniformIndexingEXT: Self =
        Self::UniformTexelBufferArrayNonUniformIndexing;
    pub const StorageTexelBufferArrayNonUniformIndexingEXT: Self =
        Self::StorageTexelBufferArrayNonUniformIndexing;
    pub const VulkanMemoryModelKHR: Self = Self::VulkanMemoryModel;
    pub const VulkanMemoryModelDeviceScopeKHR: Self = Self::VulkanMemoryModelDeviceScope;
    pub const PhysicalStorageBufferAddressesEXT: Self = Self::PhysicalStorageBufferAddresses;
    pub const ComputeDerivativeGroupLinearNV: Self = Self::ComputeDerivativeGroupLinearKHR;
    pub const DemoteToHelperInvocationEXT: Self = Self::DemoteToHelperInvocation;
    pub const FPGAMemoryAttributesINTEL: Self = Self::FPGAMemoryAttributesALTERA;
    pub const ArbitraryPrecisionIntegersINTEL: Self = Self::ArbitraryPrecisionIntegersALTERA;
    pub const ArbitraryPrecisionFloatingPointINTEL: Self =
        Self::ArbitraryPrecisionFloatingPointALTERA;
    pub const FPGALoopControlsINTEL: Self = Self::FPGALoopControlsALTERA;
    pub const FPGAMemoryAccessesINTEL: Self = Self::FPGAMemoryAccessesALTERA;
    pub const FPGAClusterAttributesINTEL: Self = Self::FPGAClusterAttributesALTERA;
    pub const LoopFuseINTEL: Self = Self::LoopFuseALTERA;
    pub const FPGADSPControlINTEL: Self = Self::FPGADSPControlALTERA;
    pub const FPGAInvocationPipeliningAttributesINTEL: Self =
        Self::FPGAInvocationPipeliningAttributesALTERA;
    pub const FPGABufferLocationINTEL: Self = Self::FPGABufferLocationALTERA;
    pub const ArbitraryPrecisionFixedPointINTEL: Self = Self::ArbitraryPrecisionFixedPointALTERA;
    pub const USMStorageClassesINTEL: Self = Self::USMStorageClassesALTERA;
    pub const RuntimeAlignedAttributeINTEL: Self = Self::RuntimeAlignedAttributeALTERA;
    pub const IOPipesINTEL: Self = Self::IOPipesALTERA;
    pub const BlockingPipesINTEL: Self = Self::BlockingPipesALTERA;
    pub const FPGARegINTEL: Self = Self::FPGARegALTERA;
    pub const DotProductInputAllKHR: Self = Self::DotProductInputAll;
    pub const DotProductInput4x8BitKHR: Self = Self::DotProductInput4x8Bit;
    pub const DotProductInput4x8BitPackedKHR: Self = Self::DotProductInput4x8BitPacked;
    pub const DotProductKHR: Self = Self::DotProduct;
    pub const OptNoneINTEL: Self = Self::OptNoneEXT;
    pub const FPGAClusterAttributesV2INTEL: Self = Self::FPGAClusterAttributesV2ALTERA;
    pub const TaskSequenceINTEL: Self = Self::TaskSequenceALTERA;
    pub const FPGALatencyControlINTEL: Self = Self::FPGALatencyControlALTERA;
    pub const FPGAArgumentInterfacesINTEL: Self = Self::FPGAArgumentInterfacesALTERA;
    pub const GlobalVariableFPGADecorationsINTEL: Self = Self::GlobalVariableFPGADecorationsALTERA;
}
impl core::str::FromStr for Capability {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Matrix" => Self::Matrix,
            "Shader" => Self::Shader,
            "Geometry" => Self::Geometry,
            "Tessellation" => Self::Tessellation,
            "Addresses" => Self::Addresses,
            "Linkage" => Self::Linkage,
            "Kernel" => Self::Kernel,
            "Vector16" => Self::Vector16,
            "Float16Buffer" => Self::Float16Buffer,
            "Float16" => Self::Float16,
            "Float64" => Self::Float64,
            "Int64" => Self::Int64,
            "Int64Atomics" => Self::Int64Atomics,
            "ImageBasic" => Self::ImageBasic,
            "ImageReadWrite" => Self::ImageReadWrite,
            "ImageMipmap" => Self::ImageMipmap,
            "Pipes" => Self::Pipes,
            "Groups" => Self::Groups,
            "DeviceEnqueue" => Self::DeviceEnqueue,
            "LiteralSampler" => Self::LiteralSampler,
            "AtomicStorage" => Self::AtomicStorage,
            "Int16" => Self::Int16,
            "TessellationPointSize" => Self::TessellationPointSize,
            "GeometryPointSize" => Self::GeometryPointSize,
            "ImageGatherExtended" => Self::ImageGatherExtended,
            "StorageImageMultisample" => Self::StorageImageMultisample,
            "UniformBufferArrayDynamicIndexing" => Self::UniformBufferArrayDynamicIndexing,
            "SampledImageArrayDynamicIndexing" => Self::SampledImageArrayDynamicIndexing,
            "StorageBufferArrayDynamicIndexing" => Self::StorageBufferArrayDynamicIndexing,
            "StorageImageArrayDynamicIndexing" => Self::StorageImageArrayDynamicIndexing,
            "ClipDistance" => Self::ClipDistance,
            "CullDistance" => Self::CullDistance,
            "ImageCubeArray" => Self::ImageCubeArray,
            "SampleRateShading" => Self::SampleRateShading,
            "ImageRect" => Self::ImageRect,
            "SampledRect" => Self::SampledRect,
            "GenericPointer" => Self::GenericPointer,
            "Int8" => Self::Int8,
            "InputAttachment" => Self::InputAttachment,
            "SparseResidency" => Self::SparseResidency,
            "MinLod" => Self::MinLod,
            "Sampled1D" => Self::Sampled1D,
            "Image1D" => Self::Image1D,
            "SampledCubeArray" => Self::SampledCubeArray,
            "SampledBuffer" => Self::SampledBuffer,
            "ImageBuffer" => Self::ImageBuffer,
            "ImageMSArray" => Self::ImageMSArray,
            "StorageImageExtendedFormats" => Self::StorageImageExtendedFormats,
            "ImageQuery" => Self::ImageQuery,
            "DerivativeControl" => Self::DerivativeControl,
            "InterpolationFunction" => Self::InterpolationFunction,
            "TransformFeedback" => Self::TransformFeedback,
            "GeometryStreams" => Self::GeometryStreams,
            "StorageImageReadWithoutFormat" => Self::StorageImageReadWithoutFormat,
            "StorageImageWriteWithoutFormat" => Self::StorageImageWriteWithoutFormat,
            "MultiViewport" => Self::MultiViewport,
            "SubgroupDispatch" => Self::SubgroupDispatch,
            "NamedBarrier" => Self::NamedBarrier,
            "PipeStorage" => Self::PipeStorage,
            "GroupNonUniform" => Self::GroupNonUniform,
            "GroupNonUniformVote" => Self::GroupNonUniformVote,
            "GroupNonUniformArithmetic" => Self::GroupNonUniformArithmetic,
            "GroupNonUniformBallot" => Self::GroupNonUniformBallot,
            "GroupNonUniformShuffle" => Self::GroupNonUniformShuffle,
            "GroupNonUniformShuffleRelative" => Self::GroupNonUniformShuffleRelative,
            "GroupNonUniformClustered" => Self::GroupNonUniformClustered,
            "GroupNonUniformQuad" => Self::GroupNonUniformQuad,
            "ShaderLayer" => Self::ShaderLayer,
            "ShaderViewportIndex" => Self::ShaderViewportIndex,
            "UniformDecoration" => Self::UniformDecoration,
            "CoreBuiltinsARM" => Self::CoreBuiltinsARM,
            "TileImageColorReadAccessEXT" => Self::TileImageColorReadAccessEXT,
            "TileImageDepthReadAccessEXT" => Self::TileImageDepthReadAccessEXT,
            "TileImageStencilReadAccessEXT" => Self::TileImageStencilReadAccessEXT,
            "TensorsARM" => Self::TensorsARM,
            "StorageTensorArrayDynamicIndexingARM" => Self::StorageTensorArrayDynamicIndexingARM,
            "StorageTensorArrayNonUniformIndexingARM" => {
                Self::StorageTensorArrayNonUniformIndexingARM
            }
            "GraphARM" => Self::GraphARM,
            "CooperativeMatrixLayoutsARM" => Self::CooperativeMatrixLayoutsARM,
            "Float8EXT" => Self::Float8EXT,
            "Float8CooperativeMatrixEXT" => Self::Float8CooperativeMatrixEXT,
            "FragmentShadingRateKHR" => Self::FragmentShadingRateKHR,
            "SubgroupBallotKHR" => Self::SubgroupBallotKHR,
            "DrawParameters" => Self::DrawParameters,
            "WorkgroupMemoryExplicitLayoutKHR" => Self::WorkgroupMemoryExplicitLayoutKHR,
            "WorkgroupMemoryExplicitLayout8BitAccessKHR" => {
                Self::WorkgroupMemoryExplicitLayout8BitAccessKHR
            }
            "WorkgroupMemoryExplicitLayout16BitAccessKHR" => {
                Self::WorkgroupMemoryExplicitLayout16BitAccessKHR
            }
            "SubgroupVoteKHR" => Self::SubgroupVoteKHR,
            "StorageBuffer16BitAccess" => Self::StorageBuffer16BitAccess,
            "StorageUniformBufferBlock16" => Self::StorageBuffer16BitAccess,
            "UniformAndStorageBuffer16BitAccess" => Self::UniformAndStorageBuffer16BitAccess,
            "StorageUniform16" => Self::UniformAndStorageBuffer16BitAccess,
            "StoragePushConstant16" => Self::StoragePushConstant16,
            "StorageInputOutput16" => Self::StorageInputOutput16,
            "DeviceGroup" => Self::DeviceGroup,
            "MultiView" => Self::MultiView,
            "VariablePointersStorageBuffer" => Self::VariablePointersStorageBuffer,
            "VariablePointers" => Self::VariablePointers,
            "AtomicStorageOps" => Self::AtomicStorageOps,
            "SampleMaskPostDepthCoverage" => Self::SampleMaskPostDepthCoverage,
            "StorageBuffer8BitAccess" => Self::StorageBuffer8BitAccess,
            "UniformAndStorageBuffer8BitAccess" => Self::UniformAndStorageBuffer8BitAccess,
            "StoragePushConstant8" => Self::StoragePushConstant8,
            "DenormPreserve" => Self::DenormPreserve,
            "DenormFlushToZero" => Self::DenormFlushToZero,
            "SignedZeroInfNanPreserve" => Self::SignedZeroInfNanPreserve,
            "RoundingModeRTE" => Self::RoundingModeRTE,
            "RoundingModeRTZ" => Self::RoundingModeRTZ,
            "RayQueryProvisionalKHR" => Self::RayQueryProvisionalKHR,
            "RayQueryKHR" => Self::RayQueryKHR,
            "UntypedPointersKHR" => Self::UntypedPointersKHR,
            "RayTraversalPrimitiveCullingKHR" => Self::RayTraversalPrimitiveCullingKHR,
            "RayTracingKHR" => Self::RayTracingKHR,
            "TextureSampleWeightedQCOM" => Self::TextureSampleWeightedQCOM,
            "TextureBoxFilterQCOM" => Self::TextureBoxFilterQCOM,
            "TextureBlockMatchQCOM" => Self::TextureBlockMatchQCOM,
            "TileShadingQCOM" => Self::TileShadingQCOM,
            "CooperativeMatrixConversionQCOM" => Self::CooperativeMatrixConversionQCOM,
            "TextureBlockMatch2QCOM" => Self::TextureBlockMatch2QCOM,
            "Float16ImageAMD" => Self::Float16ImageAMD,
            "ImageGatherBiasLodAMD" => Self::ImageGatherBiasLodAMD,
            "FragmentMaskAMD" => Self::FragmentMaskAMD,
            "StencilExportEXT" => Self::StencilExportEXT,
            "ImageReadWriteLodAMD" => Self::ImageReadWriteLodAMD,
            "Int64ImageEXT" => Self::Int64ImageEXT,
            "ShaderClockKHR" => Self::ShaderClockKHR,
            "ShaderEnqueueAMDX" => Self::ShaderEnqueueAMDX,
            "QuadControlKHR" => Self::QuadControlKHR,
            "Int4TypeINTEL" => Self::Int4TypeINTEL,
            "Int4CooperativeMatrixINTEL" => Self::Int4CooperativeMatrixINTEL,
            "BFloat16TypeKHR" => Self::BFloat16TypeKHR,
            "BFloat16DotProductKHR" => Self::BFloat16DotProductKHR,
            "BFloat16CooperativeMatrixKHR" => Self::BFloat16CooperativeMatrixKHR,
            "DescriptorHeapEXT" => Self::DescriptorHeapEXT,
            "SampleMaskOverrideCoverageNV" => Self::SampleMaskOverrideCoverageNV,
            "GeometryShaderPassthroughNV" => Self::GeometryShaderPassthroughNV,
            "ShaderViewportIndexLayerEXT" => Self::ShaderViewportIndexLayerEXT,
            "ShaderViewportIndexLayerNV" => Self::ShaderViewportIndexLayerEXT,
            "ShaderViewportMaskNV" => Self::ShaderViewportMaskNV,
            "ShaderStereoViewNV" => Self::ShaderStereoViewNV,
            "PerViewAttributesNV" => Self::PerViewAttributesNV,
            "FragmentFullyCoveredEXT" => Self::FragmentFullyCoveredEXT,
            "MeshShadingNV" => Self::MeshShadingNV,
            "ImageFootprintNV" => Self::ImageFootprintNV,
            "MeshShadingEXT" => Self::MeshShadingEXT,
            "FragmentBarycentricKHR" => Self::FragmentBarycentricKHR,
            "FragmentBarycentricNV" => Self::FragmentBarycentricKHR,
            "ComputeDerivativeGroupQuadsKHR" => Self::ComputeDerivativeGroupQuadsKHR,
            "ComputeDerivativeGroupQuadsNV" => Self::ComputeDerivativeGroupQuadsKHR,
            "FragmentDensityEXT" => Self::FragmentDensityEXT,
            "ShadingRateNV" => Self::FragmentDensityEXT,
            "GroupNonUniformPartitionedEXT" => Self::GroupNonUniformPartitionedEXT,
            "GroupNonUniformPartitionedNV" => Self::GroupNonUniformPartitionedEXT,
            "ShaderNonUniform" => Self::ShaderNonUniform,
            "ShaderNonUniformEXT" => Self::ShaderNonUniform,
            "RuntimeDescriptorArray" => Self::RuntimeDescriptorArray,
            "RuntimeDescriptorArrayEXT" => Self::RuntimeDescriptorArray,
            "InputAttachmentArrayDynamicIndexing" => Self::InputAttachmentArrayDynamicIndexing,
            "InputAttachmentArrayDynamicIndexingEXT" => Self::InputAttachmentArrayDynamicIndexing,
            "UniformTexelBufferArrayDynamicIndexing" => {
                Self::UniformTexelBufferArrayDynamicIndexing
            }
            "UniformTexelBufferArrayDynamicIndexingEXT" => {
                Self::UniformTexelBufferArrayDynamicIndexing
            }
            "StorageTexelBufferArrayDynamicIndexing" => {
                Self::StorageTexelBufferArrayDynamicIndexing
            }
            "StorageTexelBufferArrayDynamicIndexingEXT" => {
                Self::StorageTexelBufferArrayDynamicIndexing
            }
            "UniformBufferArrayNonUniformIndexing" => Self::UniformBufferArrayNonUniformIndexing,
            "UniformBufferArrayNonUniformIndexingEXT" => Self::UniformBufferArrayNonUniformIndexing,
            "SampledImageArrayNonUniformIndexing" => Self::SampledImageArrayNonUniformIndexing,
            "SampledImageArrayNonUniformIndexingEXT" => Self::SampledImageArrayNonUniformIndexing,
            "StorageBufferArrayNonUniformIndexing" => Self::StorageBufferArrayNonUniformIndexing,
            "StorageBufferArrayNonUniformIndexingEXT" => Self::StorageBufferArrayNonUniformIndexing,
            "StorageImageArrayNonUniformIndexing" => Self::StorageImageArrayNonUniformIndexing,
            "StorageImageArrayNonUniformIndexingEXT" => Self::StorageImageArrayNonUniformIndexing,
            "InputAttachmentArrayNonUniformIndexing" => {
                Self::InputAttachmentArrayNonUniformIndexing
            }
            "InputAttachmentArrayNonUniformIndexingEXT" => {
                Self::InputAttachmentArrayNonUniformIndexing
            }
            "UniformTexelBufferArrayNonUniformIndexing" => {
                Self::UniformTexelBufferArrayNonUniformIndexing
            }
            "UniformTexelBufferArrayNonUniformIndexingEXT" => {
                Self::UniformTexelBufferArrayNonUniformIndexing
            }
            "StorageTexelBufferArrayNonUniformIndexing" => {
                Self::StorageTexelBufferArrayNonUniformIndexing
            }
            "StorageTexelBufferArrayNonUniformIndexingEXT" => {
                Self::StorageTexelBufferArrayNonUniformIndexing
            }
            "RayTracingPositionFetchKHR" => Self::RayTracingPositionFetchKHR,
            "RayTracingNV" => Self::RayTracingNV,
            "RayTracingMotionBlurNV" => Self::RayTracingMotionBlurNV,
            "VulkanMemoryModel" => Self::VulkanMemoryModel,
            "VulkanMemoryModelKHR" => Self::VulkanMemoryModel,
            "VulkanMemoryModelDeviceScope" => Self::VulkanMemoryModelDeviceScope,
            "VulkanMemoryModelDeviceScopeKHR" => Self::VulkanMemoryModelDeviceScope,
            "PhysicalStorageBufferAddresses" => Self::PhysicalStorageBufferAddresses,
            "PhysicalStorageBufferAddressesEXT" => Self::PhysicalStorageBufferAddresses,
            "ComputeDerivativeGroupLinearKHR" => Self::ComputeDerivativeGroupLinearKHR,
            "ComputeDerivativeGroupLinearNV" => Self::ComputeDerivativeGroupLinearKHR,
            "RayTracingProvisionalKHR" => Self::RayTracingProvisionalKHR,
            "CooperativeMatrixNV" => Self::CooperativeMatrixNV,
            "FragmentShaderSampleInterlockEXT" => Self::FragmentShaderSampleInterlockEXT,
            "FragmentShaderShadingRateInterlockEXT" => Self::FragmentShaderShadingRateInterlockEXT,
            "ShaderSMBuiltinsNV" => Self::ShaderSMBuiltinsNV,
            "FragmentShaderPixelInterlockEXT" => Self::FragmentShaderPixelInterlockEXT,
            "DemoteToHelperInvocation" => Self::DemoteToHelperInvocation,
            "DemoteToHelperInvocationEXT" => Self::DemoteToHelperInvocation,
            "DisplacementMicromapNV" => Self::DisplacementMicromapNV,
            "RayTracingOpacityMicromapEXT" => Self::RayTracingOpacityMicromapEXT,
            "ShaderInvocationReorderNV" => Self::ShaderInvocationReorderNV,
            "ShaderInvocationReorderEXT" => Self::ShaderInvocationReorderEXT,
            "BindlessTextureNV" => Self::BindlessTextureNV,
            "RayQueryPositionFetchKHR" => Self::RayQueryPositionFetchKHR,
            "CooperativeVectorNV" => Self::CooperativeVectorNV,
            "AtomicFloat16VectorNV" => Self::AtomicFloat16VectorNV,
            "RayTracingDisplacementMicromapNV" => Self::RayTracingDisplacementMicromapNV,
            "RawAccessChainsNV" => Self::RawAccessChainsNV,
            "RayTracingSpheresGeometryNV" => Self::RayTracingSpheresGeometryNV,
            "RayTracingLinearSweptSpheresGeometryNV" => {
                Self::RayTracingLinearSweptSpheresGeometryNV
            }
            "PushConstantBanksNV" => Self::PushConstantBanksNV,
            "LongVectorEXT" => Self::LongVectorEXT,
            "Shader64BitIndexingEXT" => Self::Shader64BitIndexingEXT,
            "CooperativeMatrixReductionsNV" => Self::CooperativeMatrixReductionsNV,
            "CooperativeMatrixConversionsNV" => Self::CooperativeMatrixConversionsNV,
            "CooperativeMatrixPerElementOperationsNV" => {
                Self::CooperativeMatrixPerElementOperationsNV
            }
            "CooperativeMatrixTensorAddressingNV" => Self::CooperativeMatrixTensorAddressingNV,
            "CooperativeMatrixBlockLoadsNV" => Self::CooperativeMatrixBlockLoadsNV,
            "CooperativeVectorTrainingNV" => Self::CooperativeVectorTrainingNV,
            "RayTracingClusterAccelerationStructureNV" => {
                Self::RayTracingClusterAccelerationStructureNV
            }
            "TensorAddressingNV" => Self::TensorAddressingNV,
            "SubgroupShuffleINTEL" => Self::SubgroupShuffleINTEL,
            "SubgroupBufferBlockIOINTEL" => Self::SubgroupBufferBlockIOINTEL,
            "SubgroupImageBlockIOINTEL" => Self::SubgroupImageBlockIOINTEL,
            "SubgroupImageMediaBlockIOINTEL" => Self::SubgroupImageMediaBlockIOINTEL,
            "RoundToInfinityINTEL" => Self::RoundToInfinityINTEL,
            "FloatingPointModeINTEL" => Self::FloatingPointModeINTEL,
            "IntegerFunctions2INTEL" => Self::IntegerFunctions2INTEL,
            "FunctionPointersINTEL" => Self::FunctionPointersINTEL,
            "IndirectReferencesINTEL" => Self::IndirectReferencesINTEL,
            "AsmINTEL" => Self::AsmINTEL,
            "AtomicFloat32MinMaxEXT" => Self::AtomicFloat32MinMaxEXT,
            "AtomicFloat64MinMaxEXT" => Self::AtomicFloat64MinMaxEXT,
            "AtomicFloat16MinMaxEXT" => Self::AtomicFloat16MinMaxEXT,
            "VectorComputeINTEL" => Self::VectorComputeINTEL,
            "VectorAnyINTEL" => Self::VectorAnyINTEL,
            "ExpectAssumeKHR" => Self::ExpectAssumeKHR,
            "SubgroupAvcMotionEstimationINTEL" => Self::SubgroupAvcMotionEstimationINTEL,
            "SubgroupAvcMotionEstimationIntraINTEL" => Self::SubgroupAvcMotionEstimationIntraINTEL,
            "SubgroupAvcMotionEstimationChromaINTEL" => {
                Self::SubgroupAvcMotionEstimationChromaINTEL
            }
            "VariableLengthArrayINTEL" => Self::VariableLengthArrayINTEL,
            "FunctionFloatControlINTEL" => Self::FunctionFloatControlINTEL,
            "FPGAMemoryAttributesALTERA" => Self::FPGAMemoryAttributesALTERA,
            "FPGAMemoryAttributesINTEL" => Self::FPGAMemoryAttributesALTERA,
            "FPFastMathModeINTEL" => Self::FPFastMathModeINTEL,
            "ArbitraryPrecisionIntegersALTERA" => Self::ArbitraryPrecisionIntegersALTERA,
            "ArbitraryPrecisionIntegersINTEL" => Self::ArbitraryPrecisionIntegersALTERA,
            "ArbitraryPrecisionFloatingPointALTERA" => Self::ArbitraryPrecisionFloatingPointALTERA,
            "ArbitraryPrecisionFloatingPointINTEL" => Self::ArbitraryPrecisionFloatingPointALTERA,
            "UnstructuredLoopControlsINTEL" => Self::UnstructuredLoopControlsINTEL,
            "FPGALoopControlsALTERA" => Self::FPGALoopControlsALTERA,
            "FPGALoopControlsINTEL" => Self::FPGALoopControlsALTERA,
            "KernelAttributesINTEL" => Self::KernelAttributesINTEL,
            "FPGAKernelAttributesINTEL" => Self::FPGAKernelAttributesINTEL,
            "FPGAMemoryAccessesALTERA" => Self::FPGAMemoryAccessesALTERA,
            "FPGAMemoryAccessesINTEL" => Self::FPGAMemoryAccessesALTERA,
            "FPGAClusterAttributesALTERA" => Self::FPGAClusterAttributesALTERA,
            "FPGAClusterAttributesINTEL" => Self::FPGAClusterAttributesALTERA,
            "LoopFuseALTERA" => Self::LoopFuseALTERA,
            "LoopFuseINTEL" => Self::LoopFuseALTERA,
            "FPGADSPControlALTERA" => Self::FPGADSPControlALTERA,
            "FPGADSPControlINTEL" => Self::FPGADSPControlALTERA,
            "MemoryAccessAliasingINTEL" => Self::MemoryAccessAliasingINTEL,
            "FPGAInvocationPipeliningAttributesALTERA" => {
                Self::FPGAInvocationPipeliningAttributesALTERA
            }
            "FPGAInvocationPipeliningAttributesINTEL" => {
                Self::FPGAInvocationPipeliningAttributesALTERA
            }
            "FPGABufferLocationALTERA" => Self::FPGABufferLocationALTERA,
            "FPGABufferLocationINTEL" => Self::FPGABufferLocationALTERA,
            "ArbitraryPrecisionFixedPointALTERA" => Self::ArbitraryPrecisionFixedPointALTERA,
            "ArbitraryPrecisionFixedPointINTEL" => Self::ArbitraryPrecisionFixedPointALTERA,
            "USMStorageClassesALTERA" => Self::USMStorageClassesALTERA,
            "USMStorageClassesINTEL" => Self::USMStorageClassesALTERA,
            "RuntimeAlignedAttributeALTERA" => Self::RuntimeAlignedAttributeALTERA,
            "RuntimeAlignedAttributeINTEL" => Self::RuntimeAlignedAttributeALTERA,
            "IOPipesALTERA" => Self::IOPipesALTERA,
            "IOPipesINTEL" => Self::IOPipesALTERA,
            "BlockingPipesALTERA" => Self::BlockingPipesALTERA,
            "BlockingPipesINTEL" => Self::BlockingPipesALTERA,
            "FPGARegALTERA" => Self::FPGARegALTERA,
            "FPGARegINTEL" => Self::FPGARegALTERA,
            "DotProductInputAll" => Self::DotProductInputAll,
            "DotProductInputAllKHR" => Self::DotProductInputAll,
            "DotProductInput4x8Bit" => Self::DotProductInput4x8Bit,
            "DotProductInput4x8BitKHR" => Self::DotProductInput4x8Bit,
            "DotProductInput4x8BitPacked" => Self::DotProductInput4x8BitPacked,
            "DotProductInput4x8BitPackedKHR" => Self::DotProductInput4x8BitPacked,
            "DotProduct" => Self::DotProduct,
            "DotProductKHR" => Self::DotProduct,
            "RayCullMaskKHR" => Self::RayCullMaskKHR,
            "CooperativeMatrixKHR" => Self::CooperativeMatrixKHR,
            "ReplicatedCompositesEXT" => Self::ReplicatedCompositesEXT,
            "BitInstructions" => Self::BitInstructions,
            "GroupNonUniformRotateKHR" => Self::GroupNonUniformRotateKHR,
            "FloatControls2" => Self::FloatControls2,
            "FMAKHR" => Self::FMAKHR,
            "AtomicFloat32AddEXT" => Self::AtomicFloat32AddEXT,
            "AtomicFloat64AddEXT" => Self::AtomicFloat64AddEXT,
            "LongCompositesINTEL" => Self::LongCompositesINTEL,
            "OptNoneEXT" => Self::OptNoneEXT,
            "OptNoneINTEL" => Self::OptNoneEXT,
            "AtomicFloat16AddEXT" => Self::AtomicFloat16AddEXT,
            "DebugInfoModuleINTEL" => Self::DebugInfoModuleINTEL,
            "BFloat16ConversionINTEL" => Self::BFloat16ConversionINTEL,
            "SplitBarrierINTEL" => Self::SplitBarrierINTEL,
            "ArithmeticFenceEXT" => Self::ArithmeticFenceEXT,
            "FPGAClusterAttributesV2ALTERA" => Self::FPGAClusterAttributesV2ALTERA,
            "FPGAClusterAttributesV2INTEL" => Self::FPGAClusterAttributesV2ALTERA,
            "FPGAKernelAttributesv2INTEL" => Self::FPGAKernelAttributesv2INTEL,
            "TaskSequenceALTERA" => Self::TaskSequenceALTERA,
            "TaskSequenceINTEL" => Self::TaskSequenceALTERA,
            "FPMaxErrorINTEL" => Self::FPMaxErrorINTEL,
            "FPGALatencyControlALTERA" => Self::FPGALatencyControlALTERA,
            "FPGALatencyControlINTEL" => Self::FPGALatencyControlALTERA,
            "FPGAArgumentInterfacesALTERA" => Self::FPGAArgumentInterfacesALTERA,
            "FPGAArgumentInterfacesINTEL" => Self::FPGAArgumentInterfacesALTERA,
            "GlobalVariableHostAccessINTEL" => Self::GlobalVariableHostAccessINTEL,
            "GlobalVariableFPGADecorationsALTERA" => Self::GlobalVariableFPGADecorationsALTERA,
            "GlobalVariableFPGADecorationsINTEL" => Self::GlobalVariableFPGADecorationsALTERA,
            "SubgroupBufferPrefetchINTEL" => Self::SubgroupBufferPrefetchINTEL,
            "Subgroup2DBlockIOINTEL" => Self::Subgroup2DBlockIOINTEL,
            "Subgroup2DBlockTransformINTEL" => Self::Subgroup2DBlockTransformINTEL,
            "Subgroup2DBlockTransposeINTEL" => Self::Subgroup2DBlockTransposeINTEL,
            "SubgroupMatrixMultiplyAccumulateINTEL" => Self::SubgroupMatrixMultiplyAccumulateINTEL,
            "TernaryBitwiseFunctionINTEL" => Self::TernaryBitwiseFunctionINTEL,
            "UntypedVariableLengthArrayINTEL" => Self::UntypedVariableLengthArrayINTEL,
            "SpecConditionalINTEL" => Self::SpecConditionalINTEL,
            "FunctionVariantsINTEL" => Self::FunctionVariantsINTEL,
            "GroupUniformArithmeticKHR" => Self::GroupUniformArithmeticKHR,
            "TensorFloat32RoundingINTEL" => Self::TensorFloat32RoundingINTEL,
            "MaskedGatherScatterINTEL" => Self::MaskedGatherScatterINTEL,
            "CacheControlsINTEL" => Self::CacheControlsINTEL,
            "RegisterLimitsINTEL" => Self::RegisterLimitsINTEL,
            "BindlessImagesINTEL" => Self::BindlessImagesINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [RayQueryIntersection](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_ray_query_intersection_a_ray_query_intersection)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum RayQueryIntersection {
    RayQueryCandidateIntersectionKHR = 0u32,
    RayQueryCommittedIntersectionKHR = 1u32,
}
impl RayQueryIntersection {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, RayQueryIntersection>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl RayQueryIntersection {}
impl core::str::FromStr for RayQueryIntersection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RayQueryCandidateIntersectionKHR" => Self::RayQueryCandidateIntersectionKHR,
            "RayQueryCommittedIntersectionKHR" => Self::RayQueryCommittedIntersectionKHR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [RayQueryCommittedIntersectionType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_ray_query_committed_intersection_type_a_ray_query_committed_intersection_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum RayQueryCommittedIntersectionType {
    RayQueryCommittedIntersectionNoneKHR = 0u32,
    RayQueryCommittedIntersectionTriangleKHR = 1u32,
    RayQueryCommittedIntersectionGeneratedKHR = 2u32,
}
impl RayQueryCommittedIntersectionType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe {
                core::mem::transmute::<u32, RayQueryCommittedIntersectionType>(n)
            },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl RayQueryCommittedIntersectionType {}
impl core::str::FromStr for RayQueryCommittedIntersectionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RayQueryCommittedIntersectionNoneKHR" => Self::RayQueryCommittedIntersectionNoneKHR,
            "RayQueryCommittedIntersectionTriangleKHR" => {
                Self::RayQueryCommittedIntersectionTriangleKHR
            }
            "RayQueryCommittedIntersectionGeneratedKHR" => {
                Self::RayQueryCommittedIntersectionGeneratedKHR
            }
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [RayQueryCandidateIntersectionType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_ray_query_candidate_intersection_type_a_ray_query_candidate_intersection_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum RayQueryCandidateIntersectionType {
    RayQueryCandidateIntersectionTriangleKHR = 0u32,
    RayQueryCandidateIntersectionAABBKHR = 1u32,
}
impl RayQueryCandidateIntersectionType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe {
                core::mem::transmute::<u32, RayQueryCandidateIntersectionType>(n)
            },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl RayQueryCandidateIntersectionType {}
impl core::str::FromStr for RayQueryCandidateIntersectionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RayQueryCandidateIntersectionTriangleKHR" => {
                Self::RayQueryCandidateIntersectionTriangleKHR
            }
            "RayQueryCandidateIntersectionAABBKHR" => Self::RayQueryCandidateIntersectionAABBKHR,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [PackedVectorFormat](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_packed_vector_format_a_packed_vector_format)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum PackedVectorFormat {
    PackedVectorFormat4x8Bit = 0u32,
}
impl PackedVectorFormat {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32 => unsafe { core::mem::transmute::<u32, PackedVectorFormat>(0u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl PackedVectorFormat {
    pub const PackedVectorFormat4x8BitKHR: Self = Self::PackedVectorFormat4x8Bit;
}
impl core::str::FromStr for PackedVectorFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "PackedVectorFormat4x8Bit" => Self::PackedVectorFormat4x8Bit,
            "PackedVectorFormat4x8BitKHR" => Self::PackedVectorFormat4x8Bit,
            _ => return Err(()),
        })
    }
}
bitflags! { # [doc = "SPIR-V operand kind: [CooperativeMatrixOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_cooperative_matrix_operands_a_cooperative_matrix_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct CooperativeMatrixOperands : u32 { const NONE_KHR = 0u32 ; const MATRIX_A_SIGNED_COMPONENTS_KHR = 1u32 ; const MATRIX_B_SIGNED_COMPONENTS_KHR = 2u32 ; const MATRIX_C_SIGNED_COMPONENTS_KHR = 4u32 ; const MATRIX_RESULT_SIGNED_COMPONENTS_KHR = 8u32 ; const SATURATING_ACCUMULATION_KHR = 16u32 ; } }
#[doc = "SPIR-V operand kind: [CooperativeMatrixLayout](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_cooperative_matrix_layout_a_cooperative_matrix_layout)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum CooperativeMatrixLayout {
    RowMajorKHR = 0u32,
    ColumnMajorKHR = 1u32,
    RowBlockedInterleavedARM = 4202u32,
    ColumnBlockedInterleavedARM = 4203u32,
}
impl CooperativeMatrixLayout {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, CooperativeMatrixLayout>(n) },
            4202u32..=4203u32 => unsafe { core::mem::transmute::<u32, CooperativeMatrixLayout>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl CooperativeMatrixLayout {}
impl core::str::FromStr for CooperativeMatrixLayout {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RowMajorKHR" => Self::RowMajorKHR,
            "ColumnMajorKHR" => Self::ColumnMajorKHR,
            "RowBlockedInterleavedARM" => Self::RowBlockedInterleavedARM,
            "ColumnBlockedInterleavedARM" => Self::ColumnBlockedInterleavedARM,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [CooperativeMatrixUse](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_cooperative_matrix_use_a_cooperative_matrix_use)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum CooperativeMatrixUse {
    MatrixAKHR = 0u32,
    MatrixBKHR = 1u32,
    MatrixAccumulatorKHR = 2u32,
}
impl CooperativeMatrixUse {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, CooperativeMatrixUse>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl CooperativeMatrixUse {}
impl core::str::FromStr for CooperativeMatrixUse {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "MatrixAKHR" => Self::MatrixAKHR,
            "MatrixBKHR" => Self::MatrixBKHR,
            "MatrixAccumulatorKHR" => Self::MatrixAccumulatorKHR,
            _ => return Err(()),
        })
    }
}
bitflags! { # [doc = "SPIR-V operand kind: [CooperativeMatrixReduce](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_cooperative_matrix_reduce_a_cooperative_matrix_reduce)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct CooperativeMatrixReduce : u32 { const ROW = 1u32 ; const COLUMN = 2u32 ; const _2X2 = 4u32 ; } }
#[doc = "SPIR-V operand kind: [TensorClampMode](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_tensor_clamp_mode_a_tensor_clamp_mode)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum TensorClampMode {
    Undefined = 0u32,
    Constant = 1u32,
    ClampToEdge = 2u32,
    Repeat = 3u32,
    RepeatMirrored = 4u32,
}
impl TensorClampMode {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=4u32 => unsafe { core::mem::transmute::<u32, TensorClampMode>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl TensorClampMode {}
impl core::str::FromStr for TensorClampMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Undefined" => Self::Undefined,
            "Constant" => Self::Constant,
            "ClampToEdge" => Self::ClampToEdge,
            "Repeat" => Self::Repeat,
            "RepeatMirrored" => Self::RepeatMirrored,
            _ => return Err(()),
        })
    }
}
bitflags! { # [doc = "SPIR-V operand kind: [TensorAddressingOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_tensor_addressing_operands_a_tensor_addressing_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct TensorAddressingOperands : u32 { const NONE = 0u32 ; const TENSOR_VIEW = 1u32 ; const DECODE_FUNC = 2u32 ; } }
#[doc = "SPIR-V operand kind: [InitializationModeQualifier](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_initialization_mode_qualifier_a_initialization_mode_qualifier)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum InitializationModeQualifier {
    InitOnDeviceReprogramALTERA = 0u32,
    InitOnDeviceResetALTERA = 1u32,
}
impl InitializationModeQualifier {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=1u32 => unsafe { core::mem::transmute::<u32, InitializationModeQualifier>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl InitializationModeQualifier {
    pub const InitOnDeviceReprogramINTEL: Self = Self::InitOnDeviceReprogramALTERA;
    pub const InitOnDeviceResetINTEL: Self = Self::InitOnDeviceResetALTERA;
}
impl core::str::FromStr for InitializationModeQualifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "InitOnDeviceReprogramALTERA" => Self::InitOnDeviceReprogramALTERA,
            "InitOnDeviceReprogramINTEL" => Self::InitOnDeviceReprogramALTERA,
            "InitOnDeviceResetALTERA" => Self::InitOnDeviceResetALTERA,
            "InitOnDeviceResetINTEL" => Self::InitOnDeviceResetALTERA,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [LoadCacheControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_load_cache_control_a_load_cache_control)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum LoadCacheControl {
    UncachedINTEL = 0u32,
    CachedINTEL = 1u32,
    StreamingINTEL = 2u32,
    InvalidateAfterReadINTEL = 3u32,
    ConstCachedINTEL = 4u32,
}
impl LoadCacheControl {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=4u32 => unsafe { core::mem::transmute::<u32, LoadCacheControl>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl LoadCacheControl {}
impl core::str::FromStr for LoadCacheControl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UncachedINTEL" => Self::UncachedINTEL,
            "CachedINTEL" => Self::CachedINTEL,
            "StreamingINTEL" => Self::StreamingINTEL,
            "InvalidateAfterReadINTEL" => Self::InvalidateAfterReadINTEL,
            "ConstCachedINTEL" => Self::ConstCachedINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [StoreCacheControl](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_store_cache_control_a_store_cache_control)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum StoreCacheControl {
    UncachedINTEL = 0u32,
    WriteThroughINTEL = 1u32,
    WriteBackINTEL = 2u32,
    StreamingINTEL = 3u32,
}
impl StoreCacheControl {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, StoreCacheControl>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl StoreCacheControl {}
impl core::str::FromStr for StoreCacheControl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UncachedINTEL" => Self::UncachedINTEL,
            "WriteThroughINTEL" => Self::WriteThroughINTEL,
            "WriteBackINTEL" => Self::WriteBackINTEL,
            "StreamingINTEL" => Self::StreamingINTEL,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [NamedMaximumNumberOfRegisters](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_named_maximum_number_of_registers_a_named_maximum_number_of_registers)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NamedMaximumNumberOfRegisters {
    AutoINTEL = 0u32,
}
impl NamedMaximumNumberOfRegisters {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32 => unsafe { core::mem::transmute::<u32, NamedMaximumNumberOfRegisters>(0u32) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl NamedMaximumNumberOfRegisters {}
impl core::str::FromStr for NamedMaximumNumberOfRegisters {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AutoINTEL" => Self::AutoINTEL,
            _ => return Err(()),
        })
    }
}
bitflags! { # [doc = "SPIR-V operand kind: [MatrixMultiplyAccumulateOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_matrix_multiply_accumulate_operands_a_matrix_multiply_accumulate_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct MatrixMultiplyAccumulateOperands : u32 { const NONE = 0u32 ; const MATRIX_A_SIGNED_COMPONENTS_INTEL = 1u32 ; const MATRIX_B_SIGNED_COMPONENTS_INTEL = 2u32 ; const MATRIX_CB_FLOAT16_INTEL = 4u32 ; const MATRIX_RESULT_B_FLOAT16_INTEL = 8u32 ; const MATRIX_A_PACKED_INT8_INTEL = 16u32 ; const MATRIX_B_PACKED_INT8_INTEL = 32u32 ; const MATRIX_A_PACKED_INT4_INTEL = 64u32 ; const MATRIX_B_PACKED_INT4_INTEL = 128u32 ; const MATRIX_ATF32INTEL = 256u32 ; const MATRIX_BTF32INTEL = 512u32 ; const MATRIX_A_PACKED_FLOAT16_INTEL = 1024u32 ; const MATRIX_B_PACKED_FLOAT16_INTEL = 2048u32 ; const MATRIX_A_PACKED_B_FLOAT16_INTEL = 4096u32 ; const MATRIX_B_PACKED_B_FLOAT16_INTEL = 8192u32 ; } }
#[doc = "SPIR-V operand kind: [FPEncoding](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_fp_encoding_a_fp_encoding)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FPEncoding {
    BFloat16KHR = 0u32,
    Float8E4M3EXT = 4214u32,
    Float8E5M2EXT = 4215u32,
}
impl FPEncoding {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32 => unsafe { core::mem::transmute::<u32, FPEncoding>(0u32) },
            4214u32..=4215u32 => unsafe { core::mem::transmute::<u32, FPEncoding>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl FPEncoding {}
impl core::str::FromStr for FPEncoding {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "BFloat16KHR" => Self::BFloat16KHR,
            "Float8E4M3EXT" => Self::Float8E4M3EXT,
            "Float8E5M2EXT" => Self::Float8E5M2EXT,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [CooperativeVectorMatrixLayout](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_cooperative_vector_matrix_layout_a_cooperative_vector_matrix_layout)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum CooperativeVectorMatrixLayout {
    RowMajorNV = 0u32,
    ColumnMajorNV = 1u32,
    InferencingOptimalNV = 2u32,
    TrainingOptimalNV = 3u32,
}
impl CooperativeVectorMatrixLayout {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=3u32 => unsafe { core::mem::transmute::<u32, CooperativeVectorMatrixLayout>(n) },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl CooperativeVectorMatrixLayout {}
impl core::str::FromStr for CooperativeVectorMatrixLayout {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "RowMajorNV" => Self::RowMajorNV,
            "ColumnMajorNV" => Self::ColumnMajorNV,
            "InferencingOptimalNV" => Self::InferencingOptimalNV,
            "TrainingOptimalNV" => Self::TrainingOptimalNV,
            _ => return Err(()),
        })
    }
}
#[doc = "SPIR-V operand kind: [ComponentType](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_component_type_a_component_type)"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ComponentType {
    Float16NV = 0u32,
    Float32NV = 1u32,
    Float64NV = 2u32,
    SignedInt8NV = 3u32,
    SignedInt16NV = 4u32,
    SignedInt32NV = 5u32,
    SignedInt64NV = 6u32,
    UnsignedInt8NV = 7u32,
    UnsignedInt16NV = 8u32,
    UnsignedInt32NV = 9u32,
    UnsignedInt64NV = 10u32,
    SignedInt8PackedNV = 1000491000u32,
    UnsignedInt8PackedNV = 1000491001u32,
    FloatE4M3NV = 1000491002u32,
    FloatE5M2NV = 1000491003u32,
}
impl ComponentType {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=10u32 => unsafe { core::mem::transmute::<u32, ComponentType>(n) },
            1000491000u32..=1000491003u32 => unsafe {
                core::mem::transmute::<u32, ComponentType>(n)
            },
            _ => return None,
        })
    }
}
#[allow(non_upper_case_globals)]
impl ComponentType {}
impl core::str::FromStr for ComponentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Float16NV" => Self::Float16NV,
            "Float32NV" => Self::Float32NV,
            "Float64NV" => Self::Float64NV,
            "SignedInt8NV" => Self::SignedInt8NV,
            "SignedInt16NV" => Self::SignedInt16NV,
            "SignedInt32NV" => Self::SignedInt32NV,
            "SignedInt64NV" => Self::SignedInt64NV,
            "UnsignedInt8NV" => Self::UnsignedInt8NV,
            "UnsignedInt16NV" => Self::UnsignedInt16NV,
            "UnsignedInt32NV" => Self::UnsignedInt32NV,
            "UnsignedInt64NV" => Self::UnsignedInt64NV,
            "SignedInt8PackedNV" => Self::SignedInt8PackedNV,
            "UnsignedInt8PackedNV" => Self::UnsignedInt8PackedNV,
            "FloatE4M3NV" => Self::FloatE4M3NV,
            "FloatE5M2NV" => Self::FloatE5M2NV,
            _ => return Err(()),
        })
    }
}
bitflags! { # [doc = "SPIR-V operand kind: [TensorOperands](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_tensor_operands_a_tensor_operands)"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serialize" , derive (serde :: Serialize))] # [cfg_attr (feature = "deserialize" , derive (serde :: Deserialize))] pub struct TensorOperands : u32 { const NONE_ARM = 0u32 ; const NONTEMPORAL_ARM = 1u32 ; const OUT_OF_BOUNDS_VALUE_ARM = 2u32 ; const MAKE_ELEMENT_AVAILABLE_ARM = 4u32 ; const MAKE_ELEMENT_VISIBLE_ARM = 8u32 ; const NON_PRIVATE_ELEMENT_ARM = 16u32 ; } }
#[doc = "SPIR-V [instructions](https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_instructions_a_instructions) opcodes"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Op {
    Nop = 0u32,
    Undef = 1u32,
    SourceContinued = 2u32,
    Source = 3u32,
    SourceExtension = 4u32,
    Name = 5u32,
    MemberName = 6u32,
    String = 7u32,
    Line = 8u32,
    Extension = 10u32,
    ExtInstImport = 11u32,
    ExtInst = 12u32,
    MemoryModel = 14u32,
    EntryPoint = 15u32,
    ExecutionMode = 16u32,
    Capability = 17u32,
    TypeVoid = 19u32,
    TypeBool = 20u32,
    TypeInt = 21u32,
    TypeFloat = 22u32,
    TypeVector = 23u32,
    TypeMatrix = 24u32,
    TypeImage = 25u32,
    TypeSampler = 26u32,
    TypeSampledImage = 27u32,
    TypeArray = 28u32,
    TypeRuntimeArray = 29u32,
    TypeStruct = 30u32,
    TypeOpaque = 31u32,
    TypePointer = 32u32,
    TypeFunction = 33u32,
    TypeEvent = 34u32,
    TypeDeviceEvent = 35u32,
    TypeReserveId = 36u32,
    TypeQueue = 37u32,
    TypePipe = 38u32,
    TypeForwardPointer = 39u32,
    ConstantTrue = 41u32,
    ConstantFalse = 42u32,
    Constant = 43u32,
    ConstantComposite = 44u32,
    ConstantSampler = 45u32,
    ConstantNull = 46u32,
    SpecConstantTrue = 48u32,
    SpecConstantFalse = 49u32,
    SpecConstant = 50u32,
    SpecConstantComposite = 51u32,
    SpecConstantOp = 52u32,
    Function = 54u32,
    FunctionParameter = 55u32,
    FunctionEnd = 56u32,
    FunctionCall = 57u32,
    Variable = 59u32,
    ImageTexelPointer = 60u32,
    Load = 61u32,
    Store = 62u32,
    CopyMemory = 63u32,
    CopyMemorySized = 64u32,
    AccessChain = 65u32,
    InBoundsAccessChain = 66u32,
    PtrAccessChain = 67u32,
    ArrayLength = 68u32,
    GenericPtrMemSemantics = 69u32,
    InBoundsPtrAccessChain = 70u32,
    Decorate = 71u32,
    MemberDecorate = 72u32,
    DecorationGroup = 73u32,
    GroupDecorate = 74u32,
    GroupMemberDecorate = 75u32,
    VectorExtractDynamic = 77u32,
    VectorInsertDynamic = 78u32,
    VectorShuffle = 79u32,
    CompositeConstruct = 80u32,
    CompositeExtract = 81u32,
    CompositeInsert = 82u32,
    CopyObject = 83u32,
    Transpose = 84u32,
    SampledImage = 86u32,
    ImageSampleImplicitLod = 87u32,
    ImageSampleExplicitLod = 88u32,
    ImageSampleDrefImplicitLod = 89u32,
    ImageSampleDrefExplicitLod = 90u32,
    ImageSampleProjImplicitLod = 91u32,
    ImageSampleProjExplicitLod = 92u32,
    ImageSampleProjDrefImplicitLod = 93u32,
    ImageSampleProjDrefExplicitLod = 94u32,
    ImageFetch = 95u32,
    ImageGather = 96u32,
    ImageDrefGather = 97u32,
    ImageRead = 98u32,
    ImageWrite = 99u32,
    Image = 100u32,
    ImageQueryFormat = 101u32,
    ImageQueryOrder = 102u32,
    ImageQuerySizeLod = 103u32,
    ImageQuerySize = 104u32,
    ImageQueryLod = 105u32,
    ImageQueryLevels = 106u32,
    ImageQuerySamples = 107u32,
    ConvertFToU = 109u32,
    ConvertFToS = 110u32,
    ConvertSToF = 111u32,
    ConvertUToF = 112u32,
    UConvert = 113u32,
    SConvert = 114u32,
    FConvert = 115u32,
    QuantizeToF16 = 116u32,
    ConvertPtrToU = 117u32,
    SatConvertSToU = 118u32,
    SatConvertUToS = 119u32,
    ConvertUToPtr = 120u32,
    PtrCastToGeneric = 121u32,
    GenericCastToPtr = 122u32,
    GenericCastToPtrExplicit = 123u32,
    Bitcast = 124u32,
    SNegate = 126u32,
    FNegate = 127u32,
    IAdd = 128u32,
    FAdd = 129u32,
    ISub = 130u32,
    FSub = 131u32,
    IMul = 132u32,
    FMul = 133u32,
    UDiv = 134u32,
    SDiv = 135u32,
    FDiv = 136u32,
    UMod = 137u32,
    SRem = 138u32,
    SMod = 139u32,
    FRem = 140u32,
    FMod = 141u32,
    VectorTimesScalar = 142u32,
    MatrixTimesScalar = 143u32,
    VectorTimesMatrix = 144u32,
    MatrixTimesVector = 145u32,
    MatrixTimesMatrix = 146u32,
    OuterProduct = 147u32,
    Dot = 148u32,
    IAddCarry = 149u32,
    ISubBorrow = 150u32,
    UMulExtended = 151u32,
    SMulExtended = 152u32,
    Any = 154u32,
    All = 155u32,
    IsNan = 156u32,
    IsInf = 157u32,
    IsFinite = 158u32,
    IsNormal = 159u32,
    SignBitSet = 160u32,
    LessOrGreater = 161u32,
    Ordered = 162u32,
    Unordered = 163u32,
    LogicalEqual = 164u32,
    LogicalNotEqual = 165u32,
    LogicalOr = 166u32,
    LogicalAnd = 167u32,
    LogicalNot = 168u32,
    Select = 169u32,
    IEqual = 170u32,
    INotEqual = 171u32,
    UGreaterThan = 172u32,
    SGreaterThan = 173u32,
    UGreaterThanEqual = 174u32,
    SGreaterThanEqual = 175u32,
    ULessThan = 176u32,
    SLessThan = 177u32,
    ULessThanEqual = 178u32,
    SLessThanEqual = 179u32,
    FOrdEqual = 180u32,
    FUnordEqual = 181u32,
    FOrdNotEqual = 182u32,
    FUnordNotEqual = 183u32,
    FOrdLessThan = 184u32,
    FUnordLessThan = 185u32,
    FOrdGreaterThan = 186u32,
    FUnordGreaterThan = 187u32,
    FOrdLessThanEqual = 188u32,
    FUnordLessThanEqual = 189u32,
    FOrdGreaterThanEqual = 190u32,
    FUnordGreaterThanEqual = 191u32,
    ShiftRightLogical = 194u32,
    ShiftRightArithmetic = 195u32,
    ShiftLeftLogical = 196u32,
    BitwiseOr = 197u32,
    BitwiseXor = 198u32,
    BitwiseAnd = 199u32,
    Not = 200u32,
    BitFieldInsert = 201u32,
    BitFieldSExtract = 202u32,
    BitFieldUExtract = 203u32,
    BitReverse = 204u32,
    BitCount = 205u32,
    DPdx = 207u32,
    DPdy = 208u32,
    Fwidth = 209u32,
    DPdxFine = 210u32,
    DPdyFine = 211u32,
    FwidthFine = 212u32,
    DPdxCoarse = 213u32,
    DPdyCoarse = 214u32,
    FwidthCoarse = 215u32,
    EmitVertex = 218u32,
    EndPrimitive = 219u32,
    EmitStreamVertex = 220u32,
    EndStreamPrimitive = 221u32,
    ControlBarrier = 224u32,
    MemoryBarrier = 225u32,
    AtomicLoad = 227u32,
    AtomicStore = 228u32,
    AtomicExchange = 229u32,
    AtomicCompareExchange = 230u32,
    AtomicCompareExchangeWeak = 231u32,
    AtomicIIncrement = 232u32,
    AtomicIDecrement = 233u32,
    AtomicIAdd = 234u32,
    AtomicISub = 235u32,
    AtomicSMin = 236u32,
    AtomicUMin = 237u32,
    AtomicSMax = 238u32,
    AtomicUMax = 239u32,
    AtomicAnd = 240u32,
    AtomicOr = 241u32,
    AtomicXor = 242u32,
    Phi = 245u32,
    LoopMerge = 246u32,
    SelectionMerge = 247u32,
    Label = 248u32,
    Branch = 249u32,
    BranchConditional = 250u32,
    Switch = 251u32,
    Kill = 252u32,
    Return = 253u32,
    ReturnValue = 254u32,
    Unreachable = 255u32,
    LifetimeStart = 256u32,
    LifetimeStop = 257u32,
    GroupAsyncCopy = 259u32,
    GroupWaitEvents = 260u32,
    GroupAll = 261u32,
    GroupAny = 262u32,
    GroupBroadcast = 263u32,
    GroupIAdd = 264u32,
    GroupFAdd = 265u32,
    GroupFMin = 266u32,
    GroupUMin = 267u32,
    GroupSMin = 268u32,
    GroupFMax = 269u32,
    GroupUMax = 270u32,
    GroupSMax = 271u32,
    ReadPipe = 274u32,
    WritePipe = 275u32,
    ReservedReadPipe = 276u32,
    ReservedWritePipe = 277u32,
    ReserveReadPipePackets = 278u32,
    ReserveWritePipePackets = 279u32,
    CommitReadPipe = 280u32,
    CommitWritePipe = 281u32,
    IsValidReserveId = 282u32,
    GetNumPipePackets = 283u32,
    GetMaxPipePackets = 284u32,
    GroupReserveReadPipePackets = 285u32,
    GroupReserveWritePipePackets = 286u32,
    GroupCommitReadPipe = 287u32,
    GroupCommitWritePipe = 288u32,
    EnqueueMarker = 291u32,
    EnqueueKernel = 292u32,
    GetKernelNDrangeSubGroupCount = 293u32,
    GetKernelNDrangeMaxSubGroupSize = 294u32,
    GetKernelWorkGroupSize = 295u32,
    GetKernelPreferredWorkGroupSizeMultiple = 296u32,
    RetainEvent = 297u32,
    ReleaseEvent = 298u32,
    CreateUserEvent = 299u32,
    IsValidEvent = 300u32,
    SetUserEventStatus = 301u32,
    CaptureEventProfilingInfo = 302u32,
    GetDefaultQueue = 303u32,
    BuildNDRange = 304u32,
    ImageSparseSampleImplicitLod = 305u32,
    ImageSparseSampleExplicitLod = 306u32,
    ImageSparseSampleDrefImplicitLod = 307u32,
    ImageSparseSampleDrefExplicitLod = 308u32,
    ImageSparseSampleProjImplicitLod = 309u32,
    ImageSparseSampleProjExplicitLod = 310u32,
    ImageSparseSampleProjDrefImplicitLod = 311u32,
    ImageSparseSampleProjDrefExplicitLod = 312u32,
    ImageSparseFetch = 313u32,
    ImageSparseGather = 314u32,
    ImageSparseDrefGather = 315u32,
    ImageSparseTexelsResident = 316u32,
    NoLine = 317u32,
    AtomicFlagTestAndSet = 318u32,
    AtomicFlagClear = 319u32,
    ImageSparseRead = 320u32,
    SizeOf = 321u32,
    TypePipeStorage = 322u32,
    ConstantPipeStorage = 323u32,
    CreatePipeFromPipeStorage = 324u32,
    GetKernelLocalSizeForSubgroupCount = 325u32,
    GetKernelMaxNumSubgroups = 326u32,
    TypeNamedBarrier = 327u32,
    NamedBarrierInitialize = 328u32,
    MemoryNamedBarrier = 329u32,
    ModuleProcessed = 330u32,
    ExecutionModeId = 331u32,
    DecorateId = 332u32,
    GroupNonUniformElect = 333u32,
    GroupNonUniformAll = 334u32,
    GroupNonUniformAny = 335u32,
    GroupNonUniformAllEqual = 336u32,
    GroupNonUniformBroadcast = 337u32,
    GroupNonUniformBroadcastFirst = 338u32,
    GroupNonUniformBallot = 339u32,
    GroupNonUniformInverseBallot = 340u32,
    GroupNonUniformBallotBitExtract = 341u32,
    GroupNonUniformBallotBitCount = 342u32,
    GroupNonUniformBallotFindLSB = 343u32,
    GroupNonUniformBallotFindMSB = 344u32,
    GroupNonUniformShuffle = 345u32,
    GroupNonUniformShuffleXor = 346u32,
    GroupNonUniformShuffleUp = 347u32,
    GroupNonUniformShuffleDown = 348u32,
    GroupNonUniformIAdd = 349u32,
    GroupNonUniformFAdd = 350u32,
    GroupNonUniformIMul = 351u32,
    GroupNonUniformFMul = 352u32,
    GroupNonUniformSMin = 353u32,
    GroupNonUniformUMin = 354u32,
    GroupNonUniformFMin = 355u32,
    GroupNonUniformSMax = 356u32,
    GroupNonUniformUMax = 357u32,
    GroupNonUniformFMax = 358u32,
    GroupNonUniformBitwiseAnd = 359u32,
    GroupNonUniformBitwiseOr = 360u32,
    GroupNonUniformBitwiseXor = 361u32,
    GroupNonUniformLogicalAnd = 362u32,
    GroupNonUniformLogicalOr = 363u32,
    GroupNonUniformLogicalXor = 364u32,
    GroupNonUniformQuadBroadcast = 365u32,
    GroupNonUniformQuadSwap = 366u32,
    CopyLogical = 400u32,
    PtrEqual = 401u32,
    PtrNotEqual = 402u32,
    PtrDiff = 403u32,
    ColorAttachmentReadEXT = 4160u32,
    DepthAttachmentReadEXT = 4161u32,
    StencilAttachmentReadEXT = 4162u32,
    TypeTensorARM = 4163u32,
    TensorReadARM = 4164u32,
    TensorWriteARM = 4165u32,
    TensorQuerySizeARM = 4166u32,
    GraphConstantARM = 4181u32,
    GraphEntryPointARM = 4182u32,
    GraphARM = 4183u32,
    GraphInputARM = 4184u32,
    GraphSetOutputARM = 4185u32,
    GraphEndARM = 4186u32,
    TypeGraphARM = 4190u32,
    TerminateInvocation = 4416u32,
    TypeUntypedPointerKHR = 4417u32,
    UntypedVariableKHR = 4418u32,
    UntypedAccessChainKHR = 4419u32,
    UntypedInBoundsAccessChainKHR = 4420u32,
    SubgroupBallotKHR = 4421u32,
    SubgroupFirstInvocationKHR = 4422u32,
    UntypedPtrAccessChainKHR = 4423u32,
    UntypedInBoundsPtrAccessChainKHR = 4424u32,
    UntypedArrayLengthKHR = 4425u32,
    UntypedPrefetchKHR = 4426u32,
    FmaKHR = 4427u32,
    SubgroupAllKHR = 4428u32,
    SubgroupAnyKHR = 4429u32,
    SubgroupAllEqualKHR = 4430u32,
    GroupNonUniformRotateKHR = 4431u32,
    SubgroupReadInvocationKHR = 4432u32,
    ExtInstWithForwardRefsKHR = 4433u32,
    UntypedGroupAsyncCopyKHR = 4434u32,
    TraceRayKHR = 4445u32,
    ExecuteCallableKHR = 4446u32,
    ConvertUToAccelerationStructureKHR = 4447u32,
    IgnoreIntersectionKHR = 4448u32,
    TerminateRayKHR = 4449u32,
    SDot = 4450u32,
    UDot = 4451u32,
    SUDot = 4452u32,
    SDotAccSat = 4453u32,
    UDotAccSat = 4454u32,
    SUDotAccSat = 4455u32,
    TypeCooperativeMatrixKHR = 4456u32,
    CooperativeMatrixLoadKHR = 4457u32,
    CooperativeMatrixStoreKHR = 4458u32,
    CooperativeMatrixMulAddKHR = 4459u32,
    CooperativeMatrixLengthKHR = 4460u32,
    ConstantCompositeReplicateEXT = 4461u32,
    SpecConstantCompositeReplicateEXT = 4462u32,
    CompositeConstructReplicateEXT = 4463u32,
    TypeRayQueryKHR = 4472u32,
    RayQueryInitializeKHR = 4473u32,
    RayQueryTerminateKHR = 4474u32,
    RayQueryGenerateIntersectionKHR = 4475u32,
    RayQueryConfirmIntersectionKHR = 4476u32,
    RayQueryProceedKHR = 4477u32,
    RayQueryGetIntersectionTypeKHR = 4479u32,
    ImageSampleWeightedQCOM = 4480u32,
    ImageBoxFilterQCOM = 4481u32,
    ImageBlockMatchSSDQCOM = 4482u32,
    ImageBlockMatchSADQCOM = 4483u32,
    BitCastArrayQCOM = 4497u32,
    ImageBlockMatchWindowSSDQCOM = 4500u32,
    ImageBlockMatchWindowSADQCOM = 4501u32,
    ImageBlockMatchGatherSSDQCOM = 4502u32,
    ImageBlockMatchGatherSADQCOM = 4503u32,
    CompositeConstructCoopMatQCOM = 4540u32,
    CompositeExtractCoopMatQCOM = 4541u32,
    ExtractSubArrayQCOM = 4542u32,
    GroupIAddNonUniformAMD = 5000u32,
    GroupFAddNonUniformAMD = 5001u32,
    GroupFMinNonUniformAMD = 5002u32,
    GroupUMinNonUniformAMD = 5003u32,
    GroupSMinNonUniformAMD = 5004u32,
    GroupFMaxNonUniformAMD = 5005u32,
    GroupUMaxNonUniformAMD = 5006u32,
    GroupSMaxNonUniformAMD = 5007u32,
    FragmentMaskFetchAMD = 5011u32,
    FragmentFetchAMD = 5012u32,
    ReadClockKHR = 5056u32,
    AllocateNodePayloadsAMDX = 5074u32,
    EnqueueNodePayloadsAMDX = 5075u32,
    TypeNodePayloadArrayAMDX = 5076u32,
    FinishWritingNodePayloadAMDX = 5078u32,
    NodePayloadArrayLengthAMDX = 5090u32,
    IsNodePayloadValidAMDX = 5101u32,
    ConstantStringAMDX = 5103u32,
    SpecConstantStringAMDX = 5104u32,
    GroupNonUniformQuadAllKHR = 5110u32,
    GroupNonUniformQuadAnyKHR = 5111u32,
    TypeBufferEXT = 5115u32,
    BufferPointerEXT = 5119u32,
    UntypedImageTexelPointerEXT = 5126u32,
    MemberDecorateIdEXT = 5127u32,
    ConstantSizeOfEXT = 5129u32,
    HitObjectRecordHitMotionNV = 5249u32,
    HitObjectRecordHitWithIndexMotionNV = 5250u32,
    HitObjectRecordMissMotionNV = 5251u32,
    HitObjectGetWorldToObjectNV = 5252u32,
    HitObjectGetObjectToWorldNV = 5253u32,
    HitObjectGetObjectRayDirectionNV = 5254u32,
    HitObjectGetObjectRayOriginNV = 5255u32,
    HitObjectTraceRayMotionNV = 5256u32,
    HitObjectGetShaderRecordBufferHandleNV = 5257u32,
    HitObjectGetShaderBindingTableRecordIndexNV = 5258u32,
    HitObjectRecordEmptyNV = 5259u32,
    HitObjectTraceRayNV = 5260u32,
    HitObjectRecordHitNV = 5261u32,
    HitObjectRecordHitWithIndexNV = 5262u32,
    HitObjectRecordMissNV = 5263u32,
    HitObjectExecuteShaderNV = 5264u32,
    HitObjectGetCurrentTimeNV = 5265u32,
    HitObjectGetAttributesNV = 5266u32,
    HitObjectGetHitKindNV = 5267u32,
    HitObjectGetPrimitiveIndexNV = 5268u32,
    HitObjectGetGeometryIndexNV = 5269u32,
    HitObjectGetInstanceIdNV = 5270u32,
    HitObjectGetInstanceCustomIndexNV = 5271u32,
    HitObjectGetWorldRayDirectionNV = 5272u32,
    HitObjectGetWorldRayOriginNV = 5273u32,
    HitObjectGetRayTMaxNV = 5274u32,
    HitObjectGetRayTMinNV = 5275u32,
    HitObjectIsEmptyNV = 5276u32,
    HitObjectIsHitNV = 5277u32,
    HitObjectIsMissNV = 5278u32,
    ReorderThreadWithHitObjectNV = 5279u32,
    ReorderThreadWithHintNV = 5280u32,
    TypeHitObjectNV = 5281u32,
    ImageSampleFootprintNV = 5283u32,
    TypeVectorIdEXT = 5288u32,
    CooperativeVectorMatrixMulNV = 5289u32,
    CooperativeVectorOuterProductAccumulateNV = 5290u32,
    CooperativeVectorReduceSumAccumulateNV = 5291u32,
    CooperativeVectorMatrixMulAddNV = 5292u32,
    CooperativeMatrixConvertNV = 5293u32,
    EmitMeshTasksEXT = 5294u32,
    SetMeshOutputsEXT = 5295u32,
    GroupNonUniformPartitionEXT = 5296u32,
    WritePackedPrimitiveIndices4x8NV = 5299u32,
    FetchMicroTriangleVertexPositionNV = 5300u32,
    FetchMicroTriangleVertexBarycentricNV = 5301u32,
    CooperativeVectorLoadNV = 5302u32,
    CooperativeVectorStoreNV = 5303u32,
    HitObjectRecordFromQueryEXT = 5304u32,
    HitObjectRecordMissEXT = 5305u32,
    HitObjectRecordMissMotionEXT = 5306u32,
    HitObjectGetIntersectionTriangleVertexPositionsEXT = 5307u32,
    HitObjectGetRayFlagsEXT = 5308u32,
    HitObjectSetShaderBindingTableRecordIndexEXT = 5309u32,
    HitObjectReorderExecuteShaderEXT = 5310u32,
    HitObjectTraceReorderExecuteEXT = 5311u32,
    HitObjectTraceMotionReorderExecuteEXT = 5312u32,
    TypeHitObjectEXT = 5313u32,
    ReorderThreadWithHintEXT = 5314u32,
    ReorderThreadWithHitObjectEXT = 5315u32,
    HitObjectTraceRayEXT = 5316u32,
    HitObjectTraceRayMotionEXT = 5317u32,
    HitObjectRecordEmptyEXT = 5318u32,
    HitObjectExecuteShaderEXT = 5319u32,
    HitObjectGetCurrentTimeEXT = 5320u32,
    HitObjectGetAttributesEXT = 5321u32,
    HitObjectGetHitKindEXT = 5322u32,
    HitObjectGetPrimitiveIndexEXT = 5323u32,
    HitObjectGetGeometryIndexEXT = 5324u32,
    HitObjectGetInstanceIdEXT = 5325u32,
    HitObjectGetInstanceCustomIndexEXT = 5326u32,
    HitObjectGetObjectRayOriginEXT = 5327u32,
    HitObjectGetObjectRayDirectionEXT = 5328u32,
    HitObjectGetWorldRayDirectionEXT = 5329u32,
    HitObjectGetWorldRayOriginEXT = 5330u32,
    HitObjectGetObjectToWorldEXT = 5331u32,
    HitObjectGetWorldToObjectEXT = 5332u32,
    HitObjectGetRayTMaxEXT = 5333u32,
    ReportIntersectionKHR = 5334u32,
    IgnoreIntersectionNV = 5335u32,
    TerminateRayNV = 5336u32,
    TraceNV = 5337u32,
    TraceMotionNV = 5338u32,
    TraceRayMotionNV = 5339u32,
    RayQueryGetIntersectionTriangleVertexPositionsKHR = 5340u32,
    TypeAccelerationStructureKHR = 5341u32,
    ExecuteCallableNV = 5344u32,
    RayQueryGetIntersectionClusterIdNV = 5345u32,
    HitObjectGetClusterIdNV = 5346u32,
    HitObjectGetRayTMinEXT = 5347u32,
    HitObjectGetShaderBindingTableRecordIndexEXT = 5348u32,
    HitObjectGetShaderRecordBufferHandleEXT = 5349u32,
    HitObjectIsEmptyEXT = 5350u32,
    HitObjectIsHitEXT = 5351u32,
    HitObjectIsMissEXT = 5352u32,
    TypeCooperativeMatrixNV = 5358u32,
    CooperativeMatrixLoadNV = 5359u32,
    CooperativeMatrixStoreNV = 5360u32,
    CooperativeMatrixMulAddNV = 5361u32,
    CooperativeMatrixLengthNV = 5362u32,
    BeginInvocationInterlockEXT = 5364u32,
    EndInvocationInterlockEXT = 5365u32,
    CooperativeMatrixReduceNV = 5366u32,
    CooperativeMatrixLoadTensorNV = 5367u32,
    CooperativeMatrixStoreTensorNV = 5368u32,
    CooperativeMatrixPerElementOpNV = 5369u32,
    TypeTensorLayoutNV = 5370u32,
    TypeTensorViewNV = 5371u32,
    CreateTensorLayoutNV = 5372u32,
    TensorLayoutSetDimensionNV = 5373u32,
    TensorLayoutSetStrideNV = 5374u32,
    TensorLayoutSliceNV = 5375u32,
    TensorLayoutSetClampValueNV = 5376u32,
    CreateTensorViewNV = 5377u32,
    TensorViewSetDimensionNV = 5378u32,
    TensorViewSetStrideNV = 5379u32,
    DemoteToHelperInvocation = 5380u32,
    IsHelperInvocationEXT = 5381u32,
    TensorViewSetClipNV = 5382u32,
    TensorLayoutSetBlockSizeNV = 5384u32,
    CooperativeMatrixTransposeNV = 5390u32,
    ConvertUToImageNV = 5391u32,
    ConvertUToSamplerNV = 5392u32,
    ConvertImageToUNV = 5393u32,
    ConvertSamplerToUNV = 5394u32,
    ConvertUToSampledImageNV = 5395u32,
    ConvertSampledImageToUNV = 5396u32,
    SamplerImageAddressingModeNV = 5397u32,
    RawAccessChainNV = 5398u32,
    RayQueryGetIntersectionSpherePositionNV = 5427u32,
    RayQueryGetIntersectionSphereRadiusNV = 5428u32,
    RayQueryGetIntersectionLSSPositionsNV = 5429u32,
    RayQueryGetIntersectionLSSRadiiNV = 5430u32,
    RayQueryGetIntersectionLSSHitValueNV = 5431u32,
    HitObjectGetSpherePositionNV = 5432u32,
    HitObjectGetSphereRadiusNV = 5433u32,
    HitObjectGetLSSPositionsNV = 5434u32,
    HitObjectGetLSSRadiiNV = 5435u32,
    HitObjectIsSphereHitNV = 5436u32,
    HitObjectIsLSSHitNV = 5437u32,
    RayQueryIsSphereHitNV = 5438u32,
    RayQueryIsLSSHitNV = 5439u32,
    SubgroupShuffleINTEL = 5571u32,
    SubgroupShuffleDownINTEL = 5572u32,
    SubgroupShuffleUpINTEL = 5573u32,
    SubgroupShuffleXorINTEL = 5574u32,
    SubgroupBlockReadINTEL = 5575u32,
    SubgroupBlockWriteINTEL = 5576u32,
    SubgroupImageBlockReadINTEL = 5577u32,
    SubgroupImageBlockWriteINTEL = 5578u32,
    SubgroupImageMediaBlockReadINTEL = 5580u32,
    SubgroupImageMediaBlockWriteINTEL = 5581u32,
    UCountLeadingZerosINTEL = 5585u32,
    UCountTrailingZerosINTEL = 5586u32,
    AbsISubINTEL = 5587u32,
    AbsUSubINTEL = 5588u32,
    IAddSatINTEL = 5589u32,
    UAddSatINTEL = 5590u32,
    IAverageINTEL = 5591u32,
    UAverageINTEL = 5592u32,
    IAverageRoundedINTEL = 5593u32,
    UAverageRoundedINTEL = 5594u32,
    ISubSatINTEL = 5595u32,
    USubSatINTEL = 5596u32,
    IMul32x16INTEL = 5597u32,
    UMul32x16INTEL = 5598u32,
    ConstantFunctionPointerINTEL = 5600u32,
    FunctionPointerCallINTEL = 5601u32,
    AsmTargetINTEL = 5609u32,
    AsmINTEL = 5610u32,
    AsmCallINTEL = 5611u32,
    AtomicFMinEXT = 5614u32,
    AtomicFMaxEXT = 5615u32,
    AssumeTrueKHR = 5630u32,
    ExpectKHR = 5631u32,
    DecorateString = 5632u32,
    MemberDecorateString = 5633u32,
    VmeImageINTEL = 5699u32,
    TypeVmeImageINTEL = 5700u32,
    TypeAvcImePayloadINTEL = 5701u32,
    TypeAvcRefPayloadINTEL = 5702u32,
    TypeAvcSicPayloadINTEL = 5703u32,
    TypeAvcMcePayloadINTEL = 5704u32,
    TypeAvcMceResultINTEL = 5705u32,
    TypeAvcImeResultINTEL = 5706u32,
    TypeAvcImeResultSingleReferenceStreamoutINTEL = 5707u32,
    TypeAvcImeResultDualReferenceStreamoutINTEL = 5708u32,
    TypeAvcImeSingleReferenceStreaminINTEL = 5709u32,
    TypeAvcImeDualReferenceStreaminINTEL = 5710u32,
    TypeAvcRefResultINTEL = 5711u32,
    TypeAvcSicResultINTEL = 5712u32,
    SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL = 5713u32,
    SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL = 5714u32,
    SubgroupAvcMceGetDefaultInterShapePenaltyINTEL = 5715u32,
    SubgroupAvcMceSetInterShapePenaltyINTEL = 5716u32,
    SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL = 5717u32,
    SubgroupAvcMceSetInterDirectionPenaltyINTEL = 5718u32,
    SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL = 5719u32,
    SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL = 5720u32,
    SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL = 5721u32,
    SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL = 5722u32,
    SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL = 5723u32,
    SubgroupAvcMceSetMotionVectorCostFunctionINTEL = 5724u32,
    SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL = 5725u32,
    SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL = 5726u32,
    SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL = 5727u32,
    SubgroupAvcMceSetAcOnlyHaarINTEL = 5728u32,
    SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL = 5729u32,
    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL = 5730u32,
    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL = 5731u32,
    SubgroupAvcMceConvertToImePayloadINTEL = 5732u32,
    SubgroupAvcMceConvertToImeResultINTEL = 5733u32,
    SubgroupAvcMceConvertToRefPayloadINTEL = 5734u32,
    SubgroupAvcMceConvertToRefResultINTEL = 5735u32,
    SubgroupAvcMceConvertToSicPayloadINTEL = 5736u32,
    SubgroupAvcMceConvertToSicResultINTEL = 5737u32,
    SubgroupAvcMceGetMotionVectorsINTEL = 5738u32,
    SubgroupAvcMceGetInterDistortionsINTEL = 5739u32,
    SubgroupAvcMceGetBestInterDistortionsINTEL = 5740u32,
    SubgroupAvcMceGetInterMajorShapeINTEL = 5741u32,
    SubgroupAvcMceGetInterMinorShapeINTEL = 5742u32,
    SubgroupAvcMceGetInterDirectionsINTEL = 5743u32,
    SubgroupAvcMceGetInterMotionVectorCountINTEL = 5744u32,
    SubgroupAvcMceGetInterReferenceIdsINTEL = 5745u32,
    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL = 5746u32,
    SubgroupAvcImeInitializeINTEL = 5747u32,
    SubgroupAvcImeSetSingleReferenceINTEL = 5748u32,
    SubgroupAvcImeSetDualReferenceINTEL = 5749u32,
    SubgroupAvcImeRefWindowSizeINTEL = 5750u32,
    SubgroupAvcImeAdjustRefOffsetINTEL = 5751u32,
    SubgroupAvcImeConvertToMcePayloadINTEL = 5752u32,
    SubgroupAvcImeSetMaxMotionVectorCountINTEL = 5753u32,
    SubgroupAvcImeSetUnidirectionalMixDisableINTEL = 5754u32,
    SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL = 5755u32,
    SubgroupAvcImeSetWeightedSadINTEL = 5756u32,
    SubgroupAvcImeEvaluateWithSingleReferenceINTEL = 5757u32,
    SubgroupAvcImeEvaluateWithDualReferenceINTEL = 5758u32,
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL = 5759u32,
    SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL = 5760u32,
    SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL = 5761u32,
    SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL = 5762u32,
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL = 5763u32,
    SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL = 5764u32,
    SubgroupAvcImeConvertToMceResultINTEL = 5765u32,
    SubgroupAvcImeGetSingleReferenceStreaminINTEL = 5766u32,
    SubgroupAvcImeGetDualReferenceStreaminINTEL = 5767u32,
    SubgroupAvcImeStripSingleReferenceStreamoutINTEL = 5768u32,
    SubgroupAvcImeStripDualReferenceStreamoutINTEL = 5769u32,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL = 5770u32,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL = 5771u32,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL = 5772u32,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL = 5773u32,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL = 5774u32,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL = 5775u32,
    SubgroupAvcImeGetBorderReachedINTEL = 5776u32,
    SubgroupAvcImeGetTruncatedSearchIndicationINTEL = 5777u32,
    SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL = 5778u32,
    SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL = 5779u32,
    SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL = 5780u32,
    SubgroupAvcFmeInitializeINTEL = 5781u32,
    SubgroupAvcBmeInitializeINTEL = 5782u32,
    SubgroupAvcRefConvertToMcePayloadINTEL = 5783u32,
    SubgroupAvcRefSetBidirectionalMixDisableINTEL = 5784u32,
    SubgroupAvcRefSetBilinearFilterEnableINTEL = 5785u32,
    SubgroupAvcRefEvaluateWithSingleReferenceINTEL = 5786u32,
    SubgroupAvcRefEvaluateWithDualReferenceINTEL = 5787u32,
    SubgroupAvcRefEvaluateWithMultiReferenceINTEL = 5788u32,
    SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL = 5789u32,
    SubgroupAvcRefConvertToMceResultINTEL = 5790u32,
    SubgroupAvcSicInitializeINTEL = 5791u32,
    SubgroupAvcSicConfigureSkcINTEL = 5792u32,
    SubgroupAvcSicConfigureIpeLumaINTEL = 5793u32,
    SubgroupAvcSicConfigureIpeLumaChromaINTEL = 5794u32,
    SubgroupAvcSicGetMotionVectorMaskINTEL = 5795u32,
    SubgroupAvcSicConvertToMcePayloadINTEL = 5796u32,
    SubgroupAvcSicSetIntraLumaShapePenaltyINTEL = 5797u32,
    SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL = 5798u32,
    SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL = 5799u32,
    SubgroupAvcSicSetBilinearFilterEnableINTEL = 5800u32,
    SubgroupAvcSicSetSkcForwardTransformEnableINTEL = 5801u32,
    SubgroupAvcSicSetBlockBasedRawSkipSadINTEL = 5802u32,
    SubgroupAvcSicEvaluateIpeINTEL = 5803u32,
    SubgroupAvcSicEvaluateWithSingleReferenceINTEL = 5804u32,
    SubgroupAvcSicEvaluateWithDualReferenceINTEL = 5805u32,
    SubgroupAvcSicEvaluateWithMultiReferenceINTEL = 5806u32,
    SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL = 5807u32,
    SubgroupAvcSicConvertToMceResultINTEL = 5808u32,
    SubgroupAvcSicGetIpeLumaShapeINTEL = 5809u32,
    SubgroupAvcSicGetBestIpeLumaDistortionINTEL = 5810u32,
    SubgroupAvcSicGetBestIpeChromaDistortionINTEL = 5811u32,
    SubgroupAvcSicGetPackedIpeLumaModesINTEL = 5812u32,
    SubgroupAvcSicGetIpeChromaModeINTEL = 5813u32,
    SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL = 5814u32,
    SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL = 5815u32,
    SubgroupAvcSicGetInterRawSadsINTEL = 5816u32,
    VariableLengthArrayINTEL = 5818u32,
    SaveMemoryINTEL = 5819u32,
    RestoreMemoryINTEL = 5820u32,
    ArbitraryFloatSinCosPiALTERA = 5840u32,
    ArbitraryFloatCastALTERA = 5841u32,
    ArbitraryFloatCastFromIntALTERA = 5842u32,
    ArbitraryFloatCastToIntALTERA = 5843u32,
    ArbitraryFloatAddALTERA = 5846u32,
    ArbitraryFloatSubALTERA = 5847u32,
    ArbitraryFloatMulALTERA = 5848u32,
    ArbitraryFloatDivALTERA = 5849u32,
    ArbitraryFloatGTALTERA = 5850u32,
    ArbitraryFloatGEALTERA = 5851u32,
    ArbitraryFloatLTALTERA = 5852u32,
    ArbitraryFloatLEALTERA = 5853u32,
    ArbitraryFloatEQALTERA = 5854u32,
    ArbitraryFloatRecipALTERA = 5855u32,
    ArbitraryFloatRSqrtALTERA = 5856u32,
    ArbitraryFloatCbrtALTERA = 5857u32,
    ArbitraryFloatHypotALTERA = 5858u32,
    ArbitraryFloatSqrtALTERA = 5859u32,
    ArbitraryFloatLogINTEL = 5860u32,
    ArbitraryFloatLog2INTEL = 5861u32,
    ArbitraryFloatLog10INTEL = 5862u32,
    ArbitraryFloatLog1pINTEL = 5863u32,
    ArbitraryFloatExpINTEL = 5864u32,
    ArbitraryFloatExp2INTEL = 5865u32,
    ArbitraryFloatExp10INTEL = 5866u32,
    ArbitraryFloatExpm1INTEL = 5867u32,
    ArbitraryFloatSinINTEL = 5868u32,
    ArbitraryFloatCosINTEL = 5869u32,
    ArbitraryFloatSinCosINTEL = 5870u32,
    ArbitraryFloatSinPiINTEL = 5871u32,
    ArbitraryFloatCosPiINTEL = 5872u32,
    ArbitraryFloatASinINTEL = 5873u32,
    ArbitraryFloatASinPiINTEL = 5874u32,
    ArbitraryFloatACosINTEL = 5875u32,
    ArbitraryFloatACosPiINTEL = 5876u32,
    ArbitraryFloatATanINTEL = 5877u32,
    ArbitraryFloatATanPiINTEL = 5878u32,
    ArbitraryFloatATan2INTEL = 5879u32,
    ArbitraryFloatPowINTEL = 5880u32,
    ArbitraryFloatPowRINTEL = 5881u32,
    ArbitraryFloatPowNINTEL = 5882u32,
    LoopControlINTEL = 5887u32,
    AliasDomainDeclINTEL = 5911u32,
    AliasScopeDeclINTEL = 5912u32,
    AliasScopeListDeclINTEL = 5913u32,
    FixedSqrtALTERA = 5923u32,
    FixedRecipALTERA = 5924u32,
    FixedRsqrtALTERA = 5925u32,
    FixedSinALTERA = 5926u32,
    FixedCosALTERA = 5927u32,
    FixedSinCosALTERA = 5928u32,
    FixedSinPiALTERA = 5929u32,
    FixedCosPiALTERA = 5930u32,
    FixedSinCosPiALTERA = 5931u32,
    FixedLogALTERA = 5932u32,
    FixedExpALTERA = 5933u32,
    PtrCastToCrossWorkgroupALTERA = 5934u32,
    CrossWorkgroupCastToPtrALTERA = 5938u32,
    ReadPipeBlockingALTERA = 5946u32,
    WritePipeBlockingALTERA = 5947u32,
    FPGARegALTERA = 5949u32,
    RayQueryGetRayTMinKHR = 6016u32,
    RayQueryGetRayFlagsKHR = 6017u32,
    RayQueryGetIntersectionTKHR = 6018u32,
    RayQueryGetIntersectionInstanceCustomIndexKHR = 6019u32,
    RayQueryGetIntersectionInstanceIdKHR = 6020u32,
    RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR = 6021u32,
    RayQueryGetIntersectionGeometryIndexKHR = 6022u32,
    RayQueryGetIntersectionPrimitiveIndexKHR = 6023u32,
    RayQueryGetIntersectionBarycentricsKHR = 6024u32,
    RayQueryGetIntersectionFrontFaceKHR = 6025u32,
    RayQueryGetIntersectionCandidateAABBOpaqueKHR = 6026u32,
    RayQueryGetIntersectionObjectRayDirectionKHR = 6027u32,
    RayQueryGetIntersectionObjectRayOriginKHR = 6028u32,
    RayQueryGetWorldRayDirectionKHR = 6029u32,
    RayQueryGetWorldRayOriginKHR = 6030u32,
    RayQueryGetIntersectionObjectToWorldKHR = 6031u32,
    RayQueryGetIntersectionWorldToObjectKHR = 6032u32,
    AtomicFAddEXT = 6035u32,
    TypeBufferSurfaceINTEL = 6086u32,
    TypeStructContinuedINTEL = 6090u32,
    ConstantCompositeContinuedINTEL = 6091u32,
    SpecConstantCompositeContinuedINTEL = 6092u32,
    CompositeConstructContinuedINTEL = 6096u32,
    ConvertFToBF16INTEL = 6116u32,
    ConvertBF16ToFINTEL = 6117u32,
    ControlBarrierArriveINTEL = 6142u32,
    ControlBarrierWaitINTEL = 6143u32,
    ArithmeticFenceEXT = 6145u32,
    TaskSequenceCreateALTERA = 6163u32,
    TaskSequenceAsyncALTERA = 6164u32,
    TaskSequenceGetALTERA = 6165u32,
    TaskSequenceReleaseALTERA = 6166u32,
    TypeTaskSequenceALTERA = 6199u32,
    SubgroupBlockPrefetchINTEL = 6221u32,
    Subgroup2DBlockLoadINTEL = 6231u32,
    Subgroup2DBlockLoadTransformINTEL = 6232u32,
    Subgroup2DBlockLoadTransposeINTEL = 6233u32,
    Subgroup2DBlockPrefetchINTEL = 6234u32,
    Subgroup2DBlockStoreINTEL = 6235u32,
    SubgroupMatrixMultiplyAccumulateINTEL = 6237u32,
    BitwiseFunctionINTEL = 6242u32,
    UntypedVariableLengthArrayINTEL = 6244u32,
    ConditionalExtensionINTEL = 6248u32,
    ConditionalEntryPointINTEL = 6249u32,
    ConditionalCapabilityINTEL = 6250u32,
    SpecConstantTargetINTEL = 6251u32,
    SpecConstantArchitectureINTEL = 6252u32,
    SpecConstantCapabilitiesINTEL = 6253u32,
    ConditionalCopyObjectINTEL = 6254u32,
    GroupIMulKHR = 6401u32,
    GroupFMulKHR = 6402u32,
    GroupBitwiseAndKHR = 6403u32,
    GroupBitwiseOrKHR = 6404u32,
    GroupBitwiseXorKHR = 6405u32,
    GroupLogicalAndKHR = 6406u32,
    GroupLogicalOrKHR = 6407u32,
    GroupLogicalXorKHR = 6408u32,
    RoundFToTF32INTEL = 6426u32,
    MaskedGatherINTEL = 6428u32,
    MaskedScatterINTEL = 6429u32,
    ConvertHandleToImageINTEL = 6529u32,
    ConvertHandleToSamplerINTEL = 6530u32,
    ConvertHandleToSampledImageINTEL = 6531u32,
}
impl Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=8u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            10u32..=12u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            14u32..=17u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            19u32..=39u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            41u32..=46u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            48u32..=52u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            54u32..=57u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            59u32..=75u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            77u32..=84u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            86u32..=107u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            109u32..=124u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            126u32..=152u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            154u32..=191u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            194u32..=205u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            207u32..=215u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            218u32..=221u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            224u32..=225u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            227u32..=242u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            245u32..=257u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            259u32..=271u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            274u32..=288u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            291u32..=366u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            400u32..=403u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4160u32..=4166u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4181u32..=4186u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4190u32 => unsafe { core::mem::transmute::<u32, Op>(4190u32) },
            4416u32..=4434u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4445u32..=4463u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4472u32..=4477u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4479u32..=4483u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4497u32 => unsafe { core::mem::transmute::<u32, Op>(4497u32) },
            4500u32..=4503u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            4540u32..=4542u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5000u32..=5007u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5011u32..=5012u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5056u32 => unsafe { core::mem::transmute::<u32, Op>(5056u32) },
            5074u32..=5076u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5078u32 => unsafe { core::mem::transmute::<u32, Op>(5078u32) },
            5090u32 => unsafe { core::mem::transmute::<u32, Op>(5090u32) },
            5101u32 => unsafe { core::mem::transmute::<u32, Op>(5101u32) },
            5103u32..=5104u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5110u32..=5111u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5115u32 => unsafe { core::mem::transmute::<u32, Op>(5115u32) },
            5119u32 => unsafe { core::mem::transmute::<u32, Op>(5119u32) },
            5126u32..=5127u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5129u32 => unsafe { core::mem::transmute::<u32, Op>(5129u32) },
            5249u32..=5281u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5283u32 => unsafe { core::mem::transmute::<u32, Op>(5283u32) },
            5288u32..=5296u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5299u32..=5341u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5344u32..=5352u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5358u32..=5362u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5364u32..=5382u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5384u32 => unsafe { core::mem::transmute::<u32, Op>(5384u32) },
            5390u32..=5398u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5427u32..=5439u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5571u32..=5578u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5580u32..=5581u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5585u32..=5598u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5600u32..=5601u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5609u32..=5611u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5614u32..=5615u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5630u32..=5633u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5699u32..=5816u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5818u32..=5820u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5840u32..=5843u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5846u32..=5882u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5887u32 => unsafe { core::mem::transmute::<u32, Op>(5887u32) },
            5911u32..=5913u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5923u32..=5934u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5938u32 => unsafe { core::mem::transmute::<u32, Op>(5938u32) },
            5946u32..=5947u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            5949u32 => unsafe { core::mem::transmute::<u32, Op>(5949u32) },
            6016u32..=6032u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6035u32 => unsafe { core::mem::transmute::<u32, Op>(6035u32) },
            6086u32 => unsafe { core::mem::transmute::<u32, Op>(6086u32) },
            6090u32..=6092u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6096u32 => unsafe { core::mem::transmute::<u32, Op>(6096u32) },
            6116u32..=6117u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6142u32..=6143u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6145u32 => unsafe { core::mem::transmute::<u32, Op>(6145u32) },
            6163u32..=6166u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6199u32 => unsafe { core::mem::transmute::<u32, Op>(6199u32) },
            6221u32 => unsafe { core::mem::transmute::<u32, Op>(6221u32) },
            6231u32..=6235u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6237u32 => unsafe { core::mem::transmute::<u32, Op>(6237u32) },
            6242u32 => unsafe { core::mem::transmute::<u32, Op>(6242u32) },
            6244u32 => unsafe { core::mem::transmute::<u32, Op>(6244u32) },
            6248u32..=6254u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6401u32..=6408u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6426u32 => unsafe { core::mem::transmute::<u32, Op>(6426u32) },
            6428u32..=6429u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            6529u32..=6531u32 => unsafe { core::mem::transmute::<u32, Op>(n) },
            _ => return None,
        })
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(non_upper_case_globals)]
impl Op {
    pub const SDotKHR: Self = Self::SDot;
    pub const UDotKHR: Self = Self::UDot;
    pub const SUDotKHR: Self = Self::SUDot;
    pub const SDotAccSatKHR: Self = Self::SDotAccSat;
    pub const UDotAccSatKHR: Self = Self::UDotAccSat;
    pub const SUDotAccSatKHR: Self = Self::SUDotAccSat;
    pub const TypeCooperativeVectorNV: Self = Self::TypeVectorIdEXT;
    pub const GroupNonUniformPartitionNV: Self = Self::GroupNonUniformPartitionEXT;
    pub const ReportIntersectionNV: Self = Self::ReportIntersectionKHR;
    pub const TypeAccelerationStructureNV: Self = Self::TypeAccelerationStructureKHR;
    pub const RayQueryGetClusterIdNV: Self = Self::RayQueryGetIntersectionClusterIdNV;
    pub const DemoteToHelperInvocationEXT: Self = Self::DemoteToHelperInvocation;
    pub const DecorateStringGOOGLE: Self = Self::DecorateString;
    pub const MemberDecorateStringGOOGLE: Self = Self::MemberDecorateString;
    pub const ArbitraryFloatSinCosPiINTEL: Self = Self::ArbitraryFloatSinCosPiALTERA;
    pub const ArbitraryFloatCastINTEL: Self = Self::ArbitraryFloatCastALTERA;
    pub const ArbitraryFloatCastFromIntINTEL: Self = Self::ArbitraryFloatCastFromIntALTERA;
    pub const ArbitraryFloatCastToIntINTEL: Self = Self::ArbitraryFloatCastToIntALTERA;
    pub const ArbitraryFloatAddINTEL: Self = Self::ArbitraryFloatAddALTERA;
    pub const ArbitraryFloatSubINTEL: Self = Self::ArbitraryFloatSubALTERA;
    pub const ArbitraryFloatMulINTEL: Self = Self::ArbitraryFloatMulALTERA;
    pub const ArbitraryFloatDivINTEL: Self = Self::ArbitraryFloatDivALTERA;
    pub const ArbitraryFloatGTINTEL: Self = Self::ArbitraryFloatGTALTERA;
    pub const ArbitraryFloatGEINTEL: Self = Self::ArbitraryFloatGEALTERA;
    pub const ArbitraryFloatLTINTEL: Self = Self::ArbitraryFloatLTALTERA;
    pub const ArbitraryFloatLEINTEL: Self = Self::ArbitraryFloatLEALTERA;
    pub const ArbitraryFloatEQINTEL: Self = Self::ArbitraryFloatEQALTERA;
    pub const ArbitraryFloatRecipINTEL: Self = Self::ArbitraryFloatRecipALTERA;
    pub const ArbitraryFloatRSqrtINTEL: Self = Self::ArbitraryFloatRSqrtALTERA;
    pub const ArbitraryFloatCbrtINTEL: Self = Self::ArbitraryFloatCbrtALTERA;
    pub const ArbitraryFloatHypotINTEL: Self = Self::ArbitraryFloatHypotALTERA;
    pub const ArbitraryFloatSqrtINTEL: Self = Self::ArbitraryFloatSqrtALTERA;
    pub const FixedSqrtINTEL: Self = Self::FixedSqrtALTERA;
    pub const FixedRecipINTEL: Self = Self::FixedRecipALTERA;
    pub const FixedRsqrtINTEL: Self = Self::FixedRsqrtALTERA;
    pub const FixedSinINTEL: Self = Self::FixedSinALTERA;
    pub const FixedCosINTEL: Self = Self::FixedCosALTERA;
    pub const FixedSinCosINTEL: Self = Self::FixedSinCosALTERA;
    pub const FixedSinPiINTEL: Self = Self::FixedSinPiALTERA;
    pub const FixedCosPiINTEL: Self = Self::FixedCosPiALTERA;
    pub const FixedSinCosPiINTEL: Self = Self::FixedSinCosPiALTERA;
    pub const FixedLogINTEL: Self = Self::FixedLogALTERA;
    pub const FixedExpINTEL: Self = Self::FixedExpALTERA;
    pub const PtrCastToCrossWorkgroupINTEL: Self = Self::PtrCastToCrossWorkgroupALTERA;
    pub const CrossWorkgroupCastToPtrINTEL: Self = Self::CrossWorkgroupCastToPtrALTERA;
    pub const ReadPipeBlockingINTEL: Self = Self::ReadPipeBlockingALTERA;
    pub const WritePipeBlockingINTEL: Self = Self::WritePipeBlockingALTERA;
    pub const FPGARegINTEL: Self = Self::FPGARegALTERA;
    pub const TaskSequenceCreateINTEL: Self = Self::TaskSequenceCreateALTERA;
    pub const TaskSequenceAsyncINTEL: Self = Self::TaskSequenceAsyncALTERA;
    pub const TaskSequenceGetINTEL: Self = Self::TaskSequenceGetALTERA;
    pub const TaskSequenceReleaseINTEL: Self = Self::TaskSequenceReleaseALTERA;
    pub const TypeTaskSequenceINTEL: Self = Self::TypeTaskSequenceALTERA;
    #[doc = r" Returns [`true`] if the given opcode is a type-declaring instruction."]
    #[doc = r""]
    #[doc = r" <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_type_declaration_instructions>"]
    pub fn is_type(self) -> bool {
        matches!(
            self,
            Self::TypeVoid
                | Self::TypeBool
                | Self::TypeInt
                | Self::TypeFloat
                | Self::TypeVector
                | Self::TypeMatrix
                | Self::TypeImage
                | Self::TypeSampler
                | Self::TypeSampledImage
                | Self::TypeArray
                | Self::TypeRuntimeArray
                | Self::TypeStruct
                | Self::TypeOpaque
                | Self::TypePointer
                | Self::TypeFunction
                | Self::TypeEvent
                | Self::TypeDeviceEvent
                | Self::TypeReserveId
                | Self::TypeQueue
                | Self::TypePipe
                | Self::TypeForwardPointer
                | Self::TypePipeStorage
                | Self::TypeNamedBarrier
                | Self::TypeTensorARM
                | Self::TypeGraphARM
                | Self::TypeUntypedPointerKHR
                | Self::TypeCooperativeMatrixKHR
                | Self::TypeRayQueryKHR
                | Self::TypeNodePayloadArrayAMDX
                | Self::TypeBufferEXT
                | Self::TypeHitObjectNV
                | Self::TypeVectorIdEXT
                | Self::TypeHitObjectEXT
                | Self::TypeAccelerationStructureKHR
                | Self::TypeCooperativeMatrixNV
                | Self::TypeTensorLayoutNV
                | Self::TypeTensorViewNV
                | Self::TypeBufferSurfaceINTEL
                | Self::TypeStructContinuedINTEL
        )
    }
    #[doc = r" Returns [`true`] if the given opcode is a constant-defining instruction."]
    #[doc = r""]
    #[doc = r" <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_constant_creation_instructions>"]
    pub fn is_constant(self) -> bool {
        matches!(
            self,
            Self::ConstantTrue
                | Self::ConstantFalse
                | Self::Constant
                | Self::ConstantComposite
                | Self::ConstantSampler
                | Self::ConstantNull
                | Self::SpecConstantTrue
                | Self::SpecConstantFalse
                | Self::SpecConstant
                | Self::SpecConstantComposite
                | Self::SpecConstantOp
                | Self::ConstantCompositeReplicateEXT
                | Self::SpecConstantCompositeReplicateEXT
                | Self::ConstantSizeOfEXT
                | Self::ConstantCompositeContinuedINTEL
                | Self::SpecConstantCompositeContinuedINTEL
                | Self::SpecConstantTargetINTEL
                | Self::SpecConstantArchitectureINTEL
                | Self::SpecConstantCapabilitiesINTEL
        )
    }
    #[doc = r" Returns [`true`] if the given opcode is an annotation instruction."]
    #[doc = r""]
    #[doc = r" <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Annotation>"]
    pub fn is_annotation(self) -> bool {
        matches!(
            self,
            Self::Decorate
                | Self::MemberDecorate
                | Self::DecorationGroup
                | Self::GroupDecorate
                | Self::GroupMemberDecorate
                | Self::DecorateId
                | Self::MemberDecorateIdEXT
                | Self::DecorateString
                | Self::MemberDecorateString
        )
    }
    #[doc = r" Returns [`true`] if the given opcode is a debug instruction."]
    #[doc = r""]
    #[doc = r" <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_debug_instructions>"]
    pub fn is_debug(self) -> bool {
        matches!(
            self,
            Self::SourceContinued
                | Self::Source
                | Self::SourceExtension
                | Self::Name
                | Self::MemberName
                | Self::String
                | Self::Line
                | Self::NoLine
                | Self::ModuleProcessed
        )
    }
    #[doc = r" Returns [`true`] if the given opcode is a control-flow instruction."]
    #[doc = r""]
    #[doc = r" <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_control_flow_instructions>"]
    pub fn is_control_flow(self) -> bool {
        matches!(
            self,
            Self::Phi
                | Self::LoopMerge
                | Self::SelectionMerge
                | Self::Label
                | Self::Branch
                | Self::BranchConditional
                | Self::Switch
                | Self::Kill
                | Self::Return
                | Self::ReturnValue
                | Self::Unreachable
                | Self::LifetimeStart
                | Self::LifetimeStop
                | Self::TerminateInvocation
                | Self::DemoteToHelperInvocation
        )
    }
}
#[doc = "[Arm.MotionEngine.100](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.arm.motion-engine.100.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum ArmMotionEngine100Op {
    MIN_SAD = 0u32,
    MIN_SAD_COST = 1u32,
    RAW_SAD = 2u32,
}
impl ArmMotionEngine100Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=2u32 => unsafe { core::mem::transmute::<u32, ArmMotionEngine100Op>(n) },
            _ => return None,
        })
    }
}
#[doc = "[DebugInfo](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.debuginfo.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum DebuginfoOp {
    DebugInfoNone = 0u32,
    DebugCompilationUnit = 1u32,
    DebugTypeBasic = 2u32,
    DebugTypePointer = 3u32,
    DebugTypeQualifier = 4u32,
    DebugTypeArray = 5u32,
    DebugTypeVector = 6u32,
    DebugTypedef = 7u32,
    DebugTypeFunction = 8u32,
    DebugTypeEnum = 9u32,
    DebugTypeComposite = 10u32,
    DebugTypeMember = 11u32,
    DebugTypeInheritance = 12u32,
    DebugTypePtrToMember = 13u32,
    DebugTypeTemplate = 14u32,
    DebugTypeTemplateParameter = 15u32,
    DebugTypeTemplateTemplateParameter = 16u32,
    DebugTypeTemplateParameterPack = 17u32,
    DebugGlobalVariable = 18u32,
    DebugFunctionDeclaration = 19u32,
    DebugFunction = 20u32,
    DebugLexicalBlock = 21u32,
    DebugLexicalBlockDiscriminator = 22u32,
    DebugScope = 23u32,
    DebugNoScope = 24u32,
    DebugInlinedAt = 25u32,
    DebugLocalVariable = 26u32,
    DebugInlinedVariable = 27u32,
    DebugDeclare = 28u32,
    DebugValue = 29u32,
    DebugOperation = 30u32,
    DebugExpression = 31u32,
    DebugMacroDef = 32u32,
    DebugMacroUndef = 33u32,
}
impl DebuginfoOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=33u32 => unsafe { core::mem::transmute::<u32, DebuginfoOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[GLSL.std.450](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.glsl.std.450.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum GlslStd450Op {
    Round = 1u32,
    RoundEven = 2u32,
    Trunc = 3u32,
    FAbs = 4u32,
    SAbs = 5u32,
    FSign = 6u32,
    SSign = 7u32,
    Floor = 8u32,
    Ceil = 9u32,
    Fract = 10u32,
    Radians = 11u32,
    Degrees = 12u32,
    Sin = 13u32,
    Cos = 14u32,
    Tan = 15u32,
    Asin = 16u32,
    Acos = 17u32,
    Atan = 18u32,
    Sinh = 19u32,
    Cosh = 20u32,
    Tanh = 21u32,
    Asinh = 22u32,
    Acosh = 23u32,
    Atanh = 24u32,
    Atan2 = 25u32,
    Pow = 26u32,
    Exp = 27u32,
    Log = 28u32,
    Exp2 = 29u32,
    Log2 = 30u32,
    Sqrt = 31u32,
    InverseSqrt = 32u32,
    Determinant = 33u32,
    MatrixInverse = 34u32,
    Modf = 35u32,
    ModfStruct = 36u32,
    FMin = 37u32,
    UMin = 38u32,
    SMin = 39u32,
    FMax = 40u32,
    UMax = 41u32,
    SMax = 42u32,
    FClamp = 43u32,
    UClamp = 44u32,
    SClamp = 45u32,
    FMix = 46u32,
    IMix = 47u32,
    Step = 48u32,
    SmoothStep = 49u32,
    Fma = 50u32,
    Frexp = 51u32,
    FrexpStruct = 52u32,
    Ldexp = 53u32,
    PackSnorm4x8 = 54u32,
    PackUnorm4x8 = 55u32,
    PackSnorm2x16 = 56u32,
    PackUnorm2x16 = 57u32,
    PackHalf2x16 = 58u32,
    PackDouble2x32 = 59u32,
    UnpackSnorm2x16 = 60u32,
    UnpackUnorm2x16 = 61u32,
    UnpackHalf2x16 = 62u32,
    UnpackSnorm4x8 = 63u32,
    UnpackUnorm4x8 = 64u32,
    UnpackDouble2x32 = 65u32,
    Length = 66u32,
    Distance = 67u32,
    Cross = 68u32,
    Normalize = 69u32,
    FaceForward = 70u32,
    Reflect = 71u32,
    Refract = 72u32,
    FindILsb = 73u32,
    FindSMsb = 74u32,
    FindUMsb = 75u32,
    InterpolateAtCentroid = 76u32,
    InterpolateAtSample = 77u32,
    InterpolateAtOffset = 78u32,
    NMin = 79u32,
    NMax = 80u32,
    NClamp = 81u32,
}
impl GlslStd450Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=81u32 => unsafe { core::mem::transmute::<u32, GlslStd450Op>(n) },
            _ => return None,
        })
    }
}
#[doc = "[NonSemantic.ClspvReflection](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.nonsemantic.clspvreflection.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NonsemanticClspvreflectionOp {
    Kernel = 1u32,
    ArgumentInfo = 2u32,
    ArgumentStorageBuffer = 3u32,
    ArgumentUniform = 4u32,
    ArgumentPodStorageBuffer = 5u32,
    ArgumentPodUniform = 6u32,
    ArgumentPodPushConstant = 7u32,
    ArgumentSampledImage = 8u32,
    ArgumentStorageImage = 9u32,
    ArgumentSampler = 10u32,
    ArgumentWorkgroup = 11u32,
    SpecConstantWorkgroupSize = 12u32,
    SpecConstantGlobalOffset = 13u32,
    SpecConstantWorkDim = 14u32,
    PushConstantGlobalOffset = 15u32,
    PushConstantEnqueuedLocalSize = 16u32,
    PushConstantGlobalSize = 17u32,
    PushConstantRegionOffset = 18u32,
    PushConstantNumWorkgroups = 19u32,
    PushConstantRegionGroupOffset = 20u32,
    ConstantDataStorageBuffer = 21u32,
    ConstantDataUniform = 22u32,
    LiteralSampler = 23u32,
    PropertyRequiredWorkgroupSize = 24u32,
    SpecConstantSubgroupMaxSize = 25u32,
    ArgumentPointerPushConstant = 26u32,
    ArgumentPointerUniform = 27u32,
    ProgramScopeVariablesStorageBuffer = 28u32,
    ProgramScopeVariablePointerRelocation = 29u32,
    ImageArgumentInfoChannelOrderPushConstant = 30u32,
    ImageArgumentInfoChannelDataTypePushConstant = 31u32,
    ImageArgumentInfoChannelOrderUniform = 32u32,
    ImageArgumentInfoChannelDataTypeUniform = 33u32,
    ArgumentStorageTexelBuffer = 34u32,
    ArgumentUniformTexelBuffer = 35u32,
    ConstantDataPointerPushConstant = 36u32,
    ProgramScopeVariablePointerPushConstant = 37u32,
    PrintfInfo = 38u32,
    PrintfBufferStorageBuffer = 39u32,
    PrintfBufferPointerPushConstant = 40u32,
    NormalizedSamplerMaskPushConstant = 41u32,
    WorkgroupVariableSize = 42u32,
}
impl NonsemanticClspvreflectionOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=42u32 => unsafe { core::mem::transmute::<u32, NonsemanticClspvreflectionOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[NonSemantic.DebugBreak](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.nonsemantic.debugbreak.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NonsemanticDebugbreakOp {
    DebugBreak = 1u32,
}
impl NonsemanticDebugbreakOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32 => unsafe { core::mem::transmute::<u32, NonsemanticDebugbreakOp>(1u32) },
            _ => return None,
        })
    }
}
#[doc = "[NonSemantic.DebugPrintf](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.nonsemantic.debugprintf.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NonsemanticDebugprintfOp {
    DebugPrintf = 1u32,
}
impl NonsemanticDebugprintfOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32 => unsafe { core::mem::transmute::<u32, NonsemanticDebugprintfOp>(1u32) },
            _ => return None,
        })
    }
}
#[doc = "[NonSemantic.Shader.DebugInfo.100](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.nonsemantic.shader.debuginfo.100.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NonsemanticShaderDebuginfo100Op {
    DebugInfoNone = 0u32,
    DebugCompilationUnit = 1u32,
    DebugTypeBasic = 2u32,
    DebugTypePointer = 3u32,
    DebugTypeQualifier = 4u32,
    DebugTypeArray = 5u32,
    DebugTypeVector = 6u32,
    DebugTypedef = 7u32,
    DebugTypeFunction = 8u32,
    DebugTypeEnum = 9u32,
    DebugTypeComposite = 10u32,
    DebugTypeMember = 11u32,
    DebugTypeInheritance = 12u32,
    DebugTypePtrToMember = 13u32,
    DebugTypeTemplate = 14u32,
    DebugTypeTemplateParameter = 15u32,
    DebugTypeTemplateTemplateParameter = 16u32,
    DebugTypeTemplateParameterPack = 17u32,
    DebugGlobalVariable = 18u32,
    DebugFunctionDeclaration = 19u32,
    DebugFunction = 20u32,
    DebugLexicalBlock = 21u32,
    DebugLexicalBlockDiscriminator = 22u32,
    DebugScope = 23u32,
    DebugNoScope = 24u32,
    DebugInlinedAt = 25u32,
    DebugLocalVariable = 26u32,
    DebugInlinedVariable = 27u32,
    DebugDeclare = 28u32,
    DebugValue = 29u32,
    DebugOperation = 30u32,
    DebugExpression = 31u32,
    DebugMacroDef = 32u32,
    DebugMacroUndef = 33u32,
    DebugImportedEntity = 34u32,
    DebugSource = 35u32,
    DebugFunctionDefinition = 101u32,
    DebugSourceContinued = 102u32,
    DebugLine = 103u32,
    DebugNoLine = 104u32,
    DebugBuildIdentifier = 105u32,
    DebugStoragePath = 106u32,
    DebugEntryPoint = 107u32,
    DebugTypeMatrix = 108u32,
}
impl NonsemanticShaderDebuginfo100Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=35u32 => unsafe {
                core::mem::transmute::<u32, NonsemanticShaderDebuginfo100Op>(n)
            },
            101u32..=108u32 => unsafe {
                core::mem::transmute::<u32, NonsemanticShaderDebuginfo100Op>(n)
            },
            _ => return None,
        })
    }
}
#[doc = "[NonSemantic.VkspReflection](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.nonsemantic.vkspreflection.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum NonsemanticVkspreflectionOp {
    Configuration = 1u32,
    StartCounter = 2u32,
    StopCounter = 3u32,
    PushConstants = 4u32,
    SpecializationMapEntry = 5u32,
    DescriptorSetBuffer = 6u32,
    DescriptorSetImage = 7u32,
    DescriptorSetSampler = 8u32,
}
impl NonsemanticVkspreflectionOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=8u32 => unsafe { core::mem::transmute::<u32, NonsemanticVkspreflectionOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[OpenCL.DebugInfo.100](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.opencl.debuginfo.100.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum OpenclDebuginfo100Op {
    DebugInfoNone = 0u32,
    DebugCompilationUnit = 1u32,
    DebugTypeBasic = 2u32,
    DebugTypePointer = 3u32,
    DebugTypeQualifier = 4u32,
    DebugTypeArray = 5u32,
    DebugTypeVector = 6u32,
    DebugTypedef = 7u32,
    DebugTypeFunction = 8u32,
    DebugTypeEnum = 9u32,
    DebugTypeComposite = 10u32,
    DebugTypeMember = 11u32,
    DebugTypeInheritance = 12u32,
    DebugTypePtrToMember = 13u32,
    DebugTypeTemplate = 14u32,
    DebugTypeTemplateParameter = 15u32,
    DebugTypeTemplateTemplateParameter = 16u32,
    DebugTypeTemplateParameterPack = 17u32,
    DebugGlobalVariable = 18u32,
    DebugFunctionDeclaration = 19u32,
    DebugFunction = 20u32,
    DebugLexicalBlock = 21u32,
    DebugLexicalBlockDiscriminator = 22u32,
    DebugScope = 23u32,
    DebugNoScope = 24u32,
    DebugInlinedAt = 25u32,
    DebugLocalVariable = 26u32,
    DebugInlinedVariable = 27u32,
    DebugDeclare = 28u32,
    DebugValue = 29u32,
    DebugOperation = 30u32,
    DebugExpression = 31u32,
    DebugMacroDef = 32u32,
    DebugMacroUndef = 33u32,
    DebugImportedEntity = 34u32,
    DebugSource = 35u32,
    DebugModuleINTEL = 36u32,
}
impl OpenclDebuginfo100Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=36u32 => unsafe { core::mem::transmute::<u32, OpenclDebuginfo100Op>(n) },
            _ => return None,
        })
    }
}
#[doc = "[OpenCL.std](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.opencl.std.100.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum OpenclStd100Op {
    acos = 0u32,
    acosh = 1u32,
    acospi = 2u32,
    asin = 3u32,
    asinh = 4u32,
    asinpi = 5u32,
    atan = 6u32,
    atan2 = 7u32,
    atanh = 8u32,
    atanpi = 9u32,
    atan2pi = 10u32,
    cbrt = 11u32,
    ceil = 12u32,
    copysign = 13u32,
    cos = 14u32,
    cosh = 15u32,
    cospi = 16u32,
    erfc = 17u32,
    erf = 18u32,
    exp = 19u32,
    exp2 = 20u32,
    exp10 = 21u32,
    expm1 = 22u32,
    fabs = 23u32,
    fdim = 24u32,
    floor = 25u32,
    fma = 26u32,
    fmax = 27u32,
    fmin = 28u32,
    fmod = 29u32,
    fract = 30u32,
    frexp = 31u32,
    hypot = 32u32,
    ilogb = 33u32,
    ldexp = 34u32,
    lgamma = 35u32,
    lgamma_r = 36u32,
    log = 37u32,
    log2 = 38u32,
    log10 = 39u32,
    log1p = 40u32,
    logb = 41u32,
    mad = 42u32,
    maxmag = 43u32,
    minmag = 44u32,
    modf = 45u32,
    nan = 46u32,
    nextafter = 47u32,
    pow = 48u32,
    pown = 49u32,
    powr = 50u32,
    remainder = 51u32,
    remquo = 52u32,
    rint = 53u32,
    rootn = 54u32,
    round = 55u32,
    rsqrt = 56u32,
    sin = 57u32,
    sincos = 58u32,
    sinh = 59u32,
    sinpi = 60u32,
    sqrt = 61u32,
    tan = 62u32,
    tanh = 63u32,
    tanpi = 64u32,
    tgamma = 65u32,
    trunc = 66u32,
    half_cos = 67u32,
    half_divide = 68u32,
    half_exp = 69u32,
    half_exp2 = 70u32,
    half_exp10 = 71u32,
    half_log = 72u32,
    half_log2 = 73u32,
    half_log10 = 74u32,
    half_powr = 75u32,
    half_recip = 76u32,
    half_rsqrt = 77u32,
    half_sin = 78u32,
    half_sqrt = 79u32,
    half_tan = 80u32,
    native_cos = 81u32,
    native_divide = 82u32,
    native_exp = 83u32,
    native_exp2 = 84u32,
    native_exp10 = 85u32,
    native_log = 86u32,
    native_log2 = 87u32,
    native_log10 = 88u32,
    native_powr = 89u32,
    native_recip = 90u32,
    native_rsqrt = 91u32,
    native_sin = 92u32,
    native_sqrt = 93u32,
    native_tan = 94u32,
    fclamp = 95u32,
    degrees = 96u32,
    fmax_common = 97u32,
    fmin_common = 98u32,
    mix = 99u32,
    radians = 100u32,
    step = 101u32,
    smoothstep = 102u32,
    sign = 103u32,
    cross = 104u32,
    distance = 105u32,
    length = 106u32,
    normalize = 107u32,
    fast_distance = 108u32,
    fast_length = 109u32,
    fast_normalize = 110u32,
    s_abs = 141u32,
    s_abs_diff = 142u32,
    s_add_sat = 143u32,
    u_add_sat = 144u32,
    s_hadd = 145u32,
    u_hadd = 146u32,
    s_rhadd = 147u32,
    u_rhadd = 148u32,
    s_clamp = 149u32,
    u_clamp = 150u32,
    clz = 151u32,
    ctz = 152u32,
    s_mad_hi = 153u32,
    u_mad_sat = 154u32,
    s_mad_sat = 155u32,
    s_max = 156u32,
    u_max = 157u32,
    s_min = 158u32,
    u_min = 159u32,
    s_mul_hi = 160u32,
    rotate = 161u32,
    s_sub_sat = 162u32,
    u_sub_sat = 163u32,
    u_upsample = 164u32,
    s_upsample = 165u32,
    popcount = 166u32,
    s_mad24 = 167u32,
    u_mad24 = 168u32,
    s_mul24 = 169u32,
    u_mul24 = 170u32,
    vloadn = 171u32,
    vstoren = 172u32,
    vload_half = 173u32,
    vload_halfn = 174u32,
    vstore_half = 175u32,
    vstore_half_r = 176u32,
    vstore_halfn = 177u32,
    vstore_halfn_r = 178u32,
    vloada_halfn = 179u32,
    vstorea_halfn = 180u32,
    vstorea_halfn_r = 181u32,
    shuffle = 182u32,
    shuffle2 = 183u32,
    printf = 184u32,
    prefetch = 185u32,
    bitselect = 186u32,
    select = 187u32,
    u_abs = 201u32,
    u_abs_diff = 202u32,
    u_mul_hi = 203u32,
    u_mad_hi = 204u32,
}
impl OpenclStd100Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=110u32 => unsafe { core::mem::transmute::<u32, OpenclStd100Op>(n) },
            141u32..=187u32 => unsafe { core::mem::transmute::<u32, OpenclStd100Op>(n) },
            201u32..=204u32 => unsafe { core::mem::transmute::<u32, OpenclStd100Op>(n) },
            _ => return None,
        })
    }
}
#[doc = "[SPV_AMD_gcn_shader](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.spv-amd-gcn-shader.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SpvAmdGcnShaderOp {
    CubeFaceIndexAMD = 1u32,
    CubeFaceCoordAMD = 2u32,
    TimeAMD = 3u32,
}
impl SpvAmdGcnShaderOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=3u32 => unsafe { core::mem::transmute::<u32, SpvAmdGcnShaderOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[SPV_AMD_shader_ballot](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.spv-amd-shader-ballot.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SpvAmdShaderBallotOp {
    SwizzleInvocationsAMD = 1u32,
    SwizzleInvocationsMaskedAMD = 2u32,
    WriteInvocationAMD = 3u32,
    MbcntAMD = 4u32,
}
impl SpvAmdShaderBallotOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=4u32 => unsafe { core::mem::transmute::<u32, SpvAmdShaderBallotOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[SPV_AMD_shader_explicit_vertex_parameter](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.spv-amd-shader-explicit-vertex-parameter.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SpvAmdShaderExplicitVertexParameterOp {
    InterpolateAtVertexAMD = 1u32,
}
impl SpvAmdShaderExplicitVertexParameterOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32 => unsafe {
                core::mem::transmute::<u32, SpvAmdShaderExplicitVertexParameterOp>(1u32)
            },
            _ => return None,
        })
    }
}
#[doc = "[SPV_AMD_shader_trinary_minmax](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.spv-amd-shader-trinary-minmax.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum SpvAmdShaderTrinaryMinmaxOp {
    FMin3AMD = 1u32,
    UMin3AMD = 2u32,
    SMin3AMD = 3u32,
    FMax3AMD = 4u32,
    UMax3AMD = 5u32,
    SMax3AMD = 6u32,
    FMid3AMD = 7u32,
    UMid3AMD = 8u32,
    SMid3AMD = 9u32,
}
impl SpvAmdShaderTrinaryMinmaxOp {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            1u32..=9u32 => unsafe { core::mem::transmute::<u32, SpvAmdShaderTrinaryMinmaxOp>(n) },
            _ => return None,
        })
    }
}
#[doc = "[TOSA.001000.1](https://github.com/KhronosGroup/SPIRV-Headers/blob/main/include/spirv/unified1/extinst.tosa.001000.1.grammar.json) extended instruction opcode"]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum Tosa0010001Op {
    ARGMAX = 0u32,
    AVG_POOL2D = 1u32,
    CONV2D = 2u32,
    CONV3D = 3u32,
    DEPTHWISE_CONV2D = 4u32,
    FFT2D = 5u32,
    MATMUL = 6u32,
    MAX_POOL2D = 7u32,
    RFFT2D = 8u32,
    TRANSPOSE_CONV2D = 9u32,
    CLAMP = 10u32,
    ERF = 11u32,
    SIGMOID = 12u32,
    TANH = 13u32,
    ADD = 14u32,
    ARITHMETIC_RIGHT_SHIFT = 15u32,
    BITWISE_AND = 16u32,
    BITWISE_OR = 17u32,
    BITWISE_XOR = 18u32,
    INTDIV = 19u32,
    LOGICAL_AND = 20u32,
    LOGICAL_LEFT_SHIFT = 21u32,
    LOGICAL_RIGHT_SHIFT = 22u32,
    LOGICAL_OR = 23u32,
    LOGICAL_XOR = 24u32,
    MAXIMUM = 25u32,
    MINIMUM = 26u32,
    MUL = 27u32,
    POW = 28u32,
    SUB = 29u32,
    TABLE = 30u32,
    ABS = 31u32,
    BITWISE_NOT = 32u32,
    CEIL = 33u32,
    CLZ = 34u32,
    COS = 35u32,
    EXP = 36u32,
    FLOOR = 37u32,
    LOG = 38u32,
    LOGICAL_NOT = 39u32,
    NEGATE = 40u32,
    RECIPROCAL = 41u32,
    RSQRT = 42u32,
    SIN = 43u32,
    SELECT = 44u32,
    EQUAL = 45u32,
    GREATER = 46u32,
    GREATER_EQUAL = 47u32,
    REDUCE_ALL = 48u32,
    REDUCE_ANY = 49u32,
    REDUCE_MAX = 50u32,
    REDUCE_MIN = 51u32,
    REDUCE_PRODUCT = 52u32,
    REDUCE_SUM = 53u32,
    CONCAT = 54u32,
    PAD = 55u32,
    RESHAPE = 56u32,
    REVERSE = 57u32,
    SLICE = 58u32,
    TILE = 59u32,
    TRANSPOSE = 60u32,
    GATHER = 61u32,
    SCATTER = 62u32,
    RESIZE = 63u32,
    CAST = 64u32,
    RESCALE = 65u32,
}
impl Tosa0010001Op {
    pub fn from_u32(n: u32) -> Option<Self> {
        Some(match n {
            0u32..=65u32 => unsafe { core::mem::transmute::<u32, Tosa0010001Op>(n) },
            _ => return None,
        })
    }
}
