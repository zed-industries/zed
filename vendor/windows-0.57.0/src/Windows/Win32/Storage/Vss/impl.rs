pub trait IVssAdmin_Impl: Sized {
    fn RegisterProvider(&self, pproviderid: &windows_core::GUID, classid: &windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, providerid: &windows_core::GUID) -> windows_core::Result<()>;
    fn QueryProviders(&self) -> windows_core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssAdmin {}
impl IVssAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: isize>() -> IVssAdmin_Vtbl {
        unsafe extern "system" fn RegisterProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, classid: windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAdmin_Impl::RegisterProvider(this, core::mem::transmute(&pproviderid), core::mem::transmute(&classid), core::mem::transmute_copy(&pwszprovidername), core::mem::transmute_copy(&eprovidertype), core::mem::transmute_copy(&pwszproviderversion), core::mem::transmute(&providerversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAdmin_Impl::UnregisterProvider(this, core::mem::transmute(&providerid)).into()
        }
        unsafe extern "system" fn QueryProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssAdmin_Impl::QueryProviders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAdmin_Impl::AbortAllSnapshotsInProgress(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
            QueryProviders: QueryProviders::<Identity, Impl, OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAdmin as windows_core::Interface>::IID
    }
}
pub trait IVssAdminEx_Impl: Sized + IVssAdmin_Impl {
    fn GetProviderCapability(&self, pproviderid: &windows_core::GUID) -> windows_core::Result<u64>;
    fn GetProviderContext(&self, providerid: &windows_core::GUID) -> windows_core::Result<i32>;
    fn SetProviderContext(&self, providerid: &windows_core::GUID, lcontext: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssAdminEx {}
impl IVssAdminEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: isize>() -> IVssAdminEx_Vtbl {
        unsafe extern "system" fn GetProviderCapability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssAdminEx_Impl::GetProviderCapability(this, core::mem::transmute(&pproviderid)) {
                Ok(ok__) => {
                    core::ptr::write(plloriginalcapabilitymask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, plcontext: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssAdminEx_Impl::GetProviderContext(this, core::mem::transmute(&providerid)) {
                Ok(ok__) => {
                    core::ptr::write(plcontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAdminEx_Impl::SetProviderContext(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)).into()
        }
        Self {
            base__: IVssAdmin_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProviderCapability: GetProviderCapability::<Identity, Impl, OFFSET>,
            GetProviderContext: GetProviderContext::<Identity, Impl, OFFSET>,
            SetProviderContext: SetProviderContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAdminEx as windows_core::Interface>::IID || iid == &<IVssAdmin as windows_core::Interface>::IID
    }
}
pub trait IVssAsync_Impl: Sized {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssAsync {}
impl IVssAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: isize>() -> IVssAsync_Vtbl {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAsync_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAsync_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssAsync_Impl::QueryStatus(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&preserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAsync as windows_core::Interface>::IID
    }
}
pub trait IVssComponent_Impl: Sized {
    fn GetLogicalPath(&self, pbstrpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::Result<()>;
    fn GetComponentName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> windows_core::Result<()>;
    fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> windows_core::Result<()>;
    fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn SetBackupMetadata(&self, wszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupMetadata(&self, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddPartialFile(&self, wszpath: &windows_core::PCWSTR, wszfilename: &windows_core::PCWSTR, wszranges: &windows_core::PCWSTR, wszmetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> windows_core::Result<()>;
    fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> windows_core::Result<()>;
    fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> windows_core::Result<()>;
    fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> windows_core::Result<()>;
    fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn AddDirectedTarget(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilename: &windows_core::PCWSTR, wszsourcerangelist: &windows_core::PCWSTR, wszdestinationpath: &windows_core::PCWSTR, wszdestinationfilename: &windows_core::PCWSTR, wszdestinationrangelist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> windows_core::Result<()>;
    fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreMetadata(&self, wszrestoremetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::Result<()>;
    fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::Result<()>;
    fn SetPreRestoreFailureMsg(&self, wszprerestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPostRestoreFailureMsg(&self, wszpostrestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBackupStamp(&self, wszbackupstamp: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBackupOptions(&self, pbstrbackupoptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> windows_core::Result<()>;
    fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::Result<()>;
    fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyTime(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> windows_core::Result<()>;
    fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssComponent {}
impl IVssComponent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>() -> IVssComponent_Vtbl {
        unsafe extern "system" fn GetLogicalPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrpath)).into()
        }
        unsafe extern "system" fn GetComponentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetComponentType(this, core::mem::transmute_copy(&pct)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsucceeded: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetBackupSucceeded(this, core::mem::transmute_copy(&pbsucceeded)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmappings: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetAlternateLocationMappingCount(this, core::mem::transmute_copy(&pcmappings)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponent_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping)) {
                Ok(ok__) => {
                    core::ptr::write(ppfiledesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetBackupMetadata(this, core::mem::transmute(&wszdata)).into()
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetBackupMetadata(this, core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn AddPartialFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, wszranges: windows_core::PCWSTR, wszmetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::AddPartialFile(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute(&wszranges), core::mem::transmute(&wszmetadata)).into()
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpartialfiles: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetPartialFileCount(this, core::mem::transmute_copy(&pcpartialfiles)).into()
        }
        unsafe extern "system" fn GetPartialFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrrange: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrmetadata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetPartialFile(this, core::mem::transmute_copy(&ipartialfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pbstrrange), core::mem::transmute_copy(&pbstrmetadata)).into()
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbselectedforrestore: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::IsSelectedForRestore(this, core::mem::transmute_copy(&pbselectedforrestore)).into()
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbadditionalrestores: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetAdditionalRestores(this, core::mem::transmute_copy(&pbadditionalrestores)).into()
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnewtarget: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetNewTargetCount(this, core::mem::transmute_copy(&pcnewtarget)).into()
        }
        unsafe extern "system" fn GetNewTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponent_Impl::GetNewTarget(this, core::mem::transmute_copy(&inewtarget)) {
                Ok(ok__) => {
                    core::ptr::write(ppfiledesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilename: windows_core::PCWSTR, wszsourcerangelist: windows_core::PCWSTR, wszdestinationpath: windows_core::PCWSTR, wszdestinationfilename: windows_core::PCWSTR, wszdestinationrangelist: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::AddDirectedTarget(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilename), core::mem::transmute(&wszsourcerangelist), core::mem::transmute(&wszdestinationpath), core::mem::transmute(&wszdestinationfilename), core::mem::transmute(&wszdestinationrangelist)).into()
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdirectedtarget: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetDirectedTargetCount(this, core::mem::transmute_copy(&pcdirectedtarget)).into()
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsourcefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsourcerangelist: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationrangelist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetDirectedTarget(this, core::mem::transmute_copy(&idirectedtarget), core::mem::transmute_copy(&pbstrsourcepath), core::mem::transmute_copy(&pbstrsourcefilename), core::mem::transmute_copy(&pbstrsourcerangelist), core::mem::transmute_copy(&pbstrdestinationpath), core::mem::transmute_copy(&pbstrdestinationfilename), core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszrestoremetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetRestoreMetadata(this, core::mem::transmute(&wszrestoremetadata)).into()
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoremetadata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetRestoreMetadata(this, core::mem::transmute_copy(&pbstrrestoremetadata)).into()
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetRestoreTarget(this, core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetRestoreTarget(this, core::mem::transmute_copy(&ptarget)).into()
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszprerestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetPreRestoreFailureMsg(this, core::mem::transmute(&wszprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprerestorefailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetPreRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpostrestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetPostRestoreFailureMsg(this, core::mem::transmute(&wszpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostrestorefailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetPostRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetBackupStamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::SetBackupStamp(this, core::mem::transmute(&wszbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupStamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetPreviousBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupoptions: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetBackupOptions(this, core::mem::transmute_copy(&pbstrbackupoptions)).into()
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoreoptions: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetRestoreOptions(this, core::mem::transmute_copy(&pbstrrestoreoptions)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetRestoreSubcomponentCount(this, core::mem::transmute_copy(&pcrestoresubcomponent)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrcomponentname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbrepair: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetRestoreSubcomponent(this, core::mem::transmute_copy(&icomponent), core::mem::transmute_copy(&pbstrlogicalpath), core::mem::transmute_copy(&pbstrcomponentname), core::mem::transmute_copy(&pbrepair)).into()
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetFileRestoreStatus(this, core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::AddDifferencedFilesByLastModifyTime(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&ftlastmodifytime)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::AddDifferencedFilesByLastModifyLSN(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&bstrlsnstring)).into()
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdifferencedfiles: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetDifferencedFilesCount(this, core::mem::transmute_copy(&pcdifferencedfiles)).into()
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrfilespec: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponent_Impl::GetDifferencedFile(this, core::mem::transmute_copy(&idifferencedfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilespec), core::mem::transmute_copy(&pbrecursive), core::mem::transmute_copy(&pbstrlsnstring), core::mem::transmute_copy(&pftlastmodifytime)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentType: GetComponentType::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, Impl, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, Impl, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, Impl, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, Impl, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, Impl, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, Impl, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, Impl, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, Impl, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, Impl, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, Impl, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, Impl, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, Impl, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, Impl, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, Impl, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, Impl, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, Impl, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, Impl, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, Impl, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, Impl, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, Impl, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, Impl, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, Impl, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, Impl, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, Impl, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
pub trait IVssComponentEx_Impl: Sized + IVssComponent_Impl {
    fn SetPrepareForBackupFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPostSnapshotFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool>;
    fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IVssComponentEx {}
impl IVssComponentEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>() -> IVssComponentEx_Vtbl {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponentEx_Impl::SetPrepareForBackupFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponentEx_Impl::SetPostSnapshotFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponentEx_Impl::GetPrepareForBackupFailureMsg(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfailuremsg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponentEx_Impl::GetPostSnapshotFailureMsg(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfailuremsg, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbauth: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponentEx_Impl::GetAuthoritativeRestore(this) {
                Ok(ok__) => {
                    core::ptr::write(pbauth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRollForward<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponentEx_Impl::GetRollForward(this, core::mem::transmute_copy(&prolltype), core::mem::transmute_copy(&pbstrpoint)).into()
        }
        unsafe extern "system" fn GetRestoreName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssComponentEx_Impl::GetRestoreName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IVssComponent_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, Impl, OFFSET>,
            GetRollForward: GetRollForward::<Identity, Impl, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
pub trait IVssComponentEx2_Impl: Sized + IVssComponentEx_Impl {
    fn SetFailure(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssComponentEx2 {}
impl IVssComponentEx2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: isize>() -> IVssComponentEx2_Vtbl {
        unsafe extern "system" fn SetFailure<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponentEx2_Impl::SetFailure(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&hrapplication), core::mem::transmute(&wszapplicationmessage), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetFailure<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwreserved: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssComponentEx2_Impl::GetFailure(this, core::mem::transmute_copy(&phr), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self {
            base__: IVssComponentEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFailure: SetFailure::<Identity, Impl, OFFSET>,
            GetFailure: GetFailure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx2 as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID || iid == &<IVssComponentEx as windows_core::Interface>::IID
    }
}
pub trait IVssCreateExpressWriterMetadata_Impl: Sized {
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &windows_core::GUID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IVssCreateExpressWriterMetadata {}
impl IVssCreateExpressWriterMetadata_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>() -> IVssCreateExpressWriterMetadata_Vtbl {
        unsafe extern "system" fn AddExcludeFiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: windows_core::GUID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssCreateExpressWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn SaveAsXML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssCreateExpressWriterMetadata_Impl::SaveAsXML(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrxml, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Identity, Impl, OFFSET>,
            AddComponent: AddComponent::<Identity, Impl, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, Impl, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, Impl, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, Impl, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, Impl, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IVssCreateWriterMetadata_Impl: Sized {
    fn AddIncludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddDatabaseFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddDatabaseLogFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>;
    fn AddAlternateLocationMapping(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilespec: &windows_core::PCWSTR, brecursive: u8, wszdestination: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &windows_core::GUID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn GetDocument(&self) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument>;
    fn SaveAsXML(&self, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata_Vtbl {
    pub const fn new<Impl: IVssCreateWriterMetadata_Impl>() -> IVssCreateWriterMetadata_Vtbl {
        unsafe extern "system" fn AddIncludeFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddIncludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation)).into()
        }
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
        }
        unsafe extern "system" fn AddDatabaseFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddDatabaseFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddDatabaseLogFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilespec: windows_core::PCWSTR, brecursive: u8, wszdestination: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddAlternateLocationMapping(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszdestination)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: windows_core::GUID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn GetDocument<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match IVssCreateWriterMetadata_Impl::GetDocument(this) {
                Ok(ok__) => {
                    core::ptr::write(pdoc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssCreateWriterMetadata_Impl::SaveAsXML(this, core::mem::transmute_copy(&pbstrxml)).into()
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Impl>,
            AddExcludeFiles: AddExcludeFiles::<Impl>,
            AddComponent: AddComponent::<Impl>,
            AddDatabaseFiles: AddDatabaseFiles::<Impl>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Impl>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Impl>,
            SetRestoreMethod: SetRestoreMethod::<Impl>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Impl>,
            AddComponentDependency: AddComponentDependency::<Impl>,
            SetBackupSchema: SetBackupSchema::<Impl>,
            GetDocument: GetDocument::<Impl>,
            SaveAsXML: SaveAsXML::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
struct IVssCreateWriterMetadata_ImplVtbl<T: IVssCreateWriterMetadata_Impl>(std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<T: IVssCreateWriterMetadata_Impl> IVssCreateWriterMetadata_ImplVtbl<T> {
    const VTABLE: IVssCreateWriterMetadata_Vtbl = IVssCreateWriterMetadata_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata {
    pub fn new<'a, T: IVssCreateWriterMetadata_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssCreateWriterMetadata_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt_Impl: Sized {
    fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()>;
    fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()>;
    fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForSnapshot(&self, snapshotid: &windows_core::GUID) -> windows_core::Result<IVssEnumMgmtObject>;
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt {}
impl IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
        unsafe extern "system" fn AddDiffArea<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt_Impl::AddDiffArea(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt_Impl::ChangeDiffAreaMaximumSize(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryVolumesSupportedForDiffAreas(this, core::mem::transmute_copy(&pwszoriginalvolumename)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasOnVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForSnapshot(this, core::mem::transmute(&snapshotid)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddDiffArea: AddDiffArea::<Identity, Impl, OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Identity, Impl, OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt2_Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt_Impl {
    fn ChangeDiffAreaMaximumSizeEx(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::Result<()>;
    fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> windows_core::Result<IVssAsync>;
    fn SetSnapshotPriority(&self, idsnapshot: &windows_core::GUID, priority: u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt2 {}
impl IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::ChangeDiffAreaMaximumSizeEx(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace), core::mem::transmute_copy(&bvolatile)).into()
        }
        unsafe extern "system" fn MigrateDiffAreas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::MigrateDiffAreas(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&pwsznewdiffareavolumename)).into()
        }
        unsafe extern "system" fn QueryMigrationStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssDifferentialSoftwareSnapshotMgmt2_Impl::QueryMigrationStatus(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename)) {
                Ok(ok__) => {
                    core::ptr::write(ppasync, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshot: windows_core::GUID, priority: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::SetSnapshotPriority(this, core::mem::transmute(&idsnapshot), core::mem::transmute_copy(&priority)).into()
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Identity, Impl, OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Identity, Impl, OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Identity, Impl, OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt3_Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt2_Impl {
    fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::Result<()>;
    fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::Result<()>;
    fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> windows_core::Result<()>;
    fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> windows_core::Result<()>;
    fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: &windows_core::GUID, idsnapshotyounger: &windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt3 {}
impl IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
        unsafe extern "system" fn SetVolumeProtectLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::SetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::GetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::ClearVolumeProtectFault(this, core::mem::transmute_copy(&pwszvolumename)).into()
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdiffareavolumename: *const u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::DeleteUnusedDiffAreas(this, core::mem::transmute_copy(&pwszdiffareavolumename)).into()
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshotolder: windows_core::GUID, idsnapshotyounger: windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::QuerySnapshotDeltaBitmap(this, core::mem::transmute(&idsnapshotolder), core::mem::transmute(&idsnapshotyounger), core::mem::transmute_copy(&pcblocksizeperbit), core::mem::transmute_copy(&pcbitmaplength), core::mem::transmute_copy(&ppbbitmap)).into()
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Identity, Impl, OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Identity, Impl, OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt3 as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as windows_core::Interface>::IID
    }
}
pub trait IVssEnumMgmtObject_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: *mut Option<IVssEnumMgmtObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssEnumMgmtObject {}
impl IVssEnumMgmtObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>() -> IVssEnumMgmtObject_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumMgmtObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumMgmtObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumMgmtObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumMgmtObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssEnumMgmtObject as windows_core::Interface>::IID
    }
}
pub trait IVssEnumObject_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: *mut Option<IVssEnumObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssEnumObject {}
impl IVssEnumObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: isize>() -> IVssEnumObject_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssEnumObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssEnumObject as windows_core::Interface>::IID
    }
}
pub trait IVssExpressWriter_Impl: Sized {
    fn CreateMetadata(&self, writerid: &windows_core::GUID, writername: &windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(&self, metadata: &windows_core::PCWSTR, reserved: u32) -> windows_core::Result<()>;
    fn Register(&self) -> windows_core::Result<()>;
    fn Unregister(&self, writerid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssExpressWriter {}
impl IVssExpressWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: isize>() -> IVssExpressWriter_Vtbl {
        unsafe extern "system" fn CreateMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID, writername: windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssExpressWriter_Impl::CreateMetadata(this, core::mem::transmute(&writerid), core::mem::transmute(&writername), core::mem::transmute_copy(&usagetype), core::mem::transmute_copy(&versionmajor), core::mem::transmute_copy(&versionminor), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(ppmetadata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: windows_core::PCWSTR, reserved: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssExpressWriter_Impl::LoadMetadata(this, core::mem::transmute(&metadata), core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssExpressWriter_Impl::Register(this).into()
        }
        unsafe extern "system" fn Unregister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssExpressWriter_Impl::Unregister(this, core::mem::transmute(&writerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadata: CreateMetadata::<Identity, Impl, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExpressWriter as windows_core::Interface>::IID
    }
}
pub trait IVssFileShareSnapshotProvider_Impl: Sized {
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&self, sourceobjectid: &windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: &windows_core::GUID) -> windows_core::Result<()>;
    fn IsPathSupported(&self, pwszsharepath: *const u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()>;
    fn SetSnapshotProperty(&self, snapshotid: &windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssFileShareSnapshotProvider {}
impl IVssFileShareSnapshotProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssFileShareSnapshotProvider_Vtbl {
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssFileShareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&lnewcontext), core::mem::transmute(&providerid)).into()
        }
        unsafe extern "system" fn IsPathSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssFileShareSnapshotProvider_Impl::IsPathSupported(this, core::mem::transmute_copy(&pwszsharepath)) {
                Ok(ok__) => {
                    core::ptr::write(pbsupportedbythisprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::IsPathSnapshotted(this, core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssFileShareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsPathSupported: IsPathSupported::<Identity, Impl, OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssFileShareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
pub trait IVssHardwareSnapshotProvider_Impl: Sized {
    fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn LocateLuns(&self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl windows_core::RuntimeName for IVssHardwareSnapshotProvider {}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl IVssHardwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssHardwareSnapshotProvider_Vtbl {
        unsafe extern "system" fn AreLunsSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::AreLunsSupported(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&rgwszdevices), core::mem::transmute_copy(&pluninformation), core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn FillInLunInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::FillInLunInfo(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pluninfo), core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgluninformation)).into()
        }
        unsafe extern "system" fn GetTargetLuns<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::GetTargetLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgsourceluns), core::mem::transmute_copy(&rgdestinationluns)).into()
        }
        unsafe extern "system" fn LocateLuns<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::LocateLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgsourceluns)).into()
        }
        unsafe extern "system" fn OnLunEmpty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProvider_Impl::OnLunEmpty(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pinformation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AreLunsSupported: AreLunsSupported::<Identity, Impl, OFFSET>,
            FillInLunInfo: FillInLunInfo::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            GetTargetLuns: GetTargetLuns::<Identity, Impl, OFFSET>,
            LocateLuns: LocateLuns::<Identity, Impl, OFFSET>,
            OnLunEmpty: OnLunEmpty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
pub trait IVssHardwareSnapshotProviderEx_Impl: Sized + IVssHardwareSnapshotProvider_Impl {
    fn GetProviderCapabilities(&self) -> windows_core::Result<u64>;
    fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::Result<()>;
    fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<IVssAsync>;
    fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl windows_core::RuntimeName for IVssHardwareSnapshotProviderEx {}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl IVssHardwareSnapshotProviderEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>() -> IVssHardwareSnapshotProviderEx_Vtbl {
        unsafe extern "system" fn GetProviderCapabilities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssHardwareSnapshotProviderEx_Impl::GetProviderCapabilities(this) {
                Ok(ok__) => {
                    core::ptr::write(plloriginalcapabilitymask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProviderEx_Impl::OnLunStateChange(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ResyncLuns<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssHardwareSnapshotProviderEx_Impl::ResyncLuns(this, core::mem::transmute_copy(&psourceluns), core::mem::transmute_copy(&ptargetluns), core::mem::transmute_copy(&dwcount)) {
                Ok(ok__) => {
                    core::ptr::write(ppasync, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReuseLuns<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssHardwareSnapshotProviderEx_Impl::OnReuseLuns(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount)).into()
        }
        Self {
            base__: IVssHardwareSnapshotProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProviderCapabilities: GetProviderCapabilities::<Identity, Impl, OFFSET>,
            OnLunStateChange: OnLunStateChange::<Identity, Impl, OFFSET>,
            ResyncLuns: ResyncLuns::<Identity, Impl, OFFSET>,
            OnReuseLuns: OnReuseLuns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProviderEx as windows_core::Interface>::IID || iid == &<IVssHardwareSnapshotProvider as windows_core::Interface>::IID
    }
}
pub trait IVssProviderCreateSnapshotSet_Impl: Sized {
    fn EndPrepareSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PreCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn CommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PostCommitSnapshots(&self, snapshotsetid: &windows_core::GUID, lsnapshotscount: i32) -> windows_core::Result<()>;
    fn PreFinalCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PostFinalCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn AbortSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssProviderCreateSnapshotSet {}
impl IVssProviderCreateSnapshotSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>() -> IVssProviderCreateSnapshotSet_Vtbl {
        unsafe extern "system" fn EndPrepareSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::EndPrepareSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PreCommitSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::PreCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn CommitSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::CommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostCommitSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, lsnapshotscount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::PostCommitSnapshots(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&lsnapshotscount)).into()
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::PreFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::PostFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn AbortSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderCreateSnapshotSet_Impl::AbortSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EndPrepareSnapshots: EndPrepareSnapshots::<Identity, Impl, OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Identity, Impl, OFFSET>,
            CommitSnapshots: CommitSnapshots::<Identity, Impl, OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Identity, Impl, OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            AbortSnapshots: AbortSnapshots::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssProviderCreateSnapshotSet as windows_core::Interface>::IID
    }
}
pub trait IVssProviderNotifications_Impl: Sized {
    fn OnLoad(&self, pcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnUnload(&self, bforceunload: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssProviderNotifications {}
impl IVssProviderNotifications_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>() -> IVssProviderNotifications_Vtbl {
        unsafe extern "system" fn OnLoad<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderNotifications_Impl::OnLoad(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssProviderNotifications_Impl::OnUnload(this, core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssProviderNotifications as windows_core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt_Impl: Sized {
    fn GetProviderMgmtInterface(&self, providerid: &windows_core::GUID, interfaceid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryVolumesSupportedForSnapshots(&self, providerid: &windows_core::GUID, lcontext: i32) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: &windows_core::GUID) -> windows_core::Result<IVssEnumObject>;
}
impl windows_core::RuntimeName for IVssSnapshotMgmt {}
impl IVssSnapshotMgmt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>() -> IVssSnapshotMgmt_Vtbl {
        unsafe extern "system" fn GetProviderMgmtInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, interfaceid: *const windows_core::GUID, ppitf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSnapshotMgmt_Impl::GetProviderMgmtInterface(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&interfaceid)) {
                Ok(ok__) => {
                    core::ptr::write(ppitf, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSnapshotMgmt_Impl::QueryVolumesSupportedForSnapshots(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, providerid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSnapshotMgmt_Impl::QuerySnapshotsByVolume(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute(&providerid)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Identity, Impl, OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt as windows_core::Interface>::IID
    }
}
pub trait IVssSnapshotMgmt2_Impl: Sized {
    fn GetMinDiffAreaSize(&self) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IVssSnapshotMgmt2 {}
impl IVssSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: isize>() -> IVssSnapshotMgmt2_Vtbl {
        unsafe extern "system" fn GetMinDiffAreaSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllmindiffareasize: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSnapshotMgmt2_Impl::GetMinDiffAreaSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pllmindiffareasize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMinDiffAreaSize: GetMinDiffAreaSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt2 as windows_core::Interface>::IID
    }
}
pub trait IVssSoftwareSnapshotProvider_Impl: Sized {
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&self, sourceobjectid: &windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::Result<()>;
    fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()>;
    fn SetSnapshotProperty(&self, snapshotid: &windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn RevertToSnapshot(&self, snapshotid: &windows_core::GUID) -> windows_core::Result<()>;
    fn QueryRevertStatus(&self, pwszvolume: *const u16) -> windows_core::Result<IVssAsync>;
}
impl windows_core::RuntimeName for IVssSoftwareSnapshotProvider {}
impl IVssSoftwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>() -> IVssSoftwareSnapshotProvider_Vtbl {
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSoftwareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&lnewcontext)).into()
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSoftwareSnapshotProvider_Impl::IsVolumeSupported(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    core::ptr::write(pbsupportedbythisprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::IsVolumeSnapshotted(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssSoftwareSnapshotProvider_Impl::RevertToSnapshot(this, core::mem::transmute(&snapshotid)).into()
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssSoftwareSnapshotProvider_Impl::QueryRevertStatus(this, core::mem::transmute_copy(&pwszvolume)) {
                Ok(ok__) => {
                    core::ptr::write(ppasync, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, Impl, OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, Impl, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSoftwareSnapshotProvider as windows_core::Interface>::IID
    }
}
pub trait IVssWMDependency_Impl: Sized {
    fn GetWriterId(&self, pwriterid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetLogicalPath(&self, pbstrlogicalpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetComponentName(&self, pbstrcomponentname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssWMDependency {}
impl IVssWMDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: isize>() -> IVssWMDependency_Vtbl {
        unsafe extern "system" fn GetWriterId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwriterid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssWMDependency_Impl::GetWriterId(this, core::mem::transmute_copy(&pwriterid)).into()
        }
        unsafe extern "system" fn GetLogicalPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlogicalpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssWMDependency_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrlogicalpath)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomponentname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVssWMDependency_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrcomponentname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterId: GetWriterId::<Identity, Impl, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMDependency as windows_core::Interface>::IID
    }
}
pub trait IVssWMFiledesc_Impl: Sized {
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRecursive(&self) -> windows_core::Result<bool>;
    fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackupTypeMask(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVssWMFiledesc {}
impl IVssWMFiledesc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>() -> IVssWMFiledesc_Vtbl {
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssWMFiledesc_Impl::GetPath(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilespec<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilespec: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssWMFiledesc_Impl::GetFilespec(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfilespec, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecursive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrecursive: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssWMFiledesc_Impl::GetRecursive(this) {
                Ok(ok__) => {
                    core::ptr::write(pbrecursive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstralternatelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssWMFiledesc_Impl::GetAlternateLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstralternatelocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtypemask: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVssWMFiledesc_Impl::GetBackupTypeMask(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwtypemask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetFilespec: GetFilespec::<Identity, Impl, OFFSET>,
            GetRecursive: GetRecursive::<Identity, Impl, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, Impl, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMFiledesc as windows_core::Interface>::IID
    }
}
pub trait IVssWriterComponents_Impl: Sized {
    fn GetComponentCount(&self, pccomponents: *mut u32) -> windows_core::Result<()>;
    fn GetWriterInfo(&self, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent>;
}
impl IVssWriterComponents_Vtbl {
    pub const fn new<Impl: IVssWriterComponents_Impl>() -> IVssWriterComponents_Vtbl {
        unsafe extern "system" fn GetComponentCount<Impl: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pccomponents: *mut u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssWriterComponents_Impl::GetComponentCount(this, core::mem::transmute_copy(&pccomponents)).into()
        }
        unsafe extern "system" fn GetWriterInfo<Impl: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IVssWriterComponents_Impl::GetWriterInfo(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter)).into()
        }
        unsafe extern "system" fn GetComponent<Impl: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match IVssWriterComponents_Impl::GetComponent(this, core::mem::transmute_copy(&icomponent)) {
                Ok(ok__) => {
                    core::ptr::write(ppcomponent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { GetComponentCount: GetComponentCount::<Impl>, GetWriterInfo: GetWriterInfo::<Impl>, GetComponent: GetComponent::<Impl> }
    }
}
#[doc(hidden)]
struct IVssWriterComponents_ImplVtbl<T: IVssWriterComponents_Impl>(std::marker::PhantomData<T>);
impl<T: IVssWriterComponents_Impl> IVssWriterComponents_ImplVtbl<T> {
    const VTABLE: IVssWriterComponents_Vtbl = IVssWriterComponents_Vtbl::new::<T>();
}
impl IVssWriterComponents {
    pub fn new<'a, T: IVssWriterComponents_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssWriterComponents_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
