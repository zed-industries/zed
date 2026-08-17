pub trait IEnumWbemClassObject_Impl: Sized {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut Option<IWbemClassObject>, pureturned: *mut u32) -> windows_core::HRESULT;
    fn NextAsync(&self, ucount: u32, psink: Option<&IWbemObjectSink>) -> windows_core::HRESULT;
    fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject>;
    fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT;
}
impl windows_core::RuntimeName for IEnumWbemClassObject {}
impl IEnumWbemClassObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>() -> IEnumWbemClassObject_Vtbl {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumWbemClassObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&apobjects), core::mem::transmute_copy(&pureturned))
        }
        unsafe extern "system" fn NextAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucount: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumWbemClassObject_Impl::NextAsync(this, core::mem::transmute_copy(&ucount), windows_core::from_raw_borrowed(&psink))
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumWbemClassObject_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumWbemClassObject_Impl::Skip(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ncount))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            NextAsync: NextAsync::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IMofCompiler_Impl: Sized {
    fn CompileFile(&self, filename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CompileBuffer(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CreateBMOF(&self, textfilename: &windows_core::PCWSTR, bmoffilename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMofCompiler {}
impl IMofCompiler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>() -> IMofCompiler_Vtbl {
        unsafe extern "system" fn CompileFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMofCompiler_Impl::CompileFile(this, core::mem::transmute(&filename), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CompileBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMofCompiler_Impl::CompileBuffer(this, core::mem::transmute_copy(&buffsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CreateBMOF<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textfilename: windows_core::PCWSTR, bmoffilename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMofCompiler_Impl::CreateBMOF(this, core::mem::transmute(&textfilename), core::mem::transmute(&bmoffilename), core::mem::transmute(&serverandnamespace), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompileFile: CompileFile::<Identity, Impl, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, Impl, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMofCompiler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemDateTime_Impl: Sized + super::Com::IDispatch_Impl {
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemDateTime {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemDateTime_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>() -> ISWbemDateTime_Vtbl {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(strvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetValue(this, core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn Year<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Year(this) {
                Ok(ok__) => {
                    core::ptr::write(iyear, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetYear(this, core::mem::transmute_copy(&iyear)).into()
        }
        unsafe extern "system" fn YearSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::YearSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(byearspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYearSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetYearSpecified(this, core::mem::transmute_copy(&byearspecified)).into()
        }
        unsafe extern "system" fn Month<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Month(this) {
                Ok(ok__) => {
                    core::ptr::write(imonth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMonth(this, core::mem::transmute_copy(&imonth)).into()
        }
        unsafe extern "system" fn MonthSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::MonthSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bmonthspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMonthSpecified(this, core::mem::transmute_copy(&bmonthspecified)).into()
        }
        unsafe extern "system" fn Day<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Day(this) {
                Ok(ok__) => {
                    core::ptr::write(iday, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetDay(this, core::mem::transmute_copy(&iday)).into()
        }
        unsafe extern "system" fn DaySpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::DaySpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bdayspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaySpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetDaySpecified(this, core::mem::transmute_copy(&bdayspecified)).into()
        }
        unsafe extern "system" fn Hours<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Hours(this) {
                Ok(ok__) => {
                    core::ptr::write(ihours, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHours<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetHours(this, core::mem::transmute_copy(&ihours)).into()
        }
        unsafe extern "system" fn HoursSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::HoursSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bhoursspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetHoursSpecified(this, core::mem::transmute_copy(&bhoursspecified)).into()
        }
        unsafe extern "system" fn Minutes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Minutes(this) {
                Ok(ok__) => {
                    core::ptr::write(iminutes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMinutes(this, core::mem::transmute_copy(&iminutes)).into()
        }
        unsafe extern "system" fn MinutesSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::MinutesSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bminutesspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMinutesSpecified(this, core::mem::transmute_copy(&bminutesspecified)).into()
        }
        unsafe extern "system" fn Seconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Seconds(this) {
                Ok(ok__) => {
                    core::ptr::write(iseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetSeconds(this, core::mem::transmute_copy(&iseconds)).into()
        }
        unsafe extern "system" fn SecondsSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::SecondsSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bsecondsspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetSecondsSpecified(this, core::mem::transmute_copy(&bsecondsspecified)).into()
        }
        unsafe extern "system" fn Microseconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::Microseconds(this) {
                Ok(ok__) => {
                    core::ptr::write(imicroseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicroseconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMicroseconds(this, core::mem::transmute_copy(&imicroseconds)).into()
        }
        unsafe extern "system" fn MicrosecondsSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::MicrosecondsSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(bmicrosecondsspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetMicrosecondsSpecified(this, core::mem::transmute_copy(&bmicrosecondsspecified)).into()
        }
        unsafe extern "system" fn UTC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::UTC(this) {
                Ok(ok__) => {
                    core::ptr::write(iutc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetUTC(this, core::mem::transmute_copy(&iutc)).into()
        }
        unsafe extern "system" fn UTCSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::UTCSpecified(this) {
                Ok(ok__) => {
                    core::ptr::write(butcspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetUTCSpecified(this, core::mem::transmute_copy(&butcspecified)).into()
        }
        unsafe extern "system" fn IsInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::IsInterval(this) {
                Ok(ok__) => {
                    core::ptr::write(bisinterval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetIsInterval(this, core::mem::transmute_copy(&bisinterval)).into()
        }
        unsafe extern "system" fn GetVarDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, dvardate: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::GetVarDate(this, core::mem::transmute_copy(&bislocal)) {
                Ok(ok__) => {
                    core::ptr::write(dvardate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetVarDate(this, core::mem::transmute_copy(&dvardate), core::mem::transmute_copy(&bislocal)).into()
        }
        unsafe extern "system" fn GetFileTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, strfiletime: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemDateTime_Impl::GetFileTime(this, core::mem::transmute_copy(&bislocal)) {
                Ok(ok__) => {
                    core::ptr::write(strfiletime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfiletime: core::mem::MaybeUninit<windows_core::BSTR>, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemDateTime_Impl::SetFileTime(this, core::mem::transmute(&strfiletime), core::mem::transmute_copy(&bislocal)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Year: Year::<Identity, Impl, OFFSET>,
            SetYear: SetYear::<Identity, Impl, OFFSET>,
            YearSpecified: YearSpecified::<Identity, Impl, OFFSET>,
            SetYearSpecified: SetYearSpecified::<Identity, Impl, OFFSET>,
            Month: Month::<Identity, Impl, OFFSET>,
            SetMonth: SetMonth::<Identity, Impl, OFFSET>,
            MonthSpecified: MonthSpecified::<Identity, Impl, OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Identity, Impl, OFFSET>,
            Day: Day::<Identity, Impl, OFFSET>,
            SetDay: SetDay::<Identity, Impl, OFFSET>,
            DaySpecified: DaySpecified::<Identity, Impl, OFFSET>,
            SetDaySpecified: SetDaySpecified::<Identity, Impl, OFFSET>,
            Hours: Hours::<Identity, Impl, OFFSET>,
            SetHours: SetHours::<Identity, Impl, OFFSET>,
            HoursSpecified: HoursSpecified::<Identity, Impl, OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Identity, Impl, OFFSET>,
            Minutes: Minutes::<Identity, Impl, OFFSET>,
            SetMinutes: SetMinutes::<Identity, Impl, OFFSET>,
            MinutesSpecified: MinutesSpecified::<Identity, Impl, OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Identity, Impl, OFFSET>,
            Seconds: Seconds::<Identity, Impl, OFFSET>,
            SetSeconds: SetSeconds::<Identity, Impl, OFFSET>,
            SecondsSpecified: SecondsSpecified::<Identity, Impl, OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Identity, Impl, OFFSET>,
            Microseconds: Microseconds::<Identity, Impl, OFFSET>,
            SetMicroseconds: SetMicroseconds::<Identity, Impl, OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Identity, Impl, OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Identity, Impl, OFFSET>,
            UTC: UTC::<Identity, Impl, OFFSET>,
            SetUTC: SetUTC::<Identity, Impl, OFFSET>,
            UTCSpecified: UTCSpecified::<Identity, Impl, OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Identity, Impl, OFFSET>,
            IsInterval: IsInterval::<Identity, Impl, OFFSET>,
            SetIsInterval: SetIsInterval::<Identity, Impl, OFFSET>,
            GetVarDate: GetVarDate::<Identity, Impl, OFFSET>,
            SetVarDate: SetVarDate::<Identity, Impl, OFFSET>,
            GetFileTime: GetFileTime::<Identity, Impl, OFFSET>,
            SetFileTime: SetFileTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemDateTime as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemEventSource_Impl: Sized + super::Com::IDispatch_Impl {
    fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemEventSource {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemEventSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>() -> ISWbemEventSource_Vtbl {
        unsafe extern "system" fn NextEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemEventSource_Impl::NextEvent(this, core::mem::transmute_copy(&itimeoutms)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemEventSource_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NextEvent: NextEvent::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemEventSource as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLastError_Impl: Sized + ISWbemObject_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemLastError {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLastError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLastError_Impl, const OFFSET: isize>() -> ISWbemLastError_Vtbl {
        Self { base__: ISWbemObject_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLastError as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectServer(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemServices>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>() -> ISWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: core::mem::MaybeUninit<windows_core::BSTR>, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strserver), core::mem::transmute(&strnamespace), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute(&strauthority), core::mem::transmute_copy(&isecurityflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemservices, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemLocator_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectServer: ConnectServer::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethod_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn OutParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemMethod {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethod_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>() -> ISWbemMethod_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethod_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(strname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethod_Impl::Origin(this) {
                Ok(ok__) => {
                    core::ptr::write(strorigin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbeminparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethod_Impl::InParameters(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbeminparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethod_Impl::OutParameters(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemoutparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethod_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemqualifierset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            InParameters: InParameters::<Identity, Impl, OFFSET>,
            OutParameters: OutParameters::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethod as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethodSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemMethodSet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethodSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>() -> ISWbemMethodSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethodSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethodSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemmethod, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemMethodSet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethodSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemNamedValue {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>() -> ISWbemNamedValue_Vtbl {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValue_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(varvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemNamedValue_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValue_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(strname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValue as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValueSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varvalue: *const windows_core::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemNamedValueSet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValueSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>() -> ISWbemNamedValueSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValueSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValueSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemnamedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValueSet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValueSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varvalue), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemnamedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemNamedValueSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemNamedValueSet_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemnamedvalueset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemNamedValueSet_Impl::DeleteAll(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn Put_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Instances_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Subclasses_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Associators_(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn References_(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod_(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Clone_(&self) -> windows_core::Result<ISWbemObject>;
    fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn CompareTo_(&self, objwbemobject: Option<&super::Com::IDispatch>, iflags: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn Properties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn Methods_(&self) -> windows_core::Result<ISWbemMethodSet>;
    fn Derivation_(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Path_(&self) -> windows_core::Result<ISWbemObjectPath>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemObject {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>() -> ISWbemObject_Vtbl {
        unsafe extern "system" fn Put_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Put_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::PutAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::Delete_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::DeleteAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Instances_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Instances_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::InstancesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Subclasses_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Subclasses_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::SubclassesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Associators_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strassocclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultrole: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Associators_(this, core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            objwbemsink: *mut core::ffi::c_void,
            strassocclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultrole: core::mem::MaybeUninit<windows_core::BSTR>,
            strrole: core::mem::MaybeUninit<windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut core::ffi::c_void,
            objwbemasynccontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::AssociatorsAsync_(
                this,
                windows_core::from_raw_borrowed(&objwbemsink),
                core::mem::transmute(&strassocclass),
                core::mem::transmute(&strresultclass),
                core::mem::transmute(&strresultrole),
                core::mem::transmute(&strrole),
                core::mem::transmute_copy(&bclassesonly),
                core::mem::transmute_copy(&bschemaonly),
                core::mem::transmute(&strrequiredassocqualifier),
                core::mem::transmute(&strrequiredqualifier),
                core::mem::transmute_copy(&iflags),
                windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                windows_core::from_raw_borrowed(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn References_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::References_(this, core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::ReferencesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::ExecMethod_(this, core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemoutparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObject_Impl::ExecMethodAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Clone_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Clone_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, strobjecttext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::GetObjectText_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(strobjecttext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::SpawnDerivedClass_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::SpawnInstance_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, bresult: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::CompareTo_(this, windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(bresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemqualifierset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Properties_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbempropertyset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemmethodset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Methods_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemmethodset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Derivation_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclassnamearray: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Derivation_(this) {
                Ok(ok__) => {
                    core::ptr::write(strclassnamearray, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Path_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObject_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Put_: Put_::<Identity, Impl, OFFSET>,
            PutAsync_: PutAsync_::<Identity, Impl, OFFSET>,
            Delete_: Delete_::<Identity, Impl, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, Impl, OFFSET>,
            Instances_: Instances_::<Identity, Impl, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, Impl, OFFSET>,
            Subclasses_: Subclasses_::<Identity, Impl, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, Impl, OFFSET>,
            Associators_: Associators_::<Identity, Impl, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, Impl, OFFSET>,
            References_: References_::<Identity, Impl, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, Impl, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, Impl, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, Impl, OFFSET>,
            Clone_: Clone_::<Identity, Impl, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, Impl, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, Impl, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, Impl, OFFSET>,
            CompareTo_: CompareTo_::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            Properties_: Properties_::<Identity, Impl, OFFSET>,
            Methods_: Methods_::<Identity, Impl, OFFSET>,
            Derivation_: Derivation_::<Identity, Impl, OFFSET>,
            Path_: Path_::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectEx_Impl: Sized + ISWbemObject_Impl {
    fn Refresh_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn GetText_(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<windows_core::BSTR>;
    fn SetFromText_(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemObjectEx {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>() -> ISWbemObjectEx_Vtbl {
        unsafe extern "system" fn Refresh_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectEx_Impl::Refresh_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn SystemProperties_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectEx_Impl::SystemProperties_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbempropertyset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, bstext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectEx_Impl::GetText_(this, core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(bstext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromText_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstext: core::mem::MaybeUninit<windows_core::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectEx_Impl::SetFromText_(this, core::mem::transmute(&bstext), core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        Self {
            base__: ISWbemObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Refresh_: Refresh_::<Identity, Impl, OFFSET>,
            SystemProperties_: SystemProperties_::<Identity, Impl, OFFSET>,
            GetText_: GetText_::<Identity, Impl, OFFSET>,
            SetFromText_: SetFromText_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectPath_Impl: Sized + super::Com::IDispatch_Impl {
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemObjectPath {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>() -> ISWbemObjectPath_Vtbl {
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(strpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetPath(this, core::mem::transmute(&strpath)).into()
        }
        unsafe extern "system" fn RelPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::RelPath(this) {
                Ok(ok__) => {
                    core::ptr::write(strrelpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetRelPath(this, core::mem::transmute(&strrelpath)).into()
        }
        unsafe extern "system" fn Server<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Server(this) {
                Ok(ok__) => {
                    core::ptr::write(strserver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetServer(this, core::mem::transmute(&strserver)).into()
        }
        unsafe extern "system" fn Namespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Namespace(this) {
                Ok(ok__) => {
                    core::ptr::write(strnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetNamespace(this, core::mem::transmute(&strnamespace)).into()
        }
        unsafe extern "system" fn ParentNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strparentnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::ParentNamespace(this) {
                Ok(ok__) => {
                    core::ptr::write(strparentnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(strdisplayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetDisplayName(this, core::mem::transmute(&strdisplayname)).into()
        }
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Class(this) {
                Ok(ok__) => {
                    core::ptr::write(strclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetClass(this, core::mem::transmute(&strclass)).into()
        }
        unsafe extern "system" fn IsClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::IsClass(this) {
                Ok(ok__) => {
                    core::ptr::write(bisclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetAsClass(this).into()
        }
        unsafe extern "system" fn IsSingleton<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bissingleton: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::IsSingleton(this) {
                Ok(ok__) => {
                    core::ptr::write(bissingleton, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSingleton<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetAsSingleton(this).into()
        }
        unsafe extern "system" fn Keys<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Keys(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemnamedvalueset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Locale(this) {
                Ok(ok__) => {
                    core::ptr::write(strlocale, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetLocale(this, core::mem::transmute(&strlocale)).into()
        }
        unsafe extern "system" fn Authority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectPath_Impl::Authority(this) {
                Ok(ok__) => {
                    core::ptr::write(strauthority, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemObjectPath_Impl::SetAuthority(this, core::mem::transmute(&strauthority)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            RelPath: RelPath::<Identity, Impl, OFFSET>,
            SetRelPath: SetRelPath::<Identity, Impl, OFFSET>,
            Server: Server::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetNamespace: SetNamespace::<Identity, Impl, OFFSET>,
            ParentNamespace: ParentNamespace::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            IsClass: IsClass::<Identity, Impl, OFFSET>,
            SetAsClass: SetAsClass::<Identity, Impl, OFFSET>,
            IsSingleton: IsSingleton::<Identity, Impl, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, Impl, OFFSET>,
            Keys: Keys::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            Locale: Locale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            Authority: Authority::<Identity, Impl, OFFSET>,
            SetAuthority: SetAuthority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectPath as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemObjectSet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>() -> ISWbemObjectSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectSet_Impl::Item(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectSet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectSet_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemObjectSet_Impl::ItemIndex(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            ItemIndex: ItemIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilege_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEnabled(&self, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemPrivilege {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilege_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>() -> ISWbemPrivilege_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilege_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(bisenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemPrivilege_Impl::SetIsEnabled(this, core::mem::transmute_copy(&bisenabled)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilege_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(strdisplayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilege_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(strdisplayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilege_Impl::Identifier(this) {
                Ok(ok__) => {
                    core::ptr::write(iprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Identifier: Identifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilege as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilegeSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
    fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
    fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemPrivilegeSet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilegeSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>() -> ISWbemPrivilegeSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilegeSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilegeSet_Impl::Item(this, core::mem::transmute_copy(&iprivilege)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilegeSet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilegeSet_Impl::Add(this, core::mem::transmute_copy(&iprivilege), core::mem::transmute_copy(&bisenabled)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemPrivilegeSet_Impl::Remove(this, core::mem::transmute_copy(&iprivilege)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemPrivilegeSet_Impl::DeleteAll(this).into()
        }
        unsafe extern "system" fn AddAsString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprivilege: core::mem::MaybeUninit<windows_core::BSTR>, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPrivilegeSet_Impl::AddAsString(this, core::mem::transmute(&strprivilege), core::mem::transmute_copy(&bisenabled)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemprivilege, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
            AddAsString: AddAsString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CIMType(&self) -> windows_core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn IsArray(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>() -> ISWbemProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(varvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemProperty_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(strname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::IsLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(bislocal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::Origin(this) {
                Ok(ok__) => {
                    core::ptr::write(strorigin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CIMType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::CIMType(this) {
                Ok(ok__) => {
                    core::ptr::write(icimtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemqualifierset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisarray: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemProperty_Impl::IsArray(this) {
                Ok(ok__) => {
                    core::ptr::write(bisarray, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            CIMType: CIMType::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            IsArray: IsArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPropertySet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemPropertySet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>() -> ISWbemPropertySet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPropertySet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPropertySet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPropertySet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemPropertySet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&icimtype), core::mem::transmute_copy(&bisarray), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemPropertySet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPropertySet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifier_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, varvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemQualifier {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>() -> ISWbemQualifier_Vtbl {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(varvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemQualifier_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(strname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::IsLocal(this) {
                Ok(ok__) => {
                    core::ptr::write(bislocal, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::PropagatesToSubclass(this) {
                Ok(ok__) => {
                    core::ptr::write(bpropagatestosubclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemQualifier_Impl::SetPropagatesToSubclass(this, core::mem::transmute_copy(&bpropagatestosubclass)).into()
        }
        unsafe extern "system" fn PropagatesToInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::PropagatesToInstance(this) {
                Ok(ok__) => {
                    core::ptr::write(bpropagatestoinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemQualifier_Impl::SetPropagatesToInstance(this, core::mem::transmute_copy(&bpropagatestoinstance)).into()
        }
        unsafe extern "system" fn IsOverridable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::IsOverridable(this) {
                Ok(ok__) => {
                    core::ptr::write(bisoverridable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverridable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemQualifier_Impl::SetIsOverridable(this, core::mem::transmute_copy(&bisoverridable)).into()
        }
        unsafe extern "system" fn IsAmended<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisamended: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifier_Impl::IsAmended(this) {
                Ok(ok__) => {
                    core::ptr::write(bisamended, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Identity, Impl, OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Identity, Impl, OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Identity, Impl, OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Identity, Impl, OFFSET>,
            IsOverridable: IsOverridable::<Identity, Impl, OFFSET>,
            SetIsOverridable: SetIsOverridable::<Identity, Impl, OFFSET>,
            IsAmended: IsAmended::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifier as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifierSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varval: *const windows_core::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemQualifierSet {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifierSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>() -> ISWbemQualifierSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifierSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifierSet_Impl::Item(this, core::mem::transmute(&name), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemqualifier, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifierSet_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varval: *const core::mem::MaybeUninit<windows_core::VARIANT>, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemQualifierSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varval), core::mem::transmute_copy(&bpropagatestosubclass), core::mem::transmute_copy(&bpropagatestoinstance), core::mem::transmute_copy(&bisoverridable), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemqualifier, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemQualifierSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefreshableItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn Index(&self) -> windows_core::Result<i32>;
    fn Refresher(&self) -> windows_core::Result<ISWbemRefresher>;
    fn IsSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Object(&self) -> windows_core::Result<ISWbemObjectEx>;
    fn ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet>;
    fn Remove(&self, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemRefreshableItem {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefreshableItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>() -> ISWbemRefreshableItem_Vtbl {
        unsafe extern "system" fn Index<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefreshableItem_Impl::Index(this) {
                Ok(ok__) => {
                    core::ptr::write(iindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemrefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefreshableItem_Impl::Refresher(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemrefresher, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisset: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefreshableItem_Impl::IsSet(this) {
                Ok(ok__) => {
                    core::ptr::write(bisset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Object<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefreshableItem_Impl::Object(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefreshableItem_Impl::ObjectSet(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemRefreshableItem_Impl::Remove(this, core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Refresher: Refresher::<Identity, Impl, OFFSET>,
            IsSet: IsSet::<Identity, Impl, OFFSET>,
            Object: Object::<Identity, Impl, OFFSET>,
            ObjectSet: ObjectSet::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefresher_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, objwbemservices: Option<&ISWbemServicesEx>, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&self, objwbemservices: Option<&ISWbemServicesEx>, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()>;
    fn Refresh(&self, iflags: i32) -> windows_core::Result<()>;
    fn AutoReconnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoReconnect(&self, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemRefresher {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefresher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>() -> ISWbemRefresher_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::Item(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemrefreshableitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(icount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsinstancepath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::Add(this, windows_core::from_raw_borrowed(&objwbemservices), core::mem::transmute(&bsinstancepath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemrefreshableitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsclassname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::AddEnum(this, windows_core::from_raw_borrowed(&objwbemservices), core::mem::transmute(&bsclassname), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemrefreshableitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemRefresher_Impl::Remove(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn AutoReconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemRefresher_Impl::AutoReconnect(this) {
                Ok(ok__) => {
                    core::ptr::write(bcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemRefresher_Impl::SetAutoReconnect(this, core::mem::transmute_copy(&bcount)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemRefresher_Impl::DeleteAll(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            AutoReconnect: AutoReconnect::<Identity, Impl, OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefresher as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSecurity_Impl: Sized + super::Com::IDispatch_Impl {
    fn ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()>;
    fn AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()>;
    fn Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>() -> ISWbemSecurity_Vtbl {
        unsafe extern "system" fn ImpersonationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemSecurity_Impl::ImpersonationLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(iimpersonationlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemSecurity_Impl::SetImpersonationLevel(this, core::mem::transmute_copy(&iimpersonationlevel)).into()
        }
        unsafe extern "system" fn AuthenticationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemSecurity_Impl::AuthenticationLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(iauthenticationlevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemSecurity_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&iauthenticationlevel)).into()
        }
        unsafe extern "system" fn Privileges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemprivilegeset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemSecurity_Impl::Privileges(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemprivilegeset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImpersonationLevel: ImpersonationLevel::<Identity, Impl, OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Identity, Impl, OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Identity, Impl, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSecurity as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServices_Impl: Sized + super::Com::IDispatch_Impl {
    fn Get(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn GetAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn InstancesOf(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SubclassesOf(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn AssociatorsOf(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ReferencesTo(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemServices {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>() -> ISWbemServices_Vtbl {
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::Get(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::GetAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::Delete(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::DeleteAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn InstancesOf<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::InstancesOf(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::InstancesOfAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn SubclassesOf<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::SubclassesOf(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::SubclassesOfAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::ExecQueryAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn AssociatorsOf<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>,
            strassocclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultrole: core::mem::MaybeUninit<windows_core::BSTR>,
            strrole: core::mem::MaybeUninit<windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut core::ffi::c_void,
            objwbemobjectset: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::AssociatorsOf(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            objwbemsink: *mut core::ffi::c_void,
            strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>,
            strassocclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultrole: core::mem::MaybeUninit<windows_core::BSTR>,
            strrole: core::mem::MaybeUninit<windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut core::ffi::c_void,
            objwbemasynccontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::AssociatorsOfAsync(
                this,
                windows_core::from_raw_borrowed(&objwbemsink),
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
                windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                windows_core::from_raw_borrowed(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn ReferencesTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::ReferencesTo(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::ReferencesToAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemeventsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemeventsource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::ExecNotificationQueryAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemoutparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServices_Impl::ExecMethodAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Security_<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServices_Impl::Security_(this) {
                Ok(ok__) => {
                    core::ptr::write(objwbemsecurity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Get: Get::<Identity, Impl, OFFSET>,
            GetAsync: GetAsync::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, Impl, OFFSET>,
            InstancesOf: InstancesOf::<Identity, Impl, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, Impl, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, Impl, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, Impl, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, Impl, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, Impl, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServices as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServicesEx_Impl: Sized + ISWbemServices_Impl {
    fn Put(&self, objwbemobject: Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync(&self, objwbemsink: Option<&ISWbemSink>, objwbemobject: Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemServicesEx {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>() -> ISWbemServicesEx_Vtbl {
        unsafe extern "system" fn Put<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISWbemServicesEx_Impl::Put(this, windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    core::ptr::write(objwbemobjectpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemServicesEx_Impl::PutAsync(this, windows_core::from_raw_borrowed(&objwbemsink), windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        Self { base__: ISWbemServices_Vtbl::new::<Identity, Impl, OFFSET>(), Put: Put::<Identity, Impl, OFFSET>, PutAsync: PutAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServicesEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSink_Impl: Sized + super::Com::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemSink {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: isize>() -> ISWbemSink_Vtbl {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISWbemSink_Impl::Cancel(this).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSink as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSinkEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISWbemSinkEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSinkEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSinkEvents_Impl, const OFFSET: isize>() -> ISWbemSinkEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IUnsecuredApartment_Impl: Sized {
    fn CreateObjectStub(&self, pobject: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IUnsecuredApartment {}
impl IUnsecuredApartment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: isize>() -> IUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateObjectStub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUnsecuredApartment_Impl::CreateObjectStub(this, windows_core::from_raw_borrowed(&pobject)) {
                Ok(ok__) => {
                    core::ptr::write(ppstub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObjectStub: CreateObjectStub::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMIExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn WMIObjectPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWMIObject(&self) -> windows_core::Result<ISWbemObject>;
    fn GetWMIServices(&self) -> windows_core::Result<ISWbemServices>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMIExtension {}
#[cfg(feature = "Win32_System_Com")]
impl IWMIExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>() -> IWMIExtension_Vtbl {
        unsafe extern "system" fn WMIObjectPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strwmiobjectpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWMIExtension_Impl::WMIObjectPath(this) {
                Ok(ok__) => {
                    core::ptr::write(strwmiobjectpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWMIExtension_Impl::GetWMIObject(this) {
                Ok(ok__) => {
                    core::ptr::write(objwmiobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWMIExtension_Impl::GetWMIServices(this) {
                Ok(ok__) => {
                    core::ptr::write(objwmiservices, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WMIObjectPath: WMIObjectPath::<Identity, Impl, OFFSET>,
            GetWMIObject: GetWMIObject::<Identity, Impl, OFFSET>,
            GetWMIServices: GetWMIServices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIExtension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWbemAddressResolution_Impl: Sized {
    fn Resolve(&self, wsznamespacepath: &windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemAddressResolution {}
impl IWbemAddressResolution_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: isize>() -> IWbemAddressResolution_Vtbl {
        unsafe extern "system" fn Resolve<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespacepath: windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemAddressResolution_Impl::Resolve(this, core::mem::transmute(&wsznamespacepath), core::mem::transmute_copy(&wszaddresstype), core::mem::transmute_copy(&pdwaddresslength), core::mem::transmute_copy(&pabbinaryaddress)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemAddressResolution as windows_core::Interface>::IID
    }
}
pub trait IWbemBackupRestore_Impl: Sized {
    fn Backup(&self, strbackuptofile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn Restore(&self, strrestorefromfile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemBackupRestore {}
impl IWbemBackupRestore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>() -> IWbemBackupRestore_Vtbl {
        unsafe extern "system" fn Backup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbackuptofile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemBackupRestore_Impl::Backup(this, core::mem::transmute(&strbackuptofile), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrestorefromfile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemBackupRestore_Impl::Restore(this, core::mem::transmute(&strrestorefromfile), core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
pub trait IWbemBackupRestoreEx_Impl: Sized + IWbemBackupRestore_Impl {
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemBackupRestoreEx {}
impl IWbemBackupRestoreEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>() -> IWbemBackupRestoreEx_Vtbl {
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemBackupRestoreEx_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemBackupRestoreEx_Impl::Resume(this).into()
        }
        Self {
            base__: IWbemBackupRestore_Vtbl::new::<Identity, Impl, OFFSET>(),
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as windows_core::Interface>::IID || iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
pub trait IWbemCallResult_Impl: Sized {
    fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject>;
    fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices>;
    fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IWbemCallResult {}
impl IWbemCallResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>() -> IWbemCallResult_Vtbl {
        unsafe extern "system" fn GetResultObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemCallResult_Impl::GetResultObject(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    core::ptr::write(ppresultobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemCallResult_Impl::GetResultString(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    core::ptr::write(pstrresultstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemCallResult_Impl::GetResultServices(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    core::ptr::write(ppservices, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemCallResult_Impl::GetCallStatus(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    core::ptr::write(plstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResultObject: GetResultObject::<Identity, Impl, OFFSET>,
            GetResultString: GetResultString::<Identity, Impl, OFFSET>,
            GetResultServices: GetResultServices::<Identity, Impl, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemCallResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWbemClassObject_Impl: Sized {
    fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet>;
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut windows_core::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *const windows_core::VARIANT, r#type: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, wszqualifiername: &windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const windows_core::VARIANT) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut windows_core::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn GetPropertyQualifierSet(&self, wszproperty: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn Clone(&self) -> windows_core::Result<IWbemClassObject>;
    fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn CompareTo(&self, lflags: WBEM_COMPARISON_FLAG, pcompareto: Option<&IWbemClassObject>) -> windows_core::Result<()>;
    fn GetPropertyOrigin(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn InheritsFrom(&self, strancestor: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn PutMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, pinsignature: Option<&IWbemClassObject>, poutsignature: Option<&IWbemClassObject>) -> windows_core::Result<()>;
    fn DeleteMethod(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn EndMethodEnumeration(&self) -> windows_core::Result<()>;
    fn GetMethodQualifierSet(&self, wszmethod: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWbemClassObject {}
#[cfg(feature = "Win32_System_Com")]
impl IWbemClassObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>() -> IWbemClassObject_Vtbl {
        unsafe extern "system" fn GetQualifierSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetQualifierSet(this) {
                Ok(ok__) => {
                    core::ptr::write(ppqualset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *const core::mem::MaybeUninit<windows_core::VARIANT>, r#type: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::Delete(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszqualifiername: windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const core::mem::MaybeUninit<windows_core::VARIANT>, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetNames(this, core::mem::transmute(&wszqualifiername), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pqualifierval)) {
                Ok(ok__) => {
                    core::ptr::write(pnames, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&strname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::EndEnumeration(this).into()
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszproperty: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetPropertyQualifierSet(this, core::mem::transmute(&wszproperty)) {
                Ok(ok__) => {
                    core::ptr::write(ppqualset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcopy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrobjecttext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetObjectText(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(pstrobjecttext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::SpawnDerivedClass(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppnewclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::SpawnInstance(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppnewinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_COMPARISON_FLAG, pcompareto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::CompareTo(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcompareto)).into()
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pstrclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetPropertyOrigin(this, core::mem::transmute(&wszname)) {
                Ok(ok__) => {
                    core::ptr::write(pstrclassname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InheritsFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strancestor: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::InheritsFrom(this, core::mem::transmute(&strancestor)).into()
        }
        unsafe extern "system" fn GetMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::GetMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn PutMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pinsignature: *mut core::ffi::c_void, poutsignature: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::PutMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pinsignature), windows_core::from_raw_borrowed(&poutsignature)).into()
        }
        unsafe extern "system" fn DeleteMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::DeleteMethod(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::BeginMethodEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn NextMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::NextMethod(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClassObject_Impl::EndMethodEnumeration(this).into()
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethod: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetMethodQualifierSet(this, core::mem::transmute(&wszmethod)) {
                Ok(ok__) => {
                    core::ptr::write(ppqualset, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, pstrclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClassObject_Impl::GetMethodOrigin(this, core::mem::transmute(&wszmethodname)) {
                Ok(ok__) => {
                    core::ptr::write(pstrclassname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetObjectText: GetObjectText::<Identity, Impl, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, Impl, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, Impl, OFFSET>,
            CompareTo: CompareTo::<Identity, Impl, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, Impl, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, Impl, OFFSET>,
            GetMethod: GetMethod::<Identity, Impl, OFFSET>,
            PutMethod: PutMethod::<Identity, Impl, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, Impl, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, Impl, OFFSET>,
            NextMethod: NextMethod::<Identity, Impl, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, Impl, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, Impl, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IWbemClientConnectionTransport_Impl: Sized {
    fn Open(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn OpenAsync(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn Cancel(&self, lflags: i32, phandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemClientConnectionTransport {}
impl IWbemClientConnectionTransport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>() -> IWbemClientConnectionTransport_Vtbl {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClientConnectionTransport_Impl::Open(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface), core::mem::transmute_copy(&pcallres)).into()
        }
        unsafe extern "system" fn OpenAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClientConnectionTransport_Impl::OpenAsync(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemClientConnectionTransport_Impl::Cancel(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&phandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemClientTransport_Impl: Sized {
    fn ConnectServer(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemClientTransport {}
impl IWbemClientTransport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: isize>() -> IWbemClientTransport_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lsecurityflags: i32, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemClientTransport_Impl::ConnectServer(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemConfigureRefresher_Impl: Sized {
    fn AddObjectByPath(&self, pnamespace: Option<&IWbemServices>, wszpath: &windows_core::PCWSTR, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddObjectByTemplate(&self, pnamespace: Option<&IWbemServices>, ptemplate: Option<&IWbemClassObject>, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddRefresher(&self, prefresher: Option<&IWbemRefresher>, lflags: i32, plid: *mut i32) -> windows_core::Result<()>;
    fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn AddEnum(&self, pnamespace: Option<&IWbemServices>, wszclassname: &windows_core::PCWSTR, lflags: i32, pcontext: Option<&IWbemContext>, ppenum: *mut Option<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConfigureRefresher {}
impl IWbemConfigureRefresher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>() -> IWbemConfigureRefresher_Vtbl {
        unsafe extern "system" fn AddObjectByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConfigureRefresher_Impl::AddObjectByPath(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConfigureRefresher_Impl::AddObjectByTemplate(this, windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&ptemplate), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddRefresher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConfigureRefresher_Impl::AddRefresher(this, windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConfigureRefresher_Impl::Remove(this, core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclassname: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConfigureRefresher_Impl::AddEnum(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclassname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&plid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Identity, Impl, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, Impl, OFFSET>,
            AddRefresher: AddRefresher::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as windows_core::Interface>::IID
    }
}
pub trait IWbemConnectorLogin_Impl: Sized {
    fn ConnectorLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConnectorLogin {}
impl IWbemConnectorLogin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: isize>() -> IWbemConnectorLogin_Vtbl {
        unsafe extern "system" fn ConnectorLogin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConnectorLogin_Impl::ConnectorLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectorLogin: ConnectorLogin::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as windows_core::Interface>::IID
    }
}
pub trait IWbemConstructClassObject_Impl: Sized {
    fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPropertyOrigin(&self, wszpropertyname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetServerNamespace(&self, wszserver: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConstructClassObject {}
impl IWbemConstructClassObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>() -> IWbemConstructClassObject_Vtbl {
        unsafe extern "system" fn SetInheritanceChain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConstructClassObject_Impl::SetInheritanceChain(this, core::mem::transmute_copy(&lnumantecedents), core::mem::transmute_copy(&awszantecedents)).into()
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConstructClassObject_Impl::SetPropertyOrigin(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConstructClassObject_Impl::SetMethodOrigin(this, core::mem::transmute(&wszmethodname), core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetServerNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszserver: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemConstructClassObject_Impl::SetServerNamespace(this, core::mem::transmute(&wszserver), core::mem::transmute(&wsznamespace)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Identity, Impl, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, Impl, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, Impl, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWbemContext_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IWbemContext>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn SetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32, pvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn DeleteValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWbemContext {}
#[cfg(feature = "Win32_System_Com")]
impl IWbemContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>() -> IWbemContext_Vtbl {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemContext_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnewcopy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemContext_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(pnames, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::EndEnumeration(this).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::SetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemContext_Impl::GetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::DeleteValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemContext_Impl::DeleteAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemContext as windows_core::Interface>::IID
    }
}
pub trait IWbemDecoupledBasicEventProvider_Impl: Sized + IWbemDecoupledRegistrar_Impl {
    fn GetSink(&self, a_flags: i32, a_context: Option<&IWbemContext>) -> windows_core::Result<IWbemObjectSink>;
    fn GetService(&self, a_flags: i32, a_context: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemDecoupledBasicEventProvider {}
impl IWbemDecoupledBasicEventProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>() -> IWbemDecoupledBasicEventProvider_Vtbl {
        unsafe extern "system" fn GetSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_sink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemDecoupledBasicEventProvider_Impl::GetSink(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context)) {
                Ok(ok__) => {
                    core::ptr::write(a_sink, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemDecoupledBasicEventProvider_Impl::GetService(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context)) {
                Ok(ok__) => {
                    core::ptr::write(a_service, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWbemDecoupledRegistrar_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSink: GetSink::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as windows_core::Interface>::IID || iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
pub trait IWbemDecoupledRegistrar_Impl: Sized {
    fn Register(&self, a_flags: i32, a_context: Option<&IWbemContext>, a_user: &windows_core::PCWSTR, a_locale: &windows_core::PCWSTR, a_scope: &windows_core::PCWSTR, a_registration: &windows_core::PCWSTR, piunknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnRegister(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemDecoupledRegistrar {}
impl IWbemDecoupledRegistrar_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>() -> IWbemDecoupledRegistrar_Vtbl {
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_user: windows_core::PCWSTR, a_locale: windows_core::PCWSTR, a_scope: windows_core::PCWSTR, a_registration: windows_core::PCWSTR, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemDecoupledRegistrar_Impl::Register(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context), core::mem::transmute(&a_user), core::mem::transmute(&a_locale), core::mem::transmute(&a_scope), core::mem::transmute(&a_registration), windows_core::from_raw_borrowed(&piunknown)).into()
        }
        unsafe extern "system" fn UnRegister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemDecoupledRegistrar_Impl::UnRegister(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, Impl, OFFSET>,
            UnRegister: UnRegister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
pub trait IWbemEventConsumerProvider_Impl: Sized {
    fn FindConsumer(&self, plogicalconsumer: Option<&IWbemClassObject>) -> windows_core::Result<IWbemUnboundObjectSink>;
}
impl windows_core::RuntimeName for IWbemEventConsumerProvider {}
impl IWbemEventConsumerProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: isize>() -> IWbemEventConsumerProvider_Vtbl {
        unsafe extern "system" fn FindConsumer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, ppconsumer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemEventConsumerProvider_Impl::FindConsumer(this, windows_core::from_raw_borrowed(&plogicalconsumer)) {
                Ok(ok__) => {
                    core::ptr::write(ppconsumer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindConsumer: FindConsumer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProvider_Impl: Sized {
    fn ProvideEvents(&self, psink: Option<&IWbemObjectSink>, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProvider {}
impl IWbemEventProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: isize>() -> IWbemEventProvider_Vtbl {
        unsafe extern "system" fn ProvideEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventProvider_Impl::ProvideEvents(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideEvents: ProvideEvents::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProviderQuerySink_Impl: Sized {
    fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::Result<()>;
    fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProviderQuerySink {}
impl IWbemEventProviderQuerySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>() -> IWbemEventProviderQuerySink_Vtbl {
        unsafe extern "system" fn NewQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventProviderQuerySink_Impl::NewQuery(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery)).into()
        }
        unsafe extern "system" fn CancelQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventProviderQuerySink_Impl::CancelQuery(this, core::mem::transmute_copy(&dwid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewQuery: NewQuery::<Identity, Impl, OFFSET>,
            CancelQuery: CancelQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProviderSecurity_Impl: Sized {
    fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProviderSecurity {}
impl IWbemEventProviderSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: isize>() -> IWbemEventProviderSecurity_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventProviderSecurity_Impl::AccessCheck(this, core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery), core::mem::transmute_copy(&lsidlength), core::mem::transmute_copy(&psid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessCheck: AccessCheck::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as windows_core::Interface>::IID
    }
}
pub trait IWbemEventSink_Impl: Sized + IWbemObjectSink_Impl {
    fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<()>;
    fn GetRestrictedSink(&self, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventSink {}
impl IWbemEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>() -> IWbemEventSink_Vtbl {
        unsafe extern "system" fn SetSinkSecurity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsdlength: i32, psd: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventSink_Impl::SetSinkSecurity(this, core::mem::transmute_copy(&lsdlength), core::mem::transmute_copy(&psd)).into()
        }
        unsafe extern "system" fn IsActive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventSink_Impl::IsActive(this).into()
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemEventSink_Impl::GetRestrictedSink(this, core::mem::transmute_copy(&lnumqueries), core::mem::transmute_copy(&awszqueries), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    core::ptr::write(ppsink, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemEventSink_Impl::SetBatchingParameters(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&dwmaxsendlatency)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Identity, Impl, OFFSET>,
            IsActive: IsActive::<Identity, Impl, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, Impl, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventSink as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemHiPerfEnum_Impl: Sized {
    fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()>;
    fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::Result<()>;
    fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut Option<IWbemObjectAccess>, pureturned: *mut u32) -> windows_core::Result<()>;
    fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemHiPerfEnum {}
impl IWbemHiPerfEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>() -> IWbemHiPerfEnum_Vtbl {
        unsafe extern "system" fn AddObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfEnum_Impl::AddObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids), core::mem::transmute_copy(&apobj)).into()
        }
        unsafe extern "system" fn RemoveObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfEnum_Impl::RemoveObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids)).into()
        }
        unsafe extern "system" fn GetObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfEnum_Impl::GetObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&pureturned)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfEnum_Impl::RemoveAll(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjects: AddObjects::<Identity, Impl, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as windows_core::Interface>::IID
    }
}
pub trait IWbemHiPerfProvider_Impl: Sized {
    fn QueryInstances(&self, pnamespace: Option<&IWbemServices>, wszclass: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>, psink: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateRefresher(&self, pnamespace: Option<&IWbemServices>, lflags: i32) -> windows_core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(&self, pnamespace: Option<&IWbemServices>, ptemplate: Option<&IWbemObjectAccess>, prefresher: Option<&IWbemRefresher>, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>;
    fn StopRefreshing(&self, prefresher: Option<&IWbemRefresher>, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn CreateRefreshableEnum(&self, pnamespace: Option<&IWbemServices>, wszclass: &windows_core::PCWSTR, prefresher: Option<&IWbemRefresher>, lflags: i32, pcontext: Option<&IWbemContext>, phiperfenum: Option<&IWbemHiPerfEnum>) -> windows_core::Result<i32>;
    fn GetObjects(&self, pnamespace: Option<&IWbemServices>, lnumobjects: i32, apobj: *mut Option<IWbemObjectAccess>, lflags: i32, pcontext: Option<&IWbemContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemHiPerfProvider {}
impl IWbemHiPerfProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>() -> IWbemHiPerfProvider_Vtbl {
        unsafe extern "system" fn QueryInstances<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfProvider_Impl::QueryInstances(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn CreateRefresher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lflags: i32, pprefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemHiPerfProvider_Impl::CreateRefresher(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(pprefresher, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfProvider_Impl::CreateRefreshableObject(this, windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&ptemplate), windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn StopRefreshing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfProvider_Impl::StopRefreshing(this, windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, phiperfenum: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemHiPerfProvider_Impl::CreateRefreshableEnum(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclass), windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), windows_core::from_raw_borrowed(&phiperfenum)) {
                Ok(ok__) => {
                    core::ptr::write(plid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemHiPerfProvider_Impl::GetObjects(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInstances: QueryInstances::<Identity, Impl, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, Impl, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, Impl, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, Impl, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemLevel1Login_Impl: Sized {
    fn EstablishPosition(&self, wszlocalelist: &windows_core::PCWSTR, dwnumlocales: u32) -> windows_core::Result<u32>;
    fn RequestChallenge(&self, wsznetworkresource: &windows_core::PCWSTR, wszuser: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn WBEMLogin(&self, wszpreferredlocale: &windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
    fn NTLMLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemLevel1Login {}
impl IWbemLevel1Login_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>() -> IWbemLevel1Login_Vtbl {
        unsafe extern "system" fn EstablishPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlocalelist: windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemLevel1Login_Impl::EstablishPosition(this, core::mem::transmute(&wszlocalelist), core::mem::transmute_copy(&dwnumlocales)) {
                Ok(ok__) => {
                    core::ptr::write(reserved, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestChallenge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszuser: windows_core::PCWSTR, nonce: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemLevel1Login_Impl::RequestChallenge(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszuser)) {
                Ok(ok__) => {
                    core::ptr::write(nonce, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WBEMLogin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpreferredlocale: windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemLevel1Login_Impl::WBEMLogin(this, core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&accesstoken), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NTLMLogin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemLevel1Login_Impl::NTLMLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EstablishPosition: EstablishPosition::<Identity, Impl, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, Impl, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, Impl, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLevel1Login as windows_core::Interface>::IID
    }
}
pub trait IWbemLocator_Impl: Sized {
    fn ConnectServer(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemLocator {}
impl IWbemLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: isize>() -> IWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnetworkresource: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lsecurityflags: i32, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLocator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWbemObjectAccess_Impl: Sized + IWbemClassObject_Impl {
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWbemObjectAccess {}
#[cfg(feature = "Win32_System_Com")]
impl IWbemObjectAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>() -> IWbemObjectAccess_Vtbl {
        unsafe extern "system" fn GetPropertyHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::GetPropertyHandle(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plhandle)).into()
        }
        unsafe extern "system" fn WritePropertyValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::WritePropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lnumbytes), core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::ReadPropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lbuffersize), core::mem::transmute_copy(&plnumbytes), core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadDWORD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectAccess_Impl::ReadDWORD(this, core::mem::transmute_copy(&lhandle)) {
                Ok(ok__) => {
                    core::ptr::write(pdw, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteDWORD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, dw: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::WriteDWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ReadQWORD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectAccess_Impl::ReadQWORD(this, core::mem::transmute_copy(&lhandle)) {
                Ok(ok__) => {
                    core::ptr::write(pqw, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteQWORD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pw: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::WriteQWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pw)).into()
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, ptype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::GetPropertyInfoByHandle(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::Lock(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectAccess_Impl::Unlock(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: IWbemClassObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Identity, Impl, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, Impl, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, Impl, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, Impl, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, Impl, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, Impl, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, Impl, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectAccess as windows_core::Interface>::IID || iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IWbemObjectSink_Impl: Sized {
    fn Indicate(&self, lobjectcount: i32, apobjarray: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn SetStatus(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: Option<&IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemObjectSink {}
impl IWbemObjectSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>() -> IWbemObjectSink_Vtbl {
        unsafe extern "system" fn Indicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectSink_Impl::Indicate(this, core::mem::transmute_copy(&lobjectcount), core::mem::transmute_copy(&apobjarray)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hresult: windows_core::HRESULT, strparam: core::mem::MaybeUninit<windows_core::BSTR>, pobjparam: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectSink_Impl::SetStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hresult), core::mem::transmute(&strparam), windows_core::from_raw_borrowed(&pobjparam)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Indicate: Indicate::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemObjectSinkEx_Impl: Sized + IWbemObjectSink_Impl {
    fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WriteError(&self, pobjerror: Option<&IWbemClassObject>) -> windows_core::Result<u8>;
    fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8>;
    fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()>;
    fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const windows_core::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemObjectSinkEx {}
impl IWbemObjectSinkEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>() -> IWbemObjectSinkEx_Vtbl {
        unsafe extern "system" fn WriteMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uchannel: u32, strmessage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectSinkEx_Impl::WriteMessage(this, core::mem::transmute_copy(&uchannel), core::mem::transmute(&strmessage)).into()
        }
        unsafe extern "system" fn WriteError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjerror: *mut core::ffi::c_void, pureturned: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectSinkEx_Impl::WriteError(this, windows_core::from_raw_borrowed(&pobjerror)) {
                Ok(ok__) => {
                    core::ptr::write(pureturned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmessage: core::mem::MaybeUninit<windows_core::BSTR>, uprompttype: u8, pureturned: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectSinkEx_Impl::PromptUser(this, core::mem::transmute(&strmessage), core::mem::transmute_copy(&uprompttype)) {
                Ok(ok__) => {
                    core::ptr::write(pureturned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stractivity: core::mem::MaybeUninit<windows_core::BSTR>, strcurrentoperation: core::mem::MaybeUninit<windows_core::BSTR>, strstatusdescription: core::mem::MaybeUninit<windows_core::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectSinkEx_Impl::WriteProgress(this, core::mem::transmute(&stractivity), core::mem::transmute(&strcurrentoperation), core::mem::transmute(&strstatusdescription), core::mem::transmute_copy(&upercentcomplete), core::mem::transmute_copy(&usecondsremaining)).into()
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, vtvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>, ultype: u32, ulflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemObjectSinkEx_Impl::WriteStreamParameter(this, core::mem::transmute(&strname), core::mem::transmute_copy(&vtvalue), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteMessage: WriteMessage::<Identity, Impl, OFFSET>,
            WriteError: WriteError::<Identity, Impl, OFFSET>,
            PromptUser: PromptUser::<Identity, Impl, OFFSET>,
            WriteProgress: WriteProgress::<Identity, Impl, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemObjectTextSrc_Impl: Sized {
    fn GetText(&self, lflags: i32, pobj: Option<&IWbemClassObject>, uobjtextformat: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<windows_core::BSTR>;
    fn CreateFromText(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemClassObject>;
}
impl windows_core::RuntimeName for IWbemObjectTextSrc {}
impl IWbemObjectTextSrc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>() -> IWbemObjectTextSrc_Vtbl {
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pobj: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, strtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectTextSrc_Impl::GetText(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pobj), core::mem::transmute_copy(&uobjtextformat), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(strtext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strtext: core::mem::MaybeUninit<windows_core::BSTR>, uobjtextformat: u32, pctx: *mut core::ffi::c_void, pnewobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemObjectTextSrc_Impl::CreateFromText(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strtext), core::mem::transmute_copy(&uobjtextformat), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(pnewobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            CreateFromText: CreateFromText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as windows_core::Interface>::IID
    }
}
pub trait IWbemPath_Impl: Sized {
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
    fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut Option<IWbemPathKeyList>) -> windows_core::Result<()>;
    fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllScopes(&self) -> windows_core::Result<()>;
    fn SetClassName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClassName(&self, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&self, lflags: i32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()>;
    fn IsRelative(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsRelativeOrChild(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL;
    fn IsLocal(&self, wszmachine: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsSameClassName(&self, wszclass: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IWbemPath {}
impl IWbemPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>() -> IWbemPath_Vtbl {
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umode: u32, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetText(this, core::mem::transmute_copy(&umode), core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPath_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                Ok(ok__) => {
                    core::ptr::write(puresponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetServer(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetServer(this, core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPath_Impl::GetNamespaceCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::RemoveNamespaceAt(this, core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::RemoveAllNamespaces(this).into()
        }
        unsafe extern "system" fn GetScopeCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPath_Impl::GetScopeCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszclass: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszclass)).into()
        }
        unsafe extern "system" fn SetScopeFromText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetScopeFromText(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn GetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&puclassnamebufsize), core::mem::transmute_copy(&pszclass), core::mem::transmute_copy(&pkeylist)).into()
        }
        unsafe extern "system" fn GetScopeAsText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetScopeAsText(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&putextbufsize), core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn RemoveScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::RemoveScope(this, core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::RemoveAllScopes(this).into()
        }
        unsafe extern "system" fn SetClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::SetClassName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::GetClassName(this, core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetKeyList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPath_Impl::GetKeyList(this) {
                Ok(ok__) => {
                    core::ptr::write(pout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::CreateClassPart(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn DeleteClassPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::DeleteClassPart(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsRelative<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::IsRelative(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace))
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::IsRelativeOrChild(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace), core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn IsLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::IsLocal(this, core::mem::transmute(&wszmachine))
        }
        unsafe extern "system" fn IsSameClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPath_Impl::IsSameClassName(this, core::mem::transmute(&wszclass))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            GetServer: GetServer::<Identity, Impl, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, Impl, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, Impl, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, Impl, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, Impl, OFFSET>,
            GetScope: GetScope::<Identity, Impl, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, Impl, OFFSET>,
            RemoveScope: RemoveScope::<Identity, Impl, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            GetClassName: GetClassName::<Identity, Impl, OFFSET>,
            GetKeyList: GetKeyList::<Identity, Impl, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, Impl, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, Impl, OFFSET>,
            IsRelative: IsRelative::<Identity, Impl, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPath as windows_core::Interface>::IID
    }
}
pub trait IWbemPathKeyList_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn SetKey(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetKey2(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut windows_core::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn RemoveKey(&self, wszname: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()>;
    fn MakeSingleton(&self, bset: u8) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemPathKeyList {}
impl IWbemPathKeyList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>() -> IWbemPathKeyList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pukeycount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPathKeyList_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pukeycount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::SetKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn SetKey2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::SetKey2(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn GetKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::GetKey(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pukeyvalbufsize), core::mem::transmute_copy(&pkeyval), core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn GetKey2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::GetKey2(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pkeyvalue), core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn RemoveKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::RemoveKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::RemoveAllKeys(this, core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn MakeSingleton<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bset: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::MakeSingleton(this, core::mem::transmute_copy(&bset)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPathKeyList_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                Ok(ok__) => {
                    core::ptr::write(puresponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPathKeyList_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            SetKey: SetKey::<Identity, Impl, OFFSET>,
            SetKey2: SetKey2::<Identity, Impl, OFFSET>,
            GetKey: GetKey::<Identity, Impl, OFFSET>,
            GetKey2: GetKey2::<Identity, Impl, OFFSET>,
            RemoveKey: RemoveKey::<Identity, Impl, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, Impl, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPathKeyList as windows_core::Interface>::IID
    }
}
pub trait IWbemPropertyProvider_Impl: Sized {
    fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemPropertyProvider {}
impl IWbemPropertyProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>() -> IWbemPropertyProvider_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strclassmapping: core::mem::MaybeUninit<windows_core::BSTR>, strinstmapping: core::mem::MaybeUninit<windows_core::BSTR>, strpropmapping: core::mem::MaybeUninit<windows_core::BSTR>, pvvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemPropertyProvider_Impl::GetProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping)) {
                Ok(ok__) => {
                    core::ptr::write(pvvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strclassmapping: core::mem::MaybeUninit<windows_core::BSTR>, strinstmapping: core::mem::MaybeUninit<windows_core::BSTR>, strpropmapping: core::mem::MaybeUninit<windows_core::BSTR>, pvvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemPropertyProvider_Impl::PutProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping), core::mem::transmute_copy(&pvvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            PutProperty: PutProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderIdentity_Impl: Sized {
    fn SetRegistrationObject(&self, lflags: i32, pprovreg: Option<&IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderIdentity {}
impl IWbemProviderIdentity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: isize>() -> IWbemProviderIdentity_Vtbl {
        unsafe extern "system" fn SetRegistrationObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pprovreg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemProviderIdentity_Impl::SetRegistrationObject(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pprovreg)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderInit_Impl: Sized {
    fn Initialize(&self, wszuser: &windows_core::PCWSTR, lflags: i32, wsznamespace: &windows_core::PCWSTR, wszlocale: &windows_core::PCWSTR, pnamespace: Option<&IWbemServices>, pctx: Option<&IWbemContext>, pinitsink: Option<&IWbemProviderInitSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderInit {}
impl IWbemProviderInit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: isize>() -> IWbemProviderInit_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuser: windows_core::PCWSTR, lflags: i32, wsznamespace: windows_core::PCWSTR, wszlocale: windows_core::PCWSTR, pnamespace: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, pinitsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemProviderInit_Impl::Initialize(this, core::mem::transmute(&wszuser), core::mem::transmute_copy(&lflags), core::mem::transmute(&wsznamespace), core::mem::transmute(&wszlocale), windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinitsink)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInit as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderInitSink_Impl: Sized {
    fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderInitSink {}
impl IWbemProviderInitSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: isize>() -> IWbemProviderInitSink_Vtbl {
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstatus: i32, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemProviderInitSink_Impl::SetStatus(this, core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetStatus: SetStatus::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWbemQualifierSet_Impl: Sized {
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut windows_core::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, pval: *const windows_core::VARIANT, lflavor: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut windows_core::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWbemQualifierSet {}
#[cfg(feature = "Win32_System_Com")]
impl IWbemQualifierSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>() -> IWbemQualifierSet_Vtbl {
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>, plflavor: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pval: *const core::mem::MaybeUninit<windows_core::VARIANT>, lflavor: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&lflavor)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::Delete(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemQualifierSet_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(pnames, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>, plflavor: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQualifierSet_Impl::EndEnumeration(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQualifierSet as windows_core::Interface>::IID
    }
}
pub trait IWbemQuery_Impl: Sized {
    fn Empty(&self) -> windows_core::Result<()>;
    fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()>;
    fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()>;
    fn Parse(&self, pszlang: &windows_core::PCWSTR, pszquery: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemQuery {}
impl IWbemQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>() -> IWbemQuery_Vtbl {
        unsafe extern "system" fn Empty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::Empty(this).into()
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::SetLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::TestLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn Parse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlang: windows_core::PCWSTR, pszquery: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::Parse(this, core::mem::transmute(&pszlang), core::mem::transmute(&pszquery), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn GetAnalysis<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::GetAnalysis(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&panalysis)).into()
        }
        unsafe extern "system" fn FreeMemory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::FreeMemory(this, core::mem::transmute_copy(&pmem)).into()
        }
        unsafe extern "system" fn GetQueryInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemQuery_Impl::GetQueryInfo(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uinfoid), core::mem::transmute_copy(&ubufsize), core::mem::transmute_copy(&pdestbuf)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Empty: Empty::<Identity, Impl, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, Impl, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, Impl, OFFSET>,
            Parse: Parse::<Identity, Impl, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, Impl, OFFSET>,
            FreeMemory: FreeMemory::<Identity, Impl, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQuery as windows_core::Interface>::IID
    }
}
pub trait IWbemRefresher_Impl: Sized {
    fn Refresh(&self, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemRefresher {}
impl IWbemRefresher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: isize>() -> IWbemRefresher_Vtbl {
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Refresh: Refresh::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemRefresher as windows_core::Interface>::IID
    }
}
pub trait IWbemServices_Impl: Sized {
    fn OpenNamespace(&self, strnamespace: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppworkingnamespace: *mut Option<IWbemServices>, ppresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, psink: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn QueryObjectSink(&self, lflags: WBEM_GENERIC_FLAG_TYPE) -> windows_core::Result<IWbemObjectSink>;
    fn GetObject(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppobject: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn GetObjectAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutClass(&self, pobject: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutClassAsync(&self, pobject: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteClass(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteClassAsync(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateClassEnum(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutInstance(&self, pinst: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutInstanceAsync(&self, pinst: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteInstance(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteInstanceAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateInstanceEnum(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, pinparams: Option<&IWbemClassObject>, ppoutparams: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn ExecMethodAsync(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, pinparams: Option<&IWbemClassObject>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemServices {}
impl IWbemServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>() -> IWbemServices_Vtbl {
        unsafe extern "system" fn OpenNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppworkingnamespace: *mut *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::OpenNamespace(this, core::mem::transmute(&strnamespace), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppworkingnamespace), core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::CancelAsyncCall(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn QueryObjectSink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, ppresponsehandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemServices_Impl::QueryObjectSink(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppresponsehandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::GetObject(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppobject), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn GetObjectAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::GetObjectAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::PutClass(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutClassAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::PutClassAsync(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::DeleteClass(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::DeleteClassAsync(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateClassEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemServices_Impl::CreateClassEnum(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::CreateClassEnumAsync(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::PutInstance(this, windows_core::from_raw_borrowed(&pinst), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::PutInstanceAsync(this, windows_core::from_raw_borrowed(&pinst), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::DeleteInstance(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::DeleteInstanceAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemServices_Impl::CreateInstanceEnum(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::CreateInstanceEnumAsync(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, ppoutparams: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinparams), core::mem::transmute_copy(&ppoutparams), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinparams), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenNamespace: OpenNamespace::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, Impl, OFFSET>,
            PutClass: PutClass::<Identity, Impl, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, Impl, OFFSET>,
            DeleteClass: DeleteClass::<Identity, Impl, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, Impl, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, Impl, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, Impl, OFFSET>,
            PutInstance: PutInstance::<Identity, Impl, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, Impl, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, Impl, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, Impl, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, Impl, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemServices as windows_core::Interface>::IID
    }
}
pub trait IWbemShutdown_Impl: Sized {
    fn Shutdown(&self, ureason: i32, umaxmilliseconds: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemShutdown {}
impl IWbemShutdown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: isize>() -> IWbemShutdown_Vtbl {
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemShutdown_Impl::Shutdown(this, core::mem::transmute_copy(&ureason), core::mem::transmute_copy(&umaxmilliseconds), windows_core::from_raw_borrowed(&pctx)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemShutdown as windows_core::Interface>::IID
    }
}
pub trait IWbemStatusCodeText_Impl: Sized {
    fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IWbemStatusCodeText {}
impl IWbemStatusCodeText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>() -> IWbemStatusCodeText_Vtbl {
        unsafe extern "system" fn GetErrorCodeText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemStatusCodeText_Impl::GetErrorCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(messagetext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemStatusCodeText_Impl::GetFacilityCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(messagetext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Identity, Impl, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as windows_core::Interface>::IID
    }
}
pub trait IWbemTransport_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemTransport {}
impl IWbemTransport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: isize>() -> IWbemTransport_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemTransport_Impl::Initialize(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemUnboundObjectSink_Impl: Sized {
    fn IndicateToConsumer(&self, plogicalconsumer: Option<&IWbemClassObject>, lnumobjects: i32, apobjects: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemUnboundObjectSink {}
impl IWbemUnboundObjectSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: isize>() -> IWbemUnboundObjectSink_Vtbl {
        unsafe extern "system" fn IndicateToConsumer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWbemUnboundObjectSink_Impl::IndicateToConsumer(this, windows_core::from_raw_borrowed(&plogicalconsumer), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobjects)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemUnsecuredApartment_Impl: Sized + IUnsecuredApartment_Impl {
    fn CreateSinkStub(&self, psink: Option<&IWbemObjectSink>, dwflags: u32, wszreserved: &windows_core::PCWSTR) -> windows_core::Result<IWbemObjectSink>;
}
impl windows_core::RuntimeName for IWbemUnsecuredApartment {}
impl IWbemUnsecuredApartment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: isize>() -> IWbemUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateSinkStub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwflags: u32, wszreserved: windows_core::PCWSTR, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWbemUnsecuredApartment_Impl::CreateSinkStub(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&dwflags), core::mem::transmute(&wszreserved)) {
                Ok(ok__) => {
                    core::ptr::write(ppstub, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUnsecuredApartment_Vtbl::new::<Identity, Impl, OFFSET>(), CreateSinkStub: CreateSinkStub::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as windows_core::Interface>::IID || iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
