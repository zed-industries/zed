#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12CreateDevice<P0, T>(padapter: P0, minimumfeaturelevel: super::Direct3D::D3D_FEATURE_LEVEL, result__: *mut Option<T>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("d3d12.dll" "system" fn D3D12CreateDevice(padapter : * mut core::ffi::c_void, minimumfeaturelevel : super::Direct3D:: D3D_FEATURE_LEVEL, riid : *const windows_core::GUID, ppdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12CreateDevice(padapter.param().abi(), minimumfeaturelevel, &T::IID, result__ as *mut _ as *mut _).ok()
}
#[inline]
pub unsafe fn D3D12CreateRootSignatureDeserializer(psrcdata: *const core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const windows_core::GUID, pprootsignaturedeserializer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("d3d12.dll" "system" fn D3D12CreateRootSignatureDeserializer(psrcdata : *const core::ffi::c_void, srcdatasizeinbytes : usize, prootsignaturedeserializerinterface : *const windows_core::GUID, pprootsignaturedeserializer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12CreateRootSignatureDeserializer(psrcdata, srcdatasizeinbytes, prootsignaturedeserializerinterface, pprootsignaturedeserializer).ok()
}
#[inline]
pub unsafe fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata: *const core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const windows_core::GUID, pprootsignaturedeserializer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("d3d12.dll" "system" fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata : *const core::ffi::c_void, srcdatasizeinbytes : usize, prootsignaturedeserializerinterface : *const windows_core::GUID, pprootsignaturedeserializer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12CreateVersionedRootSignatureDeserializer(psrcdata, srcdatasizeinbytes, prootsignaturedeserializerinterface, pprootsignaturedeserializer).ok()
}
#[inline]
pub unsafe fn D3D12EnableExperimentalFeatures(numfeatures: u32, piids: *const windows_core::GUID, pconfigurationstructs: Option<*const core::ffi::c_void>, pconfigurationstructsizes: Option<*const u32>) -> windows_core::Result<()> {
    windows_targets::link!("d3d12.dll" "system" fn D3D12EnableExperimentalFeatures(numfeatures : u32, piids : *const windows_core::GUID, pconfigurationstructs : *const core::ffi::c_void, pconfigurationstructsizes : *const u32) -> windows_core::HRESULT);
    D3D12EnableExperimentalFeatures(numfeatures, piids, core::mem::transmute(pconfigurationstructs.unwrap_or(std::ptr::null())), core::mem::transmute(pconfigurationstructsizes.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn D3D12GetDebugInterface<T>(result__: *mut Option<T>) -> windows_core::Result<()>
where
    T: windows_core::Interface,
{
    windows_targets::link!("d3d12.dll" "system" fn D3D12GetDebugInterface(riid : *const windows_core::GUID, ppvdebug : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12GetDebugInterface(&T::IID, result__ as *mut _ as *mut _).ok()
}
#[inline]
pub unsafe fn D3D12GetInterface<T>(rclsid: *const windows_core::GUID, result__: *mut Option<T>) -> windows_core::Result<()>
where
    T: windows_core::Interface,
{
    windows_targets::link!("d3d12.dll" "system" fn D3D12GetInterface(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppvdebug : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12GetInterface(rclsid, &T::IID, result__ as *mut _ as *mut _).ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12SerializeRootSignature(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorblob: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()> {
    windows_targets::link!("d3d12.dll" "system" fn D3D12SerializeRootSignature(prootsignature : *const D3D12_ROOT_SIGNATURE_DESC, version : D3D_ROOT_SIGNATURE_VERSION, ppblob : *mut * mut core::ffi::c_void, pperrorblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12SerializeRootSignature(prootsignature, version, core::mem::transmute(ppblob), core::mem::transmute(pperrorblob.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[inline]
pub unsafe fn D3D12SerializeVersionedRootSignature(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorblob: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()> {
    windows_targets::link!("d3d12.dll" "system" fn D3D12SerializeVersionedRootSignature(prootsignature : *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob : *mut * mut core::ffi::c_void, pperrorblob : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    D3D12SerializeVersionedRootSignature(prootsignature, core::mem::transmute(ppblob), core::mem::transmute(pperrorblob.unwrap_or(std::ptr::null_mut()))).ok()
}
windows_core::imp::define_interface!(ID3D12CommandAllocator, ID3D12CommandAllocator_Vtbl, 0x6102dee4_af59_4b09_b999_b44d73f09b24);
impl core::ops::Deref for ID3D12CommandAllocator {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12CommandAllocator, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12CommandAllocator {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID3D12CommandAllocator {}
unsafe impl Sync for ID3D12CommandAllocator {}
#[repr(C)]
pub struct ID3D12CommandAllocator_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12CommandList, ID3D12CommandList_Vtbl, 0x7116d91c_e7e4_47ce_b8c6_ec8168f437e5);
impl core::ops::Deref for ID3D12CommandList {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12CommandList, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12CommandList {
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12CommandList {}
unsafe impl Sync for ID3D12CommandList {}
#[repr(C)]
pub struct ID3D12CommandList_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE,
}
windows_core::imp::define_interface!(ID3D12CommandQueue, ID3D12CommandQueue_Vtbl, 0x0ec870a6_5d7e_4c22_8cfc_5baae07616ed);
impl core::ops::Deref for ID3D12CommandQueue {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12CommandQueue, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12CommandQueue {
    pub unsafe fn UpdateTileMappings<P0, P1>(&self, presource: P0, numresourceregions: u32, presourceregionstartcoordinates: Option<*const D3D12_TILED_RESOURCE_COORDINATE>, presourceregionsizes: Option<*const D3D12_TILE_REGION_SIZE>, pheap: P1, numranges: u32, prangeflags: Option<*const D3D12_TILE_RANGE_FLAGS>, pheaprangestartoffsets: Option<*const u32>, prangetilecounts: Option<*const u32>, flags: D3D12_TILE_MAPPING_FLAGS)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Heap>,
    {
        (windows_core::Interface::vtable(self).UpdateTileMappings)(
            windows_core::Interface::as_raw(self),
            presource.param().abi(),
            numresourceregions,
            core::mem::transmute(presourceregionstartcoordinates.unwrap_or(std::ptr::null())),
            core::mem::transmute(presourceregionsizes.unwrap_or(std::ptr::null())),
            pheap.param().abi(),
            numranges,
            core::mem::transmute(prangeflags.unwrap_or(std::ptr::null())),
            core::mem::transmute(pheaprangestartoffsets.unwrap_or(std::ptr::null())),
            core::mem::transmute(prangetilecounts.unwrap_or(std::ptr::null())),
            flags,
        )
    }
    pub unsafe fn CopyTileMappings<P0, P1>(&self, pdstresource: P0, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: P1, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CopyTileMappings)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), pdstregionstartcoordinate, psrcresource.param().abi(), psrcregionstartcoordinate, pregionsize, flags)
    }
    pub unsafe fn ExecuteCommandLists(&self, ppcommandlists: &[Option<ID3D12CommandList>]) {
        (windows_core::Interface::vtable(self).ExecuteCommandLists)(windows_core::Interface::as_raw(self), ppcommandlists.len().try_into().unwrap(), core::mem::transmute(ppcommandlists.as_ptr()))
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), metadata, core::mem::transmute(pdata.unwrap_or(std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), metadata, core::mem::transmute(pdata.unwrap_or(std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Signal<P0>(&self, pfence: P0, value: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self), pfence.param().abi(), value).ok()
    }
    pub unsafe fn Wait<P0>(&self, pfence: P0, value: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), pfence.param().abi(), value).ok()
    }
    pub unsafe fn GetTimestampFrequency(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimestampFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetClockCalibration(&self, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockCalibration)(windows_core::Interface::as_raw(self), pgputimestamp, pcputimestamp).ok()
    }
    pub unsafe fn GetDesc(&self) -> D3D12_COMMAND_QUEUE_DESC {
        let mut result__: D3D12_COMMAND_QUEUE_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12CommandQueue {}
unsafe impl Sync for ID3D12CommandQueue {}
#[repr(C)]
pub struct ID3D12CommandQueue_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub UpdateTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D12_TILED_RESOURCE_COORDINATE, *const D3D12_TILE_REGION_SIZE, *mut core::ffi::c_void, u32, *const D3D12_TILE_RANGE_FLAGS, *const u32, *const u32, D3D12_TILE_MAPPING_FLAGS),
    pub CopyTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_TILED_RESOURCE_COORDINATE, *mut core::ffi::c_void, *const D3D12_TILED_RESOURCE_COORDINATE, *const D3D12_TILE_REGION_SIZE, D3D12_TILE_MAPPING_FLAGS),
    pub ExecuteCommandLists: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void),
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetTimestampFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetClockCalibration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_COMMAND_QUEUE_DESC),
}
windows_core::imp::define_interface!(ID3D12CommandSignature, ID3D12CommandSignature_Vtbl, 0xc36a797c_ec80_4f0a_8985_a7b2475082d1);
impl core::ops::Deref for ID3D12CommandSignature {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12CommandSignature, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12CommandSignature {}
unsafe impl Send for ID3D12CommandSignature {}
unsafe impl Sync for ID3D12CommandSignature {}
#[repr(C)]
pub struct ID3D12CommandSignature_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
windows_core::imp::define_interface!(ID3D12Debug, ID3D12Debug_Vtbl, 0x344488b7_6846_474b_b989_f027448245e0);
impl core::ops::Deref for ID3D12Debug {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug, windows_core::IUnknown);
impl ID3D12Debug {
    pub unsafe fn EnableDebugLayer(&self) {
        (windows_core::Interface::vtable(self).EnableDebugLayer)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12Debug {}
unsafe impl Sync for ID3D12Debug {}
#[repr(C)]
pub struct ID3D12Debug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDebugLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D12Debug1, ID3D12Debug1_Vtbl, 0xaffaa4ca_63fe_4d8e_b8ad_159000af4304);
impl core::ops::Deref for ID3D12Debug1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug1, windows_core::IUnknown);
impl ID3D12Debug1 {
    pub unsafe fn EnableDebugLayer(&self) {
        (windows_core::Interface::vtable(self).EnableDebugLayer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableGPUBasedValidation)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableSynchronizedCommandQueueValidation)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
}
unsafe impl Send for ID3D12Debug1 {}
unsafe impl Sync for ID3D12Debug1 {}
#[repr(C)]
pub struct ID3D12Debug1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDebugLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetEnableGPUBasedValidation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub SetEnableSynchronizedCommandQueueValidation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID3D12Debug2, ID3D12Debug2_Vtbl, 0x93a665c4_a3b2_4e5d_b692_a26ae14e3374);
impl core::ops::Deref for ID3D12Debug2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug2, windows_core::IUnknown);
impl ID3D12Debug2 {
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (windows_core::Interface::vtable(self).SetGPUBasedValidationFlags)(windows_core::Interface::as_raw(self), flags)
    }
}
unsafe impl Send for ID3D12Debug2 {}
unsafe impl Sync for ID3D12Debug2 {}
#[repr(C)]
pub struct ID3D12Debug2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGPUBasedValidationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_GPU_BASED_VALIDATION_FLAGS),
}
windows_core::imp::define_interface!(ID3D12Debug3, ID3D12Debug3_Vtbl, 0x5cf4e58f_f671_4ff1_a542_3686e3d153d1);
impl core::ops::Deref for ID3D12Debug3 {
    type Target = ID3D12Debug;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug3, windows_core::IUnknown, ID3D12Debug);
impl ID3D12Debug3 {
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableGPUBasedValidation)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableSynchronizedCommandQueueValidation)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (windows_core::Interface::vtable(self).SetGPUBasedValidationFlags)(windows_core::Interface::as_raw(self), flags)
    }
}
unsafe impl Send for ID3D12Debug3 {}
unsafe impl Sync for ID3D12Debug3 {}
#[repr(C)]
pub struct ID3D12Debug3_Vtbl {
    pub base__: ID3D12Debug_Vtbl,
    pub SetEnableGPUBasedValidation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub SetEnableSynchronizedCommandQueueValidation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub SetGPUBasedValidationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_GPU_BASED_VALIDATION_FLAGS),
}
windows_core::imp::define_interface!(ID3D12Debug4, ID3D12Debug4_Vtbl, 0x014b816e_9ec5_4a2f_a845_ffbe441ce13a);
impl core::ops::Deref for ID3D12Debug4 {
    type Target = ID3D12Debug3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug4, windows_core::IUnknown, ID3D12Debug, ID3D12Debug3);
impl ID3D12Debug4 {
    pub unsafe fn DisableDebugLayer(&self) {
        (windows_core::Interface::vtable(self).DisableDebugLayer)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12Debug4 {}
unsafe impl Sync for ID3D12Debug4 {}
#[repr(C)]
pub struct ID3D12Debug4_Vtbl {
    pub base__: ID3D12Debug3_Vtbl,
    pub DisableDebugLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D12Debug5, ID3D12Debug5_Vtbl, 0x548d6b12_09fa_40e0_9069_5dcd589a52c9);
impl core::ops::Deref for ID3D12Debug5 {
    type Target = ID3D12Debug4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug5, windows_core::IUnknown, ID3D12Debug, ID3D12Debug3, ID3D12Debug4);
impl ID3D12Debug5 {
    pub unsafe fn SetEnableAutoName<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableAutoName)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
}
unsafe impl Send for ID3D12Debug5 {}
unsafe impl Sync for ID3D12Debug5 {}
#[repr(C)]
pub struct ID3D12Debug5_Vtbl {
    pub base__: ID3D12Debug4_Vtbl,
    pub SetEnableAutoName: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID3D12Debug6, ID3D12Debug6_Vtbl, 0x82a816d6_5d01_4157_97d0_4975463fd1ed);
impl core::ops::Deref for ID3D12Debug6 {
    type Target = ID3D12Debug5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Debug6, windows_core::IUnknown, ID3D12Debug, ID3D12Debug3, ID3D12Debug4, ID3D12Debug5);
impl ID3D12Debug6 {
    pub unsafe fn SetForceLegacyBarrierValidation<P0>(&self, enable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetForceLegacyBarrierValidation)(windows_core::Interface::as_raw(self), enable.param().abi())
    }
}
unsafe impl Send for ID3D12Debug6 {}
unsafe impl Sync for ID3D12Debug6 {}
#[repr(C)]
pub struct ID3D12Debug6_Vtbl {
    pub base__: ID3D12Debug5_Vtbl,
    pub SetForceLegacyBarrierValidation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID3D12DebugCommandList, ID3D12DebugCommandList_Vtbl, 0x09e0bf36_54ac_484f_8847_4baeeab6053f);
impl core::ops::Deref for ID3D12DebugCommandList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandList, windows_core::IUnknown);
impl ID3D12DebugCommandList {
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertResourceState)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, state)
    }
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFeatureMask)(windows_core::Interface::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (windows_core::Interface::vtable(self).GetFeatureMask)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12DebugCommandList {}
unsafe impl Sync for ID3D12DebugCommandList {}
#[repr(C)]
pub struct ID3D12DebugCommandList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssertResourceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> super::super::Foundation::BOOL,
    pub SetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_FEATURE) -> windows_core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_DEBUG_FEATURE,
}
windows_core::imp::define_interface!(ID3D12DebugCommandList1, ID3D12DebugCommandList1_Vtbl, 0x102ca951_311b_4b01_b11f_ecb83e061b37);
impl core::ops::Deref for ID3D12DebugCommandList1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandList1, windows_core::IUnknown);
impl ID3D12DebugCommandList1 {
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertResourceState)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, state)
    }
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
}
unsafe impl Send for ID3D12DebugCommandList1 {}
unsafe impl Sync for ID3D12DebugCommandList1 {}
#[repr(C)]
pub struct ID3D12DebugCommandList1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssertResourceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> super::super::Foundation::BOOL,
    pub SetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DebugCommandList2, ID3D12DebugCommandList2_Vtbl, 0xaeb575cf_4e06_48be_ba3b_c450fc96652e);
impl core::ops::Deref for ID3D12DebugCommandList2 {
    type Target = ID3D12DebugCommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandList2, windows_core::IUnknown, ID3D12DebugCommandList);
impl ID3D12DebugCommandList2 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
}
unsafe impl Send for ID3D12DebugCommandList2 {}
unsafe impl Sync for ID3D12DebugCommandList2 {}
#[repr(C)]
pub struct ID3D12DebugCommandList2_Vtbl {
    pub base__: ID3D12DebugCommandList_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DebugCommandList3, ID3D12DebugCommandList3_Vtbl, 0x197d5e15_4d37_4d34_af78_724cd70fdb1f);
impl core::ops::Deref for ID3D12DebugCommandList3 {
    type Target = ID3D12DebugCommandList2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandList3, windows_core::IUnknown, ID3D12DebugCommandList, ID3D12DebugCommandList2);
impl ID3D12DebugCommandList3 {
    pub unsafe fn AssertResourceAccess<P0>(&self, presource: P0, subresource: u32, access: D3D12_BARRIER_ACCESS)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertResourceAccess)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, access)
    }
    pub unsafe fn AssertTextureLayout<P0>(&self, presource: P0, subresource: u32, layout: D3D12_BARRIER_LAYOUT)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertTextureLayout)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, layout)
    }
}
unsafe impl Send for ID3D12DebugCommandList3 {}
unsafe impl Sync for ID3D12DebugCommandList3 {}
#[repr(C)]
pub struct ID3D12DebugCommandList3_Vtbl {
    pub base__: ID3D12DebugCommandList2_Vtbl,
    pub AssertResourceAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D12_BARRIER_ACCESS),
    pub AssertTextureLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D12_BARRIER_LAYOUT),
}
windows_core::imp::define_interface!(ID3D12DebugCommandQueue, ID3D12DebugCommandQueue_Vtbl, 0x09e0bf36_54ac_484f_8847_4baeeab6053a);
impl core::ops::Deref for ID3D12DebugCommandQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandQueue, windows_core::IUnknown);
impl ID3D12DebugCommandQueue {
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertResourceState)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, state)
    }
}
unsafe impl Send for ID3D12DebugCommandQueue {}
unsafe impl Sync for ID3D12DebugCommandQueue {}
#[repr(C)]
pub struct ID3D12DebugCommandQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssertResourceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D12DebugCommandQueue1, ID3D12DebugCommandQueue1_Vtbl, 0x16be35a2_bfd6_49f2_bcae_eaae4aff862d);
impl core::ops::Deref for ID3D12DebugCommandQueue1 {
    type Target = ID3D12DebugCommandQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugCommandQueue1, windows_core::IUnknown, ID3D12DebugCommandQueue);
impl ID3D12DebugCommandQueue1 {
    pub unsafe fn AssertResourceAccess<P0>(&self, presource: P0, subresource: u32, access: D3D12_BARRIER_ACCESS)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertResourceAccess)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, access)
    }
    pub unsafe fn AssertTextureLayout<P0>(&self, presource: P0, subresource: u32, layout: D3D12_BARRIER_LAYOUT)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AssertTextureLayout)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, layout)
    }
}
unsafe impl Send for ID3D12DebugCommandQueue1 {}
unsafe impl Sync for ID3D12DebugCommandQueue1 {}
#[repr(C)]
pub struct ID3D12DebugCommandQueue1_Vtbl {
    pub base__: ID3D12DebugCommandQueue_Vtbl,
    pub AssertResourceAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D12_BARRIER_ACCESS),
    pub AssertTextureLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D12_BARRIER_LAYOUT),
}
windows_core::imp::define_interface!(ID3D12DebugDevice, ID3D12DebugDevice_Vtbl, 0x3febd6dd_4973_4787_8194_e45f9e28923e);
impl core::ops::Deref for ID3D12DebugDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugDevice, windows_core::IUnknown);
impl ID3D12DebugDevice {
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFeatureMask)(windows_core::Interface::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (windows_core::Interface::vtable(self).GetFeatureMask)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReportLiveDeviceObjects)(windows_core::Interface::as_raw(self), flags).ok()
    }
}
unsafe impl Send for ID3D12DebugDevice {}
unsafe impl Sync for ID3D12DebugDevice {}
#[repr(C)]
pub struct ID3D12DebugDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_FEATURE) -> windows_core::HRESULT,
    pub GetFeatureMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_DEBUG_FEATURE,
    pub ReportLiveDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_RLDO_FLAGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DebugDevice1, ID3D12DebugDevice1_Vtbl, 0xa9b71770_d099_4a65_a698_3dee10020f88);
impl core::ops::Deref for ID3D12DebugDevice1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugDevice1, windows_core::IUnknown);
impl ID3D12DebugDevice1 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReportLiveDeviceObjects)(windows_core::Interface::as_raw(self), flags).ok()
    }
}
unsafe impl Send for ID3D12DebugDevice1 {}
unsafe impl Sync for ID3D12DebugDevice1 {}
#[repr(C)]
pub struct ID3D12DebugDevice1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_DEVICE_PARAMETER_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_DEVICE_PARAMETER_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLiveDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_RLDO_FLAGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DebugDevice2, ID3D12DebugDevice2_Vtbl, 0x60eccbc1_378d_4df1_894c_f8ac5ce4d7dd);
impl core::ops::Deref for ID3D12DebugDevice2 {
    type Target = ID3D12DebugDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DebugDevice2, windows_core::IUnknown, ID3D12DebugDevice);
impl ID3D12DebugDevice2 {
    pub unsafe fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
    pub unsafe fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDebugParameter)(windows_core::Interface::as_raw(self), r#type, pdata, datasize).ok()
    }
}
unsafe impl Send for ID3D12DebugDevice2 {}
unsafe impl Sync for ID3D12DebugDevice2 {}
#[repr(C)]
pub struct ID3D12DebugDevice2_Vtbl {
    pub base__: ID3D12DebugDevice_Vtbl,
    pub SetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_DEVICE_PARAMETER_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugParameter: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEBUG_DEVICE_PARAMETER_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DescriptorHeap, ID3D12DescriptorHeap_Vtbl, 0x8efb471d_616c_4f49_90f7_127bb763fa51);
impl core::ops::Deref for ID3D12DescriptorHeap {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DescriptorHeap, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12DescriptorHeap {
    pub unsafe fn GetDesc(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        let mut result__: D3D12_DESCRIPTOR_HEAP_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetCPUDescriptorHandleForHeapStart(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let mut result__: D3D12_CPU_DESCRIPTOR_HANDLE = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCPUDescriptorHandleForHeapStart)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUDescriptorHandleForHeapStart(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        let mut result__: D3D12_GPU_DESCRIPTOR_HANDLE = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGPUDescriptorHandleForHeapStart)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12DescriptorHeap {}
unsafe impl Sync for ID3D12DescriptorHeap {}
#[repr(C)]
pub struct ID3D12DescriptorHeap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DESCRIPTOR_HEAP_DESC),
    pub GetCPUDescriptorHandleForHeapStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_CPU_DESCRIPTOR_HANDLE),
    pub GetGPUDescriptorHandleForHeapStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_GPU_DESCRIPTOR_HANDLE),
}
windows_core::imp::define_interface!(ID3D12Device, ID3D12Device_Vtbl, 0x189819f1_1db6_4b57_be54_1821339b85f7);
impl core::ops::Deref for ID3D12Device {
    type Target = ID3D12Object;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device, windows_core::IUnknown, ID3D12Object);
impl ID3D12Device {
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetNodeCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCommandQueue)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCommandAllocator)(windows_core::Interface::as_raw(self), r#type, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateGraphicsPipelineState)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateComputePipelineState)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ID3D12CommandAllocator>,
        P1: windows_core::Param<ID3D12PipelineState>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCommandList)(windows_core::Interface::as_raw(self), nodemask, r#type, pcommandallocator.param().abi(), pinitialstate.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateDescriptorHeap)(windows_core::Interface::as_raw(self), pdescriptorheapdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (windows_core::Interface::vtable(self).GetDescriptorHandleIncrementSize)(windows_core::Interface::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateRootSignature)(windows_core::Interface::as_raw(self), nodemask, core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len().try_into().unwrap(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (windows_core::Interface::vtable(self).CreateConstantBufferView)(windows_core::Interface::as_raw(self), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(destdescriptor))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(destdescriptor))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateUnorderedAccessView)(windows_core::Interface::as_raw(self), presource.param().abi(), pcounterresource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(destdescriptor))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(destdescriptor))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pdesc.unwrap_or(std::ptr::null())), core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (windows_core::Interface::vtable(self).CreateSampler)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (windows_core::Interface::vtable(self).CopyDescriptors)(windows_core::Interface::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (windows_core::Interface::vtable(self).CopyDescriptorsSimple)(windows_core::Interface::as_raw(self), numdescriptors, core::mem::transmute(destdescriptorrangestart), core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceAllocationInfo)(windows_core::Interface::as_raw(self), &mut result__, visiblemask, presourcedescs.len().try_into().unwrap(), core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomHeapProperties)(windows_core::Interface::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateCommittedResource)(windows_core::Interface::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateHeap)(windows_core::Interface::as_raw(self), pdesc, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Heap>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreatePlacedResource)(windows_core::Interface::as_raw(self), pheap.param().abi(), heapoffset, pdesc, initialstate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateReservedResource)(windows_core::Interface::as_raw(self), pdesc, initialstate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<ID3D12DeviceChild>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pobject.param().abi(), core::mem::transmute(pattributes.unwrap_or(std::ptr::null())), access, name.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).OpenSharedHandle)(windows_core::Interface::as_raw(self), nthandle.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenSharedHandleByName)(windows_core::Interface::as_raw(self), name.param().abi(), access, &mut result__).map(|| result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[Option<ID3D12Pageable>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MakeResident)(windows_core::Interface::as_raw(self), ppobjects.len().try_into().unwrap(), core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[Option<ID3D12Pageable>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Evict)(windows_core::Interface::as_raw(self), ppobjects.len().try_into().unwrap(), core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFence)(windows_core::Interface::as_raw(self), initialvalue, flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: Option<*mut u32>, prowsizeinbytes: Option<*mut u64>, ptotalbytes: Option<*mut u64>) {
        (windows_core::Interface::vtable(self).GetCopyableFootprints)(windows_core::Interface::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, core::mem::transmute(playouts.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumrows.unwrap_or(std::ptr::null_mut())), core::mem::transmute(prowsizeinbytes.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ptotalbytes.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateQueryHeap)(windows_core::Interface::as_raw(self), pdesc, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStablePowerState)(windows_core::Interface::as_raw(self), enable.param().abi()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12RootSignature>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateCommandSignature)(windows_core::Interface::as_raw(self), pdesc, prootsignature.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: Option<*mut u32>, ppackedmipdesc: Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).GetResourceTiling)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), core::mem::transmute(pnumtilesforentireresource.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppackedmipdesc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumsubresourcetilings.unwrap_or(std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAdapterLuid)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12Device {}
unsafe impl Sync for ID3D12Device {}
#[repr(C)]
pub struct ID3D12Device_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetNodeCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub CreateCommandQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_COMMAND_QUEUE_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCommandAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_COMMAND_LIST_TYPE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateGraphicsPipelineState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateGraphicsPipelineState: usize,
    pub CreateComputePipelineState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_COMPUTE_PIPELINE_STATE_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D12_COMMAND_LIST_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateDescriptorHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_DESCRIPTOR_HEAP_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescriptorHandleIncrementSize: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DESCRIPTOR_HEAP_TYPE) -> u32,
    pub CreateRootSignature: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConstantBufferView: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_CONSTANT_BUFFER_VIEW_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_SHADER_RESOURCE_VIEW_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_UNORDERED_ACCESS_VIEW_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateUnorderedAccessView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_RENDER_TARGET_VIEW_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_DEPTH_STENCIL_VIEW_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateDepthStencilView: usize,
    pub CreateSampler: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_SAMPLER_DESC, D3D12_CPU_DESCRIPTOR_HANDLE),
    pub CopyDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_CPU_DESCRIPTOR_HANDLE, *const u32, u32, *const D3D12_CPU_DESCRIPTOR_HANDLE, *const u32, D3D12_DESCRIPTOR_HEAP_TYPE),
    pub CopyDescriptorsSimple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_TYPE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_ALLOCATION_INFO, u32, u32, *const D3D12_RESOURCE_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo: usize,
    pub GetCustomHeapProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_HEAP_PROPERTIES, u32, D3D12_HEAP_TYPE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_PROPERTIES, D3D12_HEAP_FLAGS, *const D3D12_RESOURCE_DESC, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource: usize,
    pub CreateHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreatePlacedResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *const D3D12_RESOURCE_DESC, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreatePlacedResource: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateReservedResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOURCE_DESC, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateReservedResource: usize,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
    pub OpenSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenSharedHandleByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub MakeResident: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Evict: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFence: unsafe extern "system" fn(*mut core::ffi::c_void, u64, D3D12_FENCE_FLAGS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetCopyableFootprints: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOURCE_DESC, u32, u32, u64, *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, *mut u32, *mut u64, *mut u64),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetCopyableFootprints: usize,
    pub CreateQueryHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_QUERY_HEAP_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStablePowerState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateCommandSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_COMMAND_SIGNATURE_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourceTiling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut D3D12_PACKED_MIP_INFO, *mut D3D12_TILE_SHAPE, *mut u32, u32, *mut D3D12_SUBRESOURCE_TILING),
    pub GetAdapterLuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID),
}
windows_core::imp::define_interface!(ID3D12Device1, ID3D12Device1_Vtbl, 0x77acce80_638e_4e65_8895_c1f23386863e);
impl core::ops::Deref for ID3D12Device1 {
    type Target = ID3D12Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device1, windows_core::IUnknown, ID3D12Object, ID3D12Device);
impl ID3D12Device1 {
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreatePipelineLibrary)(windows_core::Interface::as_raw(self), core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len().try_into().unwrap(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetEventOnMultipleFenceCompletion)(windows_core::Interface::as_raw(self), core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.param().abi()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResidencyPriority)(windows_core::Interface::as_raw(self), numobjects, core::mem::transmute(ppobjects), ppriorities).ok()
    }
}
unsafe impl Send for ID3D12Device1 {}
unsafe impl Sync for ID3D12Device1 {}
#[repr(C)]
pub struct ID3D12Device1_Vtbl {
    pub base__: ID3D12Device_Vtbl,
    pub CreatePipelineLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventOnMultipleFenceCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *const u64, u32, D3D12_MULTIPLE_FENCE_WAIT_FLAGS, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetResidencyPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const D3D12_RESIDENCY_PRIORITY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device10, ID3D12Device10_Vtbl, 0x517f8718_aa66_49f9_b02b_a7ab89c06031);
impl core::ops::Deref for ID3D12Device10 {
    type Target = ID3D12Device9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device10, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8, ID3D12Device9);
impl ID3D12Device10 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource3<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, pcastableformats: Option<&[super::Dxgi::Common::DXGI_FORMAT]>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateCommittedResource3)(windows_core::Interface::as_raw(self), pheapproperties, heapflags, pdesc, initiallayout, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pprotectedsession.param().abi(), pcastableformats.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcastableformats.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource2<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pcastableformats: Option<&[super::Dxgi::Common::DXGI_FORMAT]>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Heap>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreatePlacedResource2)(windows_core::Interface::as_raw(self), pheap.param().abi(), heapoffset, pdesc, initiallayout, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pcastableformats.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcastableformats.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource2<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, pcastableformats: Option<&[super::Dxgi::Common::DXGI_FORMAT]>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateReservedResource2)(windows_core::Interface::as_raw(self), pdesc, initiallayout, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pprotectedsession.param().abi(), pcastableformats.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcastableformats.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D12Device10 {}
unsafe impl Sync for ID3D12Device10 {}
#[repr(C)]
pub struct ID3D12Device10_Vtbl {
    pub base__: ID3D12Device9_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource3: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_PROPERTIES, D3D12_HEAP_FLAGS, *const D3D12_RESOURCE_DESC1, D3D12_BARRIER_LAYOUT, *const D3D12_CLEAR_VALUE, *mut core::ffi::c_void, u32, *const super::Dxgi::Common::DXGI_FORMAT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource3: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreatePlacedResource2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *const D3D12_RESOURCE_DESC1, D3D12_BARRIER_LAYOUT, *const D3D12_CLEAR_VALUE, u32, *const super::Dxgi::Common::DXGI_FORMAT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreatePlacedResource2: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateReservedResource2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOURCE_DESC, D3D12_BARRIER_LAYOUT, *const D3D12_CLEAR_VALUE, *mut core::ffi::c_void, u32, *const super::Dxgi::Common::DXGI_FORMAT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateReservedResource2: usize,
}
windows_core::imp::define_interface!(ID3D12Device11, ID3D12Device11_Vtbl, 0x5405c344_d457_444e_b4dd_2366e45aee39);
impl core::ops::Deref for ID3D12Device11 {
    type Target = ID3D12Device10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device11, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8, ID3D12Device9, ID3D12Device10);
impl ID3D12Device11 {
    pub unsafe fn CreateSampler2(&self, pdesc: *const D3D12_SAMPLER_DESC2, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (windows_core::Interface::vtable(self).CreateSampler2)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(destdescriptor))
    }
}
unsafe impl Send for ID3D12Device11 {}
unsafe impl Sync for ID3D12Device11 {}
#[repr(C)]
pub struct ID3D12Device11_Vtbl {
    pub base__: ID3D12Device10_Vtbl,
    pub CreateSampler2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_SAMPLER_DESC2, D3D12_CPU_DESCRIPTOR_HANDLE),
}
windows_core::imp::define_interface!(ID3D12Device12, ID3D12Device12_Vtbl, 0x5af5c532_4c91_4cd0_b541_15a405395fc5);
impl core::ops::Deref for ID3D12Device12 {
    type Target = ID3D12Device11;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device12, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8, ID3D12Device9, ID3D12Device10, ID3D12Device11);
impl ID3D12Device12 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo3(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, pnumcastableformats: Option<*const u32>, ppcastableformats: Option<*const *const super::Dxgi::Common::DXGI_FORMAT>, presourceallocationinfo1: Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceAllocationInfo3)(windows_core::Interface::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, core::mem::transmute(pnumcastableformats.unwrap_or(std::ptr::null())), core::mem::transmute(ppcastableformats.unwrap_or(std::ptr::null())), core::mem::transmute(presourceallocationinfo1.unwrap_or(std::ptr::null_mut())));
        result__
    }
}
unsafe impl Send for ID3D12Device12 {}
unsafe impl Sync for ID3D12Device12 {}
#[repr(C)]
pub struct ID3D12Device12_Vtbl {
    pub base__: ID3D12Device11_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_ALLOCATION_INFO, u32, u32, *const D3D12_RESOURCE_DESC1, *const u32, *const *const super::Dxgi::Common::DXGI_FORMAT, *mut D3D12_RESOURCE_ALLOCATION_INFO1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo3: usize,
}
windows_core::imp::define_interface!(ID3D12Device13, ID3D12Device13_Vtbl, 0x14eecffc_4df8_40f7_a118_5c816f45695e);
impl core::ops::Deref for ID3D12Device13 {
    type Target = ID3D12Device12;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device13, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8, ID3D12Device9, ID3D12Device10, ID3D12Device11, ID3D12Device12);
impl ID3D12Device13 {
    pub unsafe fn OpenExistingHeapFromAddress1<T>(&self, paddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenExistingHeapFromAddress1)(windows_core::Interface::as_raw(self), paddress, size, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12Device13 {}
unsafe impl Sync for ID3D12Device13 {}
#[repr(C)]
pub struct ID3D12Device13_Vtbl {
    pub base__: ID3D12Device12_Vtbl,
    pub OpenExistingHeapFromAddress1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device2, ID3D12Device2_Vtbl, 0x30baa41e_b15b_475c_a0bb_1af5c5b64328);
impl core::ops::Deref for ID3D12Device2 {
    type Target = ID3D12Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device2, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1);
impl ID3D12Device2 {
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreatePipelineState)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12Device2 {}
unsafe impl Sync for ID3D12Device2 {}
#[repr(C)]
pub struct ID3D12Device2_Vtbl {
    pub base__: ID3D12Device1_Vtbl,
    pub CreatePipelineState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_PIPELINE_STATE_STREAM_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device3, ID3D12Device3_Vtbl, 0x81dadc15_2bad_4392_93c5_101345c4aa98);
impl core::ops::Deref for ID3D12Device3 {
    type Target = ID3D12Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device3, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2);
impl ID3D12Device3 {
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const core::ffi::c_void) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenExistingHeapFromAddress)(windows_core::Interface::as_raw(self), paddress, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenExistingHeapFromFileMapping)(windows_core::Interface::as_raw(self), hfilemapping.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[Option<ID3D12Pageable>], pfencetosignal: P0, fencevaluetosignal: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).EnqueueMakeResident)(windows_core::Interface::as_raw(self), flags, ppobjects.len().try_into().unwrap(), core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.param().abi(), fencevaluetosignal).ok()
    }
}
unsafe impl Send for ID3D12Device3 {}
unsafe impl Sync for ID3D12Device3 {}
#[repr(C)]
pub struct ID3D12Device3_Vtbl {
    pub base__: ID3D12Device2_Vtbl,
    pub OpenExistingHeapFromAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenExistingHeapFromFileMapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnqueueMakeResident: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_RESIDENCY_FLAGS, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device4, ID3D12Device4_Vtbl, 0xe865df17_a9ee_46f9_a463_3098315aa2e5);
impl core::ops::Deref for ID3D12Device4 {
    type Target = ID3D12Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device4, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3);
impl ID3D12Device4 {
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCommandList1)(windows_core::Interface::as_raw(self), nodemask, r#type, flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateProtectedResourceSession)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateCommittedResource1)(windows_core::Interface::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pprotectedsession.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateHeap1)(windows_core::Interface::as_raw(self), pdesc, pprotectedsession.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateReservedResource1)(windows_core::Interface::as_raw(self), pdesc, initialstate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pprotectedsession.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceAllocationInfo1)(windows_core::Interface::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, core::mem::transmute(presourceallocationinfo1.unwrap_or(std::ptr::null_mut())));
        result__
    }
}
unsafe impl Send for ID3D12Device4 {}
unsafe impl Sync for ID3D12Device4 {}
#[repr(C)]
pub struct ID3D12Device4_Vtbl {
    pub base__: ID3D12Device3_Vtbl,
    pub CreateCommandList1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_LIST_FLAGS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_PROPERTIES, D3D12_HEAP_FLAGS, *const D3D12_RESOURCE_DESC, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource1: usize,
    pub CreateHeap1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateReservedResource1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOURCE_DESC, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateReservedResource1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_ALLOCATION_INFO, u32, u32, *const D3D12_RESOURCE_DESC, *mut D3D12_RESOURCE_ALLOCATION_INFO1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo1: usize,
}
windows_core::imp::define_interface!(ID3D12Device5, ID3D12Device5_Vtbl, 0x8b4f173b_2fea_4b80_8f58_4307191ab95d);
impl core::ops::Deref for ID3D12Device5 {
    type Target = ID3D12Device4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device5, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4);
impl ID3D12Device5 {
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ID3D12LifetimeOwner>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateLifetimeTracker)(windows_core::Interface::as_raw(self), powner.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveDevice(&self) {
        (windows_core::Interface::vtable(self).RemoveDevice)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: Option<*mut D3D12_META_COMMAND_DESC>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateMetaCommands)(windows_core::Interface::as_raw(self), pnummetacommands, core::mem::transmute(pdescs.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const windows_core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateMetaCommandParameters)(windows_core::Interface::as_raw(self), commandid, stage, core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(std::ptr::null_mut())), pparametercount, core::mem::transmute(pparameterdescs.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const windows_core::GUID, nodemask: u32, pcreationparametersdata: Option<*const core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateMetaCommand)(windows_core::Interface::as_raw(self), commandid, nodemask, core::mem::transmute(pcreationparametersdata.unwrap_or(std::ptr::null())), creationparametersdatasizeinbytes, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateStateObject)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (windows_core::Interface::vtable(self).GetRaytracingAccelerationStructurePrebuildInfo)(windows_core::Interface::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (windows_core::Interface::vtable(self).CheckDriverMatchingIdentifier)(windows_core::Interface::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
}
unsafe impl Send for ID3D12Device5 {}
unsafe impl Sync for ID3D12Device5 {}
#[repr(C)]
pub struct ID3D12Device5_Vtbl {
    pub base__: ID3D12Device4_Vtbl,
    pub CreateLifetimeTracker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveDevice: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EnumerateMetaCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D12_META_COMMAND_DESC) -> windows_core::HRESULT,
    pub EnumerateMetaCommandParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, D3D12_META_COMMAND_PARAMETER_STAGE, *mut u32, *mut u32, *mut D3D12_META_COMMAND_PARAMETER_DESC) -> windows_core::HRESULT,
    pub CreateMetaCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_STATE_OBJECT_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRaytracingAccelerationStructurePrebuildInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRaytracingAccelerationStructurePrebuildInfo: usize,
    pub CheckDriverMatchingIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_SERIALIZED_DATA_TYPE, *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS,
}
windows_core::imp::define_interface!(ID3D12Device6, ID3D12Device6_Vtbl, 0xc70b221b_40e4_4a17_89af_025a0727a6dc);
impl core::ops::Deref for ID3D12Device6 {
    type Target = ID3D12Device5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device6, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5);
impl ID3D12Device6 {
    pub unsafe fn SetBackgroundProcessingMode<P0>(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: P0, pbfurthermeasurementsdesired: Option<*mut super::super::Foundation::BOOL>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetBackgroundProcessingMode)(windows_core::Interface::as_raw(self), mode, measurementsaction, heventtosignaluponcompletion.param().abi(), core::mem::transmute(pbfurthermeasurementsdesired.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
unsafe impl Send for ID3D12Device6 {}
unsafe impl Sync for ID3D12Device6 {}
#[repr(C)]
pub struct ID3D12Device6_Vtbl {
    pub base__: ID3D12Device5_Vtbl,
    pub SetBackgroundProcessingMode: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_BACKGROUND_PROCESSING_MODE, D3D12_MEASUREMENTS_ACTION, super::super::Foundation::HANDLE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device7, ID3D12Device7_Vtbl, 0x5c014b53_68a1_4b9b_8bd1_dd6046b9358b);
impl core::ops::Deref for ID3D12Device7 {
    type Target = ID3D12Device6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device7, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6);
impl ID3D12Device7 {
    pub unsafe fn AddToStateObject<P0, T>(&self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ID3D12StateObject>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).AddToStateObject)(windows_core::Interface::as_raw(self), paddition, pstateobjecttogrowfrom.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateProtectedResourceSession1<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateProtectedResourceSession1)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12Device7 {}
unsafe impl Sync for ID3D12Device7 {}
#[repr(C)]
pub struct ID3D12Device7_Vtbl {
    pub base__: ID3D12Device6_Vtbl,
    pub AddToStateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_STATE_OBJECT_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateProtectedResourceSession1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Device8, ID3D12Device8_Vtbl, 0x9218e6bb_f944_4f7e_a75c_b1b2c7b701f3);
impl core::ops::Deref for ID3D12Device8 {
    type Target = ID3D12Device7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device8, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7);
impl ID3D12Device8 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo2(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceAllocationInfo2)(windows_core::Interface::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, core::mem::transmute(presourceallocationinfo1.unwrap_or(std::ptr::null_mut())));
        result__
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource2<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateCommittedResource2)(windows_core::Interface::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), pprotectedsession.param().abi(), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource1<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: Option<*const D3D12_CLEAR_VALUE>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12Heap>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreatePlacedResource1)(windows_core::Interface::as_raw(self), pheap.param().abi(), heapoffset, pdesc, initialstate, core::mem::transmute(poptimizedclearvalue.unwrap_or(std::ptr::null())), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateSamplerFeedbackUnorderedAccessView<P0, P1>(&self, ptargetedresource: P0, pfeedbackresource: P1, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateSamplerFeedbackUnorderedAccessView)(windows_core::Interface::as_raw(self), ptargetedresource.param().abi(), pfeedbackresource.param().abi(), core::mem::transmute(destdescriptor))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints1(&self, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: Option<*mut u32>, prowsizeinbytes: Option<*mut u64>, ptotalbytes: Option<*mut u64>) {
        (windows_core::Interface::vtable(self).GetCopyableFootprints1)(windows_core::Interface::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, core::mem::transmute(playouts.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pnumrows.unwrap_or(std::ptr::null_mut())), core::mem::transmute(prowsizeinbytes.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ptotalbytes.unwrap_or(std::ptr::null_mut())))
    }
}
unsafe impl Send for ID3D12Device8 {}
unsafe impl Sync for ID3D12Device8 {}
#[repr(C)]
pub struct ID3D12Device8_Vtbl {
    pub base__: ID3D12Device7_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetResourceAllocationInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_ALLOCATION_INFO, u32, u32, *const D3D12_RESOURCE_DESC1, *mut D3D12_RESOURCE_ALLOCATION_INFO1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetResourceAllocationInfo2: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateCommittedResource2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_HEAP_PROPERTIES, D3D12_HEAP_FLAGS, *const D3D12_RESOURCE_DESC1, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateCommittedResource2: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreatePlacedResource1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *const D3D12_RESOURCE_DESC1, D3D12_RESOURCE_STATES, *const D3D12_CLEAR_VALUE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreatePlacedResource1: usize,
    pub CreateSamplerFeedbackUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, D3D12_CPU_DESCRIPTOR_HANDLE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetCopyableFootprints1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOURCE_DESC1, u32, u32, u64, *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, *mut u32, *mut u64, *mut u64),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetCopyableFootprints1: usize,
}
windows_core::imp::define_interface!(ID3D12Device9, ID3D12Device9_Vtbl, 0x4c80e962_f032_4f60_bc9e_ebc2cfa1d83c);
impl core::ops::Deref for ID3D12Device9 {
    type Target = ID3D12Device8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Device9, windows_core::IUnknown, ID3D12Object, ID3D12Device, ID3D12Device1, ID3D12Device2, ID3D12Device3, ID3D12Device4, ID3D12Device5, ID3D12Device6, ID3D12Device7, ID3D12Device8);
impl ID3D12Device9 {
    pub unsafe fn CreateShaderCacheSession<T>(&self, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateShaderCacheSession)(windows_core::Interface::as_raw(self), pdesc, &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn ShaderCacheControl(&self, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShaderCacheControl)(windows_core::Interface::as_raw(self), kinds, control).ok()
    }
    pub unsafe fn CreateCommandQueue1<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateCommandQueue1)(windows_core::Interface::as_raw(self), pdesc, creatorid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12Device9 {}
unsafe impl Sync for ID3D12Device9 {}
#[repr(C)]
pub struct ID3D12Device9_Vtbl {
    pub base__: ID3D12Device8_Vtbl,
    pub CreateShaderCacheSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_SHADER_CACHE_SESSION_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShaderCacheControl: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_SHADER_CACHE_KIND_FLAGS, D3D12_SHADER_CACHE_CONTROL_FLAGS) -> windows_core::HRESULT,
    pub CreateCommandQueue1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_COMMAND_QUEUE_DESC, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DeviceChild, ID3D12DeviceChild_Vtbl, 0x905db94b_a00c_4140_9df5_2b64ca9ea357);
impl core::ops::Deref for ID3D12DeviceChild {
    type Target = ID3D12Object;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceChild, windows_core::IUnknown, ID3D12Object);
impl ID3D12DeviceChild {
    pub unsafe fn GetDevice<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D12DeviceChild {}
unsafe impl Sync for ID3D12DeviceChild {}
#[repr(C)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DeviceConfiguration, ID3D12DeviceConfiguration_Vtbl, 0x78dbf87b_f766_422b_a61c_c8c446bdb9ad);
impl core::ops::Deref for ID3D12DeviceConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceConfiguration, windows_core::IUnknown);
impl ID3D12DeviceConfiguration {
    pub unsafe fn GetDesc(&self) -> D3D12_DEVICE_CONFIGURATION_DESC {
        let mut result__: D3D12_DEVICE_CONFIGURATION_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetEnabledExperimentalFeatures(&self, pguids: &mut [windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEnabledExperimentalFeatures)(windows_core::Interface::as_raw(self), core::mem::transmute(pguids.as_ptr()), pguids.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SerializeVersionedRootSignature(&self, pdesc: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppresult: *mut Option<super::Direct3D::ID3DBlob>, pperror: Option<*mut Option<super::Direct3D::ID3DBlob>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SerializeVersionedRootSignature)(windows_core::Interface::as_raw(self), pdesc, core::mem::transmute(ppresult), core::mem::transmute(pperror.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVersionedRootSignatureDeserializer<T>(&self, pblob: *const core::ffi::c_void, size: usize) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateVersionedRootSignatureDeserializer)(windows_core::Interface::as_raw(self), pblob, size, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12DeviceConfiguration {}
unsafe impl Sync for ID3D12DeviceConfiguration {}
#[repr(C)]
pub struct ID3D12DeviceConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DEVICE_CONFIGURATION_DESC),
    pub GetEnabledExperimentalFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SerializeVersionedRootSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SerializeVersionedRootSignature: usize,
    pub CreateVersionedRootSignatureDeserializer: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DeviceFactory, ID3D12DeviceFactory_Vtbl, 0x61f307d3_d34e_4e7c_8374_3ba4de23cccb);
impl core::ops::Deref for ID3D12DeviceFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceFactory, windows_core::IUnknown);
impl ID3D12DeviceFactory {
    pub unsafe fn InitializeFromGlobalState(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromGlobalState)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ApplyToGlobalState(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyToGlobalState)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, flags: D3D12_DEVICE_FACTORY_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetFlags(&self) -> D3D12_DEVICE_FACTORY_FLAGS {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetConfigurationInterface<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetConfigurationInterface)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableExperimentalFeatures(&self, numfeatures: u32, piids: *const windows_core::GUID, pconfigurationstructs: Option<*const core::ffi::c_void>, pconfigurationstructsizes: Option<*const u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableExperimentalFeatures)(windows_core::Interface::as_raw(self), numfeatures, piids, core::mem::transmute(pconfigurationstructs.unwrap_or(std::ptr::null())), core::mem::transmute(pconfigurationstructsizes.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDevice<P0, T>(&self, adapter: P0, featurelevel: super::Direct3D::D3D_FEATURE_LEVEL, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), adapter.param().abi(), featurelevel, &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D12DeviceFactory {}
unsafe impl Sync for ID3D12DeviceFactory {}
#[repr(C)]
pub struct ID3D12DeviceFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromGlobalState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyToGlobalState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DEVICE_FACTORY_FLAGS) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_DEVICE_FACTORY_FLAGS,
    pub GetConfigurationInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableExperimentalFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Direct3D::D3D_FEATURE_LEVEL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedData, ID3D12DeviceRemovedExtendedData_Vtbl, 0x98931d33_5ae8_4791_aa3c_1a73a2934e71);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedData, windows_core::IUnknown);
impl ID3D12DeviceRemovedExtendedData {
    pub unsafe fn GetAutoBreadcrumbsOutput(&self) -> windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutoBreadcrumbsOutput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput(&self) -> windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageFaultAllocationOutput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedData {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedData {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAutoBreadcrumbsOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> windows_core::HRESULT,
    pub GetPageFaultAllocationOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedData1, ID3D12DeviceRemovedExtendedData1_Vtbl, 0x9727a022_cf1d_4dda_9eba_effa653fc506);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedData1 {
    type Target = ID3D12DeviceRemovedExtendedData;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedData1, windows_core::IUnknown, ID3D12DeviceRemovedExtendedData);
impl ID3D12DeviceRemovedExtendedData1 {
    pub unsafe fn GetAutoBreadcrumbsOutput1(&self) -> windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutoBreadcrumbsOutput1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput1(&self) -> windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageFaultAllocationOutput1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedData1 {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedData1 {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedData1_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedData_Vtbl,
    pub GetAutoBreadcrumbsOutput1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> windows_core::HRESULT,
    pub GetPageFaultAllocationOutput1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedData2, ID3D12DeviceRemovedExtendedData2_Vtbl, 0x67fc5816_e4ca_4915_bf18_42541272da54);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedData2 {
    type Target = ID3D12DeviceRemovedExtendedData1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedData2, windows_core::IUnknown, ID3D12DeviceRemovedExtendedData, ID3D12DeviceRemovedExtendedData1);
impl ID3D12DeviceRemovedExtendedData2 {
    pub unsafe fn GetPageFaultAllocationOutput2(&self, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPageFaultAllocationOutput2)(windows_core::Interface::as_raw(self), poutput).ok()
    }
    pub unsafe fn GetDeviceState(&self) -> D3D12_DRED_DEVICE_STATE {
        (windows_core::Interface::vtable(self).GetDeviceState)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedData2 {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedData2 {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedData2_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedData1_Vtbl,
    pub GetPageFaultAllocationOutput2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE,
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedDataSettings, ID3D12DeviceRemovedExtendedDataSettings_Vtbl, 0x82bc481c_6b9b_4030_aedb_7ee3d1df1e63);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedDataSettings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedDataSettings, windows_core::IUnknown);
impl ID3D12DeviceRemovedExtendedDataSettings {
    pub unsafe fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (windows_core::Interface::vtable(self).SetAutoBreadcrumbsEnablement)(windows_core::Interface::as_raw(self), enablement)
    }
    pub unsafe fn SetPageFaultEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (windows_core::Interface::vtable(self).SetPageFaultEnablement)(windows_core::Interface::as_raw(self), enablement)
    }
    pub unsafe fn SetWatsonDumpEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (windows_core::Interface::vtable(self).SetWatsonDumpEnablement)(windows_core::Interface::as_raw(self), enablement)
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedDataSettings {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedDataSettings {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAutoBreadcrumbsEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DRED_ENABLEMENT),
    pub SetPageFaultEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DRED_ENABLEMENT),
    pub SetWatsonDumpEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DRED_ENABLEMENT),
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedDataSettings1, ID3D12DeviceRemovedExtendedDataSettings1_Vtbl, 0xdbd5ae51_3317_4f0a_adf9_1d7cedcaae0b);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedDataSettings1 {
    type Target = ID3D12DeviceRemovedExtendedDataSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedDataSettings1, windows_core::IUnknown, ID3D12DeviceRemovedExtendedDataSettings);
impl ID3D12DeviceRemovedExtendedDataSettings1 {
    pub unsafe fn SetBreadcrumbContextEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (windows_core::Interface::vtable(self).SetBreadcrumbContextEnablement)(windows_core::Interface::as_raw(self), enablement)
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedDataSettings1 {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedDataSettings1 {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedDataSettings_Vtbl,
    pub SetBreadcrumbContextEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_DRED_ENABLEMENT),
}
windows_core::imp::define_interface!(ID3D12DeviceRemovedExtendedDataSettings2, ID3D12DeviceRemovedExtendedDataSettings2_Vtbl, 0x61552388_01ab_4008_a436_83db189566ea);
impl core::ops::Deref for ID3D12DeviceRemovedExtendedDataSettings2 {
    type Target = ID3D12DeviceRemovedExtendedDataSettings1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceRemovedExtendedDataSettings2, windows_core::IUnknown, ID3D12DeviceRemovedExtendedDataSettings, ID3D12DeviceRemovedExtendedDataSettings1);
impl ID3D12DeviceRemovedExtendedDataSettings2 {
    pub unsafe fn UseMarkersOnlyAutoBreadcrumbs<P0>(&self, markersonly: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UseMarkersOnlyAutoBreadcrumbs)(windows_core::Interface::as_raw(self), markersonly.param().abi())
    }
}
unsafe impl Send for ID3D12DeviceRemovedExtendedDataSettings2 {}
unsafe impl Sync for ID3D12DeviceRemovedExtendedDataSettings2 {}
#[repr(C)]
pub struct ID3D12DeviceRemovedExtendedDataSettings2_Vtbl {
    pub base__: ID3D12DeviceRemovedExtendedDataSettings1_Vtbl,
    pub UseMarkersOnlyAutoBreadcrumbs: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID3D12Fence, ID3D12Fence_Vtbl, 0x0a753dcf_c4d8_4b91_adf6_be5a60d95a76);
impl core::ops::Deref for ID3D12Fence {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Fence, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12Fence {
    pub unsafe fn GetCompletedValue(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetCompletedValue)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetEventOnCompletion<P0>(&self, value: u64, hevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetEventOnCompletion)(windows_core::Interface::as_raw(self), value, hevent.param().abi()).ok()
    }
    pub unsafe fn Signal(&self, value: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self), value).ok()
    }
}
unsafe impl Send for ID3D12Fence {}
unsafe impl Sync for ID3D12Fence {}
#[repr(C)]
pub struct ID3D12Fence_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetCompletedValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub SetEventOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Fence1, ID3D12Fence1_Vtbl, 0x433685fe_e22b_4ca0_a8db_b5b4f4dd0e4a);
impl core::ops::Deref for ID3D12Fence1 {
    type Target = ID3D12Fence;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Fence1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Fence);
impl ID3D12Fence1 {
    pub unsafe fn GetCreationFlags(&self) -> D3D12_FENCE_FLAGS {
        (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12Fence1 {}
unsafe impl Sync for ID3D12Fence1 {}
#[repr(C)]
pub struct ID3D12Fence1_Vtbl {
    pub base__: ID3D12Fence_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_FENCE_FLAGS,
}
windows_core::imp::define_interface!(ID3D12FunctionParameterReflection, ID3D12FunctionParameterReflection_Vtbl);
impl ID3D12FunctionParameterReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
}
unsafe impl Send for ID3D12FunctionParameterReflection {}
unsafe impl Sync for ID3D12FunctionParameterReflection {}
#[repr(C)]
pub struct ID3D12FunctionParameterReflection_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
}
windows_core::imp::define_interface!(ID3D12FunctionReflection, ID3D12FunctionReflection_Vtbl);
impl ID3D12FunctionReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), bufferindex)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc).ok()
    }
    pub unsafe fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection> {
        (windows_core::Interface::vtable(self).GetFunctionParameter)(windows_core::Interface::as_raw(self), parameterindex)
    }
}
unsafe impl Send for ID3D12FunctionReflection {}
unsafe impl Sync for ID3D12FunctionReflection {}
#[repr(C)]
pub struct ID3D12FunctionReflection_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_FUNCTION_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDescByName: usize,
    pub GetFunctionParameter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D12FunctionParameterReflection>,
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList, ID3D12GraphicsCommandList_Vtbl, 0x5b160d0f_ac1b_4185_8ba8_b3ae42a5a455);
impl core::ops::Deref for ID3D12GraphicsCommandList {
    type Target = ID3D12CommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList);
impl ID3D12GraphicsCommandList {
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12CommandAllocator>,
        P1: windows_core::Param<ID3D12PipelineState>,
    {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), pallocator.param().abi(), pinitialstate.param().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: windows_core::Param<ID3D12PipelineState>,
    {
        (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self), ppipelinestate.param().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CopyBufferRegion)(windows_core::Interface::as_raw(self), pdstbuffer.param().abi(), dstoffset, psrcbuffer.param().abi(), srcoffset, numbytes)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: Option<*const D3D12_BOX>) {
        (windows_core::Interface::vtable(self).CopyTextureRegion)(windows_core::Interface::as_raw(self), pdst, dstx, dsty, dstz, psrc, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi())
    }
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CopyTiles)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.param().abi(), bufferstartoffsetinbytes, flags)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.len().try_into().unwrap(), core::mem::transmute(pviewports.as_ptr()))
    }
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.len().try_into().unwrap(), core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: Option<&[f32; 4]>) {
        (windows_core::Interface::vtable(self).OMSetBlendFactor)(windows_core::Interface::as_raw(self), core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (windows_core::Interface::vtable(self).OMSetStencilRef)(windows_core::Interface::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: windows_core::Param<ID3D12PipelineState>,
    {
        (windows_core::Interface::vtable(self).SetPipelineState)(windows_core::Interface::as_raw(self), ppipelinestate.param().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (windows_core::Interface::vtable(self).ResourceBarrier)(windows_core::Interface::as_raw(self), pbarriers.len().try_into().unwrap(), core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: windows_core::Param<ID3D12GraphicsCommandList>,
    {
        (windows_core::Interface::vtable(self).ExecuteBundle)(windows_core::Interface::as_raw(self), pcommandlist.param().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[Option<ID3D12DescriptorHeap>]) {
        (windows_core::Interface::vtable(self).SetDescriptorHeaps)(windows_core::Interface::as_raw(self), ppdescriptorheaps.len().try_into().unwrap(), core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: windows_core::Param<ID3D12RootSignature>,
    {
        (windows_core::Interface::vtable(self).SetComputeRootSignature)(windows_core::Interface::as_raw(self), prootsignature.param().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: windows_core::Param<ID3D12RootSignature>,
    {
        (windows_core::Interface::vtable(self).SetGraphicsRootSignature)(windows_core::Interface::as_raw(self), prootsignature.param().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (windows_core::Interface::vtable(self).SetComputeRootDescriptorTable)(windows_core::Interface::as_raw(self), rootparameterindex, core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (windows_core::Interface::vtable(self).SetGraphicsRootDescriptorTable)(windows_core::Interface::as_raw(self), rootparameterindex, core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (windows_core::Interface::vtable(self).SetComputeRoot32BitConstant)(windows_core::Interface::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (windows_core::Interface::vtable(self).SetGraphicsRoot32BitConstant)(windows_core::Interface::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (windows_core::Interface::vtable(self).SetComputeRoot32BitConstants)(windows_core::Interface::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (windows_core::Interface::vtable(self).SetGraphicsRoot32BitConstants)(windows_core::Interface::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetComputeRootConstantBufferView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetGraphicsRootConstantBufferView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetComputeRootShaderResourceView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetGraphicsRootShaderResourceView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetComputeRootUnorderedAccessView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (windows_core::Interface::vtable(self).SetGraphicsRootUnorderedAccessView)(windows_core::Interface::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pview.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), numrendertargetdescriptors, core::mem::transmute(prendertargetdescriptors.unwrap_or(std::ptr::null())), rtssinglehandletodescriptorrange.param().abi(), core::mem::transmute(pdepthstencildescriptor.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len().try_into().unwrap(), core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: &[f32; 4], prects: Option<&[super::super::Foundation::RECT]>) {
        (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), core::mem::transmute(rendertargetview), core::mem::transmute(colorrgba.as_ptr()), prects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: &[u32; 4], prects: &[super::super::Foundation::RECT])
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ClearUnorderedAccessViewUint)(windows_core::Interface::as_raw(self), core::mem::transmute(viewgpuhandleincurrentheap), core::mem::transmute(viewcpuhandle), presource.param().abi(), core::mem::transmute(values.as_ptr()), prects.len().try_into().unwrap(), core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: &[f32; 4], prects: &[super::super::Foundation::RECT])
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ClearUnorderedAccessViewFloat)(windows_core::Interface::as_raw(self), core::mem::transmute(viewgpuhandleincurrentheap), core::mem::transmute(viewcpuhandle), presource.param().abi(), core::mem::transmute(values.as_ptr()), prects.len().try_into().unwrap(), core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: Option<*const D3D12_DISCARD_REGION>)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi(), core::mem::transmute(pregion.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<ID3D12QueryHeap>,
    {
        (windows_core::Interface::vtable(self).BeginQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<ID3D12QueryHeap>,
    {
        (windows_core::Interface::vtable(self).EndQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: windows_core::Param<ID3D12QueryHeap>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ResolveQueryData)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, startindex, numqueries, pdestinationbuffer.param().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), pbuffer.param().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), metadata, core::mem::transmute(pdata.unwrap_or(std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), metadata, core::mem::transmute(pdata.unwrap_or(std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: windows_core::Param<ID3D12CommandSignature>,
        P1: windows_core::Param<ID3D12Resource>,
        P2: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ExecuteIndirect)(windows_core::Interface::as_raw(self), pcommandsignature.param().abi(), maxcommandcount, pargumentbuffer.param().abi(), argumentbufferoffset, pcountbuffer.param().abi(), countbufferoffset)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList {}
unsafe impl Sync for ID3D12GraphicsCommandList {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList_Vtbl {
    pub base__: ID3D12CommandList_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub CopyBufferRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut core::ffi::c_void, u64, u64),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CopyTextureRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_TEXTURE_COPY_LOCATION, u32, u32, u32, *const D3D12_TEXTURE_COPY_LOCATION, *const D3D12_BOX),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CopyTextureRegion: usize,
    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub CopyTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_TILED_RESOURCE_COORDINATE, *const D3D12_TILE_REGION_SIZE, *mut core::ffi::c_void, u64, D3D12_TILE_COPY_FLAGS),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::Dxgi::Common::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct3D::D3D_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    IASetPrimitiveTopology: usize,
    pub RSSetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_VIEWPORT),
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT),
    pub OMSetBlendFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32),
    pub OMSetStencilRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub SetPipelineState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub ResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_RESOURCE_BARRIER),
    pub ExecuteBundle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetDescriptorHeaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void),
    pub SetComputeRootSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetGraphicsRootSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetComputeRootDescriptorTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D12_GPU_DESCRIPTOR_HANDLE),
    pub SetGraphicsRootDescriptorTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D12_GPU_DESCRIPTOR_HANDLE),
    pub SetComputeRoot32BitConstant: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub SetGraphicsRoot32BitConstant: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub SetComputeRoot32BitConstants: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void, u32),
    pub SetGraphicsRoot32BitConstants: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void, u32),
    pub SetComputeRootConstantBufferView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    pub SetGraphicsRootConstantBufferView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    pub SetComputeRootShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    pub SetGraphicsRootShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    pub SetComputeRootUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    pub SetGraphicsRootUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IASetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_INDEX_BUFFER_VIEW),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IASetIndexBuffer: usize,
    pub IASetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const D3D12_VERTEX_BUFFER_VIEW),
    pub SOSetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const D3D12_STREAM_OUTPUT_BUFFER_VIEW),
    pub OMSetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_CPU_DESCRIPTOR_HANDLE, super::super::Foundation::BOOL, *const D3D12_CPU_DESCRIPTOR_HANDLE),
    pub ClearDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_CLEAR_FLAGS, f32, u8, u32, *const super::super::Foundation::RECT),
    pub ClearRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_CPU_DESCRIPTOR_HANDLE, *const f32, u32, *const super::super::Foundation::RECT),
    pub ClearUnorderedAccessViewUint: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_CPU_DESCRIPTOR_HANDLE, *mut core::ffi::c_void, *const u32, u32, *const super::super::Foundation::RECT),
    pub ClearUnorderedAccessViewFloat: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_CPU_DESCRIPTOR_HANDLE, *mut core::ffi::c_void, *const f32, u32, *const super::super::Foundation::RECT),
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_DISCARD_REGION),
    pub BeginQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D12_QUERY_TYPE, u32),
    pub EndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D12_QUERY_TYPE, u32),
    pub ResolveQueryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D12_QUERY_TYPE, u32, u32, *mut core::ffi::c_void, u64),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, D3D12_PREDICATION_OP),
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub ExecuteIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u64, *mut core::ffi::c_void, u64),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList1_Vtbl, 0x553103fb_1fe7_4557_bb38_946d7d0e7ca7);
impl core::ops::Deref for ID3D12GraphicsCommandList1 {
    type Target = ID3D12GraphicsCommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList);
impl ID3D12GraphicsCommandList1 {
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AtomicCopyBufferUINT)(windows_core::Interface::as_raw(self), pdstbuffer.param().abi(), dstoffset, psrcbuffer.param().abi(), srcoffset, dependencies, core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).AtomicCopyBufferUINT64)(windows_core::Interface::as_raw(self), pdstbuffer.param().abi(), dstoffset, psrcbuffer.param().abi(), srcoffset, dependencies, core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (windows_core::Interface::vtable(self).OMSetDepthBounds)(windows_core::Interface::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (windows_core::Interface::vtable(self).SetSamplePositions)(windows_core::Interface::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).ResolveSubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, psrcresource.param().abi(), srcsubresource, core::mem::transmute(psrcrect.unwrap_or(std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (windows_core::Interface::vtable(self).SetViewInstanceMask)(windows_core::Interface::as_raw(self), mask)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList1 {}
unsafe impl Sync for ID3D12GraphicsCommandList1 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList1_Vtbl {
    pub base__: ID3D12GraphicsCommandList_Vtbl,
    pub AtomicCopyBufferUINT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut core::ffi::c_void, u64, u32, *const *mut core::ffi::c_void, *const D3D12_SUBRESOURCE_RANGE_UINT64),
    pub AtomicCopyBufferUINT64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut core::ffi::c_void, u64, u32, *const *mut core::ffi::c_void, *const D3D12_SUBRESOURCE_RANGE_UINT64),
    pub OMSetDepthBounds: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub SetSamplePositions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const D3D12_SAMPLE_POSITION),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResolveSubresourceRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, u32, *const super::super::Foundation::RECT, super::Dxgi::Common::DXGI_FORMAT, D3D12_RESOLVE_MODE),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResolveSubresourceRegion: usize,
    pub SetViewInstanceMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList2_Vtbl, 0x38c3e585_ff17_412c_9150_4fc6f9d72a28);
impl core::ops::Deref for ID3D12GraphicsCommandList2 {
    type Target = ID3D12GraphicsCommandList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList2, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1);
impl ID3D12GraphicsCommandList2 {
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (windows_core::Interface::vtable(self).WriteBufferImmediate)(windows_core::Interface::as_raw(self), count, pparams, core::mem::transmute(pmodes.unwrap_or(std::ptr::null())))
    }
}
unsafe impl Send for ID3D12GraphicsCommandList2 {}
unsafe impl Sync for ID3D12GraphicsCommandList2 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList2_Vtbl {
    pub base__: ID3D12GraphicsCommandList1_Vtbl,
    pub WriteBufferImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, *const D3D12_WRITEBUFFERIMMEDIATE_MODE),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList3_Vtbl, 0x6fda83a7_b84c_4e38_9ac8_c7bd22016b3d);
impl core::ops::Deref for ID3D12GraphicsCommandList3 {
    type Target = ID3D12GraphicsCommandList2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList3, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2);
impl ID3D12GraphicsCommandList3 {
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: windows_core::Param<ID3D12ProtectedResourceSession>,
    {
        (windows_core::Interface::vtable(self).SetProtectedResourceSession)(windows_core::Interface::as_raw(self), pprotectedresourcesession.param().abi())
    }
}
unsafe impl Send for ID3D12GraphicsCommandList3 {}
unsafe impl Sync for ID3D12GraphicsCommandList3 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList3_Vtbl {
    pub base__: ID3D12GraphicsCommandList2_Vtbl,
    pub SetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList4_Vtbl, 0x8754318e_d3a9_4541_98cf_645b50dc4874);
impl core::ops::Deref for ID3D12GraphicsCommandList4 {
    type Target = ID3D12GraphicsCommandList3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList4, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3);
impl ID3D12GraphicsCommandList4 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn BeginRenderPass(&self, prendertargets: Option<&[D3D12_RENDER_PASS_RENDER_TARGET_DESC]>, pdepthstencil: Option<*const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC>, flags: D3D12_RENDER_PASS_FLAGS) {
        (windows_core::Interface::vtable(self).BeginRenderPass)(windows_core::Interface::as_raw(self), prendertargets.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prendertargets.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pdepthstencil.unwrap_or(std::ptr::null())), flags)
    }
    pub unsafe fn EndRenderPass(&self) {
        (windows_core::Interface::vtable(self).EndRenderPass)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn InitializeMetaCommand<P0>(&self, pmetacommand: P0, pinitializationparametersdata: Option<*const core::ffi::c_void>, initializationparametersdatasizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12MetaCommand>,
    {
        (windows_core::Interface::vtable(self).InitializeMetaCommand)(windows_core::Interface::as_raw(self), pmetacommand.param().abi(), core::mem::transmute(pinitializationparametersdata.unwrap_or(std::ptr::null())), initializationparametersdatasizeinbytes)
    }
    pub unsafe fn ExecuteMetaCommand<P0>(&self, pmetacommand: P0, pexecutionparametersdata: Option<*const core::ffi::c_void>, executionparametersdatasizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12MetaCommand>,
    {
        (windows_core::Interface::vtable(self).ExecuteMetaCommand)(windows_core::Interface::as_raw(self), pmetacommand.param().abi(), core::mem::transmute(pexecutionparametersdata.unwrap_or(std::ptr::null())), executionparametersdatasizeinbytes)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn BuildRaytracingAccelerationStructure(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, ppostbuildinfodescs: Option<&[D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC]>) {
        (windows_core::Interface::vtable(self).BuildRaytracingAccelerationStructure)(windows_core::Interface::as_raw(self), pdesc, ppostbuildinfodescs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppostbuildinfodescs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, psourceaccelerationstructuredata: &[u64]) {
        (windows_core::Interface::vtable(self).EmitRaytracingAccelerationStructurePostbuildInfo)(windows_core::Interface::as_raw(self), pdesc, psourceaccelerationstructuredata.len().try_into().unwrap(), core::mem::transmute(psourceaccelerationstructuredata.as_ptr()))
    }
    pub unsafe fn CopyRaytracingAccelerationStructure(&self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
        (windows_core::Interface::vtable(self).CopyRaytracingAccelerationStructure)(windows_core::Interface::as_raw(self), destaccelerationstructuredata, sourceaccelerationstructuredata, mode)
    }
    pub unsafe fn SetPipelineState1<P0>(&self, pstateobject: P0)
    where
        P0: windows_core::Param<ID3D12StateObject>,
    {
        (windows_core::Interface::vtable(self).SetPipelineState1)(windows_core::Interface::as_raw(self), pstateobject.param().abi())
    }
    pub unsafe fn DispatchRays(&self, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
        (windows_core::Interface::vtable(self).DispatchRays)(windows_core::Interface::as_raw(self), pdesc)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList4 {}
unsafe impl Sync for ID3D12GraphicsCommandList4 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList4_Vtbl {
    pub base__: ID3D12GraphicsCommandList3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub BeginRenderPass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, D3D12_RENDER_PASS_FLAGS),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    BeginRenderPass: usize,
    pub EndRenderPass: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub InitializeMetaCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
    pub ExecuteMetaCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub BuildRaytracingAccelerationStructure: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, u32, *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    BuildRaytracingAccelerationStructure: usize,
    pub EmitRaytracingAccelerationStructurePostbuildInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, u32, *const u64),
    pub CopyRaytracingAccelerationStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE),
    pub SetPipelineState1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DispatchRays: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_DISPATCH_RAYS_DESC),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList5, ID3D12GraphicsCommandList5_Vtbl, 0x55050859_4024_474c_87f5_6472eaee44ea);
impl core::ops::Deref for ID3D12GraphicsCommandList5 {
    type Target = ID3D12GraphicsCommandList4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList5, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4);
impl ID3D12GraphicsCommandList5 {
    pub unsafe fn RSSetShadingRate(&self, baseshadingrate: D3D12_SHADING_RATE, combiners: Option<*const D3D12_SHADING_RATE_COMBINER>) {
        (windows_core::Interface::vtable(self).RSSetShadingRate)(windows_core::Interface::as_raw(self), baseshadingrate, core::mem::transmute(combiners.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn RSSetShadingRateImage<P0>(&self, shadingrateimage: P0)
    where
        P0: windows_core::Param<ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).RSSetShadingRateImage)(windows_core::Interface::as_raw(self), shadingrateimage.param().abi())
    }
}
unsafe impl Send for ID3D12GraphicsCommandList5 {}
unsafe impl Sync for ID3D12GraphicsCommandList5 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList5_Vtbl {
    pub base__: ID3D12GraphicsCommandList4_Vtbl,
    pub RSSetShadingRate: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_SHADING_RATE, *const D3D12_SHADING_RATE_COMBINER),
    pub RSSetShadingRateImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList6, ID3D12GraphicsCommandList6_Vtbl, 0xc3827890_e548_4cfa_96cf_5689a9370f80);
impl core::ops::Deref for ID3D12GraphicsCommandList6 {
    type Target = ID3D12GraphicsCommandList5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList6, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList5);
impl ID3D12GraphicsCommandList6 {
    pub unsafe fn DispatchMesh(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (windows_core::Interface::vtable(self).DispatchMesh)(windows_core::Interface::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList6 {}
unsafe impl Sync for ID3D12GraphicsCommandList6 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList6_Vtbl {
    pub base__: ID3D12GraphicsCommandList5_Vtbl,
    pub DispatchMesh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList7, ID3D12GraphicsCommandList7_Vtbl, 0xdd171223_8b61_4769_90e3_160ccde4e2c1);
impl core::ops::Deref for ID3D12GraphicsCommandList7 {
    type Target = ID3D12GraphicsCommandList6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList7, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList5, ID3D12GraphicsCommandList6);
impl ID3D12GraphicsCommandList7 {
    pub unsafe fn Barrier(&self, pbarriergroups: &[D3D12_BARRIER_GROUP]) {
        (windows_core::Interface::vtable(self).Barrier)(windows_core::Interface::as_raw(self), pbarriergroups.len().try_into().unwrap(), core::mem::transmute(pbarriergroups.as_ptr()))
    }
}
unsafe impl Send for ID3D12GraphicsCommandList7 {}
unsafe impl Sync for ID3D12GraphicsCommandList7 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList7_Vtbl {
    pub base__: ID3D12GraphicsCommandList6_Vtbl,
    pub Barrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_BARRIER_GROUP),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList8, ID3D12GraphicsCommandList8_Vtbl, 0xee936ef9_599d_4d28_938e_23c4ad05ce51);
impl core::ops::Deref for ID3D12GraphicsCommandList8 {
    type Target = ID3D12GraphicsCommandList7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList8, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList5, ID3D12GraphicsCommandList6, ID3D12GraphicsCommandList7);
impl ID3D12GraphicsCommandList8 {
    pub unsafe fn OMSetFrontAndBackStencilRef(&self, frontstencilref: u32, backstencilref: u32) {
        (windows_core::Interface::vtable(self).OMSetFrontAndBackStencilRef)(windows_core::Interface::as_raw(self), frontstencilref, backstencilref)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList8 {}
unsafe impl Sync for ID3D12GraphicsCommandList8 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList8_Vtbl {
    pub base__: ID3D12GraphicsCommandList7_Vtbl,
    pub OMSetFrontAndBackStencilRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
}
windows_core::imp::define_interface!(ID3D12GraphicsCommandList9, ID3D12GraphicsCommandList9_Vtbl, 0x34ed2808_ffe6_4c2b_b11a_cabd2b0c59e1);
impl core::ops::Deref for ID3D12GraphicsCommandList9 {
    type Target = ID3D12GraphicsCommandList8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12GraphicsCommandList9, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12CommandList, ID3D12GraphicsCommandList, ID3D12GraphicsCommandList1, ID3D12GraphicsCommandList2, ID3D12GraphicsCommandList3, ID3D12GraphicsCommandList4, ID3D12GraphicsCommandList5, ID3D12GraphicsCommandList6, ID3D12GraphicsCommandList7, ID3D12GraphicsCommandList8);
impl ID3D12GraphicsCommandList9 {
    pub unsafe fn RSSetDepthBias(&self, depthbias: f32, depthbiasclamp: f32, slopescaleddepthbias: f32) {
        (windows_core::Interface::vtable(self).RSSetDepthBias)(windows_core::Interface::as_raw(self), depthbias, depthbiasclamp, slopescaleddepthbias)
    }
    pub unsafe fn IASetIndexBufferStripCutValue(&self, ibstripcutvalue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) {
        (windows_core::Interface::vtable(self).IASetIndexBufferStripCutValue)(windows_core::Interface::as_raw(self), ibstripcutvalue)
    }
}
unsafe impl Send for ID3D12GraphicsCommandList9 {}
unsafe impl Sync for ID3D12GraphicsCommandList9 {}
#[repr(C)]
pub struct ID3D12GraphicsCommandList9_Vtbl {
    pub base__: ID3D12GraphicsCommandList8_Vtbl,
    pub RSSetDepthBias: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32),
    pub IASetIndexBufferStripCutValue: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_INDEX_BUFFER_STRIP_CUT_VALUE),
}
windows_core::imp::define_interface!(ID3D12Heap, ID3D12Heap_Vtbl, 0x6b3b2502_6e51_45b3_90ee_9884265e8df3);
impl core::ops::Deref for ID3D12Heap {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Heap, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12Heap {
    pub unsafe fn GetDesc(&self) -> D3D12_HEAP_DESC {
        let mut result__: D3D12_HEAP_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12Heap {}
unsafe impl Sync for ID3D12Heap {}
#[repr(C)]
pub struct ID3D12Heap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_HEAP_DESC),
}
windows_core::imp::define_interface!(ID3D12Heap1, ID3D12Heap1_Vtbl, 0x572f7389_2168_49e3_9693_d6df5871bf6d);
impl core::ops::Deref for ID3D12Heap1 {
    type Target = ID3D12Heap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Heap1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Heap);
impl ID3D12Heap1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D12Heap1 {}
unsafe impl Sync for ID3D12Heap1 {}
#[repr(C)]
pub struct ID3D12Heap1_Vtbl {
    pub base__: ID3D12Heap_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12InfoQueue, ID3D12InfoQueue_Vtbl, 0x0742a90b_c387_483f_b946_30a7e4e61458);
impl core::ops::Deref for ID3D12InfoQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12InfoQueue, windows_core::IUnknown);
impl ID3D12InfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: Option<*mut D3D12_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
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
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn AddMessage<P0>(&self, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), category, severity, id, pdescription.param().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnCategory<P0>(&self, category: D3D12_MESSAGE_CATEGORY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), category, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnSeverity<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity, benable.param().abi()).ok()
    }
    pub unsafe fn SetBreakOnID<P0>(&self, id: D3D12_MESSAGE_ID, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), id, benable.param().abi()).ok()
    }
    pub unsafe fn GetBreakOnCategory(&self, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), category)
    }
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), severity)
    }
    pub unsafe fn GetBreakOnID(&self, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
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
unsafe impl Send for ID3D12InfoQueue {}
unsafe impl Sync for ID3D12InfoQueue {}
#[repr(C)]
pub struct ID3D12InfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut D3D12_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumStoredMessagesAllowedByRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_CATEGORY, D3D12_MESSAGE_SEVERITY, D3D12_MESSAGE_ID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_CATEGORY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_SEVERITY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_ID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D12InfoQueue1, ID3D12InfoQueue1_Vtbl, 0x2852dd88_b484_4c0c_b6b1_67168500e600);
impl core::ops::Deref for ID3D12InfoQueue1 {
    type Target = ID3D12InfoQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12InfoQueue1, windows_core::IUnknown, ID3D12InfoQueue);
impl ID3D12InfoQueue1 {
    pub unsafe fn RegisterMessageCallback(&self, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *mut core::ffi::c_void, pcallbackcookie: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterMessageCallback)(windows_core::Interface::as_raw(self), callbackfunc, callbackfilterflags, pcontext, pcallbackcookie).ok()
    }
    pub unsafe fn UnregisterMessageCallback(&self, callbackcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterMessageCallback)(windows_core::Interface::as_raw(self), callbackcookie).ok()
    }
}
unsafe impl Send for ID3D12InfoQueue1 {}
unsafe impl Sync for ID3D12InfoQueue1 {}
#[repr(C)]
pub struct ID3D12InfoQueue1_Vtbl {
    pub base__: ID3D12InfoQueue_Vtbl,
    pub RegisterMessageCallback: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12MessageFunc, D3D12_MESSAGE_CALLBACK_FLAGS, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnregisterMessageCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12LibraryReflection, ID3D12LibraryReflection_Vtbl, 0x8e349d19_54db_4a56_9dc9_119d87bdb804);
impl core::ops::Deref for ID3D12LibraryReflection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12LibraryReflection, windows_core::IUnknown);
impl ID3D12LibraryReflection {
    pub unsafe fn GetDesc(&self) -> windows_core::Result<D3D12_LIBRARY_DESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D12FunctionReflection> {
        (windows_core::Interface::vtable(self).GetFunctionByIndex)(windows_core::Interface::as_raw(self), functionindex)
    }
}
unsafe impl Send for ID3D12LibraryReflection {}
unsafe impl Sync for ID3D12LibraryReflection {}
#[repr(C)]
pub struct ID3D12LibraryReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_LIBRARY_DESC) -> windows_core::HRESULT,
    pub GetFunctionByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D12FunctionReflection>,
}
windows_core::imp::define_interface!(ID3D12LifetimeOwner, ID3D12LifetimeOwner_Vtbl, 0xe667af9f_cd56_4f46_83ce_032e595d70a8);
impl core::ops::Deref for ID3D12LifetimeOwner {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12LifetimeOwner, windows_core::IUnknown);
impl ID3D12LifetimeOwner {
    pub unsafe fn LifetimeStateUpdated(&self, newstate: D3D12_LIFETIME_STATE) {
        (windows_core::Interface::vtable(self).LifetimeStateUpdated)(windows_core::Interface::as_raw(self), newstate)
    }
}
unsafe impl Send for ID3D12LifetimeOwner {}
unsafe impl Sync for ID3D12LifetimeOwner {}
#[repr(C)]
pub struct ID3D12LifetimeOwner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LifetimeStateUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_LIFETIME_STATE),
}
windows_core::imp::define_interface!(ID3D12LifetimeTracker, ID3D12LifetimeTracker_Vtbl, 0x3fd03d36_4eb1_424a_a582_494ecb8ba813);
impl core::ops::Deref for ID3D12LifetimeTracker {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12LifetimeTracker, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12LifetimeTracker {
    pub unsafe fn DestroyOwnedObject<P0>(&self, pobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12DeviceChild>,
    {
        (windows_core::Interface::vtable(self).DestroyOwnedObject)(windows_core::Interface::as_raw(self), pobject.param().abi()).ok()
    }
}
unsafe impl Send for ID3D12LifetimeTracker {}
unsafe impl Sync for ID3D12LifetimeTracker {}
#[repr(C)]
pub struct ID3D12LifetimeTracker_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub DestroyOwnedObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12ManualWriteTrackingResource, ID3D12ManualWriteTrackingResource_Vtbl, 0x86ca3b85_49ad_4b6e_aed5_eddb18540f41);
impl core::ops::Deref for ID3D12ManualWriteTrackingResource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ManualWriteTrackingResource, windows_core::IUnknown);
impl ID3D12ManualWriteTrackingResource {
    pub unsafe fn TrackWrite(&self, subresource: u32, pwrittenrange: Option<*const D3D12_RANGE>) {
        (windows_core::Interface::vtable(self).TrackWrite)(windows_core::Interface::as_raw(self), subresource, core::mem::transmute(pwrittenrange.unwrap_or(std::ptr::null())))
    }
}
unsafe impl Send for ID3D12ManualWriteTrackingResource {}
unsafe impl Sync for ID3D12ManualWriteTrackingResource {}
#[repr(C)]
pub struct ID3D12ManualWriteTrackingResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TrackWrite: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_RANGE),
}
windows_core::imp::define_interface!(ID3D12MetaCommand, ID3D12MetaCommand_Vtbl, 0xdbb84c27_36ce_4fc9_b801_f048c46ac570);
impl core::ops::Deref for ID3D12MetaCommand {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12MetaCommand, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12MetaCommand {
    pub unsafe fn GetRequiredParameterResourceSize(&self, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
        (windows_core::Interface::vtable(self).GetRequiredParameterResourceSize)(windows_core::Interface::as_raw(self), stage, parameterindex)
    }
}
unsafe impl Send for ID3D12MetaCommand {}
unsafe impl Sync for ID3D12MetaCommand {}
#[repr(C)]
pub struct ID3D12MetaCommand_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub GetRequiredParameterResourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_META_COMMAND_PARAMETER_STAGE, u32) -> u64,
}
windows_core::imp::define_interface!(ID3D12Object, ID3D12Object_Vtbl, 0xc4fec28f_7966_4e95_9f94_f431cb56c3b8);
impl core::ops::Deref for ID3D12Object {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Object, windows_core::IUnknown);
impl ID3D12Object {
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
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
}
unsafe impl Send for ID3D12Object {}
unsafe impl Sync for ID3D12Object {}
#[repr(C)]
pub struct ID3D12Object_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Pageable, ID3D12Pageable_Vtbl, 0x63ee58fb_1268_4835_86da_f008ce62f0d6);
impl core::ops::Deref for ID3D12Pageable {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Pageable, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12Pageable {}
unsafe impl Send for ID3D12Pageable {}
unsafe impl Sync for ID3D12Pageable {}
#[repr(C)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D12PipelineLibrary, ID3D12PipelineLibrary_Vtbl, 0xc64226a8_9201_46af_b4cc_53fb9ff7414f);
impl core::ops::Deref for ID3D12PipelineLibrary {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12PipelineLibrary, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12PipelineLibrary {
    pub unsafe fn StorePipeline<P0, P1>(&self, pname: P0, ppipeline: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ID3D12PipelineState>,
    {
        (windows_core::Interface::vtable(self).StorePipeline)(windows_core::Interface::as_raw(self), pname.param().abi(), ppipeline.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn LoadGraphicsPipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).LoadGraphicsPipeline)(windows_core::Interface::as_raw(self), pname.param().abi(), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadComputePipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).LoadComputePipeline)(windows_core::Interface::as_raw(self), pname.param().abi(), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSerializedSize(&self) -> usize {
        (windows_core::Interface::vtable(self).GetSerializedSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Serialize(&self, pdata: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID3D12PipelineLibrary {}
unsafe impl Sync for ID3D12PipelineLibrary {}
#[repr(C)]
pub struct ID3D12PipelineLibrary_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub StorePipeline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub LoadGraphicsPipeline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    LoadGraphicsPipeline: usize,
    pub LoadComputePipeline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const D3D12_COMPUTE_PIPELINE_STATE_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12PipelineLibrary1, ID3D12PipelineLibrary1_Vtbl, 0x80eabf42_2568_4e5e_bd82_c37f86961dc3);
impl core::ops::Deref for ID3D12PipelineLibrary1 {
    type Target = ID3D12PipelineLibrary;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12PipelineLibrary1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12PipelineLibrary);
impl ID3D12PipelineLibrary1 {
    pub unsafe fn LoadPipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).LoadPipeline)(windows_core::Interface::as_raw(self), pname.param().abi(), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12PipelineLibrary1 {}
unsafe impl Sync for ID3D12PipelineLibrary1 {}
#[repr(C)]
pub struct ID3D12PipelineLibrary1_Vtbl {
    pub base__: ID3D12PipelineLibrary_Vtbl,
    pub LoadPipeline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const D3D12_PIPELINE_STATE_STREAM_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12PipelineState, ID3D12PipelineState_Vtbl, 0x765a30f3_f624_4c6f_a828_ace948622445);
impl core::ops::Deref for ID3D12PipelineState {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12PipelineState, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12PipelineState {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetCachedBlob(&self) -> windows_core::Result<super::Direct3D::ID3DBlob> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCachedBlob)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID3D12PipelineState {}
unsafe impl Sync for ID3D12PipelineState {}
#[repr(C)]
pub struct ID3D12PipelineState_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetCachedBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetCachedBlob: usize,
}
windows_core::imp::define_interface!(ID3D12ProtectedResourceSession, ID3D12ProtectedResourceSession_Vtbl, 0x6cd696f4_f289_40cc_8091_5a6c0a099c3d);
impl core::ops::Deref for ID3D12ProtectedResourceSession {
    type Target = ID3D12ProtectedSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ProtectedResourceSession, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12ProtectedSession);
impl ID3D12ProtectedResourceSession {
    pub unsafe fn GetDesc(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC {
        let mut result__: D3D12_PROTECTED_RESOURCE_SESSION_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12ProtectedResourceSession {}
unsafe impl Sync for ID3D12ProtectedResourceSession {}
#[repr(C)]
pub struct ID3D12ProtectedResourceSession_Vtbl {
    pub base__: ID3D12ProtectedSession_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC),
}
windows_core::imp::define_interface!(ID3D12ProtectedResourceSession1, ID3D12ProtectedResourceSession1_Vtbl, 0xd6f12dd6_76fb_406e_8961_4296eefc0409);
impl core::ops::Deref for ID3D12ProtectedResourceSession1 {
    type Target = ID3D12ProtectedResourceSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ProtectedResourceSession1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12ProtectedSession, ID3D12ProtectedResourceSession);
impl ID3D12ProtectedResourceSession1 {
    pub unsafe fn GetDesc1(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
        let mut result__: D3D12_PROTECTED_RESOURCE_SESSION_DESC1 = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12ProtectedResourceSession1 {}
unsafe impl Sync for ID3D12ProtectedResourceSession1 {}
#[repr(C)]
pub struct ID3D12ProtectedResourceSession1_Vtbl {
    pub base__: ID3D12ProtectedResourceSession_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1),
}
windows_core::imp::define_interface!(ID3D12ProtectedSession, ID3D12ProtectedSession_Vtbl, 0xa1533d18_0ac1_4084_85b9_89a96116806b);
impl core::ops::Deref for ID3D12ProtectedSession {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ProtectedSession, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12ProtectedSession {
    pub unsafe fn GetStatusFence<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetStatusFence)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetSessionStatus(&self) -> D3D12_PROTECTED_SESSION_STATUS {
        (windows_core::Interface::vtable(self).GetSessionStatus)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12ProtectedSession {}
unsafe impl Sync for ID3D12ProtectedSession {}
#[repr(C)]
pub struct ID3D12ProtectedSession_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub GetStatusFence: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSessionStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS,
}
windows_core::imp::define_interface!(ID3D12QueryHeap, ID3D12QueryHeap_Vtbl, 0x0d9658ae_ed45_469e_a61d_970ec583cab4);
impl core::ops::Deref for ID3D12QueryHeap {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12QueryHeap, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12QueryHeap {}
unsafe impl Send for ID3D12QueryHeap {}
unsafe impl Sync for ID3D12QueryHeap {}
#[repr(C)]
pub struct ID3D12QueryHeap_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
windows_core::imp::define_interface!(ID3D12Resource, ID3D12Resource_Vtbl, 0x696442be_a72e_4059_bc79_5b5c98040fad);
impl core::ops::Deref for ID3D12Resource {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Resource, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12Resource {
    pub unsafe fn Map(&self, subresource: u32, preadrange: Option<*const D3D12_RANGE>, ppdata: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, core::mem::transmute(preadrange.unwrap_or(std::ptr::null())), core::mem::transmute(ppdata.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32, pwrittenrange: Option<*const D3D12_RANGE>) {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource, core::mem::transmute(pwrittenrange.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> D3D12_RESOURCE_DESC {
        let mut result__: D3D12_RESOURCE_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUVirtualAddress(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetGPUVirtualAddress)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn WriteToSubresource(&self, dstsubresource: u32, pdstbox: Option<*const D3D12_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteToSubresource)(windows_core::Interface::as_raw(self), dstsubresource, core::mem::transmute(pdstbox.unwrap_or(std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch).ok()
    }
    pub unsafe fn ReadFromSubresource(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: Option<*const D3D12_BOX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadFromSubresource)(windows_core::Interface::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, srcsubresource, core::mem::transmute(psrcbox.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetHeapProperties(&self, pheapproperties: Option<*mut D3D12_HEAP_PROPERTIES>, pheapflags: Option<*mut D3D12_HEAP_FLAGS>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHeapProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(pheapproperties.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pheapflags.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
unsafe impl Send for ID3D12Resource {}
unsafe impl Sync for ID3D12Resource {}
#[repr(C)]
pub struct ID3D12Resource_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_RANGE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_RANGE),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub GetGPUVirtualAddress: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub WriteToSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_BOX, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReadFromSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, *const D3D12_BOX) -> windows_core::HRESULT,
    pub GetHeapProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_HEAP_PROPERTIES, *mut D3D12_HEAP_FLAGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Resource1, ID3D12Resource1_Vtbl, 0x9d5e227a_4430_4161_88b3_3eca6bb16e19);
impl core::ops::Deref for ID3D12Resource1 {
    type Target = ID3D12Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Resource1, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Resource);
impl ID3D12Resource1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl Send for ID3D12Resource1 {}
unsafe impl Sync for ID3D12Resource1 {}
#[repr(C)]
pub struct ID3D12Resource1_Vtbl {
    pub base__: ID3D12Resource_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Resource2, ID3D12Resource2_Vtbl, 0xbe36ec3b_ea85_4aeb_a45a_e9d76404a495);
impl core::ops::Deref for ID3D12Resource2 {
    type Target = ID3D12Resource1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Resource2, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable, ID3D12Resource, ID3D12Resource1);
impl ID3D12Resource2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self) -> D3D12_RESOURCE_DESC1 {
        let mut result__: D3D12_RESOURCE_DESC1 = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12Resource2 {}
unsafe impl Sync for ID3D12Resource2 {}
#[repr(C)]
pub struct ID3D12Resource2_Vtbl {
    pub base__: ID3D12Resource1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_RESOURCE_DESC1),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
}
windows_core::imp::define_interface!(ID3D12RootSignature, ID3D12RootSignature_Vtbl, 0xc54a6b66_72df_4ee8_8be5_a946a1429214);
impl core::ops::Deref for ID3D12RootSignature {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12RootSignature, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12RootSignature {}
unsafe impl Send for ID3D12RootSignature {}
unsafe impl Sync for ID3D12RootSignature {}
#[repr(C)]
pub struct ID3D12RootSignature_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
windows_core::imp::define_interface!(ID3D12RootSignatureDeserializer, ID3D12RootSignatureDeserializer_Vtbl, 0x34ab647b_3cc8_46ac_841b_c0965645c046);
impl core::ops::Deref for ID3D12RootSignatureDeserializer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12RootSignatureDeserializer, windows_core::IUnknown);
impl ID3D12RootSignatureDeserializer {
    pub unsafe fn GetRootSignatureDesc(&self) -> *mut D3D12_ROOT_SIGNATURE_DESC {
        (windows_core::Interface::vtable(self).GetRootSignatureDesc)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12RootSignatureDeserializer {}
unsafe impl Sync for ID3D12RootSignatureDeserializer {}
#[repr(C)]
pub struct ID3D12RootSignatureDeserializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRootSignatureDesc: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC,
}
windows_core::imp::define_interface!(ID3D12SDKConfiguration, ID3D12SDKConfiguration_Vtbl, 0xe9eb5314_33aa_42b2_a718_d77f58b1f1c7);
impl core::ops::Deref for ID3D12SDKConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12SDKConfiguration, windows_core::IUnknown);
impl ID3D12SDKConfiguration {
    pub unsafe fn SetSDKVersion<P0>(&self, sdkversion: u32, sdkpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).SetSDKVersion)(windows_core::Interface::as_raw(self), sdkversion, sdkpath.param().abi()).ok()
    }
}
unsafe impl Send for ID3D12SDKConfiguration {}
unsafe impl Sync for ID3D12SDKConfiguration {}
#[repr(C)]
pub struct ID3D12SDKConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSDKVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12SDKConfiguration1, ID3D12SDKConfiguration1_Vtbl, 0x8aaf9303_ad25_48b9_9a57_d9c37e009d9f);
impl core::ops::Deref for ID3D12SDKConfiguration1 {
    type Target = ID3D12SDKConfiguration;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12SDKConfiguration1, windows_core::IUnknown, ID3D12SDKConfiguration);
impl ID3D12SDKConfiguration1 {
    pub unsafe fn CreateDeviceFactory<P0, T>(&self, sdkversion: u32, sdkpath: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateDeviceFactory)(windows_core::Interface::as_raw(self), sdkversion, sdkpath.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FreeUnusedSDKs(&self) {
        (windows_core::Interface::vtable(self).FreeUnusedSDKs)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12SDKConfiguration1 {}
unsafe impl Sync for ID3D12SDKConfiguration1 {}
#[repr(C)]
pub struct ID3D12SDKConfiguration1_Vtbl {
    pub base__: ID3D12SDKConfiguration_Vtbl,
    pub CreateDeviceFactory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeUnusedSDKs: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID3D12ShaderCacheSession, ID3D12ShaderCacheSession_Vtbl, 0x28e2495d_0f64_4ae4_a6ec_129255dc49a8);
impl core::ops::Deref for ID3D12ShaderCacheSession {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ShaderCacheSession, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild);
impl ID3D12ShaderCacheSession {
    pub unsafe fn FindValue(&self, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *mut core::ffi::c_void, pvaluesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindValue)(windows_core::Interface::as_raw(self), pkey, keysize, pvalue, pvaluesize).ok()
    }
    pub unsafe fn StoreValue(&self, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *const core::ffi::c_void, valuesize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StoreValue)(windows_core::Interface::as_raw(self), pkey, keysize, pvalue, valuesize).ok()
    }
    pub unsafe fn SetDeleteOnDestroy(&self) {
        (windows_core::Interface::vtable(self).SetDeleteOnDestroy)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D12_SHADER_CACHE_SESSION_DESC {
        let mut result__: D3D12_SHADER_CACHE_SESSION_DESC = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID3D12ShaderCacheSession {}
unsafe impl Sync for ID3D12ShaderCacheSession {}
#[repr(C)]
pub struct ID3D12ShaderCacheSession_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
    pub FindValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StoreValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetDeleteOnDestroy: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_CACHE_SESSION_DESC),
}
windows_core::imp::define_interface!(ID3D12ShaderReflection, ID3D12ShaderReflection_Vtbl, 0x5a58797d_a72c_478d_8ba2_efc6b0efe88e);
impl core::ops::Deref for ID3D12ShaderReflection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12ShaderReflection, windows_core::IUnknown);
impl ID3D12ShaderReflection {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatchConstantParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc).ok()
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>
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
unsafe impl Send for ID3D12ShaderReflection {}
unsafe impl Sync for ID3D12ShaderReflection {}
#[repr(C)]
pub struct ID3D12ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPatchConstantParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPatchConstantParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
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
windows_core::imp::define_interface!(ID3D12ShaderReflectionConstantBuffer, ID3D12ShaderReflectionConstantBuffer_Vtbl);
impl ID3D12ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionVariable> {
        (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi())
    }
}
unsafe impl Send for ID3D12ShaderReflectionConstantBuffer {}
unsafe impl Sync for ID3D12ShaderReflectionConstantBuffer {}
#[repr(C)]
pub struct ID3D12ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
}
windows_core::imp::define_interface!(ID3D12ShaderReflectionType, ID3D12ShaderReflectionType_Vtbl);
impl ID3D12ShaderReflectionType {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionType>
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
        P0: windows_core::Param<ID3D12ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), ptype.param().abi()).ok()
    }
    pub unsafe fn GetSubType(&self) -> Option<ID3D12ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBaseClass(&self) -> Option<ID3D12ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetBaseClass)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetNumInterfaces(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetNumInterfaces)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D12ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetInterfaceByIndex)(windows_core::Interface::as_raw(self), uindex)
    }
    pub unsafe fn IsOfType<P0>(&self, ptype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).IsOfType)(windows_core::Interface::as_raw(self), ptype.param().abi()).ok()
    }
    pub unsafe fn ImplementsInterface<P0>(&self, pbase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D12ShaderReflectionType>,
    {
        (windows_core::Interface::vtable(self).ImplementsInterface)(windows_core::Interface::as_raw(self), pbase.param().abi()).ok()
    }
}
unsafe impl Send for ID3D12ShaderReflectionType {}
unsafe impl Sync for ID3D12ShaderReflectionType {}
#[repr(C)]
pub struct ID3D12ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetNumInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetInterfaceByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionType>,
    pub IsOfType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImplementsInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12ShaderReflectionVariable, ID3D12ShaderReflectionVariable_Vtbl);
impl ID3D12ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetType(&self) -> Option<ID3D12ShaderReflectionType> {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBuffer(&self) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetInterfaceSlot)(windows_core::Interface::as_raw(self), uarrayindex)
    }
}
unsafe impl Send for ID3D12ShaderReflectionVariable {}
unsafe impl Sync for ID3D12ShaderReflectionVariable {}
#[repr(C)]
pub struct ID3D12ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetInterfaceSlot: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
windows_core::imp::define_interface!(ID3D12SharingContract, ID3D12SharingContract_Vtbl, 0x0adf7d52_929c_4e61_addb_ffed30de66ef);
impl core::ops::Deref for ID3D12SharingContract {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12SharingContract, windows_core::IUnknown);
impl ID3D12SharingContract {
    pub unsafe fn Present<P0, P1>(&self, presource: P0, subresource: u32, window: P1)
    where
        P0: windows_core::Param<ID3D12Resource>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), presource.param().abi(), subresource, window.param().abi())
    }
    pub unsafe fn SharedFenceSignal<P0>(&self, pfence: P0, fencevalue: u64)
    where
        P0: windows_core::Param<ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).SharedFenceSignal)(windows_core::Interface::as_raw(self), pfence.param().abi(), fencevalue)
    }
    pub unsafe fn BeginCapturableWork(&self, guid: *const windows_core::GUID) {
        (windows_core::Interface::vtable(self).BeginCapturableWork)(windows_core::Interface::as_raw(self), guid)
    }
    pub unsafe fn EndCapturableWork(&self, guid: *const windows_core::GUID) {
        (windows_core::Interface::vtable(self).EndCapturableWork)(windows_core::Interface::as_raw(self), guid)
    }
}
unsafe impl Send for ID3D12SharingContract {}
unsafe impl Sync for ID3D12SharingContract {}
#[repr(C)]
pub struct ID3D12SharingContract_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::HWND),
    pub SharedFenceSignal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64),
    pub BeginCapturableWork: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID),
    pub EndCapturableWork: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID),
}
windows_core::imp::define_interface!(ID3D12StateObject, ID3D12StateObject_Vtbl, 0x47016943_fca8_4594_93ea_af258b55346d);
impl core::ops::Deref for ID3D12StateObject {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12StateObject, windows_core::IUnknown, ID3D12Object, ID3D12DeviceChild, ID3D12Pageable);
impl ID3D12StateObject {}
unsafe impl Send for ID3D12StateObject {}
unsafe impl Sync for ID3D12StateObject {}
#[repr(C)]
pub struct ID3D12StateObject_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
}
windows_core::imp::define_interface!(ID3D12StateObjectProperties, ID3D12StateObjectProperties_Vtbl, 0xde5fa827_9bf9_4f26_89ff_d7f56fde3860);
impl core::ops::Deref for ID3D12StateObjectProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12StateObjectProperties, windows_core::IUnknown);
impl ID3D12StateObjectProperties {
    pub unsafe fn GetShaderIdentifier<P0>(&self, pexportname: P0) -> *mut core::ffi::c_void
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetShaderIdentifier)(windows_core::Interface::as_raw(self), pexportname.param().abi())
    }
    pub unsafe fn GetShaderStackSize<P0>(&self, pexportname: P0) -> u64
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetShaderStackSize)(windows_core::Interface::as_raw(self), pexportname.param().abi())
    }
    pub unsafe fn GetPipelineStackSize(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetPipelineStackSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPipelineStackSize(&self, pipelinestacksizeinbytes: u64) {
        (windows_core::Interface::vtable(self).SetPipelineStackSize)(windows_core::Interface::as_raw(self), pipelinestacksizeinbytes)
    }
}
unsafe impl Send for ID3D12StateObjectProperties {}
unsafe impl Sync for ID3D12StateObjectProperties {}
#[repr(C)]
pub struct ID3D12StateObjectProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetShaderIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> *mut core::ffi::c_void,
    pub GetShaderStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> u64,
    pub GetPipelineStackSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub SetPipelineStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
}
windows_core::imp::define_interface!(ID3D12SwapChainAssistant, ID3D12SwapChainAssistant_Vtbl, 0xf1df64b6_57fd_49cd_8807_c0eb88b45c8f);
impl core::ops::Deref for ID3D12SwapChainAssistant {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12SwapChainAssistant, windows_core::IUnknown);
impl ID3D12SwapChainAssistant {
    pub unsafe fn GetLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLUID)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetSwapChainObject<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetSwapChainObject)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentResourceAndCommandQueue<T>(&self, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetCurrentResourceAndCommandQueue)(windows_core::Interface::as_raw(self), riidresource, ppvresource, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertImplicitSync(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertImplicitSync)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID3D12SwapChainAssistant {}
unsafe impl Sync for ID3D12SwapChainAssistant {}
#[repr(C)]
pub struct ID3D12SwapChainAssistant_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID),
    pub GetSwapChainObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentResourceAndCommandQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertImplicitSync: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID3D12Tools, ID3D12Tools_Vtbl, 0x7071e1f0_e84b_4b33_974f_12fa49de65c5);
impl core::ops::Deref for ID3D12Tools {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12Tools, windows_core::IUnknown);
impl ID3D12Tools {
    pub unsafe fn EnableShaderInstrumentation<P0>(&self, benable: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableShaderInstrumentation)(windows_core::Interface::as_raw(self), benable.param().abi())
    }
    pub unsafe fn ShaderInstrumentationEnabled(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).ShaderInstrumentationEnabled)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12Tools {}
unsafe impl Sync for ID3D12Tools {}
#[repr(C)]
pub struct ID3D12Tools_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableShaderInstrumentation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub ShaderInstrumentationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID3D12VersionedRootSignatureDeserializer, ID3D12VersionedRootSignatureDeserializer_Vtbl, 0x7f91ce67_090c_4bb7_b78e_ed8ff2e31da0);
impl core::ops::Deref for ID3D12VersionedRootSignatureDeserializer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VersionedRootSignatureDeserializer, windows_core::IUnknown);
impl ID3D12VersionedRootSignatureDeserializer {
    pub unsafe fn GetRootSignatureDescAtVersion(&self, converttoversion: D3D_ROOT_SIGNATURE_VERSION) -> windows_core::Result<*mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRootSignatureDescAtVersion)(windows_core::Interface::as_raw(self), converttoversion, &mut result__).map(|| result__)
    }
    pub unsafe fn GetUnconvertedRootSignatureDesc(&self) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        (windows_core::Interface::vtable(self).GetUnconvertedRootSignatureDesc)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID3D12VersionedRootSignatureDeserializer {}
unsafe impl Sync for ID3D12VersionedRootSignatureDeserializer {}
#[repr(C)]
pub struct ID3D12VersionedRootSignatureDeserializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRootSignatureDescAtVersion: unsafe extern "system" fn(*mut core::ffi::c_void, D3D_ROOT_SIGNATURE_VERSION, *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> windows_core::HRESULT,
    pub GetUnconvertedRootSignatureDesc: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC,
}
windows_core::imp::define_interface!(ID3D12VirtualizationGuestDevice, ID3D12VirtualizationGuestDevice_Vtbl, 0xbc66d368_7373_4943_8757_fc87dc79e476);
impl core::ops::Deref for ID3D12VirtualizationGuestDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VirtualizationGuestDevice, windows_core::IUnknown);
impl ID3D12VirtualizationGuestDevice {
    pub unsafe fn ShareWithHost<P0>(&self, pobject: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<ID3D12DeviceChild>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShareWithHost)(windows_core::Interface::as_raw(self), pobject.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateFenceFd<P0>(&self, pfence: P0, fencevalue: u64) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ID3D12Fence>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFenceFd)(windows_core::Interface::as_raw(self), pfence.param().abi(), fencevalue, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID3D12VirtualizationGuestDevice {}
unsafe impl Sync for ID3D12VirtualizationGuestDevice {}
#[repr(C)]
pub struct ID3D12VirtualizationGuestDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShareWithHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateFenceFd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut i32) -> windows_core::HRESULT,
}
pub const CLSID_D3D12Debug: windows_core::GUID = windows_core::GUID::from_u128(0xf2352aeb_dd84_49fe_b97b_a9dcfdcc1b4f);
pub const CLSID_D3D12DeviceFactory: windows_core::GUID = windows_core::GUID::from_u128(0x114863bf_c386_4aee_b39d_8f0bbb062955);
pub const CLSID_D3D12DeviceRemovedExtendedData: windows_core::GUID = windows_core::GUID::from_u128(0x4a75bbc4_9ff4_4ad8_9f18_abae84dc5ff2);
pub const CLSID_D3D12SDKConfiguration: windows_core::GUID = windows_core::GUID::from_u128(0x7cda6aca_a03e_49c8_9458_0334d20e07ce);
pub const CLSID_D3D12Tools: windows_core::GUID = windows_core::GUID::from_u128(0xe38216b1_3c8c_4833_aa09_0a06b65d96c8);
pub const D3D12ExperimentalShaderModels: windows_core::GUID = windows_core::GUID::from_u128(0x76f5573e_f13a_40f5_b297_81ce9e18933f);
pub const D3D12TiledResourceTier4: windows_core::GUID = windows_core::GUID::from_u128(0xc9c4725f_a81a_4f56_8c5b_c51039d694fb);
pub const D3D12_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D12_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D12_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D12_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D12_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D12_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(23i32);
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT64: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(24i32);
pub const D3D12_AUTO_BREADCRUMB_OP_BARRIER: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(45i32);
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINEVENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(1i32);
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINSUBMISSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(19i32);
pub const D3D12_AUTO_BREADCRUMB_OP_BEGIN_COMMAND_LIST: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(46i32);
pub const D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(31i32);
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARDEPTHSTENCILVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(14i32);
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARRENDERTARGETVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(12i32);
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(13i32);
pub const D3D12_AUTO_BREADCRUMB_OP_COPYBUFFERREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(7i32);
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(33i32);
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRESOURCE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(9i32);
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTEXTUREREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(8i32);
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTILES: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(10i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(21i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(27i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME2: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(29i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCH: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(6i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHMESH: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(42i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHRAYS: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(34i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINDEXEDINSTANCED: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(4i32);
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINSTANCED: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(3i32);
pub const D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(32i32);
pub const D3D12_AUTO_BREADCRUMB_OP_ENCODEFRAME: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(43i32);
pub const D3D12_AUTO_BREADCRUMB_OP_ENDEVENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(2i32);
pub const D3D12_AUTO_BREADCRUMB_OP_ENDSUBMISSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(20i32);
pub const D3D12_AUTO_BREADCRUMB_OP_ESTIMATEMOTION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(37i32);
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEBUNDLE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(16i32);
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEEXTENSIONCOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(41i32);
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEINDIRECT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(5i32);
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEMETACOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(36i32);
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEEXTENSIONCOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(40i32);
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEMETACOMMAND: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(35i32);
pub const D3D12_AUTO_BREADCRUMB_OP_PRESENT: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(17i32);
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(22i32);
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(30i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEENCODEROUTPUTMETADATA: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(44i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEMOTIONVECTORHEAP: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(38i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEQUERYDATA: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(18i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(11i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCEREGION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(25i32);
pub const D3D12_AUTO_BREADCRUMB_OP_RESOURCEBARRIER: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(15i32);
pub const D3D12_AUTO_BREADCRUMB_OP_SETMARKER: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(0i32);
pub const D3D12_AUTO_BREADCRUMB_OP_SETPIPELINESTATE1: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(39i32);
pub const D3D12_AUTO_BREADCRUMB_OP_SETPROTECTEDRESOURCESESSION: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(28i32);
pub const D3D12_AUTO_BREADCRUMB_OP_WRITEBUFFERIMMEDIATE: D3D12_AUTO_BREADCRUMB_OP = D3D12_AUTO_BREADCRUMB_OP(26i32);
pub const D3D12_AXIS_SHADING_RATE_1X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(0i32);
pub const D3D12_AXIS_SHADING_RATE_2X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(1i32);
pub const D3D12_AXIS_SHADING_RATE_4X: D3D12_AXIS_SHADING_RATE = D3D12_AXIS_SHADING_RATE(2i32);
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOWED: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(0i32);
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOW_INTRUSIVE_MEASUREMENTS: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(1i32);
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_BACKGROUND_WORK: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(2i32);
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_PROFILING_BY_SYSTEM: D3D12_BACKGROUND_PROCESSING_MODE = D3D12_BACKGROUND_PROCESSING_MODE(3i32);
pub const D3D12_BARRIER_ACCESS_COMMON: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(0i32);
pub const D3D12_BARRIER_ACCESS_CONSTANT_BUFFER: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(2i32);
pub const D3D12_BARRIER_ACCESS_COPY_DEST: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(1024i32);
pub const D3D12_BARRIER_ACCESS_COPY_SOURCE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(2048i32);
pub const D3D12_BARRIER_ACCESS_DEPTH_STENCIL_READ: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(64i32);
pub const D3D12_BARRIER_ACCESS_DEPTH_STENCIL_WRITE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(32i32);
pub const D3D12_BARRIER_ACCESS_INDEX_BUFFER: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(4i32);
pub const D3D12_BARRIER_ACCESS_INDIRECT_ARGUMENT: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(512i32);
pub const D3D12_BARRIER_ACCESS_NO_ACCESS: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(-2147483648i32);
pub const D3D12_BARRIER_ACCESS_PREDICATION: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(512i32);
pub const D3D12_BARRIER_ACCESS_RAYTRACING_ACCELERATION_STRUCTURE_READ: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(16384i32);
pub const D3D12_BARRIER_ACCESS_RAYTRACING_ACCELERATION_STRUCTURE_WRITE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(32768i32);
pub const D3D12_BARRIER_ACCESS_RENDER_TARGET: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(8i32);
pub const D3D12_BARRIER_ACCESS_RESOLVE_DEST: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(4096i32);
pub const D3D12_BARRIER_ACCESS_RESOLVE_SOURCE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(8192i32);
pub const D3D12_BARRIER_ACCESS_SHADER_RESOURCE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(128i32);
pub const D3D12_BARRIER_ACCESS_SHADING_RATE_SOURCE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(65536i32);
pub const D3D12_BARRIER_ACCESS_STREAM_OUTPUT: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(256i32);
pub const D3D12_BARRIER_ACCESS_UNORDERED_ACCESS: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(16i32);
pub const D3D12_BARRIER_ACCESS_VERTEX_BUFFER: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(1i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_DECODE_READ: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(131072i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_DECODE_WRITE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(262144i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_ENCODE_READ: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(2097152i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_ENCODE_WRITE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(4194304i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_PROCESS_READ: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(524288i32);
pub const D3D12_BARRIER_ACCESS_VIDEO_PROCESS_WRITE: D3D12_BARRIER_ACCESS = D3D12_BARRIER_ACCESS(1048576i32);
pub const D3D12_BARRIER_LAYOUT_COMMON: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(0i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COMMON: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(24i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_DEST: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(29i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_SOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(28i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_GENERIC_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(25i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_SHADER_RESOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(27i32);
pub const D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_UNORDERED_ACCESS: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(26i32);
pub const D3D12_BARRIER_LAYOUT_COPY_DEST: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(8i32);
pub const D3D12_BARRIER_LAYOUT_COPY_SOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(7i32);
pub const D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(5i32);
pub const D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_WRITE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(4i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COMMON: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(18i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_DEST: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(23i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_SOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(22i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_GENERIC_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(19i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_SHADER_RESOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(21i32);
pub const D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_UNORDERED_ACCESS: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(20i32);
pub const D3D12_BARRIER_LAYOUT_GENERIC_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(1i32);
pub const D3D12_BARRIER_LAYOUT_PRESENT: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(0i32);
pub const D3D12_BARRIER_LAYOUT_RENDER_TARGET: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(2i32);
pub const D3D12_BARRIER_LAYOUT_RESOLVE_DEST: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(10i32);
pub const D3D12_BARRIER_LAYOUT_RESOLVE_SOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(9i32);
pub const D3D12_BARRIER_LAYOUT_SHADER_RESOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(6i32);
pub const D3D12_BARRIER_LAYOUT_SHADING_RATE_SOURCE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(11i32);
pub const D3D12_BARRIER_LAYOUT_UNDEFINED: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(-1i32);
pub const D3D12_BARRIER_LAYOUT_UNORDERED_ACCESS: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(3i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_DECODE_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(12i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_DECODE_WRITE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(13i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_ENCODE_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(16i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_ENCODE_WRITE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(17i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_PROCESS_READ: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(14i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_PROCESS_WRITE: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(15i32);
pub const D3D12_BARRIER_LAYOUT_VIDEO_QUEUE_COMMON: D3D12_BARRIER_LAYOUT = D3D12_BARRIER_LAYOUT(30i32);
pub const D3D12_BARRIER_SYNC_ALL: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(1i32);
pub const D3D12_BARRIER_SYNC_ALL_SHADING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(4096i32);
pub const D3D12_BARRIER_SYNC_BUILD_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(8388608i32);
pub const D3D12_BARRIER_SYNC_CLEAR_UNORDERED_ACCESS_VIEW: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(32768i32);
pub const D3D12_BARRIER_SYNC_COMPUTE_SHADING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(128i32);
pub const D3D12_BARRIER_SYNC_COPY: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(512i32);
pub const D3D12_BARRIER_SYNC_COPY_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(16777216i32);
pub const D3D12_BARRIER_SYNC_DEPTH_STENCIL: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(32i32);
pub const D3D12_BARRIER_SYNC_DRAW: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(2i32);
pub const D3D12_BARRIER_SYNC_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(16384i32);
pub const D3D12_BARRIER_SYNC_EXECUTE_INDIRECT: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(2048i32);
pub const D3D12_BARRIER_SYNC_INDEX_INPUT: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(4i32);
pub const D3D12_BARRIER_SYNC_NONE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(0i32);
pub const D3D12_BARRIER_SYNC_NON_PIXEL_SHADING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(8192i32);
pub const D3D12_BARRIER_SYNC_PIXEL_SHADING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(16i32);
pub const D3D12_BARRIER_SYNC_PREDICATION: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(2048i32);
pub const D3D12_BARRIER_SYNC_RAYTRACING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(256i32);
pub const D3D12_BARRIER_SYNC_RENDER_TARGET: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(64i32);
pub const D3D12_BARRIER_SYNC_RESOLVE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(1024i32);
pub const D3D12_BARRIER_SYNC_SPLIT: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(-2147483648i32);
pub const D3D12_BARRIER_SYNC_VERTEX_SHADING: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(8i32);
pub const D3D12_BARRIER_SYNC_VIDEO_DECODE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(1048576i32);
pub const D3D12_BARRIER_SYNC_VIDEO_ENCODE: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(4194304i32);
pub const D3D12_BARRIER_SYNC_VIDEO_PROCESS: D3D12_BARRIER_SYNC = D3D12_BARRIER_SYNC(2097152i32);
pub const D3D12_BARRIER_TYPE_BUFFER: D3D12_BARRIER_TYPE = D3D12_BARRIER_TYPE(2i32);
pub const D3D12_BARRIER_TYPE_GLOBAL: D3D12_BARRIER_TYPE = D3D12_BARRIER_TYPE(0i32);
pub const D3D12_BARRIER_TYPE_TEXTURE: D3D12_BARRIER_TYPE = D3D12_BARRIER_TYPE(1i32);
pub const D3D12_BLEND_ALPHA_FACTOR: D3D12_BLEND = D3D12_BLEND(20i32);
pub const D3D12_BLEND_BLEND_FACTOR: D3D12_BLEND = D3D12_BLEND(14i32);
pub const D3D12_BLEND_DEST_ALPHA: D3D12_BLEND = D3D12_BLEND(7i32);
pub const D3D12_BLEND_DEST_COLOR: D3D12_BLEND = D3D12_BLEND(9i32);
pub const D3D12_BLEND_INV_ALPHA_FACTOR: D3D12_BLEND = D3D12_BLEND(21i32);
pub const D3D12_BLEND_INV_BLEND_FACTOR: D3D12_BLEND = D3D12_BLEND(15i32);
pub const D3D12_BLEND_INV_DEST_ALPHA: D3D12_BLEND = D3D12_BLEND(8i32);
pub const D3D12_BLEND_INV_DEST_COLOR: D3D12_BLEND = D3D12_BLEND(10i32);
pub const D3D12_BLEND_INV_SRC1_ALPHA: D3D12_BLEND = D3D12_BLEND(19i32);
pub const D3D12_BLEND_INV_SRC1_COLOR: D3D12_BLEND = D3D12_BLEND(17i32);
pub const D3D12_BLEND_INV_SRC_ALPHA: D3D12_BLEND = D3D12_BLEND(6i32);
pub const D3D12_BLEND_INV_SRC_COLOR: D3D12_BLEND = D3D12_BLEND(4i32);
pub const D3D12_BLEND_ONE: D3D12_BLEND = D3D12_BLEND(2i32);
pub const D3D12_BLEND_OP_ADD: D3D12_BLEND_OP = D3D12_BLEND_OP(1i32);
pub const D3D12_BLEND_OP_MAX: D3D12_BLEND_OP = D3D12_BLEND_OP(5i32);
pub const D3D12_BLEND_OP_MIN: D3D12_BLEND_OP = D3D12_BLEND_OP(4i32);
pub const D3D12_BLEND_OP_REV_SUBTRACT: D3D12_BLEND_OP = D3D12_BLEND_OP(3i32);
pub const D3D12_BLEND_OP_SUBTRACT: D3D12_BLEND_OP = D3D12_BLEND_OP(2i32);
pub const D3D12_BLEND_SRC1_ALPHA: D3D12_BLEND = D3D12_BLEND(18i32);
pub const D3D12_BLEND_SRC1_COLOR: D3D12_BLEND = D3D12_BLEND(16i32);
pub const D3D12_BLEND_SRC_ALPHA: D3D12_BLEND = D3D12_BLEND(5i32);
pub const D3D12_BLEND_SRC_ALPHA_SAT: D3D12_BLEND = D3D12_BLEND(11i32);
pub const D3D12_BLEND_SRC_COLOR: D3D12_BLEND = D3D12_BLEND(3i32);
pub const D3D12_BLEND_ZERO: D3D12_BLEND = D3D12_BLEND(1i32);
pub const D3D12_BUFFER_SRV_FLAG_NONE: D3D12_BUFFER_SRV_FLAGS = D3D12_BUFFER_SRV_FLAGS(0i32);
pub const D3D12_BUFFER_SRV_FLAG_RAW: D3D12_BUFFER_SRV_FLAGS = D3D12_BUFFER_SRV_FLAGS(1i32);
pub const D3D12_BUFFER_UAV_FLAG_NONE: D3D12_BUFFER_UAV_FLAGS = D3D12_BUFFER_UAV_FLAGS(0i32);
pub const D3D12_BUFFER_UAV_FLAG_RAW: D3D12_BUFFER_UAV_FLAGS = D3D12_BUFFER_UAV_FLAGS(1i32);
pub const D3D12_CLEAR_FLAG_DEPTH: D3D12_CLEAR_FLAGS = D3D12_CLEAR_FLAGS(1i32);
pub const D3D12_CLEAR_FLAG_STENCIL: D3D12_CLEAR_FLAGS = D3D12_CLEAR_FLAGS(2i32);
pub const D3D12_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D12_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub const D3D12_COLOR_WRITE_ENABLE_ALL: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(15i32);
pub const D3D12_COLOR_WRITE_ENABLE_ALPHA: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(8i32);
pub const D3D12_COLOR_WRITE_ENABLE_BLUE: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(4i32);
pub const D3D12_COLOR_WRITE_ENABLE_GREEN: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(2i32);
pub const D3D12_COLOR_WRITE_ENABLE_RED: D3D12_COLOR_WRITE_ENABLE = D3D12_COLOR_WRITE_ENABLE(1i32);
pub const D3D12_COMMAND_LIST_FLAG_NONE: D3D12_COMMAND_LIST_FLAGS = D3D12_COMMAND_LIST_FLAGS(0i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(2i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(4i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(8i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(1i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(0i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(16i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(64i32);
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS: D3D12_COMMAND_LIST_SUPPORT_FLAGS = D3D12_COMMAND_LIST_SUPPORT_FLAGS(32i32);
pub const D3D12_COMMAND_LIST_TYPE_BUNDLE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(1i32);
pub const D3D12_COMMAND_LIST_TYPE_COMPUTE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(2i32);
pub const D3D12_COMMAND_LIST_TYPE_COPY: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(3i32);
pub const D3D12_COMMAND_LIST_TYPE_DIRECT: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(0i32);
pub const D3D12_COMMAND_LIST_TYPE_NONE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(-1i32);
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(4i32);
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(6i32);
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE(5i32);
pub const D3D12_COMMAND_POOL_FLAG_NONE: D3D12_COMMAND_POOL_FLAGS = D3D12_COMMAND_POOL_FLAGS(0i32);
pub const D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT: D3D12_COMMAND_QUEUE_FLAGS = D3D12_COMMAND_QUEUE_FLAGS(1i32);
pub const D3D12_COMMAND_QUEUE_FLAG_NONE: D3D12_COMMAND_QUEUE_FLAGS = D3D12_COMMAND_QUEUE_FLAGS(0i32);
pub const D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(10000i32);
pub const D3D12_COMMAND_QUEUE_PRIORITY_HIGH: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(100i32);
pub const D3D12_COMMAND_QUEUE_PRIORITY_NORMAL: D3D12_COMMAND_QUEUE_PRIORITY = D3D12_COMMAND_QUEUE_PRIORITY(0i32);
pub const D3D12_COMMAND_RECORDER_FLAG_NONE: D3D12_COMMAND_RECORDER_FLAGS = D3D12_COMMAND_RECORDER_FLAGS(0i32);
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D12_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D12_COMPARISON_FUNC_ALWAYS: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(8i32);
pub const D3D12_COMPARISON_FUNC_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(3i32);
pub const D3D12_COMPARISON_FUNC_GREATER: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(5i32);
pub const D3D12_COMPARISON_FUNC_GREATER_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(7i32);
pub const D3D12_COMPARISON_FUNC_LESS: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(2i32);
pub const D3D12_COMPARISON_FUNC_LESS_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(4i32);
pub const D3D12_COMPARISON_FUNC_NEVER: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(1i32);
pub const D3D12_COMPARISON_FUNC_NONE: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(0i32);
pub const D3D12_COMPARISON_FUNC_NOT_EQUAL: D3D12_COMPARISON_FUNC = D3D12_COMPARISON_FUNC(6i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF: D3D12_CONSERVATIVE_RASTERIZATION_MODE = D3D12_CONSERVATIVE_RASTERIZATION_MODE(0i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON: D3D12_CONSERVATIVE_RASTERIZATION_MODE = D3D12_CONSERVATIVE_RASTERIZATION_MODE(1i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_1: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(1i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_2: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(2i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_3: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(3i32);
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED: D3D12_CONSERVATIVE_RASTERIZATION_TIER = D3D12_CONSERVATIVE_RASTERIZATION_TIER(0i32);
pub const D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: u32 = 256u32;
pub const D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(1i32);
pub const D3D12_CPU_PAGE_PROPERTY_UNKNOWN: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(0i32);
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_BACK: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(3i32);
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE: D3D12_CPU_PAGE_PROPERTY = D3D12_CPU_PAGE_PROPERTY(2i32);
pub const D3D12_CROSS_NODE_SHARING_TIER_1: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(2i32);
pub const D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(1i32);
pub const D3D12_CROSS_NODE_SHARING_TIER_2: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(3i32);
pub const D3D12_CROSS_NODE_SHARING_TIER_3: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(4i32);
pub const D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED: D3D12_CROSS_NODE_SHARING_TIER = D3D12_CROSS_NODE_SHARING_TIER(0i32);
pub const D3D12_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
pub const D3D12_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
pub const D3D12_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
pub const D3D12_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
pub const D3D12_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
pub const D3D12_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
pub const D3D12_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
pub const D3D12_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
pub const D3D12_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
pub const D3D12_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
pub const D3D12_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
pub const D3D12_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
pub const D3D12_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
pub const D3D12_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
pub const D3D12_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
pub const D3D12_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
pub const D3D12_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
pub const D3D12_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
pub const D3D12_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
pub const D3D12_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
pub const D3D12_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
pub const D3D12_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
pub const D3D12_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
pub const D3D12_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
pub const D3D12_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
pub const D3D12_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
pub const D3D12_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
pub const D3D12_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
pub const D3D12_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
pub const D3D12_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
pub const D3D12_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
pub const D3D12_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D12_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
pub const D3D12_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
pub const D3D12_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
pub const D3D12_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
pub const D3D12_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
pub const D3D12_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
pub const D3D12_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub const D3D12_CULL_MODE_BACK: D3D12_CULL_MODE = D3D12_CULL_MODE(3i32);
pub const D3D12_CULL_MODE_FRONT: D3D12_CULL_MODE = D3D12_CULL_MODE(2i32);
pub const D3D12_CULL_MODE_NONE: D3D12_CULL_MODE = D3D12_CULL_MODE(1i32);
pub const D3D12_DEBUG_COMMAND_LIST_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE = D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE(0i32);
pub const D3D12_DEBUG_DEVICE_PARAMETER_FEATURE_FLAGS: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(0i32);
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(1i32);
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_SLOWDOWN_PERFORMANCE_FACTOR: D3D12_DEBUG_DEVICE_PARAMETER_TYPE = D3D12_DEBUG_DEVICE_PARAMETER_TYPE(2i32);
pub const D3D12_DEBUG_FEATURE_ALLOW_BEHAVIOR_CHANGING_DEBUG_AIDS: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(1i32);
pub const D3D12_DEBUG_FEATURE_CONSERVATIVE_RESOURCE_STATE_TRACKING: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(2i32);
pub const D3D12_DEBUG_FEATURE_DISABLE_VIRTUALIZED_BUNDLES_VALIDATION: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(4i32);
pub const D3D12_DEBUG_FEATURE_EMULATE_WINDOWS7: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(8i32);
pub const D3D12_DEBUG_FEATURE_NONE: D3D12_DEBUG_FEATURE = D3D12_DEBUG_FEATURE(0i32);
pub const D3D12_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D12_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D12_DEFAULT_DEPTH_BIAS: i32 = 0i32;
pub const D3D12_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D12_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
pub const D3D12_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4194304u32;
pub const D3D12_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
pub const D3D12_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D12_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 5768u32;
pub const D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D12_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D12_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D12_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D12_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D12_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
pub const D3D12_DEPTH_WRITE_MASK_ALL: D3D12_DEPTH_WRITE_MASK = D3D12_DEPTH_WRITE_MASK(1i32);
pub const D3D12_DEPTH_WRITE_MASK_ZERO: D3D12_DEPTH_WRITE_MASK = D3D12_DEPTH_WRITE_MASK(0i32);
pub const D3D12_DESCRIPTOR_HEAP_FLAG_NONE: D3D12_DESCRIPTOR_HEAP_FLAGS = D3D12_DESCRIPTOR_HEAP_FLAGS(0i32);
pub const D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE: D3D12_DESCRIPTOR_HEAP_FLAGS = D3D12_DESCRIPTOR_HEAP_FLAGS(1i32);
pub const D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(0i32);
pub const D3D12_DESCRIPTOR_HEAP_TYPE_DSV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(3i32);
pub const D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(4i32);
pub const D3D12_DESCRIPTOR_HEAP_TYPE_RTV: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(2i32);
pub const D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE(1i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(8i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(4i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_VOLATILE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(2i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(65536i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(1i32);
pub const D3D12_DESCRIPTOR_RANGE_FLAG_NONE: D3D12_DESCRIPTOR_RANGE_FLAGS = D3D12_DESCRIPTOR_RANGE_FLAGS(0i32);
pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = 4294967295u32;
pub const D3D12_DESCRIPTOR_RANGE_TYPE_CBV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(2i32);
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(3i32);
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SRV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(0i32);
pub const D3D12_DESCRIPTOR_RANGE_TYPE_UAV: D3D12_DESCRIPTOR_RANGE_TYPE = D3D12_DESCRIPTOR_RANGE_TYPE(1i32);
pub const D3D12_DEVICE_FACTORY_FLAG_ALLOW_RETURNING_EXISTING_DEVICE: D3D12_DEVICE_FACTORY_FLAGS = D3D12_DEVICE_FACTORY_FLAGS(1i32);
pub const D3D12_DEVICE_FACTORY_FLAG_ALLOW_RETURNING_INCOMPATIBLE_EXISTING_DEVICE: D3D12_DEVICE_FACTORY_FLAGS = D3D12_DEVICE_FACTORY_FLAGS(2i32);
pub const D3D12_DEVICE_FACTORY_FLAG_DISALLOW_STORING_NEW_DEVICE_AS_SINGLETON: D3D12_DEVICE_FACTORY_FLAGS = D3D12_DEVICE_FACTORY_FLAGS(4i32);
pub const D3D12_DEVICE_FACTORY_FLAG_NONE: D3D12_DEVICE_FACTORY_FLAGS = D3D12_DEVICE_FACTORY_FLAGS(0i32);
pub const D3D12_DEVICE_FLAG_AUTO_DEBUG_NAME_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(512i32);
pub const D3D12_DEVICE_FLAG_DEBUG_LAYER_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(1i32);
pub const D3D12_DEVICE_FLAG_DRED_AUTO_BREADCRUMBS_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(8i32);
pub const D3D12_DEVICE_FLAG_DRED_BREADCRUMB_CONTEXT_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(64i32);
pub const D3D12_DEVICE_FLAG_DRED_PAGE_FAULT_REPORTING_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(16i32);
pub const D3D12_DEVICE_FLAG_DRED_USE_MARKERS_ONLY_BREADCRUMBS: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(128i32);
pub const D3D12_DEVICE_FLAG_DRED_WATSON_REPORTING_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(32i32);
pub const D3D12_DEVICE_FLAG_FORCE_LEGACY_STATE_VALIDATION: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(1024i32);
pub const D3D12_DEVICE_FLAG_GPU_BASED_VALIDATION_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(2i32);
pub const D3D12_DEVICE_FLAG_NONE: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(0i32);
pub const D3D12_DEVICE_FLAG_SHADER_INSTRUMENTATION_ENABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(256i32);
pub const D3D12_DEVICE_FLAG_SYNCHRONIZED_COMMAND_QUEUE_VALIDATION_DISABLED: D3D12_DEVICE_FLAGS = D3D12_DEVICE_FLAGS(4i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_ALLOCATOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(20i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_LIST: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(22i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_POOL: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(40i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_QUEUE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(19i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_RECORDER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(41i32);
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_SIGNATURE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(28i32);
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSION: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(36i32);
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSIONPOLICY: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(37i32);
pub const D3D12_DRED_ALLOCATION_TYPE_DESCRIPTOR_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(24i32);
pub const D3D12_DRED_ALLOCATION_TYPE_FENCE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(23i32);
pub const D3D12_DRED_ALLOCATION_TYPE_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(25i32);
pub const D3D12_DRED_ALLOCATION_TYPE_INVALID: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(-1i32);
pub const D3D12_DRED_ALLOCATION_TYPE_METACOMMAND: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(43i32);
pub const D3D12_DRED_ALLOCATION_TYPE_PASS: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(35i32);
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_LIBRARY: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(29i32);
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_STATE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(21i32);
pub const D3D12_DRED_ALLOCATION_TYPE_PROTECTEDRESOURCESESSION: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(38i32);
pub const D3D12_DRED_ALLOCATION_TYPE_QUERY_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(27i32);
pub const D3D12_DRED_ALLOCATION_TYPE_RESOURCE: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(34i32);
pub const D3D12_DRED_ALLOCATION_TYPE_SCHEDULINGGROUP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(44i32);
pub const D3D12_DRED_ALLOCATION_TYPE_STATE_OBJECT: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(42i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(30i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(39i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(48i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(49i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_EXTENSION_COMMAND: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(47i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_ESTIMATOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(45i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_VECTOR_HEAP: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(46i32);
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_PROCESSOR: D3D12_DRED_ALLOCATION_TYPE = D3D12_DRED_ALLOCATION_TYPE(32i32);
pub const D3D12_DRED_DEVICE_STATE_FAULT: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(6i32);
pub const D3D12_DRED_DEVICE_STATE_HUNG: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(3i32);
pub const D3D12_DRED_DEVICE_STATE_PAGEFAULT: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(7i32);
pub const D3D12_DRED_DEVICE_STATE_UNKNOWN: D3D12_DRED_DEVICE_STATE = D3D12_DRED_DEVICE_STATE(0i32);
pub const D3D12_DRED_ENABLEMENT_FORCED_OFF: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(1i32);
pub const D3D12_DRED_ENABLEMENT_FORCED_ON: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(2i32);
pub const D3D12_DRED_ENABLEMENT_SYSTEM_CONTROLLED: D3D12_DRED_ENABLEMENT = D3D12_DRED_ENABLEMENT(0i32);
pub const D3D12_DRED_FLAG_DISABLE_AUTOBREADCRUMBS: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(2i32);
pub const D3D12_DRED_FLAG_FORCE_ENABLE: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(1i32);
pub const D3D12_DRED_FLAG_NONE: D3D12_DRED_FLAGS = D3D12_DRED_FLAGS(0i32);
pub const D3D12_DRED_PAGE_FAULT_FLAGS_NONE: D3D12_DRED_PAGE_FAULT_FLAGS = D3D12_DRED_PAGE_FAULT_FLAGS(0i32);
pub const D3D12_DRED_VERSION_1_0: D3D12_DRED_VERSION = D3D12_DRED_VERSION(1i32);
pub const D3D12_DRED_VERSION_1_1: D3D12_DRED_VERSION = D3D12_DRED_VERSION(2i32);
pub const D3D12_DRED_VERSION_1_2: D3D12_DRED_VERSION = D3D12_DRED_VERSION(3i32);
pub const D3D12_DRED_VERSION_1_3: D3D12_DRED_VERSION = D3D12_DRED_VERSION(4i32);
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_COMPATIBLE_WITH_DEVICE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(0i32);
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_TYPE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(4i32);
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_VERSION: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(3i32);
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNRECOGNIZED: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(2i32);
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNSUPPORTED_TYPE: D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS = D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(1i32);
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967287u32;
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
pub const D3D12_DSV_DIMENSION_TEXTURE1D: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(1i32);
pub const D3D12_DSV_DIMENSION_TEXTURE1DARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(2i32);
pub const D3D12_DSV_DIMENSION_TEXTURE2D: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(3i32);
pub const D3D12_DSV_DIMENSION_TEXTURE2DARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(4i32);
pub const D3D12_DSV_DIMENSION_TEXTURE2DMS: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(5i32);
pub const D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(6i32);
pub const D3D12_DSV_DIMENSION_UNKNOWN: D3D12_DSV_DIMENSION = D3D12_DSV_DIMENSION(0i32);
pub const D3D12_DSV_FLAG_NONE: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(0i32);
pub const D3D12_DSV_FLAG_READ_ONLY_DEPTH: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(1i32);
pub const D3D12_DSV_FLAG_READ_ONLY_STENCIL: D3D12_DSV_FLAGS = D3D12_DSV_FLAGS(2i32);
pub const D3D12_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_ELEMENTS_LAYOUT_ARRAY: D3D12_ELEMENTS_LAYOUT = D3D12_ELEMENTS_LAYOUT(0i32);
pub const D3D12_ELEMENTS_LAYOUT_ARRAY_OF_POINTERS: D3D12_ELEMENTS_LAYOUT = D3D12_ELEMENTS_LAYOUT(1i32);
pub const D3D12_EXPORT_FLAG_NONE: D3D12_EXPORT_FLAGS = D3D12_EXPORT_FLAGS(0i32);
pub const D3D12_FEATURE_ARCHITECTURE: D3D12_FEATURE = D3D12_FEATURE(1i32);
pub const D3D12_FEATURE_ARCHITECTURE1: D3D12_FEATURE = D3D12_FEATURE(16i32);
pub const D3D12_FEATURE_COMMAND_QUEUE_PRIORITY: D3D12_FEATURE = D3D12_FEATURE(20i32);
pub const D3D12_FEATURE_CROSS_NODE: D3D12_FEATURE = D3D12_FEATURE(25i32);
pub const D3D12_FEATURE_D3D12_OPTIONS: D3D12_FEATURE = D3D12_FEATURE(0i32);
pub const D3D12_FEATURE_D3D12_OPTIONS1: D3D12_FEATURE = D3D12_FEATURE(8i32);
pub const D3D12_FEATURE_D3D12_OPTIONS10: D3D12_FEATURE = D3D12_FEATURE(39i32);
pub const D3D12_FEATURE_D3D12_OPTIONS11: D3D12_FEATURE = D3D12_FEATURE(40i32);
pub const D3D12_FEATURE_D3D12_OPTIONS12: D3D12_FEATURE = D3D12_FEATURE(41i32);
pub const D3D12_FEATURE_D3D12_OPTIONS13: D3D12_FEATURE = D3D12_FEATURE(42i32);
pub const D3D12_FEATURE_D3D12_OPTIONS14: D3D12_FEATURE = D3D12_FEATURE(43i32);
pub const D3D12_FEATURE_D3D12_OPTIONS15: D3D12_FEATURE = D3D12_FEATURE(44i32);
pub const D3D12_FEATURE_D3D12_OPTIONS16: D3D12_FEATURE = D3D12_FEATURE(45i32);
pub const D3D12_FEATURE_D3D12_OPTIONS17: D3D12_FEATURE = D3D12_FEATURE(46i32);
pub const D3D12_FEATURE_D3D12_OPTIONS18: D3D12_FEATURE = D3D12_FEATURE(47i32);
pub const D3D12_FEATURE_D3D12_OPTIONS19: D3D12_FEATURE = D3D12_FEATURE(48i32);
pub const D3D12_FEATURE_D3D12_OPTIONS2: D3D12_FEATURE = D3D12_FEATURE(18i32);
pub const D3D12_FEATURE_D3D12_OPTIONS20: D3D12_FEATURE = D3D12_FEATURE(49i32);
pub const D3D12_FEATURE_D3D12_OPTIONS3: D3D12_FEATURE = D3D12_FEATURE(21i32);
pub const D3D12_FEATURE_D3D12_OPTIONS4: D3D12_FEATURE = D3D12_FEATURE(23i32);
pub const D3D12_FEATURE_D3D12_OPTIONS5: D3D12_FEATURE = D3D12_FEATURE(27i32);
pub const D3D12_FEATURE_D3D12_OPTIONS6: D3D12_FEATURE = D3D12_FEATURE(30i32);
pub const D3D12_FEATURE_D3D12_OPTIONS7: D3D12_FEATURE = D3D12_FEATURE(32i32);
pub const D3D12_FEATURE_D3D12_OPTIONS8: D3D12_FEATURE = D3D12_FEATURE(36i32);
pub const D3D12_FEATURE_D3D12_OPTIONS9: D3D12_FEATURE = D3D12_FEATURE(37i32);
pub const D3D12_FEATURE_DISPLAYABLE: D3D12_FEATURE = D3D12_FEATURE(28i32);
pub const D3D12_FEATURE_EXISTING_HEAPS: D3D12_FEATURE = D3D12_FEATURE(22i32);
pub const D3D12_FEATURE_FEATURE_LEVELS: D3D12_FEATURE = D3D12_FEATURE(2i32);
pub const D3D12_FEATURE_FORMAT_INFO: D3D12_FEATURE = D3D12_FEATURE(5i32);
pub const D3D12_FEATURE_FORMAT_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(3i32);
pub const D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(6i32);
pub const D3D12_FEATURE_HARDWARE_COPY: D3D12_FEATURE = D3D12_FEATURE(52i32);
pub const D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS: D3D12_FEATURE = D3D12_FEATURE(4i32);
pub const D3D12_FEATURE_PLACED_RESOURCE_SUPPORT_INFO: D3D12_FEATURE = D3D12_FEATURE(51i32);
pub const D3D12_FEATURE_PREDICATION: D3D12_FEATURE = D3D12_FEATURE(50i32);
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT: D3D12_FEATURE = D3D12_FEATURE(10i32);
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES: D3D12_FEATURE = D3D12_FEATURE(34i32);
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT: D3D12_FEATURE = D3D12_FEATURE(33i32);
pub const D3D12_FEATURE_QUERY_META_COMMAND: D3D12_FEATURE = D3D12_FEATURE(31i32);
pub const D3D12_FEATURE_ROOT_SIGNATURE: D3D12_FEATURE = D3D12_FEATURE(12i32);
pub const D3D12_FEATURE_SERIALIZATION: D3D12_FEATURE = D3D12_FEATURE(24i32);
pub const D3D12_FEATURE_SHADER_CACHE: D3D12_FEATURE = D3D12_FEATURE(19i32);
pub const D3D12_FEATURE_SHADER_MODEL: D3D12_FEATURE = D3D12_FEATURE(7i32);
pub const D3D12_FENCE_FLAG_NONE: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(0i32);
pub const D3D12_FENCE_FLAG_NON_MONITORED: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(4i32);
pub const D3D12_FENCE_FLAG_SHARED: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(1i32);
pub const D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER: D3D12_FENCE_FLAGS = D3D12_FENCE_FLAGS(2i32);
pub const D3D12_FILL_MODE_SOLID: D3D12_FILL_MODE = D3D12_FILL_MODE(3i32);
pub const D3D12_FILL_MODE_WIREFRAME: D3D12_FILL_MODE = D3D12_FILL_MODE(2i32);
pub const D3D12_FILTER_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(85i32);
pub const D3D12_FILTER_COMPARISON_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(213i32);
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(144i32);
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(145i32);
pub const D3D12_FILTER_COMPARISON_MIN_MAG_ANISOTROPIC_MIP_POINT: D3D12_FILTER = D3D12_FILTER(212i32);
pub const D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(148i32);
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(149i32);
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(128i32);
pub const D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(129i32);
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(132i32);
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(133i32);
pub const D3D12_FILTER_MAXIMUM_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(469i32);
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(400i32);
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(401i32);
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_ANISOTROPIC_MIP_POINT: D3D12_FILTER = D3D12_FILTER(468i32);
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(404i32);
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(405i32);
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(384i32);
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(385i32);
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(388i32);
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(389i32);
pub const D3D12_FILTER_MINIMUM_ANISOTROPIC: D3D12_FILTER = D3D12_FILTER(341i32);
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(272i32);
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(273i32);
pub const D3D12_FILTER_MINIMUM_MIN_MAG_ANISOTROPIC_MIP_POINT: D3D12_FILTER = D3D12_FILTER(340i32);
pub const D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(276i32);
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(277i32);
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(256i32);
pub const D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(257i32);
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(260i32);
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(261i32);
pub const D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(16i32);
pub const D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(17i32);
pub const D3D12_FILTER_MIN_MAG_ANISOTROPIC_MIP_POINT: D3D12_FILTER = D3D12_FILTER(84i32);
pub const D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(20i32);
pub const D3D12_FILTER_MIN_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(21i32);
pub const D3D12_FILTER_MIN_MAG_MIP_POINT: D3D12_FILTER = D3D12_FILTER(0i32);
pub const D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(1i32);
pub const D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D12_FILTER = D3D12_FILTER(4i32);
pub const D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D12_FILTER = D3D12_FILTER(5i32);
pub const D3D12_FILTER_REDUCTION_TYPE_COMPARISON: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(1i32);
pub const D3D12_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
pub const D3D12_FILTER_REDUCTION_TYPE_MAXIMUM: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(3i32);
pub const D3D12_FILTER_REDUCTION_TYPE_MINIMUM: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(2i32);
pub const D3D12_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub const D3D12_FILTER_REDUCTION_TYPE_STANDARD: D3D12_FILTER_REDUCTION_TYPE = D3D12_FILTER_REDUCTION_TYPE(0i32);
pub const D3D12_FILTER_TYPE_LINEAR: D3D12_FILTER_TYPE = D3D12_FILTER_TYPE(1i32);
pub const D3D12_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D12_FILTER_TYPE_POINT: D3D12_FILTER_TYPE = D3D12_FILTER_TYPE(0i32);
pub const D3D12_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D12_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D12_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D12_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D12_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D12_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D12_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub const D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16777216i32);
pub const D3D12_FORMAT_SUPPORT1_BLENDABLE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(32768i32);
pub const D3D12_FORMAT_SUPPORT1_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1i32);
pub const D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1048576i32);
pub const D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(134217728i32);
pub const D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(65536i32);
pub const D3D12_FORMAT_SUPPORT1_DISPLAY: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(524288i32);
pub const D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4i32);
pub const D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2i32);
pub const D3D12_FORMAT_SUPPORT1_MIP: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4096i32);
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(4194304i32);
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2097152i32);
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(262144i32);
pub const D3D12_FORMAT_SUPPORT1_NONE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(0i32);
pub const D3D12_FORMAT_SUPPORT1_RENDER_TARGET: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16384i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(8388608i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(67108864i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_LOAD: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(256i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(512i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1024i32);
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(2048i32);
pub const D3D12_FORMAT_SUPPORT1_SO_BUFFER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(8i32);
pub const D3D12_FORMAT_SUPPORT1_TEXTURE1D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(16i32);
pub const D3D12_FORMAT_SUPPORT1_TEXTURE2D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(32i32);
pub const D3D12_FORMAT_SUPPORT1_TEXTURE3D: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(64i32);
pub const D3D12_FORMAT_SUPPORT1_TEXTURECUBE: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(128i32);
pub const D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(33554432i32);
pub const D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(1073741824i32);
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(536870912i32);
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT: D3D12_FORMAT_SUPPORT1 = D3D12_FORMAT_SUPPORT1(268435456i32);
pub const D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(16384i32);
pub const D3D12_FORMAT_SUPPORT2_NONE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(0i32);
pub const D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(256i32);
pub const D3D12_FORMAT_SUPPORT2_SAMPLER_FEEDBACK: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(32768i32);
pub const D3D12_FORMAT_SUPPORT2_TILED: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(512i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(1i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(2i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(4i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(8i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(16i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(32i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(64i32);
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE: D3D12_FORMAT_SUPPORT2 = D3D12_FORMAT_SUPPORT2(128i32);
pub const D3D12_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D12_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D12_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D12_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING: D3D12_GPU_BASED_VALIDATION_FLAGS = D3D12_GPU_BASED_VALIDATION_FLAGS(1i32);
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_NONE: D3D12_GPU_BASED_VALIDATION_FLAGS = D3D12_GPU_BASED_VALIDATION_FLAGS(0i32);
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS_VALID_MASK: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(7i32);
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_GUARDED_VALIDATION_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(4i32);
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_TRACKING_ONLY_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(1i32);
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_UNGUARDED_VALIDATION_SHADERS: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(2i32);
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_NONE: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS = D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(0i32);
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_GUARDED_VALIDATION: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(3i32);
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_NONE: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(0i32);
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_STATE_TRACKING_ONLY: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(1i32);
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_UNGUARDED_VALIDATION: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(2i32);
pub const D3D12_GRAPHICS_STATE_COMPUTE_ROOT_SIGNATURE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(32i32);
pub const D3D12_GRAPHICS_STATE_DESCRIPTOR_HEAP: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(8i32);
pub const D3D12_GRAPHICS_STATE_GRAPHICS_ROOT_SIGNATURE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(16i32);
pub const D3D12_GRAPHICS_STATE_IA_INDEX_BUFFER: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(2i32);
pub const D3D12_GRAPHICS_STATE_IA_PRIMITIVE_TOPOLOGY: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(4i32);
pub const D3D12_GRAPHICS_STATE_IA_VERTEX_BUFFERS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(1i32);
pub const D3D12_GRAPHICS_STATE_NONE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(0i32);
pub const D3D12_GRAPHICS_STATE_OM_BLEND_FACTOR: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(2048i32);
pub const D3D12_GRAPHICS_STATE_OM_DEPTH_BOUNDS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(16384i32);
pub const D3D12_GRAPHICS_STATE_OM_RENDER_TARGETS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(512i32);
pub const D3D12_GRAPHICS_STATE_OM_STENCIL_REF: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(1024i32);
pub const D3D12_GRAPHICS_STATE_PIPELINE_STATE: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(4096i32);
pub const D3D12_GRAPHICS_STATE_PREDICATION: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(256i32);
pub const D3D12_GRAPHICS_STATE_RS_SCISSOR_RECTS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(128i32);
pub const D3D12_GRAPHICS_STATE_RS_VIEWPORTS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(64i32);
pub const D3D12_GRAPHICS_STATE_SAMPLE_POSITIONS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(32768i32);
pub const D3D12_GRAPHICS_STATE_SO_TARGETS: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(8192i32);
pub const D3D12_GRAPHICS_STATE_VIEW_INSTANCE_MASK: D3D12_GRAPHICS_STATES = D3D12_GRAPHICS_STATES(65536i32);
pub const D3D12_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
pub const D3D12_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
pub const D3D12_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
pub const D3D12_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(0i32);
pub const D3D12_HEAP_FLAG_ALLOW_DISPLAY: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(8i32);
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(192i32);
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(68i32);
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(132i32);
pub const D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(1024i32);
pub const D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(512i32);
pub const D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(2048i32);
pub const D3D12_HEAP_FLAG_CREATE_NOT_ZEROED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(4096i32);
pub const D3D12_HEAP_FLAG_DENY_BUFFERS: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(4i32);
pub const D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(128i32);
pub const D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(64i32);
pub const D3D12_HEAP_FLAG_HARDWARE_PROTECTED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(256i32);
pub const D3D12_HEAP_FLAG_NONE: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(0i32);
pub const D3D12_HEAP_FLAG_SHARED: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(1i32);
pub const D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(32i32);
pub const D3D12_HEAP_FLAG_TOOLS_USE_MANUAL_WRITE_TRACKING: D3D12_HEAP_FLAGS = D3D12_HEAP_FLAGS(8192i32);
pub const D3D12_HEAP_SERIALIZATION_TIER_0: D3D12_HEAP_SERIALIZATION_TIER = D3D12_HEAP_SERIALIZATION_TIER(0i32);
pub const D3D12_HEAP_SERIALIZATION_TIER_10: D3D12_HEAP_SERIALIZATION_TIER = D3D12_HEAP_SERIALIZATION_TIER(10i32);
pub const D3D12_HEAP_TYPE_CUSTOM: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(4i32);
pub const D3D12_HEAP_TYPE_DEFAULT: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(1i32);
pub const D3D12_HEAP_TYPE_GPU_UPLOAD: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(5i32);
pub const D3D12_HEAP_TYPE_READBACK: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(3i32);
pub const D3D12_HEAP_TYPE_UPLOAD: D3D12_HEAP_TYPE = D3D12_HEAP_TYPE(2i32);
pub const D3D12_HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE: D3D12_HIT_GROUP_TYPE = D3D12_HIT_GROUP_TYPE(1i32);
pub const D3D12_HIT_GROUP_TYPE_TRIANGLES: D3D12_HIT_GROUP_TYPE = D3D12_HIT_GROUP_TYPE(0i32);
pub const D3D12_HIT_KIND_TRIANGLE_BACK_FACE: D3D12_HIT_KIND = D3D12_HIT_KIND(255i32);
pub const D3D12_HIT_KIND_TRIANGLE_FRONT_FACE: D3D12_HIT_KIND = D3D12_HIT_KIND(254i32);
pub const D3D12_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D12_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
pub const D3D12_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
pub const D3D12_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
pub const D3D12_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D12_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D12_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D12_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D12_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
pub const D3D12_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(1i32);
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(2i32);
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(0i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(5i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(6i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(2i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(10i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_RAYS: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(9i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(0i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(1i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(4i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(7i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(8i32);
pub const D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW: D3D12_INDIRECT_ARGUMENT_TYPE = D3D12_INDIRECT_ARGUMENT_TYPE(3i32);
pub const D3D12_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub const D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA: D3D12_INPUT_CLASSIFICATION = D3D12_INPUT_CLASSIFICATION(1i32);
pub const D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA: D3D12_INPUT_CLASSIFICATION = D3D12_INPUT_CLASSIFICATION(0i32);
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D12_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
pub const D3D12_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
pub const D3D12_LIFETIME_STATE_IN_USE: D3D12_LIFETIME_STATE = D3D12_LIFETIME_STATE(0i32);
pub const D3D12_LIFETIME_STATE_NOT_IN_USE: D3D12_LIFETIME_STATE = D3D12_LIFETIME_STATE(1i32);
pub const D3D12_LINEAR_GAMMA: f32 = 1f32;
pub const D3D12_LINE_RASTERIZATION_MODE_ALIASED: D3D12_LINE_RASTERIZATION_MODE = D3D12_LINE_RASTERIZATION_MODE(0i32);
pub const D3D12_LINE_RASTERIZATION_MODE_ALPHA_ANTIALIASED: D3D12_LINE_RASTERIZATION_MODE = D3D12_LINE_RASTERIZATION_MODE(1i32);
pub const D3D12_LINE_RASTERIZATION_MODE_QUADRILATERAL_NARROW: D3D12_LINE_RASTERIZATION_MODE = D3D12_LINE_RASTERIZATION_MODE(3i32);
pub const D3D12_LINE_RASTERIZATION_MODE_QUADRILATERAL_WIDE: D3D12_LINE_RASTERIZATION_MODE = D3D12_LINE_RASTERIZATION_MODE(2i32);
pub const D3D12_LOGIC_OP_AND: D3D12_LOGIC_OP = D3D12_LOGIC_OP(6i32);
pub const D3D12_LOGIC_OP_AND_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(13i32);
pub const D3D12_LOGIC_OP_AND_REVERSE: D3D12_LOGIC_OP = D3D12_LOGIC_OP(12i32);
pub const D3D12_LOGIC_OP_CLEAR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(0i32);
pub const D3D12_LOGIC_OP_COPY: D3D12_LOGIC_OP = D3D12_LOGIC_OP(2i32);
pub const D3D12_LOGIC_OP_COPY_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(3i32);
pub const D3D12_LOGIC_OP_EQUIV: D3D12_LOGIC_OP = D3D12_LOGIC_OP(11i32);
pub const D3D12_LOGIC_OP_INVERT: D3D12_LOGIC_OP = D3D12_LOGIC_OP(5i32);
pub const D3D12_LOGIC_OP_NAND: D3D12_LOGIC_OP = D3D12_LOGIC_OP(7i32);
pub const D3D12_LOGIC_OP_NOOP: D3D12_LOGIC_OP = D3D12_LOGIC_OP(4i32);
pub const D3D12_LOGIC_OP_NOR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(9i32);
pub const D3D12_LOGIC_OP_OR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(8i32);
pub const D3D12_LOGIC_OP_OR_INVERTED: D3D12_LOGIC_OP = D3D12_LOGIC_OP(15i32);
pub const D3D12_LOGIC_OP_OR_REVERSE: D3D12_LOGIC_OP = D3D12_LOGIC_OP(14i32);
pub const D3D12_LOGIC_OP_SET: D3D12_LOGIC_OP = D3D12_LOGIC_OP(1i32);
pub const D3D12_LOGIC_OP_XOR: D3D12_LOGIC_OP = D3D12_LOGIC_OP(10i32);
pub const D3D12_MAG_FILTER_SHIFT: u32 = 2u32;
pub const D3D12_MAJOR_VERSION: u32 = 12u32;
pub const D3D12_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D12_MAX_DEPTH: f32 = 1f32;
pub const D3D12_MAX_LIVE_STATIC_SAMPLERS: u32 = 2032u32;
pub const D3D12_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D12_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D12_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D12_MAX_ROOT_COST: u32 = 64u32;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1: u32 = 1000000u32;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_2: u32 = 1000000u32;
pub const D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE: u32 = 2048u32;
pub const D3D12_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
pub const D3D12_MAX_VIEW_INSTANCE_COUNT: u32 = 4u32;
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(1i32);
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS_HIGH_PRIORITY: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(2i32);
pub const D3D12_MEASUREMENTS_ACTION_DISCARD_PREVIOUS: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(3i32);
pub const D3D12_MEASUREMENTS_ACTION_KEEP_ALL: D3D12_MEASUREMENTS_ACTION = D3D12_MEASUREMENTS_ACTION(0i32);
pub const D3D12_MEMORY_POOL_L0: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(1i32);
pub const D3D12_MEMORY_POOL_L1: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(2i32);
pub const D3D12_MEMORY_POOL_UNKNOWN: D3D12_MEMORY_POOL = D3D12_MEMORY_POOL(0i32);
pub const D3D12_MESH_SHADER_TIER_1: D3D12_MESH_SHADER_TIER = D3D12_MESH_SHADER_TIER(10i32);
pub const D3D12_MESH_SHADER_TIER_NOT_SUPPORTED: D3D12_MESH_SHADER_TIER = D3D12_MESH_SHADER_TIER(0i32);
pub const D3D12_MESSAGE_CALLBACK_FLAG_NONE: D3D12_MESSAGE_CALLBACK_FLAGS = D3D12_MESSAGE_CALLBACK_FLAGS(0i32);
pub const D3D12_MESSAGE_CALLBACK_IGNORE_FILTERS: D3D12_MESSAGE_CALLBACK_FLAGS = D3D12_MESSAGE_CALLBACK_FLAGS(1i32);
pub const D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(0i32);
pub const D3D12_MESSAGE_CATEGORY_CLEANUP: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(3i32);
pub const D3D12_MESSAGE_CATEGORY_COMPILATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(4i32);
pub const D3D12_MESSAGE_CATEGORY_EXECUTION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(9i32);
pub const D3D12_MESSAGE_CATEGORY_INITIALIZATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(2i32);
pub const D3D12_MESSAGE_CATEGORY_MISCELLANEOUS: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(1i32);
pub const D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(8i32);
pub const D3D12_MESSAGE_CATEGORY_SHADER: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(10i32);
pub const D3D12_MESSAGE_CATEGORY_STATE_CREATION: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(5i32);
pub const D3D12_MESSAGE_CATEGORY_STATE_GETTING: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(7i32);
pub const D3D12_MESSAGE_CATEGORY_STATE_SETTING: D3D12_MESSAGE_CATEGORY = D3D12_MESSAGE_CATEGORY(6i32);
pub const D3D12_MESSAGE_ID_ADD_TO_STATE_OBJECT_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1246i32);
pub const D3D12_MESSAGE_ID_ALPHA_BLEND_FACTOR_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1349i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1039i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_SUBRESOURCE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1038i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DST_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1029i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_ARCHITECTURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1026i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1036i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_SUBRESOURCE_RANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1037i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1142i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1028i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_OFFSET_ALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1033i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1143i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1031i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_RESOURCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1034i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_SUBRESOURCE_RANGES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1035i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1027i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_SRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1030i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_SRC_RANGE_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1032i32);
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_ZERO_DEPENDENCIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1040i32);
pub const D3D12_MESSAGE_ID_BARRIER_INTEROP_INVALID_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1350i32);
pub const D3D12_MESSAGE_ID_BARRIER_INTEROP_INVALID_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1351i32);
pub const D3D12_MESSAGE_ID_BEGIN_END_EVENT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(955i32);
pub const D3D12_MESSAGE_ID_BEGIN_END_QUERY_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(731i32);
pub const D3D12_MESSAGE_ID_BEGIN_EVENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1014i32);
pub const D3D12_MESSAGE_ID_BUFFER_BARRIER_SUBREGION_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1340i32);
pub const D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1158i32);
pub const D3D12_MESSAGE_ID_BUNDLE_PIPELINE_STATE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(610i32);
pub const D3D12_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1192i32);
pub const D3D12_MESSAGE_ID_CANNOT_CHANGE_COMMAND_RECORDER_TARGET_WHILE_RECORDING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1134i32);
pub const D3D12_MESSAGE_ID_CANNOT_CREATE_GRAPHICS_AND_VIDEO_COMMAND_RECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1153i32);
pub const D3D12_MESSAGE_ID_CANNOT_EXECUTE_EMPTY_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1131i32);
pub const D3D12_MESSAGE_ID_CANNOT_RESET_COMMAND_POOL_WITH_OPEN_COMMAND_LISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1132i32);
pub const D3D12_MESSAGE_ID_CANNOT_USE_COMMAND_RECORDER_WITHOUT_CURRENT_TARGET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1133i32);
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_DRIVER_REPORTED_ISSUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1201i32);
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1200i32);
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(135i32);
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_MISMATCHINGCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(821i32);
pub const D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(820i32);
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1156i32);
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1215i32);
pub const D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1314i32);
pub const D3D12_MESSAGE_ID_CLOSE_COMMAND_LIST_OPEN_QUERY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(732i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CANNOT_RESET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(543i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CONTENTION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(540i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(541i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET_BUNDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(542i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(552i32);
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_USAGE_WITH_CREATECOMMANDLIST1_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1130i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_CLOSED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(547i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DESCRIPTOR_TABLE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(991i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(953i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(952i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ELEMENT_OFFSET_UNALIGNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1348i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(212i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(211i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(213i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_OFFSET_UNALIGNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(222i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(219i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_RENDER_TARGET_DELETED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(924i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(201i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(200i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(202i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(209i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(210i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_STRIDE_UNALIGNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(221i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_MULTIPLE_SWAPCHAIN_BUFFER_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(904i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OPEN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(544i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(903i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_PIPELINE_STATE_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1045i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(992i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(993i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(994i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SETRENDERTARGETS_INVALIDNUMRENDERTARGETS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(908i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1023i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_SAMPLER_MODE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1321i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(553i32);
pub const D3D12_MESSAGE_ID_COMMAND_LIST_TOO_MANY_SWAPCHAIN_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(905i32);
pub const D3D12_MESSAGE_ID_COMMAND_POOL_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1135i32);
pub const D3D12_MESSAGE_ID_COMMAND_QUEUE_TOO_MANY_SWAPCHAIN_REFERENCES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(906i32);
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_CONTENTION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1128i32);
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_SUPPORT_FLAGS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1127i32);
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_USAGE_WITH_CREATECOMMANDLIST_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1129i32);
pub const D3D12_MESSAGE_ID_COMPUTE_ONLY_DEVICE_OPERATION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1157i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_DSTRANGEOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(841i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDCOPYFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(845i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDDSTRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(840i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDSRCRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(843i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_DST_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1140i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_SRC_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1141i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(839i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(842i32);
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_SRCRANGEOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(844i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(944i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(946i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1098i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1304i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(943i32);
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(945i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_DSTREGIONOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(858i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(875i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_FORMATMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(874i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDCOPYFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(876i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTCOORDINATES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(872i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(854i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDSPLACEDFOOTPRINTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(857i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(853i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(851i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTPLACEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(856i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(849i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(848i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTROWPITCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(855i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(850i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(873i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(867i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDSPLACEDFOOTPRINTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(870i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(866i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(864i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCPLACEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(869i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(862i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCEDIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(861i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCROWPITCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(868i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(863i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1097i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1303i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(846i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(859i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_SRCREGIONOUTOFBOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(871i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(852i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(847i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(865i32);
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(860i32);
pub const D3D12_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(494i32);
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_INVALID_RANGES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(653i32);
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_WRITE_ONLY_DESCRIPTOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(654i32);
pub const D3D12_MESSAGE_ID_COPY_INVALIDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1067i32);
pub const D3D12_MESSAGE_ID_COPY_ON_SAME_SUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(998i32);
pub const D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1160i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(18i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(3i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(12i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(13i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(14i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(15i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(16i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(17i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(4i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(5i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(6i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(7i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(8i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(9i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(10i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(11i32);
pub const D3D12_MESSAGE_ID_CORRUPTED_THIS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(2i32);
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1278i32);
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1279i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOPALPHA_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1317i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOP_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1316i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(113i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(116i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(112i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(115i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(403i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(117i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(111i32);
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(114i32);
pub const D3D12_MESSAGE_ID_CREATECOMMANDLIST_NULL_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1313i32);
pub const D3D12_MESSAGE_ID_CREATECOMMANDSIGNATURE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(743i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_CS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(882i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_INVALID_SHADER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(881i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_MISSING_ROOT_SIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(883i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(323i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(322i32);
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(321i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_DEPTHBOUNDSTEST_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1017i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INDEPENDENT_MASKS_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1354i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(106i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(109i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(108i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(107i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(101i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(100i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(102i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(105i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(104i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(103i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(46i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(48i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(276i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(47i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(49i32);
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(45i32);
pub const D3D12_MESSAGE_ID_CREATEDEVICE_DEBUG_LAYER_STARTUP_OPTIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1016i32);
pub const D3D12_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(506i32);
pub const D3D12_MESSAGE_ID_CREATEDEVICE_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(507i32);
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(297i32);
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(295i32);
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(296i32);
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(294i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(89i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(90i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(82i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(84i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(75i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(287i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(80i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(86i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(73i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(74i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(83i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(284i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(280i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(88i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(91i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(87i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(81i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(72i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(79i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(76i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(85i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(240i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(285i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(286i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(278i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(70i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(71i32);
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(69i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_ALL_RENDER_TARGETS_HAVE_UNKNOWN_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(925i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_NOT_MS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1250i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1244i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DEPTHSTENCILVIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(680i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(687i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_INPUT_PRIMITIVE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(681i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(689i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_CONTROL_POINT_COUNT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(669i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(670i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(686i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_XOR_DS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(667i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(668i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(658i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_SHADER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1253i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_INDEX_BUFFER_PROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(684i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_PRIMITIVETOPOLOGY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(673i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_RENDER_TARGET_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(656i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SAMPLE_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(685i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SYSTEMVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(674i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(671i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_FORCED_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(672i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_METADATA_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1103i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(691i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(683i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_NOT_PS_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1251i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_PSO_DESC_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1248i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1245i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(675i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(676i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(678i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_POSITION_NOT_PRESENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(682i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_RT_OUTPUT_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(974i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(677i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(690i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDERTARGETVIEW_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(679i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDER_TARGET_WRONG_WRITE_MASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1382i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RTV_FORMAT_NOT_UNKNOWN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(655i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RUNTIME_INTERNAL_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1105i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_COMPONENTTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(661i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_HS_DS_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(659i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_MINPRECISION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(665i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(664i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERINDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(660i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERMASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(662i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(666i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SYSTEMVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(663i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_MODEL_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1046i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VERTEX_SHADER_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(657i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VIEW_INSTANCING_VERTEX_SIZE_EXCEEDED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1104i32);
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VS_ROOT_SIGNATURE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(688i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(629i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(632i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1362i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(631i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(628i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(624i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_NULLDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(623i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(633i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(626i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(625i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMEMORYPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(627i32);
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(630i32);
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(292i32);
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(290i32);
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(291i32);
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(289i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(62i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(253i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(55i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(61i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(54i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(57i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(56i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(59i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(60i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(65i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(64i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(52i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(58i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(53i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(239i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(245i32);
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(63i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1267i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_GROUPSHAREDEXCEEDSMAXSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1273i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1265i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_MISMATCHEDASMSPAYLOADSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1276i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1266i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTPUTEXCEEDSMAXSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1272i32);
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_TOPOLOGY_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1323i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_ADAPTERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(964i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_DRIVERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(963i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_INVALIDLIBRARYBLOB: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(962i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(965i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBADAPTERMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(885i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDESCMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(887i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDRIVERVERSIONMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(886i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBIGNORED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(888i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CANNOT_DEDUCE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1022i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_DUPLICATE_SUBOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1018i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALIDCACHEDBLOB: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(884i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(922i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_STREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1021i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_MS_INCOMPLETE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1249i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_UNKNOWN_SUBOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1019i32);
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_ZERO_SIZE_STREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1020i32);
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(283i32);
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(93i32);
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(94i32);
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(92i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1147i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1148i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_OFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1149i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1150i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1151i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_BUFFER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1144i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_RESOURCE_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1145i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1152i32);
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1146i32);
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_COPY_QUEUE_TIMESTAMPS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1079i32);
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(729i32);
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_VIDEO_DECODE_STATISTICS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(978i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(96i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(97i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(95i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(401i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(98i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(647i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_LINERASTERIZATIONMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1379i32);
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_NON_WHOLE_DYNAMIC_DEPTH_BIAS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1365i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(37i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(41i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(38i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(40i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(42i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(39i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(35i32);
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(36i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(641i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(640i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(638i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1363i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1342i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(701i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAPPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(634i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLRESOURCEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(700i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(642i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(636i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(639i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(635i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDMEMORYPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(637i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_CLEARVALUEDENORMFLUSH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(818i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(721i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT_SMALLRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1380i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(602i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(815i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(817i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(604i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(720i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(597i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(738i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(724i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMIPLEVELS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(722i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMISCFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(599i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDSAMPLEDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(723i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(603i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_STATE_IGNORED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1328i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDCLEARVALUEFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(816i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDDIMENSIONALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(718i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(737i32);
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDLAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(719i32);
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_ALREADYOPEN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1285i32);
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_DISABLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1284i32);
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1283i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(27i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(31i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(28i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(30i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(32i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(29i32);
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(26i32);
pub const D3D12_MESSAGE_ID_CREATESHADER_INVALIDBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(622i32);
pub const D3D12_MESSAGE_ID_CREATESHAREDHEAP_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(913i32);
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(911i32);
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(912i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(341i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(345i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(354i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(342i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(344i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(340i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(343i32);
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(346i32);
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(277i32);
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(67i32);
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(68i32);
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(66i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(558i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(560i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1122i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(557i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1115i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(569i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_ALLOCATOR_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(977i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1155i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE_FOR_FEATURE_LEVEL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1224i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1126i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_POOL_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1125i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_HARDWARE_SUPPORT_FOR_GLOBAL_REALTIME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1025i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_PRIVILEGE_FOR_GLOBAL_REALTIME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1024i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1120i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_SUPPORT_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1119i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_MORE_RECORDERS_THAN_LOGICAL_PROCESSORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1121i32);
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1118i32);
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(650i32);
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(649i32);
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1068i32);
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1069i32);
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(563i32);
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(645i32);
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_LARGE_NUM_DESCRIPTORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1013i32);
pub const D3D12_MESSAGE_ID_CREATE_FENCE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1007i32);
pub const D3D12_MESSAGE_ID_CREATE_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(566i32);
pub const D3D12_MESSAGE_ID_CREATE_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(565i32);
pub const D3D12_MESSAGE_ID_CREATE_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1163i32);
pub const D3D12_MESSAGE_ID_CREATE_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1137i32);
pub const D3D12_MESSAGE_ID_CREATE_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(567i32);
pub const D3D12_MESSAGE_ID_CREATE_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(966i32);
pub const D3D12_MESSAGE_ID_CREATE_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(559i32);
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1070i32);
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1247i32);
pub const D3D12_MESSAGE_ID_CREATE_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(568i32);
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(910i32);
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_PRIORITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(920i32);
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(909i32);
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_VIDEO_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(976i32);
pub const D3D12_MESSAGE_ID_CREATE_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(562i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(564i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_BLOB_NOT_FOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(696i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_DESERIALIZE_FAILED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(697i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_INVALID_CONFIGURATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(698i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(699i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_UNIQUE_IN_DXIL_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1230i32);
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_UNBOUNDED_STATIC_DESCRIPTORS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1277i32);
pub const D3D12_MESSAGE_ID_CREATE_SAMPLER_COMPARISON_FUNC_IGNORED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1361i32);
pub const D3D12_MESSAGE_ID_CREATE_SAMPLER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(742i32);
pub const D3D12_MESSAGE_ID_CREATE_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1280i32);
pub const D3D12_MESSAGE_ID_CREATE_STATE_OBJECT_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1194i32);
pub const D3D12_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1167i32);
pub const D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(652i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(979i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1051i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(980i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1083i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(981i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1177i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1180i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1297i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1300i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1225i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1183i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1186i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1052i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1053i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1060i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1061i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODER_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1102i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_FAILURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1099i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1100i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1311i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1312i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1309i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1310i32);
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_PROCESSOR_CAPS_FAILURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1108i32);
pub const D3D12_MESSAGE_ID_D3D12_MESSAGES_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1387i32);
pub const D3D12_MESSAGE_ID_DATA_STATIC_DESCRIPTOR_INVALID_DATA_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1002i32);
pub const D3D12_MESSAGE_ID_DATA_STATIC_WHILE_SET_AT_EXECUTE_DESCRIPTOR_INVALID_DATA_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1003i32);
pub const D3D12_MESSAGE_ID_DECODE_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(988i32);
pub const D3D12_MESSAGE_ID_DEPRECATED_API: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(989i32);
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_FORMAT_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(615i32);
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(616i32);
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HANDLE_WITH_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1042i32);
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HEAP_NOT_SHADER_VISIBLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1315i32);
pub const D3D12_MESSAGE_ID_DESTROYOWNEDOBJECT_OBJECTNOTOWNED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1166i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(584i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(586i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1124i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(583i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1117i32);
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(595i32);
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1074i32);
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1075i32);
pub const D3D12_MESSAGE_ID_DESTROY_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(589i32);
pub const D3D12_MESSAGE_ID_DESTROY_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(592i32);
pub const D3D12_MESSAGE_ID_DESTROY_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(591i32);
pub const D3D12_MESSAGE_ID_DESTROY_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1165i32);
pub const D3D12_MESSAGE_ID_DESTROY_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1139i32);
pub const D3D12_MESSAGE_ID_DESTROY_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(593i32);
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(968i32);
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(585i32);
pub const D3D12_MESSAGE_ID_DESTROY_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1076i32);
pub const D3D12_MESSAGE_ID_DESTROY_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(594i32);
pub const D3D12_MESSAGE_ID_DESTROY_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(588i32);
pub const D3D12_MESSAGE_ID_DESTROY_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(590i32);
pub const D3D12_MESSAGE_ID_DESTROY_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1282i32);
pub const D3D12_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1169i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(985i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1057i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(986i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1085i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(987i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1179i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1182i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1299i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1302i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1227i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1185i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1188i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1058i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1059i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1064i32);
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1065i32);
pub const D3D12_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(318i32);
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(448i32);
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(447i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(422i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(337i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(431i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(414i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(333i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(427i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(418i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(335i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(429i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(416i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(334i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(428i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(412i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(332i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(426i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(420i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(336i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(430i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(410i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(331i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(425i32);
pub const D3D12_MESSAGE_ID_DEVICE_CREATE_SHARED_HANDLE_INVALIDARG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1041i32);
pub const D3D12_MESSAGE_ID_DEVICE_OPEN_SHARED_HANDLE_ACCESS_DENIED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1330i32);
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(232i32);
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(234i32);
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(233i32);
pub const D3D12_MESSAGE_ID_DISCARD_INVALID_SUBRESOURCE_RANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(995i32);
pub const D3D12_MESSAGE_ID_DISCARD_NO_RECTS_FOR_NON_TEXTURE2D: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(997i32);
pub const D3D12_MESSAGE_ID_DISCARD_ONE_SUBRESOURCE_FOR_MIPS_WITH_RECTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(996i32);
pub const D3D12_MESSAGE_ID_DISPATCH_RAYS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1161i32);
pub const D3D12_MESSAGE_ID_DRAW_EMPTY_SCISSOR_RECTANGLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(695i32);
pub const D3D12_MESSAGE_ID_DRAW_POTENTIALLY_OUTSIDE_OF_VALID_RENDER_AREA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1378i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_DEPTH_BIAS_FLAG_MISSING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1366i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_DEPTH_BIAS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1364i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_DEPTH_BIAS_NO_PIPELINE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1367i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_INDEX_BUFFER_STRIP_CUT_FLAG_MISSING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1368i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_INDEX_BUFFER_STRIP_CUT_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1359i32);
pub const D3D12_MESSAGE_ID_DYNAMIC_INDEX_BUFFER_STRIP_CUT_NO_PIPELINE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1369i32);
pub const D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1159i32);
pub const D3D12_MESSAGE_ID_EMPTY_DISPATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1254i32);
pub const D3D12_MESSAGE_ID_EMPTY_ROOT_DESCRIPTOR_TABLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1347i32);
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1305i32);
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1306i32);
pub const D3D12_MESSAGE_ID_END_EVENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1015i32);
pub const D3D12_MESSAGE_ID_ENHANCED_BARRIERS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1343i32);
pub const D3D12_MESSAGE_ID_ENQUEUE_MAKE_RESIDENT_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1111i32);
pub const D3D12_MESSAGE_ID_ESTIMATE_MOTION_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1220i32);
pub const D3D12_MESSAGE_ID_EVICT_NULLOBJECTARRAY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(705i32);
pub const D3D12_MESSAGE_ID_EVICT_UNDERFLOW: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1136i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_BUNDLENOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(835i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_COMMANDLISTMISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(836i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_FAILEDCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(838i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_GPU_WRITTEN_READBACK_RESOURCE_MAPPED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(927i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_OPENCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(837i32);
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_WRONGSWAPCHAINBUFFERREFERENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(907i32);
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_DESCRIPTOR_HEAP_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(693i32);
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_OPEN_BUNDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(692i32);
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_STATIC_DESCRIPTOR_DATA_STATIC_NOT_SET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1004i32);
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(694i32);
pub const D3D12_MESSAGE_ID_EXECUTE_INDIRECT_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(744i32);
pub const D3D12_MESSAGE_ID_EXECUTE_INDIRECT_ZERO_COMMAND_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1357i32);
pub const D3D12_MESSAGE_ID_FENCE_INVALIDOPERATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1078i32);
pub const D3D12_MESSAGE_ID_GENERIC_DEVICE_OPERATION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1381i32);
pub const D3D12_MESSAGE_ID_GEOMETRY_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1232i32);
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDBASEOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(740i32);
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDSUBRESOURCERANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(739i32);
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_UNSUPPORTED_BUFFER_WIDTH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1322i32);
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDBASEOFFSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(740i32);
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDSUBRESOURCERANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(739i32);
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_INVALIDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(644i32);
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_UNRECOGNIZEDHEAPTYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(643i32);
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_HEAP_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1222i32);
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(745i32);
pub const D3D12_MESSAGE_ID_GETHEAPPROPERTIES_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(901i32);
pub const D3D12_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(20i32);
pub const D3D12_MESSAGE_ID_GETRESOURCEALLOCATIONINFO_INVALIDRDESCS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(702i32);
pub const D3D12_MESSAGE_ID_GET_PIPELINE_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1197i32);
pub const D3D12_MESSAGE_ID_GET_PROGRAM_IDENTIFIER_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(118i32);
pub const D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1162i32);
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1195i32);
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_SIZE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1199i32);
pub const D3D12_MESSAGE_ID_GET_SHADER_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1196i32);
pub const D3D12_MESSAGE_ID_GET_WORK_GRAPH_PROPERTIES_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(119i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_HEAP_INDEX_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(936i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TABLE_REGISTER_INDEX_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(937i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(939i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_UNINITIALIZED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(938i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_RESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(942i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_TEXTURE_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1358i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(958i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_ACCESS_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1005i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_STATE_IMPRECISE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1044i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_ARGUMENT_UNINITIALIZED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(935i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_DESCRIPTOR_ACCESS_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(961i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SAMPLER_MODE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1006i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SRV_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(940i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UAV_RESOURCE_DIMENSION_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(941i32);
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1000i32);
pub const D3D12_MESSAGE_ID_GRAPHICS_PIPELINE_STATE_DESC_ZERO_SAMPLE_MASK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1352i32);
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_HAS_NO_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(923i32);
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_INTERSECTS_MULTIPLE_BUFFERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(926i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1332i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1334i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_RESOURCE_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1337i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_SYNC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1333i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1335i32);
pub const D3D12_MESSAGE_ID_INCOMPATIBLE_BARRIER_VALUES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1331i32);
pub const D3D12_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1193i32);
pub const D3D12_MESSAGE_ID_INDEPENDENT_STENCIL_REF_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1353i32);
pub const D3D12_MESSAGE_ID_INVALID_BUNDLE_API: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(546i32);
pub const D3D12_MESSAGE_ID_INVALID_CAST_TARGET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1371i32);
pub const D3D12_MESSAGE_ID_INVALID_DESCRIPTOR_HANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(646i32);
pub const D3D12_MESSAGE_ID_INVALID_NODE_INDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(900i32);
pub const D3D12_MESSAGE_ID_INVALID_SUBRESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(538i32);
pub const D3D12_MESSAGE_ID_INVALID_USE_OF_NON_RESIDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(608i32);
pub const D3D12_MESSAGE_ID_INVALID_VIDEO_EXTENSION_COMMAND_ID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1228i32);
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDKEY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(918i32);
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(917i32);
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_WRONGSTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(919i32);
pub const D3D12_MESSAGE_ID_LEGACY_BARRIER_VALIDATION_FORCED_ON: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1346i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDALLOCATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(571i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDLIST12: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(573i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDPOOL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1123i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(570i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDRECORDER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1116i32);
pub const D3D12_MESSAGE_ID_LIVE_COMMANDSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(582i32);
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1071i32);
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION_POLICY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1072i32);
pub const D3D12_MESSAGE_ID_LIVE_DESCRIPTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(576i32);
pub const D3D12_MESSAGE_ID_LIVE_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(274i32);
pub const D3D12_MESSAGE_ID_LIVE_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(579i32);
pub const D3D12_MESSAGE_ID_LIVE_LIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(578i32);
pub const D3D12_MESSAGE_ID_LIVE_LIFETIMETRACKER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1164i32);
pub const D3D12_MESSAGE_ID_LIVE_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1138i32);
pub const D3D12_MESSAGE_ID_LIVE_MONITOREDFENCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(580i32);
pub const D3D12_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(255i32);
pub const D3D12_MESSAGE_ID_LIVE_PIPELINELIBRARY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(967i32);
pub const D3D12_MESSAGE_ID_LIVE_PIPELINESTATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(572i32);
pub const D3D12_MESSAGE_ID_LIVE_PROTECTED_RESOURCE_SESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1073i32);
pub const D3D12_MESSAGE_ID_LIVE_QUERYHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(581i32);
pub const D3D12_MESSAGE_ID_LIVE_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(575i32);
pub const D3D12_MESSAGE_ID_LIVE_ROOTSIGNATURE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(577i32);
pub const D3D12_MESSAGE_ID_LIVE_SHADERCACHESESSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1281i32);
pub const D3D12_MESSAGE_ID_LIVE_SWAPCHAIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(275i32);
pub const D3D12_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1168i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(982i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1054i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(983i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1084i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODESTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(984i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1178i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1181i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1298i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODERHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1301i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOEXTENSIONCOMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1226i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONESTIMATOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1184i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONVECTORHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1187i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDLIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1055i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDQUEUE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1056i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSOR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1062i32);
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSSTREAM: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1063i32);
pub const D3D12_MESSAGE_ID_LOADPIPELINE_INVALIDDESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(972i32);
pub const D3D12_MESSAGE_ID_LOADPIPELINE_NAMENOTFOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(971i32);
pub const D3D12_MESSAGE_ID_MAKERESIDENT_NULLOBJECTARRAY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(703i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(833i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDDATAPOINTER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(832i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(822i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(828i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(824i32);
pub const D3D12_MESSAGE_ID_MAP_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(826i32);
pub const D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(930i32);
pub const D3D12_MESSAGE_ID_MAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(834i32);
pub const D3D12_MESSAGE_ID_MESH_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1275i32);
pub const D3D12_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(19i32);
pub const D3D12_MESSAGE_ID_META_COMMAND_FAILED_ENUMERATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1173i32);
pub const D3D12_MESSAGE_ID_META_COMMAND_ID_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1171i32);
pub const D3D12_MESSAGE_ID_META_COMMAND_INVALID_GPU_VIRTUAL_ADDRESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1176i32);
pub const D3D12_MESSAGE_ID_META_COMMAND_PARAMETER_SIZE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1174i32);
pub const D3D12_MESSAGE_ID_META_COMMAND_UNSUPPORTED_PARAMS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1172i32);
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1189i32);
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1190i32);
pub const D3D12_MESSAGE_ID_NODE_MASK_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(902i32);
pub const D3D12_MESSAGE_ID_NONNORMALIZED_COORDINATE_SAMPLING_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1370i32);
pub const D3D12_MESSAGE_ID_NONZERO_SAMPLER_FEEDBACK_MIP_REGION_WITH_INCOMPATIBLE_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1252i32);
pub const D3D12_MESSAGE_ID_NON_OPTIMAL_BARRIER_ONLY_EXECUTE_COMMAND_LISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1356i32);
pub const D3D12_MESSAGE_ID_NON_RETAIL_SHADER_MODEL_WONT_VALIDATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1243i32);
pub const D3D12_MESSAGE_ID_NO_COMPUTE_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(933i32);
pub const D3D12_MESSAGE_ID_NO_GRAPHICS_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(932i32);
pub const D3D12_MESSAGE_ID_NO_VIDEO_API_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1106i32);
pub const D3D12_MESSAGE_ID_OBJECT_ACCESSED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1047i32);
pub const D3D12_MESSAGE_ID_OBJECT_DELETED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(921i32);
pub const D3D12_MESSAGE_ID_OBJECT_EVICTED_WHILE_STILL_IN_USE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(960i32);
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDADDRESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1088i32);
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDARG_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1086i32);
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDHANDLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1089i32);
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_OUTOFMEMORY_RETURN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1087i32);
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1112i32);
pub const D3D12_MESSAGE_ID_OUT_OF_BOUNDS_BARRIER_SUBRESOURCE_RANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1336i32);
pub const D3D12_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1191i32);
pub const D3D12_MESSAGE_ID_OVERSIZED_DISPATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1296i32);
pub const D3D12_MESSAGE_ID_PIPELINELIBRARY_SERIALIZE_NOTENOUGHMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(973i32);
pub const D3D12_MESSAGE_ID_PIPELINE_STATE_TYPE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(951i32);
pub const D3D12_MESSAGE_ID_PIX_EVENT_UNDERFLOW: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1384i32);
pub const D3D12_MESSAGE_ID_POSSIBLE_INVALID_USE_OF_NON_RESIDENT_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(609i32);
pub const D3D12_MESSAGE_ID_POSSIBLY_INVALID_SUBRESOURCE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(607i32);
pub const D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(611i32);
pub const D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_TRIANGLE_FANS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1360i32);
pub const D3D12_MESSAGE_ID_PROBABLE_PIX_EVENT_LEAK: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1383i32);
pub const D3D12_MESSAGE_ID_PROCESS_FRAME_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1066i32);
pub const D3D12_MESSAGE_ID_PROGRAMMABLE_MSAA_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1048i32);
pub const D3D12_MESSAGE_ID_PROTECTED_RESOURCE_SESSION_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1077i32);
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(898i32);
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(896i32);
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(894i32);
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(895i32);
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(897i32);
pub const D3D12_MESSAGE_ID_RECREATEAT_INSUFFICIENT_SUPPORT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1386i32);
pub const D3D12_MESSAGE_ID_RECREATEAT_INVALID_TARGET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1385i32);
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDOBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(916i32);
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDSIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(915i32);
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_UNRECOGNIZEDPROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(914i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_CLOSE_COMMAND_LIST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1206i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_END_WITHOUT_BEGIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1205i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_NEST_RENDER_PASSES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1204i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_COMMANDLIST_INVALID_END_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1372i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_COMMANDLIST_INVALID_START_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1373i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_DISALLOWED_API_CALLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1203i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1170i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_GPU_WORK_WHILE_SUSPENDED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1207i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_INVALID_RESOURCE_BARRIER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1202i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_LOCAL_DEPTH_STENCIL_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1377i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_LOCAL_PRESERVE_RENDER_PARAMETERS_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1376i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1374i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_LOCAL_PRESERVE_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1375i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_NO_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1213i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_SUSPEND_RESUME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1208i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_PRIOR_SUSPEND_WITHIN_EXECUTECOMMANDLISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1209i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_SUBSEQUENT_RESUME_WITHIN_EXECUTECOMMANDLISTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1210i32);
pub const D3D12_MESSAGE_ID_RENDER_PASS_UNSUPPORTED_RESOLVE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1214i32);
pub const D3D12_MESSAGE_ID_RENDER_TARGET_FORMAT_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(613i32);
pub const D3D12_MESSAGE_ID_RENDER_TARGET_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(614i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCEREGION_INVALID_RECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1050i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDDSTRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(948i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDSRCRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(950i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(878i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(880i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SUBRESOURCE_INDEX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(877i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLDST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(947i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLSRC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(949i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_FLAGS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(934i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(879i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_INVALID_MIP_LEVEL_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1269i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_ARRAY_SIZE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1270i32);
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_INVALID_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1268i32);
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1307i32);
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_UNSUPPORTED_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1308i32);
pub const D3D12_MESSAGE_ID_RESOLVE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1221i32);
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_DATA_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(733i32);
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_INVALID_QUERY_STATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1319i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_BEFORE_AFTER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(527i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_DUPLICATE_SUBRESOURCE_TRANSITIONS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1008i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(526i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINED_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(531i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(537i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(536i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(530i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS_FOR_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(532i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_HEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(741i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(528i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SPLIT_BARRIER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(533i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(521i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(519i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MATCHING_STATES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(525i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_BEGIN_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(957i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_COMMAND_LIST_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(990i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_MISC_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(524i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISSING_BIND_FLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(523i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_NULL_POINTER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(520i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_POSSIBLE_BEFORE_AFTER_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(956i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_RESERVED_BITS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(522i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(529i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_BEGIN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(535i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_END: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(534i32);
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_ZERO_BARRIERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(954i32);
pub const D3D12_MESSAGE_ID_RESOURCE_FORMAT_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1255i32);
pub const D3D12_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(310i32);
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATEIMAGE_REQUIRES_TIER_2: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1236i32);
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATE_REQUIRES_TIER_1: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1237i32);
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_COMBINER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1235i32);
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_SHADING_RATE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1233i32);
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_SHADING_RATE_NOT_PERMITTED_BY_CAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1234i32);
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_DEFERRED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1080i32);
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMCLEAR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1082i32);
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMFIRSTUSE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1081i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_MISMATCHING_TARGETED_RESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1271i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_NULL_ARGUMENTS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1262i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_REQUIRES_FEEDBACK_MAP_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1264i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_DIMENSION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1257i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1260i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_MIP_REGION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1256i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1258i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_QUALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1259i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_REQUIRES_UNORDERED_ACCESS_FLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1261i32);
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_UAV_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1263i32);
pub const D3D12_MESSAGE_ID_SETDEPTHBOUNDS_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1043i32);
pub const D3D12_MESSAGE_ID_SETEVENTONMULTIPLEFENCECOMPLETION_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(975i32);
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(24i32);
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(21i32);
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_NO_ACCESS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1320i32);
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(25i32);
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PAGEABLE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(999i32);
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PRIORITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1009i32);
pub const D3D12_MESSAGE_ID_SETSAMPLEPOSITIONS_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1049i32);
pub const D3D12_MESSAGE_ID_SETTING_SHADING_RATE_FROM_MS_REQUIRES_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1325i32);
pub const D3D12_MESSAGE_ID_SETVIEWINSTANCEMASK_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1094i32);
pub const D3D12_MESSAGE_ID_SET_BACKGROUND_PROCESSING_MODE_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1223i32);
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_HEAP_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(554i32);
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_TABLE_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(708i32);
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(725i32);
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(715i32);
pub const D3D12_MESSAGE_ID_SET_PIPELINE_STACK_SIZE_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1198i32);
pub const D3D12_MESSAGE_ID_SET_PREDICATION_INVALID_PARAMETERS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(734i32);
pub const D3D12_MESSAGE_ID_SET_PROGRAM_ERROR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(120i32);
pub const D3D12_MESSAGE_ID_SET_RENDER_TARGETS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(728i32);
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(710i32);
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(709i32);
pub const D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(711i32);
pub const D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(712i32);
pub const D3D12_MESSAGE_ID_SET_SCISSOR_RECTS_INVALID_RECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1338i32);
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(727i32);
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(717i32);
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(726i32);
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID_DESC: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(713i32);
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_DEVELOPERMODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1286i32);
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_IGNOREDFLAG: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1289i32);
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_INVALIDFLAGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1287i32);
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_SHADERCACHECLEAR_NOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1327i32);
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_STATEALREADYSET: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1288i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_CORRUPT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1294i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_DISABLED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1295i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_FINDVALUE_NOTFOUND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1293i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_SHADERCACHEDELETE_NOTSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1326i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_ALREADYPRESENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1290i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_CACHEFULL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1292i32);
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_HASHCOLLISION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1291i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_ARRAY_SIZE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1239i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1238i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_MIP_LEVEL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1240i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_COUNT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1241i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_QUALITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1242i32);
pub const D3D12_MESSAGE_ID_SHADING_RATE_SOURCE_REQUIRES_DIMENSION_TEXTURE2D: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1339i32);
pub const D3D12_MESSAGE_ID_STATIC_DESCRIPTOR_INVALID_DESCRIPTOR_CHANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1001i32);
pub const D3D12_MESSAGE_ID_STOREPIPELINE_DUPLICATENAME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(970i32);
pub const D3D12_MESSAGE_ID_STOREPIPELINE_NONAME: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(969i32);
pub const D3D12_MESSAGE_ID_STRING_FROM_APPLICATION: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1i32);
pub const D3D12_MESSAGE_ID_TEXTURE_BARRIER_SUBRESOURCES_OUT_OF_BOUNDS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1355i32);
pub const D3D12_MESSAGE_ID_TIMESTAMPS_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(735i32);
pub const D3D12_MESSAGE_ID_TOO_MANY_NODES_SPECIFIED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(899i32);
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_COMMAND_QUEUE_MISMATCH: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1211i32);
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1212i32);
pub const D3D12_MESSAGE_ID_UNINITIALIZED_META_COMMAND: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1175i32);
pub const D3D12_MESSAGE_ID_UNKNOWN: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(0i32);
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(823i32);
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(829i32);
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(825i32);
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(827i32);
pub const D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(931i32);
pub const D3D12_MESSAGE_ID_UNMAP_RANGE_NOT_EMPTY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(929i32);
pub const D3D12_MESSAGE_ID_UNSUPPORTED_BARRIER_LAYOUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1341i32);
pub const D3D12_MESSAGE_ID_UNUSED_CROSS_EXECUTE_SPLIT_BARRIER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1329i32);
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(493i32);
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_POSSIBLY_MISMATCHING_PROPERTIES: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1154i32);
pub const D3D12_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(959i32);
pub const D3D12_MESSAGE_ID_VARIABLE_SHADING_RATE_NOT_ALLOWED_WITH_TIR: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1231i32);
pub const D3D12_MESSAGE_ID_VERTEX_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1274i32);
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_ESTIMATOR_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1218i32);
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1219i32);
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_FRAME_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1110i32);
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_INVALID_INPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1101i32);
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1114i32);
pub const D3D12_MESSAGE_ID_VIDEO_EXTENSION_COMMAND_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1229i32);
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_FRAMES_INVALID_ARGUMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1113i32);
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_INVALID_INPUT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1107i32);
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_UNSUPPORTED_FORMAT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1109i32);
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_INVALIDARGS: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1096i32);
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_UNSUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1095i32);
pub const D3D12_MESSAGE_ID_VRS_SUM_COMBINER_REQUIRES_CAPABILITY: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1324i32);
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_SIGNAL: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1216i32);
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_WAIT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1217i32);
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_ALIGNMENT: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1092i32);
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_DEST: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1090i32);
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_MODE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1091i32);
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_NOT_SUPPORTED: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1093i32);
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_EMPTYBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(893i32);
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDBOX: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(891i32);
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDHEAP: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(889i32);
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(890i32);
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDSUBRESOURCE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(892i32);
pub const D3D12_MESSAGE_ID_WRITE_COMBINE_PERFORMANCE_WARNING: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(1318i32);
pub const D3D12_MESSAGE_ID_WRONG_COMMAND_ALLOCATOR_TYPE: D3D12_MESSAGE_ID = D3D12_MESSAGE_ID(549i32);
pub const D3D12_MESSAGE_SEVERITY_CORRUPTION: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(0i32);
pub const D3D12_MESSAGE_SEVERITY_ERROR: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(1i32);
pub const D3D12_MESSAGE_SEVERITY_INFO: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(3i32);
pub const D3D12_MESSAGE_SEVERITY_MESSAGE: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(4i32);
pub const D3D12_MESSAGE_SEVERITY_WARNING: D3D12_MESSAGE_SEVERITY = D3D12_MESSAGE_SEVERITY(2i32);
pub const D3D12_META_COMMAND_PARAMETER_FLAG_INPUT: D3D12_META_COMMAND_PARAMETER_FLAGS = D3D12_META_COMMAND_PARAMETER_FLAGS(1i32);
pub const D3D12_META_COMMAND_PARAMETER_FLAG_OUTPUT: D3D12_META_COMMAND_PARAMETER_FLAGS = D3D12_META_COMMAND_PARAMETER_FLAGS(2i32);
pub const D3D12_META_COMMAND_PARAMETER_STAGE_CREATION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(0i32);
pub const D3D12_META_COMMAND_PARAMETER_STAGE_EXECUTION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(2i32);
pub const D3D12_META_COMMAND_PARAMETER_STAGE_INITIALIZATION: D3D12_META_COMMAND_PARAMETER_STAGE = D3D12_META_COMMAND_PARAMETER_STAGE(1i32);
pub const D3D12_META_COMMAND_PARAMETER_TYPE_CPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(3i32);
pub const D3D12_META_COMMAND_PARAMETER_TYPE_FLOAT: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(0i32);
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(4i32);
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_VIRTUAL_ADDRESS: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(2i32);
pub const D3D12_META_COMMAND_PARAMETER_TYPE_UINT64: D3D12_META_COMMAND_PARAMETER_TYPE = D3D12_META_COMMAND_PARAMETER_TYPE(1i32);
pub const D3D12_MINOR_VERSION: u32 = 0u32;
pub const D3D12_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D12_MIN_DEPTH: f32 = 0f32;
pub const D3D12_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D12_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D12_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D12_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D12_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D12_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D12_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(0i32);
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(1i32);
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_NONE: D3D12_MULTIPLE_FENCE_WAIT_FLAGS = D3D12_MULTIPLE_FENCE_WAIT_FLAGS(0i32);
pub const D3D12_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS = D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(0i32);
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS = D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(1i32);
pub const D3D12_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967288u32;
pub const D3D12_PACKED_TILE: u32 = 4294967295u32;
pub const D3D12_PIPELINE_STATE_FLAG_DYNAMIC_DEPTH_BIAS: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(4i32);
pub const D3D12_PIPELINE_STATE_FLAG_DYNAMIC_INDEX_BUFFER_STRIP_CUT: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(8i32);
pub const D3D12_PIPELINE_STATE_FLAG_NONE: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(0i32);
pub const D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG: D3D12_PIPELINE_STATE_FLAGS = D3D12_PIPELINE_STATE_FLAGS(1i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(24i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(8i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(19i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(6i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(11i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(21i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL2: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(26i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(16i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(3i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(20i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(5i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(4i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(13i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(12i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MAX_VALID: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(29i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(25i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(18i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(14i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(2i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(10i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER1: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(27i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER2: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(28i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(15i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(0i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(17i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(9i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(7i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(22i32);
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(1i32);
pub const D3D12_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub const D3D12_PREDICATION_OP_EQUAL_ZERO: D3D12_PREDICATION_OP = D3D12_PREDICATION_OP(0i32);
pub const D3D12_PREDICATION_OP_NOT_EQUAL_ZERO: D3D12_PREDICATION_OP = D3D12_PREDICATION_OP(1i32);
pub const D3D12_PREVIEW_SDK_VERSION: u32 = 712u32;
pub const D3D12_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(2i32);
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(4i32);
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(1i32);
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(3i32);
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED: D3D12_PRIMITIVE_TOPOLOGY_TYPE = D3D12_PRIMITIVE_TOPOLOGY_TYPE(0i32);
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(1i32);
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(2i32);
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(0i32);
pub const D3D12_PROTECTED_RESOURCES_SESSION_HARDWARE_PROTECTED: windows_core::GUID = windows_core::GUID::from_u128(0x62b0084e_c70e_4daa_a109_30ff8d5a0482);
pub const D3D12_PROTECTED_RESOURCE_SESSION_FLAG_NONE: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_FLAGS(0i32);
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_NONE: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(0i32);
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS = D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(1i32);
pub const D3D12_PROTECTED_SESSION_STATUS_INVALID: D3D12_PROTECTED_SESSION_STATUS = D3D12_PROTECTED_SESSION_STATUS(1i32);
pub const D3D12_PROTECTED_SESSION_STATUS_OK: D3D12_PROTECTED_SESSION_STATUS = D3D12_PROTECTED_SESSION_STATUS(0i32);
pub const D3D12_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
pub const D3D12_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D12_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D12_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D12_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D12_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub const D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(5i32);
pub const D3D12_QUERY_HEAP_TYPE_OCCLUSION: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(0i32);
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(2i32);
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(7i32);
pub const D3D12_QUERY_HEAP_TYPE_SO_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(3i32);
pub const D3D12_QUERY_HEAP_TYPE_TIMESTAMP: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(1i32);
pub const D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS: D3D12_QUERY_HEAP_TYPE = D3D12_QUERY_HEAP_TYPE(4i32);
pub const D3D12_QUERY_TYPE_BINARY_OCCLUSION: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(1i32);
pub const D3D12_QUERY_TYPE_OCCLUSION: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(0i32);
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(3i32);
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS1: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(10i32);
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(4i32);
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(5i32);
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(6i32);
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(7i32);
pub const D3D12_QUERY_TYPE_TIMESTAMP: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(2i32);
pub const D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS: D3D12_QUERY_TYPE = D3D12_QUERY_TYPE(8i32);
pub const D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_RAYTRACING_AABB_BYTE_ALIGNMENT: u32 = 8u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(2i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(1i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(16i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(0i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(32i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(8i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(4i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_CLONE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(0i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_COMPACT: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(1i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_DESERIALIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(4i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_SERIALIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(3i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_VISUALIZATION_DECODE_FOR_TOOLS: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(2i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(0i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(3i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(2i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(1i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(1i32);
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(0i32);
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NONE: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(0i32);
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(2i32);
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE: D3D12_RAYTRACING_GEOMETRY_FLAGS = D3D12_RAYTRACING_GEOMETRY_FLAGS(1i32);
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS: D3D12_RAYTRACING_GEOMETRY_TYPE = D3D12_RAYTRACING_GEOMETRY_TYPE(1i32);
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES: D3D12_RAYTRACING_GEOMETRY_TYPE = D3D12_RAYTRACING_GEOMETRY_TYPE(0i32);
pub const D3D12_RAYTRACING_INSTANCE_DESCS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_NON_OPAQUE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(8i32);
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_OPAQUE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(4i32);
pub const D3D12_RAYTRACING_INSTANCE_FLAG_NONE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(0i32);
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(1i32);
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE: D3D12_RAYTRACING_INSTANCE_FLAGS = D3D12_RAYTRACING_INSTANCE_FLAGS(2i32);
pub const D3D12_RAYTRACING_MAX_ATTRIBUTE_SIZE_IN_BYTES: u32 = 32u32;
pub const D3D12_RAYTRACING_MAX_DECLARABLE_TRACE_RECURSION_DEPTH: u32 = 31u32;
pub const D3D12_RAYTRACING_MAX_GEOMETRIES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
pub const D3D12_RAYTRACING_MAX_INSTANCES_PER_TOP_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
pub const D3D12_RAYTRACING_MAX_PRIMITIVES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 536870912u32;
pub const D3D12_RAYTRACING_MAX_RAY_GENERATION_SHADER_THREADS: u32 = 1073741824u32;
pub const D3D12_RAYTRACING_MAX_SHADER_RECORD_STRIDE: u32 = 4096u32;
pub const D3D12_RAYTRACING_PIPELINE_FLAG_NONE: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(0i32);
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(512i32);
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_TRIANGLES: D3D12_RAYTRACING_PIPELINE_FLAGS = D3D12_RAYTRACING_PIPELINE_FLAGS(256i32);
pub const D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT: u32 = 32u32;
pub const D3D12_RAYTRACING_SHADER_TABLE_BYTE_ALIGNMENT: u32 = 64u32;
pub const D3D12_RAYTRACING_TIER_1_0: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(10i32);
pub const D3D12_RAYTRACING_TIER_1_1: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(11i32);
pub const D3D12_RAYTRACING_TIER_NOT_SUPPORTED: D3D12_RAYTRACING_TIER = D3D12_RAYTRACING_TIER(0i32);
pub const D3D12_RAYTRACING_TRANSFORM3X4_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_RAY_FLAG_ACCEPT_FIRST_HIT_AND_END_SEARCH: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(4i32);
pub const D3D12_RAY_FLAG_CULL_BACK_FACING_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(16i32);
pub const D3D12_RAY_FLAG_CULL_FRONT_FACING_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(32i32);
pub const D3D12_RAY_FLAG_CULL_NON_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(128i32);
pub const D3D12_RAY_FLAG_CULL_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(64i32);
pub const D3D12_RAY_FLAG_FORCE_NON_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(2i32);
pub const D3D12_RAY_FLAG_FORCE_OPAQUE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(1i32);
pub const D3D12_RAY_FLAG_NONE: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(0i32);
pub const D3D12_RAY_FLAG_SKIP_CLOSEST_HIT_SHADER: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(8i32);
pub const D3D12_RAY_FLAG_SKIP_PROCEDURAL_PRIMITIVES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(512i32);
pub const D3D12_RAY_FLAG_SKIP_TRIANGLES: D3D12_RAY_FLAGS = D3D12_RAY_FLAGS(256i32);
pub const D3D12_RECREATE_AT_TIER_1: D3D12_RECREATE_AT_TIER = D3D12_RECREATE_AT_TIER(1i32);
pub const D3D12_RECREATE_AT_TIER_NOT_SUPPORTED: D3D12_RECREATE_AT_TIER = D3D12_RECREATE_AT_TIER(0i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(2i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(0i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(3i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(1i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE_LOCAL_RENDER: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(4i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE_LOCAL_SRV: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(5i32);
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE_LOCAL_UAV: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(6i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(0i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(3i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(1i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE_LOCAL_RENDER: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(4i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE_LOCAL_SRV: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(5i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE_LOCAL_UAV: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(6i32);
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_RESOLVE: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(2i32);
pub const D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(1i32);
pub const D3D12_RENDER_PASS_FLAG_BIND_READ_ONLY_DEPTH: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(8i32);
pub const D3D12_RENDER_PASS_FLAG_BIND_READ_ONLY_STENCIL: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(16i32);
pub const D3D12_RENDER_PASS_FLAG_NONE: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(0i32);
pub const D3D12_RENDER_PASS_FLAG_RESUMING_PASS: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(4i32);
pub const D3D12_RENDER_PASS_FLAG_SUSPENDING_PASS: D3D12_RENDER_PASS_FLAGS = D3D12_RENDER_PASS_FLAGS(2i32);
pub const D3D12_RENDER_PASS_TIER_0: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(0i32);
pub const D3D12_RENDER_PASS_TIER_1: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(1i32);
pub const D3D12_RENDER_PASS_TIER_2: D3D12_RENDER_PASS_TIER = D3D12_RENDER_PASS_TIER(2i32);
pub const D3D12_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D12_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D12_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D12_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D12_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D12_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D12_REQ_MIP_LEVELS: u32 = 15u32;
pub const D3D12_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D12_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
pub const D3D12_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
pub const D3D12_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_SUBRESOURCES: u32 = 30720u32;
pub const D3D12_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
pub const D3D12_RESIDENCY_FLAG_DENY_OVERBUDGET: D3D12_RESIDENCY_FLAGS = D3D12_RESIDENCY_FLAGS(1i32);
pub const D3D12_RESIDENCY_FLAG_NONE: D3D12_RESIDENCY_FLAGS = D3D12_RESIDENCY_FLAGS(0i32);
pub const D3D12_RESIDENCY_PRIORITY_HIGH: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(-1610547200i32);
pub const D3D12_RESIDENCY_PRIORITY_LOW: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(1342177280i32);
pub const D3D12_RESIDENCY_PRIORITY_MAXIMUM: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(-939524096i32);
pub const D3D12_RESIDENCY_PRIORITY_MINIMUM: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(671088640i32);
pub const D3D12_RESIDENCY_PRIORITY_NORMAL: D3D12_RESIDENCY_PRIORITY = D3D12_RESIDENCY_PRIORITY(2013265920i32);
pub const D3D12_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub const D3D12_RESOLVE_MODE_AVERAGE: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(3i32);
pub const D3D12_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(5i32);
pub const D3D12_RESOLVE_MODE_DECOMPRESS: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(0i32);
pub const D3D12_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(4i32);
pub const D3D12_RESOLVE_MODE_MAX: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(2i32);
pub const D3D12_RESOLVE_MODE_MIN: D3D12_RESOLVE_MODE = D3D12_RESOLVE_MODE(1i32);
pub const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 4294967295u32;
pub const D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(1i32);
pub const D3D12_RESOURCE_BARRIER_FLAG_END_ONLY: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(2i32);
pub const D3D12_RESOURCE_BARRIER_FLAG_NONE: D3D12_RESOURCE_BARRIER_FLAGS = D3D12_RESOURCE_BARRIER_FLAGS(0i32);
pub const D3D12_RESOURCE_BARRIER_TYPE_ALIASING: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(1i32);
pub const D3D12_RESOURCE_BARRIER_TYPE_TRANSITION: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(0i32);
pub const D3D12_RESOURCE_BARRIER_TYPE_UAV: D3D12_RESOURCE_BARRIER_TYPE = D3D12_RESOURCE_BARRIER_TYPE(2i32);
pub const D3D12_RESOURCE_BINDING_TIER_1: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(1i32);
pub const D3D12_RESOURCE_BINDING_TIER_2: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(2i32);
pub const D3D12_RESOURCE_BINDING_TIER_3: D3D12_RESOURCE_BINDING_TIER = D3D12_RESOURCE_BINDING_TIER(3i32);
pub const D3D12_RESOURCE_DIMENSION_BUFFER: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(1i32);
pub const D3D12_RESOURCE_DIMENSION_TEXTURE1D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(2i32);
pub const D3D12_RESOURCE_DIMENSION_TEXTURE2D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(3i32);
pub const D3D12_RESOURCE_DIMENSION_TEXTURE3D: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(4i32);
pub const D3D12_RESOURCE_DIMENSION_UNKNOWN: D3D12_RESOURCE_DIMENSION = D3D12_RESOURCE_DIMENSION(0i32);
pub const D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(16i32);
pub const D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(2i32);
pub const D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(1i32);
pub const D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(32i32);
pub const D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(4i32);
pub const D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(8i32);
pub const D3D12_RESOURCE_FLAG_NONE: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(0i32);
pub const D3D12_RESOURCE_FLAG_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(256i32);
pub const D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(64i32);
pub const D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY: D3D12_RESOURCE_FLAGS = D3D12_RESOURCE_FLAGS(128i32);
pub const D3D12_RESOURCE_HEAP_TIER_1: D3D12_RESOURCE_HEAP_TIER = D3D12_RESOURCE_HEAP_TIER(1i32);
pub const D3D12_RESOURCE_HEAP_TIER_2: D3D12_RESOURCE_HEAP_TIER = D3D12_RESOURCE_HEAP_TIER(2i32);
pub const D3D12_RESOURCE_STATE_ALL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(192i32);
pub const D3D12_RESOURCE_STATE_COMMON: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(0i32);
pub const D3D12_RESOURCE_STATE_COPY_DEST: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1024i32);
pub const D3D12_RESOURCE_STATE_COPY_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2048i32);
pub const D3D12_RESOURCE_STATE_DEPTH_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(32i32);
pub const D3D12_RESOURCE_STATE_DEPTH_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(16i32);
pub const D3D12_RESOURCE_STATE_GENERIC_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2755i32);
pub const D3D12_RESOURCE_STATE_INDEX_BUFFER: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2i32);
pub const D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(512i32);
pub const D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(64i32);
pub const D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(128i32);
pub const D3D12_RESOURCE_STATE_PREDICATION: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(512i32);
pub const D3D12_RESOURCE_STATE_PRESENT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(0i32);
pub const D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4194304i32);
pub const D3D12_RESOURCE_STATE_RENDER_TARGET: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4i32);
pub const D3D12_RESOURCE_STATE_RESERVED_INTERNAL_100000: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1048576i32);
pub const D3D12_RESOURCE_STATE_RESERVED_INTERNAL_4000: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(16384i32);
pub const D3D12_RESOURCE_STATE_RESERVED_INTERNAL_40000000: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1073741824i32);
pub const D3D12_RESOURCE_STATE_RESERVED_INTERNAL_8000: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(32768i32);
pub const D3D12_RESOURCE_STATE_RESERVED_INTERNAL_80000000: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(-2147483648i32);
pub const D3D12_RESOURCE_STATE_RESOLVE_DEST: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(4096i32);
pub const D3D12_RESOURCE_STATE_RESOLVE_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8192i32);
pub const D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(16777216i32);
pub const D3D12_RESOURCE_STATE_STREAM_OUT: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(256i32);
pub const D3D12_RESOURCE_STATE_UNORDERED_ACCESS: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8i32);
pub const D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(1i32);
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(65536i32);
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(131072i32);
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(2097152i32);
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(8388608i32);
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(262144i32);
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE: D3D12_RESOURCE_STATES = D3D12_RESOURCE_STATES(524288i32);
pub const D3D12_RLDO_DETAIL: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(2i32);
pub const D3D12_RLDO_IGNORE_INTERNAL: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(4i32);
pub const D3D12_RLDO_NONE: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(0i32);
pub const D3D12_RLDO_SUMMARY: D3D12_RLDO_FLAGS = D3D12_RLDO_FLAGS(1i32);
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(8i32);
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(4i32);
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(2i32);
pub const D3D12_ROOT_DESCRIPTOR_FLAG_NONE: D3D12_ROOT_DESCRIPTOR_FLAGS = D3D12_ROOT_DESCRIPTOR_FLAGS(0i32);
pub const D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(1i32);
pub const D3D12_ROOT_PARAMETER_TYPE_CBV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(2i32);
pub const D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(0i32);
pub const D3D12_ROOT_PARAMETER_TYPE_SRV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(3i32);
pub const D3D12_ROOT_PARAMETER_TYPE_UAV: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE(4i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(1i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(64i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(1024i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(256i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(8i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(16i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(4i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(512i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(32i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(2i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(128i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_NONE: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(0i32);
pub const D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED: D3D12_ROOT_SIGNATURE_FLAGS = D3D12_ROOT_SIGNATURE_FLAGS(2048i32);
pub const D3D12_RS_SET_SHADING_RATE_COMBINER_COUNT: u32 = 2u32;
pub const D3D12_RTV_DIMENSION_BUFFER: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(1i32);
pub const D3D12_RTV_DIMENSION_TEXTURE1D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(2i32);
pub const D3D12_RTV_DIMENSION_TEXTURE1DARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(3i32);
pub const D3D12_RTV_DIMENSION_TEXTURE2D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(4i32);
pub const D3D12_RTV_DIMENSION_TEXTURE2DARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(5i32);
pub const D3D12_RTV_DIMENSION_TEXTURE2DMS: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(6i32);
pub const D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(7i32);
pub const D3D12_RTV_DIMENSION_TEXTURE3D: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(8i32);
pub const D3D12_RTV_DIMENSION_UNKNOWN: D3D12_RTV_DIMENSION = D3D12_RTV_DIMENSION(0i32);
pub const D3D12_SAMPLER_FEEDBACK_TIER_0_9: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(90i32);
pub const D3D12_SAMPLER_FEEDBACK_TIER_1_0: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(100i32);
pub const D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED: D3D12_SAMPLER_FEEDBACK_TIER = D3D12_SAMPLER_FEEDBACK_TIER(0i32);
pub const D3D12_SAMPLER_FLAG_NONE: D3D12_SAMPLER_FLAGS = D3D12_SAMPLER_FLAGS(0i32);
pub const D3D12_SAMPLER_FLAG_NON_NORMALIZED_COORDINATES: D3D12_SAMPLER_FLAGS = D3D12_SAMPLER_FLAGS(2i32);
pub const D3D12_SAMPLER_FLAG_UINT_BORDER_COLOR: D3D12_SAMPLER_FLAGS = D3D12_SAMPLER_FLAGS(1i32);
pub const D3D12_SDK_VERSION: u32 = 611u32;
pub const D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_SERIALIZED_DATA_TYPE = D3D12_SERIALIZED_DATA_TYPE(0i32);
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_CLEAR: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(4i32);
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_DISABLE: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(1i32);
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_ENABLE: D3D12_SHADER_CACHE_CONTROL_FLAGS = D3D12_SHADER_CACHE_CONTROL_FLAGS(2i32);
pub const D3D12_SHADER_CACHE_FLAG_DRIVER_VERSIONED: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(1i32);
pub const D3D12_SHADER_CACHE_FLAG_NONE: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(0i32);
pub const D3D12_SHADER_CACHE_FLAG_USE_WORKING_DIR: D3D12_SHADER_CACHE_FLAGS = D3D12_SHADER_CACHE_FLAGS(2i32);
pub const D3D12_SHADER_CACHE_KIND_FLAG_APPLICATION_MANAGED: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(8i32);
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CACHE_FOR_DRIVER: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(1i32);
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CONVERSIONS: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(2i32);
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_DRIVER_MANAGED: D3D12_SHADER_CACHE_KIND_FLAGS = D3D12_SHADER_CACHE_KIND_FLAGS(4i32);
pub const D3D12_SHADER_CACHE_MODE_DISK: D3D12_SHADER_CACHE_MODE = D3D12_SHADER_CACHE_MODE(1i32);
pub const D3D12_SHADER_CACHE_MODE_MEMORY: D3D12_SHADER_CACHE_MODE = D3D12_SHADER_CACHE_MODE(0i32);
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(8i32);
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(4i32);
pub const D3D12_SHADER_CACHE_SUPPORT_DRIVER_MANAGED_CACHE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(16i32);
pub const D3D12_SHADER_CACHE_SUPPORT_LIBRARY: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(2i32);
pub const D3D12_SHADER_CACHE_SUPPORT_NONE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(0i32);
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_CONTROL_CLEAR: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(32i32);
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_SESSION_DELETE: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(64i32);
pub const D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO: D3D12_SHADER_CACHE_SUPPORT_FLAGS = D3D12_SHADER_CACHE_SUPPORT_FLAGS(1i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES: u32 = 4096u32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_0: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(4i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_1: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(5i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(0i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(1i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(2i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3: D3D12_SHADER_COMPONENT_MAPPING = D3D12_SHADER_COMPONENT_MAPPING(3i32);
pub const D3D12_SHADER_COMPONENT_MAPPING_MASK: u32 = 7u32;
pub const D3D12_SHADER_COMPONENT_MAPPING_SHIFT: u32 = 3u32;
pub const D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES: u32 = 32u32;
pub const D3D12_SHADER_MAJOR_VERSION: u32 = 5u32;
pub const D3D12_SHADER_MAX_INSTANCES: u32 = 65535u32;
pub const D3D12_SHADER_MAX_INTERFACES: u32 = 253u32;
pub const D3D12_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
pub const D3D12_SHADER_MAX_TYPES: u32 = 65535u32;
pub const D3D12_SHADER_MINOR_VERSION: u32 = 1u32;
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(1i32);
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(2i32);
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE: D3D12_SHADER_MIN_PRECISION_SUPPORT = D3D12_SHADER_MIN_PRECISION_SUPPORT(0i32);
pub const D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(0i32);
pub const D3D12_SHADER_VISIBILITY_AMPLIFICATION: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(6i32);
pub const D3D12_SHADER_VISIBILITY_DOMAIN: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(3i32);
pub const D3D12_SHADER_VISIBILITY_GEOMETRY: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(4i32);
pub const D3D12_SHADER_VISIBILITY_HULL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(2i32);
pub const D3D12_SHADER_VISIBILITY_MESH: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(7i32);
pub const D3D12_SHADER_VISIBILITY_PIXEL: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(5i32);
pub const D3D12_SHADER_VISIBILITY_VERTEX: D3D12_SHADER_VISIBILITY = D3D12_SHADER_VISIBILITY(1i32);
pub const D3D12_SHADING_RATE_1X1: D3D12_SHADING_RATE = D3D12_SHADING_RATE(0i32);
pub const D3D12_SHADING_RATE_1X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(1i32);
pub const D3D12_SHADING_RATE_2X1: D3D12_SHADING_RATE = D3D12_SHADING_RATE(4i32);
pub const D3D12_SHADING_RATE_2X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(5i32);
pub const D3D12_SHADING_RATE_2X4: D3D12_SHADING_RATE = D3D12_SHADING_RATE(6i32);
pub const D3D12_SHADING_RATE_4X2: D3D12_SHADING_RATE = D3D12_SHADING_RATE(9i32);
pub const D3D12_SHADING_RATE_4X4: D3D12_SHADING_RATE = D3D12_SHADING_RATE(10i32);
pub const D3D12_SHADING_RATE_COMBINER_MAX: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(3i32);
pub const D3D12_SHADING_RATE_COMBINER_MIN: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(2i32);
pub const D3D12_SHADING_RATE_COMBINER_OVERRIDE: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(1i32);
pub const D3D12_SHADING_RATE_COMBINER_PASSTHROUGH: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(0i32);
pub const D3D12_SHADING_RATE_COMBINER_SUM: D3D12_SHADING_RATE_COMBINER = D3D12_SHADING_RATE_COMBINER(4i32);
pub const D3D12_SHADING_RATE_VALID_MASK: u32 = 3u32;
pub const D3D12_SHADING_RATE_X_AXIS_SHIFT: u32 = 2u32;
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(0i32);
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(1i32);
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(2i32);
pub const D3D12_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D12_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
pub const D3D12_SHVER_AMPLIFICATION_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(14i32);
pub const D3D12_SHVER_ANY_HIT_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(9i32);
pub const D3D12_SHVER_CALLABLE_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(12i32);
pub const D3D12_SHVER_CLOSEST_HIT_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(10i32);
pub const D3D12_SHVER_COMPUTE_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(5i32);
pub const D3D12_SHVER_DOMAIN_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(4i32);
pub const D3D12_SHVER_GEOMETRY_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(2i32);
pub const D3D12_SHVER_HULL_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(3i32);
pub const D3D12_SHVER_INTERSECTION_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(8i32);
pub const D3D12_SHVER_LIBRARY: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(6i32);
pub const D3D12_SHVER_MESH_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(13i32);
pub const D3D12_SHVER_MISS_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(11i32);
pub const D3D12_SHVER_PIXEL_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(0i32);
pub const D3D12_SHVER_RAY_GENERATION_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(7i32);
pub const D3D12_SHVER_RESERVED0: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(65520i32);
pub const D3D12_SHVER_VERTEX_SHADER: D3D12_SHADER_VERSION_TYPE = D3D12_SHADER_VERSION_TYPE(1i32);
pub const D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D12_SMALL_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
pub const D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4096u32;
pub const D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D12_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
pub const D3D12_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D12_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
pub const D3D12_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
pub const D3D12_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
pub const D3D12_SO_STREAM_COUNT: u32 = 4u32;
pub const D3D12_SPEC_DATE_DAY: u32 = 14u32;
pub const D3D12_SPEC_DATE_MONTH: u32 = 11u32;
pub const D3D12_SPEC_DATE_YEAR: u32 = 2014u32;
pub const D3D12_SPEC_VERSION: f64 = 1.16f64;
pub const D3D12_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D12_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D12_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D12_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D12_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D12_SRV_DIMENSION_BUFFER: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(1i32);
pub const D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(11i32);
pub const D3D12_SRV_DIMENSION_TEXTURE1D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(2i32);
pub const D3D12_SRV_DIMENSION_TEXTURE1DARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(3i32);
pub const D3D12_SRV_DIMENSION_TEXTURE2D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(4i32);
pub const D3D12_SRV_DIMENSION_TEXTURE2DARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(5i32);
pub const D3D12_SRV_DIMENSION_TEXTURE2DMS: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(6i32);
pub const D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(7i32);
pub const D3D12_SRV_DIMENSION_TEXTURE3D: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(8i32);
pub const D3D12_SRV_DIMENSION_TEXTURECUBE: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(9i32);
pub const D3D12_SRV_DIMENSION_TEXTURECUBEARRAY: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(10i32);
pub const D3D12_SRV_DIMENSION_UNKNOWN: D3D12_SRV_DIMENSION = D3D12_SRV_DIMENSION(0i32);
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D12_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D12_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D12_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D12_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_EXTERNAL_DEPENDENCIES_ON_LOCAL_DEFINITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(2i32);
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_LOCAL_DEPENDENCIES_ON_EXTERNAL_DEFINITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(1i32);
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_STATE_OBJECT_ADDITIONS: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(4i32);
pub const D3D12_STATE_OBJECT_FLAG_NONE: D3D12_STATE_OBJECT_FLAGS = D3D12_STATE_OBJECT_FLAGS(0i32);
pub const D3D12_STATE_OBJECT_TYPE_COLLECTION: D3D12_STATE_OBJECT_TYPE = D3D12_STATE_OBJECT_TYPE(0i32);
pub const D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE: D3D12_STATE_OBJECT_TYPE = D3D12_STATE_OBJECT_TYPE(3i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(5i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(8i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(6i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(1i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(11i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(2i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_MAX_VALID: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(13i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(3i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(10i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(12i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(9i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(0i32);
pub const D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION: D3D12_STATE_SUBOBJECT_TYPE = D3D12_STATE_SUBOBJECT_TYPE(7i32);
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(1i32);
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK_UINT: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(3i32);
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(2i32);
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE_UINT: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(4i32);
pub const D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK: D3D12_STATIC_BORDER_COLOR = D3D12_STATIC_BORDER_COLOR(0i32);
pub const D3D12_STENCIL_OP_DECR: D3D12_STENCIL_OP = D3D12_STENCIL_OP(8i32);
pub const D3D12_STENCIL_OP_DECR_SAT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(5i32);
pub const D3D12_STENCIL_OP_INCR: D3D12_STENCIL_OP = D3D12_STENCIL_OP(7i32);
pub const D3D12_STENCIL_OP_INCR_SAT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(4i32);
pub const D3D12_STENCIL_OP_INVERT: D3D12_STENCIL_OP = D3D12_STENCIL_OP(6i32);
pub const D3D12_STENCIL_OP_KEEP: D3D12_STENCIL_OP = D3D12_STENCIL_OP(1i32);
pub const D3D12_STENCIL_OP_REPLACE: D3D12_STENCIL_OP = D3D12_STENCIL_OP(3i32);
pub const D3D12_STENCIL_OP_ZERO: D3D12_STENCIL_OP = D3D12_STENCIL_OP(2i32);
pub const D3D12_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D12_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
pub const D3D12_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
pub const D3D12_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
pub const D3D12_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D12_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D12_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D12_TEXTURE_ADDRESS_MODE_BORDER: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(4i32);
pub const D3D12_TEXTURE_ADDRESS_MODE_CLAMP: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(3i32);
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(2i32);
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(5i32);
pub const D3D12_TEXTURE_ADDRESS_MODE_WRAP: D3D12_TEXTURE_ADDRESS_MODE = D3D12_TEXTURE_ADDRESS_MODE(1i32);
pub const D3D12_TEXTURE_BARRIER_FLAG_DISCARD: D3D12_TEXTURE_BARRIER_FLAGS = D3D12_TEXTURE_BARRIER_FLAGS(1i32);
pub const D3D12_TEXTURE_BARRIER_FLAG_NONE: D3D12_TEXTURE_BARRIER_FLAGS = D3D12_TEXTURE_BARRIER_FLAGS(0i32);
pub const D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT: D3D12_TEXTURE_COPY_TYPE = D3D12_TEXTURE_COPY_TYPE(1i32);
pub const D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX: D3D12_TEXTURE_COPY_TYPE = D3D12_TEXTURE_COPY_TYPE(0i32);
pub const D3D12_TEXTURE_DATA_PITCH_ALIGNMENT: u32 = 256u32;
pub const D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT: u32 = 512u32;
pub const D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(3i32);
pub const D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(2i32);
pub const D3D12_TEXTURE_LAYOUT_ROW_MAJOR: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(1i32);
pub const D3D12_TEXTURE_LAYOUT_UNKNOWN: D3D12_TEXTURE_LAYOUT = D3D12_TEXTURE_LAYOUT(0i32);
pub const D3D12_TILED_RESOURCES_TIER_1: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(1i32);
pub const D3D12_TILED_RESOURCES_TIER_2: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(2i32);
pub const D3D12_TILED_RESOURCES_TIER_3: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(3i32);
pub const D3D12_TILED_RESOURCES_TIER_4: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(4i32);
pub const D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED: D3D12_TILED_RESOURCES_TIER = D3D12_TILED_RESOURCES_TIER(0i32);
pub const D3D12_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
pub const D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(2i32);
pub const D3D12_TILE_COPY_FLAG_NONE: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(0i32);
pub const D3D12_TILE_COPY_FLAG_NO_HAZARD: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(1i32);
pub const D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: D3D12_TILE_COPY_FLAGS = D3D12_TILE_COPY_FLAGS(4i32);
pub const D3D12_TILE_MAPPING_FLAG_NONE: D3D12_TILE_MAPPING_FLAGS = D3D12_TILE_MAPPING_FLAGS(0i32);
pub const D3D12_TILE_MAPPING_FLAG_NO_HAZARD: D3D12_TILE_MAPPING_FLAGS = D3D12_TILE_MAPPING_FLAGS(1i32);
pub const D3D12_TILE_RANGE_FLAG_NONE: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(0i32);
pub const D3D12_TILE_RANGE_FLAG_NULL: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(1i32);
pub const D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(4i32);
pub const D3D12_TILE_RANGE_FLAG_SKIP: D3D12_TILE_RANGE_FLAGS = D3D12_TILE_RANGE_FLAGS(2i32);
pub const D3D12_TRACKED_WORKLOAD_MAX_INSTANCES: u32 = 32u32;
pub const D3D12_TRI_STATE_FALSE: D3D12_TRI_STATE = D3D12_TRI_STATE(0i32);
pub const D3D12_TRI_STATE_TRUE: D3D12_TRI_STATE = D3D12_TRI_STATE(1i32);
pub const D3D12_TRI_STATE_UNKNOWN: D3D12_TRI_STATE = D3D12_TRI_STATE(-1i32);
pub const D3D12_UAV_COUNTER_PLACEMENT_ALIGNMENT: u32 = 4096u32;
pub const D3D12_UAV_DIMENSION_BUFFER: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(1i32);
pub const D3D12_UAV_DIMENSION_TEXTURE1D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(2i32);
pub const D3D12_UAV_DIMENSION_TEXTURE1DARRAY: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(3i32);
pub const D3D12_UAV_DIMENSION_TEXTURE2D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(4i32);
pub const D3D12_UAV_DIMENSION_TEXTURE2DARRAY: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(5i32);
pub const D3D12_UAV_DIMENSION_TEXTURE2DMS: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(6i32);
pub const D3D12_UAV_DIMENSION_TEXTURE2DMSARRAY: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(7i32);
pub const D3D12_UAV_DIMENSION_TEXTURE3D: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(8i32);
pub const D3D12_UAV_DIMENSION_UNKNOWN: D3D12_UAV_DIMENSION = D3D12_UAV_DIMENSION(0i32);
pub const D3D12_UAV_SLOT_COUNT: u32 = 64u32;
pub const D3D12_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub const D3D12_VARIABLE_SHADING_RATE_TIER_1: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(1i32);
pub const D3D12_VARIABLE_SHADING_RATE_TIER_2: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(2i32);
pub const D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED: D3D12_VARIABLE_SHADING_RATE_TIER = D3D12_VARIABLE_SHADING_RATE_TIER(0i32);
pub const D3D12_VIDEO_DECODE_MAX_ARGUMENTS: u32 = 10u32;
pub const D3D12_VIDEO_DECODE_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
pub const D3D12_VIDEO_DECODE_MIN_BITSTREAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D12_VIDEO_DECODE_MIN_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D12_VIDEO_DECODE_STATUS_MACROBLOCKS_AFFECTED_UNKNOWN: u32 = 4294967295u32;
pub const D3D12_VIDEO_ENCODER_AV1_INVALID_DPB_RESOURCE_INDEX: u32 = 255u32;
pub const D3D12_VIDEO_ENCODER_AV1_MAX_TILE_COLS: u32 = 64u32;
pub const D3D12_VIDEO_ENCODER_AV1_MAX_TILE_ROWS: u32 = 64u32;
pub const D3D12_VIDEO_ENCODER_AV1_SUPERRES_DENOM_MIN: u32 = 9u32;
pub const D3D12_VIDEO_ENCODER_AV1_SUPERRES_NUM: u32 = 8u32;
pub const D3D12_VIDEO_PROCESS_MAX_FILTERS: u32 = 32u32;
pub const D3D12_VIDEO_PROCESS_STEREO_VIEWS: u32 = 2u32;
pub const D3D12_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D12_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
pub const D3D12_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
pub const D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING: D3D12_VIEW_INSTANCING_FLAGS = D3D12_VIEW_INSTANCING_FLAGS(1i32);
pub const D3D12_VIEW_INSTANCING_FLAG_NONE: D3D12_VIEW_INSTANCING_FLAGS = D3D12_VIEW_INSTANCING_FLAGS(0i32);
pub const D3D12_VIEW_INSTANCING_TIER_1: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(1i32);
pub const D3D12_VIEW_INSTANCING_TIER_2: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(2i32);
pub const D3D12_VIEW_INSTANCING_TIER_3: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(3i32);
pub const D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED: D3D12_VIEW_INSTANCING_TIER = D3D12_VIEW_INSTANCING_TIER(0i32);
pub const D3D12_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_WAVE_MMA_TIER_1_0: D3D12_WAVE_MMA_TIER = D3D12_WAVE_MMA_TIER(10i32);
pub const D3D12_WAVE_MMA_TIER_NOT_SUPPORTED: D3D12_WAVE_MMA_TIER = D3D12_WAVE_MMA_TIER(0i32);
pub const D3D12_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D12_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D12_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_DEFAULT: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(0i32);
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_IN: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(1i32);
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_OUT: D3D12_WRITEBUFFERIMMEDIATE_MODE = D3D12_WRITEBUFFERIMMEDIATE_MODE(2i32);
pub const D3D_HIGHEST_SHADER_MODEL: D3D_SHADER_MODEL = D3D_SHADER_MODEL(104i32);
pub const D3D_ROOT_SIGNATURE_VERSION_1: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(1i32);
pub const D3D_ROOT_SIGNATURE_VERSION_1_0: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(1i32);
pub const D3D_ROOT_SIGNATURE_VERSION_1_1: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(2i32);
pub const D3D_ROOT_SIGNATURE_VERSION_1_2: D3D_ROOT_SIGNATURE_VERSION = D3D_ROOT_SIGNATURE_VERSION(3i32);
pub const D3D_SHADER_FEATURE_ADVANCED_TEXTURE_OPS: u32 = 536870912u32;
pub const D3D_SHADER_FEATURE_WRITEABLE_MSAA_TEXTURES: u32 = 1073741824u32;
pub const D3D_SHADER_MODEL_5_1: D3D_SHADER_MODEL = D3D_SHADER_MODEL(81i32);
pub const D3D_SHADER_MODEL_6_0: D3D_SHADER_MODEL = D3D_SHADER_MODEL(96i32);
pub const D3D_SHADER_MODEL_6_1: D3D_SHADER_MODEL = D3D_SHADER_MODEL(97i32);
pub const D3D_SHADER_MODEL_6_2: D3D_SHADER_MODEL = D3D_SHADER_MODEL(98i32);
pub const D3D_SHADER_MODEL_6_3: D3D_SHADER_MODEL = D3D_SHADER_MODEL(99i32);
pub const D3D_SHADER_MODEL_6_4: D3D_SHADER_MODEL = D3D_SHADER_MODEL(100i32);
pub const D3D_SHADER_MODEL_6_5: D3D_SHADER_MODEL = D3D_SHADER_MODEL(101i32);
pub const D3D_SHADER_MODEL_6_6: D3D_SHADER_MODEL = D3D_SHADER_MODEL(102i32);
pub const D3D_SHADER_MODEL_6_7: D3D_SHADER_MODEL = D3D_SHADER_MODEL(103i32);
pub const D3D_SHADER_MODEL_6_8: D3D_SHADER_MODEL = D3D_SHADER_MODEL(104i32);
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456u32;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608u32;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304u32;
pub const D3D_SHADER_REQUIRES_BARYCENTRICS: u32 = 131072u32;
pub const D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216u32;
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: u32 = 1024u32;
pub const D3D_SHADER_REQUIRES_INT64_OPS: u32 = 32768u32;
pub const D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS: u32 = 262144u32;
pub const D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1: u32 = 1048576u32;
pub const D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432u32;
pub const D3D_SHADER_REQUIRES_ROVS: u32 = 4096u32;
pub const D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864u32;
pub const D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK: u32 = 2097152u32;
pub const D3D_SHADER_REQUIRES_SHADING_RATE: u32 = 524288u32;
pub const D3D_SHADER_REQUIRES_STENCIL_REF: u32 = 512u32;
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048u32;
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192u32;
pub const D3D_SHADER_REQUIRES_VIEW_ID: u32 = 65536u32;
pub const D3D_SHADER_REQUIRES_WAVE_MMA: u32 = 134217728u32;
pub const D3D_SHADER_REQUIRES_WAVE_OPS: u32 = 16384u32;
pub const DXGI_DEBUG_D3D12: windows_core::GUID = windows_core::GUID::from_u128(0xcf59a98c_a950_4326_91ef_9bbaa17bfd95);
pub const LUID_DEFINED: u32 = 1u32;
pub const NUM_D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODES: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE = D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(4i32);
pub const WKPDID_D3DAutoDebugObjectNameW: windows_core::GUID = windows_core::GUID::from_u128(0xd4902e36_757a_4942_9594_b6769afa43cd);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_AUTO_BREADCRUMB_OP(pub i32);
impl windows_core::TypeKind for D3D12_AUTO_BREADCRUMB_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_AUTO_BREADCRUMB_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_AUTO_BREADCRUMB_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_AXIS_SHADING_RATE(pub i32);
impl windows_core::TypeKind for D3D12_AXIS_SHADING_RATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_AXIS_SHADING_RATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_AXIS_SHADING_RATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BACKGROUND_PROCESSING_MODE(pub i32);
impl windows_core::TypeKind for D3D12_BACKGROUND_PROCESSING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BACKGROUND_PROCESSING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BACKGROUND_PROCESSING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BARRIER_ACCESS(pub i32);
impl windows_core::TypeKind for D3D12_BARRIER_ACCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BARRIER_ACCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BARRIER_ACCESS").field(&self.0).finish()
    }
}
impl D3D12_BARRIER_ACCESS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_BARRIER_ACCESS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_BARRIER_ACCESS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_BARRIER_ACCESS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_BARRIER_ACCESS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_BARRIER_ACCESS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BARRIER_LAYOUT(pub i32);
impl windows_core::TypeKind for D3D12_BARRIER_LAYOUT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BARRIER_LAYOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BARRIER_LAYOUT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BARRIER_SYNC(pub i32);
impl windows_core::TypeKind for D3D12_BARRIER_SYNC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BARRIER_SYNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BARRIER_SYNC").field(&self.0).finish()
    }
}
impl D3D12_BARRIER_SYNC {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_BARRIER_SYNC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_BARRIER_SYNC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_BARRIER_SYNC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_BARRIER_SYNC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_BARRIER_SYNC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BARRIER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_BARRIER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BARRIER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BARRIER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BLEND(pub i32);
impl windows_core::TypeKind for D3D12_BLEND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BLEND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BLEND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BLEND_OP(pub i32);
impl windows_core::TypeKind for D3D12_BLEND_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BLEND_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BLEND_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BUFFER_SRV_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_BUFFER_SRV_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BUFFER_SRV_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BUFFER_SRV_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_BUFFER_SRV_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_BUFFER_SRV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_BUFFER_SRV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_BUFFER_UAV_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_BUFFER_UAV_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_BUFFER_UAV_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_BUFFER_UAV_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_BUFFER_UAV_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_BUFFER_UAV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_BUFFER_UAV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CLEAR_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_CLEAR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CLEAR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CLEAR_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_CLEAR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_CLEAR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_CLEAR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COLOR_WRITE_ENABLE(pub i32);
impl windows_core::TypeKind for D3D12_COLOR_WRITE_ENABLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_LIST_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_LIST_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_LIST_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_COMMAND_LIST_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_COMMAND_LIST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_COMMAND_LIST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_LIST_SUPPORT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_LIST_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_LIST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_LIST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_POOL_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_POOL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_POOL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_POOL_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_COMMAND_POOL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_COMMAND_POOL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_COMMAND_POOL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_QUEUE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_QUEUE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_QUEUE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_QUEUE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_COMMAND_QUEUE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_COMMAND_QUEUE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_COMMAND_QUEUE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_QUEUE_PRIORITY(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_QUEUE_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_QUEUE_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_QUEUE_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMMAND_RECORDER_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_COMMAND_RECORDER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMMAND_RECORDER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_RECORDER_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_COMMAND_RECORDER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_COMMAND_RECORDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_COMMAND_RECORDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_COMPARISON_FUNC(pub i32);
impl windows_core::TypeKind for D3D12_COMPARISON_FUNC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_COMPARISON_FUNC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_COMPARISON_FUNC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CONSERVATIVE_RASTERIZATION_MODE(pub i32);
impl windows_core::TypeKind for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CONSERVATIVE_RASTERIZATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CONSERVATIVE_RASTERIZATION_TIER(pub i32);
impl windows_core::TypeKind for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CONSERVATIVE_RASTERIZATION_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CPU_PAGE_PROPERTY(pub i32);
impl windows_core::TypeKind for D3D12_CPU_PAGE_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CPU_PAGE_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CPU_PAGE_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CROSS_NODE_SHARING_TIER(pub i32);
impl windows_core::TypeKind for D3D12_CROSS_NODE_SHARING_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CROSS_NODE_SHARING_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CROSS_NODE_SHARING_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_CULL_MODE(pub i32);
impl windows_core::TypeKind for D3D12_CULL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_CULL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_CULL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEBUG_DEVICE_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_DEVICE_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEBUG_FEATURE(pub i32);
impl windows_core::TypeKind for D3D12_DEBUG_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEBUG_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_FEATURE").field(&self.0).finish()
    }
}
impl D3D12_DEBUG_FEATURE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DEBUG_FEATURE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DEBUG_FEATURE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DEBUG_FEATURE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DEBUG_FEATURE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DEBUG_FEATURE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEPTH_WRITE_MASK(pub i32);
impl windows_core::TypeKind for D3D12_DEPTH_WRITE_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DESCRIPTOR_HEAP_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_HEAP_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DESCRIPTOR_HEAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DESCRIPTOR_HEAP_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_DESCRIPTOR_HEAP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DESCRIPTOR_HEAP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_HEAP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DESCRIPTOR_RANGE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_RANGE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DESCRIPTOR_RANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DESCRIPTOR_RANGE_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_DESCRIPTOR_RANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DESCRIPTOR_RANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_RANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEVICE_FACTORY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DEVICE_FACTORY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEVICE_FACTORY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEVICE_FACTORY_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DEVICE_FACTORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DEVICE_FACTORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DEVICE_FACTORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DEVICE_FACTORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DEVICE_FACTORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DEVICE_FACTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DEVICE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DEVICE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DEVICE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_ALLOCATION_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_DRED_ALLOCATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_ALLOCATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_DEVICE_STATE(pub i32);
impl windows_core::TypeKind for D3D12_DRED_DEVICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_DEVICE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_DEVICE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_ENABLEMENT(pub i32);
impl windows_core::TypeKind for D3D12_DRED_ENABLEMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_ENABLEMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_ENABLEMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DRED_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_PAGE_FAULT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DRED_PAGE_FAULT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_PAGE_FAULT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DRED_PAGE_FAULT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRED_VERSION(pub i32);
impl windows_core::TypeKind for D3D12_DRED_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRED_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRED_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(pub i32);
impl windows_core::TypeKind for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DSV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D12_DSV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DSV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DSV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_DSV_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_DSV_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_DSV_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_DSV_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_DSV_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_DSV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_DSV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_DSV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_DSV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_DSV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_ELEMENTS_LAYOUT(pub i32);
impl windows_core::TypeKind for D3D12_ELEMENTS_LAYOUT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_ELEMENTS_LAYOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_ELEMENTS_LAYOUT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_EXPORT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_EXPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_EXPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_EXPORT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_EXPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_EXPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_EXPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FEATURE(pub i32);
impl windows_core::TypeKind for D3D12_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FENCE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_FENCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FENCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FENCE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_FENCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_FENCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_FENCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FILL_MODE(pub i32);
impl windows_core::TypeKind for D3D12_FILL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FILL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FILTER(pub i32);
impl windows_core::TypeKind for D3D12_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FILTER_REDUCTION_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_FILTER_REDUCTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FILTER_REDUCTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FILTER_REDUCTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FILTER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_FILTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FILTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FILTER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FORMAT_SUPPORT1(pub i32);
impl windows_core::TypeKind for D3D12_FORMAT_SUPPORT1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FORMAT_SUPPORT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FORMAT_SUPPORT1").field(&self.0).finish()
    }
}
impl D3D12_FORMAT_SUPPORT1 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_FORMAT_SUPPORT1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_FORMAT_SUPPORT1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_FORMAT_SUPPORT2(pub i32);
impl windows_core::TypeKind for D3D12_FORMAT_SUPPORT2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_FORMAT_SUPPORT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_FORMAT_SUPPORT2").field(&self.0).finish()
    }
}
impl D3D12_FORMAT_SUPPORT2 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_FORMAT_SUPPORT2 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_FORMAT_SUPPORT2 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_GPU_BASED_VALIDATION_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_GPU_BASED_VALIDATION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_GPU_BASED_VALIDATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_GPU_BASED_VALIDATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_GPU_BASED_VALIDATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_GPU_BASED_VALIDATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(pub i32);
impl windows_core::TypeKind for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_GRAPHICS_STATES(pub i32);
impl windows_core::TypeKind for D3D12_GRAPHICS_STATES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_GRAPHICS_STATES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_GRAPHICS_STATES").field(&self.0).finish()
    }
}
impl D3D12_GRAPHICS_STATES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_GRAPHICS_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_GRAPHICS_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_HEAP_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_HEAP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_HEAP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_HEAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_HEAP_SERIALIZATION_TIER(pub i32);
impl windows_core::TypeKind for D3D12_HEAP_SERIALIZATION_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_HEAP_SERIALIZATION_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_SERIALIZATION_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_HEAP_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_HEAP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_HEAP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_HIT_GROUP_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_HIT_GROUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_HIT_GROUP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_HIT_GROUP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_HIT_KIND(pub i32);
impl windows_core::TypeKind for D3D12_HIT_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_HIT_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_HIT_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(pub i32);
impl windows_core::TypeKind for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_INDEX_BUFFER_STRIP_CUT_VALUE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_INDIRECT_ARGUMENT_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_INDIRECT_ARGUMENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_INDIRECT_ARGUMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_INPUT_CLASSIFICATION(pub i32);
impl windows_core::TypeKind for D3D12_INPUT_CLASSIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_LIFETIME_STATE(pub i32);
impl windows_core::TypeKind for D3D12_LIFETIME_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_LIFETIME_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_LIFETIME_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_LINE_RASTERIZATION_MODE(pub i32);
impl windows_core::TypeKind for D3D12_LINE_RASTERIZATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_LINE_RASTERIZATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_LINE_RASTERIZATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_LOGIC_OP(pub i32);
impl windows_core::TypeKind for D3D12_LOGIC_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_LOGIC_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_LOGIC_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MEASUREMENTS_ACTION(pub i32);
impl windows_core::TypeKind for D3D12_MEASUREMENTS_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MEASUREMENTS_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MEASUREMENTS_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MEMORY_POOL(pub i32);
impl windows_core::TypeKind for D3D12_MEMORY_POOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MEMORY_POOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MEMORY_POOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MESH_SHADER_TIER(pub i32);
impl windows_core::TypeKind for D3D12_MESH_SHADER_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MESH_SHADER_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MESH_SHADER_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MESSAGE_CALLBACK_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_MESSAGE_CALLBACK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_CALLBACK_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_MESSAGE_CALLBACK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_MESSAGE_CALLBACK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_MESSAGE_CALLBACK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_MESSAGE_CALLBACK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MESSAGE_CATEGORY(pub i32);
impl windows_core::TypeKind for D3D12_MESSAGE_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MESSAGE_ID(pub i32);
impl windows_core::TypeKind for D3D12_MESSAGE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MESSAGE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MESSAGE_SEVERITY(pub i32);
impl windows_core::TypeKind for D3D12_MESSAGE_SEVERITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_META_COMMAND_PARAMETER_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_META_COMMAND_PARAMETER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_META_COMMAND_PARAMETER_STAGE(pub i32);
impl windows_core::TypeKind for D3D12_META_COMMAND_PARAMETER_STAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_STAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_STAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_META_COMMAND_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_META_COMMAND_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MULTIPLE_FENCE_WAIT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MULTIPLE_FENCE_WAIT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PIPELINE_STATE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_PIPELINE_STATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PIPELINE_STATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PIPELINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_PIPELINE_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_PIPELINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_PIPELINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PIPELINE_STATE_SUBOBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PREDICATION_OP(pub i32);
impl windows_core::TypeKind for D3D12_PREDICATION_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PREDICATION_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PREDICATION_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PRIMITIVE_TOPOLOGY_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PRIMITIVE_TOPOLOGY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(pub i32);
impl windows_core::TypeKind for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_RESOURCE_SESSION_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_PROTECTED_SESSION_STATUS(pub i32);
impl windows_core::TypeKind for D3D12_PROTECTED_SESSION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_PROTECTED_SESSION_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_SESSION_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_QUERY_HEAP_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_QUERY_HEAP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_QUERY_HEAP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_QUERY_HEAP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_QUERY_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_QUERY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_QUERY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_QUERY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_GEOMETRY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_GEOMETRY_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RAYTRACING_GEOMETRY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_GEOMETRY_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_GEOMETRY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_INSTANCE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_INSTANCE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RAYTRACING_INSTANCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_PIPELINE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_PIPELINE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RAYTRACING_PIPELINE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAYTRACING_TIER(pub i32);
impl windows_core::TypeKind for D3D12_RAYTRACING_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAYTRACING_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RAY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RAY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RAY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RAY_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RAY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RAY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RAY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RAY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RAY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RAY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RECREATE_AT_TIER(pub i32);
impl windows_core::TypeKind for D3D12_RECREATE_AT_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RECREATE_AT_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RECREATE_AT_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_ENDING_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RENDER_PASS_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RENDER_PASS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RENDER_PASS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RENDER_PASS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RENDER_PASS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RENDER_PASS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RENDER_PASS_TIER(pub i32);
impl windows_core::TypeKind for D3D12_RENDER_PASS_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RENDER_PASS_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESIDENCY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RESIDENCY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESIDENCY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESIDENCY_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RESIDENCY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RESIDENCY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RESIDENCY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESIDENCY_PRIORITY(pub i32);
impl windows_core::TypeKind for D3D12_RESIDENCY_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESIDENCY_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESIDENCY_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOLVE_MODE(pub i32);
impl windows_core::TypeKind for D3D12_RESOLVE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOLVE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOLVE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_BARRIER_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_BARRIER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_BARRIER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BARRIER_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RESOURCE_BARRIER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RESOURCE_BARRIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RESOURCE_BARRIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_BARRIER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_BARRIER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_BARRIER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BARRIER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_BINDING_TIER(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_BINDING_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_BINDING_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BINDING_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RESOURCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_HEAP_TIER(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_HEAP_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_HEAP_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_HEAP_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RESOURCE_STATES(pub i32);
impl windows_core::TypeKind for D3D12_RESOURCE_STATES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RESOURCE_STATES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_STATES").field(&self.0).finish()
    }
}
impl D3D12_RESOURCE_STATES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RESOURCE_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RESOURCE_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RLDO_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_RLDO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RLDO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RLDO_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_RLDO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_RLDO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_RLDO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_RLDO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_RLDO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_RLDO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_ROOT_DESCRIPTOR_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_DESCRIPTOR_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_ROOT_DESCRIPTOR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_ROOT_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_ROOT_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_ROOT_PARAMETER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_ROOT_SIGNATURE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_ROOT_SIGNATURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_ROOT_SIGNATURE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_SIGNATURE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_ROOT_SIGNATURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_ROOT_SIGNATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_ROOT_SIGNATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_RTV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D12_RTV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_RTV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_RTV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SAMPLER_FEEDBACK_TIER(pub i32);
impl windows_core::TypeKind for D3D12_SAMPLER_FEEDBACK_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SAMPLER_FEEDBACK_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SAMPLER_FEEDBACK_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SAMPLER_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_SAMPLER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SAMPLER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SAMPLER_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_SAMPLER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SAMPLER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SAMPLER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SAMPLER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SAMPLER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SAMPLER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SERIALIZED_DATA_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_SERIALIZED_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SERIALIZED_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SERIALIZED_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_CACHE_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_SHADER_CACHE_CONTROL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_CACHE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_CACHE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_CACHE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_SHADER_CACHE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SHADER_CACHE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SHADER_CACHE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_CACHE_KIND_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_CACHE_KIND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_KIND_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_SHADER_CACHE_KIND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_CACHE_MODE(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_CACHE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_CACHE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_CACHE_SUPPORT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_COMPONENT_MAPPING(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_COMPONENT_MAPPING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_COMPONENT_MAPPING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_COMPONENT_MAPPING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_MIN_PRECISION_SUPPORT(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_MIN_PRECISION_SUPPORT").field(&self.0).finish()
    }
}
impl D3D12_SHADER_MIN_PRECISION_SUPPORT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_VERSION_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_VERSION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_VERSION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_VERSION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADER_VISIBILITY(pub i32);
impl windows_core::TypeKind for D3D12_SHADER_VISIBILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADER_VISIBILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_VISIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADING_RATE(pub i32);
impl windows_core::TypeKind for D3D12_SHADING_RATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADING_RATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADING_RATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHADING_RATE_COMBINER(pub i32);
impl windows_core::TypeKind for D3D12_SHADING_RATE_COMBINER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHADING_RATE_COMBINER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHADING_RATE_COMBINER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(pub i32);
impl windows_core::TypeKind for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_SRV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D12_SRV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_SRV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_SRV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_STATE_OBJECT_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_STATE_OBJECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_STATE_OBJECT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_STATE_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_STATE_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_STATE_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_STATE_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_STATE_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_STATE_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_STATE_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_STATE_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_STATE_SUBOBJECT_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_STATE_SUBOBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_STATE_SUBOBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_STATE_SUBOBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_STATIC_BORDER_COLOR(pub i32);
impl windows_core::TypeKind for D3D12_STATIC_BORDER_COLOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_STATIC_BORDER_COLOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_STATIC_BORDER_COLOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_STENCIL_OP(pub i32);
impl windows_core::TypeKind for D3D12_STENCIL_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_STENCIL_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_STENCIL_OP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TEXTURE_ADDRESS_MODE(pub i32);
impl windows_core::TypeKind for D3D12_TEXTURE_ADDRESS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TEXTURE_BARRIER_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_TEXTURE_BARRIER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TEXTURE_BARRIER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_BARRIER_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_TEXTURE_BARRIER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_TEXTURE_BARRIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_TEXTURE_BARRIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_TEXTURE_BARRIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_TEXTURE_BARRIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_TEXTURE_BARRIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TEXTURE_COPY_TYPE(pub i32);
impl windows_core::TypeKind for D3D12_TEXTURE_COPY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TEXTURE_COPY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_COPY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TEXTURE_LAYOUT(pub i32);
impl windows_core::TypeKind for D3D12_TEXTURE_LAYOUT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TEXTURE_LAYOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_LAYOUT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TILED_RESOURCES_TIER(pub i32);
impl windows_core::TypeKind for D3D12_TILED_RESOURCES_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TILED_RESOURCES_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TILED_RESOURCES_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TILE_COPY_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_TILE_COPY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TILE_COPY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TILE_COPY_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_TILE_COPY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_TILE_COPY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_TILE_COPY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TILE_MAPPING_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_TILE_MAPPING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TILE_MAPPING_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TILE_MAPPING_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_TILE_MAPPING_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_TILE_MAPPING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_TILE_MAPPING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TILE_RANGE_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_TILE_RANGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TILE_RANGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TILE_RANGE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_TRI_STATE(pub i32);
impl windows_core::TypeKind for D3D12_TRI_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_TRI_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_TRI_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_UAV_DIMENSION(pub i32);
impl windows_core::TypeKind for D3D12_UAV_DIMENSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_UAV_DIMENSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_UAV_DIMENSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_VARIABLE_SHADING_RATE_TIER(pub i32);
impl windows_core::TypeKind for D3D12_VARIABLE_SHADING_RATE_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_VARIABLE_SHADING_RATE_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_VARIABLE_SHADING_RATE_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_VIEW_INSTANCING_FLAGS(pub i32);
impl windows_core::TypeKind for D3D12_VIEW_INSTANCING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_VIEW_INSTANCING_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_VIEW_INSTANCING_FLAGS").field(&self.0).finish()
    }
}
impl D3D12_VIEW_INSTANCING_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D3D12_VIEW_INSTANCING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D3D12_VIEW_INSTANCING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_VIEW_INSTANCING_TIER(pub i32);
impl windows_core::TypeKind for D3D12_VIEW_INSTANCING_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_VIEW_INSTANCING_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_VIEW_INSTANCING_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_WAVE_MMA_TIER(pub i32);
impl windows_core::TypeKind for D3D12_WAVE_MMA_TIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_WAVE_MMA_TIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_WAVE_MMA_TIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D12_WRITEBUFFERIMMEDIATE_MODE(pub i32);
impl windows_core::TypeKind for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D12_WRITEBUFFERIMMEDIATE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D_ROOT_SIGNATURE_VERSION(pub i32);
impl windows_core::TypeKind for D3D_ROOT_SIGNATURE_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D_ROOT_SIGNATURE_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D_ROOT_SIGNATURE_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D3D_SHADER_MODEL(pub i32);
impl windows_core::TypeKind for D3D_SHADER_MODEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D3D_SHADER_MODEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D3D_SHADER_MODEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_AUTO_BREADCRUMB_NODE {
    pub pCommandListDebugNameA: *const u8,
    pub pCommandListDebugNameW: windows_core::PCWSTR,
    pub pCommandQueueDebugNameA: *const u8,
    pub pCommandQueueDebugNameW: windows_core::PCWSTR,
    pub pCommandList: core::mem::ManuallyDrop<Option<ID3D12GraphicsCommandList>>,
    pub pCommandQueue: core::mem::ManuallyDrop<Option<ID3D12CommandQueue>>,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *const u32,
    pub pCommandHistory: *const D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *const D3D12_AUTO_BREADCRUMB_NODE,
}
impl Clone for D3D12_AUTO_BREADCRUMB_NODE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_AUTO_BREADCRUMB_NODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_AUTO_BREADCRUMB_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_AUTO_BREADCRUMB_NODE1 {
    pub pCommandListDebugNameA: *const u8,
    pub pCommandListDebugNameW: windows_core::PCWSTR,
    pub pCommandQueueDebugNameA: *const u8,
    pub pCommandQueueDebugNameW: windows_core::PCWSTR,
    pub pCommandList: core::mem::ManuallyDrop<Option<ID3D12GraphicsCommandList>>,
    pub pCommandQueue: core::mem::ManuallyDrop<Option<ID3D12CommandQueue>>,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *const u32,
    pub pCommandHistory: *const D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *const D3D12_AUTO_BREADCRUMB_NODE1,
    pub BreadcrumbContextsCount: u32,
    pub pBreadcrumbContexts: *mut D3D12_DRED_BREADCRUMB_CONTEXT,
}
impl Clone for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_AUTO_BREADCRUMB_NODE1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_BARRIER_GROUP {
    pub Type: D3D12_BARRIER_TYPE,
    pub NumBarriers: u32,
    pub Anonymous: D3D12_BARRIER_GROUP_0,
}
impl windows_core::TypeKind for D3D12_BARRIER_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BARRIER_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_BARRIER_GROUP_0 {
    pub pGlobalBarriers: *const D3D12_GLOBAL_BARRIER,
    pub pTextureBarriers: *const D3D12_TEXTURE_BARRIER,
    pub pBufferBarriers: *const D3D12_BUFFER_BARRIER,
}
impl windows_core::TypeKind for D3D12_BARRIER_GROUP_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BARRIER_GROUP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BARRIER_SUBRESOURCE_RANGE {
    pub IndexOrFirstMipLevel: u32,
    pub NumMipLevels: u32,
    pub FirstArraySlice: u32,
    pub NumArraySlices: u32,
    pub FirstPlane: u32,
    pub NumPlanes: u32,
}
impl windows_core::TypeKind for D3D12_BARRIER_SUBRESOURCE_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BARRIER_SUBRESOURCE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC; 8],
}
impl windows_core::TypeKind for D3D12_BLEND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl windows_core::TypeKind for D3D12_BOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_BUFFER_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub Offset: u64,
    pub Size: u64,
}
impl Clone for D3D12_BUFFER_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_BUFFER_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BUFFER_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BUFFER_RTV {
    pub FirstElement: u64,
    pub NumElements: u32,
}
impl windows_core::TypeKind for D3D12_BUFFER_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BUFFER_SRV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub Flags: D3D12_BUFFER_SRV_FLAGS,
}
impl windows_core::TypeKind for D3D12_BUFFER_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BUFFER_UAV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub CounterOffsetInBytes: u64,
    pub Flags: D3D12_BUFFER_UAV_FLAGS,
}
impl windows_core::TypeKind for D3D12_BUFFER_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BUFFER_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    pub DestAccelerationStructureData: u64,
    pub Inputs: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS,
    pub SourceAccelerationStructureData: u64,
    pub ScratchAccelerationStructureData: u64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub Flags: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS,
    pub NumDescs: u32,
    pub DescsLayout: D3D12_ELEMENTS_LAYOUT,
    pub Anonymous: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    pub InstanceDescs: u64,
    pub pGeometryDescs: *const D3D12_RAYTRACING_GEOMETRY_DESC,
    pub ppGeometryDescs: *const *const D3D12_RAYTRACING_GEOMETRY_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub NumDescs: u32,
}
impl windows_core::TypeKind for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_CACHED_PIPELINE_STATE {
    pub pCachedBlob: *const core::ffi::c_void,
    pub CachedBlobSizeInBytes: usize,
}
impl windows_core::TypeKind for D3D12_CACHED_PIPELINE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_CACHED_PIPELINE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_CLEAR_VALUE {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Anonymous: D3D12_CLEAR_VALUE_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_CLEAR_VALUE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_CLEAR_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_CLEAR_VALUE_0 {
    pub Color: [f32; 4],
    pub DepthStencil: D3D12_DEPTH_STENCIL_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_CLEAR_VALUE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_CLEAR_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_COMMAND_QUEUE_DESC {
    pub Type: D3D12_COMMAND_LIST_TYPE,
    pub Priority: i32,
    pub Flags: D3D12_COMMAND_QUEUE_FLAGS,
    pub NodeMask: u32,
}
impl windows_core::TypeKind for D3D12_COMMAND_QUEUE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_COMMAND_QUEUE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_COMMAND_SIGNATURE_DESC {
    pub ByteStride: u32,
    pub NumArgumentDescs: u32,
    pub pArgumentDescs: *const D3D12_INDIRECT_ARGUMENT_DESC,
    pub NodeMask: u32,
}
impl windows_core::TypeKind for D3D12_COMMAND_SIGNATURE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_COMMAND_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_COMPUTE_PIPELINE_STATE_DESC {
    pub pRootSignature: core::mem::ManuallyDrop<Option<ID3D12RootSignature>>,
    pub CS: D3D12_SHADER_BYTECODE,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
impl Clone for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_CONSTANT_BUFFER_VIEW_DESC {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
}
impl windows_core::TypeKind for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_CPU_DESCRIPTOR_HANDLE {
    pub ptr: usize,
}
impl windows_core::TypeKind for D3D12_CPU_DESCRIPTOR_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    pub ShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
}
impl windows_core::TypeKind for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    pub MaxMessagesPerCommandList: u32,
    pub DefaultShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
    pub PipelineStateCreateFlags: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS,
}
impl windows_core::TypeKind for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    pub SlowdownFactor: f32,
}
impl windows_core::TypeKind for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D12_STENCIL_OP,
    pub StencilDepthFailOp: D3D12_STENCIL_OP,
    pub StencilPassOp: D3D12_STENCIL_OP,
    pub StencilFunc: D3D12_COMPARISON_FUNC,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCILOP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEPTH_STENCILOP_DESC1 {
    pub StencilFailOp: D3D12_STENCIL_OP,
    pub StencilDepthFailOp: D3D12_STENCIL_OP,
    pub StencilPassOp: D3D12_STENCIL_OP,
    pub StencilFunc: D3D12_COMPARISON_FUNC,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCILOP_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCILOP_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEPTH_STENCIL_DESC1 {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
    pub DepthBoundsTestEnable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCIL_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEPTH_STENCIL_DESC2 {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC1,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC1,
    pub DepthBoundsTestEnable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCIL_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_DEPTH_STENCIL_VALUE {
    pub Depth: f32,
    pub Stencil: u8,
}
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEPTH_STENCIL_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_DSV_DIMENSION,
    pub Flags: D3D12_DSV_FLAGS,
    pub Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D12_TEX1D_DSV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D12_TEX2D_DSV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D12_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DESCRIPTOR_HEAP_DESC {
    pub Type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub NumDescriptors: u32,
    pub Flags: D3D12_DESCRIPTOR_HEAP_FLAGS,
    pub NodeMask: u32,
}
impl windows_core::TypeKind for D3D12_DESCRIPTOR_HEAP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DESCRIPTOR_HEAP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DESCRIPTOR_RANGE {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl windows_core::TypeKind for D3D12_DESCRIPTOR_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DESCRIPTOR_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DESCRIPTOR_RANGE1 {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_DESCRIPTOR_RANGE_FLAGS,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl windows_core::TypeKind for D3D12_DESCRIPTOR_RANGE1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DESCRIPTOR_RANGE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEVICE_CONFIGURATION_DESC {
    pub Flags: D3D12_DEVICE_FLAGS,
    pub GpuBasedValidationFlags: u32,
    pub SDKVersion: u32,
    pub NumEnabledExperimentalFeatures: u32,
}
impl windows_core::TypeKind for D3D12_DEVICE_CONFIGURATION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEVICE_CONFIGURATION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    pub Flags: D3D12_DRED_FLAGS,
    pub pHeadAutoBreadcrumbNode: *mut D3D12_AUTO_BREADCRUMB_NODE,
}
impl windows_core::TypeKind for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    pub DeviceRemovedReason: windows_core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT,
}
impl windows_core::TypeKind for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    pub DeviceRemovedReason: windows_core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT1,
}
impl windows_core::TypeKind for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    pub DeviceRemovedReason: windows_core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT2,
    pub DeviceState: D3D12_DRED_DEVICE_STATE,
}
impl windows_core::TypeKind for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DISCARD_REGION {
    pub NumRects: u32,
    pub pRects: *const super::super::Foundation::RECT,
    pub FirstSubresource: u32,
    pub NumSubresources: u32,
}
impl windows_core::TypeKind for D3D12_DISCARD_REGION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DISCARD_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DISPATCH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl windows_core::TypeKind for D3D12_DISPATCH_ARGUMENTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DISPATCH_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DISPATCH_MESH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl windows_core::TypeKind for D3D12_DISPATCH_MESH_ARGUMENTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DISPATCH_RAYS_DESC {
    pub RayGenerationShaderRecord: D3D12_GPU_VIRTUAL_ADDRESS_RANGE,
    pub MissShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub HitGroupTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub CallableShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl windows_core::TypeKind for D3D12_DISPATCH_RAYS_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DISPATCH_RAYS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRAW_ARGUMENTS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
impl windows_core::TypeKind for D3D12_DRAW_ARGUMENTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRAW_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRAW_INDEXED_ARGUMENTS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
impl windows_core::TypeKind for D3D12_DRAW_INDEXED_ARGUMENTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_ALLOCATION_NODE {
    pub ObjectNameA: *const u8,
    pub ObjectNameW: windows_core::PCWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *const D3D12_DRED_ALLOCATION_NODE,
}
impl windows_core::TypeKind for D3D12_DRED_ALLOCATION_NODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_ALLOCATION_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_DRED_ALLOCATION_NODE1 {
    pub ObjectNameA: *const u8,
    pub ObjectNameW: windows_core::PCWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pObject: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for D3D12_DRED_ALLOCATION_NODE1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_DRED_ALLOCATION_NODE1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_ALLOCATION_NODE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    pub pHeadAutoBreadcrumbNode: *const D3D12_AUTO_BREADCRUMB_NODE,
}
impl windows_core::TypeKind for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    pub pHeadAutoBreadcrumbNode: *const D3D12_AUTO_BREADCRUMB_NODE1,
}
impl windows_core::TypeKind for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_BREADCRUMB_CONTEXT {
    pub BreadcrumbIndex: u32,
    pub pContextString: windows_core::PCWSTR,
}
impl windows_core::TypeKind for D3D12_DRED_BREADCRUMB_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE,
}
impl windows_core::TypeKind for D3D12_DRED_PAGE_FAULT_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
}
impl windows_core::TypeKind for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *const D3D12_DRED_ALLOCATION_NODE1,
    pub PageFaultFlags: D3D12_DRED_PAGE_FAULT_FLAGS,
}
impl windows_core::TypeKind for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DXIL_LIBRARY_DESC {
    pub DXILLibrary: D3D12_SHADER_BYTECODE,
    pub NumExports: u32,
    pub pExports: *const D3D12_EXPORT_DESC,
}
impl windows_core::TypeKind for D3D12_DXIL_LIBRARY_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DXIL_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub SubobjectToAssociate: windows_core::PCWSTR,
    pub NumExports: u32,
    pub pExports: *const windows_core::PCWSTR,
}
impl windows_core::TypeKind for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_EXISTING_COLLECTION_DESC {
    pub pExistingCollection: core::mem::ManuallyDrop<Option<ID3D12StateObject>>,
    pub NumExports: u32,
    pub pExports: *const D3D12_EXPORT_DESC,
}
impl Clone for D3D12_EXISTING_COLLECTION_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_EXISTING_COLLECTION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_EXISTING_COLLECTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_EXPORT_DESC {
    pub Name: windows_core::PCWSTR,
    pub ExportToRename: windows_core::PCWSTR,
    pub Flags: D3D12_EXPORT_FLAGS,
}
impl windows_core::TypeKind for D3D12_EXPORT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_EXPORT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_ARCHITECTURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE1 {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
    pub IsolatedMMU: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    pub CommandListType: D3D12_COMMAND_LIST_TYPE,
    pub Priority: u32,
    pub PriorityForTypeIsSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_CROSS_NODE {
    pub SharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub AtomicShaderInstructions: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_CROSS_NODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_CROSS_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS {
    pub DoublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
    pub OutputMergerLogicOp: super::super::Foundation::BOOL,
    pub MinPrecisionSupport: D3D12_SHADER_MIN_PRECISION_SUPPORT,
    pub TiledResourcesTier: D3D12_TILED_RESOURCES_TIER,
    pub ResourceBindingTier: D3D12_RESOURCE_BINDING_TIER,
    pub PSSpecifiedStencilRefSupported: super::super::Foundation::BOOL,
    pub TypedUAVLoadAdditionalFormats: super::super::Foundation::BOOL,
    pub ROVsSupported: super::super::Foundation::BOOL,
    pub ConservativeRasterizationTier: D3D12_CONSERVATIVE_RASTERIZATION_TIER,
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub StandardSwizzle64KBSupported: super::super::Foundation::BOOL,
    pub CrossNodeSharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub CrossAdapterRowMajorTextureSupported: super::super::Foundation::BOOL,
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: super::super::Foundation::BOOL,
    pub ResourceHeapTier: D3D12_RESOURCE_HEAP_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    pub WaveOps: super::super::Foundation::BOOL,
    pub WaveLaneCountMin: u32,
    pub WaveLaneCountMax: u32,
    pub TotalLaneCount: u32,
    pub ExpandedComputeResourceStates: super::super::Foundation::BOOL,
    pub Int64ShaderOps: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    pub VariableRateShadingSumCombinerSupported: super::super::Foundation::BOOL,
    pub MeshShaderPerPrimitiveShadingRateSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    pub AtomicInt64OnDescriptorHeapResourceSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS12 {
    pub MSPrimitivesPipelineStatisticIncludesCulledPrimitives: D3D12_TRI_STATE,
    pub EnhancedBarriersSupported: super::super::Foundation::BOOL,
    pub RelaxedFormatCastingSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS12 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS13 {
    pub UnrestrictedBufferTextureCopyPitchSupported: super::super::Foundation::BOOL,
    pub UnrestrictedVertexElementAlignmentSupported: super::super::Foundation::BOOL,
    pub InvertedViewportHeightFlipsYSupported: super::super::Foundation::BOOL,
    pub InvertedViewportDepthFlipsZSupported: super::super::Foundation::BOOL,
    pub TextureCopyBetweenDimensionsSupported: super::super::Foundation::BOOL,
    pub AlphaBlendFactorSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS13 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS14 {
    pub AdvancedTextureOpsSupported: super::super::Foundation::BOOL,
    pub WriteableMSAATexturesSupported: super::super::Foundation::BOOL,
    pub IndependentFrontAndBackStencilRefMaskSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS14 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS15 {
    pub TriangleFanSupported: super::super::Foundation::BOOL,
    pub DynamicIndexBufferStripCutSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS15 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS16 {
    pub DynamicDepthBiasSupported: super::super::Foundation::BOOL,
    pub GPUUploadHeapSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS16 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS17 {
    pub NonNormalizedCoordinateSamplersSupported: super::super::Foundation::BOOL,
    pub ManualWriteTrackingResourceSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS17 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS18 {
    pub RenderPassesValid: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS18 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS19 {
    pub MismatchingOutputDimensionsSupported: super::super::Foundation::BOOL,
    pub SupportedSampleCountsWithNoOutputs: u32,
    pub PointSamplingAddressesNeverRoundUp: super::super::Foundation::BOOL,
    pub RasterizerDesc2Supported: super::super::Foundation::BOOL,
    pub NarrowQuadrilateralLinesSupported: super::super::Foundation::BOOL,
    pub AnisoFilterWithPointMipSupported: super::super::Foundation::BOOL,
    pub MaxSamplerDescriptorHeapSize: u32,
    pub MaxSamplerDescriptorHeapSizeWithStaticSamplers: u32,
    pub MaxViewDescriptorHeapSize: u32,
    pub ComputeOnlyCustomHeapSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS19 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    pub DepthBoundsTestSupported: super::super::Foundation::BOOL,
    pub ProgrammableSamplePositionsTier: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS20 {
    pub ComputeOnlyWriteWatchSupported: super::super::Foundation::BOOL,
    pub RecreateAtTier: D3D12_RECREATE_AT_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS20 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    pub CopyQueueTimestampQueriesSupported: super::super::Foundation::BOOL,
    pub CastingFullyTypedFormatSupported: super::super::Foundation::BOOL,
    pub WriteBufferImmediateSupportFlags: D3D12_COMMAND_LIST_SUPPORT_FLAGS,
    pub ViewInstancingTier: D3D12_VIEW_INSTANCING_TIER,
    pub BarycentricsSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    pub MSAA64KBAlignedTextureSupported: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
    pub Native16BitShaderOpsSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    pub SRVOnlyTiledResourceTier3: super::super::Foundation::BOOL,
    pub RenderPassesTier: D3D12_RENDER_PASS_TIER,
    pub RaytracingTier: D3D12_RAYTRACING_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    pub AdditionalShadingRatesSupported: super::super::Foundation::BOOL,
    pub PerPrimitiveShadingRateSupportedWithViewportIndexing: super::super::Foundation::BOOL,
    pub VariableShadingRateTier: D3D12_VARIABLE_SHADING_RATE_TIER,
    pub ShadingRateImageTileSize: u32,
    pub BackgroundProcessingSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    pub MeshShaderTier: D3D12_MESH_SHADER_TIER,
    pub SamplerFeedbackTier: D3D12_SAMPLER_FEEDBACK_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    pub UnalignedBlockTexturesSupported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    pub MeshShaderPipelineStatsSupported: super::super::Foundation::BOOL,
    pub MeshShaderSupportsFullRangeRenderTargetArrayIndex: super::super::Foundation::BOOL,
    pub AtomicInt64OnTypedResourceSupported: super::super::Foundation::BOOL,
    pub AtomicInt64OnGroupSharedSupported: super::super::Foundation::BOOL,
    pub DerivativesInMeshAndAmplificationShadersSupported: super::super::Foundation::BOOL,
    pub WaveMMATier: D3D12_WAVE_MMA_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_DISPLAYABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_EXISTING_HEAPS {
    pub Supported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_FEATURE_LEVELS {
    pub NumFeatureLevels: u32,
    pub pFeatureLevelsRequested: *const super::Direct3D::D3D_FEATURE_LEVEL,
    pub MaxSupportedFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_FORMAT_INFO {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub PlaneCount: u8,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_FEATURE_DATA_FORMAT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Support1: D3D12_FORMAT_SUPPORT1,
    pub Support2: D3D12_FORMAT_SUPPORT2,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_HARDWARE_COPY {
    pub Supported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_HARDWARE_COPY {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_HARDWARE_COPY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleCount: u32,
    pub Flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    pub NumQualityLevels: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_PLACED_RESOURCE_SUPPORT_INFO {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub DestHeapProperties: D3D12_HEAP_PROPERTIES,
    pub Supported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_FEATURE_DATA_PLACED_RESOURCE_SUPPORT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_FEATURE_DATA_PLACED_RESOURCE_SUPPORT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_PREDICATION {
    pub Supported: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_PREDICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_PREDICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    pub NodeIndex: u32,
    pub Support: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    pub NodeIndex: u32,
    pub Count: u32,
    pub pTypes: *mut windows_core::GUID,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    pub NodeIndex: u32,
    pub Count: u32,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    pub CommandId: windows_core::GUID,
    pub NodeMask: u32,
    pub pQueryInputData: *const core::ffi::c_void,
    pub QueryInputDataSizeInBytes: usize,
    pub pQueryOutputData: *mut core::ffi::c_void,
    pub QueryOutputDataSizeInBytes: usize,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    pub HighestVersion: D3D_ROOT_SIGNATURE_VERSION,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_SERIALIZATION {
    pub NodeIndex: u32,
    pub HeapSerializationTier: D3D12_HEAP_SERIALIZATION_TIER,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_SERIALIZATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_SERIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: D3D12_SHADER_CACHE_SUPPORT_FLAGS,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_SHADER_CACHE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FEATURE_DATA_SHADER_MODEL {
    pub HighestShaderModel: D3D_SHADER_MODEL,
}
impl windows_core::TypeKind for D3D12_FEATURE_DATA_SHADER_MODEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_FUNCTION_DESC {
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
impl windows_core::TypeKind for D3D12_FUNCTION_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_GLOBAL_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
}
impl windows_core::TypeKind for D3D12_GLOBAL_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GLOBAL_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_GLOBAL_ROOT_SIGNATURE {
    pub pGlobalRootSignature: core::mem::ManuallyDrop<Option<ID3D12RootSignature>>,
}
impl Clone for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_GLOBAL_ROOT_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
    pub ptr: u64,
}
impl windows_core::TypeKind for D3D12_GPU_DESCRIPTOR_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    pub StartAddress: u64,
    pub StrideInBytes: u64,
}
impl windows_core::TypeKind for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
    pub StrideInBytes: u64,
}
impl windows_core::TypeKind for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Debug, PartialEq)]
pub struct D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    pub pRootSignature: core::mem::ManuallyDrop<Option<ID3D12RootSignature>>,
    pub VS: D3D12_SHADER_BYTECODE,
    pub PS: D3D12_SHADER_BYTECODE,
    pub DS: D3D12_SHADER_BYTECODE,
    pub HS: D3D12_SHADER_BYTECODE,
    pub GS: D3D12_SHADER_BYTECODE,
    pub StreamOutput: D3D12_STREAM_OUTPUT_DESC,
    pub BlendState: D3D12_BLEND_DESC,
    pub SampleMask: u32,
    pub RasterizerState: D3D12_RASTERIZER_DESC,
    pub DepthStencilState: D3D12_DEPTH_STENCIL_DESC,
    pub InputLayout: D3D12_INPUT_LAYOUT_DESC,
    pub IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub NumRenderTargets: u32,
    pub RTVFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub DSVFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_HEAP_DESC {
    pub SizeInBytes: u64,
    pub Properties: D3D12_HEAP_PROPERTIES,
    pub Alignment: u64,
    pub Flags: D3D12_HEAP_FLAGS,
}
impl windows_core::TypeKind for D3D12_HEAP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_HEAP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_HEAP_PROPERTIES {
    pub Type: D3D12_HEAP_TYPE,
    pub CPUPageProperty: D3D12_CPU_PAGE_PROPERTY,
    pub MemoryPoolPreference: D3D12_MEMORY_POOL,
    pub CreationNodeMask: u32,
    pub VisibleNodeMask: u32,
}
impl windows_core::TypeKind for D3D12_HEAP_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_HEAP_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_HIT_GROUP_DESC {
    pub HitGroupExport: windows_core::PCWSTR,
    pub Type: D3D12_HIT_GROUP_TYPE,
    pub AnyHitShaderImport: windows_core::PCWSTR,
    pub ClosestHitShaderImport: windows_core::PCWSTR,
    pub IntersectionShaderImport: windows_core::PCWSTR,
}
impl windows_core::TypeKind for D3D12_HIT_GROUP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_HIT_GROUP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_INDEX_BUFFER_VIEW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_INDEX_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC {
    pub Type: D3D12_INDIRECT_ARGUMENT_TYPE,
    pub Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_INDIRECT_ARGUMENT_DESC_0 {
    pub VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4,
    pub Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1,
    pub ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0,
    pub ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2,
    pub UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    pub RootParameterIndex: u32,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    pub RootParameterIndex: u32,
    pub DestOffsetIn32BitValues: u32,
    pub Num32BitValuesToSet: u32,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    pub RootParameterIndex: u32,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    pub RootParameterIndex: u32,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    pub Slot: u32,
}
impl windows_core::TypeKind for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INFO_QUEUE_FILTER {
    pub AllowList: D3D12_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D12_INFO_QUEUE_FILTER_DESC,
}
impl windows_core::TypeKind for D3D12_INFO_QUEUE_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D12_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D12_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D12_MESSAGE_ID,
}
impl windows_core::TypeKind for D3D12_INFO_QUEUE_FILTER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D12_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_INPUT_ELEMENT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_INPUT_LAYOUT_DESC {
    pub pInputElementDescs: *const D3D12_INPUT_ELEMENT_DESC,
    pub NumElements: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_INPUT_LAYOUT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_INPUT_LAYOUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_LIBRARY_DESC {
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl windows_core::TypeKind for D3D12_LIBRARY_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_LOCAL_ROOT_SIGNATURE {
    pub pLocalRootSignature: core::mem::ManuallyDrop<Option<ID3D12RootSignature>>,
}
impl Clone for D3D12_LOCAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_LOCAL_ROOT_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_LOCAL_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_MEMCPY_DEST {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: usize,
    pub SlicePitch: usize,
}
impl windows_core::TypeKind for D3D12_MEMCPY_DEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_MEMCPY_DEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_MESSAGE {
    pub Category: D3D12_MESSAGE_CATEGORY,
    pub Severity: D3D12_MESSAGE_SEVERITY,
    pub ID: D3D12_MESSAGE_ID,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl windows_core::TypeKind for D3D12_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_META_COMMAND_DESC {
    pub Id: windows_core::GUID,
    pub Name: windows_core::PCWSTR,
    pub InitializationDirtyState: D3D12_GRAPHICS_STATES,
    pub ExecutionDirtyState: D3D12_GRAPHICS_STATES,
}
impl windows_core::TypeKind for D3D12_META_COMMAND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_META_COMMAND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_META_COMMAND_PARAMETER_DESC {
    pub Name: windows_core::PCWSTR,
    pub Type: D3D12_META_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_META_COMMAND_PARAMETER_FLAGS,
    pub RequiredResourceState: D3D12_RESOURCE_STATES,
    pub StructureOffset: u32,
}
impl windows_core::TypeKind for D3D12_META_COMMAND_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_META_COMMAND_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_MIP_REGION {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl windows_core::TypeKind for D3D12_MIP_REGION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_MIP_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_NODE_MASK {
    pub NodeMask: u32,
}
impl windows_core::TypeKind for D3D12_NODE_MASK {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_NODE_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PACKED_MIP_INFO {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
impl windows_core::TypeKind for D3D12_PACKED_MIP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_PACKED_MIP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PARAMETER_DESC {
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
impl windows_core::TypeKind for D3D12_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PIPELINE_STATE_STREAM_DESC {
    pub SizeInBytes: usize,
    pub pPipelineStateSubobjectStream: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for D3D12_PIPELINE_STATE_STREAM_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    pub Offset: u64,
    pub Footprint: D3D12_SUBRESOURCE_FOOTPRINT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
}
impl windows_core::TypeKind for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
    pub ProtectionType: windows_core::GUID,
}
impl windows_core::TypeKind for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS {
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
impl windows_core::TypeKind for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
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
    pub ASInvocations: u64,
    pub MSInvocations: u64,
    pub MSPrimitives: u64,
}
impl windows_core::TypeKind for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl windows_core::TypeKind for D3D12_QUERY_DATA_SO_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_QUERY_HEAP_DESC {
    pub Type: D3D12_QUERY_HEAP_TYPE,
    pub Count: u32,
    pub NodeMask: u32,
}
impl windows_core::TypeKind for D3D12_QUERY_HEAP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_QUERY_HEAP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RANGE {
    pub Begin: usize,
    pub End: usize,
}
impl windows_core::TypeKind for D3D12_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RANGE_UINT64 {
    pub Begin: u64,
    pub End: u64,
}
impl windows_core::TypeKind for D3D12_RANGE_UINT64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RANGE_UINT64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_RASTERIZER_DESC {
    pub FillMode: D3D12_FILL_MODE,
    pub CullMode: D3D12_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}
impl windows_core::TypeKind for D3D12_RASTERIZER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_RASTERIZER_DESC1 {
    pub FillMode: D3D12_FILL_MODE,
    pub CullMode: D3D12_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: f32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}
impl windows_core::TypeKind for D3D12_RASTERIZER_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RASTERIZER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_RASTERIZER_DESC2 {
    pub FillMode: D3D12_FILL_MODE,
    pub CullMode: D3D12_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: f32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub LineRasterizationMode: D3D12_LINE_RASTERIZATION_MODE,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}
impl windows_core::TypeKind for D3D12_RASTERIZER_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RASTERIZER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_RAYTRACING_AABB {
    pub MinX: f32,
    pub MinY: f32,
    pub MinZ: f32,
    pub MaxX: f32,
    pub MaxY: f32,
    pub MaxZ: f32,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_AABB {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_AABB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    pub CompactedSizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    pub CurrentSizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    pub DestBuffer: u64,
    pub InfoType: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    pub SerializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointers: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    pub DecodedSizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    pub ResultDataMaxSizeInBytes: u64,
    pub ScratchDataSizeInBytes: u64,
    pub UpdateScratchDataSizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    pub Location: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    pub AABBCount: u64,
    pub AABBs: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_RAYTRACING_GEOMETRY_DESC {
    pub Type: D3D12_RAYTRACING_GEOMETRY_TYPE,
    pub Flags: D3D12_RAYTRACING_GEOMETRY_FLAGS,
    pub Anonymous: D3D12_RAYTRACING_GEOMETRY_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RAYTRACING_GEOMETRY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    pub Triangles: D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC,
    pub AABBs: D3D12_RAYTRACING_GEOMETRY_AABBS_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    pub Transform3x4: u64,
    pub IndexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub VertexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub IndexCount: u32,
    pub VertexCount: u32,
    pub IndexBuffer: u64,
    pub VertexBuffer: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_RAYTRACING_INSTANCE_DESC {
    pub Transform: [f32; 12],
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub AccelerationStructure: u64,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_INSTANCE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_INSTANCE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG {
    pub MaxTraceRecursionDepth: u32,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_PIPELINE_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    pub MaxTraceRecursionDepth: u32,
    pub Flags: D3D12_RAYTRACING_PIPELINE_FLAGS,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RAYTRACING_SHADER_CONFIG {
    pub MaxPayloadSizeInBytes: u32,
    pub MaxAttributeSizeInBytes: u32,
}
impl windows_core::TypeKind for D3D12_RAYTRACING_SHADER_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RAYTRACING_SHADER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS {
    pub Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_BEGINNING_ACCESS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    pub Clear: D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS,
    pub PreserveLocal: D3D12_RENDER_PASS_BEGINNING_ACCESS_PRESERVE_LOCAL_PARAMETERS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    pub ClearValue: D3D12_CLEAR_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    pub AdditionalWidth: u32,
    pub AdditionalHeight: u32,
}
impl windows_core::TypeKind for D3D12_RENDER_PASS_BEGINNING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RENDER_PASS_BEGINNING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub DepthBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub StencilBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub DepthEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
    pub StencilEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS {
    pub Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_ENDING_ACCESS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_RENDER_PASS_ENDING_ACCESS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_ENDING_ACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    pub Resolve: core::mem::ManuallyDrop<D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS>,
    pub PreserveLocal: D3D12_RENDER_PASS_ENDING_ACCESS_PRESERVE_LOCAL_PARAMETERS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    pub AdditionalWidth: u32,
    pub AdditionalHeight: u32,
}
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RENDER_PASS_ENDING_ACCESS_PRESERVE_LOCAL_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    pub pSrcResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub pDstResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub SubresourceCount: u32,
    pub pSubresourceParameters: *const D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ResolveMode: D3D12_RESOLVE_MODE,
    pub PreserveResolveSource: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    pub SrcSubresource: u32,
    pub DstSubresource: u32,
    pub DstX: u32,
    pub DstY: u32,
    pub SrcRect: super::super::Foundation::RECT,
}
impl windows_core::TypeKind for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub BeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub EndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub LogicOpEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D12_BLEND,
    pub DestBlend: D3D12_BLEND,
    pub BlendOp: D3D12_BLEND_OP,
    pub SrcBlendAlpha: D3D12_BLEND,
    pub DestBlendAlpha: D3D12_BLEND,
    pub BlendOpAlpha: D3D12_BLEND_OP,
    pub LogicOp: D3D12_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
impl windows_core::TypeKind for D3D12_RENDER_TARGET_BLEND_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_RTV_DIMENSION,
    pub Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_TARGET_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_RTV,
    pub Texture1D: D3D12_TEX1D_RTV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D12_TEX2D_RTV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D12_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D12_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RENDER_TARGET_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_ALIASING_BARRIER {
    pub pResourceBefore: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub pResourceAfter: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
}
impl Clone for D3D12_RESOURCE_ALIASING_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_RESOURCE_ALIASING_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_ALIASING_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_ALLOCATION_INFO {
    pub SizeInBytes: u64,
    pub Alignment: u64,
}
impl windows_core::TypeKind for D3D12_RESOURCE_ALLOCATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_ALLOCATION_INFO1 {
    pub Offset: u64,
    pub Alignment: u64,
    pub SizeInBytes: u64,
}
impl windows_core::TypeKind for D3D12_RESOURCE_ALLOCATION_INFO1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D3D12_RESOURCE_BARRIER {
    pub Type: D3D12_RESOURCE_BARRIER_TYPE,
    pub Flags: D3D12_RESOURCE_BARRIER_FLAGS,
    pub Anonymous: D3D12_RESOURCE_BARRIER_0,
}
impl Clone for D3D12_RESOURCE_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_RESOURCE_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union D3D12_RESOURCE_BARRIER_0 {
    pub Transition: core::mem::ManuallyDrop<D3D12_RESOURCE_TRANSITION_BARRIER>,
    pub Aliasing: core::mem::ManuallyDrop<D3D12_RESOURCE_ALIASING_BARRIER>,
    pub UAV: core::mem::ManuallyDrop<D3D12_RESOURCE_UAV_BARRIER>,
}
impl Clone for D3D12_RESOURCE_BARRIER_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_RESOURCE_BARRIER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_BARRIER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_DESC {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RESOURCE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RESOURCE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_DESC1 {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
    pub SamplerFeedbackMipRegion: D3D12_MIP_REGION,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RESOURCE_DESC1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RESOURCE_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_TRANSITION_BARRIER {
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub Subresource: u32,
    pub StateBefore: D3D12_RESOURCE_STATES,
    pub StateAfter: D3D12_RESOURCE_STATES,
}
impl Clone for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_RESOURCE_TRANSITION_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
}
impl Clone for D3D12_RESOURCE_UAV_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_RESOURCE_UAV_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_RESOURCE_UAV_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_CONSTANTS {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Num32BitValues: u32,
}
impl windows_core::TypeKind for D3D12_ROOT_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_CONSTANTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_DESCRIPTOR {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
}
impl windows_core::TypeKind for D3D12_ROOT_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_DESCRIPTOR1 {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_ROOT_DESCRIPTOR_FLAGS,
}
impl windows_core::TypeKind for D3D12_ROOT_DESCRIPTOR1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_DESCRIPTOR1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const D3D12_DESCRIPTOR_RANGE,
}
impl windows_core::TypeKind for D3D12_ROOT_DESCRIPTOR_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE1 {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *const D3D12_DESCRIPTOR_RANGE1,
}
impl windows_core::TypeKind for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_ROOT_PARAMETER {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl windows_core::TypeKind for D3D12_ROOT_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_ROOT_PARAMETER_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR,
}
impl windows_core::TypeKind for D3D12_ROOT_PARAMETER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_ROOT_PARAMETER1 {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER1_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl windows_core::TypeKind for D3D12_ROOT_PARAMETER1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_PARAMETER1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_ROOT_PARAMETER1_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR1,
}
impl windows_core::TypeKind for D3D12_ROOT_PARAMETER1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_PARAMETER1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub NumParameters: u32,
    pub pParameters: *const D3D12_ROOT_PARAMETER,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl windows_core::TypeKind for D3D12_ROOT_SIGNATURE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_SIGNATURE_DESC1 {
    pub NumParameters: u32,
    pub pParameters: *const D3D12_ROOT_PARAMETER1,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl windows_core::TypeKind for D3D12_ROOT_SIGNATURE_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_SIGNATURE_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_ROOT_SIGNATURE_DESC2 {
    pub NumParameters: u32,
    pub pParameters: *const D3D12_ROOT_PARAMETER1,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *const D3D12_STATIC_SAMPLER_DESC1,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl windows_core::TypeKind for D3D12_ROOT_SIGNATURE_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_ROOT_SIGNATURE_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_RT_FORMAT_ARRAY {
    pub RTFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub NumRenderTargets: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_RT_FORMAT_ARRAY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_RT_FORMAT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl windows_core::TypeKind for D3D12_SAMPLER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_SAMPLER_DESC2 {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub Anonymous: D3D12_SAMPLER_DESC2_0,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub Flags: D3D12_SAMPLER_FLAGS,
}
impl windows_core::TypeKind for D3D12_SAMPLER_DESC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SAMPLER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_SAMPLER_DESC2_0 {
    pub FloatBorderColor: [f32; 4],
    pub UintBorderColor: [u32; 4],
}
impl windows_core::TypeKind for D3D12_SAMPLER_DESC2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SAMPLER_DESC2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SAMPLE_POSITION {
    pub X: i8,
    pub Y: i8,
}
impl windows_core::TypeKind for D3D12_SAMPLE_POSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SAMPLE_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    pub DriverOpaqueGUID: windows_core::GUID,
    pub DriverOpaqueVersioningData: [u8; 16],
}
impl windows_core::TypeKind for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    pub DriverMatchingIdentifier: D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER,
    pub SerializedSizeInBytesIncludingHeader: u64,
    pub DeserializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointersAfterHeader: u64,
}
impl windows_core::TypeKind for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D12_SHADER_BUFFER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_BYTECODE {
    pub pShaderBytecode: *const core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl windows_core::TypeKind for D3D12_SHADER_BYTECODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SHADER_BYTECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_CACHE_SESSION_DESC {
    pub Identifier: windows_core::GUID,
    pub Mode: D3D12_SHADER_CACHE_MODE,
    pub Flags: D3D12_SHADER_CACHE_FLAGS,
    pub MaximumInMemoryCacheSizeBytes: u32,
    pub MaximumInMemoryCacheEntries: u32,
    pub MaximumValueFileSizeBytes: u32,
    pub Version: u64,
}
impl windows_core::TypeKind for D3D12_SHADER_CACHE_SESSION_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SHADER_CACHE_SESSION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_DESC {
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
impl windows_core::TypeKind for D3D12_SHADER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_INPUT_BIND_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
    pub Space: u32,
    pub uID: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for D3D12_SHADER_INPUT_BIND_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_SRV_DIMENSION,
    pub Shader4ComponentMapping: u32,
    pub Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_SHADER_RESOURCE_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_SRV,
    pub Texture1D: D3D12_TEX1D_SRV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D12_TEX2D_SRV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D12_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D12_TEX3D_SRV,
    pub TextureCube: D3D12_TEXCUBE_SRV,
    pub TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV,
    pub RaytracingAccelerationStructure: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_TYPE_DESC {
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
impl windows_core::TypeKind for D3D12_SHADER_TYPE_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SHADER_VARIABLE_DESC {
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
impl windows_core::TypeKind for D3D12_SHADER_VARIABLE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SIGNATURE_PARAMETER_DESC {
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
impl windows_core::TypeKind for D3D12_SIGNATURE_PARAMETER_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3D12_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl windows_core::TypeKind for D3D12_SO_DECLARATION_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_STATE_OBJECT_CONFIG {
    pub Flags: D3D12_STATE_OBJECT_FLAGS,
}
impl windows_core::TypeKind for D3D12_STATE_OBJECT_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STATE_OBJECT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_STATE_OBJECT_DESC {
    pub Type: D3D12_STATE_OBJECT_TYPE,
    pub NumSubobjects: u32,
    pub pSubobjects: *const D3D12_STATE_SUBOBJECT,
}
impl windows_core::TypeKind for D3D12_STATE_OBJECT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STATE_OBJECT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_STATE_SUBOBJECT {
    pub Type: D3D12_STATE_SUBOBJECT_TYPE,
    pub pDesc: *const core::ffi::c_void,
}
impl windows_core::TypeKind for D3D12_STATE_SUBOBJECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STATE_SUBOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_STATIC_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: D3D12_STATIC_BORDER_COLOR,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl windows_core::TypeKind for D3D12_STATIC_SAMPLER_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STATIC_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_STATIC_SAMPLER_DESC1 {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: D3D12_STATIC_BORDER_COLOR,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
    pub Flags: D3D12_SAMPLER_FLAGS,
}
impl windows_core::TypeKind for D3D12_STATIC_SAMPLER_DESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STATIC_SAMPLER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u64,
    pub BufferFilledSizeLocation: u64,
}
impl windows_core::TypeKind for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_STREAM_OUTPUT_DESC {
    pub pSODeclaration: *const D3D12_SO_DECLARATION_ENTRY,
    pub NumEntries: u32,
    pub pBufferStrides: *const u32,
    pub NumStrides: u32,
    pub RasterizedStream: u32,
}
impl windows_core::TypeKind for D3D12_STREAM_OUTPUT_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_STREAM_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub pSubobjectToAssociate: *const D3D12_STATE_SUBOBJECT,
    pub NumExports: u32,
    pub pExports: *const windows_core::PCWSTR,
}
impl windows_core::TypeKind for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBRESOURCE_DATA {
    pub pData: *const core::ffi::c_void,
    pub RowPitch: isize,
    pub SlicePitch: isize,
}
impl windows_core::TypeKind for D3D12_SUBRESOURCE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBRESOURCE_FOOTPRINT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub RowPitch: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_SUBRESOURCE_FOOTPRINT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_SUBRESOURCE_FOOTPRINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBRESOURCE_INFO {
    pub Offset: u64,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl windows_core::TypeKind for D3D12_SUBRESOURCE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SUBRESOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBRESOURCE_RANGE_UINT64 {
    pub Subresource: u32,
    pub Range: D3D12_RANGE_UINT64,
}
impl windows_core::TypeKind for D3D12_SUBRESOURCE_RANGE_UINT64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
impl windows_core::TypeKind for D3D12_SUBRESOURCE_TILING {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_SUBRESOURCE_TILING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEX1D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_ARRAY_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_RTV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEX1D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX1D_UAV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX1D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX1D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_ARRAY_UAV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_ARRAY_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_ARRAY_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2DMS_UAV {
    pub UnusedField_NothingToDefine: u32,
}
impl windows_core::TypeKind for D3D12_TEX2DMS_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2DMS_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_ARRAY_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_ARRAY_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEX2D_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_ARRAY_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_DSV {
    pub MipSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_DSV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_DSV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_RTV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEX2D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX2D_UAV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl windows_core::TypeKind for D3D12_TEX2D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX2D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl windows_core::TypeKind for D3D12_TEX3D_RTV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX3D_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEX3D_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX3D_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl windows_core::TypeKind for D3D12_TEX3D_UAV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEX3D_UAV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEXCUBE_ARRAY_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl windows_core::TypeKind for D3D12_TEXCUBE_SRV {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct D3D12_TEXTURE_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
    pub LayoutBefore: D3D12_BARRIER_LAYOUT,
    pub LayoutAfter: D3D12_BARRIER_LAYOUT,
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub Subresources: D3D12_BARRIER_SUBRESOURCE_RANGE,
    pub Flags: D3D12_TEXTURE_BARRIER_FLAGS,
}
impl Clone for D3D12_TEXTURE_BARRIER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for D3D12_TEXTURE_BARRIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TEXTURE_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_TEXTURE_COPY_LOCATION {
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
    pub Type: D3D12_TEXTURE_COPY_TYPE,
    pub Anonymous: D3D12_TEXTURE_COPY_LOCATION_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Clone for D3D12_TEXTURE_COPY_LOCATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_TEXTURE_COPY_LOCATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_TEXTURE_COPY_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_TEXTURE_COPY_LOCATION_0 {
    pub PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    pub SubresourceIndex: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_TEXTURE_COPY_LOCATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_TEXTURE_COPY_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
impl windows_core::TypeKind for D3D12_TILED_RESOURCE_COORDINATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TILED_RESOURCE_COORDINATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub UseBox: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
impl windows_core::TypeKind for D3D12_TILE_REGION_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TILE_REGION_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
impl windows_core::TypeKind for D3D12_TILE_SHAPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_TILE_SHAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub struct D3D12_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_UAV_DIMENSION,
    pub Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy)]
pub union D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_UAV,
    pub Texture1D: D3D12_TEX1D_UAV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D12_TEX2D_UAV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_UAV,
    pub Texture2DMS: D3D12_TEX2DMS_UAV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_UAV,
    pub Texture3D: D3D12_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    pub Version: D3D12_DRED_VERSION,
    pub Anonymous: D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0,
}
impl windows_core::TypeKind for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    pub Dred_1_0: D3D12_DEVICE_REMOVED_EXTENDED_DATA,
    pub Dred_1_1: D3D12_DEVICE_REMOVED_EXTENDED_DATA1,
    pub Dred_1_2: D3D12_DEVICE_REMOVED_EXTENDED_DATA2,
    pub Dred_1_3: D3D12_DEVICE_REMOVED_EXTENDED_DATA3,
}
impl windows_core::TypeKind for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    pub Version: D3D_ROOT_SIGNATURE_VERSION,
    pub Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0,
}
impl windows_core::TypeKind for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    pub Desc_1_0: D3D12_ROOT_SIGNATURE_DESC,
    pub Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1,
    pub Desc_1_2: D3D12_ROOT_SIGNATURE_DESC2,
}
impl windows_core::TypeKind for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_VERTEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub StrideInBytes: u32,
}
impl windows_core::TypeKind for D3D12_VERTEX_BUFFER_VIEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VERTEX_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl windows_core::TypeKind for D3D12_VIEWPORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VIEWPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_VIEW_INSTANCE_LOCATION {
    pub ViewportArrayIndex: u32,
    pub RenderTargetArrayIndex: u32,
}
impl windows_core::TypeKind for D3D12_VIEW_INSTANCE_LOCATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VIEW_INSTANCE_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_VIEW_INSTANCING_DESC {
    pub ViewInstanceCount: u32,
    pub pViewInstanceLocations: *const D3D12_VIEW_INSTANCE_LOCATION,
    pub Flags: D3D12_VIEW_INSTANCING_FLAGS,
}
impl windows_core::TypeKind for D3D12_VIEW_INSTANCING_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_VIEW_INSTANCING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    pub Dest: u64,
    pub Value: u32,
}
impl windows_core::TypeKind for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
impl Default for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12MessageFunc = Option<unsafe extern "system" fn(category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: windows_core::PCSTR, pcontext: *mut core::ffi::c_void)>;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: Option<windows_core::IUnknown>, param1: super::Direct3D::D3D_FEATURE_LEVEL, param2: *const windows_core::GUID, param3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_D3D12_CREATE_ROOT_SIGNATURE_DESERIALIZER = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const windows_core::GUID, pprootsignaturedeserializer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_D3D12_CREATE_VERSIONED_ROOT_SIGNATURE_DESERIALIZER = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const windows_core::GUID, pprootsignaturedeserializer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_D3D12_GET_DEBUG_INTERFACE = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_D3D12_GET_INTERFACE = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_ROOT_SIGNATURE = Option<unsafe extern "system" fn(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorblob: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE = Option<unsafe extern "system" fn(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorblob: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
