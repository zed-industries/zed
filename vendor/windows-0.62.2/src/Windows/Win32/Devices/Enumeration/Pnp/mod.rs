#[inline]
pub unsafe fn SwDeviceClose(hswdevice: HSWDEVICE) {
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceClose(hswdevice : HSWDEVICE));
    unsafe { SwDeviceClose(hswdevice) }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<P0, P1>(pszenumeratorname: P0, pszparentdeviceinstance: P1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, pproperties: Option<&[super::super::Properties::DEVPROPERTY]>, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HSWDEVICE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceCreate(pszenumeratorname : windows_core::PCWSTR, pszparentdeviceinstance : windows_core::PCWSTR, pcreateinfo : *const SW_DEVICE_CREATE_INFO, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, pcallback : SW_DEVICE_CREATE_CALLBACK, pcontext : *const core::ffi::c_void, phswdevice : *mut HSWDEVICE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SwDeviceCreate(pszenumeratorname.param().abi(), pszparentdeviceinstance.param().abi(), pcreateinfo, pproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcallback, pcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SwDeviceGetLifetime(hswdevice: HSWDEVICE) -> windows_core::Result<SW_DEVICE_LIFETIME> {
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceGetLifetime(hswdevice : HSWDEVICE, plifetime : *mut SW_DEVICE_LIFETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SwDeviceGetLifetime(hswdevice, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<P1>(hswdevice: HSWDEVICE, pszdeviceinterfaceid: P1, pproperties: &[super::super::Properties::DEVPROPERTY]) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceInterfacePropertySet(hswdevice : HSWDEVICE, pszdeviceinterfaceid : windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> windows_core::HRESULT);
    unsafe { SwDeviceInterfacePropertySet(hswdevice, pszdeviceinterfaceid.param().abi(), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<P2>(hswdevice: HSWDEVICE, pinterfaceclassguid: *const windows_core::GUID, pszreferencestring: P2, pproperties: Option<&[super::super::Properties::DEVPROPERTY]>, fenabled: bool) -> windows_core::Result<windows_core::PWSTR>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceRegister(hswdevice : HSWDEVICE, pinterfaceclassguid : *const windows_core::GUID, pszreferencestring : windows_core::PCWSTR, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY, fenabled : windows_core::BOOL, ppszdeviceinterfaceid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SwDeviceInterfaceRegister(hswdevice, pinterfaceclassguid, pszreferencestring.param().abi(), pproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), fenabled.into(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<P1>(hswdevice: HSWDEVICE, pszdeviceinterfaceid: P1, fenabled: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceInterfaceSetState(hswdevice : HSWDEVICE, pszdeviceinterfaceid : windows_core::PCWSTR, fenabled : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { SwDeviceInterfaceSetState(hswdevice, pszdeviceinterfaceid.param().abi(), fenabled.into()).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SwDevicePropertySet(hswdevice: HSWDEVICE, pproperties: &[super::super::Properties::DEVPROPERTY]) -> windows_core::Result<()> {
    windows_core::link!("cfgmgr32.dll" "system" fn SwDevicePropertySet(hswdevice : HSWDEVICE, cpropertycount : u32, pproperties : *const super::super::Properties:: DEVPROPERTY) -> windows_core::HRESULT);
    unsafe { SwDevicePropertySet(hswdevice, pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())).ok() }
}
#[inline]
pub unsafe fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> windows_core::Result<()> {
    windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceSetLifetime(hswdevice : HSWDEVICE, lifetime : SW_DEVICE_LIFETIME) -> windows_core::HRESULT);
    unsafe { SwDeviceSetLifetime(hswdevice, lifetime).ok() }
}
#[inline]
pub unsafe fn SwMemFree(pmem: *const core::ffi::c_void) {
    windows_core::link!("cfgmgr32.dll" "system" fn SwMemFree(pmem : *const core::ffi::c_void));
    unsafe { SwMemFree(pmem) }
}
pub const ADDRESS_FAMILY_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("AddressFamily");
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
pub const FAULT_INVALID_ACTION: u32 = 401u32;
pub const FAULT_INVALID_ARG: u32 = 402u32;
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSWDEVICE(pub *mut core::ffi::c_void);
impl HSWDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HSWDEVICE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("cfgmgr32.dll" "system" fn SwDeviceClose(hswdevice : *mut core::ffi::c_void));
            unsafe {
                SwDeviceClose(self.0);
            }
        }
    }
}
impl Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IUPnPAddressFamilyControl, IUPnPAddressFamilyControl_Vtbl, 0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
windows_core::imp::interface_hierarchy!(IUPnPAddressFamilyControl, windows_core::IUnknown);
impl IUPnPAddressFamilyControl {
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetAddressFamily(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAddressFamily)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUPnPAddressFamilyControl_Impl: windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwflags: i32) -> windows_core::Result<()>;
    fn GetAddressFamily(&self) -> windows_core::Result<i32>;
}
impl IUPnPAddressFamilyControl_Vtbl {
    pub const fn new<Identity: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAddressFamily<Identity: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPAddressFamilyControl_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetAddressFamily<Identity: IUPnPAddressFamilyControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPAddressFamilyControl_Impl::GetAddressFamily(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            GetAddressFamily: GetAddressFamily::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPAddressFamilyControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPAddressFamilyControl {}
windows_core::imp::define_interface!(IUPnPAsyncResult, IUPnPAsyncResult_Vtbl, 0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
windows_core::imp::interface_hierarchy!(IUPnPAsyncResult, windows_core::IUnknown);
impl IUPnPAsyncResult {
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AsyncOperationComplete)(windows_core::Interface::as_raw(self), ullrequestid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IUPnPAsyncResult_Impl: windows_core::IUnknownImpl {
    fn AsyncOperationComplete(&self, ullrequestid: u64) -> windows_core::Result<()>;
}
impl IUPnPAsyncResult_Vtbl {
    pub const fn new<Identity: IUPnPAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncOperationComplete<Identity: IUPnPAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPAsyncResult_Impl::AsyncOperationComplete(this, core::mem::transmute_copy(&ullrequestid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPAsyncResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPAsyncResult {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Load(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)).ok() }
    }
    pub unsafe fn LoadAsync<P1>(&self, bstrurl: &windows_core::BSTR, punkcallback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), punkcallback.param().abi()).ok() }
    }
    pub unsafe fn LoadResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RootDevice(&self) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeviceByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceByUDN)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrudn), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub ReadyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceByUDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDescriptionDocument_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn ReadyState(&self) -> windows_core::Result<i32>;
    fn Load(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadAsync(&self, bstrurl: &windows_core::BSTR, punkcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LoadResult(&self) -> windows_core::Result<i32>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn RootDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn DeviceByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDescriptionDocument_Vtbl {
    pub const fn new<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadyState<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreadystate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDescriptionDocument_Impl::ReadyState(this) {
                    Ok(ok__) => {
                        plreadystate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Load<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDescriptionDocument_Impl::Load(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, punkcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDescriptionDocument_Impl::LoadAsync(this, core::mem::transmute(&bstrurl), core::mem::transmute_copy(&punkcallback)).into()
            }
        }
        unsafe extern "system" fn LoadResult<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDescriptionDocument_Impl::LoadResult(this) {
                    Ok(ok__) => {
                        phrerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Abort<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDescriptionDocument_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn RootDevice<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudrootdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDescriptionDocument_Impl::RootDevice(this) {
                    Ok(ok__) => {
                        ppudrootdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceByUDN<Identity: IUPnPDescriptionDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: *mut core::ffi::c_void, ppuddevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDescriptionDocument_Impl::DeviceByUDN(this, core::mem::transmute(&bstrudn)) {
                    Ok(ok__) => {
                        ppuddevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ReadyState: ReadyState::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            LoadAsync: LoadAsync::<Identity, OFFSET>,
            LoadResult: LoadResult::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            RootDevice: RootDevice::<Identity, OFFSET>,
            DeviceByUDN: DeviceByUDN::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPDescriptionDocument {}
windows_core::imp::define_interface!(IUPnPDescriptionDocumentCallback, IUPnPDescriptionDocumentCallback_Vtbl, 0x77394c69_5486_40d6_9bc3_4991983e02da);
windows_core::imp::interface_hierarchy!(IUPnPDescriptionDocumentCallback, windows_core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    pub unsafe fn LoadComplete(&self, hrloadresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadComplete)(windows_core::Interface::as_raw(self), hrloadresult).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IUPnPDescriptionDocumentCallback_Impl: windows_core::IUnknownImpl {
    fn LoadComplete(&self, hrloadresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IUPnPDescriptionDocumentCallback_Vtbl {
    pub const fn new<Identity: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadComplete<Identity: IUPnPDescriptionDocumentCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrloadresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDescriptionDocumentCallback_Impl::LoadComplete(this, core::mem::transmute_copy(&hrloadresult)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadComplete: LoadComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDescriptionDocumentCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPDescriptionDocumentCallback {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRootDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RootDevice(&self) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ParentDevice(&self) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HasChildren(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Children(&self) -> windows_core::Result<IUPnPDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UniqueDeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UniqueDeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PresentationURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PresentationURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ManufacturerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManufacturerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ManufacturerURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManufacturerURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ModelName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModelName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ModelNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModelNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ModelURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModelURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UPC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UPC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IconURL(&self, bstrencodingformat: &windows_core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IconURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrencodingformat), lsizex, lsizey, lbitdepth, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Services(&self) -> windows_core::Result<IUPnPServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevice_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub IsRootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RootDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UniqueDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PresentationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ManufacturerURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModelURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UPC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevice_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn IsRootDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn RootDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn ParentDevice(&self) -> windows_core::Result<IUPnPDevice>;
    fn HasChildren(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Children(&self) -> windows_core::Result<IUPnPDevices>;
    fn UniqueDeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PresentationURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ManufacturerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ManufacturerURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModelURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UPC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SerialNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IconURL(&self, bstrencodingformat: &windows_core::BSTR, lsizex: i32, lsizey: i32, lbitdepth: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Services(&self) -> windows_core::Result<IUPnPServices>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDevice_Vtbl {
    pub const fn new<Identity: IUPnPDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRootDevice<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::IsRootDevice(this) {
                    Ok(ok__) => {
                        pvarb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RootDevice<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudrootdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::RootDevice(this) {
                    Ok(ok__) => {
                        ppudrootdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParentDevice<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuddeviceparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ParentDevice(this) {
                    Ok(ok__) => {
                        ppuddeviceparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasChildren<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarb: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::HasChildren(this) {
                    Ok(ok__) => {
                        pvarb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Children<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppudchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::Children(this) {
                    Ok(ok__) => {
                        ppudchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UniqueDeviceName<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::UniqueDeviceName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::Type(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PresentationURL<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::PresentationURL(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManufacturerName<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ManufacturerName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ManufacturerURL<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ManufacturerURL(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelName<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ModelName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ModelNumber(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::Description(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelURL<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::ModelURL(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UPC<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::UPC(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::SerialNumber(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IconURL<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrencodingformat: *mut core::ffi::c_void, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::IconURL(this, core::mem::transmute(&bstrencodingformat), core::mem::transmute_copy(&lsizex), core::mem::transmute_copy(&lsizey), core::mem::transmute_copy(&lbitdepth)) {
                    Ok(ok__) => {
                        pbstriconurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Services<Identity: IUPnPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppusservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevice_Impl::Services(this) {
                    Ok(ok__) => {
                        ppusservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRootDevice: IsRootDevice::<Identity, OFFSET>,
            RootDevice: RootDevice::<Identity, OFFSET>,
            ParentDevice: ParentDevice::<Identity, OFFSET>,
            HasChildren: HasChildren::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            UniqueDeviceName: UniqueDeviceName::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            PresentationURL: PresentationURL::<Identity, OFFSET>,
            ManufacturerName: ManufacturerName::<Identity, OFFSET>,
            ManufacturerURL: ManufacturerURL::<Identity, OFFSET>,
            ModelName: ModelName::<Identity, OFFSET>,
            ModelNumber: ModelNumber::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            ModelURL: ModelURL::<Identity, OFFSET>,
            UPC: UPC::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            IconURL: IconURL::<Identity, OFFSET>,
            Services: Services::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDevice as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPDevice {}
windows_core::imp::define_interface!(IUPnPDeviceControl, IUPnPDeviceControl_Vtbl, 0x204810ba_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPDeviceControl, windows_core::IUnknown);
impl IUPnPDeviceControl {
    pub unsafe fn Initialize(&self, bstrxmldesc: &windows_core::BSTR, bstrdeviceidentifier: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxmldesc), core::mem::transmute_copy(bstrdeviceidentifier), core::mem::transmute_copy(bstrinitstring)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetServiceObject(&self, bstrudn: &windows_core::BSTR, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrudn), core::mem::transmute_copy(bstrserviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetServiceObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetServiceObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceControl_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, bstrxmldesc: &windows_core::BSTR, bstrdeviceidentifier: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetServiceObject(&self, bstrudn: &windows_core::BSTR, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceControl_Vtbl {
    pub const fn new<Identity: IUPnPDeviceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: *mut core::ffi::c_void, bstrdeviceidentifier: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceControl_Impl::Initialize(this, core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrinitstring)).into()
            }
        }
        unsafe extern "system" fn GetServiceObject<Identity: IUPnPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: *mut core::ffi::c_void, bstrserviceid: *mut core::ffi::c_void, ppdispservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceControl_Impl::GetServiceObject(this, core::mem::transmute(&bstrudn), core::mem::transmute(&bstrserviceid)) {
                    Ok(ok__) => {
                        ppdispservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetServiceObject: GetServiceObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceControl {}
windows_core::imp::define_interface!(IUPnPDeviceControlHttpHeaders, IUPnPDeviceControlHttpHeaders_Vtbl, 0x204810bb_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPDeviceControlHttpHeaders, windows_core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdditionalResponseHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeaders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdditionalResponseHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPDeviceControlHttpHeaders_Impl: windows_core::IUnknownImpl {
    fn GetAdditionalResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IUPnPDeviceControlHttpHeaders_Vtbl {
    pub const fn new<Identity: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdditionalResponseHeaders<Identity: IUPnPDeviceControlHttpHeaders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhttpresponseheaders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceControlHttpHeaders_Impl::GetAdditionalResponseHeaders(this) {
                    Ok(ok__) => {
                        bstrhttpresponseheaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAdditionalResponseHeaders: GetAdditionalResponseHeaders::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceControlHttpHeaders as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPDeviceControlHttpHeaders {}
windows_core::imp::define_interface!(IUPnPDeviceDocumentAccess, IUPnPDeviceDocumentAccess_Vtbl, 0xe7772804_3287_418e_9072_cf2b47238981);
windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccess, windows_core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPDeviceDocumentAccess_Impl: windows_core::IUnknownImpl {
    fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IUPnPDeviceDocumentAccess_Vtbl {
    pub const fn new<Identity: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentURL<Identity: IUPnPDeviceDocumentAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceDocumentAccess_Impl::GetDocumentURL(this) {
                    Ok(ok__) => {
                        pbstrdocument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocumentURL: GetDocumentURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPDeviceDocumentAccess {}
windows_core::imp::define_interface!(IUPnPDeviceDocumentAccessEx, IUPnPDeviceDocumentAccessEx_Vtbl, 0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
windows_core::imp::interface_hierarchy!(IUPnPDeviceDocumentAccessEx, windows_core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPDeviceDocumentAccessEx_Impl: windows_core::IUnknownImpl {
    fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IUPnPDeviceDocumentAccessEx_Vtbl {
    pub const fn new<Identity: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocument<Identity: IUPnPDeviceDocumentAccessEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceDocumentAccessEx_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        pbstrdocument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDocument: GetDocument::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceDocumentAccessEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPDeviceDocumentAccessEx {}
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
    pub unsafe fn FindByType(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32) -> windows_core::Result<IUPnPDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindByType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtypeuri), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAsyncFind<P2>(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32, punkdevicefindercallback: P2) -> windows_core::Result<i32>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAsyncFind)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtypeuri), dwflags, punkdevicefindercallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartAsyncFind)(windows_core::Interface::as_raw(self), lfinddata).ok() }
    }
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncFind)(windows_core::Interface::as_raw(self), lfinddata).ok() }
    }
    pub unsafe fn FindByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindByUDN)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrudn), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinder_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub FindByType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CancelAsyncFind: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FindByUDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDeviceFinder_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn FindByType(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32) -> windows_core::Result<IUPnPDevices>;
    fn CreateAsyncFind(&self, bstrtypeuri: &windows_core::BSTR, dwflags: u32, punkdevicefindercallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<i32>;
    fn StartAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()>;
    fn CancelAsyncFind(&self, lfinddata: i32) -> windows_core::Result<()>;
    fn FindByUDN(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDeviceFinder_Vtbl {
    pub const fn new<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindByType<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypeuri: *mut core::ffi::c_void, dwflags: u32, pdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceFinder_Impl::FindByType(this, core::mem::transmute(&bstrtypeuri), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAsyncFind<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypeuri: *mut core::ffi::c_void, dwflags: u32, punkdevicefindercallback: *mut core::ffi::c_void, plfinddata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceFinder_Impl::CreateAsyncFind(this, core::mem::transmute(&bstrtypeuri), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&punkdevicefindercallback)) {
                    Ok(ok__) => {
                        plfinddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAsyncFind<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinder_Impl::StartAsyncFind(this, core::mem::transmute_copy(&lfinddata)).into()
            }
        }
        unsafe extern "system" fn CancelAsyncFind<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinder_Impl::CancelAsyncFind(this, core::mem::transmute_copy(&lfinddata)).into()
            }
        }
        unsafe extern "system" fn FindByUDN<Identity: IUPnPDeviceFinder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: *mut core::ffi::c_void, pdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDeviceFinder_Impl::FindByUDN(this, core::mem::transmute(&bstrudn)) {
                    Ok(ok__) => {
                        pdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FindByType: FindByType::<Identity, OFFSET>,
            CreateAsyncFind: CreateAsyncFind::<Identity, OFFSET>,
            StartAsyncFind: StartAsyncFind::<Identity, OFFSET>,
            CancelAsyncFind: CancelAsyncFind::<Identity, OFFSET>,
            FindByUDN: FindByUDN::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinder as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPDeviceFinder {}
windows_core::imp::define_interface!(IUPnPDeviceFinderAddCallbackWithInterface, IUPnPDeviceFinderAddCallbackWithInterface_Vtbl, 0x983dfc0b_1796_44df_8975_ca545b620ee5);
windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderAddCallbackWithInterface, windows_core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAddedWithInterface<P1>(&self, lfinddata: i32, pdevice: P1, pguidinterface: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IUPnPDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeviceAddedWithInterface)(windows_core::Interface::as_raw(self), lfinddata, pdevice.param().abi(), pguidinterface).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAddedWithInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAddedWithInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderAddCallbackWithInterface_Impl: windows_core::IUnknownImpl {
    fn DeviceAddedWithInterface(&self, lfinddata: i32, pdevice: windows_core::Ref<IUPnPDevice>, pguidinterface: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderAddCallbackWithInterface_Vtbl {
    pub const fn new<Identity: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceAddedWithInterface<Identity: IUPnPDeviceFinderAddCallbackWithInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, pdevice: *mut core::ffi::c_void, pguidinterface: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinderAddCallbackWithInterface_Impl::DeviceAddedWithInterface(this, core::mem::transmute_copy(&lfinddata), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pguidinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeviceAddedWithInterface: DeviceAddedWithInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderAddCallbackWithInterface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceFinderAddCallbackWithInterface {}
windows_core::imp::define_interface!(IUPnPDeviceFinderCallback, IUPnPDeviceFinderCallback_Vtbl, 0x415a984a_88b3_49f3_92af_0508bedf0d6c);
windows_core::imp::interface_hierarchy!(IUPnPDeviceFinderCallback, windows_core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceAdded<P1>(&self, lfinddata: i32, pdevice: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IUPnPDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeviceAdded)(windows_core::Interface::as_raw(self), lfinddata, pdevice.param().abi()).ok() }
    }
    pub unsafe fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceRemoved)(windows_core::Interface::as_raw(self), lfinddata, core::mem::transmute_copy(bstrudn)).ok() }
    }
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SearchComplete)(windows_core::Interface::as_raw(self), lfinddata).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceAdded: usize,
    pub DeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderCallback_Impl: windows_core::IUnknownImpl {
    fn DeviceAdded(&self, lfinddata: i32, pdevice: windows_core::Ref<IUPnPDevice>) -> windows_core::Result<()>;
    fn DeviceRemoved(&self, lfinddata: i32, bstrudn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SearchComplete(&self, lfinddata: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPDeviceFinderCallback_Vtbl {
    pub const fn new<Identity: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceAdded<Identity: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, pdevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinderCallback_Impl::DeviceAdded(this, core::mem::transmute_copy(&lfinddata), core::mem::transmute_copy(&pdevice)).into()
            }
        }
        unsafe extern "system" fn DeviceRemoved<Identity: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32, bstrudn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinderCallback_Impl::DeviceRemoved(this, core::mem::transmute_copy(&lfinddata), core::mem::transmute(&bstrudn)).into()
            }
        }
        unsafe extern "system" fn SearchComplete<Identity: IUPnPDeviceFinderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfinddata: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceFinderCallback_Impl::SearchComplete(this, core::mem::transmute_copy(&lfinddata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceAdded: DeviceAdded::<Identity, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, OFFSET>,
            SearchComplete: SearchComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceFinderCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPDeviceFinderCallback {}
windows_core::imp::define_interface!(IUPnPDeviceProvider, IUPnPDeviceProvider_Vtbl, 0x204810b8_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPDeviceProvider, windows_core::IUnknown);
impl IUPnPDeviceProvider {
    pub unsafe fn Start(&self, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinitstring)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPDeviceProvider_Impl: windows_core::IUnknownImpl {
    fn Start(&self, bstrinitstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl IUPnPDeviceProvider_Vtbl {
    pub const fn new<Identity: IUPnPDeviceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceProvider_Impl::Start(this, core::mem::transmute(&bstrinitstring)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IUPnPDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPDeviceProvider_Impl::Stop(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDeviceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPDeviceProvider {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrudn), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPDevices_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrudn: &windows_core::BSTR) -> windows_core::Result<IUPnPDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPDevices_Vtbl {
    pub const fn new<Identity: IUPnPDevices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevices_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevices_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IUPnPDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrudn: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPDevices_Impl::get_Item(this, core::mem::transmute(&bstrudn)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPDevices as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPDevices {}
windows_core::imp::define_interface!(IUPnPEventSink, IUPnPEventSink_Vtbl, 0x204810b4_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPEventSink, windows_core::IUnknown);
impl IUPnPEventSink {
    pub unsafe fn OnStateChanged(&self, rgdispidchanges: &[i32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), rgdispidchanges.len().try_into().unwrap(), core::mem::transmute(rgdispidchanges.as_ptr())).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OnStateChangedSafe(&self, varsadispidchanges: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStateChangedSafe)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsadispidchanges)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OnStateChangedSafe: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OnStateChangedSafe: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPEventSink_Impl: windows_core::IUnknownImpl {
    fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> windows_core::Result<()>;
    fn OnStateChangedSafe(&self, varsadispidchanges: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPEventSink_Vtbl {
    pub const fn new<Identity: IUPnPEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStateChanged<Identity: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPEventSink_Impl::OnStateChanged(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&rgdispidchanges)).into()
            }
        }
        unsafe extern "system" fn OnStateChangedSafe<Identity: IUPnPEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsadispidchanges: super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPEventSink_Impl::OnStateChangedSafe(this, core::mem::transmute(&varsadispidchanges)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnStateChangedSafe: OnStateChangedSafe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPEventSink {}
windows_core::imp::define_interface!(IUPnPEventSource, IUPnPEventSource_Vtbl, 0x204810b5_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPEventSource, windows_core::IUnknown);
impl IUPnPEventSource {
    pub unsafe fn Advise<P0>(&self, pessubscriber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pessubscriber.param().abi()).ok() }
    }
    pub unsafe fn Unadvise<P0>(&self, pessubscriber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pessubscriber.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPEventSource_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, pessubscriber: windows_core::Ref<IUPnPEventSink>) -> windows_core::Result<()>;
    fn Unadvise(&self, pessubscriber: windows_core::Ref<IUPnPEventSink>) -> windows_core::Result<()>;
}
impl IUPnPEventSource_Vtbl {
    pub const fn new<Identity: IUPnPEventSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pessubscriber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPEventSource_Impl::Advise(this, core::mem::transmute_copy(&pessubscriber)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IUPnPEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pessubscriber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPEventSource_Impl::Unadvise(this, core::mem::transmute_copy(&pessubscriber)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Advise: Advise::<Identity, OFFSET>, Unadvise: Unadvise::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPEventSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPEventSource {}
windows_core::imp::define_interface!(IUPnPHttpHeaderControl, IUPnPHttpHeaderControl_Vtbl, 0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
windows_core::imp::interface_hierarchy!(IUPnPHttpHeaderControl, windows_core::IUnknown);
impl IUPnPHttpHeaderControl {
    pub unsafe fn AddRequestHeaders(&self, bstrhttpheaders: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRequestHeaders)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhttpheaders)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRequestHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPHttpHeaderControl_Impl: windows_core::IUnknownImpl {
    fn AddRequestHeaders(&self, bstrhttpheaders: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IUPnPHttpHeaderControl_Vtbl {
    pub const fn new<Identity: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRequestHeaders<Identity: IUPnPHttpHeaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhttpheaders: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPHttpHeaderControl_Impl::AddRequestHeaders(this, core::mem::transmute(&bstrhttpheaders)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddRequestHeaders: AddRequestHeaders::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPHttpHeaderControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPHttpHeaderControl {}
windows_core::imp::define_interface!(IUPnPRegistrar, IUPnPRegistrar_Vtbl, 0x204810b6_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPRegistrar, windows_core::IUnknown);
impl IUPnPRegistrar {
    pub unsafe fn RegisterDevice(&self, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxmldesc), core::mem::transmute_copy(bstrprogiddevicecontrolclass), core::mem::transmute_copy(bstrinitstring), core::mem::transmute_copy(bstrcontainerid), core::mem::transmute_copy(bstrresourcepath), nlifetime, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RegisterRunningDevice<P1>(&self, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: P1, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterRunningDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxmldesc), punkdevicecontrol.param().abi(), core::mem::transmute_copy(bstrinitstring), core::mem::transmute_copy(bstrresourcepath), nlifetime, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RegisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR, bstrprogidproviderclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterDeviceProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovidername), core::mem::transmute_copy(bstrprogidproviderclass), core::mem::transmute_copy(bstrinitstring), core::mem::transmute_copy(bstrcontainerid)).ok() }
    }
    pub unsafe fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrtemplateudn: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUniqueDeviceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceidentifier), core::mem::transmute_copy(bstrtemplateudn), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UnregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, fpermanent: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceidentifier), fpermanent.into()).ok() }
    }
    pub unsafe fn UnregisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDeviceProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovidername)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterRunningDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUniqueDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UnregisterDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterDevice(&self, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterRunningDevice(&self, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: windows_core::Ref<windows_core::IUnknown>, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR, bstrprogidproviderclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetUniqueDeviceName(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrtemplateudn: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn UnregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, fpermanent: windows_core::BOOL) -> windows_core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IUPnPRegistrar_Vtbl {
    pub const fn new<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterDevice<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: *mut core::ffi::c_void, bstrprogiddevicecontrolclass: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void, bstrcontainerid: *mut core::ffi::c_void, bstrresourcepath: *mut core::ffi::c_void, nlifetime: i32, pbstrdeviceidentifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRegistrar_Impl::RegisterDevice(this, core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrprogiddevicecontrolclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)) {
                    Ok(ok__) => {
                        pbstrdeviceidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterRunningDevice<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmldesc: *mut core::ffi::c_void, punkdevicecontrol: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void, bstrresourcepath: *mut core::ffi::c_void, nlifetime: i32, pbstrdeviceidentifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRegistrar_Impl::RegisterRunningDevice(this, core::mem::transmute(&bstrxmldesc), core::mem::transmute_copy(&punkdevicecontrol), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)) {
                    Ok(ok__) => {
                        pbstrdeviceidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: *mut core::ffi::c_void, bstrprogidproviderclass: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void, bstrcontainerid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPRegistrar_Impl::RegisterDeviceProvider(this, core::mem::transmute(&bstrprovidername), core::mem::transmute(&bstrprogidproviderclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid)).into()
            }
        }
        unsafe extern "system" fn GetUniqueDeviceName<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: *mut core::ffi::c_void, bstrtemplateudn: *mut core::ffi::c_void, pbstrudn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRegistrar_Impl::GetUniqueDeviceName(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrtemplateudn)) {
                    Ok(ok__) => {
                        pbstrudn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: *mut core::ffi::c_void, fpermanent: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPRegistrar_Impl::UnregisterDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute_copy(&fpermanent)).into()
            }
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: IUPnPRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPRegistrar_Impl::UnregisterDeviceProvider(this, core::mem::transmute(&bstrprovidername)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, OFFSET>,
            RegisterRunningDevice: RegisterRunningDevice::<Identity, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, OFFSET>,
            GetUniqueDeviceName: GetUniqueDeviceName::<Identity, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPRegistrar {}
windows_core::imp::define_interface!(IUPnPRemoteEndpointInfo, IUPnPRemoteEndpointInfo_Vtbl, 0xc92eb863_0269_4aff_9c72_75321bba2952);
windows_core::imp::interface_hierarchy!(IUPnPRemoteEndpointInfo, windows_core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    pub unsafe fn GetDwordValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDwordValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvaluename), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvaluename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetGuidValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuidValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvaluename), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDwordValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IUPnPRemoteEndpointInfo_Impl: windows_core::IUnknownImpl {
    fn GetDwordValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn GetStringValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetGuidValue(&self, bstrvaluename: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
}
impl IUPnPRemoteEndpointInfo_Vtbl {
    pub const fn new<Identity: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDwordValue<Identity: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: *mut core::ffi::c_void, pdwvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRemoteEndpointInfo_Impl::GetDwordValue(this, core::mem::transmute(&bstrvaluename)) {
                    Ok(ok__) => {
                        pdwvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRemoteEndpointInfo_Impl::GetStringValue(this, core::mem::transmute(&bstrvaluename)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: IUPnPRemoteEndpointInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluename: *mut core::ffi::c_void, pguidvalue: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPRemoteEndpointInfo_Impl::GetGuidValue(this, core::mem::transmute(&bstrvaluename)) {
                    Ok(ok__) => {
                        pguidvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDwordValue: GetDwordValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPRemoteEndpointInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPRemoteEndpointInfo {}
windows_core::imp::define_interface!(IUPnPReregistrar, IUPnPReregistrar_Vtbl, 0x204810b7_73b2_11d4_bf42_00b0d0118b56);
windows_core::imp::interface_hierarchy!(IUPnPReregistrar, windows_core::IUnknown);
impl IUPnPReregistrar {
    pub unsafe fn ReregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReregisterDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceidentifier), core::mem::transmute_copy(bstrxmldesc), core::mem::transmute_copy(bstrprogiddevicecontrolclass), core::mem::transmute_copy(bstrinitstring), core::mem::transmute_copy(bstrcontainerid), core::mem::transmute_copy(bstrresourcepath), nlifetime).ok() }
    }
    pub unsafe fn ReregisterRunningDevice<P2>(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: P2, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReregisterRunningDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceidentifier), core::mem::transmute_copy(bstrxmldesc), punkdevicecontrol.param().abi(), core::mem::transmute_copy(bstrinitstring), core::mem::transmute_copy(bstrresourcepath), nlifetime).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReregisterDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReregisterRunningDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IUPnPReregistrar_Impl: windows_core::IUnknownImpl {
    fn ReregisterDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, bstrprogiddevicecontrolclass: &windows_core::BSTR, bstrinitstring: &windows_core::BSTR, bstrcontainerid: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()>;
    fn ReregisterRunningDevice(&self, bstrdeviceidentifier: &windows_core::BSTR, bstrxmldesc: &windows_core::BSTR, punkdevicecontrol: windows_core::Ref<windows_core::IUnknown>, bstrinitstring: &windows_core::BSTR, bstrresourcepath: &windows_core::BSTR, nlifetime: i32) -> windows_core::Result<()>;
}
impl IUPnPReregistrar_Vtbl {
    pub const fn new<Identity: IUPnPReregistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReregisterDevice<Identity: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: *mut core::ffi::c_void, bstrxmldesc: *mut core::ffi::c_void, bstrprogiddevicecontrolclass: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void, bstrcontainerid: *mut core::ffi::c_void, bstrresourcepath: *mut core::ffi::c_void, nlifetime: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPReregistrar_Impl::ReregisterDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrxmldesc), core::mem::transmute(&bstrprogiddevicecontrolclass), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrcontainerid), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)).into()
            }
        }
        unsafe extern "system" fn ReregisterRunningDevice<Identity: IUPnPReregistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceidentifier: *mut core::ffi::c_void, bstrxmldesc: *mut core::ffi::c_void, punkdevicecontrol: *mut core::ffi::c_void, bstrinitstring: *mut core::ffi::c_void, bstrresourcepath: *mut core::ffi::c_void, nlifetime: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPReregistrar_Impl::ReregisterRunningDevice(this, core::mem::transmute(&bstrdeviceidentifier), core::mem::transmute(&bstrxmldesc), core::mem::transmute_copy(&punkdevicecontrol), core::mem::transmute(&bstrinitstring), core::mem::transmute(&bstrresourcepath), core::mem::transmute_copy(&nlifetime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReregisterDevice: ReregisterDevice::<Identity, OFFSET>,
            ReregisterRunningDevice: ReregisterRunningDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPReregistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPReregistrar {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn QueryStateVariable(&self, bstrvariablename: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryStateVariable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvariablename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InvokeAction(&self, bstractionname: &windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InvokeAction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstractionname), core::mem::transmute_copy(vinactionargs), core::mem::transmute(pvoutactionargs), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServiceTypeIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceTypeIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddCallback<P0>(&self, punkcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddCallback)(windows_core::Interface::as_raw(self), punkcallback.param().abi()).ok() }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LastTransportStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastTransportStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPService_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub QueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    QueryStateVariable: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::System::Variant::VARIANT, *mut super::super::super::System::Variant::VARIANT, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InvokeAction: usize,
    pub ServiceTypeIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastTransportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPService_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn QueryStateVariable(&self, bstrvariablename: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn InvokeAction(&self, bstractionname: &windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn ServiceTypeIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddCallback(&self, punkcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastTransportStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPService_Vtbl {
    pub const fn new<Identity: IUPnPService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryStateVariable<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvariablename: *mut core::ffi::c_void, pvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPService_Impl::QueryStateVariable(this, core::mem::transmute(&bstrvariablename)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeAction<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstractionname: *mut core::ffi::c_void, vinactionargs: super::super::super::System::Variant::VARIANT, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPService_Impl::InvokeAction(this, core::mem::transmute(&bstractionname), core::mem::transmute(&vinactionargs), core::mem::transmute_copy(&pvoutactionargs)) {
                    Ok(ok__) => {
                        pvretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceTypeIdentifier<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPService_Impl::ServiceTypeIdentifier(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddCallback<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPService_Impl::AddCallback(this, core::mem::transmute_copy(&punkcallback)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPService_Impl::Id(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastTransportStatus<Identity: IUPnPService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPService_Impl::LastTransportStatus(this) {
                    Ok(ok__) => {
                        plvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueryStateVariable: QueryStateVariable::<Identity, OFFSET>,
            InvokeAction: InvokeAction::<Identity, OFFSET>,
            ServiceTypeIdentifier: ServiceTypeIdentifier::<Identity, OFFSET>,
            AddCallback: AddCallback::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LastTransportStatus: LastTransportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPService as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPService {}
windows_core::imp::define_interface!(IUPnPServiceAsync, IUPnPServiceAsync_Vtbl, 0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
windows_core::imp::interface_hierarchy!(IUPnPServiceAsync, windows_core::IUnknown);
impl IUPnPServiceAsync {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BeginInvokeAction<P2>(&self, bstractionname: &windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pasyncresult: P2) -> windows_core::Result<u64>
    where
        P2: windows_core::Param<IUPnPAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginInvokeAction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstractionname), core::mem::transmute_copy(vinactionargs), pasyncresult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndInvokeAction)(windows_core::Interface::as_raw(self), ullrequestid, core::mem::transmute(pvoutactionargs), core::mem::transmute(pvretval)).ok() }
    }
    pub unsafe fn BeginQueryStateVariable<P1>(&self, bstrvariablename: &windows_core::BSTR, pasyncresult: P1) -> windows_core::Result<u64>
    where
        P1: windows_core::Param<IUPnPAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginQueryStateVariable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvariablename), pasyncresult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndQueryStateVariable)(windows_core::Interface::as_raw(self), ullrequestid, core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn BeginSubscribeToEvents<P0, P1>(&self, punkcallback: P0, pasyncresult: P1) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IUPnPAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSubscribeToEvents)(windows_core::Interface::as_raw(self), punkcallback.param().abi(), pasyncresult.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndSubscribeToEvents)(windows_core::Interface::as_raw(self), ullrequestid).ok() }
    }
    pub unsafe fn BeginSCPDDownload<P0>(&self, pasyncresult: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IUPnPAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSCPDDownload)(windows_core::Interface::as_raw(self), pasyncresult.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndSCPDDownload)(windows_core::Interface::as_raw(self), ullrequestid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncOperation)(windows_core::Interface::as_raw(self), ullrequestid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BeginInvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::System::Variant::VARIANT, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BeginInvokeAction: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EndInvokeAction: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut super::super::super::System::Variant::VARIANT, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EndInvokeAction: usize,
    pub BeginQueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EndQueryStateVariable: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EndQueryStateVariable: usize,
    pub BeginSubscribeToEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndSubscribeToEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub BeginSCPDDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub EndSCPDDownload: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAsyncOperation: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceAsync_Impl: windows_core::IUnknownImpl {
    fn BeginInvokeAction(&self, bstractionname: &windows_core::BSTR, vinactionargs: &super::super::super::System::Variant::VARIANT, pasyncresult: windows_core::Ref<IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn BeginQueryStateVariable(&self, bstrvariablename: &windows_core::BSTR, pasyncresult: windows_core::Ref<IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn BeginSubscribeToEvents(&self, punkcallback: windows_core::Ref<windows_core::IUnknown>, pasyncresult: windows_core::Ref<IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndSubscribeToEvents(&self, ullrequestid: u64) -> windows_core::Result<()>;
    fn BeginSCPDDownload(&self, pasyncresult: windows_core::Ref<IUPnPAsyncResult>) -> windows_core::Result<u64>;
    fn EndSCPDDownload(&self, ullrequestid: u64) -> windows_core::Result<windows_core::BSTR>;
    fn CancelAsyncOperation(&self, ullrequestid: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServiceAsync_Vtbl {
    pub const fn new<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginInvokeAction<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstractionname: *mut core::ffi::c_void, vinactionargs: super::super::super::System::Variant::VARIANT, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceAsync_Impl::BeginInvokeAction(this, core::mem::transmute(&bstractionname), core::mem::transmute(&vinactionargs), core::mem::transmute_copy(&pasyncresult)) {
                    Ok(ok__) => {
                        pullrequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndInvokeAction<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Variant::VARIANT, pvretval: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceAsync_Impl::EndInvokeAction(this, core::mem::transmute_copy(&ullrequestid), core::mem::transmute_copy(&pvoutactionargs), core::mem::transmute_copy(&pvretval)).into()
            }
        }
        unsafe extern "system" fn BeginQueryStateVariable<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvariablename: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceAsync_Impl::BeginQueryStateVariable(this, core::mem::transmute(&bstrvariablename), core::mem::transmute_copy(&pasyncresult)) {
                    Ok(ok__) => {
                        pullrequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndQueryStateVariable<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceAsync_Impl::EndQueryStateVariable(this, core::mem::transmute_copy(&ullrequestid), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn BeginSubscribeToEvents<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkcallback: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceAsync_Impl::BeginSubscribeToEvents(this, core::mem::transmute_copy(&punkcallback), core::mem::transmute_copy(&pasyncresult)) {
                    Ok(ok__) => {
                        pullrequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndSubscribeToEvents<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceAsync_Impl::EndSubscribeToEvents(this, core::mem::transmute_copy(&ullrequestid)).into()
            }
        }
        unsafe extern "system" fn BeginSCPDDownload<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pullrequestid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceAsync_Impl::BeginSCPDDownload(this, core::mem::transmute_copy(&pasyncresult)) {
                    Ok(ok__) => {
                        pullrequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndSCPDDownload<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceAsync_Impl::EndSCPDDownload(this, core::mem::transmute_copy(&ullrequestid)) {
                    Ok(ok__) => {
                        pbstrscpddoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelAsyncOperation<Identity: IUPnPServiceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullrequestid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceAsync_Impl::CancelAsyncOperation(this, core::mem::transmute_copy(&ullrequestid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginInvokeAction: BeginInvokeAction::<Identity, OFFSET>,
            EndInvokeAction: EndInvokeAction::<Identity, OFFSET>,
            BeginQueryStateVariable: BeginQueryStateVariable::<Identity, OFFSET>,
            EndQueryStateVariable: EndQueryStateVariable::<Identity, OFFSET>,
            BeginSubscribeToEvents: BeginSubscribeToEvents::<Identity, OFFSET>,
            EndSubscribeToEvents: EndSubscribeToEvents::<Identity, OFFSET>,
            BeginSCPDDownload: BeginSCPDDownload::<Identity, OFFSET>,
            EndSCPDDownload: EndSCPDDownload::<Identity, OFFSET>,
            CancelAsyncOperation: CancelAsyncOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceAsync as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPServiceAsync {}
windows_core::imp::define_interface!(IUPnPServiceCallback, IUPnPServiceCallback_Vtbl, 0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
windows_core::imp::interface_hierarchy!(IUPnPServiceCallback, windows_core::IUnknown);
impl IUPnPServiceCallback {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn StateVariableChanged<P0, P1>(&self, pus: P0, pcwszstatevarname: P1, vavalue: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPService>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StateVariableChanged)(windows_core::Interface::as_raw(self), pus.param().abi(), pcwszstatevarname.param().abi(), core::mem::transmute_copy(vavalue)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceInstanceDied<P0>(&self, pus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUPnPService>,
    {
        unsafe { (windows_core::Interface::vtable(self).ServiceInstanceDied)(windows_core::Interface::as_raw(self), pus.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub StateVariableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    StateVariableChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceInstanceDied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceInstanceDied: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServiceCallback_Impl: windows_core::IUnknownImpl {
    fn StateVariableChanged(&self, pus: windows_core::Ref<IUPnPService>, pcwszstatevarname: &windows_core::PCWSTR, vavalue: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ServiceInstanceDied(&self, pus: windows_core::Ref<IUPnPService>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServiceCallback_Vtbl {
    pub const fn new<Identity: IUPnPServiceCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StateVariableChanged<Identity: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pus: *mut core::ffi::c_void, pcwszstatevarname: windows_core::PCWSTR, vavalue: super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceCallback_Impl::StateVariableChanged(this, core::mem::transmute_copy(&pus), core::mem::transmute(&pcwszstatevarname), core::mem::transmute(&vavalue)).into()
            }
        }
        unsafe extern "system" fn ServiceInstanceDied<Identity: IUPnPServiceCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceCallback_Impl::ServiceInstanceDied(this, core::mem::transmute_copy(&pus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StateVariableChanged: StateVariableChanged::<Identity, OFFSET>,
            ServiceInstanceDied: ServiceInstanceDied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPServiceCallback {}
windows_core::imp::define_interface!(IUPnPServiceDocumentAccess, IUPnPServiceDocumentAccess_Vtbl, 0x21905529_0a5e_4589_825d_7e6d87ea6998);
windows_core::imp::interface_hierarchy!(IUPnPServiceDocumentAccess, windows_core::IUnknown);
impl IUPnPServiceDocumentAccess {
    pub unsafe fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUPnPServiceDocumentAccess_Impl: windows_core::IUnknownImpl {
    fn GetDocumentURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDocument(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IUPnPServiceDocumentAccess_Vtbl {
    pub const fn new<Identity: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentURL<Identity: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdocurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceDocumentAccess_Impl::GetDocumentURL(this) {
                    Ok(ok__) => {
                        pbstrdocurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IUPnPServiceDocumentAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServiceDocumentAccess_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        pbstrdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentURL: GetDocumentURL::<Identity, OFFSET>,
            GetDocument: GetDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceDocumentAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPServiceDocumentAccess {}
windows_core::imp::define_interface!(IUPnPServiceEnumProperty, IUPnPServiceEnumProperty_Vtbl, 0x38873b37_91bb_49f4_b249_2e8efbb8a816);
windows_core::imp::interface_hierarchy!(IUPnPServiceEnumProperty, windows_core::IUnknown);
impl IUPnPServiceEnumProperty {
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceEnumProperty)(windows_core::Interface::as_raw(self), dwmask).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetServiceEnumProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IUPnPServiceEnumProperty_Impl: windows_core::IUnknownImpl {
    fn SetServiceEnumProperty(&self, dwmask: u32) -> windows_core::Result<()>;
}
impl IUPnPServiceEnumProperty_Vtbl {
    pub const fn new<Identity: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetServiceEnumProperty<Identity: IUPnPServiceEnumProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUPnPServiceEnumProperty_Impl::SetServiceEnumProperty(this, core::mem::transmute_copy(&dwmask)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetServiceEnumProperty: SetServiceEnumProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServiceEnumProperty as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUPnPServiceEnumProperty {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<IUPnPService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServices_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPServices_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrserviceid: &windows_core::BSTR) -> windows_core::Result<IUPnPService>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPServices_Vtbl {
    pub const fn new<Identity: IUPnPServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IUPnPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServices_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUPnPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServices_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IUPnPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceid: *mut core::ffi::c_void, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPServices_Impl::get_Item(this, core::mem::transmute(&bstrserviceid)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPServices as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPServices {}
pub const REMOTE_ADDRESS_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("RemoteAddress");
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(8i32);
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(4i32);
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(0i32);
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(1i32);
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = SW_DEVICE_CAPABILITIES(2i32);
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(0i32);
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(2i32);
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = SW_DEVICE_LIFETIME(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SW_DEVICE_CAPABILITIES(pub i32);
pub type SW_DEVICE_CREATE_CALLBACK = Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: windows_core::HRESULT, pcontext: *const core::ffi::c_void, pszdeviceinstanceid: windows_core::PCWSTR)>;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SW_DEVICE_LIFETIME(pub i32);
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
