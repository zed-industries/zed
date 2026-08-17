#[inline]
pub unsafe fn DXCoreCreateAdapterFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const windows_core::GUID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DXCoreCreateAdapterFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXCoreAdapterPreference(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXCoreAdapterProperty(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXCoreAdapterState(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXCoreNotificationType(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXCoreSegmentGroup(pub u32);
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(7u32);
pub const DedicatedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(8u32);
pub const DriverDescription: DXCoreAdapterProperty = DXCoreAdapterProperty(2u32);
pub const DriverVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(1u32);
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = DXCoreAdapterProperty(6u32);
pub const Hardware: DXCoreAdapterPreference = DXCoreAdapterPreference(0u32);
pub const HardwareID: DXCoreAdapterProperty = DXCoreAdapterProperty(3u32);
pub const HardwareIDParts: DXCoreAdapterProperty = DXCoreAdapterProperty(14u32);
pub const HighPerformance: DXCoreAdapterPreference = DXCoreAdapterPreference(2u32);
windows_core::imp::define_interface!(IDXCoreAdapter, IDXCoreAdapter_Vtbl, 0xf0db4c7f_fe5a_42a2_bd62_f2a6cf6fc83e);
windows_core::imp::interface_hierarchy!(IDXCoreAdapter, windows_core::IUnknown);
impl IDXCoreAdapter {
    pub unsafe fn IsValid(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsAttributeSupported(&self, attributeguid: *const windows_core::GUID) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsAttributeSupported)(windows_core::Interface::as_raw(self), attributeguid) }
    }
    pub unsafe fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsPropertySupported)(windows_core::Interface::as_raw(self), property) }
    }
    pub unsafe fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), property, buffersize, propertydata as _).ok() }
    }
    pub unsafe fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertySize)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsQueryStateSupported)(windows_core::Interface::as_raw(self), property) }
    }
    pub unsafe fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: Option<*const core::ffi::c_void>, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryState)(windows_core::Interface::as_raw(self), state, inputstatedetailssize, inputstatedetails.unwrap_or(core::mem::zeroed()) as _, outputbuffersize, outputbuffer as _).ok() }
    }
    pub unsafe fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsSetStateSupported)(windows_core::Interface::as_raw(self), property) }
    }
    pub unsafe fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: Option<*const core::ffi::c_void>, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state, inputstatedetailssize, inputstatedetails.unwrap_or(core::mem::zeroed()) as _, inputdatasize, inputdata).ok() }
    }
    pub unsafe fn GetFactory<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IDXCoreAdapter_Impl: windows_core::IUnknownImpl {
    fn IsValid(&self) -> bool;
    fn IsAttributeSupported(&self, attributeguid: *const windows_core::GUID) -> bool;
    fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool;
    fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> windows_core::Result<usize>;
    fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFactory(&self, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXCoreAdapter_Vtbl {
    pub const fn new<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsValid<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn IsAttributeSupported<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeguid: *const windows_core::GUID) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::IsAttributeSupported(this, core::mem::transmute_copy(&attributeguid))
            }
        }
        unsafe extern "system" fn IsPropertySupported<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::IsPropertySupported(this, core::mem::transmute_copy(&property))
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::GetProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&propertydata)).into()
            }
        }
        unsafe extern "system" fn GetPropertySize<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXCoreAdapter_Impl::GetPropertySize(this, core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        buffersize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsQueryStateSupported<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::IsQueryStateSupported(this, core::mem::transmute_copy(&property))
            }
        }
        unsafe extern "system" fn QueryState<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::QueryState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&inputstatedetailssize), core::mem::transmute_copy(&inputstatedetails), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&outputbuffer)).into()
            }
        }
        unsafe extern "system" fn IsSetStateSupported<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::IsSetStateSupported(this, core::mem::transmute_copy(&property))
            }
        }
        unsafe extern "system" fn SetState<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::SetState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&inputstatedetailssize), core::mem::transmute_copy(&inputstatedetails), core::mem::transmute_copy(&inputdatasize), core::mem::transmute_copy(&inputdata)).into()
            }
        }
        unsafe extern "system" fn GetFactory<Identity: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapter_Impl::GetFactory(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvfactory)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsValid: IsValid::<Identity, OFFSET>,
            IsAttributeSupported: IsAttributeSupported::<Identity, OFFSET>,
            IsPropertySupported: IsPropertySupported::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetPropertySize: GetPropertySize::<Identity, OFFSET>,
            IsQueryStateSupported: IsQueryStateSupported::<Identity, OFFSET>,
            QueryState: QueryState::<Identity, OFFSET>,
            IsSetStateSupported: IsSetStateSupported::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            GetFactory: GetFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXCoreAdapter {}
windows_core::imp::define_interface!(IDXCoreAdapterFactory, IDXCoreAdapterFactory_Vtbl, 0x78ee5945_c36e_4b13_a669_005dd11c0f06);
windows_core::imp::interface_hierarchy!(IDXCoreAdapterFactory, windows_core::IUnknown);
impl IDXCoreAdapterFactory {
    pub unsafe fn CreateAdapterList<T>(&self, filterattributes: &[windows_core::GUID]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateAdapterList)(windows_core::Interface::as_raw(self), filterattributes.len().try_into().unwrap(), core::mem::transmute(filterattributes.as_ptr()), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetAdapterByLuid<T>(&self, adapterluid: *const super::super::Foundation::LUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAdapterByLuid)(windows_core::Interface::as_raw(self), adapterluid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsNotificationTypeSupported)(windows_core::Interface::as_raw(self), notificationtype) }
    }
    pub unsafe fn RegisterEventNotification<P0>(&self, dxcoreobject: P0, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEventNotification)(windows_core::Interface::as_raw(self), dxcoreobject.param().abi(), notificationtype, callbackfunction, callbackcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterEventNotification(&self, eventcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterEventNotification)(windows_core::Interface::as_raw(self), eventcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateAdapterList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterByLuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::LUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsNotificationTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreNotificationType) -> bool,
    pub RegisterEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DXCoreNotificationType, PFN_DXCORE_NOTIFICATION_CALLBACK, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnregisterEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDXCoreAdapterFactory_Impl: windows_core::IUnknownImpl {
    fn CreateAdapterList(&self, numattributes: u32, filterattributes: *const windows_core::GUID, riid: *const windows_core::GUID, ppvadapterlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterByLuid(&self, adapterluid: *const super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool;
    fn RegisterEventNotification(&self, dxcoreobject: windows_core::Ref<windows_core::IUnknown>, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn UnregisterEventNotification(&self, eventcookie: u32) -> windows_core::Result<()>;
}
impl IDXCoreAdapterFactory_Vtbl {
    pub const fn new<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateAdapterList<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numattributes: u32, filterattributes: *const windows_core::GUID, riid: *const windows_core::GUID, ppvadapterlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterFactory_Impl::CreateAdapterList(this, core::mem::transmute_copy(&numattributes), core::mem::transmute_copy(&filterattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapterlist)).into()
            }
        }
        unsafe extern "system" fn GetAdapterByLuid<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterFactory_Impl::GetAdapterByLuid(this, core::mem::transmute_copy(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterFactory_Impl::IsNotificationTypeSupported(this, core::mem::transmute_copy(&notificationtype))
            }
        }
        unsafe extern "system" fn RegisterEventNotification<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoreobject: *mut core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const core::ffi::c_void, eventcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXCoreAdapterFactory_Impl::RegisterEventNotification(this, core::mem::transmute_copy(&dxcoreobject), core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&callbackfunction), core::mem::transmute_copy(&callbackcontext)) {
                    Ok(ok__) => {
                        eventcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterEventNotification<Identity: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterFactory_Impl::UnregisterEventNotification(this, core::mem::transmute_copy(&eventcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAdapterList: CreateAdapterList::<Identity, OFFSET>,
            GetAdapterByLuid: GetAdapterByLuid::<Identity, OFFSET>,
            IsNotificationTypeSupported: IsNotificationTypeSupported::<Identity, OFFSET>,
            RegisterEventNotification: RegisterEventNotification::<Identity, OFFSET>,
            UnregisterEventNotification: UnregisterEventNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapterFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXCoreAdapterFactory {}
windows_core::imp::define_interface!(IDXCoreAdapterList, IDXCoreAdapterList_Vtbl, 0x526c7776_40e9_459b_b711_f32ad76dfc28);
windows_core::imp::interface_hierarchy!(IDXCoreAdapterList, windows_core::IUnknown);
impl IDXCoreAdapterList {
    pub unsafe fn GetAdapter<T>(&self, index: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), index, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsStale(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsStale)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFactory<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Sort(&self, preferences: &[DXCoreAdapterPreference]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), preferences.len().try_into().unwrap(), core::mem::transmute(preferences.as_ptr())).ok() }
    }
    pub unsafe fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsAdapterPreferenceSupported)(windows_core::Interface::as_raw(self), preference) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXCoreAdapterList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsStale: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sort: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DXCoreAdapterPreference) -> windows_core::HRESULT,
    pub IsAdapterPreferenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, DXCoreAdapterPreference) -> bool,
}
pub trait IDXCoreAdapterList_Impl: windows_core::IUnknownImpl {
    fn GetAdapter(&self, index: u32, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn IsStale(&self) -> bool;
    fn GetFactory(&self, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Sort(&self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> windows_core::Result<()>;
    fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool;
}
impl IDXCoreAdapterList_Vtbl {
    pub const fn new<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::GetAdapter(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::GetAdapterCount(this)
            }
        }
        unsafe extern "system" fn IsStale<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::IsStale(this)
            }
        }
        unsafe extern "system" fn GetFactory<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::GetFactory(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvfactory)).into()
            }
        }
        unsafe extern "system" fn Sort<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::Sort(this, core::mem::transmute_copy(&numpreferences), core::mem::transmute_copy(&preferences)).into()
            }
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Identity: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXCoreAdapterList_Impl::IsAdapterPreferenceSupported(this, core::mem::transmute_copy(&preference))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, OFFSET>,
            IsStale: IsStale::<Identity, OFFSET>,
            GetFactory: GetFactory::<Identity, OFFSET>,
            Sort: Sort::<Identity, OFFSET>,
            IsAdapterPreferenceSupported: IsAdapterPreferenceSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapterList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXCoreAdapterList {}
pub const InstanceLuid: DXCoreAdapterProperty = DXCoreAdapterProperty(0u32);
pub const IsDetachable: DXCoreAdapterProperty = DXCoreAdapterProperty(13u32);
pub const IsDriverUpdateInProgress: DXCoreAdapterState = DXCoreAdapterState(0u32);
pub const IsHardware: DXCoreAdapterProperty = DXCoreAdapterProperty(11u32);
pub const IsIntegrated: DXCoreAdapterProperty = DXCoreAdapterProperty(12u32);
pub const KmdModelVersion: DXCoreAdapterProperty = DXCoreAdapterProperty(4u32);
pub const Local: DXCoreSegmentGroup = DXCoreSegmentGroup(0u32);
pub const MinimumPower: DXCoreAdapterPreference = DXCoreAdapterPreference(1u32);
pub const NonLocal: DXCoreSegmentGroup = DXCoreSegmentGroup(1u32);
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: windows_core::Ref<windows_core::IUnknown>, context: *const core::ffi::c_void)>;
pub const SharedSystemMemory: DXCoreAdapterProperty = DXCoreAdapterProperty(9u32);
pub const _FACDXCORE: u32 = 2176u32;
