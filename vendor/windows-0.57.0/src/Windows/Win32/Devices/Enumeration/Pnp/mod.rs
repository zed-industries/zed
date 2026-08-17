#[inline]
pub unsafe fn SwDeviceClose<P0>(hswdevice: P0)
where
    P0: windows_core::Param<HSWDEVICE>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceClose(hswdevice : HSWDEVICE));
    SwDeviceClose(hswdevice.param().abi())
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<P0, P1>(pszenumeratorname: P0, pszparentdeviceinstance: P1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, pproperties: Option<&[super::super::Properties::DEVPROPERTY]>, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HSWDEVICE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceCreate(pszenumeratorname : windows_core::PCWSTR, pszparentdeviceinstance : windows_core::PCWSTR, pcreateinfo : *const SW_DEVICE_CREATE_INFO, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, pcallback : SW_DEVICE_CREATE_CALLBACK, pcontext : *const core::ffi::c_void, phswdevice : *mut HSWDEVICE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SwDeviceCreate(pszenumeratorname.param().abi(), pszparentdeviceinstance.param().abi(), pcreateinfo, pproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcallback, core::mem::transmute(pcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn SwDeviceGetLifetime<P0>(hswdevice: P0) -> windows_core::Result<SW_DEVICE_LIFETIME>
where
    P0: windows_core::Param<HSWDEVICE>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceGetLifetime(hswdevice : HSWDEVICE, plifetime : *mut SW_DEVICE_LIFETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SwDeviceGetLifetime(hswdevice.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<P0, P1>(hswdevice: P0, pszdeviceinterfaceid: P1, pproperties: &[super::super::Properties::DEVPROPERTY]) -> windows_core::Result<()>
where
    P0: windows_core::Param<HSWDEVICE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfacePropertySet(hswdevice : HSWDEVICE, pszdeviceinterfaceid : windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> windows_core::HRESULT);
    SwDeviceInterfacePropertySet(hswdevice.param().abi(), pszdeviceinterfaceid.param().abi(), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok()
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<P0, P1, P2>(hswdevice: P0, pinterfaceclassguid: *const windows_core::GUID, pszreferencestring: P1, pproperties: Option<&[super::super::Properties::DEVPROPERTY]>, fenabled: P2) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<HSWDEVICE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceRegister(hswdevice : HSWDEVICE, pinterfaceclassguid : *const windows_core::GUID, pszreferencestring : windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, fenabled : super::super::super::Foundation:: BOOL, ppszdeviceinterfaceid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SwDeviceInterfaceRegister(hswdevice.param().abi(), pinterfaceclassguid, pszreferencestring.param().abi(), pproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), fenabled.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<P0, P1, P2>(hswdevice: P0, pszdeviceinterfaceid: P1, fenabled: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<HSWDEVICE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceSetState(hswdevice : HSWDEVICE, pszdeviceinterfaceid : windows_core::PCWSTR, fenabled : super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    SwDeviceInterfaceSetState(hswdevice.param().abi(), pszdeviceinterfaceid.param().abi(), fenabled.param().abi()).ok()
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDevicePropertySet<P0>(hswdevice: P0, pproperties: &[super::super::Properties::DEVPROPERTY]) -> windows_core::Result<()>
where
    P0: windows_core::Param<HSWDEVICE>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDevicePropertySet(hswdevice : HSWDEVICE, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> windows_core::HRESULT);
    SwDevicePropertySet(hswdevice.param().abi(), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok()
}
#[inline]
pub unsafe fn SwDeviceSetLifetime<P0>(hswdevice: P0, lifetime: SW_DEVICE_LIFETIME) -> windows_core::Result<()>
where
    P0: windows_core::Param<HSWDEVICE>,
{
    windows_targets::link!("cfgmgr32.dll" "system" fn SwDeviceSetLifetime(hswdevice : HSWDEVICE, lifetime : SW_DEVICE_LIFETIME) -> windows_core::HRESULT);
    SwDeviceSetLifetime(hswdevice.param().abi(), lifetime).ok()
}
#[inline]
pub unsafe fn SwMemFree(pmem: *const core::ffi::c_void) {
    windows_targets::link!("cfgmgr32.dll" "system" fn SwMemFree(pmem : *const core::ffi::c_void));
    SwMemFree(pmem)
}
windows_core::imp::define_interface!(IUPnPAddressFamilyControl, IUPnPAddressFamilyControl_Vtbl, 0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
impl core::ops::Deref for IUPnPAddressFamilyControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPAddressFamilyControl, windows_core::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetAddressFamily(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAddressFamily)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUPnPAddressFamilyControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPAsyncResult, IUPnPAsyncResult_Vtbl, 0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
impl core::ops::Deref for IUPnPAsyncResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPAsyncResult, windows_core::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AsyncOperationComplete)(windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
}
#[repr(C)]
pub struct IUPnPAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPDescriptionDocument, IUPnPDescriptionDocument_Vtbl, 0x11d1c1b2_7daa_4c9e_9595_7f82ed206d1e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPDescriptionDocument {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPDescriptionDocument, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDescriptionDocument {
    pub unsafe fn ReadyState(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Load<P0>(&self, bstrurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), bstrurl.param().abi()).ok()
    }
    pub unsafe fn LoadAsync<P0, P1>(&self, bstrurl: P0, punkcallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).LoadAsync)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), punkcallback.param().abi()).ok()
    }
    pub unsafe fn LoadResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> windows_core::Result<IUPnPDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceByUDN<P0>(&self, bstrudn: P0) -> windows_core::Result<IUPnPDevice>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceByUDN)(windows_core::Interface::as_raw(self), bstrudn.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDescriptionDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub ReadyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceByUDN: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceByUDN: usize,
}
windows_core::imp::define_interface!(IUPnPDescriptionDocumentCallback, IUPnPDescriptionDocumentCallback_Vtbl, 0x77394c69_5486_40d6_9bc3_4991983e02da);
impl core::ops::Deref for IUPnPDescriptionDocumentCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDescriptionDocumentCallback, windows_core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadComplete)(windows_core::Interface::as_raw(self), hrloadresult).ok()
    }
}
#[repr(C)]
pub struct IUPnPDescriptionDocumentCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPDevice, IUPnPDevice_Vtbl, 0x3d44d0d1_98c9_4889_acd1_f9d674bf2221);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPDevice {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPDevice, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevice {
    pub unsafe fn IsRootDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRootDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootDevice(&self) -> windows_core::Result<IUPnPDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParentDevice(&self) -> windows_core::Result<IUPnPDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasChildren(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> windows_core::Result<IUPnPDevices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UniqueDeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UniqueDeviceName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PresentationURL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PresentationURL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ManufacturerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ManufacturerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ManufacturerURL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ManufacturerURL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModelName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModelName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModelNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModelNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModelURL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModelURL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UPC(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UPC)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IconURL<P0>(&self, bstrencodingformat: P0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IconURL)(windows_core::Interface::as_raw(self), bstrencodingformat.param().abi(), lsizex, lsizey, lbitdepth, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> windows_core::Result<IUPnPServices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDevice_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub IsRootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootDevice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ParentDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParentDevice: usize,
    pub HasChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub UniqueDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PresentationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ManufacturerURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModelURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UPC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IconURL: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
}
windows_core::imp::define_interface!(IUPnPDeviceControl, IUPnPDeviceControl_Vtbl, 0x204810ba_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPDeviceControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceControl, windows_core::IUnknown);
impl IUPnPDeviceControl {
    pub unsafe fn Initialize<P0, P1, P2>(&self, bstrxmldesc: P0, bstrdeviceidentifier: P1, bstrinitstring: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), bstrxmldesc.param().abi(), bstrdeviceidentifier.param().abi(), bstrinitstring.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetServiceObject<P0, P1>(&self, bstrudn: P0, bstrserviceid: P1) -> windows_core::Result<super::super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetServiceObject)(windows_core::Interface::as_raw(self), bstrudn.param().abi(), bstrserviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUPnPDeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetServiceObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetServiceObject: usize,
}
windows_core::imp::define_interface!(IUPnPDeviceControlHttpHeaders, IUPnPDeviceControlHttpHeaders_Vtbl, 0x204810bb_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPDeviceControlHttpHeaders {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceControlHttpHeaders, windows_core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAdditionalResponseHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUPnPDeviceControlHttpHeaders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPDeviceDocumentAccess, IUPnPDeviceDocumentAccess_Vtbl, 0xe7772804_3287_418e_9072_cf2b47238981);
impl core::ops::Deref for IUPnPDeviceDocumentAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccess, windows_core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentURL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUPnPDeviceDocumentAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPDeviceDocumentAccessEx, IUPnPDeviceDocumentAccessEx_Vtbl, 0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
impl core::ops::Deref for IUPnPDeviceDocumentAccessEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccessEx, windows_core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUPnPDeviceDocumentAccessEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPDeviceFinder, IUPnPDeviceFinder_Vtbl, 0xadda3d55_6f72_4319_bff9_18600a539b10);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPDeviceFinder {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPDeviceFinder, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByType<P0>(&self, bstrtypeuri: P0, dwflags: u32) -> windows_core::Result<IUPnPDevices>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindByType)(windows_core::Interface::as_raw(self), bstrtypeuri.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAsyncFind<P0, P1>(&self, bstrtypeuri: P0, dwflags: u32, punkdevicefindercallback: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAsyncFind)(windows_core::Interface::as_raw(self), bstrtypeuri.param().abi(), dwflags, punkdevicefindercallback.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartAsyncFind)(windows_core::Interface::as_raw(self), lfinddata).ok()
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelAsyncFind)(windows_core::Interface::as_raw(self), lfinddata).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindByUDN<P0>(&self, bstrudn: P0) -> windows_core::Result<IUPnPDevice>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindByUDN)(windows_core::Interface::as_raw(self), bstrudn.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDeviceFinder_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByType: usize,
    pub CreateAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FindByUDN: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindByUDN: usize,
}
windows_core::imp::define_interface!(IUPnPDeviceFinderAddCallbackWithInterface, IUPnPDeviceFinderAddCallbackWithInterface_Vtbl, 0x983dfc0b_1796_44df_8975_ca545b620ee5);
impl core::ops::Deref for IUPnPDeviceFinderAddCallbackWithInterface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderAddCallbackWithInterface, windows_core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAddedWithInterface<P0>(&self, lfinddata: i32, pdevice: P0, pguidinterface: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPDevice>,
    {
        (windows_core::Interface::vtable(self).DeviceAddedWithInterface)(windows_core::Interface::as_raw(self), lfinddata, pdevice.param().abi(), pguidinterface).ok()
    }
}
#[repr(C)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAddedWithInterface: usize,
}
windows_core::imp::define_interface!(IUPnPDeviceFinderCallback, IUPnPDeviceFinderCallback_Vtbl, 0x415a984a_88b3_49f3_92af_0508bedf0d6c);
impl core::ops::Deref for IUPnPDeviceFinderCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderCallback, windows_core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAdded<P0>(&self, lfinddata: i32, pdevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPDevice>,
    {
        (windows_core::Interface::vtable(self).DeviceAdded)(windows_core::Interface::as_raw(self), lfinddata, pdevice.param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<P0>(&self, lfinddata: i32, bstrudn: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeviceRemoved)(windows_core::Interface::as_raw(self), lfinddata, bstrudn.param().abi()).ok()
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SearchComplete)(windows_core::Interface::as_raw(self), lfinddata).ok()
    }
}
#[repr(C)]
pub struct IUPnPDeviceFinderCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAdded: usize,
    pub DeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPDeviceProvider, IUPnPDeviceProvider_Vtbl, 0x204810b8_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPDeviceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPDeviceProvider, windows_core::IUnknown);
impl IUPnPDeviceProvider {
    pub unsafe fn Start<P0>(&self, bstrinitstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), bstrinitstring.param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUPnPDeviceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPDevices, IUPnPDevices_Vtbl, 0xfdbc0c73_bda3_4c66_ac4f_f2d96fdad68c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPDevices {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPDevices, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDevices {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, bstrudn: P0) -> windows_core::Result<IUPnPDevice>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), bstrudn.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPDevices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
windows_core::imp::define_interface!(IUPnPEventSink, IUPnPEventSink_Vtbl, 0x204810b4_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPEventSink, windows_core::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, rgdispidchanges: &[i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), rgdispidchanges.len().try_into().unwrap(), core::mem::transmute(rgdispidchanges.as_ptr())).ok()
    }
    pub unsafe fn OnStateChangedSafe<P0>(&self, varsadispidchanges: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).OnStateChangedSafe)(windows_core::Interface::as_raw(self), varsadispidchanges.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUPnPEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32) -> windows_core::HRESULT,
    pub OnStateChangedSafe: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPEventSource, IUPnPEventSource_Vtbl, 0x204810b5_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPEventSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPEventSource, windows_core::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<P0>(&self, pessubscriber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPEventSink>,
    {
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pessubscriber.param().abi()).ok()
    }
    pub unsafe fn Unadvise<P0>(&self, pessubscriber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPEventSink>,
    {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pessubscriber.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUPnPEventSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPHttpHeaderControl, IUPnPHttpHeaderControl_Vtbl, 0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
impl core::ops::Deref for IUPnPHttpHeaderControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPHttpHeaderControl, windows_core::IUnknown);
impl IUPnPHttpHeaderControl {
    pub unsafe fn AddRequestHeaders<P0>(&self, bstrhttpheaders: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRequestHeaders)(windows_core::Interface::as_raw(self), bstrhttpheaders.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUPnPHttpHeaderControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRequestHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPRegistrar, IUPnPRegistrar_Vtbl, 0x204810b6_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPRegistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPRegistrar, windows_core::IUnknown);
impl IUPnPRegistrar {
    pub unsafe fn RegisterDevice<P0, P1, P2, P3, P4>(&self, bstrxmldesc: P0, bstrprogiddevicecontrolclass: P1, bstrinitstring: P2, bstrcontainerid: P3, bstrresourcepath: P4, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterDevice)(windows_core::Interface::as_raw(self), bstrxmldesc.param().abi(), bstrprogiddevicecontrolclass.param().abi(), bstrinitstring.param().abi(), bstrcontainerid.param().abi(), bstrresourcepath.param().abi(), nlifetime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterRunningDevice<P0, P1, P2, P3>(&self, bstrxmldesc: P0, punkdevicecontrol: P1, bstrinitstring: P2, bstrresourcepath: P3, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterRunningDevice)(windows_core::Interface::as_raw(self), bstrxmldesc.param().abi(), punkdevicecontrol.param().abi(), bstrinitstring.param().abi(), bstrresourcepath.param().abi(), nlifetime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterDeviceProvider<P0, P1, P2, P3>(&self, bstrprovidername: P0, bstrprogidproviderclass: P1, bstrinitstring: P2, bstrcontainerid: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterDeviceProvider)(windows_core::Interface::as_raw(self), bstrprovidername.param().abi(), bstrprogidproviderclass.param().abi(), bstrinitstring.param().abi(), bstrcontainerid.param().abi()).ok()
    }
    pub unsafe fn GetUniqueDeviceName<P0, P1>(&self, bstrdeviceidentifier: P0, bstrtemplateudn: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUniqueDeviceName)(windows_core::Interface::as_raw(self), bstrdeviceidentifier.param().abi(), bstrtemplateudn.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterDevice<P0, P1>(&self, bstrdeviceidentifier: P0, fpermanent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UnregisterDevice)(windows_core::Interface::as_raw(self), bstrdeviceidentifier.param().abi(), fpermanent.param().abi()).ok()
    }
    pub unsafe fn UnregisterDeviceProvider<P0>(&self, bstrprovidername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterDeviceProvider)(windows_core::Interface::as_raw(self), bstrprovidername.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUPnPRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RegisterRunningDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetUniqueDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPRemoteEndpointInfo, IUPnPRemoteEndpointInfo_Vtbl, 0xc92eb863_0269_4aff_9c72_75321bba2952);
impl core::ops::Deref for IUPnPRemoteEndpointInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPRemoteEndpointInfo, windows_core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    pub unsafe fn GetDwordValue<P0>(&self, bstrvaluename: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDwordValue)(windows_core::Interface::as_raw(self), bstrvaluename.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStringValue<P0>(&self, bstrvaluename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), bstrvaluename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGuidValue<P0>(&self, bstrvaluename: P0) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGuidValue)(windows_core::Interface::as_raw(self), bstrvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUPnPRemoteEndpointInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDwordValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPReregistrar, IUPnPReregistrar_Vtbl, 0x204810b7_73b2_11d4_bf42_00b0d0118b56);
impl core::ops::Deref for IUPnPReregistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPReregistrar, windows_core::IUnknown);
impl IUPnPReregistrar {
    pub unsafe fn ReregisterDevice<P0, P1, P2, P3, P4, P5>(&self, bstrdeviceidentifier: P0, bstrxmldesc: P1, bstrprogiddevicecontrolclass: P2, bstrinitstring: P3, bstrcontainerid: P4, bstrresourcepath: P5, nlifetime: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ReregisterDevice)(windows_core::Interface::as_raw(self), bstrdeviceidentifier.param().abi(), bstrxmldesc.param().abi(), bstrprogiddevicecontrolclass.param().abi(), bstrinitstring.param().abi(), bstrcontainerid.param().abi(), bstrresourcepath.param().abi(), nlifetime).ok()
    }
    pub unsafe fn ReregisterRunningDevice<P0, P1, P2, P3, P4>(&self, bstrdeviceidentifier: P0, bstrxmldesc: P1, punkdevicecontrol: P2, bstrinitstring: P3, bstrresourcepath: P4, nlifetime: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ReregisterRunningDevice)(windows_core::Interface::as_raw(self), bstrdeviceidentifier.param().abi(), bstrxmldesc.param().abi(), punkdevicecontrol.param().abi(), bstrinitstring.param().abi(), bstrresourcepath.param().abi(), nlifetime).ok()
    }
}
#[repr(C)]
pub struct IUPnPReregistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ReregisterRunningDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPService, IUPnPService_Vtbl, 0xa295019c_dc65_47dd_90dc_7fe918a1ab44);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPService {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPService, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPService {
    pub unsafe fn QueryStateVariable<P0>(&self, bstrvariablename: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryStateVariable)(windows_core::Interface::as_raw(self), bstrvariablename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InvokeAction<P0, P1>(&self, bstractionname: P0, vinactionargs: P1, pvoutactionargs: *mut windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InvokeAction)(windows_core::Interface::as_raw(self), bstractionname.param().abi(), vinactionargs.param().abi(), core::mem::transmute(pvoutactionargs), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceTypeIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceTypeIdentifier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddCallback<P0>(&self, punkcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddCallback)(windows_core::Interface::as_raw(self), punkcallback.param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastTransportStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastTransportStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPService_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub QueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub InvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ServiceTypeIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastTransportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPServiceAsync, IUPnPServiceAsync_Vtbl, 0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
impl core::ops::Deref for IUPnPServiceAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPServiceAsync, windows_core::IUnknown);
impl IUPnPServiceAsync {
    pub unsafe fn BeginInvokeAction<P0, P1, P2>(&self, bstractionname: P0, vinactionargs: P1, pasyncresult: P2) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<IUPnPAsyncResult>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginInvokeAction)(windows_core::Interface::as_raw(self), bstractionname.param().abi(), vinactionargs.param().abi(), pasyncresult.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut windows_core::VARIANT, pvretval: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndInvokeAction)(windows_core::Interface::as_raw(self), ullrequestid, core::mem::transmute(pvoutactionargs), core::mem::transmute(pvretval)).ok()
    }
    pub unsafe fn BeginQueryStateVariable<P0, P1>(&self, bstrvariablename: P0, pasyncresult: P1) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IUPnPAsyncResult>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginQueryStateVariable)(windows_core::Interface::as_raw(self), bstrvariablename.param().abi(), pasyncresult.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndQueryStateVariable)(windows_core::Interface::as_raw(self), ullrequestid, core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn BeginSubscribeToEvents<P0, P1>(&self, punkcallback: P0, pasyncresult: P1) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IUPnPAsyncResult>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginSubscribeToEvents)(windows_core::Interface::as_raw(self), punkcallback.param().abi(), pasyncresult.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndSubscribeToEvents)(windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
    pub unsafe fn BeginSCPDDownload<P0>(&self, pasyncresult: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IUPnPAsyncResult>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginSCPDDownload)(windows_core::Interface::as_raw(self), pasyncresult.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndSCPDDownload)(windows_core::Interface::as_raw(self), ullrequestid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelAsyncOperation)(windows_core::Interface::as_raw(self), ullrequestid).ok()
    }
}
#[repr(C)]
pub struct IUPnPServiceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginInvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndInvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub BeginQueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndQueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndSCPDDownload: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CancelAsyncOperation: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPServiceCallback, IUPnPServiceCallback_Vtbl, 0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
impl core::ops::Deref for IUPnPServiceCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPServiceCallback, windows_core::IUnknown);
impl IUPnPServiceCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StateVariableChanged<P0, P1, P2>(&self, pus: P0, pcwszstatevarname: P1, vavalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPService>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).StateVariableChanged)(windows_core::Interface::as_raw(self), pus.param().abi(), pcwszstatevarname.param().abi(), vavalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceInstanceDied<P0>(&self, pus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPService>,
    {
        (windows_core::Interface::vtable(self).ServiceInstanceDied)(windows_core::Interface::as_raw(self), pus.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUPnPServiceCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub StateVariableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StateVariableChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceInstanceDied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceInstanceDied: usize,
}
windows_core::imp::define_interface!(IUPnPServiceDocumentAccess, IUPnPServiceDocumentAccess_Vtbl, 0x21905529_0a5e_4589_825d_7e6d87ea6998);
impl core::ops::Deref for IUPnPServiceDocumentAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPServiceDocumentAccess, windows_core::IUnknown);
impl IUPnPServiceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentURL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUPnPServiceDocumentAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUPnPServiceEnumProperty, IUPnPServiceEnumProperty_Vtbl, 0x38873b37_91bb_49f4_b249_2e8efbb8a816);
impl core::ops::Deref for IUPnPServiceEnumProperty {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUPnPServiceEnumProperty, windows_core::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetServiceEnumProperty)(windows_core::Interface::as_raw(self), dwmask).ok()
    }
}
#[repr(C)]
pub struct IUPnPServiceEnumProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetServiceEnumProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPServices, IUPnPServices_Vtbl, 0x3f8c8e9e_9a7a_4dc8_bc41_ff31fa374956);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPServices {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPServices, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPServices {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, bstrserviceid: P0) -> windows_core::Result<IUPnPService>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), bstrserviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPServices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
pub const ADDRESS_FAMILY_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AddressFamily");
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
pub const REMOTE_ADDRESS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RemoteAddress");
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
pub const UPNP_E_ACTION_REQUEST_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040210_u32 as _);
pub const UPNP_E_ACTION_SPECIFIC_BASE: windows_core::HRESULT = windows_core::HRESULT(0x80040300_u32 as _);
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const UPNP_E_DEVICE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80040214_u32 as _);
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const UPNP_E_DEVICE_NOTREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8004A032_u32 as _);
pub const UPNP_E_DEVICE_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004A031_u32 as _);
pub const UPNP_E_DEVICE_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040217_u32 as _);
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8004A021_u32 as _);
pub const UPNP_E_DUPLICATE_SERVICE_ID: windows_core::HRESULT = windows_core::HRESULT(0x8004A022_u32 as _);
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x80040216_u32 as _);
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040501_u32 as _);
pub const UPNP_E_ICON_ELEMENT_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const UPNP_E_ICON_NODE_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const UPNP_E_INVALID_ACTION: windows_core::HRESULT = windows_core::HRESULT(0x80040207_u32 as _);
pub const UPNP_E_INVALID_ARGUMENTS: windows_core::HRESULT = windows_core::HRESULT(0x80040208_u32 as _);
pub const UPNP_E_INVALID_DESCRIPTION: windows_core::HRESULT = windows_core::HRESULT(0x8004A023_u32 as _);
pub const UPNP_E_INVALID_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80040500_u32 as _);
pub const UPNP_E_INVALID_ICON: windows_core::HRESULT = windows_core::HRESULT(0x8004A025_u32 as _);
pub const UPNP_E_INVALID_ROOT_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x8004A027_u32 as _);
pub const UPNP_E_INVALID_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8004A024_u32 as _);
pub const UPNP_E_INVALID_VARIABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040213_u32 as _);
pub const UPNP_E_INVALID_XML: windows_core::HRESULT = windows_core::HRESULT(0x8004A026_u32 as _);
pub const UPNP_E_OUT_OF_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x80040209_u32 as _);
pub const UPNP_E_PROTOCOL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80040215_u32 as _);
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004A020_u32 as _);
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const UPNP_E_SUFFIX_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x8004A028_u32 as _);
pub const UPNP_E_TRANSPORT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80040211_u32 as _);
pub const UPNP_E_URLBASE_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8004A029_u32 as _);
pub const UPNP_E_VALUE_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x8004A030_u32 as _);
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80040212_u32 as _);
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
impl windows_core::TypeKind for SW_DEVICE_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SW_DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SW_DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SW_DEVICE_LIFETIME(pub i32);
impl windows_core::TypeKind for SW_DEVICE_LIFETIME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SW_DEVICE_LIFETIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SW_DEVICE_LIFETIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSWDEVICE(pub isize);
impl HSWDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HSWDEVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: windows_core::PCWSTR,
    pub pszzHardwareIds: windows_core::PCWSTR,
    pub pszzCompatibleIds: windows_core::PCWSTR,
    pub pContainerId: *const windows_core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: windows_core::PCWSTR,
    pub pszDeviceLocation: windows_core::PCWSTR,
    pub pSecurityDescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for SW_DEVICE_CREATE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UPnPDescriptionDocument: windows_core::GUID = windows_core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
pub const UPnPDescriptionDocumentEx: windows_core::GUID = windows_core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
pub const UPnPDevice: windows_core::GUID = windows_core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
pub const UPnPDeviceFinder: windows_core::GUID = windows_core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
pub const UPnPDeviceFinderEx: windows_core::GUID = windows_core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
pub const UPnPDevices: windows_core::GUID = windows_core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
pub const UPnPRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
pub const UPnPRemoteEndpointInfo: windows_core::GUID = windows_core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
pub const UPnPService: windows_core::GUID = windows_core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
pub const UPnPServices: windows_core::GUID = windows_core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
pub type SW_DEVICE_CREATE_CALLBACK = Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: windows_core::HRESULT, pcontext: *const core::ffi::c_void, pszdeviceinstanceid: windows_core::PCWSTR)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
