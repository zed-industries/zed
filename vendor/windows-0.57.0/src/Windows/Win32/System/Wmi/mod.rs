#[inline]
pub unsafe fn MI_Application_InitializeV1(flags: u32, applicationid: Option<*const u16>, extendederror: Option<*mut *mut MI_Instance>, application: *mut MI_Application) -> MI_Result {
    windows_targets::link!("mi.dll" "cdecl" fn MI_Application_InitializeV1(flags : u32, applicationid : *const u16, extendederror : *mut *mut MI_Instance, application : *mut MI_Application) -> MI_Result);
    MI_Application_InitializeV1(flags, core::mem::transmute(applicationid.unwrap_or(std::ptr::null())), core::mem::transmute(extendederror.unwrap_or(std::ptr::null_mut())), application)
}
windows_core::imp::define_interface!(IEnumWbemClassObject, IEnumWbemClassObject_Vtbl, 0x027947e1_d731_11ce_a357_000000000001);
impl core::ops::Deref for IEnumWbemClassObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumWbemClassObject, windows_core::IUnknown);
impl IEnumWbemClassObject {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self, ltimeout: i32, apobjects: &mut [Option<IWbemClassObject>], pureturned: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ltimeout, apobjects.len().try_into().unwrap(), core::mem::transmute(apobjects.as_ptr()), pureturned)
    }
    pub unsafe fn NextAsync<P0>(&self, ucount: u32, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).NextAsync)(windows_core::Interface::as_raw(self), ucount, psink.param().abi())
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ltimeout, ncount)
    }
}
#[repr(C)]
pub struct IEnumWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMofCompiler, IMofCompiler_Vtbl, 0x6daf974e_2e37_11d2_aec9_00c04fb68820);
impl core::ops::Deref for IMofCompiler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMofCompiler, windows_core::IUnknown);
impl IMofCompiler {
    pub unsafe fn CompileFile<P0, P1, P2, P3, P4>(&self, filename: P0, serverandnamespace: P1, user: P2, authority: P3, password: P4, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CompileFile)(windows_core::Interface::as_raw(self), filename.param().abi(), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
    pub unsafe fn CompileBuffer<P0, P1, P2, P3>(&self, pbuffer: &[u8], serverandnamespace: P0, user: P1, authority: P2, password: P3, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CompileBuffer)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
    pub unsafe fn CreateBMOF<P0, P1, P2>(&self, textfilename: P0, bmoffilename: P1, serverandnamespace: P2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateBMOF)(windows_core::Interface::as_raw(self), textfilename.param().abi(), bmoffilename.param().abi(), serverandnamespace.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
}
#[repr(C)]
pub struct IMofCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompileFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemDateTime, ISWbemDateTime_Vtbl, 0x5e97458a_cf77_11d3_b38f_00105a1f473a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemDateTime {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemDateTime, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemDateTime {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, strvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), strvalue.param().abi()).ok()
    }
    pub unsafe fn Year(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Year)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetYear(&self, iyear: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetYear)(windows_core::Interface::as_raw(self), iyear).ok()
    }
    pub unsafe fn YearSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).YearSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetYearSpecified<P0>(&self, byearspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetYearSpecified)(windows_core::Interface::as_raw(self), byearspecified.param().abi()).ok()
    }
    pub unsafe fn Month(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Month)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMonth(&self, imonth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMonth)(windows_core::Interface::as_raw(self), imonth).ok()
    }
    pub unsafe fn MonthSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MonthSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMonthSpecified<P0>(&self, bmonthspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMonthSpecified)(windows_core::Interface::as_raw(self), bmonthspecified.param().abi()).ok()
    }
    pub unsafe fn Day(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Day)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDay(&self, iday: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDay)(windows_core::Interface::as_raw(self), iday).ok()
    }
    pub unsafe fn DaySpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DaySpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDaySpecified<P0>(&self, bdayspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDaySpecified)(windows_core::Interface::as_raw(self), bdayspecified.param().abi()).ok()
    }
    pub unsafe fn Hours(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hours)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHours(&self, ihours: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHours)(windows_core::Interface::as_raw(self), ihours).ok()
    }
    pub unsafe fn HoursSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HoursSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHoursSpecified<P0>(&self, bhoursspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHoursSpecified)(windows_core::Interface::as_raw(self), bhoursspecified.param().abi()).ok()
    }
    pub unsafe fn Minutes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Minutes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinutes(&self, iminutes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinutes)(windows_core::Interface::as_raw(self), iminutes).ok()
    }
    pub unsafe fn MinutesSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinutesSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinutesSpecified<P0>(&self, bminutesspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMinutesSpecified)(windows_core::Interface::as_raw(self), bminutesspecified.param().abi()).ok()
    }
    pub unsafe fn Seconds(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Seconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSeconds(&self, iseconds: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSeconds)(windows_core::Interface::as_raw(self), iseconds).ok()
    }
    pub unsafe fn SecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSecondsSpecified<P0>(&self, bsecondsspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSecondsSpecified)(windows_core::Interface::as_raw(self), bsecondsspecified.param().abi()).ok()
    }
    pub unsafe fn Microseconds(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Microseconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMicroseconds(&self, imicroseconds: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMicroseconds)(windows_core::Interface::as_raw(self), imicroseconds).ok()
    }
    pub unsafe fn MicrosecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MicrosecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMicrosecondsSpecified<P0>(&self, bmicrosecondsspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMicrosecondsSpecified)(windows_core::Interface::as_raw(self), bmicrosecondsspecified.param().abi()).ok()
    }
    pub unsafe fn UTC(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UTC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUTC(&self, iutc: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUTC)(windows_core::Interface::as_raw(self), iutc).ok()
    }
    pub unsafe fn UTCSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UTCSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUTCSpecified<P0>(&self, butcspecified: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUTCSpecified)(windows_core::Interface::as_raw(self), butcspecified.param().abi()).ok()
    }
    pub unsafe fn IsInterval(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsInterval<P0>(&self, bisinterval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsInterval)(windows_core::Interface::as_raw(self), bisinterval.param().abi()).ok()
    }
    pub unsafe fn GetVarDate<P0>(&self, bislocal: P0) -> windows_core::Result<f64>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVarDate)(windows_core::Interface::as_raw(self), bislocal.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVarDate<P0>(&self, dvardate: f64, bislocal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVarDate)(windows_core::Interface::as_raw(self), dvardate, bislocal.param().abi()).ok()
    }
    pub unsafe fn GetFileTime<P0>(&self, bislocal: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileTime)(windows_core::Interface::as_raw(self), bislocal.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileTime<P0, P1>(&self, strfiletime: P0, bislocal: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFileTime)(windows_core::Interface::as_raw(self), strfiletime.param().abi(), bislocal.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemDateTime_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Year: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetYear: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub YearSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetYearSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MonthSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMonthSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaySpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDaySpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Hours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHours: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HoursSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHoursSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Minutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinutes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinutesSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMinutesSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Seconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Microseconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMicroseconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MicrosecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMicrosecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UTC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUTC: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UTCSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUTCSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetVarDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut f64) -> windows_core::HRESULT,
    pub SetVarDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemEventSource, ISWbemEventSource_Vtbl, 0x27d54d92_0ebe_11d2_8b22_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemEventSource {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemEventSource, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemEventSource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextEvent)(windows_core::Interface::as_raw(self), itimeoutms, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemEventSource_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NextEvent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemLastError, ISWbemLastError_Vtbl, 0xd962db84_d4bb_11d1_8b09_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemLastError {
    type Target = ISWbemObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemLastError, windows_core::IUnknown, super::Com::IDispatch, ISWbemObject);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLastError {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemLastError_Vtbl {
    pub base__: ISWbemObject_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemLocator, ISWbemLocator_Vtbl, 0x76a6415b_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLocator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5, P6>(&self, strserver: P0, strnamespace: P1, struser: P2, strpassword: P3, strlocale: P4, strauthority: P5, isecurityflags: i32, objwbemnamedvalueset: P6) -> windows_core::Result<ISWbemServices>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), strserver.param().abi(), strnamespace.param().abi(), struser.param().abi(), strpassword.param().abi(), strlocale.param().abi(), strauthority.param().abi(), isecurityflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConnectServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemMethod, ISWbemMethod_Vtbl, 0x422e8e90_d955_11d1_8b09_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemMethod {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemMethod, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethod {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Origin)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InParameters(&self) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutParameters(&self) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OutParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemMethod_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemMethodSet, ISWbemMethodSet_Vtbl, 0xc93ba292_d955_11d1_8b09_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemMethodSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemMethodSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethodSet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<ISWbemMethod>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemMethodSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemNamedValue, ISWbemNamedValue_Vtbl, 0x76a64164_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemNamedValue {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemNamedValue, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValue {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemNamedValue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemNamedValueSet, ISWbemNamedValueSet_Vtbl, 0xcf2376ea_ce8c_11d1_8b05_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemNamedValueSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemNamedValueSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValueSet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<ISWbemNamedValue>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, strname: P0, varvalue: *const windows_core::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), strname.param().abi(), core::mem::transmute(varvalue), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemNamedValueSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemObject, ISWbemObject_Vtbl, 0x76a6415a_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemObject {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemObject, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObject {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<ISWbemObjectPath>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Put_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).PutAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Delete_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).DeleteAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Instances_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).InstancesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Subclasses_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SubclassesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Associators_<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, strassocclass: P0, strresultclass: P1, strresultrole: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredassocqualifier: P6, strrequiredqualifier: P7, iflags: i32, objwbemnamedvalueset: P8) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P5: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P6: windows_core::Param<windows_core::BSTR>,
        P7: windows_core::Param<windows_core::BSTR>,
        P8: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Associators_)(windows_core::Interface::as_raw(self), strassocclass.param().abi(), strresultclass.param().abi(), strresultrole.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredassocqualifier.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>(&self, objwbemsink: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9, objwbemasynccontext: P10) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P6: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P7: windows_core::Param<windows_core::BSTR>,
        P8: windows_core::Param<windows_core::BSTR>,
        P9: windows_core::Param<super::Com::IDispatch>,
        P10: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).AssociatorsAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strassocclass.param().abi(), strresultclass.param().abi(), strresultrole.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredassocqualifier.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn References_<P0, P1, P2, P3, P4, P5>(&self, strresultclass: P0, strrole: P1, bclassesonly: P2, bschemaonly: P3, strrequiredqualifier: P4, iflags: i32, objwbemnamedvalueset: P5) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).References_)(windows_core::Interface::as_raw(self), strresultclass.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4, P5, P6, P7>(&self, objwbemsink: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6, objwbemasynccontext: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<super::Com::IDispatch>,
        P7: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ReferencesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strresultclass.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1, P2>(&self, strmethodname: P0, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecMethod_)(windows_core::Interface::as_raw(self), strmethodname.param().abi(), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ExecMethodAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strmethodname.param().abi(), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectText_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpawnDerivedClass_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpawnInstance_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareTo_)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> windows_core::Result<ISWbemMethodSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Methods_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Derivation_(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Derivation_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> windows_core::Result<ISWbemObjectPath> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Put_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Delete_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Delete_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Instances_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Instances_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subclasses_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subclasses_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Associators_: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Associators_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AssociatorsAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssociatorsAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub References_: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    References_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReferencesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReferencesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethod_: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethod_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethodAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethodAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone_: usize,
    pub GetObjectText_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnDerivedClass_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnDerivedClass_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnInstance_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnInstance_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareTo_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareTo_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Methods_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Methods_: usize,
    pub Derivation_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Path_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Path_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemObjectEx, ISWbemObjectEx_Vtbl, 0x269ad56a_8a67_4129_bc8c_0506dcfe9880);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemObjectEx {
    type Target = ISWbemObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemObjectEx, windows_core::IUnknown, super::Com::IDispatch, ISWbemObject);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectEx {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Refresh_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Refresh_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SystemProperties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetText_<P0>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText_)(windows_core::Interface::as_raw(self), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFromText_<P0, P1>(&self, bstext: P0, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetFromText_)(windows_core::Interface::as_raw(self), bstext.param().abi(), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectEx_Vtbl {
    pub base__: ISWbemObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresh_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresh_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemProperties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemProperties_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetText_: unsafe extern "system" fn(*mut core::ffi::c_void, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetText_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFromText_: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFromText_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemObjectPath, ISWbemObjectPath_Vtbl, 0x5791bc27_ce9c_11d1_97bf_0000f81e849c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemObjectPath {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemObjectPath, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectPath {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPath<P0>(&self, strpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), strpath.param().abi()).ok()
    }
    pub unsafe fn RelPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RelPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRelPath<P0>(&self, strrelpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRelPath)(windows_core::Interface::as_raw(self), strrelpath.param().abi()).ok()
    }
    pub unsafe fn Server(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServer<P0>(&self, strserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), strserver.param().abi()).ok()
    }
    pub unsafe fn Namespace(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Namespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNamespace<P0>(&self, strnamespace: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNamespace)(windows_core::Interface::as_raw(self), strnamespace.param().abi()).ok()
    }
    pub unsafe fn ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentNamespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, strdisplayname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), strdisplayname.param().abi()).ok()
    }
    pub unsafe fn Class(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClass<P0>(&self, strclass: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), strclass.param().abi()).ok()
    }
    pub unsafe fn IsClass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAsClass(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAsClass)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsSingleton(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSingleton)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAsSingleton(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAsSingleton)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keys(&self) -> windows_core::Result<ISWbemNamedValueSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Keys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Locale(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Locale)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocale<P0>(&self, strlocale: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), strlocale.param().abi()).ok()
    }
    pub unsafe fn Authority(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Authority)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuthority<P0>(&self, strauthority: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthority)(windows_core::Interface::as_raw(self), strauthority.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectPath_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RelPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRelPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ParentNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAsClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Keys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    pub Locale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Authority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemObjectSet, ISWbemObjectSet_Vtbl, 0x76a6415f_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemObjectSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemObjectSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectSet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strobjectpath: P0, iflags: i32) -> windows_core::Result<ISWbemObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ItemIndex)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemObjectSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemIndex: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemPrivilege, ISWbemPrivilege_Vtbl, 0x26ee67bd_5804_11d2_8b4a_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemPrivilege {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemPrivilege, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilege {
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsEnabled<P0>(&self, bisenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsEnabled)(windows_core::Interface::as_raw(self), bisenabled.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPrivilege_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemPrivilegeEnum) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemPrivilegeSet, ISWbemPrivilegeSet_Vtbl, 0x26ee67bf_5804_11d2_8b4a_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemPrivilegeSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemPrivilegeSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilegeSet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iprivilege, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, iprivilege: WbemPrivilegeEnum, bisenabled: P0) -> windows_core::Result<ISWbemPrivilege>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), iprivilege, bisenabled.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iprivilege).ok()
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAsString<P0, P1>(&self, strprivilege: P0, bisenabled: P1) -> windows_core::Result<ISWbemPrivilege>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddAsString)(windows_core::Interface::as_raw(self), strprivilege.param().abi(), bisenabled.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPrivilegeSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAsString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAsString: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemProperty, ISWbemProperty_Vtbl, 0x1a388f98_d4ba_11d1_8b09_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemProperty {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemProperty, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemProperty {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Origin)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CIMType(&self) -> windows_core::Result<WbemCimtypeEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CIMType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsArray(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CIMType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemCimtypeEnum) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    pub IsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemPropertySet, ISWbemPropertySet_Vtbl, 0xdea0a7b2_d4ba_11d1_8b09_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemPropertySet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemPropertySet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPropertySet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<ISWbemProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1>(&self, strname: P0, icimtype: WbemCimtypeEnum, bisarray: P1, iflags: i32) -> windows_core::Result<ISWbemProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), strname.param().abi(), icimtype, bisarray.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemPropertySet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WbemCimtypeEnum, super::super::Foundation::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemQualifier, ISWbemQualifier_Vtbl, 0x79b05932_d3b7_11d1_8b06_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemQualifier {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemQualifier, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifier {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PropagatesToSubclass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropagatesToSubclass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPropagatesToSubclass<P0>(&self, bpropagatestosubclass: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPropagatesToSubclass)(windows_core::Interface::as_raw(self), bpropagatestosubclass.param().abi()).ok()
    }
    pub unsafe fn PropagatesToInstance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropagatesToInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPropagatesToInstance<P0>(&self, bpropagatestoinstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPropagatesToInstance)(windows_core::Interface::as_raw(self), bpropagatestoinstance.param().abi()).ok()
    }
    pub unsafe fn IsOverridable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOverridable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsOverridable<P0>(&self, bisoverridable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsOverridable)(windows_core::Interface::as_raw(self), bisoverridable.param().abi()).ok()
    }
    pub unsafe fn IsAmended(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAmended)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemQualifier_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsAmended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemQualifierSet, ISWbemQualifierSet_Vtbl, 0x9b16ed16_d3df_11d1_8b08_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemQualifierSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemQualifierSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifierSet {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0, iflags: i32) -> windows_core::Result<ISWbemQualifier>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), name.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2, P3>(&self, strname: P0, varval: *const windows_core::VARIANT, bpropagatestosubclass: P1, bpropagatestoinstance: P2, bisoverridable: P3, iflags: i32) -> windows_core::Result<ISWbemQualifier>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), strname.param().abi(), core::mem::transmute(varval), bpropagatestosubclass.param().abi(), bpropagatestoinstance.param().abi(), bisoverridable.param().abi(), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), strname.param().abi(), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemQualifierSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemRefreshableItem, ISWbemRefreshableItem_Vtbl, 0x5ad4bf92_daab_11d3_b38f_00105a1f473a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemRefreshableItem {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemRefreshableItem, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefreshableItem {
    pub unsafe fn Index(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Refresher(&self) -> windows_core::Result<ISWbemRefresher> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Refresher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Object(&self) -> windows_core::Result<ISWbemObjectEx> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Object)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove(&self, iflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemRefreshableItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresher: usize,
    pub IsSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Object: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Object: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectSet: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemRefresher, ISWbemRefresher_Vtbl, 0x14d8250e_d9c2_11d3_b38f_00105a1f473a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemRefresher {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemRefresher, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefresher {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2>(&self, objwbemservices: P0, bsinstancepath: P1, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), bsinstancepath.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddEnum<P0, P1, P2>(&self, objwbemservices: P0, bsclassname: P1, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), bsclassname.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iindex, iflags).ok()
    }
    pub unsafe fn Refresh(&self, iflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), iflags).ok()
    }
    pub unsafe fn AutoReconnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoReconnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoReconnect<P0>(&self, bcount: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoReconnect)(windows_core::Interface::as_raw(self), bcount.param().abi()).ok()
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemRefresher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddEnum: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemSecurity, ISWbemSecurity_Vtbl, 0xb54d66e6_2287_11d2_8b33_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemSecurity {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemSecurity, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSecurity {
    pub unsafe fn ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImpersonationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetImpersonationLevel)(windows_core::Interface::as_raw(self), iimpersonationlevel).ok()
    }
    pub unsafe fn AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationLevel)(windows_core::Interface::as_raw(self), iauthenticationlevel).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Privileges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSecurity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub SetImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Privileges: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemServices, ISWbemServices_Vtbl, 0x76a6415c_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemServices {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemServices, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServices {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).GetAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strobjectpath.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), iflags, objwbemnamedvalueset.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).DeleteAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strobjectpath.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOf<P0, P1>(&self, strclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstancesOf)(windows_core::Interface::as_raw(self), strclass.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).InstancesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strclass.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOf<P0, P1>(&self, strsuperclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubclassesOf)(windows_core::Interface::as_raw(self), strsuperclass.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strsuperclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SubclassesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strsuperclass.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), strquery.param().abi(), strquerylanguage.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, lflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strquery.param().abi(), strquerylanguage.param().abi(), lflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssociatorsOf<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9>(&self, strobjectpath: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P6: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P7: windows_core::Param<windows_core::BSTR>,
        P8: windows_core::Param<windows_core::BSTR>,
        P9: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AssociatorsOf)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), strassocclass.param().abi(), strresultclass.param().abi(), strresultrole.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredassocqualifier.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssociatorsOfAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11>(&self, objwbemsink: P0, strobjectpath: P1, strassocclass: P2, strresultclass: P3, strresultrole: P4, strrole: P5, bclassesonly: P6, bschemaonly: P7, strrequiredassocqualifier: P8, strrequiredqualifier: P9, iflags: i32, objwbemnamedvalueset: P10, objwbemasynccontext: P11) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P7: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P8: windows_core::Param<windows_core::BSTR>,
        P9: windows_core::Param<windows_core::BSTR>,
        P10: windows_core::Param<super::Com::IDispatch>,
        P11: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).AssociatorsOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strobjectpath.param().abi(), strassocclass.param().abi(), strresultclass.param().abi(), strresultrole.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredassocqualifier.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReferencesTo<P0, P1, P2, P3, P4, P5, P6>(&self, strobjectpath: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6) -> windows_core::Result<ISWbemObjectSet>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferencesTo)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), strresultclass.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReferencesToAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, objwbemsink: P0, strobjectpath: P1, strresultclass: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredqualifier: P6, iflags: i32, objwbemnamedvalueset: P7, objwbemasynccontext: P8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P5: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P6: windows_core::Param<windows_core::BSTR>,
        P7: windows_core::Param<super::Com::IDispatch>,
        P8: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ReferencesToAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strobjectpath.param().abi(), strresultclass.param().abi(), strrole.param().abi(), bclassesonly.param().abi(), bschemaonly.param().abi(), strrequiredqualifier.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemEventSource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), strquery.param().abi(), strquerylanguage.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strquery.param().abi(), strquerylanguage.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod<P0, P1, P2, P3>(&self, strobjectpath: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), strmethodname.param().abi(), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3, P4, P5>(&self, objwbemsink: P0, strobjectpath: P1, strmethodname: P2, objwbeminparameters: P3, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
        P5: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), strobjectpath.param().abi(), strmethodname.param().abi(), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemServices_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Get: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Delete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesOf: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesOf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesOfAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesOf: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesOf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesOfAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecQueryAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AssociatorsOf: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssociatorsOf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AssociatorsOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssociatorsOfAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReferencesTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReferencesTo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReferencesToAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReferencesToAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecNotificationQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecNotificationQueryAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethod: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethodAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemServicesEx, ISWbemServicesEx_Vtbl, 0xd2f68443_85dc_427e_91d8_366554cc754c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemServicesEx {
    type Target = ISWbemServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemServicesEx, windows_core::IUnknown, super::Com::IDispatch, ISWbemServices);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesEx {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put<P0, P1>(&self, objwbemobject: P0, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectPath>
    where
        P0: windows_core::Param<ISWbemObjectEx>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, objwbemobject: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISWbemSink>,
        P1: windows_core::Param<ISWbemObjectEx>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).PutAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemServicesEx_Vtbl {
    pub base__: ISWbemServices_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemSink, ISWbemSink_Vtbl, 0x75718c9f_f029_11d1_a1ac_00c04fb6c223);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemSink {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemSink, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSink {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISWbemSinkEvents, ISWbemSinkEvents_Vtbl, 0x75718ca0_f029_11d1_a1ac_00c04fb6c223);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISWbemSinkEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISWbemSinkEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSinkEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISWbemSinkEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IUnsecuredApartment, IUnsecuredApartment_Vtbl, 0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
impl core::ops::Deref for IUnsecuredApartment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUnsecuredApartment, windows_core::IUnknown);
impl IUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateObjectStub)(windows_core::Interface::as_raw(self), pobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUnsecuredApartment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObjectStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWMIExtension, IWMIExtension_Vtbl, 0xadc1f06e_5c7e_11d2_8b74_00104b2afb41);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWMIExtension {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWMIExtension, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWMIExtension {
    pub unsafe fn WMIObjectPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WMIObjectPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIObject(&self) -> windows_core::Result<ISWbemObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWMIObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIServices(&self) -> windows_core::Result<ISWbemServices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWMIServices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWMIExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub WMIObjectPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIServices: usize,
}
windows_core::imp::define_interface!(IWbemAddressResolution, IWbemAddressResolution_Vtbl, 0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
impl core::ops::Deref for IWbemAddressResolution {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemAddressResolution, windows_core::IUnknown);
impl IWbemAddressResolution {
    pub unsafe fn Resolve<P0>(&self, wsznamespacepath: P0, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), wsznamespacepath.param().abi(), core::mem::transmute(wszaddresstype), pdwaddresslength, pabbinaryaddress).ok()
    }
}
#[repr(C)]
pub struct IWbemAddressResolution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemBackupRestore, IWbemBackupRestore_Vtbl, 0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
impl core::ops::Deref for IWbemBackupRestore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemBackupRestore, windows_core::IUnknown);
impl IWbemBackupRestore {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), strbackuptofile.param().abi(), lflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), strrestorefromfile.param().abi(), lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemBackupRestore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemBackupRestoreEx, IWbemBackupRestoreEx_Vtbl, 0xa359dec5_e813_4834_8a2a_ba7f1d777d76);
impl core::ops::Deref for IWbemBackupRestoreEx {
    type Target = IWbemBackupRestore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemBackupRestoreEx, windows_core::IUnknown, IWbemBackupRestore);
impl IWbemBackupRestoreEx {
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWbemBackupRestoreEx_Vtbl {
    pub base__: IWbemBackupRestore_Vtbl,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemCallResult, IWbemCallResult_Vtbl, 0x44aca675_e8fc_11d0_a07c_00c04fb68820);
impl core::ops::Deref for IWbemCallResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemCallResult, windows_core::IUnknown);
impl IWbemCallResult {
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResultObject)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResultString)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResultServices)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCallStatus)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWbemCallResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResultObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetResultServices: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemClassObject, IWbemClassObject_Vtbl, 0xdc12a681_737f_11cf_884d_00aa004b2e24);
impl core::ops::Deref for IWbemClassObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemClassObject, windows_core::IUnknown);
impl IWbemClassObject {
    pub unsafe fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQualifierSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut windows_core::VARIANT, ptype: Option<*mut i32>, plflavor: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), core::mem::transmute(ptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(plflavor.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const windows_core::VARIANT, r#type: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), r#type).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const windows_core::VARIANT) -> windows_core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), wszqualifiername.param().abi(), lflags, core::mem::transmute(pqualifierval), &mut result__).map(|| result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lenumflags).ok()
    }
    pub unsafe fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut windows_core::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(strname), core::mem::transmute(pval), ptype, plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyQualifierSet)(windows_core::Interface::as_raw(self), wszproperty.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWbemClassObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectText)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpawnDerivedClass)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpawnInstance)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CompareTo<P0>(&self, lflags: WBEM_COMPARISON_FLAG, pcompareto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).CompareTo)(windows_core::Interface::as_raw(self), lflags, pcompareto.param().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyOrigin)(windows_core::Interface::as_raw(self), wszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InheritsFrom)(windows_core::Interface::as_raw(self), strancestor.param().abi()).ok()
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<P0, P1, P2>(&self, wszname: P0, lflags: i32, pinsignature: P1, poutsignature: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).PutMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, pinsignature.param().abi(), poutsignature.param().abi()).ok()
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteMethod)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginMethodEnumeration)(windows_core::Interface::as_raw(self), lenumflags).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextMethod)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndMethodEnumeration)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMethodQualifierSet)(windows_core::Interface::as_raw(self), wszmethod.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WBEM_CONDITION_FLAG_TYPE, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SpawnDerivedClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(*mut core::ffi::c_void, WBEM_COMPARISON_FLAG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InheritsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NextMethod: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemClientConnectionTransport, IWbemClientConnectionTransport_Vtbl, 0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
impl core::ops::Deref for IWbemClientConnectionTransport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemClientConnectionTransport, windows_core::IUnknown);
impl IWbemClientConnectionTransport {
    pub unsafe fn Open<P0, P1, P2, P3, P4, P5, T>(&self, straddresstype: P0, abbinaryaddress: &[u8], strobject: P1, struser: P2, strpassword: P3, strlocale: P4, lflags: i32, pctx: P5, pcallres: *mut Option<IWbemCallResult>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), straddresstype.param().abi(), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), strobject.param().abi(), struser.param().abi(), strpassword.param().abi(), strlocale.param().abi(), lflags, pctx.param().abi(), &T::IID, &mut result__, core::mem::transmute(pcallres)).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenAsync<P0, P1, P2, P3, P4, P5, P6>(&self, straddresstype: P0, abbinaryaddress: &[u8], strobject: P1, struser: P2, strpassword: P3, strlocale: P4, lflags: i32, pctx: P5, riid: *const windows_core::GUID, presponsehandler: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<IWbemContext>,
        P6: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).OpenAsync)(windows_core::Interface::as_raw(self), straddresstype.param().abi(), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), strobject.param().abi(), struser.param().abi(), strpassword.param().abi(), strlocale.param().abi(), lflags, pctx.param().abi(), riid, presponsehandler.param().abi()).ok()
    }
    pub unsafe fn Cancel<P0>(&self, lflags: i32, phandler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), lflags, phandler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemClientConnectionTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const u8, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const u8, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemClientTransport, IWbemClientTransport_Vtbl, 0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
impl core::ops::Deref for IWbemClientTransport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemClientTransport, windows_core::IUnknown);
impl IWbemClientTransport {
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5, P6>(&self, straddresstype: P0, abbinaryaddress: &[u8], strnetworkresource: P1, struser: P2, strpassword: P3, strlocale: P4, lsecurityflags: i32, strauthority: P5, pctx: P6) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), straddresstype.param().abi(), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), strnetworkresource.param().abi(), struser.param().abi(), strpassword.param().abi(), strlocale.param().abi(), lsecurityflags, strauthority.param().abi(), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemClientTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const u8, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemConfigureRefresher, IWbemConfigureRefresher_Vtbl, 0x49353c92_516b_11d1_aea6_00c04fb68820);
impl core::ops::Deref for IWbemConfigureRefresher {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemConfigureRefresher, windows_core::IUnknown);
impl IWbemConfigureRefresher {
    pub unsafe fn AddObjectByPath<P0, P1, P2>(&self, pnamespace: P0, wszpath: P1, lflags: i32, pcontext: P2, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).AddObjectByPath)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszpath.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn AddObjectByTemplate<P0, P1, P2>(&self, pnamespace: P0, ptemplate: P1, lflags: i32, pcontext: P2, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).AddObjectByTemplate)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn AddRefresher<P0>(&self, prefresher: P0, lflags: i32, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemRefresher>,
    {
        (windows_core::Interface::vtable(self).AddRefresher)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lflags, plid).ok()
    }
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lid, lflags).ok()
    }
    pub unsafe fn AddEnum<P0, P1, P2>(&self, pnamespace: P0, wszclassname: P1, lflags: i32, pcontext: P2, ppenum: *mut Option<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclassname.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(ppenum), plid).ok()
    }
}
#[repr(C)]
pub struct IWbemConfigureRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjectByPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemConnectorLogin, IWbemConnectorLogin_Vtbl, 0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
impl core::ops::Deref for IWbemConnectorLogin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemConnectorLogin, windows_core::IUnknown);
impl IWbemConnectorLogin {
    pub unsafe fn ConnectorLogin<P0, P1, P2, T>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ConnectorLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemConnectorLogin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectorLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemConstructClassObject, IWbemConstructClassObject_Vtbl, 0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
impl core::ops::Deref for IWbemConstructClassObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemConstructClassObject, windows_core::IUnknown);
impl IWbemConstructClassObject {
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInheritanceChain)(windows_core::Interface::as_raw(self), lnumantecedents, awszantecedents).ok()
    }
    pub unsafe fn SetPropertyOrigin<P0>(&self, wszpropertyname: P0, loriginindex: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyOrigin)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), loriginindex).ok()
    }
    pub unsafe fn SetMethodOrigin<P0>(&self, wszmethodname: P0, loriginindex: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), loriginindex).ok()
    }
    pub unsafe fn SetServerNamespace<P0, P1>(&self, wszserver: P0, wsznamespace: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetServerNamespace)(windows_core::Interface::as_raw(self), wszserver.param().abi(), wsznamespace.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemConstructClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInheritanceChain: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemContext, IWbemContext_Vtbl, 0x44aca674_e8fc_11d0_a07c_00c04fb68820);
impl core::ops::Deref for IWbemContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemContext, windows_core::IUnknown);
impl IWbemContext {
    pub unsafe fn Clone(&self) -> windows_core::Result<IWbemContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, wszname: P0, lflags: i32, pvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags).ok()
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWbemContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemDecoupledBasicEventProvider, IWbemDecoupledBasicEventProvider_Vtbl, 0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
impl core::ops::Deref for IWbemDecoupledBasicEventProvider {
    type Target = IWbemDecoupledRegistrar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemDecoupledBasicEventProvider, windows_core::IUnknown, IWbemDecoupledRegistrar);
impl IWbemDecoupledBasicEventProvider {
    pub unsafe fn GetSink<P0>(&self, a_flags: i32, a_context: P0) -> windows_core::Result<IWbemObjectSink>
    where
        P0: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetService<P0>(&self, a_flags: i32, a_context: P0) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemDecoupledBasicEventProvider_Vtbl {
    pub base__: IWbemDecoupledRegistrar_Vtbl,
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemDecoupledRegistrar, IWbemDecoupledRegistrar_Vtbl, 0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
impl core::ops::Deref for IWbemDecoupledRegistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemDecoupledRegistrar, windows_core::IUnknown);
impl IWbemDecoupledRegistrar {
    pub unsafe fn Register<P0, P1, P2, P3, P4, P5>(&self, a_flags: i32, a_context: P0, a_user: P1, a_locale: P2, a_scope: P3, a_registration: P4, piunknown: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemContext>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), a_user.param().abi(), a_locale.param().abi(), a_scope.param().abi(), a_registration.param().abi(), piunknown.param().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnRegister)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWbemDecoupledRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegister: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemEventConsumerProvider, IWbemEventConsumerProvider_Vtbl, 0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
impl core::ops::Deref for IWbemEventConsumerProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemEventConsumerProvider, windows_core::IUnknown);
impl IWbemEventConsumerProvider {
    pub unsafe fn FindConsumer<P0>(&self, plogicalconsumer: P0) -> windows_core::Result<IWbemUnboundObjectSink>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemEventConsumerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemEventProvider, IWbemEventProvider_Vtbl, 0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
impl core::ops::Deref for IWbemEventProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemEventProvider, windows_core::IUnknown);
impl IWbemEventProvider {
    pub unsafe fn ProvideEvents<P0>(&self, psink: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).ProvideEvents)(windows_core::Interface::as_raw(self), psink.param().abi(), lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemEventProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProvideEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemEventProviderQuerySink, IWbemEventProviderQuerySink_Vtbl, 0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
impl core::ops::Deref for IWbemEventProviderQuerySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemEventProviderQuerySink, windows_core::IUnknown);
impl IWbemEventProviderQuerySink {
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NewQuery)(windows_core::Interface::as_raw(self), dwid, wszquerylanguage, wszquery).ok()
    }
    pub unsafe fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelQuery)(windows_core::Interface::as_raw(self), dwid).ok()
    }
}
#[repr(C)]
pub struct IWbemEventProviderQuerySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *const u16) -> windows_core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemEventProviderSecurity, IWbemEventProviderSecurity_Vtbl, 0x631f7d96_d993_11d2_b339_00105a1f4aaf);
impl core::ops::Deref for IWbemEventProviderSecurity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemEventProviderSecurity, windows_core::IUnknown);
impl IWbemEventProviderSecurity {
    pub unsafe fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, psid: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AccessCheck)(windows_core::Interface::as_raw(self), wszquerylanguage, wszquery, psid.len().try_into().unwrap(), core::mem::transmute(psid.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWbemEventProviderSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemEventSink, IWbemEventSink_Vtbl, 0x3ae0080a_7e3a_4366_bf89_0feedc931659);
impl core::ops::Deref for IWbemEventSink {
    type Target = IWbemObjectSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemEventSink, windows_core::IUnknown, IWbemObjectSink);
impl IWbemEventSink {
    pub unsafe fn SetSinkSecurity(&self, psd: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSinkSecurity)(windows_core::Interface::as_raw(self), psd.len().try_into().unwrap(), core::mem::transmute(psd.as_ptr())).ok()
    }
    pub unsafe fn IsActive(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRestrictedSink<P0>(&self, awszqueries: &[windows_core::PCWSTR], pcallback: P0) -> windows_core::Result<IWbemEventSink>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestrictedSink)(windows_core::Interface::as_raw(self), awszqueries.len().try_into().unwrap(), core::mem::transmute(awszqueries.as_ptr()), pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBatchingParameters)(windows_core::Interface::as_raw(self), lflags, dwmaxbuffersize, dwmaxsendlatency).ok()
    }
}
#[repr(C)]
pub struct IWbemEventSink_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub SetSinkSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemHiPerfEnum, IWbemHiPerfEnum_Vtbl, 0x2705c288_79ae_11d2_b348_00105a1f8177);
impl core::ops::Deref for IWbemHiPerfEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemHiPerfEnum, windows_core::IUnknown);
impl IWbemHiPerfEnum {
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddObjects)(windows_core::Interface::as_raw(self), lflags, unumobjects, apids, core::mem::transmute(apobj)).ok()
    }
    pub unsafe fn RemoveObjects(&self, lflags: i32, apids: &[i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveObjects)(windows_core::Interface::as_raw(self), lflags, apids.len().try_into().unwrap(), core::mem::transmute(apids.as_ptr())).ok()
    }
    pub unsafe fn GetObjects(&self, lflags: i32, apobj: &mut [Option<IWbemObjectAccess>], pureturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), lflags, apobj.len().try_into().unwrap(), core::mem::transmute(apobj.as_ptr()), pureturned).ok()
    }
    pub unsafe fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self), lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemHiPerfEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32) -> windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemHiPerfProvider, IWbemHiPerfProvider_Vtbl, 0x49353c93_516b_11d1_aea6_00c04fb68820);
impl core::ops::Deref for IWbemHiPerfProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemHiPerfProvider, windows_core::IUnknown);
impl IWbemHiPerfProvider {
    pub unsafe fn QueryInstances<P0, P1, P2, P3>(&self, pnamespace: P0, wszclass: P1, lflags: i32, pctx: P2, psink: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).QueryInstances)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass.param().abi(), lflags, pctx.param().abi(), psink.param().abi()).ok()
    }
    pub unsafe fn CreateRefresher<P0>(&self, pnamespace: P0, lflags: i32) -> windows_core::Result<IWbemRefresher>
    where
        P0: windows_core::Param<IWbemServices>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRefresher)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRefreshableObject<P0, P1, P2, P3>(&self, pnamespace: P0, ptemplate: P1, prefresher: P2, lflags: i32, pcontext: P3, pprefreshable: *mut Option<IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemObjectAccess>,
        P2: windows_core::Param<IWbemRefresher>,
        P3: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).CreateRefreshableObject)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn StopRefreshing<P0>(&self, prefresher: P0, lid: i32, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemRefresher>,
    {
        (windows_core::Interface::vtable(self).StopRefreshing)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lid, lflags).ok()
    }
    pub unsafe fn CreateRefreshableEnum<P0, P1, P2, P3, P4>(&self, pnamespace: P0, wszclass: P1, prefresher: P2, lflags: i32, pcontext: P3, phiperfenum: P4) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemRefresher>,
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemHiPerfEnum>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRefreshableEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), phiperfenum.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjects<P0, P1>(&self, pnamespace: P0, apobj: &mut [Option<IWbemObjectAccess>], lflags: i32, pcontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), apobj.len().try_into().unwrap(), core::mem::transmute(apobj.as_ptr()), lflags, pcontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemHiPerfProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRefreshableObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StopRefreshing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CreateRefreshableEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemLevel1Login, IWbemLevel1Login_Vtbl, 0xf309ad18_d86a_11d0_a075_00c04fb68820);
impl core::ops::Deref for IWbemLevel1Login {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemLevel1Login, windows_core::IUnknown);
impl IWbemLevel1Login {
    pub unsafe fn EstablishPosition<P0>(&self, wszlocalelist: P0, dwnumlocales: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EstablishPosition)(windows_core::Interface::as_raw(self), wszlocalelist.param().abi(), dwnumlocales, &mut result__).map(|| result__)
    }
    pub unsafe fn RequestChallenge<P0, P1>(&self, wsznetworkresource: P0, wszuser: P1) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestChallenge)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszuser.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn WBEMLogin<P0, P1>(&self, wszpreferredlocale: P0, accesstoken: *const u8, lflags: i32, pctx: P1) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WBEMLogin)(windows_core::Interface::as_raw(self), wszpreferredlocale.param().abi(), accesstoken, lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NTLMLogin<P0, P1, P2>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P2) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NTLMLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemLevel1Login_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EstablishPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    pub WBEMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NTLMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemLocator, IWbemLocator_Vtbl, 0xdc12a687_737f_11cf_884d_00aa004b2e24);
impl core::ops::Deref for IWbemLocator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemLocator, windows_core::IUnknown);
impl IWbemLocator {
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5>(&self, strnetworkresource: P0, struser: P1, strpassword: P2, strlocale: P3, lsecurityflags: i32, strauthority: P4, pctx: P5) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), strnetworkresource.param().abi(), struser.param().abi(), strpassword.param().abi(), strlocale.param().abi(), lsecurityflags, strauthority.param().abi(), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemLocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemObjectAccess, IWbemObjectAccess_Vtbl, 0x49353c9a_516b_11d1_aea6_00c04fb68820);
impl core::ops::Deref for IWbemObjectAccess {
    type Target = IWbemClassObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectAccess, windows_core::IUnknown, IWbemClassObject);
impl IWbemObjectAccess {
    pub unsafe fn GetPropertyHandle<P0>(&self, wszpropertyname: P0, ptype: *mut i32, plhandle: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyHandle)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), ptype, plhandle).ok()
    }
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, adata: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WritePropertyValue)(windows_core::Interface::as_raw(self), lhandle, adata.len().try_into().unwrap(), core::mem::transmute(adata.as_ptr())).ok()
    }
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, plnumbytes: *mut i32, adata: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadPropertyValue)(windows_core::Interface::as_raw(self), lhandle, adata.len().try_into().unwrap(), plnumbytes, core::mem::transmute(adata.as_ptr())).ok()
    }
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadDWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
    }
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteDWORD)(windows_core::Interface::as_raw(self), lhandle, dw).ok()
    }
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadQWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
    }
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteQWORD)(windows_core::Interface::as_raw(self), lhandle, pw).ok()
    }
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyInfoByHandle)(windows_core::Interface::as_raw(self), lhandle, core::mem::transmute(pstrname), ptype).ok()
    }
    pub unsafe fn Lock(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Unlock(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemObjectAccess_Vtbl {
    pub base__: IWbemClassObject_Vtbl,
    pub GetPropertyHandle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub WritePropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const u8) -> windows_core::HRESULT,
    pub ReadPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32, *mut u8) -> windows_core::HRESULT,
    pub ReadDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u64) -> windows_core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u64) -> windows_core::HRESULT,
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemObjectSink, IWbemObjectSink_Vtbl, 0x7c857801_7381_11cf_884d_00aa004b2e24);
impl core::ops::Deref for IWbemObjectSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectSink, windows_core::IUnknown);
impl IWbemObjectSink {
    pub unsafe fn Indicate(&self, apobjarray: &[Option<IWbemClassObject>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Indicate)(windows_core::Interface::as_raw(self), apobjarray.len().try_into().unwrap(), core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0, P1>(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: P0, pobjparam: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lflags, hresult, strparam.param().abi(), pobjparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Indicate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemObjectSinkEx, IWbemObjectSinkEx_Vtbl, 0xe7d35cfa_348b_485e_b524_252725d697ca);
impl core::ops::Deref for IWbemObjectSinkEx {
    type Target = IWbemObjectSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectSinkEx, windows_core::IUnknown, IWbemObjectSink);
impl IWbemObjectSinkEx {
    pub unsafe fn WriteMessage<P0>(&self, uchannel: u32, strmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).WriteMessage)(windows_core::Interface::as_raw(self), uchannel, strmessage.param().abi()).ok()
    }
    pub unsafe fn WriteError<P0>(&self, pobjerror: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteError)(windows_core::Interface::as_raw(self), pobjerror.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn PromptUser<P0>(&self, strmessage: P0, uprompttype: u8) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PromptUser)(windows_core::Interface::as_raw(self), strmessage.param().abi(), uprompttype, &mut result__).map(|| result__)
    }
    pub unsafe fn WriteProgress<P0, P1, P2>(&self, stractivity: P0, strcurrentoperation: P1, strstatusdescription: P2, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).WriteProgress)(windows_core::Interface::as_raw(self), stractivity.param().abi(), strcurrentoperation.param().abi(), strstatusdescription.param().abi(), upercentcomplete, usecondsremaining).ok()
    }
    pub unsafe fn WriteStreamParameter<P0>(&self, strname: P0, vtvalue: *const windows_core::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).WriteStreamParameter)(windows_core::Interface::as_raw(self), strname.param().abi(), core::mem::transmute(vtvalue), ultype, ulflags).ok()
    }
}
#[repr(C)]
pub struct IWbemObjectSinkEx_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub WriteMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub WriteError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PromptUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u8, *mut u8) -> windows_core::HRESULT,
    pub WriteProgress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, u32) -> windows_core::HRESULT,
    pub WriteStreamParameter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemObjectTextSrc, IWbemObjectTextSrc_Vtbl, 0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
impl core::ops::Deref for IWbemObjectTextSrc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectTextSrc, windows_core::IUnknown);
impl IWbemObjectTextSrc {
    pub unsafe fn GetText<P0, P1>(&self, lflags: i32, pobj: P0, uobjtextformat: u32, pctx: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P1: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pobj.param().abi(), uobjtextformat, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFromText<P0, P1>(&self, lflags: i32, strtext: P0, uobjtextformat: u32, pctx: P1) -> windows_core::Result<IWbemClassObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFromText)(windows_core::Interface::as_raw(self), lflags, strtext.param().abi(), uobjtextformat, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemObjectTextSrc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateFromText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemPath, IWbemPath_Vtbl, 0x3bc15af2_736c_477e_9e51_238af8667dcc);
impl core::ops::Deref for IWbemPath {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemPath, windows_core::IUnknown);
impl IWbemPath {
    pub unsafe fn SetText<P0>(&self, umode: u32, pszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), umode, pszpath.param().abi()).ok()
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength, core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
    }
    pub unsafe fn SetServer<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetServer)(windows_core::Interface::as_raw(self), punamebuflength, core::mem::transmute(pname)).ok()
    }
    pub unsafe fn GetNamespaceCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamespaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNamespaceAt<P0>(&self, uindex: u32, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, pszname.param().abi()).ok()
    }
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, punamebuflength, core::mem::transmute(pname)).ok()
    }
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveNamespaceAt)(windows_core::Interface::as_raw(self), uindex).ok()
    }
    pub unsafe fn RemoveAllNamespaces(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllNamespaces)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetScopeCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScopeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScope<P0>(&self, uindex: u32, pszclass: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), uindex, pszclass.param().abi()).ok()
    }
    pub unsafe fn SetScopeFromText<P0>(&self, uindex: u32, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetScopeFromText)(windows_core::Interface::as_raw(self), uindex, psztext.param().abi()).ok()
    }
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut Option<IWbemPathKeyList>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScope)(windows_core::Interface::as_raw(self), uindex, puclassnamebufsize, core::mem::transmute(pszclass), core::mem::transmute(pkeylist)).ok()
    }
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScopeAsText)(windows_core::Interface::as_raw(self), uindex, putextbufsize, core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveScope)(windows_core::Interface::as_raw(self), uindex).ok()
    }
    pub unsafe fn RemoveAllScopes(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllScopes)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClassName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetClassName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassName)(windows_core::Interface::as_raw(self), pubufflength, core::mem::transmute(pszname)).ok()
    }
    pub unsafe fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeyList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateClassPart<P0>(&self, lflags: i32, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateClassPart)(windows_core::Interface::as_raw(self), lflags, name.param().abi()).ok()
    }
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteClassPart)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn IsRelative<P0, P1>(&self, wszmachine: P0, wsznamespace: P1) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsRelative)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi())
    }
    pub unsafe fn IsRelativeOrChild<P0, P1>(&self, wszmachine: P0, wsznamespace: P1, lflags: i32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsRelativeOrChild)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi(), lflags)
    }
    pub unsafe fn IsLocal<P0>(&self, wszmachine: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), wszmachine.param().abi())
    }
    pub unsafe fn IsSameClassName<P0>(&self, wszclass: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsSameClassName)(windows_core::Interface::as_raw(self), wszclass.param().abi())
    }
}
#[repr(C)]
pub struct IWbemPath_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetNamespaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RemoveNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScopeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetScopeFromText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScopeAsText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RemoveScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllScopes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetKeyList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteClassPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsRelative: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> super::super::Foundation::BOOL,
    pub IsRelativeOrChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> super::super::Foundation::BOOL,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> super::super::Foundation::BOOL,
    pub IsSameClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IWbemPathKeyList, IWbemPathKeyList_Vtbl, 0x9ae62877_7544_4bb0_aa26_a13824659ed6);
impl core::ops::Deref for IWbemPathKeyList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemPathKeyList, windows_core::IUnknown);
impl IWbemPathKeyList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKey<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, pkeyval).ok()
    }
    pub unsafe fn SetKey2<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetKey2)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, core::mem::transmute(pkeyval)).ok()
    }
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKey)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize, core::mem::transmute(pszkeyname), pukeyvalbufsize, pkeyval, puapparentcimtype).ok()
    }
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut windows_core::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKey2)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize, core::mem::transmute(pszkeyname), core::mem::transmute(pkeyvalue), puapparentcimtype).ok()
    }
    pub unsafe fn RemoveKey<P0>(&self, wszname: P0, uflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags).ok()
    }
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllKeys)(windows_core::Interface::as_raw(self), uflags).ok()
    }
    pub unsafe fn MakeSingleton(&self, bset: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MakeSingleton)(windows_core::Interface::as_raw(self), bset).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength, core::mem::transmute(psztext)).ok()
    }
}
#[repr(C)]
pub struct IWbemPathKeyList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub RemoveKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MakeSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemPropertyProvider, IWbemPropertyProvider_Vtbl, 0xce61e841_65bc_11d0_b6bd_00aa003240c7);
impl core::ops::Deref for IWbemPropertyProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemPropertyProvider, windows_core::IUnknown);
impl IWbemPropertyProvider {
    pub unsafe fn GetProperty<P0, P1, P2, P3>(&self, lflags: i32, strlocale: P0, strclassmapping: P1, strinstmapping: P2, strpropmapping: P3) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lflags, strlocale.param().abi(), strclassmapping.param().abi(), strinstmapping.param().abi(), strpropmapping.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutProperty<P0, P1, P2, P3>(&self, lflags: i32, strlocale: P0, strclassmapping: P1, strinstmapping: P2, strpropmapping: P3, pvvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), lflags, strlocale.param().abi(), strclassmapping.param().abi(), strinstmapping.param().abi(), strpropmapping.param().abi(), core::mem::transmute(pvvalue)).ok()
    }
}
#[repr(C)]
pub struct IWbemPropertyProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemProviderIdentity, IWbemProviderIdentity_Vtbl, 0x631f7d97_d993_11d2_b339_00105a1f4aaf);
impl core::ops::Deref for IWbemProviderIdentity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemProviderIdentity, windows_core::IUnknown);
impl IWbemProviderIdentity {
    pub unsafe fn SetRegistrationObject<P0>(&self, lflags: i32, pprovreg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).SetRegistrationObject)(windows_core::Interface::as_raw(self), lflags, pprovreg.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemProviderIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRegistrationObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemProviderInit, IWbemProviderInit_Vtbl, 0x1be41572_91dd_11d1_aeb2_00c04fb68820);
impl core::ops::Deref for IWbemProviderInit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemProviderInit, windows_core::IUnknown);
impl IWbemProviderInit {
    pub unsafe fn Initialize<P0, P1, P2, P3, P4, P5>(&self, wszuser: P0, lflags: i32, wsznamespace: P1, wszlocale: P2, pnamespace: P3, pctx: P4, pinitsink: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemServices>,
        P4: windows_core::Param<IWbemContext>,
        P5: windows_core::Param<IWbemProviderInitSink>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), wszuser.param().abi(), lflags, wsznamespace.param().abi(), wszlocale.param().abi(), pnamespace.param().abi(), pctx.param().abi(), pinitsink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemProviderInit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemProviderInitSink, IWbemProviderInitSink_Vtbl, 0x1be41571_91dd_11d1_aeb2_00c04fb68820);
impl core::ops::Deref for IWbemProviderInitSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemProviderInitSink, windows_core::IUnknown);
impl IWbemProviderInitSink {
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lstatus, lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemProviderInitSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemQualifierSet, IWbemQualifierSet_Vtbl, 0xdc12a680_737f_11cf_884d_00aa004b2e24);
impl core::ops::Deref for IWbemQualifierSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemQualifierSet, windows_core::IUnknown);
impl IWbemQualifierSet {
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut windows_core::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), plflavor).ok()
    }
    pub unsafe fn Put<P0>(&self, wszname: P0, pval: *const windows_core::VARIANT, lflavor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), core::mem::transmute(pval), lflavor).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut windows_core::VARIANT, plflavor: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pval), plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWbemQualifierSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32) -> windows_core::HRESULT,
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32) -> windows_core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemQuery, IWbemQuery_Vtbl, 0x81166f58_dd98_11d3_a120_00105a1f515a);
impl core::ops::Deref for IWbemQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemQuery, windows_core::IUnknown);
impl IWbemQuery {
    pub unsafe fn Empty(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Empty)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize, pufeatures).ok()
    }
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TestLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize, pufeatures).ok()
    }
    pub unsafe fn Parse<P0, P1>(&self, pszlang: P0, pszquery: P1, uflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Parse)(windows_core::Interface::as_raw(self), pszlang.param().abi(), pszquery.param().abi(), uflags).ok()
    }
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAnalysis)(windows_core::Interface::as_raw(self), uanalysistype, uflags, panalysis).ok()
    }
    pub unsafe fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeMemory)(windows_core::Interface::as_raw(self), pmem).ok()
    }
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQueryInfo)(windows_core::Interface::as_raw(self), uanalysistype, uinfoid, ubufsize, pdestbuf).ok()
    }
}
#[repr(C)]
pub struct IWbemQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Empty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguageFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u32) -> windows_core::HRESULT,
    pub TestLanguageFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemRefresher, IWbemRefresher_Vtbl, 0x49353c99_516b_11d1_aea6_00c04fb68820);
impl core::ops::Deref for IWbemRefresher {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemRefresher, windows_core::IUnknown);
impl IWbemRefresher {
    pub unsafe fn Refresh(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), lflags).ok()
    }
}
#[repr(C)]
pub struct IWbemRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemServices, IWbemServices_Vtbl, 0x9556dc99_828c_11cf_a37e_00aa003240c7);
impl core::ops::Deref for IWbemServices {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemServices, windows_core::IUnknown);
impl IWbemServices {
    pub unsafe fn OpenNamespace<P0, P1>(&self, strnamespace: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppworkingnamespace: Option<*mut Option<IWbemServices>>, ppresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).OpenNamespace)(windows_core::Interface::as_raw(self), strnamespace.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppworkingnamespace.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CancelAsyncCall<P0>(&self, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).CancelAsyncCall)(windows_core::Interface::as_raw(self), psink.param().abi()).ok()
    }
    pub unsafe fn QueryObjectSink(&self, lflags: WBEM_GENERIC_FLAG_TYPE) -> windows_core::Result<IWbemObjectSink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryObjectSink)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObject<P0, P1>(&self, strobjectpath: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppobject: Option<*mut Option<IWbemClassObject>>, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppobject.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetObjectAsync<P0, P1, P2>(&self, strobjectpath: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).GetObjectAsync)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn PutClass<P0, P1>(&self, pobject: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).PutClass)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn PutClassAsync<P0, P1, P2>(&self, pobject: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).PutClassAsync)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn DeleteClass<P0, P1>(&self, strclass: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).DeleteClass)(windows_core::Interface::as_raw(self), strclass.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DeleteClassAsync<P0, P1, P2>(&self, strclass: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).DeleteClassAsync)(windows_core::Interface::as_raw(self), strclass.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn CreateClassEnum<P0, P1>(&self, strsuperclass: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1) -> windows_core::Result<IEnumWbemClassObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateClassEnum)(windows_core::Interface::as_raw(self), strsuperclass.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateClassEnumAsync<P0, P1, P2>(&self, strsuperclass: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).CreateClassEnumAsync)(windows_core::Interface::as_raw(self), strsuperclass.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn PutInstance<P0, P1>(&self, pinst: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).PutInstance)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn PutInstanceAsync<P0, P1, P2>(&self, pinst: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).PutInstanceAsync)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn DeleteInstance<P0, P1>(&self, strobjectpath: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).DeleteInstance)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), lflags, pctx.param().abi(), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DeleteInstanceAsync<P0, P1, P2>(&self, strobjectpath: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).DeleteInstanceAsync)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn CreateInstanceEnum<P0, P1>(&self, strfilter: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1) -> windows_core::Result<IEnumWbemClassObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstanceEnum)(windows_core::Interface::as_raw(self), strfilter.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstanceEnumAsync<P0, P1, P2>(&self, strfilter: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P1, presponsehandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).CreateInstanceEnumAsync)(windows_core::Interface::as_raw(self), strfilter.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn ExecQuery<P0, P1, P2>(&self, strquerylanguage: P0, strquery: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), strquerylanguage.param().abi(), strquery.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExecQueryAsync<P0, P1, P2, P3>(&self, strquerylanguage: P0, strquery: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), strquerylanguage.param().abi(), strquery.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn ExecNotificationQuery<P0, P1, P2>(&self, strquerylanguage: P0, strquery: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), strquerylanguage.param().abi(), strquery.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2, P3>(&self, strquerylanguage: P0, strquery: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), strquerylanguage.param().abi(), strquery.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok()
    }
    pub unsafe fn ExecMethod<P0, P1, P2, P3>(&self, strobjectpath: P0, strmethodname: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, pinparams: P3, ppoutparams: Option<*mut Option<IWbemClassObject>>, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), strmethodname.param().abi(), lflags, pctx.param().abi(), pinparams.param().abi(), core::mem::transmute(ppoutparams.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppcallresult.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3, P4>(&self, strobjectpath: P0, strmethodname: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, pinparams: P3, presponsehandler: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemClassObject>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), strobjectpath.param().abi(), strmethodname.param().abi(), lflags, pctx.param().abi(), pinparams.param().abi(), presponsehandler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(*mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClass: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnum: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnum: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemShutdown, IWbemShutdown_Vtbl, 0xb7b31df9_d515_11d3_a11c_00105a1f515a);
impl core::ops::Deref for IWbemShutdown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemShutdown, windows_core::IUnknown);
impl IWbemShutdown {
    pub unsafe fn Shutdown<P0>(&self, ureason: i32, umaxmilliseconds: u32, pctx: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemContext>,
    {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), ureason, umaxmilliseconds, pctx.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWbemShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemStatusCodeText, IWbemStatusCodeText_Vtbl, 0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
impl core::ops::Deref for IWbemStatusCodeText {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemStatusCodeText, windows_core::IUnknown);
impl IWbemStatusCodeText {
    pub unsafe fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFacilityCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemStatusCodeText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFacilityCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemTransport, IWbemTransport_Vtbl, 0x553fe584_2156_11d0_b6ae_00aa003240c7);
impl core::ops::Deref for IWbemTransport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemTransport, windows_core::IUnknown);
impl IWbemTransport {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWbemTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemUnboundObjectSink, IWbemUnboundObjectSink_Vtbl, 0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
impl core::ops::Deref for IWbemUnboundObjectSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemUnboundObjectSink, windows_core::IUnknown);
impl IWbemUnboundObjectSink {
    pub unsafe fn IndicateToConsumer<P0>(&self, plogicalconsumer: P0, apobjects: &[Option<IWbemClassObject>]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        (windows_core::Interface::vtable(self).IndicateToConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), apobjects.len().try_into().unwrap(), core::mem::transmute(apobjects.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWbemUnboundObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IndicateToConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWbemUnsecuredApartment, IWbemUnsecuredApartment_Vtbl, 0x31739d04_3471_4cf4_9a7c_57a44ae71956);
impl core::ops::Deref for IWbemUnsecuredApartment {
    type Target = IUnsecuredApartment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemUnsecuredApartment, windows_core::IUnknown, IUnsecuredApartment);
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateSinkStub<P0, P1>(&self, psink: P0, dwflags: u32, wszreserved: P1) -> windows_core::Result<IWbemObjectSink>
    where
        P0: windows_core::Param<IWbemObjectSink>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSinkStub)(windows_core::Interface::as_raw(self), psink.param().abi(), dwflags, wszreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWbemUnsecuredApartment_Vtbl {
    pub base__: IUnsecuredApartment_Vtbl,
    pub CreateSinkStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(11i32);
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(103i32);
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(101i32);
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(0i32);
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8192i32);
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4095i32);
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(13i32);
pub const CIM_REAL32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4i32);
pub const CIM_REAL64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(5i32);
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(102i32);
pub const CIM_SINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(2i32);
pub const CIM_SINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(3i32);
pub const CIM_SINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(20i32);
pub const CIM_SINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(16i32);
pub const CIM_STRING: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8i32);
pub const CIM_UINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(18i32);
pub const CIM_UINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(19i32);
pub const CIM_UINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(21i32);
pub const CIM_UINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(17i32);
pub const MI_ARRAY: MI_Type = MI_Type(16i32);
pub const MI_BOOLEAN: MI_Type = MI_Type(0i32);
pub const MI_BOOLEANA: MI_Type = MI_Type(16i32);
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = MI_CallbackMode(2i32);
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = MI_CallbackMode(1i32);
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = MI_CallbackMode(0i32);
pub const MI_CALL_VERSION: u32 = 1u32;
pub const MI_CHAR16: MI_Type = MI_Type(11i32);
pub const MI_CHAR16A: MI_Type = MI_Type(27i32);
pub const MI_CHAR_TYPE: u32 = 2u32;
pub const MI_DATETIME: MI_Type = MI_Type(12i32);
pub const MI_DATETIMEA: MI_Type = MI_Type(28i32);
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(0i32);
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(4i32);
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(2i32);
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(3i32);
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(1i32);
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = MI_ErrorCategory(18i32);
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = MI_ErrorCategory(28i32);
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = MI_ErrorCategory(2i32);
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = MI_ErrorCategory(27i32);
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = MI_ErrorCategory(4i32);
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = MI_ErrorCategory(3i32);
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = MI_ErrorCategory(24i32);
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = MI_ErrorCategory(5i32);
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = MI_ErrorCategory(6i32);
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = MI_ErrorCategory(7i32);
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = MI_ErrorCategory(8i32);
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = MI_ErrorCategory(9i32);
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(29i32);
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = MI_ErrorCategory(10i32);
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = MI_ErrorCategory(31i32);
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = MI_ErrorCategory(11i32);
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = MI_ErrorCategory(12i32);
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = MI_ErrorCategory(0i32);
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = MI_ErrorCategory(13i32);
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = MI_ErrorCategory(1i32);
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = MI_ErrorCategory(14i32);
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = MI_ErrorCategory(15i32);
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = MI_ErrorCategory(17i32);
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = MI_ErrorCategory(26i32);
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(30i32);
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = MI_ErrorCategory(22i32);
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = MI_ErrorCategory(19i32);
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = MI_ErrorCategory(20i32);
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = MI_ErrorCategory(21i32);
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = MI_ErrorCategory(25i32);
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = MI_ErrorCategory(16i32);
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = MI_ErrorCategory(23i32);
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
pub const MI_FLAG_ANY: u32 = 127u32;
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
pub const MI_FLAG_CLASS: u32 = 1u32;
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
pub const MI_FLAG_IN: u32 = 8192u32;
pub const MI_FLAG_INDICATION: u32 = 32u32;
pub const MI_FLAG_KEY: u32 = 4096u32;
pub const MI_FLAG_METHOD: u32 = 2u32;
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
pub const MI_FLAG_NULL: u32 = 536870912u32;
pub const MI_FLAG_OUT: u32 = 16384u32;
pub const MI_FLAG_PARAMETER: u32 = 8u32;
pub const MI_FLAG_PROPERTY: u32 = 4u32;
pub const MI_FLAG_READONLY: u32 = 2097152u32;
pub const MI_FLAG_REFERENCE: u32 = 64u32;
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
pub const MI_FLAG_STATIC: u32 = 65536u32;
pub const MI_FLAG_STREAM: u32 = 1048576u32;
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
pub const MI_FLAG_VERSION: u32 = 469762048u32;
pub const MI_INSTANCE: MI_Type = MI_Type(15i32);
pub const MI_INSTANCEA: MI_Type = MI_Type(31i32);
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = MI_LocaleType(3i32);
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = MI_LocaleType(2i32);
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = MI_LocaleType(1i32);
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = MI_LocaleType(0i32);
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(0i32);
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(2i32);
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(1i32);
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(3i32);
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = MI_PromptType(1i32);
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = MI_PromptType(0i32);
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(0i32);
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(1i32);
pub const MI_REAL32: MI_Type = MI_Type(9i32);
pub const MI_REAL32A: MI_Type = MI_Type(25i32);
pub const MI_REAL64: MI_Type = MI_Type(10i32);
pub const MI_REAL64A: MI_Type = MI_Type(26i32);
pub const MI_REASON_NONE: MI_CancellationReason = MI_CancellationReason(0i32);
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = MI_CancellationReason(3i32);
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = MI_CancellationReason(2i32);
pub const MI_REASON_TIMEOUT: MI_CancellationReason = MI_CancellationReason(1i32);
pub const MI_REFERENCE: MI_Type = MI_Type(14i32);
pub const MI_REFERENCEA: MI_Type = MI_Type(30i32);
pub const MI_RESULT_ACCESS_DENIED: MI_Result = MI_Result(2i32);
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = MI_Result(11i32);
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = MI_Result(8i32);
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = MI_Result(9i32);
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = MI_Result(26i32);
pub const MI_RESULT_FAILED: MI_Result = MI_Result(1i32);
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = MI_Result(25i32);
pub const MI_RESULT_INVALID_CLASS: MI_Result = MI_Result(5i32);
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = MI_Result(21i32);
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = MI_Result(3i32);
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = MI_Result(22i32);
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = MI_Result(4i32);
pub const MI_RESULT_INVALID_QUERY: MI_Result = MI_Result(15i32);
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = MI_Result(10i32);
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = MI_Result(16i32);
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = MI_Result(17i32);
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = MI_Result(20i32);
pub const MI_RESULT_NOT_FOUND: MI_Result = MI_Result(6i32);
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = MI_Result(7i32);
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = MI_Result(12i32);
pub const MI_RESULT_OK: MI_Result = MI_Result(0i32);
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = MI_Result(24i32);
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = MI_Result(23i32);
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = MI_Result(14i32);
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = MI_Result(28i32);
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = MI_Result(27i32);
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = MI_Result(13i32);
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
pub const MI_SINT16: MI_Type = MI_Type(4i32);
pub const MI_SINT16A: MI_Type = MI_Type(20i32);
pub const MI_SINT32: MI_Type = MI_Type(6i32);
pub const MI_SINT32A: MI_Type = MI_Type(22i32);
pub const MI_SINT64: MI_Type = MI_Type(8i32);
pub const MI_SINT64A: MI_Type = MI_Type(24i32);
pub const MI_SINT8: MI_Type = MI_Type(2i32);
pub const MI_SINT8A: MI_Type = MI_Type(18i32);
pub const MI_STRING: MI_Type = MI_Type(13i32);
pub const MI_STRINGA: MI_Type = MI_Type(29i32);
pub const MI_SUBSCRIBE_BOOKMARK_NEWEST: windows_core::PCWSTR = windows_core::w!("MI_SUBSCRIBE_BOOKMARK_NEWEST");
pub const MI_SUBSCRIBE_BOOKMARK_OLDEST: windows_core::PCWSTR = windows_core::w!("MI_SUBSCRIBE_BOOKMARK_OLDEST");
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(1i32);
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(2i32);
pub const MI_UINT16: MI_Type = MI_Type(3i32);
pub const MI_UINT16A: MI_Type = MI_Type(19i32);
pub const MI_UINT32: MI_Type = MI_Type(5i32);
pub const MI_UINT32A: MI_Type = MI_Type(21i32);
pub const MI_UINT64: MI_Type = MI_Type(7i32);
pub const MI_UINT64A: MI_Type = MI_Type(23i32);
pub const MI_UINT8: MI_Type = MI_Type(1i32);
pub const MI_UINT8A: MI_Type = MI_Type(17i32);
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = WBEMSTATUS(-2147213309i32);
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = WBEMSTATUS(-2147213311i32);
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = WBEMSTATUS(-2147213310i32);
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = WBEMSTATUS(-2147205089i32);
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205094i32);
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147205093i32);
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205087i32);
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = WBEMSTATUS(-2147205073i32);
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = WBEMSTATUS(-2147205072i32);
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205098i32);
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205079i32);
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205100i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205116i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = WBEMSTATUS(-2147205115i32);
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205114i32);
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = WBEMSTATUS(-2147205095i32);
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205086i32);
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205117i32);
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205111i32);
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = WBEMSTATUS(-2147205108i32);
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = WBEMSTATUS(-2147205119i32);
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = WBEMSTATUS(-2147205118i32);
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = WBEMSTATUS(-2147205112i32);
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205113i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = WBEMSTATUS(-2147205085i32);
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = WBEMSTATUS(-2147205083i32);
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205104i32);
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205097i32);
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205071i32);
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205076i32);
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = WBEMSTATUS(-2147205103i32);
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = WBEMSTATUS(-2147205090i32);
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205080i32);
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205096i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = WBEMSTATUS(-2147205092i32);
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205101i32);
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = WBEMSTATUS(-2147205102i32);
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205075i32);
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = WBEMSTATUS(-2147205084i32);
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = WBEMSTATUS(-2147205081i32);
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = WBEMSTATUS(-2147205082i32);
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = WBEMSTATUS(-2147205088i32);
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147205091i32);
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = WBEMSTATUS(-2147205074i32);
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147205107i32);
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147205099i32);
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = WBEMSTATUS(-2147205106i32);
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = WBEMSTATUS(-2147205105i32);
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = WBEMSTATUS(-2147205110i32);
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205109i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205077i32);
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205078i32);
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(1i32);
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(2i32);
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(4i32);
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(1i32);
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(16i32);
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(32i32);
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(2i32);
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(8i32);
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(4i32);
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1i32);
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2048i32);
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(256i32);
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(128i32);
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2i32);
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16i32);
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(64i32);
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4i32);
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32i32);
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8i32);
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8192i32);
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4096i32);
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32768i32);
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(131072i32);
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16384i32);
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(512i32);
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1024i32);
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(65536i32);
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(2i32);
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(1i32);
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(8i32);
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(0i32);
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(1i32);
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
pub const WBEM_AUTHENTICATION_METHOD_MASK: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(15i32);
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(0i32);
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(1i32);
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(-2147217405i32);
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217315i32);
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(-2147217383i32);
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217301i32);
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217306i32);
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = WBEMSTATUS(-2147217312i32);
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = WBEMSTATUS(-2147217348i32);
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = WBEMSTATUS(-2147217358i32);
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = WBEMSTATUS(-2147217307i32);
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = WBEMSTATUS(-2147217377i32);
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = WBEMSTATUS(-2147217364i32);
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217328i32);
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217335i32);
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = WBEMSTATUS(-2147217337i32);
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = WBEMSTATUS(-2147217371i32);
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = WBEMSTATUS(-2147217370i32);
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217292i32);
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = WBEMSTATUS(-2147217305i32);
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217295i32);
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217398i32);
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217288i32);
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217273i32);
pub const WBEM_E_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217407i32);
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217274i32);
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = WBEMSTATUS(-2147217296i32);
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = WBEMSTATUS(-2147217368i32);
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217378i32);
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217376i32);
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217388i32);
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = WBEMSTATUS(-2147217302i32);
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217363i32);
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217392i32);
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = WBEMSTATUS(-2147217401i32);
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217341i32);
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = WBEMSTATUS(-2147217338i32);
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = WBEMSTATUS(-2147217294i32);
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217280i32);
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217362i32);
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = WBEMSTATUS(-2147217361i32);
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = WBEMSTATUS(-2147217394i32);
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217393i32);
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = WBEMSTATUS(-2147217350i32);
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217386i32);
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = WBEMSTATUS(-2147217309i32);
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217400i32);
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217353i32);
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217359i32);
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217366i32);
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = WBEMSTATUS(-2147217390i32);
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217342i32);
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217367i32);
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217385i32);
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217384i32);
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = WBEMSTATUS(-2147217397i32);
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = WBEMSTATUS(-2147217395i32);
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147217375i32);
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = WBEMSTATUS(-2147217308i32);
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = WBEMSTATUS(-2147217343i32);
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217344i32);
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217322i32);
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217291i32);
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217323i32);
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = WBEMSTATUS(-2147217317i32);
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = WBEMSTATUS(-2147217318i32);
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217354i32);
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = WBEMSTATUS(-2147217352i32);
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217374i32);
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(-2147217399i32);
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217319i32);
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217406i32);
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147217396i32);
pub const WBEM_E_NO_KEY: WBEMSTATUS = WBEMSTATUS(-2147217271i32);
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = WBEMSTATUS(-2147217277i32);
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = WBEMSTATUS(-2147217304i32);
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = WBEMSTATUS(-2147217349i32);
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = WBEMSTATUS(-2147217402i32);
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217382i32);
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = WBEMSTATUS(-2147217351i32);
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = WBEMSTATUS(-2147217310i32);
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217356i32);
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217380i32);
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217381i32);
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217293i32);
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217316i32);
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217276i32);
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217270i32);
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217404i32);
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217389i32);
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = WBEMSTATUS(-2147217372i32);
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217391i32);
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217275i32);
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = WBEMSTATUS(-2147217279i32);
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217272i32);
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217290i32);
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217369i32);
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = WBEMSTATUS(-2147217311i32);
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = WBEMSTATUS(-2147217300i32);
pub const WBEM_E_READ_ONLY: WBEMSTATUS = WBEMSTATUS(-2147217373i32);
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217321i32);
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = WBEMSTATUS(-2147217289i32);
pub const WBEM_E_RESERVED_001: WBEMSTATUS = WBEMSTATUS(-2147217299i32);
pub const WBEM_E_RESERVED_002: WBEMSTATUS = WBEMSTATUS(-2147217298i32);
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209214i32);
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209215i32);
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217339i32);
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = WBEMSTATUS(-2147217357i32);
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217278i32);
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217360i32);
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217303i32);
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = WBEMSTATUS(-2147217327i32);
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = WBEMSTATUS(-2147217340i32);
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217387i32);
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217403i32);
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = WBEMSTATUS(-2147217379i32);
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217313i32);
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217346i32);
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217345i32);
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217320i32);
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = WBEMSTATUS(-2147217336i32);
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217297i32);
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217355i32);
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = WBEMSTATUS(-2147217347i32);
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217325i32);
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217324i32);
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217326i32);
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147217365i32);
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = WBEMSTATUS(-2147217287i32);
pub const WBEM_E_VETO_PUT: WBEMSTATUS = WBEMSTATUS(-2147217286i32);
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(65536i32);
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING_FLAG_TYPE = WBEM_LOCKING_FLAG_TYPE(1i32);
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(0i32);
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(2i32);
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(0i32);
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(1i32);
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(0i32);
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(1i32);
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(512i32);
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(256i32);
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(256i32);
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(64i32);
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(128i32);
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(8i32);
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(2i32);
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(0i32);
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(512i32);
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(16i32);
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(256i32);
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(32i32);
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(32i32);
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(16i32);
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(8i32);
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(4i32);
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(32i32);
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(2i32);
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(1i32);
pub const WBEM_FLAG_INPROC_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(0i32);
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(4i32);
pub const WBEM_FLAG_LOCAL_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(1i32);
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(16i32);
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(2i32);
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(1i32);
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(2i32);
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(64i32);
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(64i32);
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = WBEM_TEXT_FLAG_TYPE(1i32);
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(2i32);
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = WBEM_PROVIDER_FLAGS(65536i32);
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(32i32);
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(2i32);
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(0i32);
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(1i32);
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(8i32);
pub const WBEM_FLAG_REMOTE_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(2i32);
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(128i32);
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(1i32);
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(1i32);
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(32i32);
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(256i32);
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(1048576i32);
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(48i32);
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(1i32);
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(2i32);
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(64i32);
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(1i32);
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(32i32);
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(131072i32);
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(16i32);
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(4i32);
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(2i32);
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(1i32);
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(96i32);
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(15i32);
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(32i32);
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(64i32);
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(4i32);
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(1i32);
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(2i32);
pub const WBEM_INFINITE: i32 = -1i32;
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(768i32);
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(112i32);
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(126976i32);
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(96i32);
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = WBEM_LIMITS(4096i32);
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = WBEM_LIMITS(64i32);
pub const WBEM_MAX_PATH: WBEM_LIMITS = WBEM_LIMITS(8192i32);
pub const WBEM_MAX_QUERY: WBEM_LIMITS = WBEM_LIMITS(16384i32);
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = WBEM_LIMITS(1024i32);
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(2i32);
pub const WBEM_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_NO_WAIT: i32 = 0i32;
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(8i32);
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(32i32);
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(2i32);
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(0i32);
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(1i32);
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(128i32);
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(64i32);
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(3i32);
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(1i32);
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(2i32);
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(0i32);
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(256i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(4096i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1024i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(512i32);
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2048i32);
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2i32);
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1i32);
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(262153i32);
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(262145i32);
pub const WBEM_S_DIFFERENT: WBEMSTATUS = WBEMSTATUS(262147i32);
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = WBEMSTATUS(262152i32);
pub const WBEM_S_FALSE: WBEMSTATUS = WBEMSTATUS(1i32);
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274434i32);
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(0i32);
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274433i32);
pub const WBEM_S_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = WBEMSTATUS(262149i32);
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = WBEMSTATUS(262150i32);
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = WBEMSTATUS(262160i32);
pub const WBEM_S_PENDING: WBEMSTATUS = WBEMSTATUS(262151i32);
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = WBEMSTATUS(262146i32);
pub const WBEM_S_SAME: WBEMSTATUS = WBEMSTATUS(0i32);
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(262167i32);
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274435i32);
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = WBEMSTATUS(262148i32);
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(16i32);
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(2i32);
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(3i32);
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(4i32);
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(134217728i32);
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(1i32);
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(8i32);
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1i32);
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(256i32);
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2048i32);
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(512i32);
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2i32);
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(128i32);
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(64i32);
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(4i32);
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(32i32);
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(16i32);
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1024i32);
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(10i32);
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(11i32);
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(12i32);
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(13i32);
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(14i32);
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(15i32);
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(16i32);
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(17i32);
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(18i32);
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(19i32);
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(1i32);
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(20i32);
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(21i32);
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(22i32);
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(23i32);
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(24i32);
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(25i32);
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(26i32);
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(27i32);
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(28i32);
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(29i32);
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(2i32);
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(30i32);
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(31i32);
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(32i32);
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(33i32);
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(34i32);
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(35i32);
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(36i32);
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(37i32);
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(38i32);
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(39i32);
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(3i32);
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(4i32);
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(5i32);
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(6i32);
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(7i32);
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(8i32);
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(9i32);
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(8192i32);
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(64i32);
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(32i32);
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(16i32);
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(4096i32);
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(2048i32);
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(1024i32);
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(8i32);
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(512i32);
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(128i32);
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(2i32);
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(4i32);
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(256i32);
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(1i32);
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(32i32);
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(6i32);
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(9i32);
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(11i32);
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(10i32);
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(7i32);
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(0i32);
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(16i32);
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(64i32);
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(1i32);
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = WMI_OBJ_TEXT(13i32);
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(2i32);
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = WMI_OBJ_TEXT(3i32);
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = WMI_OBJ_TEXT(12i32);
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = WMI_OBJ_TEXT(4i32);
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = WMI_OBJ_TEXT(5i32);
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = WMI_OBJ_TEXT(6i32);
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = WMI_OBJ_TEXT(7i32);
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = WMI_OBJ_TEXT(8i32);
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = WMI_OBJ_TEXT(9i32);
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = WMI_OBJ_TEXT(10i32);
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = WMI_OBJ_TEXT(11i32);
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(3i32);
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(2i32);
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(0i32);
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(1i32);
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(4i32);
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(5i32);
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(6i32);
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = WbemChangeFlagEnum(65536i32);
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(2i32);
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = WbemChangeFlagEnum(128i32);
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = WbemChangeFlagEnum(64i32);
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(1i32);
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = WbemChangeFlagEnum(32i32);
pub const wbemCimtypeBoolean: WbemCimtypeEnum = WbemCimtypeEnum(11i32);
pub const wbemCimtypeChar16: WbemCimtypeEnum = WbemCimtypeEnum(103i32);
pub const wbemCimtypeDatetime: WbemCimtypeEnum = WbemCimtypeEnum(101i32);
pub const wbemCimtypeObject: WbemCimtypeEnum = WbemCimtypeEnum(13i32);
pub const wbemCimtypeReal32: WbemCimtypeEnum = WbemCimtypeEnum(4i32);
pub const wbemCimtypeReal64: WbemCimtypeEnum = WbemCimtypeEnum(5i32);
pub const wbemCimtypeReference: WbemCimtypeEnum = WbemCimtypeEnum(102i32);
pub const wbemCimtypeSint16: WbemCimtypeEnum = WbemCimtypeEnum(2i32);
pub const wbemCimtypeSint32: WbemCimtypeEnum = WbemCimtypeEnum(3i32);
pub const wbemCimtypeSint64: WbemCimtypeEnum = WbemCimtypeEnum(20i32);
pub const wbemCimtypeSint8: WbemCimtypeEnum = WbemCimtypeEnum(16i32);
pub const wbemCimtypeString: WbemCimtypeEnum = WbemCimtypeEnum(8i32);
pub const wbemCimtypeUint16: WbemCimtypeEnum = WbemCimtypeEnum(18i32);
pub const wbemCimtypeUint32: WbemCimtypeEnum = WbemCimtypeEnum(19i32);
pub const wbemCimtypeUint64: WbemCimtypeEnum = WbemCimtypeEnum(21i32);
pub const wbemCimtypeUint8: WbemCimtypeEnum = WbemCimtypeEnum(17i32);
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = WbemComparisonFlagEnum(16i32);
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = WbemComparisonFlagEnum(8i32);
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = WbemComparisonFlagEnum(4i32);
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = WbemComparisonFlagEnum(32i32);
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = WbemComparisonFlagEnum(2i32);
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = WbemComparisonFlagEnum(1i32);
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = WbemComparisonFlagEnum(0i32);
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = WbemConnectOptionsEnum(128i32);
pub const wbemErrAccessDenied: WbemErrorEnum = WbemErrorEnum(-2147217405i32);
pub const wbemErrAggregatingByObject: WbemErrorEnum = WbemErrorEnum(-2147217315i32);
pub const wbemErrAlreadyExists: WbemErrorEnum = WbemErrorEnum(-2147217383i32);
pub const wbemErrAmbiguousOperation: WbemErrorEnum = WbemErrorEnum(-2147217301i32);
pub const wbemErrAmendedObject: WbemErrorEnum = WbemErrorEnum(-2147217306i32);
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = WbemErrorEnum(-2147217312i32);
pub const wbemErrBufferTooSmall: WbemErrorEnum = WbemErrorEnum(-2147217348i32);
pub const wbemErrCallCancelled: WbemErrorEnum = WbemErrorEnum(-2147217358i32);
pub const wbemErrCannotBeAbstract: WbemErrorEnum = WbemErrorEnum(-2147217307i32);
pub const wbemErrCannotBeKey: WbemErrorEnum = WbemErrorEnum(-2147217377i32);
pub const wbemErrCannotBeSingleton: WbemErrorEnum = WbemErrorEnum(-2147217364i32);
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = WbemErrorEnum(-2147217328i32);
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = WbemErrorEnum(-2147217335i32);
pub const wbemErrCircularReference: WbemErrorEnum = WbemErrorEnum(-2147217337i32);
pub const wbemErrClassHasChildren: WbemErrorEnum = WbemErrorEnum(-2147217371i32);
pub const wbemErrClassHasInstances: WbemErrorEnum = WbemErrorEnum(-2147217370i32);
pub const wbemErrClassNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217292i32);
pub const wbemErrClientTooSlow: WbemErrorEnum = WbemErrorEnum(-2147217305i32);
pub const wbemErrConnectionFailed: WbemErrorEnum = WbemErrorEnum(-2147217295i32);
pub const wbemErrCriticalError: WbemErrorEnum = WbemErrorEnum(-2147217398i32);
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = WbemErrorEnum(-2147217288i32);
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = WbemErrorEnum(-2147217273i32);
pub const wbemErrFailed: WbemErrorEnum = WbemErrorEnum(-2147217407i32);
pub const wbemErrFatalTransportError: WbemErrorEnum = WbemErrorEnum(-2147217274i32);
pub const wbemErrForcedRollback: WbemErrorEnum = WbemErrorEnum(-2147217298i32);
pub const wbemErrHandleOutOfDate: WbemErrorEnum = WbemErrorEnum(-2147217296i32);
pub const wbemErrIllegalNull: WbemErrorEnum = WbemErrorEnum(-2147217368i32);
pub const wbemErrIllegalOperation: WbemErrorEnum = WbemErrorEnum(-2147217378i32);
pub const wbemErrIncompleteClass: WbemErrorEnum = WbemErrorEnum(-2147217376i32);
pub const wbemErrInitializationFailure: WbemErrorEnum = WbemErrorEnum(-2147217388i32);
pub const wbemErrInvalidAssociation: WbemErrorEnum = WbemErrorEnum(-2147217302i32);
pub const wbemErrInvalidCimType: WbemErrorEnum = WbemErrorEnum(-2147217363i32);
pub const wbemErrInvalidClass: WbemErrorEnum = WbemErrorEnum(-2147217392i32);
pub const wbemErrInvalidContext: WbemErrorEnum = WbemErrorEnum(-2147217401i32);
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = WbemErrorEnum(-2147217341i32);
pub const wbemErrInvalidFlavor: WbemErrorEnum = WbemErrorEnum(-2147217338i32);
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = WbemErrorEnum(-2147217294i32);
pub const wbemErrInvalidLocale: WbemErrorEnum = WbemErrorEnum(-2147217280i32);
pub const wbemErrInvalidMethod: WbemErrorEnum = WbemErrorEnum(-2147217362i32);
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = WbemErrorEnum(-2147217361i32);
pub const wbemErrInvalidNamespace: WbemErrorEnum = WbemErrorEnum(-2147217394i32);
pub const wbemErrInvalidObject: WbemErrorEnum = WbemErrorEnum(-2147217393i32);
pub const wbemErrInvalidObjectPath: WbemErrorEnum = WbemErrorEnum(-2147217350i32);
pub const wbemErrInvalidOperation: WbemErrorEnum = WbemErrorEnum(-2147217386i32);
pub const wbemErrInvalidOperator: WbemErrorEnum = WbemErrorEnum(-2147217309i32);
pub const wbemErrInvalidParameter: WbemErrorEnum = WbemErrorEnum(-2147217400i32);
pub const wbemErrInvalidParameterId: WbemErrorEnum = WbemErrorEnum(-2147217353i32);
pub const wbemErrInvalidProperty: WbemErrorEnum = WbemErrorEnum(-2147217359i32);
pub const wbemErrInvalidPropertyType: WbemErrorEnum = WbemErrorEnum(-2147217366i32);
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = WbemErrorEnum(-2147217390i32);
pub const wbemErrInvalidQualifier: WbemErrorEnum = WbemErrorEnum(-2147217342i32);
pub const wbemErrInvalidQualifierType: WbemErrorEnum = WbemErrorEnum(-2147217367i32);
pub const wbemErrInvalidQuery: WbemErrorEnum = WbemErrorEnum(-2147217385i32);
pub const wbemErrInvalidQueryType: WbemErrorEnum = WbemErrorEnum(-2147217384i32);
pub const wbemErrInvalidStream: WbemErrorEnum = WbemErrorEnum(-2147217397i32);
pub const wbemErrInvalidSuperclass: WbemErrorEnum = WbemErrorEnum(-2147217395i32);
pub const wbemErrInvalidSyntax: WbemErrorEnum = WbemErrorEnum(-2147217375i32);
pub const wbemErrLocalCredentials: WbemErrorEnum = WbemErrorEnum(-2147217308i32);
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = WbemErrorEnum(-2147217343i32);
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = WbemErrorEnum(-2147217344i32);
pub const wbemErrMethodDisabled: WbemErrorEnum = WbemErrorEnum(-2147217322i32);
pub const wbemErrMethodNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217291i32);
pub const wbemErrMethodNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217323i32);
pub const wbemErrMissingAggregationList: WbemErrorEnum = WbemErrorEnum(-2147217317i32);
pub const wbemErrMissingGroupWithin: WbemErrorEnum = WbemErrorEnum(-2147217318i32);
pub const wbemErrMissingParameter: WbemErrorEnum = WbemErrorEnum(-2147217354i32);
pub const wbemErrNoSchema: WbemErrorEnum = WbemErrorEnum(-2147217277i32);
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = WbemErrorEnum(-2147217352i32);
pub const wbemErrNondecoratedObject: WbemErrorEnum = WbemErrorEnum(-2147217374i32);
pub const wbemErrNotAvailable: WbemErrorEnum = WbemErrorEnum(-2147217399i32);
pub const wbemErrNotEventClass: WbemErrorEnum = WbemErrorEnum(-2147217319i32);
pub const wbemErrNotFound: WbemErrorEnum = WbemErrorEnum(-2147217406i32);
pub const wbemErrNotSupported: WbemErrorEnum = WbemErrorEnum(-2147217396i32);
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = WbemErrorEnum(-2147217304i32);
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = WbemErrorEnum(-2147217349i32);
pub const wbemErrOutOfMemory: WbemErrorEnum = WbemErrorEnum(-2147217402i32);
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217382i32);
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = WbemErrorEnum(-2147217351i32);
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = WbemErrorEnum(-2147217310i32);
pub const wbemErrPropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217356i32);
pub const wbemErrPropagatedProperty: WbemErrorEnum = WbemErrorEnum(-2147217380i32);
pub const wbemErrPropagatedQualifier: WbemErrorEnum = WbemErrorEnum(-2147217381i32);
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217293i32);
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = WbemErrorEnum(-2147217316i32);
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = WbemErrorEnum(-2147217276i32);
pub const wbemErrProviderFailure: WbemErrorEnum = WbemErrorEnum(-2147217404i32);
pub const wbemErrProviderLoadFailure: WbemErrorEnum = WbemErrorEnum(-2147217389i32);
pub const wbemErrProviderNotCapable: WbemErrorEnum = WbemErrorEnum(-2147217372i32);
pub const wbemErrProviderNotFound: WbemErrorEnum = WbemErrorEnum(-2147217391i32);
pub const wbemErrProviderNotRegistered: WbemErrorEnum = WbemErrorEnum(-2147217275i32);
pub const wbemErrProviderSuspended: WbemErrorEnum = WbemErrorEnum(-2147217279i32);
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217290i32);
pub const wbemErrQueryNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217369i32);
pub const wbemErrQueueOverflow: WbemErrorEnum = WbemErrorEnum(-2147217311i32);
pub const wbemErrQuotaViolation: WbemErrorEnum = WbemErrorEnum(-2147217300i32);
pub const wbemErrReadOnly: WbemErrorEnum = WbemErrorEnum(-2147217373i32);
pub const wbemErrRefresherBusy: WbemErrorEnum = WbemErrorEnum(-2147217321i32);
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = WbemErrorEnum(-2147213311i32);
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = WbemErrorEnum(-2147213310i32);
pub const wbemErrRerunCommand: WbemErrorEnum = WbemErrorEnum(-2147217289i32);
pub const wbemErrResetToDefault: WbemErrorEnum = WbemErrorEnum(-2147209214i32);
pub const wbemErrServerTooBusy: WbemErrorEnum = WbemErrorEnum(-2147217339i32);
pub const wbemErrShuttingDown: WbemErrorEnum = WbemErrorEnum(-2147217357i32);
pub const wbemErrSynchronizationRequired: WbemErrorEnum = WbemErrorEnum(-2147217278i32);
pub const wbemErrSystemProperty: WbemErrorEnum = WbemErrorEnum(-2147217360i32);
pub const wbemErrTimedout: WbemErrorEnum = WbemErrorEnum(-2147209215i32);
pub const wbemErrTimeout: WbemErrorEnum = WbemErrorEnum(-2147217303i32);
pub const wbemErrTooManyProperties: WbemErrorEnum = WbemErrorEnum(-2147217327i32);
pub const wbemErrTooMuchData: WbemErrorEnum = WbemErrorEnum(-2147217340i32);
pub const wbemErrTransactionConflict: WbemErrorEnum = WbemErrorEnum(-2147217299i32);
pub const wbemErrTransportFailure: WbemErrorEnum = WbemErrorEnum(-2147217387i32);
pub const wbemErrTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217403i32);
pub const wbemErrUnexpected: WbemErrorEnum = WbemErrorEnum(-2147217379i32);
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = WbemErrorEnum(-2147217313i32);
pub const wbemErrUnknownObjectType: WbemErrorEnum = WbemErrorEnum(-2147217346i32);
pub const wbemErrUnknownPacketType: WbemErrorEnum = WbemErrorEnum(-2147217345i32);
pub const wbemErrUnparsableQuery: WbemErrorEnum = WbemErrorEnum(-2147217320i32);
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = WbemErrorEnum(-2147217336i32);
pub const wbemErrUnsupportedLocale: WbemErrorEnum = WbemErrorEnum(-2147217297i32);
pub const wbemErrUnsupportedParameter: WbemErrorEnum = WbemErrorEnum(-2147217355i32);
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = WbemErrorEnum(-2147217347i32);
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217325i32);
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217324i32);
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217326i32);
pub const wbemErrValueOutOfRange: WbemErrorEnum = WbemErrorEnum(-2147217365i32);
pub const wbemErrVetoDelete: WbemErrorEnum = WbemErrorEnum(-2147217286i32);
pub const wbemErrVetoPut: WbemErrorEnum = WbemErrorEnum(-2147217287i32);
pub const wbemFlagBidirectional: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagDirectRead: WbemFlagEnum = WbemFlagEnum(512i32);
pub const wbemFlagDontSendStatus: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagEnsureLocatable: WbemFlagEnum = WbemFlagEnum(256i32);
pub const wbemFlagForwardOnly: WbemFlagEnum = WbemFlagEnum(32i32);
pub const wbemFlagGetDefault: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagNoErrorObject: WbemFlagEnum = WbemFlagEnum(64i32);
pub const wbemFlagReturnErrorObject: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagReturnImmediately: WbemFlagEnum = WbemFlagEnum(16i32);
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSendOnlySelected: WbemFlagEnum = WbemFlagEnum(0i32);
pub const wbemFlagSendStatus: WbemFlagEnum = WbemFlagEnum(128i32);
pub const wbemFlagSpawnInstance: WbemFlagEnum = WbemFlagEnum(1i32);
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = WbemFlagEnum(131072i32);
pub const wbemFlagUseCurrentTime: WbemFlagEnum = WbemFlagEnum(1i32);
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(1i32);
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(4i32);
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(2i32);
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(3i32);
pub const wbemNoErr: WbemErrorEnum = WbemErrorEnum(0i32);
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(1i32);
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(2i32);
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = WbemPrivilegeEnum(20i32);
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = WbemPrivilegeEnum(16i32);
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = WbemPrivilegeEnum(22i32);
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = WbemPrivilegeEnum(14i32);
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = WbemPrivilegeEnum(15i32);
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = WbemPrivilegeEnum(1i32);
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = WbemPrivilegeEnum(19i32);
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = WbemPrivilegeEnum(26i32);
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = WbemPrivilegeEnum(13i32);
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = WbemPrivilegeEnum(4i32);
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = WbemPrivilegeEnum(9i32);
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = WbemPrivilegeEnum(3i32);
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = WbemPrivilegeEnum(5i32);
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = WbemPrivilegeEnum(27i32);
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = WbemPrivilegeEnum(2i32);
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = WbemPrivilegeEnum(12i32);
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(23i32);
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = WbemPrivilegeEnum(17i32);
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = WbemPrivilegeEnum(7i32);
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(18i32);
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = WbemPrivilegeEnum(25i32);
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = WbemPrivilegeEnum(21i32);
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = WbemPrivilegeEnum(10i32);
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = WbemPrivilegeEnum(11i32);
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = WbemPrivilegeEnum(8i32);
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = WbemPrivilegeEnum(6i32);
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = WbemPrivilegeEnum(24i32);
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = WbemQueryFlagEnum(0i32);
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = WbemQueryFlagEnum(2i32);
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = WbemQueryFlagEnum(1i32);
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = WbemTextFlagEnum(1i32);
pub const wbemTimeoutInfinite: WbemTimeout = WbemTimeout(-1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CIMTYPE_ENUMERATION(pub i32);
impl windows_core::TypeKind for CIMTYPE_ENUMERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CIMTYPE_ENUMERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CIMTYPE_ENUMERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_CallbackMode(pub i32);
impl windows_core::TypeKind for MI_CallbackMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_CallbackMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_CallbackMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_CancellationReason(pub i32);
impl windows_core::TypeKind for MI_CancellationReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_CancellationReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_CancellationReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_DestinationOptions_ImpersonationType(pub i32);
impl windows_core::TypeKind for MI_DestinationOptions_ImpersonationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_DestinationOptions_ImpersonationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_DestinationOptions_ImpersonationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_ErrorCategory(pub i32);
impl windows_core::TypeKind for MI_ErrorCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_ErrorCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_ErrorCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_LocaleType(pub i32);
impl windows_core::TypeKind for MI_LocaleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_LocaleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_LocaleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_OperationCallback_ResponseType(pub i32);
impl windows_core::TypeKind for MI_OperationCallback_ResponseType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_OperationCallback_ResponseType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_OperationCallback_ResponseType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_PromptType(pub i32);
impl windows_core::TypeKind for MI_PromptType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_PromptType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_PromptType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_ProviderArchitecture(pub i32);
impl windows_core::TypeKind for MI_ProviderArchitecture {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_ProviderArchitecture {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_ProviderArchitecture").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_Result(pub i32);
impl windows_core::TypeKind for MI_Result {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_Result {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_Result").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_SubscriptionDeliveryType(pub i32);
impl windows_core::TypeKind for MI_SubscriptionDeliveryType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_SubscriptionDeliveryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_SubscriptionDeliveryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MI_Type(pub i32);
impl windows_core::TypeKind for MI_Type {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MI_Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MI_Type").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEMSTATUS(pub i32);
impl windows_core::TypeKind for WBEMSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEMSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEMSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEMSTATUS_FORMAT(pub i32);
impl windows_core::TypeKind for WBEMSTATUS_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEMSTATUS_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEMSTATUS_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_BACKUP_RESTORE_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_BACKUP_RESTORE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_BACKUP_RESTORE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_BACKUP_RESTORE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_BATCH_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_BATCH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_BATCH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_BATCH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_CHANGE_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_CHANGE_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_CHANGE_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_CHANGE_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_COMPARISON_FLAG(pub i32);
impl windows_core::TypeKind for WBEM_COMPARISON_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_COMPARISON_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_COMPARISON_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_COMPILER_OPTIONS(pub i32);
impl windows_core::TypeKind for WBEM_COMPILER_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_COMPILER_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_COMPILER_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_CONDITION_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_CONDITION_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_CONDITION_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_CONDITION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_CONNECT_OPTIONS(pub i32);
impl windows_core::TypeKind for WBEM_CONNECT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_CONNECT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_CONNECT_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_EXTRA_RETURN_CODES(pub i32);
impl windows_core::TypeKind for WBEM_EXTRA_RETURN_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_EXTRA_RETURN_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_EXTRA_RETURN_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_FLAVOR_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_FLAVOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_FLAVOR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_FLAVOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_GENERIC_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_GENERIC_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_GENERIC_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_GENERIC_FLAG_TYPE").field(&self.0).finish()
    }
}
impl WBEM_GENERIC_FLAG_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_GENUS_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_GENUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_GENUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_GENUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_GET_KEY_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_GET_KEY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_GET_KEY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_GET_KEY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_GET_TEXT_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_GET_TEXT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_GET_TEXT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_GET_TEXT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_INFORMATION_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_INFORMATION_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_INFORMATION_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_INFORMATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_LIMITATION_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_LIMITATION_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_LIMITATION_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_LIMITATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_LIMITS(pub i32);
impl windows_core::TypeKind for WBEM_LIMITS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_LIMITS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_LIMITS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_LOCKING_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_LOCKING_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_LOCKING_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_LOCKING_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_LOGIN_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_LOGIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_LOGIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_LOGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_PATH_CREATE_FLAG(pub i32);
impl windows_core::TypeKind for WBEM_PATH_CREATE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_PATH_CREATE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_PATH_CREATE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_PATH_STATUS_FLAG(pub i32);
impl windows_core::TypeKind for WBEM_PATH_STATUS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_PATH_STATUS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_PATH_STATUS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_PROVIDER_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_PROVIDER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_REQUIREMENTS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_QUERY_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_QUERY_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_QUERY_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_QUERY_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_REFRESHER_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_REFRESHER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_REFRESHER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_REFRESHER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_SECURITY_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_SECURITY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_SECURITY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_SECURITY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_SHUTDOWN_FLAGS(pub i32);
impl windows_core::TypeKind for WBEM_SHUTDOWN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_STATUS_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_STATUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_STATUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_STATUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_TEXT_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_TEXT_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_TEXT_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_TEXT_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WBEM_UNSECAPP_FLAG_TYPE(pub i32);
impl windows_core::TypeKind for WBEM_UNSECAPP_FLAG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WBEM_UNSECAPP_FLAG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WBEM_UNSECAPP_FLAG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMIQ_ANALYSIS_TYPE(pub i32);
impl windows_core::TypeKind for WMIQ_ANALYSIS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMIQ_ANALYSIS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMIQ_ANALYSIS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMIQ_ASSOCQ_FLAGS(pub i32);
impl windows_core::TypeKind for WMIQ_ASSOCQ_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMIQ_ASSOCQ_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMIQ_ASSOCQ_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMIQ_LANGUAGE_FEATURES(pub i32);
impl windows_core::TypeKind for WMIQ_LANGUAGE_FEATURES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMIQ_LANGUAGE_FEATURES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMIQ_LANGUAGE_FEATURES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMIQ_RPNF_FEATURE(pub i32);
impl windows_core::TypeKind for WMIQ_RPNF_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMIQ_RPNF_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMIQ_RPNF_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMIQ_RPN_TOKEN_FLAGS(pub i32);
impl windows_core::TypeKind for WMIQ_RPN_TOKEN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMIQ_RPN_TOKEN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMIQ_RPN_TOKEN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMI_OBJ_TEXT(pub i32);
impl windows_core::TypeKind for WMI_OBJ_TEXT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMI_OBJ_TEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMI_OBJ_TEXT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemAuthenticationLevelEnum(pub i32);
impl windows_core::TypeKind for WbemAuthenticationLevelEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemAuthenticationLevelEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemAuthenticationLevelEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemChangeFlagEnum(pub i32);
impl windows_core::TypeKind for WbemChangeFlagEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemChangeFlagEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemChangeFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemCimtypeEnum(pub i32);
impl windows_core::TypeKind for WbemCimtypeEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemCimtypeEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemCimtypeEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemComparisonFlagEnum(pub i32);
impl windows_core::TypeKind for WbemComparisonFlagEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemComparisonFlagEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemComparisonFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemConnectOptionsEnum(pub i32);
impl windows_core::TypeKind for WbemConnectOptionsEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemConnectOptionsEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemConnectOptionsEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemErrorEnum(pub i32);
impl windows_core::TypeKind for WbemErrorEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemErrorEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemErrorEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemFlagEnum(pub i32);
impl windows_core::TypeKind for WbemFlagEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemFlagEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemImpersonationLevelEnum(pub i32);
impl windows_core::TypeKind for WbemImpersonationLevelEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemImpersonationLevelEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemImpersonationLevelEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemObjectTextFormatEnum(pub i32);
impl windows_core::TypeKind for WbemObjectTextFormatEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemObjectTextFormatEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemObjectTextFormatEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemPrivilegeEnum(pub i32);
impl windows_core::TypeKind for WbemPrivilegeEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemPrivilegeEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemPrivilegeEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemQueryFlagEnum(pub i32);
impl windows_core::TypeKind for WbemQueryFlagEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemQueryFlagEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemQueryFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemTextFlagEnum(pub i32);
impl windows_core::TypeKind for WbemTextFlagEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemTextFlagEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemTextFlagEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WbemTimeout(pub i32);
impl windows_core::TypeKind for WbemTimeout {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WbemTimeout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WbemTimeout").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ApplicationFT,
}
impl windows_core::TypeKind for MI_Application {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Application {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ApplicationFT {
    pub Close: isize,
    pub NewSession: isize,
    pub NewHostedProvider: isize,
    pub NewInstance: isize,
    pub NewDestinationOptions: isize,
    pub NewOperationOptions: isize,
    pub NewSubscriptionDeliveryOptions: isize,
    pub NewSerializer: isize,
    pub NewDeserializer: isize,
    pub NewInstanceFromClass: isize,
    pub NewClass: isize,
}
impl windows_core::TypeKind for MI_ApplicationFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ApplicationFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Array {
    pub data: *mut core::ffi::c_void,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Array {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Array {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ArrayField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ArrayField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_BooleanA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_BooleanAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_BooleanAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_BooleanField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_BooleanField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Char16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Char16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Char16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Char16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Char16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Char16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Class {
    pub ft: *const MI_ClassFT,
    pub classDecl: *const MI_ClassDecl,
    pub namespaceName: *const u16,
    pub serverName: *const u16,
    pub reserved: [isize; 4],
}
impl windows_core::TypeKind for MI_Class {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Class {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *const u16,
    pub superClassDecl: *const MI_ClassDecl,
    pub methods: *const *const MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *const MI_SchemaDecl,
    pub providerFT: *const MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl windows_core::TypeKind for MI_ClassDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ClassFT {
    pub GetClassNameA: isize,
    pub GetNameSpace: isize,
    pub GetServerName: isize,
    pub GetElementCount: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub GetClassQualifierSet: isize,
    pub GetMethodCount: isize,
    pub GetMethodAt: isize,
    pub GetMethod: isize,
    pub GetParentClassName: isize,
    pub GetParentClass: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl windows_core::TypeKind for MI_ClassFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ClassFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *const MI_ApplicationFT,
    pub sessionFT: *const MI_SessionFT,
    pub operationFT: *const MI_OperationFT,
    pub hostedProviderFT: *const MI_HostedProviderFT,
    pub serializerFT: *const MI_SerializerFT,
    pub deserializerFT: *const MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *const MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *const MI_DestinationOptionsFT,
    pub operationOptionsFT: *const MI_OperationOptionsFT,
    pub utilitiesFT: *const MI_UtilitiesFT,
}
impl windows_core::TypeKind for MI_ClientFT_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstBooleanA {
    pub data: *const u8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstBooleanA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstBooleanAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstBooleanAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstBooleanField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstBooleanField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstChar16A {
    pub data: *const u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstChar16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstChar16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstChar16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstChar16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstChar16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstDatetimeA {
    pub data: *const MI_Datetime,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstDatetimeA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstDatetimeAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstDatetimeAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstDatetimeField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstInstanceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstInstanceA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstInstanceAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstInstanceAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstInstanceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstInstanceField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReal32A {
    pub data: *const f32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstReal32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReal32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReal32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReal64A {
    pub data: *const f64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstReal64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReal64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReal64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReal64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReferenceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstReferenceA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReferenceAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReferenceAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstReferenceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstReferenceField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint16A {
    pub data: *const i16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstSint16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint32A {
    pub data: *const i32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstSint32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint64A {
    pub data: *const i64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstSint64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint8A {
    pub data: *const i8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstSint8A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint8AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint8AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstSint8Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstSint8Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstStringA {
    pub data: *const *const u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstStringA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstStringAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstStringAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstStringField {
    pub value: *const u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstStringField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint16A {
    pub data: *const u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstUint16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint32A {
    pub data: *const u32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstUint32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint64A {
    pub data: *const u64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstUint64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint8A {
    pub data: *const u8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ConstUint8A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint8AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint8AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ConstUint8Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ConstUint8Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Context {
    pub ft: *const MI_ContextFT,
    pub reserved: [isize; 3],
}
impl windows_core::TypeKind for MI_Context {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ContextFT {
    pub PostResult: isize,
    pub PostInstance: isize,
    pub PostIndication: isize,
    pub ConstructInstance: isize,
    pub ConstructParameters: isize,
    pub NewInstance: isize,
    pub NewDynamicInstance: isize,
    pub NewParameters: isize,
    pub Canceled: isize,
    pub GetLocale: isize,
    pub RegisterCancel: isize,
    pub RequestUnload: isize,
    pub RefuseUnload: isize,
    pub GetLocalSession: isize,
    pub SetStringOption: isize,
    pub GetStringOption: isize,
    pub GetNumberOption: isize,
    pub GetCustomOption: isize,
    pub GetCustomOptionCount: isize,
    pub GetCustomOptionAt: isize,
    pub WriteMessage: isize,
    pub WriteProgress: isize,
    pub WriteStreamParameter: isize,
    pub WriteCimError: isize,
    pub PromptUser: isize,
    pub ShouldProcess: isize,
    pub ShouldContinue: isize,
    pub PostError: isize,
    pub PostCimError: isize,
    pub WriteError: isize,
}
impl windows_core::TypeKind for MI_ContextFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ContextFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
}
impl windows_core::TypeKind for MI_Datetime {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Datetime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MI_Datetime_0 {
    pub timestamp: MI_Timestamp,
    pub interval: MI_Interval,
}
impl windows_core::TypeKind for MI_Datetime_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Datetime_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl windows_core::TypeKind for MI_DatetimeA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_DatetimeAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DatetimeAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_DatetimeField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl windows_core::TypeKind for MI_Deserializer {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Deserializer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
impl windows_core::TypeKind for MI_DeserializerFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DeserializerFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_DestinationOptionsFT,
}
impl windows_core::TypeKind for MI_DestinationOptions {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_DestinationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub AddCredentials: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl windows_core::TypeKind for MI_DestinationOptionsFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_DestinationOptionsFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
}
impl windows_core::TypeKind for MI_FeatureDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Filter {
    pub ft: *const MI_FilterFT,
    pub reserved: [isize; 3],
}
impl windows_core::TypeKind for MI_Filter {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Filter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
impl windows_core::TypeKind for MI_FilterFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_FilterFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_HostedProviderFT,
}
impl windows_core::TypeKind for MI_HostedProvider {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
impl windows_core::TypeKind for MI_HostedProviderFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_HostedProviderFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Instance {
    pub ft: *const MI_InstanceFT,
    pub classDecl: *const MI_ClassDecl,
    pub serverName: *const u16,
    pub nameSpace: *const u16,
    pub reserved: [isize; 4],
}
impl windows_core::TypeKind for MI_Instance {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Instance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl windows_core::TypeKind for MI_InstanceA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_InstanceAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_InstanceAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
impl windows_core::TypeKind for MI_InstanceExFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_InstanceExFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_InstanceFT {
    pub Clone: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub IsA: isize,
    pub GetClassNameA: isize,
    pub SetNameSpace: isize,
    pub GetNameSpace: isize,
    pub GetElementCount: isize,
    pub AddElement: isize,
    pub SetElement: isize,
    pub SetElementAt: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub ClearElement: isize,
    pub ClearElementAt: isize,
    pub GetServerName: isize,
    pub SetServerName: isize,
    pub GetClass: isize,
}
impl windows_core::TypeKind for MI_InstanceFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_InstanceFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_InstanceField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Interval {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub microseconds: u32,
    pub __padding1: u32,
    pub __padding2: u32,
    pub __padding3: u32,
}
impl windows_core::TypeKind for MI_Interval {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Interval {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *const *const MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub schema: *const MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl windows_core::TypeKind for MI_MethodDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *const MI_ProviderFT,
}
impl windows_core::TypeKind for MI_Module {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Module {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MI_Module_Self(pub isize);
impl Default for MI_Module_Self {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MI_Module_Self {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ObjectDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationFT,
}
impl windows_core::TypeKind for MI_Operation {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Operation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MI_OperationCallbacks {
    pub callbackContext: *mut core::ffi::c_void,
    pub promptUser: MI_OperationCallback_PromptUser,
    pub writeError: MI_OperationCallback_WriteError,
    pub writeMessage: MI_OperationCallback_WriteMessage,
    pub writeProgress: MI_OperationCallback_WriteProgress,
    pub instanceResult: MI_OperationCallback_Instance,
    pub indicationResult: MI_OperationCallback_Indication,
    pub classResult: MI_OperationCallback_Class,
    pub streamedParameterResult: MI_OperationCallback_StreamedParameter,
}
impl windows_core::TypeKind for MI_OperationCallbacks {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
impl windows_core::TypeKind for MI_OperationFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_OperationFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationOptionsFT,
}
impl windows_core::TypeKind for MI_OperationOptions {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_OperationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetCustomOption: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetEnabledChannels: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl windows_core::TypeKind for MI_OperationOptionsFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_OperationOptionsFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
}
impl windows_core::TypeKind for MI_ParameterDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ParameterSetFT,
}
impl windows_core::TypeKind for MI_ParameterSet {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
impl windows_core::TypeKind for MI_ParameterSetFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ParameterSetFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub value: *const core::ffi::c_void,
}
impl windows_core::TypeKind for MI_PropertyDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_PropertySet {
    pub ft: *const MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl windows_core::TypeKind for MI_PropertySet {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_PropertySetFT {
    pub GetElementCount: isize,
    pub ContainsElement: isize,
    pub AddElement: isize,
    pub GetElementAt: isize,
    pub Clear: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl windows_core::TypeKind for MI_PropertySetFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_PropertySetFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MI_ProviderFT {
    pub Load: MI_ProviderFT_Load,
    pub Unload: MI_ProviderFT_Unload,
    pub GetInstance: MI_ProviderFT_GetInstance,
    pub EnumerateInstances: MI_ProviderFT_EnumerateInstances,
    pub CreateInstance: MI_ProviderFT_CreateInstance,
    pub ModifyInstance: MI_ProviderFT_ModifyInstance,
    pub DeleteInstance: MI_ProviderFT_DeleteInstance,
    pub AssociatorInstances: MI_ProviderFT_AssociatorInstances,
    pub ReferenceInstances: MI_ProviderFT_ReferenceInstances,
    pub EnableIndications: MI_ProviderFT_EnableIndications,
    pub DisableIndications: MI_ProviderFT_DisableIndications,
    pub Subscribe: MI_ProviderFT_Subscribe,
    pub Unsubscribe: MI_ProviderFT_Unsubscribe,
    pub Invoke: MI_ProviderFT_Invoke,
}
impl windows_core::TypeKind for MI_ProviderFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ProviderFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Qualifier {
    pub name: *const u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *const core::ffi::c_void,
}
impl windows_core::TypeKind for MI_Qualifier {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_QualifierDecl {
    pub name: *const u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *const core::ffi::c_void,
}
impl windows_core::TypeKind for MI_QualifierDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_QualifierSetFT,
}
impl windows_core::TypeKind for MI_QualifierSet {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
impl windows_core::TypeKind for MI_QualifierSetFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_QualifierSetFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Real32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Real32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Real32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Real64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Real64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Real64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Real64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl windows_core::TypeKind for MI_ReferenceA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ReferenceAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ReferenceAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_ReferenceField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *const *const MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *const *const MI_ClassDecl,
    pub numClassDecls: u32,
}
impl windows_core::TypeKind for MI_SchemaDecl {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl windows_core::TypeKind for MI_Serializer {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Serializer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
impl windows_core::TypeKind for MI_SerializerFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SerializerFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Server {
    pub serverFT: *const MI_ServerFT,
    pub contextFT: *const MI_ContextFT,
    pub instanceFT: *const MI_InstanceFT,
    pub propertySetFT: *const MI_PropertySetFT,
    pub filterFT: *const MI_FilterFT,
}
impl windows_core::TypeKind for MI_Server {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Server {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
impl windows_core::TypeKind for MI_ServerFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_ServerFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SessionFT,
}
impl windows_core::TypeKind for MI_Session {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Session {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl windows_core::TypeKind for MI_SessionCallbacks {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SessionFT {
    pub Close: isize,
    pub GetApplication: isize,
    pub GetInstance: isize,
    pub ModifyInstance: isize,
    pub CreateInstance: isize,
    pub DeleteInstance: isize,
    pub Invoke: isize,
    pub EnumerateInstances: isize,
    pub QueryInstances: isize,
    pub AssociatorInstances: isize,
    pub ReferenceInstances: isize,
    pub Subscribe: isize,
    pub GetClass: isize,
    pub EnumerateClasses: isize,
    pub TestConnection: isize,
}
impl windows_core::TypeKind for MI_SessionFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SessionFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Sint16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Sint32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Sint64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Sint8A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint8AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint8AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Sint8Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Sint8Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_StringA {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_StringA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_StringAField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_StringAField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_StringField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_StringField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SubscriptionDeliveryOptionsFT,
}
impl windows_core::TypeKind for MI_SubscriptionDeliveryOptions {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_SubscriptionDeliveryOptionsFT {
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetDateTime: isize,
    pub SetInterval: isize,
    pub AddCredentials: isize,
    pub Delete: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetDateTime: isize,
    pub GetInterval: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
}
impl windows_core::TypeKind for MI_SubscriptionDeliveryOptionsFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_SubscriptionDeliveryOptionsFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microseconds: u32,
    pub utc: i32,
}
impl windows_core::TypeKind for MI_Timestamp {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Timestamp {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Uint16A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint16AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint16AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint16Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint16Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Uint32A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint32AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint32AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint32Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint32Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Uint64A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint64AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint64AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint64Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint64Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl windows_core::TypeKind for MI_Uint8A {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint8AField {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint8AField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl windows_core::TypeKind for MI_Uint8Field {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Uint8Field {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_UserCredentials {
    pub authenticationType: *const u16,
    pub credentials: MI_UserCredentials_0,
}
impl windows_core::TypeKind for MI_UserCredentials {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_UserCredentials {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *const u16,
}
impl windows_core::TypeKind for MI_UserCredentials_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_UserCredentials_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_UsernamePasswordCreds {
    pub domain: *const u16,
    pub username: *const u16,
    pub password: *const u16,
}
impl windows_core::TypeKind for MI_UsernamePasswordCreds {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
}
impl windows_core::TypeKind for MI_UtilitiesFT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_UtilitiesFT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MI_Value {
    pub boolean: u8,
    pub uint8: u8,
    pub sint8: i8,
    pub uint16: u16,
    pub sint16: i16,
    pub uint32: u32,
    pub sint32: i32,
    pub uint64: u64,
    pub sint64: i64,
    pub real32: f32,
    pub real64: f64,
    pub char16: u16,
    pub datetime: MI_Datetime,
    pub string: *mut u16,
    pub instance: *mut MI_Instance,
    pub reference: *mut MI_Instance,
    pub booleana: MI_BooleanA,
    pub uint8a: MI_Uint8A,
    pub sint8a: MI_Sint8A,
    pub uint16a: MI_Uint16A,
    pub sint16a: MI_Sint16A,
    pub uint32a: MI_Uint32A,
    pub sint32a: MI_Sint32A,
    pub uint64a: MI_Uint64A,
    pub sint64a: MI_Sint64A,
    pub real32a: MI_Real32A,
    pub real64a: MI_Real64A,
    pub char16a: MI_Char16A,
    pub datetimea: MI_DatetimeA,
    pub stringa: MI_StringA,
    pub referencea: MI_ReferenceA,
    pub instancea: MI_InstanceA,
    pub array: MI_Array,
}
impl windows_core::TypeKind for MI_Value {
    type TypeKind = windows_core::CopyType;
}
impl Default for MI_Value {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MofCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: windows_core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut core::ffi::c_void,
    pub m_pbTruthTable: *mut super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SWbemAnalysisMatrix {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
impl windows_core::TypeKind for SWbemAnalysisMatrixList {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: core::mem::ManuallyDrop<Option<IWbemPath>>,
    pub m_pszPath: windows_core::PWSTR,
    pub m_pszQueryText: windows_core::PWSTR,
    pub m_pszResultClass: windows_core::PWSTR,
    pub m_pszAssocClass: windows_core::PWSTR,
    pub m_pszRole: windows_core::PWSTR,
    pub m_pszResultRole: windows_core::PWSTR,
    pub m_pszRequiredQualifier: windows_core::PWSTR,
    pub m_pszRequiredAssocQualifier: windows_core::PWSTR,
}
impl Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SWbemAssocQueryInf {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SWbemDateTime: windows_core::GUID = windows_core::GUID::from_u128(0x47dfbe54_cf76_11d3_b38f_00105a1f473a);
pub const SWbemEventSource: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d58_21ae_11d2_8b33_00600806d9b6);
pub const SWbemLastError: windows_core::GUID = windows_core::GUID::from_u128(0xc2feeeac_cfcd_11d1_8b05_00600806d9b6);
pub const SWbemLocator: windows_core::GUID = windows_core::GUID::from_u128(0x76a64158_cb41_11d1_8b02_00600806d9b6);
pub const SWbemMethod: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5b_21ae_11d2_8b33_00600806d9b6);
pub const SWbemMethodSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5a_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValue: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValueSet: windows_core::GUID = windows_core::GUID::from_u128(0x9aed384e_ce8b_11d1_8b05_00600806d9b6);
pub const SWbemObject: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d62_21ae_11d2_8b33_00600806d9b6);
pub const SWbemObjectEx: windows_core::GUID = windows_core::GUID::from_u128(0xd6bdafb2_9435_491f_bb87_6aa0f0bc31a2);
pub const SWbemObjectPath: windows_core::GUID = windows_core::GUID::from_u128(0x5791bc26_ce9c_11d1_97bf_0000f81e849c);
pub const SWbemObjectSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d61_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPrivilege: windows_core::GUID = windows_core::GUID::from_u128(0x26ee67bc_5804_11d2_8b4a_00600806d9b6);
pub const SWbemPrivilegeSet: windows_core::GUID = windows_core::GUID::from_u128(0x26ee67be_5804_11d2_8b4a_00600806d9b6);
pub const SWbemProperty: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5d_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPropertySet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5c_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifier: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5f_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifierSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5e_21ae_11d2_8b33_00600806d9b6);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *const windows_core::PCWSTR,
    pub m_bArraysUsed: super::super::Foundation::BOOL,
    pub m_pbArrayElUsed: *mut super::super::Foundation::BOOL,
    pub m_puArrayIndex: *mut u32,
}
impl windows_core::TypeKind for SWbemQueryQualifiedName {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SWbemRefreshableItem: windows_core::GUID = windows_core::GUID::from_u128(0x8c6854bc_de4b_11d3_b390_00105a1f473a);
pub const SWbemRefresher: windows_core::GUID = windows_core::GUID::from_u128(0xd269bf5c_d9c1_11d3_b38f_00105a1f473a);
#[repr(C)]
#[derive(Clone, Copy)]
pub union SWbemRpnConst {
    pub m_pszStrVal: windows_core::PCWSTR,
    pub m_bBoolVal: super::super::Foundation::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
impl windows_core::TypeKind for SWbemRpnConst {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: windows_core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *const windows_core::PCWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *const windows_core::PCWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
impl windows_core::TypeKind for SWbemRpnEncodedQuery {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: windows_core::PCWSTR,
    pub m_pszLeftFunc: windows_core::PCWSTR,
}
impl windows_core::TypeKind for SWbemRpnQueryToken {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
impl windows_core::TypeKind for SWbemRpnTokenList {
    type TypeKind = windows_core::CopyType;
}
impl Default for SWbemRpnTokenList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SWbemSecurity: windows_core::GUID = windows_core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
pub const SWbemServices: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
pub const SWbemServicesEx: windows_core::GUID = windows_core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
pub const SWbemSink: windows_core::GUID = windows_core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
pub const UnsecuredApartment: windows_core::GUID = windows_core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: windows_core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
impl windows_core::TypeKind for WBEM_COMPILE_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WBEM_COMPILE_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMIExtension: windows_core::GUID = windows_core::GUID::from_u128(0xf0975afe_5c7f_11d2_8b74_00104b2afb41);
pub const WbemAdministrativeLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemAuthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemBackupRestore: windows_core::GUID = windows_core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
pub const WbemClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
pub const WbemContext: windows_core::GUID = windows_core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
pub const WbemDCOMTransport: windows_core::GUID = windows_core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
pub const WbemDecoupledBasicEventProvider: windows_core::GUID = windows_core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
pub const WbemDecoupledRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
pub const WbemDefPath: windows_core::GUID = windows_core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
pub const WbemLevel1Login: windows_core::GUID = windows_core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
pub const WbemLocalAddrRes: windows_core::GUID = windows_core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
pub const WbemLocator: windows_core::GUID = windows_core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
pub const WbemObjectTextSrc: windows_core::GUID = windows_core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
pub const WbemQuery: windows_core::GUID = windows_core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
pub const WbemRefresher: windows_core::GUID = windows_core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
pub const WbemStatusCodeText: windows_core::GUID = windows_core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
pub const WbemUnauthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
pub const WbemUninitializedClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
pub type MI_CancelCallback = Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const core::ffi::c_void)>;
pub type MI_Deserializer_ClassObjectNeeded = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
pub type MI_MainFunction = Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
pub type MI_MethodDecl_Invoke = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance)>;
pub type MI_Module_Load = Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context)>;
pub type MI_Module_Unload = Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context)>;
pub type MI_OperationCallback_Class = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Indication = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Instance = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_PromptUser = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize)>;
pub type MI_OperationCallback_StreamedParameter = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize)>;
pub type MI_OperationCallback_WriteError = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize)>;
pub type MI_OperationCallback_WriteMessage = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, channel: u32, message: *const u16)>;
pub type MI_OperationCallback_WriteProgress = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32)>;
pub type MI_ProviderFT_AssociatorInstances = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_CreateInstance = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance)>;
pub type MI_ProviderFT_DeleteInstance = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance)>;
pub type MI_ProviderFT_DisableIndications = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
pub type MI_ProviderFT_EnableIndications = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
pub type MI_ProviderFT_EnumerateInstances = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_GetInstance = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet)>;
pub type MI_ProviderFT_Invoke = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance)>;
pub type MI_ProviderFT_Load = Option<unsafe extern "system" fn(self_: *mut *mut core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context)>;
pub type MI_ProviderFT_ModifyInstance = Option<unsafe extern "system" fn(self_: *mut core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet)>;
pub type MI_ProviderFT_ReferenceInstances = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
pub type MI_ProviderFT_Subscribe = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut core::ffi::c_void)>;
pub type MI_ProviderFT_Unload = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context)>;
pub type MI_ProviderFT_Unsubscribe = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const core::ffi::c_void)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
