#[cfg(feature = "Win32_System_Com")]
pub trait DICounterItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DICounterItem {}
#[cfg(feature = "Win32_System_Com")]
impl DICounterItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DICounterItem_Vtbl
    where
        Identity: DICounterItem_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DICounterItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DILogFileItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
impl DILogFileItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DILogFileItem_Vtbl
    where
        Identity: DILogFileItem_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DILogFileItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitor_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DISystemMonitor {}
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DISystemMonitor_Vtbl
    where
        Identity: DISystemMonitor_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitor as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitorEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DISystemMonitorEvents_Vtbl
    where
        Identity: DISystemMonitorEvents_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitorEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitorInternal_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DISystemMonitorInternal {}
#[cfg(feature = "Win32_System_Com")]
impl DISystemMonitorInternal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DISystemMonitorInternal_Vtbl
    where
        Identity: DISystemMonitorInternal_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitorInternal as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAlertDataCollector_Impl: Sized + IDataCollector_Impl {
    fn AlertThresholds(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn EventLog(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventLog(&self, log: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAlertDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl IAlertDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAlertDataCollector_Vtbl
    where
        Identity: IAlertDataCollector_Impl,
    {
        unsafe extern "system" fn AlertThresholds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::AlertThresholds(this) {
                Ok(ok__) => {
                    alerts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertThresholds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetAlertThresholds(this, core::mem::transmute_copy(&alerts)).into()
        }
        unsafe extern "system" fn EventLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::EventLog(this) {
                Ok(ok__) => {
                    log.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetEventLog(this, core::mem::transmute_copy(&log)).into()
        }
        unsafe extern "system" fn SampleInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::SampleInterval(this) {
                Ok(ok__) => {
                    interval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn Task<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::Task(this) {
                Ok(ok__) => {
                    task.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetTask(this, core::mem::transmute(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::TaskRunAsSelf(this) {
                Ok(ok__) => {
                    runasself.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::TaskArguments(this) {
                Ok(ok__) => {
                    task.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::TaskUserTextArguments(this) {
                Ok(ok__) => {
                    task.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&task)).into()
        }
        unsafe extern "system" fn TriggerDataCollectorSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlertDataCollector_Impl::TriggerDataCollectorSet(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggerDataCollectorSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IAlertDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlertDataCollector_Impl::SetTriggerDataCollectorSet(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            AlertThresholds: AlertThresholds::<Identity, OFFSET>,
            SetAlertThresholds: SetAlertThresholds::<Identity, OFFSET>,
            EventLog: EventLog::<Identity, OFFSET>,
            SetEventLog: SetEventLog::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            TriggerDataCollectorSet: TriggerDataCollectorSet::<Identity, OFFSET>,
            SetTriggerDataCollectorSet: SetTriggerDataCollectorSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlertDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IApiTracingDataCollector_Impl: Sized + IDataCollector_Impl {
    fn LogApiNamesOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogApiNamesOnly(&self, logapinames: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogApisRecursively(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogApisRecursively(&self, logrecursively: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExePath(&self, exepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IncludeModules(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn IncludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn ExcludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IApiTracingDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl IApiTracingDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IApiTracingDataCollector_Vtbl
    where
        Identity: IApiTracingDataCollector_Impl,
    {
        unsafe extern "system" fn LogApiNamesOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::LogApiNamesOnly(this) {
                Ok(ok__) => {
                    logapinames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApiNamesOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetLogApiNamesOnly(this, core::mem::transmute_copy(&logapinames)).into()
        }
        unsafe extern "system" fn LogApisRecursively<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::LogApisRecursively(this) {
                Ok(ok__) => {
                    logrecursively.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApisRecursively<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetLogApisRecursively(this, core::mem::transmute_copy(&logrecursively)).into()
        }
        unsafe extern "system" fn ExePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::ExePath(this) {
                Ok(ok__) => {
                    exepath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetExePath(this, core::mem::transmute(&exepath)).into()
        }
        unsafe extern "system" fn LogFilePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::LogFilePath(this) {
                Ok(ok__) => {
                    logfilepath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFilePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetLogFilePath(this, core::mem::transmute(&logfilepath)).into()
        }
        unsafe extern "system" fn IncludeModules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::IncludeModules(this) {
                Ok(ok__) => {
                    includemodules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeModules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetIncludeModules(this, core::mem::transmute_copy(&includemodules)).into()
        }
        unsafe extern "system" fn IncludeApis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::IncludeApis(this) {
                Ok(ok__) => {
                    includeapis.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeApis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetIncludeApis(this, core::mem::transmute_copy(&includeapis)).into()
        }
        unsafe extern "system" fn ExcludeApis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApiTracingDataCollector_Impl::ExcludeApis(this) {
                Ok(ok__) => {
                    excludeapis.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeApis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IApiTracingDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApiTracingDataCollector_Impl::SetExcludeApis(this, core::mem::transmute_copy(&excludeapis)).into()
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            LogApiNamesOnly: LogApiNamesOnly::<Identity, OFFSET>,
            SetLogApiNamesOnly: SetLogApiNamesOnly::<Identity, OFFSET>,
            LogApisRecursively: LogApisRecursively::<Identity, OFFSET>,
            SetLogApisRecursively: SetLogApisRecursively::<Identity, OFFSET>,
            ExePath: ExePath::<Identity, OFFSET>,
            SetExePath: SetExePath::<Identity, OFFSET>,
            LogFilePath: LogFilePath::<Identity, OFFSET>,
            SetLogFilePath: SetLogFilePath::<Identity, OFFSET>,
            IncludeModules: IncludeModules::<Identity, OFFSET>,
            SetIncludeModules: SetIncludeModules::<Identity, OFFSET>,
            IncludeApis: IncludeApis::<Identity, OFFSET>,
            SetIncludeApis: SetIncludeApis::<Identity, OFFSET>,
            ExcludeApis: ExcludeApis::<Identity, OFFSET>,
            SetExcludeApis: SetExcludeApis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConfigurationDataCollector_Impl: Sized + IDataCollector_Impl {
    fn FileMaxCount(&self) -> windows_core::Result<u32>;
    fn SetFileMaxCount(&self, count: u32) -> windows_core::Result<()>;
    fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn FileMaxTotalSize(&self) -> windows_core::Result<u32>;
    fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::Result<()>;
    fn Files(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn ManagementQueries(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn QueryNetworkAdapters(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetQueryNetworkAdapters(&self, network: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RegistryKeys(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IConfigurationDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl IConfigurationDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConfigurationDataCollector_Vtbl
    where
        Identity: IConfigurationDataCollector_Impl,
    {
        unsafe extern "system" fn FileMaxCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::FileMaxCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetFileMaxCount(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn FileMaxRecursiveDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::FileMaxRecursiveDepth(this) {
                Ok(ok__) => {
                    depth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxRecursiveDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetFileMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn FileMaxTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::FileMaxTotalSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetFileMaxTotalSize(this, core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn Files<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::Files(this) {
                Ok(ok__) => {
                    files.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetFiles(this, core::mem::transmute_copy(&files)).into()
        }
        unsafe extern "system" fn ManagementQueries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::ManagementQueries(this) {
                Ok(ok__) => {
                    queries.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManagementQueries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetManagementQueries(this, core::mem::transmute_copy(&queries)).into()
        }
        unsafe extern "system" fn QueryNetworkAdapters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::QueryNetworkAdapters(this) {
                Ok(ok__) => {
                    network.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryNetworkAdapters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetQueryNetworkAdapters(this, core::mem::transmute_copy(&network)).into()
        }
        unsafe extern "system" fn RegistryKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::RegistryKeys(this) {
                Ok(ok__) => {
                    query.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetRegistryKeys(this, core::mem::transmute_copy(&query)).into()
        }
        unsafe extern "system" fn RegistryMaxRecursiveDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::RegistryMaxRecursiveDepth(this) {
                Ok(ok__) => {
                    depth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryMaxRecursiveDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetRegistryMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn SystemStateFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConfigurationDataCollector_Impl::SystemStateFile(this) {
                Ok(ok__) => {
                    filename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemStateFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IConfigurationDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConfigurationDataCollector_Impl::SetSystemStateFile(this, core::mem::transmute(&filename)).into()
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            FileMaxCount: FileMaxCount::<Identity, OFFSET>,
            SetFileMaxCount: SetFileMaxCount::<Identity, OFFSET>,
            FileMaxRecursiveDepth: FileMaxRecursiveDepth::<Identity, OFFSET>,
            SetFileMaxRecursiveDepth: SetFileMaxRecursiveDepth::<Identity, OFFSET>,
            FileMaxTotalSize: FileMaxTotalSize::<Identity, OFFSET>,
            SetFileMaxTotalSize: SetFileMaxTotalSize::<Identity, OFFSET>,
            Files: Files::<Identity, OFFSET>,
            SetFiles: SetFiles::<Identity, OFFSET>,
            ManagementQueries: ManagementQueries::<Identity, OFFSET>,
            SetManagementQueries: SetManagementQueries::<Identity, OFFSET>,
            QueryNetworkAdapters: QueryNetworkAdapters::<Identity, OFFSET>,
            SetQueryNetworkAdapters: SetQueryNetworkAdapters::<Identity, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, OFFSET>,
            SetRegistryKeys: SetRegistryKeys::<Identity, OFFSET>,
            RegistryMaxRecursiveDepth: RegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SetRegistryMaxRecursiveDepth: SetRegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SystemStateFile: SystemStateFile::<Identity, OFFSET>,
            SetSystemStateFile: SetSystemStateFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
pub trait ICounterItem_Impl: Sized {
    fn Value(&self) -> windows_core::Result<f64>;
    fn SetColor(&self, color: u32) -> windows_core::Result<()>;
    fn Color(&self) -> windows_core::Result<u32>;
    fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()>;
    fn LineStyle(&self) -> windows_core::Result<i32>;
    fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()>;
    fn ScaleFactor(&self) -> windows_core::Result<i32>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICounterItem {}
impl ICounterItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICounterItem_Vtbl
    where
        Identity: ICounterItem_Impl,
    {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblvalue: *mut f64) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::Value(this) {
                Ok(ok__) => {
                    pdblvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::Color(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwidth: i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::SetWidth(this, core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::Width(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ilinestyle: i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::SetLineStyle(this, core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::LineStyle(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscale: i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::SetScaleFactor(this, core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::ScaleFactor(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem_Impl::Path(this) {
                Ok(ok__) => {
                    pstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, status: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::GetValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounterItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem_Impl::GetStatistics(this, core::mem::transmute_copy(&max), core::mem::transmute_copy(&min), core::mem::transmute_copy(&avg), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Color: Color::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, OFFSET>,
            LineStyle: LineStyle::<Identity, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounterItem as windows_core::Interface>::IID
    }
}
pub trait ICounterItem2_Impl: Sized + ICounterItem_Impl {
    fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ICounterItem2 {}
impl ICounterItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICounterItem2_Vtbl
    where
        Identity: ICounterItem2_Impl,
    {
        unsafe extern "system" fn SetSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICounterItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem2_Impl::SetSelected(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICounterItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem2_Impl::Selected(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICounterItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounterItem2_Impl::SetVisible(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICounterItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem2_Impl::Visible(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICounterItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounterItem2_Impl::GetDataAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iwhich)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICounterItem_Vtbl::new::<Identity, OFFSET>(),
            SetSelected: SetSelected::<Identity, OFFSET>,
            Selected: Selected::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            GetDataAt: GetDataAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounterItem2 as windows_core::Interface>::IID || iid == &<ICounterItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICounters_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<DICounterItem>;
    fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DICounterItem>;
    fn Remove(&self, index: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICounters {}
#[cfg(feature = "Win32_System_Com")]
impl ICounters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICounters_Vtbl
    where
        Identity: ICounters_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plong: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICounters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounters_Impl::Count(this) {
                Ok(ok__) => {
                    plong.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICounters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounters_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICounters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounters_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathname: core::mem::MaybeUninit<windows_core::BSTR>, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICounters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICounters_Impl::Add(this, core::mem::transmute(&pathname)) {
                Ok(ok__) => {
                    ppi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICounters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICounters_Impl::Remove(this, core::mem::transmute(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounters as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollector_Impl: Sized + super::Com::IDispatch_Impl {
    fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet>;
    fn SetDataCollectorSet(&self, group: Option<&IDataCollectorSet>) -> windows_core::Result<()>;
    fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType>;
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogAppend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogAppend(&self, append: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogCircular(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogCircular(&self, circular: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOverwrite(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Index(&self) -> windows_core::Result<i32>;
    fn SetIndex(&self, index: i32) -> windows_core::Result<()>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn CreateOutputLocation(&self, latest: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl IDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataCollector_Vtbl
    where
        Identity: IDataCollector_Impl,
    {
        unsafe extern "system" fn DataCollectorSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::DataCollectorSet(this) {
                Ok(ok__) => {
                    group.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCollectorSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetDataCollectorSet(this, windows_core::from_raw_borrowed(&group)).into()
        }
        unsafe extern "system" fn DataCollectorType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DataCollectorType) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::DataCollectorType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::FileName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetFileName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn FileNameFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::FileNameFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetFileNameFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn FileNameFormatPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::FileNameFormatPattern(this) {
                Ok(ok__) => {
                    pattern.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormatPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetFileNameFormatPattern(this, core::mem::transmute(&pattern)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::LatestOutputLocation(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn LogAppend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::LogAppend(this) {
                Ok(ok__) => {
                    append.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogAppend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetLogAppend(this, core::mem::transmute_copy(&append)).into()
        }
        unsafe extern "system" fn LogCircular<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::LogCircular(this) {
                Ok(ok__) => {
                    circular.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogCircular<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetLogCircular(this, core::mem::transmute_copy(&circular)).into()
        }
        unsafe extern "system" fn LogOverwrite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::LogOverwrite(this) {
                Ok(ok__) => {
                    overwrite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOverwrite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetLogOverwrite(this, core::mem::transmute_copy(&overwrite)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn OutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::OutputLocation(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::Index(this) {
                Ok(ok__) => {
                    index.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollector_Impl::SetIndex(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::Xml(this) {
                Ok(ok__) => {
                    xml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: core::mem::MaybeUninit<windows_core::BSTR>, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::SetXml(this, core::mem::transmute(&xml)) {
                Ok(ok__) => {
                    validation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, latest: super::super::Foundation::VARIANT_BOOL, location: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollector_Impl::CreateOutputLocation(this, core::mem::transmute_copy(&latest)) {
                Ok(ok__) => {
                    location.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectorSet: DataCollectorSet::<Identity, OFFSET>,
            SetDataCollectorSet: SetDataCollectorSet::<Identity, OFFSET>,
            DataCollectorType: DataCollectorType::<Identity, OFFSET>,
            FileName: FileName::<Identity, OFFSET>,
            SetFileName: SetFileName::<Identity, OFFSET>,
            FileNameFormat: FileNameFormat::<Identity, OFFSET>,
            SetFileNameFormat: SetFileNameFormat::<Identity, OFFSET>,
            FileNameFormatPattern: FileNameFormatPattern::<Identity, OFFSET>,
            SetFileNameFormatPattern: SetFileNameFormatPattern::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            LogAppend: LogAppend::<Identity, OFFSET>,
            SetLogAppend: SetLogAppend::<Identity, OFFSET>,
            LogCircular: LogCircular::<Identity, OFFSET>,
            SetLogCircular: SetLogCircular::<Identity, OFFSET>,
            LogOverwrite: LogOverwrite::<Identity, OFFSET>,
            SetLogOverwrite: SetLogOverwrite::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            Index: Index::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IDataCollector>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, collector: Option<&IDataCollector>) -> windows_core::Result<()>;
    fn Remove(&self, collector: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, collectors: Option<&IDataCollectorCollection>) -> windows_core::Result<()>;
    fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: *mut Option<IValueMap>, pcollector: *mut Option<IDataCollector>) -> windows_core::Result<()>;
    fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataCollectorCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataCollectorCollection_Vtbl
    where
        Identity: IDataCollectorCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    collector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorCollection_Impl::Add(this, windows_core::from_raw_borrowed(&collector)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorCollection_Impl::Remove(this, core::mem::transmute(&collector)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorCollection_Impl::AddRange(this, windows_core::from_raw_borrowed(&collectors)).into()
        }
        unsafe extern "system" fn CreateDataCollectorFromXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: core::mem::MaybeUninit<windows_core::BSTR>, pvalidation: *mut *mut core::ffi::c_void, pcollector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorCollection_Impl::CreateDataCollectorFromXml(this, core::mem::transmute(&bstrxml), core::mem::transmute_copy(&pvalidation), core::mem::transmute_copy(&pcollector)).into()
        }
        unsafe extern "system" fn CreateDataCollector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DataCollectorType, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorCollection_Impl::CreateDataCollector(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    collector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Identity, OFFSET>,
            CreateDataCollector: CreateDataCollector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection>;
    fn Duration(&self) -> windows_core::Result<u32>;
    fn SetDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Keywords(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RootPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRootPath(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Segment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSegment(&self, segment: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SegmentMaxDuration(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn SegmentMaxSize(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxSize(&self, size: u32) -> windows_core::Result<()>;
    fn SerialNumber(&self) -> windows_core::Result<u32>;
    fn SetSerialNumber(&self, index: u32) -> windows_core::Result<()>;
    fn Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<DataCollectorSetStatus>;
    fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectory(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Schedules(&self) -> windows_core::Result<IScheduleCollection>;
    fn SchedulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSchedulesEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Security(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurity(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopOnCompletion(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStopOnCompletion(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DataManager(&self) -> windows_core::Result<IDataManager>;
    fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Start(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Stop(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataCollectorSet {}
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataCollectorSet_Vtbl
    where
        Identity: IDataCollectorSet_Impl,
    {
        unsafe extern "system" fn DataCollectors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::DataCollectors(this) {
                Ok(ok__) => {
                    collectors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Duration(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetDuration(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Description(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn DescriptionUnresolved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, descr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::DescriptionUnresolved(this) {
                Ok(ok__) => {
                    descr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::DisplayName(this) {
                Ok(ok__) => {
                    displayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
        }
        unsafe extern "system" fn DisplayNameUnresolved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::DisplayNameUnresolved(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Keywords(this) {
                Ok(ok__) => {
                    keywords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetKeywords(this, core::mem::transmute_copy(&keywords)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::LatestOutputLocation(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::OutputLocation(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::RootPath(this) {
                Ok(ok__) => {
                    folder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetRootPath(this, core::mem::transmute(&folder)).into()
        }
        unsafe extern "system" fn Segment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Segment(this) {
                Ok(ok__) => {
                    segment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSegment(this, core::mem::transmute_copy(&segment)).into()
        }
        unsafe extern "system" fn SegmentMaxDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SegmentMaxDuration(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSegmentMaxDuration(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn SegmentMaxSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SegmentMaxSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSegmentMaxSize(this, core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SerialNumber(this) {
                Ok(ok__) => {
                    index.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSerialNumber(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Server<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Server(this) {
                Ok(ok__) => {
                    server.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DataCollectorSetStatus) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Status(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subdirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Subdirectory(this) {
                Ok(ok__) => {
                    folder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSubdirectory(this, core::mem::transmute(&folder)).into()
        }
        unsafe extern "system" fn SubdirectoryFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SubdirectoryFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSubdirectoryFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SubdirectoryFormatPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SubdirectoryFormatPattern(this) {
                Ok(ok__) => {
                    pattern.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormatPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSubdirectoryFormatPattern(this, core::mem::transmute(&pattern)).into()
        }
        unsafe extern "system" fn Task<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Task(this) {
                Ok(ok__) => {
                    task.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetTask(this, core::mem::transmute(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::TaskRunAsSelf(this) {
                Ok(ok__) => {
                    runasself.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::TaskArguments(this) {
                Ok(ok__) => {
                    task.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::TaskUserTextArguments(this) {
                Ok(ok__) => {
                    usertext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&usertext)).into()
        }
        unsafe extern "system" fn Schedules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschedules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Schedules(this) {
                Ok(ok__) => {
                    ppschedules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchedulesEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SchedulesEnabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchedulesEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSchedulesEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn UserAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::UserAccount(this) {
                Ok(ok__) => {
                    user.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Xml(this) {
                Ok(ok__) => {
                    xml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsecurity: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Security(this) {
                Ok(ok__) => {
                    pbstrsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsecurity: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetSecurity(this, core::mem::transmute(&bstrsecurity)).into()
        }
        unsafe extern "system" fn StopOnCompletion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::StopOnCompletion(this) {
                Ok(ok__) => {
                    stop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopOnCompletion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetStopOnCompletion(this, core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn DataManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datamanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::DataManager(this) {
                Ok(ok__) => {
                    datamanager.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetCredentials(this, core::mem::transmute(&user), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, server: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::Query(this, core::mem::transmute(&name), core::mem::transmute(&server)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, server: core::mem::MaybeUninit<windows_core::BSTR>, mode: CommitMode, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::Commit(this, core::mem::transmute(&name), core::mem::transmute(&server), core::mem::transmute_copy(&mode)) {
                Ok(ok__) => {
                    validation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::Start(this, core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::Stop(this, core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn SetXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: core::mem::MaybeUninit<windows_core::BSTR>, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::SetXml(this, core::mem::transmute(&xml)) {
                Ok(ok__) => {
                    validation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSet_Impl::SetValue(this, core::mem::transmute(&key), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSet_Impl::GetValue(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectors: DataCollectors::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            DescriptionUnresolved: DescriptionUnresolved::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            DisplayNameUnresolved: DisplayNameUnresolved::<Identity, OFFSET>,
            Keywords: Keywords::<Identity, OFFSET>,
            SetKeywords: SetKeywords::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            RootPath: RootPath::<Identity, OFFSET>,
            SetRootPath: SetRootPath::<Identity, OFFSET>,
            Segment: Segment::<Identity, OFFSET>,
            SetSegment: SetSegment::<Identity, OFFSET>,
            SegmentMaxDuration: SegmentMaxDuration::<Identity, OFFSET>,
            SetSegmentMaxDuration: SetSegmentMaxDuration::<Identity, OFFSET>,
            SegmentMaxSize: SegmentMaxSize::<Identity, OFFSET>,
            SetSegmentMaxSize: SetSegmentMaxSize::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            SetSerialNumber: SetSerialNumber::<Identity, OFFSET>,
            Server: Server::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            Subdirectory: Subdirectory::<Identity, OFFSET>,
            SetSubdirectory: SetSubdirectory::<Identity, OFFSET>,
            SubdirectoryFormat: SubdirectoryFormat::<Identity, OFFSET>,
            SetSubdirectoryFormat: SetSubdirectoryFormat::<Identity, OFFSET>,
            SubdirectoryFormatPattern: SubdirectoryFormatPattern::<Identity, OFFSET>,
            SetSubdirectoryFormatPattern: SetSubdirectoryFormatPattern::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            Schedules: Schedules::<Identity, OFFSET>,
            SchedulesEnabled: SchedulesEnabled::<Identity, OFFSET>,
            SetSchedulesEnabled: SetSchedulesEnabled::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            StopOnCompletion: StopOnCompletion::<Identity, OFFSET>,
            SetStopOnCompletion: SetStopOnCompletion::<Identity, OFFSET>,
            DataManager: DataManager::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorSetCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IDataCollectorSet>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, set: Option<&IDataCollectorSet>) -> windows_core::Result<()>;
    fn Remove(&self, set: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, sets: Option<&IDataCollectorSetCollection>) -> windows_core::Result<()>;
    fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataCollectorSetCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSetCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataCollectorSetCollection_Vtbl
    where
        Identity: IDataCollectorSetCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSetCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, set: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSetCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    set.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataCollectorSetCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSetCollection_Impl::Add(this, windows_core::from_raw_borrowed(&set)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSetCollection_Impl::Remove(this, core::mem::transmute(&set)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSetCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sets: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSetCollection_Impl::AddRange(this, windows_core::from_raw_borrowed(&sets)).into()
        }
        unsafe extern "system" fn GetDataCollectorSets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: core::mem::MaybeUninit<windows_core::BSTR>, filter: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataCollectorSetCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataCollectorSetCollection_Impl::GetDataCollectorSets(this, core::mem::transmute(&server), core::mem::transmute(&filter)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckBeforeRunning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCheckBeforeRunning(&self, fcheck: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MinFreeDisk(&self) -> windows_core::Result<u32>;
    fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::Result<()>;
    fn MaxSize(&self) -> windows_core::Result<u32>;
    fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::Result<()>;
    fn MaxFolderCount(&self) -> windows_core::Result<u32>;
    fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::Result<()>;
    fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy>;
    fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()>;
    fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection>;
    fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRules(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataManager {}
#[cfg(feature = "Win32_System_Com")]
impl IDataManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataManager_Vtbl
    where
        Identity: IDataManager_Impl,
    {
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::Enabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn CheckBeforeRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcheck: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::CheckBeforeRunning(this) {
                Ok(ok__) => {
                    pfcheck.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBeforeRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcheck: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetCheckBeforeRunning(this, core::mem::transmute_copy(&fcheck)).into()
        }
        unsafe extern "system" fn MinFreeDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::MinFreeDisk(this) {
                Ok(ok__) => {
                    minfreedisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinFreeDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetMinFreeDisk(this, core::mem::transmute_copy(&minfreedisk)).into()
        }
        unsafe extern "system" fn MaxSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::MaxSize(this) {
                Ok(ok__) => {
                    pulmaxsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxsize: u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetMaxSize(this, core::mem::transmute_copy(&ulmaxsize)).into()
        }
        unsafe extern "system" fn MaxFolderCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxfoldercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::MaxFolderCount(this) {
                Ok(ok__) => {
                    pulmaxfoldercount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxFolderCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxfoldercount: u32) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetMaxFolderCount(this, core::mem::transmute_copy(&ulmaxfoldercount)).into()
        }
        unsafe extern "system" fn ResourcePolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::ResourcePolicy(this) {
                Ok(ok__) => {
                    ppolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourcePolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ResourcePolicy) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetResourcePolicy(this, core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn FolderActions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::FolderActions(this) {
                Ok(ok__) => {
                    actions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::ReportSchema(this) {
                Ok(ok__) => {
                    reportschema.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetReportSchema(this, core::mem::transmute(&reportschema)).into()
        }
        unsafe extern "system" fn ReportFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::ReportFileName(this) {
                Ok(ok__) => {
                    pbstrfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetReportFileName(this, core::mem::transmute(&pbstrfilename)).into()
        }
        unsafe extern "system" fn RuleTargetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::RuleTargetFileName(this) {
                Ok(ok__) => {
                    filename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleTargetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetRuleTargetFileName(this, core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn EventsFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::EventsFileName(this) {
                Ok(ok__) => {
                    pbstrfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetEventsFileName(this, core::mem::transmute(&pbstrfilename)).into()
        }
        unsafe extern "system" fn Rules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::Rules(this) {
                Ok(ok__) => {
                    pbstrxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::SetRules(this, core::mem::transmute(&bstrxml)).into()
        }
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: DataManagerSteps, bstrfolder: core::mem::MaybeUninit<windows_core::BSTR>, errors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataManager_Impl::Run(this, core::mem::transmute_copy(&steps), core::mem::transmute(&bstrfolder)) {
                Ok(ok__) => {
                    errors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extract<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cabfilename: core::mem::MaybeUninit<windows_core::BSTR>, destinationpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDataManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataManager_Impl::Extract(this, core::mem::transmute(&cabfilename), core::mem::transmute(&destinationpath)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            CheckBeforeRunning: CheckBeforeRunning::<Identity, OFFSET>,
            SetCheckBeforeRunning: SetCheckBeforeRunning::<Identity, OFFSET>,
            MinFreeDisk: MinFreeDisk::<Identity, OFFSET>,
            SetMinFreeDisk: SetMinFreeDisk::<Identity, OFFSET>,
            MaxSize: MaxSize::<Identity, OFFSET>,
            SetMaxSize: SetMaxSize::<Identity, OFFSET>,
            MaxFolderCount: MaxFolderCount::<Identity, OFFSET>,
            SetMaxFolderCount: SetMaxFolderCount::<Identity, OFFSET>,
            ResourcePolicy: ResourcePolicy::<Identity, OFFSET>,
            SetResourcePolicy: SetResourcePolicy::<Identity, OFFSET>,
            FolderActions: FolderActions::<Identity, OFFSET>,
            ReportSchema: ReportSchema::<Identity, OFFSET>,
            SetReportSchema: SetReportSchema::<Identity, OFFSET>,
            ReportFileName: ReportFileName::<Identity, OFFSET>,
            SetReportFileName: SetReportFileName::<Identity, OFFSET>,
            RuleTargetFileName: RuleTargetFileName::<Identity, OFFSET>,
            SetRuleTargetFileName: SetRuleTargetFileName::<Identity, OFFSET>,
            EventsFileName: EventsFileName::<Identity, OFFSET>,
            SetEventsFileName: SetEventsFileName::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            SetRules: SetRules::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFolderAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Age(&self) -> windows_core::Result<u32>;
    fn SetAge(&self, ulage: u32) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn SetSize(&self, ulage: u32) -> windows_core::Result<()>;
    fn Actions(&self) -> windows_core::Result<FolderActionSteps>;
    fn SetActions(&self, steps: FolderActionSteps) -> windows_core::Result<()>;
    fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFolderAction {}
#[cfg(feature = "Win32_System_Com")]
impl IFolderAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFolderAction_Vtbl
    where
        Identity: IFolderAction_Impl,
    {
        unsafe extern "system" fn Age<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderAction_Impl::Age(this) {
                Ok(ok__) => {
                    pulage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderAction_Impl::SetAge(this, core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderAction_Impl::Size(this) {
                Ok(ok__) => {
                    pulage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderAction_Impl::SetSize(this, core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Actions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: *mut FolderActionSteps) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderAction_Impl::Actions(this) {
                Ok(ok__) => {
                    steps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: FolderActionSteps) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderAction_Impl::SetActions(this, core::mem::transmute_copy(&steps)).into()
        }
        unsafe extern "system" fn SendCabTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderAction_Impl::SendCabTo(this) {
                Ok(ok__) => {
                    pbstrdestination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendCabTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFolderAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderAction_Impl::SetSendCabTo(this, core::mem::transmute(&bstrdestination)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Age: Age::<Identity, OFFSET>,
            SetAge: SetAge::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            SetActions: SetActions::<Identity, OFFSET>,
            SendCabTo: SendCabTo::<Identity, OFFSET>,
            SetSendCabTo: SetSendCabTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFolderActionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IFolderAction>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, action: Option<&IFolderAction>) -> windows_core::Result<()>;
    fn Remove(&self, index: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, actions: Option<&IFolderActionCollection>) -> windows_core::Result<()>;
    fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFolderActionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IFolderActionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFolderActionCollection_Vtbl
    where
        Identity: IFolderActionCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderActionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderActionCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    action.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#enum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderActionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    r#enum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderActionCollection_Impl::Add(this, windows_core::from_raw_borrowed(&action)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderActionCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFolderActionCollection_Impl::AddRange(this, windows_core::from_raw_borrowed(&actions)).into()
        }
        unsafe extern "system" fn CreateFolderAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFolderActionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderActionCollection_Impl::CreateFolderAction(this) {
                Ok(ok__) => {
                    folderaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateFolderAction: CreateFolderAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderActionCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ILogFileItem_Impl: Sized {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ILogFileItem {}
impl ILogFileItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILogFileItem_Vtbl
    where
        Identity: ILogFileItem_Impl,
    {
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILogFileItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILogFileItem_Impl::Path(this) {
                Ok(ok__) => {
                    pstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Path: Path::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogFileItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILogFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<DILogFileItem>;
    fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DILogFileItem>;
    fn Remove(&self, index: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ILogFiles {}
#[cfg(feature = "Win32_System_Com")]
impl ILogFiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILogFiles_Vtbl
    where
        Identity: ILogFiles_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plong: *mut i32) -> windows_core::HRESULT
        where
            Identity: ILogFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILogFiles_Impl::Count(this) {
                Ok(ok__) => {
                    plong.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILogFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILogFiles_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppiunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILogFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILogFiles_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathname: core::mem::MaybeUninit<windows_core::BSTR>, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILogFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILogFiles_Impl::Add(this, core::mem::transmute(&pathname)) {
                Ok(ok__) => {
                    ppi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ILogFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILogFiles_Impl::Remove(this, core::mem::transmute(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogFiles as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPerformanceCounterDataCollector_Impl: Sized + IDataCollector_Impl {
    fn DataSourceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PerformanceCounters(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn LogFileFormat(&self) -> windows_core::Result<FileFormat>;
    fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn SegmentMaxRecords(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPerformanceCounterDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl IPerformanceCounterDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPerformanceCounterDataCollector_Vtbl
    where
        Identity: IPerformanceCounterDataCollector_Impl,
    {
        unsafe extern "system" fn DataSourceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerformanceCounterDataCollector_Impl::DataSourceName(this) {
                Ok(ok__) => {
                    dsn.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerformanceCounterDataCollector_Impl::SetDataSourceName(this, core::mem::transmute(&dsn)).into()
        }
        unsafe extern "system" fn PerformanceCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerformanceCounterDataCollector_Impl::PerformanceCounters(this) {
                Ok(ok__) => {
                    counters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerformanceCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerformanceCounterDataCollector_Impl::SetPerformanceCounters(this, core::mem::transmute_copy(&counters)).into()
        }
        unsafe extern "system" fn LogFileFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut FileFormat) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerformanceCounterDataCollector_Impl::LogFileFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFileFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: FileFormat) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerformanceCounterDataCollector_Impl::SetLogFileFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SampleInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerformanceCounterDataCollector_Impl::SampleInterval(this) {
                Ok(ok__) => {
                    interval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerformanceCounterDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn SegmentMaxRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerformanceCounterDataCollector_Impl::SegmentMaxRecords(this) {
                Ok(ok__) => {
                    records.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: u32) -> windows_core::HRESULT
        where
            Identity: IPerformanceCounterDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerformanceCounterDataCollector_Impl::SetSegmentMaxRecords(this, core::mem::transmute_copy(&records)).into()
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            DataSourceName: DataSourceName::<Identity, OFFSET>,
            SetDataSourceName: SetDataSourceName::<Identity, OFFSET>,
            PerformanceCounters: PerformanceCounters::<Identity, OFFSET>,
            SetPerformanceCounters: SetPerformanceCounters::<Identity, OFFSET>,
            LogFileFormat: LogFileFormat::<Identity, OFFSET>,
            SetLogFileFormat: SetLogFileFormat::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            SegmentMaxRecords: SegmentMaxRecords::<Identity, OFFSET>,
            SetSegmentMaxRecords: SetSegmentMaxRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchedule_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartDate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetStartDate(&self, start: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn EndDate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetEndDate(&self, end: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetStartTime(&self, start: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Days(&self) -> windows_core::Result<WeekDays>;
    fn SetDays(&self, days: WeekDays) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchedule {}
#[cfg(feature = "Win32_System_Com")]
impl ISchedule_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISchedule_Vtbl
    where
        Identity: ISchedule_Impl,
    {
        unsafe extern "system" fn StartDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchedule_Impl::StartDate(this) {
                Ok(ok__) => {
                    start.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchedule_Impl::SetStartDate(this, core::mem::transmute(&start)).into()
        }
        unsafe extern "system" fn EndDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchedule_Impl::EndDate(this) {
                Ok(ok__) => {
                    end.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchedule_Impl::SetEndDate(this, core::mem::transmute(&end)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchedule_Impl::StartTime(this) {
                Ok(ok__) => {
                    start.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchedule_Impl::SetStartTime(this, core::mem::transmute(&start)).into()
        }
        unsafe extern "system" fn Days<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: *mut WeekDays) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchedule_Impl::Days(this) {
                Ok(ok__) => {
                    days.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDays<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: WeekDays) -> windows_core::HRESULT
        where
            Identity: ISchedule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchedule_Impl::SetDays(this, core::mem::transmute_copy(&days)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartDate: StartDate::<Identity, OFFSET>,
            SetStartDate: SetStartDate::<Identity, OFFSET>,
            EndDate: EndDate::<Identity, OFFSET>,
            SetEndDate: SetEndDate::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            Days: Days::<Identity, OFFSET>,
            SetDays: SetDays::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchedule as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScheduleCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<ISchedule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pschedule: Option<&ISchedule>) -> windows_core::Result<()>;
    fn Remove(&self, vschedule: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, pschedules: Option<&IScheduleCollection>) -> windows_core::Result<()>;
    fn CreateSchedule(&self) -> windows_core::Result<ISchedule>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IScheduleCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IScheduleCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IScheduleCollection_Vtbl
    where
        Identity: IScheduleCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduleCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppschedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduleCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppschedule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduleCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedule: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduleCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pschedule)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vschedule: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduleCollection_Impl::Remove(this, core::mem::transmute(&vschedule)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduleCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedules: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScheduleCollection_Impl::AddRange(this, windows_core::from_raw_borrowed(&pschedules)).into()
        }
        unsafe extern "system" fn CreateSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, schedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScheduleCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScheduleCollection_Impl::CreateSchedule(this) {
                Ok(ok__) => {
                    schedule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateSchedule: CreateSchedule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduleCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor_Impl: Sized {
    fn Appearance(&self) -> windows_core::Result<i32>;
    fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn SetBackColor(&self, color: u32) -> windows_core::Result<()>;
    fn BorderStyle(&self) -> windows_core::Result<i32>;
    fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()>;
    fn ForeColor(&self) -> windows_core::Result<u32>;
    fn SetForeColor(&self, color: u32) -> windows_core::Result<()>;
    fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&self, pfont: Option<&super::Ole::IFontDisp>) -> windows_core::Result<()>;
    fn Counters(&self) -> windows_core::Result<ICounters>;
    fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MaximumScale(&self) -> windows_core::Result<i32>;
    fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MinimumScale(&self) -> windows_core::Result<i32>;
    fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()>;
    fn UpdateInterval(&self) -> windows_core::Result<f32>;
    fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()>;
    fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CollectSample(&self) -> windows_core::Result<()>;
    fn UpdateGraph(&self) -> windows_core::Result<()>;
    fn BrowseCounters(&self) -> windows_core::Result<()>;
    fn DisplayProperties(&self) -> windows_core::Result<()>;
    fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem>;
    fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem>;
    fn DeleteCounter(&self, pctr: Option<&ICounterItem>) -> windows_core::Result<()>;
    fn BackColorCtl(&self) -> windows_core::Result<u32>;
    fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()>;
    fn LogViewStart(&self) -> windows_core::Result<f64>;
    fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()>;
    fn LogViewStop(&self) -> windows_core::Result<f64>;
    fn GridColor(&self) -> windows_core::Result<u32>;
    fn SetGridColor(&self, color: u32) -> windows_core::Result<()>;
    fn TimeBarColor(&self) -> windows_core::Result<u32>;
    fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()>;
    fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paste(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()>;
    fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()>;
    fn DisplayFilter(&self) -> windows_core::Result<i32>;
    fn LogFiles(&self) -> windows_core::Result<ILogFiles>;
    fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()>;
    fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ISystemMonitor {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemMonitor_Vtbl
    where
        Identity: ISystemMonitor_Impl,
    {
        unsafe extern "system" fn Appearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::Appearance(this) {
                Ok(ok__) => {
                    iappearance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetAppearance(this, core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::BackColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetBackColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::BorderStyle(this) {
                Ok(ok__) => {
                    iborderstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetBorderStyle(this, core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ForeColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetForeColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::Font(this) {
                Ok(ok__) => {
                    ppfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::putref_Font(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicounters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::Counters(this) {
                Ok(ok__) => {
                    ppicounters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowVerticalGrid(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowVerticalGrid(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowHorizontalGrid(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowHorizontalGrid(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowLegend(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowLegend(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowScaleLabels(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowScaleLabels(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowValueBar(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowValueBar(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetMaximumScale(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::MaximumScale(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetMinimumScale(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::MinimumScale(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: f32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetUpdateInterval(this, core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalue: *mut f32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::UpdateInterval(this) {
                Ok(ok__) => {
                    pfvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetDisplayType(this, core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::DisplayType(this) {
                Ok(ok__) => {
                    pedisplaytype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetManualUpdate(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ManualUpdate(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetGraphTitle(this, core::mem::transmute(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::GraphTitle(this) {
                Ok(ok__) => {
                    pbstitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetYAxisLabel(this, core::mem::transmute(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::YAxisLabel(this) {
                Ok(ok__) => {
                    pbstitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::CollectSample(this).into()
        }
        unsafe extern "system" fn UpdateGraph<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::UpdateGraph(this).into()
        }
        unsafe extern "system" fn BrowseCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::BrowseCounters(this).into()
        }
        unsafe extern "system" fn DisplayProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::DisplayProperties(this).into()
        }
        unsafe extern "system" fn Counter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::Counter(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    ppicounter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bspath: core::mem::MaybeUninit<windows_core::BSTR>, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::AddCounter(this, core::mem::transmute(&bspath)) {
                Ok(ok__) => {
                    ppicounter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctr: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::DeleteCounter(this, windows_core::from_raw_borrowed(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::BackColorCtl(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetBackColorCtl(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetLogFileName(this, core::mem::transmute(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::LogFileName(this) {
                Ok(ok__) => {
                    bsfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetLogViewStart(this, core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::LogViewStart(this) {
                Ok(ok__) => {
                    starttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetLogViewStop(this, core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::LogViewStop(this) {
                Ok(ok__) => {
                    stoptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::GridColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetGridColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::TimeBarColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetTimeBarColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::Highlight(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetHighlight(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ShowToolbar(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetShowToolbar(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::Paste(this).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::Copy(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::Reset(this).into()
        }
        unsafe extern "system" fn SetReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetReadOnly(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetReportValueType(this, core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::ReportValueType(this) {
                Ok(ok__) => {
                    pereportvaluetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetMonitorDuplicateInstances(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::MonitorDuplicateInstances(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetDisplayFilter(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::DisplayFilter(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilogfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::LogFiles(this) {
                Ok(ok__) => {
                    ppilogfiles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetDataSourceType(this, core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::DataSourceType(this) {
                Ok(ok__) => {
                    pedatasourcetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetSqlDsnName(this, core::mem::transmute(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::SqlDsnName(this) {
                Ok(ok__) => {
                    bssqldsnname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor_Impl::SetSqlLogSetName(this, core::mem::transmute(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor_Impl::SqlLogSetName(this) {
                Ok(ok__) => {
                    bssqllogsetname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, OFFSET>,
            SetAppearance: SetAppearance::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BorderStyle: BorderStyle::<Identity, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, OFFSET>,
            ForeColor: ForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            Font: Font::<Identity, OFFSET>,
            putref_Font: putref_Font::<Identity, OFFSET>,
            Counters: Counters::<Identity, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, OFFSET>,
            ShowLegend: ShowLegend::<Identity, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, OFFSET>,
            MaximumScale: MaximumScale::<Identity, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, OFFSET>,
            MinimumScale: MinimumScale::<Identity, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            DisplayType: DisplayType::<Identity, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, OFFSET>,
            GraphTitle: GraphTitle::<Identity, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, OFFSET>,
            CollectSample: CollectSample::<Identity, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, OFFSET>,
            Counter: Counter::<Identity, OFFSET>,
            AddCounter: AddCounter::<Identity, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, OFFSET>,
            LogFileName: LogFileName::<Identity, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, OFFSET>,
            LogViewStart: LogViewStart::<Identity, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, OFFSET>,
            LogViewStop: LogViewStop::<Identity, OFFSET>,
            GridColor: GridColor::<Identity, OFFSET>,
            SetGridColor: SetGridColor::<Identity, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, OFFSET>,
            Highlight: Highlight::<Identity, OFFSET>,
            SetHighlight: SetHighlight::<Identity, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, OFFSET>,
            ReportValueType: ReportValueType::<Identity, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, OFFSET>,
            LogFiles: LogFiles::<Identity, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, OFFSET>,
            DataSourceType: DataSourceType::<Identity, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor2_Impl: Sized + ISystemMonitor_Impl {
    fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()>;
    fn DataPointCount(&self) -> windows_core::Result<i32>;
    fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>;
    fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>;
    fn ClearData(&self) -> windows_core::Result<()>;
    fn LogSourceStartTime(&self) -> windows_core::Result<f64>;
    fn LogSourceStopTime(&self) -> windows_core::Result<f64>;
    fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()>;
    fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()>;
    fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>;
    fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ISystemMonitor2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemMonitor2_Vtbl
    where
        Identity: ISystemMonitor2_Impl,
    {
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetEnableDigitGrouping(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::EnableDigitGrouping(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetEnableToolTips(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::EnableToolTips(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetShowTimeAxisLabels(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::ShowTimeAxisLabels(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetChartScroll(this, core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbscroll: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::ChartScroll(this) {
                Ok(ok__) => {
                    pbscroll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewcount: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetDataPointCount(this, core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidatapointcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::DataPointCount(this) {
                Ok(ok__) => {
                    pidatapointcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::ScaleToFit(this, core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, esysmonfiletype: SysmonFileType) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SaveAs(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::Relog(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype), core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::ClearData(this).into()
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::LogSourceStartTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemMonitor2_Impl::LogSourceStopTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64, stoptime: f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::SetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::GetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::BatchingLock(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsettingfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemMonitor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitor2_Impl::LoadSettings(this, core::mem::transmute(&bstrsettingfilename)).into()
        }
        Self {
            base__: ISystemMonitor_Vtbl::new::<Identity, OFFSET>(),
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, OFFSET>,
            ChartScroll: ChartScroll::<Identity, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, OFFSET>,
            DataPointCount: DataPointCount::<Identity, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, OFFSET>,
            SaveAs: SaveAs::<Identity, OFFSET>,
            Relog: Relog::<Identity, OFFSET>,
            ClearData: ClearData::<Identity, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, OFFSET>,
            BatchingLock: BatchingLock::<Identity, OFFSET>,
            LoadSettings: LoadSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitor2 as windows_core::Interface>::IID || iid == &<ISystemMonitor as windows_core::Interface>::IID
    }
}
pub trait ISystemMonitorEvents_Impl: Sized {
    fn OnCounterSelected(&self, index: i32);
    fn OnCounterAdded(&self, index: i32);
    fn OnCounterDeleted(&self, index: i32);
    fn OnSampleCollected(&self);
    fn OnDblClick(&self, index: i32);
}
impl windows_core::RuntimeName for ISystemMonitorEvents {}
impl ISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemMonitorEvents_Vtbl
    where
        Identity: ISystemMonitorEvents_Impl,
    {
        unsafe extern "system" fn OnCounterSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32)
        where
            Identity: ISystemMonitorEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitorEvents_Impl::OnCounterSelected(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32)
        where
            Identity: ISystemMonitorEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitorEvents_Impl::OnCounterAdded(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32)
        where
            Identity: ISystemMonitorEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitorEvents_Impl::OnCounterDeleted(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnSampleCollected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ISystemMonitorEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitorEvents_Impl::OnSampleCollected(this)
        }
        unsafe extern "system" fn OnDblClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32)
        where
            Identity: ISystemMonitorEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemMonitorEvents_Impl::OnDblClick(this, core::mem::transmute_copy(&index))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCounterSelected: OnCounterSelected::<Identity, OFFSET>,
            OnCounterAdded: OnCounterAdded::<Identity, OFFSET>,
            OnCounterDeleted: OnCounterDeleted::<Identity, OFFSET>,
            OnSampleCollected: OnSampleCollected::<Identity, OFFSET>,
            OnDblClick: OnDblClick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitorEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataCollector_Impl: Sized + IDataCollector_Impl {
    fn BufferSize(&self) -> windows_core::Result<u32>;
    fn SetBufferSize(&self, size: u32) -> windows_core::Result<()>;
    fn BuffersLost(&self) -> windows_core::Result<u32>;
    fn SetBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn BuffersWritten(&self) -> windows_core::Result<u32>;
    fn SetBuffersWritten(&self, buffers: u32) -> windows_core::Result<()>;
    fn ClockType(&self) -> windows_core::Result<ClockType>;
    fn SetClockType(&self, clock: ClockType) -> windows_core::Result<()>;
    fn EventsLost(&self) -> windows_core::Result<u32>;
    fn SetEventsLost(&self, events: u32) -> windows_core::Result<()>;
    fn ExtendedModes(&self) -> windows_core::Result<u32>;
    fn SetExtendedModes(&self, mode: u32) -> windows_core::Result<()>;
    fn FlushTimer(&self) -> windows_core::Result<u32>;
    fn SetFlushTimer(&self, seconds: u32) -> windows_core::Result<()>;
    fn FreeBuffers(&self) -> windows_core::Result<u32>;
    fn SetFreeBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn IsKernelTrace(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MaximumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn MinimumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn NumberOfBuffers(&self) -> windows_core::Result<u32>;
    fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn PreallocateFile(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPreallocateFile(&self, allocate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProcessMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetProcessMode(&self, process: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RealTimeBuffersLost(&self) -> windows_core::Result<u32>;
    fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn SessionId(&self) -> windows_core::Result<u64>;
    fn SetSessionId(&self, id: u64) -> windows_core::Result<()>;
    fn SessionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSessionName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SessionThreadId(&self) -> windows_core::Result<u32>;
    fn SetSessionThreadId(&self, tid: u32) -> windows_core::Result<()>;
    fn StreamMode(&self) -> windows_core::Result<StreamMode>;
    fn SetStreamMode(&self, mode: StreamMode) -> windows_core::Result<()>;
    fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITraceDataCollector {}
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceDataCollector_Vtbl
    where
        Identity: ITraceDataCollector_Impl,
    {
        unsafe extern "system" fn BufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::BufferSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetBufferSize(this, core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn BuffersLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::BuffersLost(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn BuffersWritten<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::BuffersWritten(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersWritten<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetBuffersWritten(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn ClockType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: *mut ClockType) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::ClockType(this) {
                Ok(ok__) => {
                    clock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: ClockType) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetClockType(this, core::mem::transmute_copy(&clock)).into()
        }
        unsafe extern "system" fn EventsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::EventsLost(this) {
                Ok(ok__) => {
                    events.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetEventsLost(this, core::mem::transmute_copy(&events)).into()
        }
        unsafe extern "system" fn ExtendedModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::ExtendedModes(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetExtendedModes(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn FlushTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::FlushTimer(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlushTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetFlushTimer(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn FreeBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::FreeBuffers(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetFreeBuffers(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::Guid(this) {
                Ok(ok__) => {
                    guid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn IsKernelTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, kernel: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::IsKernelTrace(this) {
                Ok(ok__) => {
                    kernel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::MaximumBuffers(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetMaximumBuffers(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn MinimumBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::MinimumBuffers(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetMinimumBuffers(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn NumberOfBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::NumberOfBuffers(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetNumberOfBuffers(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn PreallocateFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::PreallocateFile(this) {
                Ok(ok__) => {
                    allocate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreallocateFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetPreallocateFile(this, core::mem::transmute_copy(&allocate)).into()
        }
        unsafe extern "system" fn ProcessMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::ProcessMode(this) {
                Ok(ok__) => {
                    process.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetProcessMode(this, core::mem::transmute_copy(&process)).into()
        }
        unsafe extern "system" fn RealTimeBuffersLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::RealTimeBuffersLost(this) {
                Ok(ok__) => {
                    buffers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealTimeBuffersLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetRealTimeBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn SessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut u64) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::SessionId(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u64) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetSessionId(this, core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SessionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::SessionName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetSessionName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn SessionThreadId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::SessionThreadId(this) {
                Ok(ok__) => {
                    tid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionThreadId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetSessionThreadId(this, core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn StreamMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut StreamMode) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::StreamMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: StreamMode) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataCollector_Impl::SetStreamMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn TraceDataProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataCollector_Impl::TraceDataProviders(this) {
                Ok(ok__) => {
                    providers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            BufferSize: BufferSize::<Identity, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, OFFSET>,
            BuffersLost: BuffersLost::<Identity, OFFSET>,
            SetBuffersLost: SetBuffersLost::<Identity, OFFSET>,
            BuffersWritten: BuffersWritten::<Identity, OFFSET>,
            SetBuffersWritten: SetBuffersWritten::<Identity, OFFSET>,
            ClockType: ClockType::<Identity, OFFSET>,
            SetClockType: SetClockType::<Identity, OFFSET>,
            EventsLost: EventsLost::<Identity, OFFSET>,
            SetEventsLost: SetEventsLost::<Identity, OFFSET>,
            ExtendedModes: ExtendedModes::<Identity, OFFSET>,
            SetExtendedModes: SetExtendedModes::<Identity, OFFSET>,
            FlushTimer: FlushTimer::<Identity, OFFSET>,
            SetFlushTimer: SetFlushTimer::<Identity, OFFSET>,
            FreeBuffers: FreeBuffers::<Identity, OFFSET>,
            SetFreeBuffers: SetFreeBuffers::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            IsKernelTrace: IsKernelTrace::<Identity, OFFSET>,
            MaximumBuffers: MaximumBuffers::<Identity, OFFSET>,
            SetMaximumBuffers: SetMaximumBuffers::<Identity, OFFSET>,
            MinimumBuffers: MinimumBuffers::<Identity, OFFSET>,
            SetMinimumBuffers: SetMinimumBuffers::<Identity, OFFSET>,
            NumberOfBuffers: NumberOfBuffers::<Identity, OFFSET>,
            SetNumberOfBuffers: SetNumberOfBuffers::<Identity, OFFSET>,
            PreallocateFile: PreallocateFile::<Identity, OFFSET>,
            SetPreallocateFile: SetPreallocateFile::<Identity, OFFSET>,
            ProcessMode: ProcessMode::<Identity, OFFSET>,
            SetProcessMode: SetProcessMode::<Identity, OFFSET>,
            RealTimeBuffersLost: RealTimeBuffersLost::<Identity, OFFSET>,
            SetRealTimeBuffersLost: SetRealTimeBuffersLost::<Identity, OFFSET>,
            SessionId: SessionId::<Identity, OFFSET>,
            SetSessionId: SetSessionId::<Identity, OFFSET>,
            SessionName: SessionName::<Identity, OFFSET>,
            SetSessionName: SetSessionName::<Identity, OFFSET>,
            SessionThreadId: SessionThreadId::<Identity, OFFSET>,
            SetSessionThreadId: SetSessionThreadId::<Identity, OFFSET>,
            StreamMode: StreamMode::<Identity, OFFSET>,
            SetStreamMode: SetStreamMode::<Identity, OFFSET>,
            TraceDataProviders: TraceDataProviders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataProvider_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Level(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAny(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAll(&self) -> windows_core::Result<IValueMap>;
    fn Properties(&self) -> windows_core::Result<IValueMap>;
    fn FilterEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFilterEnabled(&self, filterenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FilterType(&self) -> windows_core::Result<u32>;
    fn SetFilterType(&self, ultype: u32) -> windows_core::Result<()>;
    fn FilterData(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Resolve(&self, pfrom: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITraceDataProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceDataProvider_Vtbl
    where
        Identity: ITraceDataProvider_Impl,
    {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::DisplayName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::Guid(this) {
                Ok(ok__) => {
                    guid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn Level<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::Level(this) {
                Ok(ok__) => {
                    pplevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAny<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::KeywordsAny(this) {
                Ok(ok__) => {
                    ppkeywords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::KeywordsAll(this) {
                Ok(ok__) => {
                    ppkeywords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::Properties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::FilterEnabled(this) {
                Ok(ok__) => {
                    filterenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetFilterEnabled(this, core::mem::transmute_copy(&filterenabled)).into()
        }
        unsafe extern "system" fn FilterType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultype: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::FilterType(this) {
                Ok(ok__) => {
                    pultype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultype: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetFilterType(this, core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn FilterData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::FilterData(this) {
                Ok(ok__) => {
                    ppdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetFilterData(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrserver: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::Query(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrserver)).into()
        }
        unsafe extern "system" fn Resolve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::Resolve(this, windows_core::from_raw_borrowed(&pfrom)).into()
        }
        unsafe extern "system" fn SetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProvider_Impl::SetSecurity(this, core::mem::transmute(&sddl)).into()
        }
        unsafe extern "system" fn GetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinfo: u32, sddl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::GetSecurity(this, core::mem::transmute_copy(&securityinfo)) {
                Ok(ok__) => {
                    sddl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredProcesses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProvider_Impl::GetRegisteredProcesses(this) {
                Ok(ok__) => {
                    processes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            Level: Level::<Identity, OFFSET>,
            KeywordsAny: KeywordsAny::<Identity, OFFSET>,
            KeywordsAll: KeywordsAll::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            FilterEnabled: FilterEnabled::<Identity, OFFSET>,
            SetFilterEnabled: SetFilterEnabled::<Identity, OFFSET>,
            FilterType: FilterType::<Identity, OFFSET>,
            SetFilterType: SetFilterType::<Identity, OFFSET>,
            FilterData: FilterData::<Identity, OFFSET>,
            SetFilterData: SetFilterData::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProvider as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataProviderCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<ITraceDataProvider>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pprovider: Option<&ITraceDataProvider>) -> windows_core::Result<()>;
    fn Remove(&self, vprovider: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, providers: Option<&ITraceDataProviderCollection>) -> windows_core::Result<()>;
    fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider>;
    fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITraceDataProviderCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProviderCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceDataProviderCollection_Vtbl
    where
        Identity: ITraceDataProviderCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProviderCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProviderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    ppprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProviderCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pprovider)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprovider: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::Remove(this, core::mem::transmute(&vprovider)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::AddRange(this, windows_core::from_raw_borrowed(&providers)).into()
        }
        unsafe extern "system" fn CreateTraceDataProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceDataProviderCollection_Impl::CreateTraceDataProvider(this) {
                Ok(ok__) => {
                    provider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTraceDataProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::GetTraceDataProviders(this, core::mem::transmute(&server)).into()
        }
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: core::mem::MaybeUninit<windows_core::BSTR>, pid: u32) -> windows_core::HRESULT
        where
            Identity: ITraceDataProviderCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceDataProviderCollection_Impl::GetTraceDataProvidersByProcess(this, core::mem::transmute(&server), core::mem::transmute_copy(&pid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateTraceDataProvider: CreateTraceDataProvider::<Identity, OFFSET>,
            GetTraceDataProviders: GetTraceDataProviders::<Identity, OFFSET>,
            GetTraceDataProvidersByProcess: GetTraceDataProvidersByProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProviderCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IValueMap_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<IValueMapItem>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
    fn Add(&self, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, map: Option<&IValueMap>) -> windows_core::Result<()>;
    fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IValueMap {}
#[cfg(feature = "Win32_System_Com")]
impl IValueMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IValueMap_Vtbl
    where
        Identity: IValueMap_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::get_Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::Description(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::Value(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::SetValue(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::ValueMapType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::Add(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::Remove(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, map: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMap_Impl::AddRange(this, windows_core::from_raw_borrowed(&map)).into()
        }
        unsafe extern "system" fn CreateValueMapItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValueMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMap_Impl::CreateValueMapItem(this) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMap as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IValueMapItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetKey(&self, key: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IValueMapItem {}
#[cfg(feature = "Win32_System_Com")]
impl IValueMapItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IValueMapItem_Vtbl
    where
        Identity: IValueMapItem_Impl,
    {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMapItem_Impl::Description(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMapItem_Impl::SetDescription(this, core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMapItem_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMapItem_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Key<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMapItem_Impl::Key(this) {
                Ok(ok__) => {
                    key.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMapItem_Impl::SetKey(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMapItem_Impl::Value(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMapItem_Impl::SetValue(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IValueMapItem_Impl::ValueMapType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT
        where
            Identity: IValueMapItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValueMapItem_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMapItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait _ICounterItemUnion_Impl: Sized {
    fn Value(&self) -> windows_core::Result<f64>;
    fn SetColor(&self, color: u32) -> windows_core::Result<()>;
    fn Color(&self) -> windows_core::Result<u32>;
    fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()>;
    fn LineStyle(&self) -> windows_core::Result<i32>;
    fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()>;
    fn ScaleFactor(&self) -> windows_core::Result<i32>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for _ICounterItemUnion {}
impl _ICounterItemUnion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _ICounterItemUnion_Vtbl
    where
        Identity: _ICounterItemUnion_Impl,
    {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblvalue: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Value(this) {
                Ok(ok__) => {
                    pdblvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Color(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwidth: i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetWidth(this, core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Width(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ilinestyle: i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetLineStyle(this, core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::LineStyle(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscale: i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetScaleFactor(this, core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::ScaleFactor(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Path(this) {
                Ok(ok__) => {
                    pstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, status: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::GetValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::GetStatistics(this, core::mem::transmute_copy(&max), core::mem::transmute_copy(&min), core::mem::transmute_copy(&avg), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetSelected(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Selected(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ICounterItemUnion_Impl::SetVisible(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::Visible(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: _ICounterItemUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ICounterItemUnion_Impl::GetDataAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iwhich)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Color: Color::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, OFFSET>,
            LineStyle: LineStyle::<Identity, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetSelected: SetSelected::<Identity, OFFSET>,
            Selected: Selected::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            GetDataAt: GetDataAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ICounterItemUnion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISystemMonitorUnion_Impl: Sized {
    fn Appearance(&self) -> windows_core::Result<i32>;
    fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn SetBackColor(&self, color: u32) -> windows_core::Result<()>;
    fn BorderStyle(&self) -> windows_core::Result<i32>;
    fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()>;
    fn ForeColor(&self) -> windows_core::Result<u32>;
    fn SetForeColor(&self, color: u32) -> windows_core::Result<()>;
    fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&self, pfont: Option<&super::Ole::IFontDisp>) -> windows_core::Result<()>;
    fn Counters(&self) -> windows_core::Result<ICounters>;
    fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MaximumScale(&self) -> windows_core::Result<i32>;
    fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MinimumScale(&self) -> windows_core::Result<i32>;
    fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()>;
    fn UpdateInterval(&self) -> windows_core::Result<f32>;
    fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()>;
    fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CollectSample(&self) -> windows_core::Result<()>;
    fn UpdateGraph(&self) -> windows_core::Result<()>;
    fn BrowseCounters(&self) -> windows_core::Result<()>;
    fn DisplayProperties(&self) -> windows_core::Result<()>;
    fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem>;
    fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem>;
    fn DeleteCounter(&self, pctr: Option<&ICounterItem>) -> windows_core::Result<()>;
    fn BackColorCtl(&self) -> windows_core::Result<u32>;
    fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()>;
    fn LogViewStart(&self) -> windows_core::Result<f64>;
    fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()>;
    fn LogViewStop(&self) -> windows_core::Result<f64>;
    fn GridColor(&self) -> windows_core::Result<u32>;
    fn SetGridColor(&self, color: u32) -> windows_core::Result<()>;
    fn TimeBarColor(&self) -> windows_core::Result<u32>;
    fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()>;
    fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paste(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()>;
    fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()>;
    fn DisplayFilter(&self) -> windows_core::Result<i32>;
    fn LogFiles(&self) -> windows_core::Result<ILogFiles>;
    fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()>;
    fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()>;
    fn DataPointCount(&self) -> windows_core::Result<i32>;
    fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>;
    fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>;
    fn ClearData(&self) -> windows_core::Result<()>;
    fn LogSourceStartTime(&self) -> windows_core::Result<f64>;
    fn LogSourceStopTime(&self) -> windows_core::Result<f64>;
    fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()>;
    fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()>;
    fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>;
    fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for _ISystemMonitorUnion {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISystemMonitorUnion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _ISystemMonitorUnion_Vtbl
    where
        Identity: _ISystemMonitorUnion_Impl,
    {
        unsafe extern "system" fn Appearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::Appearance(this) {
                Ok(ok__) => {
                    iappearance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetAppearance(this, core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::BackColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetBackColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::BorderStyle(this) {
                Ok(ok__) => {
                    iborderstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetBorderStyle(this, core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ForeColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetForeColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::Font(this) {
                Ok(ok__) => {
                    ppfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::putref_Font(this, windows_core::from_raw_borrowed(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicounters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::Counters(this) {
                Ok(ok__) => {
                    ppicounters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowVerticalGrid(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowVerticalGrid(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowHorizontalGrid(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowHorizontalGrid(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowLegend(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowLegend(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowScaleLabels(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowScaleLabels(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowValueBar(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowValueBar(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetMaximumScale(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::MaximumScale(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetMinimumScale(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::MinimumScale(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: f32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetUpdateInterval(this, core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalue: *mut f32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::UpdateInterval(this) {
                Ok(ok__) => {
                    pfvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetDisplayType(this, core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::DisplayType(this) {
                Ok(ok__) => {
                    pedisplaytype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetManualUpdate(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ManualUpdate(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetGraphTitle(this, core::mem::transmute(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::GraphTitle(this) {
                Ok(ok__) => {
                    pbstitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetYAxisLabel(this, core::mem::transmute(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::YAxisLabel(this) {
                Ok(ok__) => {
                    pbstitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::CollectSample(this).into()
        }
        unsafe extern "system" fn UpdateGraph<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::UpdateGraph(this).into()
        }
        unsafe extern "system" fn BrowseCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::BrowseCounters(this).into()
        }
        unsafe extern "system" fn DisplayProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::DisplayProperties(this).into()
        }
        unsafe extern "system" fn Counter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::Counter(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    ppicounter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bspath: core::mem::MaybeUninit<windows_core::BSTR>, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::AddCounter(this, core::mem::transmute(&bspath)) {
                Ok(ok__) => {
                    ppicounter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctr: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::DeleteCounter(this, windows_core::from_raw_borrowed(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::BackColorCtl(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetBackColorCtl(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetLogFileName(this, core::mem::transmute(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogFileName(this) {
                Ok(ok__) => {
                    bsfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetLogViewStart(this, core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogViewStart(this) {
                Ok(ok__) => {
                    starttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetLogViewStop(this, core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogViewStop(this) {
                Ok(ok__) => {
                    stoptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::GridColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetGridColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::TimeBarColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetTimeBarColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::Highlight(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetHighlight(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowToolbar(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowToolbar(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::Paste(this).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::Copy(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::Reset(this).into()
        }
        unsafe extern "system" fn SetReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetReadOnly(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetReportValueType(this, core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ReportValueType(this) {
                Ok(ok__) => {
                    pereportvaluetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetMonitorDuplicateInstances(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::MonitorDuplicateInstances(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetDisplayFilter(this, core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::DisplayFilter(this) {
                Ok(ok__) => {
                    pivalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilogfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogFiles(this) {
                Ok(ok__) => {
                    ppilogfiles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetDataSourceType(this, core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::DataSourceType(this) {
                Ok(ok__) => {
                    pedatasourcetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetSqlDsnName(this, core::mem::transmute(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::SqlDsnName(this) {
                Ok(ok__) => {
                    bssqldsnname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetSqlLogSetName(this, core::mem::transmute(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::SqlLogSetName(this) {
                Ok(ok__) => {
                    bssqllogsetname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetEnableDigitGrouping(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::EnableDigitGrouping(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetEnableToolTips(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::EnableToolTips(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetShowTimeAxisLabels(this, core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ShowTimeAxisLabels(this) {
                Ok(ok__) => {
                    pbstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetChartScroll(this, core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbscroll: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::ChartScroll(this) {
                Ok(ok__) => {
                    pbscroll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewcount: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetDataPointCount(this, core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidatapointcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::DataPointCount(this) {
                Ok(ok__) => {
                    pidatapointcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::ScaleToFit(this, core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, esysmonfiletype: SysmonFileType) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SaveAs(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::Relog(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype), core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::ClearData(this).into()
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogSourceStartTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match _ISystemMonitorUnion_Impl::LogSourceStopTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64, stoptime: f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::SetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::GetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::BatchingLock(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsettingfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: _ISystemMonitorUnion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISystemMonitorUnion_Impl::LoadSettings(this, core::mem::transmute(&bstrsettingfilename)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, OFFSET>,
            SetAppearance: SetAppearance::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BorderStyle: BorderStyle::<Identity, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, OFFSET>,
            ForeColor: ForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            Font: Font::<Identity, OFFSET>,
            putref_Font: putref_Font::<Identity, OFFSET>,
            Counters: Counters::<Identity, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, OFFSET>,
            ShowLegend: ShowLegend::<Identity, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, OFFSET>,
            MaximumScale: MaximumScale::<Identity, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, OFFSET>,
            MinimumScale: MinimumScale::<Identity, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            DisplayType: DisplayType::<Identity, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, OFFSET>,
            GraphTitle: GraphTitle::<Identity, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, OFFSET>,
            CollectSample: CollectSample::<Identity, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, OFFSET>,
            Counter: Counter::<Identity, OFFSET>,
            AddCounter: AddCounter::<Identity, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, OFFSET>,
            LogFileName: LogFileName::<Identity, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, OFFSET>,
            LogViewStart: LogViewStart::<Identity, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, OFFSET>,
            LogViewStop: LogViewStop::<Identity, OFFSET>,
            GridColor: GridColor::<Identity, OFFSET>,
            SetGridColor: SetGridColor::<Identity, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, OFFSET>,
            Highlight: Highlight::<Identity, OFFSET>,
            SetHighlight: SetHighlight::<Identity, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, OFFSET>,
            ReportValueType: ReportValueType::<Identity, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, OFFSET>,
            LogFiles: LogFiles::<Identity, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, OFFSET>,
            DataSourceType: DataSourceType::<Identity, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, OFFSET>,
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, OFFSET>,
            ChartScroll: ChartScroll::<Identity, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, OFFSET>,
            DataPointCount: DataPointCount::<Identity, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, OFFSET>,
            SaveAs: SaveAs::<Identity, OFFSET>,
            Relog: Relog::<Identity, OFFSET>,
            ClearData: ClearData::<Identity, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, OFFSET>,
            BatchingLock: BatchingLock::<Identity, OFFSET>,
            LoadSettings: LoadSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISystemMonitorUnion as windows_core::Interface>::IID
    }
}
