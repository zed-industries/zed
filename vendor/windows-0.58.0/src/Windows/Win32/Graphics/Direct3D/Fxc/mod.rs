#[inline]
pub unsafe fn D3DCompile<P0, P1, P2, P3>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: Option<*mut Option<super::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::ID3DInclude>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCompile(psrcdata : *const core::ffi::c_void, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DCompile(psrcdata, srcdatasize, psourcename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3DCompile2<P0, P1, P2, P3>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, secondarydataflags: u32, psecondarydata: Option<*const core::ffi::c_void>, secondarydatasize: usize, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: Option<*mut Option<super::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::ID3DInclude>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCompile2(psrcdata : *const core::ffi::c_void, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, secondarydataflags : u32, psecondarydata : *const core::ffi::c_void, secondarydatasize : usize, ppcode : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DCompile2(psrcdata, srcdatasize, psourcename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, secondarydataflags, core::mem::transmute(psecondarydata.unwrap_or(std::ptr::null())), secondarydatasize, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3DCompileFromFile<P0, P1, P2, P3>(pfilename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: Option<*mut Option<super::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::ID3DInclude>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCompileFromFile(pfilename : windows_core::PCWSTR, pdefines : *const super:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DCompileFromFile(pfilename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3DCompressShaders(pshaderdata: &[D3D_SHADER_DATA], uflags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCompressShaders(unumshaders : u32, pshaderdata : *const D3D_SHADER_DATA, uflags : u32, ppcompresseddata : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DCompressShaders(pshaderdata.len().try_into().unwrap(), core::mem::transmute(pshaderdata.as_ptr()), uflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DCreateBlob(size: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCreateBlob(size : usize, ppblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DCreateBlob(size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateFunctionLinkingGraph(uflags: u32) -> windows_core::Result<super::super::Direct3D11::ID3D11FunctionLinkingGraph> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCreateFunctionLinkingGraph(uflags : u32, ppfunctionlinkinggraph : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DCreateFunctionLinkingGraph(uflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateLinker() -> windows_core::Result<super::super::Direct3D11::ID3D11Linker> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DCreateLinker(pplinker : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DCreateLinker(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DDecompressShaders(psrcdata: *const core::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: Option<*const u32>, uflags: u32, ppshaders: *mut Option<super::ID3DBlob>, ptotalshaders: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DDecompressShaders(psrcdata : *const core::ffi::c_void, srcdatasize : usize, unumshaders : u32, ustartindex : u32, pindices : *const u32, uflags : u32, ppshaders : *mut * mut core::ffi::c_void, ptotalshaders : *mut u32) -> windows_core::HRESULT);
    D3DDecompressShaders(psrcdata, srcdatasize, unumshaders, ustartindex, core::mem::transmute(pindices.unwrap_or(std::ptr::null())), uflags, core::mem::transmute(ppshaders), core::mem::transmute(ptotalshaders.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3DDisassemble<P0>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: P0) -> windows_core::Result<super::ID3DBlob>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble(psrcdata : *const core::ffi::c_void, srcdatasize : usize, flags : u32, szcomments : windows_core::PCSTR, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DDisassemble(psrcdata, srcdatasize, flags, szcomments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D10")]
#[inline]
pub unsafe fn D3DDisassemble10Effect<P0>(peffect: P0, flags: u32) -> windows_core::Result<super::ID3DBlob>
where
    P0: windows_core::Param<super::super::Direct3D10::ID3D10Effect>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble10Effect(peffect : * mut core::ffi::c_void, flags : u32, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DDisassemble10Effect(peffect.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DDisassembleRegion<P0>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: P0, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: Option<*mut usize>, ppdisassembly: *mut Option<super::ID3DBlob>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DDisassembleRegion(psrcdata : *const core::ffi::c_void, srcdatasize : usize, flags : u32, szcomments : windows_core::PCSTR, startbyteoffset : usize, numinsts : usize, pfinishbyteoffset : *mut usize, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DDisassembleRegion(psrcdata, srcdatasize, flags, szcomments.param().abi(), startbyteoffset, numinsts, core::mem::transmute(pfinishbyteoffset.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdisassembly)).ok()
}
#[inline]
pub unsafe fn D3DGetBlobPart(psrcdata: *const core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetBlobPart(psrcdata : *const core::ffi::c_void, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, pppart : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DGetBlobPart(psrcdata, srcdatasize, part, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DGetDebugInfo(psrcdata: *const core::ffi::c_void, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetDebugInfo(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ppdebuginfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DGetDebugInfo(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const core::ffi::c_void, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetInputAndOutputSignatureBlob(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DGetInputAndOutputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DGetInputSignatureBlob(psrcdata: *const core::ffi::c_void, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetInputSignatureBlob(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DGetInputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DGetOutputSignatureBlob(psrcdata: *const core::ffi::c_void, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetOutputSignatureBlob(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DGetOutputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DGetTraceInstructionOffsets(psrcdata: *const core::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, poffsets: Option<&mut [usize]>, ptotalinsts: Option<*mut usize>) -> windows_core::Result<()> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DGetTraceInstructionOffsets(psrcdata : *const core::ffi::c_void, srcdatasize : usize, flags : u32, startinstindex : usize, numinsts : usize, poffsets : *mut usize, ptotalinsts : *mut usize) -> windows_core::HRESULT);
    D3DGetTraceInstructionOffsets(psrcdata, srcdatasize, flags, startinstindex, poffsets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(poffsets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ptotalinsts.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DLoadModule(psrcdata: *const core::ffi::c_void, cbsrcdatasize: usize) -> windows_core::Result<super::super::Direct3D11::ID3D11Module> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DLoadModule(psrcdata : *const core::ffi::c_void, cbsrcdatasize : usize, ppmodule : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DLoadModule(psrcdata, cbsrcdatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DPreprocess<P0, P1>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P1, ppcodetext: *mut Option<super::ID3DBlob>, pperrormsgs: Option<*mut Option<super::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::ID3DInclude>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DPreprocess(psrcdata : *const core::ffi::c_void, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, ppcodetext : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DPreprocess(psrcdata, srcdatasize, psourcename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), core::mem::transmute(ppcodetext), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3DReadFileToBlob<P0>(pfilename: P0) -> windows_core::Result<super::ID3DBlob>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DReadFileToBlob(pfilename : windows_core::PCWSTR, ppcontents : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DReadFileToBlob(pfilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DReflect(psrcdata: *const core::ffi::c_void, srcdatasize: usize, pinterface: *const windows_core::GUID, ppreflector: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DReflect(psrcdata : *const core::ffi::c_void, srcdatasize : usize, pinterface : *const windows_core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DReflect(psrcdata, srcdatasize, pinterface, ppreflector).ok()
}
#[inline]
pub unsafe fn D3DReflectLibrary(psrcdata: *const core::ffi::c_void, srcdatasize: usize, riid: *const windows_core::GUID, ppreflector: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DReflectLibrary(psrcdata : *const core::ffi::c_void, srcdatasize : usize, riid : *const windows_core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DReflectLibrary(psrcdata, srcdatasize, riid, ppreflector).ok()
}
#[inline]
pub unsafe fn D3DSetBlobPart(psrcdata: *const core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const core::ffi::c_void, partsize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DSetBlobPart(psrcdata : *const core::ffi::c_void, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, ppart : *const core::ffi::c_void, partsize : usize, ppnewshader : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DSetBlobPart(psrcdata, srcdatasize, part, flags, ppart, partsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DStripShader(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ustripflags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DStripShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ustripflags : u32, ppstrippedblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DStripShader(pshaderbytecode, bytecodelength, ustripflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DWriteBlobToFile<P0, P1, P2>(pblob: P0, pfilename: P1, boverwrite: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::ID3DBlob>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DWriteBlobToFile(pblob : * mut core::ffi::c_void, pfilename : windows_core::PCWSTR, boverwrite : super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    D3DWriteBlobToFile(pblob.param().abi(), pfilename.param().abi(), boverwrite.param().abi()).ok()
}
pub const D3DCOMPILER_DLL_A: windows_core::PCSTR = windows_core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_W: windows_core::PCWSTR = windows_core::w!("d3dcompiler_47.dll");
pub const D3DCOMPILER_STRIP_DEBUG_INFO: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(2i32);
pub const D3DCOMPILER_STRIP_PRIVATE_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(8i32);
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(1i32);
pub const D3DCOMPILER_STRIP_ROOT_SIGNATURE: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(16i32);
pub const D3DCOMPILER_STRIP_TEST_BLOBS: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(4i32);
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 2097152u32;
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 512u32;
pub const D3DCOMPILE_DEBUG: u32 = 1u32;
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: u32 = 1u32;
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 8192u32;
pub const D3DCOMPILE_NO_PRESHADER: u32 = 256u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 16384u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 32768u32;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 32u32;
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1024u32;
pub const D3DCOMPILE_RESERVED16: u32 = 65536u32;
pub const D3DCOMPILE_RESERVED17: u32 = 131072u32;
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 524288u32;
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: u32 = 1u32;
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: u32 = 2u32;
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: u32 = 4u32;
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 2u32;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(4i32);
pub const D3D_BLOB_DEBUG_INFO: D3D_BLOB_PART = D3D_BLOB_PART(5i32);
pub const D3D_BLOB_DEBUG_NAME: D3D_BLOB_PART = D3D_BLOB_PART(12i32);
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(2i32);
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(0i32);
pub const D3D_BLOB_LEGACY_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(6i32);
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(1i32);
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(3i32);
pub const D3D_BLOB_PDB: D3D_BLOB_PART = D3D_BLOB_PART(9i32);
pub const D3D_BLOB_PRIVATE_DATA: D3D_BLOB_PART = D3D_BLOB_PART(10i32);
pub const D3D_BLOB_ROOT_SIGNATURE: D3D_BLOB_PART = D3D_BLOB_PART(11i32);
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(32768i32);
pub const D3D_BLOB_TEST_COMPILE_DETAILS: D3D_BLOB_PART = D3D_BLOB_PART(32769i32);
pub const D3D_BLOB_TEST_COMPILE_PERF: D3D_BLOB_PART = D3D_BLOB_PART(32770i32);
pub const D3D_BLOB_TEST_COMPILE_REPORT: D3D_BLOB_PART = D3D_BLOB_PART(32771i32);
pub const D3D_BLOB_XNA_PREPASS_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(7i32);
pub const D3D_BLOB_XNA_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(8i32);
pub const D3D_COMPILER_VERSION: u32 = 47u32;
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: u32 = 1u32;
pub const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 16u32;
pub const D3D_DISASM_ENABLE_COLOR_CODE: u32 = 1u32;
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 2u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: u32 = 8u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 4u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: u32 = 32u32;
pub const D3D_DISASM_INSTRUCTION_ONLY: u32 = 64u32;
pub const D3D_DISASM_PRINT_HEX_LITERALS: u32 = 128u32;
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DCOMPILER_STRIP_FLAGS(pub i32);
impl windows_core::TypeKind for D3DCOMPILER_STRIP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DCOMPILER_STRIP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DCOMPILER_STRIP_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D_BLOB_PART(pub i32);
impl windows_core::TypeKind for D3D_BLOB_PART {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D_BLOB_PART {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D_BLOB_PART").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D_SHADER_DATA {
    pub pBytecode: *const core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl windows_core::TypeKind for D3D_SHADER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D_SHADER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type pD3DCompile = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: Option<super::ID3DInclude>, pentrypoint: windows_core::PCSTR, ptarget: windows_core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
pub type pD3DDisassemble = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: windows_core::PCSTR, ppdisassembly: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
pub type pD3DPreprocess = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: Option<super::ID3DInclude>, ppcodetext: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
