#[inline]
pub unsafe fn DMProcessConfigXMLFiltered<P0>(pszxmlin: P0, rgszallowedcspnodes: &[windows_core::PCWSTR]) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dmprocessxmlfiltered.dll" "system" fn DMProcessConfigXMLFiltered(pszxmlin : windows_core::PCWSTR, rgszallowedcspnodes : *const windows_core::PCWSTR, dwnumallowedcspnodes : u32, pbstrxmlout : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DMProcessConfigXMLFiltered(pszxmlin.param().abi(), core::mem::transmute(rgszallowedcspnodes.as_ptr()), rgszallowedcspnodes.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
pub const CLSID_WPD_NAMESPACE_EXTENSION: windows_core::GUID = windows_core::GUID::from_u128(0x35786d3c_b075_49b9_88dd_029876e11c01);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DELETE_OBJECT_OPTIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_RADIO_STATE(pub i32);
pub const DEVPKEY_MTPBTH_IsConnected: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xea1237fa_589d_4472_84e4_0abe36fd62ef), pid: 2 };
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(2i32);
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(4i32);
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(5i32);
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(0i32);
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(3i32);
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(1i32);
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
pub const E_WPD_DEVICE_ALREADY_OPENED: windows_core::HRESULT = windows_core::HRESULT(0x802A0001_u32 as _);
pub const E_WPD_DEVICE_IS_HUNG: windows_core::HRESULT = windows_core::HRESULT(0x802A0006_u32 as _);
pub const E_WPD_DEVICE_NOT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x802A0002_u32 as _);
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x802A0003_u32 as _);
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x802A00CA_u32 as _);
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x802A0004_u32 as _);
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x802A00CB_u32 as _);
pub const E_WPD_OBJECT_NOT_COMMITED: windows_core::HRESULT = windows_core::HRESULT(0x802A0005_u32 as _);
pub const E_WPD_SERVICE_ALREADY_OPENED: windows_core::HRESULT = windows_core::HRESULT(0x802A00C8_u32 as _);
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x802A00CC_u32 as _);
pub const E_WPD_SERVICE_NOT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x802A00C9_u32 as _);
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: windows_core::HRESULT = windows_core::HRESULT(0x802A0065_u32 as _);
pub const E_WPD_SMS_INVALID_RECIPIENT: windows_core::HRESULT = windows_core::HRESULT(0x802A0064_u32 as _);
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x802A0066_u32 as _);
pub const EnumBthMtpConnectors: windows_core::GUID = windows_core::GUID::from_u128(0xa1570149_e645_4f43_8b0d_409b061db2fc);
pub const FACILITY_WPD: u32 = 42u32;
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: windows_core::GUID = windows_core::GUID::from_u128(0x6ac27878_a6fa_4155_ba85_f98f491d4f33);
pub const GUID_DEVINTERFACE_WPD_PRIVATE: windows_core::GUID = windows_core::GUID::from_u128(0xba0c718f_4ded_49b7_bdd3_fabe28661211);
pub const GUID_DEVINTERFACE_WPD_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0x9ef44f80_3d64_4246_a6aa_206f328d1edc);
windows_core::imp::define_interface!(IConnectionRequestCallback, IConnectionRequestCallback_Vtbl, 0x272c9ae0_7161_4ae0_91bd_9f448ee9c427);
windows_core::imp::interface_hierarchy!(IConnectionRequestCallback, windows_core::IUnknown);
impl IConnectionRequestCallback {
    pub unsafe fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), hrstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IConnectionRequestCallback_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IConnectionRequestCallback_Vtbl {
    pub const fn new<Identity: IConnectionRequestCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: IConnectionRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectionRequestCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionRequestCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConnectionRequestCallback {}
windows_core::imp::define_interface!(IEnumPortableDeviceConnectors, IEnumPortableDeviceConnectors_Vtbl, 0xbfdef549_9247_454f_bd82_06fe80853faa);
windows_core::imp::interface_hierarchy!(IEnumPortableDeviceConnectors, windows_core::IUnknown);
impl IEnumPortableDeviceConnectors {
    pub unsafe fn Next(&self, pconnectors: &mut [Option<IPortableDeviceConnector>], pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pconnectors.len().try_into().unwrap(), core::mem::transmute(pconnectors.as_ptr()), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, cconnectors: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnectors).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceConnectors> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceConnectors_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumPortableDeviceConnectors_Impl: windows_core::IUnknownImpl {
    fn Next(&self, crequested: u32, pconnectors: *mut Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cconnectors: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceConnectors>;
}
impl IEnumPortableDeviceConnectors_Vtbl {
    pub const fn new<Identity: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequested: u32, pconnectors: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceConnectors_Impl::Next(this, core::mem::transmute_copy(&crequested), core::mem::transmute_copy(&pconnectors), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnectors: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceConnectors_Impl::Skip(this, core::mem::transmute_copy(&cconnectors)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceConnectors_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumPortableDeviceConnectors_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPortableDeviceConnectors as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumPortableDeviceConnectors {}
windows_core::imp::define_interface!(IEnumPortableDeviceObjectIDs, IEnumPortableDeviceObjectIDs_Vtbl, 0x10ece955_cf41_4728_bfa0_41eedf1bbf19);
windows_core::imp::interface_hierarchy!(IEnumPortableDeviceObjectIDs, windows_core::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    pub unsafe fn Next(&self, pobjids: &mut [windows_core::PWSTR], pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pobjids.len().try_into().unwrap(), core::mem::transmute(pobjids.as_ptr()), pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cobjects: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cobjects) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceObjectIDs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumPortableDeviceObjectIDs_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cobjects: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IEnumPortableDeviceObjectIDs_Vtbl {
    pub const fn new<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Next(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&pobjids), core::mem::transmute_copy(&pcfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Skip(this, core::mem::transmute_copy(&cobjects))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumPortableDeviceObjectIDs_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumPortableDeviceObjectIDs_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumPortableDeviceObjectIDs {}
windows_core::imp::define_interface!(IMediaRadioManager, IMediaRadioManager_Vtbl, 0x6cfdcab5_fc47_42a5_9241_074b58830e73);
windows_core::imp::interface_hierarchy!(IMediaRadioManager, windows_core::IUnknown);
impl IMediaRadioManager {
    pub unsafe fn GetRadioInstances(&self) -> windows_core::Result<IRadioInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRadioInstances)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSystemRadioStateChange)(windows_core::Interface::as_raw(self), sysradiostate, utimeoutsec).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRadioInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSystemRadioStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, SYSTEM_RADIO_STATE, u32) -> windows_core::HRESULT,
}
pub trait IMediaRadioManager_Impl: windows_core::IUnknownImpl {
    fn GetRadioInstances(&self) -> windows_core::Result<IRadioInstanceCollection>;
    fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()>;
}
impl IMediaRadioManager_Vtbl {
    pub const fn new<Identity: IMediaRadioManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRadioInstances<Identity: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaRadioManager_Impl::GetRadioInstances(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Identity: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaRadioManager_Impl::OnSystemRadioStateChange(this, core::mem::transmute_copy(&sysradiostate), core::mem::transmute_copy(&utimeoutsec)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRadioInstances: GetRadioInstances::<Identity, OFFSET>,
            OnSystemRadioStateChange: OnSystemRadioStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaRadioManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaRadioManager {}
windows_core::imp::define_interface!(IMediaRadioManagerNotifySink, IMediaRadioManagerNotifySink_Vtbl, 0x89d81f5f_c147_49ed_a11c_77b20c31e7c9);
windows_core::imp::interface_hierarchy!(IMediaRadioManagerNotifySink, windows_core::IUnknown);
impl IMediaRadioManagerNotifySink {
    pub unsafe fn OnInstanceAdd<P0>(&self, pradioinstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRadioInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInstanceAdd)(windows_core::Interface::as_raw(self), pradioinstance.param().abi()).ok() }
    }
    pub unsafe fn OnInstanceRemove(&self, bstrradioinstanceid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnInstanceRemove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrradioinstanceid)).ok() }
    }
    pub unsafe fn OnInstanceRadioChange(&self, bstrradioinstanceid: &windows_core::BSTR, radiostate: DEVICE_RADIO_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnInstanceRadioChange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrradioinstanceid), radiostate).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInstanceAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInstanceRemove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInstanceRadioChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DEVICE_RADIO_STATE) -> windows_core::HRESULT,
}
pub trait IMediaRadioManagerNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnInstanceAdd(&self, pradioinstance: windows_core::Ref<IRadioInstance>) -> windows_core::Result<()>;
    fn OnInstanceRemove(&self, bstrradioinstanceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnInstanceRadioChange(&self, bstrradioinstanceid: &windows_core::BSTR, radiostate: DEVICE_RADIO_STATE) -> windows_core::Result<()>;
}
impl IMediaRadioManagerNotifySink_Vtbl {
    pub const fn new<Identity: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInstanceAdd<Identity: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pradioinstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaRadioManagerNotifySink_Impl::OnInstanceAdd(this, core::mem::transmute_copy(&pradioinstance)).into()
            }
        }
        unsafe extern "system" fn OnInstanceRemove<Identity: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrradioinstanceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaRadioManagerNotifySink_Impl::OnInstanceRemove(this, core::mem::transmute(&bstrradioinstanceid)).into()
            }
        }
        unsafe extern "system" fn OnInstanceRadioChange<Identity: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrradioinstanceid: *mut core::ffi::c_void, radiostate: DEVICE_RADIO_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaRadioManagerNotifySink_Impl::OnInstanceRadioChange(this, core::mem::transmute(&bstrradioinstanceid), core::mem::transmute_copy(&radiostate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInstanceAdd: OnInstanceAdd::<Identity, OFFSET>,
            OnInstanceRemove: OnInstanceRemove::<Identity, OFFSET>,
            OnInstanceRadioChange: OnInstanceRadioChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaRadioManagerNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaRadioManagerNotifySink {}
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
windows_core::imp::define_interface!(IPortableDevice, IPortableDevice_Vtbl, 0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
windows_core::imp::interface_hierarchy!(IPortableDevice, windows_core::IUnknown);
impl IPortableDevice {
    pub unsafe fn Open<P0, P1>(&self, pszpnpdeviceid: P0, pclientinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pclientinfo.param().abi()).ok() }
    }
    pub unsafe fn SendCommand<P1>(&self, dwflags: u32, pparameters: P1) -> windows_core::Result<IPortableDeviceValues>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCommand)(windows_core::Interface::as_raw(self), dwflags, pparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Content(&self) -> windows_core::Result<IPortableDeviceContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Capabilities(&self) -> windows_core::Result<IPortableDeviceCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Advise<P1, P2>(&self, dwflags: u32, pcallback: P1, pparameters: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<IPortableDeviceEventCallback>,
        P2: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi(), pparameters.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pszcookie.param().abi()).ok() }
    }
    pub unsafe fn GetPnPDeviceID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPnPDeviceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPnPDeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IPortableDevice_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pszpnpdeviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceCapabilities>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Advise(&self, dwflags: u32, pcallback: windows_core::Ref<IPortableDeviceEventCallback>, pparameters: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPnPDeviceID(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IPortableDevice_Vtbl {
    pub const fn new<Identity: IPortableDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Open(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pclientinfo)).into()
            }
        }
        unsafe extern "system" fn SendCommand<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Content<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Content(this) {
                    Ok(ok__) => {
                        ppcontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Capabilities<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::Advise(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppszcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevice_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
            }
        }
        unsafe extern "system" fn GetPnPDeviceID<Identity: IPortableDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevice_Impl::GetPnPDeviceID(this) {
                    Ok(ok__) => {
                        ppszpnpdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDevice {}
windows_core::imp::define_interface!(IPortableDeviceCapabilities, IPortableDeviceCapabilities_Vtbl, 0x2c8c6dbf_e3dc_4061_becc_8542e810d126);
windows_core::imp::interface_hierarchy!(IPortableDeviceCapabilities, windows_core::IUnknown);
impl IPortableDeviceCapabilities {
    pub unsafe fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCommandOptions)(windows_core::Interface::as_raw(self), command, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunctionalCategories(&self) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionalCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunctionalObjects(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionalObjects)(windows_core::Interface::as_raw(self), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedContentTypes(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedContentTypes)(windows_core::Interface::as_raw(self), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormats)(windows_core::Interface::as_raw(self), contenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormatProperties)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const windows_core::GUID, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFixedPropertyAttributes)(windows_core::Interface::as_raw(self), format, key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedEvents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEventOptions(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventOptions)(windows_core::Interface::as_raw(self), event, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCommandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunctionalCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunctionalObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFixedPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceCapabilities_Impl: windows_core::IUnknownImpl {
    fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetFunctionalCategories(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(&self, contenttype: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(&self, format: *const windows_core::GUID, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventOptions(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
}
impl IPortableDeviceCapabilities_Vtbl {
    pub const fn new<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedCommands<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedCommands(this) {
                    Ok(ok__) => {
                        ppcommands.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::super::Foundation::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcategories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFunctionalCategories(this) {
                    Ok(ok__) => {
                        ppcategories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFunctionalObjects(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        ppobjectids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppcontenttypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedContentTypes(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        ppcontenttypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *const windows_core::GUID, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedFormats(this, core::mem::transmute_copy(&contenttype)) {
                    Ok(ok__) => {
                        ppformats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, key: *const super::super::Foundation::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetFixedPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceCapabilities_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetSupportedEvents(this) {
                    Ok(ok__) => {
                        ppevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventOptions<Identity: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceCapabilities_Impl::GetEventOptions(this, core::mem::transmute_copy(&event)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Identity, OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Identity, OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventOptions: GetEventOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceCapabilities {}
windows_core::imp::define_interface!(IPortableDeviceConnector, IPortableDeviceConnector_Vtbl, 0x625e2df8_6392_4cf0_9ad1_3cfa5f17775c);
windows_core::imp::interface_hierarchy!(IPortableDeviceConnector, windows_core::IUnknown);
impl IPortableDeviceConnector {
    pub unsafe fn Connect<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConnectionRequestCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn Disconnect<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConnectionRequestCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn Cancel<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConnectionRequestCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn GetProperty(&self, ppropertykey: *const super::super::Foundation::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), ppropertykey, ppropertytype as _, ppdata as _, pcbdata as _).ok() }
    }
    #[cfg(feature = "Win32_Devices_Properties")]
    pub unsafe fn SetProperty(&self, ppropertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), ppropertykey, propertytype, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetPnPID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPnPID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::DEVPROPKEY, *mut super::Properties::DEVPROPTYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Devices_Properties")]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::DEVPROPKEY, super::Properties::DEVPROPTYPE, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))]
    SetProperty: usize,
    pub GetPnPID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Devices_Properties")]
pub trait IPortableDeviceConnector_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, pcallback: windows_core::Ref<IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn Disconnect(&self, pcallback: windows_core::Ref<IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn Cancel(&self, pcallback: windows_core::Ref<IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn GetProperty(&self, ppropertykey: *const super::super::Foundation::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn SetProperty(&self, ppropertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn GetPnPID(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_Properties")]
impl IPortableDeviceConnector_Vtbl {
    pub const fn new<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceConnector_Impl::Connect(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceConnector_Impl::Disconnect(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceConnector_Impl::Cancel(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertykey: *const super::super::Foundation::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceConnector_Impl::GetProperty(this, core::mem::transmute_copy(&ppropertykey), core::mem::transmute_copy(&ppropertytype), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceConnector_Impl::SetProperty(this, core::mem::transmute_copy(&ppropertykey), core::mem::transmute_copy(&propertytype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn GetPnPID<Identity: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszpnpid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceConnector_Impl::GetPnPID(this) {
                    Ok(ok__) => {
                        ppwszpnpid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetPnPID: GetPnPID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceConnector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::RuntimeName for IPortableDeviceConnector {}
windows_core::imp::define_interface!(IPortableDeviceContent, IPortableDeviceContent_Vtbl, 0x6a96ed84_7c73_4480_9938_bf5af477d426);
windows_core::imp::interface_hierarchy!(IPortableDeviceContent, windows_core::IUnknown);
impl IPortableDeviceContent {
    pub unsafe fn EnumObjects<P1, P2>(&self, dwflags: u32, pszparentobjectid: P1, pfilter: P2) -> windows_core::Result<IEnumPortableDeviceObjectIDs>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), dwflags, pszparentobjectid.param().abi(), pfilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IPortableDeviceProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Transfer(&self) -> windows_core::Result<IPortableDeviceResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Transfer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<P0>(&self, pvalues: P0, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectWithPropertiesOnly)(windows_core::Interface::as_raw(self), pvalues.param().abi(), ppszobjectid as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateObjectWithPropertiesAndData<P0>(&self, pvalues: P0, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectWithPropertiesAndData)(windows_core::Interface::as_raw(self), pvalues.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _, ppszcookie as _).ok() }
    }
    pub unsafe fn Delete<P1>(&self, dwoptions: u32, pobjectids: P1, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDevicePropVariantCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwoptions, pobjectids.param().abi(), core::mem::transmute(ppresults)).ok() }
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<P0>(&self, ppersistentuniqueids: P0) -> windows_core::Result<IPortableDevicePropVariantCollection>
    where
        P0: windows_core::Param<IPortableDevicePropVariantCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectIDsFromPersistentUniqueIDs)(windows_core::Interface::as_raw(self), ppersistentuniqueids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Move<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pszdestinationfolderobjectid.param().abi(), core::mem::transmute(ppresults)).ok() }
    }
    pub unsafe fn Copy<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pszdestinationfolderobjectid.param().abi(), core::mem::transmute(ppresults)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Transfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateObjectWithPropertiesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateObjectWithPropertiesAndData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateObjectWithPropertiesAndData: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectIDsFromPersistentUniqueIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent_Impl: windows_core::IUnknownImpl {
    fn EnumObjects(&self, dwflags: u32, pszparentobjectid: &windows_core::PCWSTR, pfilter: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(&self) -> windows_core::Result<IPortableDeviceProperties>;
    fn Transfer(&self) -> windows_core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(&self, pvalues: windows_core::Ref<IPortableDeviceValues>, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn CreateObjectWithPropertiesAndData(&self, pvalues: windows_core::Ref<IPortableDeviceValues>, ppdata: windows_core::OutRef<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Delete(&self, dwoptions: u32, pobjectids: windows_core::Ref<IPortableDevicePropVariantCollection>, ppresults: windows_core::OutRef<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(&self, ppersistentuniqueids: windows_core::Ref<IPortableDevicePropVariantCollection>) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Move(&self, pobjectids: windows_core::Ref<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: windows_core::OutRef<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn Copy(&self, pobjectids: windows_core::Ref<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: windows_core::OutRef<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent_Vtbl {
    pub const fn new<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumObjects<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszparentobjectid: windows_core::PCWSTR, pfilter: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::EnumObjects(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszparentobjectid), core::mem::transmute_copy(&pfilter)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Transfer<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::Transfer(this) {
                    Ok(ok__) => {
                        ppresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::CreateObjectWithPropertiesOnly(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&ppszobjectid)).into()
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::CreateObjectWithPropertiesAndData(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32, pobjectids: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Delete(this, core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&pobjectids), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppersistentuniqueids: *mut core::ffi::c_void, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceContent_Impl::GetObjectIDsFromPersistentUniqueIDs(this, core::mem::transmute_copy(&ppersistentuniqueids)) {
                    Ok(ok__) => {
                        ppobjectids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Move(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent_Impl::Copy(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Transfer: Transfer::<Identity, OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Identity, OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceContent {}
windows_core::imp::define_interface!(IPortableDeviceContent2, IPortableDeviceContent2_Vtbl, 0x9b4add96_f6bf_4034_8708_eca72bf10554);
impl core::ops::Deref for IPortableDeviceContent2 {
    type Target = IPortableDeviceContent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPortableDeviceContent2, windows_core::IUnknown, IPortableDeviceContent);
impl IPortableDeviceContent2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateObjectWithPropertiesAndData<P0, P1>(&self, pszobjectid: P0, pproperties: P1, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateObjectWithPropertiesAndData)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pproperties.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2_Vtbl {
    pub base__: IPortableDeviceContent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateObjectWithPropertiesAndData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateObjectWithPropertiesAndData: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent2_Impl: IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(&self, pszobjectid: &windows_core::PCWSTR, pproperties: windows_core::Ref<IPortableDeviceValues>, ppdata: windows_core::OutRef<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent2_Vtbl {
    pub const fn new<Identity: IPortableDeviceContent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Identity: IPortableDeviceContent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pproperties: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceContent2_Impl::UpdateObjectWithPropertiesAndData(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pproperties), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into()
            }
        }
        Self {
            base__: IPortableDeviceContent_Vtbl::new::<Identity, OFFSET>(),
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as windows_core::Interface>::IID || iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceContent2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPortableDeviceDataStream, IPortableDeviceDataStream_Vtbl, 0x88e04db3_1012_4d64_9996_f703a950d3f4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPortableDeviceDataStream {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPortableDeviceDataStream, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStream {
    pub unsafe fn GetObjectID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStream_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub GetObjectID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDataStream_Impl: super::super::System::Com::IStream_Impl {
    fn GetObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStream_Vtbl {
    pub const fn new<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectID<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceDataStream_Impl::GetObjectID(this) {
                    Ok(ok__) => {
                        ppszobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceDataStream_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            GetObjectID: GetObjectID::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceDataStream {}
windows_core::imp::define_interface!(IPortableDeviceDispatchFactory, IPortableDeviceDispatchFactory_Vtbl, 0x5e1eafc3_e3d7_4132_96fa_759c0f9d1e0f);
windows_core::imp::interface_hierarchy!(IPortableDeviceDispatchFactory, windows_core::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceDispatch<P0>(&self, pszpnpdeviceid: P0) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceDispatch)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceDispatch: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDispatchFactory_Impl: windows_core::IUnknownImpl {
    fn GetDeviceDispatch(&self, pszpnpdeviceid: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDispatchFactory_Vtbl {
    pub const fn new<Identity: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceDispatch<Identity: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, ppdevicedispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceDispatchFactory_Impl::GetDeviceDispatch(this, core::mem::transmute(&pszpnpdeviceid)) {
                    Ok(ok__) => {
                        ppdevicedispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceDispatch: GetDeviceDispatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceDispatchFactory {}
windows_core::imp::define_interface!(IPortableDeviceEventCallback, IPortableDeviceEventCallback_Vtbl, 0xa8792a31_f385_493c_a893_40f64eb45f6e);
windows_core::imp::interface_hierarchy!(IPortableDeviceEventCallback, windows_core::IUnknown);
impl IPortableDeviceEventCallback {
    pub unsafe fn OnEvent<P0>(&self, peventparameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), peventparameters.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceEventCallback_Impl: windows_core::IUnknownImpl {
    fn OnEvent(&self, peventparameters: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
}
impl IPortableDeviceEventCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEvent<Identity: IPortableDeviceEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceEventCallback_Impl::OnEvent(this, core::mem::transmute_copy(&peventparameters)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceEventCallback {}
windows_core::imp::define_interface!(IPortableDeviceKeyCollection, IPortableDeviceKeyCollection_Vtbl, 0xdada2357_e0ad_492e_98db_dd61c53ba353);
windows_core::imp::interface_hierarchy!(IPortableDeviceKeyCollection, windows_core::IUnknown);
impl IPortableDeviceKeyCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok() }
    }
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, pkey).ok() }
    }
    pub unsafe fn Add(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), key).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IPortableDeviceKeyCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn Add(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl IPortableDeviceKeyCollection_Vtbl {
    pub const fn new<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceKeyCollection {}
windows_core::imp::define_interface!(IPortableDeviceManager, IPortableDeviceManager_Vtbl, 0xa1567595_4c2f_4574_a6fa_ecef917b9a40);
windows_core::imp::interface_hierarchy!(IPortableDeviceManager, windows_core::IUnknown);
impl IPortableDeviceManager {
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDevices)(windows_core::Interface::as_raw(self), ppnpdeviceids as _, pcpnpdeviceids as _).ok() }
    }
    pub unsafe fn RefreshDeviceList(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshDeviceList)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDeviceFriendlyName<P0>(&self, pszpnpdeviceid: P0, pdevicefriendlyname: windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceFriendlyName)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), core::mem::transmute(pdevicefriendlyname), pcchdevicefriendlyname as _).ok() }
    }
    pub unsafe fn GetDeviceDescription<P0>(&self, pszpnpdeviceid: P0, pdevicedescription: windows_core::PWSTR, pcchdevicedescription: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceDescription)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), core::mem::transmute(pdevicedescription), pcchdevicedescription as _).ok() }
    }
    pub unsafe fn GetDeviceManufacturer<P0>(&self, pszpnpdeviceid: P0, pdevicemanufacturer: windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), core::mem::transmute(pdevicemanufacturer), pcchdevicemanufacturer as _).ok() }
    }
    pub unsafe fn GetDeviceProperty<P0, P1>(&self, pszpnpdeviceid: P0, pszdevicepropertyname: P1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceProperty)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), pszdevicepropertyname.param().abi(), pdata as _, pcbdata as _, pdwtype as _).ok() }
    }
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateDevices)(windows_core::Interface::as_raw(self), ppnpdeviceids as _, pcpnpdeviceids as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub RefreshDeviceList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u8, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPrivateDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IPortableDeviceManager_Impl: windows_core::IUnknownImpl {
    fn GetDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
    fn RefreshDeviceList(&self) -> windows_core::Result<()>;
    fn GetDeviceFriendlyName(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicefriendlyname: windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceDescription(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicedescription: windows_core::PWSTR, pcchdevicedescription: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicemanufacturer: windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceProperty(&self, pszpnpdeviceid: &windows_core::PCWSTR, pszdevicepropertyname: &windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::Result<()>;
    fn GetPrivateDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
}
impl IPortableDeviceManager_Vtbl {
    pub const fn new<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevices<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
            }
        }
        unsafe extern "system" fn RefreshDeviceList<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::RefreshDeviceList(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicefriendlyname: windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceFriendlyName(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicefriendlyname), core::mem::transmute_copy(&pcchdevicefriendlyname)).into()
            }
        }
        unsafe extern "system" fn GetDeviceDescription<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicedescription: windows_core::PWSTR, pcchdevicedescription: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceDescription(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicedescription), core::mem::transmute_copy(&pcchdevicedescription)).into()
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicemanufacturer: windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceManufacturer(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&pdevicemanufacturer), core::mem::transmute_copy(&pcchdevicemanufacturer)).into()
            }
        }
        unsafe extern "system" fn GetDeviceProperty<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pszdevicepropertyname: windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetDeviceProperty(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pszdevicepropertyname), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&pdwtype)).into()
            }
        }
        unsafe extern "system" fn GetPrivateDevices<Identity: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceManager_Impl::GetPrivateDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevices: GetDevices::<Identity, OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Identity, OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Identity, OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Identity, OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceManager {}
windows_core::imp::define_interface!(IPortableDevicePropVariantCollection, IPortableDevicePropVariantCollection_Vtbl, 0x89b2e422_4f1b_4316_bcef_a44afea83eb3);
windows_core::imp::interface_hierarchy!(IPortableDevicePropVariantCollection, windows_core::IUnknown);
impl IPortableDevicePropVariantCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeType(&self, vt: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeType)(windows_core::Interface::as_raw(self), vt).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Add: usize,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPortableDevicePropVariantCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<u16>;
    fn ChangeType(&self, vt: u16) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPortableDevicePropVariantCollection_Vtbl {
    pub const fn new<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvt: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropVariantCollection_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeType<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vt: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::ChangeType(this, core::mem::transmute_copy(&vt)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            ChangeType: ChangeType::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPortableDevicePropVariantCollection {}
windows_core::imp::define_interface!(IPortableDeviceProperties, IPortableDeviceProperties_Vtbl, 0x7f6d695c_03df_4439_a809_59266beee3a6);
windows_core::imp::interface_hierarchy!(IPortableDeviceProperties, windows_core::IUnknown);
impl IPortableDeviceProperties {
    pub unsafe fn GetSupportedProperties<P0>(&self, pszobjectid: P0) -> windows_core::Result<IPortableDeviceKeyCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedProperties)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyAttributes<P0>(&self, pszobjectid: P0, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyAttributes)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetValues<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::Result<IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValues)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetValues<P0, P1>(&self, pszobjectid: P0, pvalues: P1) -> windows_core::Result<IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetValues)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pvalues.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi()).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValues: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValues: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceProperties_Impl: windows_core::IUnknownImpl {
    fn GetSupportedProperties(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetValues(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<IPortableDeviceKeyCollection>) -> windows_core::Result<IPortableDeviceValues>;
    fn SetValues(&self, pszobjectid: &windows_core::PCWSTR, pvalues: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IPortableDeviceProperties_Vtbl {
    pub const fn new<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedProperties<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetSupportedProperties(this, core::mem::transmute(&pszobjectid)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetPropertyAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValues<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::GetValues(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValues<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pvalues: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceProperties_Impl::SetValues(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pvalues)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceProperties_Impl::Delete(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceProperties_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedProperties: GetSupportedProperties::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetValues: GetValues::<Identity, OFFSET>,
            SetValues: SetValues::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceProperties {}
windows_core::imp::define_interface!(IPortableDevicePropertiesBulk, IPortableDevicePropertiesBulk_Vtbl, 0x482b05c0_4056_44ed_9e0f_5e23b009da93);
windows_core::imp::interface_hierarchy!(IPortableDevicePropertiesBulk, windows_core::IUnknown);
impl IPortableDevicePropertiesBulk {
    pub unsafe fn QueueGetValuesByObjectList<P0, P1, P2>(&self, pobjectids: P0, pkeys: P1, pcallback: P2) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<IPortableDevicePropVariantCollection>,
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
        P2: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGetValuesByObjectList)(windows_core::Interface::as_raw(self), pobjectids.param().abi(), pkeys.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueGetValuesByObjectFormat<P1, P3, P4>(&self, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: P1, dwdepth: u32, pkeys: P3, pcallback: P4) -> windows_core::Result<windows_core::GUID>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IPortableDeviceKeyCollection>,
        P4: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGetValuesByObjectFormat)(windows_core::Interface::as_raw(self), pguidobjectformat, pszparentobjectid.param().abi(), dwdepth, pkeys.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueSetValuesByObjectList<P0, P1>(&self, pobjectvalues: P0, pcallback: P1) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<IPortableDeviceValuesCollection>,
        P1: windows_core::Param<IPortableDevicePropertiesBulkCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueSetValuesByObjectList)(windows_core::Interface::as_raw(self), pobjectvalues.param().abi(), pcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Start(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pcontext).ok() }
    }
    pub unsafe fn Cancel(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pcontext).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueueGetValuesByObjectList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub QueueGetValuesByObjectFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub QueueSetValuesByObjectList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IPortableDevicePropertiesBulk_Impl: windows_core::IUnknownImpl {
    fn QueueGetValuesByObjectList(&self, pobjectids: windows_core::Ref<IPortableDevicePropVariantCollection>, pkeys: windows_core::Ref<IPortableDeviceKeyCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueGetValuesByObjectFormat(&self, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: &windows_core::PCWSTR, dwdepth: u32, pkeys: windows_core::Ref<IPortableDeviceKeyCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueSetValuesByObjectList(&self, pobjectvalues: windows_core::Ref<IPortableDeviceValuesCollection>, pcallback: windows_core::Ref<IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn Start(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Cancel(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IPortableDevicePropertiesBulk_Vtbl {
    pub const fn new<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGetValuesByObjectList<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectList(this, core::mem::transmute_copy(&pobjectids), core::mem::transmute_copy(&pkeys), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: windows_core::PCWSTR, dwdepth: u32, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectFormat(this, core::mem::transmute_copy(&pguidobjectformat), core::mem::transmute(&pszparentobjectid), core::mem::transmute_copy(&dwdepth), core::mem::transmute_copy(&pkeys), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectvalues: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropertiesBulk_Impl::QueueSetValuesByObjectList(this, core::mem::transmute_copy(&pobjectvalues), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulk_Impl::Start(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulk_Impl::Cancel(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Identity, OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Identity, OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDevicePropertiesBulk {}
windows_core::imp::define_interface!(IPortableDevicePropertiesBulkCallback, IPortableDevicePropertiesBulkCallback_Vtbl, 0x9deacb80_11e8_40e3_a9f3_f557986a7845);
windows_core::imp::interface_hierarchy!(IPortableDevicePropertiesBulkCallback, windows_core::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    pub unsafe fn OnStart(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStart)(windows_core::Interface::as_raw(self), pcontext).ok() }
    }
    pub unsafe fn OnProgress<P1>(&self, pcontext: *const windows_core::GUID, presults: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValuesCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), pcontext, presults.param().abi()).ok() }
    }
    pub unsafe fn OnEnd(&self, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEnd)(windows_core::Interface::as_raw(self), pcontext, hrstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IPortableDevicePropertiesBulkCallback_Impl: windows_core::IUnknownImpl {
    fn OnStart(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnProgress(&self, pcontext: *const windows_core::GUID, presults: windows_core::Ref<IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn OnEnd(&self, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IPortableDevicePropertiesBulkCallback_Vtbl {
    pub const fn new<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStart<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnStart(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, presults: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnProgress(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&presults)).into()
            }
        }
        unsafe extern "system" fn OnEnd<Identity: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropertiesBulkCallback_Impl::OnEnd(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStart: OnStart::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnEnd: OnEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDevicePropertiesBulkCallback {}
windows_core::imp::define_interface!(IPortableDeviceResources, IPortableDeviceResources_Vtbl, 0xfd8878ac_d841_4d17_891c_e6829cdb6934);
windows_core::imp::interface_hierarchy!(IPortableDeviceResources, windows_core::IUnknown);
impl IPortableDeviceResources {
    pub unsafe fn GetSupportedResources<P0>(&self, pszobjectid: P0) -> windows_core::Result<IPortableDeviceKeyCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedResources)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResourceAttributes<P0>(&self, pszobjectid: P0, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceAttributes)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream<P0>(&self, pszobjectid: P0, key: *const super::super::Foundation::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), key, dwmode, pdwoptimalbuffersize as _, core::mem::transmute(ppstream)).ok() }
    }
    pub unsafe fn Delete<P0, P1>(&self, pszobjectid: P0, pkeys: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pszobjectid.param().abi(), pkeys.param().abi()).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResource<P0>(&self, presourceattributes: P0, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateResource)(windows_core::Interface::as_raw(self), presourceattributes.param().abi(), core::mem::transmute(ppdata), pdwoptimalwritebuffersize as _, ppszcookie as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResources_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedResources: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourceAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::PROPERTYKEY, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResource: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceResources_Impl: windows_core::IUnknownImpl {
    fn GetSupportedResources(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetStream(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: windows_core::OutRef<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: windows_core::Ref<IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn CreateResource(&self, presourceattributes: windows_core::Ref<IPortableDeviceValues>, ppdata: windows_core::OutRef<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceResources_Vtbl {
    pub const fn new<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedResources<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceResources_Impl::GetSupportedResources(this, core::mem::transmute(&pszobjectid)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY, ppresourceattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceResources_Impl::GetResourceAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppresourceattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::Foundation::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::GetStream(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdwoptimalbuffersize), core::mem::transmute_copy(&ppstream)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::Delete(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&pkeys)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn CreateResource<Identity: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceattributes: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceResources_Impl::CreateResource(this, core::mem::transmute_copy(&presourceattributes), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedResources: GetSupportedResources::<Identity, OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            CreateResource: CreateResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceResources as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceResources {}
windows_core::imp::define_interface!(IPortableDeviceService, IPortableDeviceService_Vtbl, 0xd3bd3a44_d7b5_40a9_98b7_2fa4d01dec08);
windows_core::imp::interface_hierarchy!(IPortableDeviceService, windows_core::IUnknown);
impl IPortableDeviceService {
    pub unsafe fn Open<P0, P1>(&self, pszpnpserviceid: P0, pclientinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), pclientinfo.param().abi()).ok() }
    }
    pub unsafe fn Capabilities(&self) -> windows_core::Result<IPortableDeviceServiceCapabilities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Content(&self) -> windows_core::Result<IPortableDeviceContent2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Methods(&self) -> windows_core::Result<IPortableDeviceServiceMethods> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Methods)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetServiceObjectID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceObjectID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPnPServiceID(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPnPServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Advise<P1, P2>(&self, dwflags: u32, pcallback: P1, pparameters: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P1: windows_core::Param<IPortableDeviceEventCallback>,
        P2: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), dwflags, pcallback.param().abi(), pparameters.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise<P0>(&self, pszcookie: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), pszcookie.param().abi()).ok() }
    }
    pub unsafe fn SendCommand<P1>(&self, dwflags: u32, pparameters: P1) -> windows_core::Result<IPortableDeviceValues>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCommand)(windows_core::Interface::as_raw(self), dwflags, pparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Methods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceObjectID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPnPServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SendCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceService_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent2>;
    fn Methods(&self) -> windows_core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetServiceObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPnPServiceID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Advise(&self, dwflags: u32, pcallback: windows_core::Ref<IPortableDeviceEventCallback>, pparameters: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
}
impl IPortableDeviceService_Vtbl {
    pub const fn new<Identity: IPortableDeviceService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Open(this, core::mem::transmute(&pszpnpserviceid), core::mem::transmute_copy(&pclientinfo)).into()
            }
        }
        unsafe extern "system" fn Capabilities<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        ppcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Content<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Content(this) {
                    Ok(ok__) => {
                        ppcontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Methods<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Methods(this) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetServiceObjectID<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszserviceobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::GetServiceObjectID(this) {
                    Ok(ok__) => {
                        ppszserviceobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpserviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::GetPnPServiceID(this) {
                    Ok(ok__) => {
                        ppszpnpserviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::Advise(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppszcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceService_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
            }
        }
        unsafe extern "system" fn SendCommand<Identity: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceService_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pparameters)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Methods: Methods::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Identity, OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceService {}
windows_core::imp::define_interface!(IPortableDeviceServiceActivation, IPortableDeviceServiceActivation_Vtbl, 0xe56b0534_d9b9_425c_9b99_75f97cb3d7c8);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceActivation, windows_core::IUnknown);
impl IPortableDeviceServiceActivation {
    pub unsafe fn OpenAsync<P0, P1, P2>(&self, pszpnpserviceid: P0, pclientinfo: P1, pcallback: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IPortableDeviceValues>,
        P2: windows_core::Param<IPortableDeviceServiceOpenCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenAsync)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), pclientinfo.param().abi(), pcallback.param().abi()).ok() }
    }
    pub unsafe fn CancelOpenAsync(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelOpenAsync)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelOpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceActivation_Impl: windows_core::IUnknownImpl {
    fn OpenAsync(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: windows_core::Ref<IPortableDeviceValues>, pcallback: windows_core::Ref<IPortableDeviceServiceOpenCallback>) -> windows_core::Result<()>;
    fn CancelOpenAsync(&self) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceActivation_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenAsync<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceActivation_Impl::OpenAsync(this, core::mem::transmute(&pszpnpserviceid), core::mem::transmute_copy(&pclientinfo), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn CancelOpenAsync<Identity: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceActivation_Impl::CancelOpenAsync(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceActivation {}
windows_core::imp::define_interface!(IPortableDeviceServiceCapabilities, IPortableDeviceServiceCapabilities_Vtbl, 0x24dbd89d_413e_43e0_bd5b_197f3c56c886);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceCapabilities, windows_core::IUnknown);
impl IPortableDeviceServiceCapabilities {
    pub unsafe fn GetSupportedMethods(&self) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedMethods)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedMethodsByFormat)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMethodAttributes(&self, method: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodAttributes)(windows_core::Interface::as_raw(self), method, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodParameterAttributes)(windows_core::Interface::as_raw(self), method, parameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedFormats(&self) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormats)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFormatAttributes(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatAttributes)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedFormatProperties)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const windows_core::GUID, property: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatPropertyAttributes)(windows_core::Interface::as_raw(self), format, property, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedEvents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEventAttributes(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventAttributes)(windows_core::Interface::as_raw(self), event, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEventParameterAttributes(&self, event: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventParameterAttributes)(windows_core::Interface::as_raw(self), event, parameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInheritedServices)(windows_core::Interface::as_raw(self), dwinheritancetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValuesCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatRenderingProfiles)(windows_core::Interface::as_raw(self), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCommandOptions)(windows_core::Interface::as_raw(self), command, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedMethodsByFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodParameterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormatAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFormatProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormatPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventParameterAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInheritedServices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormatRenderingProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCommandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceCapabilities_Impl: windows_core::IUnknownImpl {
    fn GetSupportedMethods(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(&self, method: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetMethodParameterAttributes(&self, method: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormats(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(&self, format: *const windows_core::GUID, property: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetEventParameterAttributes(&self, event: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetInheritedServices(&self, dwinheritancetype: u32) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceCapabilities_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedMethods<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethods(this) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethodsByFormat(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppmethods.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetMethodAttributes(this, core::mem::transmute_copy(&method)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetMethodParameterAttributes(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&parameter)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormats(this) {
                    Ok(ok__) => {
                        ppformats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatAttributes(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, property: *const super::super::Foundation::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedEvents(this) {
                    Ok(ok__) => {
                        ppevents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetEventAttributes(this, core::mem::transmute_copy(&event)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, parameter: *const super::super::Foundation::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetEventParameterAttributes(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&parameter)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInheritedServices<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetInheritedServices(this, core::mem::transmute_copy(&dwinheritancetype)) {
                    Ok(ok__) => {
                        ppservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, pprenderingprofiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetFormatRenderingProfiles(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pprenderingprofiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetSupportedCommands(this) {
                    Ok(ok__) => {
                        ppcommands.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::super::Foundation::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceCapabilities_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedMethods: GetSupportedMethods::<Identity, OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Identity, OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Identity, OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventAttributes: GetEventAttributes::<Identity, OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Identity, OFFSET>,
            GetInheritedServices: GetInheritedServices::<Identity, OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Identity, OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceCapabilities {}
windows_core::imp::define_interface!(IPortableDeviceServiceManager, IPortableDeviceServiceManager_Vtbl, 0xa8abc4e9_a84a_47a9_80b3_c5d9b172a961);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceManager, windows_core::IUnknown);
impl IPortableDeviceServiceManager {
    pub unsafe fn GetDeviceServices<P0>(&self, pszpnpdeviceid: P0, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceServices)(windows_core::Interface::as_raw(self), pszpnpdeviceid.param().abi(), guidservicecategory, pservices as _, pcservices as _).ok() }
    }
    pub unsafe fn GetDeviceForService<P0>(&self, pszpnpserviceid: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceForService)(windows_core::Interface::as_raw(self), pszpnpserviceid.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceServices: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceForService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceManager_Impl: windows_core::IUnknownImpl {
    fn GetDeviceServices(&self, pszpnpdeviceid: &windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceForService(&self, pszpnpserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl IPortableDeviceServiceManager_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceServices<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceManager_Impl::GetDeviceServices(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&guidservicecategory), core::mem::transmute_copy(&pservices), core::mem::transmute_copy(&pcservices)).into()
            }
        }
        unsafe extern "system" fn GetDeviceForService<Identity: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceServiceManager_Impl::GetDeviceForService(this, core::mem::transmute(&pszpnpserviceid)) {
                    Ok(ok__) => {
                        ppszpnpdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceServices: GetDeviceServices::<Identity, OFFSET>,
            GetDeviceForService: GetDeviceForService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceManager {}
windows_core::imp::define_interface!(IPortableDeviceServiceMethodCallback, IPortableDeviceServiceMethodCallback_Vtbl, 0xc424233c_afce_4828_a756_7ed7a2350083);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceMethodCallback, windows_core::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    pub unsafe fn OnComplete<P1>(&self, hrstatus: windows_core::HRESULT, presults: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), hrstatus, presults.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceMethodCallback_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT, presults: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceMethodCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, presults: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethodCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&presults)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceMethodCallback {}
windows_core::imp::define_interface!(IPortableDeviceServiceMethods, IPortableDeviceServiceMethods_Vtbl, 0xe20333c9_fd34_412d_a381_cc6f2d820df7);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceMethods, windows_core::IUnknown);
impl IPortableDeviceServiceMethods {
    pub unsafe fn Invoke<P1>(&self, method: *const windows_core::GUID, pparameters: P1, ppresults: *mut Option<IPortableDeviceValues>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), method, pparameters.param().abi(), core::mem::transmute(ppresults)).ok() }
    }
    pub unsafe fn InvokeAsync<P1, P2>(&self, method: *const windows_core::GUID, pparameters: P1, pcallback: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
        P2: windows_core::Param<IPortableDeviceServiceMethodCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeAsync)(windows_core::Interface::as_raw(self), method, pparameters.param().abi(), pcallback.param().abi()).ok() }
    }
    pub unsafe fn Cancel<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceServiceMethodCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethods_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceMethods_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, method: *const windows_core::GUID, pparameters: windows_core::Ref<IPortableDeviceValues>, ppresults: windows_core::OutRef<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn InvokeAsync(&self, method: *const windows_core::GUID, pparameters: windows_core::Ref<IPortableDeviceValues>, pcallback: windows_core::Ref<IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
    fn Cancel(&self, pcallback: windows_core::Ref<IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceMethods_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::Invoke(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&ppresults)).into()
            }
        }
        unsafe extern "system" fn InvokeAsync<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::InvokeAsync(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceMethods_Impl::Cancel(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
            InvokeAsync: InvokeAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceMethods {}
windows_core::imp::define_interface!(IPortableDeviceServiceOpenCallback, IPortableDeviceServiceOpenCallback_Vtbl, 0xbced49c8_8efe_41ed_960b_61313abd47a9);
windows_core::imp::interface_hierarchy!(IPortableDeviceServiceOpenCallback, windows_core::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    pub unsafe fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), hrstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IPortableDeviceServiceOpenCallback_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IPortableDeviceServiceOpenCallback_Vtbl {
    pub const fn new<Identity: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceServiceOpenCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceServiceOpenCallback {}
windows_core::imp::define_interface!(IPortableDeviceUnitsStream, IPortableDeviceUnitsStream_Vtbl, 0x5e98025f_bfc4_47a2_9a5f_bc900a507c67);
windows_core::imp::interface_hierarchy!(IPortableDeviceUnitsStream, windows_core::IUnknown);
impl IPortableDeviceUnitsStream {
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SeekInUnits)(windows_core::Interface::as_raw(self), dlibmove, units, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SeekInUnits: unsafe extern "system" fn(*mut core::ffi::c_void, i64, WPD_STREAM_UNITS, u32, *mut u64) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPortableDeviceUnitsStream_Impl: windows_core::IUnknownImpl {
    fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IPortableDeviceUnitsStream_Vtbl {
    pub const fn new<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SeekInUnits<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceUnitsStream_Impl::SeekInUnits(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&units), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceUnitsStream_Impl::Cancel(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SeekInUnits: SeekInUnits::<Identity, OFFSET>, Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceUnitsStream {}
windows_core::imp::define_interface!(IPortableDeviceValues, IPortableDeviceValues_Vtbl, 0x6848f6f2_3155_4f86_b6f5_263eeeab3143);
windows_core::imp::interface_hierarchy!(IPortableDeviceValues, windows_core::IUnknown);
impl IPortableDeviceValues {
    pub unsafe fn GetCount(&self, pcelt: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelt).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAt(&self, index: u32, pkey: *mut super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, pkey as _, core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStringValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), key, value.param().abi()).ok() }
    }
    pub unsafe fn GetStringValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnsignedIntegerValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnsignedIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignedIntegerValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetSignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignedIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnsignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnsignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFloatValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFloatValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetFloatValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFloatValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetErrorValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetErrorValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetErrorValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeyValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetKeyValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetKeyValue)(windows_core::Interface::as_raw(self), key, pvalue as _).ok() }
    }
    pub unsafe fn SetBoolValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBoolValue)(windows_core::Interface::as_raw(self), key, value.into()).ok() }
    }
    pub unsafe fn GetBoolValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoolValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIUnknownValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIUnknownValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()).ok() }
    }
    pub unsafe fn GetIUnknownValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUnknownValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetGuidValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGuidValue)(windows_core::Interface::as_raw(self), key, value).ok() }
    }
    pub unsafe fn GetGuidValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuidValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBufferValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBufferValue)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pvalue.as_ptr()), pvalue.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetBufferValue(&self, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBufferValue)(windows_core::Interface::as_raw(self), key, ppvalue as _, pcbvalue as _).ok() }
    }
    pub unsafe fn SetIPortableDeviceValuesValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceValuesValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()).ok() }
    }
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDevicePropVariantCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDevicePropVariantCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()).ok() }
    }
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDevicePropVariantCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceKeyCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()).ok() }
    }
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceKeyCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<P1>(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValuesCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceValuesCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()).ok() }
    }
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValuesCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveValue)(windows_core::Interface::as_raw(self), key).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesFromPropertyStore<P0>(&self, pstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyValuesFromPropertyStore)(windows_core::Interface::as_raw(self), pstore.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CopyValuesToPropertyStore<P0>(&self, pstore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyValuesToPropertyStore)(windows_core::Interface::as_raw(self), pstore.param().abi()).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValues_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::PROPERTYKEY, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValue: usize,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetUnsignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, u32) -> windows_core::HRESULT,
    pub GetUnsignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut u32) -> windows_core::HRESULT,
    pub SetSignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, i32) -> windows_core::HRESULT,
    pub GetSignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut i32) -> windows_core::HRESULT,
    pub SetUnsignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, u64) -> windows_core::HRESULT,
    pub GetUnsignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut u64) -> windows_core::HRESULT,
    pub SetSignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, i64) -> windows_core::HRESULT,
    pub GetSignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut i64) -> windows_core::HRESULT,
    pub SetFloatValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, f32) -> windows_core::HRESULT,
    pub GetFloatValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut f32) -> windows_core::HRESULT,
    pub SetErrorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetErrorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub SetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub GetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    pub SetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIUnknownValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIUnknownValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetBufferValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *const u8, u32) -> windows_core::HRESULT,
    pub GetBufferValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetIPortableDeviceValuesValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIPortableDeviceValuesValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesFromPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesFromPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CopyValuesToPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CopyValuesToPropertyStore: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceValues_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelt: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, index: u32, pkey: *mut super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn SetValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetStringValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStringValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUnsignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: u32) -> windows_core::Result<()>;
    fn GetUnsignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<u32>;
    fn SetSignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: i32) -> windows_core::Result<()>;
    fn GetSignedIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<i32>;
    fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: u64) -> windows_core::Result<()>;
    fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<u64>;
    fn SetSignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: i64) -> windows_core::Result<()>;
    fn GetSignedLargeIntegerValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<i64>;
    fn SetFloatValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: f32) -> windows_core::Result<()>;
    fn GetFloatValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<f32>;
    fn SetErrorValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetErrorValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::HRESULT>;
    fn SetKeyValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetKeyValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn SetBoolValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBoolValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL>;
    fn SetIUnknownValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetIUnknownValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::IUnknown>;
    fn SetGuidValue(&self, key: *const super::super::Foundation::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGuidValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<windows_core::GUID>;
    fn SetBufferValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> windows_core::Result<()>;
    fn GetBufferValue(&self, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> windows_core::Result<()>;
    fn SetIPortableDeviceValuesValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(&self, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
    fn CopyValuesFromPropertyStore(&self, pstore: windows_core::Ref<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn CopyValuesToPropertyStore(&self, pstore: windows_core::Ref<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceValues_Vtbl {
    pub const fn new<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetCount(this, core::mem::transmute_copy(&pcelt)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pkey: *mut super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetStringValue(this, core::mem::transmute_copy(&key), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetStringValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetUnsignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetUnsignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetSignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetSignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFloatValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetFloatValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFloatValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetFloatValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetErrorValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetErrorValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetErrorValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetBoolValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetBoolValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBoolValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetBoolValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIUnknownValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIUnknownValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIUnknownValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuidValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetGuidValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetGuidValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cbvalue)).into()
            }
        }
        unsafe extern "system" fn GetBufferValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcbvalue)).into()
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::RemoveValue(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::CopyValuesFromPropertyStore(this, core::mem::transmute_copy(&pstore)).into()
            }
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::CopyValuesToPropertyStore(this, core::mem::transmute_copy(&pstore)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Identity, OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Identity, OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Identity, OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Identity, OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Identity, OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Identity, OFFSET>,
            SetFloatValue: SetFloatValue::<Identity, OFFSET>,
            GetFloatValue: GetFloatValue::<Identity, OFFSET>,
            SetErrorValue: SetErrorValue::<Identity, OFFSET>,
            GetErrorValue: GetErrorValue::<Identity, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
            SetBoolValue: SetBoolValue::<Identity, OFFSET>,
            GetBoolValue: GetBoolValue::<Identity, OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Identity, OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Identity, OFFSET>,
            SetGuidValue: SetGuidValue::<Identity, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, OFFSET>,
            SetBufferValue: SetBufferValue::<Identity, OFFSET>,
            GetBufferValue: GetBufferValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Identity, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            RemoveValue: RemoveValue::<Identity, OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Identity, OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValues as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IPortableDeviceValues {}
windows_core::imp::define_interface!(IPortableDeviceValuesCollection, IPortableDeviceValuesCollection_Vtbl, 0x6e3f2d79_4e07_48c4_8208_d8c2e5af4a99);
windows_core::imp::interface_hierarchy!(IPortableDeviceValuesCollection, windows_core::IUnknown);
impl IPortableDeviceValuesCollection {
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok() }
    }
    pub unsafe fn GetAt(&self, dwindex: u32) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pvalues.param().abi()).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IPortableDeviceValuesCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32) -> windows_core::Result<IPortableDeviceValues>;
    fn Add(&self, pvalues: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl IPortableDeviceValuesCollection_Vtbl {
    pub const fn new<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValuesCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::Add(this, core::mem::transmute_copy(&pvalues)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPortableDeviceValuesCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPortableDeviceWebControl, IPortableDeviceWebControl_Vtbl, 0x94fc7953_5ca1_483a_8aee_df52e7747d00);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPortableDeviceWebControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPortableDeviceWebControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceWebControl {
    pub unsafe fn GetDeviceFromId(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceFromId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceFromIdAsync<P1, P2>(&self, deviceid: &windows_core::BSTR, pcompletionhandler: P1, perrorhandler: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
        P2: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceFromIdAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceid), pcompletionhandler.param().abi(), perrorhandler.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetDeviceFromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceFromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPortableDeviceWebControl_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetDeviceFromId(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetDeviceFromIdAsync(&self, deviceid: &windows_core::BSTR, pcompletionhandler: windows_core::Ref<super::super::System::Com::IDispatch>, perrorhandler: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPortableDeviceWebControl_Vtbl {
    pub const fn new<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceFromId<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceWebControl_Impl::GetDeviceFromId(this, core::mem::transmute(&deviceid)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Identity: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, pcompletionhandler: *mut core::ffi::c_void, perrorhandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceWebControl_Impl::GetDeviceFromIdAsync(this, core::mem::transmute(&deviceid), core::mem::transmute_copy(&pcompletionhandler), core::mem::transmute_copy(&perrorhandler)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceFromId: GetDeviceFromId::<Identity, OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPortableDeviceWebControl {}
windows_core::imp::define_interface!(IRadioInstance, IRadioInstance_Vtbl, 0x70aa1c9e_f2b4_4c61_86d3_6b9fb75fd1a2);
windows_core::imp::interface_hierarchy!(IRadioInstance, windows_core::IUnknown);
impl IRadioInstance {
    pub unsafe fn GetRadioManagerSignature(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRadioManagerSignature)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInstanceSignature(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceSignature)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFriendlyName(&self, lcid: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), lcid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRadioState(&self) -> windows_core::Result<DEVICE_RADIO_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRadioState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRadioState)(windows_core::Interface::as_raw(self), radiostate, utimeoutsec).ok() }
    }
    pub unsafe fn IsMultiComm(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsMultiComm)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsAssociatingDevice(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsAssociatingDevice)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRadioManagerSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetInstanceSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEVICE_RADIO_STATE) -> windows_core::HRESULT,
    pub SetRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, DEVICE_RADIO_STATE, u32) -> windows_core::HRESULT,
    pub IsMultiComm: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsAssociatingDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IRadioInstance_Impl: windows_core::IUnknownImpl {
    fn GetRadioManagerSignature(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetInstanceSignature(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFriendlyName(&self, lcid: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetRadioState(&self) -> windows_core::Result<DEVICE_RADIO_STATE>;
    fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()>;
    fn IsMultiComm(&self) -> windows_core::BOOL;
    fn IsAssociatingDevice(&self) -> windows_core::BOOL;
}
impl IRadioInstance_Vtbl {
    pub const fn new<Identity: IRadioInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRadioManagerSignature<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidsignature: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstance_Impl::GetRadioManagerSignature(this) {
                    Ok(ok__) => {
                        pguidsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstanceSignature<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstance_Impl::GetInstanceSignature(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstance_Impl::GetFriendlyName(this, core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRadioState<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstance_Impl::GetRadioState(this) {
                    Ok(ok__) => {
                        pradiostate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRadioState<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadioInstance_Impl::SetRadioState(this, core::mem::transmute_copy(&radiostate), core::mem::transmute_copy(&utimeoutsec)).into()
            }
        }
        unsafe extern "system" fn IsMultiComm<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadioInstance_Impl::IsMultiComm(this)
            }
        }
        unsafe extern "system" fn IsAssociatingDevice<Identity: IRadioInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadioInstance_Impl::IsAssociatingDevice(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRadioManagerSignature: GetRadioManagerSignature::<Identity, OFFSET>,
            GetInstanceSignature: GetInstanceSignature::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetRadioState: GetRadioState::<Identity, OFFSET>,
            SetRadioState: SetRadioState::<Identity, OFFSET>,
            IsMultiComm: IsMultiComm::<Identity, OFFSET>,
            IsAssociatingDevice: IsAssociatingDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadioInstance as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRadioInstance {}
windows_core::imp::define_interface!(IRadioInstanceCollection, IRadioInstanceCollection_Vtbl, 0xe5791fae_5665_4e0c_95be_5fde31644185);
windows_core::imp::interface_hierarchy!(IRadioInstanceCollection, windows_core::IUnknown);
impl IRadioInstanceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, uindex: u32) -> windows_core::Result<IRadioInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRadioInstanceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, uindex: u32) -> windows_core::Result<IRadioInstance>;
}
impl IRadioInstanceCollection_Vtbl {
    pub const fn new<Identity: IRadioInstanceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinstance: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstanceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppradioinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRadioInstanceCollection_Impl::GetAt(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppradioinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadioInstanceCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRadioInstanceCollection {}
windows_core::imp::define_interface!(IWpdSerializer, IWpdSerializer_Vtbl, 0xb32f4002_bb27_45ff_af4f_06631c1e8dad);
windows_core::imp::interface_hierarchy!(IWpdSerializer, windows_core::IUnknown);
impl IWpdSerializer {
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: &[u8]) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesFromBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<P1>(&self, presults: P1, pbuffer: &mut [u8], pdwbyteswritten: *mut u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteIPortableDeviceValuesToBuffer)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), presults.param().abi(), core::mem::transmute(pbuffer.as_ptr()), pdwbyteswritten as _).ok() }
    }
    pub unsafe fn GetBufferFromIPortableDeviceValues<P0>(&self, psource: P0, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBufferFromIPortableDeviceValues)(windows_core::Interface::as_raw(self), psource.param().abi(), ppbuffer as _, pdwbuffersize as _).ok() }
    }
    pub unsafe fn GetSerializedSize<P0>(&self, psource: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSerializedSize)(windows_core::Interface::as_raw(self), psource.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIPortableDeviceValuesFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteIPortableDeviceValuesToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetBufferFromIPortableDeviceValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetSerializedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWpdSerializer_Impl: windows_core::IUnknownImpl {
    fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const u8, dwinputbufferlength: u32) -> windows_core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(&self, dwoutputbufferlength: u32, presults: windows_core::Ref<IPortableDeviceValues>, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> windows_core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(&self, psource: windows_core::Ref<IPortableDeviceValues>, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetSerializedSize(&self, psource: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<u32>;
}
impl IWpdSerializer_Vtbl {
    pub const fn new<Identity: IWpdSerializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWpdSerializer_Impl::GetIPortableDeviceValuesFromBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwinputbufferlength)) {
                    Ok(ok__) => {
                        ppparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputbufferlength: u32, presults: *mut core::ffi::c_void, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWpdSerializer_Impl::WriteIPortableDeviceValuesToBuffer(this, core::mem::transmute_copy(&dwoutputbufferlength), core::mem::transmute_copy(&presults), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pdwbyteswritten)).into()
            }
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWpdSerializer_Impl::GetBufferFromIPortableDeviceValues(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pdwbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetSerializedSize<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWpdSerializer_Impl::GetSerializedSize(this, core::mem::transmute_copy(&psource)) {
                    Ok(ok__) => {
                        pdwsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Identity, OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Identity, OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Identity, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWpdSerializer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWpdSerializer {}
pub const NAME_3GPP2File: windows_core::PCWSTR = windows_core::w!("3GPP2File");
pub const NAME_3GPPFile: windows_core::PCWSTR = windows_core::w!("3GPPFile");
pub const NAME_AACFile: windows_core::PCWSTR = windows_core::w!("AACFile");
pub const NAME_AIFFFile: windows_core::PCWSTR = windows_core::w!("AIFFFile");
pub const NAME_AMRFile: windows_core::PCWSTR = windows_core::w!("AMRFile");
pub const NAME_ASFFile: windows_core::PCWSTR = windows_core::w!("ASFFile");
pub const NAME_ASXPlaylist: windows_core::PCWSTR = windows_core::w!("ASXPlaylist");
pub const NAME_ATSCTSFile: windows_core::PCWSTR = windows_core::w!("ATSCTSFile");
pub const NAME_AVCHDFile: windows_core::PCWSTR = windows_core::w!("AVCHDFile");
pub const NAME_AVIFile: windows_core::PCWSTR = windows_core::w!("AVIFile");
pub const NAME_AbstractActivity: windows_core::PCWSTR = windows_core::w!("AbstractActivity");
pub const NAME_AbstractActivityOccurrence: windows_core::PCWSTR = windows_core::w!("AbstractActivityOccurrence");
pub const NAME_AbstractAudioAlbum: windows_core::PCWSTR = windows_core::w!("AbstractAudioAlbum");
pub const NAME_AbstractAudioPlaylist: windows_core::PCWSTR = windows_core::w!("AbstractAudioPlaylist");
pub const NAME_AbstractAudioVideoAlbum: windows_core::PCWSTR = windows_core::w!("AbstractAudioVideoAlbum");
pub const NAME_AbstractChapteredProduction: windows_core::PCWSTR = windows_core::w!("AbstractChapteredProduction");
pub const NAME_AbstractContact: windows_core::PCWSTR = windows_core::w!("AbstractContact");
pub const NAME_AbstractContactGroup: windows_core::PCWSTR = windows_core::w!("AbstractContactGroup");
pub const NAME_AbstractDocument: windows_core::PCWSTR = windows_core::w!("AbstractDocument");
pub const NAME_AbstractImageAlbum: windows_core::PCWSTR = windows_core::w!("AbstractImageAlbum");
pub const NAME_AbstractMediacast: windows_core::PCWSTR = windows_core::w!("AbstractMediacast");
pub const NAME_AbstractMessage: windows_core::PCWSTR = windows_core::w!("AbstractMessage");
pub const NAME_AbstractMessageFolder: windows_core::PCWSTR = windows_core::w!("AbstractMessageFolder");
pub const NAME_AbstractMultimediaAlbum: windows_core::PCWSTR = windows_core::w!("AbstractMultimediaAlbum");
pub const NAME_AbstractNote: windows_core::PCWSTR = windows_core::w!("AbstractNote");
pub const NAME_AbstractTask: windows_core::PCWSTR = windows_core::w!("AbstractTask");
pub const NAME_AbstractVideoAlbum: windows_core::PCWSTR = windows_core::w!("AbstractVideoAlbum");
pub const NAME_AbstractVideoPlaylist: windows_core::PCWSTR = windows_core::w!("AbstractVideoPlaylist");
pub const NAME_AnchorResults: windows_core::PCWSTR = windows_core::w!("AnchorResults");
pub const NAME_AnchorResults_Anchor: windows_core::PCWSTR = windows_core::w!("Anchor");
pub const NAME_AnchorResults_AnchorState: windows_core::PCWSTR = windows_core::w!("AnchorState");
pub const NAME_AnchorResults_ResultObjectID: windows_core::PCWSTR = windows_core::w!("ResultObjectID");
pub const NAME_AnchorSyncKnowledge: windows_core::PCWSTR = windows_core::w!("AnchorSyncKnowledge");
pub const NAME_AnchorSyncSvc: windows_core::PCWSTR = windows_core::w!("AnchorSync");
pub const NAME_AnchorSyncSvc_BeginSync: windows_core::PCWSTR = windows_core::w!("BeginSync");
pub const NAME_AnchorSyncSvc_CurrentAnchor: windows_core::PCWSTR = windows_core::w!("AnchorCurrentAnchor");
pub const NAME_AnchorSyncSvc_EndSync: windows_core::PCWSTR = windows_core::w!("EndSync");
pub const NAME_AnchorSyncSvc_FilterType: windows_core::PCWSTR = windows_core::w!("FilterType");
pub const NAME_AnchorSyncSvc_GetChangesSinceAnchor: windows_core::PCWSTR = windows_core::w!("GetChangesSinceAnchor");
pub const NAME_AnchorSyncSvc_KnowledgeObjectID: windows_core::PCWSTR = windows_core::w!("AnchorKnowledgeObjectID");
pub const NAME_AnchorSyncSvc_LastSyncProxyID: windows_core::PCWSTR = windows_core::w!("AnchorLastSyncProxyID");
pub const NAME_AnchorSyncSvc_LocalOnlyDelete: windows_core::PCWSTR = windows_core::w!("LocalOnlyDelete");
pub const NAME_AnchorSyncSvc_ProviderVersion: windows_core::PCWSTR = windows_core::w!("AnchorProviderVersion");
pub const NAME_AnchorSyncSvc_ReplicaID: windows_core::PCWSTR = windows_core::w!("AnchorReplicaID");
pub const NAME_AnchorSyncSvc_SyncFormat: windows_core::PCWSTR = windows_core::w!("SyncFormat");
pub const NAME_AnchorSyncSvc_VersionProps: windows_core::PCWSTR = windows_core::w!("AnchorVersionProps");
pub const NAME_Association: windows_core::PCWSTR = windows_core::w!("Association");
pub const NAME_AudibleFile: windows_core::PCWSTR = windows_core::w!("AudibleFile");
pub const NAME_AudioObj_AudioBitDepth: windows_core::PCWSTR = windows_core::w!("AudioBitDepth");
pub const NAME_AudioObj_AudioBitRate: windows_core::PCWSTR = windows_core::w!("AudioBitRate");
pub const NAME_AudioObj_AudioBlockAlignment: windows_core::PCWSTR = windows_core::w!("AudioBlockAlignment");
pub const NAME_AudioObj_AudioFormatCode: windows_core::PCWSTR = windows_core::w!("AudioFormatCode");
pub const NAME_AudioObj_Channels: windows_core::PCWSTR = windows_core::w!("Channels");
pub const NAME_AudioObj_Lyrics: windows_core::PCWSTR = windows_core::w!("Lyrics");
pub const NAME_BMPImage: windows_core::PCWSTR = windows_core::w!("BMPImage");
pub const NAME_CIFFImage: windows_core::PCWSTR = windows_core::w!("CIFFImage");
pub const NAME_CalendarObj_Accepted: windows_core::PCWSTR = windows_core::w!("Accepted");
pub const NAME_CalendarObj_BeginDateTime: windows_core::PCWSTR = windows_core::w!("BeginDateTime");
pub const NAME_CalendarObj_BusyStatus: windows_core::PCWSTR = windows_core::w!("BusyStatus");
pub const NAME_CalendarObj_Declined: windows_core::PCWSTR = windows_core::w!("Declined");
pub const NAME_CalendarObj_EndDateTime: windows_core::PCWSTR = windows_core::w!("EndDateTime");
pub const NAME_CalendarObj_Location: windows_core::PCWSTR = windows_core::w!("Location");
pub const NAME_CalendarObj_PatternDuration: windows_core::PCWSTR = windows_core::w!("PatternDuration");
pub const NAME_CalendarObj_PatternStartTime: windows_core::PCWSTR = windows_core::w!("PatternStartTime");
pub const NAME_CalendarObj_ReminderOffset: windows_core::PCWSTR = windows_core::w!("ReminderOffset");
pub const NAME_CalendarObj_Tentative: windows_core::PCWSTR = windows_core::w!("Tentative");
pub const NAME_CalendarObj_TimeZone: windows_core::PCWSTR = windows_core::w!("TimeZone");
pub const NAME_CalendarSvc: windows_core::PCWSTR = windows_core::w!("Calendar");
pub const NAME_CalendarSvc_SyncWindowEnd: windows_core::PCWSTR = windows_core::w!("SyncWindowEnd");
pub const NAME_CalendarSvc_SyncWindowStart: windows_core::PCWSTR = windows_core::w!("SyncWindowStart");
pub const NAME_ContactObj_AnniversaryDate: windows_core::PCWSTR = windows_core::w!("AnniversaryDate");
pub const NAME_ContactObj_Assistant: windows_core::PCWSTR = windows_core::w!("Assistant");
pub const NAME_ContactObj_Birthdate: windows_core::PCWSTR = windows_core::w!("Birthdate");
pub const NAME_ContactObj_BusinessAddressCity: windows_core::PCWSTR = windows_core::w!("BusinessAddressCity");
pub const NAME_ContactObj_BusinessAddressCountry: windows_core::PCWSTR = windows_core::w!("BusinessAddressCountry");
pub const NAME_ContactObj_BusinessAddressFull: windows_core::PCWSTR = windows_core::w!("BusinessAddressFull");
pub const NAME_ContactObj_BusinessAddressLine2: windows_core::PCWSTR = windows_core::w!("BusinessAddressLine2");
pub const NAME_ContactObj_BusinessAddressPostalCode: windows_core::PCWSTR = windows_core::w!("BusinessAddressPostalCode");
pub const NAME_ContactObj_BusinessAddressRegion: windows_core::PCWSTR = windows_core::w!("BusinessAddressRegion");
pub const NAME_ContactObj_BusinessAddressStreet: windows_core::PCWSTR = windows_core::w!("BusinessAddressStreet");
pub const NAME_ContactObj_BusinessEmail: windows_core::PCWSTR = windows_core::w!("BusinessEmail");
pub const NAME_ContactObj_BusinessEmail2: windows_core::PCWSTR = windows_core::w!("BusinessEmail2");
pub const NAME_ContactObj_BusinessFax: windows_core::PCWSTR = windows_core::w!("BusinessFax");
pub const NAME_ContactObj_BusinessPhone: windows_core::PCWSTR = windows_core::w!("BusinessPhone");
pub const NAME_ContactObj_BusinessPhone2: windows_core::PCWSTR = windows_core::w!("BusinessPhone2");
pub const NAME_ContactObj_BusinessWebAddress: windows_core::PCWSTR = windows_core::w!("BusinessWebAddress");
pub const NAME_ContactObj_Children: windows_core::PCWSTR = windows_core::w!("Children");
pub const NAME_ContactObj_Email: windows_core::PCWSTR = windows_core::w!("Email");
pub const NAME_ContactObj_FamilyName: windows_core::PCWSTR = windows_core::w!("FamilyName");
pub const NAME_ContactObj_Fax: windows_core::PCWSTR = windows_core::w!("Fax");
pub const NAME_ContactObj_GivenName: windows_core::PCWSTR = windows_core::w!("GivenName");
pub const NAME_ContactObj_IMAddress: windows_core::PCWSTR = windows_core::w!("IMAddress");
pub const NAME_ContactObj_IMAddress2: windows_core::PCWSTR = windows_core::w!("IMAddress2");
pub const NAME_ContactObj_IMAddress3: windows_core::PCWSTR = windows_core::w!("IMAddress3");
pub const NAME_ContactObj_MiddleNames: windows_core::PCWSTR = windows_core::w!("MiddleNames");
pub const NAME_ContactObj_MobilePhone: windows_core::PCWSTR = windows_core::w!("MobilePhone");
pub const NAME_ContactObj_MobilePhone2: windows_core::PCWSTR = windows_core::w!("MobilePhone2");
pub const NAME_ContactObj_Organization: windows_core::PCWSTR = windows_core::w!("Organization");
pub const NAME_ContactObj_OtherAddressCity: windows_core::PCWSTR = windows_core::w!("OtherAddressCity");
pub const NAME_ContactObj_OtherAddressCountry: windows_core::PCWSTR = windows_core::w!("OtherAddressCountry");
pub const NAME_ContactObj_OtherAddressFull: windows_core::PCWSTR = windows_core::w!("OtherAddressFull");
pub const NAME_ContactObj_OtherAddressLine2: windows_core::PCWSTR = windows_core::w!("OtherAddressLine2");
pub const NAME_ContactObj_OtherAddressPostalCode: windows_core::PCWSTR = windows_core::w!("OtherAddressPostalCode");
pub const NAME_ContactObj_OtherAddressRegion: windows_core::PCWSTR = windows_core::w!("OtherAddressRegion");
pub const NAME_ContactObj_OtherAddressStreet: windows_core::PCWSTR = windows_core::w!("OtherAddressStreet");
pub const NAME_ContactObj_OtherEmail: windows_core::PCWSTR = windows_core::w!("OtherEmail");
pub const NAME_ContactObj_OtherPhone: windows_core::PCWSTR = windows_core::w!("OtherPhone");
pub const NAME_ContactObj_Pager: windows_core::PCWSTR = windows_core::w!("Pager");
pub const NAME_ContactObj_PersonalAddressCity: windows_core::PCWSTR = windows_core::w!("PersonalAddressCity");
pub const NAME_ContactObj_PersonalAddressCountry: windows_core::PCWSTR = windows_core::w!("PersonalAddressCountry");
pub const NAME_ContactObj_PersonalAddressFull: windows_core::PCWSTR = windows_core::w!("PersonalAddressFull");
pub const NAME_ContactObj_PersonalAddressLine2: windows_core::PCWSTR = windows_core::w!("PersonalAddressLine2");
pub const NAME_ContactObj_PersonalAddressPostalCode: windows_core::PCWSTR = windows_core::w!("PersonalAddressPostalCode");
pub const NAME_ContactObj_PersonalAddressRegion: windows_core::PCWSTR = windows_core::w!("PersonalAddressRegion");
pub const NAME_ContactObj_PersonalAddressStreet: windows_core::PCWSTR = windows_core::w!("PersonalAddressStreet");
pub const NAME_ContactObj_PersonalEmail: windows_core::PCWSTR = windows_core::w!("PersonalEmail");
pub const NAME_ContactObj_PersonalEmail2: windows_core::PCWSTR = windows_core::w!("PersonalEmail2");
pub const NAME_ContactObj_PersonalFax: windows_core::PCWSTR = windows_core::w!("PersonalFax");
pub const NAME_ContactObj_PersonalPhone: windows_core::PCWSTR = windows_core::w!("PersonalPhone");
pub const NAME_ContactObj_PersonalPhone2: windows_core::PCWSTR = windows_core::w!("PersonalPhone2");
pub const NAME_ContactObj_PersonalWebAddress: windows_core::PCWSTR = windows_core::w!("PersonalWebAddress");
pub const NAME_ContactObj_Phone: windows_core::PCWSTR = windows_core::w!("Phone");
pub const NAME_ContactObj_PhoneticFamilyName: windows_core::PCWSTR = windows_core::w!("PhoneticFamilyName");
pub const NAME_ContactObj_PhoneticGivenName: windows_core::PCWSTR = windows_core::w!("PhoneticGivenName");
pub const NAME_ContactObj_PhoneticOrganization: windows_core::PCWSTR = windows_core::w!("PhoneticOrganization");
pub const NAME_ContactObj_Ringtone: windows_core::PCWSTR = windows_core::w!("Ringtone");
pub const NAME_ContactObj_Role: windows_core::PCWSTR = windows_core::w!("Role");
pub const NAME_ContactObj_Spouse: windows_core::PCWSTR = windows_core::w!("Spouse");
pub const NAME_ContactObj_Suffix: windows_core::PCWSTR = windows_core::w!("Suffix");
pub const NAME_ContactObj_Title: windows_core::PCWSTR = windows_core::w!("Title");
pub const NAME_ContactObj_WebAddress: windows_core::PCWSTR = windows_core::w!("WebAddress");
pub const NAME_ContactSvc_SyncWithPhoneOnly: windows_core::PCWSTR = windows_core::w!("FilterType");
pub const NAME_ContactsSvc: windows_core::PCWSTR = windows_core::w!("Contacts");
pub const NAME_DPOFDocument: windows_core::PCWSTR = windows_core::w!("DPOFDocument");
pub const NAME_DVBTSFile: windows_core::PCWSTR = windows_core::w!("DVBTSFile");
pub const NAME_DeviceExecutable: windows_core::PCWSTR = windows_core::w!("DeviceExecutable");
pub const NAME_DeviceMetadataCAB: windows_core::PCWSTR = windows_core::w!("DeviceMetadataCAB");
pub const NAME_DeviceMetadataObj_ContentID: windows_core::PCWSTR = windows_core::w!("ContentID");
pub const NAME_DeviceMetadataObj_DefaultCAB: windows_core::PCWSTR = windows_core::w!("DefaultCAB");
pub const NAME_DeviceMetadataSvc: windows_core::PCWSTR = windows_core::w!("Metadata");
pub const NAME_DeviceScript: windows_core::PCWSTR = windows_core::w!("DeviceScript");
pub const NAME_EXIFImage: windows_core::PCWSTR = windows_core::w!("EXIFImage");
pub const NAME_ExcelDocument: windows_core::PCWSTR = windows_core::w!("ExcelDocument");
pub const NAME_FLACFile: windows_core::PCWSTR = windows_core::w!("FLACFile");
pub const NAME_FirmwareFile: windows_core::PCWSTR = windows_core::w!("FirmwareFile");
pub const NAME_FlashPixImage: windows_core::PCWSTR = windows_core::w!("FlashPixImage");
pub const NAME_FullEnumSyncKnowledge: windows_core::PCWSTR = windows_core::w!("FullEnumSyncKnowledge");
pub const NAME_FullEnumSyncSvc: windows_core::PCWSTR = windows_core::w!("FullEnumSync");
pub const NAME_FullEnumSyncSvc_BeginSync: windows_core::PCWSTR = windows_core::w!("BeginSync");
pub const NAME_FullEnumSyncSvc_EndSync: windows_core::PCWSTR = windows_core::w!("EndSync");
pub const NAME_FullEnumSyncSvc_FilterType: windows_core::PCWSTR = windows_core::w!("FilterType");
pub const NAME_FullEnumSyncSvc_KnowledgeObjectID: windows_core::PCWSTR = windows_core::w!("FullEnumKnowledgeObjectID");
pub const NAME_FullEnumSyncSvc_LastSyncProxyID: windows_core::PCWSTR = windows_core::w!("FullEnumLastSyncProxyID");
pub const NAME_FullEnumSyncSvc_LocalOnlyDelete: windows_core::PCWSTR = windows_core::w!("LocalOnlyDelete");
pub const NAME_FullEnumSyncSvc_ProviderVersion: windows_core::PCWSTR = windows_core::w!("FullEnumProviderVersion");
pub const NAME_FullEnumSyncSvc_ReplicaID: windows_core::PCWSTR = windows_core::w!("FullEnumReplicaID");
pub const NAME_FullEnumSyncSvc_SyncFormat: windows_core::PCWSTR = windows_core::w!("SyncFormat");
pub const NAME_FullEnumSyncSvc_VersionProps: windows_core::PCWSTR = windows_core::w!("FullEnumVersionProps");
pub const NAME_GIFImage: windows_core::PCWSTR = windows_core::w!("GIFImage");
pub const NAME_GenericObj_AllowedFolderContents: windows_core::PCWSTR = windows_core::w!("AllowedFolderContents");
pub const NAME_GenericObj_AssociationDesc: windows_core::PCWSTR = windows_core::w!("AssociationDesc");
pub const NAME_GenericObj_AssociationType: windows_core::PCWSTR = windows_core::w!("AssociationType");
pub const NAME_GenericObj_Copyright: windows_core::PCWSTR = windows_core::w!("Copyright");
pub const NAME_GenericObj_Corrupt: windows_core::PCWSTR = windows_core::w!("Corrupt");
pub const NAME_GenericObj_DRMStatus: windows_core::PCWSTR = windows_core::w!("DRMStatus");
pub const NAME_GenericObj_DateAccessed: windows_core::PCWSTR = windows_core::w!("DateAccessed");
pub const NAME_GenericObj_DateAdded: windows_core::PCWSTR = windows_core::w!("DateAdded");
pub const NAME_GenericObj_DateAuthored: windows_core::PCWSTR = windows_core::w!("DateAuthored");
pub const NAME_GenericObj_DateCreated: windows_core::PCWSTR = windows_core::w!("DateCreated");
pub const NAME_GenericObj_DateModified: windows_core::PCWSTR = windows_core::w!("DateModified");
pub const NAME_GenericObj_DateRevised: windows_core::PCWSTR = windows_core::w!("DateRevised");
pub const NAME_GenericObj_Description: windows_core::PCWSTR = windows_core::w!("Description");
pub const NAME_GenericObj_Hidden: windows_core::PCWSTR = windows_core::w!("Hidden");
pub const NAME_GenericObj_Keywords: windows_core::PCWSTR = windows_core::w!("Keywords");
pub const NAME_GenericObj_LanguageLocale: windows_core::PCWSTR = windows_core::w!("LanguageLocale");
pub const NAME_GenericObj_Name: windows_core::PCWSTR = windows_core::w!("Name");
pub const NAME_GenericObj_NonConsumable: windows_core::PCWSTR = windows_core::w!("NonConsumable");
pub const NAME_GenericObj_ObjectFileName: windows_core::PCWSTR = windows_core::w!("ObjectFileName");
pub const NAME_GenericObj_ObjectFormat: windows_core::PCWSTR = windows_core::w!("ObjectFormat");
pub const NAME_GenericObj_ObjectID: windows_core::PCWSTR = windows_core::w!("ObjectID");
pub const NAME_GenericObj_ObjectSize: windows_core::PCWSTR = windows_core::w!("ObjectSize");
pub const NAME_GenericObj_ParentID: windows_core::PCWSTR = windows_core::w!("ParentID");
pub const NAME_GenericObj_PersistentUID: windows_core::PCWSTR = windows_core::w!("PersistentUID");
pub const NAME_GenericObj_PropertyBag: windows_core::PCWSTR = windows_core::w!("PropertyBag");
pub const NAME_GenericObj_ProtectionStatus: windows_core::PCWSTR = windows_core::w!("ProtectionStatus");
pub const NAME_GenericObj_ReferenceParentID: windows_core::PCWSTR = windows_core::w!("ReferenceParentID");
pub const NAME_GenericObj_StorageID: windows_core::PCWSTR = windows_core::w!("StorageID");
pub const NAME_GenericObj_SubDescription: windows_core::PCWSTR = windows_core::w!("SubDescription");
pub const NAME_GenericObj_SyncID: windows_core::PCWSTR = windows_core::w!("SyncID");
pub const NAME_GenericObj_SystemObject: windows_core::PCWSTR = windows_core::w!("SystemObject");
pub const NAME_GenericObj_TimeToLive: windows_core::PCWSTR = windows_core::w!("TimeToLive");
pub const NAME_HDPhotoImage: windows_core::PCWSTR = windows_core::w!("HDPhotoImage");
pub const NAME_HTMLDocument: windows_core::PCWSTR = windows_core::w!("HTMLDocument");
pub const NAME_HintsSvc: windows_core::PCWSTR = windows_core::w!("Hints");
pub const NAME_ICalendarActivity: windows_core::PCWSTR = windows_core::w!("ICalendar");
pub const NAME_ImageObj_Aperature: windows_core::PCWSTR = windows_core::w!("Aperature");
pub const NAME_ImageObj_Exposure: windows_core::PCWSTR = windows_core::w!("Exposure");
pub const NAME_ImageObj_ISOSpeed: windows_core::PCWSTR = windows_core::w!("ISOSpeed");
pub const NAME_ImageObj_ImageBitDepth: windows_core::PCWSTR = windows_core::w!("ImageBitDepth");
pub const NAME_ImageObj_IsColorCorrected: windows_core::PCWSTR = windows_core::w!("IsColorCorrected");
pub const NAME_ImageObj_IsCropped: windows_core::PCWSTR = windows_core::w!("IsCropped");
pub const NAME_JFIFImage: windows_core::PCWSTR = windows_core::w!("JFIFImage");
pub const NAME_JP2Image: windows_core::PCWSTR = windows_core::w!("JP2Image");
pub const NAME_JPEGXRImage: windows_core::PCWSTR = windows_core::w!("JPEGXRImage");
pub const NAME_JPXImage: windows_core::PCWSTR = windows_core::w!("JPXImage");
pub const NAME_M3UPlaylist: windows_core::PCWSTR = windows_core::w!("M3UPlaylist");
pub const NAME_MHTDocument: windows_core::PCWSTR = windows_core::w!("MHTDocument");
pub const NAME_MP3File: windows_core::PCWSTR = windows_core::w!("MP3File");
pub const NAME_MPEG2File: windows_core::PCWSTR = windows_core::w!("MPEG2File");
pub const NAME_MPEG4File: windows_core::PCWSTR = windows_core::w!("MPEG4File");
pub const NAME_MPEGFile: windows_core::PCWSTR = windows_core::w!("MPEGFile");
pub const NAME_MPLPlaylist: windows_core::PCWSTR = windows_core::w!("MPLPlaylist");
pub const NAME_MediaObj_AlbumArtist: windows_core::PCWSTR = windows_core::w!("AlbumArtist");
pub const NAME_MediaObj_AlbumName: windows_core::PCWSTR = windows_core::w!("AlbumName");
pub const NAME_MediaObj_Artist: windows_core::PCWSTR = windows_core::w!("Artist");
pub const NAME_MediaObj_AudioEncodingProfile: windows_core::PCWSTR = windows_core::w!("AudioEncodingProfile");
pub const NAME_MediaObj_BitRateType: windows_core::PCWSTR = windows_core::w!("BitRateType");
pub const NAME_MediaObj_BookmarkByte: windows_core::PCWSTR = windows_core::w!("BookmarkByte");
pub const NAME_MediaObj_BookmarkObject: windows_core::PCWSTR = windows_core::w!("BookmarkObject");
pub const NAME_MediaObj_BookmarkTime: windows_core::PCWSTR = windows_core::w!("BookmarkTime");
pub const NAME_MediaObj_BufferSize: windows_core::PCWSTR = windows_core::w!("BufferSize");
pub const NAME_MediaObj_Composer: windows_core::PCWSTR = windows_core::w!("Composer");
pub const NAME_MediaObj_Credits: windows_core::PCWSTR = windows_core::w!("Credits");
pub const NAME_MediaObj_DateOriginalRelease: windows_core::PCWSTR = windows_core::w!("DateOriginalRelease");
pub const NAME_MediaObj_Duration: windows_core::PCWSTR = windows_core::w!("Duration");
pub const NAME_MediaObj_Editor: windows_core::PCWSTR = windows_core::w!("Editor");
pub const NAME_MediaObj_EffectiveRating: windows_core::PCWSTR = windows_core::w!("EffectiveRating");
pub const NAME_MediaObj_EncodingProfile: windows_core::PCWSTR = windows_core::w!("EncodingProfile");
pub const NAME_MediaObj_EncodingQuality: windows_core::PCWSTR = windows_core::w!("EncodingQuality");
pub const NAME_MediaObj_Genre: windows_core::PCWSTR = windows_core::w!("Genre");
pub const NAME_MediaObj_GeographicOrigin: windows_core::PCWSTR = windows_core::w!("GeographicOrigin");
pub const NAME_MediaObj_Height: windows_core::PCWSTR = windows_core::w!("Height");
pub const NAME_MediaObj_MediaType: windows_core::PCWSTR = windows_core::w!("MediaType");
pub const NAME_MediaObj_MediaUID: windows_core::PCWSTR = windows_core::w!("MediaUID");
pub const NAME_MediaObj_Mood: windows_core::PCWSTR = windows_core::w!("Mood");
pub const NAME_MediaObj_Owner: windows_core::PCWSTR = windows_core::w!("Owner");
pub const NAME_MediaObj_ParentalRating: windows_core::PCWSTR = windows_core::w!("ParentalRating");
pub const NAME_MediaObj_Producer: windows_core::PCWSTR = windows_core::w!("Producer");
pub const NAME_MediaObj_SampleRate: windows_core::PCWSTR = windows_core::w!("SampleRate");
pub const NAME_MediaObj_SkipCount: windows_core::PCWSTR = windows_core::w!("SkipCount");
pub const NAME_MediaObj_SubscriptionContentID: windows_core::PCWSTR = windows_core::w!("SubscriptionContentID");
pub const NAME_MediaObj_Subtitle: windows_core::PCWSTR = windows_core::w!("Subtitle");
pub const NAME_MediaObj_TotalBitRate: windows_core::PCWSTR = windows_core::w!("TotalBitRate");
pub const NAME_MediaObj_Track: windows_core::PCWSTR = windows_core::w!("Track");
pub const NAME_MediaObj_URLLink: windows_core::PCWSTR = windows_core::w!("URLLink");
pub const NAME_MediaObj_URLSource: windows_core::PCWSTR = windows_core::w!("URLSource");
pub const NAME_MediaObj_UseCount: windows_core::PCWSTR = windows_core::w!("UseCount");
pub const NAME_MediaObj_UserRating: windows_core::PCWSTR = windows_core::w!("UserRating");
pub const NAME_MediaObj_WebMaster: windows_core::PCWSTR = windows_core::w!("WebMaster");
pub const NAME_MediaObj_Width: windows_core::PCWSTR = windows_core::w!("Width");
pub const NAME_MessageObj_BCC: windows_core::PCWSTR = windows_core::w!("BCC");
pub const NAME_MessageObj_Body: windows_core::PCWSTR = windows_core::w!("Body");
pub const NAME_MessageObj_CC: windows_core::PCWSTR = windows_core::w!("CC");
pub const NAME_MessageObj_Category: windows_core::PCWSTR = windows_core::w!("Category");
pub const NAME_MessageObj_PatternDayOfMonth: windows_core::PCWSTR = windows_core::w!("PatternDayOfMonth");
pub const NAME_MessageObj_PatternDayOfWeek: windows_core::PCWSTR = windows_core::w!("PatternDayOfWeek");
pub const NAME_MessageObj_PatternDeleteDates: windows_core::PCWSTR = windows_core::w!("PatternDeleteDates");
pub const NAME_MessageObj_PatternInstance: windows_core::PCWSTR = windows_core::w!("PatternInstance");
pub const NAME_MessageObj_PatternMonthOfYear: windows_core::PCWSTR = windows_core::w!("PatternMonthOfYear");
pub const NAME_MessageObj_PatternOriginalDateTime: windows_core::PCWSTR = windows_core::w!("PatternOriginalDateTime");
pub const NAME_MessageObj_PatternPeriod: windows_core::PCWSTR = windows_core::w!("PatternPeriod");
pub const NAME_MessageObj_PatternType: windows_core::PCWSTR = windows_core::w!("PatternType");
pub const NAME_MessageObj_PatternValidEndDate: windows_core::PCWSTR = windows_core::w!("PatternValidEndDate");
pub const NAME_MessageObj_PatternValidStartDate: windows_core::PCWSTR = windows_core::w!("PatternValidStartDate");
pub const NAME_MessageObj_Priority: windows_core::PCWSTR = windows_core::w!("Priority");
pub const NAME_MessageObj_Read: windows_core::PCWSTR = windows_core::w!("Read");
pub const NAME_MessageObj_ReceivedTime: windows_core::PCWSTR = windows_core::w!("ReceivedTime");
pub const NAME_MessageObj_Sender: windows_core::PCWSTR = windows_core::w!("Sender");
pub const NAME_MessageObj_Subject: windows_core::PCWSTR = windows_core::w!("Subject");
pub const NAME_MessageObj_To: windows_core::PCWSTR = windows_core::w!("To");
pub const NAME_MessageSvc: windows_core::PCWSTR = windows_core::w!("Message");
pub const NAME_NotesSvc: windows_core::PCWSTR = windows_core::w!("Notes");
pub const NAME_OGGFile: windows_core::PCWSTR = windows_core::w!("OGGFile");
pub const NAME_PCDImage: windows_core::PCWSTR = windows_core::w!("PCDImage");
pub const NAME_PICTImage: windows_core::PCWSTR = windows_core::w!("PICTImage");
pub const NAME_PNGImage: windows_core::PCWSTR = windows_core::w!("PNGImage");
pub const NAME_PSLPlaylist: windows_core::PCWSTR = windows_core::w!("PSLPlaylist");
pub const NAME_PowerPointDocument: windows_core::PCWSTR = windows_core::w!("PowerPointDocument");
pub const NAME_QCELPFile: windows_core::PCWSTR = windows_core::w!("QCELPFile");
pub const NAME_RingtonesSvc: windows_core::PCWSTR = windows_core::w!("Ringtones");
pub const NAME_RingtonesSvc_DefaultRingtone: windows_core::PCWSTR = windows_core::w!("DefaultRingtone");
pub const NAME_Services_ServiceDisplayName: windows_core::PCWSTR = windows_core::w!("ServiceDisplayName");
pub const NAME_Services_ServiceIcon: windows_core::PCWSTR = windows_core::w!("ServiceIcon");
pub const NAME_Services_ServiceLocale: windows_core::PCWSTR = windows_core::w!("ServiceLocale");
pub const NAME_StatusSvc: windows_core::PCWSTR = windows_core::w!("Status");
pub const NAME_StatusSvc_BatteryLife: windows_core::PCWSTR = windows_core::w!("BatteryLife");
pub const NAME_StatusSvc_ChargingState: windows_core::PCWSTR = windows_core::w!("ChargingState");
pub const NAME_StatusSvc_MissedCalls: windows_core::PCWSTR = windows_core::w!("MissedCalls");
pub const NAME_StatusSvc_NetworkName: windows_core::PCWSTR = windows_core::w!("NetworkName");
pub const NAME_StatusSvc_NetworkType: windows_core::PCWSTR = windows_core::w!("NetworkType");
pub const NAME_StatusSvc_NewPictures: windows_core::PCWSTR = windows_core::w!("NewPictures");
pub const NAME_StatusSvc_Roaming: windows_core::PCWSTR = windows_core::w!("Roaming");
pub const NAME_StatusSvc_SignalStrength: windows_core::PCWSTR = windows_core::w!("SignalStrength");
pub const NAME_StatusSvc_StorageCapacity: windows_core::PCWSTR = windows_core::w!("StorageCapacity");
pub const NAME_StatusSvc_StorageFreeSpace: windows_core::PCWSTR = windows_core::w!("StorageFreeSpace");
pub const NAME_StatusSvc_TextMessages: windows_core::PCWSTR = windows_core::w!("TextMessages");
pub const NAME_StatusSvc_VoiceMail: windows_core::PCWSTR = windows_core::w!("VoiceMail");
pub const NAME_SyncObj_LastAuthorProxyID: windows_core::PCWSTR = windows_core::w!("LastAuthorProxyID");
pub const NAME_SyncSvc_BeginSync: windows_core::PCWSTR = windows_core::w!("BeginSync");
pub const NAME_SyncSvc_EndSync: windows_core::PCWSTR = windows_core::w!("EndSync");
pub const NAME_SyncSvc_FilterType: windows_core::PCWSTR = windows_core::w!("FilterType");
pub const NAME_SyncSvc_LocalOnlyDelete: windows_core::PCWSTR = windows_core::w!("LocalOnlyDelete");
pub const NAME_SyncSvc_SyncFormat: windows_core::PCWSTR = windows_core::w!("SyncFormat");
pub const NAME_SyncSvc_SyncObjectReferences: windows_core::PCWSTR = windows_core::w!("SyncObjectReferences");
pub const NAME_TIFFEPImage: windows_core::PCWSTR = windows_core::w!("TIFFEPImage");
pub const NAME_TIFFITImage: windows_core::PCWSTR = windows_core::w!("TIFFITImage");
pub const NAME_TIFFImage: windows_core::PCWSTR = windows_core::w!("TIFFImage");
pub const NAME_TaskObj_BeginDate: windows_core::PCWSTR = windows_core::w!("BeginDate");
pub const NAME_TaskObj_Complete: windows_core::PCWSTR = windows_core::w!("Complete");
pub const NAME_TaskObj_EndDate: windows_core::PCWSTR = windows_core::w!("EndDate");
pub const NAME_TaskObj_ReminderDateTime: windows_core::PCWSTR = windows_core::w!("ReminderDateTime");
pub const NAME_TasksSvc: windows_core::PCWSTR = windows_core::w!("Tasks");
pub const NAME_TasksSvc_SyncActiveOnly: windows_core::PCWSTR = windows_core::w!("FilterType");
pub const NAME_TextDocument: windows_core::PCWSTR = windows_core::w!("TextDocument");
pub const NAME_Undefined: windows_core::PCWSTR = windows_core::w!("Undefined");
pub const NAME_UndefinedAudio: windows_core::PCWSTR = windows_core::w!("UndefinedAudio");
pub const NAME_UndefinedCollection: windows_core::PCWSTR = windows_core::w!("UndefinedCollection");
pub const NAME_UndefinedDocument: windows_core::PCWSTR = windows_core::w!("UndefinedDocument");
pub const NAME_UndefinedVideo: windows_core::PCWSTR = windows_core::w!("UndefinedVideo");
pub const NAME_UnknownImage: windows_core::PCWSTR = windows_core::w!("UnknownImage");
pub const NAME_VCalendar1Activity: windows_core::PCWSTR = windows_core::w!("VCalendar1");
pub const NAME_VCard2Contact: windows_core::PCWSTR = windows_core::w!("VCard2Contact");
pub const NAME_VCard3Contact: windows_core::PCWSTR = windows_core::w!("VCard3Contact");
pub const NAME_VideoObj_KeyFrameDistance: windows_core::PCWSTR = windows_core::w!("KeyFrameDistance");
pub const NAME_VideoObj_ScanType: windows_core::PCWSTR = windows_core::w!("ScanType");
pub const NAME_VideoObj_Source: windows_core::PCWSTR = windows_core::w!("Source");
pub const NAME_VideoObj_VideoBitRate: windows_core::PCWSTR = windows_core::w!("VideoBitRate");
pub const NAME_VideoObj_VideoFormatCode: windows_core::PCWSTR = windows_core::w!("VideoFormatCode");
pub const NAME_VideoObj_VideoFrameRate: windows_core::PCWSTR = windows_core::w!("VideoFrameRate");
pub const NAME_WAVFile: windows_core::PCWSTR = windows_core::w!("WAVFile");
pub const NAME_WBMPImage: windows_core::PCWSTR = windows_core::w!("WBMPImage");
pub const NAME_WMAFile: windows_core::PCWSTR = windows_core::w!("WMAFile");
pub const NAME_WMVFile: windows_core::PCWSTR = windows_core::w!("WMVFile");
pub const NAME_WPLPlaylist: windows_core::PCWSTR = windows_core::w!("WPLPlaylist");
pub const NAME_WordDocument: windows_core::PCWSTR = windows_core::w!("WordDocument");
pub const NAME_XMLDocument: windows_core::PCWSTR = windows_core::w!("XMLDocument");
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(0i32);
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(1i32);
pub const PORTABLE_DEVICE_DRM_SCHEME_PDDRM: windows_core::PCWSTR = windows_core::w!("PDDRM");
pub const PORTABLE_DEVICE_DRM_SCHEME_WMDRM10_PD: windows_core::PCWSTR = windows_core::w!("WMDRM10-PD");
pub const PORTABLE_DEVICE_ICON: windows_core::PCWSTR = windows_core::w!("Icons");
pub const PORTABLE_DEVICE_IS_MASS_STORAGE: windows_core::PCWSTR = windows_core::w!("PortableDeviceIsMassStorage");
pub const PORTABLE_DEVICE_NAMESPACE_EXCLUDE_FROM_SHELL: windows_core::PCWSTR = windows_core::w!("PortableDeviceNameSpaceExcludeFromShell");
pub const PORTABLE_DEVICE_NAMESPACE_THUMBNAIL_CONTENT_TYPES: windows_core::PCWSTR = windows_core::w!("PortableDeviceNameSpaceThumbnailContentTypes");
pub const PORTABLE_DEVICE_NAMESPACE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PortableDeviceNameSpaceTimeout");
pub const PORTABLE_DEVICE_TYPE: windows_core::PCWSTR = windows_core::w!("PortableDeviceType");
pub const PortableDevice: windows_core::GUID = windows_core::GUID::from_u128(0x728a21c5_3d9e_48d7_9810_864848f0f404);
pub const PortableDeviceDispatchFactory: windows_core::GUID = windows_core::GUID::from_u128(0x43232233_8338_4658_ae01_0b4ae830b6b0);
pub const PortableDeviceFTM: windows_core::GUID = windows_core::GUID::from_u128(0xf7c0039a_4762_488a_b4b3_760ef9a1ba9b);
pub const PortableDeviceKeyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xde2d022d_2480_43be_97f0_d1fa2cf98f4f);
pub const PortableDeviceManager: windows_core::GUID = windows_core::GUID::from_u128(0x0af10cec_2ecd_4b92_9581_34f6ae0637f3);
pub const PortableDevicePropVariantCollection: windows_core::GUID = windows_core::GUID::from_u128(0x08a99e2f_6d6d_4b80_af5a_baf2bcbe4cb9);
pub const PortableDeviceService: windows_core::GUID = windows_core::GUID::from_u128(0xef5db4c2_9312_422c_9152_411cd9c4dd84);
pub const PortableDeviceServiceFTM: windows_core::GUID = windows_core::GUID::from_u128(0x1649b154_c794_497a_9b03_f3f0121302f3);
pub const PortableDeviceValues: windows_core::GUID = windows_core::GUID::from_u128(0x0c15d503_d017_47ce_9016_7b3f978721cc);
pub const PortableDeviceValuesCollection: windows_core::GUID = windows_core::GUID::from_u128(0x3882134d_14cf_4220_9cb4_435f86d83f60);
pub const PortableDeviceWebControl: windows_core::GUID = windows_core::GUID::from_u128(0x186dd02c_2dec_41b5_a7d4_b59056fade51);
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(1i32);
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(0i32);
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(1i32);
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMS_MESSAGE_TYPES(pub i32);
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(0i32);
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(1i32);
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(0i32);
pub const STR_WPDNSE_FAST_ENUM: windows_core::PCWSTR = windows_core::w!("WPDNSE Fast Enum");
pub const STR_WPDNSE_SIMPLE_ITEM: windows_core::PCWSTR = windows_core::w!("WPDNSE SimpleItem");
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_RADIO_STATE(pub i32);
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
pub const TYPE_CalendarSvc: u32 = 0u32;
pub const TYPE_ContactsSvc: u32 = 0u32;
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
pub const TYPE_HintsSvc: u32 = 0u32;
pub const TYPE_MessageSvc: u32 = 0u32;
pub const TYPE_NotesSvc: u32 = 0u32;
pub const TYPE_RingtonesSvc: u32 = 0u32;
pub const TYPE_StatusSvc: u32 = 0u32;
pub const TYPE_TasksSvc: u32 = 0u32;
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 6 };
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 5 };
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 2 };
pub const WPDNSE_OBJECT_HAS_ICON: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 4 };
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 3 };
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6), pid: 7 };
pub const WPDNSE_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x34d71409_4b47_4d80_aaac_3a28a4a3b3e6);
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: windows_core::GUID = windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4);
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 3 };
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10e54a3e_052d_4777_a13c_de7614be2bc4), pid: 2 };
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 10 };
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 13 };
pub const WPD_APPOINTMENT_LOCATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 3 };
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3);
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 9 };
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 8 };
pub const WPD_APPOINTMENT_RESOURCES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 11 };
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 12 };
pub const WPD_APPOINTMENT_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf99efd03_431d_40d8_a1c9_4e220d9c88d3), pid: 7 };
pub const WPD_AUDIO_BITRATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 9 };
pub const WPD_AUDIO_BIT_DEPTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 12 };
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 13 };
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 10 };
pub const WPD_AUDIO_FORMAT_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 11 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_BITRATE_TYPES(pub i32);
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(1i32);
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(3i32);
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(0i32);
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_CAPTURE_MODES(pub i32);
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(2i32);
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(1i32);
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(3i32);
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(0i32);
pub const WPD_CATEGORY_CAPABILITIES: windows_core::GUID = windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356);
pub const WPD_CATEGORY_COMMON: windows_core::GUID = windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a);
pub const WPD_CATEGORY_DEVICE_HINTS: windows_core::GUID = windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84);
pub const WPD_CATEGORY_MEDIA_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8);
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: windows_core::GUID = windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56);
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: windows_core::GUID = windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4);
pub const WPD_CATEGORY_NULL: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const WPD_CATEGORY_OBJECT_ENUMERATION: windows_core::GUID = windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec);
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: windows_core::GUID = windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089);
pub const WPD_CATEGORY_OBJECT_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804);
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: windows_core::GUID = windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e);
pub const WPD_CATEGORY_OBJECT_RESOURCES: windows_core::GUID = windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a);
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: windows_core::GUID = windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89);
pub const WPD_CATEGORY_SERVICE_COMMON: windows_core::GUID = windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee);
pub const WPD_CATEGORY_SERVICE_METHODS: windows_core::GUID = windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc);
pub const WPD_CATEGORY_SMS: windows_core::GUID = windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1);
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32);
pub const WPD_CATEGORY_STORAGE: windows_core::GUID = windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94);
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 3 };
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 3 };
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 2 };
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 4 };
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30), pid: 2 };
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96), pid: 2 };
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f), pid: 4 };
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: windows_core::GUID = windows_core::GUID::from_u128(0x6309ffef_a87c_4ca7_8434_797576e40a96);
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: windows_core::GUID = windows_core::GUID::from_u128(0x3e3595da_4d71_49fe_a0b4_d4406c3ae93f);
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: windows_core::GUID = windows_core::GUID::from_u128(0x65c160f8_1367_4ce2_939d_8310839f0d30);
pub const WPD_CLASS_EXTENSION_V1: windows_core::GUID = windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051);
pub const WPD_CLASS_EXTENSION_V2: windows_core::GUID = windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758);
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 9 };
pub const WPD_CLIENT_EVENT_COOKIE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 11 };
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859);
pub const WPD_CLIENT_MAJOR_VERSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 3 };
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 13 };
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 12 };
pub const WPD_CLIENT_MINOR_VERSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 4 };
pub const WPD_CLIENT_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 2 };
pub const WPD_CLIENT_REVISION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 5 };
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 8 };
pub const WPD_CLIENT_SHARE_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 10 };
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 7 };
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x204d9f0c_2292_4080_9f42_40664e70f859), pid: 6 };
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(1i32);
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(0i32);
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_COLOR_CORRECTED_STATUS_VALUES(pub i32);
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(16i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(8i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: super::super::Foundation::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: super::super::Foundation::PROPERTYKEY,
}
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(1i32);
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_COMMAND_ACCESS_TYPES(pub i32);
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 3 };
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 11 };
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 9 };
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 5 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 2 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 6 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 10 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 7 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 8 };
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 4 };
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 2 };
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 3 };
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 2 };
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 3 };
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 3 };
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 2 };
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 4 };
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 2 };
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 2 };
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 4 };
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 2 };
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x59b433ba_fe44_4d8d_808c_6bcb9b0f15e8), pid: 3 };
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 17 };
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 12 };
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 13 };
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 14 };
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 11 };
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 18 };
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 15 };
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 16 };
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 4 };
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 3 };
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 2 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 9 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 3 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 2 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 7 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 8 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 6 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 10 };
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 4 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 7 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 6 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 5 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 4 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 3 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 2 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 10 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 9 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 8 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 7 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 4 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 6 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 3 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 2 };
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 5 };
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 7 };
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 12 };
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 9 };
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 8 };
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 3 };
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 2 };
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 4 };
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5 };
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 10 };
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 11 };
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 13 };
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 6 };
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 4 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 16 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 11 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 12 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 7 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 9 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 14 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 13 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 4 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 5 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 15 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 10 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 6 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 8 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 2 };
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 3 };
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 2 };
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 3 };
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 4 };
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 2 };
pub const WPD_COMMAND_SMS_SEND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 2 };
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4fcd6982_22a2_4b05_a48b_62d38bf27b32), pid: 2 };
pub const WPD_COMMAND_STORAGE_EJECT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 4 };
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 2 };
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 3 };
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 6 };
pub const WPD_COMMON_INFORMATION_NOTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 7 };
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f);
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 4 };
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 5 };
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb28ae94b_05a4_4e8e_be01_72cc7e099d8f), pid: 2 };
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 62 };
pub const WPD_CONTACT_ASSISTANT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 61 };
pub const WPD_CONTACT_BIRTHDATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 57 };
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 34 };
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 35 };
pub const WPD_CONTACT_BUSINESS_FAX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 45 };
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 17 };
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 40 };
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 41 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 20 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 23 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 18 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 19 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 22 };
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 21 };
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 50 };
pub const WPD_CONTACT_CHILDREN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 60 };
pub const WPD_CONTACT_COMPANY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 54 };
pub const WPD_CONTACT_DISPLAY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 2 };
pub const WPD_CONTACT_FIRST_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 3 };
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 51 };
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 52 };
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 53 };
pub const WPD_CONTACT_LAST_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 5 };
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 4 };
pub const WPD_CONTACT_MOBILE_PHONE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 42 };
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 43 };
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b);
pub const WPD_CONTACT_OTHER_EMAILS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 36 };
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 24 };
pub const WPD_CONTACT_OTHER_PHONES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 47 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 27 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 25 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 26 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 29 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 30 };
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 28 };
pub const WPD_CONTACT_PAGER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 46 };
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 32 };
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 33 };
pub const WPD_CONTACT_PERSONAL_FAX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 44 };
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 10 };
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 38 };
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 39 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 13 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 16 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 11 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 12 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 15 };
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 14 };
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 49 };
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 55 };
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 8 };
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 9 };
pub const WPD_CONTACT_PREFIX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 6 };
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 31 };
pub const WPD_CONTACT_PRIMARY_FAX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 58 };
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 37 };
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 48 };
pub const WPD_CONTACT_RINGTONE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 63 };
pub const WPD_CONTACT_ROLE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 56 };
pub const WPD_CONTACT_SPOUSE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 59 };
pub const WPD_CONTACT_SUFFIX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfbd4fdab_987d_4777_b3f9_726185a9312b), pid: 7 };
pub const WPD_CONTENT_TYPE_ALL: windows_core::GUID = windows_core::GUID::from_u128(0x80e170d2_1055_4a3e_b952_82cc4f8a8689);
pub const WPD_CONTENT_TYPE_APPOINTMENT: windows_core::GUID = windows_core::GUID::from_u128(0x0fed060e_8793_4b1e_90c9_48ac389ac631);
pub const WPD_CONTENT_TYPE_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x4ad2c85e_5e2d_45e5_8864_4f229e3c6cf0);
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: windows_core::GUID = windows_core::GUID::from_u128(0xaa18737e_5009_48fa_ae21_85f24383b4e6);
pub const WPD_CONTENT_TYPE_CALENDAR: windows_core::GUID = windows_core::GUID::from_u128(0xa1fd5967_6023_49a0_9df1_f8060be751b0);
pub const WPD_CONTENT_TYPE_CERTIFICATE: windows_core::GUID = windows_core::GUID::from_u128(0xdc3876e8_a948_4060_9050_cbd77e8a3d87);
pub const WPD_CONTENT_TYPE_CONTACT: windows_core::GUID = windows_core::GUID::from_u128(0xeaba8313_4525_4707_9f0e_87c6808e9435);
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: windows_core::GUID = windows_core::GUID::from_u128(0x346b8932_4c36_40d8_9415_1828291f9de9);
pub const WPD_CONTENT_TYPE_DOCUMENT: windows_core::GUID = windows_core::GUID::from_u128(0x680adf52_950a_4041_9b41_65e393648155);
pub const WPD_CONTENT_TYPE_EMAIL: windows_core::GUID = windows_core::GUID::from_u128(0x8038044a_7e51_4f8f_883d_1d0623d14533);
pub const WPD_CONTENT_TYPE_FOLDER: windows_core::GUID = windows_core::GUID::from_u128(0x27e2e392_a111_48e0_ab0c_e17705a05f85);
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: windows_core::GUID = windows_core::GUID::from_u128(0x99ed0160_17ff_4c44_9d98_1d7a6f941921);
pub const WPD_CONTENT_TYPE_GENERIC_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x0085e0a6_8d34_45d7_bc5c_447e59c73d48);
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: windows_core::GUID = windows_core::GUID::from_u128(0xe80eaaf8_b2db_4133_b67e_1bef4b4a6e5f);
pub const WPD_CONTENT_TYPE_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0xef2107d5_a52a_4243_a26b_62d4176d7603);
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: windows_core::GUID = windows_core::GUID::from_u128(0x75793148_15f5_4a30_a813_54ed8a37e226);
pub const WPD_CONTENT_TYPE_MEDIA_CAST: windows_core::GUID = windows_core::GUID::from_u128(0x5e88b3cc_3e65_4e62_bfff_229495253ab0);
pub const WPD_CONTENT_TYPE_MEMO: windows_core::GUID = windows_core::GUID::from_u128(0x9cd20ecf_3b50_414f_a641_e473ffe45751);
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: windows_core::GUID = windows_core::GUID::from_u128(0x00f0c3ac_a593_49ac_9219_24abca5a2563);
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: windows_core::GUID = windows_core::GUID::from_u128(0x031da7ee_18c8_4205_847e_89a11261d0f3);
pub const WPD_CONTENT_TYPE_PLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0x1a33f7e4_af13_48f5_994e_77369dfe04a3);
pub const WPD_CONTENT_TYPE_PROGRAM: windows_core::GUID = windows_core::GUID::from_u128(0xd269f96a_247c_4bff_98fb_97f3c49220e6);
pub const WPD_CONTENT_TYPE_SECTION: windows_core::GUID = windows_core::GUID::from_u128(0x821089f5_1d91_4dc9_be3c_bbb1b35b18ce);
pub const WPD_CONTENT_TYPE_TASK: windows_core::GUID = windows_core::GUID::from_u128(0x63252f2c_887f_4cb6_b1ac_d29855dcef6c);
pub const WPD_CONTENT_TYPE_TELEVISION: windows_core::GUID = windows_core::GUID::from_u128(0x60a169cf_f2ae_4e21_9375_9677f11c1c6e);
pub const WPD_CONTENT_TYPE_UNSPECIFIED: windows_core::GUID = windows_core::GUID::from_u128(0x28d8d31e_249c_454e_aabc_34883168e634);
pub const WPD_CONTENT_TYPE_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x9261b03c_3d78_4519_85e3_02c5e1f50bb9);
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: windows_core::GUID = windows_core::GUID::from_u128(0x012b0db7_d4c1_45d6_b081_94b87779614f);
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0x0bac070a_9f5f_4da4_a8f6_3de44d68fd6c);
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(1i32);
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(0i32);
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_CROPPED_STATUS_VALUES(pub i32);
pub const WPD_DEVICE_DATETIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 11 };
pub const WPD_DEVICE_EDP_IDENTITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed), pid: 1 };
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 3 };
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 12 };
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 2 };
pub const WPD_DEVICE_MANUFACTURER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 7 };
pub const WPD_DEVICE_MODEL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 8 };
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 3 };
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 16 };
pub const WPD_DEVICE_OBJECT_ID: windows_core::PCWSTR = windows_core::w!("DEVICE");
pub const WPD_DEVICE_POWER_LEVEL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 4 };
pub const WPD_DEVICE_POWER_SOURCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 5 };
pub const WPD_DEVICE_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc);
pub const WPD_DEVICE_PROPERTIES_V2: windows_core::GUID = windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799);
pub const WPD_DEVICE_PROPERTIES_V3: windows_core::GUID = windows_core::GUID::from_u128(0x6c2b878c_c2ec_490d_b425_d7a75e23e5ed);
pub const WPD_DEVICE_PROTOCOL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 6 };
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 9 };
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 13 };
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 14 };
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 10 };
pub const WPD_DEVICE_SYNC_PARTNER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 2 };
pub const WPD_DEVICE_TRANSPORT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 4 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_DEVICE_TRANSPORTS(pub i32);
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(3i32);
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(2i32);
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(0i32);
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(1i32);
pub const WPD_DEVICE_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x26d4979a_e643_4626_9e2b_736dc0c92fdc), pid: 15 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_DEVICE_TYPES(pub i32);
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(6i32);
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(1i32);
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(0i32);
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(2i32);
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(5i32);
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(3i32);
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(4i32);
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x463dd662_7fc4_4291_911c_7f4c9cca9799), pid: 5 };
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x0b110203_eb95_4f02_93e0_97c631493ad5);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_EFFECT_MODES(pub i32);
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = WPD_EFFECT_MODES(2i32);
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = WPD_EFFECT_MODES(1i32);
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = WPD_EFFECT_MODES(3i32);
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = WPD_EFFECT_MODES(0i32);
pub const WPD_EMAIL_BCC_LINE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 4 };
pub const WPD_EMAIL_CC_LINE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 3 };
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 9 };
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 7 };
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5);
pub const WPD_EMAIL_RECEIVED_TIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 8 };
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 10 };
pub const WPD_EMAIL_TO_LINE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41f8f65a_5484_4782_b13d_4740dd7c37c5), pid: 2 };
pub const WPD_EVENT_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d);
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 2 };
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 4 };
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10c96578_2e81_4111_adde_e08ca6138f6d), pid: 3 };
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0x36885aa1_cd54_4daa_b3d0_afb3e03f5999);
pub const WPD_EVENT_DEVICE_REMOVED: windows_core::GUID = windows_core::GUID::from_u128(0xe4cbca1b_6918_48b9_85ee_02be7c850af9);
pub const WPD_EVENT_DEVICE_RESET: windows_core::GUID = windows_core::GUID::from_u128(0x7755cf53_c1ed_44f3_b5a2_451e2c376b27);
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_5738_4ff2_8445_be3126691059);
pub const WPD_EVENT_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x2ba2e40a_6b4c_4295_bb43_26322b99aeb2);
pub const WPD_EVENT_OBJECT_ADDED: windows_core::GUID = windows_core::GUID::from_u128(0xa726da95_e207_4b02_8d44_bef2e86cbffc);
pub const WPD_EVENT_OBJECT_REMOVED: windows_core::GUID = windows_core::GUID::from_u128(0xbe82ab88_a52c_4823_96e5_d0272671fc38);
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: windows_core::GUID = windows_core::GUID::from_u128(0x8d16a0a1_f2c6_41da_8f19_5e53721adbf2);
pub const WPD_EVENT_OBJECT_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0x1445a759_2e01_485d_9f27_ff07dae697ab);
pub const WPD_EVENT_OPTIONS_V1: windows_core::GUID = windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b);
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 3 };
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3d8dad7_a361_4b83_8a48_5b02ce10713b), pid: 2 };
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 8 };
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 3 };
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 7 };
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 6 };
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 5 };
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 4 };
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0), pid: 2 };
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846), pid: 2 };
pub const WPD_EVENT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x15ab1953_f817_4fef_a921_5676e838f6e0);
pub const WPD_EVENT_PROPERTIES_V2: windows_core::GUID = windows_core::GUID::from_u128(0x52807b8a_4914_4323_9b9a_74f654b2b846);
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: windows_core::GUID = windows_core::GUID::from_u128(0x8a33f5f8_0acc_4d9b_9cc4_112d353b86ca);
pub const WPD_EVENT_STORAGE_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0x3782616b_22bc_4474_a251_3070f8d38857);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_EXPOSURE_METERING_MODES(pub i32);
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(1i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(4i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(2i32);
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(3i32);
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_EXPOSURE_PROGRAM_MODES(pub i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(6i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(3i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(2i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(5i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(1i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(7i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(4i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_FLASH_MODES(pub i32);
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(1i32);
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = WPD_FLASH_MODES(6i32);
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(3i32);
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = WPD_FLASH_MODES(2i32);
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(4i32);
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(5i32);
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = WPD_FLASH_MODES(0i32);
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = WPD_FOCUS_MODES(2i32);
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = WPD_FOCUS_MODES(3i32);
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = WPD_FOCUS_MODES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_FOCUS_METERING_MODES(pub i32);
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(1i32);
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(2i32);
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_FOCUS_MODES(pub i32);
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = WPD_FOCUS_MODES(0i32);
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d), pid: 2 };
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x7e9a7abf_e568_4b34_aa2f_13bb12ab177d);
pub const WPD_FORMAT_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f);
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 3 };
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xa0a02000_bcaf_4be8_b3f5_233f231cf58f), pid: 2 };
pub const WPD_FUNCTIONAL_CATEGORY_ALL: windows_core::GUID = windows_core::GUID::from_u128(0x2d8a6512_a74c_448e_ba8a_f4ac07c49399);
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x3f2a1919_c7c2_4a00_855d_f57cf06debbb);
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0x08ea466b_e3a4_4336_a1f3_a44d2b5c438c);
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: windows_core::GUID = windows_core::GUID::from_u128(0x48f4db72_7c6a_4ab0_9e1a_470e3cdbf26a);
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x08600ba4_a7ba_4a01_ab0e_0065d0a356d3);
pub const WPD_FUNCTIONAL_CATEGORY_SMS: windows_core::GUID = windows_core::GUID::from_u128(0x0044a0b1_c1e9_4afd_b358_a62c6117c9cf);
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x613ca327_ab93_4900_b4fa_895bb5874b79);
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: windows_core::GUID = windows_core::GUID::from_u128(0x23f05bbc_15de_4c2a_a55b_a9af5ce412ef);
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0xe23e5f6b_7243_43aa_8df1_0eb3d968a918);
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598), pid: 2 };
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x8f052d93_abca_4fc5_a5ac_b01df4dbe598);
pub const WPD_IMAGE_BITDEPTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 3 };
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 5 };
pub const WPD_IMAGE_CROPPED_STATUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 4 };
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 8 };
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 7 };
pub const WPD_IMAGE_FNUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 6 };
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 9 };
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db);
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x63d64908_9fa1_479f_85ba_9952216447db), pid: 10 };
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 25 };
pub const WPD_MEDIA_ARTIST: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 24 };
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 49 };
pub const WPD_MEDIA_BITRATE_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 3 };
pub const WPD_MEDIA_BUY_NOW: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 20 };
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 36 };
pub const WPD_MEDIA_COMPOSER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 11 };
pub const WPD_MEDIA_COPYRIGHT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 4 };
pub const WPD_MEDIA_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 31 };
pub const WPD_MEDIA_DESTINATION_URL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 30 };
pub const WPD_MEDIA_DURATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 19 };
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 12 };
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 21 };
pub const WPD_MEDIA_GENRE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 32 };
pub const WPD_MEDIA_GUID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 38 };
pub const WPD_MEDIA_HEIGHT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 23 };
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 8 };
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 35 };
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 27 };
pub const WPD_MEDIA_META_GENRE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 10 };
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 34 };
pub const WPD_MEDIA_OWNER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 26 };
pub const WPD_MEDIA_PARENTAL_RATING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 9 };
pub const WPD_MEDIA_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8);
pub const WPD_MEDIA_RELEASE_DATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 14 };
pub const WPD_MEDIA_SAMPLE_RATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 15 };
pub const WPD_MEDIA_SKIP_COUNT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 7 };
pub const WPD_MEDIA_SOURCE_URL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 29 };
pub const WPD_MEDIA_STAR_RATING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 16 };
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 5 };
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 39 };
pub const WPD_MEDIA_SUB_TITLE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 13 };
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 33 };
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 37 };
pub const WPD_MEDIA_TITLE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 18 };
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 2 };
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 17 };
pub const WPD_MEDIA_USE_COUNT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 6 };
pub const WPD_MEDIA_WEBMASTER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 28 };
pub const WPD_MEDIA_WIDTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2ed8ba05_0ad3_42dc_b0d0_bc95ac396ac8), pid: 22 };
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x5ffbfc7b_7483_41ad_afb9_da3f4e592b8d);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_META_GENRES(pub i32);
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(64i32);
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(37i32);
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(1i32);
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = WPD_META_GENRES(48i32);
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(17i32);
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(33i32);
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(36i32);
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = WPD_META_GENRES(66i32);
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(35i32);
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(34i32);
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(40i32);
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = WPD_META_GENRES(18i32);
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = WPD_META_GENRES(19i32);
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = WPD_META_GENRES(20i32);
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = WPD_META_GENRES(21i32);
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(38i32);
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(39i32);
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = WPD_META_GENRES(0i32);
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(65i32);
pub const WPD_METHOD_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a);
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 4 };
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 3 };
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 2 };
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf17a5071_f039_44af_8efe_432cf32e432a), pid: 5 };
pub const WPD_MUSIC_ALBUM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 3 };
pub const WPD_MUSIC_LYRICS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 6 };
pub const WPD_MUSIC_MOOD: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 8 };
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6);
pub const WPD_MUSIC_TRACK: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb324f56a_dc5d_46e5_b6df_d2ea414888c6), pid: 4 };
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 2 };
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274);
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4c93c1f_b203_43f1_a100_5a07d11b0274), pid: 3 };
pub const WPD_OBJECT_BACK_REFERENCES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 21 };
pub const WPD_OBJECT_CAN_DELETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 26 };
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 23 };
pub const WPD_OBJECT_CONTENT_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 7 };
pub const WPD_OBJECT_DATE_AUTHORED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 20 };
pub const WPD_OBJECT_DATE_CREATED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 18 };
pub const WPD_OBJECT_DATE_MODIFIED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 19 };
pub const WPD_OBJECT_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 6 };
pub const WPD_OBJECT_FORMAT_3G2: windows_core::GUID = windows_core::GUID::from_u128(0xb9850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3G2A: windows_core::GUID = windows_core::GUID::from_u128(0x1a11202d_8759_4e34_ba5e_b1211087eee4);
pub const WPD_OBJECT_FORMAT_3GP: windows_core::GUID = windows_core::GUID::from_u128(0xb9840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_3GPA: windows_core::GUID = windows_core::GUID::from_u128(0xe5172730_f971_41ef_a10b_2271a0019d7a);
pub const WPD_OBJECT_FORMAT_AAC: windows_core::GUID = windows_core::GUID::from_u128(0xb9030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: windows_core::GUID = windows_core::GUID::from_u128(0xbb810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: windows_core::GUID = windows_core::GUID::from_u128(0xba060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: windows_core::GUID = windows_core::GUID::from_u128(0xba0b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AIFF: windows_core::GUID = windows_core::GUID::from_u128(0x30070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xc1f62eb2_4bb3_479c_9cfa_05b5f3a57b22);
pub const WPD_OBJECT_FORMAT_AMR: windows_core::GUID = windows_core::GUID::from_u128(0xb9080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASF: windows_core::GUID = windows_core::GUID::from_u128(0x300c0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0xba130000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ATSCTS: windows_core::GUID = windows_core::GUID::from_u128(0xb9870000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AUDIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xb9040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVCHD: windows_core::GUID = windows_core::GUID::from_u128(0xb9860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_AVI: windows_core::GUID = windows_core::GUID::from_u128(0x300a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_BMP: windows_core::GUID = windows_core::GUID::from_u128(0x38040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_CIFF: windows_core::GUID = windows_core::GUID::from_u128(0x38050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DPOF: windows_core::GUID = windows_core::GUID::from_u128(0x30060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_DVBTS: windows_core::GUID = windows_core::GUID::from_u128(0xb9880000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXECUTABLE: windows_core::GUID = windows_core::GUID::from_u128(0x30030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_EXIF: windows_core::GUID = windows_core::GUID::from_u128(0x38010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLAC: windows_core::GUID = windows_core::GUID::from_u128(0xb9060000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_FLASHPIX: windows_core::GUID = windows_core::GUID::from_u128(0x38030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_GIF: windows_core::GUID = windows_core::GUID::from_u128(0x38070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_HTML: windows_core::GUID = windows_core::GUID::from_u128(0x30050000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICALENDAR: windows_core::GUID = windows_core::GUID::from_u128(0xbe030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_ICON: windows_core::GUID = windows_core::GUID::from_u128(0x077232ed_102c_4638_9c22_83f142bfc822);
pub const WPD_OBJECT_FORMAT_JFIF: windows_core::GUID = windows_core::GUID::from_u128(0x38080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JP2: windows_core::GUID = windows_core::GUID::from_u128(0x380f0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPEGXR: windows_core::GUID = windows_core::GUID::from_u128(0xb8040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_JPX: windows_core::GUID = windows_core::GUID::from_u128(0x38100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0xba110000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_M4A: windows_core::GUID = windows_core::GUID::from_u128(0x30aba7ac_6ffd_4c23_a359_3e9b52f3f1c8);
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: windows_core::GUID = windows_core::GUID::from_u128(0xba840000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: windows_core::GUID = windows_core::GUID::from_u128(0xba850000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: windows_core::GUID = windows_core::GUID::from_u128(0xba860000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: windows_core::GUID = windows_core::GUID::from_u128(0xb1040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: windows_core::GUID = windows_core::GUID::from_u128(0xba830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MKV: windows_core::GUID = windows_core::GUID::from_u128(0xb9900000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP2: windows_core::GUID = windows_core::GUID::from_u128(0xb9830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP3: windows_core::GUID = windows_core::GUID::from_u128(0x30090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MP4: windows_core::GUID = windows_core::GUID::from_u128(0xb9820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPEG: windows_core::GUID = windows_core::GUID::from_u128(0x300b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0xba120000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: windows_core::GUID = windows_core::GUID::from_u128(0xb1020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_OGG: windows_core::GUID = windows_core::GUID::from_u128(0xb9020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PCD: windows_core::GUID = windows_core::GUID::from_u128(0x38090000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PICT: windows_core::GUID = windows_core::GUID::from_u128(0x380a0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0xba140000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PNG: windows_core::GUID = windows_core::GUID::from_u128(0x380b0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: windows_core::GUID = windows_core::GUID::from_u128(0x30010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_QCELP: windows_core::GUID = windows_core::GUID::from_u128(0xb9070000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_SCRIPT: windows_core::GUID = windows_core::GUID::from_u128(0x30020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TEXT: windows_core::GUID = windows_core::GUID::from_u128(0x30040000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFF: windows_core::GUID = windows_core::GUID::from_u128(0x380d0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFEP: windows_core::GUID = windows_core::GUID::from_u128(0x38020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_TIFFIT: windows_core::GUID = windows_core::GUID::from_u128(0x380e0000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: windows_core::GUID = windows_core::GUID::from_u128(0x30000000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCALENDAR1: windows_core::GUID = windows_core::GUID::from_u128(0xbe020000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD2: windows_core::GUID = windows_core::GUID::from_u128(0xbb820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_VCARD3: windows_core::GUID = windows_core::GUID::from_u128(0xbb830000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WAVE: windows_core::GUID = windows_core::GUID::from_u128(0x30080000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb8030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: windows_core::GUID = windows_core::GUID::from_u128(0xb8810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMA: windows_core::GUID = windows_core::GUID::from_u128(0xb9010000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WMV: windows_core::GUID = windows_core::GUID::from_u128(0xb9810000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: windows_core::GUID = windows_core::GUID::from_u128(0xba100000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: windows_core::GUID = windows_core::GUID::from_u128(0xb1030000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_FORMAT_XML: windows_core::GUID = windows_core::GUID::from_u128(0xba820000_ae6c_4804_98ba_c57b46965fe7);
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 24 };
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 25 };
pub const WPD_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 2 };
pub const WPD_OBJECT_ISHIDDEN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 9 };
pub const WPD_OBJECT_ISSYSTEM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 10 };
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 17 };
pub const WPD_OBJECT_KEYWORDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 15 };
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 27 };
pub const WPD_OBJECT_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 4 };
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 13 };
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 12 };
pub const WPD_OBJECT_PARENT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 3 };
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 5 };
pub const WPD_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c);
pub const WPD_OBJECT_PROPERTIES_V2: windows_core::GUID = windows_core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775);
pub const WPD_OBJECT_REFERENCES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 14 };
pub const WPD_OBJECT_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 11 };
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0373cd3d_4a46_40d7_b4d8_73e8da74e775), pid: 2 };
pub const WPD_OBJECT_SYNC_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef6b490d_5cd8_437a_affc_da8b60ee4a3c), pid: 16 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_OPERATION_STATES(pub i32);
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(6i32);
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(4i32);
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(5i32);
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(3i32);
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = WPD_OPERATION_STATES(2i32);
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(1i32);
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(0i32);
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 5001 };
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5003 };
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5001 };
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 5002 };
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 5001 };
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 5001 };
pub const WPD_PARAMETER_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58);
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 5 };
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 9 };
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 4 };
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = WpdParameterAttributeForm(2i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = WpdParameterAttributeForm(4i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = WpdParameterAttributeForm(1i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = WpdParameterAttributeForm(3i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = WpdParameterAttributeForm(0i32);
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 11 };
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 13 };
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 2 };
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 7 };
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 6 };
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 8 };
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 10 };
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 3 };
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe6864dd7_f325_45ea_a1d5_97cf73b6ca58), pid: 12 };
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(1i32);
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(3i32);
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(2i32);
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_PARAMETER_USAGE_TYPES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_POWER_SOURCES(pub i32);
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = WPD_POWER_SOURCES(0i32);
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = WPD_POWER_SOURCES(1i32);
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: windows_core::GUID = windows_core::GUID::from_u128(0x4d545058_8900_40b3_8f1d_dc246e1e8370);
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: windows_core::GUID = windows_core::GUID::from_u128(0x4d545058_4fce_4578_95c8_8698a9bc0f49);
pub const WPD_PROPERTY_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37);
pub const WPD_PROPERTY_ATTRIBUTES_V2: windows_core::GUID = windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e);
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 5 };
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 3 };
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 4 };
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 6 };
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 11 };
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 7 };
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 2 };
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = WpdAttributeForm(2i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = WpdAttributeForm(4i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = WpdAttributeForm(1i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = WpdAttributeForm(3i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = WpdAttributeForm(0i32);
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 13 };
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 2 };
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 9 };
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 8 };
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 10 };
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xab7943d8_6332_445f_a00d_8d5ef1e96f37), pid: 12 };
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5d9da160_74ae_43cc_85a9_fe555a80798e), pid: 3 };
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1002 };
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1003 };
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1008 };
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1007 };
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1014 };
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1015 };
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1010 };
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1009 };
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1004 };
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1005 };
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1006 };
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1012 };
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1011 };
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1001 };
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0cabec78_6b74_41c6_9216_2639d1fce356), pid: 1013 };
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1001 };
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x33fb0d11_64a3_4fac_b4c7_3dfeaa99b051), pid: 1002 };
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1002 };
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1001 };
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7f0779b5_fa2b_4766_9cb2_f73ba30b6758), pid: 1003 };
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1011 };
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1009 };
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1010 };
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1001 };
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1002 };
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1006 };
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1004 };
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1003 };
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1008 };
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf0422a9c_5dc8_4440_b5bd_5df28835658a), pid: 1007 };
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1002 };
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0d5fb92b_cb46_4c4f_8343_0bc3d3f17c84), pid: 1001 };
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_ef88_4e4d_95c3_4f327f728a96), pid: 1011 };
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1001 };
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1002 };
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1013 };
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1003 };
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1004 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1006 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1012 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1009 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1008 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1010 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1011 };
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1007 };
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1014 };
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4d545058_1a2e_4106_a357_771e0819fc56), pid: 1005 };
pub const WPD_PROPERTY_NULL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000), pid: 0 };
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1004 };
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1002 };
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1005 };
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1003 };
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb7474e91_e7f8_4ad9_b400_ad1a4b58eeec), pid: 1001 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1002 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1013 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1001 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1005 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1007 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1010 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1011 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1012 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1003 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1004 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1016 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1006 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1009 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1008 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1015 };
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xef1e43dd_a9ed_4341_8bcc_186192aea089), pid: 1014 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1002 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1005 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1007 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1001 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1006 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1004 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1003 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11c824dd_04cd_4e4e_8c7b_f6efb794d84e), pid: 1008 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1001 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1003 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1006 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1002 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1004 };
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9e5582e4_0814_44e6_981a_b2998d583804), pid: 1005 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1002 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1005 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1010 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1007 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1006 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1008 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1009 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1001 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1011 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1014 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1004 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1003 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1012 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1013 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1016 };
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb3a2b22d_a595_4108_be0a_fc3c965f3d4a), pid: 1015 };
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x78f9c6fc_79b8_473c_9060_6bd23dd072c4), pid: 1001 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1018 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1019 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1012 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1013 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1002 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1007 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1008 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1014 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1015 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1003 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1004 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1005 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1006 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1010 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1009 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1016 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1017 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1011 };
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x24457e74_2e9f_44f9_8c57_1d1bcb170b89), pid: 1001 };
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1001 };
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1004 };
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1005 };
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1002 };
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2d521ca8_c1b0_4268_a342_cf19321569bc), pid: 1003 };
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x322f071d_36ef_477f_b4b5_6f52d734baee), pid: 1001 };
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1004 };
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1002 };
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1001 };
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xafc25d66_fe0d_4114_9097_970c93e920d1), pid: 1003 };
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1002 };
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd8f907a6_34cc_45fa_97fb_d007fa47ec94), pid: 1001 };
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4);
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 2 };
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 4 };
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc53d039f_ee23_4a31_8590_7639879870b4), pid: 3 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(pub i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(0i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(1i32);
pub const WPD_RESOURCE_ALBUM_ART: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf02aa354_2300_4e2d_a1b9_3b6730f7fa21), pid: 0 };
pub const WPD_RESOURCE_ATTRIBUTES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6);
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 5 };
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 3 };
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 4 };
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 8 };
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 6 };
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 7 };
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 9 };
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1eb6f604_9278_429f_93cc_5bb8c06656b6), pid: 2 };
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3bc13982_85b1_48e0_95a6_8d3ad06be117), pid: 0 };
pub const WPD_RESOURCE_BRANDING_ART: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb633b1ae_6caf_4a87_9589_22ded6dd5899), pid: 0 };
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2c4d6803_80ea_4580_af9a_5be1a23eddcb), pid: 0 };
pub const WPD_RESOURCE_DEFAULT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe81e79be_34f0_41bf_b53f_f1a06ae87842), pid: 0 };
pub const WPD_RESOURCE_GENERIC: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb9b9f515_ba70_4647_94dc_fa4925e95a07), pid: 0 };
pub const WPD_RESOURCE_ICON: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf195fed8_aa28_4ee3_b153_e182dd5edc39), pid: 0 };
pub const WPD_RESOURCE_THUMBNAIL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc7c407ba_98fa_46b5_9960_23fec124cfde), pid: 0 };
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb566ee42_6368_4290_8662_70182fb79f20), pid: 0 };
pub const WPD_SECTION_DATA_LENGTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 3 };
pub const WPD_SECTION_DATA_OFFSET: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 2 };
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 5 };
pub const WPD_SECTION_DATA_UNITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66), pid: 4 };
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(0i32);
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_SECTION_DATA_UNITS_VALUES(pub i32);
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x516afd2b_c64e_44f0_98dc_bee1c88f7d66);
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = WPD_SERVICE_INHERITANCE_TYPES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_SERVICE_INHERITANCE_TYPES(pub i32);
pub const WPD_SERVICE_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06);
pub const WPD_SERVICE_VERSION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7510698a_cb54_481c_b8db_0d75c93f1c06), pid: 2 };
pub const WPD_SMS_ENCODING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 5 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_SMS_ENCODING_TYPES(pub i32);
pub const WPD_SMS_MAX_PAYLOAD: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 4 };
pub const WPD_SMS_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d);
pub const WPD_SMS_PROVIDER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 2 };
pub const WPD_SMS_TIMEOUT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7e1074cc_50ff_4dd1_a742_53be6f093a0d), pid: 3 };
pub const WPD_STILL_IMAGE_ARTIST: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 29 };
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 24 };
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 23 };
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 31 };
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 30 };
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 17 };
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 3 };
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 18 };
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260);
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 2 };
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 4 };
pub const WPD_STILL_IMAGE_CONTRAST: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 19 };
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 21 };
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 22 };
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 16 };
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 15 };
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 11 };
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 14 };
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 13 };
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 12 };
pub const WPD_STILL_IMAGE_FNUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 7 };
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 8 };
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 9 };
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 27 };
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 10 };
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 6 };
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 20 };
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 26 };
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 25 };
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 28 };
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58c571ec_1bcb_42a7_8ac5_bb291573a260), pid: 5 };
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 11 };
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(0i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(1i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_STORAGE_ACCESS_CAPABILITY_VALUES(pub i32);
pub const WPD_STORAGE_CAPACITY: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 4 };
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 10 };
pub const WPD_STORAGE_DESCRIPTION: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 7 };
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 3 };
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 5 };
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 6 };
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 9 };
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a);
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 8 };
pub const WPD_STORAGE_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01a3057a_74d6_4e80_bea7_dc4c212ce50a), pid: 2 };
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(3i32);
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(1i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(4i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(2i32);
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_STORAGE_TYPE_VALUES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_STREAM_UNITS(pub i32);
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(0i32);
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(1i32);
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(8i32);
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(4i32);
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(2i32);
pub const WPD_TASK_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7);
pub const WPD_TASK_OWNER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 11 };
pub const WPD_TASK_PERCENT_COMPLETE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 8 };
pub const WPD_TASK_REMINDER_DATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 10 };
pub const WPD_TASK_STATUS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe354e95e_d8a0_4637_a03a_0cb26838dbc7), pid: 6 };
pub const WPD_VIDEO_AUTHOR: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 2 };
pub const WPD_VIDEO_BITRATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 13 };
pub const WPD_VIDEO_BUFFER_SIZE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 8 };
pub const WPD_VIDEO_CREDITS: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 9 };
pub const WPD_VIDEO_FOURCC_CODE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 14 };
pub const WPD_VIDEO_FRAMERATE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 15 };
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 10 };
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: windows_core::GUID = windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a);
pub const WPD_VIDEO_QUALITY_SETTING: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 11 };
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 5 };
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 7 };
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 4 };
pub const WPD_VIDEO_SCAN_TYPE: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x346f2163_f998_4146_8b01_d19b4c00de9a), pid: 12 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_VIDEO_SCAN_TYPES(pub i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(3i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(2i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(5i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(4i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(6i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(7i32);
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(1i32);
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(0i32);
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(2i32);
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(4i32);
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(7i32);
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(5i32);
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(1i32);
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPD_WHITE_BALANCE_SETTINGS(pub i32);
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(6i32);
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WpdAttributeForm(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WpdParameterAttributeForm(pub i32);
pub const WpdSerializer: windows_core::GUID = windows_core::GUID::from_u128(0x0b91a74b_ad7c_4a9d_b563_29eef9167172);
