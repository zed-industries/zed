#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D11CreateDevice<P0, P1>(padapter: P0, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: P1, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>, sdkversion: u32, ppdevice: Option<*mut Option<ID3D11Device>>, pfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : * mut core::ffi::c_void, drivertype : super::Direct3D:: D3D_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : D3D11_CREATE_DEVICE_FLAG, pfeaturelevels : *const super::Direct3D:: D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void, pfeaturelevel : *mut super::Direct3D:: D3D_FEATURE_LEVEL, ppimmediatecontext : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D11CreateDevice(padapter.param().abi(), drivertype, software.param().abi(), flags, core::mem::transmute(pfeaturelevels.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pfeaturelevels.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), sdkversion, core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfeaturelevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppimmediatecontext.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[inline]
pub unsafe fn D3D11CreateDeviceAndSwapChain<P0, P1>(padapter: P0, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: P1, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>, sdkversion: u32, pswapchaindesc: Option<*const super::Dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::Dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D11Device>>, pfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("d3d11.dll" "system" fn D3D11CreateDeviceAndSwapChain(padapter : * mut core::ffi::c_void, drivertype : super::Direct3D:: D3D_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : D3D11_CREATE_DEVICE_FLAG, pfeaturelevels : *const super::Direct3D:: D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, pswapchaindesc : *const super::Dxgi:: DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut * mut core::ffi::c_void, ppdevice : *mut * mut core::ffi::c_void, pfeaturelevel : *mut super::Direct3D:: D3D_FEATURE_LEVEL, ppimmediatecontext : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D11CreateDeviceAndSwapChain(
        padapter.param().abi(),
        drivertype,
        software.param().abi(),
        flags,
        core::mem::transmute(pfeaturelevels.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pfeaturelevels.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        sdkversion,
        core::mem::transmute(pswapchaindesc.unwrap_or(std::ptr::null())),
        core::mem::transmute(ppswapchain.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(ppdevice.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pfeaturelevel.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(ppimmediatecontext.unwrap_or(std::ptr::null_mut())),
    )
    .ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3DDisassemble11Trace<P0>(psrcdata: *const core::ffi::c_void, srcdatasize: usize, ptrace: P0, startstep: u32, numsteps: u32, flags: u32) -> windows_core::Result<super::Direct3D::ID3DBlob>
where
    P0: windows_core::Param<ID3D11ShaderTrace>,
{
    windows_targets::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble11Trace(psrcdata : *const core::ffi::c_void, srcdatasize : usize, ptrace : * mut core::ffi::c_void, startstep : u32, numsteps : u32, flags : u32, ppdisassembly : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DDisassemble11Trace(psrcdata, srcdatasize, ptrace.param().abi(), startstep, numsteps, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DX11CreateFFT<P0>(pdevicecontext: P0, pdesc: *const D3DX11_FFT_DESC, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT(pdevicecontext : * mut core::ffi::c_void, pdesc : *const D3DX11_FFT_DESC, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT(pdevicecontext.param().abi(), pdesc, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT1DComplex<P0>(pdevicecontext: P0, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT1DComplex(pdevicecontext : * mut core::ffi::c_void, x : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT1DComplex(pdevicecontext.param().abi(), x, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT1DReal<P0>(pdevicecontext: P0, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT1DReal(pdevicecontext : * mut core::ffi::c_void, x : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT1DReal(pdevicecontext.param().abi(), x, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT2DComplex<P0>(pdevicecontext: P0, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT2DComplex(pdevicecontext : * mut core::ffi::c_void, x : u32, y : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT2DComplex(pdevicecontext.param().abi(), x, y, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT2DReal<P0>(pdevicecontext: P0, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT2DReal(pdevicecontext : * mut core::ffi::c_void, x : u32, y : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT2DReal(pdevicecontext.param().abi(), x, y, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT3DComplex<P0>(pdevicecontext: P0, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT3DComplex(pdevicecontext : * mut core::ffi::c_void, x : u32, y : u32, z : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT3DComplex(pdevicecontext.param().abi(), x, y, z, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateFFT3DReal<P0>(pdevicecontext: P0, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut Option<ID3DX11FFT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateFFT3DReal(pdevicecontext : * mut core::ffi::c_void, x : u32, y : u32, z : u32, flags : u32, pbufferinfo : *mut D3DX11_FFT_BUFFER_INFO, ppfft : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3DX11CreateFFT3DReal(pdevicecontext.param().abi(), x, y, z, flags, pbufferinfo, core::mem::transmute(ppfft)).ok()
}
#[inline]
pub unsafe fn D3DX11CreateScan<P0>(pdevicecontext: P0, maxelementscansize: u32, maxscancount: u32) -> windows_core::Result<ID3DX11Scan>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateScan(pdevicecontext : * mut core::ffi::c_void, maxelementscansize : u32, maxscancount : u32, ppscan : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DX11CreateScan(pdevicecontext.param().abi(), maxelementscansize, maxscancount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D3DX11CreateSegmentedScan<P0>(pdevicecontext: P0, maxelementscansize: u32) -> windows_core::Result<ID3DX11SegmentedScan>
where
    P0: windows_core::Param<ID3D11DeviceContext>,
{
    windows_targets::link!("d3dcsx.dll" "system" fn D3DX11CreateSegmentedScan(pdevicecontext : * mut core::ffi::c_void, maxelementscansize : u32, ppscan : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D3DX11CreateSegmentedScan(pdevicecontext.param().abi(), maxelementscansize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(ID3D11Asynchronous, ID3D11Asynchronous_Vtbl, 0x4b35d0cd_1e15_4258_9c98_1b1333f6dd3b);
impl core::ops::Deref for ID3D11Asynchronous {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Asynchronous, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Asynchronous {
    pub unsafe fn GetDataSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D11Asynchronous {}
unsafe impl Sync for ID3D11Asynchronous {}
#[repr(C)]
pub struct ID3D11Asynchronous_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID3D11AuthenticatedChannel, ID3D11AuthenticatedChannel_Vtbl, 0x3015a308_dcbd_47aa_a747_192486d14d4a);
impl core::ops::Deref for ID3D11AuthenticatedChannel {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11AuthenticatedChannel, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11AuthenticatedChannel {
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCertificate(&self, pcertificate: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), pcertificate.len().try_into().unwrap(), core::mem::transmute(pcertificate.as_ptr())).ok()
    }
    pub unsafe fn GetChannelHandle(&self) -> super::super::Foundation::HANDLE {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelHandle)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11AuthenticatedChannel {}
unsafe impl Sync for ID3D11AuthenticatedChannel {}
#[repr(C)]
pub struct ID3D11AuthenticatedChannel_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub GetChannelHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE),
}
windows_core::imp::define_interface!(ID3D11BlendState, ID3D11BlendState_Vtbl, 0x75b68faa_347d_4159_8f45_a0640f01cd9a);
impl core::ops::Deref for ID3D11BlendState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11BlendState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11BlendState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11BlendState {}
unsafe impl Sync for ID3D11BlendState {}
#[repr(C)]
pub struct ID3D11BlendState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BLEND_DESC),
}
windows_core::imp::define_interface!(ID3D11BlendState1, ID3D11BlendState1_Vtbl, 0xcc86fabe_da55_401d_85e7_e3c9de2877e9);
impl core::ops::Deref for ID3D11BlendState1 {
    type Target = ID3D11BlendState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11BlendState1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11BlendState);
impl ID3D11BlendState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_BLEND_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11BlendState1 {}
unsafe impl Sync for ID3D11BlendState1 {}
#[repr(C)]
pub struct ID3D11BlendState1_Vtbl {
    pub base__: ID3D11BlendState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BLEND_DESC1),
}
windows_core::imp::define_interface!(ID3D11Buffer, ID3D11Buffer_Vtbl, 0x48570b85_d1ee_4fcd_a250_eb350722b037);
impl core::ops::Deref for ID3D11Buffer {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Buffer, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Buffer {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_BUFFER_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Buffer {}
unsafe impl Sync for ID3D11Buffer {}
#[repr(C)]
pub struct ID3D11Buffer_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BUFFER_DESC),
}
windows_core::imp::define_interface!(ID3D11ClassInstance, ID3D11ClassInstance_Vtbl, 0xa6cd7faa_b0b7_4a2f_9436_8662a65797cb);
impl core::ops::Deref for ID3D11ClassInstance {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ClassInstance, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11ClassInstance {
    pub unsafe fn GetClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassLinkage)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
    pub unsafe fn GetInstanceName(&self, pinstancename: windows_core::PSTR, pbufferlength: *mut usize) {
        (windows_core::Interface::vtable(self).GetInstanceName)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstancename), pbufferlength)
    }
    pub unsafe fn GetTypeName(&self, ptypename: windows_core::PSTR, pbufferlength: *mut usize) {
        (windows_core::Interface::vtable(self).GetTypeName)(windows_core::Interface::as_raw(self), core::mem::transmute(ptypename), pbufferlength)
    }
}
unsafe impl Send for ID3D11ClassInstance {}
unsafe impl Sync for ID3D11ClassInstance {}
#[repr(C)]
pub struct ID3D11ClassInstance_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetClassLinkage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_CLASS_INSTANCE_DESC),
    pub GetInstanceName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut usize),
    pub GetTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut usize),
}
windows_core::imp::define_interface!(ID3D11ClassLinkage, ID3D11ClassLinkage_Vtbl, 0xddf57cba_9543_46e4_a12b_f207a0fe7fed);
impl core::ops::Deref for ID3D11ClassLinkage {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ClassLinkage, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11ClassLinkage {
    pub unsafe fn GetClassInstance<P0>(&self, pclassinstancename: P0, instanceindex: u32) -> windows_core::Result<ID3D11ClassInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassInstance)(windows_core::Interface::as_raw(self), pclassinstancename.param().abi(), instanceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateClassInstance<P0>(&self, pclasstypename: P0, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> windows_core::Result<ID3D11ClassInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateClassInstance)(windows_core::Interface::as_raw(self), pclasstypename.param().abi(), constantbufferoffset, constantvectoroffset, textureoffset, sampleroffset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D11ClassLinkage {}
unsafe impl Sync for ID3D11ClassLinkage {}
#[repr(C)]
pub struct ID3D11ClassLinkage_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetClassInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11CommandList, ID3D11CommandList_Vtbl, 0xa24bc4d1_769e_43f7_8013_98ff566c18e2);
impl core::ops::Deref for ID3D11CommandList {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11CommandList, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11CommandList {
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetContextFlags)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D11CommandList {}
unsafe impl Sync for ID3D11CommandList {}
#[repr(C)]
pub struct ID3D11CommandList_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID3D11ComputeShader, ID3D11ComputeShader_Vtbl, 0x4f5b196e_c2bd_495e_bd01_1fded38e4969);
impl core::ops::Deref for ID3D11ComputeShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ComputeShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11ComputeShader {}
unsafe impl Send for ID3D11ComputeShader {}
unsafe impl Sync for ID3D11ComputeShader {}
#[repr(C)]
pub struct ID3D11ComputeShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11Counter, ID3D11Counter_Vtbl, 0x6e8c49fb_a371_4770_b440_29086022b741);
impl core::ops::Deref for ID3D11Counter {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Counter, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous);
impl ID3D11Counter {
    pub unsafe fn GetDesc(&self) -> D3D11_COUNTER_DESC {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11Counter {}
unsafe impl Sync for ID3D11Counter {}
#[repr(C)]
pub struct ID3D11Counter_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_DESC),
}
windows_core::imp::define_interface!(ID3D11CryptoSession, ID3D11CryptoSession_Vtbl, 0x9b32f9ad_bdcc_40a6_a39d_d5c865845720);
impl core::ops::Deref for ID3D11CryptoSession {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11CryptoSession, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11CryptoSession {
    pub unsafe fn GetCryptoType(&self) -> windows_core::GUID {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCryptoType)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetDecoderProfile(&self) -> windows_core::GUID {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDecoderProfile)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetCertificateSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCertificate(&self, pcertificate: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), pcertificate.len().try_into().unwrap(), core::mem::transmute(pcertificate.as_ptr())).ok()
    }
    pub unsafe fn GetCryptoSessionHandle(&self) -> super::super::Foundation::HANDLE {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCryptoSessionHandle)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11CryptoSession {}
unsafe impl Sync for ID3D11CryptoSession {}
#[repr(C)]
pub struct ID3D11CryptoSession_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetCryptoType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
    pub GetDecoderProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
    pub GetCertificateSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub GetCryptoSessionHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE),
}
windows_core::imp::define_interface!(ID3D11Debug, ID3D11Debug_Vtbl, 0x79cf2233_7536_4948_9d36_1e4692dc5760);
impl core::ops::Deref for ID3D11Debug {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Debug, windows_core::IUnknown);
impl ID3D11Debug {
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
    pub unsafe fn ValidateContext<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11DeviceContext>,
    {
        (windows_core::Interface::vtable(self).ValidateContext)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok()
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D11_RLDO_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReportLiveDeviceObjects)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn ValidateContextForDispatch<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11DeviceContext>,
    {
        (windows_core::Interface::vtable(self).ValidateContextForDispatch)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok()
    }
}
unsafe impl Send for ID3D11Debug {}
unsafe impl Sync for ID3D11Debug {}
#[repr(C)]
pub struct ID3D11Debug_Vtbl {
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
    pub ValidateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportLiveDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_RLDO_FLAGS) -> windows_core::HRESULT,
    pub ValidateContextForDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11DepthStencilState, ID3D11DepthStencilState_Vtbl, 0x03823efb_8d8f_4e1c_9aa2_f64bb2cbfdf1);
impl core::ops::Deref for ID3D11DepthStencilState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DepthStencilState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DepthStencilState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11DepthStencilState {}
unsafe impl Sync for ID3D11DepthStencilState {}
#[repr(C)]
pub struct ID3D11DepthStencilState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_DEPTH_STENCIL_DESC),
}
windows_core::imp::define_interface!(ID3D11DepthStencilView, ID3D11DepthStencilView_Vtbl, 0x9fdac92a_1876_48c3_afad_25b94f84a9b6);
impl core::ops::Deref for ID3D11DepthStencilView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DepthStencilView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11DepthStencilView {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11DepthStencilView {}
unsafe impl Sync for ID3D11DepthStencilView {}
#[repr(C)]
pub struct ID3D11DepthStencilView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11Device, ID3D11Device_Vtbl, 0xdb6f6ddb_ac77_4e88_8253_819df9bbf140);
impl core::ops::Deref for ID3D11Device {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
impl ID3D11Device {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: Option<*mut Option<ID3D11Buffer>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBuffer)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(ppbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: Option<*mut Option<ID3D11Texture1D>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTexture1D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(pptexture1d.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: Option<*mut Option<ID3D11Texture2D>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTexture2D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(pptexture2d.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: Option<*mut Option<ID3D11Texture3D>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTexture3D)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(pptexture3d.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: Option<*mut Option<ID3D11ShaderResourceView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppsrview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: Option<*mut Option<ID3D11UnorderedAccessView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateUnorderedAccessView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppuaview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: Option<*mut Option<ID3D11RenderTargetView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(pprtview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(ppdepthstencilview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: Option<*mut Option<ID3D11InputLayout>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateInputLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len().try_into().unwrap(), core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len().try_into().unwrap(), core::mem::transmute(ppinputlayout.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: Option<*mut Option<ID3D11VertexShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(ppvertexshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateGeometryShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(ppgeometryshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(
            windows_core::Interface::as_raw(self),
            core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len().try_into().unwrap(),
            core::mem::transmute(psodeclaration.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pbufferstrides.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            rasterizedstream,
            pclasslinkage.param().abi(),
            core::mem::transmute(ppgeometryshader.unwrap_or(std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: Option<*mut Option<ID3D11PixelShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(pppixelshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: Option<*mut Option<ID3D11HullShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateHullShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(pphullshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: Option<*mut Option<ID3D11DomainShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateDomainShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(ppdomainshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: Option<*mut Option<ID3D11ComputeShader>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ClassLinkage>,
    {
        (windows_core::Interface::vtable(self).CreateComputeShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pclasslinkage.param().abi(), core::mem::transmute(ppcomputeshader.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateClassLinkage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: Option<*mut Option<ID3D11BlendState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBlendState)(windows_core::Interface::as_raw(self), pblendstatedesc, core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencildesc, core::mem::transmute(ppdepthstencilstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateRasterizerState)(windows_core::Interface::as_raw(self), prasterizerdesc, core::mem::transmute(pprasterizerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: Option<*mut Option<ID3D11SamplerState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateSamplerState)(windows_core::Interface::as_raw(self), psamplerdesc, core::mem::transmute(ppsamplerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: Option<*mut Option<ID3D11Query>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), pquerydesc, core::mem::transmute(ppquery.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: Option<*mut Option<ID3D11Predicate>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreatePredicate)(windows_core::Interface::as_raw(self), ppredicatedesc, core::mem::transmute(pppredicate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: Option<*mut Option<ID3D11Counter>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateCounter)(windows_core::Interface::as_raw(self), pcounterdesc, core::mem::transmute(ppcounter.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDeferredContext)(windows_core::Interface::as_raw(self), contextflags, core::mem::transmute(ppdeferredcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
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
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCounterInfo)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: Option<*mut u32>, szunits: windows_core::PSTR, punitslength: Option<*mut u32>, szdescription: windows_core::PSTR, pdescriptionlength: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckCounter)(windows_core::Interface::as_raw(self), pdesc, ptype, pactivecounters, core::mem::transmute(szname), core::mem::transmute(pnamelength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szunits), core::mem::transmute(punitslength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szdescription), core::mem::transmute(pdescriptionlength.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
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
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> windows_core::Result<ID3D11DeviceContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImmediateContext)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D11Device {}
unsafe impl Sync for ID3D11Device {}
#[repr(C)]
pub struct ID3D11Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture1D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE1D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture1D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture2D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture2D: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture3D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE3D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture3D: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateUnorderedAccessView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_DEPTH_STENCIL_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateDepthStencilView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateInputLayout: usize,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const D3D11_SO_DECLARATION_ENTRY, u32, *const u32, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateHullShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDomainShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComputeShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassLinkage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BLEND_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_DEPTH_STENCIL_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_SAMPLER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_COUNTER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDeferredContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenSharedResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_COUNTER_DESC, *mut D3D11_COUNTER_TYPE, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetFeatureLevel: usize,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID3D11Device1, ID3D11Device1_Vtbl, 0xa04bfb29_08ef_43d6_a49c_a9bdbdcbe686);
impl core::ops::Deref for ID3D11Device1 {
    type Target = ID3D11Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device1, windows_core::IUnknown, ID3D11Device);
impl ID3D11Device1 {
    pub unsafe fn GetImmediateContext1(&self) -> windows_core::Result<ID3D11DeviceContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImmediateContext1)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDeferredContext1)(windows_core::Interface::as_raw(self), contextflags, core::mem::transmute(ppdeferredcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: Option<*mut Option<ID3D11BlendState1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateBlendState1)(windows_core::Interface::as_raw(self), pblendstatedesc, core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateRasterizerState1)(windows_core::Interface::as_raw(self), prasterizerdesc, core::mem::transmute(pprasterizerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppcontextstate: Option<*mut Option<ID3DDeviceContextState>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDeviceContextState)(windows_core::Interface::as_raw(self), flags, core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len().try_into().unwrap(), sdkversion, emulatedinterface, core::mem::transmute(pchosenfeaturelevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppcontextstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn OpenSharedResource1<P0, T>(&self, hresource: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenSharedResource1)(windows_core::Interface::as_raw(self), hresource.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenSharedResourceByName)(windows_core::Interface::as_raw(self), lpname.param().abi(), dwdesiredaccess, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D11Device1 {}
unsafe impl Sync for ID3D11Device1 {}
#[repr(C)]
pub struct ID3D11Device1_Vtbl {
    pub base__: ID3D11Device_Vtbl,
    pub GetImmediateContext1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BLEND_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub CreateDeviceContextState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Direct3D::D3D_FEATURE_LEVEL, u32, u32, *const windows_core::GUID, *mut super::Direct3D::D3D_FEATURE_LEVEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    CreateDeviceContextState: usize,
    pub OpenSharedResource1: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenSharedResourceByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11Device2, ID3D11Device2_Vtbl, 0x9d06dffa_d1e5_4d07_83a8_1bb123f2f841);
impl core::ops::Deref for ID3D11Device2 {
    type Target = ID3D11Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device2, windows_core::IUnknown, ID3D11Device, ID3D11Device1);
impl ID3D11Device2 {
    pub unsafe fn GetImmediateContext2(&self) -> windows_core::Result<ID3D11DeviceContext2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImmediateContext2)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext2>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDeferredContext2)(windows_core::Interface::as_raw(self), contextflags, core::mem::transmute(ppdeferredcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: Option<*mut u32>, ppackedmipdesc: Option<*mut D3D11_PACKED_MIP_DESC>, pstandardtileshapefornonpackedmips: Option<*mut D3D11_TILE_SHAPE>, pnumsubresourcetilings: Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).GetResourceTiling)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), core::mem::transmute(pnumtilesforentireresource.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppackedmipdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumsubresourcetilings.unwrap_or(std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels1)(windows_core::Interface::as_raw(self), format, samplecount, flags, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11Device2 {}
unsafe impl Sync for ID3D11Device2 {}
#[repr(C)]
pub struct ID3D11Device2_Vtbl {
    pub base__: ID3D11Device1_Vtbl,
    pub GetImmediateContext2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourceTiling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut D3D11_PACKED_MIP_DESC, *mut D3D11_TILE_SHAPE, *mut u32, u32, *mut D3D11_SUBRESOURCE_TILING),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckMultisampleQualityLevels1: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckMultisampleQualityLevels1: usize,
}
windows_core::imp::define_interface!(ID3D11Device3, ID3D11Device3_Vtbl, 0xa05c8c37_d2c6_4732_b3a0_9ce0b0dc9ae6);
impl core::ops::Deref for ID3D11Device3 {
    type Target = ID3D11Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device3, windows_core::IUnknown, ID3D11Device, ID3D11Device1, ID3D11Device2);
impl ID3D11Device3 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: Option<*mut Option<ID3D11Texture2D1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTexture2D1)(windows_core::Interface::as_raw(self), pdesc1, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(pptexture2d.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: Option<*mut Option<ID3D11Texture3D1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTexture3D1)(windows_core::Interface::as_raw(self), pdesc1, core::mem::transmute(pinitialdata.unwrap_or(std::ptr::null())), core::mem::transmute(pptexture3d.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState2>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateRasterizerState2)(windows_core::Interface::as_raw(self), prasterizerdesc, core::mem::transmute(pprasterizerstate.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC1>, ppsrview1: Option<*mut Option<ID3D11ShaderResourceView1>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateShaderResourceView1)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc1.unwrap_or(std::ptr::null())), core::mem::transmute(ppsrview1.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC1>, ppuaview1: Option<*mut Option<ID3D11UnorderedAccessView1>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateUnorderedAccessView1)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc1.unwrap_or(std::ptr::null())), core::mem::transmute(ppuaview1.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_RENDER_TARGET_VIEW_DESC1>, pprtview1: Option<*mut Option<ID3D11RenderTargetView1>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateRenderTargetView1)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc1.unwrap_or(std::ptr::null())), core::mem::transmute(pprtview1.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: Option<*mut Option<ID3D11Query1>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateQuery1)(windows_core::Interface::as_raw(self), pquerydesc1, core::mem::transmute(ppquery1.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetImmediateContext3(&self) -> windows_core::Result<ID3D11DeviceContext3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImmediateContext3)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext3>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateDeferredContext3)(windows_core::Interface::as_raw(self), contextflags, core::mem::transmute(ppdeferredcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteToSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).WriteToSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, core::mem::transmute(pdstbox.unwrap_or(std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ReadFromSubresource<P0>(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: P0, srcsubresource: u32, psrcbox: Option<*const D3D11_BOX>)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).ReadFromSubresource)(windows_core::Interface::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, psrcresource.param().abi(), srcsubresource, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null())))
    }
}
unsafe impl Send for ID3D11Device3 {}
unsafe impl Sync for ID3D11Device3 {}
#[repr(C)]
pub struct ID3D11Device3_Vtbl {
    pub base__: ID3D11Device2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture2D1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE2D_DESC1, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture2D1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateTexture3D1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE3D_DESC1, *const D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateTexture3D1: usize,
    pub CreateRasterizerState2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateShaderResourceView1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateUnorderedAccessView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateUnorderedAccessView1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_RENDER_TARGET_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView1: usize,
    pub CreateQuery1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteToSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_BOX, *const core::ffi::c_void, u32, u32),
    pub ReadFromSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, u32, *const D3D11_BOX),
}
windows_core::imp::define_interface!(ID3D11Device4, ID3D11Device4_Vtbl, 0x8992ab71_02e6_4b8d_ba48_b056dcda42c4);
impl core::ops::Deref for ID3D11Device4 {
    type Target = ID3D11Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device4, windows_core::IUnknown, ID3D11Device, ID3D11Device1, ID3D11Device2, ID3D11Device3);
impl ID3D11Device4 {
    pub unsafe fn RegisterDeviceRemovedEvent<P0>(&self, hevent: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterDeviceRemovedEvent)(windows_core::Interface::as_raw(self), hevent.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnregisterDeviceRemoved(&self, dwcookie: u32) {
        (windows_core::Interface::vtable(self).UnregisterDeviceRemoved)(windows_core::Interface::as_raw(self), dwcookie)
    }
}
unsafe impl Send for ID3D11Device4 {}
unsafe impl Sync for ID3D11Device4 {}
#[repr(C)]
pub struct ID3D11Device4_Vtbl {
    pub base__: ID3D11Device3_Vtbl,
    pub RegisterDeviceRemovedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ID3D11Device5, ID3D11Device5_Vtbl, 0x8ffde202_a0e7_45df_9e01_e837801b5ea0);
impl core::ops::Deref for ID3D11Device5 {
    type Target = ID3D11Device4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Device5, windows_core::IUnknown, ID3D11Device, ID3D11Device1, ID3D11Device2, ID3D11Device3, ID3D11Device4);
impl ID3D11Device5 {
    pub unsafe fn OpenSharedFence<P0, T>(&self, hfence: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).OpenSharedFence)(windows_core::Interface::as_raw(self), hfence.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D11_FENCE_FLAG, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateFence)(windows_core::Interface::as_raw(self), initialvalue, flags, &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D11Device5 {}
unsafe impl Sync for ID3D11Device5 {}
#[repr(C)]
pub struct ID3D11Device5_Vtbl {
    pub base__: ID3D11Device4_Vtbl,
    pub OpenSharedFence: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFence: unsafe extern "system" fn(*mut core::ffi::c_void, u64, D3D11_FENCE_FLAG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11DeviceChild, ID3D11DeviceChild_Vtbl, 0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae);
impl core::ops::Deref for ID3D11DeviceChild {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
impl ID3D11DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D11Device> {
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
unsafe impl Send for ID3D11DeviceChild {}
unsafe impl Sync for ID3D11DeviceChild {}
#[repr(C)]
pub struct ID3D11DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11DeviceContext, ID3D11DeviceContext_Vtbl, 0xc0bfa96c_e089_44fb_8eaf_26f8796190da);
impl core::ops::Deref for ID3D11DeviceContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DeviceContext {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).VSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).PSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11PixelShader>,
    {
        (windows_core::Interface::vtable(self).PSSetShader)(windows_core::Interface::as_raw(self), ppixelshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).PSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11VertexShader>,
    {
        (windows_core::Interface::vtable(self).VSSetShader)(windows_core::Interface::as_raw(self), pvertexshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, maptype, mapflags, core::mem::transmute(pmappedresource.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).PSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D11InputLayout>,
    {
        (windows_core::Interface::vtable(self).IASetInputLayout)(windows_core::Interface::as_raw(self), pinputlayout.param().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*const Option<ID3D11Buffer>>, pstrides: Option<*const u32>, poffsets: Option<*const u32>) {
        (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppvertexbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pstrides.unwrap_or(std::ptr::null())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.param().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).GSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11GeometryShader>,
    {
        (windows_core::Interface::vtable(self).GSSetShader)(windows_core::Interface::as_raw(self), pshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).VSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).VSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), pasync.param().abi())
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), pasync.param().abi())
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: Option<*mut core::ffi::c_void>, datasize: u32, getdataflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pasync.param().abi(), core::mem::transmute(pdata.unwrap_or(std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: windows_core::Param<ID3D11Predicate>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), ppredicate.param().abi(), predicatevalue.param().abi())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).GSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).GSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>, pdepthstencilview: P0)
    where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi())
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P0>(&self, pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>, pdepthstencilview: P0, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>, puavinitialcounts: Option<*const u32>)
    where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        (windows_core::Interface::vtable(self).OMSetRenderTargetsAndUnorderedAccessViews)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi(), uavstartslot, numuavs, core::mem::transmute(ppunorderedaccessviews.unwrap_or(std::ptr::null())), core::mem::transmute(puavinitialcounts.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: Option<&[f32; 4]>, samplemask: u32)
    where
        P0: windows_core::Param<ID3D11BlendState>,
    {
        (windows_core::Interface::vtable(self).OMSetBlendState)(windows_core::Interface::as_raw(self), pblendstate.param().abi(), core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D11DepthStencilState>,
    {
        (windows_core::Interface::vtable(self).OMSetDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencilstate.param().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: Option<*const Option<ID3D11Buffer>>, poffsets: Option<*const u32>) {
        (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), numbuffers, core::mem::transmute(ppsotargets.unwrap_or(std::ptr::null())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).DrawIndexedInstancedIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).DrawInstancedIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).DispatchIndirect)(windows_core::Interface::as_raw(self), pbufferforargs.param().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D11RasterizerState>,
    {
        (windows_core::Interface::vtable(self).RSSetState)(windows_core::Interface::as_raw(self), prasterizerstate.param().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D11_VIEWPORT]>) {
        (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviewports.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[super::super::Foundation::RECT]>) {
        (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: Option<*const D3D11_BOX>)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CopySubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).UpdateSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, core::mem::transmute(pdstbox.unwrap_or(std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn CopyStructureCount<P0, P1>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P1)
    where
        P0: windows_core::Param<ID3D11Buffer>,
        P1: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).CopyStructureCount)(windows_core::Interface::as_raw(self), pdstbuffer.param().abi(), dstalignedbyteoffset, psrcview.param().abi())
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D11RenderTargetView>,
    {
        (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), prendertargetview.param().abi(), core::mem::transmute(colorrgba.as_ptr()))
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: &[u32; 4])
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).ClearUnorderedAccessViewUint)(windows_core::Interface::as_raw(self), punorderedaccessview.param().abi(), core::mem::transmute(values.as_ptr()))
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: &[f32; 4])
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).ClearUnorderedAccessViewFloat)(windows_core::Interface::as_raw(self), punorderedaccessview.param().abi(), core::mem::transmute(values.as_ptr()))
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), pdepthstencilview.param().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D11ShaderResourceView>,
    {
        (windows_core::Interface::vtable(self).GenerateMips)(windows_core::Interface::as_raw(self), pshaderresourceview.param().abi())
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).SetResourceMinLOD)(windows_core::Interface::as_raw(self), presource.param().abi(), minlod)
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).GetResourceMinLOD)(windows_core::Interface::as_raw(self), presource.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format)
    }
    pub unsafe fn ExecuteCommandList<P0, P1>(&self, pcommandlist: P0, restorecontextstate: P1)
    where
        P0: windows_core::Param<ID3D11CommandList>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ExecuteCommandList)(windows_core::Interface::as_raw(self), pcommandlist.param().abi(), restorecontextstate.param().abi())
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).HSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11HullShader>,
    {
        (windows_core::Interface::vtable(self).HSSetShader)(windows_core::Interface::as_raw(self), phullshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).HSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).HSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).DSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11DomainShader>,
    {
        (windows_core::Interface::vtable(self).DSSetShader)(windows_core::Interface::as_raw(self), pdomainshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).DSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).DSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).CSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>, puavinitialcounts: Option<*const u32>) {
        (windows_core::Interface::vtable(self).CSSetUnorderedAccessViews)(windows_core::Interface::as_raw(self), startslot, numuavs, core::mem::transmute(ppunorderedaccessviews.unwrap_or(std::ptr::null())), core::mem::transmute(puavinitialcounts.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>)
    where
        P0: windows_core::Param<ID3D11ComputeShader>,
    {
        (windows_core::Interface::vtable(self).CSSetShader)(windows_core::Interface::as_raw(self), pcomputeshader.param().abi(), core::mem::transmute(ppclassinstances.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).CSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).CSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).VSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).PSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut Option<ID3D11PixelShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).PSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pppixelshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).PSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut Option<ID3D11VertexShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).VSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppvertexshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).PSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D11InputLayout> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IAGetInputLayout)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*mut Option<ID3D11Buffer>>, pstrides: Option<*mut u32>, poffsets: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).IAGetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppvertexbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstrides.unwrap_or(std::ptr::null_mut())), core::mem::transmute(poffsets.unwrap_or(std::ptr::null_mut())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: Option<*mut Option<ID3D11Buffer>>, format: Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).IAGetIndexBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pindexbuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(format.unwrap_or(std::ptr::null_mut())), core::mem::transmute(offset.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).GSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut Option<ID3D11GeometryShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).GSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppgeometryshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).VSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).VSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GetPredication(&self, pppredicate: Option<*mut Option<ID3D11Predicate>>, ppredicatevalue: Option<*mut super::super::Foundation::BOOL>) {
        (windows_core::Interface::vtable(self).GetPredication)(windows_core::Interface::as_raw(self), core::mem::transmute(pppredicate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppredicatevalue.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).GSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).GSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: Option<&mut [Option<ID3D11RenderTargetView>]>, ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>) {
        (windows_core::Interface::vtable(self).OMGetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(ppdepthstencilview.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: Option<&mut [Option<ID3D11RenderTargetView>]>, ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>, uavstartslot: u32, ppunorderedaccessviews: Option<&mut [Option<ID3D11UnorderedAccessView>]>) {
        (windows_core::Interface::vtable(self).OMGetRenderTargetsAndUnorderedAccessViews)(
            windows_core::Interface::as_raw(self),
            pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            core::mem::transmute(ppdepthstencilview.unwrap_or(std::ptr::null_mut())),
            uavstartslot,
            ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: Option<*mut Option<ID3D11BlendState>>, blendfactor: Option<&mut [f32; 4]>, psamplemask: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).OMGetBlendState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppblendstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(psamplemask.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>, pstencilref: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).OMGetDepthStencilState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppdepthstencilstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstencilref.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).SOGetTargets)(windows_core::Interface::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsotargets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D11RasterizerState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RSGetState)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: Option<*mut D3D11_VIEWPORT>) {
        (windows_core::Interface::vtable(self).RSGetViewports)(windows_core::Interface::as_raw(self), pnumviewports, core::mem::transmute(pviewports.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: Option<*mut super::super::Foundation::RECT>) {
        (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), pnumrects, core::mem::transmute(prects.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).HSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut Option<ID3D11HullShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).HSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pphullshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).HSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).HSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).DSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut Option<ID3D11DomainShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).DSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppdomainshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).DSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).DSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D11ShaderResourceView>]>) {
        (windows_core::Interface::vtable(self).CSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: Option<&mut [Option<ID3D11UnorderedAccessView>]>) {
        (windows_core::Interface::vtable(self).CSGetUnorderedAccessViews)(windows_core::Interface::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut Option<ID3D11ComputeShader>, ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>, pnumclassinstances: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).CSGetShader)(windows_core::Interface::as_raw(self), core::mem::transmute(ppcomputeshader), core::mem::transmute(ppclassinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumclassinstances.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D11SamplerState>]>) {
        (windows_core::Interface::vtable(self).CSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D11Buffer>]>) {
        (windows_core::Interface::vtable(self).CSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearState(&self) {
        (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetContextFlags)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn FinishCommandList<P0>(&self, restoredeferredcontextstate: P0, ppcommandlist: Option<*mut Option<ID3D11CommandList>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).FinishCommandList)(windows_core::Interface::as_raw(self), restoredeferredcontextstate.param().abi(), core::mem::transmute(ppcommandlist.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
unsafe impl Send for ID3D11DeviceContext {}
unsafe impl Sync for ID3D11DeviceContext {}
#[repr(C)]
pub struct ID3D11DeviceContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub PSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
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
    pub GSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IASetPrimitiveTopology: usize,
    pub VSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub GSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32),
    pub OMSetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const u32),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DrawIndexedInstancedIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub DrawInstancedIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub DispatchIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_VIEWPORT),
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT),
    pub CopySubresourceRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const D3D11_BOX),
    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub UpdateSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_BOX, *const core::ffi::c_void, u32, u32),
    pub CopyStructureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub ClearRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearUnorderedAccessViewUint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u32),
    pub ClearUnorderedAccessViewFloat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32),
    pub GetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresource: usize,
    pub ExecuteCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub HSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub HSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub DSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32),
    pub CSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32),
    pub CSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub PSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub PSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IAGetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::Dxgi::Common::DXGI_FORMAT, *mut u32),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
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
    pub OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut u32),
    pub OMGetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_VIEWPORT),
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::RECT),
    pub HSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub HSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub DSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetUnorderedAccessViews: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub CSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FinishCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11DeviceContext1, ID3D11DeviceContext1_Vtbl, 0xbb2c6faa_b5fb_4082_8e6b_388b8cfa90e1);
impl core::ops::Deref for ID3D11DeviceContext1 {
    type Target = ID3D11DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11DeviceContext);
impl ID3D11DeviceContext1 {
    pub unsafe fn CopySubresourceRegion1<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: Option<*const D3D11_BOX>, copyflags: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CopySubresourceRegion1)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null())), copyflags)
    }
    pub unsafe fn UpdateSubresource1<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).UpdateSubresource1)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, core::mem::transmute(pdstbox.unwrap_or(std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch, copyflags)
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi())
    }
    pub unsafe fn DiscardView<P0>(&self, presourceview: P0)
    where
        P0: windows_core::Param<ID3D11View>,
    {
        (windows_core::Interface::vtable(self).DiscardView)(windows_core::Interface::as_raw(self), presourceview.param().abi())
    }
    pub unsafe fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).VSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).HSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).DSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).GSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).PSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        (windows_core::Interface::vtable(self).CSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).VSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).HSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).DSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).GSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).PSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        (windows_core::Interface::vtable(self).CSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, core::mem::transmute(ppconstantbuffers.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfirstconstant.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumconstants.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn SwapDeviceContextState<P0>(&self, pstate: P0, pppreviousstate: Option<*mut Option<ID3DDeviceContextState>>)
    where
        P0: windows_core::Param<ID3DDeviceContextState>,
    {
        (windows_core::Interface::vtable(self).SwapDeviceContextState)(windows_core::Interface::as_raw(self), pstate.param().abi(), core::mem::transmute(pppreviousstate.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn ClearView<P0>(&self, pview: P0, color: &[f32; 4], prect: Option<&[super::super::Foundation::RECT]>)
    where
        P0: windows_core::Param<ID3D11View>,
    {
        (windows_core::Interface::vtable(self).ClearView)(windows_core::Interface::as_raw(self), pview.param().abi(), core::mem::transmute(color.as_ptr()), core::mem::transmute(prect.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prect.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
    pub unsafe fn DiscardView1<P0>(&self, presourceview: P0, prects: Option<&[super::super::Foundation::RECT]>)
    where
        P0: windows_core::Param<ID3D11View>,
    {
        (windows_core::Interface::vtable(self).DiscardView1)(windows_core::Interface::as_raw(self), presourceview.param().abi(), core::mem::transmute(prects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), prects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
    }
}
unsafe impl Send for ID3D11DeviceContext1 {}
unsafe impl Sync for ID3D11DeviceContext1 {}
#[repr(C)]
pub struct ID3D11DeviceContext1_Vtbl {
    pub base__: ID3D11DeviceContext_Vtbl,
    pub CopySubresourceRegion1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const D3D11_BOX, u32),
    pub UpdateSubresource1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_BOX, *const core::ffi::c_void, u32, u32, u32),
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DiscardView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub VSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub HSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub DSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub GSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub PSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub CSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub VSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub HSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub DSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub GSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub PSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub CSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub SwapDeviceContextState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub ClearView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, *const super::super::Foundation::RECT, u32),
    pub DiscardView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Foundation::RECT, u32),
}
windows_core::imp::define_interface!(ID3D11DeviceContext2, ID3D11DeviceContext2_Vtbl, 0x420d5b32_b90c_4da4_bef0_359f6a24a83a);
impl core::ops::Deref for ID3D11DeviceContext2 {
    type Target = ID3D11DeviceContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext2, windows_core::IUnknown, ID3D11DeviceChild, ID3D11DeviceContext, ID3D11DeviceContext1);
impl ID3D11DeviceContext2 {
    pub unsafe fn UpdateTileMappings<P0, P1>(&self, ptiledresource: P0, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: Option<*const D3D11_TILED_RESOURCE_COORDINATE>, ptiledresourceregionsizes: Option<*const D3D11_TILE_REGION_SIZE>, ptilepool: P1, numranges: u32, prangeflags: Option<*const u32>, ptilepoolstartoffsets: Option<*const u32>, prangetilecounts: Option<*const u32>, flags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).UpdateTileMappings)(
            windows_core::Interface::as_raw(self),
            ptiledresource.param().abi(),
            numtiledresourceregions,
            core::mem::transmute(ptiledresourceregionstartcoordinates.unwrap_or(std::ptr::null())),
            core::mem::transmute(ptiledresourceregionsizes.unwrap_or(std::ptr::null())),
            ptilepool.param().abi(),
            numranges,
            core::mem::transmute(prangeflags.unwrap_or(std::ptr::null())),
            core::mem::transmute(ptilepoolstartoffsets.unwrap_or(std::ptr::null())),
            core::mem::transmute(prangetilecounts.unwrap_or(std::ptr::null())),
            flags,
        )
        .ok()
    }
    pub unsafe fn CopyTileMappings<P0, P1>(&self, pdesttiledresource: P0, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: P1, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CopyTileMappings)(windows_core::Interface::as_raw(self), pdesttiledresource.param().abi(), pdestregionstartcoordinate, psourcetiledresource.param().abi(), psourceregionstartcoordinate, ptileregionsize, flags).ok()
    }
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).CopyTiles)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.param().abi(), bufferstartoffsetinbytes, flags)
    }
    pub unsafe fn UpdateTiles<P0>(&self, pdesttiledresource: P0, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).UpdateTiles)(windows_core::Interface::as_raw(self), pdesttiledresource.param().abi(), pdesttileregionstartcoordinate, pdesttileregionsize, psourcetiledata, flags)
    }
    pub unsafe fn ResizeTilePool<P0>(&self, ptilepool: P0, newsizeinbytes: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        (windows_core::Interface::vtable(self).ResizeTilePool)(windows_core::Interface::as_raw(self), ptilepool.param().abi(), newsizeinbytes).ok()
    }
    pub unsafe fn TiledResourceBarrier<P0, P1>(&self, ptiledresourceorviewaccessbeforebarrier: P0, ptiledresourceorviewaccessafterbarrier: P1)
    where
        P0: windows_core::Param<ID3D11DeviceChild>,
        P1: windows_core::Param<ID3D11DeviceChild>,
    {
        (windows_core::Interface::vtable(self).TiledResourceBarrier)(windows_core::Interface::as_raw(self), ptiledresourceorviewaccessbeforebarrier.param().abi(), ptiledresourceorviewaccessafterbarrier.param().abi())
    }
    pub unsafe fn IsAnnotationEnabled(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsAnnotationEnabled)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMarkerInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMarkerInt)(windows_core::Interface::as_raw(self), plabel.param().abi(), data)
    }
    pub unsafe fn BeginEventInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BeginEventInt)(windows_core::Interface::as_raw(self), plabel.param().abi(), data)
    }
    pub unsafe fn EndEvent(&self) {
        (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D11DeviceContext2 {}
unsafe impl Sync for ID3D11DeviceContext2 {}
#[repr(C)]
pub struct ID3D11DeviceContext2_Vtbl {
    pub base__: ID3D11DeviceContext1_Vtbl,
    pub UpdateTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *mut core::ffi::c_void, u32, *const u32, *const u32, *const u32, u32) -> windows_core::HRESULT,
    pub CopyTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, u32) -> windows_core::HRESULT,
    pub CopyTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *mut core::ffi::c_void, u64, u32),
    pub UpdateTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *const core::ffi::c_void, u32),
    pub ResizeTilePool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub TiledResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IsAnnotationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub SetMarkerInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32),
    pub BeginEventInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D11DeviceContext3, ID3D11DeviceContext3_Vtbl, 0xb4e3c01d_e79e_4637_91b2_510e9f4c9b8f);
impl core::ops::Deref for ID3D11DeviceContext3 {
    type Target = ID3D11DeviceContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext3, windows_core::IUnknown, ID3D11DeviceChild, ID3D11DeviceContext, ID3D11DeviceContext1, ID3D11DeviceContext2);
impl ID3D11DeviceContext3 {
    pub unsafe fn Flush1<P0>(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: P0)
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).Flush1)(windows_core::Interface::as_raw(self), contexttype, hevent.param().abi())
    }
    pub unsafe fn SetHardwareProtectionState<P0>(&self, hwprotectionenable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHardwareProtectionState)(windows_core::Interface::as_raw(self), hwprotectionenable.param().abi())
    }
    pub unsafe fn GetHardwareProtectionState(&self) -> super::super::Foundation::BOOL {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHardwareProtectionState)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11DeviceContext3 {}
unsafe impl Sync for ID3D11DeviceContext3 {}
#[repr(C)]
pub struct ID3D11DeviceContext3_Vtbl {
    pub base__: ID3D11DeviceContext2_Vtbl,
    pub Flush1: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_CONTEXT_TYPE, super::super::Foundation::HANDLE),
    pub SetHardwareProtectionState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub GetHardwareProtectionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID3D11DeviceContext4, ID3D11DeviceContext4_Vtbl, 0x917600da_f58c_4c33_98d8_3e15b390fa24);
impl core::ops::Deref for ID3D11DeviceContext4 {
    type Target = ID3D11DeviceContext3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext4, windows_core::IUnknown, ID3D11DeviceChild, ID3D11DeviceContext, ID3D11DeviceContext1, ID3D11DeviceContext2, ID3D11DeviceContext3);
impl ID3D11DeviceContext4 {
    pub unsafe fn Signal<P0>(&self, pfence: P0, value: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Fence>,
    {
        (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self), pfence.param().abi(), value).ok()
    }
    pub unsafe fn Wait<P0>(&self, pfence: P0, value: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Fence>,
    {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), pfence.param().abi(), value).ok()
    }
}
unsafe impl Send for ID3D11DeviceContext4 {}
unsafe impl Sync for ID3D11DeviceContext4 {}
#[repr(C)]
pub struct ID3D11DeviceContext4_Vtbl {
    pub base__: ID3D11DeviceContext3_Vtbl,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11DomainShader, ID3D11DomainShader_Vtbl, 0xf582c508_0f36_490c_9977_31eece268cfa);
impl core::ops::Deref for ID3D11DomainShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DomainShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DomainShader {}
unsafe impl Send for ID3D11DomainShader {}
unsafe impl Sync for ID3D11DomainShader {}
#[repr(C)]
pub struct ID3D11DomainShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11Fence, ID3D11Fence_Vtbl, 0xaffde9d1_1df7_4bb7_8a34_0f46251dab80);
impl core::ops::Deref for ID3D11Fence {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Fence, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Fence {
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P0>(&self, pattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), core::mem::transmute(pattributes.unwrap_or(std::ptr::null())), dwaccess, lpname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCompletedValue(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetCompletedValue)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetEventOnCompletion<P0>(&self, value: u64, hevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetEventOnCompletion)(windows_core::Interface::as_raw(self), value, hevent.param().abi()).ok()
    }
}
unsafe impl Send for ID3D11Fence {}
unsafe impl Sync for ID3D11Fence {}
#[repr(C)]
pub struct ID3D11Fence_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
    pub GetCompletedValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub SetEventOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11FunctionLinkingGraph, ID3D11FunctionLinkingGraph_Vtbl, 0x54133220_1ce8_43d3_8236_9855c5ceecff);
impl core::ops::Deref for ID3D11FunctionLinkingGraph {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11FunctionLinkingGraph, windows_core::IUnknown);
impl ID3D11FunctionLinkingGraph {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateModuleInstance(&self, ppmoduleinstance: *mut Option<ID3D11ModuleInstance>, pperrorbuffer: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateModuleInstance)(windows_core::Interface::as_raw(self), core::mem::transmute(ppmoduleinstance), core::mem::transmute(pperrorbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetInputSignature(&self, pinputparameters: &[D3D11_PARAMETER_DESC]) -> windows_core::Result<ID3D11LinkingNode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetInputSignature)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputparameters.as_ptr()), pinputparameters.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetOutputSignature(&self, poutputparameters: &[D3D11_PARAMETER_DESC]) -> windows_core::Result<ID3D11LinkingNode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetOutputSignature)(windows_core::Interface::as_raw(self), core::mem::transmute(poutputparameters.as_ptr()), poutputparameters.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CallFunction<P0, P1, P2>(&self, pmoduleinstancenamespace: P0, pmodulewithfunctionprototype: P1, pfunctionname: P2) -> windows_core::Result<ID3D11LinkingNode>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<ID3D11Module>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallFunction)(windows_core::Interface::as_raw(self), pmoduleinstancenamespace.param().abi(), pmodulewithfunctionprototype.param().abi(), pfunctionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PassValue<P0, P1>(&self, psrcnode: P0, srcparameterindex: i32, pdstnode: P1, dstparameterindex: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11LinkingNode>,
        P1: windows_core::Param<ID3D11LinkingNode>,
    {
        (windows_core::Interface::vtable(self).PassValue)(windows_core::Interface::as_raw(self), psrcnode.param().abi(), srcparameterindex, pdstnode.param().abi(), dstparameterindex).ok()
    }
    pub unsafe fn PassValueWithSwizzle<P0, P1, P2, P3>(&self, psrcnode: P0, srcparameterindex: i32, psrcswizzle: P1, pdstnode: P2, dstparameterindex: i32, pdstswizzle: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11LinkingNode>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<ID3D11LinkingNode>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).PassValueWithSwizzle)(windows_core::Interface::as_raw(self), psrcnode.param().abi(), srcparameterindex, psrcswizzle.param().abi(), pdstnode.param().abi(), dstparameterindex, pdstswizzle.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetLastError(&self, pperrorbuffer: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), core::mem::transmute(pperrorbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GenerateHlsl(&self, uflags: u32) -> windows_core::Result<super::Direct3D::ID3DBlob> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateHlsl)(windows_core::Interface::as_raw(self), uflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D11FunctionLinkingGraph {}
unsafe impl Sync for ID3D11FunctionLinkingGraph {}
#[repr(C)]
pub struct ID3D11FunctionLinkingGraph_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub CreateModuleInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    CreateModuleInstance: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetInputSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_PARAMETER_DESC, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetInputSignature: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetOutputSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_PARAMETER_DESC, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetOutputSignature: usize,
    pub CallFunction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PassValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PassValueWithSwizzle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, windows_core::PCSTR, *mut core::ffi::c_void, i32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetLastError: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GenerateHlsl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GenerateHlsl: usize,
}
windows_core::imp::define_interface!(ID3D11FunctionParameterReflection, ID3D11FunctionParameterReflection_Vtbl);
impl ID3D11FunctionParameterReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
}
unsafe impl Send for ID3D11FunctionParameterReflection {}
unsafe impl Sync for ID3D11FunctionParameterReflection {}
#[repr(C)]
pub struct ID3D11FunctionParameterReflection_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11FunctionReflection, ID3D11FunctionReflection_Vtbl);
impl ID3D11FunctionReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), bufferindex)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc).ok()
    }
    pub unsafe fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection> {
        (windows_core::Interface::vtable(self).GetFunctionParameter)(windows_core::Interface::as_raw(self), parameterindex)
    }
}
unsafe impl Send for ID3D11FunctionReflection {}
unsafe impl Sync for ID3D11FunctionReflection {}
#[repr(C)]
pub struct ID3D11FunctionReflection_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_FUNCTION_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetFunctionParameter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D11FunctionParameterReflection>,
}
windows_core::imp::define_interface!(ID3D11GeometryShader, ID3D11GeometryShader_Vtbl, 0x38325b96_effb_4022_ba02_2e795b70275c);
impl core::ops::Deref for ID3D11GeometryShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11GeometryShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11GeometryShader {}
unsafe impl Send for ID3D11GeometryShader {}
unsafe impl Sync for ID3D11GeometryShader {}
#[repr(C)]
pub struct ID3D11GeometryShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11HullShader, ID3D11HullShader_Vtbl, 0x8e5c6061_628a_4c8e_8264_bbe45cb3d5dd);
impl core::ops::Deref for ID3D11HullShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11HullShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11HullShader {}
unsafe impl Send for ID3D11HullShader {}
unsafe impl Sync for ID3D11HullShader {}
#[repr(C)]
pub struct ID3D11HullShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11InfoQueue, ID3D11InfoQueue_Vtbl, 0x6543dbb6_1b48_42f5_ab82_e97ec74326f6);
impl core::ops::Deref for ID3D11InfoQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11InfoQueue, windows_core::IUnknown);
impl ID3D11InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: Option<*mut D3D11_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: Option<*mut D3D11_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: Option<*mut D3D11_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddMessage<P0>(&self, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), category, severity, id, pdescription.param().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: D3D11_MESSAGE_SEVERITY, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnCategory<P0>(&self, category: D3D11_MESSAGE_CATEGORY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), category, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnSeverity<P0>(&self, severity: D3D11_MESSAGE_SEVERITY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnID<P0>(&self, id: D3D11_MESSAGE_ID, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), id, benable.param().abi()).ok()
    }
    pub unsafe fn GetBreakOnCategory(&self, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), category)
    }
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity)
    }
    pub unsafe fn GetBreakOnID(&self, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
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
unsafe impl Send for ID3D11InfoQueue {}
unsafe impl Sync for ID3D11InfoQueue {}
#[repr(C)]
pub struct ID3D11InfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut D3D11_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_CATEGORY, D3D11_MESSAGE_SEVERITY, D3D11_MESSAGE_ID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_CATEGORY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_SEVERITY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_ID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D11InputLayout, ID3D11InputLayout_Vtbl, 0xe4819ddc_4cf0_4025_bd26_5de82a3e07b7);
impl core::ops::Deref for ID3D11InputLayout {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11InputLayout, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11InputLayout {}
unsafe impl Send for ID3D11InputLayout {}
unsafe impl Sync for ID3D11InputLayout {}
#[repr(C)]
pub struct ID3D11InputLayout_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11LibraryReflection, ID3D11LibraryReflection_Vtbl, 0x54384f1b_5b3e_4bb7_ae01_60ba3097cbb6);
impl core::ops::Deref for ID3D11LibraryReflection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11LibraryReflection, windows_core::IUnknown);
impl ID3D11LibraryReflection {
    pub unsafe fn GetDesc(&self) -> windows_core::Result<D3D11_LIBRARY_DESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D11FunctionReflection> {
        (windows_core::Interface::vtable(self).GetFunctionByIndex)(windows_core::Interface::as_raw(self), functionindex)
    }
}
unsafe impl Send for ID3D11LibraryReflection {}
unsafe impl Sync for ID3D11LibraryReflection {}
#[repr(C)]
pub struct ID3D11LibraryReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_LIBRARY_DESC) -> windows_core::HRESULT,
    pub GetFunctionByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D11FunctionReflection>,
}
windows_core::imp::define_interface!(ID3D11Linker, ID3D11Linker_Vtbl, 0x59a6cd0e_e10d_4c1f_88c0_63aba1daf30e);
impl core::ops::Deref for ID3D11Linker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Linker, windows_core::IUnknown);
impl ID3D11Linker {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn Link<P0, P1, P2>(&self, pentry: P0, pentryname: P1, ptargetname: P2, uflags: u32, ppshaderblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorbuffer: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ModuleInstance>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).Link)(windows_core::Interface::as_raw(self), pentry.param().abi(), pentryname.param().abi(), ptargetname.param().abi(), uflags, core::mem::transmute(ppshaderblob), core::mem::transmute(pperrorbuffer.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UseLibrary<P0>(&self, plibrarymi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ModuleInstance>,
    {
        (windows_core::Interface::vtable(self).UseLibrary)(windows_core::Interface::as_raw(self), plibrarymi.param().abi()).ok()
    }
    pub unsafe fn AddClipPlaneFromCBuffer(&self, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddClipPlaneFromCBuffer)(windows_core::Interface::as_raw(self), ucbufferslot, ucbufferentry).ok()
    }
}
unsafe impl Send for ID3D11Linker {}
unsafe impl Sync for ID3D11Linker {}
#[repr(C)]
pub struct ID3D11Linker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub Link: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    Link: usize,
    pub UseLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddClipPlaneFromCBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11LinkingNode, ID3D11LinkingNode_Vtbl, 0xd80dd70c_8d2f_4751_94a1_03c79b3556db);
impl core::ops::Deref for ID3D11LinkingNode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11LinkingNode, windows_core::IUnknown);
impl ID3D11LinkingNode {}
unsafe impl Send for ID3D11LinkingNode {}
unsafe impl Sync for ID3D11LinkingNode {}
#[repr(C)]
pub struct ID3D11LinkingNode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(ID3D11Module, ID3D11Module_Vtbl, 0xcac701ee_80fc_4122_8242_10b39c8cec34);
impl core::ops::Deref for ID3D11Module {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Module, windows_core::IUnknown);
impl ID3D11Module {
    pub unsafe fn CreateInstance<P0>(&self, pnamespace: P0) -> windows_core::Result<ID3D11ModuleInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D11Module {}
unsafe impl Sync for ID3D11Module {}
#[repr(C)]
pub struct ID3D11Module_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11ModuleInstance, ID3D11ModuleInstance_Vtbl, 0x469e07f7_045a_48d5_aa12_68a478cdf75d);
impl core::ops::Deref for ID3D11ModuleInstance {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ModuleInstance, windows_core::IUnknown);
impl ID3D11ModuleInstance {
    pub unsafe fn BindConstantBuffer(&self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindConstantBuffer)(windows_core::Interface::as_raw(self), usrcslot, udstslot, cbdstoffset).ok()
    }
    pub unsafe fn BindConstantBufferByName<P0>(&self, pname: P0, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).BindConstantBufferByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, cbdstoffset).ok()
    }
    pub unsafe fn BindResource(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindResource)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount).ok()
    }
    pub unsafe fn BindResourceByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).BindResourceByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount).ok()
    }
    pub unsafe fn BindSampler(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindSampler)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount).ok()
    }
    pub unsafe fn BindSamplerByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).BindSamplerByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount).ok()
    }
    pub unsafe fn BindUnorderedAccessView(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindUnorderedAccessView)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount).ok()
    }
    pub unsafe fn BindUnorderedAccessViewByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).BindUnorderedAccessViewByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount).ok()
    }
    pub unsafe fn BindResourceAsUnorderedAccessView(&self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindResourceAsUnorderedAccessView)(windows_core::Interface::as_raw(self), usrcsrvslot, udstuavslot, ucount).ok()
    }
    pub unsafe fn BindResourceAsUnorderedAccessViewByName<P0>(&self, psrvname: P0, udstuavslot: u32, ucount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).BindResourceAsUnorderedAccessViewByName)(windows_core::Interface::as_raw(self), psrvname.param().abi(), udstuavslot, ucount).ok()
    }
}
unsafe impl Send for ID3D11ModuleInstance {}
unsafe impl Sync for ID3D11ModuleInstance {}
#[repr(C)]
pub struct ID3D11ModuleInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindResourceByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindSampler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindSamplerByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindUnorderedAccessViewByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindResourceAsUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindResourceAsUnorderedAccessViewByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11Multithread, ID3D11Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D11Multithread {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Multithread, windows_core::IUnknown);
impl ID3D11Multithread {
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
unsafe impl Send for ID3D11Multithread {}
unsafe impl Sync for ID3D11Multithread {}
#[repr(C)]
pub struct ID3D11Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D11PixelShader, ID3D11PixelShader_Vtbl, 0xea82e40d_51dc_4f33_93d4_db7c9125ae8c);
impl core::ops::Deref for ID3D11PixelShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11PixelShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11PixelShader {}
unsafe impl Send for ID3D11PixelShader {}
unsafe impl Sync for ID3D11PixelShader {}
#[repr(C)]
pub struct ID3D11PixelShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11Predicate, ID3D11Predicate_Vtbl, 0x9eb576dd_9f77_4d86_81aa_8bab5fe490e2);
impl core::ops::Deref for ID3D11Predicate {
    type Target = ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Predicate, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous, ID3D11Query);
impl ID3D11Predicate {}
unsafe impl Send for ID3D11Predicate {}
unsafe impl Sync for ID3D11Predicate {}
#[repr(C)]
pub struct ID3D11Predicate_Vtbl {
    pub base__: ID3D11Query_Vtbl,
}
windows_core::imp::define_interface!(ID3D11Query, ID3D11Query_Vtbl, 0xd6c00747_87b7_425e_b84d_44d108560afd);
impl core::ops::Deref for ID3D11Query {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Query, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous);
impl ID3D11Query {
    pub unsafe fn GetDesc(&self) -> D3D11_QUERY_DESC {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11Query {}
unsafe impl Sync for ID3D11Query {}
#[repr(C)]
pub struct ID3D11Query_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_QUERY_DESC),
}
windows_core::imp::define_interface!(ID3D11Query1, ID3D11Query1_Vtbl, 0x631b4766_36dc_461d_8db6_c47e13e60916);
impl core::ops::Deref for ID3D11Query1 {
    type Target = ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Query1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Asynchronous, ID3D11Query);
impl ID3D11Query1 {
    pub unsafe fn GetDesc1(&self) -> D3D11_QUERY_DESC1 {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11Query1 {}
unsafe impl Sync for ID3D11Query1 {}
#[repr(C)]
pub struct ID3D11Query1_Vtbl {
    pub base__: ID3D11Query_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_QUERY_DESC1),
}
windows_core::imp::define_interface!(ID3D11RasterizerState, ID3D11RasterizerState_Vtbl, 0x9bb4ab81_ab1a_4d8f_b506_fc04200b6ee7);
impl core::ops::Deref for ID3D11RasterizerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11RasterizerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11RasterizerState {}
unsafe impl Sync for ID3D11RasterizerState {}
#[repr(C)]
pub struct ID3D11RasterizerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC),
}
windows_core::imp::define_interface!(ID3D11RasterizerState1, ID3D11RasterizerState1_Vtbl, 0x1217d7a6_5039_418c_b042_9cbe256afd6e);
impl core::ops::Deref for ID3D11RasterizerState1 {
    type Target = ID3D11RasterizerState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11RasterizerState);
impl ID3D11RasterizerState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11RasterizerState1 {}
unsafe impl Sync for ID3D11RasterizerState1 {}
#[repr(C)]
pub struct ID3D11RasterizerState1_Vtbl {
    pub base__: ID3D11RasterizerState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC1),
}
windows_core::imp::define_interface!(ID3D11RasterizerState2, ID3D11RasterizerState2_Vtbl, 0x6fbd02fb_209f_46c4_b059_2ed15586a6ac);
impl core::ops::Deref for ID3D11RasterizerState2 {
    type Target = ID3D11RasterizerState1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState2, windows_core::IUnknown, ID3D11DeviceChild, ID3D11RasterizerState, ID3D11RasterizerState1);
impl ID3D11RasterizerState2 {
    pub unsafe fn GetDesc2(&self, pdesc: *mut D3D11_RASTERIZER_DESC2) {
        (windows_core::Interface::vtable(self).GetDesc2)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11RasterizerState2 {}
unsafe impl Sync for ID3D11RasterizerState2 {}
#[repr(C)]
pub struct ID3D11RasterizerState2_Vtbl {
    pub base__: ID3D11RasterizerState1_Vtbl,
    pub GetDesc2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC2),
}
windows_core::imp::define_interface!(ID3D11RefDefaultTrackingOptions, ID3D11RefDefaultTrackingOptions_Vtbl, 0x03916615_c644_418c_9bf4_75db5be63ca0);
impl core::ops::Deref for ID3D11RefDefaultTrackingOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RefDefaultTrackingOptions, windows_core::IUnknown);
impl ID3D11RefDefaultTrackingOptions {
    pub unsafe fn SetTrackingOptions(&self, resourcetypeflags: u32, options: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrackingOptions)(windows_core::Interface::as_raw(self), resourcetypeflags, options).ok()
    }
}
unsafe impl Send for ID3D11RefDefaultTrackingOptions {}
unsafe impl Sync for ID3D11RefDefaultTrackingOptions {}
#[repr(C)]
pub struct ID3D11RefDefaultTrackingOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTrackingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11RefTrackingOptions, ID3D11RefTrackingOptions_Vtbl, 0x193dacdf_0db2_4c05_a55c_ef06cac56fd9);
impl core::ops::Deref for ID3D11RefTrackingOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RefTrackingOptions, windows_core::IUnknown);
impl ID3D11RefTrackingOptions {
    pub unsafe fn SetTrackingOptions(&self, uoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrackingOptions)(windows_core::Interface::as_raw(self), uoptions).ok()
    }
}
unsafe impl Send for ID3D11RefTrackingOptions {}
unsafe impl Sync for ID3D11RefTrackingOptions {}
#[repr(C)]
pub struct ID3D11RefTrackingOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTrackingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11RenderTargetView, ID3D11RenderTargetView_Vtbl, 0xdfdba067_0b8d_4865_875b_d7b4516cc164);
impl core::ops::Deref for ID3D11RenderTargetView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RenderTargetView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11RenderTargetView {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11RenderTargetView {}
unsafe impl Sync for ID3D11RenderTargetView {}
#[repr(C)]
pub struct ID3D11RenderTargetView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11RenderTargetView1, ID3D11RenderTargetView1_Vtbl, 0xffbe2e23_f011_418a_ac56_5ceed7c5b94b);
impl core::ops::Deref for ID3D11RenderTargetView1 {
    type Target = ID3D11RenderTargetView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11RenderTargetView1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View, ID3D11RenderTargetView);
impl ID3D11RenderTargetView1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1)
    }
}
unsafe impl Send for ID3D11RenderTargetView1 {}
unsafe impl Sync for ID3D11RenderTargetView1 {}
#[repr(C)]
pub struct ID3D11RenderTargetView1_Vtbl {
    pub base__: ID3D11RenderTargetView_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RENDER_TARGET_VIEW_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D11Resource, ID3D11Resource_Vtbl, 0xdc8e63f3_d12b_4952_b47b_5e45026a862d);
impl core::ops::Deref for ID3D11Resource {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Resource, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Resource {
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
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
unsafe impl Send for ID3D11Resource {}
unsafe impl Sync for ID3D11Resource {}
#[repr(C)]
pub struct ID3D11Resource_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID3D11SamplerState, ID3D11SamplerState_Vtbl, 0xda6fea51_564c_4487_9810_f0d0f9b4e3a5);
impl core::ops::Deref for ID3D11SamplerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11SamplerState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11SamplerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SAMPLER_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11SamplerState {}
unsafe impl Sync for ID3D11SamplerState {}
#[repr(C)]
pub struct ID3D11SamplerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SAMPLER_DESC),
}
windows_core::imp::define_interface!(ID3D11ShaderReflection, ID3D11ShaderReflection_Vtbl, 0x8d536ca1_0cca_4956_a837_786963755584);
impl core::ops::Deref for ID3D11ShaderReflection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderReflection, windows_core::IUnknown);
impl ID3D11ShaderReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatchConstantParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc).ok()
    }
    pub unsafe fn GetMovInstructionCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetMovInstructionCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetMovcInstructionCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetConversionInstructionCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetBitwiseInstructionCount)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetGSInputPrimitive(&self) -> super::Direct3D::D3D_PRIMITIVE {
        (windows_core::Interface::vtable(self).GetGSInputPrimitive)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsSampleFrequencyShader(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsSampleFrequencyShader)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumInterfaceSlots(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetNumInterfaceSlots)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMinFeatureLevel(&self) -> windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinFeatureLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetThreadGroupSize(&self, psizex: Option<*mut u32>, psizey: Option<*mut u32>, psizez: Option<*mut u32>) -> u32 {
        (windows_core::Interface::vtable(self).GetThreadGroupSize)(windows_core::Interface::as_raw(self), core::mem::transmute(psizex.unwrap_or(std::ptr::null_mut())), core::mem::transmute(psizey.unwrap_or(std::ptr::null_mut())), core::mem::transmute(psizez.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GetRequiresFlags(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetRequiresFlags)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D11ShaderReflection {}
unsafe impl Sync for ID3D11ShaderReflection {}
#[repr(C)]
pub struct ID3D11ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPatchConstantParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPatchConstantParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetMovcInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetConversionInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetGSInputPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetGSInputPrimitive: usize,
    pub IsSampleFrequencyShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetNumInterfaceSlots: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetMinFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetMinFeatureLevel: usize,
    pub GetThreadGroupSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> u32,
    pub GetRequiresFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
windows_core::imp::define_interface!(ID3D11ShaderReflectionConstantBuffer, ID3D11ShaderReflectionConstantBuffer_Vtbl);
impl ID3D11ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionVariable> {
        (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
}
unsafe impl Send for ID3D11ShaderReflectionConstantBuffer {}
unsafe impl Sync for ID3D11ShaderReflectionConstantBuffer {}
#[repr(C)]
pub struct ID3D11ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
}
windows_core::imp::define_interface!(ID3D11ShaderReflectionType, ID3D11ShaderReflectionType_Vtbl);
impl ID3D11ShaderReflectionType {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionType>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR {
        (windows_core::Interface::vtable(self).GetMemberTypeName)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn IsEqual<P0>(&self, ptype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), ptype.param().abi()).ok()
    }
    pub unsafe fn GetSubType(&self) -> Option<ID3D11ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBaseClass(&self) -> Option<ID3D11ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetBaseClass)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumInterfaces(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetNumInterfaces)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D11ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetInterfaceByIndex)(windows_core::Interface::as_raw(self), uindex)
    }
    pub unsafe fn IsOfType<P0>(&self, ptype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).IsOfType)(windows_core::Interface::as_raw(self), ptype.param().abi()).ok()
    }
    pub unsafe fn ImplementsInterface<P0>(&self, pbase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).ImplementsInterface)(windows_core::Interface::as_raw(self), pbase.param().abi()).ok()
    }
}
unsafe impl Send for ID3D11ShaderReflectionType {}
unsafe impl Sync for ID3D11ShaderReflectionType {}
#[repr(C)]
pub struct ID3D11ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetNumInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetInterfaceByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionType>,
    pub IsOfType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImplementsInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11ShaderReflectionVariable, ID3D11ShaderReflectionVariable_Vtbl);
impl ID3D11ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetType(&self) -> Option<ID3D11ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBuffer(&self) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetInterfaceSlot)(windows_core::Interface::as_raw(self), uarrayindex)
    }
}
unsafe impl Send for ID3D11ShaderReflectionVariable {}
unsafe impl Sync for ID3D11ShaderReflectionVariable {}
#[repr(C)]
pub struct ID3D11ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetInterfaceSlot: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
windows_core::imp::define_interface!(ID3D11ShaderResourceView, ID3D11ShaderResourceView_Vtbl, 0xb0e06fe0_8192_4e1a_b1ca_36d7414710b2);
impl core::ops::Deref for ID3D11ShaderResourceView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderResourceView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11ShaderResourceView {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11ShaderResourceView {}
unsafe impl Sync for ID3D11ShaderResourceView {}
#[repr(C)]
pub struct ID3D11ShaderResourceView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11ShaderResourceView1, ID3D11ShaderResourceView1_Vtbl, 0x91308b87_9040_411d_8c67_c39253ce3802);
impl core::ops::Deref for ID3D11ShaderResourceView1 {
    type Target = ID3D11ShaderResourceView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderResourceView1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View, ID3D11ShaderResourceView);
impl ID3D11ShaderResourceView1 {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1)
    }
}
unsafe impl Send for ID3D11ShaderResourceView1 {}
unsafe impl Sync for ID3D11ShaderResourceView1 {}
#[repr(C)]
pub struct ID3D11ShaderResourceView1_Vtbl {
    pub base__: ID3D11ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D11ShaderTrace, ID3D11ShaderTrace_Vtbl, 0x36b013e6_2811_4845_baa7_d623fe0df104);
impl core::ops::Deref for ID3D11ShaderTrace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderTrace, windows_core::IUnknown);
impl ID3D11ShaderTrace {
    pub unsafe fn TraceReady(&self, ptestcount: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TraceReady)(windows_core::Interface::as_raw(self), core::mem::transmute(ptestcount.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ResetTrace(&self) {
        (windows_core::Interface::vtable(self).ResetTrace)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTraceStats(&self, ptracestats: *mut D3D11_TRACE_STATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTraceStats)(windows_core::Interface::as_raw(self), ptracestats).ok()
    }
    pub unsafe fn PSSelectStamp(&self, stampindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PSSelectStamp)(windows_core::Interface::as_raw(self), stampindex).ok()
    }
    pub unsafe fn GetInitialRegisterContents(&self, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInitialRegisterContents)(windows_core::Interface::as_raw(self), pregister, pvalue).ok()
    }
    pub unsafe fn GetStep(&self, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStep)(windows_core::Interface::as_raw(self), stepindex, ptracestep).ok()
    }
    pub unsafe fn GetWrittenRegister(&self, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWrittenRegister)(windows_core::Interface::as_raw(self), stepindex, writtenregisterindex, pregister, pvalue).ok()
    }
    pub unsafe fn GetReadRegister(&self, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReadRegister)(windows_core::Interface::as_raw(self), stepindex, readregisterindex, pregister, pvalue).ok()
    }
}
unsafe impl Send for ID3D11ShaderTrace {}
unsafe impl Sync for ID3D11ShaderTrace {}
#[repr(C)]
pub struct ID3D11ShaderTrace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TraceReady: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ResetTrace: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetTraceStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TRACE_STATS) -> windows_core::HRESULT,
    pub PSSelectStamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInitialRegisterContents: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TRACE_REGISTER, *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT,
    pub GetStep: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_TRACE_STEP) -> windows_core::HRESULT,
    pub GetWrittenRegister: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D11_TRACE_REGISTER, *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT,
    pub GetReadRegister: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D11_TRACE_REGISTER, *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11ShaderTraceFactory, ID3D11ShaderTraceFactory_Vtbl, 0x1fbad429_66ab_41cc_9617_667ac10e4459);
impl core::ops::Deref for ID3D11ShaderTraceFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11ShaderTraceFactory, windows_core::IUnknown);
impl ID3D11ShaderTraceFactory {
    pub unsafe fn CreateShaderTrace<P0>(&self, pshader: P0, ptracedesc: *const D3D11_SHADER_TRACE_DESC) -> windows_core::Result<ID3D11ShaderTrace>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateShaderTrace)(windows_core::Interface::as_raw(self), pshader.param().abi(), ptracedesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D11ShaderTraceFactory {}
unsafe impl Sync for ID3D11ShaderTraceFactory {}
#[repr(C)]
pub struct ID3D11ShaderTraceFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateShaderTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_SHADER_TRACE_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11SwitchToRef, ID3D11SwitchToRef_Vtbl, 0x1ef337e3_58e7_4f83_a692_db221f5ed47e);
impl core::ops::Deref for ID3D11SwitchToRef {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11SwitchToRef, windows_core::IUnknown);
impl ID3D11SwitchToRef {
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
unsafe impl Send for ID3D11SwitchToRef {}
unsafe impl Sync for ID3D11SwitchToRef {}
#[repr(C)]
pub struct ID3D11SwitchToRef_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    pub GetUseRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D11Texture1D, ID3D11Texture1D_Vtbl, 0xf8fb5c27_c6b3_4f75_a4c8_439af2ef564c);
impl core::ops::Deref for ID3D11Texture1D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture1D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture1D {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE1D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Texture1D {}
unsafe impl Sync for ID3D11Texture1D {}
#[repr(C)]
pub struct ID3D11Texture1D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11Texture2D, ID3D11Texture2D_Vtbl, 0x6f15aaf2_d208_4e89_9ab4_489535d34f9c);
impl core::ops::Deref for ID3D11Texture2D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture2D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture2D {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Texture2D {}
unsafe impl Sync for ID3D11Texture2D {}
#[repr(C)]
pub struct ID3D11Texture2D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11Texture2D1, ID3D11Texture2D1_Vtbl, 0x51218251_1e33_4617_9ccb_4d3a4367e7bb);
impl core::ops::Deref for ID3D11Texture2D1 {
    type Target = ID3D11Texture2D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture2D1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource, ID3D11Texture2D);
impl ID3D11Texture2D1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Texture2D1 {}
unsafe impl Sync for ID3D11Texture2D1 {}
#[repr(C)]
pub struct ID3D11Texture2D1_Vtbl {
    pub base__: ID3D11Texture2D_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D11Texture3D, ID3D11Texture3D_Vtbl, 0x037e866e_f56d_4357_a8af_9dabbe6e250e);
impl core::ops::Deref for ID3D11Texture3D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture3D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
impl ID3D11Texture3D {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Texture3D {}
unsafe impl Sync for ID3D11Texture3D {}
#[repr(C)]
pub struct ID3D11Texture3D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11Texture3D1, ID3D11Texture3D1_Vtbl, 0x0c711683_2853_4846_9bb0_f3e60639e46a);
impl core::ops::Deref for ID3D11Texture3D1 {
    type Target = ID3D11Texture3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Texture3D1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource, ID3D11Texture3D);
impl ID3D11Texture3D1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11Texture3D1 {}
unsafe impl Sync for ID3D11Texture3D1 {}
#[repr(C)]
pub struct ID3D11Texture3D1_Vtbl {
    pub base__: ID3D11Texture3D_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE3D_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D11TracingDevice, ID3D11TracingDevice_Vtbl, 0x1911c771_1587_413e_a7e0_fb26c3de0268);
impl core::ops::Deref for ID3D11TracingDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11TracingDevice, windows_core::IUnknown);
impl ID3D11TracingDevice {
    pub unsafe fn SetShaderTrackingOptionsByType(&self, resourcetypeflags: u32, options: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetShaderTrackingOptionsByType)(windows_core::Interface::as_raw(self), resourcetypeflags, options).ok()
    }
    pub unsafe fn SetShaderTrackingOptions<P0>(&self, pshader: P0, options: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetShaderTrackingOptions)(windows_core::Interface::as_raw(self), pshader.param().abi(), options).ok()
    }
}
unsafe impl Send for ID3D11TracingDevice {}
unsafe impl Sync for ID3D11TracingDevice {}
#[repr(C)]
pub struct ID3D11TracingDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetShaderTrackingOptionsByType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetShaderTrackingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11UnorderedAccessView, ID3D11UnorderedAccessView_Vtbl, 0x28acf509_7f5c_48f6_8611_f316010a6380);
impl core::ops::Deref for ID3D11UnorderedAccessView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11UnorderedAccessView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11UnorderedAccessView {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11UnorderedAccessView {}
unsafe impl Sync for ID3D11UnorderedAccessView {}
#[repr(C)]
pub struct ID3D11UnorderedAccessView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_UNORDERED_ACCESS_VIEW_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D11UnorderedAccessView1, ID3D11UnorderedAccessView1_Vtbl, 0x7b3b6153_a886_4544_ab37_6537c8500403);
impl core::ops::Deref for ID3D11UnorderedAccessView1 {
    type Target = ID3D11UnorderedAccessView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11UnorderedAccessView1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View, ID3D11UnorderedAccessView);
impl ID3D11UnorderedAccessView1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1)
    }
}
unsafe impl Send for ID3D11UnorderedAccessView1 {}
unsafe impl Sync for ID3D11UnorderedAccessView1 {}
#[repr(C)]
pub struct ID3D11UnorderedAccessView1_Vtbl {
    pub base__: ID3D11UnorderedAccessView_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D11VertexShader, ID3D11VertexShader_Vtbl, 0x3b301d64_d678_4289_8897_22f8928b72f3);
impl core::ops::Deref for ID3D11VertexShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VertexShader, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VertexShader {}
unsafe impl Send for ID3D11VertexShader {}
unsafe impl Sync for ID3D11VertexShader {}
#[repr(C)]
pub struct ID3D11VertexShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D11VideoContext, ID3D11VideoContext_Vtbl, 0x61f21c45_3c0e_4a74_9cea_67100d9ad5e4);
impl core::ops::Deref for ID3D11VideoContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoContext, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoContext {
    pub unsafe fn GetDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).GetDecoderBuffer)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), r#type, pbuffersize, ppbuffer).ok()
    }
    pub unsafe fn ReleaseDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).ReleaseDecoderBuffer)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), r#type).ok()
    }
    pub unsafe fn DecoderBeginFrame<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
        P1: windows_core::Param<ID3D11VideoDecoderOutputView>,
    {
        (windows_core::Interface::vtable(self).DecoderBeginFrame)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pview.param().abi(), contentkeysize, core::mem::transmute(pcontentkey.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).DecoderEndFrame)(windows_core::Interface::as_raw(self), pdecoder.param().abi()).ok()
    }
    pub unsafe fn SubmitDecoderBuffers<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).SubmitDecoderBuffers)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn DecoderExtension<P0>(&self, pdecoder: P0, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).DecoderExtension)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pextensiondata)
    }
    pub unsafe fn VideoProcessorSetOutputTargetRect<P0, P1>(&self, pvideoprocessor: P0, enable: P1, prect: Option<*const super::super::Foundation::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputTargetRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.param().abi(), core::mem::transmute(prect.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VideoProcessorSetOutputBackgroundColor<P0, P1>(&self, pvideoprocessor: P0, ycbcr: P1, pcolor: *const D3D11_VIDEO_COLOR)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputBackgroundColor)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), ycbcr.param().abi(), pcolor)
    }
    pub unsafe fn VideoProcessorSetOutputColorSpace<P0>(&self, pvideoprocessor: P0, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pcolorspace)
    }
    pub unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputAlphaFillMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), alphafillmode, streamindex)
    }
    pub unsafe fn VideoProcessorSetOutputConstriction<P0, P1>(&self, pvideoprocessor: P0, enable: P1, size: super::super::Foundation::SIZE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputConstriction)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.param().abi(), core::mem::transmute(size))
    }
    pub unsafe fn VideoProcessorSetOutputStereoMode<P0, P1>(&self, pvideoprocessor: P0, enable: P1)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputStereoMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enable.param().abi())
    }
    pub unsafe fn VideoProcessorSetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorGetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputTargetRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), enabled, prect)
    }
    pub unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputBackgroundColor)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pycbcr, pcolor)
    }
    pub unsafe fn VideoProcessorGetOutputColorSpace<P0>(&self, pvideoprocessor: P0) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputAlphaFillMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), palphafillmode, pstreamindex)
    }
    pub unsafe fn VideoProcessorGetOutputConstriction<P0>(&self, pvideoprocessor: P0, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputConstriction)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), penabled, psize)
    }
    pub unsafe fn VideoProcessorGetOutputStereoMode<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputStereoMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorSetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamFrameFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, frameformat)
    }
    pub unsafe fn VideoProcessorSetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pcolorspace)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetStreamOutputRate<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: P1, pcustomrate: Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamOutputRate)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, outputrate, repeatframe.param().abi(), core::mem::transmute(pcustomrate.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VideoProcessorSetStreamSourceRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: Option<*const super::super::Foundation::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamSourceRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), core::mem::transmute(prect.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VideoProcessorSetStreamDestRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: Option<*const super::super::Foundation::RECT>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamDestRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), core::mem::transmute(prect.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VideoProcessorSetStreamAlpha<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, alpha: f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamAlpha)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), alpha)
    }
    pub unsafe fn VideoProcessorSetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: Option<&[u32]>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamPalette)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, psourceaspectratio: Option<*const super::Dxgi::Common::DXGI_RATIONAL>, pdestinationaspectratio: Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamPixelAspectRatio)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), core::mem::transmute(psourceaspectratio.unwrap_or(std::ptr::null())), core::mem::transmute(pdestinationaspectratio.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn VideoProcessorSetStreamLumaKey<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, lower: f32, upper: f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamLumaKey)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), lower, upper)
    }
    pub unsafe fn VideoProcessorSetStreamStereoFormat<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: P2, baseviewframe0: P3, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamStereoFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), format, leftviewframe0.param().abi(), baseviewframe0.param().abi(), flipmode, monooffset)
    }
    pub unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamAutoProcessingMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi())
    }
    pub unsafe fn VideoProcessorSetStreamFilter<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: P1, level: i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamFilter)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, filter, enable.param().abi(), level)
    }
    pub unsafe fn VideoProcessorSetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorGetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamFrameFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamColorSpace)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamOutputRate)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, poutputrate, prepeatframe, pcustomrate)
    }
    pub unsafe fn VideoProcessorGetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamSourceRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled, prect)
    }
    pub unsafe fn VideoProcessorGetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamDestRect)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled, prect)
    }
    pub unsafe fn VideoProcessorGetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamAlpha)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled, palpha)
    }
    pub unsafe fn VideoProcessorGetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: &mut [u32])
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamPalette)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pentries.len().try_into().unwrap(), core::mem::transmute(pentries.as_ptr()))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamPixelAspectRatio)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled, psourceaspectratio, pdestinationaspectratio)
    }
    pub unsafe fn VideoProcessorGetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamLumaKey)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penabled, plower, pupper)
    }
    pub unsafe fn VideoProcessorGetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamStereoFormat)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable, pformat, pleftviewframe0, pbaseviewframe0, pflipmode, monooffset)
    }
    pub unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamAutoProcessingMode)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamFilter)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, filter, penabled, plevel)
    }
    pub unsafe fn VideoProcessorGetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamExtension)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorBlt<P0, P1>(&self, pvideoprocessor: P0, pview: P1, outputframe: u32, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<ID3D11VideoProcessorOutputView>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorBlt)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), pview.param().abi(), outputframe, pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr())).ok()
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchange<P0>(&self, pcryptosession: P0, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        (windows_core::Interface::vtable(self).NegotiateCryptoSessionKeyExchange)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), datasize, pdata).ok()
    }
    pub unsafe fn EncryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, ivsize: u32, piv: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        (windows_core::Interface::vtable(self).EncryptionBlt)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), psrcsurface.param().abi(), pdstsurface.param().abi(), ivsize, core::mem::transmute(piv.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn DecryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, pencryptedblockinfo: Option<*const D3D11_ENCRYPTED_BLOCK_INFO>, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>, ivsize: u32, piv: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        (windows_core::Interface::vtable(self).DecryptionBlt)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), psrcsurface.param().abi(), pdstsurface.param().abi(), core::mem::transmute(pencryptedblockinfo.unwrap_or(std::ptr::null())), contentkeysize, core::mem::transmute(pcontentkey.unwrap_or(std::ptr::null())), ivsize, core::mem::transmute(piv.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn StartSessionKeyRefresh<P0>(&self, pcryptosession: P0, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        (windows_core::Interface::vtable(self).StartSessionKeyRefresh)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), randomnumbersize, prandomnumber)
    }
    pub unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        (windows_core::Interface::vtable(self).FinishSessionKeyRefresh)(windows_core::Interface::as_raw(self), pcryptosession.param().abi())
    }
    pub unsafe fn GetEncryptionBltKey<P0>(&self, pcryptosession: P0, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        (windows_core::Interface::vtable(self).GetEncryptionBltKey)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), keysize, preadbackkey).ok()
    }
    pub unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(&self, pchannel: P0, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        (windows_core::Interface::vtable(self).NegotiateAuthenticatedChannelKeyExchange)(windows_core::Interface::as_raw(self), pchannel.param().abi(), datasize, pdata).ok()
    }
    pub unsafe fn QueryAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        (windows_core::Interface::vtable(self).QueryAuthenticatedChannel)(windows_core::Interface::as_raw(self), pchannel.param().abi(), inputsize, pinput, outputsize, poutput).ok()
    }
    pub unsafe fn ConfigureAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        (windows_core::Interface::vtable(self).ConfigureAuthenticatedChannel)(windows_core::Interface::as_raw(self), pchannel.param().abi(), inputsize, pinput, poutput).ok()
    }
    pub unsafe fn VideoProcessorSetStreamRotation<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, rotation: D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamRotation)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), rotation)
    }
    pub unsafe fn VideoProcessorGetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamRotation)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable, protation)
    }
}
unsafe impl Send for ID3D11VideoContext {}
unsafe impl Sync for ID3D11VideoContext {}
#[repr(C)]
pub struct ID3D11VideoContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDecoderBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_DECODER_BUFFER_TYPE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseDecoderBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::HRESULT,
    pub DecoderBeginFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub DecoderEndFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubmitDecoderBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::HRESULT,
    pub DecoderExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_EXTENSION) -> i32,
    pub VideoProcessorSetOutputTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *const super::super::Foundation::RECT),
    pub VideoProcessorSetOutputBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *const D3D11_VIDEO_COLOR),
    pub VideoProcessorSetOutputColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    pub VideoProcessorSetOutputAlphaFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, u32),
    pub VideoProcessorSetOutputConstriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::SIZE),
    pub VideoProcessorSetOutputStereoMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub VideoProcessorSetOutputExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> i32,
    pub VideoProcessorGetOutputTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::RECT),
    pub VideoProcessorGetOutputBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut D3D11_VIDEO_COLOR),
    pub VideoProcessorGetOutputColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    pub VideoProcessorGetOutputAlphaFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, *mut u32),
    pub VideoProcessorGetOutputConstriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::SIZE),
    pub VideoProcessorGetOutputStereoMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL),
    pub VideoProcessorGetOutputExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> i32,
    pub VideoProcessorSetStreamFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_FRAME_FORMAT),
    pub VideoProcessorSetStreamColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorSetStreamOutputRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, super::super::Foundation::BOOL, *const super::Dxgi::Common::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorSetStreamOutputRate: usize,
    pub VideoProcessorSetStreamSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *const super::super::Foundation::RECT),
    pub VideoProcessorSetStreamDestRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *const super::super::Foundation::RECT),
    pub VideoProcessorSetStreamAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, f32),
    pub VideoProcessorSetStreamPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorSetStreamPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *const super::Dxgi::Common::DXGI_RATIONAL, *const super::Dxgi::Common::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorSetStreamPixelAspectRatio: usize,
    pub VideoProcessorSetStreamLumaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, f32, f32),
    pub VideoProcessorSetStreamStereoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, super::super::Foundation::BOOL, super::super::Foundation::BOOL, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, i32),
    pub VideoProcessorSetStreamAutoProcessingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL),
    pub VideoProcessorSetStreamFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_FILTER, super::super::Foundation::BOOL, i32),
    pub VideoProcessorSetStreamExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *const core::ffi::c_void) -> i32,
    pub VideoProcessorGetStreamFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_FRAME_FORMAT),
    pub VideoProcessorGetStreamColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorGetStreamOutputRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, *mut super::super::Foundation::BOOL, *mut super::Dxgi::Common::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorGetStreamOutputRate: usize,
    pub VideoProcessorGetStreamSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::RECT),
    pub VideoProcessorGetStreamDestRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::RECT),
    pub VideoProcessorGetStreamAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut f32),
    pub VideoProcessorGetStreamPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorGetStreamPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut super::Dxgi::Common::DXGI_RATIONAL, *mut super::Dxgi::Common::DXGI_RATIONAL),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorGetStreamPixelAspectRatio: usize,
    pub VideoProcessorGetStreamLumaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut f32, *mut f32),
    pub VideoProcessorGetStreamStereoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, *mut i32),
    pub VideoProcessorGetStreamAutoProcessingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL),
    pub VideoProcessorGetStreamFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_PROCESSOR_FILTER, *mut super::super::Foundation::BOOL, *mut i32),
    pub VideoProcessorGetStreamExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> i32,
    pub VideoProcessorBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::HRESULT,
    pub NegotiateCryptoSessionKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub DecryptionBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_ENCRYPTED_BLOCK_INFO, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub StartSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void),
    pub FinishSessionKeyRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetEncryptionBltKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NegotiateAuthenticatedChannelKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::HRESULT,
    pub VideoProcessorSetStreamRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, D3D11_VIDEO_PROCESSOR_ROTATION),
    pub VideoProcessorGetStreamRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut D3D11_VIDEO_PROCESSOR_ROTATION),
}
windows_core::imp::define_interface!(ID3D11VideoContext1, ID3D11VideoContext1_Vtbl, 0xa7f026da_a5f8_4487_a564_15e34357651e);
impl core::ops::Deref for ID3D11VideoContext1 {
    type Target = ID3D11VideoContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoContext1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11VideoContext);
impl ID3D11VideoContext1 {
    pub unsafe fn SubmitDecoderBuffers1<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC1]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).SubmitDecoderBuffers1)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn GetDataForNewHardwareKey<P0>(&self, pcryptosession: P0, pprivatinputdata: &[u8]) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataForNewHardwareKey)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), pprivatinputdata.len().try_into().unwrap(), core::mem::transmute(pprivatinputdata.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn CheckCryptoSessionStatus<P0>(&self, pcryptosession: P0) -> windows_core::Result<D3D11_CRYPTO_SESSION_STATUS>
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCryptoSessionStatus)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderEnableDownsampling<P0>(&self, pdecoder: P0, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).DecoderEnableDownsampling)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), inputcolorspace, poutputdesc, referenceframecount).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderUpdateDownsampling<P0>(&self, pdecoder: P0, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).DecoderUpdateDownsampling)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), poutputdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetOutputColorSpace1<P0>(&self, pvideoprocessor: P0, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), colorspace)
    }
    pub unsafe fn VideoProcessorSetOutputShaderUsage<P0, P1>(&self, pvideoprocessor: P0, shaderusage: P1)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputShaderUsage)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), shaderusage.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetOutputColorSpace1<P0>(&self, pvideoprocessor: P0) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetOutputShaderUsage<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputShaderUsage)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, colorspace)
    }
    pub unsafe fn VideoProcessorSetStreamMirror<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, fliphorizontal: P2, flipvertical: P3)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamMirror)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.param().abi(), fliphorizontal.param().abi(), flipvertical.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
        result__
    }
    pub unsafe fn VideoProcessorGetStreamMirror<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamMirror)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable, pfliphorizontal, pflipvertical)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetBehaviorHints<P0>(&self, pvideoprocessor: P0, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT]) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VideoProcessorGetBehaviorHints)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), outputwidth, outputheight, outputformat, pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr()), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11VideoContext1 {}
unsafe impl Sync for ID3D11VideoContext1 {}
#[repr(C)]
pub struct ID3D11VideoContext1_Vtbl {
    pub base__: ID3D11VideoContext_Vtbl,
    pub SubmitDecoderBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::HRESULT,
    pub GetDataForNewHardwareKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub CheckCryptoSessionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_CRYPTO_SESSION_STATUS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DecoderEnableDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *const D3D11_VIDEO_SAMPLE_DESC, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DecoderEnableDownsampling: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DecoderUpdateDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DecoderUpdateDownsampling: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorSetOutputColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorSetOutputColorSpace1: usize,
    pub VideoProcessorSetOutputShaderUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorGetOutputColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorGetOutputColorSpace1: usize,
    pub VideoProcessorGetOutputShaderUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorSetStreamColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorSetStreamColorSpace1: usize,
    pub VideoProcessorSetStreamMirror: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorGetStreamColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorGetStreamColorSpace1: usize,
    pub VideoProcessorGetStreamMirror: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub VideoProcessorGetBehaviorHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, super::Dxgi::Common::DXGI_FORMAT, u32, *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    VideoProcessorGetBehaviorHints: usize,
}
windows_core::imp::define_interface!(ID3D11VideoContext2, ID3D11VideoContext2_Vtbl, 0xc4e7374c_6243_4d1b_ae87_52b4f740e261);
impl core::ops::Deref for ID3D11VideoContext2 {
    type Target = ID3D11VideoContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoContext2, windows_core::IUnknown, ID3D11DeviceChild, ID3D11VideoContext, ID3D11VideoContext1);
impl ID3D11VideoContext2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorSetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: Option<*const core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetOutputHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), r#type, size, core::mem::transmute(phdrmetadata.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorGetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetOutputHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), ptype, size, core::mem::transmute(pmetadata.unwrap_or(std::ptr::null_mut())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorSetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: Option<*const core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorSetStreamHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, r#type, size, core::mem::transmute(phdrmetadata.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorGetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        (windows_core::Interface::vtable(self).VideoProcessorGetStreamHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, ptype, size, core::mem::transmute(pmetadata.unwrap_or(std::ptr::null_mut())))
    }
}
unsafe impl Send for ID3D11VideoContext2 {}
unsafe impl Sync for ID3D11VideoContext2 {}
#[repr(C)]
pub struct ID3D11VideoContext2_Vtbl {
    pub base__: ID3D11VideoContext1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub VideoProcessorSetOutputHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Dxgi::DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    VideoProcessorSetOutputHDRMetaData: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub VideoProcessorGetOutputHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, u32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    VideoProcessorGetOutputHDRMetaData: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub VideoProcessorSetStreamHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::Dxgi::DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    VideoProcessorSetStreamHDRMetaData: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub VideoProcessorGetStreamHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, u32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    VideoProcessorGetStreamHDRMetaData: usize,
}
windows_core::imp::define_interface!(ID3D11VideoContext3, ID3D11VideoContext3_Vtbl, 0xa9e2faa0_cb39_418f_a0b7_d8aad4de672e);
impl core::ops::Deref for ID3D11VideoContext3 {
    type Target = ID3D11VideoContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoContext3, windows_core::IUnknown, ID3D11DeviceChild, ID3D11VideoContext, ID3D11VideoContext1, ID3D11VideoContext2);
impl ID3D11VideoContext3 {
    pub unsafe fn DecoderBeginFrame1<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>, numcomponenthistograms: u32, phistogramoffsets: Option<*const u32>, pphistogrambuffers: Option<*const Option<ID3D11Buffer>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
        P1: windows_core::Param<ID3D11VideoDecoderOutputView>,
    {
        (windows_core::Interface::vtable(self).DecoderBeginFrame1)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pview.param().abi(), contentkeysize, core::mem::transmute(pcontentkey.unwrap_or(std::ptr::null())), numcomponenthistograms, core::mem::transmute(phistogramoffsets.unwrap_or(std::ptr::null())), core::mem::transmute(pphistogrambuffers.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SubmitDecoderBuffers2<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC2]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        (windows_core::Interface::vtable(self).SubmitDecoderBuffers2)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
}
unsafe impl Send for ID3D11VideoContext3 {}
unsafe impl Sync for ID3D11VideoContext3 {}
#[repr(C)]
pub struct ID3D11VideoContext3_Vtbl {
    pub base__: ID3D11VideoContext2_Vtbl,
    pub DecoderBeginFrame1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *const u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubmitDecoderBuffers2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11VideoDecoder, ID3D11VideoDecoder_Vtbl, 0x3c9c5b51_995d_48d1_9b8d_fa5caeded65c);
impl core::ops::Deref for ID3D11VideoDecoder {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDecoder, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoDecoder {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCreationParameters(&self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pvideodesc, pconfig).ok()
    }
    pub unsafe fn GetDriverHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDriverHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11VideoDecoder {}
unsafe impl Sync for ID3D11VideoDecoder {}
#[repr(C)]
pub struct ID3D11VideoDecoder_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_DECODER_DESC, *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetCreationParameters: usize,
    pub GetDriverHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11VideoDecoderOutputView, ID3D11VideoDecoderOutputView_Vtbl, 0xc2931aea_2a85_4f20_860f_fba1fd256e18);
impl core::ops::Deref for ID3D11VideoDecoderOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDecoderOutputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoDecoderOutputView {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D11VideoDecoderOutputView {}
unsafe impl Sync for ID3D11VideoDecoderOutputView {}
#[repr(C)]
pub struct ID3D11VideoDecoderOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC),
}
windows_core::imp::define_interface!(ID3D11VideoDevice, ID3D11VideoDevice_Vtbl, 0x10ec4d5b_975a_4689_b9e4_d0aac30fe333);
impl core::ops::Deref for ID3D11VideoDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice, windows_core::IUnknown);
impl ID3D11VideoDevice {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<ID3D11VideoDecoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVideoDecoder)(windows_core::Interface::as_raw(self), pvideodesc, pconfig, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateVideoProcessor<P0>(&self, penum: P0, rateconversionindex: u32) -> windows_core::Result<ID3D11VideoProcessor>
    where
        P0: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVideoProcessor)(windows_core::Interface::as_raw(self), penum.param().abi(), rateconversionindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> windows_core::Result<ID3D11AuthenticatedChannel> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAuthenticatedChannel)(windows_core::Interface::as_raw(self), channeltype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, pkeyexchangetype: *const windows_core::GUID) -> windows_core::Result<ID3D11CryptoSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCryptoSession)(windows_core::Interface::as_raw(self), pcryptotype, core::mem::transmute(pdecoderprofile.unwrap_or(std::ptr::null())), pkeyexchangetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateVideoDecoderOutputView<P0>(&self, presource: P0, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: Option<*mut Option<ID3D11VideoDecoderOutputView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        (windows_core::Interface::vtable(self).CreateVideoDecoderOutputView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc, core::mem::transmute(ppvdovview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorInputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: Option<*mut Option<ID3D11VideoProcessorInputView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        (windows_core::Interface::vtable(self).CreateVideoProcessorInputView)(windows_core::Interface::as_raw(self), presource.param().abi(), penum.param().abi(), pdesc, core::mem::transmute(ppvpiview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorOutputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: Option<*mut Option<ID3D11VideoProcessorOutputView>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        (windows_core::Interface::vtable(self).CreateVideoProcessorOutputView)(windows_core::Interface::as_raw(self), presource.param().abi(), penum.param().abi(), pdesc, core::mem::transmute(ppvpoview.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<ID3D11VideoProcessorEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVideoProcessorEnumerator)(windows_core::Interface::as_raw(self), pdesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVideoDecoderProfileCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetVideoDecoderProfileCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetVideoDecoderProfile(&self, index: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVideoDecoderProfile)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const windows_core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckVideoDecoderFormat)(windows_core::Interface::as_raw(self), pdecoderprofile, format, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVideoDecoderConfigCount)(windows_core::Interface::as_raw(self), pdesc, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVideoDecoderConfig)(windows_core::Interface::as_raw(self), pdesc, index, pconfig).ok()
    }
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: Option<*const windows_core::GUID>, pdecoderprofile: Option<*const windows_core::GUID>, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContentProtectionCaps)(windows_core::Interface::as_raw(self), core::mem::transmute(pcryptotype.unwrap_or(std::ptr::null())), core::mem::transmute(pdecoderprofile.unwrap_or(std::ptr::null())), pcaps).ok()
    }
    pub unsafe fn CheckCryptoKeyExchange(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, index: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckCryptoKeyExchange)(windows_core::Interface::as_raw(self), pcryptotype, core::mem::transmute(pdecoderprofile.unwrap_or(std::ptr::null())), index, &mut result__).map(|| result__)
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
unsafe impl Send for ID3D11VideoDevice {}
unsafe impl Sync for ID3D11VideoDevice {}
#[repr(C)]
pub struct ID3D11VideoDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVideoDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, *const D3D11_VIDEO_DECODER_CONFIG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVideoDecoder: usize,
    pub CreateVideoProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAuthenticatedChannel: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_AUTHENTICATED_CHANNEL_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCryptoSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoDecoderOutputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoProcessorInputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoProcessorOutputView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVideoProcessorEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVideoProcessorEnumerator: usize,
    pub GetVideoDecoderProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetVideoDecoderProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckVideoDecoderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::Dxgi::Common::DXGI_FORMAT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckVideoDecoderFormat: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetVideoDecoderConfigCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetVideoDecoderConfigCount: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetVideoDecoderConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, u32, *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetVideoDecoderConfig: usize,
    pub GetContentProtectionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::HRESULT,
    pub CheckCryptoKeyExchange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11VideoDevice1, ID3D11VideoDevice1_Vtbl, 0x29da1d51_1321_4454_804b_f5fc9f861f0f);
impl core::ops::Deref for ID3D11VideoDevice1 {
    type Target = ID3D11VideoDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice1, windows_core::IUnknown, ID3D11VideoDevice);
impl ID3D11VideoDevice1 {
    pub unsafe fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCryptoSessionPrivateDataSize)(windows_core::Interface::as_raw(self), pcryptotype, core::mem::transmute(pdecoderprofile.unwrap_or(std::ptr::null())), pkeyexchangetype, pprivateinputsize, pprivateoutputsize).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderCaps(&self, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: Option<*const windows_core::GUID>) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVideoDecoderCaps)(windows_core::Interface::as_raw(self), pdecoderprofile, samplewidth, sampleheight, pframerate, bitrate, core::mem::transmute(pcryptotype.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckVideoDecoderDownsampling)(windows_core::Interface::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, poutputdesc, psupported, prealtimehint).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> windows_core::Result<D3D11_VIDEO_SAMPLE_DESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecommendVideoDecoderDownsampleParameters)(windows_core::Interface::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11VideoDevice1 {}
unsafe impl Sync for ID3D11VideoDevice1 {}
#[repr(C)]
pub struct ID3D11VideoDevice1_Vtbl {
    pub base__: ID3D11VideoDevice_Vtbl,
    pub GetCryptoSessionPrivateDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetVideoDecoderCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const super::Dxgi::Common::DXGI_RATIONAL, u32, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetVideoDecoderCaps: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckVideoDecoderDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *const D3D11_VIDEO_DECODER_CONFIG, *const super::Dxgi::Common::DXGI_RATIONAL, *const D3D11_VIDEO_SAMPLE_DESC, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckVideoDecoderDownsampling: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub RecommendVideoDecoderDownsampleParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_VIDEO_DECODER_DESC, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *const D3D11_VIDEO_DECODER_CONFIG, *const super::Dxgi::Common::DXGI_RATIONAL, *mut D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    RecommendVideoDecoderDownsampleParameters: usize,
}
windows_core::imp::define_interface!(ID3D11VideoDevice2, ID3D11VideoDevice2_Vtbl, 0x59c0cb01_35f0_4a70_8f67_87905c906a53);
impl core::ops::Deref for ID3D11VideoDevice2 {
    type Target = ID3D11VideoDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice2, windows_core::IUnknown, ID3D11VideoDevice, ID3D11VideoDevice1);
impl ID3D11VideoDevice2 {
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchangeMT<P0>(&self, pcryptosession: P0, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        (windows_core::Interface::vtable(self).NegotiateCryptoSessionKeyExchangeMT)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), flags, datasize, pdata).ok()
    }
}
unsafe impl Send for ID3D11VideoDevice2 {}
unsafe impl Sync for ID3D11VideoDevice2 {}
#[repr(C)]
pub struct ID3D11VideoDevice2_Vtbl {
    pub base__: ID3D11VideoDevice1_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_FEATURE_VIDEO, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NegotiateCryptoSessionKeyExchangeMT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11VideoProcessor, ID3D11VideoProcessor_Vtbl, 0x1d7b0652_185f_41c6_85ce_0c5be3d4ae6c);
impl core::ops::Deref for ID3D11VideoProcessor {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessor, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoProcessor {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
        (windows_core::Interface::vtable(self).GetContentDesc)(windows_core::Interface::as_raw(self), pdesc)
    }
    pub unsafe fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
        (windows_core::Interface::vtable(self).GetRateConversionCaps)(windows_core::Interface::as_raw(self), pcaps)
    }
}
unsafe impl Send for ID3D11VideoProcessor {}
unsafe impl Sync for ID3D11VideoProcessor {}
#[repr(C)]
pub struct ID3D11VideoProcessor_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetContentDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetContentDesc: usize,
    pub GetRateConversionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS),
}
windows_core::imp::define_interface!(ID3D11VideoProcessorEnumerator, ID3D11VideoProcessorEnumerator_Vtbl, 0x31627037_53ab_4200_9061_05faa9ab45f9);
impl core::ops::Deref for ID3D11VideoProcessorEnumerator {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorEnumerator, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11VideoProcessorEnumerator {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoProcessorContentDesc(&self, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVideoProcessorContentDesc)(windows_core::Interface::as_raw(self), pcontentdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckVideoProcessorFormat(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckVideoProcessorFormat)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
    }
    pub unsafe fn GetVideoProcessorCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVideoProcessorCaps)(windows_core::Interface::as_raw(self), pcaps).ok()
    }
    pub unsafe fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVideoProcessorRateConversionCaps)(windows_core::Interface::as_raw(self), typeindex, pcaps).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVideoProcessorCustomRate)(windows_core::Interface::as_raw(self), typeindex, customrateindex, prate).ok()
    }
    pub unsafe fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVideoProcessorFilterRange)(windows_core::Interface::as_raw(self), filter, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11VideoProcessorEnumerator {}
unsafe impl Sync for ID3D11VideoProcessorEnumerator {}
#[repr(C)]
pub struct ID3D11VideoProcessorEnumerator_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetVideoProcessorContentDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetVideoProcessorContentDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckVideoProcessorFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckVideoProcessorFormat: usize,
    pub GetVideoProcessorCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::HRESULT,
    pub GetVideoProcessorRateConversionCaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetVideoProcessorCustomRate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetVideoProcessorCustomRate: usize,
    pub GetVideoProcessorFilterRange: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_VIDEO_PROCESSOR_FILTER, *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D11VideoProcessorEnumerator1, ID3D11VideoProcessorEnumerator1_Vtbl, 0x465217f2_5568_43cf_b5b9_f61d54531ca1);
impl core::ops::Deref for ID3D11VideoProcessorEnumerator1 {
    type Target = ID3D11VideoProcessorEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorEnumerator1, windows_core::IUnknown, ID3D11DeviceChild, ID3D11VideoProcessorEnumerator);
impl ID3D11VideoProcessorEnumerator1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckVideoProcessorFormatConversion(&self, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckVideoProcessorFormatConversion)(windows_core::Interface::as_raw(self), inputformat, inputcolorspace, outputformat, outputcolorspace, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D11VideoProcessorEnumerator1 {}
unsafe impl Sync for ID3D11VideoProcessorEnumerator1 {}
#[repr(C)]
pub struct ID3D11VideoProcessorEnumerator1_Vtbl {
    pub base__: ID3D11VideoProcessorEnumerator_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckVideoProcessorFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, super::Dxgi::Common::DXGI_FORMAT, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckVideoProcessorFormatConversion: usize,
}
windows_core::imp::define_interface!(ID3D11VideoProcessorInputView, ID3D11VideoProcessorInputView_Vtbl, 0x11ec5a5f_51dc_4945_ab34_6e8c21300ea5);
impl core::ops::Deref for ID3D11VideoProcessorInputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorInputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoProcessorInputView {
    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11VideoProcessorInputView {}
unsafe impl Sync for ID3D11VideoProcessorInputView {}
#[repr(C)]
pub struct ID3D11VideoProcessorInputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC),
}
windows_core::imp::define_interface!(ID3D11VideoProcessorOutputView, ID3D11VideoProcessorOutputView_Vtbl, 0xa048285e_25a9_4527_bd93_d68b68c44254);
impl core::ops::Deref for ID3D11VideoProcessorOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorOutputView, windows_core::IUnknown, ID3D11DeviceChild, ID3D11View);
impl ID3D11VideoProcessorOutputView {
    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D11VideoProcessorOutputView {}
unsafe impl Sync for ID3D11VideoProcessorOutputView {}
#[repr(C)]
pub struct ID3D11VideoProcessorOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC),
}
windows_core::imp::define_interface!(ID3D11View, ID3D11View_Vtbl, 0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1);
impl core::ops::Deref for ID3D11View {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11View, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11View {
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D11Resource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID3D11View {}
unsafe impl Sync for ID3D11View {}
#[repr(C)]
pub struct ID3D11View_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3DDeviceContextState, ID3DDeviceContextState_Vtbl, 0x5c1e0d8a_7c23_48f9_8c59_a92958ceff11);
impl core::ops::Deref for ID3DDeviceContextState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3DDeviceContextState, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3DDeviceContextState {}
unsafe impl Send for ID3DDeviceContextState {}
unsafe impl Sync for ID3DDeviceContextState {}
#[repr(C)]
pub struct ID3DDeviceContextState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3DUserDefinedAnnotation, ID3DUserDefinedAnnotation_Vtbl, 0xb2daad8b_03d4_4dbf_95eb_32ab4b63d0ab);
impl core::ops::Deref for ID3DUserDefinedAnnotation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3DUserDefinedAnnotation, windows_core::IUnknown);
impl ID3DUserDefinedAnnotation {
    pub unsafe fn BeginEvent<P0>(&self, name: P0) -> i32
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn EndEvent(&self) -> i32 {
        (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetMarker<P0>(&self, name: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn GetStatus(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3DUserDefinedAnnotation {}
unsafe impl Sync for ID3DUserDefinedAnnotation {}
#[repr(C)]
pub struct ID3DUserDefinedAnnotation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> i32,
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3DX11FFT, ID3DX11FFT_Vtbl, 0xb3f7a938_4c93_4310_a675_b30d6de50553);
impl core::ops::Deref for ID3DX11FFT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3DX11FFT, windows_core::IUnknown);
impl ID3DX11FFT {
    pub unsafe fn SetForwardScale(&self, forwardscale: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetForwardScale)(windows_core::Interface::as_raw(self), forwardscale).ok()
    }
    pub unsafe fn GetForwardScale(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetForwardScale)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetInverseScale(&self, inversescale: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInverseScale)(windows_core::Interface::as_raw(self), inversescale).ok()
    }
    pub unsafe fn GetInverseScale(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetInverseScale)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AttachBuffersAndPrecompute(&self, pptempbuffers: &[Option<ID3D11UnorderedAccessView>], ppprecomputebuffersizes: &[Option<ID3D11UnorderedAccessView>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachBuffersAndPrecompute)(windows_core::Interface::as_raw(self), pptempbuffers.len().try_into().unwrap(), core::mem::transmute(pptempbuffers.as_ptr()), ppprecomputebuffersizes.len().try_into().unwrap(), core::mem::transmute(ppprecomputebuffersizes.as_ptr())).ok()
    }
    pub unsafe fn ForwardTransform<P0>(&self, pinputbuffer: P0, ppoutputbuffer: *mut Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).ForwardTransform)(windows_core::Interface::as_raw(self), pinputbuffer.param().abi(), core::mem::transmute(ppoutputbuffer)).ok()
    }
    pub unsafe fn InverseTransform<P0>(&self, pinputbuffer: P0, ppoutputbuffer: *mut Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).InverseTransform)(windows_core::Interface::as_raw(self), pinputbuffer.param().abi(), core::mem::transmute(ppoutputbuffer)).ok()
    }
}
unsafe impl Send for ID3DX11FFT {}
unsafe impl Sync for ID3DX11FFT {}
#[repr(C)]
pub struct ID3DX11FFT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetForwardScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetForwardScale: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub SetInverseScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetInverseScale: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub AttachBuffersAndPrecompute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForwardTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InverseTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3DX11Scan, ID3DX11Scan_Vtbl, 0x5089b68f_e71d_4d38_be8e_f363b95a9405);
impl core::ops::Deref for ID3DX11Scan {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3DX11Scan, windows_core::IUnknown);
impl ID3DX11Scan {
    pub unsafe fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScanDirection)(windows_core::Interface::as_raw(self), direction).ok()
    }
    pub unsafe fn Scan<P0, P1>(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: P0, pdst: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
        P1: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), elementtype, opcode, elementscansize, psrc.param().abi(), pdst.param().abi()).ok()
    }
    pub unsafe fn Multiscan<P0, P1>(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: P0, pdst: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
        P1: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).Multiscan)(windows_core::Interface::as_raw(self), elementtype, opcode, elementscansize, elementscanpitch, scancount, psrc.param().abi(), pdst.param().abi()).ok()
    }
}
unsafe impl Send for ID3DX11Scan {}
unsafe impl Sync for ID3DX11Scan {}
#[repr(C)]
pub struct ID3DX11Scan_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScanDirection: unsafe extern "system" fn(*mut core::ffi::c_void, D3DX11_SCAN_DIRECTION) -> windows_core::HRESULT,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, D3DX11_SCAN_DATA_TYPE, D3DX11_SCAN_OPCODE, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Multiscan: unsafe extern "system" fn(*mut core::ffi::c_void, D3DX11_SCAN_DATA_TYPE, D3DX11_SCAN_OPCODE, u32, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3DX11SegmentedScan, ID3DX11SegmentedScan_Vtbl, 0xa915128c_d954_4c79_bfe1_64db923194d6);
impl core::ops::Deref for ID3DX11SegmentedScan {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3DX11SegmentedScan, windows_core::IUnknown);
impl ID3DX11SegmentedScan {
    pub unsafe fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScanDirection)(windows_core::Interface::as_raw(self), direction).ok()
    }
    pub unsafe fn SegScan<P0, P1, P2>(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: P0, psrcelementflags: P1, pdst: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
        P1: windows_core::Param<ID3D11UnorderedAccessView>,
        P2: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        (windows_core::Interface::vtable(self).SegScan)(windows_core::Interface::as_raw(self), elementtype, opcode, elementscansize, psrc.param().abi(), psrcelementflags.param().abi(), pdst.param().abi()).ok()
    }
}
unsafe impl Send for ID3DX11SegmentedScan {}
unsafe impl Sync for ID3DX11SegmentedScan {}
#[repr(C)]
pub struct ID3DX11SegmentedScan_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScanDirection: unsafe extern "system" fn(*mut core::ffi::c_void, D3DX11_SCAN_DIRECTION) -> windows_core::HRESULT,
    pub SegScan: unsafe extern "system" fn(*mut core::ffi::c_void, D3DX11_SCAN_DATA_TYPE, D3DX11_SCAN_OPCODE, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const D3D11_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED: D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG(1i32);
pub const D3D11_1_UAV_SLOT_COUNT: u32 = 64u32;
pub const D3D11_2_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
pub const D3D11_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D11_4_VIDEO_DECODER_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D11_4_VIDEO_DECODER_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
pub const D3D11_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D11_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D11_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D11_APPNAME_STRING: windows_core::PCWSTR = windows_core::w!("Name");
pub const D3D11_APPSIZE_STRING: windows_core::PCWSTR = windows_core::w!("Size");
pub const D3D11_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub const D3D11_ASYNC_GETDATA_DONOTFLUSH: D3D11_ASYNC_GETDATA_FLAG = D3D11_ASYNC_GETDATA_FLAG(1i32);
pub const D3D11_AUTHENTICATED_CHANNEL_D3D11: D3D11_AUTHENTICATED_CHANNEL_TYPE = D3D11_AUTHENTICATED_CHANNEL_TYPE(1i32);
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_HARDWARE: D3D11_AUTHENTICATED_CHANNEL_TYPE = D3D11_AUTHENTICATED_CHANNEL_TYPE(3i32);
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_SOFTWARE: D3D11_AUTHENTICATED_CHANNEL_TYPE = D3D11_AUTHENTICATED_CHANNEL_TYPE(2i32);
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: windows_core::GUID = windows_core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: windows_core::GUID = windows_core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: windows_core::GUID = windows_core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: windows_core::GUID = windows_core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
pub const D3D11_BIND_CONSTANT_BUFFER: D3D11_BIND_FLAG = D3D11_BIND_FLAG(4i32);
pub const D3D11_BIND_DECODER: D3D11_BIND_FLAG = D3D11_BIND_FLAG(512i32);
pub const D3D11_BIND_DEPTH_STENCIL: D3D11_BIND_FLAG = D3D11_BIND_FLAG(64i32);
pub const D3D11_BIND_INDEX_BUFFER: D3D11_BIND_FLAG = D3D11_BIND_FLAG(2i32);
pub const D3D11_BIND_RENDER_TARGET: D3D11_BIND_FLAG = D3D11_BIND_FLAG(32i32);
pub const D3D11_BIND_SHADER_RESOURCE: D3D11_BIND_FLAG = D3D11_BIND_FLAG(8i32);
pub const D3D11_BIND_STREAM_OUTPUT: D3D11_BIND_FLAG = D3D11_BIND_FLAG(16i32);
pub const D3D11_BIND_UNORDERED_ACCESS: D3D11_BIND_FLAG = D3D11_BIND_FLAG(128i32);
pub const D3D11_BIND_VERTEX_BUFFER: D3D11_BIND_FLAG = D3D11_BIND_FLAG(1i32);
pub const D3D11_BIND_VIDEO_ENCODER: D3D11_BIND_FLAG = D3D11_BIND_FLAG(1024i32);
pub const D3D11_BLEND_BLEND_FACTOR: D3D11_BLEND = D3D11_BLEND(14i32);
pub const D3D11_BLEND_DEST_ALPHA: D3D11_BLEND = D3D11_BLEND(7i32);
pub const D3D11_BLEND_DEST_COLOR: D3D11_BLEND = D3D11_BLEND(9i32);
pub const D3D11_BLEND_INV_BLEND_FACTOR: D3D11_BLEND = D3D11_BLEND(15i32);
pub const D3D11_BLEND_INV_DEST_ALPHA: D3D11_BLEND = D3D11_BLEND(8i32);
pub const D3D11_BLEND_INV_DEST_COLOR: D3D11_BLEND = D3D11_BLEND(10i32);
pub const D3D11_BLEND_INV_SRC1_ALPHA: D3D11_BLEND = D3D11_BLEND(19i32);
pub const D3D11_BLEND_INV_SRC1_COLOR: D3D11_BLEND = D3D11_BLEND(17i32);
pub const D3D11_BLEND_INV_SRC_ALPHA: D3D11_BLEND = D3D11_BLEND(6i32);
pub const D3D11_BLEND_INV_SRC_COLOR: D3D11_BLEND = D3D11_BLEND(4i32);
pub const D3D11_BLEND_ONE: D3D11_BLEND = D3D11_BLEND(2i32);
pub const D3D11_BLEND_OP_ADD: D3D11_BLEND_OP = D3D11_BLEND_OP(1i32);
pub const D3D11_BLEND_OP_MAX: D3D11_BLEND_OP = D3D11_BLEND_OP(5i32);
pub const D3D11_BLEND_OP_MIN: D3D11_BLEND_OP = D3D11_BLEND_OP(4i32);
pub const D3D11_BLEND_OP_REV_SUBTRACT: D3D11_BLEND_OP = D3D11_BLEND_OP(3i32);
pub const D3D11_BLEND_OP_SUBTRACT: D3D11_BLEND_OP = D3D11_BLEND_OP(2i32);
pub const D3D11_BLEND_SRC1_ALPHA: D3D11_BLEND = D3D11_BLEND(18i32);
pub const D3D11_BLEND_SRC1_COLOR: D3D11_BLEND = D3D11_BLEND(16i32);
pub const D3D11_BLEND_SRC_ALPHA: D3D11_BLEND = D3D11_BLEND(5i32);
pub const D3D11_BLEND_SRC_ALPHA_SAT: D3D11_BLEND = D3D11_BLEND(11i32);
pub const D3D11_BLEND_SRC_COLOR: D3D11_BLEND = D3D11_BLEND(3i32);
pub const D3D11_BLEND_ZERO: D3D11_BLEND = D3D11_BLEND(1i32);
pub const D3D11_BREAKON_CATEGORY: windows_core::PCWSTR = windows_core::w!("BreakOn_CATEGORY_%s");
pub const D3D11_BREAKON_ID_DECIMAL: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%d");
pub const D3D11_BREAKON_ID_STRING: windows_core::PCWSTR = windows_core::w!("BreakOn_ID_%s");
pub const D3D11_BREAKON_SEVERITY: windows_core::PCWSTR = windows_core::w!("BreakOn_SEVERITY_%s");
pub const D3D11_BUFFEREX_SRV_FLAG_RAW: D3D11_BUFFEREX_SRV_FLAG = D3D11_BUFFEREX_SRV_FLAG(1i32);
pub const D3D11_BUFFER_UAV_FLAG_APPEND: D3D11_BUFFER_UAV_FLAG = D3D11_BUFFER_UAV_FLAG(2i32);
pub const D3D11_BUFFER_UAV_FLAG_COUNTER: D3D11_BUFFER_UAV_FLAG = D3D11_BUFFER_UAV_FLAG(4i32);
pub const D3D11_BUFFER_UAV_FLAG_RAW: D3D11_BUFFER_UAV_FLAG = D3D11_BUFFER_UAV_FLAG(1i32);
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: D3D11_BUS_TYPE = D3D11_BUS_TYPE(262144i32);
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: D3D11_BUS_TYPE = D3D11_BUS_TYPE(327680i32);
pub const D3D11_BUS_IMPL_MODIFIER_INSIDE_OF_CHIPSET: D3D11_BUS_TYPE = D3D11_BUS_TYPE(65536i32);
pub const D3D11_BUS_IMPL_MODIFIER_NON_STANDARD: D3D11_BUS_TYPE = D3D11_BUS_TYPE(-2147483648i32);
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: D3D11_BUS_TYPE = D3D11_BUS_TYPE(131072i32);
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: D3D11_BUS_TYPE = D3D11_BUS_TYPE(196608i32);
pub const D3D11_BUS_TYPE_AGP: D3D11_BUS_TYPE = D3D11_BUS_TYPE(4i32);
pub const D3D11_BUS_TYPE_OTHER: D3D11_BUS_TYPE = D3D11_BUS_TYPE(0i32);
pub const D3D11_BUS_TYPE_PCI: D3D11_BUS_TYPE = D3D11_BUS_TYPE(1i32);
pub const D3D11_BUS_TYPE_PCIEXPRESS: D3D11_BUS_TYPE = D3D11_BUS_TYPE(3i32);
pub const D3D11_BUS_TYPE_PCIX: D3D11_BUS_TYPE = D3D11_BUS_TYPE(2i32);
pub const D3D11_CENTER_MULTISAMPLE_PATTERN: D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-2i32);
pub const D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE: D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG(1i32);
pub const D3D11_CLEAR_DEPTH: D3D11_CLEAR_FLAG = D3D11_CLEAR_FLAG(1u32);
pub const D3D11_CLEAR_STENCIL: D3D11_CLEAR_FLAG = D3D11_CLEAR_FLAG(2u32);
pub const D3D11_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D11_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub const D3D11_COLOR_WRITE_ENABLE_ALL: D3D11_COLOR_WRITE_ENABLE = D3D11_COLOR_WRITE_ENABLE(15i32);
pub const D3D11_COLOR_WRITE_ENABLE_ALPHA: D3D11_COLOR_WRITE_ENABLE = D3D11_COLOR_WRITE_ENABLE(8i32);
pub const D3D11_COLOR_WRITE_ENABLE_BLUE: D3D11_COLOR_WRITE_ENABLE = D3D11_COLOR_WRITE_ENABLE(4i32);
pub const D3D11_COLOR_WRITE_ENABLE_GREEN: D3D11_COLOR_WRITE_ENABLE = D3D11_COLOR_WRITE_ENABLE(2i32);
pub const D3D11_COLOR_WRITE_ENABLE_RED: D3D11_COLOR_WRITE_ENABLE = D3D11_COLOR_WRITE_ENABLE(1i32);
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D11_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D11_COMPARISON_ALWAYS: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(8i32);
pub const D3D11_COMPARISON_EQUAL: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(3i32);
pub const D3D11_COMPARISON_FILTERING_BIT: u32 = 128u32;
pub const D3D11_COMPARISON_GREATER: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(5i32);
pub const D3D11_COMPARISON_GREATER_EQUAL: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(7i32);
pub const D3D11_COMPARISON_LESS: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(2i32);
pub const D3D11_COMPARISON_LESS_EQUAL: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(4i32);
pub const D3D11_COMPARISON_NEVER: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(1i32);
pub const D3D11_COMPARISON_NOT_EQUAL: D3D11_COMPARISON_FUNC = D3D11_COMPARISON_FUNC(6i32);
pub const D3D11_COMPUTE_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(6i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_OFF: D3D11_CONSERVATIVE_RASTERIZATION_MODE = D3D11_CONSERVATIVE_RASTERIZATION_MODE(0i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_ON: D3D11_CONSERVATIVE_RASTERIZATION_MODE = D3D11_CONSERVATIVE_RASTERIZATION_MODE(1i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_NOT_SUPPORTED: D3D11_CONSERVATIVE_RASTERIZATION_TIER = D3D11_CONSERVATIVE_RASTERIZATION_TIER(0i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_1: D3D11_CONSERVATIVE_RASTERIZATION_TIER = D3D11_CONSERVATIVE_RASTERIZATION_TIER(1i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_2: D3D11_CONSERVATIVE_RASTERIZATION_TIER = D3D11_CONSERVATIVE_RASTERIZATION_TIER(2i32);
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_3: D3D11_CONSERVATIVE_RASTERIZATION_TIER = D3D11_CONSERVATIVE_RASTERIZATION_TIER(3i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_CONTENT_KEY: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(16i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_DECRYPTION_BLT: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(1024i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(64i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK_KEY: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(128i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPT_SLICEDATA_ONLY: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(512i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_FRESHEN_SESSION_KEY: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(32i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(2i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(16384i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION_MULTI_THREADED: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(32768i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECTED_MEMORY_PAGEABLE: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(4096i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECT_UNCOMPRESSED: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(2048i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(8192i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_PARTIAL_DECRYPTION: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(8i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_PROTECTION_ALWAYS_ON: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(4i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_SEQUENTIAL_CTR_IV: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(256i32);
pub const D3D11_CONTENT_PROTECTION_CAPS_SOFTWARE: D3D11_CONTENT_PROTECTION_CAPS = D3D11_CONTENT_PROTECTION_CAPS(1i32);
pub const D3D11_CONTEXT_TYPE_3D: D3D11_CONTEXT_TYPE = D3D11_CONTEXT_TYPE(1i32);
pub const D3D11_CONTEXT_TYPE_ALL: D3D11_CONTEXT_TYPE = D3D11_CONTEXT_TYPE(0i32);
pub const D3D11_CONTEXT_TYPE_COMPUTE: D3D11_CONTEXT_TYPE = D3D11_CONTEXT_TYPE(2i32);
pub const D3D11_CONTEXT_TYPE_COPY: D3D11_CONTEXT_TYPE = D3D11_CONTEXT_TYPE(3i32);
pub const D3D11_CONTEXT_TYPE_VIDEO: D3D11_CONTEXT_TYPE = D3D11_CONTEXT_TYPE(4i32);
pub const D3D11_COPY_DISCARD: D3D11_COPY_FLAGS = D3D11_COPY_FLAGS(2i32);
pub const D3D11_COPY_NO_OVERWRITE: D3D11_COPY_FLAGS = D3D11_COPY_FLAGS(1i32);
pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: D3D11_COUNTER = D3D11_COUNTER(1073741824i32);
pub const D3D11_COUNTER_TYPE_FLOAT32: D3D11_COUNTER_TYPE = D3D11_COUNTER_TYPE(0i32);
pub const D3D11_COUNTER_TYPE_UINT16: D3D11_COUNTER_TYPE = D3D11_COUNTER_TYPE(1i32);
pub const D3D11_COUNTER_TYPE_UINT32: D3D11_COUNTER_TYPE = D3D11_COUNTER_TYPE(2i32);
pub const D3D11_COUNTER_TYPE_UINT64: D3D11_COUNTER_TYPE = D3D11_COUNTER_TYPE(3i32);
pub const D3D11_CPU_ACCESS_READ: D3D11_CPU_ACCESS_FLAG = D3D11_CPU_ACCESS_FLAG(131072i32);
pub const D3D11_CPU_ACCESS_WRITE: D3D11_CPU_ACCESS_FLAG = D3D11_CPU_ACCESS_FLAG(65536i32);
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(32u32);
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(2u32);
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(64u32);
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(256u32);
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(128u32);
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(8u32);
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(1u32);
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(4u32);
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(2048u32);
pub const D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAG_NONE: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS = D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS(0i32);
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST: D3D11_CRYPTO_SESSION_STATUS = D3D11_CRYPTO_SESSION_STATUS(2i32);
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_LOST: D3D11_CRYPTO_SESSION_STATUS = D3D11_CRYPTO_SESSION_STATUS(1i32);
pub const D3D11_CRYPTO_SESSION_STATUS_OK: D3D11_CRYPTO_SESSION_STATUS = D3D11_CRYPTO_SESSION_STATUS(0i32);
pub const D3D11_CRYPTO_TYPE_AES128_CTR: windows_core::GUID = windows_core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3D11_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
pub const D3D11_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
pub const D3D11_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
pub const D3D11_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
pub const D3D11_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
pub const D3D11_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
pub const D3D11_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
pub const D3D11_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
pub const D3D11_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
pub const D3D11_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
pub const D3D11_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
pub const D3D11_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
pub const D3D11_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
pub const D3D11_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
pub const D3D11_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
pub const D3D11_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
pub const D3D11_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
pub const D3D11_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
pub const D3D11_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
pub const D3D11_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
pub const D3D11_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
pub const D3D11_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
pub const D3D11_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
pub const D3D11_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
pub const D3D11_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
pub const D3D11_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
pub const D3D11_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
pub const D3D11_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
pub const D3D11_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
pub const D3D11_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
pub const D3D11_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
pub const D3D11_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D11_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
pub const D3D11_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
pub const D3D11_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
pub const D3D11_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
pub const D3D11_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
pub const D3D11_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
pub const D3D11_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub const D3D11_CULL_BACK: D3D11_CULL_MODE = D3D11_CULL_MODE(3i32);
pub const D3D11_CULL_FRONT: D3D11_CULL_MODE = D3D11_CULL_MODE(2i32);
pub const D3D11_CULL_NONE: D3D11_CULL_MODE = D3D11_CULL_MODE(1i32);
pub const D3D11_DEBUG_FEATURE_ALWAYS_DISCARD_OFFERED_RESOURCE: u32 = 8u32;
pub const D3D11_DEBUG_FEATURE_AVOID_BEHAVIOR_CHANGING_DEBUG_AIDS: u32 = 64u32;
pub const D3D11_DEBUG_FEATURE_DISABLE_TILED_RESOURCE_MAPPING_TRACKING_AND_VALIDATION: u32 = 128u32;
pub const D3D11_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
pub const D3D11_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
pub const D3D11_DEBUG_FEATURE_NEVER_DISCARD_OFFERED_RESOURCE: u32 = 16u32;
pub const D3D11_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CBCS: windows_core::GUID = windows_core::GUID::from_u128(0x422d9319_9d21_4bb7_9371_faf5a82c3e04);
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CENC: windows_core::GUID = windows_core::GUID::from_u128(0xb0405235_c13d_44f2_9ae5_dd48e08e5b67);
pub const D3D11_DECODER_ENCRYPTION_HW_CENC: windows_core::GUID = windows_core::GUID::from_u128(0x89d6ac4f_09f2_4229_b2cd_37740a6dfd81);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2_420: windows_core::GUID = windows_core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE0: windows_core::GUID = windows_core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE1: windows_core::GUID = windows_core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D11_DECODER_PROFILE_H264_IDCT_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_IDCT_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_FGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D11_DECODER_PROFILE_H264_VLD_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D11_DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN: windows_core::GUID = windows_core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10: windows_core::GUID = windows_core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D11_DECODER_PROFILE_MPEG1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const D3D11_DECODER_PROFILE_MPEG2_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const D3D11_DECODER_PROFILE_MPEG2_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const D3D11_DECODER_PROFILE_MPEG2_VLD: windows_core::GUID = windows_core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D11_DECODER_PROFILE_MPEG2and1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC: windows_core::GUID = windows_core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC: windows_core::GUID = windows_core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE: windows_core::GUID = windows_core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D11_DECODER_PROFILE_VC1_D2010: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VC1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_VP8_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_10BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const D3D11_DECODER_PROFILE_VP9_VLD_PROFILE0: windows_core::GUID = windows_core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D11_DECODER_PROFILE_WMV8_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV8_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_MOCOMP: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DECODER_PROFILE_WMV9_POSTPROC: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D11_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D11_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D11_DEFAULT_DEPTH_BIAS: u32 = 0u32;
pub const D3D11_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D11_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
pub const D3D11_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D11_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D11_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D11_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D11_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D11_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D11_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D11_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D11_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D11_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
pub const D3D11_DEPTH_WRITE_MASK_ALL: D3D11_DEPTH_WRITE_MASK = D3D11_DEPTH_WRITE_MASK(1i32);
pub const D3D11_DEPTH_WRITE_MASK_ZERO: D3D11_DEPTH_WRITE_MASK = D3D11_DEPTH_WRITE_MASK(0i32);
pub const D3D11_DEVICE_CONTEXT_DEFERRED: D3D11_DEVICE_CONTEXT_TYPE = D3D11_DEVICE_CONTEXT_TYPE(1i32);
pub const D3D11_DEVICE_CONTEXT_IMMEDIATE: D3D11_DEVICE_CONTEXT_TYPE = D3D11_DEVICE_CONTEXT_TYPE(0i32);
pub const D3D11_DOMAIN_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(3i32);
pub const D3D11_DSV_DIMENSION_TEXTURE1D: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(1i32);
pub const D3D11_DSV_DIMENSION_TEXTURE1DARRAY: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(2i32);
pub const D3D11_DSV_DIMENSION_TEXTURE2D: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(3i32);
pub const D3D11_DSV_DIMENSION_TEXTURE2DARRAY: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(4i32);
pub const D3D11_DSV_DIMENSION_TEXTURE2DMS: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(5i32);
pub const D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(6i32);
pub const D3D11_DSV_DIMENSION_UNKNOWN: D3D11_DSV_DIMENSION = D3D11_DSV_DIMENSION(0i32);
pub const D3D11_DSV_READ_ONLY_DEPTH: D3D11_DSV_FLAG = D3D11_DSV_FLAG(1i32);
pub const D3D11_DSV_READ_ONLY_STENCIL: D3D11_DSV_FLAG = D3D11_DSV_FLAG(2i32);
pub const D3D11_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_ENABLE_BREAK_ON_MESSAGE: windows_core::PCWSTR = windows_core::w!("EnableBreakOnMessage");
pub const D3D11_FEATURE_ARCHITECTURE_INFO: D3D11_FEATURE = D3D11_FEATURE(6i32);
pub const D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS: D3D11_FEATURE = D3D11_FEATURE(4i32);
pub const D3D11_FEATURE_D3D11_OPTIONS: D3D11_FEATURE = D3D11_FEATURE(5i32);
pub const D3D11_FEATURE_D3D11_OPTIONS1: D3D11_FEATURE = D3D11_FEATURE(10i32);
pub const D3D11_FEATURE_D3D11_OPTIONS2: D3D11_FEATURE = D3D11_FEATURE(14i32);
pub const D3D11_FEATURE_D3D11_OPTIONS3: D3D11_FEATURE = D3D11_FEATURE(15i32);
pub const D3D11_FEATURE_D3D11_OPTIONS4: D3D11_FEATURE = D3D11_FEATURE(17i32);
pub const D3D11_FEATURE_D3D11_OPTIONS5: D3D11_FEATURE = D3D11_FEATURE(19i32);
pub const D3D11_FEATURE_D3D9_OPTIONS: D3D11_FEATURE = D3D11_FEATURE(7i32);
pub const D3D11_FEATURE_D3D9_OPTIONS1: D3D11_FEATURE = D3D11_FEATURE(13i32);
pub const D3D11_FEATURE_D3D9_SHADOW_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(9i32);
pub const D3D11_FEATURE_D3D9_SIMPLE_INSTANCING_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(11i32);
pub const D3D11_FEATURE_DISPLAYABLE: D3D11_FEATURE = D3D11_FEATURE(20i32);
pub const D3D11_FEATURE_DOUBLES: D3D11_FEATURE = D3D11_FEATURE(1i32);
pub const D3D11_FEATURE_FORMAT_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(2i32);
pub const D3D11_FEATURE_FORMAT_SUPPORT2: D3D11_FEATURE = D3D11_FEATURE(3i32);
pub const D3D11_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(16i32);
pub const D3D11_FEATURE_MARKER_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(12i32);
pub const D3D11_FEATURE_SHADER_CACHE: D3D11_FEATURE = D3D11_FEATURE(18i32);
pub const D3D11_FEATURE_SHADER_MIN_PRECISION_SUPPORT: D3D11_FEATURE = D3D11_FEATURE(8i32);
pub const D3D11_FEATURE_THREADING: D3D11_FEATURE = D3D11_FEATURE(0i32);
pub const D3D11_FEATURE_VIDEO_DECODER_HISTOGRAM: D3D11_FEATURE_VIDEO = D3D11_FEATURE_VIDEO(0i32);
pub const D3D11_FENCE_FLAG_NONE: D3D11_FENCE_FLAG = D3D11_FENCE_FLAG(0i32);
pub const D3D11_FENCE_FLAG_NON_MONITORED: D3D11_FENCE_FLAG = D3D11_FENCE_FLAG(8i32);
pub const D3D11_FENCE_FLAG_SHARED: D3D11_FENCE_FLAG = D3D11_FENCE_FLAG(2i32);
pub const D3D11_FENCE_FLAG_SHARED_CROSS_ADAPTER: D3D11_FENCE_FLAG = D3D11_FENCE_FLAG(4i32);
pub const D3D11_FILL_SOLID: D3D11_FILL_MODE = D3D11_FILL_MODE(3i32);
pub const D3D11_FILL_WIREFRAME: D3D11_FILL_MODE = D3D11_FILL_MODE(2i32);
pub const D3D11_FILTER_ANISOTROPIC: D3D11_FILTER = D3D11_FILTER(85i32);
pub const D3D11_FILTER_COMPARISON_ANISOTROPIC: D3D11_FILTER = D3D11_FILTER(213i32);
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(144i32);
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(145i32);
pub const D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(148i32);
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(149i32);
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(128i32);
pub const D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(129i32);
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(132i32);
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(133i32);
pub const D3D11_FILTER_MAXIMUM_ANISOTROPIC: D3D11_FILTER = D3D11_FILTER(469i32);
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(400i32);
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(401i32);
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(404i32);
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(405i32);
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(384i32);
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(385i32);
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(388i32);
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(389i32);
pub const D3D11_FILTER_MINIMUM_ANISOTROPIC: D3D11_FILTER = D3D11_FILTER(341i32);
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(272i32);
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(273i32);
pub const D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(276i32);
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(277i32);
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(256i32);
pub const D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(257i32);
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(260i32);
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(261i32);
pub const D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(16i32);
pub const D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(17i32);
pub const D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(20i32);
pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(21i32);
pub const D3D11_FILTER_MIN_MAG_MIP_POINT: D3D11_FILTER = D3D11_FILTER(0i32);
pub const D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(1i32);
pub const D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D11_FILTER = D3D11_FILTER(4i32);
pub const D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D11_FILTER = D3D11_FILTER(5i32);
pub const D3D11_FILTER_REDUCTION_TYPE_COMPARISON: D3D11_FILTER_REDUCTION_TYPE = D3D11_FILTER_REDUCTION_TYPE(1i32);
pub const D3D11_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
pub const D3D11_FILTER_REDUCTION_TYPE_MAXIMUM: D3D11_FILTER_REDUCTION_TYPE = D3D11_FILTER_REDUCTION_TYPE(3i32);
pub const D3D11_FILTER_REDUCTION_TYPE_MINIMUM: D3D11_FILTER_REDUCTION_TYPE = D3D11_FILTER_REDUCTION_TYPE(2i32);
pub const D3D11_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub const D3D11_FILTER_REDUCTION_TYPE_STANDARD: D3D11_FILTER_REDUCTION_TYPE = D3D11_FILTER_REDUCTION_TYPE(0i32);
pub const D3D11_FILTER_TYPE_LINEAR: D3D11_FILTER_TYPE = D3D11_FILTER_TYPE(1i32);
pub const D3D11_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D11_FILTER_TYPE_POINT: D3D11_FILTER_TYPE = D3D11_FILTER_TYPE(0i32);
pub const D3D11_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D11_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D11_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D11_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D11_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D11_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D11_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub const D3D11_FORCE_DEBUGGABLE: windows_core::PCWSTR = windows_core::w!("ForceDebuggable");
pub const D3D11_FORCE_SHADER_SKIP_OPTIMIZATION: windows_core::PCWSTR = windows_core::w!("ForceShaderSkipOptimization");
pub const D3D11_FORMAT_SUPPORT2_DISPLAYABLE: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(65536i32);
pub const D3D11_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(16384i32);
pub const D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(256i32);
pub const D3D11_FORMAT_SUPPORT2_SHAREABLE: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(1024i32);
pub const D3D11_FORMAT_SUPPORT2_TILED: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(512i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(1i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(2i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(4i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(8i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(16i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(32i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_LOAD: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(64i32);
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_STORE: D3D11_FORMAT_SUPPORT2 = D3D11_FORMAT_SUPPORT2(128i32);
pub const D3D11_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(16777216i32);
pub const D3D11_FORMAT_SUPPORT_BLENDABLE: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(32768i32);
pub const D3D11_FORMAT_SUPPORT_BUFFER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(1i32);
pub const D3D11_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(1048576i32);
pub const D3D11_FORMAT_SUPPORT_CPU_LOCKABLE: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(131072i32);
pub const D3D11_FORMAT_SUPPORT_DECODER_OUTPUT: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(134217728i32);
pub const D3D11_FORMAT_SUPPORT_DEPTH_STENCIL: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(65536i32);
pub const D3D11_FORMAT_SUPPORT_DISPLAY: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(524288i32);
pub const D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(4i32);
pub const D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(2i32);
pub const D3D11_FORMAT_SUPPORT_MIP: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(4096i32);
pub const D3D11_FORMAT_SUPPORT_MIP_AUTOGEN: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(8192i32);
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(4194304i32);
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(2097152i32);
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(262144i32);
pub const D3D11_FORMAT_SUPPORT_RENDER_TARGET: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(16384i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(8388608i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER_COMPARISON: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(67108864i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_LOAD: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(256i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(512i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(1024i32);
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(2048i32);
pub const D3D11_FORMAT_SUPPORT_SO_BUFFER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(8i32);
pub const D3D11_FORMAT_SUPPORT_TEXTURE1D: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(16i32);
pub const D3D11_FORMAT_SUPPORT_TEXTURE2D: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(32i32);
pub const D3D11_FORMAT_SUPPORT_TEXTURE3D: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(64i32);
pub const D3D11_FORMAT_SUPPORT_TEXTURECUBE: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(128i32);
pub const D3D11_FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(33554432i32);
pub const D3D11_FORMAT_SUPPORT_VIDEO_ENCODER: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(1073741824i32);
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(536870912i32);
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT: D3D11_FORMAT_SUPPORT = D3D11_FORMAT_SUPPORT(268435456i32);
pub const D3D11_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D11_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D11_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D11_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub const D3D11_GEOMETRY_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(4i32);
pub const D3D11_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
pub const D3D11_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
pub const D3D11_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
pub const D3D11_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D11_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
pub const D3D11_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
pub const D3D11_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
pub const D3D11_HULL_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(2i32);
pub const D3D11_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D11_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D11_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D11_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D11_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
pub const D3D11_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_INFOQUEUE_STORAGE_FILTER_OVERRIDE: windows_core::PCWSTR = windows_core::w!("InfoQueueStorageFilterOverride");
pub const D3D11_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub const D3D11_INPUT_PER_INSTANCE_DATA: D3D11_INPUT_CLASSIFICATION = D3D11_INPUT_CLASSIFICATION(1i32);
pub const D3D11_INPUT_PER_VERTEX_DATA: D3D11_INPUT_CLASSIFICATION = D3D11_INPUT_CLASSIFICATION(0i32);
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
pub const D3D11_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
pub const D3D11_KEY_EXCHANGE_HW_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xb1170d8a_628d_4da3_ad3b_82ddb08b4970);
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: windows_core::GUID = windows_core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
pub const D3D11_LINEAR_GAMMA: f32 = 1f32;
pub const D3D11_LOGIC_OP_AND: D3D11_LOGIC_OP = D3D11_LOGIC_OP(6i32);
pub const D3D11_LOGIC_OP_AND_INVERTED: D3D11_LOGIC_OP = D3D11_LOGIC_OP(13i32);
pub const D3D11_LOGIC_OP_AND_REVERSE: D3D11_LOGIC_OP = D3D11_LOGIC_OP(12i32);
pub const D3D11_LOGIC_OP_CLEAR: D3D11_LOGIC_OP = D3D11_LOGIC_OP(0i32);
pub const D3D11_LOGIC_OP_COPY: D3D11_LOGIC_OP = D3D11_LOGIC_OP(2i32);
pub const D3D11_LOGIC_OP_COPY_INVERTED: D3D11_LOGIC_OP = D3D11_LOGIC_OP(3i32);
pub const D3D11_LOGIC_OP_EQUIV: D3D11_LOGIC_OP = D3D11_LOGIC_OP(11i32);
pub const D3D11_LOGIC_OP_INVERT: D3D11_LOGIC_OP = D3D11_LOGIC_OP(5i32);
pub const D3D11_LOGIC_OP_NAND: D3D11_LOGIC_OP = D3D11_LOGIC_OP(7i32);
pub const D3D11_LOGIC_OP_NOOP: D3D11_LOGIC_OP = D3D11_LOGIC_OP(4i32);
pub const D3D11_LOGIC_OP_NOR: D3D11_LOGIC_OP = D3D11_LOGIC_OP(9i32);
pub const D3D11_LOGIC_OP_OR: D3D11_LOGIC_OP = D3D11_LOGIC_OP(8i32);
pub const D3D11_LOGIC_OP_OR_INVERTED: D3D11_LOGIC_OP = D3D11_LOGIC_OP(15i32);
pub const D3D11_LOGIC_OP_OR_REVERSE: D3D11_LOGIC_OP = D3D11_LOGIC_OP(14i32);
pub const D3D11_LOGIC_OP_SET: D3D11_LOGIC_OP = D3D11_LOGIC_OP(1i32);
pub const D3D11_LOGIC_OP_XOR: D3D11_LOGIC_OP = D3D11_LOGIC_OP(10i32);
pub const D3D11_MAG_FILTER_SHIFT: u32 = 2u32;
pub const D3D11_MAJOR_VERSION: u32 = 11u32;
pub const D3D11_MAP_FLAG_DO_NOT_WAIT: D3D11_MAP_FLAG = D3D11_MAP_FLAG(1048576i32);
pub const D3D11_MAP_READ: D3D11_MAP = D3D11_MAP(1i32);
pub const D3D11_MAP_READ_WRITE: D3D11_MAP = D3D11_MAP(3i32);
pub const D3D11_MAP_WRITE: D3D11_MAP = D3D11_MAP(2i32);
pub const D3D11_MAP_WRITE_DISCARD: D3D11_MAP = D3D11_MAP(4i32);
pub const D3D11_MAP_WRITE_NO_OVERWRITE: D3D11_MAP = D3D11_MAP(5i32);
pub const D3D11_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D11_MAX_DEPTH: f32 = 1f32;
pub const D3D11_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D11_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D11_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
pub const D3D11_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(0i32);
pub const D3D11_MESSAGE_CATEGORY_CLEANUP: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(3i32);
pub const D3D11_MESSAGE_CATEGORY_COMPILATION: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(4i32);
pub const D3D11_MESSAGE_CATEGORY_EXECUTION: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(9i32);
pub const D3D11_MESSAGE_CATEGORY_INITIALIZATION: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(2i32);
pub const D3D11_MESSAGE_CATEGORY_MISCELLANEOUS: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(1i32);
pub const D3D11_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(8i32);
pub const D3D11_MESSAGE_CATEGORY_SHADER: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(10i32);
pub const D3D11_MESSAGE_CATEGORY_STATE_CREATION: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(5i32);
pub const D3D11_MESSAGE_CATEGORY_STATE_GETTING: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(7i32);
pub const D3D11_MESSAGE_CATEGORY_STATE_SETTING: D3D11_MESSAGE_CATEGORY = D3D11_MESSAGE_CATEGORY(6i32);
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146029i32);
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDBIND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146028i32);
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146027i32);
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146026i32);
pub const D3D11_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(392i32);
pub const D3D11_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(297i32);
pub const D3D11_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(298i32);
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(296i32);
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(295i32);
pub const D3D11_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(299i32);
pub const D3D11_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146278i32);
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(402i32);
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(403i32);
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_INVALIDINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145994i32);
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145993i32);
pub const D3D11_MESSAGE_ID_CHECKCRYPTOSESSIONSTATUS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146085i32);
pub const D3D11_MESSAGE_ID_CHECKFEATURESUPPORT_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097371i32);
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(321i32);
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097401i32);
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(322i32);
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146138i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146090i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146089i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146091i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145768i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145769i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMATCONVERSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146096i32);
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145799i32);
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(262i32);
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DEPTH_READONLY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097369i32);
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(263i32);
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_STENCIL_READONLY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097370i32);
pub const D3D11_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(261i32);
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146145i32);
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097405i32);
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWUINT_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146144i32);
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_DENORMFLUSH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097355i32);
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_INVALIDPROCESSIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146015i32);
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146011i32);
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_UNSUPPORTEDCONFIGURE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146013i32);
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146012i32);
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146014i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(285i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(284i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(286i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048637i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048597i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048598i32);
pub const D3D11_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048596i32);
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097399i32);
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097397i32);
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097400i32);
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_LARGEOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097398i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION1_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145755i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_EMPTYSOURCEBOX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146078i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(282i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(278i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(281i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(280i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(283i32);
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(279i32);
pub const D3D11_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146126i32);
pub const D3D11_MESSAGE_ID_COPYTILES_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146127i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(28i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(13i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(22i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(23i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(24i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(25i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(26i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(27i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(14i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(15i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(16i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(17i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(18i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(19i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(20i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(21i32);
pub const D3D11_MESSAGE_ID_CORRUPTED_THIS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(12i32);
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145997i32);
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145995i32);
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145998i32);
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145996i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(214i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(217i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(213i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(216i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145941i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(218i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(212i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(215i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048600i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048612i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048613i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048623i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048622i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(220i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048624i32);
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(219i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(69i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(64i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(72i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(63i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(66i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(65i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(67i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(68i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(58i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSTRUCTURESTRIDE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097339i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146120i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(73i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(71i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(70i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(60i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(61i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(57i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(62i32);
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(59i32);
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097320i32);
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097324i32);
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097322i32);
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097323i32);
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097321i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(400i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(401i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(399i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(396i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(397i32);
pub const D3D11_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(398i32);
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145951i32);
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145952i32);
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097163i32);
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_CALL_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097164i32);
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_COMMANDLISTFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097161i32);
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097165i32);
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_SINGLETHREADED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097162i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(206i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(209i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(208i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(207i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(201i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(200i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(202i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(205i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(204i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(203i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(211i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048577i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(210i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(148i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(143i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(145i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097153i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(144i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(146i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(149i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(147i32);
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(142i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_FEATURELEVELS_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145752i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFEATURELEVEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145751i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145750i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDREFIID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145753i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146142i32);
pub const D3D11_MESSAGE_ID_CREATEDEVICE_WARNING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146143i32);
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097193i32);
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097197i32);
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097195i32);
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097196i32);
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097194i32);
pub const D3D11_MESSAGE_ID_CREATEFENCE_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146256i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(188i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(189i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(177i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097159i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(181i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(183i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(174i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTREAMS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097156i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097172i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(179i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(185i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(172i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(173i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(182i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097169i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097157i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(187i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(190i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(186i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(180i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(171i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(178i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(175i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(184i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(386i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(176i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097170i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTREAMS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097158i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097171i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097155i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(169i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(170i32);
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(168i32);
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097177i32);
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097181i32);
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097179i32);
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097180i32);
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097178i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(160i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(420i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(153i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(159i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(152i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(155i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(154i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(157i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(158i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_INSTANCING_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146124i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_STEPRATE_NOT_1: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146123i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(163i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(164i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(162i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(150i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(156i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(151i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(385i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(391i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(161i32);
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048599i32);
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097160i32);
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(192i32);
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(193i32);
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(191i32);
pub const D3D11_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(395i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_DECODENOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146158i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_ENCODENOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146159i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDCONTEXTTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146157i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(233i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(232i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(235i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(234i32);
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNSUPPORTEDCONTEXTTTYPEFORQUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146222i32);
pub const D3D11_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(394i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048578i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048601i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(195i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(196i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(194i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145757i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(197i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146155i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(199i32);
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(198i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146165i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(140i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145944i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(135i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(137i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(136i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146163i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(138i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146164i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(141i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(139i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(133i32);
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(134i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048630i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048588i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048617i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048610i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048634i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048608i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048589i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048585i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048616i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048609i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048614i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048587i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048586i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048631i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048615i32);
pub const D3D11_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048611i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048635i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048581i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048580i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(222i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(223i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(224i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(227i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(221i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(226i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(229i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(228i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(225i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048605i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048604i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048579i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048625i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(231i32);
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(230i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESESOURCEVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097359i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146162i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048607i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048606i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(131i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145942i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(126i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(128i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097340i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(127i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146160i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(129i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146161i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048603i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(132i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(130i32);
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(125i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(87i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(82i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(81i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(84i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(83i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(85i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(86i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(76i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146121i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(90i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(89i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(88i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(78i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(79i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(74i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(80i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(77i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(75i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(104i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(99i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(98i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(101i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(100i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(102i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(103i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(93i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146122i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(107i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(106i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(105i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(95i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(96i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(91i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(97i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(94i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(92i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(121i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(116i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(115i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(118i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(117i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(119i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(120i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(110i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(124i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(123i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(122i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(112i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(113i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(108i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(114i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(111i32);
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(109i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_AMBIGUOUSVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146168i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097351i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDARRAYWITHDECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145943i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097342i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097344i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097358i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097343i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146166i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097341i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANEINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146167i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097352i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_TOOMANYOBJECTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097353i32);
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097345i32);
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097154i32);
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(166i32);
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(167i32);
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(165i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145915i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145914i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145910i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145916i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145912i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145909i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145908i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145907i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145911i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145913i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145762i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145763i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145760i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145759i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145758i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145761i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDFRAMEFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145793i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDINPUTFRAMERATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145795i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDOUTPUTFRAMERATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145796i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145794i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDWIDTHHEIGHT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145797i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145792i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145791i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145928i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145927i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145920i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145929i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145923i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFOURCC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145924i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145925i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMISC: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145921i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMSAA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146073i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145919i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145922i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145918i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145917i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_UNSUPPORTEDMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145926i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145938i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDBIND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145933i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDDIMENSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145939i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145934i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145935i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMSAA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146074i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145932i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145931i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145930i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145937i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDMIP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145936i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145808i32);
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145807i32);
pub const D3D11_MESSAGE_ID_CREATE_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146148i32);
pub const D3D11_MESSAGE_ID_CREATE_BLENDSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097270i32);
pub const D3D11_MESSAGE_ID_CREATE_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097228i32);
pub const D3D11_MESSAGE_ID_CREATE_CLASSINSTANCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097290i32);
pub const D3D11_MESSAGE_ID_CREATE_CLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097293i32);
pub const D3D11_MESSAGE_ID_CREATE_COMMANDLIST: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097287i32);
pub const D3D11_MESSAGE_ID_CREATE_COMPUTESHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097298i32);
pub const D3D11_MESSAGE_ID_CREATE_CONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097225i32);
pub const D3D11_MESSAGE_ID_CREATE_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097285i32);
pub const D3D11_MESSAGE_ID_CREATE_CRYPTOSESSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146147i32);
pub const D3D11_MESSAGE_ID_CREATE_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145732i32);
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097273i32);
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097246i32);
pub const D3D11_MESSAGE_ID_CREATE_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145735i32);
pub const D3D11_MESSAGE_ID_CREATE_DOMAINSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097255i32);
pub const D3D11_MESSAGE_ID_CREATE_FENCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146250i32);
pub const D3D11_MESSAGE_ID_CREATE_GEOMETRYSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097258i32);
pub const D3D11_MESSAGE_ID_CREATE_HULLSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097252i32);
pub const D3D11_MESSAGE_ID_CREATE_INPUTLAYOUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097264i32);
pub const D3D11_MESSAGE_ID_CREATE_PIXELSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097261i32);
pub const D3D11_MESSAGE_ID_CREATE_PREDICATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097282i32);
pub const D3D11_MESSAGE_ID_CREATE_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145733i32);
pub const D3D11_MESSAGE_ID_CREATE_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145734i32);
pub const D3D11_MESSAGE_ID_CREATE_QUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097279i32);
pub const D3D11_MESSAGE_ID_CREATE_RASTERIZERSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097276i32);
pub const D3D11_MESSAGE_ID_CREATE_RENDERTARGETVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097243i32);
pub const D3D11_MESSAGE_ID_CREATE_SAMPLER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097267i32);
pub const D3D11_MESSAGE_ID_CREATE_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097240i32);
pub const D3D11_MESSAGE_ID_CREATE_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146253i32);
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE1D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097231i32);
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097234i32);
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE3D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097237i32);
pub const D3D11_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146267i32);
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_DEADLINE_TYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146272i32);
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_ENGINE_TYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146273i32);
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_MAX_INSTANCES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146271i32);
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146270i32);
pub const D3D11_MESSAGE_ID_CREATE_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097301i32);
pub const D3D11_MESSAGE_ID_CREATE_VERTEXSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097249i32);
pub const D3D11_MESSAGE_ID_CREATE_VIDEODECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145729i32);
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSOR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145731i32);
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145730i32);
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097326i32);
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146021i32);
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097319i32);
pub const D3D11_MESSAGE_ID_CSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097415i32);
pub const D3D11_MESSAGE_ID_CSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097318i32);
pub const D3D11_MESSAGE_ID_CSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097418i32);
pub const D3D11_MESSAGE_ID_CSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097349i32);
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048638i32);
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048576i32);
pub const D3D11_MESSAGE_ID_D3D10_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(443i32);
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146118i32);
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145728i32);
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146153i32);
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146119i32);
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146257i32);
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146154i32);
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146284i32);
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146258i32);
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097424i32);
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_START: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097152i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145785i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_MISC_FLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146265i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_OFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146266i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_SIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146263i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_USAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146264i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146262i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT_COUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146261i32);
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145784i32);
pub const D3D11_MESSAGE_ID_DECODERENDFRAME_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145786i32);
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145790i32);
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145789i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145983i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MULTISAMPLED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145979i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_NOT_RENDER_TARGET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145981i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_OFFERED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145985i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_WRONGDEVICE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145976i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_FORMAT_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145977i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145974i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SIZE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145978i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_CONTENT_UNDEFINED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145986i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145982i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_NOT_STAGING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145980i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_OFFERED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145984i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_WRONGDEVICE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145975i32);
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145973i32);
pub const D3D11_MESSAGE_ID_DEFERRED_CONTEXT_REMOVAL_PROCESS_AT_FAULT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097206i32);
pub const D3D11_MESSAGE_ID_DESTROY_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146152i32);
pub const D3D11_MESSAGE_ID_DESTROY_BLENDSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097272i32);
pub const D3D11_MESSAGE_ID_DESTROY_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097230i32);
pub const D3D11_MESSAGE_ID_DESTROY_CLASSINSTANCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097292i32);
pub const D3D11_MESSAGE_ID_DESTROY_CLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097295i32);
pub const D3D11_MESSAGE_ID_DESTROY_COMMANDLIST: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097289i32);
pub const D3D11_MESSAGE_ID_DESTROY_COMPUTESHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097300i32);
pub const D3D11_MESSAGE_ID_DESTROY_CONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097227i32);
pub const D3D11_MESSAGE_ID_DESTROY_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097286i32);
pub const D3D11_MESSAGE_ID_DESTROY_CRYPTOSESSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146151i32);
pub const D3D11_MESSAGE_ID_DESTROY_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145746i32);
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097275i32);
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097248i32);
pub const D3D11_MESSAGE_ID_DESTROY_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145749i32);
pub const D3D11_MESSAGE_ID_DESTROY_DOMAINSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097257i32);
pub const D3D11_MESSAGE_ID_DESTROY_FENCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146252i32);
pub const D3D11_MESSAGE_ID_DESTROY_GEOMETRYSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097260i32);
pub const D3D11_MESSAGE_ID_DESTROY_HULLSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097254i32);
pub const D3D11_MESSAGE_ID_DESTROY_INPUTLAYOUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097266i32);
pub const D3D11_MESSAGE_ID_DESTROY_PIXELSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097263i32);
pub const D3D11_MESSAGE_ID_DESTROY_PREDICATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097284i32);
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145747i32);
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145748i32);
pub const D3D11_MESSAGE_ID_DESTROY_QUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097281i32);
pub const D3D11_MESSAGE_ID_DESTROY_RASTERIZERSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097278i32);
pub const D3D11_MESSAGE_ID_DESTROY_RENDERTARGETVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097245i32);
pub const D3D11_MESSAGE_ID_DESTROY_SAMPLER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097269i32);
pub const D3D11_MESSAGE_ID_DESTROY_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097242i32);
pub const D3D11_MESSAGE_ID_DESTROY_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146255i32);
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE1D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097233i32);
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097236i32);
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE3D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097239i32);
pub const D3D11_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146269i32);
pub const D3D11_MESSAGE_ID_DESTROY_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097303i32);
pub const D3D11_MESSAGE_ID_DESTROY_VERTEXSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097251i32);
pub const D3D11_MESSAGE_ID_DESTROY_VIDEODECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145743i32);
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145745i32);
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145744i32);
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097315i32);
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097314i32);
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_UNRECOGNIZED_FEATURE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097313i32);
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146076i32);
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146066i32);
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146075i32);
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146035i32);
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_NOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146062i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146048i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097338i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146049i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146059i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146040i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097334i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146041i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146055i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146044i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097336i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146045i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146057i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146042i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097335i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146043i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146056i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146038i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097333i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146039i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146054i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146046i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097337i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146047i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146058i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATESHADER_CLASSLINKAGE_FULL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097312i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146036i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097332i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_SHADEREXTENSIONSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146037i32);
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146053i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097330i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097331i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097329i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSGETUNORDEREDACCESSS_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097357i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097327i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097317i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097328i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097316i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097325i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSS_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097356i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097354i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097403i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097402i32);
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_TOOMANYVIEWS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097404i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISCARDVIEW_INVALIDVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145754i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_INVALID_ARG_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097360i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097362i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097361i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097396i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_BOUND_RESOURCE_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097389i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097390i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_ZERO: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097391i32);
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097395i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(340i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(339i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(336i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_INVALID_ARG_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097207i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097209i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097208i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(338i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(337i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(364i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(350i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(351i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_DEPTHSTENCILVIEW_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146080i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(360i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_CONTROL_POINT_COUNT_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097223i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_SIGNATURE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097221i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097224i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_XOR_DS_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097205i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097222i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(358i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(357i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(359i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(368i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(349i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(365i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_SYSTEMVALUE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146156i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(417i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_FORCED_SAMPLE_COUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145940i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(377i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(376i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146079i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(363i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(369i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PIXEL_SHADER_WITHOUT_RTV_OR_DSV: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097408i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(362i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(415i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RASTERIZING_CONTROL_POINTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097219i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146081i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146082i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(416i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(370i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(372i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(371i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(373i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(361i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(421i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(390i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(352i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLE_MASK_IGNORED_ON_FL9: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146067i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(353i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(375i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(374i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_UNORDEREDACCESSVIEW_RENDERTARGETVIEW_OVERLAP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097374i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(335i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(348i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(355i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(356i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(366i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(341i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(367i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(384i32);
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(354i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097203i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097204i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097202i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097200i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097190i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097201i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097189i32);
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097198i32);
pub const D3D11_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(277i32);
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDCONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097366i32);
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097367i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(269i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(270i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(268i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(251i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(6i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(252i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(5i32);
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(249i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097187i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097188i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097186i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097184i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097174i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097185i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097173i32);
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097182i32);
pub const D3D11_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(264i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(242i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(243i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(244i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048594i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(237i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(236i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097220i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(240i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(419i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(239i32);
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(418i32);
pub const D3D11_MESSAGE_ID_DEVICE_LOCKEDOUT_INTERFACE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145945i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097346i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_INVALIDOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146060i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NO_OP: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097348i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NUMUAVS_INVALIDRANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097422i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_OVERLAPPING_OLD_SLOTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097347i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_TOOMANYVIEWS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146061i32);
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(9i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE1_ACCESS_DENIED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146117i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE1_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146068i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(383i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BY_NAME_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146069i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(381i32);
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(382i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(273i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(274i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(272i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(257i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(8i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(258i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(7i32);
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(255i32);
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(378i32);
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(380i32);
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(379i32);
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(290i32);
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(291i32);
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(294i32);
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(292i32);
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(293i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(276i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(275i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(260i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048632i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048595i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(387i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(259i32);
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048593i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETHARDWAREPROTECTION_INVALIDCONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146224i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDCONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097363i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDMINLOD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097365i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097364i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INSTANCE_DATA_BINDINGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097311i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACES_FEATURELEVEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097304i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACE_COUNT_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097305i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097306i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_DATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097309i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_INDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097307i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_TYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097308i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_UNBOUND_INSTANCE_DATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097310i32);
pub const D3D11_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(389i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_BUFFER_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097393i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_RAW_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097394i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_STRUCTURE_STRIDE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097392i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(344i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_MINPRECISION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146050i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(347i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(343i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(345i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(342i32);
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(346i32);
pub const D3D11_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(271i32);
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(10i32);
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(254i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_APPEND_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097376i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMICS_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097377i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_ADD_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097383i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_BITWISE_OPS_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097384i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_CMPSTORE_CMPEXCHANGE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097385i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_EXCHANGE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097386i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_SIGNED_MINMAX_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097387i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_UNSIGNED_MINMAX_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097388i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_BUFFER_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097379i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_COUNTER_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097406i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_DIMENSION_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097375i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_LD_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097381i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_STORE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097382i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097373i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146083i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RAW_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097380i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RETURN_TYPE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097372i32);
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_STRUCTURE_STRIDE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097378i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(266i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(267i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(265i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(247i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(4i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(248i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3i32);
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(245i32);
pub const D3D11_MESSAGE_ID_DIRTY_TILE_MAPPING_ACCESS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146133i32);
pub const D3D11_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048627i32);
pub const D3D11_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048628i32);
pub const D3D11_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048602i32);
pub const D3D11_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048626i32);
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097199i32);
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146017i32);
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097192i32);
pub const D3D11_MESSAGE_ID_DSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097414i32);
pub const D3D11_MESSAGE_ID_DSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097191i32);
pub const D3D11_MESSAGE_ID_DSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097417i32);
pub const D3D11_MESSAGE_ID_DUPLICATE_TILE_MAPPINGS_IN_COVERED_AREA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146134i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145969i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_NOT_STAGING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145967i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_OFFERED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145971i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_WRONGDEVICE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145963i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_FORMAT_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145964i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145961i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SIZE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145965i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_CONTENT_UNDEFINED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145972i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145968i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MULTISAMPLED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145966i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_OFFERED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145970i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_WRONGDEVICE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145962i32);
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145960i32);
pub const D3D11_MESSAGE_ID_END_TRACKED_WORKLOAD_INVALID_ARG: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146282i32);
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_ACCESSDENIED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097421i32);
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097419i32);
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146070i32);
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097420i32);
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_INVALID_CALL_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097168i32);
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_ONIMMEDIATECONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097166i32);
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097167i32);
pub const D3D11_MESSAGE_ID_FINISHSESSIONKEYREFRESH_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145989i32);
pub const D3D11_MESSAGE_ID_FLUSH1_INVALIDCONTEXTTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146223i32);
pub const D3D11_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048619i32);
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_INVALIDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145999i32);
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146000i32);
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_INVALIDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146001i32);
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146002i32);
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_WRONGSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146003i32);
pub const D3D11_MESSAGE_ID_GETCONTENTPROTECTIONCAPS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145992i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATESIZE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145955i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145956i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_WRONGSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145957i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146025i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_WRONGSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145958i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_INVALID_KEY_EXCHANGE_TYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146116i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146086i32);
pub const D3D11_MESSAGE_ID_GETCRYPTOTYPE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145953i32);
pub const D3D11_MESSAGE_ID_GETDATAFORNEWHARDWAREKEY_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146084i32);
pub const D3D11_MESSAGE_ID_GETDC_INACCESSIBLE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146065i32);
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145778i32);
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145779i32);
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_LOCKED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145780i32);
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145777i32);
pub const D3D11_MESSAGE_ID_GETDECODERCREATIONPARAMS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145775i32);
pub const D3D11_MESSAGE_ID_GETDECODERDRIVERHANDLE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145776i32);
pub const D3D11_MESSAGE_ID_GETDECODERPROFILE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145954i32);
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145991i32);
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145990i32);
pub const D3D11_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(51i32);
pub const D3D11_MESSAGE_ID_GETRESOURCETILING_NONTILED_RESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146139i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146087i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146088i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145770i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145771i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_INVALIDINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145773i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145772i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145774i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILECOUNT_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145764i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_INVALIDINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145766i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145765i32);
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145767i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCAPS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145800i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCONTENTDESC_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145798i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_INVALIDINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145804i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145803i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145805i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145806i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_INVALIDINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145802i32);
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145801i32);
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(250i32);
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146019i32);
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(39i32);
pub const D3D11_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(40i32);
pub const D3D11_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(38i32);
pub const D3D11_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(37i32);
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097183i32);
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146018i32);
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097176i32);
pub const D3D11_MESSAGE_ID_HSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097413i32);
pub const D3D11_MESSAGE_ID_HSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097175i32);
pub const D3D11_MESSAGE_ID_HSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097416i32);
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(241i32);
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(32i32);
pub const D3D11_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(30i32);
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048592i32);
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(238i32);
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(31i32);
pub const D3D11_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146276i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_1DESTUNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146194i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_3DESTUNSUPPORTEDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146195i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_BACKBUFFERNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146203i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_CHROMASIZEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146190i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXESINTERSECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146182i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOT2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146180i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOTSUB: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146181i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTINATIONNOT2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146173i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_DIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146171i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_EMPTYDESTBOX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146179i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146176i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_FRACTIONALDOWNSCALETOLARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146189i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_GUARDRECTSUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146175i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146199i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOMPONENTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146172i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146198i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDMIPLEVEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146178i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDNUMDESTINATIONS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146192i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSCANDATAOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146169i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSOURCESIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146197i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146177i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_LUMACHROMASIZEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146191i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_NONPOW2SCALEUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146188i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_NOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146170i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_OUTPUTDIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146187i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_SCALEUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146196i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_SUBBOXUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146193i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_TILEDRESOURCESUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146174i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDDSTTEXTUREUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146202i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146201i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146200i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPRTEDCOPYFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146204i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146183i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEODD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146185i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146184i32);
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEODD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146186i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_BACKBUFFERNOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146221i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_DIMENSIONSTOOLARGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146216i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146213i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_GUARDRECTSUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146210i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146217i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDCOMPONENTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146207i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDMIPLEVEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146215i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSCANDATAOFFSET: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146206i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146214i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_NOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146205i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_SOURCENOT2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146208i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_TILEDRESOURCESUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146209i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERMISCFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146219i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146218i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDSRCTEXTUREUSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146220i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_XSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146211i32);
pub const D3D11_MESSAGE_ID_JPEGENCODE_YSUBSAMPLEMISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146212i32);
pub const D3D11_MESSAGE_ID_LIVE_AUTHENTICATEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146150i32);
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(435i32);
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097271i32);
pub const D3D11_MESSAGE_ID_LIVE_BUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(423i32);
pub const D3D11_MESSAGE_ID_LIVE_BUFFER_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097229i32);
pub const D3D11_MESSAGE_ID_LIVE_CLASSINSTANCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097291i32);
pub const D3D11_MESSAGE_ID_LIVE_CLASSLINKAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097294i32);
pub const D3D11_MESSAGE_ID_LIVE_COMMANDLIST: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097288i32);
pub const D3D11_MESSAGE_ID_LIVE_COMPUTESHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097299i32);
pub const D3D11_MESSAGE_ID_LIVE_CONTEXT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097226i32);
pub const D3D11_MESSAGE_ID_LIVE_COUNTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(440i32);
pub const D3D11_MESSAGE_ID_LIVE_CRYPTOSESSION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146149i32);
pub const D3D11_MESSAGE_ID_LIVE_DECODEROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145739i32);
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(436i32);
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097274i32);
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(429i32);
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097247i32);
pub const D3D11_MESSAGE_ID_LIVE_DEVICE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(441i32);
pub const D3D11_MESSAGE_ID_LIVE_DEVICECONTEXTSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145742i32);
pub const D3D11_MESSAGE_ID_LIVE_DEVICE_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097296i32);
pub const D3D11_MESSAGE_ID_LIVE_DOMAINSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097256i32);
pub const D3D11_MESSAGE_ID_LIVE_FENCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146251i32);
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(431i32);
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097259i32);
pub const D3D11_MESSAGE_ID_LIVE_HULLSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097253i32);
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(433i32);
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097265i32);
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(422i32);
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097297i32);
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(432i32);
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097262i32);
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(439i32);
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097283i32);
pub const D3D11_MESSAGE_ID_LIVE_PROCESSORINPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145740i32);
pub const D3D11_MESSAGE_ID_LIVE_PROCESSOROUTPUTVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145741i32);
pub const D3D11_MESSAGE_ID_LIVE_QUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(438i32);
pub const D3D11_MESSAGE_ID_LIVE_QUERY_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097280i32);
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(437i32);
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097277i32);
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(428i32);
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097244i32);
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(434i32);
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097268i32);
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(427i32);
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097241i32);
pub const D3D11_MESSAGE_ID_LIVE_SWAPCHAIN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(442i32);
pub const D3D11_MESSAGE_ID_LIVE_SYNCHRONIZEDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146254i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(424i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097232i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(425i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097235i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(426i32);
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097238i32);
pub const D3D11_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146268i32);
pub const D3D11_MESSAGE_ID_LIVE_UNORDEREDACCESSVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097302i32);
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(430i32);
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER_WIN7: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097250i32);
pub const D3D11_MESSAGE_ID_LIVE_VIDEODECODER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145736i32);
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSOR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145738i32);
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSORENUM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145737i32);
pub const D3D11_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(29i32);
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146274i32);
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146275i32);
pub const D3D11_MESSAGE_ID_NEED_TO_CALL_TILEDRESOURCEBARRIER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146141i32);
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146004i32);
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146023i32);
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146005i32);
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146022i32);
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145959i32);
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_INVALIDKEYEXCHANGETYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146259i32);
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146260i32);
pub const D3D11_MESSAGE_ID_NO_TRACKED_WORKLOAD_SLOT_AVAILABLE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146281i32);
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_ERROR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146132i32);
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_WARNING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146131i32);
pub const D3D11_MESSAGE_ID_OFFERRELEASE_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146071i32);
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INACCESSIBLE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146072i32);
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDPRIORITY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146024i32);
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097412i32);
pub const D3D11_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(47i32);
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(48i32);
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCIL_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097368i32);
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(388i32);
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048591i32);
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048636i32);
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048590i32);
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(49i32);
pub const D3D11_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146277i32);
pub const D3D11_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(406i32);
pub const D3D11_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(409i32);
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(256i32);
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146020i32);
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(44i32);
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048584i32);
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(45i32);
pub const D3D11_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(43i32);
pub const D3D11_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(42i32);
pub const D3D11_MESSAGE_ID_PSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097350i32);
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_INVALIDPROCESSINDEX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146010i32);
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146006i32);
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_UNSUPPORTEDQUERY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146008i32);
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGCHANNEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146007i32);
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146009i32);
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(408i32);
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(407i32);
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(405i32);
pub const D3D11_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(410i32);
pub const D3D11_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(411i32);
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(414i32);
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(412i32);
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(413i32);
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146107i32);
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146106i32);
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146108i32);
pub const D3D11_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(331i32);
pub const D3D11_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(330i32);
pub const D3D11_MESSAGE_ID_REF_INFO: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(334i32);
pub const D3D11_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(329i32);
pub const D3D11_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(333i32);
pub const D3D11_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(332i32);
pub const D3D11_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(326i32);
pub const D3D11_MESSAGE_ID_REF_THREADING_MODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(327i32);
pub const D3D11_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(328i32);
pub const D3D11_MESSAGE_ID_REF_WARNING: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097407i32);
pub const D3D11_MESSAGE_ID_REF_WARNING_ATOMIC_INCONSISTENT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145946i32);
pub const D3D11_MESSAGE_ID_REF_WARNING_RAW_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145948i32);
pub const D3D11_MESSAGE_ID_REF_WARNING_READING_UNINITIALIZED_RESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145947i32);
pub const D3D11_MESSAGE_ID_REF_WARNING_WAR_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145949i32);
pub const D3D11_MESSAGE_ID_REF_WARNING_WAW_HAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145950i32);
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145782i32);
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NOTLOCKED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145783i32);
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145781i32);
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146129i32);
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_SHRINK_WITH_MAPPINGS_STILL_DEFINED_PAST_END: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146140i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097213i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097214i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097212i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097210i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097211i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_OUTOFMEMORY_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097215i32);
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_WITHOUT_INITIAL_DISCARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097216i32);
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097217i32);
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097218i32);
pub const D3D11_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(46i32);
pub const D3D11_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048629i32);
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(325i32);
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(324i32);
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(323i32);
pub const D3D11_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(404i32);
pub const D3D11_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(50i32);
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(55i32);
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(54i32);
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(52i32);
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(53i32);
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(56i32);
pub const D3D11_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(393i32);
pub const D3D11_MESSAGE_ID_SHADER_ABORT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097409i32);
pub const D3D11_MESSAGE_ID_SHADER_ERROR: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097411i32);
pub const D3D11_MESSAGE_ID_SHADER_MESSAGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097410i32);
pub const D3D11_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048633i32);
pub const D3D11_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(253i32);
pub const D3D11_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(41i32);
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145988i32);
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145987i32);
pub const D3D11_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048620i32);
pub const D3D11_MESSAGE_ID_STRING_FROM_APPLICATION: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(11i32);
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_INVALIDTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145788i32);
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145787i32);
pub const D3D11_MESSAGE_ID_SWAPDEVICECONTEXTSTATE_NOTSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146063i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(303i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(304i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(302i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(300i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(301i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(305i32);
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(306i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(310i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(311i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(309i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(307i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(308i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(312i32);
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(313i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(317i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(318i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(316i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(314i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(315i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(319i32);
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(320i32);
pub const D3D11_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048621i32);
pub const D3D11_MESSAGE_ID_TILEDRESOURCEBARRIER_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146130i32);
pub const D3D11_MESSAGE_ID_TILED_RESOURCE_TIER_1_BUFFER_TEXTURE_MISMATCH: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146146i32);
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_IN_COVERED_AREA_DUPLICATED_OUTSIDE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146135i32);
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INCOMPATIBLE_RESOURCES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146136i32);
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INPUT_AND_OUTPUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146137i32);
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_DISJOINT_FAILURE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146283i32);
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_ENGINE_TYPE_NOT_FOUND: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146280i32);
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146279i32);
pub const D3D11_MESSAGE_ID_UNKNOWN: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(0i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE1_INVALIDCOPYFLAGS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145756i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_EMPTYDESTBOX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146077i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(288i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(289i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(287i32);
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_PREFERUPDATESUBRESOURCE1: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146064i32);
pub const D3D11_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146125i32);
pub const D3D11_MESSAGE_ID_UPDATETILES_INVALID_PARAMETER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146128i32);
pub const D3D11_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(2097423i32);
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146092i32);
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146093i32);
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146094i32);
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146095i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INPUTHAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145905i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAY: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145899i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAYSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145898i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDDESTRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145896i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDFUTUREFRAMES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145894i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDINPUTRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145897i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDOUTPUT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145892i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDPASTFRAMES: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145893i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDRIGHTRESOURCE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145903i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSOURCERECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145895i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSTREAMCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145890i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NOSTEREOSTREAMS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145904i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145889i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_OUTPUTHAZARD: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145906i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTEXPECTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145900i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTNOTEXPECTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145901i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_STEREONOTENABLED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145902i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_TARGETRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145891i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDDESTRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146115i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSOURCERECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146114i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSTREAMCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146112i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146111i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_TARGETRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146113i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTALPHAFILLMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145824i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTBACKGROUNDCOLOR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145822i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146098i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145823i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCONSTRICTION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145825i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145829i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146228i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146227i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSHADERUSAGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146110i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSTEREOMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145828i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTTARGETRECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145821i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146240i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145880i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146245i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145885i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146248i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146104i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146236i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145876i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146239i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145879i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145888i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145887i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146246i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145886i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146235i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145875i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146234i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146233i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146232i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146243i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145883i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146249i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146105i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146237i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145877i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146241i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145881i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146242i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145882i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146247i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146034i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146238i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145878i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146244i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145884i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDFILLMODE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145816i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145815i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145813i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145814i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_INVALIDALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145811i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145810i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146097i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145812i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145827i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145817i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145826i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145820i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146226i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146225i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSHADERUSAGE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146109i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145818i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145819i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTTARGETRECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145809i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145847i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145846i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145845i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146051i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145867i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145866i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146100i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146099i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145834i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145833i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145844i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145843i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145842i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145874i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145873i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDFILTER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145870i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDLEVEL: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145872i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145869i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145868i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145871i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145831i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145832i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145830i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSIZE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146231i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146230i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146229i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDRANGE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145857i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145856i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145855i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145858i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146102i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146101i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146103i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDFLAG: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145837i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDRATE: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145836i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145838i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145835i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDALPHA: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145851i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145850i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145849i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145848i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDRATIO: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145854i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145853i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145852i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146052i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALID: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146032i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146031i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146030i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146033i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDRECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145841i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145840i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145839i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FLIPUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145862i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FORMATUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145864i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDFORMAT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145865i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDSTREAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145860i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_MONOOFFSETUNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145863i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_NULLPARAM: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145859i32);
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_UNSUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3145861i32);
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(246i32);
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(3146016i32);
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(35i32);
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048582i32);
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048583i32);
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(36i32);
pub const D3D11_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(34i32);
pub const D3D11_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(33i32);
pub const D3D11_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D11_MESSAGE_ID = D3D11_MESSAGE_ID(1048618i32);
pub const D3D11_MESSAGE_SEVERITY_CORRUPTION: D3D11_MESSAGE_SEVERITY = D3D11_MESSAGE_SEVERITY(0i32);
pub const D3D11_MESSAGE_SEVERITY_ERROR: D3D11_MESSAGE_SEVERITY = D3D11_MESSAGE_SEVERITY(1i32);
pub const D3D11_MESSAGE_SEVERITY_INFO: D3D11_MESSAGE_SEVERITY = D3D11_MESSAGE_SEVERITY(3i32);
pub const D3D11_MESSAGE_SEVERITY_MESSAGE: D3D11_MESSAGE_SEVERITY = D3D11_MESSAGE_SEVERITY(4i32);
pub const D3D11_MESSAGE_SEVERITY_WARNING: D3D11_MESSAGE_SEVERITY = D3D11_MESSAGE_SEVERITY(2i32);
pub const D3D11_MINOR_VERSION: u32 = 0u32;
pub const D3D11_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D11_MIN_DEPTH: f32 = 0f32;
pub const D3D11_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D11_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D11_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D11_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D11_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D11_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D11_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D11_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D11_MUTE_CATEGORY: windows_core::PCWSTR = windows_core::w!("Mute_CATEGORY_%s");
pub const D3D11_MUTE_DEBUG_OUTPUT: windows_core::PCWSTR = windows_core::w!("MuteDebugOutput");
pub const D3D11_MUTE_ID_DECIMAL: windows_core::PCWSTR = windows_core::w!("Mute_ID_%d");
pub const D3D11_MUTE_ID_STRING: windows_core::PCWSTR = windows_core::w!("Mute_ID_%s");
pub const D3D11_MUTE_SEVERITY: windows_core::PCWSTR = windows_core::w!("Mute_SEVERITY_%s");
pub const D3D11_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
pub const D3D11_PACKED_TILE: u32 = 4294967295u32;
pub const D3D11_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub const D3D11_PIXEL_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(5i32);
pub const D3D11_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D11_PROCESSIDTYPE_DWM: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE(1i32);
pub const D3D11_PROCESSIDTYPE_HANDLE: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE(2i32);
pub const D3D11_PROCESSIDTYPE_UNKNOWN: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE = D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE(0i32);
pub const D3D11_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
pub const D3D11_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D11_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D11_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D11_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D11_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub const D3D11_QUERY_EVENT: D3D11_QUERY = D3D11_QUERY(0i32);
pub const D3D11_QUERY_MISC_PREDICATEHINT: D3D11_QUERY_MISC_FLAG = D3D11_QUERY_MISC_FLAG(1i32);
pub const D3D11_QUERY_OCCLUSION: D3D11_QUERY = D3D11_QUERY(1i32);
pub const D3D11_QUERY_OCCLUSION_PREDICATE: D3D11_QUERY = D3D11_QUERY(5i32);
pub const D3D11_QUERY_PIPELINE_STATISTICS: D3D11_QUERY = D3D11_QUERY(4i32);
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE: D3D11_QUERY = D3D11_QUERY(7i32);
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0: D3D11_QUERY = D3D11_QUERY(9i32);
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1: D3D11_QUERY = D3D11_QUERY(11i32);
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2: D3D11_QUERY = D3D11_QUERY(13i32);
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3: D3D11_QUERY = D3D11_QUERY(15i32);
pub const D3D11_QUERY_SO_STATISTICS: D3D11_QUERY = D3D11_QUERY(6i32);
pub const D3D11_QUERY_SO_STATISTICS_STREAM0: D3D11_QUERY = D3D11_QUERY(8i32);
pub const D3D11_QUERY_SO_STATISTICS_STREAM1: D3D11_QUERY = D3D11_QUERY(10i32);
pub const D3D11_QUERY_SO_STATISTICS_STREAM2: D3D11_QUERY = D3D11_QUERY(12i32);
pub const D3D11_QUERY_SO_STATISTICS_STREAM3: D3D11_QUERY = D3D11_QUERY(14i32);
pub const D3D11_QUERY_TIMESTAMP: D3D11_QUERY = D3D11_QUERY(2i32);
pub const D3D11_QUERY_TIMESTAMP_DISJOINT: D3D11_QUERY = D3D11_QUERY(3i32);
pub const D3D11_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D11_RAISE_FLAG = D3D11_RAISE_FLAG(1i32);
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D11_REGKEY_PATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Direct3D");
pub const D3D11_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D11_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D11_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D11_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D11_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D11_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D11_REQ_MIP_LEVELS: u32 = 15u32;
pub const D3D11_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D11_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
pub const D3D11_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
pub const D3D11_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
pub const D3D11_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub const D3D11_RESOURCE_DIMENSION_BUFFER: D3D11_RESOURCE_DIMENSION = D3D11_RESOURCE_DIMENSION(1i32);
pub const D3D11_RESOURCE_DIMENSION_TEXTURE1D: D3D11_RESOURCE_DIMENSION = D3D11_RESOURCE_DIMENSION(2i32);
pub const D3D11_RESOURCE_DIMENSION_TEXTURE2D: D3D11_RESOURCE_DIMENSION = D3D11_RESOURCE_DIMENSION(3i32);
pub const D3D11_RESOURCE_DIMENSION_TEXTURE3D: D3D11_RESOURCE_DIMENSION = D3D11_RESOURCE_DIMENSION(4i32);
pub const D3D11_RESOURCE_DIMENSION_UNKNOWN: D3D11_RESOURCE_DIMENSION = D3D11_RESOURCE_DIMENSION(0i32);
pub const D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(32i32);
pub const D3D11_RESOURCE_MISC_BUFFER_STRUCTURED: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(64i32);
pub const D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(16i32);
pub const D3D11_RESOURCE_MISC_GDI_COMPATIBLE: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(512i32);
pub const D3D11_RESOURCE_MISC_GENERATE_MIPS: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(1i32);
pub const D3D11_RESOURCE_MISC_GUARDED: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(32768i32);
pub const D3D11_RESOURCE_MISC_HW_PROTECTED: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(524288i32);
pub const D3D11_RESOURCE_MISC_RESOURCE_CLAMP: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(128i32);
pub const D3D11_RESOURCE_MISC_RESTRICTED_CONTENT: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(4096i32);
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(8192i32);
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(16384i32);
pub const D3D11_RESOURCE_MISC_SHARED: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(2i32);
pub const D3D11_RESOURCE_MISC_SHARED_DISPLAYABLE: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(1048576i32);
pub const D3D11_RESOURCE_MISC_SHARED_EXCLUSIVE_WRITER: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(2097152i32);
pub const D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(256i32);
pub const D3D11_RESOURCE_MISC_SHARED_NTHANDLE: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(2048i32);
pub const D3D11_RESOURCE_MISC_TEXTURECUBE: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(4i32);
pub const D3D11_RESOURCE_MISC_TILED: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(262144i32);
pub const D3D11_RESOURCE_MISC_TILE_POOL: D3D11_RESOURCE_MISC_FLAG = D3D11_RESOURCE_MISC_FLAG(131072i32);
pub const D3D11_RLDO_DETAIL: D3D11_RLDO_FLAGS = D3D11_RLDO_FLAGS(2i32);
pub const D3D11_RLDO_IGNORE_INTERNAL: D3D11_RLDO_FLAGS = D3D11_RLDO_FLAGS(4i32);
pub const D3D11_RLDO_SUMMARY: D3D11_RLDO_FLAGS = D3D11_RLDO_FLAGS(1i32);
pub const D3D11_RTV_DIMENSION_BUFFER: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(1i32);
pub const D3D11_RTV_DIMENSION_TEXTURE1D: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(2i32);
pub const D3D11_RTV_DIMENSION_TEXTURE1DARRAY: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(3i32);
pub const D3D11_RTV_DIMENSION_TEXTURE2D: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(4i32);
pub const D3D11_RTV_DIMENSION_TEXTURE2DARRAY: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(5i32);
pub const D3D11_RTV_DIMENSION_TEXTURE2DMS: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(6i32);
pub const D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(7i32);
pub const D3D11_RTV_DIMENSION_TEXTURE3D: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(8i32);
pub const D3D11_RTV_DIMENSION_UNKNOWN: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION(0i32);
pub const D3D11_SDK_LAYERS_VERSION: u32 = 1u32;
pub const D3D11_SDK_VERSION: u32 = 7u32;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = D3D11_SHADER_CACHE_SUPPORT_FLAGS(2i32);
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = D3D11_SHADER_CACHE_SUPPORT_FLAGS(1i32);
pub const D3D11_SHADER_CACHE_SUPPORT_NONE: D3D11_SHADER_CACHE_SUPPORT_FLAGS = D3D11_SHADER_CACHE_SUPPORT_FLAGS(0i32);
pub const D3D11_SHADER_MAJOR_VERSION: u32 = 5u32;
pub const D3D11_SHADER_MAX_INSTANCES: u32 = 65535u32;
pub const D3D11_SHADER_MAX_INTERFACES: u32 = 253u32;
pub const D3D11_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
pub const D3D11_SHADER_MAX_TYPES: u32 = 65535u32;
pub const D3D11_SHADER_MINOR_VERSION: u32 = 0u32;
pub const D3D11_SHADER_MIN_PRECISION_10_BIT: D3D11_SHADER_MIN_PRECISION_SUPPORT = D3D11_SHADER_MIN_PRECISION_SUPPORT(1i32);
pub const D3D11_SHADER_MIN_PRECISION_16_BIT: D3D11_SHADER_MIN_PRECISION_SUPPORT = D3D11_SHADER_MIN_PRECISION_SUPPORT(2i32);
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_READS: u32 = 2u32;
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_WRITES: u32 = 1u32;
pub const D3D11_SHADER_TRACKING_OPTION_ALLOW_SAME: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(16i32);
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(1006i32);
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS_ALLOWING_SAME: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(1022i32);
pub const D3D11_SHADER_TRACKING_OPTION_ALL_OPTIONS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(1023i32);
pub const D3D11_SHADER_TRACKING_OPTION_IGNORE: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(0i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(32i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(512i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(2i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(64i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_UNINITIALIZED: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(1i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(4i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(128i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(8i32);
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW_ACROSS_THREADGROUPS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(256i32);
pub const D3D11_SHADER_TRACKING_OPTION_UAV_SPECIFIC_FLAGS: D3D11_SHADER_TRACKING_OPTIONS = D3D11_SHADER_TRACKING_OPTIONS(960i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(7i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(3i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_SHARED_MEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(5i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_MEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(4i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_NON_UAV: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(6i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NONE: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(0i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NON_UAV_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(2i32);
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_UAV_DEVICEMEMORY: D3D11_SHADER_TRACKING_RESOURCE_TYPE = D3D11_SHADER_TRACKING_RESOURCE_TYPE(1i32);
pub const D3D11_SHARED_RESOURCE_TIER_0: D3D11_SHARED_RESOURCE_TIER = D3D11_SHARED_RESOURCE_TIER(0i32);
pub const D3D11_SHARED_RESOURCE_TIER_1: D3D11_SHARED_RESOURCE_TIER = D3D11_SHARED_RESOURCE_TIER(1i32);
pub const D3D11_SHARED_RESOURCE_TIER_2: D3D11_SHARED_RESOURCE_TIER = D3D11_SHARED_RESOURCE_TIER(2i32);
pub const D3D11_SHARED_RESOURCE_TIER_3: D3D11_SHARED_RESOURCE_TIER = D3D11_SHARED_RESOURCE_TIER(3i32);
pub const D3D11_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D11_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
pub const D3D11_SHVER_COMPUTE_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(5i32);
pub const D3D11_SHVER_DOMAIN_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(4i32);
pub const D3D11_SHVER_GEOMETRY_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(2i32);
pub const D3D11_SHVER_HULL_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(3i32);
pub const D3D11_SHVER_PIXEL_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(0i32);
pub const D3D11_SHVER_RESERVED0: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(65520i32);
pub const D3D11_SHVER_VERTEX_SHADER: D3D11_SHADER_VERSION_TYPE = D3D11_SHADER_VERSION_TYPE(1i32);
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D11_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D11_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
pub const D3D11_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D11_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
pub const D3D11_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
pub const D3D11_SO_STREAM_COUNT: u32 = 4u32;
pub const D3D11_SPEC_DATE_DAY: u32 = 16u32;
pub const D3D11_SPEC_DATE_MONTH: u32 = 5u32;
pub const D3D11_SPEC_DATE_YEAR: u32 = 2011u32;
pub const D3D11_SPEC_VERSION: f64 = 1.07f64;
pub const D3D11_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D11_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D11_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D11_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D11_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D11_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D11_STANDARD_MULTISAMPLE_PATTERN: D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS = D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS(-1i32);
pub const D3D11_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D11_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D11_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub const D3D11_STENCIL_OP_DECR: D3D11_STENCIL_OP = D3D11_STENCIL_OP(8i32);
pub const D3D11_STENCIL_OP_DECR_SAT: D3D11_STENCIL_OP = D3D11_STENCIL_OP(5i32);
pub const D3D11_STENCIL_OP_INCR: D3D11_STENCIL_OP = D3D11_STENCIL_OP(7i32);
pub const D3D11_STENCIL_OP_INCR_SAT: D3D11_STENCIL_OP = D3D11_STENCIL_OP(4i32);
pub const D3D11_STENCIL_OP_INVERT: D3D11_STENCIL_OP = D3D11_STENCIL_OP(6i32);
pub const D3D11_STENCIL_OP_KEEP: D3D11_STENCIL_OP = D3D11_STENCIL_OP(1i32);
pub const D3D11_STENCIL_OP_REPLACE: D3D11_STENCIL_OP = D3D11_STENCIL_OP(3i32);
pub const D3D11_STENCIL_OP_ZERO: D3D11_STENCIL_OP = D3D11_STENCIL_OP(2i32);
pub const D3D11_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D11_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D11_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
pub const D3D11_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
pub const D3D11_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D11_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D11_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_X: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(1i32);
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Y: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(3i32);
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Z: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(5i32);
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_X: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(0i32);
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Y: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(2i32);
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Z: D3D11_TEXTURECUBE_FACE = D3D11_TEXTURECUBE_FACE(4i32);
pub const D3D11_TEXTURE_ADDRESS_BORDER: D3D11_TEXTURE_ADDRESS_MODE = D3D11_TEXTURE_ADDRESS_MODE(4i32);
pub const D3D11_TEXTURE_ADDRESS_CLAMP: D3D11_TEXTURE_ADDRESS_MODE = D3D11_TEXTURE_ADDRESS_MODE(3i32);
pub const D3D11_TEXTURE_ADDRESS_MIRROR: D3D11_TEXTURE_ADDRESS_MODE = D3D11_TEXTURE_ADDRESS_MODE(2i32);
pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: D3D11_TEXTURE_ADDRESS_MODE = D3D11_TEXTURE_ADDRESS_MODE(5i32);
pub const D3D11_TEXTURE_ADDRESS_WRAP: D3D11_TEXTURE_ADDRESS_MODE = D3D11_TEXTURE_ADDRESS_MODE(1i32);
pub const D3D11_TEXTURE_LAYOUT_64K_STANDARD_SWIZZLE: D3D11_TEXTURE_LAYOUT = D3D11_TEXTURE_LAYOUT(2i32);
pub const D3D11_TEXTURE_LAYOUT_ROW_MAJOR: D3D11_TEXTURE_LAYOUT = D3D11_TEXTURE_LAYOUT(1i32);
pub const D3D11_TEXTURE_LAYOUT_UNDEFINED: D3D11_TEXTURE_LAYOUT = D3D11_TEXTURE_LAYOUT(0i32);
pub const D3D11_TILED_RESOURCES_NOT_SUPPORTED: D3D11_TILED_RESOURCES_TIER = D3D11_TILED_RESOURCES_TIER(0i32);
pub const D3D11_TILED_RESOURCES_TIER_1: D3D11_TILED_RESOURCES_TIER = D3D11_TILED_RESOURCES_TIER(1i32);
pub const D3D11_TILED_RESOURCES_TIER_2: D3D11_TILED_RESOURCES_TIER = D3D11_TILED_RESOURCES_TIER(2i32);
pub const D3D11_TILED_RESOURCES_TIER_3: D3D11_TILED_RESOURCES_TIER = D3D11_TILED_RESOURCES_TIER(3i32);
pub const D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: D3D11_TILE_COPY_FLAG = D3D11_TILE_COPY_FLAG(2i32);
pub const D3D11_TILE_COPY_NO_OVERWRITE: D3D11_TILE_COPY_FLAG = D3D11_TILE_COPY_FLAG(1i32);
pub const D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: D3D11_TILE_COPY_FLAG = D3D11_TILE_COPY_FLAG(4i32);
pub const D3D11_TILE_MAPPING_NO_OVERWRITE: D3D11_TILE_MAPPING_FLAG = D3D11_TILE_MAPPING_FLAG(1i32);
pub const D3D11_TILE_RANGE_NULL: D3D11_TILE_RANGE_FLAG = D3D11_TILE_RANGE_FLAG(1i32);
pub const D3D11_TILE_RANGE_REUSE_SINGLE_TILE: D3D11_TILE_RANGE_FLAG = D3D11_TILE_RANGE_FLAG(4i32);
pub const D3D11_TILE_RANGE_SKIP: D3D11_TILE_RANGE_FLAG = D3D11_TILE_RANGE_FLAG(2i32);
pub const D3D11_TRACE_COMPONENT_W: u32 = 8u32;
pub const D3D11_TRACE_COMPONENT_X: u32 = 1u32;
pub const D3D11_TRACE_COMPONENT_Y: u32 = 2u32;
pub const D3D11_TRACE_COMPONENT_Z: u32 = 4u32;
pub const D3D11_TRACE_CONSTANT_BUFFER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(8i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(2i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE_ADJ: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(6i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_POINT: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(1i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(3i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE_ADJ: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(7i32);
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_UNDEFINED: D3D11_TRACE_GS_INPUT_PRIMITIVE = D3D11_TRACE_GS_INPUT_PRIMITIVE(0i32);
pub const D3D11_TRACE_IMMEDIATE32: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(9i32);
pub const D3D11_TRACE_IMMEDIATE64: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(33i32);
pub const D3D11_TRACE_IMMEDIATE_CONSTANT_BUFFER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(3i32);
pub const D3D11_TRACE_INDEXABLE_TEMP_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(5i32);
pub const D3D11_TRACE_INPUT_CONTROL_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(19i32);
pub const D3D11_TRACE_INPUT_COVERAGE_MASK_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(28i32);
pub const D3D11_TRACE_INPUT_CYCLE_COUNTER_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(34i32);
pub const D3D11_TRACE_INPUT_DOMAIN_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(22i32);
pub const D3D11_TRACE_INPUT_FORK_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(17i32);
pub const D3D11_TRACE_INPUT_GS_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(30i32);
pub const D3D11_TRACE_INPUT_JOIN_INSTANCE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(18i32);
pub const D3D11_TRACE_INPUT_PATCH_CONSTANT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(21i32);
pub const D3D11_TRACE_INPUT_PRIMITIVE_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(2i32);
pub const D3D11_TRACE_INPUT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(1i32);
pub const D3D11_TRACE_INPUT_THREAD_GROUP_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(26i32);
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_FLATTENED_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(29i32);
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(27i32);
pub const D3D11_TRACE_INPUT_THREAD_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(25i32);
pub const D3D11_TRACE_INTERFACE_POINTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(35i32);
pub const D3D11_TRACE_MISC_GS_CUT: u32 = 2u32;
pub const D3D11_TRACE_MISC_GS_CUT_STREAM: u32 = 16u32;
pub const D3D11_TRACE_MISC_GS_EMIT: u32 = 1u32;
pub const D3D11_TRACE_MISC_GS_EMIT_STREAM: u32 = 8u32;
pub const D3D11_TRACE_MISC_HALT: u32 = 32u32;
pub const D3D11_TRACE_MISC_MESSAGE: u32 = 64u32;
pub const D3D11_TRACE_MISC_PS_DISCARD: u32 = 4u32;
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_ID_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(16i32);
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(20i32);
pub const D3D11_TRACE_OUTPUT_COVERAGE_MASK: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(13i32);
pub const D3D11_TRACE_OUTPUT_DEPTH_GREATER_EQUAL_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(31i32);
pub const D3D11_TRACE_OUTPUT_DEPTH_LESS_EQUAL_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(32i32);
pub const D3D11_TRACE_OUTPUT_DEPTH_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(7i32);
pub const D3D11_TRACE_OUTPUT_NULL_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(0i32);
pub const D3D11_TRACE_OUTPUT_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(6i32);
pub const D3D11_TRACE_RASTERIZER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(12i32);
pub const D3D11_TRACE_REGISTER_FLAGS_RELATIVE_INDEXING: u32 = 1u32;
pub const D3D11_TRACE_RESOURCE: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(11i32);
pub const D3D11_TRACE_SAMPLER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(10i32);
pub const D3D11_TRACE_STREAM: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(14i32);
pub const D3D11_TRACE_TEMP_REGISTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(4i32);
pub const D3D11_TRACE_THIS_POINTER: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(15i32);
pub const D3D11_TRACE_THREAD_GROUP_SHARED_MEMORY: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(24i32);
pub const D3D11_TRACE_UNORDERED_ACCESS_VIEW: D3D11_TRACE_REGISTER_TYPE = D3D11_TRACE_REGISTER_TYPE(23i32);
pub const D3D11_UAV_DIMENSION_BUFFER: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(1i32);
pub const D3D11_UAV_DIMENSION_TEXTURE1D: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(2i32);
pub const D3D11_UAV_DIMENSION_TEXTURE1DARRAY: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(3i32);
pub const D3D11_UAV_DIMENSION_TEXTURE2D: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(4i32);
pub const D3D11_UAV_DIMENSION_TEXTURE2DARRAY: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(5i32);
pub const D3D11_UAV_DIMENSION_TEXTURE3D: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(8i32);
pub const D3D11_UAV_DIMENSION_UNKNOWN: D3D11_UAV_DIMENSION = D3D11_UAV_DIMENSION(0i32);
pub const D3D11_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub const D3D11_UNMUTE_SEVERITY_INFO: windows_core::PCWSTR = windows_core::w!("Unmute_SEVERITY_INFO");
pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = D3D11_USAGE(0i32);
pub const D3D11_USAGE_DYNAMIC: D3D11_USAGE = D3D11_USAGE(2i32);
pub const D3D11_USAGE_IMMUTABLE: D3D11_USAGE = D3D11_USAGE(1i32);
pub const D3D11_USAGE_STAGING: D3D11_USAGE = D3D11_USAGE(3i32);
pub const D3D11_VDOV_DIMENSION_TEXTURE2D: D3D11_VDOV_DIMENSION = D3D11_VDOV_DIMENSION(1i32);
pub const D3D11_VDOV_DIMENSION_UNKNOWN: D3D11_VDOV_DIMENSION = D3D11_VDOV_DIMENSION(0i32);
pub const D3D11_VERTEX_SHADER: D3D11_SHADER_TYPE = D3D11_SHADER_TYPE(1i32);
pub const D3D11_VIDEO_DECODER_BUFFER_BITSTREAM: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(6i32);
pub const D3D11_VIDEO_DECODER_BUFFER_DEBLOCKING_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(3i32);
pub const D3D11_VIDEO_DECODER_BUFFER_FILM_GRAIN: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(8i32);
pub const D3D11_VIDEO_DECODER_BUFFER_INVERSE_QUANTIZATION_MATRIX: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(4i32);
pub const D3D11_VIDEO_DECODER_BUFFER_MACROBLOCK_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(1i32);
pub const D3D11_VIDEO_DECODER_BUFFER_MOTION_VECTOR: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(7i32);
pub const D3D11_VIDEO_DECODER_BUFFER_PICTURE_PARAMETERS: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(0i32);
pub const D3D11_VIDEO_DECODER_BUFFER_RESIDUAL_DIFFERENCE: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(2i32);
pub const D3D11_VIDEO_DECODER_BUFFER_SLICE_CONTROL: D3D11_VIDEO_DECODER_BUFFER_TYPE = D3D11_VIDEO_DECODER_BUFFER_TYPE(5i32);
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE: D3D11_VIDEO_DECODER_CAPS = D3D11_VIDEO_DECODER_CAPS(1i32);
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC: D3D11_VIDEO_DECODER_CAPS = D3D11_VIDEO_DECODER_CAPS(4i32);
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED: D3D11_VIDEO_DECODER_CAPS = D3D11_VIDEO_DECODER_CAPS(8i32);
pub const D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME: D3D11_VIDEO_DECODER_CAPS = D3D11_VIDEO_DECODER_CAPS(2i32);
pub const D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED: D3D11_VIDEO_DECODER_CAPS = D3D11_VIDEO_DECODER_CAPS(16i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(3i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(2i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(8i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(4i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(2i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_NONE: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(0i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(1i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(2i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(4i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(1i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(1i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(0i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(1i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(2i32);
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(0i32);
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = D3D11_VIDEO_FRAME_FORMAT(2i32);
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = D3D11_VIDEO_FRAME_FORMAT(1i32);
pub const D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3D11_VIDEO_FRAME_FORMAT = D3D11_VIDEO_FRAME_FORMAT(0i32);
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_BACKGROUND: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(1i32);
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_DESTINATION: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(2i32);
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_OPAQUE: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(0i32);
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_SOURCE_STREAM: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(3i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(128i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_COLOR_CORRECTION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DENOISE: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DERINGING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_FLESH_TONE_MAPPING: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_IMAGE_STABILIZATION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(32i32);
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_SUPER_RESOLUTION: D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS = D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(64i32);
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(4i32);
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(2i32);
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(1i32);
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(8i32);
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_LINEAR_SPACE: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_NOMINAL_RANGE: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_RGB_RANGE_CONVERSION: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_xvYCC: D3D11_VIDEO_PROCESSOR_DEVICE_CAPS = D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_FILL: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_PALETTE: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_STREAM: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(128i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_CONSTRICTION: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LEGACY: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LUMA_KEY: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_METADATA_HDR10: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(2048i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_MIRROR: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(512i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_PIXEL_ASPECT_RATIO: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(256i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ROTATION: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(64i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_SHADER_USAGE: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(1024i32);
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_STEREO: D3D11_VIDEO_PROCESSOR_FEATURE_CAPS = D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(32i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(6i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_BRIGHTNESS: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(0i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_ANAMORPHIC_SCALING: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(64i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_BRIGHTNESS: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_CONTRAST: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(32i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_HUE: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_NOISE_REDUCTION: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_SATURATION: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_STEREO_ADJUSTMENT: D3D11_VIDEO_PROCESSOR_FILTER_CAPS = D3D11_VIDEO_PROCESSOR_FILTER_CAPS(128i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_CONTRAST: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(1i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_EDGE_ENHANCEMENT: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(5i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_HUE: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(2i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_NOISE_REDUCTION: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(4i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_SATURATION: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(3i32);
pub const D3D11_VIDEO_PROCESSOR_FILTER_STEREO_ADJUSTMENT: D3D11_VIDEO_PROCESSOR_FILTER = D3D11_VIDEO_PROCESSOR_FILTER(7i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_PALETTE_INTERLACED: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_INTERLACED: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_LUMA_KEY: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_PROCAMP: D3D11_VIDEO_PROCESSOR_FORMAT_CAPS = D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT(1i32);
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT(2i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_22: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_222222222223: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(256i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2224: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2332: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32322: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_55: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(32i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_64: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(64i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_87: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(128i32);
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_OTHER: D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS = D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(-2147483648i32);
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_0_255: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE(2i32);
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_16_235: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE(1i32);
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_UNDEFINED: D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE(0i32);
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_CUSTOM: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE(2i32);
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_HALF: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE(1i32);
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_NORMAL: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE(0i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BLEND: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BOB: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_FRAME_RATE_CONVERSION: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(32i32);
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_INVERSE_TELECINE: D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS = D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_ROTATION_180: D3D11_VIDEO_PROCESSOR_ROTATION = D3D11_VIDEO_PROCESSOR_ROTATION(2i32);
pub const D3D11_VIDEO_PROCESSOR_ROTATION_270: D3D11_VIDEO_PROCESSOR_ROTATION = D3D11_VIDEO_PROCESSOR_ROTATION(3i32);
pub const D3D11_VIDEO_PROCESSOR_ROTATION_90: D3D11_VIDEO_PROCESSOR_ROTATION = D3D11_VIDEO_PROCESSOR_ROTATION(1i32);
pub const D3D11_VIDEO_PROCESSOR_ROTATION_IDENTITY: D3D11_VIDEO_PROCESSOR_ROTATION = D3D11_VIDEO_PROCESSOR_ROTATION(0i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_CHECKERBOARD: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = D3D11_VIDEO_PROCESSOR_STEREO_CAPS(8i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_COLUMN_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = D3D11_VIDEO_PROCESSOR_STEREO_CAPS(4i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_FLIP_MODE: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = D3D11_VIDEO_PROCESSOR_STEREO_CAPS(16i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_MONO_OFFSET: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = D3D11_VIDEO_PROCESSOR_STEREO_CAPS(1i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_ROW_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_CAPS = D3D11_VIDEO_PROCESSOR_STEREO_CAPS(2i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME0: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE(1i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME1: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE(2i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_NONE: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE(0i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_CHECKERBOARD: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(7i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_COLUMN_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(6i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_HORIZONTAL: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(1i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(0i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO_OFFSET: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(4i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_ROW_INTERLEAVED: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(5i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_SEPARATE: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(3i32);
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_VERTICAL: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(2i32);
pub const D3D11_VIDEO_USAGE_OPTIMAL_QUALITY: D3D11_VIDEO_USAGE = D3D11_VIDEO_USAGE(2i32);
pub const D3D11_VIDEO_USAGE_OPTIMAL_SPEED: D3D11_VIDEO_USAGE = D3D11_VIDEO_USAGE(1i32);
pub const D3D11_VIDEO_USAGE_PLAYBACK_NORMAL: D3D11_VIDEO_USAGE = D3D11_VIDEO_USAGE(0i32);
pub const D3D11_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D11_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
pub const D3D11_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
pub const D3D11_VPIV_DIMENSION_TEXTURE2D: D3D11_VPIV_DIMENSION = D3D11_VPIV_DIMENSION(1i32);
pub const D3D11_VPIV_DIMENSION_UNKNOWN: D3D11_VPIV_DIMENSION = D3D11_VPIV_DIMENSION(0i32);
pub const D3D11_VPOV_DIMENSION_TEXTURE2D: D3D11_VPOV_DIMENSION = D3D11_VPOV_DIMENSION(1i32);
pub const D3D11_VPOV_DIMENSION_TEXTURE2DARRAY: D3D11_VPOV_DIMENSION = D3D11_VPOV_DIMENSION(2i32);
pub const D3D11_VPOV_DIMENSION_UNKNOWN: D3D11_VPOV_DIMENSION = D3D11_VPOV_DIMENSION(0i32);
pub const D3D11_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D11_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D11_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3DCSX_DLL: windows_core::PCWSTR = windows_core::w!("d3dcsx_47.dll");
pub const D3DCSX_DLL_A: windows_core::PCSTR = windows_core::s!("d3dcsx_47.dll");
pub const D3DCSX_DLL_W: windows_core::PCWSTR = windows_core::w!("d3dcsx_47.dll");
pub const D3DX11_FFT_CREATE_FLAG_NO_PRECOMPUTE_BUFFERS: D3DX11_FFT_CREATE_FLAG = D3DX11_FFT_CREATE_FLAG(1i32);
pub const D3DX11_FFT_DATA_TYPE_COMPLEX: D3DX11_FFT_DATA_TYPE = D3DX11_FFT_DATA_TYPE(1i32);
pub const D3DX11_FFT_DATA_TYPE_REAL: D3DX11_FFT_DATA_TYPE = D3DX11_FFT_DATA_TYPE(0i32);
pub const D3DX11_FFT_DIM_MASK_1D: D3DX11_FFT_DIM_MASK = D3DX11_FFT_DIM_MASK(1i32);
pub const D3DX11_FFT_DIM_MASK_2D: D3DX11_FFT_DIM_MASK = D3DX11_FFT_DIM_MASK(3i32);
pub const D3DX11_FFT_DIM_MASK_3D: D3DX11_FFT_DIM_MASK = D3DX11_FFT_DIM_MASK(7i32);
pub const D3DX11_FFT_MAX_DIMENSIONS: u32 = 32u32;
pub const D3DX11_FFT_MAX_PRECOMPUTE_BUFFERS: u32 = 4u32;
pub const D3DX11_FFT_MAX_TEMP_BUFFERS: u32 = 4u32;
pub const D3DX11_SCAN_DATA_TYPE_FLOAT: D3DX11_SCAN_DATA_TYPE = D3DX11_SCAN_DATA_TYPE(1i32);
pub const D3DX11_SCAN_DATA_TYPE_INT: D3DX11_SCAN_DATA_TYPE = D3DX11_SCAN_DATA_TYPE(2i32);
pub const D3DX11_SCAN_DATA_TYPE_UINT: D3DX11_SCAN_DATA_TYPE = D3DX11_SCAN_DATA_TYPE(3i32);
pub const D3DX11_SCAN_DIRECTION_BACKWARD: D3DX11_SCAN_DIRECTION = D3DX11_SCAN_DIRECTION(2i32);
pub const D3DX11_SCAN_DIRECTION_FORWARD: D3DX11_SCAN_DIRECTION = D3DX11_SCAN_DIRECTION(1i32);
pub const D3DX11_SCAN_OPCODE_ADD: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(1i32);
pub const D3DX11_SCAN_OPCODE_AND: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(5i32);
pub const D3DX11_SCAN_OPCODE_MAX: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(3i32);
pub const D3DX11_SCAN_OPCODE_MIN: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(2i32);
pub const D3DX11_SCAN_OPCODE_MUL: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(4i32);
pub const D3DX11_SCAN_OPCODE_OR: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(6i32);
pub const D3DX11_SCAN_OPCODE_XOR: D3DX11_SCAN_OPCODE = D3DX11_SCAN_OPCODE(7i32);
pub const D3D_RETURN_PARAMETER_INDEX: i32 = -1i32;
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u32 = 32u32;
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u32 = 64u32;
pub const D3D_SHADER_REQUIRES_64_UAVS: u32 = 8u32;
pub const D3D_SHADER_REQUIRES_DOUBLES: u32 = 1u32;
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u32 = 2u32;
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: u32 = 128u32;
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u32 = 16u32;
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: u32 = 256u32;
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u32 = 4u32;
pub const DXGI_DEBUG_D3D11: windows_core::GUID = windows_core::GUID::from_u128(0x4b99317b_ac39_4aa6_bb0b_baa04784798f);
pub const _FACD3D11: u32 = 2172u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_ASYNC_GETDATA_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_ASYNC_GETDATA_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_ASYNC_GETDATA_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_ASYNC_GETDATA_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_AUTHENTICATED_CHANNEL_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CHANNEL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_AUTHENTICATED_CHANNEL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_AUTHENTICATED_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BIND_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_BIND_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BIND_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BIND_FLAG").field(&self.0).finish()
    }
}
impl D3D11_BIND_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_BIND_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_BIND_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_BIND_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_BIND_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_BIND_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BLEND(pub i32);
impl windows_core::TypeKind for D3D11_BLEND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BLEND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BLEND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BLEND_OP(pub i32);
impl windows_core::TypeKind for D3D11_BLEND_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BLEND_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BLEND_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BUFFEREX_SRV_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_BUFFEREX_SRV_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BUFFEREX_SRV_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BUFFEREX_SRV_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BUFFER_UAV_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_BUFFER_UAV_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BUFFER_UAV_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BUFFER_UAV_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_BUS_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_BUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_BUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_BUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CLEAR_FLAG(pub u32);
impl windows_core::TypeKind for D3D11_CLEAR_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CLEAR_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CLEAR_FLAG").field(&self.0).finish()
    }
}
impl D3D11_CLEAR_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_CLEAR_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_CLEAR_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_COLOR_WRITE_ENABLE(pub i32);
impl windows_core::TypeKind for D3D11_COLOR_WRITE_ENABLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_COMPARISON_FUNC(pub i32);
impl windows_core::TypeKind for D3D11_COMPARISON_FUNC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_COMPARISON_FUNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_COMPARISON_FUNC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CONSERVATIVE_RASTERIZATION_MODE(pub i32);
impl windows_core::TypeKind for D3D11_CONSERVATIVE_RASTERIZATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CONSERVATIVE_RASTERIZATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CONSERVATIVE_RASTERIZATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CONSERVATIVE_RASTERIZATION_TIER(pub i32);
impl windows_core::TypeKind for D3D11_CONSERVATIVE_RASTERIZATION_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CONSERVATIVE_RASTERIZATION_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CONSERVATIVE_RASTERIZATION_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CONTENT_PROTECTION_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_CONTENT_PROTECTION_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CONTENT_PROTECTION_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CONTENT_PROTECTION_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CONTEXT_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_CONTEXT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CONTEXT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_COPY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D11_COPY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_COPY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_COPY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_COUNTER(pub i32);
impl windows_core::TypeKind for D3D11_COUNTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_COUNTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_COUNTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_COUNTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_COUNTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_COUNTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_COUNTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CPU_ACCESS_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_CPU_ACCESS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CPU_ACCESS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CPU_ACCESS_FLAG").field(&self.0).finish()
    }
}
impl D3D11_CPU_ACCESS_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_CPU_ACCESS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_CPU_ACCESS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CREATE_DEVICE_FLAG(pub u32);
impl windows_core::TypeKind for D3D11_CREATE_DEVICE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CREATE_DEVICE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CREATE_DEVICE_FLAG").field(&self.0).finish()
    }
}
impl D3D11_CREATE_DEVICE_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_CREATE_DEVICE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_CREATE_DEVICE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS").field(&self.0).finish()
    }
}
impl D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CRYPTO_SESSION_STATUS(pub i32);
impl windows_core::TypeKind for D3D11_CRYPTO_SESSION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CRYPTO_SESSION_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CRYPTO_SESSION_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_CULL_MODE(pub i32);
impl windows_core::TypeKind for D3D11_CULL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_CULL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_CULL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_DEPTH_WRITE_MASK(pub i32);
impl windows_core::TypeKind for D3D11_DEPTH_WRITE_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_DEVICE_CONTEXT_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_DEVICE_CONTEXT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_DEVICE_CONTEXT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_DEVICE_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_DSV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_DSV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_DSV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_DSV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_DSV_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_DSV_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_DSV_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_DSV_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FEATURE(pub i32);
impl windows_core::TypeKind for D3D11_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FEATURE_VIDEO(pub i32);
impl windows_core::TypeKind for D3D11_FEATURE_VIDEO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FEATURE_VIDEO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FEATURE_VIDEO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FENCE_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_FENCE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FENCE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FENCE_FLAG").field(&self.0).finish()
    }
}
impl D3D11_FENCE_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_FENCE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_FENCE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_FENCE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_FENCE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_FENCE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FILL_MODE(pub i32);
impl windows_core::TypeKind for D3D11_FILL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FILL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FILTER(pub i32);
impl windows_core::TypeKind for D3D11_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FILTER_REDUCTION_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_FILTER_REDUCTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FILTER_REDUCTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FILTER_REDUCTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FILTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_FILTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FILTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FILTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FORMAT_SUPPORT(pub i32);
impl windows_core::TypeKind for D3D11_FORMAT_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_FORMAT_SUPPORT2(pub i32);
impl windows_core::TypeKind for D3D11_FORMAT_SUPPORT2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_FORMAT_SUPPORT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_FORMAT_SUPPORT2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_INPUT_CLASSIFICATION(pub i32);
impl windows_core::TypeKind for D3D11_INPUT_CLASSIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_LOGIC_OP(pub i32);
impl windows_core::TypeKind for D3D11_LOGIC_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_LOGIC_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_LOGIC_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_MAP(pub i32);
impl windows_core::TypeKind for D3D11_MAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_MAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_MAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_MAP_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_MAP_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_MAP_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_MAP_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_MESSAGE_CATEGORY(pub i32);
impl windows_core::TypeKind for D3D11_MESSAGE_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_MESSAGE_ID(pub i32);
impl windows_core::TypeKind for D3D11_MESSAGE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_MESSAGE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_MESSAGE_SEVERITY(pub i32);
impl windows_core::TypeKind for D3D11_MESSAGE_SEVERITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_QUERY(pub i32);
impl windows_core::TypeKind for D3D11_QUERY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_QUERY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_QUERY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_QUERY_MISC_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_QUERY_MISC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_QUERY_MISC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_QUERY_MISC_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_RAISE_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_RAISE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_RAISE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_RAISE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_RESOURCE_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_RESOURCE_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_RESOURCE_MISC_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_RESOURCE_MISC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_RESOURCE_MISC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_RESOURCE_MISC_FLAG").field(&self.0).finish()
    }
}
impl D3D11_RESOURCE_MISC_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_RESOURCE_MISC_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_RESOURCE_MISC_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_RLDO_FLAGS(pub i32);
impl windows_core::TypeKind for D3D11_RLDO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_RLDO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_RLDO_FLAGS").field(&self.0).finish()
    }
}
impl D3D11_RLDO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_RLDO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_RLDO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_RLDO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_RLDO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_RLDO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_RTV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_RTV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_RTV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_RTV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_CACHE_SUPPORT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_CACHE_SUPPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_CACHE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_CACHE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_MIN_PRECISION_SUPPORT(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_MIN_PRECISION_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_MIN_PRECISION_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_MIN_PRECISION_SUPPORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_TRACKING_OPTIONS(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_TRACKING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_TRACKING_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TRACKING_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_TRACKING_RESOURCE_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_TRACKING_RESOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_TRACKING_RESOURCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TRACKING_RESOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHADER_VERSION_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_SHADER_VERSION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHADER_VERSION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_VERSION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_SHARED_RESOURCE_TIER(pub i32);
impl windows_core::TypeKind for D3D11_SHARED_RESOURCE_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_SHARED_RESOURCE_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_SHARED_RESOURCE_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS(pub i32);
impl windows_core::TypeKind for D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_STENCIL_OP(pub i32);
impl windows_core::TypeKind for D3D11_STENCIL_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_STENCIL_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_STENCIL_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TEXTURECUBE_FACE(pub i32);
impl windows_core::TypeKind for D3D11_TEXTURECUBE_FACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TEXTURECUBE_FACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURECUBE_FACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TEXTURE_ADDRESS_MODE(pub i32);
impl windows_core::TypeKind for D3D11_TEXTURE_ADDRESS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TEXTURE_LAYOUT(pub i32);
impl windows_core::TypeKind for D3D11_TEXTURE_LAYOUT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TEXTURE_LAYOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURE_LAYOUT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TILED_RESOURCES_TIER(pub i32);
impl windows_core::TypeKind for D3D11_TILED_RESOURCES_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TILED_RESOURCES_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TILED_RESOURCES_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TILE_COPY_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_TILE_COPY_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TILE_COPY_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TILE_COPY_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TILE_MAPPING_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_TILE_MAPPING_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TILE_MAPPING_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TILE_MAPPING_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TILE_RANGE_FLAG(pub i32);
impl windows_core::TypeKind for D3D11_TILE_RANGE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TILE_RANGE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TILE_RANGE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TRACE_GS_INPUT_PRIMITIVE(pub i32);
impl windows_core::TypeKind for D3D11_TRACE_GS_INPUT_PRIMITIVE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TRACE_GS_INPUT_PRIMITIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TRACE_GS_INPUT_PRIMITIVE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_TRACE_REGISTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_TRACE_REGISTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_TRACE_REGISTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_TRACE_REGISTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_UAV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_UAV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_UAV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_UAV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_USAGE(pub i32);
impl windows_core::TypeKind for D3D11_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VDOV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_VDOV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VDOV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VDOV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_DECODER_BUFFER_TYPE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_BUFFER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_DECODER_BUFFER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_DECODER_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_DECODER_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
impl D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_FRAME_FORMAT(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_FRAME_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_FRAME_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_FRAME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_DEVICE_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_DEVICE_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_DEVICE_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FEATURE_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FEATURE_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FEATURE_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FILTER_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FILTER_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FILTER_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FORMAT_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FORMAT_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FORMAT_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_RATE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_OUTPUT_RATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_OUTPUT_RATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_OUTPUT_RATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_ROTATION(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_ROTATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ROTATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ROTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_STEREO_CAPS(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_STEREO_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_STEREO_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VIDEO_USAGE(pub i32);
impl windows_core::TypeKind for D3D11_VIDEO_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VIDEO_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VPIV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_VPIV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VPIV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VPIV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D11_VPOV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D11_VPOV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D11_VPOV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D11_VPOV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_FFT_CREATE_FLAG(pub i32);
impl windows_core::TypeKind for D3DX11_FFT_CREATE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_FFT_CREATE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_CREATE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_FFT_DATA_TYPE(pub i32);
impl windows_core::TypeKind for D3DX11_FFT_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_FFT_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_FFT_DIM_MASK(pub i32);
impl windows_core::TypeKind for D3DX11_FFT_DIM_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_FFT_DIM_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_DIM_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_SCAN_DATA_TYPE(pub i32);
impl windows_core::TypeKind for D3DX11_SCAN_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_SCAN_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_SCAN_DIRECTION(pub i32);
impl windows_core::TypeKind for D3DX11_SCAN_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_SCAN_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3DX11_SCAN_OPCODE(pub i32);
impl windows_core::TypeKind for D3DX11_SCAN_OPCODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3DX11_SCAN_OPCODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_OPCODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
impl windows_core::TypeKind for D3D11_AES_CTR_IV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AES_CTR_IV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_core::GUID,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub Protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub ProcessType: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub AllowAccess: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    pub Flags: D3D11_AUTHENTICATED_PROTECTION_FLAGS_0,
    pub Value: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidCount: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_core::GUID,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub BusType: D3D11_BUS_TYPE,
    pub AccessibleInContiguousBlocks: super::super::Foundation::BOOL,
    pub AccessibleInNonContiguousBlocks: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuid: windows_core::GUID,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    pub QueryType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    pub omac: D3D11_OMAC,
    pub QueryType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDCount: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProtectionFlags: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub RestrictedSharedResourceProcessCount: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub ProcessIndex: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub UnrestrictedProtectedSharedResourceCount: u32,
}
impl windows_core::TypeKind for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
impl windows_core::TypeKind for D3D11_BLEND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
impl windows_core::TypeKind for D3D11_BLEND_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl windows_core::TypeKind for D3D11_BOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for D3D11_BUFFEREX_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFEREX_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_RTV {
    pub Anonymous1: D3D11_BUFFER_RTV_0,
    pub Anonymous2: D3D11_BUFFER_RTV_1,
}
impl windows_core::TypeKind for D3D11_BUFFER_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_RTV_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_RTV_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_SRV {
    pub Anonymous1: D3D11_BUFFER_SRV_0,
    pub Anonymous2: D3D11_BUFFER_SRV_1,
}
impl windows_core::TypeKind for D3D11_BUFFER_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_SRV_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_SRV_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BUFFER_UAV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for D3D11_BUFFER_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_BUFFER_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub InstanceId: u32,
    pub InstanceIndex: u32,
    pub TypeId: u32,
    pub ConstantBuffer: u32,
    pub BaseConstantBufferOffset: u32,
    pub BaseTexture: u32,
    pub BaseSampler: u32,
    pub Created: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_CLASS_INSTANCE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_CLASS_INSTANCE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_COMPUTE_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub ThreadIDInGroup: [u32; 3],
    pub ThreadGroupID: [u32; 3],
}
impl windows_core::TypeKind for D3D11_COMPUTE_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
impl windows_core::TypeKind for D3D11_COUNTER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_COUNTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D11_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
impl windows_core::TypeKind for D3D11_COUNTER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_COUNTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
impl windows_core::TypeKind for D3D11_DEPTH_STENCILOP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
impl windows_core::TypeKind for D3D11_DEPTH_STENCIL_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_DEPTH_STENCIL_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_DOMAIN_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl windows_core::TypeKind for D3D11_DOMAIN_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_DOMAIN_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
impl windows_core::TypeKind for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
impl windows_core::TypeKind for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_ENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
impl windows_core::TypeKind for D3D11_ENCRYPTED_BLOCK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_ENCRYPTED_BLOCK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    pub TileBasedDeferredRenderer: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
    pub OutputMergerLogicOp: super::super::Foundation::BOOL,
    pub UAVOnlyRenderingForcedSampleCount: super::super::Foundation::BOOL,
    pub DiscardAPIsSeenByDriver: super::super::Foundation::BOOL,
    pub FlagsForUpdateAndCopySeenByDriver: super::super::Foundation::BOOL,
    pub ClearView: super::super::Foundation::BOOL,
    pub CopyWithOverlap: super::super::Foundation::BOOL,
    pub ConstantBufferPartialUpdate: super::super::Foundation::BOOL,
    pub ConstantBufferOffsetting: super::super::Foundation::BOOL,
    pub MapNoOverwriteOnDynamicConstantBuffer: super::super::Foundation::BOOL,
    pub MapNoOverwriteOnDynamicBufferSRV: super::super::Foundation::BOOL,
    pub MultisampleRTVWithForcedSampleCountOne: super::super::Foundation::BOOL,
    pub SAD4ShaderInstructions: super::super::Foundation::BOOL,
    pub ExtendedDoublesShaderInstructions: super::super::Foundation::BOOL,
    pub ExtendedResourceSharing: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MinMaxFiltering: super::super::Foundation::BOOL,
    pub ClearViewAlsoSupportsDepthOnlyFormats: super::super::Foundation::BOOL,
    pub MapOnDefaultBuffers: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    pub PSSpecifiedStencilRefSupported: super::super::Foundation::BOOL,
    pub TypedUAVLoadAdditionalFormats: super::super::Foundation::BOOL,
    pub ROVsSupported: super::super::Foundation::BOOL,
    pub ConservativeRasterizationTier: D3D11_CONSERVATIVE_RASTERIZATION_TIER,
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MapOnDefaultTextures: super::super::Foundation::BOOL,
    pub StandardSwizzle: super::super::Foundation::BOOL,
    pub UnifiedMemoryArchitecture: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizer: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    pub ExtendedNV12SharedTextureSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
    pub FullNonPow2TextureSupport: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    pub FullNonPow2TextureSupported: super::super::Foundation::BOOL,
    pub DepthAsTextureWithLessEqualComparisonFilterSupported: super::super::Foundation::BOOL,
    pub SimpleInstancingSupported: super::super::Foundation::BOOL,
    pub TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    pub SupportsDepthAsTextureWithLessEqualComparisonFilter: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    pub SimpleInstancingSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: super::super::Foundation::BOOL,
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_DISPLAYABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_DOUBLES {
    pub DoublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_DOUBLES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_DOUBLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    pub InFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub OutFormatSupport: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    pub InFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub OutFormatSupport2: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
    pub Profile: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: u32,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_SHADER_CACHE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    pub PixelShaderMinPrecision: u32,
    pub AllOtherShaderStagesMinPrecision: u32,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_THREADING {
    pub DriverConcurrentCreates: super::super::Foundation::BOOL,
    pub DriverCommandLists: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_FEATURE_DATA_THREADING {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_FEATURE_DATA_THREADING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    pub DecoderDesc: D3D11_VIDEO_DECODER_DESC,
    pub Components: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
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
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: windows_core::PCSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: super::super::Foundation::BOOL,
    pub Has10Level9VertexShader: super::super::Foundation::BOOL,
    pub Has10Level9PixelShader: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_FUNCTION_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl windows_core::TypeKind for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_HULL_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl windows_core::TypeKind for D3D11_HULL_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_HULL_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_INFO_QUEUE_FILTER {
    pub AllowList: D3D11_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D11_INFO_QUEUE_FILTER_DESC,
}
impl windows_core::TypeKind for D3D11_INFO_QUEUE_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D11_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D11_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D11_MESSAGE_ID,
}
impl windows_core::TypeKind for D3D11_INFO_QUEUE_FILTER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_INPUT_ELEMENT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    pub HWProtectionFunctionID: u32,
    pub pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pub pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    pub Status: windows_core::HRESULT,
}
impl windows_core::TypeKind for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    pub PrivateDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub pbInput: [u8; 4],
}
impl windows_core::TypeKind for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    pub PrivateDataSize: u32,
    pub MaxHWProtectionDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub TransportTime: u64,
    pub ExecutionTime: u64,
    pub pbOutput: [u8; 4],
}
impl windows_core::TypeKind for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl windows_core::TypeKind for D3D11_LIBRARY_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl windows_core::TypeKind for D3D11_MAPPED_SUBRESOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_MESSAGE {
    pub Category: D3D11_MESSAGE_CATEGORY,
    pub Severity: D3D11_MESSAGE_SEVERITY,
    pub ID: D3D11_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl windows_core::TypeKind for D3D11_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_OMAC {
    pub Omac: [u8; 16],
}
impl windows_core::TypeKind for D3D11_OMAC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_PACKED_MIP_DESC {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
impl windows_core::TypeKind for D3D11_PACKED_MIP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_PACKED_MIP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_PARAMETER_DESC {
    pub Name: windows_core::PCSTR,
    pub SemanticName: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::Direct3D::D3D_INTERPOLATION_MODE,
    pub Flags: super::Direct3D::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_PIXEL_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub X: i32,
    pub Y: i32,
    pub SampleMask: u64,
}
impl windows_core::TypeKind for D3D11_PIXEL_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_PIXEL_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
impl windows_core::TypeKind for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl windows_core::TypeKind for D3D11_QUERY_DATA_SO_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_QUERY_DESC {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
}
impl windows_core::TypeKind for D3D11_QUERY_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_QUERY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_QUERY_DESC1 {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
    pub ContextType: D3D11_CONTEXT_TYPE,
}
impl windows_core::TypeKind for D3D11_QUERY_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_QUERY_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D11_RASTERIZER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_RASTERIZER_DESC1 {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
}
impl windows_core::TypeKind for D3D11_RASTERIZER_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_RASTERIZER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_RASTERIZER_DESC2 {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D11_CONSERVATIVE_RASTERIZATION_MODE,
}
impl windows_core::TypeKind for D3D11_RASTERIZER_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_RASTERIZER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
impl windows_core::TypeKind for D3D11_RENDER_TARGET_BLEND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub LogicOpEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub LogicOp: D3D11_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
impl windows_core::TypeKind for D3D11_RENDER_TARGET_BLEND_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_RENDER_TARGET_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_RENDER_TARGET_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC1_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_RENDER_TARGET_VIEW_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV1,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl windows_core::TypeKind for D3D11_SAMPLER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_SHADER_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SHADER_DESC {
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
    pub InputPrimitive: super::Direct3D::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::Direct3D::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::Direct3D::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::Direct3D::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_SHADER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
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
impl windows_core::TypeKind for D3D11_SHADER_INPUT_BIND_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D3D11_SHADER_RESOURCE_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV1,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_TRACE_DESC {
    pub Type: D3D11_SHADER_TYPE,
    pub Flags: u32,
    pub Anonymous: D3D11_SHADER_TRACE_DESC_0,
}
impl windows_core::TypeKind for D3D11_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_TRACE_DESC_0 {
    pub VertexShaderTraceDesc: D3D11_VERTEX_SHADER_TRACE_DESC,
    pub HullShaderTraceDesc: D3D11_HULL_SHADER_TRACE_DESC,
    pub DomainShaderTraceDesc: D3D11_DOMAIN_SHADER_TRACE_DESC,
    pub GeometryShaderTraceDesc: D3D11_GEOMETRY_SHADER_TRACE_DESC,
    pub PixelShaderTraceDesc: D3D11_PIXEL_SHADER_TRACE_DESC,
    pub ComputeShaderTraceDesc: D3D11_COMPUTE_SHADER_TRACE_DESC,
}
impl windows_core::TypeKind for D3D11_SHADER_TRACE_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SHADER_TRACE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: windows_core::PCSTR,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_SHADER_TYPE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SHADER_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
impl windows_core::TypeKind for D3D11_SHADER_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::Direct3D::D3D_MIN_PRECISION,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D11_SIGNATURE_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D11_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl windows_core::TypeKind for D3D11_SO_DECLARATION_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl windows_core::TypeKind for D3D11_SUBRESOURCE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
impl windows_core::TypeKind for D3D11_SUBRESOURCE_TILING {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_SUBRESOURCE_TILING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_ARRAY_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX1D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX1D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D11_TEX2DMS_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_RTV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_RTV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_SRV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_SRV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_UAV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_UAV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_VPOV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_ARRAY_VPOV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_ARRAY_VPOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_RTV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_RTV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_RTV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_SRV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_SRV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_UAV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_UAV1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_UAV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_VDOV {
    pub ArraySlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_VDOV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_VDOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_VPIV {
    pub MipSlice: u32,
    pub ArraySlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_VPIV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_VPIV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX2D_VPOV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D11_TEX2D_VPOV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX2D_VPOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl windows_core::TypeKind for D3D11_TEX3D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX3D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D11_TEX3D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX3D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl windows_core::TypeKind for D3D11_TEX3D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEX3D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
impl windows_core::TypeKind for D3D11_TEXCUBE_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl windows_core::TypeKind for D3D11_TEXCUBE_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_TEXTURE1D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_TEXTURE1D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_TEXTURE2D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_TEXTURE2D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_TEXTURE2D_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_TEXTURE2D_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_TEXTURE3D_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_TEXTURE3D_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_TEXTURE3D_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_TEXTURE3D_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
impl windows_core::TypeKind for D3D11_TILED_RESOURCE_COORDINATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TILED_RESOURCE_COORDINATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub bUseBox: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
impl windows_core::TypeKind for D3D11_TILE_REGION_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TILE_REGION_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
impl windows_core::TypeKind for D3D11_TILE_SHAPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TILE_SHAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_TRACE_REGISTER {
    pub RegType: D3D11_TRACE_REGISTER_TYPE,
    pub Anonymous: D3D11_TRACE_REGISTER_0,
    pub OperandIndex: u8,
    pub Flags: u8,
}
impl windows_core::TypeKind for D3D11_TRACE_REGISTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TRACE_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_TRACE_REGISTER_0 {
    pub Index1D: u16,
    pub Index2D: [u16; 2],
}
impl windows_core::TypeKind for D3D11_TRACE_REGISTER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TRACE_REGISTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_TRACE_STATS {
    pub TraceDesc: D3D11_SHADER_TRACE_DESC,
    pub NumInvocationsInStamp: u8,
    pub TargetStampIndex: u8,
    pub NumTraceSteps: u32,
    pub InputMask: [u8; 32],
    pub OutputMask: [u8; 32],
    pub NumTemps: u16,
    pub MaxIndexableTempIndex: u16,
    pub IndexableTempSize: [u16; 4096],
    pub ImmediateConstantBufferSize: u16,
    pub PixelPosition: [u32; 8],
    pub PixelCoverageMask: [u64; 4],
    pub PixelDiscardedMask: [u64; 4],
    pub PixelCoverageMaskAfterShader: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMask: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepth: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepthStencil: [u64; 4],
    pub PSOutputsDepth: super::super::Foundation::BOOL,
    pub PSOutputsMask: super::super::Foundation::BOOL,
    pub GSInputPrimitive: D3D11_TRACE_GS_INPUT_PRIMITIVE,
    pub GSInputsPrimitiveID: super::super::Foundation::BOOL,
    pub HSOutputPatchConstantMask: [u8; 32],
    pub DSInputPatchConstantMask: [u8; 32],
}
impl windows_core::TypeKind for D3D11_TRACE_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TRACE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TRACE_STEP {
    pub ID: u32,
    pub InstructionActive: super::super::Foundation::BOOL,
    pub NumRegistersWritten: u8,
    pub NumRegistersRead: u8,
    pub MiscOperations: u16,
    pub OpcodeType: u32,
    pub CurrentGlobalCycle: u64,
}
impl windows_core::TypeKind for D3D11_TRACE_STEP {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TRACE_STEP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_TRACE_VALUE {
    pub Bits: [u32; 4],
    pub ValidMask: u8,
}
impl windows_core::TypeKind for D3D11_TRACE_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_TRACE_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC1_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV1,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VERTEX_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl windows_core::TypeKind for D3D11_VERTEX_SHADER_TRACE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VERTEX_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_COLOR {
    pub Anonymous: D3D11_VIDEO_COLOR_0,
}
impl windows_core::TypeKind for D3D11_VIDEO_COLOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_COLOR_0 {
    pub YCbCr: D3D11_VIDEO_COLOR_YCbCrA,
    pub RGBA: D3D11_VIDEO_COLOR_RGBA,
}
impl windows_core::TypeKind for D3D11_VIDEO_COLOR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
impl windows_core::TypeKind for D3D11_VIDEO_COLOR_RGBA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_COLOR_RGBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
impl windows_core::TypeKind for D3D11_VIDEO_COLOR_YCbCrA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_COLOR_YCbCrA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    pub Caps: u32,
    pub KeyExchangeTypeCount: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
impl windows_core::TypeKind for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pub pCryptoSession: core::mem::ManuallyDrop<Option<ID3D11CryptoSession>>,
    pub BlobSize: u32,
    pub pBlob: *mut core::ffi::c_void,
    pub pKeyInfoId: *mut windows_core::GUID,
    pub PrivateDataSize: u32,
    pub pPrivateData: *mut core::ffi::c_void,
}
impl Clone for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub PartialEncryption: super::super::Foundation::BOOL,
    pub EncryptedBlockInfo: D3D11_ENCRYPTED_BLOCK_INFO,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
    pub cBlocksStripeEncrypted: u32,
    pub cBlocksStripeClear: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_CONFIG {
    pub guidConfigBitstreamEncryption: windows_core::GUID,
    pub guidConfigMBcontrolEncryption: windows_core::GUID,
    pub guidConfigResidDiffEncryption: windows_core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: windows_core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_VIDEO_DECODER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_EXTENSION {
    pub Function: u32,
    pub pPrivateInputData: *mut core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
    pub ResourceCount: u32,
    pub ppResourceList: *mut Option<ID3D11Resource>,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_EXTENSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    pub DecodeProfile: windows_core::GUID,
    pub ViewDimension: D3D11_VDOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VDOV,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    pub ClearSize: u32,
    pub EncryptedSize: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CAPS {
    pub DeviceCaps: u32,
    pub FeatureCaps: u32,
    pub FilterCaps: u32,
    pub InputFormatCaps: u32,
    pub AutoStreamCaps: u32,
    pub StereoCaps: u32,
    pub RateConversionCapsCount: u32,
    pub MaxInputStreams: u32,
    pub MaxStreamStates: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
    pub InputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
    pub Usage: D3D11_VIDEO_USAGE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: super::super::Foundation::BOOL,
    pub InputFramesOrFields: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    pub FourCC: u32,
    pub ViewDimension: D3D11_VPIV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPIV,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    pub ViewDimension: D3D11_VPOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPOV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_VPOV,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ProcessorCaps: u32,
    pub ITelecineCaps: u32,
    pub CustomRateCount: u32,
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM {
    pub Enable: super::super::Foundation::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurface: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub ppPastSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurfaceRight: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
}
impl Clone for D3D11_VIDEO_PROCESSOR_STREAM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_STREAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIDEO_PROCESSOR_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    pub Enable: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_VIDEO_SAMPLE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ColorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D11_VIDEO_SAMPLE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D11_VIDEO_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl windows_core::TypeKind for D3D11_VIEWPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D11_VIEWPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3DX11_FFT_BUFFER_INFO {
    pub NumTempBufferSizes: u32,
    pub TempBufferFloatSizes: [u32; 4],
    pub NumPrecomputeBufferSizes: u32,
    pub PrecomputeBufferFloatSizes: [u32; 4],
}
impl windows_core::TypeKind for D3DX11_FFT_BUFFER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3DX11_FFT_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3DX11_FFT_DESC {
    pub NumDimensions: u32,
    pub ElementLengths: [u32; 32],
    pub DimensionMask: u32,
    pub Type: D3DX11_FFT_DATA_TYPE,
}
impl windows_core::TypeKind for D3DX11_FFT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3DX11_FFT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
pub type PFN_D3D11_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: Option<super::Dxgi::IDXGIAdapter>, param1: super::Direct3D::D3D_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *mut Option<ID3D11Device>, param8: *mut super::Direct3D::D3D_FEATURE_LEVEL, param9: *mut Option<ID3D11DeviceContext>) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub type PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN = Option<unsafe extern "system" fn(param0: Option<super::Dxgi::IDXGIAdapter>, param1: super::Direct3D::D3D_DRIVER_TYPE, param2: super::super::Foundation::HMODULE, param3: u32, param4: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, param8: *mut Option<super::Dxgi::IDXGISwapChain>, param9: *mut Option<ID3D11Device>, param10: *mut super::Direct3D::D3D_FEATURE_LEVEL, param11: *mut Option<ID3D11DeviceContext>) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
