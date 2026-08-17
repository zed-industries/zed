#[inline]
pub unsafe fn MI_Application_InitializeV1(flags: u32, applicationid: Option<*const u16>, extendederror: Option<*mut *mut MI_Instance>, application: *mut MI_Application) -> MI_Result {
    windows_core::link!("mi.dll" "C" fn MI_Application_InitializeV1(flags : u32, applicationid : *const u16, extendederror : *mut *mut MI_Instance, application : *mut MI_Application) -> MI_Result);
    unsafe { MI_Application_InitializeV1(flags, applicationid.unwrap_or(core::mem::zeroed()) as _, extendederror.unwrap_or(core::mem::zeroed()) as _, application as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CIMTYPE_ENUMERATION(pub i32);
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
windows_core::imp::define_interface!(IEnumWbemClassObject, IEnumWbemClassObject_Vtbl, 0x027947e1_d731_11ce_a357_000000000001);
windows_core::imp::interface_hierarchy!(IEnumWbemClassObject, windows_core::IUnknown);
impl IEnumWbemClassObject {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self, ltimeout: i32, apobjects: &mut [Option<IWbemClassObject>], pureturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ltimeout, apobjects.len().try_into().unwrap(), core::mem::transmute(apobjects.as_ptr()), pureturned as _) }
    }
    pub unsafe fn NextAsync<P1>(&self, ucount: u32, psink: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).NextAsync)(windows_core::Interface::as_raw(self), ucount, psink.param().abi()) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ltimeout, ncount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
}
pub trait IEnumWbemClassObject_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut Option<IWbemClassObject>, pureturned: *mut u32) -> windows_core::HRESULT;
    fn NextAsync(&self, ucount: u32, psink: windows_core::Ref<IWbemObjectSink>) -> windows_core::HRESULT;
    fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject>;
    fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT;
}
impl IEnumWbemClassObject_Vtbl {
    pub const fn new<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&apobjects), core::mem::transmute_copy(&pureturned))
            }
        }
        unsafe extern "system" fn NextAsync<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucount: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::NextAsync(this, core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&psink))
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWbemClassObject_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWbemClassObject_Impl::Skip(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ncount))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            NextAsync: NextAsync::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWbemClassObject {}
windows_core::imp::define_interface!(IMofCompiler, IMofCompiler_Vtbl, 0x6daf974e_2e37_11d2_aec9_00c04fb68820);
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
        unsafe { (windows_core::Interface::vtable(self).CompileFile)(windows_core::Interface::as_raw(self), filename.param().abi(), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _).ok() }
    }
    pub unsafe fn CompileBuffer<P2, P3, P4, P5>(&self, pbuffer: &[u8], serverandnamespace: P2, user: P3, authority: P4, password: P5, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompileBuffer)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), serverandnamespace.param().abi(), user.param().abi(), authority.param().abi(), password.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _).ok() }
    }
    pub unsafe fn CreateBMOF<P0, P1, P2>(&self, textfilename: P0, bmoffilename: P1, serverandnamespace: P2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateBMOF)(windows_core::Interface::as_raw(self), textfilename.param().abi(), bmoffilename.param().abi(), serverandnamespace.param().abi(), loptionflags, lclassflags, linstanceflags, pinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMofCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompileFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, i32, i32, i32, *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT,
}
pub trait IMofCompiler_Impl: windows_core::IUnknownImpl {
    fn CompileFile(&self, filename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CompileBuffer(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CreateBMOF(&self, textfilename: &windows_core::PCWSTR, bmoffilename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
}
impl IMofCompiler_Vtbl {
    pub const fn new<Identity: IMofCompiler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompileFile<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CompileFile(this, core::mem::transmute(&filename), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn CompileBuffer<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CompileBuffer(this, core::mem::transmute_copy(&buffsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn CreateBMOF<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textfilename: windows_core::PCWSTR, bmoffilename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMofCompiler_Impl::CreateBMOF(this, core::mem::transmute(&textfilename), core::mem::transmute(&bmoffilename), core::mem::transmute(&serverandnamespace), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompileFile: CompileFile::<Identity, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMofCompiler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMofCompiler {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValue(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strvalue)).ok() }
    }
    pub unsafe fn Year(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Year)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetYear(&self, iyear: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetYear)(windows_core::Interface::as_raw(self), iyear).ok() }
    }
    pub unsafe fn YearSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).YearSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetYearSpecified(&self, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetYearSpecified)(windows_core::Interface::as_raw(self), byearspecified).ok() }
    }
    pub unsafe fn Month(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Month)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonth(&self, imonth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonth)(windows_core::Interface::as_raw(self), imonth).ok() }
    }
    pub unsafe fn MonthSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MonthSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonthSpecified(&self, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonthSpecified)(windows_core::Interface::as_raw(self), bmonthspecified).ok() }
    }
    pub unsafe fn Day(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Day)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDay(&self, iday: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDay)(windows_core::Interface::as_raw(self), iday).ok() }
    }
    pub unsafe fn DaySpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DaySpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDaySpecified(&self, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaySpecified)(windows_core::Interface::as_raw(self), bdayspecified).ok() }
    }
    pub unsafe fn Hours(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hours)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHours(&self, ihours: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHours)(windows_core::Interface::as_raw(self), ihours).ok() }
    }
    pub unsafe fn HoursSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoursSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHoursSpecified(&self, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHoursSpecified)(windows_core::Interface::as_raw(self), bhoursspecified).ok() }
    }
    pub unsafe fn Minutes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Minutes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinutes(&self, iminutes: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinutes)(windows_core::Interface::as_raw(self), iminutes).ok() }
    }
    pub unsafe fn MinutesSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinutesSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinutesSpecified(&self, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinutesSpecified)(windows_core::Interface::as_raw(self), bminutesspecified).ok() }
    }
    pub unsafe fn Seconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSeconds(&self, iseconds: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSeconds)(windows_core::Interface::as_raw(self), iseconds).ok() }
    }
    pub unsafe fn SecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSecondsSpecified(&self, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecondsSpecified)(windows_core::Interface::as_raw(self), bsecondsspecified).ok() }
    }
    pub unsafe fn Microseconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Microseconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMicroseconds(&self, imicroseconds: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMicroseconds)(windows_core::Interface::as_raw(self), imicroseconds).ok() }
    }
    pub unsafe fn MicrosecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MicrosecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMicrosecondsSpecified)(windows_core::Interface::as_raw(self), bmicrosecondsspecified).ok() }
    }
    pub unsafe fn UTC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UTC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUTC(&self, iutc: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUTC)(windows_core::Interface::as_raw(self), iutc).ok() }
    }
    pub unsafe fn UTCSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UTCSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUTCSpecified(&self, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUTCSpecified)(windows_core::Interface::as_raw(self), butcspecified).ok() }
    }
    pub unsafe fn IsInterval(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsInterval(&self, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsInterval)(windows_core::Interface::as_raw(self), bisinterval).ok() }
    }
    pub unsafe fn GetVarDate(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarDate)(windows_core::Interface::as_raw(self), bislocal, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVarDate(&self, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVarDate)(windows_core::Interface::as_raw(self), dvardate, bislocal).ok() }
    }
    pub unsafe fn GetFileTime(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileTime)(windows_core::Interface::as_raw(self), bislocal, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileTime(&self, strfiletime: &windows_core::BSTR, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfiletime), bislocal).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemDateTime_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub GetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemDateTime_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Year(&self) -> windows_core::Result<i32>;
    fn SetYear(&self, iyear: i32) -> windows_core::Result<()>;
    fn YearSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetYearSpecified(&self, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Month(&self) -> windows_core::Result<i32>;
    fn SetMonth(&self, imonth: i32) -> windows_core::Result<()>;
    fn MonthSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonthSpecified(&self, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Day(&self) -> windows_core::Result<i32>;
    fn SetDay(&self, iday: i32) -> windows_core::Result<()>;
    fn DaySpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaySpecified(&self, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Hours(&self) -> windows_core::Result<i32>;
    fn SetHours(&self, ihours: i32) -> windows_core::Result<()>;
    fn HoursSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHoursSpecified(&self, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Minutes(&self) -> windows_core::Result<i32>;
    fn SetMinutes(&self, iminutes: i32) -> windows_core::Result<()>;
    fn MinutesSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMinutesSpecified(&self, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Seconds(&self) -> windows_core::Result<i32>;
    fn SetSeconds(&self, iseconds: i32) -> windows_core::Result<()>;
    fn SecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSecondsSpecified(&self, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Microseconds(&self) -> windows_core::Result<i32>;
    fn SetMicroseconds(&self, imicroseconds: i32) -> windows_core::Result<()>;
    fn MicrosecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UTC(&self) -> windows_core::Result<i32>;
    fn SetUTC(&self, iutc: i32) -> windows_core::Result<()>;
    fn UTCSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUTCSpecified(&self, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsInterval(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsInterval(&self, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetVarDate(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<f64>;
    fn SetVarDate(&self, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetFileTime(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileTime(&self, strfiletime: &windows_core::BSTR, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemDateTime_Vtbl {
    pub const fn new<Identity: ISWbemDateTime_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Value(this) {
                    Ok(ok__) => {
                        strvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetValue(this, core::mem::transmute(&strvalue)).into()
            }
        }
        unsafe extern "system" fn Year<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Year(this) {
                    Ok(ok__) => {
                        iyear.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetYear<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetYear(this, core::mem::transmute_copy(&iyear)).into()
            }
        }
        unsafe extern "system" fn YearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::YearSpecified(this) {
                    Ok(ok__) => {
                        byearspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetYearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetYearSpecified(this, core::mem::transmute_copy(&byearspecified)).into()
            }
        }
        unsafe extern "system" fn Month<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Month(this) {
                    Ok(ok__) => {
                        imonth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonth<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMonth(this, core::mem::transmute_copy(&imonth)).into()
            }
        }
        unsafe extern "system" fn MonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::MonthSpecified(this) {
                    Ok(ok__) => {
                        bmonthspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMonthSpecified(this, core::mem::transmute_copy(&bmonthspecified)).into()
            }
        }
        unsafe extern "system" fn Day<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Day(this) {
                    Ok(ok__) => {
                        iday.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDay<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetDay(this, core::mem::transmute_copy(&iday)).into()
            }
        }
        unsafe extern "system" fn DaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::DaySpecified(this) {
                    Ok(ok__) => {
                        bdayspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetDaySpecified(this, core::mem::transmute_copy(&bdayspecified)).into()
            }
        }
        unsafe extern "system" fn Hours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Hours(this) {
                    Ok(ok__) => {
                        ihours.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetHours(this, core::mem::transmute_copy(&ihours)).into()
            }
        }
        unsafe extern "system" fn HoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::HoursSpecified(this) {
                    Ok(ok__) => {
                        bhoursspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetHoursSpecified(this, core::mem::transmute_copy(&bhoursspecified)).into()
            }
        }
        unsafe extern "system" fn Minutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Minutes(this) {
                    Ok(ok__) => {
                        iminutes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMinutes(this, core::mem::transmute_copy(&iminutes)).into()
            }
        }
        unsafe extern "system" fn MinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::MinutesSpecified(this) {
                    Ok(ok__) => {
                        bminutesspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMinutesSpecified(this, core::mem::transmute_copy(&bminutesspecified)).into()
            }
        }
        unsafe extern "system" fn Seconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Seconds(this) {
                    Ok(ok__) => {
                        iseconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSeconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetSeconds(this, core::mem::transmute_copy(&iseconds)).into()
            }
        }
        unsafe extern "system" fn SecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::SecondsSpecified(this) {
                    Ok(ok__) => {
                        bsecondsspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetSecondsSpecified(this, core::mem::transmute_copy(&bsecondsspecified)).into()
            }
        }
        unsafe extern "system" fn Microseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::Microseconds(this) {
                    Ok(ok__) => {
                        imicroseconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMicroseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMicroseconds(this, core::mem::transmute_copy(&imicroseconds)).into()
            }
        }
        unsafe extern "system" fn MicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::MicrosecondsSpecified(this) {
                    Ok(ok__) => {
                        bmicrosecondsspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetMicrosecondsSpecified(this, core::mem::transmute_copy(&bmicrosecondsspecified)).into()
            }
        }
        unsafe extern "system" fn UTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::UTC(this) {
                    Ok(ok__) => {
                        iutc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetUTC(this, core::mem::transmute_copy(&iutc)).into()
            }
        }
        unsafe extern "system" fn UTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::UTCSpecified(this) {
                    Ok(ok__) => {
                        butcspecified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetUTCSpecified(this, core::mem::transmute_copy(&butcspecified)).into()
            }
        }
        unsafe extern "system" fn IsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::IsInterval(this) {
                    Ok(ok__) => {
                        bisinterval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetIsInterval(this, core::mem::transmute_copy(&bisinterval)).into()
            }
        }
        unsafe extern "system" fn GetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, dvardate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::GetVarDate(this, core::mem::transmute_copy(&bislocal)) {
                    Ok(ok__) => {
                        dvardate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetVarDate(this, core::mem::transmute_copy(&dvardate), core::mem::transmute_copy(&bislocal)).into()
            }
        }
        unsafe extern "system" fn GetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, strfiletime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::GetFileTime(this, core::mem::transmute_copy(&bislocal)) {
                    Ok(ok__) => {
                        strfiletime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfiletime: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetFileTime(this, core::mem::transmute(&strfiletime), core::mem::transmute_copy(&bislocal)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Year: Year::<Identity, OFFSET>,
            SetYear: SetYear::<Identity, OFFSET>,
            YearSpecified: YearSpecified::<Identity, OFFSET>,
            SetYearSpecified: SetYearSpecified::<Identity, OFFSET>,
            Month: Month::<Identity, OFFSET>,
            SetMonth: SetMonth::<Identity, OFFSET>,
            MonthSpecified: MonthSpecified::<Identity, OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Identity, OFFSET>,
            Day: Day::<Identity, OFFSET>,
            SetDay: SetDay::<Identity, OFFSET>,
            DaySpecified: DaySpecified::<Identity, OFFSET>,
            SetDaySpecified: SetDaySpecified::<Identity, OFFSET>,
            Hours: Hours::<Identity, OFFSET>,
            SetHours: SetHours::<Identity, OFFSET>,
            HoursSpecified: HoursSpecified::<Identity, OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Identity, OFFSET>,
            Minutes: Minutes::<Identity, OFFSET>,
            SetMinutes: SetMinutes::<Identity, OFFSET>,
            MinutesSpecified: MinutesSpecified::<Identity, OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Identity, OFFSET>,
            Seconds: Seconds::<Identity, OFFSET>,
            SetSeconds: SetSeconds::<Identity, OFFSET>,
            SecondsSpecified: SecondsSpecified::<Identity, OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Identity, OFFSET>,
            Microseconds: Microseconds::<Identity, OFFSET>,
            SetMicroseconds: SetMicroseconds::<Identity, OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Identity, OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Identity, OFFSET>,
            UTC: UTC::<Identity, OFFSET>,
            SetUTC: SetUTC::<Identity, OFFSET>,
            UTCSpecified: UTCSpecified::<Identity, OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Identity, OFFSET>,
            IsInterval: IsInterval::<Identity, OFFSET>,
            SetIsInterval: SetIsInterval::<Identity, OFFSET>,
            GetVarDate: GetVarDate::<Identity, OFFSET>,
            SetVarDate: SetVarDate::<Identity, OFFSET>,
            GetFileTime: GetFileTime::<Identity, OFFSET>,
            SetFileTime: SetFileTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemDateTime as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemDateTime {}
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
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextEvent)(windows_core::Interface::as_raw(self), itimeoutms, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemEventSource_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub NextEvent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemEventSource_Impl: super::Com::IDispatch_Impl {
    fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemEventSource_Vtbl {
    pub const fn new<Identity: ISWbemEventSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NextEvent<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemEventSource_Impl::NextEvent(this, core::mem::transmute_copy(&itimeoutms)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemEventSource_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NextEvent: NextEvent::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemEventSource as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemEventSource {}
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
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLastError_Vtbl {
    pub base__: ISWbemObject_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLastError_Impl: ISWbemObject_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemLastError_Vtbl {
    pub const fn new<Identity: ISWbemLastError_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLastError as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemLastError {}
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
    pub unsafe fn ConnectServer<P7>(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: P7) -> windows_core::Result<ISWbemServices>
    where
        P7: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strserver), core::mem::transmute_copy(strnamespace), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strauthority), isecurityflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLocator_Impl: super::Com::IDispatch_Impl {
    fn ConnectServer(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemServices>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemLocator_Vtbl {
    pub const fn new<Identity: ISWbemLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, strauthority: *mut core::ffi::c_void, isecurityflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strserver), core::mem::transmute(&strnamespace), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute(&strauthority), core::mem::transmute_copy(&isecurityflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemLocator_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectServer: ConnectServer::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemLocator {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Origin)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InParameters(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OutParameters(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethod_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethod_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn OutParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemMethod_Vtbl {
    pub const fn new<Identity: ISWbemMethod_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Origin<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::Origin(this) {
                    Ok(ok__) => {
                        strorigin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbeminparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::InParameters(this) {
                    Ok(ok__) => {
                        objwbeminparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::OutParameters(this) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Origin: Origin::<Identity, OFFSET>,
            InParameters: InParameters::<Identity, OFFSET>,
            OutParameters: OutParameters::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethod as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemMethod {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethodSet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemMethodSet_Vtbl {
    pub const fn new<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethodSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemMethodSet {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValue_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemNamedValue_Vtbl {
    pub const fn new<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValue_Impl::Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValue_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValue_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValue as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemNamedValue {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, varvalue: *const super::Variant::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute(varvalue), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValueSet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varvalue: *const super::Variant::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemNamedValueSet_Vtbl {
    pub const fn new<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemnamedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: *const super::Variant::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varvalue), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemnamedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValueSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Clone(this) {
                    Ok(ok__) => {
                        objwbemnamedvalueset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValueSet_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemNamedValueSet {}
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
    pub unsafe fn Put_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectPath>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PutAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Delete_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()).ok() }
    }
    pub unsafe fn DeleteAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Instances_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Instances_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstancesAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstancesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Subclasses_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subclasses_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SubclassesAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubclassesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Associators_<P9>(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P9) -> windows_core::Result<ISWbemObjectSet>
    where
        P9: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Associators_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AssociatorsAsync_<P0, P10, P11>(&self, objwbemsink: P0, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P10, objwbemasynccontext: P11) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P10: windows_core::Param<super::Com::IDispatch>,
        P11: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssociatorsAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn References_<P6>(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P6) -> windows_core::Result<ISWbemObjectSet>
    where
        P6: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).References_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReferencesAsync_<P0, P7, P8>(&self, objwbemsink: P0, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P7, objwbemasynccontext: P8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P7: windows_core::Param<super::Com::IDispatch>,
        P8: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReferencesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn ExecMethod_<P1, P3>(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemObject>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecMethod_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecMethodAsync_<P0, P2, P4, P5>(&self, objwbemsink: P0, strmethodname: &windows_core::BSTR, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P2: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
        P5: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Clone_(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectText_)(windows_core::Interface::as_raw(self), iflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnDerivedClass_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnInstance_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareTo_)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Methods_(&self) -> windows_core::Result<ISWbemMethodSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Methods_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Derivation_(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Derivation_)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path_(&self) -> windows_core::Result<ISWbemObjectPath> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Put_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Instances_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subclasses_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Associators_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociatorsAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub References_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferencesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectText_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnDerivedClass_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnInstance_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareTo_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Methods_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Derivation_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Derivation_: usize,
    pub Path_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObject_Impl: super::Com::IDispatch_Impl {
    fn Put_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Instances_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Subclasses_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Associators_(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn References_(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod_(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Clone_(&self) -> windows_core::Result<ISWbemObject>;
    fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn CompareTo_(&self, objwbemobject: windows_core::Ref<super::Com::IDispatch>, iflags: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn Properties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn Methods_(&self) -> windows_core::Result<ISWbemMethodSet>;
    fn Derivation_(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Path_(&self) -> windows_core::Result<ISWbemObjectPath>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObject_Vtbl {
    pub const fn new<Identity: ISWbemObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Put_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Put_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::PutAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Delete_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::Delete_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn DeleteAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::DeleteAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Instances_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Instances_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstancesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::InstancesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Subclasses_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Subclasses_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::SubclassesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Associators_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Associators_(this, core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::AssociatorsAsync_(
                    this,
                    core::mem::transmute_copy(&objwbemsink),
                    core::mem::transmute(&strassocclass),
                    core::mem::transmute(&strresultclass),
                    core::mem::transmute(&strresultrole),
                    core::mem::transmute(&strrole),
                    core::mem::transmute_copy(&bclassesonly),
                    core::mem::transmute_copy(&bschemaonly),
                    core::mem::transmute(&strrequiredassocqualifier),
                    core::mem::transmute(&strrequiredqualifier),
                    core::mem::transmute_copy(&iflags),
                    core::mem::transmute_copy(&objwbemnamedvalueset),
                    core::mem::transmute_copy(&objwbemasynccontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn References_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::References_(this, core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::ReferencesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecMethod_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::ExecMethod_(this, core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::ExecMethodAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Clone_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Clone_(this) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectText_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, strobjecttext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::GetObjectText_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        strobjecttext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::SpawnDerivedClass_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnInstance_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::SpawnInstance_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareTo_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, bresult: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::CompareTo_(this, core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        bresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Properties_(this) {
                    Ok(ok__) => {
                        objwbempropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Methods_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemmethodset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Methods_(this) {
                    Ok(ok__) => {
                        objwbemmethodset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Derivation_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclassnamearray: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Derivation_(this) {
                    Ok(ok__) => {
                        strclassnamearray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Path_(this) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Put_: Put_::<Identity, OFFSET>,
            PutAsync_: PutAsync_::<Identity, OFFSET>,
            Delete_: Delete_::<Identity, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, OFFSET>,
            Instances_: Instances_::<Identity, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, OFFSET>,
            Subclasses_: Subclasses_::<Identity, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, OFFSET>,
            Associators_: Associators_::<Identity, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, OFFSET>,
            References_: References_::<Identity, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, OFFSET>,
            Clone_: Clone_::<Identity, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, OFFSET>,
            CompareTo_: CompareTo_::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
            Properties_: Properties_::<Identity, OFFSET>,
            Methods_: Methods_::<Identity, OFFSET>,
            Derivation_: Derivation_::<Identity, OFFSET>,
            Path_: Path_::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObject {}
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
    pub unsafe fn Refresh_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Refresh_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()).ok() }
    }
    pub unsafe fn SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemProperties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText_<P2>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<windows_core::BSTR>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText_)(windows_core::Interface::as_raw(self), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFromText_<P3>(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFromText_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstext), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectEx_Vtbl {
    pub base__: ISWbemObject_Vtbl,
    pub Refresh_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemProperties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText_: unsafe extern "system" fn(*mut core::ffi::c_void, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFromText_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectEx_Impl: ISWbemObject_Impl {
    fn Refresh_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn GetText_(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<windows_core::BSTR>;
    fn SetFromText_(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectEx_Vtbl {
    pub const fn new<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Refresh_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectEx_Impl::Refresh_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn SystemProperties_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectEx_Impl::SystemProperties_(this) {
                    Ok(ok__) => {
                        objwbempropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, bstext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectEx_Impl::GetText_(this, core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        bstext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFromText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstext: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectEx_Impl::SetFromText_(this, core::mem::transmute(&bstext), core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        Self {
            base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>(),
            Refresh_: Refresh_::<Identity, OFFSET>,
            SystemProperties_: SystemProperties_::<Identity, OFFSET>,
            GetText_: GetText_::<Identity, OFFSET>,
            SetFromText_: SetFromText_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectEx {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPath(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpath)).ok() }
    }
    pub unsafe fn RelPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRelPath(&self, strrelpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRelPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strrelpath)).ok() }
    }
    pub unsafe fn Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServer(&self, strserver: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strserver)).ok() }
    }
    pub unsafe fn Namespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Namespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNamespace(&self, strnamespace: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNamespace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnamespace)).ok() }
    }
    pub unsafe fn ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentNamespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, strdisplayname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strdisplayname)).ok() }
    }
    pub unsafe fn Class(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClass(&self, strclass: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass)).ok() }
    }
    pub unsafe fn IsClass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAsClass(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAsClass)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsSingleton(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSingleton)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAsSingleton(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAsSingleton)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Keys(&self) -> windows_core::Result<ISWbemNamedValueSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Keys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Locale(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Locale)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocale(&self, strlocale: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strlocale)).ok() }
    }
    pub unsafe fn Authority(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authority)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAuthority(&self, strauthority: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthority)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strauthority)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectPath_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRelPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAsClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Keys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Locale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Authority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectPath_Impl: super::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RelPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRelPath(&self, strrelpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServer(&self, strserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Namespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNamespace(&self, strnamespace: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, strdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClass(&self, strclass: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsClass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsClass(&self) -> windows_core::Result<()>;
    fn IsSingleton(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsSingleton(&self) -> windows_core::Result<()>;
    fn Keys(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn Locale(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocale(&self, strlocale: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Authority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthority(&self, strauthority: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectPath_Vtbl {
    pub const fn new<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Path(this) {
                    Ok(ok__) => {
                        strpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetPath(this, core::mem::transmute(&strpath)).into()
            }
        }
        unsafe extern "system" fn RelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::RelPath(this) {
                    Ok(ok__) => {
                        strrelpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetRelPath(this, core::mem::transmute(&strrelpath)).into()
            }
        }
        unsafe extern "system" fn Server<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Server(this) {
                    Ok(ok__) => {
                        strserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServer<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetServer(this, core::mem::transmute(&strserver)).into()
            }
        }
        unsafe extern "system" fn Namespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Namespace(this) {
                    Ok(ok__) => {
                        strnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetNamespace(this, core::mem::transmute(&strnamespace)).into()
            }
        }
        unsafe extern "system" fn ParentNamespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strparentnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::ParentNamespace(this) {
                    Ok(ok__) => {
                        strparentnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetDisplayName(this, core::mem::transmute(&strdisplayname)).into()
            }
        }
        unsafe extern "system" fn Class<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Class(this) {
                    Ok(ok__) => {
                        strclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetClass(this, core::mem::transmute(&strclass)).into()
            }
        }
        unsafe extern "system" fn IsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::IsClass(this) {
                    Ok(ok__) => {
                        bisclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetAsClass(this).into()
            }
        }
        unsafe extern "system" fn IsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bissingleton: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::IsSingleton(this) {
                    Ok(ok__) => {
                        bissingleton.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetAsSingleton(this).into()
            }
        }
        unsafe extern "system" fn Keys<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Keys(this) {
                    Ok(ok__) => {
                        objwbemnamedvalueset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Locale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Locale(this) {
                    Ok(ok__) => {
                        strlocale.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetLocale(this, core::mem::transmute(&strlocale)).into()
            }
        }
        unsafe extern "system" fn Authority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::Authority(this) {
                    Ok(ok__) => {
                        strauthority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetAuthority(this, core::mem::transmute(&strauthority)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            RelPath: RelPath::<Identity, OFFSET>,
            SetRelPath: SetRelPath::<Identity, OFFSET>,
            Server: Server::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            Namespace: Namespace::<Identity, OFFSET>,
            SetNamespace: SetNamespace::<Identity, OFFSET>,
            ParentNamespace: ParentNamespace::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Class: Class::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            IsClass: IsClass::<Identity, OFFSET>,
            SetAsClass: SetAsClass::<Identity, OFFSET>,
            IsSingleton: IsSingleton::<Identity, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, OFFSET>,
            Keys: Keys::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
            Locale: Locale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            Authority: Authority::<Identity, OFFSET>,
            SetAuthority: SetAuthority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectPath as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectPath {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemIndex)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectSet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectSet_Vtbl {
    pub const fn new<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::Item(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemIndex<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::ItemIndex(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
            ItemIndex: ItemIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectSet {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsEnabled(&self, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsEnabled)(windows_core::Interface::as_raw(self), bisenabled).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilege_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemPrivilegeEnum) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilege_Impl: super::Com::IDispatch_Impl {
    fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEnabled(&self, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPrivilege_Vtbl {
    pub const fn new<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        bisenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilege_Impl::SetIsEnabled(this, core::mem::transmute_copy(&bisenabled)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::Name(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Identifier<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::Identifier(this) {
                    Ok(ok__) => {
                        iprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            Identifier: Identifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilege as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPrivilege {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iprivilege, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), iprivilege, bisenabled, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iprivilege).ok() }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddAsString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strprivilege), bisenabled, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilegeSet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
    fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
    fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPrivilegeSet_Vtbl {
    pub const fn new<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::Item(this, core::mem::transmute_copy(&iprivilege)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::Add(this, core::mem::transmute_copy(&iprivilege), core::mem::transmute_copy(&bisenabled)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilegeSet_Impl::Remove(this, core::mem::transmute_copy(&iprivilege)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilegeSet_Impl::DeleteAll(this).into()
            }
        }
        unsafe extern "system" fn AddAsString<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprivilege: *mut core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::AddAsString(this, core::mem::transmute(&strprivilege), core::mem::transmute_copy(&bisenabled)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
            AddAsString: AddAsString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPrivilegeSet {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Origin)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CIMType(&self) -> windows_core::Result<WbemCimtypeEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CIMType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsArray(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CIMType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemCimtypeEnum) -> windows_core::HRESULT,
    pub Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemProperty_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CIMType(&self) -> windows_core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn IsArray(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemProperty_Vtbl {
    pub const fn new<Identity: ISWbemProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemProperty_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::IsLocal(this) {
                    Ok(ok__) => {
                        bislocal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Origin<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::Origin(this) {
                    Ok(ok__) => {
                        strorigin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CIMType<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::CIMType(this) {
                    Ok(ok__) => {
                        icimtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsArray<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisarray: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::IsArray(this) {
                    Ok(ok__) => {
                        bisarray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            Origin: Origin::<Identity, OFFSET>,
            CIMType: CIMType::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
            IsArray: IsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemProperty {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), icimtype, bisarray, iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertySet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WbemCimtypeEnum, super::super::Foundation::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPropertySet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPropertySet_Vtbl {
    pub const fn new<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&icimtype), core::mem::transmute_copy(&bisarray), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPropertySet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPropertySet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPropertySet {}
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(varvalue)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropagatesToSubclass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropagatesToSubclass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropagatesToSubclass(&self, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropagatesToSubclass)(windows_core::Interface::as_raw(self), bpropagatestosubclass).ok() }
    }
    pub unsafe fn PropagatesToInstance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropagatesToInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropagatesToInstance(&self, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropagatesToInstance)(windows_core::Interface::as_raw(self), bpropagatestoinstance).ok() }
    }
    pub unsafe fn IsOverridable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOverridable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsOverridable(&self, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsOverridable)(windows_core::Interface::as_raw(self), bisoverridable).ok() }
    }
    pub unsafe fn IsAmended(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAmended)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifier_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsAmended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifier_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PropagatesToSubclass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToSubclass(&self, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PropagatesToInstance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToInstance(&self, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsOverridable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsOverridable(&self, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsAmended(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemQualifier_Vtbl {
    pub const fn new<Identity: ISWbemQualifier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::IsLocal(this) {
                    Ok(ok__) => {
                        bislocal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::PropagatesToSubclass(this) {
                    Ok(ok__) => {
                        bpropagatestosubclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::SetPropagatesToSubclass(this, core::mem::transmute_copy(&bpropagatestosubclass)).into()
            }
        }
        unsafe extern "system" fn PropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::PropagatesToInstance(this) {
                    Ok(ok__) => {
                        bpropagatestoinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::SetPropagatesToInstance(this, core::mem::transmute_copy(&bpropagatestoinstance)).into()
            }
        }
        unsafe extern "system" fn IsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::IsOverridable(this) {
                    Ok(ok__) => {
                        bisoverridable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::SetIsOverridable(this, core::mem::transmute_copy(&bisoverridable)).into()
            }
        }
        unsafe extern "system" fn IsAmended<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisamended: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::IsAmended(this) {
                    Ok(ok__) => {
                        bisamended.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Identity, OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Identity, OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Identity, OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Identity, OFFSET>,
            IsOverridable: IsOverridable::<Identity, OFFSET>,
            SetIsOverridable: SetIsOverridable::<Identity, OFFSET>,
            IsAmended: IsAmended::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifier as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemQualifier {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute(varval), bpropagatestosubclass, bpropagatestoinstance, bisoverridable, iflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifierSet_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemQualifierSet_Vtbl {
    pub const fn new<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::Item(this, core::mem::transmute(&name), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemqualifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varval), core::mem::transmute_copy(&bpropagatestosubclass), core::mem::transmute_copy(&bpropagatestoinstance), core::mem::transmute_copy(&bisoverridable), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemqualifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifierSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemQualifierSet {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Refresher(&self) -> windows_core::Result<ISWbemRefresher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Refresher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Object(&self) -> windows_core::Result<ISWbemObjectEx> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Object)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iflags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefreshableItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Refresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Object: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ObjectSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefreshableItem_Impl: super::Com::IDispatch_Impl {
    fn Index(&self) -> windows_core::Result<i32>;
    fn Refresher(&self) -> windows_core::Result<ISWbemRefresher>;
    fn IsSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Object(&self) -> windows_core::Result<ISWbemObjectEx>;
    fn ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet>;
    fn Remove(&self, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemRefreshableItem_Vtbl {
    pub const fn new<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Index<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::Index(this) {
                    Ok(ok__) => {
                        iindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresher<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemrefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::Refresher(this) {
                    Ok(ok__) => {
                        objwbemrefresher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisset: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::IsSet(this) {
                    Ok(ok__) => {
                        bisset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Object<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::Object(this) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ObjectSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::ObjectSet(this) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefreshableItem_Impl::Remove(this, core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, OFFSET>,
            Refresher: Refresher::<Identity, OFFSET>,
            IsSet: IsSet::<Identity, OFFSET>,
            Object: Object::<Identity, OFFSET>,
            ObjectSet: ObjectSet::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemRefreshableItem {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0, P3>(&self, objwbemservices: P0, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), core::mem::transmute_copy(bsinstancepath), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddEnum<P0, P3>(&self, objwbemservices: P0, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), core::mem::transmute_copy(bsclassname), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iindex, iflags).ok() }
    }
    pub unsafe fn Refresh(&self, iflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), iflags).ok() }
    }
    pub unsafe fn AutoReconnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoReconnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoReconnect(&self, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoReconnect)(windows_core::Interface::as_raw(self), bcount).ok() }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefresher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefresher_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, objwbemservices: windows_core::Ref<ISWbemServicesEx>, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&self, objwbemservices: windows_core::Ref<ISWbemServicesEx>, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()>;
    fn Refresh(&self, iflags: i32) -> windows_core::Result<()>;
    fn AutoReconnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoReconnect(&self, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemRefresher_Vtbl {
    pub const fn new<Identity: ISWbemRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::Item(this, core::mem::transmute_copy(&iindex)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::Count(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsinstancepath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::Add(this, core::mem::transmute_copy(&objwbemservices), core::mem::transmute(&bsinstancepath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsclassname: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::AddEnum(this, core::mem::transmute_copy(&objwbemservices), core::mem::transmute(&bsclassname), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::Remove(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn AutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::AutoReconnect(this) {
                    Ok(ok__) => {
                        bcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::SetAutoReconnect(this, core::mem::transmute_copy(&bcount)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            AutoReconnect: AutoReconnect::<Identity, OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefresher as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemRefresher {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImpersonationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetImpersonationLevel)(windows_core::Interface::as_raw(self), iimpersonationlevel).ok() }
    }
    pub unsafe fn AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationLevel)(windows_core::Interface::as_raw(self), iauthenticationlevel).ok() }
    }
    pub unsafe fn Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Privileges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSecurity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub SetImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    pub Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSecurity_Impl: super::Com::IDispatch_Impl {
    fn ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()>;
    fn AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()>;
    fn Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSecurity_Vtbl {
    pub const fn new<Identity: ISWbemSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::ImpersonationLevel(this) {
                    Ok(ok__) => {
                        iimpersonationlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSecurity_Impl::SetImpersonationLevel(this, core::mem::transmute_copy(&iimpersonationlevel)).into()
            }
        }
        unsafe extern "system" fn AuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::AuthenticationLevel(this) {
                    Ok(ok__) => {
                        iauthenticationlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSecurity_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&iauthenticationlevel)).into()
            }
        }
        unsafe extern "system" fn Privileges<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemprivilegeset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::Privileges(this) {
                    Ok(ok__) => {
                        objwbemprivilegeset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ImpersonationLevel: ImpersonationLevel::<Identity, OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Identity, OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Identity, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, OFFSET>,
            Privileges: Privileges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSecurity as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSecurity {}
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
    pub unsafe fn Get<P2>(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObject>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAsync<P0, P3, P4>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Delete<P2>(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi()).ok() }
    }
    pub unsafe fn DeleteAsync<P0, P3, P4>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn InstancesOf<P2>(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectSet>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstancesOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstancesOfAsync<P0, P3, P4>(&self, objwbemsink: P0, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstancesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn SubclassesOf<P2>(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectSet>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubclassesOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SubclassesOfAsync<P0, P3, P4>(&self, objwbemsink: P0, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubclassesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn ExecQuery<P3>(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemObjectSet>
    where
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecQueryAsync<P0, P4, P5>(&self, objwbemsink: P0, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
        P5: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), lflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn AssociatorsOf<P10>(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P10) -> windows_core::Result<ISWbemObjectSet>
    where
        P10: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociatorsOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AssociatorsOfAsync<P0, P11, P12>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P11, objwbemasynccontext: P12) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P11: windows_core::Param<super::Com::IDispatch>,
        P12: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssociatorsOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn ReferencesTo<P7>(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P7) -> windows_core::Result<ISWbemObjectSet>
    where
        P7: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferencesTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReferencesToAsync<P0, P8, P9>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P8, objwbemasynccontext: P9) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P8: windows_core::Param<super::Com::IDispatch>,
        P9: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReferencesToAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn ExecNotificationQuery<P3>(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemEventSource>
    where
        P3: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecNotificationQueryAsync<P0, P4, P5>(&self, objwbemsink: P0, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
        P5: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn ExecMethod<P2, P4>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P4) -> windows_core::Result<ISWbemObject>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecMethodAsync<P0, P3, P5, P6>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: P3, iflags: i32, objwbemnamedvalueset: P5, objwbemasynccontext: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P5: windows_core::Param<super::Com::IDispatch>,
        P6: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
    pub unsafe fn Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServices_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociatorsOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociatorsOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferencesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferencesToAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServices_Impl: super::Com::IDispatch_Impl {
    fn Get(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn GetAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn InstancesOf(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SubclassesOf(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn AssociatorsOf(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ReferencesTo(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync(&self, objwbemsink: windows_core::Ref<super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemServices_Vtbl {
    pub const fn new<Identity: ISWbemServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::Get(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::GetAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::Delete(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn DeleteAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::DeleteAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn InstancesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::InstancesOf(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::InstancesOfAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn SubclassesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::SubclassesOf(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::SubclassesOfAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, lflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn AssociatorsOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::AssociatorsOf(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::AssociatorsOfAsync(
                    this,
                    core::mem::transmute_copy(&objwbemsink),
                    core::mem::transmute(&strobjectpath),
                    core::mem::transmute(&strassocclass),
                    core::mem::transmute(&strresultclass),
                    core::mem::transmute(&strresultrole),
                    core::mem::transmute(&strrole),
                    core::mem::transmute_copy(&bclassesonly),
                    core::mem::transmute_copy(&bschemaonly),
                    core::mem::transmute(&strrequiredassocqualifier),
                    core::mem::transmute(&strrequiredqualifier),
                    core::mem::transmute_copy(&iflags),
                    core::mem::transmute_copy(&objwbemnamedvalueset),
                    core::mem::transmute_copy(&objwbemasynccontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn ReferencesTo<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ReferencesTo(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ReferencesToAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemeventsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemeventsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecMethod<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            GetAsync: GetAsync::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, OFFSET>,
            InstancesOf: InstancesOf::<Identity, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServices as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemServices {}
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
    pub unsafe fn Put<P0, P2>(&self, objwbemobject: P0, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectPath>
    where
        P0: windows_core::Param<ISWbemObjectEx>,
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PutAsync<P0, P1, P3, P4>(&self, objwbemsink: P0, objwbemobject: P1, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISWbemSink>,
        P1: windows_core::Param<ISWbemObjectEx>,
        P3: windows_core::Param<super::Com::IDispatch>,
        P4: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesEx_Vtbl {
    pub base__: ISWbemServices_Vtbl,
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServicesEx_Impl: ISWbemServices_Impl {
    fn Put(&self, objwbemobject: windows_core::Ref<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync(&self, objwbemsink: windows_core::Ref<ISWbemSink>, objwbemobject: windows_core::Ref<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::Com::IDispatch>, objwbemasynccontext: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemServicesEx_Vtbl {
    pub const fn new<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Put<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServicesEx_Impl::Put(this, core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutAsync<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServicesEx_Impl::PutAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        Self { base__: ISWbemServices_Vtbl::new::<Identity, OFFSET>(), Put: Put::<Identity, OFFSET>, PutAsync: PutAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServicesEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemServicesEx {}
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
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSink_Impl: super::Com::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSink_Vtbl {
    pub const fn new<Identity: ISWbemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ISWbemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSink_Impl::Cancel(this).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSink as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSink {}
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
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSinkEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSinkEvents_Vtbl {
    pub const fn new<Identity: ISWbemSinkEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSinkEvents {}
windows_core::imp::define_interface!(IUnsecuredApartment, IUnsecuredApartment_Vtbl, 0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IUnsecuredApartment, windows_core::IUnknown);
impl IUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateObjectStub)(windows_core::Interface::as_raw(self), pobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsecuredApartment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObjectStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUnsecuredApartment_Impl: windows_core::IUnknownImpl {
    fn CreateObjectStub(&self, pobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl IUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateObjectStub<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnsecuredApartment_Impl::CreateObjectStub(this, core::mem::transmute_copy(&pobject)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObjectStub: CreateObjectStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUnsecuredApartment {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WMIObjectPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetWMIObject(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWMIObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWMIServices(&self) -> windows_core::Result<ISWbemServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWMIServices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMIExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub WMIObjectPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWMIObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWMIServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMIExtension_Impl: super::Com::IDispatch_Impl {
    fn WMIObjectPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWMIObject(&self) -> windows_core::Result<ISWbemObject>;
    fn GetWMIServices(&self) -> windows_core::Result<ISWbemServices>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWMIExtension_Vtbl {
    pub const fn new<Identity: IWMIExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WMIObjectPath<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strwmiobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMIExtension_Impl::WMIObjectPath(this) {
                    Ok(ok__) => {
                        strwmiobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWMIObject<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMIExtension_Impl::GetWMIObject(this) {
                    Ok(ok__) => {
                        objwmiobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWMIServices<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMIExtension_Impl::GetWMIServices(this) {
                    Ok(ok__) => {
                        objwmiservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WMIObjectPath: WMIObjectPath::<Identity, OFFSET>,
            GetWMIObject: GetWMIObject::<Identity, OFFSET>,
            GetWMIServices: GetWMIServices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIExtension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWMIExtension {}
windows_core::imp::define_interface!(IWbemAddressResolution, IWbemAddressResolution_Vtbl, 0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
windows_core::imp::interface_hierarchy!(IWbemAddressResolution, windows_core::IUnknown);
impl IWbemAddressResolution {
    pub unsafe fn Resolve<P0>(&self, wsznamespacepath: P0, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), wsznamespacepath.param().abi(), core::mem::transmute(wszaddresstype), pdwaddresslength as _, pabbinaryaddress as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemAddressResolution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IWbemAddressResolution_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, wsznamespacepath: &windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>;
}
impl IWbemAddressResolution_Vtbl {
    pub const fn new<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespacepath: windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemAddressResolution_Impl::Resolve(this, core::mem::transmute(&wsznamespacepath), core::mem::transmute_copy(&wszaddresstype), core::mem::transmute_copy(&pdwaddresslength), core::mem::transmute_copy(&pabbinaryaddress)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemAddressResolution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemAddressResolution {}
windows_core::imp::define_interface!(IWbemBackupRestore, IWbemBackupRestore_Vtbl, 0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
windows_core::imp::interface_hierarchy!(IWbemBackupRestore, windows_core::IUnknown);
impl IWbemBackupRestore {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), strbackuptofile.param().abi(), lflags).ok() }
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), strrestorefromfile.param().abi(), lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
}
pub trait IWbemBackupRestore_Impl: windows_core::IUnknownImpl {
    fn Backup(&self, strbackuptofile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn Restore(&self, strrestorefromfile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemBackupRestore_Vtbl {
    pub const fn new<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Backup<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbackuptofile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestore_Impl::Backup(this, core::mem::transmute(&strbackuptofile), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrestorefromfile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestore_Impl::Restore(this, core::mem::transmute(&strrestorefromfile), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Backup: Backup::<Identity, OFFSET>, Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemBackupRestore {}
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
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreEx_Vtbl {
    pub base__: IWbemBackupRestore_Vtbl,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemBackupRestoreEx_Impl: IWbemBackupRestore_Impl {
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl IWbemBackupRestoreEx_Vtbl {
    pub const fn new<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pause<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestoreEx_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemBackupRestoreEx_Impl::Resume(this).into()
            }
        }
        Self { base__: IWbemBackupRestore_Vtbl::new::<Identity, OFFSET>(), Pause: Pause::<Identity, OFFSET>, Resume: Resume::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as windows_core::Interface>::IID || iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemBackupRestoreEx {}
windows_core::imp::define_interface!(IWbemCallResult, IWbemCallResult_Vtbl, 0x44aca675_e8fc_11d0_a07c_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemCallResult, windows_core::IUnknown);
impl IWbemCallResult {
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultObject)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultString)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResultServices)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallStatus)(windows_core::Interface::as_raw(self), ltimeout, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemCallResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResultObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultServices: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IWbemCallResult_Impl: windows_core::IUnknownImpl {
    fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject>;
    fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices>;
    fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32>;
}
impl IWbemCallResult_Vtbl {
    pub const fn new<Identity: IWbemCallResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResultObject<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultObject(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        ppresultobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultString<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultString(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        pstrresultstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResultServices<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetResultServices(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        ppservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCallStatus<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemCallResult_Impl::GetCallStatus(this, core::mem::transmute_copy(&ltimeout)) {
                    Ok(ok__) => {
                        plstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResultObject: GetResultObject::<Identity, OFFSET>,
            GetResultString: GetResultString::<Identity, OFFSET>,
            GetResultServices: GetResultServices::<Identity, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemCallResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemCallResult {}
windows_core::imp::define_interface!(IWbemClassObject, IWbemClassObject_Vtbl, 0xdc12a681_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemClassObject, windows_core::IUnknown);
impl IWbemClassObject {
    pub unsafe fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQualifierSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: Option<*mut i32>, plflavor: Option<*mut i32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), ptype.unwrap_or(core::mem::zeroed()) as _, plflavor.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), r#type).ok() }
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT) -> windows_core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), wszqualifiername.param().abi(), lflags, core::mem::transmute(pqualifierval), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lenumflags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(strname), core::mem::transmute(pval), ptype as _, plflavor as _).ok() }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyQualifierSet)(windows_core::Interface::as_raw(self), wszproperty.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectText)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnDerivedClass)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnInstance)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CompareTo<P1>(&self, lflags: WBEM_COMPARISON_FLAG, pcompareto: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CompareTo)(windows_core::Interface::as_raw(self), lflags, pcompareto.param().abi()).ok() }
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyOrigin)(windows_core::Interface::as_raw(self), wszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InheritsFrom)(windows_core::Interface::as_raw(self), strancestor.param().abi()).ok() }
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)).ok() }
    }
    pub unsafe fn PutMethod<P0, P2, P3>(&self, wszname: P0, lflags: i32, pinsignature: P2, poutsignature: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemClassObject>,
        P3: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutMethod)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, pinsignature.param().abi(), poutsignature.param().abi()).ok() }
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteMethod)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok() }
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginMethodEnumeration)(windows_core::Interface::as_raw(self), lenumflags).ok() }
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NextMethod)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(ppinsignature), core::mem::transmute(ppoutsignature)).ok() }
    }
    pub unsafe fn EndMethodEnumeration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndMethodEnumeration)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> windows_core::Result<IWbemQualifierSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodQualifierSet)(windows_core::Interface::as_raw(self), wszmethod.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::Variant::VARIANT, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Get: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const super::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, WBEM_CONDITION_FLAG_TYPE, *const super::Variant::VARIANT, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::Variant::VARIANT, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnDerivedClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(*mut core::ffi::c_void, WBEM_COMPARISON_FLAG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InheritsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NextMethod: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndMethodEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemClassObject_Impl: windows_core::IUnknownImpl {
    fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet>;
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, wszqualifiername: &windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn GetPropertyQualifierSet(&self, wszproperty: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn Clone(&self) -> windows_core::Result<IWbemClassObject>;
    fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn CompareTo(&self, lflags: WBEM_COMPARISON_FLAG, pcompareto: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
    fn GetPropertyOrigin(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn InheritsFrom(&self, strancestor: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, ppinsignature: windows_core::OutRef<IWbemClassObject>, ppoutsignature: windows_core::OutRef<IWbemClassObject>) -> windows_core::Result<()>;
    fn PutMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, pinsignature: windows_core::Ref<IWbemClassObject>, poutsignature: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
    fn DeleteMethod(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: windows_core::OutRef<IWbemClassObject>, ppoutsignature: windows_core::OutRef<IWbemClassObject>) -> windows_core::Result<()>;
    fn EndMethodEnumeration(&self) -> windows_core::Result<()>;
    fn GetMethodQualifierSet(&self, wszmethod: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemClassObject_Vtbl {
    pub const fn new<Identity: IWbemClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetQualifierSet(this) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn Put<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Delete(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszqualifiername: windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetNames(this, core::mem::transmute(&wszqualifiername), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pqualifierval)) {
                    Ok(ok__) => {
                        pnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strname: *mut *mut core::ffi::c_void, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&strname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::EndEnumeration(this).into()
            }
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszproperty: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetPropertyQualifierSet(this, core::mem::transmute(&wszproperty)) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectText<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrobjecttext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetObjectText(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pstrobjecttext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::SpawnDerivedClass(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppnewclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnInstance<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::SpawnInstance(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppnewinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareTo<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_COMPARISON_FLAG, pcompareto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::CompareTo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcompareto)).into()
            }
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pstrclassname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetPropertyOrigin(this, core::mem::transmute(&wszname)) {
                    Ok(ok__) => {
                        pstrclassname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InheritsFrom<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strancestor: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::InheritsFrom(this, core::mem::transmute(&strancestor)).into()
            }
        }
        unsafe extern "system" fn GetMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::GetMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
            }
        }
        unsafe extern "system" fn PutMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pinsignature: *mut core::ffi::c_void, poutsignature: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::PutMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pinsignature), core::mem::transmute_copy(&poutsignature)).into()
            }
        }
        unsafe extern "system" fn DeleteMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::DeleteMethod(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::BeginMethodEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
            }
        }
        unsafe extern "system" fn NextMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::NextMethod(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
            }
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClassObject_Impl::EndMethodEnumeration(this).into()
            }
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethod: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetMethodQualifierSet(this, core::mem::transmute(&wszmethod)) {
                    Ok(ok__) => {
                        ppqualset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, pstrclassname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClassObject_Impl::GetMethodOrigin(this, core::mem::transmute(&wszmethodname)) {
                    Ok(ok__) => {
                        pstrclassname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetObjectText: GetObjectText::<Identity, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, OFFSET>,
            CompareTo: CompareTo::<Identity, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, OFFSET>,
            GetMethod: GetMethod::<Identity, OFFSET>,
            PutMethod: PutMethod::<Identity, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, OFFSET>,
            NextMethod: NextMethod::<Identity, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemClassObject {}
windows_core::imp::define_interface!(IWbemClientConnectionTransport, IWbemClientConnectionTransport_Vtbl, 0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
windows_core::imp::interface_hierarchy!(IWbemClientConnectionTransport, windows_core::IUnknown);
impl IWbemClientConnectionTransport {
    pub unsafe fn Open<P8, T>(&self, straddresstype: &windows_core::BSTR, abbinaryaddress: &[u8], strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: P8, pcallres: *mut Option<IWbemCallResult>) -> windows_core::Result<T>
    where
        P8: windows_core::Param<IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), core::mem::transmute_copy(strobject), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lflags, pctx.param().abi(), &T::IID, &mut result__, core::mem::transmute(pcallres)).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenAsync<P8, P10>(&self, straddresstype: &windows_core::BSTR, abbinaryaddress: &[u8], strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: P8, riid: *const windows_core::GUID, presponsehandler: P10) -> windows_core::Result<()>
    where
        P8: windows_core::Param<IWbemContext>,
        P10: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), core::mem::transmute_copy(strobject), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lflags, pctx.param().abi(), riid, presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn Cancel<P1>(&self, lflags: i32, phandler: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), lflags, phandler.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientConnectionTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemClientConnectionTransport_Impl: windows_core::IUnknownImpl {
    fn Open(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn OpenAsync(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, riid: *const windows_core::GUID, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn Cancel(&self, lflags: i32, phandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
}
impl IWbemClientConnectionTransport_Vtbl {
    pub const fn new<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::Open(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface), core::mem::transmute_copy(&pcallres)).into()
            }
        }
        unsafe extern "system" fn OpenAsync<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::OpenAsync(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::Cancel(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&phandler)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemClientConnectionTransport {}
windows_core::imp::define_interface!(IWbemClientTransport, IWbemClientTransport_Vtbl, 0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
windows_core::imp::interface_hierarchy!(IWbemClientTransport, windows_core::IUnknown);
impl IWbemClientTransport {
    pub unsafe fn ConnectServer<P9>(&self, straddresstype: &windows_core::BSTR, abbinaryaddress: &[u8], strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: P9) -> windows_core::Result<IWbemServices>
    where
        P9: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), abbinaryaddress.len().try_into().unwrap(), core::mem::transmute(abbinaryaddress.as_ptr()), core::mem::transmute_copy(strnetworkresource), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lsecurityflags, core::mem::transmute_copy(strauthority), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemClientTransport_Impl: windows_core::IUnknownImpl {
    fn ConnectServer(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl IWbemClientTransport_Vtbl {
    pub const fn new<Identity: IWbemClientTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: IWbemClientTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lsecurityflags: i32, strauthority: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClientTransport_Impl::ConnectServer(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientTransport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemClientTransport {}
windows_core::imp::define_interface!(IWbemConfigureRefresher, IWbemConfigureRefresher_Vtbl, 0x49353c92_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemConfigureRefresher, windows_core::IUnknown);
impl IWbemConfigureRefresher {
    pub unsafe fn AddObjectByPath<P0, P1, P3>(&self, pnamespace: P0, wszpath: P1, lflags: i32, pcontext: P3, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddObjectByPath)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszpath.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _).ok() }
    }
    pub unsafe fn AddObjectByTemplate<P0, P1, P3>(&self, pnamespace: P0, ptemplate: P1, lflags: i32, pcontext: P3, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemClassObject>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddObjectByTemplate)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _).ok() }
    }
    pub unsafe fn AddRefresher<P0>(&self, prefresher: P0, lflags: i32, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemRefresher>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRefresher)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lflags, plid as _).ok() }
    }
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lid, lflags).ok() }
    }
    pub unsafe fn AddEnum<P0, P1, P3>(&self, pnamespace: P0, wszclassname: P1, lflags: i32, pcontext: P3, ppenum: *mut Option<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclassname.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(ppenum), plid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConfigureRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjectByPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IWbemConfigureRefresher_Impl: windows_core::IUnknownImpl {
    fn AddObjectByPath(&self, pnamespace: windows_core::Ref<IWbemServices>, wszpath: &windows_core::PCWSTR, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, pprefreshable: windows_core::OutRef<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddObjectByTemplate(&self, pnamespace: windows_core::Ref<IWbemServices>, ptemplate: windows_core::Ref<IWbemClassObject>, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, pprefreshable: windows_core::OutRef<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddRefresher(&self, prefresher: windows_core::Ref<IWbemRefresher>, lflags: i32, plid: *mut i32) -> windows_core::Result<()>;
    fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn AddEnum(&self, pnamespace: windows_core::Ref<IWbemServices>, wszclassname: &windows_core::PCWSTR, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, ppenum: windows_core::OutRef<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>;
}
impl IWbemConfigureRefresher_Vtbl {
    pub const fn new<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddObjectByPath<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddObjectByPath(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddObjectByTemplate(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&ptemplate), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn AddRefresher<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddRefresher(this, core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::Remove(this, core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn AddEnum<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclassname: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConfigureRefresher_Impl::AddEnum(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszclassname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&plid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Identity, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, OFFSET>,
            AddRefresher: AddRefresher::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemConfigureRefresher {}
windows_core::imp::define_interface!(IWbemConnectorLogin, IWbemConnectorLogin_Vtbl, 0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
windows_core::imp::interface_hierarchy!(IWbemConnectorLogin, windows_core::IUnknown);
impl IWbemConnectorLogin {
    pub unsafe fn ConnectorLogin<P0, P1, P3, T>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P3) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ConnectorLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConnectorLogin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectorLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemConnectorLogin_Impl: windows_core::IUnknownImpl {
    fn ConnectorLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWbemConnectorLogin_Vtbl {
    pub const fn new<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectorLogin<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConnectorLogin_Impl::ConnectorLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectorLogin: ConnectorLogin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemConnectorLogin {}
windows_core::imp::define_interface!(IWbemConstructClassObject, IWbemConstructClassObject_Vtbl, 0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemConstructClassObject, windows_core::IUnknown);
impl IWbemConstructClassObject {
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInheritanceChain)(windows_core::Interface::as_raw(self), lnumantecedents, awszantecedents).ok() }
    }
    pub unsafe fn SetPropertyOrigin<P0>(&self, wszpropertyname: P0, loriginindex: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyOrigin)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), loriginindex).ok() }
    }
    pub unsafe fn SetMethodOrigin<P0>(&self, wszmethodname: P0, loriginindex: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), loriginindex).ok() }
    }
    pub unsafe fn SetServerNamespace<P0, P1>(&self, wszserver: P0, wsznamespace: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerNamespace)(windows_core::Interface::as_raw(self), wszserver.param().abi(), wsznamespace.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConstructClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInheritanceChain: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWbemConstructClassObject_Impl: windows_core::IUnknownImpl {
    fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPropertyOrigin(&self, wszpropertyname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetServerNamespace(&self, wszserver: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWbemConstructClassObject_Vtbl {
    pub const fn new<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInheritanceChain<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetInheritanceChain(this, core::mem::transmute_copy(&lnumantecedents), core::mem::transmute_copy(&awszantecedents)).into()
            }
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetPropertyOrigin(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&loriginindex)).into()
            }
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetMethodOrigin(this, core::mem::transmute(&wszmethodname), core::mem::transmute_copy(&loriginindex)).into()
            }
        }
        unsafe extern "system" fn SetServerNamespace<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszserver: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetServerNamespace(this, core::mem::transmute(&wszserver), core::mem::transmute(&wsznamespace)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Identity, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemConstructClassObject {}
windows_core::imp::define_interface!(IWbemContext, IWbemContext_Vtbl, 0x44aca674_e8fc_11d0_a07c_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemContext, windows_core::IUnknown);
impl IWbemContext {
    pub unsafe fn Clone(&self) -> windows_core::Result<IWbemContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue<P0>(&self, wszname: P0, lflags: i32, pvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeleteValue<P0>(&self, wszname: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteValue)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags).ok() }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
    pub DeleteValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemContext_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IWbemContext>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn SetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32, pvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn DeleteValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemContext_Vtbl {
    pub const fn new<Identity: IWbemContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppnewcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, pvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::EndEnumeration(this).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::SetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemContext_Impl::GetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::DeleteValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemContext_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            DeleteValue: DeleteValue::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemContext {}
windows_core::imp::define_interface!(IWbemDecoupledBasicEventProvider, IWbemDecoupledBasicEventProvider_Vtbl, 0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
impl core::ops::Deref for IWbemDecoupledBasicEventProvider {
    type Target = IWbemDecoupledRegistrar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemDecoupledBasicEventProvider, windows_core::IUnknown, IWbemDecoupledRegistrar);
impl IWbemDecoupledBasicEventProvider {
    pub unsafe fn GetSink<P1>(&self, a_flags: i32, a_context: P1) -> windows_core::Result<IWbemObjectSink>
    where
        P1: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetService<P1>(&self, a_flags: i32, a_context: P1) -> windows_core::Result<IWbemServices>
    where
        P1: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledBasicEventProvider_Vtbl {
    pub base__: IWbemDecoupledRegistrar_Vtbl,
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemDecoupledBasicEventProvider_Impl: IWbemDecoupledRegistrar_Impl {
    fn GetSink(&self, a_flags: i32, a_context: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemObjectSink>;
    fn GetService(&self, a_flags: i32, a_context: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl IWbemDecoupledBasicEventProvider_Vtbl {
    pub const fn new<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSink<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_sink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemDecoupledBasicEventProvider_Impl::GetSink(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context)) {
                    Ok(ok__) => {
                        a_sink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetService<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemDecoupledBasicEventProvider_Impl::GetService(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context)) {
                    Ok(ok__) => {
                        a_service.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWbemDecoupledRegistrar_Vtbl::new::<Identity, OFFSET>(),
            GetSink: GetSink::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as windows_core::Interface>::IID || iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemDecoupledBasicEventProvider {}
windows_core::imp::define_interface!(IWbemDecoupledRegistrar, IWbemDecoupledRegistrar_Vtbl, 0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
windows_core::imp::interface_hierarchy!(IWbemDecoupledRegistrar, windows_core::IUnknown);
impl IWbemDecoupledRegistrar {
    pub unsafe fn Register<P1, P2, P3, P4, P5, P6>(&self, a_flags: i32, a_context: P1, a_user: P2, a_locale: P3, a_scope: P4, a_registration: P5, piunknown: P6) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWbemContext>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), a_user.param().abi(), a_locale.param().abi(), a_scope.param().abi(), a_registration.param().abi(), piunknown.param().abi()).ok() }
    }
    pub unsafe fn UnRegister(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnRegister)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegister: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemDecoupledRegistrar_Impl: windows_core::IUnknownImpl {
    fn Register(&self, a_flags: i32, a_context: windows_core::Ref<IWbemContext>, a_user: &windows_core::PCWSTR, a_locale: &windows_core::PCWSTR, a_scope: &windows_core::PCWSTR, a_registration: &windows_core::PCWSTR, piunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnRegister(&self) -> windows_core::Result<()>;
}
impl IWbemDecoupledRegistrar_Vtbl {
    pub const fn new<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_user: windows_core::PCWSTR, a_locale: windows_core::PCWSTR, a_scope: windows_core::PCWSTR, a_registration: windows_core::PCWSTR, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemDecoupledRegistrar_Impl::Register(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context), core::mem::transmute(&a_user), core::mem::transmute(&a_locale), core::mem::transmute(&a_scope), core::mem::transmute(&a_registration), core::mem::transmute_copy(&piunknown)).into()
            }
        }
        unsafe extern "system" fn UnRegister<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemDecoupledRegistrar_Impl::UnRegister(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            UnRegister: UnRegister::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemDecoupledRegistrar {}
windows_core::imp::define_interface!(IWbemEventConsumerProvider, IWbemEventConsumerProvider_Vtbl, 0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventConsumerProvider, windows_core::IUnknown);
impl IWbemEventConsumerProvider {
    pub unsafe fn FindConsumer<P0>(&self, plogicalconsumer: P0) -> windows_core::Result<IWbemUnboundObjectSink>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventConsumerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemEventConsumerProvider_Impl: windows_core::IUnknownImpl {
    fn FindConsumer(&self, plogicalconsumer: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<IWbemUnboundObjectSink>;
}
impl IWbemEventConsumerProvider_Vtbl {
    pub const fn new<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindConsumer<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, ppconsumer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemEventConsumerProvider_Impl::FindConsumer(this, core::mem::transmute_copy(&plogicalconsumer)) {
                    Ok(ok__) => {
                        ppconsumer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindConsumer: FindConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventConsumerProvider {}
windows_core::imp::define_interface!(IWbemEventProvider, IWbemEventProvider_Vtbl, 0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventProvider, windows_core::IUnknown);
impl IWbemEventProvider {
    pub unsafe fn ProvideEvents<P0>(&self, psink: P0, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProvideEvents)(windows_core::Interface::as_raw(self), psink.param().abi(), lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProvideEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IWbemEventProvider_Impl: windows_core::IUnknownImpl {
    fn ProvideEvents(&self, psink: windows_core::Ref<IWbemObjectSink>, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemEventProvider_Vtbl {
    pub const fn new<Identity: IWbemEventProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProvideEvents<Identity: IWbemEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProvider_Impl::ProvideEvents(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideEvents: ProvideEvents::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventProvider {}
windows_core::imp::define_interface!(IWbemEventProviderQuerySink, IWbemEventProviderQuerySink_Vtbl, 0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventProviderQuerySink, windows_core::IUnknown);
impl IWbemEventProviderQuerySink {
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewQuery)(windows_core::Interface::as_raw(self), dwid, wszquerylanguage, wszquery).ok() }
    }
    pub unsafe fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelQuery)(windows_core::Interface::as_raw(self), dwid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderQuerySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *const u16) -> windows_core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWbemEventProviderQuerySink_Impl: windows_core::IUnknownImpl {
    fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::Result<()>;
    fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()>;
}
impl IWbemEventProviderQuerySink_Vtbl {
    pub const fn new<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderQuerySink_Impl::NewQuery(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery)).into()
            }
        }
        unsafe extern "system" fn CancelQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderQuerySink_Impl::CancelQuery(this, core::mem::transmute_copy(&dwid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewQuery: NewQuery::<Identity, OFFSET>,
            CancelQuery: CancelQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventProviderQuerySink {}
windows_core::imp::define_interface!(IWbemEventProviderSecurity, IWbemEventProviderSecurity_Vtbl, 0x631f7d96_d993_11d2_b339_00105a1f4aaf);
windows_core::imp::interface_hierarchy!(IWbemEventProviderSecurity, windows_core::IUnknown);
impl IWbemEventProviderSecurity {
    pub unsafe fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, psid: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AccessCheck)(windows_core::Interface::as_raw(self), wszquerylanguage, wszquery, psid.len().try_into().unwrap(), core::mem::transmute(psid.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i32, *const u8) -> windows_core::HRESULT,
}
pub trait IWbemEventProviderSecurity_Impl: windows_core::IUnknownImpl {
    fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::Result<()>;
}
impl IWbemEventProviderSecurity_Vtbl {
    pub const fn new<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessCheck<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderSecurity_Impl::AccessCheck(this, core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery), core::mem::transmute_copy(&lsidlength), core::mem::transmute_copy(&psid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessCheck: AccessCheck::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventProviderSecurity {}
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
        unsafe { (windows_core::Interface::vtable(self).SetSinkSecurity)(windows_core::Interface::as_raw(self), psd.len().try_into().unwrap(), core::mem::transmute(psd.as_ptr())).ok() }
    }
    pub unsafe fn IsActive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetRestrictedSink<P2>(&self, awszqueries: &[windows_core::PCWSTR], pcallback: P2) -> windows_core::Result<IWbemEventSink>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictedSink)(windows_core::Interface::as_raw(self), awszqueries.len().try_into().unwrap(), core::mem::transmute(awszqueries.as_ptr()), pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBatchingParameters)(windows_core::Interface::as_raw(self), lflags, dwmaxbuffersize, dwmaxsendlatency).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventSink_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub SetSinkSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, u32) -> windows_core::HRESULT,
}
pub trait IWbemEventSink_Impl: IWbemObjectSink_Impl {
    fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<()>;
    fn GetRestrictedSink(&self, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()>;
}
impl IWbemEventSink_Vtbl {
    pub const fn new<Identity: IWbemEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSinkSecurity<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsdlength: i32, psd: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::SetSinkSecurity(this, core::mem::transmute_copy(&lsdlength), core::mem::transmute_copy(&psd)).into()
            }
        }
        unsafe extern "system" fn IsActive<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::IsActive(this).into()
            }
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemEventSink_Impl::GetRestrictedSink(this, core::mem::transmute_copy(&lnumqueries), core::mem::transmute_copy(&awszqueries), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        ppsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::SetBatchingParameters(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&dwmaxsendlatency)).into()
            }
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventSink as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventSink {}
windows_core::imp::define_interface!(IWbemHiPerfEnum, IWbemHiPerfEnum_Vtbl, 0x2705c288_79ae_11d2_b348_00105a1f8177);
windows_core::imp::interface_hierarchy!(IWbemHiPerfEnum, windows_core::IUnknown);
impl IWbemHiPerfEnum {
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddObjects)(windows_core::Interface::as_raw(self), lflags, unumobjects, apids, core::mem::transmute(apobj)).ok() }
    }
    pub unsafe fn RemoveObjects(&self, lflags: i32, apids: &[i32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveObjects)(windows_core::Interface::as_raw(self), lflags, apids.len().try_into().unwrap(), core::mem::transmute(apids.as_ptr())).ok() }
    }
    pub unsafe fn GetObjects(&self, lflags: i32, apobj: &mut [Option<IWbemObjectAccess>], pureturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), lflags, apobj.len().try_into().unwrap(), core::mem::transmute(apobj.as_ptr()), pureturned as _).ok() }
    }
    pub unsafe fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const i32) -> windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IWbemHiPerfEnum_Impl: windows_core::IUnknownImpl {
    fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()>;
    fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::Result<()>;
    fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut Option<IWbemObjectAccess>, pureturned: *mut u32) -> windows_core::Result<()>;
    fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemHiPerfEnum_Vtbl {
    pub const fn new<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::AddObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids), core::mem::transmute_copy(&apobj)).into()
            }
        }
        unsafe extern "system" fn RemoveObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::RemoveObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids)).into()
            }
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::GetObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&pureturned)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfEnum_Impl::RemoveAll(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjects: AddObjects::<Identity, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemHiPerfEnum {}
windows_core::imp::define_interface!(IWbemHiPerfProvider, IWbemHiPerfProvider_Vtbl, 0x49353c93_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemHiPerfProvider, windows_core::IUnknown);
impl IWbemHiPerfProvider {
    pub unsafe fn QueryInstances<P0, P1, P3, P4>(&self, pnamespace: P0, wszclass: P1, lflags: i32, pctx: P3, psink: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInstances)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass.param().abi(), lflags, pctx.param().abi(), psink.param().abi()).ok() }
    }
    pub unsafe fn CreateRefresher<P0>(&self, pnamespace: P0, lflags: i32) -> windows_core::Result<IWbemRefresher>
    where
        P0: windows_core::Param<IWbemServices>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRefresher)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRefreshableObject<P0, P1, P2, P4>(&self, pnamespace: P0, ptemplate: P1, prefresher: P2, lflags: i32, pcontext: P4, pprefreshable: *mut Option<IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<IWbemObjectAccess>,
        P2: windows_core::Param<IWbemRefresher>,
        P4: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRefreshableObject)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _).ok() }
    }
    pub unsafe fn StopRefreshing<P0>(&self, prefresher: P0, lid: i32, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemRefresher>,
    {
        unsafe { (windows_core::Interface::vtable(self).StopRefreshing)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lid, lflags).ok() }
    }
    pub unsafe fn CreateRefreshableEnum<P0, P1, P2, P4, P5>(&self, pnamespace: P0, wszclass: P1, prefresher: P2, lflags: i32, pcontext: P4, phiperfenum: P5) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWbemRefresher>,
        P4: windows_core::Param<IWbemContext>,
        P5: windows_core::Param<IWbemHiPerfEnum>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRefreshableEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), phiperfenum.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjects<P0, P4>(&self, pnamespace: P0, apobj: &mut [Option<IWbemObjectAccess>], lflags: i32, pcontext: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemServices>,
        P4: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), apobj.len().try_into().unwrap(), core::mem::transmute(apobj.as_ptr()), lflags, pcontext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRefreshableObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StopRefreshing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CreateRefreshableEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemHiPerfProvider_Impl: windows_core::IUnknownImpl {
    fn QueryInstances(&self, pnamespace: windows_core::Ref<IWbemServices>, wszclass: &windows_core::PCWSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>, psink: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateRefresher(&self, pnamespace: windows_core::Ref<IWbemServices>, lflags: i32) -> windows_core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(&self, pnamespace: windows_core::Ref<IWbemServices>, ptemplate: windows_core::Ref<IWbemObjectAccess>, prefresher: windows_core::Ref<IWbemRefresher>, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, pprefreshable: windows_core::OutRef<IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>;
    fn StopRefreshing(&self, prefresher: windows_core::Ref<IWbemRefresher>, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn CreateRefreshableEnum(&self, pnamespace: windows_core::Ref<IWbemServices>, wszclass: &windows_core::PCWSTR, prefresher: windows_core::Ref<IWbemRefresher>, lflags: i32, pcontext: windows_core::Ref<IWbemContext>, phiperfenum: windows_core::Ref<IWbemHiPerfEnum>) -> windows_core::Result<i32>;
    fn GetObjects(&self, pnamespace: windows_core::Ref<IWbemServices>, lnumobjects: i32, apobj: *mut Option<IWbemObjectAccess>, lflags: i32, pcontext: windows_core::Ref<IWbemContext>) -> windows_core::Result<()>;
}
impl IWbemHiPerfProvider_Vtbl {
    pub const fn new<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInstances<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::QueryInstances(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn CreateRefresher<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lflags: i32, pprefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemHiPerfProvider_Impl::CreateRefresher(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pprefresher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::CreateRefreshableObject(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&ptemplate), core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn StopRefreshing<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::StopRefreshing(this, core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, phiperfenum: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemHiPerfProvider_Impl::CreateRefreshableEnum(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszclass), core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&phiperfenum)) {
                    Ok(ok__) => {
                        plid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::GetObjects(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInstances: QueryInstances::<Identity, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemHiPerfProvider {}
windows_core::imp::define_interface!(IWbemLevel1Login, IWbemLevel1Login_Vtbl, 0xf309ad18_d86a_11d0_a075_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemLevel1Login, windows_core::IUnknown);
impl IWbemLevel1Login {
    pub unsafe fn EstablishPosition<P0>(&self, wszlocalelist: P0, dwnumlocales: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EstablishPosition)(windows_core::Interface::as_raw(self), wszlocalelist.param().abi(), dwnumlocales, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestChallenge<P0, P1>(&self, wsznetworkresource: P0, wszuser: P1) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestChallenge)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszuser.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WBEMLogin<P0, P3>(&self, wszpreferredlocale: P0, accesstoken: *const u8, lflags: i32, pctx: P3) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WBEMLogin)(windows_core::Interface::as_raw(self), wszpreferredlocale.param().abi(), accesstoken, lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NTLMLogin<P0, P1, P3>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P3) -> windows_core::Result<IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NTLMLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLevel1Login_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EstablishPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    pub WBEMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NTLMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemLevel1Login_Impl: windows_core::IUnknownImpl {
    fn EstablishPosition(&self, wszlocalelist: &windows_core::PCWSTR, dwnumlocales: u32) -> windows_core::Result<u32>;
    fn RequestChallenge(&self, wsznetworkresource: &windows_core::PCWSTR, wszuser: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn WBEMLogin(&self, wszpreferredlocale: &windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
    fn NTLMLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl IWbemLevel1Login_Vtbl {
    pub const fn new<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EstablishPosition<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlocalelist: windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::EstablishPosition(this, core::mem::transmute(&wszlocalelist), core::mem::transmute_copy(&dwnumlocales)) {
                    Ok(ok__) => {
                        reserved.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestChallenge<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszuser: windows_core::PCWSTR, nonce: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::RequestChallenge(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszuser)) {
                    Ok(ok__) => {
                        nonce.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WBEMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpreferredlocale: windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::WBEMLogin(this, core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&accesstoken), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NTLMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::NTLMLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EstablishPosition: EstablishPosition::<Identity, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLevel1Login as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemLevel1Login {}
windows_core::imp::define_interface!(IWbemLocator, IWbemLocator_Vtbl, 0xdc12a687_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemLocator, windows_core::IUnknown);
impl IWbemLocator {
    pub unsafe fn ConnectServer<P6>(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: P6) -> windows_core::Result<IWbemServices>
    where
        P6: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnetworkresource), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lsecurityflags, core::mem::transmute_copy(strauthority), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemLocator_Impl: windows_core::IUnknownImpl {
    fn ConnectServer(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl IWbemLocator_Vtbl {
    pub const fn new<Identity: IWbemLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: IWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnetworkresource: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lsecurityflags: i32, strauthority: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemLocator {}
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
        unsafe { (windows_core::Interface::vtable(self).GetPropertyHandle)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), ptype as _, plhandle as _).ok() }
    }
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, adata: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyValue)(windows_core::Interface::as_raw(self), lhandle, adata.len().try_into().unwrap(), core::mem::transmute(adata.as_ptr())).ok() }
    }
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, plnumbytes: *mut i32, adata: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyValue)(windows_core::Interface::as_raw(self), lhandle, adata.len().try_into().unwrap(), plnumbytes as _, core::mem::transmute(adata.as_ptr())).ok() }
    }
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteDWORD)(windows_core::Interface::as_raw(self), lhandle, dw).ok() }
    }
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadQWORD)(windows_core::Interface::as_raw(self), lhandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteQWORD)(windows_core::Interface::as_raw(self), lhandle, pw).ok() }
    }
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfoByHandle)(windows_core::Interface::as_raw(self), lhandle, core::mem::transmute(pstrname), ptype as _).ok() }
    }
    pub unsafe fn Lock(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn Unlock(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectAccess_Vtbl {
    pub base__: IWbemClassObject_Vtbl,
    pub GetPropertyHandle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub WritePropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const u8) -> windows_core::HRESULT,
    pub ReadPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32, *mut u8) -> windows_core::HRESULT,
    pub ReadDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u64) -> windows_core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u64) -> windows_core::HRESULT,
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectAccess_Impl: IWbemClassObject_Impl {
    fn GetPropertyHandle(&self, wszpropertyname: &windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> windows_core::Result<()>;
    fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> windows_core::Result<()>;
    fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> windows_core::Result<()>;
    fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32>;
    fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::Result<()>;
    fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64>;
    fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::Result<()>;
    fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut i32) -> windows_core::Result<()>;
    fn Lock(&self, lflags: i32) -> windows_core::Result<()>;
    fn Unlock(&self, lflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemObjectAccess_Vtbl {
    pub const fn new<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::GetPropertyHandle(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plhandle)).into()
            }
        }
        unsafe extern "system" fn WritePropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WritePropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lnumbytes), core::mem::transmute_copy(&adata)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::ReadPropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lbuffersize), core::mem::transmute_copy(&plnumbytes), core::mem::transmute_copy(&adata)).into()
            }
        }
        unsafe extern "system" fn ReadDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectAccess_Impl::ReadDWORD(this, core::mem::transmute_copy(&lhandle)) {
                    Ok(ok__) => {
                        pdw.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, dw: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WriteDWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&dw)).into()
            }
        }
        unsafe extern "system" fn ReadQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectAccess_Impl::ReadQWORD(this, core::mem::transmute_copy(&lhandle)) {
                    Ok(ok__) => {
                        pqw.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pw: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::WriteQWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pw)).into()
            }
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pstrname: *mut *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::GetPropertyInfoByHandle(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::Lock(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectAccess_Impl::Unlock(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: IWbemClassObject_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Identity, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectAccess as windows_core::Interface>::IID || iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemObjectAccess {}
windows_core::imp::define_interface!(IWbemObjectSink, IWbemObjectSink_Vtbl, 0x7c857801_7381_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemObjectSink, windows_core::IUnknown);
impl IWbemObjectSink {
    pub unsafe fn Indicate(&self, apobjarray: &[Option<IWbemClassObject>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Indicate)(windows_core::Interface::as_raw(self), apobjarray.len().try_into().unwrap(), core::mem::transmute(apobjarray.as_ptr())).ok() }
    }
    pub unsafe fn SetStatus<P3>(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lflags, hresult, core::mem::transmute_copy(strparam), pobjparam.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Indicate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemObjectSink_Impl: windows_core::IUnknownImpl {
    fn Indicate(&self, lobjectcount: i32, apobjarray: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn SetStatus(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
}
impl IWbemObjectSink_Vtbl {
    pub const fn new<Identity: IWbemObjectSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Indicate<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSink_Impl::Indicate(this, core::mem::transmute_copy(&lobjectcount), core::mem::transmute_copy(&apobjarray)).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hresult: windows_core::HRESULT, strparam: *mut core::ffi::c_void, pobjparam: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSink_Impl::SetStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hresult), core::mem::transmute(&strparam), core::mem::transmute_copy(&pobjparam)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Indicate: Indicate::<Identity, OFFSET>, SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemObjectSink {}
windows_core::imp::define_interface!(IWbemObjectSinkEx, IWbemObjectSinkEx_Vtbl, 0xe7d35cfa_348b_485e_b524_252725d697ca);
impl core::ops::Deref for IWbemObjectSinkEx {
    type Target = IWbemObjectSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemObjectSinkEx, windows_core::IUnknown, IWbemObjectSink);
impl IWbemObjectSinkEx {
    pub unsafe fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteMessage)(windows_core::Interface::as_raw(self), uchannel, core::mem::transmute_copy(strmessage)).ok() }
    }
    pub unsafe fn WriteError<P0>(&self, pobjerror: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteError)(windows_core::Interface::as_raw(self), pobjerror.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PromptUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strmessage), uprompttype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteProgress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(stractivity), core::mem::transmute_copy(strcurrentoperation), core::mem::transmute_copy(strstatusdescription), upercentcomplete, usecondsremaining).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteStreamParameter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute(vtvalue), ultype, ulflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkEx_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub WriteMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PromptUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u8, *mut u8) -> windows_core::HRESULT,
    pub WriteProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub WriteStreamParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    WriteStreamParameter: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectSinkEx_Impl: IWbemObjectSink_Impl {
    fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WriteError(&self, pobjerror: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<u8>;
    fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8>;
    fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()>;
    fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemObjectSinkEx_Vtbl {
    pub const fn new<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteMessage<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uchannel: u32, strmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteMessage(this, core::mem::transmute_copy(&uchannel), core::mem::transmute(&strmessage)).into()
            }
        }
        unsafe extern "system" fn WriteError<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjerror: *mut core::ffi::c_void, pureturned: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectSinkEx_Impl::WriteError(this, core::mem::transmute_copy(&pobjerror)) {
                    Ok(ok__) => {
                        pureturned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PromptUser<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmessage: *mut core::ffi::c_void, uprompttype: u8, pureturned: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectSinkEx_Impl::PromptUser(this, core::mem::transmute(&strmessage), core::mem::transmute_copy(&uprompttype)) {
                    Ok(ok__) => {
                        pureturned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteProgress<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stractivity: *mut core::ffi::c_void, strcurrentoperation: *mut core::ffi::c_void, strstatusdescription: *mut core::ffi::c_void, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteProgress(this, core::mem::transmute(&stractivity), core::mem::transmute(&strcurrentoperation), core::mem::transmute(&strstatusdescription), core::mem::transmute_copy(&upercentcomplete), core::mem::transmute_copy(&usecondsremaining)).into()
            }
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemObjectSinkEx_Impl::WriteStreamParameter(this, core::mem::transmute(&strname), core::mem::transmute_copy(&vtvalue), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            WriteMessage: WriteMessage::<Identity, OFFSET>,
            WriteError: WriteError::<Identity, OFFSET>,
            PromptUser: PromptUser::<Identity, OFFSET>,
            WriteProgress: WriteProgress::<Identity, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemObjectSinkEx {}
windows_core::imp::define_interface!(IWbemObjectTextSrc, IWbemObjectTextSrc_Vtbl, 0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemObjectTextSrc, windows_core::IUnknown);
impl IWbemObjectTextSrc {
    pub unsafe fn GetText<P1, P3>(&self, lflags: i32, pobj: P1, uobjtextformat: u32, pctx: P3) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<IWbemClassObject>,
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pobj.param().abi(), uobjtextformat, pctx.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateFromText<P3>(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: P3) -> windows_core::Result<IWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromText)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strtext), uobjtextformat, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectTextSrc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemObjectTextSrc_Impl: windows_core::IUnknownImpl {
    fn GetText(&self, lflags: i32, pobj: windows_core::Ref<IWbemClassObject>, uobjtextformat: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<windows_core::BSTR>;
    fn CreateFromText(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IWbemClassObject>;
}
impl IWbemObjectTextSrc_Vtbl {
    pub const fn new<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pobj: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, strtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectTextSrc_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pobj), core::mem::transmute_copy(&uobjtextformat), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        strtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strtext: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, pnewobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemObjectTextSrc_Impl::CreateFromText(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strtext), core::mem::transmute_copy(&uobjtextformat), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        pnewobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            CreateFromText: CreateFromText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemObjectTextSrc {}
windows_core::imp::define_interface!(IWbemPath, IWbemPath_Vtbl, 0x3bc15af2_736c_477e_9e51_238af8667dcc);
windows_core::imp::interface_hierarchy!(IWbemPath, windows_core::IUnknown);
impl IWbemPath {
    pub unsafe fn SetText<P1>(&self, umode: u32, pszpath: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), umode, pszpath.param().abi()).ok() }
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength as _, core::mem::transmute(psztext)).ok() }
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServer<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), name.param().abi()).ok() }
    }
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetServer)(windows_core::Interface::as_raw(self), punamebuflength as _, core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn GetNamespaceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamespaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceAt<P1>(&self, uindex: u32, pszname: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, pszname.param().abi()).ok() }
    }
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, punamebuflength as _, core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveNamespaceAt)(windows_core::Interface::as_raw(self), uindex).ok() }
    }
    pub unsafe fn RemoveAllNamespaces(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllNamespaces)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetScopeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope<P1>(&self, uindex: u32, pszclass: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), uindex, pszclass.param().abi()).ok() }
    }
    pub unsafe fn SetScopeFromText<P1>(&self, uindex: u32, psztext: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScopeFromText)(windows_core::Interface::as_raw(self), uindex, psztext.param().abi()).ok() }
    }
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut Option<IWbemPathKeyList>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScope)(windows_core::Interface::as_raw(self), uindex, puclassnamebufsize as _, core::mem::transmute(pszclass), core::mem::transmute(pkeylist)).ok() }
    }
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScopeAsText)(windows_core::Interface::as_raw(self), uindex, putextbufsize as _, core::mem::transmute(psztext)).ok() }
    }
    pub unsafe fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveScope)(windows_core::Interface::as_raw(self), uindex).ok() }
    }
    pub unsafe fn RemoveAllScopes(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllScopes)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetClassName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClassName)(windows_core::Interface::as_raw(self), name.param().abi()).ok() }
    }
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: Option<windows_core::PWSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClassName)(windows_core::Interface::as_raw(self), pubufflength as _, pszname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeyList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateClassPart<P1>(&self, lflags: i32, name: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassPart)(windows_core::Interface::as_raw(self), lflags, name.param().abi()).ok() }
    }
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteClassPart)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn IsRelative<P0, P1>(&self, wszmachine: P0, wsznamespace: P1) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRelative)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi()) }
    }
    pub unsafe fn IsRelativeOrChild<P0, P1>(&self, wszmachine: P0, wsznamespace: P1, lflags: i32) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRelativeOrChild)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi(), lflags) }
    }
    pub unsafe fn IsLocal<P0>(&self, wszmachine: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), wszmachine.param().abi()) }
    }
    pub unsafe fn IsSameClassName<P0>(&self, wszclass: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsSameClassName)(windows_core::Interface::as_raw(self), wszclass.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub IsRelative: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::BOOL,
    pub IsRelativeOrChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> windows_core::BOOL,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::BOOL,
    pub IsSameClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::BOOL,
}
pub trait IWbemPath_Impl: windows_core::IUnknownImpl {
    fn SetText(&self, umode: u32, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn SetServer(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetNamespaceCount(&self) -> windows_core::Result<u32>;
    fn SetNamespaceAt(&self, uindex: u32, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllNamespaces(&self) -> windows_core::Result<()>;
    fn GetScopeCount(&self) -> windows_core::Result<u32>;
    fn SetScope(&self, uindex: u32, pszclass: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetScopeFromText(&self, uindex: u32, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: windows_core::OutRef<IWbemPathKeyList>) -> windows_core::Result<()>;
    fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllScopes(&self) -> windows_core::Result<()>;
    fn SetClassName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClassName(&self, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&self, lflags: i32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()>;
    fn IsRelative(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::BOOL;
    fn IsRelativeOrChild(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR, lflags: i32) -> windows_core::BOOL;
    fn IsLocal(&self, wszmachine: &windows_core::PCWSTR) -> windows_core::BOOL;
    fn IsSameClassName(&self, wszclass: &windows_core::PCWSTR) -> windows_core::BOOL;
}
impl IWbemPath_Vtbl {
    pub const fn new<Identity: IWbemPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umode: u32, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetText(this, core::mem::transmute_copy(&umode), core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
            }
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                    Ok(ok__) => {
                        puresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetServer(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetServer(this, core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetNamespaceCount(this) {
                    Ok(ok__) => {
                        pucount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveNamespaceAt(this, core::mem::transmute_copy(&uindex)).into()
            }
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveAllNamespaces(this).into()
            }
        }
        unsafe extern "system" fn GetScopeCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetScopeCount(this) {
                    Ok(ok__) => {
                        pucount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszclass: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszclass)).into()
            }
        }
        unsafe extern "system" fn SetScopeFromText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetScopeFromText(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn GetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&puclassnamebufsize), core::mem::transmute_copy(&pszclass), core::mem::transmute_copy(&pkeylist)).into()
            }
        }
        unsafe extern "system" fn GetScopeAsText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetScopeAsText(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&putextbufsize), core::mem::transmute_copy(&psztext)).into()
            }
        }
        unsafe extern "system" fn RemoveScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveScope(this, core::mem::transmute_copy(&uindex)).into()
            }
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveAllScopes(this).into()
            }
        }
        unsafe extern "system" fn SetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetClassName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetClassName(this, core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetKeyList<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetKeyList(this) {
                    Ok(ok__) => {
                        pout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::CreateClassPart(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn DeleteClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::DeleteClassPart(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn IsRelative<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsRelative(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace))
            }
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR, lflags: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsRelativeOrChild(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace), core::mem::transmute_copy(&lflags))
            }
        }
        unsafe extern "system" fn IsLocal<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsLocal(this, core::mem::transmute(&wszmachine))
            }
        }
        unsafe extern "system" fn IsSameClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsSameClassName(this, core::mem::transmute(&wszclass))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetText: SetText::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            GetServer: GetServer::<Identity, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, OFFSET>,
            GetScope: GetScope::<Identity, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, OFFSET>,
            RemoveScope: RemoveScope::<Identity, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, OFFSET>,
            SetClassName: SetClassName::<Identity, OFFSET>,
            GetClassName: GetClassName::<Identity, OFFSET>,
            GetKeyList: GetKeyList::<Identity, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, OFFSET>,
            IsRelative: IsRelative::<Identity, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPath as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemPath {}
windows_core::imp::define_interface!(IWbemPathKeyList, IWbemPathKeyList_Vtbl, 0x9ae62877_7544_4bb0_aa26_a13824659ed6);
windows_core::imp::interface_hierarchy!(IWbemPathKeyList, windows_core::IUnknown);
impl IWbemPathKeyList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKey<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, pkeyval).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetKey2<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKey2)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, core::mem::transmute(pkeyval)).ok() }
    }
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: Option<windows_core::PWSTR>, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetKey)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize as _, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pukeyvalbufsize as _, pkeyval as _, puapparentcimtype as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: Option<windows_core::PWSTR>, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetKey2)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize as _, pszkeyname.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pkeyvalue), puapparentcimtype as _).ok() }
    }
    pub unsafe fn RemoveKey<P0>(&self, wszname: P0, uflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags).ok() }
    }
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllKeys)(windows_core::Interface::as_raw(self), uflags).ok() }
    }
    pub unsafe fn MakeSingleton(&self, bset: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MakeSingleton)(windows_core::Interface::as_raw(self), bset).ok() }
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength as _, core::mem::transmute(psztext)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathKeyList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetKey2: usize,
    pub GetKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut super::Variant::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetKey2: usize,
    pub RemoveKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MakeSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPathKeyList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn SetKey(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetKey2(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn RemoveKey(&self, wszname: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()>;
    fn MakeSingleton(&self, bset: u8) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemPathKeyList_Vtbl {
    pub const fn new<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pukeycount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPathKeyList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pukeycount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::SetKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
            }
        }
        unsafe extern "system" fn SetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::SetKey2(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
            }
        }
        unsafe extern "system" fn GetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetKey(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pukeyvalbufsize), core::mem::transmute_copy(&pkeyval), core::mem::transmute_copy(&puapparentcimtype)).into()
            }
        }
        unsafe extern "system" fn GetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetKey2(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pkeyvalue), core::mem::transmute_copy(&puapparentcimtype)).into()
            }
        }
        unsafe extern "system" fn RemoveKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::RemoveKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::RemoveAllKeys(this, core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn MakeSingleton<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bset: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::MakeSingleton(this, core::mem::transmute_copy(&bset)).into()
            }
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPathKeyList_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                    Ok(ok__) => {
                        puresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            SetKey2: SetKey2::<Identity, OFFSET>,
            GetKey: GetKey::<Identity, OFFSET>,
            GetKey2: GetKey2::<Identity, OFFSET>,
            RemoveKey: RemoveKey::<Identity, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPathKeyList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemPathKeyList {}
windows_core::imp::define_interface!(IWbemPropertyProvider, IWbemPropertyProvider_Vtbl, 0xce61e841_65bc_11d0_b6bd_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemPropertyProvider, windows_core::IUnknown);
impl IWbemPropertyProvider {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strclassmapping), core::mem::transmute_copy(strinstmapping), core::mem::transmute_copy(strpropmapping), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strclassmapping), core::mem::transmute_copy(strinstmapping), core::mem::transmute_copy(strpropmapping), core::mem::transmute(pvvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPropertyProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPropertyProvider_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemPropertyProvider_Vtbl {
    pub const fn new<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: *mut core::ffi::c_void, strclassmapping: *mut core::ffi::c_void, strinstmapping: *mut core::ffi::c_void, strpropmapping: *mut core::ffi::c_void, pvvalue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPropertyProvider_Impl::GetProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: *mut core::ffi::c_void, strclassmapping: *mut core::ffi::c_void, strinstmapping: *mut core::ffi::c_void, strpropmapping: *mut core::ffi::c_void, pvvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPropertyProvider_Impl::PutProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping), core::mem::transmute_copy(&pvvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemPropertyProvider {}
windows_core::imp::define_interface!(IWbemProviderIdentity, IWbemProviderIdentity_Vtbl, 0x631f7d97_d993_11d2_b339_00105a1f4aaf);
windows_core::imp::interface_hierarchy!(IWbemProviderIdentity, windows_core::IUnknown);
impl IWbemProviderIdentity {
    pub unsafe fn SetRegistrationObject<P1>(&self, lflags: i32, pprovreg: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRegistrationObject)(windows_core::Interface::as_raw(self), lflags, pprovreg.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRegistrationObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemProviderIdentity_Impl: windows_core::IUnknownImpl {
    fn SetRegistrationObject(&self, lflags: i32, pprovreg: windows_core::Ref<IWbemClassObject>) -> windows_core::Result<()>;
}
impl IWbemProviderIdentity_Vtbl {
    pub const fn new<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRegistrationObject<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pprovreg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderIdentity_Impl::SetRegistrationObject(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pprovreg)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemProviderIdentity {}
windows_core::imp::define_interface!(IWbemProviderInit, IWbemProviderInit_Vtbl, 0x1be41572_91dd_11d1_aeb2_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemProviderInit, windows_core::IUnknown);
impl IWbemProviderInit {
    pub unsafe fn Initialize<P0, P2, P3, P4, P5, P6>(&self, wszuser: P0, lflags: i32, wsznamespace: P2, wszlocale: P3, pnamespace: P4, pctx: P5, pinitsink: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IWbemServices>,
        P5: windows_core::Param<IWbemContext>,
        P6: windows_core::Param<IWbemProviderInitSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), wszuser.param().abi(), lflags, wsznamespace.param().abi(), wszlocale.param().abi(), pnamespace.param().abi(), pctx.param().abi(), pinitsink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemProviderInit_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, wszuser: &windows_core::PCWSTR, lflags: i32, wsznamespace: &windows_core::PCWSTR, wszlocale: &windows_core::PCWSTR, pnamespace: windows_core::Ref<IWbemServices>, pctx: windows_core::Ref<IWbemContext>, pinitsink: windows_core::Ref<IWbemProviderInitSink>) -> windows_core::Result<()>;
}
impl IWbemProviderInit_Vtbl {
    pub const fn new<Identity: IWbemProviderInit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWbemProviderInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuser: windows_core::PCWSTR, lflags: i32, wsznamespace: windows_core::PCWSTR, wszlocale: windows_core::PCWSTR, pnamespace: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, pinitsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderInit_Impl::Initialize(this, core::mem::transmute(&wszuser), core::mem::transmute_copy(&lflags), core::mem::transmute(&wsznamespace), core::mem::transmute(&wszlocale), core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinitsink)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInit as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemProviderInit {}
windows_core::imp::define_interface!(IWbemProviderInitSink, IWbemProviderInitSink_Vtbl, 0x1be41571_91dd_11d1_aeb2_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemProviderInitSink, windows_core::IUnknown);
impl IWbemProviderInitSink {
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lstatus, lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IWbemProviderInitSink_Impl: windows_core::IUnknownImpl {
    fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemProviderInitSink_Vtbl {
    pub const fn new<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStatus<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstatus: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderInitSink_Impl::SetStatus(this, core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemProviderInitSink {}
windows_core::imp::define_interface!(IWbemQualifierSet, IWbemQualifierSet_Vtbl, 0xdc12a680_737f_11cf_884d_00aa004b2e24);
windows_core::imp::interface_hierarchy!(IWbemQualifierSet, windows_core::IUnknown);
impl IWbemQualifierSet {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), wszname.param().abi(), lflags, core::mem::transmute(pval), plflavor as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, pval: *const super::Variant::VARIANT, lflavor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), wszname.param().abi(), core::mem::transmute(pval), lflavor).ok() }
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), wszname.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), lflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEnumeration)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(pstrname), core::mem::transmute(pval), plflavor as _).ok() }
    }
    pub unsafe fn EndEnumeration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndEnumeration)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQualifierSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut super::Variant::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Get: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut super::Variant::VARIANT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemQualifierSet_Impl: windows_core::IUnknownImpl {
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, pval: *const super::Variant::VARIANT, lflavor: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemQualifierSet_Vtbl {
    pub const fn new<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn Put<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pval: *const super::Variant::VARIANT, lflavor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&lflavor)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Delete(this, core::mem::transmute(&wszname)).into()
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemQualifierSet_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut *mut core::ffi::c_void, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
            }
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQualifierSet_Impl::EndEnumeration(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQualifierSet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemQualifierSet {}
windows_core::imp::define_interface!(IWbemQuery, IWbemQuery_Vtbl, 0x81166f58_dd98_11d3_a120_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemQuery, windows_core::IUnknown);
impl IWbemQuery {
    pub unsafe fn Empty(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Empty)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize, pufeatures).ok() }
    }
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TestLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize as _, pufeatures as _).ok() }
    }
    pub unsafe fn Parse<P0, P1>(&self, pszlang: P0, pszquery: P1, uflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Parse)(windows_core::Interface::as_raw(self), pszlang.param().abi(), pszquery.param().abi(), uflags).ok() }
    }
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAnalysis)(windows_core::Interface::as_raw(self), uanalysistype, uflags, panalysis as _).ok() }
    }
    pub unsafe fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeMemory)(windows_core::Interface::as_raw(self), pmem).ok() }
    }
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetQueryInfo)(windows_core::Interface::as_raw(self), uanalysistype, uinfoid, ubufsize, pdestbuf as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IWbemQuery_Impl: windows_core::IUnknownImpl {
    fn Empty(&self) -> windows_core::Result<()>;
    fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()>;
    fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()>;
    fn Parse(&self, pszlang: &windows_core::PCWSTR, pszquery: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWbemQuery_Vtbl {
    pub const fn new<Identity: IWbemQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Empty<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::Empty(this).into()
            }
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::SetLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
            }
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::TestLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
            }
        }
        unsafe extern "system" fn Parse<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlang: windows_core::PCWSTR, pszquery: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::Parse(this, core::mem::transmute(&pszlang), core::mem::transmute(&pszquery), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn GetAnalysis<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::GetAnalysis(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&panalysis)).into()
            }
        }
        unsafe extern "system" fn FreeMemory<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::FreeMemory(this, core::mem::transmute_copy(&pmem)).into()
            }
        }
        unsafe extern "system" fn GetQueryInfo<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::GetQueryInfo(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uinfoid), core::mem::transmute_copy(&ubufsize), core::mem::transmute_copy(&pdestbuf)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Empty: Empty::<Identity, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, OFFSET>,
            Parse: Parse::<Identity, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, OFFSET>,
            FreeMemory: FreeMemory::<Identity, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQuery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemQuery {}
windows_core::imp::define_interface!(IWbemRefresher, IWbemRefresher_Vtbl, 0x49353c99_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemRefresher, windows_core::IUnknown);
impl IWbemRefresher {
    pub unsafe fn Refresh(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemRefresher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IWbemRefresher_Impl: windows_core::IUnknownImpl {
    fn Refresh(&self, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemRefresher_Vtbl {
    pub const fn new<Identity: IWbemRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Refresh<Identity: IWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Refresh: Refresh::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemRefresher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemRefresher {}
windows_core::imp::define_interface!(IWbemServices, IWbemServices_Vtbl, 0x9556dc99_828c_11cf_a37e_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemServices, windows_core::IUnknown);
impl IWbemServices {
    pub unsafe fn OpenNamespace<P2>(&self, strnamespace: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppworkingnamespace: Option<*mut Option<IWbemServices>>, ppresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenNamespace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnamespace), lflags, pctx.param().abi(), ppworkingnamespace.unwrap_or(core::mem::zeroed()) as _, ppresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CancelAsyncCall<P0>(&self, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelAsyncCall)(windows_core::Interface::as_raw(self), psink.param().abi()).ok() }
    }
    pub unsafe fn QueryObjectSink(&self, lflags: WBEM_GENERIC_FLAG_TYPE) -> windows_core::Result<IWbemObjectSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryObjectSink)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObject<P2>(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppobject: Option<*mut Option<IWbemClassObject>>, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), ppobject.unwrap_or(core::mem::zeroed()) as _, ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetObjectAsync<P2, P3>(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn PutClass<P0, P2>(&self, pobject: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutClass)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn PutClassAsync<P0, P2, P3>(&self, pobject: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutClassAsync)(windows_core::Interface::as_raw(self), pobject.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn DeleteClass<P2>(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteClass)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), lflags, pctx.param().abi(), ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DeleteClassAsync<P2, P3>(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteClassAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn CreateClassEnum<P2>(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassEnum)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateClassEnumAsync<P2, P3>(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassEnumAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn PutInstance<P0, P2>(&self, pinst: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutInstance)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn PutInstanceAsync<P0, P2, P3>(&self, pinst: P0, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutInstanceAsync)(windows_core::Interface::as_raw(self), pinst.param().abi(), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn DeleteInstance<P2>(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DeleteInstanceAsync<P2, P3>(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteInstanceAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn CreateInstanceEnum<P2>(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2) -> windows_core::Result<IEnumWbemClassObject>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceEnum)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilter), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInstanceEnumAsync<P2, P3>(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2, presponsehandler: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
        P3: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceEnumAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilter), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn ExecQuery<P3>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3) -> windows_core::Result<IEnumWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecQueryAsync<P3, P4>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3, presponsehandler: P4) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn ExecNotificationQuery<P3>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3) -> windows_core::Result<IEnumWbemClassObject>
    where
        P3: windows_core::Param<IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecNotificationQueryAsync<P3, P4>(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3, presponsehandler: P4) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquerylanguage), core::mem::transmute_copy(strquery), lflags, pctx.param().abi(), presponsehandler.param().abi()).ok() }
    }
    pub unsafe fn ExecMethod<P3, P4>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3, pinparams: P4, ppoutparams: Option<*mut Option<IWbemClassObject>>, ppcallresult: Option<*mut Option<IWbemCallResult>>) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), lflags, pctx.param().abi(), pinparams.param().abi(), ppoutparams.unwrap_or(core::mem::zeroed()) as _, ppcallresult.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ExecMethodAsync<P3, P4, P5>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P3, pinparams: P4, presponsehandler: P5) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IWbemContext>,
        P4: windows_core::Param<IWbemClassObject>,
        P5: windows_core::Param<IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), lflags, pctx.param().abi(), pinparams.param().abi(), presponsehandler.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(*mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteClassAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WBEM_GENERIC_FLAG_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemServices_Impl: windows_core::IUnknownImpl {
    fn OpenNamespace(&self, strnamespace: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppworkingnamespace: windows_core::OutRef<IWbemServices>, ppresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, psink: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn QueryObjectSink(&self, lflags: WBEM_GENERIC_FLAG_TYPE) -> windows_core::Result<IWbemObjectSink>;
    fn GetObject(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppobject: windows_core::OutRef<IWbemClassObject>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn GetObjectAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutClass(&self, pobject: windows_core::Ref<IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutClassAsync(&self, pobject: windows_core::Ref<IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteClass(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteClassAsync(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateClassEnum(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutInstance(&self, pinst: windows_core::Ref<IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutInstanceAsync(&self, pinst: windows_core::Ref<IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteInstance(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteInstanceAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateInstanceEnum(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, pinparams: windows_core::Ref<IWbemClassObject>, ppoutparams: windows_core::OutRef<IWbemClassObject>, ppcallresult: windows_core::OutRef<IWbemCallResult>) -> windows_core::Result<()>;
    fn ExecMethodAsync(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: windows_core::Ref<IWbemContext>, pinparams: windows_core::Ref<IWbemClassObject>, presponsehandler: windows_core::Ref<IWbemObjectSink>) -> windows_core::Result<()>;
}
impl IWbemServices_Vtbl {
    pub const fn new<Identity: IWbemServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenNamespace<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppworkingnamespace: *mut *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::OpenNamespace(this, core::mem::transmute(&strnamespace), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppworkingnamespace), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn QueryObjectSink<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, ppresponsehandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::QueryObjectSink(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppresponsehandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::GetObject(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppobject), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn GetObjectAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::GetObjectAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn PutClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutClass(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn PutClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutClassAsync(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn DeleteClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteClass(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteClassAsync(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn CreateClassEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::CreateClassEnum(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CreateClassEnumAsync(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn PutInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutInstance(this, core::mem::transmute_copy(&pinst), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::PutInstanceAsync(this, core::mem::transmute_copy(&pinst), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn DeleteInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteInstance(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::DeleteInstanceAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::CreateInstanceEnum(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::CreateInstanceEnumAsync(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn ExecMethod<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, ppoutparams: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinparams), core::mem::transmute_copy(&ppoutparams), core::mem::transmute_copy(&ppcallresult)).into()
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinparams), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenNamespace: OpenNamespace::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, OFFSET>,
            PutClass: PutClass::<Identity, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, OFFSET>,
            DeleteClass: DeleteClass::<Identity, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, OFFSET>,
            PutInstance: PutInstance::<Identity, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemServices {}
windows_core::imp::define_interface!(IWbemShutdown, IWbemShutdown_Vtbl, 0xb7b31df9_d515_11d3_a11c_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemShutdown, windows_core::IUnknown);
impl IWbemShutdown {
    pub unsafe fn Shutdown<P2>(&self, ureason: i32, umaxmilliseconds: u32, pctx: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), ureason, umaxmilliseconds, pctx.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemShutdown_Impl: windows_core::IUnknownImpl {
    fn Shutdown(&self, ureason: i32, umaxmilliseconds: u32, pctx: windows_core::Ref<IWbemContext>) -> windows_core::Result<()>;
}
impl IWbemShutdown_Vtbl {
    pub const fn new<Identity: IWbemShutdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Shutdown<Identity: IWbemShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemShutdown_Impl::Shutdown(this, core::mem::transmute_copy(&ureason), core::mem::transmute_copy(&umaxmilliseconds), core::mem::transmute_copy(&pctx)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemShutdown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemShutdown {}
windows_core::imp::define_interface!(IWbemStatusCodeText, IWbemStatusCodeText_Vtbl, 0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemStatusCodeText, windows_core::IUnknown);
impl IWbemStatusCodeText {
    pub unsafe fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFacilityCodeText)(windows_core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemStatusCodeText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFacilityCodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemStatusCodeText_Impl: windows_core::IUnknownImpl {
    fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
}
impl IWbemStatusCodeText_Vtbl {
    pub const fn new<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemStatusCodeText_Impl::GetErrorCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemStatusCodeText_Impl::GetFacilityCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Identity, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemStatusCodeText {}
windows_core::imp::define_interface!(IWbemTransport, IWbemTransport_Vtbl, 0x553fe584_2156_11d0_b6ae_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemTransport, windows_core::IUnknown);
impl IWbemTransport {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemTransport_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
}
impl IWbemTransport_Vtbl {
    pub const fn new<Identity: IWbemTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWbemTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemTransport_Impl::Initialize(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemTransport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemTransport {}
windows_core::imp::define_interface!(IWbemUnboundObjectSink, IWbemUnboundObjectSink_Vtbl, 0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemUnboundObjectSink, windows_core::IUnknown);
impl IWbemUnboundObjectSink {
    pub unsafe fn IndicateToConsumer<P0>(&self, plogicalconsumer: P0, apobjects: &[Option<IWbemClassObject>]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).IndicateToConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), apobjects.len().try_into().unwrap(), core::mem::transmute(apobjects.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnboundObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IndicateToConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemUnboundObjectSink_Impl: windows_core::IUnknownImpl {
    fn IndicateToConsumer(&self, plogicalconsumer: windows_core::Ref<IWbemClassObject>, lnumobjects: i32, apobjects: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
}
impl IWbemUnboundObjectSink_Vtbl {
    pub const fn new<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IndicateToConsumer<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemUnboundObjectSink_Impl::IndicateToConsumer(this, core::mem::transmute_copy(&plogicalconsumer), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobjects)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemUnboundObjectSink {}
windows_core::imp::define_interface!(IWbemUnsecuredApartment, IWbemUnsecuredApartment_Vtbl, 0x31739d04_3471_4cf4_9a7c_57a44ae71956);
impl core::ops::Deref for IWbemUnsecuredApartment {
    type Target = IUnsecuredApartment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemUnsecuredApartment, windows_core::IUnknown, IUnsecuredApartment);
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateSinkStub<P0, P2>(&self, psink: P0, dwflags: u32, wszreserved: P2) -> windows_core::Result<IWbemObjectSink>
    where
        P0: windows_core::Param<IWbemObjectSink>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSinkStub)(windows_core::Interface::as_raw(self), psink.param().abi(), dwflags, wszreserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnsecuredApartment_Vtbl {
    pub base__: IUnsecuredApartment_Vtbl,
    pub CreateSinkStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemUnsecuredApartment_Impl: IUnsecuredApartment_Impl {
    fn CreateSinkStub(&self, psink: windows_core::Ref<IWbemObjectSink>, dwflags: u32, wszreserved: &windows_core::PCWSTR) -> windows_core::Result<IWbemObjectSink>;
}
impl IWbemUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSinkStub<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwflags: u32, wszreserved: windows_core::PCWSTR, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemUnsecuredApartment_Impl::CreateSinkStub(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&dwflags), core::mem::transmute(&wszreserved)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUnsecuredApartment_Vtbl::new::<Identity, OFFSET>(), CreateSinkStub: CreateSinkStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as windows_core::Interface>::IID || iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemUnsecuredApartment {}
pub const MI_ARRAY: MI_Type = MI_Type(16i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ApplicationFT,
}
impl Default for MI_Application {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Array {
    pub data: *mut core::ffi::c_void,
    pub size: u32,
}
impl Default for MI_Array {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
pub const MI_BOOLEAN: MI_Type = MI_Type(0i32);
pub const MI_BOOLEANA: MI_Type = MI_Type(16i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = MI_CallbackMode(2i32);
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = MI_CallbackMode(1i32);
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = MI_CallbackMode(0i32);
pub const MI_CALL_VERSION: u32 = 1u32;
pub const MI_CHAR16: MI_Type = MI_Type(11i32);
pub const MI_CHAR16A: MI_Type = MI_Type(27i32);
pub const MI_CHAR_TYPE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_CallbackMode(pub i32);
pub type MI_CancelCallback = Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_CancellationReason(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl Default for MI_Char16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Class {
    pub ft: *const MI_ClassFT,
    pub classDecl: *const MI_ClassDecl,
    pub namespaceName: *const u16,
    pub serverName: *const u16,
    pub reserved: [isize; 4],
}
impl Default for MI_Class {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstBooleanA {
    pub data: *const u8,
    pub size: u32,
}
impl Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstChar16A {
    pub data: *const u16,
    pub size: u32,
}
impl Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstDatetimeA {
    pub data: *const MI_Datetime,
    pub size: u32,
}
impl Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstInstanceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstInstanceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReal32A {
    pub data: *const f32,
    pub size: u32,
}
impl Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReal64A {
    pub data: *const f64,
    pub size: u32,
}
impl Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReferenceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstReferenceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstSint16A {
    pub data: *const i16,
    pub size: u32,
}
impl Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstSint32A {
    pub data: *const i32,
    pub size: u32,
}
impl Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstSint64A {
    pub data: *const i64,
    pub size: u32,
}
impl Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstSint8A {
    pub data: *const i8,
    pub size: u32,
}
impl Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstStringA {
    pub data: *const *const u16,
    pub size: u32,
}
impl Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstStringField {
    pub value: *const u16,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstUint16A {
    pub data: *const u16,
    pub size: u32,
}
impl Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstUint32A {
    pub data: *const u32,
    pub size: u32,
}
impl Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstUint64A {
    pub data: *const u64,
    pub size: u32,
}
impl Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ConstUint8A {
    pub data: *const u8,
    pub size: u32,
}
impl Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Context {
    pub ft: *const MI_ContextFT,
    pub reserved: [isize; 3],
}
impl Default for MI_Context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const MI_DATETIME: MI_Type = MI_Type(12i32);
pub const MI_DATETIMEA: MI_Type = MI_Type(28i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
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
impl Default for MI_Datetime_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
pub type MI_Deserializer_ClassObjectNeeded = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_DestinationOptionsFT,
}
impl Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_DestinationOptions_ImpersonationType(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_ErrorCategory(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
}
impl Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Filter {
    pub ft: *const MI_FilterFT,
    pub reserved: [isize; 3],
}
impl Default for MI_Filter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_HostedProviderFT,
}
impl Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
pub const MI_INSTANCE: MI_Type = MI_Type(15i32);
pub const MI_INSTANCEA: MI_Type = MI_Type(31i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Instance {
    pub ft: *const MI_InstanceFT,
    pub classDecl: *const MI_ClassDecl,
    pub serverName: *const u16,
    pub nameSpace: *const u16,
    pub reserved: [isize; 4],
}
impl Default for MI_Instance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = MI_LocaleType(3i32);
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = MI_LocaleType(2i32);
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = MI_LocaleType(1i32);
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = MI_LocaleType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_LocaleType(pub i32);
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
pub type MI_MainFunction = Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
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
impl Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MI_MethodDecl_Invoke = Option<unsafe extern "system" fn(self_: *const core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance)>;
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
impl Default for MI_Module {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MI_Module_Load = Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MI_Module_Self(pub isize);
pub type MI_Module_Unload = Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context)>;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationFT,
}
impl Default for MI_Operation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MI_OperationCallback_Class = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Indication = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_Instance = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
pub type MI_OperationCallback_PromptUser = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_OperationCallback_ResponseType(pub i32);
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(0i32);
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(2i32);
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(1i32);
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(3i32);
pub type MI_OperationCallback_StreamedParameter = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize)>;
pub type MI_OperationCallback_WriteError = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize)>;
pub type MI_OperationCallback_WriteMessage = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, channel: u32, message: *const u16)>;
pub type MI_OperationCallback_WriteProgress = Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32)>;
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
impl Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationOptionsFT,
}
impl Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = MI_PromptType(1i32);
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = MI_PromptType(0i32);
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(0i32);
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ParameterSetFT,
}
impl Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_PromptType(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_PropertySet {
    pub ft: *const MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_ProviderArchitecture(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Qualifier {
    pub name: *const u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *const core::ffi::c_void,
}
impl Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_QualifierDecl {
    pub name: *const u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *const core::ffi::c_void,
}
impl Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_QualifierSetFT,
}
impl Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl Default for MI_Real32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl Default for MI_Real64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_Result(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *const *const MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *const *const MI_ClassDecl,
    pub numClassDecls: u32,
}
impl Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Server {
    pub serverFT: *const MI_ServerFT,
    pub contextFT: *const MI_ContextFT,
    pub instanceFT: *const MI_InstanceFT,
    pub propertySetFT: *const MI_PropertySetFT,
    pub filterFT: *const MI_FilterFT,
}
impl Default for MI_Server {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SessionFT,
}
impl Default for MI_Session {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl Default for MI_StringA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl Default for MI_StringField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SubscriptionDeliveryOptionsFT,
}
impl Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_SubscriptionDeliveryType(pub i32);
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(1i32);
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MI_Type(pub i32);
pub const MI_UINT16: MI_Type = MI_Type(3i32);
pub const MI_UINT16A: MI_Type = MI_Type(19i32);
pub const MI_UINT32: MI_Type = MI_Type(5i32);
pub const MI_UINT32A: MI_Type = MI_Type(21i32);
pub const MI_UINT64: MI_Type = MI_Type(7i32);
pub const MI_UINT64A: MI_Type = MI_Type(23i32);
pub const MI_UINT8: MI_Type = MI_Type(1i32);
pub const MI_UINT8A: MI_Type = MI_Type(17i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MI_UserCredentials {
    pub authenticationType: *const u16,
    pub credentials: MI_UserCredentials_0,
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
impl Default for MI_UserCredentials_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MI_UsernamePasswordCreds {
    pub domain: *const u16,
    pub username: *const u16,
    pub password: *const u16,
}
impl Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
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
impl Default for MI_Value {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const MofCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: windows_core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut core::ffi::c_void,
    pub m_pbTruthTable: *mut windows_core::BOOL,
}
impl Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
impl Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *const windows_core::PCWSTR,
    pub m_bArraysUsed: windows_core::BOOL,
    pub m_pbArrayElUsed: *mut windows_core::BOOL,
    pub m_puArrayIndex: *mut u32,
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
    pub m_bBoolVal: windows_core::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
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
impl Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
pub const SWbemSecurity: windows_core::GUID = windows_core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
pub const SWbemServices: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
pub const SWbemServicesEx: windows_core::GUID = windows_core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
pub const SWbemSink: windows_core::GUID = windows_core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
pub const UnsecuredApartment: windows_core::GUID = windows_core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEMSTATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEMSTATUS_FORMAT(pub i32);
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(0i32);
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(1i32);
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
pub const WBEM_AUTHENTICATION_METHOD_MASK: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(15i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_BACKUP_RESTORE_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_BATCH_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_CHANGE_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_COMPARISON_FLAG(pub i32);
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_COMPILER_OPTIONS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: windows_core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_CONDITION_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_CONNECT_OPTIONS(pub i32);
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_EXTRA_RETURN_CODES(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_FLAVOR_TYPE(pub i32);
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_GENERIC_FLAG_TYPE(pub i32);
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
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(1i32);
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_GENUS_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_GET_KEY_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_GET_TEXT_FLAGS(pub i32);
pub const WBEM_INFINITE: i32 = -1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_INFORMATION_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_LIMITATION_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_LIMITS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_LOCKING_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_LOGIN_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_PATH_CREATE_FLAG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_PATH_STATUS_FLAG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_PROVIDER_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_QUERY_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_REFRESHER_FLAGS(pub i32);
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(32i32);
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(2i32);
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(0i32);
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(1i32);
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(128i32);
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_SECURITY_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_SHUTDOWN_FLAGS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_STATUS_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_TEXT_FLAG_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WBEM_UNSECAPP_FLAG_TYPE(pub i32);
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(16i32);
pub const WMIExtension: windows_core::GUID = windows_core::GUID::from_u128(0xf0975afe_5c7f_11d2_8b74_00104b2afb41);
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(2i32);
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(3i32);
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(4i32);
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(134217728i32);
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIQ_ANALYSIS_TYPE(pub i32);
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(8i32);
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1i32);
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(256i32);
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2048i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIQ_ASSOCQ_FLAGS(pub i32);
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(512i32);
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2i32);
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(128i32);
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(64i32);
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(4i32);
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(32i32);
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(16i32);
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1024i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIQ_LANGUAGE_FEATURES(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIQ_RPNF_FEATURE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMIQ_RPN_TOKEN_FLAGS(pub i32);
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMI_OBJ_TEXT(pub i32);
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
pub const WbemAdministrativeLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemAuthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemAuthenticationLevelEnum(pub i32);
pub const WbemBackupRestore: windows_core::GUID = windows_core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemChangeFlagEnum(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemCimtypeEnum(pub i32);
pub const WbemClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemComparisonFlagEnum(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemConnectOptionsEnum(pub i32);
pub const WbemContext: windows_core::GUID = windows_core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
pub const WbemDCOMTransport: windows_core::GUID = windows_core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
pub const WbemDecoupledBasicEventProvider: windows_core::GUID = windows_core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
pub const WbemDecoupledRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
pub const WbemDefPath: windows_core::GUID = windows_core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemErrorEnum(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemFlagEnum(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemImpersonationLevelEnum(pub i32);
pub const WbemLevel1Login: windows_core::GUID = windows_core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
pub const WbemLocalAddrRes: windows_core::GUID = windows_core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
pub const WbemLocator: windows_core::GUID = windows_core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemObjectTextFormatEnum(pub i32);
pub const WbemObjectTextSrc: windows_core::GUID = windows_core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemPrivilegeEnum(pub i32);
pub const WbemQuery: windows_core::GUID = windows_core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemQueryFlagEnum(pub i32);
pub const WbemRefresher: windows_core::GUID = windows_core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
pub const WbemStatusCodeText: windows_core::GUID = windows_core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemTextFlagEnum(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WbemTimeout(pub i32);
pub const WbemUnauthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
pub const WbemUninitializedClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
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
