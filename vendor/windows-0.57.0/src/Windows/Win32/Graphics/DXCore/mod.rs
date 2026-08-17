#[inline]
pub unsafe fn DXCoreCreateAdapterFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const windows_core::GUID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    DXCoreCreateAdapterFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IDXCoreAdapter, IDXCoreAdapter_Vtbl, 0xf0db4c7f_fe5a_42a2_bd62_f2a6cf6fc83e);
impl core::ops::Deref for IDXCoreAdapter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXCoreAdapter, windows_core::IUnknown);
impl IDXCoreAdapter {
    pub unsafe fn IsValid(&self) -> bool {
        (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsAttributeSupported(&self, attributeguid: *const windows_core::GUID) -> bool {
        (windows_core::Interface::vtable(self).IsAttributeSupported)(windows_core::Interface::as_raw(self), attributeguid)
    }
    pub unsafe fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool {
        (windows_core::Interface::vtable(self).IsPropertySupported)(windows_core::Interface::as_raw(self), property)
    }
    pub unsafe fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), property, buffersize, propertydata).ok()
    }
    pub unsafe fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertySize)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
    }
    pub unsafe fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool {
        (windows_core::Interface::vtable(self).IsQueryStateSupported)(windows_core::Interface::as_raw(self), property)
    }
    pub unsafe fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: Option<*const core::ffi::c_void>, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryState)(windows_core::Interface::as_raw(self), state, inputstatedetailssize, core::mem::transmute(inputstatedetails.unwrap_or(std::ptr::null())), outputbuffersize, outputbuffer).ok()
    }
    pub unsafe fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool {
        (windows_core::Interface::vtable(self).IsSetStateSupported)(windows_core::Interface::as_raw(self), property)
    }
    pub unsafe fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: Option<*const core::ffi::c_void>, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state, inputstatedetailssize, core::mem::transmute(inputstatedetails.unwrap_or(std::ptr::null())), inputdatasize, inputdata).ok()
    }
    pub unsafe fn GetFactory<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDXCoreAdapter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub IsAttributeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> bool,
    pub IsPropertySupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterProperty) -> bool,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterProperty, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertySize: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterProperty, *mut usize) -> windows_core::HRESULT,
    pub IsQueryStateSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterState) -> bool,
    pub QueryState: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterState, usize, *const core::ffi::c_void, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSetStateSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterState) -> bool,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterState, usize, *const core::ffi::c_void, usize, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDXCoreAdapterFactory, IDXCoreAdapterFactory_Vtbl, 0x78ee5945_c36e_4b13_a669_005dd11c0f06);
impl core::ops::Deref for IDXCoreAdapterFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXCoreAdapterFactory, windows_core::IUnknown);
impl IDXCoreAdapterFactory {
    pub unsafe fn CreateAdapterList<T>(&self, filterattributes: &[windows_core::GUID]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateAdapterList)(windows_core::Interface::as_raw(self), filterattributes.len().try_into().unwrap(), core::mem::transmute(filterattributes.as_ptr()), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAdapterByLuid<T>(&self, adapterluid: *const super::super::Foundation::LUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAdapterByLuid)(windows_core::Interface::as_raw(self), adapterluid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool {
        (windows_core::Interface::vtable(self).IsNotificationTypeSupported)(windows_core::Interface::as_raw(self), notificationtype)
    }
    pub unsafe fn RegisterEventNotification<P0>(&self, dxcoreobject: P0, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterEventNotification)(windows_core::Interface::as_raw(self), dxcoreobject.param().abi(), notificationtype, callbackfunction, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
    pub unsafe fn UnregisterEventNotification(&self, eventcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterEventNotification)(windows_core::Interface::as_raw(self), eventcookie).ok()
    }
}
#[repr(C)]
pub struct IDXCoreAdapterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateAdapterList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterByLuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::LUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsNotificationTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreNotificationType) -> bool,
    pub RegisterEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DXCoreNotificationType, PFN_DXCORE_NOTIFICATION_CALLBACK, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnregisterEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDXCoreAdapterList, IDXCoreAdapterList_Vtbl, 0x526c7776_40e9_459b_b711_f32ad76dfc28);
impl core::ops::Deref for IDXCoreAdapterList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXCoreAdapterList, windows_core::IUnknown);
impl IDXCoreAdapterList {
    pub unsafe fn GetAdapter<T>(&self, index: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), index, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsStale(&self) -> bool {
        (windows_core::Interface::vtable(self).IsStale)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFactory<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Sort(&self, preferences: &[DXCoreAdapterPreference]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), preferences.len().try_into().unwrap(), core::mem::transmute(preferences.as_ptr())).ok()
    }
    pub unsafe fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool {
        (windows_core::Interface::vtable(self).IsAdapterPreferenceSupported)(windows_core::Interface::as_raw(self), preference)
    }
}
#[repr(C)]
pub struct IDXCoreAdapterList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsStale: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sort: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DXCoreAdapterPreference) -> windows_core::HRESULT,
    pub IsAdapterPreferenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterPreference) -> bool,
}
pub const AcgCompatible: DXCoreAdapterProperty = DXCoreAdapterProperty(10u32);
pub const AdapterBudgetChange: DXCoreNotificationType = DXCoreNotificationType(2u32);
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = DXCoreNotificationType(3u32);
pub const AdapterListStale: DXCoreNotificationType = DXCoreNotificationType(0u32);
pub const AdapterMemoryBudget: DXCoreAdapterState = DXCoreAdapterState(1u32);
pub const AdapterNoLongerValid: DXCoreNotificationType = DXCoreNotificationType(1u32);
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(5u32);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: windows_core::GUID = windows_core::GUID::from_u128(0x8c47866b_7583_450d_f0f0_6bada895af4b);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: windows_core::GUID = windows_core::GUID::from_u128(0x248e2800_a793_4724_abaa_23a6de1be090);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: windows_core::GUID = windows_core::GUID::from_u128(0x0c9ece4d_2f6e_4f01_8c96_e89e331b47b1);
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(7u32);
pub const DedicatedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(8u32);
pub const DriverDescription: DXCoreAdapterProperty = DXCoreAdapterProperty(2u32);
pub const DriverVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(1u32);
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(6u32);
pub const Hardware: DXCoreAdapterPreference = DXCoreAdapterPreference(0u32);
pub const HardwareID: DXCoreAdapterProperty = DXCoreAdapterProperty(3u32);
pub const HardwareIDParts: DXCoreAdapterProperty = DXCoreAdapterProperty(14u32);
pub const HighPerformance: DXCoreAdapterPreference = DXCoreAdapterPreference(2u32);
pub const InstanceLuid: DXCoreAdapterProperty = DXCoreAdapterProperty(0u32);
pub const IsDetachable: DXCoreAdapterProperty = DXCoreAdapterProperty(13u32);
pub const IsDriverUpdateInProgress: DXCoreAdapterState = DXCoreAdapterState(0u32);
pub const IsHardware: DXCoreAdapterProperty = DXCoreAdapterProperty(11u32);
pub const IsIntegrated: DXCoreAdapterProperty = DXCoreAdapterProperty(12u32);
pub const KmdModelVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(4u32);
pub const Local: DXCoreSegmentGroup = DXCoreSegmentGroup(0u32);
pub const MinimumPower: DXCoreAdapterPreference = DXCoreAdapterPreference(1u32);
pub const NonLocal: DXCoreSegmentGroup = DXCoreSegmentGroup(1u32);
pub const SharedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(9u32);
pub const _FACDXCORE: u32 = 2176u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DXCoreAdapterPreference(pub u32);
impl windows_core::TypeKind for DXCoreAdapterPreference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DXCoreAdapterPreference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DXCoreAdapterPreference").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DXCoreAdapterProperty(pub u32);
impl windows_core::TypeKind for DXCoreAdapterProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DXCoreAdapterProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DXCoreAdapterProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DXCoreAdapterState(pub u32);
impl windows_core::TypeKind for DXCoreAdapterState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DXCoreAdapterState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DXCoreAdapterState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DXCoreNotificationType(pub u32);
impl windows_core::TypeKind for DXCoreNotificationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DXCoreNotificationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DXCoreNotificationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DXCoreSegmentGroup(pub u32);
impl windows_core::TypeKind for DXCoreSegmentGroup {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DXCoreSegmentGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DXCoreSegmentGroup").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl windows_core::TypeKind for DXCoreAdapterMemoryBudget {
    type TypeKind = windows_core::CopyType;
}
impl Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl windows_core::TypeKind for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    type TypeKind = windows_core::CopyType;
}
impl Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl windows_core::TypeKind for DXCoreHardwareID {
    type TypeKind = windows_core::CopyType;
}
impl Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl windows_core::TypeKind for DXCoreHardwareIDParts {
    type TypeKind = windows_core::CopyType;
}
impl Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: Option<windows_core::IUnknown>, context: *const core::ffi::c_void)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
