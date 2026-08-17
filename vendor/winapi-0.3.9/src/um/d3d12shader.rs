// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::minwindef::{BOOL, BYTE, INT, LPVOID, UINT};
use um::d3dcommon::{
    D3D_CBUFFER_TYPE, D3D_FEATURE_LEVEL, D3D_INTERPOLATION_MODE, D3D_MIN_PRECISION, D3D_NAME,
    D3D_PARAMETER_FLAGS, D3D_PRIMITIVE, D3D_PRIMITIVE_TOPOLOGY, D3D_REGISTER_COMPONENT_TYPE,
    D3D_RESOURCE_RETURN_TYPE, D3D_SHADER_INPUT_TYPE, D3D_SHADER_VARIABLE_CLASS,
    D3D_SHADER_VARIABLE_TYPE, D3D_SRV_DIMENSION, D3D_TESSELLATOR_DOMAIN,
    D3D_TESSELLATOR_OUTPUT_PRIMITIVE, D3D_TESSELLATOR_PARTITIONING,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR};
ENUM!{enum D3D12_SHADER_VERSION_TYPE {
    D3D12_SHVER_PIXEL_SHADER = 0x0,
    D3D12_SHVER_VERTEX_SHADER = 0x1,
    D3D12_SHVER_GEOMETRY_SHADER = 0x2,
    D3D12_SHVER_HULL_SHADER = 0x3,
    D3D12_SHVER_DOMAIN_SHADER = 0x4,
    D3D12_SHVER_COMPUTE_SHADER = 0x5,
    D3D12_SHVER_RESERVED0 = 0xFFF0,
}}
STRUCT!{struct D3D12_FUNCTION_DESC {
    Version: UINT,
    Creator: LPCSTR,
    Flags: UINT,
    ConstantBuffers: UINT,
    BoundResources: UINT,
    InstructionCount: UINT,
    TempRegisterCount: UINT,
    TempArrayCount: UINT,
    DefCount: UINT,
    DclCount: UINT,
    TextureNormalInstructions: UINT,
    TextureLoadInstructions: UINT,
    TextureCompInstructions: UINT,
    TextureBiasInstructions: UINT,
    TextureGradientInstructions: UINT,
    FloatInstructionCount: UINT,
    IntInstructionCount: UINT,
    UintInstructionCount: UINT,
    StaticFlowControlCount: UINT,
    DynamicFlowControlCount: UINT,
    MacroInstructionCount: UINT,
    ArrayInstructionCount: UINT,
    MovInstructionCount: UINT,
    MovcInstructionCount: UINT,
    ConversionInstructionCount: UINT,
    BitwiseInstructionCount: UINT,
    MinFeatureLevel: D3D_FEATURE_LEVEL,
    RequiredFeatureFlags: UINT64,
    Name: LPCSTR,
    FunctionParameterCount: INT,
    HasReturn: BOOL,
    Has10Level9VertexShader: BOOL,
    Has10Level9PixelShader: BOOL,
}}
STRUCT!{struct D3D12_LIBRARY_DESC {
    Creator: LPCSTR,
    Flags: UINT,
    FunctionCount: UINT,
}}
STRUCT!{struct D3D12_PARAMETER_DESC {
    Name: LPCSTR,
    SemanticName: LPCSTR,
    Type: D3D_SHADER_VARIABLE_TYPE,
    Class: D3D_SHADER_VARIABLE_CLASS,
    Rows: UINT,
    Columns: UINT,
    InterpolationMode: D3D_INTERPOLATION_MODE,
    Flags: D3D_PARAMETER_FLAGS,
    FirstInRegister: UINT,
    FirstInComponent: UINT,
    FirstOutRegister: UINT,
    FirstOutComponent: UINT,
}}
STRUCT!{struct D3D12_SHADER_BUFFER_DESC {
    Name: LPCSTR,
    Type: D3D_CBUFFER_TYPE,
    Variables: UINT,
    Size: UINT,
    uFlags: UINT,
}}
STRUCT!{struct D3D12_SHADER_DESC {
    Version: UINT,
    Creator: LPCSTR,
    Flags: UINT,
    ConstantBuffers: UINT,
    BoundResources: UINT,
    InputParameters: UINT,
    OutputParameters: UINT,
    InstructionCount: UINT,
    TempRegisterCount: UINT,
    TempArrayCount: UINT,
    DefCount: UINT,
    DclCount: UINT,
    TextureNormalInstructions: UINT,
    TextureLoadInstructions: UINT,
    TextureCompInstructions: UINT,
    TextureBiasInstructions: UINT,
    TextureGradientInstructions: UINT,
    FloatInstructionCount: UINT,
    IntInstructionCount: UINT,
    UintInstructionCount: UINT,
    StaticFlowControlCount: UINT,
    DynamicFlowControlCount: UINT,
    MacroInstructionCount: UINT,
    ArrayInstructionCount: UINT,
    CutInstructionCount: UINT,
    EmitInstructionCount: UINT,
    GSOutputTopology: D3D_PRIMITIVE_TOPOLOGY,
    GSMaxOutputVertexCount: UINT,
    InputPrimitive: D3D_PRIMITIVE,
    PatchConstantParameters: UINT,
    cGSInstanceCount: UINT,
    cControlPoints: UINT,
    HSOutputPrimitive: D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    HSPartitioning: D3D_TESSELLATOR_PARTITIONING,
    TessellatorDomain: D3D_TESSELLATOR_DOMAIN,
    cBarrierInstructions: UINT,
    cInterlockedInstructions: UINT,
    cTextureStoreInstructions: UINT,
}}
STRUCT!{struct D3D12_SHADER_INPUT_BIND_DESC {
    Name: LPCSTR,
    Type: D3D_SHADER_INPUT_TYPE,
    BindPoint: UINT,
    BindCount: UINT,
    uFlags: UINT,
    ReturnType: D3D_RESOURCE_RETURN_TYPE,
    Dimension: D3D_SRV_DIMENSION,
    NumSamples: UINT,
    Space: UINT,
    uID: UINT,
}}
STRUCT!{struct D3D12_SHADER_TYPE_DESC {
    Class: D3D_SHADER_VARIABLE_CLASS,
    Type: D3D_SHADER_VARIABLE_TYPE,
    Rows: UINT,
    Columns: UINT,
    Elements: UINT,
    Members: UINT,
    Offset: UINT,
    Name: LPCSTR,
}}
STRUCT!{struct D3D12_SHADER_VARIABLE_DESC {
    Name: LPCSTR,
    StartOffset: UINT,
    Size: UINT,
    uFlags: UINT,
    DefaultValue: LPVOID,
    StartTexture: UINT,
    TextureSize: UINT,
    StartSampler: UINT,
    SamplerSize: UINT,
}}
STRUCT!{struct D3D12_SIGNATURE_PARAMETER_DESC {
    SemanticName: LPCSTR,
    SemanticIndex: UINT,
    Register: UINT,
    SystemValueType: D3D_NAME,
    ComponentType: D3D_REGISTER_COMPONENT_TYPE,
    Mask: BYTE,
    ReadWriteMask: BYTE,
    Stream: UINT,
    MinPrecision: D3D_MIN_PRECISION,
}}
RIDL!{#[uuid(0xec25f42d, 0x7006, 0x4f2b, 0xb3, 0x3e, 0x02, 0xcc, 0x33, 0x75, 0x73, 0x3f)]
interface ID3D12FunctionParameterReflection(ID3D12FunctionParameterReflectionVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_PARAMETER_DESC,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1108795c, 0x2772, 0x4ba9, 0xb2, 0xa8, 0xd4, 0x64, 0xdc, 0x7e, 0x27, 0x99)]
interface ID3D12FunctionReflection(ID3D12FunctionReflectionVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_FUNCTION_DESC,
    ) -> HRESULT,
    fn GetConstantBufferByIndex(
        BufferIndex: UINT,
    ) -> *mut ID3D12ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D12_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionVariable,
    fn GetResourceBindingDescByName(
        Name: LPCSTR,
        pDesc: *mut D3D12_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetFunctionParameter(
        ParameterIndex: INT,
    ) -> *mut ID3D12FunctionParameterReflection,
}}
RIDL!{#[uuid(0x8e349d19, 0x54db, 0x4a56, 0x9d, 0xc9, 0x11, 0x9d, 0x87, 0xbd, 0xb8, 0x4)]
interface ID3D12LibraryReflection(ID3D12LibraryReflectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_LIBRARY_DESC,
    ) -> HRESULT,
    fn GetFunctionByIndex(
        FunctionIndex: INT,
    ) -> *mut ID3D12FunctionReflection,
}}
DEFINE_GUID!{IID_ID3D12ShaderReflectionConstantBuffer,
    0xc59598b4, 0x48b3, 0x4869, 0xb9, 0xb1, 0xb1, 0x61, 0x8b, 0x14, 0xa8, 0xb7}
RIDL!{#[uuid(0xc59598b4, 0x48b3, 0x4869, 0xb9, 0xb1, 0xb1, 0x61, 0x8b, 0x14, 0xa8, 0xb7)]
interface ID3D12ShaderReflectionConstantBuffer(ID3D12ShaderReflectionConstantBufferVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_SHADER_BUFFER_DESC,
    ) -> HRESULT,
    fn GetVariableByIndex(
        Index: UINT,
    ) -> *mut ID3D12ShaderReflectionVariable,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionVariable,
}}
DEFINE_GUID!{IID_ID3D12ShaderReflectionType,
    0xe913c351, 0x783d, 0x48ca, 0xa1, 0xd1, 0x4f, 0x30, 0x62, 0x84, 0xad, 0x56}
RIDL!{#[uuid(0xe913c351, 0x783d, 0x48ca, 0xa1, 0xd1, 0x4f, 0x30, 0x62, 0x84, 0xad, 0x56)]
interface ID3D12ShaderReflectionType(ID3D12ShaderReflectionTypeVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_SHADER_TYPE_DESC,
    ) -> HRESULT,
    fn GetMemberTypeByIndex(
        Index: UINT,
    ) -> *mut ID3D12ShaderReflectionType,
    fn GetMemberTypeByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionType,
    fn GetMemberTypeName(
        Index: UINT,
    ) -> LPCSTR,
    fn IsEqual(
        pType: *mut ID3D12ShaderReflectionType,
    ) -> HRESULT,
    fn GetSubType() -> *mut ID3D12ShaderReflectionType,
    fn GetBaseClass() -> *mut ID3D12ShaderReflectionType,
    fn GetNumInterfaces() -> UINT,
    fn GetInterfaceByIndex(
        uIndex: UINT,
    ) -> *mut ID3D12ShaderReflectionType,
    fn IsOfType(
        pType: *mut ID3D12ShaderReflectionType,
    ) -> HRESULT,
    fn ImplementsInterface(
        pBase: *mut ID3D12ShaderReflectionType,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D12ShaderReflectionVariable,
    0x8337a8a6, 0xa216, 0x444a, 0xb2, 0xf4, 0x31, 0x47, 0x33, 0xa7, 0x3a, 0xea}
RIDL!{#[uuid(0x8337a8a6, 0xa216, 0x444a, 0xb2, 0xf4, 0x31, 0x47, 0x33, 0xa7, 0x3a, 0xea)]
interface ID3D12ShaderReflectionVariable(ID3D12ShaderReflectionVariableVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_SHADER_VARIABLE_DESC,
    ) -> HRESULT,
    fn GetType() -> *mut ID3D12ShaderReflectionType,
    fn GetBuffer() -> *mut ID3D12ShaderReflectionConstantBuffer,
    fn GetInterfaceSlot(
        uArrayIndex: UINT,
    ) -> UINT,
}}
DEFINE_GUID!{IID_ID3D12ShaderReflection,
    0x5a58797d, 0xa72c, 0x478d, 0x8b, 0xa2, 0xef, 0xc6, 0xb0, 0xef, 0xe8, 0x8e}
RIDL!{#[uuid(0x5a58797d, 0xa72c, 0x478d, 0x8b, 0xa2, 0xef, 0xc6, 0xb0, 0xef, 0xe8, 0x8e)]
interface ID3D12ShaderReflection(ID3D12ShaderReflectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetDesc(
        pDesc: *mut D3D12_SHADER_DESC,
    ) -> HRESULT,
    fn GetConstantBufferByIndex(
        Index: UINT,
    ) -> *mut ID3D12ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D12_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetInputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D12_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetOutputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D12_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetPatchConstantParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D12_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D12ShaderReflectionVariable,
    fn GetResourceBindingDescByName(
        Name: LPCSTR,
        pDesc: *mut D3D12_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetMovInstructionCount() -> UINT,
    fn GetMovcInstructionCount() -> UINT,
    fn GetConversionInstructionCount() -> UINT,
    fn GetBitwiseInstructionCount() -> UINT,
    fn GetGSInputPrimitive() -> D3D_PRIMITIVE,
    fn IsSampleFrequencyShader() -> BOOL,
    fn GetNumInterfaceSlots() -> UINT,
    fn GetMinFeatureLevel(
        pLevel: *mut D3D_FEATURE_LEVEL,
    ) -> HRESULT,
    fn GetThreadGroupSize(
        pSizeX: *mut UINT,
        pSizeY: *mut UINT,
        pSizeZ: *mut UINT,
    ) -> UINT,
    fn GetRequiresFlags() -> UINT64,
}}
DEFINE_GUID!{IID_ID3D12LibraryReflection,
    0x8e349d19, 0x54db, 0x4a56, 0x9d, 0xc9, 0x11, 0x9d, 0x87, 0xbd, 0xb8, 0x04}
DEFINE_GUID!{IID_ID3D12FunctionReflection,
    0x1108795c, 0x2772, 0x4ba9, 0xb2, 0xa8, 0xd4, 0x64, 0xdc, 0x7e, 0x27, 0x99}
DEFINE_GUID!{IID_ID3D12FunctionParameterReflection,
    0xec25f42d, 0x7006, 0x4f2b, 0xb3, 0x3e, 0x02, 0xcc, 0x33, 0x75, 0x73, 0x3f}
pub type D3D12_CBUFFER_TYPE = D3D_CBUFFER_TYPE;
pub type D3D12_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE;
pub type D3D12_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN;
pub type D3D12_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
pub type D3D12_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING;
pub type LPD3D12FUNCTIONPARAMETERREFLECTION = *mut ID3D12FunctionParameterReflection;
pub type LPD3D12FUNCTIONREFLECTION = *mut ID3D12FunctionReflection;
pub type LPD3D12LIBRARYREFLECTION = *mut ID3D12LibraryReflection;
pub type LPD3D12SHADERREFLECTION = *mut ID3D12ShaderReflection;
pub type LPD3D12SHADERREFLECTIONCONSTANTBUFFER = *mut ID3D12ShaderReflectionConstantBuffer;
pub type LPD3D12SHADERREFLECTIONTYPE = *mut ID3D12ShaderReflectionType;
pub type LPD3D12SHADERREFLECTIONVARIABLE = *mut ID3D12ShaderReflectionVariable;
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: UINT64 = 0x00000400;
pub const D3D_SHADER_REQUIRES_ROVS: UINT64 = 0x00001000;
pub const D3D_SHADER_REQUIRES_STENCIL_REF: UINT64 = 0x00000200;
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: UINT64 = 0x00000800;
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER:
    UINT64 = 0x00002000;
