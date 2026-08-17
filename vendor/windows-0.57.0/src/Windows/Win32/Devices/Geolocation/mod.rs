windows_core::imp::define_interface!(ICivicAddressReport, ICivicAddressReport_Vtbl, 0xc0b19f70_4adf_445d_87f2_cad8fd711792);
impl core::ops::Deref for ICivicAddressReport {
    type Target = ILocationReport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICivicAddressReport, windows_core::IUnknown, ILocationReport);
impl ICivicAddressReport {
    pub unsafe fn GetAddressLine1(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAddressLine1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAddressLine2(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAddressLine2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCity(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStateProvince(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStateProvince)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPostalCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPostalCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCountryRegion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCountryRegion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDetailLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDetailLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICivicAddressReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetAddressLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAddressLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStateProvince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCountryRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICivicAddressReportFactory, ICivicAddressReportFactory_Vtbl, 0xbf773b93_c64f_4bee_beb2_67c0b8df66e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICivicAddressReportFactory {
    type Target = ILocationReportFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICivicAddressReportFactory, windows_core::IUnknown, super::super::System::Com::IDispatch, ILocationReportFactory);
#[cfg(feature = "Win32_System_Com")]
impl ICivicAddressReportFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CivicAddressReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICivicAddressReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CivicAddressReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CivicAddressReport: usize,
}
windows_core::imp::define_interface!(IDefaultLocation, IDefaultLocation_Vtbl, 0xa65af77e_969a_4a2e_8aca_33bb7cbb1235);
impl core::ops::Deref for IDefaultLocation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDefaultLocation, windows_core::IUnknown);
impl IDefaultLocation {
    pub unsafe fn SetReport<P0>(&self, reporttype: *const windows_core::GUID, plocationreport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILocationReport>,
    {
        (windows_core::Interface::vtable(self).SetReport)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()).ok()
    }
    pub unsafe fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDefaultLocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDispCivicAddressReport, IDispCivicAddressReport_Vtbl, 0x16ff1a34_9e30_42c3_b44d_e22513b5767a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDispCivicAddressReport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDispCivicAddressReport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDispCivicAddressReport {
    pub unsafe fn AddressLine1(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddressLine1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddressLine2(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddressLine2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn City(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).City)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StateProvince(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StateProvince)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PostalCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CountryRegion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CountryRegion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DetailLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DetailLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDispCivicAddressReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddressLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddressLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StateProvince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CountryRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDispLatLongReport, IDispLatLongReport_Vtbl, 0x8ae32723_389b_4a11_9957_5bdd48fc9617);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDispLatLongReport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDispLatLongReport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDispLatLongReport {
    pub unsafe fn Latitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Latitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Longitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Longitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ErrorRadius(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Altitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Altitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AltitudeError(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AltitudeError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDispLatLongReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Latitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Longitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Altitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILatLongReport, ILatLongReport_Vtbl, 0x7fed806d_0ef8_4f07_80ac_36a0beae3134);
impl core::ops::Deref for ILatLongReport {
    type Target = ILocationReport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILatLongReport, windows_core::IUnknown, ILocationReport);
impl ILatLongReport {
    pub unsafe fn GetLatitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLongitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLongitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorRadius(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAltitude(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAltitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAltitudeError(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAltitudeError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ILatLongReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetLatitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetLongitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetAltitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetAltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ILatLongReportFactory, ILatLongReportFactory_Vtbl, 0x3f0804cb_b114_447d_83dd_390174ebb082);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ILatLongReportFactory {
    type Target = ILocationReportFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ILatLongReportFactory, windows_core::IUnknown, super::super::System::Com::IDispatch, ILocationReportFactory);
#[cfg(feature = "Win32_System_Com")]
impl ILatLongReportFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LatLongReport(&self) -> windows_core::Result<IDispLatLongReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LatLongReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILatLongReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LatLongReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LatLongReport: usize,
}
windows_core::imp::define_interface!(ILocation, ILocation_Vtbl, 0xab2ece69_56d9_4f28_b525_de1b0ee44237);
impl core::ops::Deref for ILocation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILocation, windows_core::IUnknown);
impl ILocation {
    pub unsafe fn RegisterForReport<P0>(&self, pevents: P0, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILocationEvents>,
    {
        (windows_core::Interface::vtable(self).RegisterForReport)(windows_core::Interface::as_raw(self), pevents.param().abi(), reporttype, dwrequestedreportinterval).ok()
    }
    pub unsafe fn UnregisterForReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterForReport)(windows_core::Interface::as_raw(self), reporttype).ok()
    }
    pub unsafe fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetReportStatus(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<LOCATION_REPORT_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReportStatus)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetReportInterval(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReportInterval)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
    }
    pub unsafe fn SetReportInterval(&self, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReportInterval)(windows_core::Interface::as_raw(self), reporttype, millisecondsrequested).ok()
    }
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn GetDesiredAccuracy(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn SetDesiredAccuracy(&self, reporttype: *const windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, desiredaccuracy).ok()
    }
    pub unsafe fn RequestPermissions<P0, P1>(&self, hparent: P0, preporttypes: &[windows_core::GUID], fmodal: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent.param().abi(), core::mem::transmute(preporttypes.as_ptr()), preporttypes.len().try_into().unwrap(), fmodal.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ILocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub UnregisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
    pub GetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub GetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))]
    GetDesiredAccuracy: usize,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))]
    SetDesiredAccuracy: usize,
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocationEvents, ILocationEvents_Vtbl, 0xcae02bbf_798b_4508_a207_35a7906dc73d);
impl core::ops::Deref for ILocationEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILocationEvents, windows_core::IUnknown);
impl ILocationEvents {
    pub unsafe fn OnLocationChanged<P0>(&self, reporttype: *const windows_core::GUID, plocationreport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILocationReport>,
    {
        (windows_core::Interface::vtable(self).OnLocationChanged)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()).ok()
    }
    pub unsafe fn OnStatusChanged(&self, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatusChanged)(windows_core::Interface::as_raw(self), reporttype, newstatus).ok()
    }
}
#[repr(C)]
pub struct ILocationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLocationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocationPower, ILocationPower_Vtbl, 0x193e7729_ab6b_4b12_8617_7596e1bb191c);
impl core::ops::Deref for ILocationPower {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILocationPower, windows_core::IUnknown);
impl ILocationPower {
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ILocationPower_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocationReport, ILocationReport_Vtbl, 0xc8b7f7ee_75d0_4db9_b62d_7a0f369ca456);
impl core::ops::Deref for ILocationReport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILocationReport, windows_core::IUnknown);
impl ILocationReport {
    pub unsafe fn GetSensorID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSensorID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILocationReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSensorID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ILocationReportFactory, ILocationReportFactory_Vtbl, 0x2daec322_90b2_47e4_bb08_0da841935a6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ILocationReportFactory {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ILocationReportFactory, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ILocationReportFactory {
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ListenForReports)(windows_core::Interface::as_raw(self), requestedreportinterval).ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopListeningForReports)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Status(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReportInterval(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReportInterval)(windows_core::Interface::as_raw(self), millisecondsrequested).ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DesiredAccuracy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesiredAccuracy)(windows_core::Interface::as_raw(self), desiredaccuracy).ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hwnd).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILocationReportFactory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ListenForReports: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StopListeningForReports: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_ICivicAddressReportFactoryEvents, _ICivicAddressReportFactoryEvents_Vtbl, 0xc96039ff_72ec_4617_89bd_84d88bedc722);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _ICivicAddressReportFactoryEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_ICivicAddressReportFactoryEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _ICivicAddressReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ICivicAddressReportFactoryEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_ILatLongReportFactoryEvents, _ILatLongReportFactoryEvents_Vtbl, 0x16ee6cb7_ab3c_424b_849f_269be551fcbc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _ILatLongReportFactoryEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_ILatLongReportFactoryEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _ILatLongReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ILatLongReportFactoryEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(3i32);
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(2i32);
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(1i32);
pub const GNSS_ClearAgnssData: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(10i32);
pub const GNSS_CustomCommand: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(256i32);
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
pub const GNSS_Event_BreadcrumbAlertEvent: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(17i32);
pub const GNSS_Event_Custom: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(32768i32);
pub const GNSS_Event_DriverRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(16i32);
pub const GNSS_Event_Error: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(3i32);
pub const GNSS_Event_FixAvailable: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(1i32);
pub const GNSS_Event_FixAvailable_2: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(18i32);
pub const GNSS_Event_GeofenceAlertData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(14i32);
pub const GNSS_Event_GeofencesTrackingStatus: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(15i32);
pub const GNSS_Event_NiRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(12i32);
pub const GNSS_Event_NmeaData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(13i32);
pub const GNSS_Event_RequireAgnss: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(2i32);
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(3i32);
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(2i32);
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(4i32);
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(1i32);
pub const GNSS_ForceOperationMode: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(4i32);
pub const GNSS_ForceSatelliteSystem: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(3i32);
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = GNSS_GEOREGIONTYPE(1i32);
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(1i32);
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(2i32);
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(0i32);
pub const GNSS_MAXSATELLITE: u32 = 64u32;
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(2i32);
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(1i32);
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(2i32);
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(3i32);
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(4i32);
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(5i32);
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(2i32);
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(1i32);
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(1i32);
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(3i32);
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(1i32);
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(2i32);
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(3i32);
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
pub const GNSS_ResetEngine: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(9i32);
pub const GNSS_ResetGeofencesTracking: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(16i32);
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
pub const GNSS_SetLocationNIRequestAllowed: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(2i32);
pub const GNSS_SetLocationServiceEnabled: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(1i32);
pub const GNSS_SetNMEALogging: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(13i32);
pub const GNSS_SetNiTimeoutInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(15i32);
pub const GNSS_SetSuplVersion: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(12i32);
pub const GNSS_SetSuplVersion2: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(17i32);
pub const GNSS_SetUplServerAccessInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(14i32);
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(2i32);
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(1i32);
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(3i32);
pub const GUID_DEVINTERFACE_GNSS: windows_core::GUID = windows_core::GUID::from_u128(0x3336e5e4_018a_4669_84c5_bd05f3bd368b);
pub const IOCTL_GNSS_CONFIG_SUPL_CERT: u32 = 2228488u32;
pub const IOCTL_GNSS_CREATE_GEOFENCE: u32 = 2228544u32;
pub const IOCTL_GNSS_DELETE_GEOFENCE: u32 = 2228548u32;
pub const IOCTL_GNSS_EXECUTE_CWTEST: u32 = 2228496u32;
pub const IOCTL_GNSS_EXECUTE_SELFTEST: u32 = 2228500u32;
pub const IOCTL_GNSS_GET_CHIPSETINFO: u32 = 2228504u32;
pub const IOCTL_GNSS_GET_DEVICE_CAPABILITY: u32 = 2228232u32;
pub const IOCTL_GNSS_GET_FIXDATA: u32 = 2228300u32;
pub const IOCTL_GNSS_INJECT_AGNSS: u32 = 2228352u32;
pub const IOCTL_GNSS_LISTEN_AGNSS: u32 = 2228416u32;
pub const IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT: u32 = 2228680u32;
pub const IOCTL_GNSS_LISTEN_DRIVER_REQUEST: u32 = 2228608u32;
pub const IOCTL_GNSS_LISTEN_ERROR: u32 = 2228420u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS: u32 = 2228556u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCE_ALERT: u32 = 2228552u32;
pub const IOCTL_GNSS_LISTEN_NI: u32 = 2228480u32;
pub const IOCTL_GNSS_LISTEN_NMEA: u32 = 2228508u32;
pub const IOCTL_GNSS_MODIFY_FIXSESSION: u32 = 2228292u32;
pub const IOCTL_GNSS_POP_BREADCRUMBS: u32 = 2228684u32;
pub const IOCTL_GNSS_RESPOND_NI: u32 = 2228492u32;
pub const IOCTL_GNSS_SEND_DRIVERCOMMAND: u32 = 2228236u32;
pub const IOCTL_GNSS_SEND_PLATFORM_CAPABILITY: u32 = 2228228u32;
pub const IOCTL_GNSS_SET_SUPL_HSLP: u32 = 2228484u32;
pub const IOCTL_GNSS_SET_V2UPL_CONFIG: u32 = 2228512u32;
pub const IOCTL_GNSS_START_BREADCRUMBING: u32 = 2228672u32;
pub const IOCTL_GNSS_START_FIXSESSION: u32 = 2228288u32;
pub const IOCTL_GNSS_STOP_BREADCRUMBING: u32 = 2228676u32;
pub const IOCTL_GNSS_STOP_FIXSESSION: u32 = 2228296u32;
pub const LOCATION_API_VERSION: u32 = 1u32;
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(2i32);
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(1i32);
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(3i32);
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(0i32);
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(4i32);
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = GNSS_DRIVER_REQUEST(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_AGNSS_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_AGNSS_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_AGNSS_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_AGNSS_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_DRIVERCOMMAND_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_DRIVERCOMMAND_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_DRIVERCOMMAND_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_DRIVERCOMMAND_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_DRIVER_REQUEST(pub i32);
impl windows_core::TypeKind for GNSS_DRIVER_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_DRIVER_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_DRIVER_REQUEST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_FIXSESSIONTYPE(pub i32);
impl windows_core::TypeKind for GNSS_FIXSESSIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_FIXSESSIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_FIXSESSIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_GEOFENCE_STATE(pub i32);
impl windows_core::TypeKind for GNSS_GEOFENCE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_GEOFENCE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_GEOFENCE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_GEOREGIONTYPE(pub i32);
impl windows_core::TypeKind for GNSS_GEOREGIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_GEOREGIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_GEOREGIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_NI_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_NI_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_NI_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_NI_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_NI_PLANE_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_NI_PLANE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_NI_PLANE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_NI_PLANE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_NI_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for GNSS_NI_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_NI_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_NI_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_NI_USER_RESPONSE(pub i32);
impl windows_core::TypeKind for GNSS_NI_USER_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_NI_USER_RESPONSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_NI_USER_RESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GNSS_SUPL_CERT_ACTION(pub i32);
impl windows_core::TypeKind for GNSS_SUPL_CERT_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GNSS_SUPL_CERT_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GNSS_SUPL_CERT_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCATION_REPORT_STATUS(pub i32);
impl windows_core::TypeKind for LOCATION_REPORT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCATION_REPORT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCATION_REPORT_STATUS").field(&self.0).finish()
    }
}
pub const CivicAddressReport: windows_core::GUID = windows_core::GUID::from_u128(0xd39e7bdd_7d05_46b8_8721_80cf035f57d7);
pub const CivicAddressReportFactory: windows_core::GUID = windows_core::GUID::from_u128(0x2a11f42c_3e81_4ad4_9cbe_45579d89671a);
pub const DefaultLocation: windows_core::GUID = windows_core::GUID::from_u128(0x8b7fbfe0_5cd7_494a_af8c_283a65707506);
pub const DispCivicAddressReport: windows_core::GUID = windows_core::GUID::from_u128(0x4c596aec_8544_4082_ba9f_eb0a7d8e65c6);
pub const DispLatLongReport: windows_core::GUID = windows_core::GUID::from_u128(0x7a7c3277_8f84_4636_95b2_ebb5507ff77e);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_AGNSS_INJECT {
    pub Size: u32,
    pub Version: u32,
    pub InjectionType: GNSS_AGNSS_REQUEST_TYPE,
    pub InjectionStatus: super::super::Foundation::NTSTATUS,
    pub InjectionDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_AGNSS_INJECT_0,
}
impl windows_core::TypeKind for GNSS_AGNSS_INJECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_INJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_AGNSS_INJECT_0 {
    pub Time: GNSS_AGNSS_INJECTTIME,
    pub Position: GNSS_AGNSS_INJECTPOSITION,
    pub BlobData: GNSS_AGNSS_INJECTBLOB,
}
impl windows_core::TypeKind for GNSS_AGNSS_INJECT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_INJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_AGNSS_INJECTBLOB {
    pub Size: u32,
    pub Version: u32,
    pub BlobOui: u32,
    pub BlobVersion: u32,
    pub AgnssFormat: u32,
    pub BlobSize: u32,
    pub BlobData: [u8; 1],
}
impl windows_core::TypeKind for GNSS_AGNSS_INJECTBLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_INJECTBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_AGNSS_INJECTPOSITION {
    pub Size: u32,
    pub Version: u32,
    pub Age: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
}
impl windows_core::TypeKind for GNSS_AGNSS_INJECTPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_INJECTPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_AGNSS_INJECTTIME {
    pub Size: u32,
    pub Version: u32,
    pub UtcTime: super::super::Foundation::FILETIME,
    pub TimeUncertainty: u32,
}
impl windows_core::TypeKind for GNSS_AGNSS_INJECTTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_INJECTTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_AGNSS_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestType: GNSS_AGNSS_REQUEST_TYPE,
    pub BlobFormat: u32,
}
impl windows_core::TypeKind for GNSS_AGNSS_REQUEST_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_AGNSS_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_BREADCRUMBING_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_BREADCRUMBING_ALERT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_BREADCRUMBING_ALERT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_BREADCRUMBING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MaximumHorizontalUncertainty: u32,
    pub MinDistanceBetweenFixes: u32,
    pub MaximumErrorTimeoutMs: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_BREADCRUMBING_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_BREADCRUMBING_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_BREADCRUMB_LIST {
    pub Size: u32,
    pub Version: u32,
    pub NumCrumbs: u32,
    pub Anonymous: GNSS_BREADCRUMB_LIST_0,
}
impl windows_core::TypeKind for GNSS_BREADCRUMB_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_BREADCRUMB_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_BREADCRUMB_LIST_0 {
    pub v1: [GNSS_BREADCRUMB_V1; 50],
}
impl windows_core::TypeKind for GNSS_BREADCRUMB_LIST_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_BREADCRUMB_LIST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_BREADCRUMB_V1 {
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub Latitude: f64,
    pub Longitude: f64,
    pub HorizontalAccuracy: u32,
    pub Speed: u16,
    pub SpeedAccuracy: u16,
    pub Altitude: i16,
    pub AltitudeAccuracy: u16,
    pub Heading: i16,
    pub HeadingAccuracy: u8,
    pub FixSuccess: u8,
}
impl windows_core::TypeKind for GNSS_BREADCRUMB_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_BREADCRUMB_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_CHIPSETINFO {
    pub Size: u32,
    pub Version: u32,
    pub ManufacturerID: [u16; 25],
    pub HardwareID: [u16; 25],
    pub FirmwareVersion: [u16; 20],
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_CHIPSETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_CHIPSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_CONTINUOUSTRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub PreferredInterval: u32,
}
impl windows_core::TypeKind for GNSS_CONTINUOUSTRACKING_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_CONTINUOUSTRACKING_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_CP_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub NotificationText: [u16; 260],
}
impl windows_core::TypeKind for GNSS_CP_NI_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_CP_NI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_CWTESTDATA {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub SignalToNoiseRatio: f64,
    pub Frequency: f64,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_CWTESTDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_CWTESTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_DEVICE_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportMultipleFixSessions: super::super::Foundation::BOOL,
    pub SupportMultipleAppSessions: super::super::Foundation::BOOL,
    pub RequireAGnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub AgnssFormatPreferred: u32,
    pub SupportDistanceTracking: super::super::Foundation::BOOL,
    pub SupportContinuousTracking: super::super::Foundation::BOOL,
    pub Reserved1: u32,
    pub Reserved2: super::super::Foundation::BOOL,
    pub Reserved3: super::super::Foundation::BOOL,
    pub Reserved4: super::super::Foundation::BOOL,
    pub Reserved5: super::super::Foundation::BOOL,
    pub GeofencingSupport: u32,
    pub Reserved6: super::super::Foundation::BOOL,
    pub Reserved7: super::super::Foundation::BOOL,
    pub SupportCpLocation: super::super::Foundation::BOOL,
    pub SupportUplV2: super::super::Foundation::BOOL,
    pub SupportSuplV1: super::super::Foundation::BOOL,
    pub SupportSuplV2: super::super::Foundation::BOOL,
    pub SupportedSuplVersion: GNSS_SUPL_VERSION,
    pub MaxGeofencesSupported: u32,
    pub SupportMultipleSuplRootCert: super::super::Foundation::BOOL,
    pub GnssBreadCrumbPayloadVersion: u32,
    pub MaxGnssBreadCrumbFixes: u32,
    pub Unused: [u8; 496],
}
impl windows_core::TypeKind for GNSS_DEVICE_CAPABILITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_DEVICE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_DISTANCETRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MovementThreshold: u32,
}
impl windows_core::TypeKind for GNSS_DISTANCETRACKING_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_DISTANCETRACKING_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_DRIVERCOMMAND_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub CommandType: GNSS_DRIVERCOMMAND_TYPE,
    pub Reserved: u32,
    pub CommandDataSize: u32,
    pub Unused: [u8; 512],
    pub CommandData: [u8; 1],
}
impl windows_core::TypeKind for GNSS_DRIVERCOMMAND_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_DRIVERCOMMAND_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_DRIVER_REQUEST_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Request: GNSS_DRIVER_REQUEST,
    pub RequestFlag: u32,
}
impl windows_core::TypeKind for GNSS_DRIVER_REQUEST_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_DRIVER_REQUEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_ERRORINFO {
    pub Size: u32,
    pub Version: u32,
    pub ErrorCode: u32,
    pub IsRecoverable: super::super::Foundation::BOOL,
    pub ErrorDescription: [u16; 256],
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_ERRORINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_ERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_EVENT {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_0,
}
impl windows_core::TypeKind for GNSS_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_EVENT_0 {
    pub FixData: GNSS_FIXDATA,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
impl windows_core::TypeKind for GNSS_EVENT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_EVENT_2 {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_2_0,
}
impl windows_core::TypeKind for GNSS_EVENT_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_EVENT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_EVENT_2_0 {
    pub FixData: GNSS_FIXDATA,
    pub FixData2: GNSS_FIXDATA_2,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
impl windows_core::TypeKind for GNSS_EVENT_2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_EVENT_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
impl windows_core::TypeKind for GNSS_FIXDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_2 {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC_2,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY_2,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
impl windows_core::TypeKind for GNSS_FIXDATA_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_ACCURACY {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: u32,
    pub HorizontalErrorMajorAxis: u32,
    pub HorizontalErrorMinorAxis: u32,
    pub HorizontalErrorAngle: u32,
    pub HeadingAccuracy: u32,
    pub AltitudeAccuracy: u32,
    pub SpeedAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f32,
    pub HorizontalDilutionOfPrecision: f32,
    pub VerticalDilutionOfPrecision: f32,
}
impl windows_core::TypeKind for GNSS_FIXDATA_ACCURACY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_ACCURACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_ACCURACY_2 {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: f64,
    pub HorizontalErrorMajorAxis: f64,
    pub HorizontalErrorMinorAxis: f64,
    pub HorizontalErrorAngle: f64,
    pub HeadingAccuracy: f64,
    pub AltitudeAccuracy: f64,
    pub SpeedAccuracy: f64,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f64,
    pub HorizontalDilutionOfPrecision: f64,
    pub VerticalDilutionOfPrecision: f64,
    pub GeometricDilutionOfPrecision: f64,
    pub TimeDilutionOfPrecision: f64,
}
impl windows_core::TypeKind for GNSS_FIXDATA_ACCURACY_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_ACCURACY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_BASIC {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
}
impl windows_core::TypeKind for GNSS_FIXDATA_BASIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_BASIC_2 {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
    pub AltitudeEllipsoid: f64,
}
impl windows_core::TypeKind for GNSS_FIXDATA_BASIC_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_BASIC_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_FIXDATA_SATELLITE {
    pub Size: u32,
    pub Version: u32,
    pub SatelliteCount: u32,
    pub SatelliteArray: [GNSS_SATELLITEINFO; 64],
}
impl windows_core::TypeKind for GNSS_FIXDATA_SATELLITE {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXDATA_SATELLITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_FIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub SessionType: GNSS_FIXSESSIONTYPE,
    pub HorizontalAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub Reserved: [u32; 9],
    pub FixLevelOfDetails: u32,
    pub Anonymous: GNSS_FIXSESSION_PARAM_0,
    pub Unused: [u8; 256],
}
impl windows_core::TypeKind for GNSS_FIXSESSION_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_FIXSESSION_PARAM_0 {
    pub SingleShotParam: GNSS_SINGLESHOT_PARAM,
    pub DistanceParam: GNSS_DISTANCETRACKING_PARAM,
    pub ContinuousParam: GNSS_CONTINUOUSTRACKING_PARAM,
    pub LkgFixParam: GNSS_LKGFIX_PARAM,
    pub UnusedParam: [u8; 268],
}
impl windows_core::TypeKind for GNSS_FIXSESSION_PARAM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_FIXSESSION_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Status: super::super::Foundation::NTSTATUS,
    pub StatusTimeStamp: super::super::Foundation::FILETIME,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_GEOFENCE_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub GeofenceState: GNSS_GEOFENCE_STATE,
    pub FixBasicData: GNSS_FIXDATA_BASIC,
    pub FixAccuracyData: GNSS_FIXDATA_ACCURACY,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOFENCE_ALERT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOFENCE_ALERT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_GEOFENCE_CREATE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub AlertTypes: u32,
    pub InitialState: GNSS_GEOFENCE_STATE,
    pub Boundary: GNSS_GEOREGION,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOFENCE_CREATE_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOFENCE_CREATE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub CreationStatus: super::super::Foundation::NTSTATUS,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOFENCE_CREATE_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_GEOFENCE_DELETE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOFENCE_DELETE_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOFENCE_DELETE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_GEOREGION {
    pub Size: u32,
    pub Version: u32,
    pub GeoRegionType: GNSS_GEOREGIONTYPE,
    pub Anonymous: GNSS_GEOREGION_0,
}
impl windows_core::TypeKind for GNSS_GEOREGION {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOREGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_GEOREGION_0 {
    pub Circle: GNSS_GEOREGION_CIRCLE,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_GEOREGION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOREGION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_GEOREGION_CIRCLE {
    pub Latitude: f64,
    pub Longitude: f64,
    pub RadiusInMeters: f64,
}
impl windows_core::TypeKind for GNSS_GEOREGION_CIRCLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_GEOREGION_CIRCLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_LKGFIX_PARAM {
    pub Size: u32,
    pub Version: u32,
}
impl windows_core::TypeKind for GNSS_LKGFIX_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_LKGFIX_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GNSS_NI_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub RequestType: GNSS_NI_REQUEST_TYPE,
    pub NotificationType: GNSS_NI_NOTIFICATION_TYPE,
    pub RequestPlaneType: GNSS_NI_PLANE_TYPE,
    pub Anonymous: GNSS_NI_REQUEST_PARAM_0,
    pub ResponseTimeInSec: u32,
    pub EmergencyLocation: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for GNSS_NI_REQUEST_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_NI_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GNSS_NI_REQUEST_PARAM_0 {
    pub SuplNiInfo: GNSS_SUPL_NI_INFO,
    pub CpNiInfo: GNSS_CP_NI_INFO,
    pub V2UplNiInfo: GNSS_V2UPL_NI_INFO,
}
impl windows_core::TypeKind for GNSS_NI_REQUEST_PARAM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_NI_REQUEST_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_NI_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub UserResponse: GNSS_NI_USER_RESPONSE,
}
impl windows_core::TypeKind for GNSS_NI_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_NI_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_NMEA_DATA {
    pub Size: u32,
    pub Version: u32,
    pub NmeaSentences: [i8; 256],
}
impl windows_core::TypeKind for GNSS_NMEA_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_NMEA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_PLATFORM_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportAgnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub Unused: [u8; 516],
}
impl windows_core::TypeKind for GNSS_PLATFORM_CAPABILITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_PLATFORM_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GNSS_SATELLITEINFO {
    pub SatelliteId: u32,
    pub UsedInPositiong: super::super::Foundation::BOOL,
    pub Elevation: f64,
    pub Azimuth: f64,
    pub SignalToNoiseRatio: f64,
}
impl windows_core::TypeKind for GNSS_SATELLITEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SATELLITEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SELFTESTCONFIG {
    pub Size: u32,
    pub Version: u32,
    pub TestType: u32,
    pub Unused: [u8; 512],
    pub InBufLen: u32,
    pub InBuffer: [u8; 1],
}
impl windows_core::TypeKind for GNSS_SELFTESTCONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SELFTESTCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SELFTESTRESULT {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub Result: u32,
    pub PinFailedBitMask: u32,
    pub Unused: [u8; 512],
    pub OutBufLen: u32,
    pub OutBuffer: [u8; 1],
}
impl windows_core::TypeKind for GNSS_SELFTESTRESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SELFTESTRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SINGLESHOT_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub ResponseTime: u32,
}
impl windows_core::TypeKind for GNSS_SINGLESHOT_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SINGLESHOT_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_STOPFIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_STOPFIXSESSION_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_STOPFIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SUPL_CERT_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub CertAction: GNSS_SUPL_CERT_ACTION,
    pub SuplCertName: [i8; 260],
    pub CertSize: u32,
    pub Unused: [u8; 512],
    pub CertData: [u8; 1],
}
impl windows_core::TypeKind for GNSS_SUPL_CERT_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SUPL_CERT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SUPL_HSLP_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub SuplHslp: [i8; 260],
    pub SuplHslpFromImsi: [i8; 260],
    pub Reserved: u32,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_SUPL_HSLP_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SUPL_HSLP_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SUPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub ClientName: [u16; 260],
    pub SuplNiUrl: [i8; 260],
}
impl windows_core::TypeKind for GNSS_SUPL_NI_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SUPL_NI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SUPL_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl windows_core::TypeKind for GNSS_SUPL_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SUPL_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_SUPL_VERSION_2 {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ServiceIndicator: u32,
}
impl windows_core::TypeKind for GNSS_SUPL_VERSION_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_SUPL_VERSION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_V2UPL_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub MPC: [i8; 260],
    pub PDE: [i8; 260],
    pub ApplicationTypeIndicator_MR: u8,
    pub Unused: [u8; 512],
}
impl windows_core::TypeKind for GNSS_V2UPL_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_V2UPL_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GNSS_V2UPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
}
impl windows_core::TypeKind for GNSS_V2UPL_NI_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GNSS_V2UPL_NI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LatLongReport: windows_core::GUID = windows_core::GUID::from_u128(0xed81c073_1f84_4ca8_a161_183c776bc651);
pub const LatLongReportFactory: windows_core::GUID = windows_core::GUID::from_u128(0x9dcc3cc8_8609_4863_bad4_03601f4c65e8);
pub const Location: windows_core::GUID = windows_core::GUID::from_u128(0xe5b8e079_ee6d_4e33_a438_c87f2e959254);
#[cfg(feature = "implement")]
core::include!("impl.rs");
