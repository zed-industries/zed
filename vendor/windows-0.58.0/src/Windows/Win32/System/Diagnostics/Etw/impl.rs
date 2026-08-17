pub trait ITraceEvent_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<ITraceEvent>;
    fn GetUserContext(&self) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetEventRecord(&self) -> windows_core::Result<*mut EVENT_RECORD>;
    fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> windows_core::Result<()>;
    fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> windows_core::Result<()>;
    fn SetProcessId(&self, processid: u32) -> windows_core::Result<()>;
    fn SetProcessorIndex(&self, processorindex: u32) -> windows_core::Result<()>;
    fn SetThreadId(&self, threadid: u32) -> windows_core::Result<()>;
    fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> windows_core::Result<()>;
    fn SetActivityId(&self, activityid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetTimeStamp(&self, timestamp: *const i64) -> windows_core::Result<()>;
    fn SetProviderId(&self, providerid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITraceEvent {}
impl ITraceEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceEvent_Vtbl
    where
        Identity: ITraceEvent_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceEvent_Impl::Clone(this) {
                Ok(ok__) => {
                    newevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usercontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceEvent_Impl::GetUserContext(this) {
                Ok(ok__) => {
                    usercontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceEvent_Impl::GetEventRecord(this) {
                Ok(ok__) => {
                    eventrecord.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, payload: *const u8, payloadsize: u32) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetPayload(this, core::mem::transmute_copy(&payload), core::mem::transmute_copy(&payloadsize)).into()
        }
        unsafe extern "system" fn SetEventDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetEventDescriptor(this, core::mem::transmute_copy(&eventdescriptor)).into()
        }
        unsafe extern "system" fn SetProcessId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: u32) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetProcessId(this, core::mem::transmute_copy(&processid)).into()
        }
        unsafe extern "system" fn SetProcessorIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorindex: u32) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetProcessorIndex(this, core::mem::transmute_copy(&processorindex)).into()
        }
        unsafe extern "system" fn SetThreadId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetThreadId(this, core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn SetThreadTimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, kerneltime: u32, usertime: u32) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetThreadTimes(this, core::mem::transmute_copy(&kerneltime), core::mem::transmute_copy(&usertime)).into()
        }
        unsafe extern "system" fn SetActivityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetActivityId(this, core::mem::transmute_copy(&activityid)).into()
        }
        unsafe extern "system" fn SetTimeStamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *const i64) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetTimeStamp(this, core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn SetProviderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITraceEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEvent_Impl::SetProviderId(this, core::mem::transmute_copy(&providerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            GetUserContext: GetUserContext::<Identity, OFFSET>,
            GetEventRecord: GetEventRecord::<Identity, OFFSET>,
            SetPayload: SetPayload::<Identity, OFFSET>,
            SetEventDescriptor: SetEventDescriptor::<Identity, OFFSET>,
            SetProcessId: SetProcessId::<Identity, OFFSET>,
            SetProcessorIndex: SetProcessorIndex::<Identity, OFFSET>,
            SetThreadId: SetThreadId::<Identity, OFFSET>,
            SetThreadTimes: SetThreadTimes::<Identity, OFFSET>,
            SetActivityId: SetActivityId::<Identity, OFFSET>,
            SetTimeStamp: SetTimeStamp::<Identity, OFFSET>,
            SetProviderId: SetProviderId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceEvent as windows_core::Interface>::IID
    }
}
pub trait ITraceEventCallback_Impl: Sized {
    fn OnBeginProcessTrace(&self, headerevent: Option<&ITraceEvent>, relogger: Option<&ITraceRelogger>) -> windows_core::Result<()>;
    fn OnFinalizeProcessTrace(&self, relogger: Option<&ITraceRelogger>) -> windows_core::Result<()>;
    fn OnEvent(&self, event: Option<&ITraceEvent>, relogger: Option<&ITraceRelogger>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITraceEventCallback {}
impl ITraceEventCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceEventCallback_Vtbl
    where
        Identity: ITraceEventCallback_Impl,
    {
        unsafe extern "system" fn OnBeginProcessTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, headerevent: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEventCallback_Impl::OnBeginProcessTrace(this, windows_core::from_raw_borrowed(&headerevent), windows_core::from_raw_borrowed(&relogger)).into()
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEventCallback_Impl::OnFinalizeProcessTrace(this, windows_core::from_raw_borrowed(&relogger)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, relogger: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceEventCallback_Impl::OnEvent(this, windows_core::from_raw_borrowed(&event), windows_core::from_raw_borrowed(&relogger)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginProcessTrace: OnBeginProcessTrace::<Identity, OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceEventCallback as windows_core::Interface>::IID
    }
}
pub trait ITraceRelogger_Impl: Sized {
    fn AddLogfileTraceStream(&self, logfilename: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE>;
    fn AddRealtimeTraceStream(&self, loggername: &windows_core::BSTR, usercontext: *const core::ffi::c_void) -> windows_core::Result<RELOGSTREAM_HANDLE>;
    fn RegisterCallback(&self, callback: Option<&ITraceEventCallback>) -> windows_core::Result<()>;
    fn Inject(&self, event: Option<&ITraceEvent>) -> windows_core::Result<()>;
    fn CreateEventInstance(&self, tracehandle: &RELOGSTREAM_HANDLE, flags: u32) -> windows_core::Result<ITraceEvent>;
    fn ProcessTrace(&self) -> windows_core::Result<()>;
    fn SetOutputFilename(&self, logfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCompressionMode(&self, compressionmode: super::super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITraceRelogger {}
impl ITraceRelogger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITraceRelogger_Vtbl
    where
        Identity: ITraceRelogger_Impl,
    {
        unsafe extern "system" fn AddLogfileTraceStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilename: core::mem::MaybeUninit<windows_core::BSTR>, usercontext: *const core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceRelogger_Impl::AddLogfileTraceStream(this, core::mem::transmute(&logfilename), core::mem::transmute_copy(&usercontext)) {
                Ok(ok__) => {
                    tracehandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggername: core::mem::MaybeUninit<windows_core::BSTR>, usercontext: *const core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceRelogger_Impl::AddRealtimeTraceStream(this, core::mem::transmute(&loggername), core::mem::transmute_copy(&usercontext)) {
                Ok(ok__) => {
                    tracehandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::RegisterCallback(this, windows_core::from_raw_borrowed(&callback)).into()
        }
        unsafe extern "system" fn Inject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::Inject(this, windows_core::from_raw_borrowed(&event)).into()
        }
        unsafe extern "system" fn CreateEventInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tracehandle: RELOGSTREAM_HANDLE, flags: u32, event: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITraceRelogger_Impl::CreateEventInstance(this, core::mem::transmute(&tracehandle), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    event.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::ProcessTrace(this).into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::SetOutputFilename(this, core::mem::transmute(&logfilename)).into()
        }
        unsafe extern "system" fn SetCompressionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::SetCompressionMode(this, core::mem::transmute_copy(&compressionmode)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITraceRelogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITraceRelogger_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddLogfileTraceStream: AddLogfileTraceStream::<Identity, OFFSET>,
            AddRealtimeTraceStream: AddRealtimeTraceStream::<Identity, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, OFFSET>,
            Inject: Inject::<Identity, OFFSET>,
            CreateEventInstance: CreateEventInstance::<Identity, OFFSET>,
            ProcessTrace: ProcessTrace::<Identity, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceRelogger as windows_core::Interface>::IID
    }
}
