// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::minwindef::{BYTE, LPVOID, UINT};
use um::d3d10::{D3D10_PRIMITIVE_TOPOLOGY, D3D10_SRV_DIMENSION};
use um::d3dcommon::{
    D3D_CBUFFER_TYPE, D3D_INCLUDE_TYPE, D3D_NAME, D3D_REGISTER_COMPONENT_TYPE,
    D3D_RESOURCE_RETURN_TYPE, D3D_SHADER_CBUFFER_FLAGS, D3D_SHADER_INPUT_FLAGS,
    D3D_SHADER_INPUT_TYPE, D3D_SHADER_MACRO, D3D_SHADER_VARIABLE_CLASS, D3D_SHADER_VARIABLE_FLAGS,
    D3D_SHADER_VARIABLE_TYPE, ID3DInclude,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR};
pub const D3D10_SHADER_DEBUG: UINT = 1 << 0;
pub const D3D10_SHADER_SKIP_VALIDATION: UINT = 1 << 1;
pub const D3D10_SHADER_SKIP_OPTIMIZATION: UINT = 1 << 2;
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: UINT = 1 << 3;
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: UINT = 1 << 4;
pub const D3D10_SHADER_PARTIAL_PRECISION: UINT = 1 << 5;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: UINT = 1 << 6;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: UINT = 1 << 7;
pub const D3D10_SHADER_NO_PRESHADER: UINT = 1 << 8;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: UINT = 1 << 9;
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: UINT = 1 << 10;
pub const D3D10_SHADER_ENABLE_STRICTNESS: UINT = 1 << 11;
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: UINT = 1 << 12;
pub const D3D10_SHADER_IEEE_STRICTNESS: UINT = 1 << 13;
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: UINT = 1 << 18;
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: UINT = 1 << 19;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: UINT = 1 << 20;
pub const D3D10_ALL_RESOURCES_BOUND: UINT = 1 << 21;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: UINT = 1 << 14;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: UINT = 0;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL2: UINT = (1 << 14) | (1 << 15);
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: UINT = 1 << 15;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: UINT = 0;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: UINT = 1 << 4;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: UINT = 1 << 5;
pub type D3D10_SHADER_MACRO = D3D_SHADER_MACRO;
pub type LPD3D10_SHADER_MACRO = *mut D3D10_SHADER_MACRO;
pub type D3D10_SHADER_VARIABLE_CLASS = D3D_SHADER_VARIABLE_CLASS;
pub type LPD3D10_SHADER_VARIABLE_CLASS = *mut D3D10_SHADER_VARIABLE_CLASS;
pub type D3D10_SHADER_VARIABLE_FLAGS = D3D_SHADER_VARIABLE_FLAGS;
pub type LPD3D10_SHADER_VARIABLE_FLAGS = *mut D3D10_SHADER_VARIABLE_FLAGS;
pub type D3D10_SHADER_VARIABLE_TYPE = D3D_SHADER_VARIABLE_TYPE;
pub type LPD3D10_SHADER_VARIABLE_TYPE = *mut D3D10_SHADER_VARIABLE_TYPE;
pub type D3D10_SHADER_INPUT_FLAGS = D3D_SHADER_INPUT_FLAGS;
pub type LPD3D10_SHADER_INPUT_FLAGS = *mut D3D10_SHADER_INPUT_FLAGS;
pub type D3D10_SHADER_INPUT_TYPE = D3D_SHADER_INPUT_TYPE;
pub type LPD3D10_SHADER_INPUT_TYPE = *mut D3D10_SHADER_INPUT_TYPE;
pub type D3D10_SHADER_CBUFFER_FLAGS = D3D_SHADER_CBUFFER_FLAGS;
pub type LPD3D10_SHADER_CBUFFER_FLAGS = *mut D3D10_SHADER_CBUFFER_FLAGS;
pub type D3D10_CBUFFER_TYPE = D3D_CBUFFER_TYPE;
pub type LPD3D10_CBUFFER_TYPE = *mut D3D10_CBUFFER_TYPE;
pub type D3D10_NAME = D3D_NAME;
pub type D3D10_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE;
pub type D3D10_REGISTER_COMPONENT_TYPE = D3D_REGISTER_COMPONENT_TYPE;
pub type D3D10_INCLUDE_TYPE = D3D_INCLUDE_TYPE;
pub type ID3D10Include = ID3DInclude;
pub type LPD3D10INCLUDE = *mut ID3DInclude;
// const IID_ID3D10Include: IID = IID_ID3DInclude;
STRUCT!{struct D3D10_SHADER_DESC {
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
    GSOutputTopology: D3D10_PRIMITIVE_TOPOLOGY,
    GSMaxOutputVertexCount: UINT,
}}
STRUCT!{struct D3D10_SHADER_BUFFER_DESC {
    Name: LPCSTR,
    Type: D3D10_CBUFFER_TYPE,
    Variables: UINT,
    Size: UINT,
    uFlags: UINT,
}}
STRUCT!{struct D3D10_SHADER_VARIABLE_DESC {
    Name: LPCSTR,
    StartOffset: UINT,
    Size: UINT,
    uFlags: UINT,
    DefaultValue: LPVOID,
}}
STRUCT!{struct D3D10_SHADER_TYPE_DESC {
    Class: D3D10_SHADER_VARIABLE_CLASS,
    Type: D3D10_SHADER_VARIABLE_TYPE,
    Rows: UINT,
    Columns: UINT,
    Elements: UINT,
    Members: UINT,
    Offset: UINT,
}}
STRUCT!{struct D3D10_SHADER_INPUT_BIND_DESC {
    Name: LPCSTR,
    Type: D3D10_SHADER_INPUT_TYPE,
    BindPoint: UINT,
    BindCount: UINT,
    uFlags: UINT,
    ReturnType: D3D10_RESOURCE_RETURN_TYPE,
    Dimension: D3D10_SRV_DIMENSION,
    NumSamples: UINT,
}}
STRUCT!{struct D3D10_SIGNATURE_PARAMETER_DESC {
    SemanticName: LPCSTR,
    SemanticIndex: UINT,
    Register: UINT,
    SystemValueType: D3D10_NAME,
    ComponentType: D3D10_REGISTER_COMPONENT_TYPE,
    Mask: BYTE,
    ReadWriteMask: BYTE,
}}
pub type LPD3D10SHADERREFLECTIONTYPE = *mut ID3D10ShaderReflectionType;
DEFINE_GUID!{IID_ID3D10ShaderReflectionType,
    0xc530ad7d, 0x9b16, 0x4395, 0xa9, 0x79, 0xba, 0x2e, 0xcf, 0xf8, 0x3a, 0xdd}
RIDL!{#[uuid(0xc530ad7d, 0x9b16, 0x4395, 0xa9, 0x79, 0xba, 0x2e, 0xcf, 0xf8, 0x3a, 0xdd)]
interface ID3D10ShaderReflectionType(ID3D10ShaderReflectionTypeVtbl) {
    fn GetDesc(
        pDesc: *mut D3D10_SHADER_TYPE_DESC,
    ) -> HRESULT,
    fn GetMemberTypeByIndex(
        Index: UINT,
    ) -> *mut ID3D10ShaderReflectionType,
    fn GetMemberTypeByName(
        Name: LPCSTR,
    ) -> *mut ID3D10ShaderReflectionType,
    fn GetMemberTypeName(
        Index: UINT,
    ) -> LPCSTR,
}}
pub type LPD3D10SHADERREFLECTIONVARIABLE = *mut ID3D10ShaderReflectionVariable;
DEFINE_GUID!{IID_ID3D10ShaderReflectionVariable,
    0x1bf63c95, 0x2650, 0x405d, 0x99, 0xc1, 0x36, 0x36, 0xbd, 0x1d, 0xa0, 0xa1}
RIDL!{#[uuid(0x1bf63c95, 0x2650, 0x405d, 0x99, 0xc1, 0x36, 0x36, 0xbd, 0x1d, 0xa0, 0xa1)]
interface ID3D10ShaderReflectionVariable(ID3D10ShaderReflectionVariableVtbl) {
    fn GetDesc(
        pDesc: *mut D3D10_SHADER_VARIABLE_DESC,
    ) -> HRESULT,
    fn GetType() -> *mut ID3D10ShaderReflectionType,
}}
pub type LPD3D10SHADERREFLECTIONCONSTANTBUFFER = *mut ID3D10ShaderReflectionConstantBuffer;
DEFINE_GUID!{IID_ID3D10ShaderReflectionConstantBuffer,
    0x66c66a94, 0xdddd, 0x4b62, 0xa6, 0x6a, 0xf0, 0xda, 0x33, 0xc2, 0xb4, 0xd0}
RIDL!{#[uuid(0x66c66a94, 0xdddd, 0x4b62, 0xa6, 0x6a, 0xf0, 0xda, 0x33, 0xc2, 0xb4, 0xd0)]
interface ID3D10ShaderReflectionConstantBuffer(ID3D10ShaderReflectionConstantBufferVtbl) {
    fn GetDesc(
        pDesc: *mut D3D10_SHADER_BUFFER_DESC,
    ) -> HRESULT,
    fn GetVariableByIndex(
        Index: UINT,
    ) -> *mut ID3D10ShaderReflectionVariable,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D10ShaderReflectionVariable,
}}
pub type LPD3D10SHADERREFLECTION = *mut ID3D10ShaderReflection;
DEFINE_GUID!{IID_ID3D10ShaderReflection,
    0xd40e20b6, 0xf8f7, 0x42ad, 0xab, 0x20, 0x4b, 0xaf, 0x8f, 0x15, 0xdf, 0xaa}
RIDL!{#[uuid(0xd40e20b6, 0xf8f7, 0x42ad, 0xab, 0x20, 0x4b, 0xaf, 0x8f, 0x15, 0xdf, 0xaa)]
interface ID3D10ShaderReflection(ID3D10ShaderReflectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetDesc(
        pDesc: *mut D3D10_SHADER_DESC,
    ) -> HRESULT,
    fn GetConstantBufferByIndex(
        Index: UINT,
    ) -> *mut ID3D10ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D10ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D10_SHADER_INPUT_BIND_DESC,
    ) -> HRESULT,
    fn GetInputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
    fn GetOutputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> HRESULT,
}}
// TODO Some functions
