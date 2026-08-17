pub trait AsyncIBackgroundCopyCallback_Impl: Sized {
    fn Begin_JobTransferred(&self, pjob: Option<&IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn Finish_JobTransferred(&self) -> windows_core::Result<()>;
    fn Begin_JobError(&self, pjob: Option<&IBackgroundCopyJob>, perror: Option<&IBackgroundCopyError>) -> windows_core::Result<()>;
    fn Finish_JobError(&self) -> windows_core::Result<()>;
    fn Begin_JobModification(&self, pjob: Option<&IBackgroundCopyJob>, dwreserved: u32) -> windows_core::Result<()>;
    fn Finish_JobModification(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIBackgroundCopyCallback {}
impl AsyncIBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIBackgroundCopyCallback_Vtbl
    where
        Identity: AsyncIBackgroundCopyCallback_Impl,
    {
        unsafe extern "system" fn Begin_JobTransferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Begin_JobTransferred(this, windows_core::from_raw_borrowed(&pjob)).into()
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Finish_JobTransferred(this).into()
        }
        unsafe extern "system" fn Begin_JobError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Begin_JobError(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&perror)).into()
        }
        unsafe extern "system" fn Finish_JobError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Finish_JobError(this).into()
        }
        unsafe extern "system" fn Begin_JobModification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Begin_JobModification(this, windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Finish_JobModification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIBackgroundCopyCallback_Impl::Finish_JobModification(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Identity, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnableBITSUploads(&self) -> windows_core::Result<()>;
    fn DisableBITSUploads(&self) -> windows_core::Result<()>;
    fn GetCleanupTaskName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCleanupTask(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBITSExtensionSetup {}
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBITSExtensionSetup_Vtbl
    where
        Identity: IBITSExtensionSetup_Impl,
    {
        unsafe extern "system" fn EnableBITSUploads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBITSExtensionSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBITSExtensionSetup_Impl::EnableBITSUploads(this).into()
        }
        unsafe extern "system" fn DisableBITSUploads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBITSExtensionSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBITSExtensionSetup_Impl::DisableBITSUploads(this).into()
        }
        unsafe extern "system" fn GetCleanupTaskName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IBITSExtensionSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBITSExtensionSetup_Impl::GetCleanupTaskName(this) {
                Ok(ok__) => {
                    ptaskname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBITSExtensionSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBITSExtensionSetup_Impl::GetCleanupTask(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnableBITSUploads: EnableBITSUploads::<Identity, OFFSET>,
            DisableBITSUploads: DisableBITSUploads::<Identity, OFFSET>,
            GetCleanupTaskName: GetCleanupTaskName::<Identity, OFFSET>,
            GetCleanupTask: GetCleanupTask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBITSExtensionSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetupFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetObject(&self, path: &windows_core::BSTR) -> windows_core::Result<IBITSExtensionSetup>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBITSExtensionSetupFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetupFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBITSExtensionSetupFactory_Vtbl
    where
        Identity: IBITSExtensionSetupFactory_Impl,
    {
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, ppextensionsetup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBITSExtensionSetupFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBITSExtensionSetupFactory_Impl::GetObject(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    ppextensionsetup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetObject: GetObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBITSExtensionSetupFactory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback_Impl: Sized {
    fn JobTransferred(&self, pjob: Option<&IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn JobError(&self, pjob: Option<&IBackgroundCopyJob>, perror: Option<&IBackgroundCopyError>) -> windows_core::Result<()>;
    fn JobModification(&self, pjob: Option<&IBackgroundCopyJob>, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyCallback {}
impl IBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyCallback_Vtbl
    where
        Identity: IBackgroundCopyCallback_Impl,
    {
        unsafe extern "system" fn JobTransferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback_Impl::JobTransferred(this, windows_core::from_raw_borrowed(&pjob)).into()
        }
        unsafe extern "system" fn JobError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback_Impl::JobError(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&perror)).into()
        }
        unsafe extern "system" fn JobModification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback_Impl::JobModification(this, windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            JobTransferred: JobTransferred::<Identity, OFFSET>,
            JobError: JobError::<Identity, OFFSET>,
            JobModification: JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback1_Impl: Sized {
    fn OnStatus(&self, pgroup: Option<&IBackgroundCopyGroup>, pjob: Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> windows_core::Result<()>;
    fn OnProgress(&self, progresstype: u32, pgroup: Option<&IBackgroundCopyGroup>, pjob: Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32) -> windows_core::Result<()>;
    fn OnProgressEx(&self, progresstype: u32, pgroup: Option<&IBackgroundCopyGroup>, pjob: Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyCallback1 {}
impl IBackgroundCopyCallback1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyCallback1_Vtbl
    where
        Identity: IBackgroundCopyCallback1_Impl,
    {
        unsafe extern "system" fn OnStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback1_Impl::OnStatus(this, windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwstatus), core::mem::transmute_copy(&dwnumofretries), core::mem::transmute_copy(&dwwin32result), core::mem::transmute_copy(&dwtransportresult)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progresstype: u32, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback1_Impl::OnProgress(this, core::mem::transmute_copy(&progresstype), windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwprogressvalue)).into()
        }
        unsafe extern "system" fn OnProgressEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progresstype: u32, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback1_Impl::OnProgressEx(this, core::mem::transmute_copy(&progresstype), windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwprogressvalue), core::mem::transmute_copy(&dwbytearraysize), core::mem::transmute_copy(&pbyte)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatus: OnStatus::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnProgressEx: OnProgressEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback1 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback2_Impl: Sized + IBackgroundCopyCallback_Impl {
    fn FileTransferred(&self, pjob: Option<&IBackgroundCopyJob>, pfile: Option<&IBackgroundCopyFile>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyCallback2 {}
impl IBackgroundCopyCallback2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyCallback2_Vtbl
    where
        Identity: IBackgroundCopyCallback2_Impl,
    {
        unsafe extern "system" fn FileTransferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, pfile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback2_Impl::FileTransferred(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&pfile)).into()
        }
        Self { base__: IBackgroundCopyCallback_Vtbl::new::<Identity, OFFSET>(), FileTransferred: FileTransferred::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyCallback3_Impl: Sized + IBackgroundCopyCallback2_Impl {
    fn FileRangesTransferred(&self, job: Option<&IBackgroundCopyJob>, file: Option<&IBackgroundCopyFile>, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyCallback3 {}
impl IBackgroundCopyCallback3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyCallback3_Vtbl
    where
        Identity: IBackgroundCopyCallback3_Impl,
    {
        unsafe extern "system" fn FileRangesTransferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyCallback3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyCallback3_Impl::FileRangesTransferred(this, windows_core::from_raw_borrowed(&job), windows_core::from_raw_borrowed(&file), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        Self { base__: IBackgroundCopyCallback2_Vtbl::new::<Identity, OFFSET>(), FileRangesTransferred: FileRangesTransferred::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback2 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyError_Impl: Sized {
    fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetFile(&self) -> windows_core::Result<IBackgroundCopyFile>;
    fn GetErrorDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetErrorContextDescription(&self, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProtocol(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IBackgroundCopyError {}
impl IBackgroundCopyError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyError_Vtbl
    where
        Identity: IBackgroundCopyError_Impl,
    {
        unsafe extern "system" fn GetError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyError_Impl::GetError(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcode)).into()
        }
        unsafe extern "system" fn GetFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyError_Impl::GetFile(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyError_Impl::GetErrorDescription(this, core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    perrordescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyError_Impl::GetErrorContextDescription(this, core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    pcontextdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyError_Impl::GetProtocol(this) {
                Ok(ok__) => {
                    pprotocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            GetFile: GetFile::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, OFFSET>,
            GetProtocol: GetProtocol::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyError as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile_Impl: Sized {
    fn GetRemoteName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLocalName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile {}
impl IBackgroundCopyFile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile_Vtbl
    where
        Identity: IBackgroundCopyFile_Impl,
    {
        unsafe extern "system" fn GetRemoteName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile_Impl::GetRemoteName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile_Impl::GetLocalName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteName: GetRemoteName::<Identity, OFFSET>,
            GetLocalName: GetLocalName::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile2_Impl: Sized + IBackgroundCopyFile_Impl {
    fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetRemoteName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile2 {}
impl IBackgroundCopyFile2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile2_Vtbl
    where
        Identity: IBackgroundCopyFile2_Impl,
    {
        unsafe extern "system" fn GetFileRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile2_Impl::GetFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetRemoteName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile2_Impl::SetRemoteName(this, core::mem::transmute(&val)).into()
        }
        Self {
            base__: IBackgroundCopyFile_Vtbl::new::<Identity, OFFSET>(),
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile3_Impl: Sized + IBackgroundCopyFile2_Impl {
    fn GetTemporaryName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetValidationState(&self, state: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetValidationState(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDownloadedFromPeer(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile3 {}
impl IBackgroundCopyFile3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile3_Vtbl
    where
        Identity: IBackgroundCopyFile3_Impl,
    {
        unsafe extern "system" fn GetTemporaryName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile3_Impl::GetTemporaryName(this) {
                Ok(ok__) => {
                    pfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile3_Impl::SetValidationState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn GetValidationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile3_Impl::GetValidationState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile3_Impl::IsDownloadedFromPeer(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyFile2_Vtbl::new::<Identity, OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Identity, OFFSET>,
            SetValidationState: SetValidationState::<Identity, OFFSET>,
            GetValidationState: GetValidationState::<Identity, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile4_Impl: Sized + IBackgroundCopyFile3_Impl {
    fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile4 {}
impl IBackgroundCopyFile4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile4_Vtbl
    where
        Identity: IBackgroundCopyFile4_Impl,
    {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile4_Impl::GetPeerDownloadStats(this, core::mem::transmute_copy(&pfromorigin), core::mem::transmute_copy(&pfrompeers)).into()
        }
        Self { base__: IBackgroundCopyFile3_Vtbl::new::<Identity, OFFSET>(), GetPeerDownloadStats: GetPeerDownloadStats::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile5_Impl: Sized + IBackgroundCopyFile4_Impl {
    fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: &BITS_FILE_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> windows_core::Result<BITS_FILE_PROPERTY_VALUE>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile5 {}
impl IBackgroundCopyFile5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile5_Vtbl
    where
        Identity: IBackgroundCopyFile5_Impl,
    {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyFile5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyFile4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyFile6_Impl: Sized + IBackgroundCopyFile5_Impl {
    fn UpdateDownloadPosition(&self, offset: u64) -> windows_core::Result<()>;
    fn RequestFileRanges(&self, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
    fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyFile6 {}
impl IBackgroundCopyFile6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyFile6_Vtbl
    where
        Identity: IBackgroundCopyFile6_Impl,
    {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u64) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile6_Impl::UpdateDownloadPosition(this, core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn RequestFileRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile6_Impl::RequestFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyFile6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyFile6_Impl::GetFilledFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        Self {
            base__: IBackgroundCopyFile5_Vtbl::new::<Identity, OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile5 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyGroup_Impl: Sized {
    fn GetProp(&self, propid: GROUPPROP) -> windows_core::Result<windows_core::VARIANT>;
    fn SetProp(&self, propid: GROUPPROP, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetProgress(&self, dwflags: u32) -> windows_core::Result<u32>;
    fn GetStatus(&self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> windows_core::Result<()>;
    fn GetJob(&self, jobid: &windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob1>;
    fn SuspendGroup(&self) -> windows_core::Result<()>;
    fn ResumeGroup(&self) -> windows_core::Result<()>;
    fn CancelGroup(&self) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn GroupID(&self) -> windows_core::Result<windows_core::GUID>;
    fn CreateJob(&self, guidjobid: &windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob1>;
    fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs1>;
    fn SwitchToForeground(&self) -> windows_core::Result<()>;
    fn QueryNewJobInterface(&self, iid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn SetNotificationPointer(&self, iid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyGroup {}
impl IBackgroundCopyGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyGroup_Vtbl
    where
        Identity: IBackgroundCopyGroup_Impl,
    {
        unsafe extern "system" fn GetProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: GROUPPROP, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::GetProp(this, core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    pvarval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: GROUPPROP, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::SetProp(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::GetProgress(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pdwprogress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pdwjobindex)).into()
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::GetJob(this, core::mem::transmute(&jobid)) {
                Ok(ok__) => {
                    ppjob.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::SuspendGroup(this).into()
        }
        unsafe extern "system" fn ResumeGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::ResumeGroup(this).into()
        }
        unsafe extern "system" fn CancelGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::CancelGroup(this).into()
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::Size(this) {
                Ok(ok__) => {
                    pdwsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidgroupid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::GroupID(this) {
                Ok(ok__) => {
                    pguidgroupid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidjobid: windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::CreateJob(this, core::mem::transmute(&guidjobid)) {
                Ok(ok__) => {
                    ppjob.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenumjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppenumjobs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::SwitchToForeground(this).into()
        }
        unsafe extern "system" fn QueryNewJobInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyGroup_Impl::QueryNewJobInterface(this, core::mem::transmute_copy(&iid)) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyGroup_Impl::SetNotificationPointer(this, core::mem::transmute_copy(&iid), windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProp: GetProp::<Identity, OFFSET>,
            SetProp: SetProp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
            SuspendGroup: SuspendGroup::<Identity, OFFSET>,
            ResumeGroup: ResumeGroup::<Identity, OFFSET>,
            CancelGroup: CancelGroup::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            GroupID: GroupID::<Identity, OFFSET>,
            CreateJob: CreateJob::<Identity, OFFSET>,
            EnumJobs: EnumJobs::<Identity, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, OFFSET>,
            QueryNewJobInterface: QueryNewJobInterface::<Identity, OFFSET>,
            SetNotificationPointer: SetNotificationPointer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyGroup as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob_Impl: Sized {
    fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::Result<()>;
    fn AddFile(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumFiles(&self) -> windows_core::Result<IEnumBackgroundCopyFiles>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn GetId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetType(&self) -> windows_core::Result<BG_JOB_TYPE>;
    fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::Result<()>;
    fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<BG_JOB_STATE>;
    fn GetError(&self) -> windows_core::Result<IBackgroundCopyError>;
    fn GetOwner(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDisplayName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDescription(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<BG_JOB_PRIORITY>;
    fn SetNotifyFlags(&self, val: u32) -> windows_core::Result<()>;
    fn GetNotifyFlags(&self) -> windows_core::Result<u32>;
    fn SetNotifyInterface(&self, val: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetNotifyInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetMinimumRetryDelay(&self, seconds: u32) -> windows_core::Result<()>;
    fn GetMinimumRetryDelay(&self) -> windows_core::Result<u32>;
    fn SetNoProgressTimeout(&self, seconds: u32) -> windows_core::Result<()>;
    fn GetNoProgressTimeout(&self) -> windows_core::Result<u32>;
    fn GetErrorCount(&self) -> windows_core::Result<u32>;
    fn SetProxySettings(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: &windows_core::PCWSTR, proxybypasslist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn TakeOwnership(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob {}
impl IBackgroundCopyJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob_Vtbl
    where
        Identity: IBackgroundCopyJob_Impl,
    {
        unsafe extern "system" fn AddFileSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::AddFileSet(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&pfileset)).into()
        }
        unsafe extern "system" fn AddFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::AddFile(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname)).into()
        }
        unsafe extern "system" fn EnumFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::EnumFiles(this) {
                Ok(ok__) => {
                    penum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::Suspend(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Complete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::Complete(this).into()
        }
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetType(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn GetTimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::GetTimes(this, core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_STATE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetState(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetError(this) {
                Ok(ok__) => {
                    pperror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetOwner(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetDisplayName(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetDescription(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: BG_JOB_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetPriority(this, core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetPriority(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetNotifyFlags(this, core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetNotifyFlags(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetNotifyInterface(this, windows_core::from_raw_borrowed(&val)).into()
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetNotifyInterface(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetMinimumRetryDelay(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetMinimumRetryDelay(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetNoProgressTimeout(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetNoProgressTimeout(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob_Impl::GetErrorCount(this) {
                Ok(ok__) => {
                    errors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: windows_core::PCWSTR, proxybypasslist: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::SetProxySettings(this, core::mem::transmute_copy(&proxyusage), core::mem::transmute(&proxylist), core::mem::transmute(&proxybypasslist)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::GetProxySettings(this, core::mem::transmute_copy(&pproxyusage), core::mem::transmute_copy(&pproxylist), core::mem::transmute_copy(&pproxybypasslist)).into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob_Impl::TakeOwnership(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFileSet: AddFileSet::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            EnumFiles: EnumFiles::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            GetTimes: GetTimes::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetError: GetError::<Identity, OFFSET>,
            GetOwner: GetOwner::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob1_Impl: Sized {
    fn CancelJob(&self) -> windows_core::Result<()>;
    fn GetProgress(&self, dwflags: u32) -> windows_core::Result<u32>;
    fn GetStatus(&self, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> windows_core::Result<()>;
    fn AddFiles(&self, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> windows_core::Result<()>;
    fn GetFile(&self, cfileindex: u32) -> windows_core::Result<FILESETINFO>;
    fn GetFileCount(&self) -> windows_core::Result<u32>;
    fn SwitchToForeground(&self) -> windows_core::Result<()>;
    fn JobID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob1 {}
impl IBackgroundCopyJob1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob1_Vtbl
    where
        Identity: IBackgroundCopyJob1_Impl,
    {
        unsafe extern "system" fn CancelJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob1_Impl::CancelJob(this).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob1_Impl::GetProgress(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pdwprogress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob1_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pdwwin32result), core::mem::transmute_copy(&pdwtransportresult), core::mem::transmute_copy(&pdwnumofretries)).into()
        }
        unsafe extern "system" fn AddFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob1_Impl::AddFiles(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&ppfileset)).into()
        }
        unsafe extern "system" fn GetFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob1_Impl::GetFile(this, core::mem::transmute_copy(&cfileindex)) {
                Ok(ok__) => {
                    pfileinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfilecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob1_Impl::GetFileCount(this) {
                Ok(ok__) => {
                    pdwfilecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob1_Impl::SwitchToForeground(this).into()
        }
        unsafe extern "system" fn JobID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidjobid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob1_Impl::JobID(this) {
                Ok(ok__) => {
                    pguidjobid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CancelJob: CancelJob::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            AddFiles: AddFiles::<Identity, OFFSET>,
            GetFile: GetFile::<Identity, OFFSET>,
            GetFileCount: GetFileCount::<Identity, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, OFFSET>,
            JobID: JobID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob1 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob2_Impl: Sized + IBackgroundCopyJob_Impl {
    fn SetNotifyCmdLine(&self, program: &windows_core::PCWSTR, parameters: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::Result<()>;
    fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> windows_core::Result<()>;
    fn SetReplyFileName(&self, replyfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetReplyFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::Result<()>;
    fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob2 {}
impl IBackgroundCopyJob2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob2_Vtbl
    where
        Identity: IBackgroundCopyJob2_Impl,
    {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, program: windows_core::PCWSTR, parameters: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::SetNotifyCmdLine(this, core::mem::transmute(&program), core::mem::transmute(&parameters)).into()
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::GetNotifyCmdLine(this, core::mem::transmute_copy(&pprogram), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetReplyProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::GetReplyProgress(this, core::mem::transmute_copy(&pprogress)).into()
        }
        unsafe extern "system" fn GetReplyData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::GetReplyData(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn SetReplyFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::SetReplyFileName(this, core::mem::transmute(&replyfilename)).into()
        }
        unsafe extern "system" fn GetReplyFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob2_Impl::GetReplyFileName(this) {
                Ok(ok__) => {
                    preplyfilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::SetCredentials(this, core::mem::transmute_copy(&credentials)).into()
        }
        unsafe extern "system" fn RemoveCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob2_Impl::RemoveCredentials(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&scheme)).into()
        }
        Self {
            base__: IBackgroundCopyJob_Vtbl::new::<Identity, OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, OFFSET>,
            GetReplyData: GetReplyData::<Identity, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob3_Impl: Sized + IBackgroundCopyJob2_Impl {
    fn ReplaceRemotePrefix(&self, oldprefix: &windows_core::PCWSTR, newprefix: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddFileWithRanges(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetFileACLFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetFileACLFlags(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob3 {}
impl IBackgroundCopyJob3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob3_Vtbl
    where
        Identity: IBackgroundCopyJob3_Impl,
    {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldprefix: windows_core::PCWSTR, newprefix: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob3_Impl::ReplaceRemotePrefix(this, core::mem::transmute(&oldprefix), core::mem::transmute(&newprefix)).into()
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob3_Impl::AddFileWithRanges(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob3_Impl::SetFileACLFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob3_Impl::GetFileACLFlags(this) {
                Ok(ok__) => {
                    flags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob2_Vtbl::new::<Identity, OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob4_Impl: Sized + IBackgroundCopyJob3_Impl {
    fn SetPeerCachingFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetPeerCachingFlags(&self) -> windows_core::Result<u32>;
    fn GetOwnerIntegrityLevel(&self) -> windows_core::Result<u32>;
    fn GetOwnerElevationState(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetMaximumDownloadTime(&self, timeout: u32) -> windows_core::Result<()>;
    fn GetMaximumDownloadTime(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob4 {}
impl IBackgroundCopyJob4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob4_Vtbl
    where
        Identity: IBackgroundCopyJob4_Impl,
    {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob4_Impl::SetPeerCachingFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob4_Impl::GetPeerCachingFlags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob4_Impl::GetOwnerIntegrityLevel(this) {
                Ok(ok__) => {
                    plevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob4_Impl::GetOwnerElevationState(this) {
                Ok(ok__) => {
                    pelevated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob4_Impl::SetMaximumDownloadTime(this, core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob4_Impl::GetMaximumDownloadTime(this) {
                Ok(ok__) => {
                    ptimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob3_Vtbl::new::<Identity, OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJob5_Impl: Sized + IBackgroundCopyJob4_Impl {
    fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: &BITS_JOB_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> windows_core::Result<BITS_JOB_PROPERTY_VALUE>;
}
impl windows_core::RuntimeName for IBackgroundCopyJob5 {}
impl IBackgroundCopyJob5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJob5_Vtbl
    where
        Identity: IBackgroundCopyJob5_Impl,
    {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJob5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJob5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJob5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob4 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJobHttpOptions_Impl: Sized {
    fn SetClientCertificateByID(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, pcerthashblob: *const u8) -> windows_core::Result<()>;
    fn SetClientCertificateByName(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, subjectname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveClientCertificate(&self) -> windows_core::Result<()>;
    fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCustomHeaders(&self, requestheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCustomHeaders(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSecurityFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetSecurityFlags(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions {}
impl IBackgroundCopyJobHttpOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions_Vtbl
    where
        Identity: IBackgroundCopyJobHttpOptions_Impl,
    {
        unsafe extern "system" fn SetClientCertificateByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, pcerthashblob: *const u8) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByID(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute_copy(&pcerthashblob)).into()
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, subjectname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByName(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&subjectname)).into()
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::RemoveClientCertificate(this).into()
        }
        unsafe extern "system" fn GetClientCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::GetClientCertificate(this, core::mem::transmute_copy(&pstorelocation), core::mem::transmute_copy(&pstorename), core::mem::transmute_copy(&ppcerthashblob), core::mem::transmute_copy(&psubjectname)).into()
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestheaders: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::SetCustomHeaders(this, core::mem::transmute(&requestheaders)).into()
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJobHttpOptions_Impl::GetCustomHeaders(this) {
                Ok(ok__) => {
                    prequestheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions_Impl::SetSecurityFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJobHttpOptions_Impl::GetSecurityFlags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Identity, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJobHttpOptions2_Impl: Sized + IBackgroundCopyJobHttpOptions_Impl {
    fn SetHttpMethod(&self, method: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHttpMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions2 {}
impl IBackgroundCopyJobHttpOptions2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions2_Vtbl
    where
        Identity: IBackgroundCopyJobHttpOptions2_Impl,
    {
        unsafe extern "system" fn SetHttpMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions2_Impl::SetHttpMethod(this, core::mem::transmute(&method)).into()
        }
        unsafe extern "system" fn GetHttpMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyJobHttpOptions2_Impl::GetHttpMethod(this) {
                Ok(ok__) => {
                    method.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions_Vtbl::new::<Identity, OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Identity, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyJobHttpOptions3_Impl: Sized + IBackgroundCopyJobHttpOptions2_Impl {
    fn SetServerCertificateValidationInterface(&self, certvalidationcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MakeCustomHeadersWriteOnly(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions3 {}
impl IBackgroundCopyJobHttpOptions3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions3_Vtbl
    where
        Identity: IBackgroundCopyJobHttpOptions3_Impl,
    {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, certvalidationcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions3_Impl::SetServerCertificateValidationInterface(this, windows_core::from_raw_borrowed(&certvalidationcallback)).into()
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyJobHttpOptions3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyJobHttpOptions3_Impl::MakeCustomHeadersWriteOnly(this).into()
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions2_Vtbl::new::<Identity, OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyManager_Impl: Sized {
    fn CreateJob(&self, displayname: &windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut Option<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn GetJob(&self, jobid: *const windows_core::GUID) -> windows_core::Result<IBackgroundCopyJob>;
    fn EnumJobs(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IBackgroundCopyManager {}
impl IBackgroundCopyManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyManager_Vtbl
    where
        Identity: IBackgroundCopyManager_Impl,
    {
        unsafe extern "system" fn CreateJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyManager_Impl::CreateJob(this, core::mem::transmute(&displayname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pjobid), core::mem::transmute_copy(&ppjob)).into()
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: *const windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyManager_Impl::GetJob(this, core::mem::transmute_copy(&jobid)) {
                Ok(ok__) => {
                    ppjob.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyManager_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyManager_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    perrordescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateJob: CreateJob::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
            EnumJobs: EnumJobs::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyQMgr_Impl: Sized {
    fn CreateGroup(&self, guidgroupid: &windows_core::GUID) -> windows_core::Result<IBackgroundCopyGroup>;
    fn GetGroup(&self, groupid: &windows_core::GUID) -> windows_core::Result<IBackgroundCopyGroup>;
    fn EnumGroups(&self, dwflags: u32) -> windows_core::Result<IEnumBackgroundCopyGroups>;
}
impl windows_core::RuntimeName for IBackgroundCopyQMgr {}
impl IBackgroundCopyQMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyQMgr_Vtbl
    where
        Identity: IBackgroundCopyQMgr_Impl,
    {
        unsafe extern "system" fn CreateGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidgroupid: windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyQMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyQMgr_Impl::CreateGroup(this, core::mem::transmute(&guidgroupid)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupid: windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyQMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyQMgr_Impl::GetGroup(this, core::mem::transmute(&groupid)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenumgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyQMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundCopyQMgr_Impl::EnumGroups(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppenumgroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateGroup: CreateGroup::<Identity, OFFSET>,
            GetGroup: GetGroup::<Identity, OFFSET>,
            EnumGroups: EnumGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyQMgr as windows_core::Interface>::IID
    }
}
pub trait IBackgroundCopyServerCertificateValidationCallback_Impl: Sized {
    fn ValidateServerCertificate(&self, job: Option<&IBackgroundCopyJob>, file: Option<&IBackgroundCopyFile>, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundCopyServerCertificateValidationCallback {}
impl IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBackgroundCopyServerCertificateValidationCallback_Vtbl
    where
        Identity: IBackgroundCopyServerCertificateValidationCallback_Impl,
    {
        unsafe extern "system" fn ValidateServerCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::HRESULT
        where
            Identity: IBackgroundCopyServerCertificateValidationCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBackgroundCopyServerCertificateValidationCallback_Impl::ValidateServerCertificate(this, windows_core::from_raw_borrowed(&job), windows_core::from_raw_borrowed(&file), core::mem::transmute_copy(&certlength), core::mem::transmute_copy(&certdata), core::mem::transmute_copy(&certencodingtype), core::mem::transmute_copy(&certstorelength), core::mem::transmute_copy(&certstoredata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as windows_core::Interface>::IID
    }
}
pub trait IBitsPeer_Impl: Sized {
    fn GetPeerName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsAuthenticated(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsAvailable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IBitsPeer {}
impl IBitsPeer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBitsPeer_Vtbl
    where
        Identity: IBitsPeer_Impl,
    {
        unsafe extern "system" fn GetPeerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBitsPeer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeer_Impl::GetPeerName(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBitsPeer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeer_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    pauth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IBitsPeer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeer_Impl::IsAvailable(this) {
                Ok(ok__) => {
                    ponline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeerName: GetPeerName::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            IsAvailable: IsAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeer as windows_core::Interface>::IID
    }
}
pub trait IBitsPeerCacheAdministration_Impl: Sized {
    fn GetMaximumCacheSize(&self) -> windows_core::Result<u32>;
    fn SetMaximumCacheSize(&self, bytes: u32) -> windows_core::Result<()>;
    fn GetMaximumContentAge(&self) -> windows_core::Result<u32>;
    fn SetMaximumContentAge(&self, seconds: u32) -> windows_core::HRESULT;
    fn GetConfigurationFlags(&self) -> windows_core::Result<u32>;
    fn SetConfigurationFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn EnumRecords(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<IBitsPeerCacheRecord>;
    fn ClearRecords(&self) -> windows_core::Result<()>;
    fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteUrl(&self, url: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumPeers(&self) -> windows_core::Result<IEnumBitsPeers>;
    fn ClearPeers(&self) -> windows_core::Result<()>;
    fn DiscoverPeers(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBitsPeerCacheAdministration {}
impl IBitsPeerCacheAdministration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBitsPeerCacheAdministration_Vtbl
    where
        Identity: IBitsPeerCacheAdministration_Impl,
    {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::GetMaximumCacheSize(this) {
                Ok(ok__) => {
                    pbytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytes: u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::SetMaximumCacheSize(this, core::mem::transmute_copy(&bytes)).into()
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pseconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::GetMaximumContentAge(this) {
                Ok(ok__) => {
                    pseconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::SetMaximumContentAge(this, core::mem::transmute_copy(&seconds))
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::GetConfigurationFlags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::SetConfigurationFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::EnumRecords(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID, pprecord: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::GetRecord(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pprecord.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::ClearRecords(this).into()
        }
        unsafe extern "system" fn DeleteRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::DeleteRecord(this, core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DeleteUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::DeleteUrl(this, core::mem::transmute(&url)).into()
        }
        unsafe extern "system" fn EnumPeers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheAdministration_Impl::EnumPeers(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPeers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::ClearPeers(this).into()
        }
        unsafe extern "system" fn DiscoverPeers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheAdministration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheAdministration_Impl::DiscoverPeers(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, OFFSET>,
            EnumRecords: EnumRecords::<Identity, OFFSET>,
            GetRecord: GetRecord::<Identity, OFFSET>,
            ClearRecords: ClearRecords::<Identity, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, OFFSET>,
            EnumPeers: EnumPeers::<Identity, OFFSET>,
            ClearPeers: ClearPeers::<Identity, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as windows_core::Interface>::IID
    }
}
pub trait IBitsPeerCacheRecord_Impl: Sized {
    fn GetId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetOriginUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn GetFileModificationTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetLastAccessTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsFileValidated(&self) -> windows_core::Result<()>;
    fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBitsPeerCacheRecord {}
impl IBitsPeerCacheRecord_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBitsPeerCacheRecord_Vtbl
    where
        Identity: IBitsPeerCacheRecord_Impl,
    {
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheRecord_Impl::GetId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheRecord_Impl::GetOriginUrl(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u64) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheRecord_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheRecord_Impl::GetFileModificationTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsPeerCacheRecord_Impl::GetLastAccessTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileValidated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheRecord_Impl::IsFileValidated(this).into()
        }
        unsafe extern "system" fn GetFileRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT
        where
            Identity: IBitsPeerCacheRecord_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsPeerCacheRecord_Impl::GetFileRanges(this, core::mem::transmute_copy(&prangecount), core::mem::transmute_copy(&ppranges)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as windows_core::Interface>::IID
    }
}
pub trait IBitsTokenOptions_Impl: Sized {
    fn SetHelperTokenFlags(&self, usageflags: BG_TOKEN) -> windows_core::Result<()>;
    fn GetHelperTokenFlags(&self) -> windows_core::Result<BG_TOKEN>;
    fn SetHelperToken(&self) -> windows_core::Result<()>;
    fn ClearHelperToken(&self) -> windows_core::Result<()>;
    fn GetHelperTokenSid(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IBitsTokenOptions {}
impl IBitsTokenOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBitsTokenOptions_Vtbl
    where
        Identity: IBitsTokenOptions_Impl,
    {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usageflags: BG_TOKEN) -> windows_core::HRESULT
        where
            Identity: IBitsTokenOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsTokenOptions_Impl::SetHelperTokenFlags(this, core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut BG_TOKEN) -> windows_core::HRESULT
        where
            Identity: IBitsTokenOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsTokenOptions_Impl::GetHelperTokenFlags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelperToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsTokenOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsTokenOptions_Impl::SetHelperToken(this).into()
        }
        unsafe extern "system" fn ClearHelperToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBitsTokenOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBitsTokenOptions_Impl::ClearHelperToken(this).into()
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IBitsTokenOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBitsTokenOptions_Impl::GetHelperTokenSid(this) {
                Ok(ok__) => {
                    psid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsTokenOptions as windows_core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyFiles_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyFiles>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBackgroundCopyFiles {}
impl IEnumBackgroundCopyFiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBackgroundCopyFiles_Vtbl
    where
        Identity: IEnumBackgroundCopyFiles_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyFiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyFiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyFiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyFiles_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as windows_core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyGroups_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyGroups>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBackgroundCopyGroups {}
impl IEnumBackgroundCopyGroups_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBackgroundCopyGroups_Vtbl
    where
        Identity: IEnumBackgroundCopyGroups_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyGroups_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyGroups_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyGroups_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyGroups_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyGroups_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyGroups as windows_core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBackgroundCopyJobs {}
impl IEnumBackgroundCopyJobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs_Vtbl
    where
        Identity: IEnumBackgroundCopyJobs_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyJobs_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyJobs_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as windows_core::Interface>::IID
    }
}
pub trait IEnumBackgroundCopyJobs1_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBackgroundCopyJobs1>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBackgroundCopyJobs1 {}
impl IEnumBackgroundCopyJobs1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs1_Vtbl
    where
        Identity: IEnumBackgroundCopyJobs1_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs1_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs1_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBackgroundCopyJobs1_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyJobs1_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBackgroundCopyJobs1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBackgroundCopyJobs1_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs1 as windows_core::Interface>::IID
    }
}
pub trait IEnumBitsPeerCacheRecords_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBitsPeerCacheRecords {}
impl IEnumBitsPeerCacheRecords_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBitsPeerCacheRecords_Vtbl
    where
        Identity: IEnumBitsPeerCacheRecords_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeerCacheRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeerCacheRecords_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeerCacheRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeerCacheRecords_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeerCacheRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeerCacheRecords_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeerCacheRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBitsPeerCacheRecords_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeerCacheRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBitsPeerCacheRecords_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as windows_core::Interface>::IID
    }
}
pub trait IEnumBitsPeers_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IBitsPeer>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBitsPeers>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumBitsPeers {}
impl IEnumBitsPeers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumBitsPeers_Vtbl
    where
        Identity: IEnumBitsPeers_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumBitsPeers_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBitsPeers_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumBitsPeers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumBitsPeers_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeers as windows_core::Interface>::IID
    }
}
