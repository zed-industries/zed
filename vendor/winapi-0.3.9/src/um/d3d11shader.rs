// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::basetsd::UINT64;
use shared::minwindef::{BOOL, BYTE, INT, LPVOID, UINT};
use um::d3dcommon::{
    D3D_CBUFFER_TYPE, D3D_FEATURE_LEVEL, D3D_INTERPOLATION_MODE, D3D_MIN_PRECISION, D3D_NAME,
    D3D_PARAMETER_FLAGS, D3D_PRIMITIVE, D3D_PRIMITIVE_TOPOLOGY, D3D_REGISTER_COMPONENT_TYPE,
    D3D_RESOURCE_RETURN_TYPE, D3D_SHADER_INPUT_TYPE, D3D_SHADER_VARIABLE_CLASS,
    D3D_SHADER_VARIABLE_TYPE, D3D_SRV_DIMENSION, D3D_TESSELLATOR_DOMAIN,
    D3D_TESSELLATOR_OUTPUT_PRIMITIVE, D3D_TESSELLATOR_PARTITIONING, ID3DBlob,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR};
ENUM!{enum D3D11_SHADER_VERSION_TYPE {
    D3D11_SHVER_PIXEL_SHADER = 0,
    D3D11_SHVER_VERTEX_SHADER = 1,
    D3D11_SHVER_GEOMETRY_SHADER = 2,
    D3D11_SHVER_HULL_SHADER = 3,
    D3D11_SHVER_DOMAIN_SHADER = 4,
    D3D11_SHVER_COMPUTE_SHADER = 5,
    D3D11_SHVER_RESERVED0 = 0xFFF0,
}}
pub const D3D_RETURN_PARAMETER_INDEX: c_int = -1;
pub type D3D11_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE;
pub type D3D11_CBUFFER_TYPE = D3D_CBUFFER_TYPE;
STRUCT!{struct D3D11_SIGNATURE_PARAMETER_DESC {
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
STRUCT!{struct D3D11_SHADER_BUFFER_DESC {
    Name: LPCSTR,
    Type: D3D_CBUFFER_TYPE,
    Variables: UINT,
    Size: UINT,
    uFlags: UINT,
}}
STRUCT!{struct D3D11_SHADER_VARIABLE_DESC {
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
STRUCT!{struct D3D11_SHADER_TYPE_DESC {
    Class: D3D_SHADER_VARIABLE_CLASS,
    Type: D3D_SHADER_VARIABLE_TYPE,
    Rows: UINT,
    Columns: UINT,
    Elements: UINT,
    Members: UINT,
    Offset: UINT,
    Name: LPCSTR,
}}
pub type D3D11_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN;
pub type D3D11_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING;
pub type D3D11_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
STRUCT!{struct D3D11_SHADER_DESC {
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
STRUCT!{struct D3D11_SHADER_INPUT_BIND_DESC {
    Name: LPCSTR,
    Type: D3D_SHADER_INPUT_TYPE,
    BindPoint: UINT,
    BindCount: UINT,
    uFlags: UINT,
    ReturnType: D3D_RESOURCE_RETURN_TYPE,
    Dimension: D3D_SRV_DIMENSION,
    NumSamples: UINT,
}}
pub const D3D_SHADER_REQUIRES_DOUBLES: UINT64 = 0x00000001;
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: UINT64 = 0x00000002;
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: UINT64 = 0x00000004;
pub const D3D_SHADER_REQUIRES_64_UAVS: UINT64 = 0x00000008;
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: UINT64 = 0x00000010;
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: UINT64 = 0x00000020;
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: UINT64 = 0x00000040;
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: UINT64 = 0x00000080;
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: UINT64 = 0x00000100;
STRUCT!{struct D3D11_LIBRARY_DESC {
    Creator: LPCSTR,
    Flags: UINT,
    FunctionCount: UINT,
}}
STRUCT!{struct D3D11_FUNCTION_DESC {
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
STRUCT!{struct D3D11_PARAMETER_DESC {
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
DEFINE_GUID!{IID_ID3D11ShaderReflectionType,
    0x6e6ffa6a, 0x9bae, 0x4613, 0xa5, 0x1e, 0x91, 0x65, 0x2d, 0x50, 0x8c, 0x21}
RIDL!{#[uuid(0x6e6ffa6a, 0x9bae, 0x4613, 0xa5, 0x1e, 0x91, 0x65, 0x2d, 0x50, 0x8c, 0x21)]
interface ID3D11ShaderReflectionType(ID3D11ShaderReflectionTypeVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_SHADER_TYPE_DESC,
    ) -> HRESULT,
    fn GetMemberTypeByIndex(
        Index: UINT,
    ) -> *mut ID3D11ShaderReflectionType,
    fn GetMemberTypeByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionType,
    fn GetMemberTypeName(
        Index: UINT,
    ) -> LPCSTR,
    fn IsEqual(
        pType: *mut ID3D11ShaderReflectionType,
    ) -> HRESULT,
    fn GetSubType() -> *mut ID3D11ShaderReflectionType,
    fn GetBaseClass() -> *mut ID3D11ShaderReflectionType,
    fn GetNumInterfaces() -> UINT,
    fn GetInterfaceByIndex(
        uIndex: UINT,
    ) -> *mut ID3D11ShaderReflectionType,
    fn IsOfType(
        pType: *mut ID3D11ShaderReflectionType,
    ) -> HRESULT,
    fn ImplementsInterface(
        pBase: *mut ID3D11ShaderReflectionType,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D11ShaderReflectionVariable,
    0x51f23923, 0xf3e5, 0x4bd1, 0x91, 0xcb, 0x60, 0x61, 0x77, 0xd8, 0xdb, 0x4c}
RIDL!{#[uuid(0x51f23923, 0xf3e5, 0x4bd1, 0x91, 0xcb, 0x60, 0x61, 0x77, 0xd8, 0xdb, 0x4c)]
interface ID3D11ShaderReflectionVariable(ID3D11ShaderReflectionVariableVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_SHADER_VARIABLE_DESC,
    ) -> HRESULT,
    fn GetType() -> *mut ID3D11ShaderReflectionType,
    fn GetBuffer() -> *mut ID3D11ShaderReflectionConstantBuffer,
    fn GetInterfaceSlot(
        uArrayIndex: UINT,
    ) -> UINT,
}}
DEFINE_GUID!{IID_ID3D11ShaderReflectionConstantBuffer,
    0xeb62d63d, 0x93dd, 0x4318, 0x8a, 0xe8, 0xc6, 0xf8, 0x3a, 0xd3, 0x71, 0xb8}
RIDL!{#[uuid(0xeb62d63d, 0x93dd, 0x4318, 0x8a, 0xe8, 0xc6, 0xf8, 0x3a, 0xd3, 0x71, 0xb8)]
interface ID3D11ShaderReflectionConstantBuffer(ID3D11ShaderReflectionConstantBufferVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_SHADER_BUFFER_DESC,
    ) -> HRESULT,
    fn GetVariableByIndex(
        Index: UINT,
    ) -> *mut ID3D11ShaderReflectionVariable,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionVariable,
}}
DEFINE_GUID!{IID_ID3D11ShaderReflection,
    0x8d536ca1, 0x0cca, 0x4956, 0xa8, 0x37, 0x78, 0x69, 0x63, 0x75, 0x55, 0x84}
RIDL!{#[uuid(0x8d536ca1, 0x0cca, 0x4956, 0xa8, 0x37, 0x78, 0x69, 0x63, 0x75, 0x55, 0x84)]
interface ID3D11ShaderReflection(ID3D11ShaderReflectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_SHADER_DESC,
    ) -> HRESULT,
    fn GetConstantBufferByIndex(
        Index: UINT,
    ) -> *mut ID3D11ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetInputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetOutputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetPatchConstantParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionVariable,
    fn GetResourceBindingDescByName(
        Name: LPCSTR,
        pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC,
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
DEFINE_GUID!{IID_ID3D11LibraryReflection,
    0x54384f1b, 0x5b3e, 0x4bb7, 0xae, 0x01, 0x60, 0xba, 0x30, 0x97, 0xcb, 0xb6}
RIDL!{#[uuid(0x54384f1b, 0x5b3e, 0x4bb7, 0xae, 0x01, 0x60, 0xba, 0x30, 0x97, 0xcb, 0xb6)]
interface ID3D11LibraryReflection(ID3D11LibraryReflectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_LIBRARY_DESC,
    ) -> HRESULT,
    fn GetFunctionByIndex(
        FunctionIndex: INT,
    ) -> *mut ID3D11FunctionReflection,
}}
DEFINE_GUID!{IID_ID3D11FunctionReflection,
    0x207bcecb, 0xd683, 0x4a06, 0xa8, 0xa3, 0x9b, 0x14, 0x9b, 0x9f, 0x73, 0xa4}
RIDL!{#[uuid(0x207bcecb, 0xd683, 0x4a06, 0xa8, 0xa3, 0x9b, 0x14, 0x9b, 0x9f, 0x73, 0xa4)]
interface ID3D11FunctionReflection(ID3D11FunctionReflectionVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_FUNCTION_DESC,
    ) -> HRESULT,
    fn GetConstantBufferByIndex(
        BufferIndex: UINT,
    ) -> *mut ID3D11ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D11ShaderReflectionVariable,
    fn GetResourceBindingDescByName(
        Name: LPCSTR,
        pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetFunctionParameter(
        ParameterIndex: INT,
    ) -> *mut ID3D11FunctionParameterReflection,
}}
DEFINE_GUID!{IID_ID3D11FunctionParameterReflection,
    0x42757488, 0x334f, 0x47fe, 0x98, 0x2e, 0x1a, 0x65, 0xd0, 0x8c, 0xc4, 0x62}
RIDL!{#[uuid(0x42757488, 0x334f, 0x47fe, 0x98, 0x2e, 0x1a, 0x65, 0xd0, 0x8c, 0xc4, 0x62)]
interface ID3D11FunctionParameterReflection(ID3D11FunctionParameterReflectionVtbl) {
    fn GetDesc(
        pDesc: *mut D3D11_PARAMETER_DESC,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D11Module,
    0xcac701ee, 0x80fc, 0x4122, 0x82, 0x42, 0x10, 0xb3, 0x9c, 0x8c, 0xec, 0x34}
RIDL!{#[uuid(0xcac701ee, 0x80fc, 0x4122, 0x82, 0x42, 0x10, 0xb3, 0x9c, 0x8c, 0xec, 0x34)]
interface ID3D11Module(ID3D11ModuleVtbl): IUnknown(IUnknownVtbl) {
    fn CreateInstance(
        pNamespace: LPCSTR,
        ppModuleInstance: *mut *mut ID3D11ModuleInstance,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D11ModuleInstance,
    0x469e07f7, 0x045a, 0x48d5, 0xaa, 0x12, 0x68, 0xa4, 0x78, 0xcd, 0xf7, 0x5d}
RIDL!{#[uuid(0x469e07f7, 0x045a, 0x48d5, 0xaa, 0x12, 0x68, 0xa4, 0x78, 0xcd, 0xf7, 0x5d)]
interface ID3D11ModuleInstance(ID3D11ModuleInstanceVtbl): IUnknown(IUnknownVtbl) {
    fn BindConstantBuffer(
        uSrcSlot: UINT,
        uDstSlot: UINT,
        cbDstOffset: UINT,
    ) -> HRESULT,
    fn BindConstantBufferByName(
        pName: LPCSTR,
        uDstSlot: UINT,
        cbDstOffset: UINT,
    ) -> HRESULT,
    fn BindResource(
        uSrcSlot: UINT,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindResourceByName(
        pName: LPCSTR,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindSampler(
        uSrcSlot: UINT,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindSamplerByName(
        pName: LPCSTR,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindUnorderedAccessView(
        uSrcSlot: UINT,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindUnorderedAccessViewByName(
        pName: LPCSTR,
        uDstSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindResourceAsUnorderedAccessView(
        uSrcSrvSlot: UINT,
        uDstUavSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
    fn BindResourceAsUnorderedAccessViewByName(
        pSrvName: LPCSTR,
        uDstUavSlot: UINT,
        uCount: UINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D11Linker,
    0x59a6cd0e, 0xe10d, 0x4c1f, 0x88, 0xc0, 0x63, 0xab, 0xa1, 0xda, 0xf3, 0x0e}
RIDL!{#[uuid(0x59a6cd0e, 0xe10d, 0x4c1f, 0x88, 0xc0, 0x63, 0xab, 0xa1, 0xda, 0xf3, 0x0e)]
interface ID3D11Linker(ID3D11LinkerVtbl): IUnknown(IUnknownVtbl) {
    fn Link(
        pEntry: *mut ID3D11ModuleInstance,
        pEntryName: LPCSTR,
        pTargetName: LPCSTR,
        uFlags: UINT,
        ppShaderBlob: *mut *mut ID3DBlob,
        ppErrorBuffer: *mut *mut ID3DBlob,
    ) -> HRESULT,
    fn UseLibrary(
        pLibraryMI: *mut ID3D11ModuleInstance,
    ) -> HRESULT,
    fn AddClipPlaneFromCBuffer(
        uCBufferSlot: UINT,
        uCBufferEntry: UINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ID3D11LinkingNode,
    0xd80dd70c, 0x8d2f, 0x4751, 0x94, 0xa1, 0x03, 0xc7, 0x9b, 0x35, 0x56, 0xdb}
RIDL!{#[uuid(0xd80dd70c, 0x8d2f, 0x4751, 0x94, 0xa1, 0x03, 0xc7, 0x9b, 0x35, 0x56, 0xdb)]
interface ID3D11LinkingNode(ID3D11LinkingNodeVtbl): IUnknown(IUnknownVtbl) {}}
DEFINE_GUID!{IID_ID3D11FunctionLinkingGraph,
    0x54133220, 0x1ce8, 0x43d3, 0x82, 0x36, 0x98, 0x55, 0xc5, 0xce, 0xec, 0xff}
RIDL!{#[uuid(0x54133220, 0x1ce8, 0x43d3, 0x82, 0x36, 0x98, 0x55, 0xc5, 0xce, 0xec, 0xff)]
interface ID3D11FunctionLinkingGraph(ID3D11FunctionLinkingGraphVtbl): IUnknown(IUnknownVtbl) {
    fn CreateModuleInstance(
        ppModuleInstance: *mut *mut ID3D11ModuleInstance,
        ppErrorBuffer: *mut *mut ID3DBlob,
    ) -> HRESULT,
    fn SetInputSignature(
        pInputParameters: *const D3D11_PARAMETER_DESC,
        cInputParameters: UINT,
        ppInputNode: *mut *mut ID3D11LinkingNode,
    ) -> HRESULT,
    fn SetOutputSignature(
        pOutputParameters: *const D3D11_PARAMETER_DESC,
        cOutputParameters: UINT,
        ppOutputNode: *mut *mut ID3D11LinkingNode,
    ) -> HRESULT,
    fn CallFunction(
        pModuleInstanceNamespace: LPCSTR,
        pModuleWithFunctionPrototype: *mut ID3D11Module,
        pFunctionName: LPCSTR,
        ppCallNode: *mut *mut ID3D11LinkingNode,
    ) -> HRESULT,
    fn PassValue(
        pSrcNode: *mut ID3D11LinkingNode,
        SrcParameterIndex: INT,
        pDstNode: *mut ID3D11LinkingNode,
        DstParameterIndex: INT,
    ) -> HRESULT,
    fn PassValueWithSwizzle(
        pSrcNode: *mut ID3D11LinkingNode,
        SrcParameterIndex: INT,
        pSrcSwizzle: LPCSTR,
        pDstNode: *mut ID3D11LinkingNode,
        DstParameterIndex: INT,
        pDstSwizzle: LPCSTR,
    ) -> HRESULT,
    fn GetLastError(
        ppErrorBuffer: *mut *mut ID3DBlob,
    ) -> HRESULT,
    fn GenerateHlsl(
        uFlags: UINT,
        ppBuffer: *mut *mut ID3DBlob,
    ) -> HRESULT,
}}
