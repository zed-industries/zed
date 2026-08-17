#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileEffectFromMemory<P2, P4>(pdata: *const core::ffi::c_void, datalength: usize, psrcfilename: P2, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P4, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut Option<super::Direct3D::ID3DBlob>, pperrors: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::Direct3D::ID3DInclude>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CompileEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, psrcfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, hlslflags : u32, fxflags : u32, ppcompiledeffect : *mut * mut core::ffi::c_void, pperrors : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CompileEffectFromMemory(pdata, datalength, psrcfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), hlslflags, fxflags, core::mem::transmute(ppcompiledeffect), pperrors.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileShader<P2, P4, P5, P6>(psrcdata: &[u8], pfilename: P2, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P4, pfunctionname: P5, pprofile: P6, flags: u32, ppshader: *mut Option<super::Direct3D::ID3DBlob>, pperrormsgs: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::Direct3D::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CompileShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, pfunctionname : windows_core::PCSTR, pprofile : windows_core::PCSTR, flags : u32, ppshader : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CompileShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pfunctionname.param().abi(), pprofile.param().abi(), flags, core::mem::transmute(ppshader), pperrormsgs.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CreateBlob(numbytes: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateBlob(numbytes : usize, ppbuffer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateBlob(numbytes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D3D10CreateDevice<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HMODULE, flags: u32, sdkversion: u32, ppdevice: Option<*mut Option<ID3D10Device>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateDevice(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDevice(padapter.param().abi(), drivertype, software, flags, sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D3D10CreateDevice1<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HMODULE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10_1.dll" "system" fn D3D10CreateDevice1(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDevice1(padapter.param().abi(), drivertype, software, flags, hardwarelevel, sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HMODULE, flags: u32, sdkversion: u32, pswapchaindesc: Option<*const super::Dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::Dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D10Device>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateDeviceAndSwapChain(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, sdkversion : u32, pswapchaindesc : *const super::Dxgi:: DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut * mut core::ffi::c_void, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDeviceAndSwapChain(padapter.param().abi(), drivertype, software, flags, sdkversion, pswapchaindesc.unwrap_or(core::mem::zeroed()) as _, ppswapchain.unwrap_or(core::mem::zeroed()) as _, ppdevice.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain1<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HMODULE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: Option<*const super::Dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::Dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10_1.dll" "system" fn D3D10CreateDeviceAndSwapChain1(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, pswapchaindesc : *const super::Dxgi:: DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut * mut core::ffi::c_void, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDeviceAndSwapChain1(padapter.param().abi(), drivertype, software, flags, hardwarelevel, sdkversion, pswapchaindesc.unwrap_or(core::mem::zeroed()) as _, ppswapchain.unwrap_or(core::mem::zeroed()) as _, ppdevice.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn D3D10CreateEffectFromMemory<P3, P4>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P3, peffectpool: P4) -> windows_core::Result<ID3D10Effect>
where
    P3: windows_core::Param<ID3D10Device>,
    P4: windows_core::Param<ID3D10EffectPool>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : * mut core::ffi::c_void, peffectpool : * mut core::ffi::c_void, ppeffect : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateEffectFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), peffectpool.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10CreateEffectPoolFromMemory<P3>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P3) -> windows_core::Result<ID3D10EffectPool>
where
    P3: windows_core::Param<ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateEffectPoolFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : * mut core::ffi::c_void, ppeffectpool : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateEffectPoolFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10CreateStateBlock<P0>(pdevice: P0, pstateblockmask: *const D3D10_STATE_BLOCK_MASK) -> windows_core::Result<ID3D10StateBlock>
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateStateBlock(pdevice : * mut core::ffi::c_void, pstateblockmask : *const D3D10_STATE_BLOCK_MASK, ppstateblock : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateStateBlock(pdevice.param().abi(), pstateblockmask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10DisassembleEffect<P0>(peffect: P0, enablecolorcode: bool) -> windows_core::Result<super::Direct3D::ID3DBlob>
where
    P0: windows_core::Param<ID3D10Effect>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10DisassembleEffect(peffect : * mut core::ffi::c_void, enablecolorcode : windows_core::BOOL, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10DisassembleEffect(peffect.param().abi(), enablecolorcode.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10DisassembleShader<P3>(pshader: *const core::ffi::c_void, bytecodelength: usize, enablecolorcode: bool, pcomments: P3) -> windows_core::Result<super::Direct3D::ID3DBlob>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10DisassembleShader(pshader : *const core::ffi::c_void, bytecodelength : usize, enablecolorcode : windows_core::BOOL, pcomments : windows_core::PCSTR, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10DisassembleShader(pshader, bytecodelength, enablecolorcode.into(), pcomments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10GetGeometryShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetGeometryShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetGeometryShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetInputAndOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetInputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetInputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10GetPixelShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetPixelShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetPixelShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetShaderDebugInfo(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetShaderDebugInfo(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppdebuginfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetShaderDebugInfo(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10GetVertexShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetVertexShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetVertexShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10PreprocessShader<P2, P4>(psrcdata: &[u8], pfilename: P2, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P4, ppshadertext: *mut Option<super::Direct3D::ID3DBlob>, pperrormsgs: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::Direct3D::ID3DInclude>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10PreprocessShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, ppshadertext : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10PreprocessShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), core::mem::transmute(ppshadertext), pperrormsgs.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn D3D10ReflectShader(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<ID3D10ShaderReflection> {
    windows_core::link!("d3d10.dll" "system" fn D3D10ReflectShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppreflector : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10ReflectShader(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDifference(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDifference(pa, pb, presult as _).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDisableAll(pmask as _).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDisableCapture(pmask as _, statetype, rangestart, rangelength).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskEnableAll(pmask as _).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskEnableCapture(pmask as _, statetype, rangestart, rangelength).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> windows_core::BOOL {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskGetSetting(pmask : *const D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, entry : u32) -> windows_core::BOOL);
    unsafe { D3D10StateBlockMaskGetSetting(pmask, statetype, entry) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskIntersect(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskIntersect(pa, pb, presult as _).ok() }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskUnion(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskUnion(pa, pb, presult as _).ok() }
}
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1u32;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152u32;
pub const D3D10_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D10_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D10_APPNAME_STRING: windows_core::PCWSTR = windows_core::w!("Name");
pub const D3D10_APPSIZE_STRING: windows_core::PCWSTR = windows_core::w!("Size");
pub const D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub const D3D10_ASYNC_GETDATA_DONOTFLUSH: D3D10_ASYNC_GETDATA_FLAG = D3D10_ASYNC_GETDATA_FLAG(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_ASYNC_GETDATA_FLAG(pub i32);
pub const D3D10_BIND_CONSTANT_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(4i32);
pub const D3D10_BIND_DEPTH_STENCIL: D3D10_BIND_FLAG = D3D10_BIND_FLAG(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_BIND_FLAG(pub i32);
pub const D3D10_BIND_INDEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(2i32);
pub const D3D10_BIND_RENDER_TARGET: D3D10_BIND_FLAG = D3D10_BIND_FLAG(32i32);
pub const D3D10_BIND_SHADER_RESOURCE: D3D10_BIND_FLAG = D3D10_BIND_FLAG(8i32);
pub const D3D10_BIND_STREAM_OUTPUT: D3D10_BIND_FLAG = D3D10_BIND_FLAG(16i32);
pub const D3D10_BIND_VERTEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_BLEND(pub i32);
pub const D3D10_BLEND_BLEND_FACTOR: D3D10_BLEND = D3D10_BLEND(14i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub BlendEnable: [windows_core::BOOL; 8],
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: [u8; 8],
}
impl Default for D3D10_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
impl Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_BLEND_DEST_ALPHA: D3D10_BLEND = D3D10_BLEND(7i32);
pub const D3D10_BLEND_DEST_COLOR: D3D10_BLEND = D3D10_BLEND(9i32);
pub const D3D10_BLEND_INV_BLEND_FACTOR: D3D10_BLEND = D3D10_BLEND(15i32);
pub const D3D10_BLEND_INV_DEST_ALPHA: D3D10_BLEND = D3D10_BLEND(8i32);
pub const D3D10_BLEND_INV_DEST_COLOR: D3D10_BLEND = D3D10_BLEND(10i32);
pub const D3D10_BLEND_INV_SRC1_ALPHA: D3D10_BLEND = D3D10_BLEND(19i32);
pub const D3D10_BLEND_INV_SRC1_COLOR: D3D10_BLEND = D3D10_BLEND(17i32);
pub const D3D10_BLEND_INV_SRC_ALPHA: D3D10_BLEND = D3D10_BLEND(6i32);
pub const D3D10_BLEND_INV_SRC_COLOR: D3D10_BLEND = D3D10_BLEND(4i32);
pub const D3D10_BLEND_ONE: D3D10_BLEND = D3D10_BLEND(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_BLEND_OP(pub i32);
pub const D3D10_BLEND_OP_ADD: D3D10_BLEND_OP = D3D10_BLEND_OP(1i32);
pub const D3D10_BLEND_OP_MAX: D3D10_BLEND_OP = D3D10_BLEND_OP(5i32);
pub const D3D10_BLEND_OP_MIN: D3D10_BLEND_OP = D3D10_BLEND_OP(4i32);
pub const D3D10_BLEND_OP_REV_SUBTRACT: D3D10_BLEND_OP = D3D10_BLEND_OP(3i32);
pub const D3D10_BLEND_OP_SUBTRACT: D3D10_BLEND_OP = D3D10_BLEND_OP(2i32);
pub const D3D10_BLEND_SRC1_ALPHA: D3D10_BLEND = D3D10_BLEND(18i32);
pub const D3D10_BLEND_SRC1_COLOR: D3D10_BLEND = D3D10_BLEND(16i32);
pub const D3D10_BLEND_SRC_ALPHA: D3D10_BLEND = D3D10_BLEND(5i32);
pub const D3D10_BLEND_SRC_ALPHA_SAT: D3D10_BLEND = D3D10_BLEND(11i32);
pub const D3D10_BLEND_SRC_COLOR: D3D10_BLEND = D3D10_BLEND(3i32);
pub const D3D10_BLEND_ZERO: D3D10_BLEND = D3D10_BLEND(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
pub const D3D10_BREAKON_CATEGORY: windows_core::PCWSTR = windows_core::w!("BreakOn_CATEGORY_%s");
pub const D3D10_BREAKON_ID_DECIMAL: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%d");
pub const D3D10_BREAKON_ID_STRING: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%s");
pub const D3D10_BREAKON_SEVERITY: windows_core::PCWSTR = windows_core::w!("BreakOn_SEVERITY_%s");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BUFFER_RTV {
    pub Anonymous1: D3D10_BUFFER_RTV_0,
    pub Anonymous2: D3D10_BUFFER_RTV_1,
}
impl Default for D3D10_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D10_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D10_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BUFFER_SRV {
    pub Anonymous1: D3D10_BUFFER_SRV_0,
    pub Anonymous2: D3D10_BUFFER_SRV_1,
}
impl Default for D3D10_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D10_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D10_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-2i32);
pub const D3D10_CLEAR_DEPTH: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_CLEAR_FLAG(pub i32);
pub const D3D10_CLEAR_STENCIL: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(2i32);
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_COLOR_WRITE_ENABLE(pub i32);
pub const D3D10_COLOR_WRITE_ENABLE_ALL: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(15i32);
pub const D3D10_COLOR_WRITE_ENABLE_ALPHA: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(8i32);
pub const D3D10_COLOR_WRITE_ENABLE_BLUE: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(4i32);
pub const D3D10_COLOR_WRITE_ENABLE_GREEN: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(2i32);
pub const D3D10_COLOR_WRITE_ENABLE_RED: D3D10_COLOR_WRITE_ENABLE = D3D10_COLOR_WRITE_ENABLE(1i32);
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D10_COMPARISON_ALWAYS: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(8i32);
pub const D3D10_COMPARISON_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(3i32);
pub const D3D10_COMPARISON_FILTERING_BIT: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_COMPARISON_FUNC(pub i32);
pub const D3D10_COMPARISON_GREATER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(5i32);
pub const D3D10_COMPARISON_GREATER_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(7i32);
pub const D3D10_COMPARISON_LESS: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(2i32);
pub const D3D10_COMPARISON_LESS_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(4i32);
pub const D3D10_COMPARISON_NEVER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(1i32);
pub const D3D10_COMPARISON_NOT_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_COUNTER(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_COUNTER_DESC {
    pub Counter: D3D10_COUNTER,
    pub MiscFlags: u32,
}
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: D3D10_COUNTER = D3D10_COUNTER(1073741824i32);
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(9i32);
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(2i32);
pub const D3D10_COUNTER_GPU_IDLE: D3D10_COUNTER = D3D10_COUNTER(0i32);
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(13i32);
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(12i32);
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D10_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(6i32);
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(4i32);
pub const D3D10_COUNTER_PIXEL_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(3i32);
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(16i32);
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(15i32);
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(14i32);
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(17i32);
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_COUNTER_TYPE(pub i32);
pub const D3D10_COUNTER_TYPE_FLOAT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(0i32);
pub const D3D10_COUNTER_TYPE_UINT16: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(1i32);
pub const D3D10_COUNTER_TYPE_UINT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(2i32);
pub const D3D10_COUNTER_TYPE_UINT64: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(3i32);
pub const D3D10_COUNTER_VERTEX_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(1i32);
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(7i32);
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(11i32);
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_CPU_ACCESS_FLAG(pub i32);
pub const D3D10_CPU_ACCESS_READ: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(131072i32);
pub const D3D10_CPU_ACCESS_WRITE: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(65536i32);
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(16i32);
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(32i32);
pub const D3D10_CREATE_DEVICE_DEBUG: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(2i32);
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1024i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_CREATE_DEVICE_FLAG(pub i32);
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(128i32);
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(8i32);
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1i32);
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(512i32);
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(4i32);
pub const D3D10_CULL_BACK: D3D10_CULL_MODE = D3D10_CULL_MODE(3i32);
pub const D3D10_CULL_FRONT: D3D10_CULL_MODE = D3D10_CULL_MODE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_CULL_MODE(pub i32);
pub const D3D10_CULL_NONE: D3D10_CULL_MODE = D3D10_CULL_MODE(1i32);
pub const D3D10_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
pub const D3D10_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
pub const D3D10_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D10_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D10_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0u32;
pub const D3D10_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D10_DEFAULT_MAX_ANISOTROPY: f32 = 16f32;
pub const D3D10_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_core::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: windows_core::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D10_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D10_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_DSV_DIMENSION,
    pub Anonymous: D3D10_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D10_TEX1D_DSV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D10_TEX2D_DSV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D10_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_DEPTH_WRITE_MASK(pub i32);
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(1i32);
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_DEVICE_STATE_TYPES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_DRIVER_TYPE(pub i32);
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(0i32);
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(2i32);
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(1i32);
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(3i32);
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = D3D10_DRIVER_TYPE(5i32);
pub const D3D10_DST_GS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(9i32);
pub const D3D10_DST_GS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(12i32);
pub const D3D10_DST_GS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(10i32);
pub const D3D10_DST_GS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(11i32);
pub const D3D10_DST_IA_INDEX_BUFFER: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(18i32);
pub const D3D10_DST_IA_INPUT_LAYOUT: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(19i32);
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(20i32);
pub const D3D10_DST_IA_VERTEX_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(17i32);
pub const D3D10_DST_OM_BLEND_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(4i32);
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(3i32);
pub const D3D10_DST_OM_RENDER_TARGETS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(2i32);
pub const D3D10_DST_PREDICATION: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(24i32);
pub const D3D10_DST_PS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(13i32);
pub const D3D10_DST_PS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(16i32);
pub const D3D10_DST_PS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(14i32);
pub const D3D10_DST_PS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(15i32);
pub const D3D10_DST_RS_RASTERIZER_STATE: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(23i32);
pub const D3D10_DST_RS_SCISSOR_RECTS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(22i32);
pub const D3D10_DST_RS_VIEWPORTS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(21i32);
pub const D3D10_DST_SO_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(1i32);
pub const D3D10_DST_VS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(5i32);
pub const D3D10_DST_VS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(8i32);
pub const D3D10_DST_VS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(6i32);
pub const D3D10_DST_VS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = D3D10_DEVICE_STATE_TYPES(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_DSV_DIMENSION(pub i32);
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(1i32);
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(2i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(3i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(4i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(5i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(6i32);
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(0i32);
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: windows_core::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: windows_core::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: windows_core::PCSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
impl Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: windows_core::PCSTR,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Elements: u32,
    pub Members: u32,
    pub Rows: u32,
    pub Columns: u32,
    pub PackedSize: u32,
    pub UnpackedSize: u32,
    pub Stride: u32,
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub Semantic: windows_core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
pub const D3D10_ENABLE_BREAK_ON_MESSAGE: windows_core::PCWSTR = windows_core::w!("EnableBreakOnMessage");
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_FEATURE_LEVEL1(pub i32);
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(40960i32);
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(41216i32);
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37120i32);
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37376i32);
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37632i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_FILL_MODE(pub i32);
pub const D3D10_FILL_SOLID: D3D10_FILL_MODE = D3D10_FILL_MODE(3i32);
pub const D3D10_FILL_WIREFRAME: D3D10_FILL_MODE = D3D10_FILL_MODE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_FILTER(pub i32);
pub const D3D10_FILTER_ANISOTROPIC: D3D10_FILTER = D3D10_FILTER(85i32);
pub const D3D10_FILTER_COMPARISON_ANISOTROPIC: D3D10_FILTER = D3D10_FILTER(213i32);
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(144i32);
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(145i32);
pub const D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(148i32);
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(149i32);
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(128i32);
pub const D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(129i32);
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(132i32);
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(133i32);
pub const D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(16i32);
pub const D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(17i32);
pub const D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(20i32);
pub const D3D10_FILTER_MIN_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(21i32);
pub const D3D10_FILTER_MIN_MAG_MIP_POINT: D3D10_FILTER = D3D10_FILTER(0i32);
pub const D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(1i32);
pub const D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = D3D10_FILTER(4i32);
pub const D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = D3D10_FILTER(5i32);
pub const D3D10_FILTER_TEXT_1BIT: D3D10_FILTER = D3D10_FILTER(-2147483648i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_FILTER_TYPE(pub i32);
pub const D3D10_FILTER_TYPE_LINEAR: D3D10_FILTER_TYPE = D3D10_FILTER_TYPE(1i32);
pub const D3D10_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D10_FILTER_TYPE_POINT: D3D10_FILTER_TYPE = D3D10_FILTER_TYPE(0i32);
pub const D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D10_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D10_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_FORMAT_SUPPORT(pub i32);
pub const D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16777216i32);
pub const D3D10_FORMAT_SUPPORT_BLENDABLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(32768i32);
pub const D3D10_FORMAT_SUPPORT_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1i32);
pub const D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1048576i32);
pub const D3D10_FORMAT_SUPPORT_CPU_LOCKABLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(131072i32);
pub const D3D10_FORMAT_SUPPORT_DEPTH_STENCIL: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(65536i32);
pub const D3D10_FORMAT_SUPPORT_DISPLAY: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(524288i32);
pub const D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4i32);
pub const D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2i32);
pub const D3D10_FORMAT_SUPPORT_MIP: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4096i32);
pub const D3D10_FORMAT_SUPPORT_MIP_AUTOGEN: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8192i32);
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(4194304i32);
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2097152i32);
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(262144i32);
pub const D3D10_FORMAT_SUPPORT_RENDER_TARGET: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16384i32);
pub const D3D10_FORMAT_SUPPORT_SHADER_GATHER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8388608i32);
pub const D3D10_FORMAT_SUPPORT_SHADER_LOAD: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(256i32);
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(512i32);
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(1024i32);
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(2048i32);
pub const D3D10_FORMAT_SUPPORT_SO_BUFFER: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(8i32);
pub const D3D10_FORMAT_SUPPORT_TEXTURE1D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(16i32);
pub const D3D10_FORMAT_SUPPORT_TEXTURE2D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(32i32);
pub const D3D10_FORMAT_SUPPORT_TEXTURE3D: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(64i32);
pub const D3D10_FORMAT_SUPPORT_TEXTURECUBE: D3D10_FORMAT_SUPPORT = D3D10_FORMAT_SUPPORT(128i32);
pub const D3D10_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D10_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D10_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D10_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_VERTICES: u32 = 6u32;
pub const D3D10_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D10_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 64u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_INFOQUEUE_STORAGE_FILTER_OVERRIDE: windows_core::PCWSTR = windows_core::w!("InfoQueueStorageFilterOverride");
pub const D3D10_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_INFO_QUEUE_FILTER {
    pub AllowList: D3D10_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D10_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D10_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D10_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D10_MESSAGE_ID,
}
impl Default for D3D10_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_INPUT_CLASSIFICATION(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(1i32);
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(0i32);
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_MAP(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_MAPPED_TEXTURE2D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
}
impl Default for D3D10_MAPPED_TEXTURE2D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_MAPPED_TEXTURE3D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl Default for D3D10_MAPPED_TEXTURE3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_MAP_FLAG(pub i32);
pub const D3D10_MAP_FLAG_DO_NOT_WAIT: D3D10_MAP_FLAG = D3D10_MAP_FLAG(1048576i32);
pub const D3D10_MAP_READ: D3D10_MAP = D3D10_MAP(1i32);
pub const D3D10_MAP_READ_WRITE: D3D10_MAP = D3D10_MAP(3i32);
pub const D3D10_MAP_WRITE: D3D10_MAP = D3D10_MAP(2i32);
pub const D3D10_MAP_WRITE_DISCARD: D3D10_MAP = D3D10_MAP(4i32);
pub const D3D10_MAP_WRITE_NO_OVERWRITE: D3D10_MAP = D3D10_MAP(5i32);
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D10_MAX_DEPTH: f32 = 1f32;
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl Default for D3D10_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_MESSAGE_CATEGORY(pub i32);
pub const D3D10_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(0i32);
pub const D3D10_MESSAGE_CATEGORY_CLEANUP: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(3i32);
pub const D3D10_MESSAGE_CATEGORY_COMPILATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(4i32);
pub const D3D10_MESSAGE_CATEGORY_EXECUTION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(9i32);
pub const D3D10_MESSAGE_CATEGORY_INITIALIZATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(2i32);
pub const D3D10_MESSAGE_CATEGORY_MISCELLANEOUS: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(1i32);
pub const D3D10_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(8i32);
pub const D3D10_MESSAGE_CATEGORY_SHADER: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(10i32);
pub const D3D10_MESSAGE_CATEGORY_STATE_CREATION: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(5i32);
pub const D3D10_MESSAGE_CATEGORY_STATE_GETTING: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(7i32);
pub const D3D10_MESSAGE_CATEGORY_STATE_SETTING: D3D10_MESSAGE_CATEGORY = D3D10_MESSAGE_CATEGORY(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_MESSAGE_ID(pub i32);
pub const D3D10_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(392i32);
pub const D3D10_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(297i32);
pub const D3D10_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(298i32);
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(296i32);
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(295i32);
pub const D3D10_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(299i32);
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(402i32);
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(403i32);
pub const D3D10_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(321i32);
pub const D3D10_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(322i32);
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(262i32);
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(263i32);
pub const D3D10_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(261i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(285i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(284i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(286i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048637i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048597i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048598i32);
pub const D3D10_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048596i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(282i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(278i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(281i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(280i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(283i32);
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(279i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(28i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(13i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(22i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(23i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(24i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(25i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(26i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(27i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(14i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(15i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(16i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(17i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(18i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(19i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(20i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(21i32);
pub const D3D10_MESSAGE_ID_CORRUPTED_THIS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(12i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(214i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(217i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(213i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(216i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(218i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(212i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(215i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048600i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048612i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048613i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048623i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048622i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(220i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048624i32);
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(219i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(69i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(64i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(72i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(63i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(66i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(65i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(67i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(68i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(58i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(73i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(71i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(70i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(60i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(61i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(57i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(62i32);
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(59i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(400i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(401i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(399i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(396i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(397i32);
pub const D3D10_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(398i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(206i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(209i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(208i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(207i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(201i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(200i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(202i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(205i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(204i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(203i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(211i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048577i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(210i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(148i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(143i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(145i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(144i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(146i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(149i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(147i32);
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(142i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(188i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(189i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(177i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(181i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(183i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(174i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(179i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(185i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(172i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(173i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(182i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(187i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(190i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(186i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(180i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(171i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(178i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(175i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(184i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(386i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(176i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(169i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(170i32);
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(168i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(160i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(420i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(153i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(159i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(152i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(155i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(154i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(157i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(158i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(163i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(164i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(162i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(150i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(156i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(151i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(385i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(391i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(161i32);
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048599i32);
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(192i32);
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(193i32);
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(191i32);
pub const D3D10_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(395i32);
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(233i32);
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(232i32);
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(235i32);
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(234i32);
pub const D3D10_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(394i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048578i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048601i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(195i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(196i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(194i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(197i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(199i32);
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(198i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(140i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(135i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(137i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(136i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(138i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(141i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(139i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(133i32);
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(134i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048630i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048588i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048617i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048610i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048634i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048608i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048589i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048585i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048616i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048609i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048614i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048587i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048586i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048631i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048615i32);
pub const D3D10_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048611i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048635i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048581i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048580i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(222i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(223i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(224i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(227i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(221i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(226i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(229i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(228i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(225i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048605i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048604i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048579i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048625i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(231i32);
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(230i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048607i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048606i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(131i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(126i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(128i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(127i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(129i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048603i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(132i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(130i32);
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(125i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(87i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(82i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(81i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(84i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(83i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(85i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(86i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(76i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(90i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(89i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(88i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(78i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(79i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(74i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(80i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(77i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(75i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(104i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(99i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(98i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(101i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(100i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(102i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(103i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(93i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(107i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(106i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(105i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(95i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(96i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(91i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(97i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(94i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(92i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(121i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(116i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(115i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(118i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(117i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(119i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(120i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(110i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(124i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(123i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(122i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(112i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(113i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(108i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(114i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(111i32);
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(109i32);
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(166i32);
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(167i32);
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(165i32);
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048638i32);
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048576i32);
pub const D3D10_MESSAGE_ID_D3D10_MESSAGES_END: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(443i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(340i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(339i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(336i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(338i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(337i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(364i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(350i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(351i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(360i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(358i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(357i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(359i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(368i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(349i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(365i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(417i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(377i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(376i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(363i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(369i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(362i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(415i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(416i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(370i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(372i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(371i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(373i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(361i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(421i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(390i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(352i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(353i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(375i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(374i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(335i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(348i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(355i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(356i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(366i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(341i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(367i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(384i32);
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(354i32);
pub const D3D10_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(277i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(269i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(270i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(268i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(251i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(6i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(252i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(5i32);
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(249i32);
pub const D3D10_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(264i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(242i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(2i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(243i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(244i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048594i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(237i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(236i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(240i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(419i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(239i32);
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(418i32);
pub const D3D10_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(9i32);
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(383i32);
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(381i32);
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(382i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(273i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(274i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(272i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(257i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(8i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(258i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(7i32);
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(255i32);
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(378i32);
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(380i32);
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(379i32);
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(290i32);
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(291i32);
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(294i32);
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(292i32);
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(293i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(276i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(275i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(260i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048632i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048595i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(387i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(259i32);
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048593i32);
pub const D3D10_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(389i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(344i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(347i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(343i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(345i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(342i32);
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(346i32);
pub const D3D10_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(271i32);
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(10i32);
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(254i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(266i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(267i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(265i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(247i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(4i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(248i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(3i32);
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(245i32);
pub const D3D10_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048627i32);
pub const D3D10_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048628i32);
pub const D3D10_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048602i32);
pub const D3D10_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048626i32);
pub const D3D10_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048619i32);
pub const D3D10_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(51i32);
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(250i32);
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(39i32);
pub const D3D10_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(40i32);
pub const D3D10_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(38i32);
pub const D3D10_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(37i32);
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(241i32);
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(32i32);
pub const D3D10_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(30i32);
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048592i32);
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(238i32);
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(31i32);
pub const D3D10_MESSAGE_ID_LIVE_BLENDSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(435i32);
pub const D3D10_MESSAGE_ID_LIVE_BUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(423i32);
pub const D3D10_MESSAGE_ID_LIVE_COUNTER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(440i32);
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(436i32);
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(429i32);
pub const D3D10_MESSAGE_ID_LIVE_DEVICE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(441i32);
pub const D3D10_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(431i32);
pub const D3D10_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(433i32);
pub const D3D10_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(422i32);
pub const D3D10_MESSAGE_ID_LIVE_PIXELSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(432i32);
pub const D3D10_MESSAGE_ID_LIVE_PREDICATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(439i32);
pub const D3D10_MESSAGE_ID_LIVE_QUERY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(438i32);
pub const D3D10_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(437i32);
pub const D3D10_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(428i32);
pub const D3D10_MESSAGE_ID_LIVE_SAMPLER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(434i32);
pub const D3D10_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(427i32);
pub const D3D10_MESSAGE_ID_LIVE_SWAPCHAIN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(442i32);
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE1D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(424i32);
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE2D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(425i32);
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE3D: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(426i32);
pub const D3D10_MESSAGE_ID_LIVE_VERTEXSHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(430i32);
pub const D3D10_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(29i32);
pub const D3D10_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(47i32);
pub const D3D10_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(48i32);
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(388i32);
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048591i32);
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048636i32);
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048590i32);
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(49i32);
pub const D3D10_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(406i32);
pub const D3D10_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(409i32);
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(256i32);
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(44i32);
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048584i32);
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(45i32);
pub const D3D10_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(43i32);
pub const D3D10_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(42i32);
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(408i32);
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(407i32);
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(405i32);
pub const D3D10_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(410i32);
pub const D3D10_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(411i32);
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(414i32);
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(412i32);
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(413i32);
pub const D3D10_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(331i32);
pub const D3D10_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(330i32);
pub const D3D10_MESSAGE_ID_REF_INFO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(334i32);
pub const D3D10_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(329i32);
pub const D3D10_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(333i32);
pub const D3D10_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(332i32);
pub const D3D10_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(326i32);
pub const D3D10_MESSAGE_ID_REF_THREADING_MODE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(327i32);
pub const D3D10_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(328i32);
pub const D3D10_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(46i32);
pub const D3D10_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048629i32);
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(325i32);
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(324i32);
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(323i32);
pub const D3D10_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(404i32);
pub const D3D10_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(50i32);
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(55i32);
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(54i32);
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(52i32);
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(53i32);
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(56i32);
pub const D3D10_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(393i32);
pub const D3D10_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048633i32);
pub const D3D10_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(253i32);
pub const D3D10_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(41i32);
pub const D3D10_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048620i32);
pub const D3D10_MESSAGE_ID_STRING_FROM_APPLICATION: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(11i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(303i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(304i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(302i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(300i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(301i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(305i32);
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(306i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(310i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(311i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(309i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(307i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(308i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(312i32);
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(313i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(317i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(318i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(316i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(314i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(315i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(319i32);
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(320i32);
pub const D3D10_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048621i32);
pub const D3D10_MESSAGE_ID_UNKNOWN: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(0i32);
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(288i32);
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(289i32);
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(287i32);
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(246i32);
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(35i32);
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048582i32);
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048583i32);
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(36i32);
pub const D3D10_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(34i32);
pub const D3D10_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(33i32);
pub const D3D10_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D10_MESSAGE_ID = D3D10_MESSAGE_ID(1048618i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_MESSAGE_SEVERITY(pub i32);
pub const D3D10_MESSAGE_SEVERITY_CORRUPTION: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(0i32);
pub const D3D10_MESSAGE_SEVERITY_ERROR: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(1i32);
pub const D3D10_MESSAGE_SEVERITY_INFO: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(3i32);
pub const D3D10_MESSAGE_SEVERITY_MESSAGE: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(4i32);
pub const D3D10_MESSAGE_SEVERITY_WARNING: D3D10_MESSAGE_SEVERITY = D3D10_MESSAGE_SEVERITY(2i32);
pub const D3D10_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_MIN_DEPTH: f32 = 0f32;
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D10_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D10_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6u32;
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D10_MUTE_CATEGORY: windows_core::PCWSTR = windows_core::w!("Mute_CATEGORY_%s");
pub const D3D10_MUTE_DEBUG_OUTPUT: windows_core::PCWSTR = windows_core::w!("MuteDebugOutput");
pub const D3D10_MUTE_ID_DECIMAL: windows_core::PCWSTR = windows_core::w!("Mute_ID_%d");
pub const D3D10_MUTE_ID_STRING: windows_core::PCWSTR = windows_core::w!("Mute_ID_%s");
pub const D3D10_MUTE_SEVERITY: windows_core::PCWSTR = windows_core::w!("Mute_SEVERITY_%s");
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_PASS_DESC {
    pub Name: windows_core::PCSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
impl Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: core::mem::ManuallyDrop<Option<ID3D10EffectShaderVariable>>,
    pub ShaderIndex: u32,
}
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13u32;
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_QUERY(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DESC {
    pub Query: D3D10_QUERY,
    pub MiscFlags: u32,
}
pub const D3D10_QUERY_EVENT: D3D10_QUERY = D3D10_QUERY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_QUERY_MISC_FLAG(pub i32);
pub const D3D10_QUERY_MISC_PREDICATEHINT: D3D10_QUERY_MISC_FLAG = D3D10_QUERY_MISC_FLAG(1i32);
pub const D3D10_QUERY_OCCLUSION: D3D10_QUERY = D3D10_QUERY(1i32);
pub const D3D10_QUERY_OCCLUSION_PREDICATE: D3D10_QUERY = D3D10_QUERY(5i32);
pub const D3D10_QUERY_PIPELINE_STATISTICS: D3D10_QUERY = D3D10_QUERY(4i32);
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: D3D10_QUERY = D3D10_QUERY(7i32);
pub const D3D10_QUERY_SO_STATISTICS: D3D10_QUERY = D3D10_QUERY(6i32);
pub const D3D10_QUERY_TIMESTAMP: D3D10_QUERY = D3D10_QUERY(2i32);
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: D3D10_QUERY = D3D10_QUERY(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_RAISE_FLAG(pub i32);
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D10_RAISE_FLAG = D3D10_RAISE_FLAG(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
}
pub const D3D10_REGKEY_PATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Direct3D");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_core::BOOL,
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D10_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_RTV_DIMENSION,
    pub Anonymous: D3D10_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D10_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_RTV,
    pub Texture1D: D3D10_TEX1D_RTV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D10_TEX2D_RTV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D10_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D10_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_REQ_MIP_LEVELS: u32 = 14u32;
pub const D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 8192u32;
pub const D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES: u32 = 128u32;
pub const D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP: u32 = 20u32;
pub const D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE1D_U_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D10_REQ_TEXTURECUBE_DIMENSION: u32 = 8192u32;
pub const D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_RESOURCE_DIMENSION(pub i32);
pub const D3D10_RESOURCE_DIMENSION_BUFFER: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(1i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(2i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(3i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(4i32);
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_RESOURCE_MISC_FLAG(pub i32);
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(32i32);
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(1i32);
pub const D3D10_RESOURCE_MISC_SHARED: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(2i32);
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(16i32);
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_RTV_DIMENSION(pub i32);
pub const D3D10_RTV_DIMENSION_BUFFER: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(1i32);
pub const D3D10_RTV_DIMENSION_TEXTURE1D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(2i32);
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(3i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(4i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(5i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(6i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(7i32);
pub const D3D10_RTV_DIMENSION_TEXTURE3D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(8i32);
pub const D3D10_RTV_DIMENSION_UNKNOWN: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SAMPLER_DESC {
    pub Filter: D3D10_FILTER,
    pub AddressU: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D10_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D10_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl Default for D3D10_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
pub const D3D10_SDK_VERSION: u32 = 29u32;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_INFO {
    pub Size: u32,
    pub Creator: u32,
    pub EntrypointName: u32,
    pub ShaderTarget: u32,
    pub CompileFlags: u32,
    pub Files: u32,
    pub FileInfo: u32,
    pub Instructions: u32,
    pub InstructionInfo: u32,
    pub Variables: u32,
    pub VariableInfo: u32,
    pub InputVariables: u32,
    pub InputVariableInfo: u32,
    pub Tokens: u32,
    pub TokenInfo: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub ScopeVariables: u32,
    pub ScopeVariableInfo: u32,
    pub UintOffset: u32,
    pub StringOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SHADER_DEBUG_INST_INFO {
    pub Id: u32,
    pub Opcode: u32,
    pub uOutputs: u32,
    pub pOutputs: [D3D10_SHADER_DEBUG_OUTPUTREG_INFO; 2],
    pub TokenId: u32,
    pub NestingLevel: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub AccessedVars: u32,
    pub AccessedVarsInfo: u32,
}
impl Default for D3D10_SHADER_DEBUG_INST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    pub OutputRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub OutputReg: u32,
    pub TempArrayReg: u32,
    pub OutputComponents: [u32; 4],
    pub OutputVars: [D3D10_SHADER_DEBUG_OUTPUTVAR; 4],
    pub IndexReg: u32,
    pub IndexComp: u32,
}
impl Default for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: windows_core::BOOL,
    pub bInfPossible: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_REGTYPE(pub i32);
pub const D3D10_SHADER_DEBUG_REG_CBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(2i32);
pub const D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(8i32);
pub const D3D10_SHADER_DEBUG_REG_INPUT: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(0i32);
pub const D3D10_SHADER_DEBUG_REG_LITERAL: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(9i32);
pub const D3D10_SHADER_DEBUG_REG_OUTPUT: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(1i32);
pub const D3D10_SHADER_DEBUG_REG_SAMPLER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(7i32);
pub const D3D10_SHADER_DEBUG_REG_TBUFFER: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(3i32);
pub const D3D10_SHADER_DEBUG_REG_TEMP: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(4i32);
pub const D3D10_SHADER_DEBUG_REG_TEMPARRAY: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(5i32);
pub const D3D10_SHADER_DEBUG_REG_TEXTURE: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(6i32);
pub const D3D10_SHADER_DEBUG_REG_UNUSED: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_SCOPETYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    pub TokenId: u32,
    pub VarType: D3D10_SHADER_DEBUG_VARTYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub StructMemberScope: u32,
    pub uArrayIndices: u32,
    pub ArrayElements: u32,
    pub ArrayStrides: u32,
    pub uVariables: u32,
    pub uFirstVariable: u32,
}
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(7i32);
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(1i32);
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(2i32);
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(4i32);
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    pub ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    pub Name: u32,
    pub uNameLen: u32,
    pub uVariables: u32,
    pub VariableData: u32,
}
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(6i32);
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(5i32);
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    pub File: u32,
    pub Line: u32,
    pub Column: u32,
    pub TokenLength: u32,
    pub TokenId: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_VARTYPE(pub i32);
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
}
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0u32;
pub const D3D10_SHADER_NO_PRESHADER: u32 = 256u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: u32 = 16384u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: u32 = 0u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: u32 = 32768u32;
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
pub const D3D10_SHADER_PARTIAL_PRECISION: u32 = 32u32;
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: u32 = 1024u32;
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
    pub TextureCubeArray: D3D10_TEXCUBE_ARRAY_SRV1,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
}
impl Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D10_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D10_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D10_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(pub i32);
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_STATE_BLOCK_MASK {
    pub VS: u8,
    pub VSSamplers: [u8; 2],
    pub VSShaderResources: [u8; 16],
    pub VSConstantBuffers: [u8; 2],
    pub GS: u8,
    pub GSSamplers: [u8; 2],
    pub GSShaderResources: [u8; 16],
    pub GSConstantBuffers: [u8; 2],
    pub PS: u8,
    pub PSSamplers: [u8; 2],
    pub PSShaderResources: [u8; 16],
    pub PSConstantBuffers: [u8; 2],
    pub IAVertexBuffers: [u8; 2],
    pub IAIndexBuffer: u8,
    pub IAInputLayout: u8,
    pub IAPrimitiveTopology: u8,
    pub OMRenderTargets: u8,
    pub OMDepthStencilState: u8,
    pub OMBlendState: u8,
    pub RSViewports: u8,
    pub RSScissorRects: u8,
    pub RSRasterizerState: u8,
    pub SOBuffers: u8,
    pub Predication: u8,
}
impl Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_STENCIL_OP(pub i32);
pub const D3D10_STENCIL_OP_DECR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(8i32);
pub const D3D10_STENCIL_OP_DECR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(5i32);
pub const D3D10_STENCIL_OP_INCR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(7i32);
pub const D3D10_STENCIL_OP_INCR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(4i32);
pub const D3D10_STENCIL_OP_INVERT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(6i32);
pub const D3D10_STENCIL_OP_KEEP: D3D10_STENCIL_OP = D3D10_STENCIL_OP(1i32);
pub const D3D10_STENCIL_OP_REPLACE: D3D10_STENCIL_OP = D3D10_STENCIL_OP(3i32);
pub const D3D10_STENCIL_OP_ZERO: D3D10_STENCIL_OP = D3D10_STENCIL_OP(2i32);
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl Default for D3D10_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: windows_core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_TEXTURECUBE_FACE(pub i32);
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_X: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(1i32);
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Y: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(3i32);
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Z: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(5i32);
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_X: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(0i32);
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Y: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(2i32);
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Z: D3D10_TEXTURECUBE_FACE = D3D10_TEXTURECUBE_FACE(4i32);
pub const D3D10_TEXTURE_ADDRESS_BORDER: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(4i32);
pub const D3D10_TEXTURE_ADDRESS_CLAMP: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(3i32);
pub const D3D10_TEXTURE_ADDRESS_MIRROR: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(2i32);
pub const D3D10_TEXTURE_ADDRESS_MIRROR_ONCE: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_TEXTURE_ADDRESS_MODE(pub i32);
pub const D3D10_TEXTURE_ADDRESS_WRAP: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(1i32);
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub const D3D10_UNMUTE_SEVERITY_INFO: windows_core::PCWSTR = windows_core::w!("Unmute_SEVERITY_INFO");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D10_USAGE(pub i32);
pub const D3D10_USAGE_DEFAULT: D3D10_USAGE = D3D10_USAGE(0i32);
pub const D3D10_USAGE_DYNAMIC: D3D10_USAGE = D3D10_USAGE(2i32);
pub const D3D10_USAGE_IMMUTABLE: D3D10_USAGE = D3D10_USAGE(1i32);
pub const D3D10_USAGE_STAGING: D3D10_USAGE = D3D10_USAGE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_VIEWPORT {
    pub TopLeftX: i32,
    pub TopLeftY: i32,
    pub Width: u32,
    pub Height: u32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
pub const D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D10_VIEWPORT_BOUNDS_MAX: u32 = 16383u32;
pub const D3D10_VIEWPORT_BOUNDS_MIN: i32 = -16384i32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_OUTPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(11i32);
pub const D3D11_SHADER_DEBUG_REG_UAV: D3D10_SHADER_DEBUG_REGTYPE = D3D10_SHADER_DEBUG_REGTYPE(12i32);
pub const D3D_MAJOR_VERSION: u32 = 10u32;
pub const D3D_MINOR_VERSION: u32 = 0u32;
pub const D3D_SPEC_DATE_DAY: u32 = 8u32;
pub const D3D_SPEC_DATE_MONTH: u32 = 8u32;
pub const D3D_SPEC_DATE_YEAR: u32 = 2006u32;
pub const D3D_SPEC_VERSION: f64 = 1.050005f64;
pub const DXGI_DEBUG_D3D10: windows_core::GUID = windows_core::GUID::from_u128(0x243b4c52_3606_4d3a_99d7_a7e7b33ed706);
pub const GUID_DeviceType: windows_core::GUID = windows_core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
windows_core::imp::define_interface!(ID3D10Asynchronous, ID3D10Asynchronous_Vtbl, 0x9b7e4c0d_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Asynchronous {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Asynchronous, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10Asynchronous {
    pub unsafe fn Begin(&self) {
        unsafe { (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn End(&self) {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetData(&self, pdata: Option<*mut core::ffi::c_void>, datasize: u32, getdataflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pdata.unwrap_or(core::mem::zeroed()) as _, datasize, getdataflags).ok() }
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Asynchronous_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
unsafe impl Send for ID3D10Asynchronous {}
unsafe impl Sync for ID3D10Asynchronous {}
pub trait ID3D10Asynchronous_Impl: ID3D10DeviceChild_Impl {
    fn Begin(&self);
    fn End(&self);
    fn GetData(&self, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::Result<()>;
    fn GetDataSize(&self) -> u32;
}
impl ID3D10Asynchronous_Vtbl {
    pub const fn new<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::Begin(this)
            }
        }
        unsafe extern "system" fn End<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::End(this)
            }
        }
        unsafe extern "system" fn GetData<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&getdataflags)).into()
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::GetDataSize(this)
            }
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Asynchronous {}
windows_core::imp::define_interface!(ID3D10BlendState, ID3D10BlendState_Vtbl, 0xedad8d19_8a35_4d6d_8566_2ea276cde161);
impl core::ops::Deref for ID3D10BlendState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10BlendState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10BlendState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC),
}
unsafe impl Send for ID3D10BlendState {}
unsafe impl Sync for ID3D10BlendState {}
pub trait ID3D10BlendState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC);
}
impl ID3D10BlendState_Vtbl {
    pub const fn new<Identity: ID3D10BlendState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10BlendState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10BlendState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10BlendState {}
windows_core::imp::define_interface!(ID3D10BlendState1, ID3D10BlendState1_Vtbl, 0xedad8d99_8a35_4d6d_8566_2ea276cde161);
impl core::ops::Deref for ID3D10BlendState1 {
    type Target = ID3D10BlendState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10BlendState1, windows_core::IUnknown, ID3D10DeviceChild, ID3D10BlendState);
impl ID3D10BlendState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState1_Vtbl {
    pub base__: ID3D10BlendState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC1),
}
unsafe impl Send for ID3D10BlendState1 {}
unsafe impl Sync for ID3D10BlendState1 {}
pub trait ID3D10BlendState1_Impl: ID3D10BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1);
}
impl ID3D10BlendState1_Vtbl {
    pub const fn new<Identity: ID3D10BlendState1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D10BlendState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10BlendState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10BlendState_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10BlendState as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10BlendState1 {}
windows_core::imp::define_interface!(ID3D10Buffer, ID3D10Buffer_Vtbl, 0x9b7e4c02_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Buffer {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Buffer, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Buffer {
    pub unsafe fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), maptype, mapflags, ppdata as _).ok() }
    }
    pub unsafe fn Unmap(&self) {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Buffer_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BUFFER_DESC),
}
unsafe impl Send for ID3D10Buffer {}
unsafe impl Sync for ID3D10Buffer {}
pub trait ID3D10Buffer_Impl: ID3D10Resource_Impl {
    fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self);
    fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC);
}
impl ID3D10Buffer_Vtbl {
    pub const fn new<Identity: ID3D10Buffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::Map(this, core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::Unmap(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Buffer as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Buffer {}
windows_core::imp::define_interface!(ID3D10Counter, ID3D10Counter_Vtbl, 0x9b7e4c11_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Counter {
    type Target = ID3D10Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Counter, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous);
impl ID3D10Counter {
    pub unsafe fn GetDesc(&self) -> D3D10_COUNTER_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Counter_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_COUNTER_DESC),
}
unsafe impl Send for ID3D10Counter {}
unsafe impl Sync for ID3D10Counter {}
pub trait ID3D10Counter_Impl: ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC);
}
impl ID3D10Counter_Vtbl {
    pub const fn new<Identity: ID3D10Counter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10Counter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Counter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Counter as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Counter {}
windows_core::imp::define_interface!(ID3D10Debug, ID3D10Debug_Vtbl, 0x9b7e4e01_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10Debug, windows_core::IUnknown);
impl ID3D10Debug {
    pub unsafe fn SetFeatureMask(&self, mask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFeatureMask)(windows_core::Interface::as_raw(self), mask).ok() }
    }
    pub unsafe fn GetFeatureMask(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureMask)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPresentPerRenderOpDelay)(windows_core::Interface::as_raw(self), milliseconds).ok() }
    }
    pub unsafe fn GetPresentPerRenderOpDelay(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPresentPerRenderOpDelay)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, pswapchain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Dxgi::IDXGISwapChain>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSwapChain)(windows_core::Interface::as_raw(self), pswapchain.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSwapChain(&self) -> windows_core::Result<super::Dxgi::IDXGISwapChain> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Debug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetPresentPerRenderOpDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPresentPerRenderOpDelay: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSwapChain: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10Debug {}
unsafe impl Sync for ID3D10Debug {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10Debug_Impl: windows_core::IUnknownImpl {
    fn SetFeatureMask(&self, mask: u32) -> windows_core::Result<()>;
    fn GetFeatureMask(&self) -> u32;
    fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> windows_core::Result<()>;
    fn GetPresentPerRenderOpDelay(&self) -> u32;
    fn SetSwapChain(&self, pswapchain: windows_core::Ref<super::Dxgi::IDXGISwapChain>) -> windows_core::Result<()>;
    fn GetSwapChain(&self) -> windows_core::Result<super::Dxgi::IDXGISwapChain>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D10Debug_Vtbl {
    pub const fn new<Identity: ID3D10Debug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFeatureMask<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::SetFeatureMask(this, core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::GetFeatureMask(this)
            }
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, milliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::SetPresentPerRenderOpDelay(this, core::mem::transmute_copy(&milliseconds)).into()
            }
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::GetPresentPerRenderOpDelay(this)
            }
        }
        unsafe extern "system" fn SetSwapChain<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pswapchain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::SetSwapChain(this, core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetSwapChain<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Debug_Impl::GetSwapChain(this) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Validate<Identity: ID3D10Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Debug_Impl::Validate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Debug as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for ID3D10Debug {}
windows_core::imp::define_interface!(ID3D10DepthStencilState, ID3D10DepthStencilState_Vtbl, 0x2b4b1cc8_a4ad_41f8_8322_ca86fc3ec675);
impl core::ops::Deref for ID3D10DepthStencilState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10DepthStencilState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10DepthStencilState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_DESC),
}
unsafe impl Send for ID3D10DepthStencilState {}
unsafe impl Sync for ID3D10DepthStencilState {}
pub trait ID3D10DepthStencilState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
impl ID3D10DepthStencilState_Vtbl {
    pub const fn new<Identity: ID3D10DepthStencilState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10DepthStencilState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DepthStencilState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10DepthStencilState {}
windows_core::imp::define_interface!(ID3D10DepthStencilView, ID3D10DepthStencilView_Vtbl, 0x9b7e4c09_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10DepthStencilView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10DepthStencilView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10DepthStencilView {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10DepthStencilView {}
unsafe impl Sync for ID3D10DepthStencilView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10DepthStencilView_Vtbl {
    pub const fn new<Identity: ID3D10DepthStencilView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10DepthStencilView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DepthStencilView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10DepthStencilView {}
windows_core::imp::define_interface!(ID3D10Device, ID3D10Device_Vtbl, 0x9b7e4c0f_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10Device, windows_core::IUnknown);
impl ID3D10Device {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0)
    where
        P0: windows_core::Param<ID3D10PixelShader>,
    {
        unsafe { (windows_core::Interface::vtable(self).PSSetShader)(windows_core::Interface::as_raw(self), ppixelshader.param().abi()) }
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0)
    where
        P0: windows_core::Param<ID3D10VertexShader>,
    {
        unsafe { (windows_core::Interface::vtable(self).VSSetShader)(windows_core::Interface::as_raw(self), pvertexshader.param().abi()) }
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation) }
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation) }
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D10InputLayout>,
    {
        unsafe { (windows_core::Interface::vtable(self).IASetInputLayout)(windows_core::Interface::as_raw(self), pinputlayout.param().abi()) }
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*const Option<ID3D10Buffer>>, pstrides: Option<*const u32>, poffsets: Option<*const u32>) {
        unsafe { (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D10Buffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.param().abi(), format, offset) }
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation) }
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        unsafe { (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation) }
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0)
    where
        P0: windows_core::Param<ID3D10GeometryShader>,
    {
        unsafe { (windows_core::Interface::vtable(self).GSSetShader)(windows_core::Interface::as_raw(self), pshader.param().abi()) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        unsafe { (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), topology) }
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn SetPredication<P0>(&self, ppredicate: P0, predicatevalue: bool)
    where
        P0: windows_core::Param<ID3D10Predicate>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), ppredicate.param().abi(), predicatevalue.into()) }
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn OMSetRenderTargets<P2>(&self, pprendertargetviews: Option<&[Option<ID3D10RenderTargetView>]>, pdepthstencilview: P2)
    where
        P2: windows_core::Param<ID3D10DepthStencilView>,
    {
        unsafe { (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi()) }
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: &[f32; 4], samplemask: u32)
    where
        P0: windows_core::Param<ID3D10BlendState>,
    {
        unsafe { (windows_core::Interface::vtable(self).OMSetBlendState)(windows_core::Interface::as_raw(self), pblendstate.param().abi(), core::mem::transmute(blendfactor.as_ptr()), samplemask) }
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D10DepthStencilState>,
    {
        unsafe { (windows_core::Interface::vtable(self).OMSetDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencilstate.param().abi(), stencilref) }
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: Option<*const Option<ID3D10Buffer>>, poffsets: Option<*const u32>) {
        unsafe { (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), numbuffers, ppsotargets.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe { (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D10RasterizerState>,
    {
        unsafe { (windows_core::Interface::vtable(self).RSSetState)(windows_core::Interface::as_raw(self), prasterizerstate.param().abi()) }
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D10_VIEWPORT]>) {
        unsafe { (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviewports.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[super::super::Foundation::RECT]>) {
        unsafe { (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn CopySubresourceRegion<P0, P5>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P5, srcsubresource: u32, psrcbox: Option<*const D3D10_BOX>)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P5: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopySubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, psrcbox.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P1: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi()) }
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D10_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, pdstbox.unwrap_or(core::mem::zeroed()) as _, psrcdata, srcrowpitch, srcdepthpitch) }
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D10RenderTargetView>,
    {
        unsafe { (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), prendertargetview.param().abi(), core::mem::transmute(colorrgba.as_ptr())) }
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: windows_core::Param<ID3D10DepthStencilView>,
    {
        unsafe { (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), pdepthstencilview.param().abi(), clearflags, depth, stencil) }
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        unsafe { (windows_core::Interface::vtable(self).GenerateMips)(windows_core::Interface::as_raw(self), pshaderresourceview.param().abi()) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P2>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P2, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P2: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format) }
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn PSGetShader(&self) -> windows_core::Result<ID3D10PixelShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn VSGetShader(&self) -> windows_core::Result<ID3D10VertexShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).PSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D10InputLayout> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetInputLayout)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*mut Option<ID3D10Buffer>>, pstrides: Option<*mut u32>, poffsets: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).IAGetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: Option<*mut Option<ID3D10Buffer>>, format: Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).IAGetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.unwrap_or(core::mem::zeroed()) as _, format.unwrap_or(core::mem::zeroed()) as _, offset.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn GSGetShader(&self) -> windows_core::Result<ID3D10GeometryShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).VSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn GetPredication(&self, pppredicate: Option<*mut Option<ID3D10Predicate>>, ppredicatevalue: Option<*mut windows_core::BOOL>) {
        unsafe { (windows_core::Interface::vtable(self).GetPredication)(windows_core::Interface::as_raw(self), pppredicate.unwrap_or(core::mem::zeroed()) as _, ppredicatevalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe { (windows_core::Interface::vtable(self).GSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: Option<&mut [Option<ID3D10RenderTargetView>]>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) {
        unsafe { (windows_core::Interface::vtable(self).OMGetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: Option<*mut Option<ID3D10BlendState>>, blendfactor: Option<&mut [f32; 4]>, psamplemask: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).OMGetBlendState)(windows_core::Interface::as_raw(self), ppblendstate.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psamplemask.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>, pstencilref: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).OMGetDepthStencilState)(windows_core::Interface::as_raw(self), ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _, pstencilref.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: Option<*mut Option<ID3D10Buffer>>, poffsets: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).SOGetTargets)(windows_core::Interface::as_raw(self), numbuffers, ppsotargets.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D10RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSGetState)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: Option<*mut D3D10_VIEWPORT>) {
        unsafe { (windows_core::Interface::vtable(self).RSGetViewports)(windows_core::Interface::as_raw(self), numviewports as _, pviewports.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: Option<*mut super::super::Foundation::RECT>) {
        unsafe { (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), numrects as _, prects.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags).ok() }
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok() }
    }
    pub unsafe fn ClearState(&self) {
        unsafe { (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Flush(&self) {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>, ppbuffer: Option<*mut Option<ID3D10Buffer>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateBuffer)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture1D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture1D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture2D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture2D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture3D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC>, ppsrview: Option<*mut Option<ID3D10ShaderResourceView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppsrview.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_RENDER_TARGET_VIEW_DESC>, pprtview: Option<*mut Option<ID3D10RenderTargetView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, pprtview.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: Option<*mut Option<ID3D10InputLayout>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateInputLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len().try_into().unwrap(), core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len().try_into().unwrap(), ppinputlayout.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8], ppvertexshader: Option<*mut Option<ID3D10VertexShader>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), ppvertexshader.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8], ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateGeometryShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), ppgeometryshader.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: Option<&[D3D10_SO_DECLARATION_ENTRY]>, outputstreamstride: u32, ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(psodeclaration.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psodeclaration.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), outputstreamstride, ppgeometryshader.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8], pppixelshader: Option<*mut Option<ID3D10PixelShader>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pppixelshader.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: Option<*mut Option<ID3D10BlendState>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencildesc, ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: Option<*mut Option<ID3D10RasterizerState>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateRasterizerState)(windows_core::Interface::as_raw(self), prasterizerdesc, pprasterizerstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: Option<*mut Option<ID3D10SamplerState>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateSamplerState)(windows_core::Interface::as_raw(self), psamplerdesc, ppsamplerstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: Option<*mut Option<ID3D10Query>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), pquerydesc, ppquery.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: Option<*mut Option<ID3D10Predicate>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreatePredicate)(windows_core::Interface::as_raw(self), ppredicatedesc, pppredicate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: Option<*mut Option<ID3D10Counter>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateCounter)(windows_core::Interface::as_raw(self), pcounterdesc, ppcounter.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(windows_core::Interface::as_raw(self), format, samplecount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D10_COUNTER_INFO {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCounterInfo)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: Option<windows_core::PSTR>, pnamelength: Option<*mut u32>, szunits: Option<windows_core::PSTR>, punitslength: Option<*mut u32>, szdescription: Option<windows_core::PSTR>, pdescriptionlength: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckCounter)(windows_core::Interface::as_raw(self), pdesc, ptype as _, pactivecounters as _, szname.unwrap_or(core::mem::zeroed()) as _, pnamelength.unwrap_or(core::mem::zeroed()) as _, szunits.unwrap_or(core::mem::zeroed()) as _, punitslength.unwrap_or(core::mem::zeroed()) as _, szdescription.unwrap_or(core::mem::zeroed()) as _, pdescriptionlength.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource, returnedinterface, ppresource.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        unsafe { (windows_core::Interface::vtable(self).SetTextFilterSize)(windows_core::Interface::as_raw(self), width, height) }
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: Option<*mut u32>, pheight: Option<*mut u32>) {
        unsafe { (windows_core::Interface::vtable(self).GetTextFilterSize)(windows_core::Interface::as_raw(self), pwidth.unwrap_or(core::mem::zeroed()) as _, pheight.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub PSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IASetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IASetIndexBuffer: usize,
    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub GSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IASetPrimitiveTopology: usize,
    pub VSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    pub GSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OMSetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const u32),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D10_VIEWPORT),
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT),
    pub CopySubresourceRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const D3D10_BOX),
    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub UpdateSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D10_BOX, *const core::ffi::c_void, u32, u32),
    pub ClearRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresource: usize,
    pub VSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub PSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub PSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IAGetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::Dxgi::Common::DXGI_FORMAT, *mut u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IAGetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IAGetPrimitiveTopology: usize,
    pub VSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL),
    pub GSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub OMGetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut u32),
    pub OMGetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D10_VIEWPORT),
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::RECT),
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CreateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BUFFER_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture1D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE1D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture1D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture2D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE2D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture2D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture3D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE3D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture3D: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_SHADER_RESOURCE_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_RENDER_TARGET_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_DEPTH_STENCIL_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateDepthStencilView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INPUT_ELEMENT_DESC, u32, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateInputLayout: usize,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const D3D10_SO_DECLARATION_ENTRY, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BLEND_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_DEPTH_STENCIL_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_RASTERIZER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_SAMPLER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_COUNTER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_COUNTER_DESC, *mut D3D10_COUNTER_TYPE, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub OpenSharedResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTextFilterSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub GetTextFilterSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32),
}
unsafe impl Send for ID3D10Device {}
unsafe impl Sync for ID3D10Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device_Impl: windows_core::IUnknownImpl {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: windows_core::Ref<ID3D10PixelShader>);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn VSSetShader(&self, pvertexshader: windows_core::Ref<ID3D10VertexShader>);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn IASetInputLayout(&self, pinputlayout: windows_core::Ref<ID3D10InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: windows_core::Ref<ID3D10Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn GSSetShader(&self, pshader: windows_core::Ref<ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn SetPredication(&self, ppredicate: windows_core::Ref<ID3D10Predicate>, predicatevalue: windows_core::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const Option<ID3D10RenderTargetView>, pdepthstencilview: windows_core::Ref<ID3D10DepthStencilView>);
    fn OMSetBlendState(&self, pblendstate: windows_core::Ref<ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: windows_core::Ref<ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn RSSetState(&self, prasterizerstate: windows_core::Ref<ID3D10RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: windows_core::Ref<ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, psrcresource: windows_core::Ref<ID3D10Resource>);
    fn UpdateSubresource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(&self, prendertargetview: windows_core::Ref<ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: windows_core::Ref<ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: windows_core::Ref<ID3D10ShaderResourceView>);
    fn ResolveSubresource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, psrcresource: windows_core::Ref<ID3D10Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: windows_core::OutRef<ID3D10PixelShader>);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn VSGetShader(&self, ppvertexshader: windows_core::OutRef<ID3D10VertexShader>);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: windows_core::OutRef<ID3D10InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: windows_core::OutRef<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: windows_core::OutRef<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn GSGetShader(&self, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn GetPredication(&self, pppredicate: windows_core::OutRef<ID3D10Predicate>, ppredicatevalue: *mut windows_core::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut Option<ID3D10RenderTargetView>, ppdepthstencilview: windows_core::OutRef<ID3D10DepthStencilView>);
    fn OMGetBlendState(&self, ppblendstate: windows_core::OutRef<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: windows_core::OutRef<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: windows_core::OutRef<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(&self, pprasterizerstate: windows_core::OutRef<ID3D10RasterizerState>);
    fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn Flush(&self);
    fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: windows_core::OutRef<ID3D10Buffer>) -> windows_core::Result<()>;
    fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: windows_core::OutRef<ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn CreateRenderTargetView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: windows_core::OutRef<ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn CreateDepthStencilView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: windows_core::OutRef<ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: windows_core::OutRef<ID3D10InputLayout>) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: windows_core::OutRef<ID3D10VertexShader>) -> windows_core::Result<()>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: windows_core::OutRef<ID3D10PixelShader>) -> windows_core::Result<()>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: windows_core::OutRef<ID3D10BlendState>) -> windows_core::Result<()>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: windows_core::OutRef<ID3D10DepthStencilState>) -> windows_core::Result<()>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: windows_core::OutRef<ID3D10RasterizerState>) -> windows_core::Result<()>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: windows_core::OutRef<ID3D10SamplerState>) -> windows_core::Result<()>;
    fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: windows_core::OutRef<ID3D10Query>) -> windows_core::Result<()>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: windows_core::OutRef<ID3D10Predicate>) -> windows_core::Result<()>;
    fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: windows_core::OutRef<ID3D10Counter>) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::Result<()>;
    fn GetCreationFlags(&self) -> u32;
    fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetTextFilterSize(&self, width: u32, height: u32);
    fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device_Vtbl {
    pub const fn new<Identity: ID3D10Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn PSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetShader(this, core::mem::transmute_copy(&ppixelshader))
            }
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn VSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetShader(this, core::mem::transmute_copy(&pvertexshader))
            }
        }
        unsafe extern "system" fn DrawIndexed<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawIndexed(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation))
            }
        }
        unsafe extern "system" fn Draw<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::Draw(this, core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&startvertexlocation))
            }
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputlayout: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetInputLayout(this, core::mem::transmute_copy(&pinputlayout))
            }
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
            }
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
            }
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation))
            }
        }
        unsafe extern "system" fn DrawInstanced<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation))
            }
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn GSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetShader(this, core::mem::transmute_copy(&pshader))
            }
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&topology))
            }
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicate: *mut core::ffi::c_void, predicatevalue: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPredication(this, core::mem::transmute_copy(&ppredicate), core::mem::transmute_copy(&predicatevalue))
            }
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&pdepthstencilview))
            }
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstate: *mut core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetBlendState(this, core::mem::transmute_copy(&pblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&samplemask))
            }
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilstate: *mut core::ffi::c_void, stencilref: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetDepthStencilState(this, core::mem::transmute_copy(&pdepthstencilstate), core::mem::transmute_copy(&stencilref))
            }
        }
        unsafe extern "system" fn SOSetTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut core::ffi::c_void, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SOSetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets))
            }
        }
        unsafe extern "system" fn DrawAuto<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawAuto(this)
            }
        }
        unsafe extern "system" fn RSSetState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerstate: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetState(this, core::mem::transmute_copy(&prasterizerstate))
            }
        }
        unsafe extern "system" fn RSSetViewports<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
            }
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
            }
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CopySubresourceRegion(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox))
            }
        }
        unsafe extern "system" fn CopyResource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CopyResource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&psrcresource))
            }
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::UpdateSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch))
            }
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertargetview: *mut core::ffi::c_void, colorrgba: *const f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearRenderTargetView(this, core::mem::transmute_copy(&prendertargetview), core::mem::transmute_copy(&colorrgba))
            }
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearDepthStencilView(this, core::mem::transmute_copy(&pdepthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil))
            }
        }
        unsafe extern "system" fn GenerateMips<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderresourceview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GenerateMips(this, core::mem::transmute_copy(&pshaderresourceview))
            }
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ResolveSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn PSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetShader(this, core::mem::transmute_copy(&pppixelshader))
            }
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn VSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetShader(this, core::mem::transmute_copy(&ppvertexshader))
            }
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinputlayout: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetInputLayout(this, core::mem::transmute_copy(&ppinputlayout))
            }
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
            }
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut *mut core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
            }
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
            }
        }
        unsafe extern "system" fn GSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetShader(this, core::mem::transmute_copy(&ppgeometryshader))
            }
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetPrimitiveTopology(this, core::mem::transmute_copy(&ptopology))
            }
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn GetPredication<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppredicate: *mut *mut core::ffi::c_void, ppredicatevalue: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetPredication(this, core::mem::transmute_copy(&pppredicate), core::mem::transmute_copy(&ppredicatevalue))
            }
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
            }
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
            }
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview))
            }
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblendstate: *mut *mut core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetBlendState(this, core::mem::transmute_copy(&ppblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&psamplemask))
            }
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdepthstencilstate: *mut *mut core::ffi::c_void, pstencilref: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetDepthStencilState(this, core::mem::transmute_copy(&ppdepthstencilstate), core::mem::transmute_copy(&pstencilref))
            }
        }
        unsafe extern "system" fn SOGetTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut core::ffi::c_void, poffsets: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SOGetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets))
            }
        }
        unsafe extern "system" fn RSGetState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprasterizerstate: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetState(this, core::mem::transmute_copy(&pprasterizerstate))
            }
        }
        unsafe extern "system" fn RSGetViewports<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
            }
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetDeviceRemovedReason(this).into()
            }
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, raiseflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags)).into()
            }
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetExceptionMode(this)
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearState(this)
            }
        }
        unsafe extern "system" fn Flush<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::Flush(this)
            }
        }
        unsafe extern "system" fn CreateBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateBuffer(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture1D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture1d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture2D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture2d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture3D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateShaderResourceView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateRenderTargetView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprtview)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateDepthStencilView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppdepthstencilview)).into()
            }
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateInputLayout(this, core::mem::transmute_copy(&pinputelementdescs), core::mem::transmute_copy(&numelements), core::mem::transmute_copy(&pshaderbytecodewithinputsignature), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppinputlayout)).into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppvertexshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateGeometryShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateGeometryShaderWithStreamOutput(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&psodeclaration), core::mem::transmute_copy(&numentries), core::mem::transmute_copy(&outputstreamstride), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pppixelshader)).into()
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateBlendState(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateDepthStencilState(this, core::mem::transmute_copy(&pdepthstencildesc), core::mem::transmute_copy(&ppdepthstencilstate)).into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateRasterizerState(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
            }
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateSamplerState(this, core::mem::transmute_copy(&psamplerdesc), core::mem::transmute_copy(&ppsamplerstate)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateQuery(this, core::mem::transmute_copy(&pquerydesc), core::mem::transmute_copy(&ppquery)).into()
            }
        }
        unsafe extern "system" fn CreatePredicate<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreatePredicate(this, core::mem::transmute_copy(&ppredicatedesc), core::mem::transmute_copy(&pppredicate)).into()
            }
        }
        unsafe extern "system" fn CreateCounter<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateCounter(this, core::mem::transmute_copy(&pcounterdesc), core::mem::transmute_copy(&ppcounter)).into()
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CheckFormatSupport(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pformatsupport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CheckMultisampleQualityLevels(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount)) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo))
            }
        }
        unsafe extern "system" fn CheckCounter<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CheckCounter(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pactivecounters), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pnamelength), core::mem::transmute_copy(&szunits), core::mem::transmute_copy(&punitslength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pdescriptionlength)).into()
            }
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetCreationFlags(this)
            }
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OpenSharedResource(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
            }
        }
        unsafe extern "system" fn SetTextFilterSize<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetTextFilterSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height))
            }
        }
        unsafe extern "system" fn GetTextFilterSize<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetTextFilterSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, OFFSET>,
            PSSetShader: PSSetShader::<Identity, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, OFFSET>,
            VSSetShader: VSSetShader::<Identity, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, OFFSET>,
            GSSetShader: GSSetShader::<Identity, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, OFFSET>,
            DrawAuto: DrawAuto::<Identity, OFFSET>,
            RSSetState: RSSetState::<Identity, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, OFFSET>,
            CopyResource: CopyResource::<Identity, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, OFFSET>,
            GenerateMips: GenerateMips::<Identity, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, OFFSET>,
            PSGetShader: PSGetShader::<Identity, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, OFFSET>,
            VSGetShader: VSGetShader::<Identity, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, OFFSET>,
            GSGetShader: GSGetShader::<Identity, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, OFFSET>,
            GetPredication: GetPredication::<Identity, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, OFFSET>,
            RSGetState: RSGetState::<Identity, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Identity, OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10Device {}
windows_core::imp::define_interface!(ID3D10Device1, ID3D10Device1_Vtbl, 0x9b7e4c8f_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Device1 {
    type Target = ID3D10Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Device1, windows_core::IUnknown, ID3D10Device);
impl ID3D10Device1 {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc: Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC1>, ppsrview: Option<*mut Option<ID3D10ShaderResourceView1>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView1)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppsrview.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: Option<*mut Option<ID3D10BlendState1>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState1)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1 {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device1_Vtbl {
    pub base__: ID3D10Device_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_SHADER_RESOURCE_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView1: usize,
    pub CreateBlendState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BLEND_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1,
}
unsafe impl Send for ID3D10Device1 {}
unsafe impl Sync for ID3D10Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1_Impl: ID3D10Device_Impl {
    fn CreateShaderResourceView1(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: windows_core::OutRef<ID3D10ShaderResourceView1>) -> windows_core::Result<()>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: windows_core::OutRef<ID3D10BlendState1>) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device1_Vtbl {
    pub const fn new<Identity: ID3D10Device1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::CreateShaderResourceView1(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
            }
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::CreateBlendState1(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::GetFeatureLevel(this)
            }
        }
        Self {
            base__: ID3D10Device_Vtbl::new::<Identity, OFFSET>(),
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device1 as windows_core::Interface>::IID || iid == &<ID3D10Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10Device1 {}
windows_core::imp::define_interface!(ID3D10DeviceChild, ID3D10DeviceChild_Vtbl, 0x9b7e4c00_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10DeviceChild, windows_core::IUnknown);
impl ID3D10DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10DeviceChild {}
unsafe impl Sync for ID3D10DeviceChild {}
pub trait ID3D10DeviceChild_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: windows_core::OutRef<ID3D10Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ID3D10DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice))
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10DeviceChild {}
windows_core::imp::define_interface!(ID3D10Effect, ID3D10Effect_Vtbl, 0x51b0ca8b_ec0b_4519_870d_8ee1cb5017c7);
windows_core::imp::interface_hierarchy!(ID3D10Effect, windows_core::IUnknown);
impl ID3D10Effect {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPool(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPool)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10EffectConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetVariableBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique> {
        unsafe { (windows_core::Interface::vtable(self).GetTechniqueByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetTechniqueByName<P0>(&self, name: P0) -> Option<ID3D10EffectTechnique>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTechniqueByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn Optimize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Optimize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsOptimized(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsOptimized)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Effect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsPool: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetVariableBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetTechniqueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectTechnique>,
    pub GetTechniqueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectTechnique>,
    pub Optimize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOptimized: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for ID3D10Effect {}
unsafe impl Sync for ID3D10Effect {}
pub trait ID3D10Effect_Impl: windows_core::IUnknownImpl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn IsPool(&self) -> windows_core::BOOL;
    fn GetDevice(&self) -> windows_core::Result<ID3D10Device>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectTechnique>;
    fn Optimize(&self) -> windows_core::Result<()>;
    fn IsOptimized(&self) -> windows_core::BOOL;
}
impl ID3D10Effect_Vtbl {
    pub const fn new<Identity: ID3D10Effect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn IsPool<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsPool(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Effect_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetVariableBySemantic<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetTechniqueByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectTechnique> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetTechniqueByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetTechniqueByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectTechnique> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetTechniqueByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn Optimize<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::Optimize(this).into()
            }
        }
        unsafe extern "system" fn IsOptimized<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsOptimized(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsValid: IsValid::<Identity, OFFSET>,
            IsPool: IsPool::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Identity, OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Identity, OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Identity, OFFSET>,
            Optimize: Optimize::<Identity, OFFSET>,
            IsOptimized: IsOptimized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Effect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Effect {}
windows_core::imp::define_interface!(ID3D10EffectBlendVariable, ID3D10EffectBlendVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectBlendVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectBlendVariable, ID3D10EffectVariable);
impl ID3D10EffectBlendVariable {
    pub unsafe fn GetBlendState(&self, index: u32) -> windows_core::Result<ID3D10BlendState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlendState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pblenddesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectBlendVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_BLEND_DESC) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectBlendVariable {}
unsafe impl Sync for ID3D10EffectBlendVariable {}
pub trait ID3D10EffectBlendVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetBlendState(&self, index: u32) -> windows_core::Result<ID3D10BlendState>;
    fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectBlendVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectBlendVariable_Impl>() -> Self {
        unsafe extern "system" fn GetBlendState<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectBlendVariable_Impl::GetBlendState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppblendstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pblenddesc)).into()
            }
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Identity>(), GetBlendState: GetBlendState::<Identity>, GetBackingStore: GetBackingStore::<Identity> }
    }
}
struct ID3D10EffectBlendVariable_ImplVtbl<T: ID3D10EffectBlendVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectBlendVariable_Impl> ID3D10EffectBlendVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectBlendVariable_Vtbl = ID3D10EffectBlendVariable_Vtbl::new::<T>();
}
impl ID3D10EffectBlendVariable {
    pub fn new<'a, T: ID3D10EffectBlendVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectBlendVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectConstantBuffer, ID3D10EffectConstantBuffer_Vtbl);
impl core::ops::Deref for ID3D10EffectConstantBuffer {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectConstantBuffer, ID3D10EffectVariable);
impl ID3D10EffectConstantBuffer {
    pub unsafe fn SetConstantBuffer<P0>(&self, pconstantbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Buffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetConstantBuffer)(windows_core::Interface::as_raw(self), pconstantbuffer.param().abi()).ok() }
    }
    pub unsafe fn GetConstantBuffer(&self) -> windows_core::Result<ID3D10Buffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConstantBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTextureBuffer<P0>(&self, ptexturebuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTextureBuffer)(windows_core::Interface::as_raw(self), ptexturebuffer.param().abi()).ok() }
    }
    pub unsafe fn GetTextureBuffer(&self) -> windows_core::Result<ID3D10ShaderResourceView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextureBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectConstantBuffer_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectConstantBuffer {}
unsafe impl Sync for ID3D10EffectConstantBuffer {}
pub trait ID3D10EffectConstantBuffer_Impl: ID3D10EffectVariable_Impl {
    fn SetConstantBuffer(&self, pconstantbuffer: windows_core::Ref<ID3D10Buffer>) -> windows_core::Result<()>;
    fn GetConstantBuffer(&self) -> windows_core::Result<ID3D10Buffer>;
    fn SetTextureBuffer(&self, ptexturebuffer: windows_core::Ref<ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetTextureBuffer(&self) -> windows_core::Result<ID3D10ShaderResourceView>;
}
impl ID3D10EffectConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D10EffectConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn SetConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pconstantbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::SetConstantBuffer(this, core::mem::transmute_copy(&pconstantbuffer)).into()
            }
        }
        unsafe extern "system" fn GetConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ppconstantbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectConstantBuffer_Impl::GetConstantBuffer(this) {
                    Ok(ok__) => {
                        ppconstantbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ptexturebuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::SetTextureBuffer(this, core::mem::transmute_copy(&ptexturebuffer)).into()
            }
        }
        unsafe extern "system" fn GetTextureBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pptexturebuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectConstantBuffer_Impl::GetTextureBuffer(this) {
                    Ok(ok__) => {
                        pptexturebuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetConstantBuffer: SetConstantBuffer::<Identity>,
            GetConstantBuffer: GetConstantBuffer::<Identity>,
            SetTextureBuffer: SetTextureBuffer::<Identity>,
            GetTextureBuffer: GetTextureBuffer::<Identity>,
        }
    }
}
struct ID3D10EffectConstantBuffer_ImplVtbl<T: ID3D10EffectConstantBuffer_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectConstantBuffer_Impl> ID3D10EffectConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10EffectConstantBuffer_Vtbl = ID3D10EffectConstantBuffer_Vtbl::new::<T>();
}
impl ID3D10EffectConstantBuffer {
    pub fn new<'a, T: ID3D10EffectConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectDepthStencilVariable, ID3D10EffectDepthStencilVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectDepthStencilVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectDepthStencilVariable, ID3D10EffectVariable);
impl ID3D10EffectDepthStencilVariable {
    pub unsafe fn GetDepthStencilState(&self, index: u32) -> windows_core::Result<ID3D10DepthStencilState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencilState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pdepthstencildesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectDepthStencilVariable {}
unsafe impl Sync for ID3D10EffectDepthStencilVariable {}
pub trait ID3D10EffectDepthStencilVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetDepthStencilState(&self, index: u32) -> windows_core::Result<ID3D10DepthStencilState>;
    fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectDepthStencilVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectDepthStencilVariable_Impl>() -> Self {
        unsafe extern "system" fn GetDepthStencilState<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectDepthStencilVariable_Impl::GetDepthStencilState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppdepthstencilstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pdepthstencildesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetDepthStencilState: GetDepthStencilState::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
struct ID3D10EffectDepthStencilVariable_ImplVtbl<T: ID3D10EffectDepthStencilVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectDepthStencilVariable_Impl> ID3D10EffectDepthStencilVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilVariable_Vtbl = ID3D10EffectDepthStencilVariable_Vtbl::new::<T>();
}
impl ID3D10EffectDepthStencilVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectDepthStencilViewVariable, ID3D10EffectDepthStencilViewVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectDepthStencilViewVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectDepthStencilViewVariable, ID3D10EffectVariable);
impl ID3D10EffectDepthStencilViewVariable {
    pub unsafe fn SetDepthStencil<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10DepthStencilView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencil)(windows_core::Interface::as_raw(self), presource.param().abi()).ok() }
    }
    pub unsafe fn GetDepthStencil(&self) -> windows_core::Result<ID3D10DepthStencilView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencil)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDepthStencilArray(&self, ppresources: &[Option<ID3D10DepthStencilView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDepthStencilArray(&self, ppresources: &mut [Option<ID3D10DepthStencilView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectDepthStencilViewVariable {}
unsafe impl Sync for ID3D10EffectDepthStencilViewVariable {}
pub trait ID3D10EffectDepthStencilViewVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetDepthStencil(&self, presource: windows_core::Ref<ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn GetDepthStencil(&self) -> windows_core::Result<ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&self, ppresources: *const Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetDepthStencilArray(&self, ppresources: *mut Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectDepthStencilViewVariable_Impl>() -> Self {
        unsafe extern "system" fn SetDepthStencil<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencil(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencil<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencil(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilArray<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetDepthStencil: SetDepthStencil::<Identity>,
            GetDepthStencil: GetDepthStencil::<Identity>,
            SetDepthStencilArray: SetDepthStencilArray::<Identity>,
            GetDepthStencilArray: GetDepthStencilArray::<Identity>,
        }
    }
}
struct ID3D10EffectDepthStencilViewVariable_ImplVtbl<T: ID3D10EffectDepthStencilViewVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectDepthStencilViewVariable_Impl> ID3D10EffectDepthStencilViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilViewVariable_Vtbl = ID3D10EffectDepthStencilViewVariable_Vtbl::new::<T>();
}
impl ID3D10EffectDepthStencilViewVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectMatrixVariable, ID3D10EffectMatrixVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectMatrixVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectMatrixVariable, ID3D10EffectVariable);
impl ID3D10EffectMatrixVariable {
    pub unsafe fn SetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn GetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrix)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn SetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTranspose)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn GetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTranspose)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectMatrixVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMatrixArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetMatrixArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub SetMatrixTranspose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMatrixTranspose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMatrixTransposeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetMatrixTransposeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectMatrixVariable {}
unsafe impl Sync for ID3D10EffectMatrixVariable {}
pub trait ID3D10EffectMatrixVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectMatrixVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectMatrixVariable_Impl>() -> Self {
        unsafe extern "system" fn SetMatrix<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrix(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetMatrix<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrix(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetMatrixArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetMatrixArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetMatrixTranspose<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrixTranspose(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTranspose<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrixTranspose(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetMatrix: SetMatrix::<Identity>,
            GetMatrix: GetMatrix::<Identity>,
            SetMatrixArray: SetMatrixArray::<Identity>,
            GetMatrixArray: GetMatrixArray::<Identity>,
            SetMatrixTranspose: SetMatrixTranspose::<Identity>,
            GetMatrixTranspose: GetMatrixTranspose::<Identity>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Identity>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Identity>,
        }
    }
}
struct ID3D10EffectMatrixVariable_ImplVtbl<T: ID3D10EffectMatrixVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectMatrixVariable_Impl> ID3D10EffectMatrixVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectMatrixVariable_Vtbl = ID3D10EffectMatrixVariable_Vtbl::new::<T>();
}
impl ID3D10EffectMatrixVariable {
    pub fn new<'a, T: ID3D10EffectMatrixVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectMatrixVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectPass, ID3D10EffectPass_Vtbl);
impl ID3D10EffectPass {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderDesc)(windows_core::Interface::as_raw(self), core::mem::transmute(pdesc)).ok() }
    }
    pub unsafe fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGeometryShaderDesc)(windows_core::Interface::as_raw(self), core::mem::transmute(pdesc)).ok() }
    }
    pub unsafe fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderDesc)(windows_core::Interface::as_raw(self), core::mem::transmute(pdesc)).ok() }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn Apply(&self, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPass_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_DESC) -> windows_core::HRESULT,
    pub GetVertexShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetGeometryShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetPixelShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectPass {}
unsafe impl Sync for ID3D10EffectPass {}
pub trait ID3D10EffectPass_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::Result<()>;
    fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn Apply(&self, flags: u32) -> windows_core::Result<()>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectPass_Vtbl {
    pub const fn new<Identity: ID3D10EffectPass_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetVertexShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetGeometryShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetPixelShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn Apply<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::Apply(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Identity>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Identity>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            Apply: Apply::<Identity>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity>,
        }
    }
}
struct ID3D10EffectPass_ImplVtbl<T: ID3D10EffectPass_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectPass_Impl> ID3D10EffectPass_ImplVtbl<T> {
    const VTABLE: ID3D10EffectPass_Vtbl = ID3D10EffectPass_Vtbl::new::<T>();
}
impl ID3D10EffectPass {
    pub fn new<'a, T: ID3D10EffectPass_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectPass_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectPool, ID3D10EffectPool_Vtbl, 0x9537ab04_3250_412e_8213_fcd2f8677933);
windows_core::imp::interface_hierarchy!(ID3D10EffectPool, windows_core::IUnknown);
impl ID3D10EffectPool {
    pub unsafe fn AsEffect(&self) -> Option<ID3D10Effect> {
        unsafe { (windows_core::Interface::vtable(self).AsEffect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsEffect: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10Effect>,
}
unsafe impl Send for ID3D10EffectPool {}
unsafe impl Sync for ID3D10EffectPool {}
pub trait ID3D10EffectPool_Impl: windows_core::IUnknownImpl {
    fn AsEffect(&self) -> Option<ID3D10Effect>;
}
impl ID3D10EffectPool_Vtbl {
    pub const fn new<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsEffect<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<ID3D10Effect> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10EffectPool_Impl::AsEffect(this)
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsEffect: AsEffect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10EffectPool as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10EffectPool {}
windows_core::imp::define_interface!(ID3D10EffectRasterizerVariable, ID3D10EffectRasterizerVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectRasterizerVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectRasterizerVariable, ID3D10EffectVariable);
impl ID3D10EffectRasterizerVariable {
    pub unsafe fn GetRasterizerState(&self, index: u32) -> windows_core::Result<ID3D10RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterizerState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, prasterizerdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRasterizerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_RASTERIZER_DESC) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectRasterizerVariable {}
unsafe impl Sync for ID3D10EffectRasterizerVariable {}
pub trait ID3D10EffectRasterizerVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetRasterizerState(&self, index: u32) -> windows_core::Result<ID3D10RasterizerState>;
    fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectRasterizerVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectRasterizerVariable_Impl>() -> Self {
        unsafe extern "system" fn GetRasterizerState<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectRasterizerVariable_Impl::GetRasterizerState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprasterizerstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&prasterizerdesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetRasterizerState: GetRasterizerState::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
struct ID3D10EffectRasterizerVariable_ImplVtbl<T: ID3D10EffectRasterizerVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectRasterizerVariable_Impl> ID3D10EffectRasterizerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRasterizerVariable_Vtbl = ID3D10EffectRasterizerVariable_Vtbl::new::<T>();
}
impl ID3D10EffectRasterizerVariable {
    pub fn new<'a, T: ID3D10EffectRasterizerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRasterizerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectRenderTargetViewVariable, ID3D10EffectRenderTargetViewVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectRenderTargetViewVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectRenderTargetViewVariable, ID3D10EffectVariable);
impl ID3D10EffectRenderTargetViewVariable {
    pub unsafe fn SetRenderTarget<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10RenderTargetView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), presource.param().abi()).ok() }
    }
    pub unsafe fn GetRenderTarget(&self) -> windows_core::Result<ID3D10RenderTargetView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRenderTargetArray(&self, ppresources: &[Option<ID3D10RenderTargetView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetRenderTargetArray(&self, ppresources: &mut [Option<ID3D10RenderTargetView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectRenderTargetViewVariable {}
unsafe impl Sync for ID3D10EffectRenderTargetViewVariable {}
pub trait ID3D10EffectRenderTargetViewVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetRenderTarget(&self, presource: windows_core::Ref<ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self) -> windows_core::Result<ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&self, ppresources: *const Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetRenderTargetArray(&self, ppresources: *mut Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectRenderTargetViewVariable_Impl>() -> Self {
        unsafe extern "system" fn SetRenderTarget<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTarget(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTarget(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetArray<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetRenderTarget: SetRenderTarget::<Identity>,
            GetRenderTarget: GetRenderTarget::<Identity>,
            SetRenderTargetArray: SetRenderTargetArray::<Identity>,
            GetRenderTargetArray: GetRenderTargetArray::<Identity>,
        }
    }
}
struct ID3D10EffectRenderTargetViewVariable_ImplVtbl<T: ID3D10EffectRenderTargetViewVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectRenderTargetViewVariable_Impl> ID3D10EffectRenderTargetViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRenderTargetViewVariable_Vtbl = ID3D10EffectRenderTargetViewVariable_Vtbl::new::<T>();
}
impl ID3D10EffectRenderTargetViewVariable {
    pub fn new<'a, T: ID3D10EffectRenderTargetViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRenderTargetViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectSamplerVariable, ID3D10EffectSamplerVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectSamplerVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectSamplerVariable, ID3D10EffectVariable);
impl ID3D10EffectSamplerVariable {
    pub unsafe fn GetSampler(&self, index: u32) -> windows_core::Result<ID3D10SamplerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampler)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackingStore(&self, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, psamplerdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectSamplerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetSampler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SAMPLER_DESC) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectSamplerVariable {}
unsafe impl Sync for ID3D10EffectSamplerVariable {}
pub trait ID3D10EffectSamplerVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetSampler(&self, index: u32) -> windows_core::Result<ID3D10SamplerState>;
    fn GetBackingStore(&self, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectSamplerVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectSamplerVariable_Impl>() -> Self {
        unsafe extern "system" fn GetSampler<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppsampler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectSamplerVariable_Impl::GetSampler(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppsampler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&psamplerdesc)).into()
            }
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Identity>(), GetSampler: GetSampler::<Identity>, GetBackingStore: GetBackingStore::<Identity> }
    }
}
struct ID3D10EffectSamplerVariable_ImplVtbl<T: ID3D10EffectSamplerVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectSamplerVariable_Impl> ID3D10EffectSamplerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectSamplerVariable_Vtbl = ID3D10EffectSamplerVariable_Vtbl::new::<T>();
}
impl ID3D10EffectSamplerVariable {
    pub fn new<'a, T: ID3D10EffectSamplerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectSamplerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectScalarVariable, ID3D10EffectScalarVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectScalarVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectScalarVariable, ID3D10EffectVariable);
impl ID3D10EffectScalarVariable {
    pub unsafe fn SetFloat(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFloat)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetFloat(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFloat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFloatArray(&self, pdata: &[f32], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetFloatArray(&self, pdata: &mut [f32], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetInt(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInt)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetInt(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIntArray(&self, pdata: &[i32], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetIntArray(&self, pdata: &mut [i32], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetBool(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBool)(windows_core::Interface::as_raw(self), value.into()).ok() }
    }
    pub unsafe fn GetBool(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBool)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoolArray(&self, pdata: &[windows_core::BOOL], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetBoolArray(&self, pdata: &mut [windows_core::BOOL], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectScalarVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetFloat: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetFloat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetFloatArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub GetFloatArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub SetInt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetInt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIntArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32, u32) -> windows_core::HRESULT,
    pub GetIntArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub SetBool: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectScalarVariable {}
unsafe impl Sync for ID3D10EffectScalarVariable {}
pub trait ID3D10EffectScalarVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetFloat(&self, value: f32) -> windows_core::Result<()>;
    fn GetFloat(&self) -> windows_core::Result<f32>;
    fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetInt(&self, value: i32) -> windows_core::Result<()>;
    fn GetInt(&self) -> windows_core::Result<i32>;
    fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetBool(&self, value: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBool(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetBoolArray(&self, pdata: *const windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectScalarVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectScalarVariable_Impl>() -> Self {
        unsafe extern "system" fn SetFloat<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetFloat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFloat<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetFloat(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFloatArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetFloatArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetInt<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetInt(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetInt<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetInt(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIntArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetIntArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetBool<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetBool(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBool<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetBool(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoolArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetBoolArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetFloat: SetFloat::<Identity>,
            GetFloat: GetFloat::<Identity>,
            SetFloatArray: SetFloatArray::<Identity>,
            GetFloatArray: GetFloatArray::<Identity>,
            SetInt: SetInt::<Identity>,
            GetInt: GetInt::<Identity>,
            SetIntArray: SetIntArray::<Identity>,
            GetIntArray: GetIntArray::<Identity>,
            SetBool: SetBool::<Identity>,
            GetBool: GetBool::<Identity>,
            SetBoolArray: SetBoolArray::<Identity>,
            GetBoolArray: GetBoolArray::<Identity>,
        }
    }
}
struct ID3D10EffectScalarVariable_ImplVtbl<T: ID3D10EffectScalarVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectScalarVariable_Impl> ID3D10EffectScalarVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectScalarVariable_Vtbl = ID3D10EffectScalarVariable_Vtbl::new::<T>();
}
impl ID3D10EffectScalarVariable {
    pub fn new<'a, T: ID3D10EffectScalarVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectScalarVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectShaderResourceVariable, ID3D10EffectShaderResourceVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectShaderResourceVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectShaderResourceVariable, ID3D10EffectVariable);
impl ID3D10EffectShaderResourceVariable {
    pub unsafe fn SetResource<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetResource)(windows_core::Interface::as_raw(self), presource.param().abi()).ok() }
    }
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D10ShaderResourceView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetResourceArray(&self, ppresources: &[Option<ID3D10ShaderResourceView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetResourceArray(&self, ppresources: &mut [Option<ID3D10ShaderResourceView>], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderResourceVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectShaderResourceVariable {}
unsafe impl Sync for ID3D10EffectShaderResourceVariable {}
pub trait ID3D10EffectShaderResourceVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetResource(&self, presource: windows_core::Ref<ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetResource(&self) -> windows_core::Result<ID3D10ShaderResourceView>;
    fn SetResourceArray(&self, ppresources: *const Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetResourceArray(&self, ppresources: *mut Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectShaderResourceVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectShaderResourceVariable_Impl>() -> Self {
        unsafe extern "system" fn SetResource<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::SetResource(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetResource<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderResourceVariable_Impl::GetResource(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResourceArray<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::SetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetResourceArray<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetResource: SetResource::<Identity>,
            GetResource: GetResource::<Identity>,
            SetResourceArray: SetResourceArray::<Identity>,
            GetResourceArray: GetResourceArray::<Identity>,
        }
    }
}
struct ID3D10EffectShaderResourceVariable_ImplVtbl<T: ID3D10EffectShaderResourceVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectShaderResourceVariable_Impl> ID3D10EffectShaderResourceVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderResourceVariable_Vtbl = ID3D10EffectShaderResourceVariable_Vtbl::new::<T>();
}
impl ID3D10EffectShaderResourceVariable {
    pub fn new<'a, T: ID3D10EffectShaderResourceVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderResourceVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectShaderVariable, ID3D10EffectShaderVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectShaderVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectShaderVariable, ID3D10EffectVariable);
impl ID3D10EffectShaderVariable {
    pub unsafe fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetShaderDesc)(windows_core::Interface::as_raw(self), shaderindex, pdesc as _).ok() }
    }
    pub unsafe fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10VertexShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10GeometryShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometryShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10PixelShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputSignatureElementDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputSignatureElementDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputSignatureElementDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputSignatureElementDesc: usize,
}
unsafe impl Send for ID3D10EffectShaderVariable {}
unsafe impl Sync for ID3D10EffectShaderVariable {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10EffectShaderVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::Result<()>;
    fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10VertexShader>;
    fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10GeometryShader>;
    fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectShaderVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectShaderVariable_Impl>() -> Self {
        unsafe extern "system" fn GetShaderDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetShaderDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetVertexShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppvs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGeometryShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetGeometryShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppgs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetPixelShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetInputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetOutputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetShaderDesc: GetShaderDesc::<Identity>,
            GetVertexShader: GetVertexShader::<Identity>,
            GetGeometryShader: GetGeometryShader::<Identity>,
            GetPixelShader: GetPixelShader::<Identity>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Identity>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10EffectShaderVariable_ImplVtbl<T: ID3D10EffectShaderVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10EffectShaderVariable_Impl> ID3D10EffectShaderVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderVariable_Vtbl = ID3D10EffectShaderVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectShaderVariable {
    pub fn new<'a, T: ID3D10EffectShaderVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectStringVariable, ID3D10EffectStringVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectStringVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectStringVariable, ID3D10EffectVariable);
impl ID3D10EffectStringVariable {
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::PCSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringArray(&self, ppstrings: &mut [windows_core::PCSTR], offset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStringArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppstrings.as_ptr()), offset, ppstrings.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectStringVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR) -> windows_core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectStringVariable {}
unsafe impl Sync for ID3D10EffectStringVariable {}
pub trait ID3D10EffectStringVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetString(&self) -> windows_core::Result<windows_core::PCSTR>;
    fn GetStringArray(&self, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectStringVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectStringVariable_Impl>() -> Self {
        unsafe extern "system" fn GetString<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstring: *mut windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectStringVariable_Impl::GetString(this) {
                    Ok(ok__) => {
                        ppstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringArray<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetStringArray(this, core::mem::transmute_copy(&ppstrings), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Identity>(), GetString: GetString::<Identity>, GetStringArray: GetStringArray::<Identity> }
    }
}
struct ID3D10EffectStringVariable_ImplVtbl<T: ID3D10EffectStringVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectStringVariable_Impl> ID3D10EffectStringVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectStringVariable_Vtbl = ID3D10EffectStringVariable_Vtbl::new::<T>();
}
impl ID3D10EffectStringVariable {
    pub fn new<'a, T: ID3D10EffectStringVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectStringVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectTechnique, ID3D10EffectTechnique_Vtbl);
impl ID3D10EffectTechnique {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass> {
        unsafe { (windows_core::Interface::vtable(self).GetPassByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetPassByName<P0>(&self, name: P0) -> Option<ID3D10EffectPass>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPassByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectTechnique_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetPassByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectPass>,
    pub GetPassByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectPass>,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectTechnique {}
unsafe impl Sync for ID3D10EffectTechnique {}
pub trait ID3D10EffectTechnique_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass>;
    fn GetPassByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectTechnique_Vtbl {
    pub const fn new<Identity: ID3D10EffectTechnique_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetPassByIndex<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectPass> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetPassByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetPassByName<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectPass> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetPassByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetPassByIndex: GetPassByIndex::<Identity>,
            GetPassByName: GetPassByName::<Identity>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity>,
        }
    }
}
struct ID3D10EffectTechnique_ImplVtbl<T: ID3D10EffectTechnique_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectTechnique_Impl> ID3D10EffectTechnique_ImplVtbl<T> {
    const VTABLE: ID3D10EffectTechnique_Vtbl = ID3D10EffectTechnique_Vtbl::new::<T>();
}
impl ID3D10EffectTechnique {
    pub fn new<'a, T: ID3D10EffectTechnique_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectTechnique_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectType, ID3D10EffectType_Vtbl);
impl ID3D10EffectType {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D10EffectType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberTypeBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetMemberName(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberSemantic)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectType_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectType>,
    pub GetMemberTypeBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectType>,
    pub GetMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub GetMemberSemantic: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
}
unsafe impl Send for ID3D10EffectType {}
unsafe impl Sync for ID3D10EffectType {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10EffectType_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberName(&self, index: u32) -> windows_core::PCSTR;
    fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectType_Vtbl {
    pub const fn new<Identity: ID3D10EffectType_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberName(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberSemantic<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberSemantic(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Identity>,
            GetMemberName: GetMemberName::<Identity>,
            GetMemberSemantic: GetMemberSemantic::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10EffectType_ImplVtbl<T: ID3D10EffectType_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10EffectType_Impl> ID3D10EffectType_ImplVtbl<T> {
    const VTABLE: ID3D10EffectType_Vtbl = ID3D10EffectType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectType {
    pub fn new<'a, T: ID3D10EffectType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectVariable, ID3D10EffectVariable_Vtbl);
impl ID3D10EffectVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount).ok() }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVariable_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectVariable {}
unsafe impl Sync for ID3D10EffectVariable {}
pub trait ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
        }
    }
}
struct ID3D10EffectVariable_ImplVtbl<T: ID3D10EffectVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectVariable_Impl> ID3D10EffectVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVariable_Vtbl = ID3D10EffectVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVariable {
    pub fn new<'a, T: ID3D10EffectVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectVectorVariable, ID3D10EffectVectorVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectVectorVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectVectorVariable, ID3D10EffectVariable);
impl ID3D10EffectVectorVariable {
    pub unsafe fn SetBoolVector(&self, pdata: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoolVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn SetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIntVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn SetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFloatVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn GetBoolVector(&self, pdata: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBoolVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn GetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIntVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn GetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFloatVector)(windows_core::Interface::as_raw(self), pdata as _).ok() }
    }
    pub unsafe fn SetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIntVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn GetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIntVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
    pub unsafe fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVectorVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub SetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub SetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub GetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10EffectVectorVariable {}
unsafe impl Sync for ID3D10EffectVectorVariable {}
pub trait ID3D10EffectVectorVariable_Impl: ID3D10EffectVariable_Impl {
    fn SetBoolVector(&self, pdata: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()>;
    fn SetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetBoolVector(&self, pdata: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()>;
    fn GetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVectorVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectVectorVariable_Impl>() -> Self {
        unsafe extern "system" fn SetBoolVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetBoolVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetIntVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetIntVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetFloatVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetFloatVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetBoolVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetBoolVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetIntVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetIntVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetFloatVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetFloatVector(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetBoolVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetIntVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetFloatVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetBoolVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetIntVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetFloatVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            SetBoolVector: SetBoolVector::<Identity>,
            SetIntVector: SetIntVector::<Identity>,
            SetFloatVector: SetFloatVector::<Identity>,
            GetBoolVector: GetBoolVector::<Identity>,
            GetIntVector: GetIntVector::<Identity>,
            GetFloatVector: GetFloatVector::<Identity>,
            SetBoolVectorArray: SetBoolVectorArray::<Identity>,
            SetIntVectorArray: SetIntVectorArray::<Identity>,
            SetFloatVectorArray: SetFloatVectorArray::<Identity>,
            GetBoolVectorArray: GetBoolVectorArray::<Identity>,
            GetIntVectorArray: GetIntVectorArray::<Identity>,
            GetFloatVectorArray: GetFloatVectorArray::<Identity>,
        }
    }
}
struct ID3D10EffectVectorVariable_ImplVtbl<T: ID3D10EffectVectorVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectVectorVariable_Impl> ID3D10EffectVectorVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVectorVariable_Vtbl = ID3D10EffectVectorVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVectorVariable {
    pub fn new<'a, T: ID3D10EffectVectorVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVectorVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10GeometryShader, ID3D10GeometryShader_Vtbl, 0x6316be88_54cd_4040_ab44_20461bc81f68);
impl core::ops::Deref for ID3D10GeometryShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10GeometryShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10GeometryShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
unsafe impl Send for ID3D10GeometryShader {}
unsafe impl Sync for ID3D10GeometryShader {}
pub trait ID3D10GeometryShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10GeometryShader_Vtbl {
    pub const fn new<Identity: ID3D10GeometryShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10GeometryShader {}
windows_core::imp::define_interface!(ID3D10InfoQueue, ID3D10InfoQueue_Vtbl, 0x1b940b17_2642_4d1f_ab1f_b99bad0c395f);
windows_core::imp::interface_hierarchy!(ID3D10InfoQueue, windows_core::IUnknown);
impl ID3D10InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), messagecountlimit).ok() }
    }
    pub unsafe fn ClearStoredMessages(&self) {
        unsafe { (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: Option<*mut D3D10_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), messageindex, pmessage.unwrap_or(core::mem::zeroed()) as _, pmessagebytelength as _).ok() }
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessages)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMessageCountLimit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok() }
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: Option<*mut D3D10_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilter)(windows_core::Interface::as_raw(self), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _).ok() }
    }
    pub unsafe fn ClearStorageFilter(&self) {
        unsafe { (windows_core::Interface::vtable(self).ClearStorageFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyStorageFilter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), pfilter).ok() }
    }
    pub unsafe fn PopStorageFilter(&self) {
        unsafe { (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok() }
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: Option<*mut D3D10_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilter)(windows_core::Interface::as_raw(self), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _).ok() }
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        unsafe { (windows_core::Interface::vtable(self).ClearRetrievalFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), pfilter).ok() }
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        unsafe { (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddMessage<P3>(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), category, severity, id, pdescription.param().abi()).ok() }
    }
    pub unsafe fn AddApplicationMessage<P1>(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()).ok() }
    }
    pub unsafe fn SetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), category, benable.into()).ok() }
    }
    pub unsafe fn SetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity, benable.into()).ok() }
    }
    pub unsafe fn SetBreakOnID(&self, id: D3D10_MESSAGE_ID, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), id, benable.into()).ok() }
    }
    pub unsafe fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), category) }
    }
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity) }
    }
    pub unsafe fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnID)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn SetMuteDebugOutput(&self, bmute: bool) {
        unsafe { (windows_core::Interface::vtable(self).SetMuteDebugOutput)(windows_core::Interface::as_raw(self), bmute.into()) }
    }
    pub unsafe fn GetMuteDebugOutput(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuteDebugOutput)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut D3D10_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_CATEGORY, D3D10_MESSAGE_SEVERITY, D3D10_MESSAGE_ID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_CATEGORY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_SEVERITY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_ID, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_CATEGORY) -> windows_core::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_SEVERITY) -> windows_core::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_ID) -> windows_core::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for ID3D10InfoQueue {}
unsafe impl Sync for ID3D10InfoQueue {}
pub trait ID3D10InfoQueue_Impl: windows_core::IUnknownImpl {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, id: D3D10_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> windows_core::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> windows_core::BOOL;
    fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> windows_core::BOOL;
    fn SetMuteDebugOutput(&self, bmute: windows_core::BOOL);
    fn GetMuteDebugOutput(&self) -> windows_core::BOOL;
}
impl ID3D10InfoQueue_Vtbl {
    pub const fn new<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagecountlimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute_copy(&messagecountlimit)).into()
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::ClearStoredMessages(this)
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetMessage(this, core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this)
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this)
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetNumStoredMessages(this)
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this)
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this)
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetMessageCountLimit(this)
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetStorageFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::ClearStorageFilter(this)
            }
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushEmptyStorageFilter(this).into()
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushCopyOfStorageFilter(this).into()
            }
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushStorageFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PopStorageFilter(this)
            }
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetStorageFilterStackSize(this)
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::ClearRetrievalFilter(this)
            }
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushEmptyRetrievalFilter(this).into()
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushCopyOfRetrievalFilter(this).into()
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::PopRetrievalFilter(this)
            }
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetRetrievalFilterStackSize(this)
            }
        }
        unsafe extern "system" fn AddMessage<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::AddMessage(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::SetBreakOnID(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute_copy(&category))
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute_copy(&severity))
            }
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D10_MESSAGE_ID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetBreakOnID(this, core::mem::transmute_copy(&id))
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute_copy(&bmute))
            }
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ID3D10InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10InfoQueue_Impl::GetMuteDebugOutput(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10InfoQueue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10InfoQueue {}
windows_core::imp::define_interface!(ID3D10InputLayout, ID3D10InputLayout_Vtbl, 0x9b7e4c0b_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10InputLayout {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10InputLayout, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InputLayout_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
unsafe impl Send for ID3D10InputLayout {}
unsafe impl Sync for ID3D10InputLayout {}
pub trait ID3D10InputLayout_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10InputLayout_Vtbl {
    pub const fn new<Identity: ID3D10InputLayout_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10InputLayout as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10InputLayout {}
windows_core::imp::define_interface!(ID3D10Multithread, ID3D10Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10Multithread, windows_core::IUnknown);
impl ID3D10Multithread {
    pub unsafe fn Enter(&self) {
        unsafe { (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Leave(&self) {
        unsafe { (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMultithreadProtected(&self, bmtprotect: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SetMultithreadProtected)(windows_core::Interface::as_raw(self), bmtprotect.into()) }
    }
    pub unsafe fn GetMultithreadProtected(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for ID3D10Multithread {}
unsafe impl Sync for ID3D10Multithread {}
pub trait ID3D10Multithread_Impl: windows_core::IUnknownImpl {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: windows_core::BOOL) -> windows_core::BOOL;
    fn GetMultithreadProtected(&self) -> windows_core::BOOL;
}
impl ID3D10Multithread_Vtbl {
    pub const fn new<Identity: ID3D10Multithread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enter<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::Enter(this)
            }
        }
        unsafe extern "system" fn Leave<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::Leave(this)
            }
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmtprotect: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::SetMultithreadProtected(this, core::mem::transmute_copy(&bmtprotect))
            }
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::GetMultithreadProtected(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Multithread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Multithread {}
windows_core::imp::define_interface!(ID3D10PixelShader, ID3D10PixelShader_Vtbl, 0x4968b601_9d00_4cde_8346_8e7f675819b6);
impl core::ops::Deref for ID3D10PixelShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10PixelShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10PixelShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
unsafe impl Send for ID3D10PixelShader {}
unsafe impl Sync for ID3D10PixelShader {}
pub trait ID3D10PixelShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10PixelShader_Vtbl {
    pub const fn new<Identity: ID3D10PixelShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10PixelShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10PixelShader {}
windows_core::imp::define_interface!(ID3D10Predicate, ID3D10Predicate_Vtbl, 0x9b7e4c10_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Predicate {
    type Target = ID3D10Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Predicate, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous, ID3D10Query);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Predicate_Vtbl {
    pub base__: ID3D10Query_Vtbl,
}
unsafe impl Send for ID3D10Predicate {}
unsafe impl Sync for ID3D10Predicate {}
pub trait ID3D10Predicate_Impl: ID3D10Query_Impl {}
impl ID3D10Predicate_Vtbl {
    pub const fn new<Identity: ID3D10Predicate_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10Query_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Predicate as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10Query as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Predicate {}
windows_core::imp::define_interface!(ID3D10Query, ID3D10Query_Vtbl, 0x9b7e4c0e_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Query {
    type Target = ID3D10Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Query, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous);
impl ID3D10Query {
    pub unsafe fn GetDesc(&self) -> D3D10_QUERY_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Query_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_QUERY_DESC),
}
unsafe impl Send for ID3D10Query {}
unsafe impl Sync for ID3D10Query {}
pub trait ID3D10Query_Impl: ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC);
}
impl ID3D10Query_Vtbl {
    pub const fn new<Identity: ID3D10Query_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10Query_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Query_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Query as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Query {}
windows_core::imp::define_interface!(ID3D10RasterizerState, ID3D10RasterizerState_Vtbl, 0xa2a07292_89af_4345_be2e_c53d9fbb6e9f);
impl core::ops::Deref for ID3D10RasterizerState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10RasterizerState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10RasterizerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RasterizerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RASTERIZER_DESC),
}
unsafe impl Send for ID3D10RasterizerState {}
unsafe impl Sync for ID3D10RasterizerState {}
pub trait ID3D10RasterizerState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC);
}
impl ID3D10RasterizerState_Vtbl {
    pub const fn new<Identity: ID3D10RasterizerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10RasterizerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10RasterizerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10RasterizerState {}
windows_core::imp::define_interface!(ID3D10RenderTargetView, ID3D10RenderTargetView_Vtbl, 0x9b7e4c08_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10RenderTargetView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10RenderTargetView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10RenderTargetView {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RenderTargetView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10RenderTargetView {}
unsafe impl Sync for ID3D10RenderTargetView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10RenderTargetView_Vtbl {
    pub const fn new<Identity: ID3D10RenderTargetView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10RenderTargetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10RenderTargetView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10RenderTargetView {}
windows_core::imp::define_interface!(ID3D10Resource, ID3D10Resource_Vtbl, 0x9b7e4c01_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Resource {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Resource, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10Resource {
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        unsafe { (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority) }
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Resource_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
unsafe impl Send for ID3D10Resource {}
unsafe impl Sync for ID3D10Resource {}
pub trait ID3D10Resource_Impl: ID3D10DeviceChild_Impl {
    fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ID3D10Resource_Vtbl {
    pub const fn new<Identity: ID3D10Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::GetType(this, core::mem::transmute_copy(&rtype))
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority))
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::GetEvictionPriority(this)
            }
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Resource as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Resource {}
windows_core::imp::define_interface!(ID3D10SamplerState, ID3D10SamplerState_Vtbl, 0x9b7e4c0c_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10SamplerState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10SamplerState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10SamplerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SamplerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SAMPLER_DESC),
}
unsafe impl Send for ID3D10SamplerState {}
unsafe impl Sync for ID3D10SamplerState {}
pub trait ID3D10SamplerState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl ID3D10SamplerState_Vtbl {
    pub const fn new<Identity: ID3D10SamplerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10SamplerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10SamplerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10SamplerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10SamplerState {}
windows_core::imp::define_interface!(ID3D10ShaderReflection, ID3D10ShaderReflection_Vtbl, 0xd40e20b6_f8f7_42ad_ab20_4baf8f15dfaa);
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection, windows_core::IUnknown);
impl ID3D10ShaderReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
}
unsafe impl Send for ID3D10ShaderReflection {}
unsafe impl Sync for ID3D10ShaderReflection {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection_Impl: windows_core::IUnknownImpl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflection_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D10ShaderReflection {}
windows_core::imp::define_interface!(ID3D10ShaderReflection1, ID3D10ShaderReflection1_Vtbl, 0xc3457783_a846_47ce_9520_cea6f66e7447);
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection1, windows_core::IUnknown);
impl ID3D10ShaderReflection1 {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _).ok() }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc as _).ok() }
    }
    pub unsafe fn GetMovInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMovInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMovcInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitwiseInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> windows_core::Result<super::Direct3D::D3D_PRIMITIVE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGSInputPrimitive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLevel9Shader(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLevel9Shader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSampleFrequencyShader(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSampleFrequencyShader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMovcInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConversionInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetGSInputPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Direct3D::D3D_PRIMITIVE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetGSInputPrimitive: usize,
    pub IsLevel9Shader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsSampleFrequencyShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10ShaderReflection1 {}
unsafe impl Sync for ID3D10ShaderReflection1 {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection1_Impl: windows_core::IUnknownImpl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetMovcInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetConversionInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetGSInputPrimitive(&self) -> windows_core::Result<super::Direct3D::D3D_PRIMITIVE>;
    fn IsLevel9Shader(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsSampleFrequencyShader(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflection1_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetMovInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetMovcInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetConversionInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetBitwiseInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetGSInputPrimitive(this) {
                    Ok(ok__) => {
                        pprim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblevel9shader: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::IsLevel9Shader(this) {
                    Ok(ok__) => {
                        pblevel9shader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsamplefrequency: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::IsSampleFrequencyShader(this) {
                    Ok(ok__) => {
                        pbsamplefrequency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Identity, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D10ShaderReflection1 {}
windows_core::imp::define_interface!(ID3D10ShaderReflectionConstantBuffer, ID3D10ShaderReflectionConstantBuffer_Vtbl);
impl ID3D10ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>,
}
unsafe impl Send for ID3D10ShaderReflectionConstantBuffer {}
unsafe impl Sync for ID3D10ShaderReflectionConstantBuffer {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionConstantBuffer_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetVariableByIndex: GetVariableByIndex::<Identity>, GetVariableByName: GetVariableByName::<Identity> }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D10ShaderReflectionConstantBuffer_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionConstantBuffer_Impl> ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionConstantBuffer_Vtbl = ID3D10ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D10ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionType, ID3D10ShaderReflectionType_Vtbl);
impl ID3D10ShaderReflectionType {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionType> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeName)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
}
unsafe impl Send for ID3D10ShaderReflectionType {}
unsafe impl Sync for ID3D10ShaderReflectionType {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionType_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionType_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            GetDesc: GetDesc::<Identity>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity>,
            GetMemberTypeName: GetMemberTypeName::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionType_ImplVtbl<T: ID3D10ShaderReflectionType_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionType_Impl> ID3D10ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionType_Vtbl = ID3D10ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType {
    pub fn new<'a, T: ID3D10ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionVariable, ID3D10ShaderReflectionVariable_Vtbl);
impl ID3D10ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10ShaderReflectionType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType>,
}
unsafe impl Send for ID3D10ShaderReflectionVariable {}
unsafe impl Sync for ID3D10ShaderReflectionVariable {}
pub trait ID3D10ShaderReflectionVariable_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D10ShaderReflectionType>;
}
impl ID3D10ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionVariable_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionVariable_Impl::GetType(this)
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetType: GetType::<Identity> }
    }
}
struct ID3D10ShaderReflectionVariable_ImplVtbl<T: ID3D10ShaderReflectionVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10ShaderReflectionVariable_Impl> ID3D10ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionVariable_Vtbl = ID3D10ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D10ShaderReflectionVariable {
    pub fn new<'a, T: ID3D10ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10ShaderResourceView, ID3D10ShaderResourceView_Vtbl, 0x9b7e4c07_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10ShaderResourceView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10ShaderResourceView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10ShaderResourceView {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10ShaderResourceView {}
unsafe impl Sync for ID3D10ShaderResourceView {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView_Vtbl {
    pub const fn new<Identity: ID3D10ShaderResourceView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderResourceView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderResourceView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView {}
windows_core::imp::define_interface!(ID3D10ShaderResourceView1, ID3D10ShaderResourceView1_Vtbl, 0x9b7e4c87_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10ShaderResourceView1 {
    type Target = ID3D10ShaderResourceView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10ShaderResourceView1, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View, ID3D10ShaderResourceView);
impl ID3D10ShaderResourceView1 {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView1_Vtbl {
    pub base__: ID3D10ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
}
unsafe impl Send for ID3D10ShaderResourceView1 {}
unsafe impl Sync for ID3D10ShaderResourceView1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1_Impl: ID3D10ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderResourceView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self { base__: ID3D10ShaderResourceView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID || iid == &<ID3D10ShaderResourceView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView1 {}
windows_core::imp::define_interface!(ID3D10StateBlock, ID3D10StateBlock_Vtbl, 0x0803425a_57f5_4dd6_9465_a87570834a08);
windows_core::imp::interface_hierarchy!(ID3D10StateBlock, windows_core::IUnknown);
impl ID3D10StateBlock {
    pub unsafe fn Capture(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseAllDeviceObjects(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseAllDeviceObjects)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10StateBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseAllDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D10StateBlock {}
unsafe impl Sync for ID3D10StateBlock {}
pub trait ID3D10StateBlock_Impl: windows_core::IUnknownImpl {
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn ReleaseAllDeviceObjects(&self) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<ID3D10Device>;
}
impl ID3D10StateBlock_Vtbl {
    pub const fn new<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capture<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::Capture(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::Apply(this).into()
            }
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::ReleaseAllDeviceObjects(this).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10StateBlock_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10StateBlock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10StateBlock {}
windows_core::imp::define_interface!(ID3D10SwitchToRef, ID3D10SwitchToRef_Vtbl, 0x9b7e4e02_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10SwitchToRef, windows_core::IUnknown);
impl ID3D10SwitchToRef {
    pub unsafe fn SetUseRef(&self, useref: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SetUseRef)(windows_core::Interface::as_raw(self), useref.into()) }
    }
    pub unsafe fn GetUseRef(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetUseRef)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SwitchToRef_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub GetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for ID3D10SwitchToRef {}
unsafe impl Sync for ID3D10SwitchToRef {}
pub trait ID3D10SwitchToRef_Impl: windows_core::IUnknownImpl {
    fn SetUseRef(&self, useref: windows_core::BOOL) -> windows_core::BOOL;
    fn GetUseRef(&self) -> windows_core::BOOL;
}
impl ID3D10SwitchToRef_Vtbl {
    pub const fn new<Identity: ID3D10SwitchToRef_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUseRef<Identity: ID3D10SwitchToRef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useref: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10SwitchToRef_Impl::SetUseRef(this, core::mem::transmute_copy(&useref))
            }
        }
        unsafe extern "system" fn GetUseRef<Identity: ID3D10SwitchToRef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10SwitchToRef_Impl::GetUseRef(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUseRef: SetUseRef::<Identity, OFFSET>,
            GetUseRef: GetUseRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10SwitchToRef as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10SwitchToRef {}
windows_core::imp::define_interface!(ID3D10Texture1D, ID3D10Texture1D_Vtbl, 0x9b7e4c03_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture1D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture1D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture1D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, ppdata as _).ok() }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture1D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10Texture1D {}
unsafe impl Sync for ID3D10Texture1D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture1D_Vtbl {
    pub const fn new<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture1D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture1D {}
windows_core::imp::define_interface!(ID3D10Texture2D, ID3D10Texture2D_Vtbl, 0x9b7e4c04_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture2D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture2D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture2D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE2D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture2D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE2D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10Texture2D {}
unsafe impl Sync for ID3D10Texture2D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture2D_Vtbl {
    pub const fn new<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Texture2D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                    Ok(ok__) => {
                        pmappedtex2d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture2D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture2D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture2D {}
windows_core::imp::define_interface!(ID3D10Texture3D, ID3D10Texture3D_Vtbl, 0x9b7e4c05_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture3D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture3D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture3D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC) {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture3D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
unsafe impl Send for ID3D10Texture3D {}
unsafe impl Sync for ID3D10Texture3D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture3D_Vtbl {
    pub const fn new<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Texture3D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                    Ok(ok__) => {
                        pmappedtex3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture3D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture3D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture3D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture3D {}
windows_core::imp::define_interface!(ID3D10VertexShader, ID3D10VertexShader_Vtbl, 0x9b7e4c0a_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10VertexShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10VertexShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10VertexShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
unsafe impl Send for ID3D10VertexShader {}
unsafe impl Sync for ID3D10VertexShader {}
pub trait ID3D10VertexShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10VertexShader_Vtbl {
    pub const fn new<Identity: ID3D10VertexShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10VertexShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10VertexShader {}
windows_core::imp::define_interface!(ID3D10View, ID3D10View_Vtbl, 0xc902b03f_60a7_49ba_9936_2a3ab37a7e33);
impl core::ops::Deref for ID3D10View {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10View, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10View {
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D10Resource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10View_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
unsafe impl Send for ID3D10View {}
unsafe impl Sync for ID3D10View {}
pub trait ID3D10View_Impl: ID3D10DeviceChild_Impl {
    fn GetResource(&self, ppresource: windows_core::OutRef<ID3D10Resource>);
}
impl ID3D10View_Vtbl {
    pub const fn new<Identity: ID3D10View_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: ID3D10View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10View_Impl::GetResource(this, core::mem::transmute_copy(&ppresource))
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10View as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10View {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_D3D10_CREATE_DEVICE1 = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: windows_core::OutRef<ID3D10Device1>) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::Dxgi::DXGI_SWAP_CHAIN_DESC, param7: windows_core::OutRef<super::Dxgi::IDXGISwapChain>, param8: windows_core::OutRef<ID3D10Device1>) -> windows_core::HRESULT>;
pub const _FACD3D10: u32 = 2169u32;
