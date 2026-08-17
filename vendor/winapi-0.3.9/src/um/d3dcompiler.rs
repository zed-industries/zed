// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::SIZE_T;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, LPCVOID, LPVOID, UINT};
use um::d3d11shader::{ID3D11FunctionLinkingGraph, ID3D11Linker, ID3D11Module};
use um::d3dcommon::{D3D_SHADER_MACRO, ID3DBlob, ID3DInclude};
use um::winnt::{HRESULT, LPCSTR, LPCWSTR};
pub const D3DCOMPILER_DLL: &'static str = "d3dcompiler_47.dll";
pub const D3D_COMPILER_VERSION: DWORD = 47;
extern "system" {
    pub fn D3DReadFileToBlob(
        pFileName: LPCWSTR,
        ppContents: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DWriteBlobToFile(
        pBlob: *mut ID3DBlob,
        pFileName: LPCWSTR,
        bOverwrite: BOOL,
    ) -> HRESULT;
}
pub const D3DCOMPILE_DEBUG: DWORD = 1 << 0;
pub const D3DCOMPILE_SKIP_VALIDATION: DWORD = 1 << 1;
pub const D3DCOMPILE_SKIP_OPTIMIZATION: DWORD = 1 << 2;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: DWORD = 1 << 3;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: DWORD = 1 << 4;
pub const D3DCOMPILE_PARTIAL_PRECISION: DWORD = 1 << 5;
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: DWORD = 1 << 6;
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: DWORD = 1 << 7;
pub const D3DCOMPILE_NO_PRESHADER: DWORD = 1 << 8;
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: DWORD = 1 << 9;
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: DWORD = 1 << 10;
pub const D3DCOMPILE_ENABLE_STRICTNESS: DWORD = 1 << 11;
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: DWORD = 1 << 12;
pub const D3DCOMPILE_IEEE_STRICTNESS: DWORD = 1 << 13;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: DWORD = 1 << 14;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: DWORD = 0;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: DWORD = (1 << 14) | (1 << 15);
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: DWORD = 1 << 15;
pub const D3DCOMPILE_RESERVED16: DWORD = 1 << 16;
pub const D3DCOMPILE_RESERVED17: DWORD = 1 << 17;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: DWORD = 1 << 18;
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: DWORD = 1 << 19;
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: DWORD = 1 << 20;
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: DWORD = 1 << 21;
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: DWORD = 1 << 0;
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: DWORD = 1 << 1;
pub const D3D_COMPILE_STANDARD_FILE_INCLUDE: *mut ID3DInclude = 1 as *mut ID3DInclude;
extern "system" {
    pub fn D3DCompile(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut ID3DInclude,
        pEntrypoint: LPCSTR,
        pTarget: LPCSTR,
        Flags1: UINT,
        Flags2: UINT,
        ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: DWORD = 0x00000001;
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: DWORD = 0x00000002;
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: DWORD = 0x00000004;
extern "system" {
    pub fn D3DCompile2(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut ID3DInclude,
        pEntrypoint: LPCSTR,
        pTarget: LPCSTR,
        Flags1: UINT,
        Flags2: UINT,
        SecondaryDataFlags: UINT,
        pSecondaryData: LPCVOID,
        SecondaryDataSize: SIZE_T,
        ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCompileFromFile(
        pFileName: LPCWSTR,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut ID3DInclude,
        pEntrypoint: LPCSTR,
        pTarget: LPCSTR,
        Flags1: UINT,
        Flags2: UINT,
        ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DPreprocess(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut ID3DInclude,
        ppCodeText: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetDebugInfo(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        ppDebugInfo: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DReflect(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        pInterface: REFIID,
        ppReflector: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3DReflectLibrary(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        riid: REFIID,
        ppReflector: *mut LPVOID,
    ) -> HRESULT;
}
pub const D3D_DISASM_ENABLE_COLOR_CODE: DWORD = 0x00000001;
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: DWORD = 0x00000002;
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: DWORD = 0x00000004;
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: DWORD = 0x00000008;
pub const D3D_DISASM_DISABLE_DEBUG_INFO: DWORD = 0x00000010;
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: DWORD = 0x00000020;
pub const D3D_DISASM_INSTRUCTION_ONLY: DWORD = 0x00000040;
pub const D3D_DISASM_PRINT_HEX_LITERALS: DWORD = 0x00000080;
extern "system" {
    pub fn D3DDisassemble(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        Flags: UINT,
        szComments: LPCSTR,
        ppDisassembly: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DDisassembleRegion(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        Flags: UINT,
        szComments: LPCSTR,
        StartByteOffset: SIZE_T,
        NumInsts: SIZE_T,
        pFinishByteOffset: *mut SIZE_T,
        ppDisassembly: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCreateLinker(
        ppLinker: *mut *mut ID3D11Linker,
    ) -> HRESULT;
    pub fn D3DLoadModule(
        pSrcData: LPCVOID,
        cbSrcDataSize: SIZE_T,
        ppModule: *mut *mut ID3D11Module,
    ) -> HRESULT;
    pub fn D3DCreateFunctionLinkingGraph(
        uFlags: UINT,
        ppFunctionLinkingGraph: *mut *mut ID3D11FunctionLinkingGraph,
    ) -> HRESULT;
}
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: DWORD = 0x00000001;
extern "system" {
    pub fn D3DGetTraceInstructionOffsets(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        Flags: UINT,
        StartInstIndex: SIZE_T,
        NumInsts: SIZE_T,
        pOffsets: *mut SIZE_T,
        pTotalInsts: *mut SIZE_T,
    ) -> HRESULT;
    pub fn D3DGetInputSignatureBlob(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetOutputSignatureBlob(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetInputAndOutputSignatureBlob(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
ENUM!{enum D3DCOMPILER_STRIP_FLAGS {
    D3DCOMPILER_STRIP_REFLECTION_DATA = 0x00000001,
    D3DCOMPILER_STRIP_DEBUG_INFO = 0x00000002,
    D3DCOMPILER_STRIP_TEST_BLOBS = 0x00000004,
    D3DCOMPILER_STRIP_PRIVATE_DATA = 0x00000008,
    D3DCOMPILER_STRIP_ROOT_SIGNATURE = 0x00000010,
    D3DCOMPILER_STRIP_FORCE_DWORD = 0x7fffffff,
}}
extern "system" {
    pub fn D3DStripShader(
        pShaderBytecode: LPCVOID,
        BytecodeLength: SIZE_T,
        uStripFlags: UINT,
        ppStrippedBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
ENUM!{enum D3D_BLOB_PART {
    D3D_BLOB_INPUT_SIGNATURE_BLOB,
    D3D_BLOB_OUTPUT_SIGNATURE_BLOB,
    D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
    D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB,
    D3D_BLOB_ALL_SIGNATURE_BLOB,
    D3D_BLOB_DEBUG_INFO,
    D3D_BLOB_LEGACY_SHADER,
    D3D_BLOB_XNA_PREPASS_SHADER,
    D3D_BLOB_XNA_SHADER,
    D3D_BLOB_PDB,
    D3D_BLOB_PRIVATE_DATA,
    D3D_BLOB_ROOT_SIGNATURE,
    D3D_BLOB_TEST_ALTERNATE_SHADER = 0x8000,
    D3D_BLOB_TEST_COMPILE_DETAILS,
    D3D_BLOB_TEST_COMPILE_PERF,
    D3D_BLOB_TEST_COMPILE_REPORT,
}}
extern "system" {
    pub fn D3DGetBlobPart(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        Part: D3D_BLOB_PART,
        Flags: UINT,
        ppPart: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DSetBlobPart(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        Part: D3D_BLOB_PART,
        Flags: UINT,
        pPart: LPCVOID,
        PartSize: SIZE_T,
        ppNewShader: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCreateBlob(
        Size: SIZE_T,
        ppBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
STRUCT!{struct D3D_SHADER_DATA {
    pBytecode: LPCVOID,
    BytecodeLength: SIZE_T,
}}
extern "system" {
    pub fn D3DCompressShaders(
        uNumShaders: UINT,
        pShaderData: *mut D3D_SHADER_DATA,
        uFlags: UINT,
        ppCompressedData: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DDecompressShaders(
        pSrcData: LPCVOID,
        SrcDataSize: SIZE_T,
        uNumShaders: UINT,
        uStartIndex: UINT,
        pIndices: *mut UINT,
        uFlags: UINT,
        ppShaders: *mut *mut ID3DBlob,
        pTotalShaders: *mut UINT,
    ) -> HRESULT;
    // pub fn D3DDisassemble10Effect(
    //     pEffect: *mut ID3D10Effect,
    //     Flags: UINT,
    //     ppDisassembly: *mut *mut ID3DBlob,
    // ) -> HRESULT;
}
