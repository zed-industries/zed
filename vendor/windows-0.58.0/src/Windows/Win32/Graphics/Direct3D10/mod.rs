#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileEffectFromMemory<P0, P1>(pdata: *const core::ffi::c_void, datalength: usize, psrcfilename: P0, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P1, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut Option<super::Direct3D::ID3DBlob>, pperrors: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Direct3D::ID3DInclude>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CompileEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, psrcfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, hlslflags : u32, fxflags : u32, ppcompiledeffect : *mut * mut core::ffi::c_void, pperrors : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CompileEffectFromMemory(pdata, datalength, psrcfilename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), hlslflags, fxflags, core::mem::transmute(ppcompiledeffect), core::mem::transmute(pperrors.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CompileShader<P0, P1, P2, P3>(psrcdata: &[u8], pfilename: P0, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P1, pfunctionname: P2, pprofile: P3, flags: u32, ppshader: *mut Option<super::Direct3D::ID3DBlob>, pperrormsgs: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Direct3D::ID3DInclude>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CompileShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, pfunctionname : windows_core::PCSTR, pprofile : windows_core::PCSTR, flags : u32, ppshader : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CompileShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), pfunctionname.param().abi(), pprofile.param().abi(), flags, core::mem::transmute(ppshader), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10CreateBlob(numbytes: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateBlob(numbytes : usize, ppbuffer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10CreateBlob(numbytes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D3D10CreateDevice<P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, sdkversion: u32, ppdevice: Option<*mut Option<ID3D10Device>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateDevice(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CreateDevice(padapter.param().abi(), drivertype, software.param().abi(), flags, sdkversion, core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D3D10CreateDevice1<P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d10_1.dll" "system" fn D3D10CreateDevice1(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CreateDevice1(padapter.param().abi(), drivertype, software.param().abi(), flags, hardwarelevel, sdkversion, core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain<P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, sdkversion: u32, pswapchaindesc: Option<*const super::Dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::Dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D10Device>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateDeviceAndSwapChain(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, sdkversion : u32, pswapchaindesc : *const super::Dxgi:: DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut * mut core::ffi::c_void, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CreateDeviceAndSwapChain(padapter.param().abi(), drivertype, software.param().abi(), flags, sdkversion, core::mem::transmute(pswapchaindesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppswapchain.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain1<P0, P1>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: P1, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: Option<*const super::Dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::Dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d10_1.dll" "system" fn D3D10CreateDeviceAndSwapChain1(padapter : * mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, pswapchaindesc : *const super::Dxgi:: DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut * mut core::ffi::c_void, ppdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10CreateDeviceAndSwapChain1(padapter.param().abi(), drivertype, software.param().abi(), flags, hardwarelevel, sdkversion, core::mem::transmute(pswapchaindesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppswapchain.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3D10CreateEffectFromMemory<P0, P1>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P0, peffectpool: P1) -> windows_core::Result<ID3D10Effect>
where
    P0: windows_core::Param<ID3D10Device>,
    P1: windows_core::Param<ID3D10EffectPool>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : * mut core::ffi::c_void, peffectpool : * mut core::ffi::c_void, ppeffect : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10CreateEffectFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), peffectpool.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10CreateEffectPoolFromMemory<P0>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P0) -> windows_core::Result<ID3D10EffectPool>
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateEffectPoolFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : * mut core::ffi::c_void, ppeffectpool : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10CreateEffectPoolFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10CreateStateBlock<P0>(pdevice: P0, pstateblockmask: *const D3D10_STATE_BLOCK_MASK) -> windows_core::Result<ID3D10StateBlock>
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10CreateStateBlock(pdevice : * mut core::ffi::c_void, pstateblockmask : *const D3D10_STATE_BLOCK_MASK, ppstateblock : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10CreateStateBlock(pdevice.param().abi(), pstateblockmask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10DisassembleEffect<P0, P1>(peffect: P0, enablecolorcode: P1) -> windows_core::Result<super::Direct3D::ID3DBlob>
where
    P0: windows_core::Param<ID3D10Effect>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10DisassembleEffect(peffect : * mut core::ffi::c_void, enablecolorcode : super::super::Foundation:: BOOL, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10DisassembleEffect(peffect.param().abi(), enablecolorcode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10DisassembleShader<P0, P1>(pshader: *const core::ffi::c_void, bytecodelength: usize, enablecolorcode: P0, pcomments: P1) -> windows_core::Result<super::Direct3D::ID3DBlob>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10DisassembleShader(pshader : *const core::ffi::c_void, bytecodelength : usize, enablecolorcode : super::super::Foundation:: BOOL, pcomments : windows_core::PCSTR, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10DisassembleShader(pshader, bytecodelength, enablecolorcode.param().abi(), pcomments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10GetGeometryShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetGeometryShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    D3D10GetGeometryShaderProfile(pdevice.param().abi())
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10GetInputAndOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetInputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetInputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10GetInputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10GetOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10GetPixelShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetPixelShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    D3D10GetPixelShaderProfile(pdevice.param().abi())
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10GetShaderDebugInfo(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::Direct3D::ID3DBlob> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetShaderDebugInfo(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppdebuginfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10GetShaderDebugInfo(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10GetVertexShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<ID3D10Device>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10GetVertexShaderProfile(pdevice : * mut core::ffi::c_void) -> windows_core::PCSTR);
    D3D10GetVertexShaderProfile(pdevice.param().abi())
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D10PreprocessShader<P0, P1>(psrcdata: &[u8], pfilename: P0, pdefines: Option<*const super::Direct3D::D3D_SHADER_MACRO>, pinclude: P1, ppshadertext: *mut Option<super::Direct3D::ID3DBlob>, pperrormsgs: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::Direct3D::ID3DInclude>,
{
    windows_targets::link!("d3d10.dll" "system" fn D3D10PreprocessShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const super::Direct3D:: D3D_SHADER_MACRO, pinclude : * mut core::ffi::c_void, ppshadertext : *mut * mut core::ffi::c_void, pperrormsgs : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D10PreprocessShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), core::mem::transmute(pdefines.unwrap_or(std::ptr::null())), pinclude.param().abi(), core::mem::transmute(ppshadertext), core::mem::transmute(pperrormsgs.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn D3D10ReflectShader(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<ID3D10ShaderReflection> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10ReflectShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppreflector : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3D10ReflectShader(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDifference(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    D3D10StateBlockMaskDifference(pa, pb, presult).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    D3D10StateBlockMaskDisableAll(pmask).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    D3D10StateBlockMaskDisableCapture(pmask, statetype, rangestart, rangelength).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    D3D10StateBlockMaskEnableAll(pmask).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    D3D10StateBlockMaskEnableCapture(pmask, statetype, rangestart, rangelength).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskGetSetting(pmask : *const D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, entry : u32) -> super::super::Foundation:: BOOL);
    D3D10StateBlockMaskGetSetting(pmask, statetype, entry)
}
#[inline]
pub unsafe fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskIntersect(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    D3D10StateBlockMaskIntersect(pa, pb, presult).ok()
}
#[inline]
pub unsafe fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
    windows_targets::link!("d3d10.dll" "system" fn D3D10StateBlockMaskUnion(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    D3D10StateBlockMaskUnion(pa, pb, presult).ok()
}
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
        (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: Option<*mut core::ffi::c_void>, datasize: u32, getdataflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10Asynchronous {}
unsafe impl Sync for ID3D10Asynchronous {}
#[repr(C)]
pub struct ID3D10Asynchronous_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10BlendState {}
unsafe impl Sync for ID3D10BlendState {}
#[repr(C)]
pub struct ID3D10BlendState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC),
}
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
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10BlendState1 {}
unsafe impl Sync for ID3D10BlendState1 {}
#[repr(C)]
pub struct ID3D10BlendState1_Vtbl {
    pub base__: ID3D10BlendState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC1),
}
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
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), maptype, mapflags, ppdata).ok()
    }
    pub unsafe fn Unmap(&self) {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10Buffer {}
unsafe impl Sync for ID3D10Buffer {}
#[repr(C)]
pub struct ID3D10Buffer_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BUFFER_DESC),
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D10Counter {}
unsafe impl Sync for ID3D10Counter {}
#[repr(C)]
pub struct ID3D10Counter_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_COUNTER_DESC),
}
windows_core::imp::define_interface!(ID3D10Debug, ID3D10Debug_Vtbl, 0x9b7e4e01_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Debug {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Debug, windows_core::IUnknown);
impl ID3D10Debug {
    pub unsafe fn SetFeatureMask(&self, mask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFeatureMask)(windows_core::Interface::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFeatureMask)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPresentPerRenderOpDelay)(windows_core::Interface::as_raw(self), milliseconds).ok()
    }
    pub unsafe fn GetPresentPerRenderOpDelay(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetPresentPerRenderOpDelay)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, pswapchain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Dxgi::IDXGISwapChain>,
    {
        (windows_core::Interface::vtable(self).SetSwapChain)(windows_core::Interface::as_raw(self), pswapchain.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSwapChain(&self) -> windows_core::Result<super::Dxgi::IDXGISwapChain> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID3D10Debug {}
unsafe impl Sync for ID3D10Debug {}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10DepthStencilState {}
unsafe impl Sync for ID3D10DepthStencilState {}
#[repr(C)]
pub struct ID3D10DepthStencilState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_DESC),
}
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10DepthStencilView {}
unsafe impl Sync for ID3D10DepthStencilView {}
#[repr(C)]
pub struct ID3D10DepthStencilView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D10Device, ID3D10Device_Vtbl, 0x9b7e4c0f_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Device {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Device, windows_core::IUnknown);
impl ID3D10Device {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).VSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).PSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0)
    where
        P0: windows_core::Param<ID3D10PixelShader>,
    {
        (windows_core::Interface::vtable(self).PSSetShader)(windows_core::Interface::as_raw(self), ppixelshader.param().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).PSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0)
    where
        P0: windows_core::Param<ID3D10VertexShader>,
    {
        (windows_core::Interface::vtable(self).VSSetShader)(windows_core::Interface::as_raw(self), pvertexshader.param().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).PSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D10InputLayout>,
    {
        (windows_core::Interface::vtable(self).IASetInputLayout)(windows_core::Interface::as_raw(self), pinputlayout.param().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*const Option<ID3D10Buffer>>, pstrides: Option<*const u32>, poffsets: Option<*const u32>) {
        (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppvertexbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pstrides.unwrap_or(std::ptr::null())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D10Buffer>,
    {
        (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.param().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).GSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0)
    where
        P0: windows_core::Param<ID3D10GeometryShader>,
    {
        (windows_core::Interface::vtable(self).GSSetShader)(windows_core::Interface::as_raw(self), pshader.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).VSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).VSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: windows_core::Param<ID3D10Predicate>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), ppredicate.param().abi(), predicatevalue.param().abi())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).GSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).GSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: Option<&[Option<ID3D10RenderTargetView>]>, pdepthstencilview: P0)
    where
        P0: windows_core::Param<ID3D10DepthStencilView>,
    {
        (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi())
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: &[f32; 4], samplemask: u32)
    where
        P0: windows_core::Param<ID3D10BlendState>,
    {
        (windows_core::Interface::vtable(self).OMSetBlendState)(windows_core::Interface::as_raw(self), pblendstate.param().abi(), core::mem::transmute(blendfactor.as_ptr()), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D10DepthStencilState>,
    {
        (windows_core::Interface::vtable(self).OMSetDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencilstate.param().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: Option<*const Option<ID3D10Buffer>>, poffsets: Option<*const u32>) {
        (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), numbuffers, core::mem::transmute(ppsotargets.unwrap_or(std::ptr::null())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D10RasterizerState>,
    {
        (windows_core::Interface::vtable(self).RSSetState)(windows_core::Interface::as_raw(self), prasterizerstate.param().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D10_VIEWPORT]>) {
        (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviewports.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[super::super::Foundation::RECT]>) {
        (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: Option<*const D3D10_BOX>)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P1: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).CopySubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P1: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D10_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).UpdateSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, core::mem::transmute(pdstbox.unwrap_or(std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D10RenderTargetView>,
    {
        (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), prendertargetview.param().abi(), core::mem::transmute(colorrgba.as_ptr()))
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: windows_core::Param<ID3D10DepthStencilView>,
    {
        (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), pdepthstencilview.param().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        (windows_core::Interface::vtable(self).GenerateMips)(windows_core::Interface::as_raw(self), pshaderresourceview.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P1: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format)
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).VSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).PSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self) -> windows_core::Result<ID3D10PixelShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).PSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self) -> windows_core::Result<ID3D10VertexShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).PSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D10InputLayout> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IAGetInputLayout)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*mut Option<ID3D10Buffer>>, pstrides: Option<*mut u32>, poffsets: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).IAGetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppvertexbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstrides.unwrap_or(std::ptr::null_mut())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null_mut())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: Option<*mut Option<ID3D10Buffer>>, format: Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).IAGetIndexBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pindexbuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(format.unwrap_or(std::ptr::null_mut())), core::mem::transmute(offset.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        (windows_core::Interface::vtable(self).GSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self) -> windows_core::Result<ID3D10GeometryShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).VSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).VSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GetPredication(&self, pppredicate: Option<*mut Option<ID3D10Predicate>>, ppredicatevalue: Option<*mut super::super::Foundation::BOOL>) {
        (windows_core::Interface::vtable(self).GetPredication)(windows_core::Interface::as_raw(self), core::mem::transmute(pppredicate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppredicatevalue.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).GSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        (windows_core::Interface::vtable(self).GSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: Option<&mut [Option<ID3D10RenderTargetView>]>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) {
        (windows_core::Interface::vtable(self).OMGetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppdepthstencilview.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: Option<*mut Option<ID3D10BlendState>>, blendfactor: Option<&mut [f32; 4]>, psamplemask: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).OMGetBlendState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(psamplemask.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>, pstencilref: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).OMGetDepthStencilState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppdepthstencilstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstencilref.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: Option<*mut Option<ID3D10Buffer>>, poffsets: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).SOGetTargets)(windows_core::Interface::as_raw(self), numbuffers, core::mem::transmute(ppsotargets.unwrap_or(std::ptr::null_mut())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D10RasterizerState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RSGetState)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: Option<*mut D3D10_VIEWPORT>) {
        (windows_core::Interface::vtable(self).RSGetViewports)(windows_core::Interface::as_raw(self), numviewports, core::mem::transmute(pviewports.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: Option<*mut super::super::Foundation::RECT>) {
        (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), numrects, core::mem::transmute(prects.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize, core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, core::mem::transmute(pdata.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const windows_core::GUID, pdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>, ppbuffer: Option<*mut Option<ID3D10Buffer>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBuffer)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(ppbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture1D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTexture1D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture2D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTexture2D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTexture3D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC>, ppsrview: Option<*mut Option<ID3D10ShaderResourceView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppsrview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_RENDER_TARGET_VIEW_DESC>, pprtview: Option<*mut Option<ID3D10RenderTargetView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(pprtview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppdepthstencilview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: Option<*mut Option<ID3D10InputLayout>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateInputLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len().try_into().unwrap(), core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len().try_into().unwrap(), core::mem::transmute(ppinputlayout.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8], ppvertexshader: Option<*mut Option<ID3D10VertexShader>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(ppvertexshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8], ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateGeometryShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(ppgeometryshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: Option<&[D3D10_SO_DECLARATION_ENTRY]>, outputstreamstride: u32, ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(psodeclaration.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psodeclaration.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), outputstreamstride, core::mem::transmute(ppgeometryshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8], pppixelshader: Option<*mut Option<ID3D10PixelShader>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(pppixelshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: Option<*mut Option<ID3D10BlendState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBlendState)(windows_core::Interface::as_raw(self), pblendstatedesc, core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencildesc, core::mem::transmute(ppdepthstencilstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: Option<*mut Option<ID3D10RasterizerState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateRasterizerState)(windows_core::Interface::as_raw(self), prasterizerdesc, core::mem::transmute(pprasterizerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: Option<*mut Option<ID3D10SamplerState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateSamplerState)(windows_core::Interface::as_raw(self), psamplerdesc, core::mem::transmute(ppsamplerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: Option<*mut Option<ID3D10Query>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), pquerydesc, core::mem::transmute(ppquery.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: Option<*mut Option<ID3D10Predicate>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreatePredicate)(windows_core::Interface::as_raw(self), ppredicatedesc, core::mem::transmute(pppredicate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: Option<*mut Option<ID3D10Counter>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateCounter)(windows_core::Interface::as_raw(self), pcounterdesc, core::mem::transmute(ppcounter.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckFormatSupport)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(windows_core::Interface::as_raw(self), format, samplecount, &mut result__).map(|| result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D10_COUNTER_INFO {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCounterInfo)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: Option<*mut u32>, szunits: windows_core::PSTR, punitslength: Option<*mut u32>, szdescription: windows_core::PSTR, pdescriptionlength: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckCounter)(windows_core::Interface::as_raw(self), pdesc, ptype, pactivecounters, core::mem::transmute(szname), core::mem::transmute(pnamelength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szunits), core::mem::transmute(punitslength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szdescription), core::mem::transmute(pdescriptionlength.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OpenSharedResource<P0>(&self, hresource: P0, returnedinterface: *const windows_core::GUID, ppresource: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource.param().abi(), returnedinterface, core::mem::transmute(ppresource.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (windows_core::Interface::vtable(self).SetTextFilterSize)(windows_core::Interface::as_raw(self), width, height)
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: Option<*mut u32>, pheight: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).GetTextFilterSize)(windows_core::Interface::as_raw(self), core::mem::transmute(pwidth.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pheight.unwrap_or(std::ptr::null_mut())))
    }
}
unsafe impl Send for ID3D10Device {}
unsafe impl Sync for ID3D10Device {}
#[repr(C)]
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
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL),
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
    pub GetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL),
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
        (windows_core::Interface::vtable(self).CreateShaderResourceView1)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppsrview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: Option<*mut Option<ID3D10BlendState1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBlendState1)(windows_core::Interface::as_raw(self), pblendstatedesc, core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1 {
        (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10Device1 {}
unsafe impl Sync for ID3D10Device1 {}
#[repr(C)]
pub struct ID3D10Device1_Vtbl {
    pub base__: ID3D10Device_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_SHADER_RESOURCE_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView1: usize,
    pub CreateBlendState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BLEND_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1,
}
windows_core::imp::define_interface!(ID3D10DeviceChild, ID3D10DeviceChild_Vtbl, 0x9b7e4c00_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10DeviceChild {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10DeviceChild, windows_core::IUnknown);
impl ID3D10DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize, core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, core::mem::transmute(pdata.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const windows_core::GUID, pdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok()
    }
}
unsafe impl Send for ID3D10DeviceChild {}
unsafe impl Sync for ID3D10DeviceChild {}
#[repr(C)]
pub struct ID3D10DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10Effect, ID3D10Effect_Vtbl, 0x51b0ca8b_ec0b_4519_870d_8ee1cb5017c7);
impl core::ops::Deref for ID3D10Effect {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Effect, windows_core::IUnknown);
impl ID3D10Effect {
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsPool(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsPool)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10EffectConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetVariableBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi())
    }
    pub unsafe fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique> {
        (windows_core::Interface::vtable(self).GetTechniqueByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetTechniqueByName<P0>(&self, name: P0) -> Option<ID3D10EffectTechnique>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetTechniqueByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn Optimize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Optimize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOptimized(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsOptimized)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10Effect {}
unsafe impl Sync for ID3D10Effect {}
#[repr(C)]
pub struct ID3D10Effect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub IsPool: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
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
    pub IsOptimized: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBlendState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pblenddesc).ok()
    }
}
unsafe impl Send for ID3D10EffectBlendVariable {}
unsafe impl Sync for ID3D10EffectBlendVariable {}
#[repr(C)]
pub struct ID3D10EffectBlendVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_BLEND_DESC) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetConstantBuffer)(windows_core::Interface::as_raw(self), pconstantbuffer.param().abi()).ok()
    }
    pub unsafe fn GetConstantBuffer(&self) -> windows_core::Result<ID3D10Buffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConstantBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTextureBuffer<P0>(&self, ptexturebuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        (windows_core::Interface::vtable(self).SetTextureBuffer)(windows_core::Interface::as_raw(self), ptexturebuffer.param().abi()).ok()
    }
    pub unsafe fn GetTextureBuffer(&self) -> windows_core::Result<ID3D10ShaderResourceView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTextureBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D10EffectConstantBuffer {}
unsafe impl Sync for ID3D10EffectConstantBuffer {}
#[repr(C)]
pub struct ID3D10EffectConstantBuffer_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDepthStencilState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pdepthstencildesc).ok()
    }
}
unsafe impl Send for ID3D10EffectDepthStencilVariable {}
unsafe impl Sync for ID3D10EffectDepthStencilVariable {}
#[repr(C)]
pub struct ID3D10EffectDepthStencilVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetDepthStencil)(windows_core::Interface::as_raw(self), presource.param().abi()).ok()
    }
    pub unsafe fn GetDepthStencil(&self) -> windows_core::Result<ID3D10DepthStencilView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDepthStencil)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDepthStencilArray(&self, ppresources: &[Option<ID3D10DepthStencilView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDepthStencilArray(&self, ppresources: &mut [Option<ID3D10DepthStencilView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D10EffectDepthStencilViewVariable {}
unsafe impl Sync for ID3D10EffectDepthStencilViewVariable {}
#[repr(C)]
pub struct ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn GetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMatrix)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMatrixArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn SetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixTranspose)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn GetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMatrixTranspose)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
}
unsafe impl Send for ID3D10EffectMatrixVariable {}
unsafe impl Sync for ID3D10EffectMatrixVariable {}
#[repr(C)]
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
windows_core::imp::define_interface!(ID3D10EffectPass, ID3D10EffectPass_Vtbl);
impl ID3D10EffectPass {
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVertexShaderDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGeometryShaderDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPixelShaderDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn Apply(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask).ok()
    }
}
unsafe impl Send for ID3D10EffectPass {}
unsafe impl Sync for ID3D10EffectPass {}
#[repr(C)]
pub struct ID3D10EffectPass_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_DESC) -> windows_core::HRESULT,
    pub GetVertexShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetGeometryShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetPixelShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10EffectPool, ID3D10EffectPool_Vtbl, 0x9537ab04_3250_412e_8213_fcd2f8677933);
impl core::ops::Deref for ID3D10EffectPool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectPool, windows_core::IUnknown);
impl ID3D10EffectPool {
    pub unsafe fn AsEffect(&self) -> Option<ID3D10Effect> {
        (windows_core::Interface::vtable(self).AsEffect)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10EffectPool {}
unsafe impl Sync for ID3D10EffectPool {}
#[repr(C)]
pub struct ID3D10EffectPool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsEffect: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10Effect>,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRasterizerState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, prasterizerdesc).ok()
    }
}
unsafe impl Send for ID3D10EffectRasterizerVariable {}
unsafe impl Sync for ID3D10EffectRasterizerVariable {}
#[repr(C)]
pub struct ID3D10EffectRasterizerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_RASTERIZER_DESC) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), presource.param().abi()).ok()
    }
    pub unsafe fn GetRenderTarget(&self) -> windows_core::Result<ID3D10RenderTargetView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRenderTargetArray(&self, ppresources: &[Option<ID3D10RenderTargetView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetRenderTargetArray(&self, ppresources: &mut [Option<ID3D10RenderTargetView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D10EffectRenderTargetViewVariable {}
unsafe impl Sync for ID3D10EffectRenderTargetViewVariable {}
#[repr(C)]
pub struct ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSampler)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBackingStore(&self, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, psamplerdesc).ok()
    }
}
unsafe impl Send for ID3D10EffectSamplerVariable {}
unsafe impl Sync for ID3D10EffectSamplerVariable {}
#[repr(C)]
pub struct ID3D10EffectSamplerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetSampler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SAMPLER_DESC) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetFloat)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetFloat(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFloat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFloatArray(&self, pdata: &[f32], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFloatArray(&self, pdata: &mut [f32], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetInt(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInt)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetInt(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIntArray(&self, pdata: &[i32], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetIntArray(&self, pdata: &mut [i32], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetBool<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBool)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn GetBool(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBool)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoolArray(&self, pdata: &[super::super::Foundation::BOOL], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetBoolArray(&self, pdata: &mut [super::super::Foundation::BOOL], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D10EffectScalarVariable {}
unsafe impl Sync for ID3D10EffectScalarVariable {}
#[repr(C)]
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
    pub SetBool: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).SetResource)(windows_core::Interface::as_raw(self), presource.param().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D10ShaderResourceView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetResourceArray(&self, ppresources: &[Option<ID3D10ShaderResourceView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetResourceArray(&self, ppresources: &mut [Option<ID3D10ShaderResourceView>], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D10EffectShaderResourceVariable {}
unsafe impl Sync for ID3D10EffectShaderResourceVariable {}
#[repr(C)]
pub struct ID3D10EffectShaderResourceVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
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
        (windows_core::Interface::vtable(self).GetShaderDesc)(windows_core::Interface::as_raw(self), shaderindex, pdesc).ok()
    }
    pub unsafe fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10VertexShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10GeometryShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeometryShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10PixelShader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc).ok()
    }
}
unsafe impl Send for ID3D10EffectShaderVariable {}
unsafe impl Sync for ID3D10EffectShaderVariable {}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStringArray(&self, ppstrings: &mut [windows_core::PCSTR], offset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppstrings.as_ptr()), offset, ppstrings.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D10EffectStringVariable {}
unsafe impl Sync for ID3D10EffectStringVariable {}
#[repr(C)]
pub struct ID3D10EffectStringVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR) -> windows_core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10EffectTechnique, ID3D10EffectTechnique_Vtbl);
impl ID3D10EffectTechnique {
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass> {
        (windows_core::Interface::vtable(self).GetPassByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetPassByName<P0>(&self, name: P0) -> Option<ID3D10EffectPass>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetPassByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask).ok()
    }
}
unsafe impl Send for ID3D10EffectTechnique {}
unsafe impl Sync for ID3D10EffectTechnique {}
#[repr(C)]
pub struct ID3D10EffectTechnique_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetPassByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectPass>,
    pub GetPassByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectPass>,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10EffectType, ID3D10EffectType_Vtbl);
impl ID3D10EffectType {
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10EffectType> {
        (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D10EffectType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetMemberTypeBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberTypeBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi())
    }
    pub unsafe fn GetMemberName(&self, index: u32) -> windows_core::PCSTR {
        (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR {
        (windows_core::Interface::vtable(self).GetMemberSemantic)(windows_core::Interface::as_raw(self), index)
    }
}
unsafe impl Send for ID3D10EffectType {}
unsafe impl Sync for ID3D10EffectType {}
#[repr(C)]
pub struct ID3D10EffectType_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
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
windows_core::imp::define_interface!(ID3D10EffectVariable, ID3D10EffectVariable_Vtbl);
impl ID3D10EffectVariable {
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount).ok()
    }
}
unsafe impl Send for ID3D10EffectVariable {}
unsafe impl Sync for ID3D10EffectVariable {}
#[repr(C)]
pub struct ID3D10EffectVariable_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
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
windows_core::imp::define_interface!(ID3D10EffectVectorVariable, ID3D10EffectVectorVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectVectorVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectVectorVariable, ID3D10EffectVariable);
impl ID3D10EffectVectorVariable {
    pub unsafe fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoolVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn SetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn SetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFloatVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBoolVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn GetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIntVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn GetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFloatVector)(windows_core::Interface::as_raw(self), pdata).ok()
    }
    pub unsafe fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIntVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
    pub unsafe fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata, offset, count).ok()
    }
}
unsafe impl Send for ID3D10EffectVectorVariable {}
unsafe impl Sync for ID3D10EffectVectorVariable {}
#[repr(C)]
pub struct ID3D10EffectVectorVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub SetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
    pub SetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub SetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub GetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10GeometryShader, ID3D10GeometryShader_Vtbl, 0x6316be88_54cd_4040_ab44_20461bc81f68);
impl core::ops::Deref for ID3D10GeometryShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10GeometryShader, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10GeometryShader {}
unsafe impl Send for ID3D10GeometryShader {}
unsafe impl Sync for ID3D10GeometryShader {}
#[repr(C)]
pub struct ID3D10GeometryShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D10InfoQueue, ID3D10InfoQueue_Vtbl, 0x1b940b17_2642_4d1f_ab1f_b99bad0c395f);
impl core::ops::Deref for ID3D10InfoQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10InfoQueue, windows_core::IUnknown);
impl ID3D10InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: Option<*mut D3D10_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), messageindex, core::mem::transmute(pmessage.unwrap_or(std::ptr::null_mut())), pmessagebytelength).ok()
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNumStoredMessages)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetMessageCountLimit)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: Option<*mut D3D10_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(pfilter.unwrap_or(std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearStorageFilter(&self) {
        (windows_core::Interface::vtable(self).ClearStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushEmptyStorageFilter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: Option<*mut D3D10_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(pfilter.unwrap_or(std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        (windows_core::Interface::vtable(self).ClearRetrievalFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddMessage<P0>(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), category, severity, id, pdescription.param().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnCategory<P0>(&self, category: D3D10_MESSAGE_CATEGORY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), category, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnSeverity<P0>(&self, severity: D3D10_MESSAGE_SEVERITY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnID<P0>(&self, id: D3D10_MESSAGE_ID, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), id, benable.param().abi()).ok()
    }
    pub unsafe fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), category)
    }
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity)
    }
    pub unsafe fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnID)(windows_core::Interface::as_raw(self), id)
    }
    pub unsafe fn SetMuteDebugOutput<P0>(&self, bmute: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMuteDebugOutput)(windows_core::Interface::as_raw(self), bmute.param().abi())
    }
    pub unsafe fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetMuteDebugOutput)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10InfoQueue {}
unsafe impl Sync for ID3D10InfoQueue {}
#[repr(C)]
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
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_CATEGORY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_SEVERITY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_ID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D10InputLayout, ID3D10InputLayout_Vtbl, 0x9b7e4c0b_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10InputLayout {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10InputLayout, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10InputLayout {}
unsafe impl Send for ID3D10InputLayout {}
unsafe impl Sync for ID3D10InputLayout {}
#[repr(C)]
pub struct ID3D10InputLayout_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D10Multithread, ID3D10Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Multithread {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Multithread, windows_core::IUnknown);
impl ID3D10Multithread {
    pub unsafe fn Enter(&self) {
        (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Leave(&self) {
        (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMultithreadProtected<P0>(&self, bmtprotect: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMultithreadProtected)(windows_core::Interface::as_raw(self), bmtprotect.param().abi())
    }
    pub unsafe fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10Multithread {}
unsafe impl Sync for ID3D10Multithread {}
#[repr(C)]
pub struct ID3D10Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D10PixelShader, ID3D10PixelShader_Vtbl, 0x4968b601_9d00_4cde_8346_8e7f675819b6);
impl core::ops::Deref for ID3D10PixelShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10PixelShader, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10PixelShader {}
unsafe impl Send for ID3D10PixelShader {}
unsafe impl Sync for ID3D10PixelShader {}
#[repr(C)]
pub struct ID3D10PixelShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D10Predicate, ID3D10Predicate_Vtbl, 0x9b7e4c10_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Predicate {
    type Target = ID3D10Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Predicate, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous, ID3D10Query);
impl ID3D10Predicate {}
unsafe impl Send for ID3D10Predicate {}
unsafe impl Sync for ID3D10Predicate {}
#[repr(C)]
pub struct ID3D10Predicate_Vtbl {
    pub base__: ID3D10Query_Vtbl,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D10Query {}
unsafe impl Sync for ID3D10Query {}
#[repr(C)]
pub struct ID3D10Query_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_QUERY_DESC),
}
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10RasterizerState {}
unsafe impl Sync for ID3D10RasterizerState {}
#[repr(C)]
pub struct ID3D10RasterizerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RASTERIZER_DESC),
}
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10RenderTargetView {}
unsafe impl Sync for ID3D10RenderTargetView {}
#[repr(C)]
pub struct ID3D10RenderTargetView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10Resource {}
unsafe impl Sync for ID3D10Resource {}
#[repr(C)]
pub struct ID3D10Resource_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10SamplerState {}
unsafe impl Sync for ID3D10SamplerState {}
#[repr(C)]
pub struct ID3D10SamplerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SAMPLER_DESC),
}
windows_core::imp::define_interface!(ID3D10ShaderReflection, ID3D10ShaderReflection_Vtbl, 0xd40e20b6_f8f7_42ad_ab20_4baf8f15dfaa);
impl core::ops::Deref for ID3D10ShaderReflection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection, windows_core::IUnknown);
impl ID3D10ShaderReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
}
unsafe impl Send for ID3D10ShaderReflection {}
unsafe impl Sync for ID3D10ShaderReflection {}
#[repr(C)]
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
windows_core::imp::define_interface!(ID3D10ShaderReflection1, ID3D10ShaderReflection1_Vtbl, 0xc3457783_a846_47ce_9520_cea6f66e7447);
impl core::ops::Deref for ID3D10ShaderReflection1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection1, windows_core::IUnknown);
impl ID3D10ShaderReflection1 {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc).ok()
    }
    pub unsafe fn GetMovInstructionCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMovInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMovcInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConversionInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitwiseInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> windows_core::Result<super::Direct3D::D3D_PRIMITIVE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGSInputPrimitive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLevel9Shader(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLevel9Shader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSampleFrequencyShader(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSampleFrequencyShader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D10ShaderReflection1 {}
unsafe impl Sync for ID3D10ShaderReflection1 {}
#[repr(C)]
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
    pub IsLevel9Shader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsSampleFrequencyShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionConstantBuffer, ID3D10ShaderReflectionConstantBuffer_Vtbl);
impl ID3D10ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
        (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
}
unsafe impl Send for ID3D10ShaderReflectionConstantBuffer {}
unsafe impl Sync for ID3D10ShaderReflectionConstantBuffer {}
#[repr(C)]
pub struct ID3D10ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>,
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionType, ID3D10ShaderReflectionType_Vtbl);
impl ID3D10ShaderReflectionType {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR {
        (windows_core::Interface::vtable(self).GetMemberTypeName)(windows_core::Interface::as_raw(self), index)
    }
}
unsafe impl Send for ID3D10ShaderReflectionType {}
unsafe impl Sync for ID3D10ShaderReflectionType {}
#[repr(C)]
pub struct ID3D10ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionVariable, ID3D10ShaderReflectionVariable_Vtbl);
impl ID3D10ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10ShaderReflectionVariable {}
unsafe impl Sync for ID3D10ShaderReflectionVariable {}
#[repr(C)]
pub struct ID3D10ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType>,
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
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10ShaderResourceView {}
unsafe impl Sync for ID3D10ShaderResourceView {}
#[repr(C)]
pub struct ID3D10ShaderResourceView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
}
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
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10ShaderResourceView1 {}
unsafe impl Sync for ID3D10ShaderResourceView1 {}
#[repr(C)]
pub struct ID3D10ShaderResourceView1_Vtbl {
    pub base__: ID3D10ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D10StateBlock, ID3D10StateBlock_Vtbl, 0x0803425a_57f5_4dd6_9465_a87570834a08);
impl core::ops::Deref for ID3D10StateBlock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10StateBlock, windows_core::IUnknown);
impl ID3D10StateBlock {
    pub unsafe fn Capture(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseAllDeviceObjects(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseAllDeviceObjects)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D10StateBlock {}
unsafe impl Sync for ID3D10StateBlock {}
#[repr(C)]
pub struct ID3D10StateBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseAllDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D10SwitchToRef, ID3D10SwitchToRef_Vtbl, 0x9b7e4e02_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10SwitchToRef {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10SwitchToRef, windows_core::IUnknown);
impl ID3D10SwitchToRef {
    pub unsafe fn SetUseRef<P0>(&self, useref: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseRef)(windows_core::Interface::as_raw(self), useref.param().abi())
    }
    pub unsafe fn GetUseRef(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetUseRef)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D10SwitchToRef {}
unsafe impl Sync for ID3D10SwitchToRef {}
#[repr(C)]
pub struct ID3D10SwitchToRef_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    pub GetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
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
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, ppdata).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10Texture1D {}
unsafe impl Sync for ID3D10Texture1D {}
#[repr(C)]
pub struct ID3D10Texture1D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10Texture2D {}
unsafe impl Sync for ID3D10Texture2D {}
#[repr(C)]
pub struct ID3D10Texture2D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE2D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D10Texture3D {}
unsafe impl Sync for ID3D10Texture3D {}
#[repr(C)]
pub struct ID3D10Texture3D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D10VertexShader, ID3D10VertexShader_Vtbl, 0x9b7e4c0a_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10VertexShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10VertexShader, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10VertexShader {}
unsafe impl Send for ID3D10VertexShader {}
unsafe impl Sync for ID3D10VertexShader {}
#[repr(C)]
pub struct ID3D10VertexShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID3D10View {}
unsafe impl Sync for ID3D10View {}
#[repr(C)]
pub struct ID3D10View_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
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
pub const D3D10_BIND_CONSTANT_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(4i32);
pub const D3D10_BIND_DEPTH_STENCIL: D3D10_BIND_FLAG = D3D10_BIND_FLAG(64i32);
pub const D3D10_BIND_INDEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(2i32);
pub const D3D10_BIND_RENDER_TARGET: D3D10_BIND_FLAG = D3D10_BIND_FLAG(32i32);
pub const D3D10_BIND_SHADER_RESOURCE: D3D10_BIND_FLAG = D3D10_BIND_FLAG(8i32);
pub const D3D10_BIND_STREAM_OUTPUT: D3D10_BIND_FLAG = D3D10_BIND_FLAG(16i32);
pub const D3D10_BIND_VERTEX_BUFFER: D3D10_BIND_FLAG = D3D10_BIND_FLAG(1i32);
pub const D3D10_BLEND_BLEND_FACTOR: D3D10_BLEND = D3D10_BLEND(14i32);
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
pub const D3D10_BREAKON_CATEGORY: windows_core::PCWSTR = windows_core::w!("BreakOn_CATEGORY_%s");
pub const D3D10_BREAKON_ID_DECIMAL: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%d");
pub const D3D10_BREAKON_ID_STRING: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%s");
pub const D3D10_BREAKON_SEVERITY: windows_core::PCWSTR = windows_core::w!("BreakOn_SEVERITY_%s");
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-2i32);
pub const D3D10_CLEAR_DEPTH: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(1i32);
pub const D3D10_CLEAR_STENCIL: D3D10_CLEAR_FLAG = D3D10_CLEAR_FLAG(2i32);
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
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
pub const D3D10_COMPARISON_GREATER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(5i32);
pub const D3D10_COMPARISON_GREATER_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(7i32);
pub const D3D10_COMPARISON_LESS: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(2i32);
pub const D3D10_COMPARISON_LESS_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(4i32);
pub const D3D10_COMPARISON_NEVER: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(1i32);
pub const D3D10_COMPARISON_NOT_EQUAL: D3D10_COMPARISON_FUNC = D3D10_COMPARISON_FUNC(6i32);
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: D3D10_COUNTER = D3D10_COUNTER(1073741824i32);
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(9i32);
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(2i32);
pub const D3D10_COUNTER_GPU_IDLE: D3D10_COUNTER = D3D10_COUNTER(0i32);
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(13i32);
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(12i32);
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(5i32);
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(6i32);
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(4i32);
pub const D3D10_COUNTER_PIXEL_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(3i32);
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(16i32);
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(15i32);
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(14i32);
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: D3D10_COUNTER = D3D10_COUNTER(17i32);
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(8i32);
pub const D3D10_COUNTER_TYPE_FLOAT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(0i32);
pub const D3D10_COUNTER_TYPE_UINT16: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(1i32);
pub const D3D10_COUNTER_TYPE_UINT32: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(2i32);
pub const D3D10_COUNTER_TYPE_UINT64: D3D10_COUNTER_TYPE = D3D10_COUNTER_TYPE(3i32);
pub const D3D10_COUNTER_VERTEX_PROCESSING: D3D10_COUNTER = D3D10_COUNTER(1i32);
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: D3D10_COUNTER = D3D10_COUNTER(7i32);
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: D3D10_COUNTER = D3D10_COUNTER(11i32);
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: D3D10_COUNTER = D3D10_COUNTER(10i32);
pub const D3D10_CPU_ACCESS_READ: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(131072i32);
pub const D3D10_CPU_ACCESS_WRITE: D3D10_CPU_ACCESS_FLAG = D3D10_CPU_ACCESS_FLAG(65536i32);
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(16i32);
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(32i32);
pub const D3D10_CREATE_DEVICE_DEBUG: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(2i32);
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1024i32);
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(128i32);
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(8i32);
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(1i32);
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(512i32);
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: D3D10_CREATE_DEVICE_FLAG = D3D10_CREATE_DEVICE_FLAG(4i32);
pub const D3D10_CULL_BACK: D3D10_CULL_MODE = D3D10_CULL_MODE(3i32);
pub const D3D10_CULL_FRONT: D3D10_CULL_MODE = D3D10_CULL_MODE(2i32);
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
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(1i32);
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = D3D10_DEPTH_WRITE_MASK(0i32);
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
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(1i32);
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(2i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(3i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(4i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(5i32);
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(6i32);
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = D3D10_DSV_DIMENSION(0i32);
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
pub const D3D10_ENABLE_BREAK_ON_MESSAGE: windows_core::PCWSTR = windows_core::w!("EnableBreakOnMessage");
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(40960i32);
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(41216i32);
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37120i32);
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37376i32);
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = D3D10_FEATURE_LEVEL1(37632i32);
pub const D3D10_FILL_SOLID: D3D10_FILL_MODE = D3D10_FILL_MODE(3i32);
pub const D3D10_FILL_WIREFRAME: D3D10_FILL_MODE = D3D10_FILL_MODE(2i32);
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
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(1i32);
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = D3D10_INPUT_CLASSIFICATION(0i32);
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
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
pub const D3D10_QUERY_EVENT: D3D10_QUERY = D3D10_QUERY(0i32);
pub const D3D10_QUERY_MISC_PREDICATEHINT: D3D10_QUERY_MISC_FLAG = D3D10_QUERY_MISC_FLAG(1i32);
pub const D3D10_QUERY_OCCLUSION: D3D10_QUERY = D3D10_QUERY(1i32);
pub const D3D10_QUERY_OCCLUSION_PREDICATE: D3D10_QUERY = D3D10_QUERY(5i32);
pub const D3D10_QUERY_PIPELINE_STATISTICS: D3D10_QUERY = D3D10_QUERY(4i32);
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: D3D10_QUERY = D3D10_QUERY(7i32);
pub const D3D10_QUERY_SO_STATISTICS: D3D10_QUERY = D3D10_QUERY(6i32);
pub const D3D10_QUERY_TIMESTAMP: D3D10_QUERY = D3D10_QUERY(2i32);
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: D3D10_QUERY = D3D10_QUERY(3i32);
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D10_RAISE_FLAG = D3D10_RAISE_FLAG(1i32);
pub const D3D10_REGKEY_PATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Direct3D");
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
pub const D3D10_RESOURCE_DIMENSION_BUFFER: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(1i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(2i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(3i32);
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(4i32);
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: D3D10_RESOURCE_DIMENSION = D3D10_RESOURCE_DIMENSION(0i32);
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(32i32);
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(1i32);
pub const D3D10_RESOURCE_MISC_SHARED: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(2i32);
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(16i32);
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: D3D10_RESOURCE_MISC_FLAG = D3D10_RESOURCE_MISC_FLAG(4i32);
pub const D3D10_RTV_DIMENSION_BUFFER: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(1i32);
pub const D3D10_RTV_DIMENSION_TEXTURE1D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(2i32);
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(3i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(4i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(5i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(6i32);
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(7i32);
pub const D3D10_RTV_DIMENSION_TEXTURE3D: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(8i32);
pub const D3D10_RTV_DIMENSION_UNKNOWN: D3D10_RTV_DIMENSION = D3D10_RTV_DIMENSION(0i32);
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
pub const D3D10_SDK_VERSION: u32 = 29u32;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
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
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(7i32);
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(1i32);
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(2i32);
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(4i32);
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(0i32);
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(6i32);
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(5i32);
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: D3D10_SHADER_DEBUG_SCOPETYPE = D3D10_SHADER_DEBUG_SCOPETYPE(3i32);
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(1i32);
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = D3D10_SHADER_DEBUG_VARTYPE(0i32);
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
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
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
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
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub const D3D10_STENCIL_OP_DECR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(8i32);
pub const D3D10_STENCIL_OP_DECR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(5i32);
pub const D3D10_STENCIL_OP_INCR: D3D10_STENCIL_OP = D3D10_STENCIL_OP(7i32);
pub const D3D10_STENCIL_OP_INCR_SAT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(4i32);
pub const D3D10_STENCIL_OP_INVERT: D3D10_STENCIL_OP = D3D10_STENCIL_OP(6i32);
pub const D3D10_STENCIL_OP_KEEP: D3D10_STENCIL_OP = D3D10_STENCIL_OP(1i32);
pub const D3D10_STENCIL_OP_REPLACE: D3D10_STENCIL_OP = D3D10_STENCIL_OP(3i32);
pub const D3D10_STENCIL_OP_ZERO: D3D10_STENCIL_OP = D3D10_STENCIL_OP(2i32);
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
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
pub const D3D10_TEXTURE_ADDRESS_WRAP: D3D10_TEXTURE_ADDRESS_MODE = D3D10_TEXTURE_ADDRESS_MODE(1i32);
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub const D3D10_UNMUTE_SEVERITY_INFO: windows_core::PCWSTR = windows_core::w!("Unmute_SEVERITY_INFO");
pub const D3D10_USAGE_DEFAULT: D3D10_USAGE = D3D10_USAGE(0i32);
pub const D3D10_USAGE_DYNAMIC: D3D10_USAGE = D3D10_USAGE(2i32);
pub const D3D10_USAGE_IMMUTABLE: D3D10_USAGE = D3D10_USAGE(1i32);
pub const D3D10_USAGE_STAGING: D3D10_USAGE = D3D10_USAGE(3i32);
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
pub const _FACD3D10: u32 = 2169u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_ASYNC_GETDATA_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_ASYNC_GETDATA_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_ASYNC_GETDATA_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_ASYNC_GETDATA_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_BIND_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_BIND_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_BIND_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_BIND_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_BLEND(pub i32);
impl windows_core::TypeKind for D3D10_BLEND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_BLEND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_BLEND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_BLEND_OP(pub i32);
impl windows_core::TypeKind for D3D10_BLEND_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_BLEND_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_BLEND_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_CLEAR_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_CLEAR_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_CLEAR_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_CLEAR_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_COLOR_WRITE_ENABLE(pub i32);
impl windows_core::TypeKind for D3D10_COLOR_WRITE_ENABLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_COMPARISON_FUNC(pub i32);
impl windows_core::TypeKind for D3D10_COMPARISON_FUNC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_COMPARISON_FUNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_COMPARISON_FUNC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_COUNTER(pub i32);
impl windows_core::TypeKind for D3D10_COUNTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_COUNTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_COUNTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D10_COUNTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_COUNTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_CPU_ACCESS_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_CPU_ACCESS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_CPU_ACCESS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_CPU_ACCESS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_CREATE_DEVICE_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_CREATE_DEVICE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_CREATE_DEVICE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_CREATE_DEVICE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_CULL_MODE(pub i32);
impl windows_core::TypeKind for D3D10_CULL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_CULL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_CULL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_DEPTH_WRITE_MASK(pub i32);
impl windows_core::TypeKind for D3D10_DEPTH_WRITE_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_DEVICE_STATE_TYPES(pub i32);
impl windows_core::TypeKind for D3D10_DEVICE_STATE_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_DEVICE_STATE_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_DEVICE_STATE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_DRIVER_TYPE(pub i32);
impl windows_core::TypeKind for D3D10_DRIVER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_DRIVER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_DRIVER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_DSV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D10_DSV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_DSV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_DSV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_FEATURE_LEVEL1(pub i32);
impl windows_core::TypeKind for D3D10_FEATURE_LEVEL1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_FEATURE_LEVEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_FEATURE_LEVEL1").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_FILL_MODE(pub i32);
impl windows_core::TypeKind for D3D10_FILL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_FILL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_FILTER(pub i32);
impl windows_core::TypeKind for D3D10_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_FILTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D10_FILTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_FILTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_FILTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_FORMAT_SUPPORT(pub i32);
impl windows_core::TypeKind for D3D10_FORMAT_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_INPUT_CLASSIFICATION(pub i32);
impl windows_core::TypeKind for D3D10_INPUT_CLASSIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_MAP(pub i32);
impl windows_core::TypeKind for D3D10_MAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_MAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_MAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_MAP_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_MAP_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_MAP_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_MAP_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_MESSAGE_CATEGORY(pub i32);
impl windows_core::TypeKind for D3D10_MESSAGE_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_MESSAGE_ID(pub i32);
impl windows_core::TypeKind for D3D10_MESSAGE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_MESSAGE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_MESSAGE_SEVERITY(pub i32);
impl windows_core::TypeKind for D3D10_MESSAGE_SEVERITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_QUERY(pub i32);
impl windows_core::TypeKind for D3D10_QUERY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_QUERY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_QUERY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_QUERY_MISC_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_QUERY_MISC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_QUERY_MISC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_QUERY_MISC_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_RAISE_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_RAISE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_RAISE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_RAISE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_RESOURCE_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D10_RESOURCE_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_RESOURCE_MISC_FLAG(pub i32);
impl windows_core::TypeKind for D3D10_RESOURCE_MISC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_RESOURCE_MISC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_MISC_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_RTV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D10_RTV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_RTV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_RTV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_SHADER_DEBUG_REGTYPE(pub i32);
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_REGTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_SHADER_DEBUG_REGTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_REGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_SHADER_DEBUG_SCOPETYPE(pub i32);
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_SCOPETYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_SCOPETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_SHADER_DEBUG_VARTYPE(pub i32);
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_VARTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_SHADER_DEBUG_VARTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_VARTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(pub i32);
impl windows_core::TypeKind for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_STENCIL_OP(pub i32);
impl windows_core::TypeKind for D3D10_STENCIL_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_STENCIL_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_STENCIL_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_TEXTURECUBE_FACE(pub i32);
impl windows_core::TypeKind for D3D10_TEXTURECUBE_FACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_TEXTURECUBE_FACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURECUBE_FACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_TEXTURE_ADDRESS_MODE(pub i32);
impl windows_core::TypeKind for D3D10_TEXTURE_ADDRESS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D10_USAGE(pub i32);
impl windows_core::TypeKind for D3D10_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D10_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D10_USAGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub BlendEnable: [super::super::Foundation::BOOL; 8],
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: [u8; 8],
}
impl windows_core::TypeKind for D3D10_BLEND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
impl windows_core::TypeKind for D3D10_BLEND_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl windows_core::TypeKind for D3D10_BOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_BOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
impl windows_core::TypeKind for D3D10_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BUFFER_RTV {
    pub Anonymous1: D3D10_BUFFER_RTV_0,
    pub Anonymous2: D3D10_BUFFER_RTV_1,
}
impl windows_core::TypeKind for D3D10_BUFFER_RTV {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_BUFFER_RTV_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_BUFFER_RTV_1 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_BUFFER_SRV {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_BUFFER_SRV_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_BUFFER_SRV_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_COUNTER_DESC {
    pub Counter: D3D10_COUNTER,
    pub MiscFlags: u32,
}
impl windows_core::TypeKind for D3D10_COUNTER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_COUNTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D10_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
impl windows_core::TypeKind for D3D10_COUNTER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_COUNTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
impl windows_core::TypeKind for D3D10_DEPTH_STENCILOP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D10_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D10_DEPTH_STENCILOP_DESC,
}
impl windows_core::TypeKind for D3D10_DEPTH_STENCIL_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for D3D10_DEPTH_STENCIL_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: super::super::Foundation::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
impl windows_core::TypeKind for D3D10_EFFECT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_EFFECT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: super::super::Foundation::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: windows_core::PCSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
impl windows_core::TypeKind for D3D10_EFFECT_SHADER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_EFFECT_TYPE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub Semantic: windows_core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
impl windows_core::TypeKind for D3D10_EFFECT_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_INFO_QUEUE_FILTER {
    pub AllowList: D3D10_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D10_INFO_QUEUE_FILTER_DESC,
}
impl windows_core::TypeKind for D3D10_INFO_QUEUE_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D10_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D10_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D10_MESSAGE_ID,
}
impl windows_core::TypeKind for D3D10_INFO_QUEUE_FILTER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D10_INPUT_ELEMENT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_MAPPED_TEXTURE2D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
}
impl windows_core::TypeKind for D3D10_MAPPED_TEXTURE2D {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_MAPPED_TEXTURE2D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_MAPPED_TEXTURE3D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl windows_core::TypeKind for D3D10_MAPPED_TEXTURE3D {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_MAPPED_TEXTURE3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl windows_core::TypeKind for D3D10_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for D3D10_PASS_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: core::mem::ManuallyDrop<Option<ID3D10EffectShaderVariable>>,
    pub ShaderIndex: u32,
}
impl Clone for D3D10_PASS_SHADER_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D10_PASS_SHADER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_PASS_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl windows_core::TypeKind for D3D10_QUERY_DATA_SO_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_QUERY_DESC {
    pub Query: D3D10_QUERY,
    pub MiscFlags: u32,
}
impl windows_core::TypeKind for D3D10_QUERY_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_QUERY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D10_RASTERIZER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
impl windows_core::TypeKind for D3D10_RENDER_TARGET_BLEND_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for D3D10_RENDER_TARGET_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for D3D10_SAMPLER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_FILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_INPUT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_INST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_INST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: super::super::Foundation::BOOL,
    pub bInfPossible: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_OUTPUTVAR {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    pub ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    pub Name: u32,
    pub uNameLen: u32,
    pub uVariables: u32,
    pub VariableData: u32,
}
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_SCOPE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    pub File: u32,
    pub Line: u32,
    pub Column: u32,
    pub TokenLength: u32,
    pub TokenId: u32,
}
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_TOKEN_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_DEBUG_VAR_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_DEBUG_VAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_INPUT_BIND_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D3D10_SHADER_RESOURCE_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SHADER_TYPE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for D3D10_SHADER_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D10_SIGNATURE_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl windows_core::TypeKind for D3D10_SO_DECLARATION_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for D3D10_STATE_BLOCK_MASK {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl windows_core::TypeKind for D3D10_SUBRESOURCE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: windows_core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
impl windows_core::TypeKind for D3D10_TECHNIQUE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D10_TEX1D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX1D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D10_TEX2DMS_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D10_TEX2D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX2D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl windows_core::TypeKind for D3D10_TEX3D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX3D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D10_TEX3D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEX3D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
impl windows_core::TypeKind for D3D10_TEXCUBE_ARRAY_SRV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D10_TEXCUBE_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D10_TEXTURE1D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_TEXTURE1D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D10_TEXTURE2D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_TEXTURE2D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D10_TEXTURE3D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D10_TEXTURE3D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_VIEWPORT {
    pub TopLeftX: i32,
    pub TopLeftY: i32,
    pub Width: u32,
    pub Height: u32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl windows_core::TypeKind for D3D10_VIEWPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D10_VIEWPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_D3D10_CREATE_DEVICE1 = Option<unsafe extern "system" fn(param0: Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut Option<ID3D10Device1>) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = Option<unsafe extern "system" fn(param0: Option<super::Dxgi::IDXGIAdapter>, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::Dxgi::DXGI_SWAP_CHAIN_DESC, param7: *mut Option<super::Dxgi::IDXGISwapChain>, param8: *mut Option<ID3D10Device1>) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
