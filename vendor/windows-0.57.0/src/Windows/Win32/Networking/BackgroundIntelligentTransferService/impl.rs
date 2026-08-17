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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>() -> AsyncIBackgroundCopyCallback_Vtbl {
        unsafe extern "system" fn Begin_JobTransferred<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Begin_JobTransferred(this, windows_core::from_raw_borrowed(&pjob)).into()
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Finish_JobTransferred(this).into()
        }
        unsafe extern "system" fn Begin_JobError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Begin_JobError(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&perror)).into()
        }
        unsafe extern "system" fn Finish_JobError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Finish_JobError(this).into()
        }
        unsafe extern "system" fn Begin_JobModification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Begin_JobModification(this, windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Finish_JobModification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            AsyncIBackgroundCopyCallback_Impl::Finish_JobModification(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Identity, Impl, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, Impl, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, Impl, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, Impl, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, Impl, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>() -> IBITSExtensionSetup_Vtbl {
        unsafe extern "system" fn EnableBITSUploads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBITSExtensionSetup_Impl::EnableBITSUploads(this).into()
        }
        unsafe extern "system" fn DisableBITSUploads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBITSExtensionSetup_Impl::DisableBITSUploads(this).into()
        }
        unsafe extern "system" fn GetCleanupTaskName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBITSExtensionSetup_Impl::GetCleanupTaskName(this) {
                Ok(ok__) => {
                    core::ptr::write(ptaskname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCleanupTask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBITSExtensionSetup_Impl::GetCleanupTask(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableBITSUploads: EnableBITSUploads::<Identity, Impl, OFFSET>,
            DisableBITSUploads: DisableBITSUploads::<Identity, Impl, OFFSET>,
            GetCleanupTaskName: GetCleanupTaskName::<Identity, Impl, OFFSET>,
            GetCleanupTask: GetCleanupTask::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: isize>() -> IBITSExtensionSetupFactory_Vtbl {
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, ppextensionsetup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBITSExtensionSetupFactory_Impl::GetObject(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(ppextensionsetup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetObject: GetObject::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback_Vtbl {
        unsafe extern "system" fn JobTransferred<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback_Impl::JobTransferred(this, windows_core::from_raw_borrowed(&pjob)).into()
        }
        unsafe extern "system" fn JobError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback_Impl::JobError(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&perror)).into()
        }
        unsafe extern "system" fn JobModification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback_Impl::JobModification(this, windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            JobTransferred: JobTransferred::<Identity, Impl, OFFSET>,
            JobError: JobError::<Identity, Impl, OFFSET>,
            JobModification: JobModification::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback1_Vtbl {
        unsafe extern "system" fn OnStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback1_Impl::OnStatus(this, windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwstatus), core::mem::transmute_copy(&dwnumofretries), core::mem::transmute_copy(&dwwin32result), core::mem::transmute_copy(&dwtransportresult)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progresstype: u32, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback1_Impl::OnProgress(this, core::mem::transmute_copy(&progresstype), windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwprogressvalue)).into()
        }
        unsafe extern "system" fn OnProgressEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progresstype: u32, pgroup: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback1_Impl::OnProgressEx(this, core::mem::transmute_copy(&progresstype), windows_core::from_raw_borrowed(&pgroup), windows_core::from_raw_borrowed(&pjob), core::mem::transmute_copy(&dwfileindex), core::mem::transmute_copy(&dwprogressvalue), core::mem::transmute_copy(&dwbytearraysize), core::mem::transmute_copy(&pbyte)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatus: OnStatus::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnProgressEx: OnProgressEx::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback2_Vtbl {
        unsafe extern "system" fn FileTransferred<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, pfile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback2_Impl::FileTransferred(this, windows_core::from_raw_borrowed(&pjob), windows_core::from_raw_borrowed(&pfile)).into()
        }
        Self { base__: IBackgroundCopyCallback_Vtbl::new::<Identity, Impl, OFFSET>(), FileTransferred: FileTransferred::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: isize>() -> IBackgroundCopyCallback3_Vtbl {
        unsafe extern "system" fn FileRangesTransferred<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyCallback3_Impl::FileRangesTransferred(this, windows_core::from_raw_borrowed(&job), windows_core::from_raw_borrowed(&file), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        Self { base__: IBackgroundCopyCallback2_Vtbl::new::<Identity, Impl, OFFSET>(), FileRangesTransferred: FileRangesTransferred::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>() -> IBackgroundCopyError_Vtbl {
        unsafe extern "system" fn GetError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyError_Impl::GetError(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcode)).into()
        }
        unsafe extern "system" fn GetFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyError_Impl::GetFile(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyError_Impl::GetErrorDescription(this, core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    core::ptr::write(perrordescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: u32, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyError_Impl::GetErrorContextDescription(this, core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    core::ptr::write(pcontextdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyError_Impl::GetProtocol(this) {
                Ok(ok__) => {
                    core::ptr::write(pprotocol, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, Impl, OFFSET>,
            GetProtocol: GetProtocol::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>() -> IBackgroundCopyFile_Vtbl {
        unsafe extern "system" fn GetRemoteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile_Impl::GetRemoteName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile_Impl::GetLocalName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteName: GetRemoteName::<Identity, Impl, OFFSET>,
            GetLocalName: GetLocalName::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>() -> IBackgroundCopyFile2_Vtbl {
        unsafe extern "system" fn GetFileRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile2_Impl::GetFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetRemoteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile2_Impl::SetRemoteName(this, core::mem::transmute(&val)).into()
        }
        Self {
            base__: IBackgroundCopyFile_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>() -> IBackgroundCopyFile3_Vtbl {
        unsafe extern "system" fn GetTemporaryName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile3_Impl::GetTemporaryName(this) {
                Ok(ok__) => {
                    core::ptr::write(pfilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile3_Impl::SetValidationState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn GetValidationState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile3_Impl::GetValidationState(this) {
                Ok(ok__) => {
                    core::ptr::write(pstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile3_Impl::IsDownloadedFromPeer(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyFile2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Identity, Impl, OFFSET>,
            SetValidationState: SetValidationState::<Identity, Impl, OFFSET>,
            GetValidationState: GetValidationState::<Identity, Impl, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile4_Impl, const OFFSET: isize>() -> IBackgroundCopyFile4_Vtbl {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile4_Impl::GetPeerDownloadStats(this, core::mem::transmute_copy(&pfromorigin), core::mem::transmute_copy(&pfrompeers)).into()
        }
        Self { base__: IBackgroundCopyFile3_Vtbl::new::<Identity, Impl, OFFSET>(), GetPeerDownloadStats: GetPeerDownloadStats::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>() -> IBackgroundCopyFile5_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyFile5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyFile4_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>() -> IBackgroundCopyFile6_Vtbl {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile6_Impl::UpdateDownloadPosition(this, core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn RequestFileRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile6_Impl::RequestFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyFile6_Impl::GetFilledFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        Self {
            base__: IBackgroundCopyFile5_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, Impl, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, Impl, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>() -> IBackgroundCopyGroup_Vtbl {
        unsafe extern "system" fn GetProp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: GROUPPROP, pvarval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::GetProp(this, core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    core::ptr::write(pvarval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: GROUPPROP, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::SetProp(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::GetProgress(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    core::ptr::write(pdwprogress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pdwjobindex)).into()
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::GetJob(this, core::mem::transmute(&jobid)) {
                Ok(ok__) => {
                    core::ptr::write(ppjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::SuspendGroup(this).into()
        }
        unsafe extern "system" fn ResumeGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::ResumeGroup(this).into()
        }
        unsafe extern "system" fn CancelGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::CancelGroup(this).into()
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidgroupid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::GroupID(this) {
                Ok(ok__) => {
                    core::ptr::write(pguidgroupid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidjobid: windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::CreateJob(this, core::mem::transmute(&guidjobid)) {
                Ok(ok__) => {
                    core::ptr::write(ppjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenumjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumjobs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::SwitchToForeground(this).into()
        }
        unsafe extern "system" fn QueryNewJobInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyGroup_Impl::QueryNewJobInterface(this, core::mem::transmute_copy(&iid)) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationPointer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyGroup_Impl::SetNotificationPointer(this, core::mem::transmute_copy(&iid), windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProp: GetProp::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            SuspendGroup: SuspendGroup::<Identity, Impl, OFFSET>,
            ResumeGroup: ResumeGroup::<Identity, Impl, OFFSET>,
            CancelGroup: CancelGroup::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            GroupID: GroupID::<Identity, Impl, OFFSET>,
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            QueryNewJobInterface: QueryNewJobInterface::<Identity, Impl, OFFSET>,
            SetNotificationPointer: SetNotificationPointer::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>() -> IBackgroundCopyJob_Vtbl {
        unsafe extern "system" fn AddFileSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::AddFileSet(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&pfileset)).into()
        }
        unsafe extern "system" fn AddFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::AddFile(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname)).into()
        }
        unsafe extern "system" fn EnumFiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::EnumFiles(this) {
                Ok(ok__) => {
                    core::ptr::write(penum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::Suspend(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Complete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::Complete(this).into()
        }
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetId(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetType(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn GetTimes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::GetTimes(this, core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetState(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetError(this) {
                Ok(ok__) => {
                    core::ptr::write(pperror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetDisplayName(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetDescription(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: BG_JOB_PRIORITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetPriority(this, core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetPriority(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetNotifyFlags(this, core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetNotifyFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetNotifyInterface(this, windows_core::from_raw_borrowed(&val)).into()
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetNotifyInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetMinimumRetryDelay(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetMinimumRetryDelay(this) {
                Ok(ok__) => {
                    core::ptr::write(seconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetNoProgressTimeout(this, core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetNoProgressTimeout(this) {
                Ok(ok__) => {
                    core::ptr::write(seconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob_Impl::GetErrorCount(this) {
                Ok(ok__) => {
                    core::ptr::write(errors, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: windows_core::PCWSTR, proxybypasslist: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::SetProxySettings(this, core::mem::transmute_copy(&proxyusage), core::mem::transmute(&proxylist), core::mem::transmute(&proxybypasslist)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::GetProxySettings(this, core::mem::transmute_copy(&pproxyusage), core::mem::transmute_copy(&pproxylist), core::mem::transmute_copy(&pproxybypasslist)).into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob_Impl::TakeOwnership(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFileSet: AddFileSet::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            EnumFiles: EnumFiles::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, Impl, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, Impl, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, Impl, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, Impl, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, Impl, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, Impl, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>() -> IBackgroundCopyJob1_Vtbl {
        unsafe extern "system" fn CancelJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob1_Impl::CancelJob(this).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob1_Impl::GetProgress(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    core::ptr::write(pdwprogress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob1_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pdwwin32result), core::mem::transmute_copy(&pdwtransportresult), core::mem::transmute_copy(&pdwnumofretries)).into()
        }
        unsafe extern "system" fn AddFiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob1_Impl::AddFiles(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&ppfileset)).into()
        }
        unsafe extern "system" fn GetFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob1_Impl::GetFile(this, core::mem::transmute_copy(&cfileindex)) {
                Ok(ok__) => {
                    core::ptr::write(pfileinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfilecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob1_Impl::GetFileCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwfilecount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchToForeground<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob1_Impl::SwitchToForeground(this).into()
        }
        unsafe extern "system" fn JobID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidjobid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob1_Impl::JobID(this) {
                Ok(ok__) => {
                    core::ptr::write(pguidjobid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CancelJob: CancelJob::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            AddFiles: AddFiles::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetFileCount: GetFileCount::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            JobID: JobID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>() -> IBackgroundCopyJob2_Vtbl {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, program: windows_core::PCWSTR, parameters: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::SetNotifyCmdLine(this, core::mem::transmute(&program), core::mem::transmute(&parameters)).into()
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::GetNotifyCmdLine(this, core::mem::transmute_copy(&pprogram), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetReplyProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::GetReplyProgress(this, core::mem::transmute_copy(&pprogress)).into()
        }
        unsafe extern "system" fn GetReplyData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::GetReplyData(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn SetReplyFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::SetReplyFileName(this, core::mem::transmute(&replyfilename)).into()
        }
        unsafe extern "system" fn GetReplyFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob2_Impl::GetReplyFileName(this) {
                Ok(ok__) => {
                    core::ptr::write(preplyfilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::SetCredentials(this, core::mem::transmute_copy(&credentials)).into()
        }
        unsafe extern "system" fn RemoveCredentials<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob2_Impl::RemoveCredentials(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&scheme)).into()
        }
        Self {
            base__: IBackgroundCopyJob_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, Impl, OFFSET>,
            GetReplyData: GetReplyData::<Identity, Impl, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, Impl, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>() -> IBackgroundCopyJob3_Vtbl {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldprefix: windows_core::PCWSTR, newprefix: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob3_Impl::ReplaceRemotePrefix(this, core::mem::transmute(&oldprefix), core::mem::transmute(&newprefix)).into()
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob3_Impl::AddFileWithRanges(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob3_Impl::SetFileACLFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob3_Impl::GetFileACLFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(flags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, Impl, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, Impl, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, Impl, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>() -> IBackgroundCopyJob4_Vtbl {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob4_Impl::SetPeerCachingFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob4_Impl::GetPeerCachingFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob4_Impl::GetOwnerIntegrityLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(plevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob4_Impl::GetOwnerElevationState(this) {
                Ok(ok__) => {
                    core::ptr::write(pelevated, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob4_Impl::SetMaximumDownloadTime(this, core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob4_Impl::GetMaximumDownloadTime(this) {
                Ok(ok__) => {
                    core::ptr::write(ptimeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob3_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, Impl, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, Impl, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, Impl, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>() -> IBackgroundCopyJob5_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJob5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJob5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJob4_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions_Vtbl {
        unsafe extern "system" fn SetClientCertificateByID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, pcerthashblob: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByID(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute_copy(&pcerthashblob)).into()
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, subjectname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByName(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&subjectname)).into()
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::RemoveClientCertificate(this).into()
        }
        unsafe extern "system" fn GetClientCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::GetClientCertificate(this, core::mem::transmute_copy(&pstorelocation), core::mem::transmute_copy(&pstorename), core::mem::transmute_copy(&ppcerthashblob), core::mem::transmute_copy(&psubjectname)).into()
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::SetCustomHeaders(this, core::mem::transmute(&requestheaders)).into()
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJobHttpOptions_Impl::GetCustomHeaders(this) {
                Ok(ok__) => {
                    core::ptr::write(prequestheaders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions_Impl::SetSecurityFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJobHttpOptions_Impl::GetSecurityFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Identity, Impl, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, Impl, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, Impl, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, Impl, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, Impl, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, Impl, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions2_Vtbl {
        unsafe extern "system" fn SetHttpMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions2_Impl::SetHttpMethod(this, core::mem::transmute(&method)).into()
        }
        unsafe extern "system" fn GetHttpMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyJobHttpOptions2_Impl::GetHttpMethod(this) {
                Ok(ok__) => {
                    core::ptr::write(method, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Identity, Impl, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>() -> IBackgroundCopyJobHttpOptions3_Vtbl {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certvalidationcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions3_Impl::SetServerCertificateValidationInterface(this, windows_core::from_raw_borrowed(&certvalidationcallback)).into()
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyJobHttpOptions3_Impl::MakeCustomHeadersWriteOnly(this).into()
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, Impl, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>() -> IBackgroundCopyManager_Vtbl {
        unsafe extern "system" fn CreateJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyManager_Impl::CreateJob(this, core::mem::transmute(&displayname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pjobid), core::mem::transmute_copy(&ppjob)).into()
        }
        unsafe extern "system" fn GetJob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: *const windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyManager_Impl::GetJob(this, core::mem::transmute_copy(&jobid)) {
                Ok(ok__) => {
                    core::ptr::write(ppjob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyManager_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, languageid: u32, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyManager_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&languageid)) {
                Ok(ok__) => {
                    core::ptr::write(perrordescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>() -> IBackgroundCopyQMgr_Vtbl {
        unsafe extern "system" fn CreateGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidgroupid: windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyQMgr_Impl::CreateGroup(this, core::mem::transmute(&guidgroupid)) {
                Ok(ok__) => {
                    core::ptr::write(ppgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupid: windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyQMgr_Impl::GetGroup(this, core::mem::transmute(&groupid)) {
                Ok(ok__) => {
                    core::ptr::write(ppgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenumgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundCopyQMgr_Impl::EnumGroups(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumgroups, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateGroup: CreateGroup::<Identity, Impl, OFFSET>,
            GetGroup: GetGroup::<Identity, Impl, OFFSET>,
            EnumGroups: EnumGroups::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>() -> IBackgroundCopyServerCertificateValidationCallback_Vtbl {
        unsafe extern "system" fn ValidateServerCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundCopyServerCertificateValidationCallback_Impl::ValidateServerCertificate(this, windows_core::from_raw_borrowed(&job), windows_core::from_raw_borrowed(&file), core::mem::transmute_copy(&certlength), core::mem::transmute_copy(&certdata), core::mem::transmute_copy(&certencodingtype), core::mem::transmute_copy(&certstorelength), core::mem::transmute_copy(&certstoredata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: isize>() -> IBitsPeer_Vtbl {
        unsafe extern "system" fn GetPeerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeer_Impl::GetPeerName(this) {
                Ok(ok__) => {
                    core::ptr::write(pname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeer_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    core::ptr::write(pauth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeer_Impl::IsAvailable(this) {
                Ok(ok__) => {
                    core::ptr::write(ponline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeerName: GetPeerName::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>() -> IBitsPeerCacheAdministration_Vtbl {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytes: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::GetMaximumCacheSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pbytes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytes: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::SetMaximumCacheSize(this, core::mem::transmute_copy(&bytes)).into()
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pseconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::GetMaximumContentAge(this) {
                Ok(ok__) => {
                    core::ptr::write(pseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::SetMaximumContentAge(this, core::mem::transmute_copy(&seconds))
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::GetConfigurationFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::SetConfigurationFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumRecords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::EnumRecords(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID, pprecord: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::GetRecord(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    core::ptr::write(pprecord, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRecords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::ClearRecords(this).into()
        }
        unsafe extern "system" fn DeleteRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::DeleteRecord(this, core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DeleteUrl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::DeleteUrl(this, core::mem::transmute(&url)).into()
        }
        unsafe extern "system" fn EnumPeers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheAdministration_Impl::EnumPeers(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPeers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::ClearPeers(this).into()
        }
        unsafe extern "system" fn DiscoverPeers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheAdministration_Impl::DiscoverPeers(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, Impl, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, Impl, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, Impl, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, Impl, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, Impl, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, Impl, OFFSET>,
            EnumRecords: EnumRecords::<Identity, Impl, OFFSET>,
            GetRecord: GetRecord::<Identity, Impl, OFFSET>,
            ClearRecords: ClearRecords::<Identity, Impl, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, Impl, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, Impl, OFFSET>,
            EnumPeers: EnumPeers::<Identity, Impl, OFFSET>,
            ClearPeers: ClearPeers::<Identity, Impl, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>() -> IBitsPeerCacheRecord_Vtbl {
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheRecord_Impl::GetId(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginUrl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheRecord_Impl::GetOriginUrl(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheRecord_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheRecord_Impl::GetFileModificationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsPeerCacheRecord_Impl::GetLastAccessTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFileValidated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheRecord_Impl::IsFileValidated(this).into()
        }
        unsafe extern "system" fn GetFileRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsPeerCacheRecord_Impl::GetFileRanges(this, core::mem::transmute_copy(&prangecount), core::mem::transmute_copy(&ppranges)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, Impl, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, Impl, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, Impl, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>() -> IBitsTokenOptions_Vtbl {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usageflags: BG_TOKEN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsTokenOptions_Impl::SetHelperTokenFlags(this, core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut BG_TOKEN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsTokenOptions_Impl::GetHelperTokenFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelperToken<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsTokenOptions_Impl::SetHelperToken(this).into()
        }
        unsafe extern "system" fn ClearHelperToken<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBitsTokenOptions_Impl::ClearHelperToken(this).into()
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitsTokenOptions_Impl::GetHelperTokenSid(this) {
                Ok(ok__) => {
                    core::ptr::write(psid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, Impl, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, Impl, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, Impl, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, Impl, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyFiles_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyFiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyFiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyFiles_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyFiles_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyGroups_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyGroups_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyGroups_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyGroups_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyGroups_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyGroups_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyJobs_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyJobs_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>() -> IEnumBackgroundCopyJobs1_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs1_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs1_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBackgroundCopyJobs1_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyJobs1_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBackgroundCopyJobs1_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>() -> IEnumBitsPeerCacheRecords_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeerCacheRecords_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeerCacheRecords_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeerCacheRecords_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBitsPeerCacheRecords_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBitsPeerCacheRecords_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>() -> IEnumBitsPeers_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumBitsPeers_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBitsPeers_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumBitsPeers_Impl::GetCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pucount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeers as windows_core::Interface>::IID
    }
}
