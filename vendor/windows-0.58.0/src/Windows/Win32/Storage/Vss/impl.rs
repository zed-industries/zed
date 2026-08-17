pub trait IVssAdmin_Impl: Sized {
    fn RegisterProvider(&self, pproviderid: &windows_core::GUID, classid: &windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, providerid: &windows_core::GUID) -> windows_core::Result<()>;
    fn QueryProviders(&self) -> windows_core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVssAdmin {}
impl IVssAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssAdmin_Vtbl
    where
        Identity: IVssAdmin_Impl,
    {
        unsafe extern "system" fn RegisterProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, classid: windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAdmin_Impl::RegisterProvider(this, core::mem::transmute(&pproviderid), core::mem::transmute(&classid), core::mem::transmute_copy(&pwszprovidername), core::mem::transmute_copy(&eprovidertype), core::mem::transmute_copy(&pwszproviderversion), core::mem::transmute(&providerversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAdmin_Impl::UnregisterProvider(this, core::mem::transmute(&providerid)).into()
        }
        unsafe extern "system" fn QueryProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssAdmin_Impl::QueryProviders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAdmin_Impl::AbortAllSnapshotsInProgress(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, OFFSET>,
            QueryProviders: QueryProviders::<Identity, OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssAdminEx_Vtbl
    where
        Identity: IVssAdminEx_Impl,
    {
        unsafe extern "system" fn GetProviderCapability<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVssAdminEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssAdminEx_Impl::GetProviderCapability(this, core::mem::transmute(&pproviderid)) {
                Ok(ok__) => {
                    plloriginalcapabilitymask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, plcontext: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVssAdminEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssAdminEx_Impl::GetProviderContext(this, core::mem::transmute(&providerid)) {
                Ok(ok__) => {
                    plcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32) -> windows_core::HRESULT
        where
            Identity: IVssAdminEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAdminEx_Impl::SetProviderContext(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)).into()
        }
        Self {
            base__: IVssAdmin_Vtbl::new::<Identity, OFFSET>(),
            GetProviderCapability: GetProviderCapability::<Identity, OFFSET>,
            GetProviderContext: GetProviderContext::<Identity, OFFSET>,
            SetProviderContext: SetProviderContext::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssAsync_Vtbl
    where
        Identity: IVssAsync_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAsync_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT
        where
            Identity: IVssAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAsync_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVssAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssAsync_Impl::QueryStatus(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&preserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            QueryStatus: QueryStatus::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssComponent_Vtbl
    where
        Identity: IVssComponent_Impl,
    {
        unsafe extern "system" fn GetLogicalPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrpath)).into()
        }
        unsafe extern "system" fn GetComponentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetComponentType(this, core::mem::transmute_copy(&pct)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsucceeded: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetBackupSucceeded(this, core::mem::transmute_copy(&pbsucceeded)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmappings: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetAlternateLocationMappingCount(this, core::mem::transmute_copy(&pcmappings)).into()
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponent_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping)) {
                Ok(ok__) => {
                    ppfiledesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdata: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetBackupMetadata(this, core::mem::transmute(&wszdata)).into()
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetBackupMetadata(this, core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn AddPartialFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, wszranges: windows_core::PCWSTR, wszmetadata: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::AddPartialFile(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute(&wszranges), core::mem::transmute(&wszmetadata)).into()
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpartialfiles: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetPartialFileCount(this, core::mem::transmute_copy(&pcpartialfiles)).into()
        }
        unsafe extern "system" fn GetPartialFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrrange: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrmetadata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetPartialFile(this, core::mem::transmute_copy(&ipartialfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pbstrrange), core::mem::transmute_copy(&pbstrmetadata)).into()
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbselectedforrestore: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::IsSelectedForRestore(this, core::mem::transmute_copy(&pbselectedforrestore)).into()
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbadditionalrestores: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetAdditionalRestores(this, core::mem::transmute_copy(&pbadditionalrestores)).into()
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnewtarget: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetNewTargetCount(this, core::mem::transmute_copy(&pcnewtarget)).into()
        }
        unsafe extern "system" fn GetNewTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponent_Impl::GetNewTarget(this, core::mem::transmute_copy(&inewtarget)) {
                Ok(ok__) => {
                    ppfiledesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilename: windows_core::PCWSTR, wszsourcerangelist: windows_core::PCWSTR, wszdestinationpath: windows_core::PCWSTR, wszdestinationfilename: windows_core::PCWSTR, wszdestinationrangelist: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::AddDirectedTarget(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilename), core::mem::transmute(&wszsourcerangelist), core::mem::transmute(&wszdestinationpath), core::mem::transmute(&wszdestinationfilename), core::mem::transmute(&wszdestinationrangelist)).into()
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdirectedtarget: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetDirectedTargetCount(this, core::mem::transmute_copy(&pcdirectedtarget)).into()
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsourcefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsourcerangelist: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdestinationrangelist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetDirectedTarget(this, core::mem::transmute_copy(&idirectedtarget), core::mem::transmute_copy(&pbstrsourcepath), core::mem::transmute_copy(&pbstrsourcefilename), core::mem::transmute_copy(&pbstrsourcerangelist), core::mem::transmute_copy(&pbstrdestinationpath), core::mem::transmute_copy(&pbstrdestinationfilename), core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszrestoremetadata: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetRestoreMetadata(this, core::mem::transmute(&wszrestoremetadata)).into()
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoremetadata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetRestoreMetadata(this, core::mem::transmute_copy(&pbstrrestoremetadata)).into()
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: VSS_RESTORE_TARGET) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetRestoreTarget(this, core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetRestoreTarget(this, core::mem::transmute_copy(&ptarget)).into()
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszprerestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetPreRestoreFailureMsg(this, core::mem::transmute(&wszprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprerestorefailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetPreRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpostrestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetPostRestoreFailureMsg(this, core::mem::transmute(&wszpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostrestorefailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetPostRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into()
        }
        unsafe extern "system" fn SetBackupStamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::SetBackupStamp(this, core::mem::transmute(&wszbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupStamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetPreviousBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
        }
        unsafe extern "system" fn GetBackupOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupoptions: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetBackupOptions(this, core::mem::transmute_copy(&pbstrbackupoptions)).into()
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoreoptions: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetRestoreOptions(this, core::mem::transmute_copy(&pbstrrestoreoptions)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetRestoreSubcomponentCount(this, core::mem::transmute_copy(&pcrestoresubcomponent)).into()
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrcomponentname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbrepair: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetRestoreSubcomponent(this, core::mem::transmute_copy(&icomponent), core::mem::transmute_copy(&pbstrlogicalpath), core::mem::transmute_copy(&pbstrcomponentname), core::mem::transmute_copy(&pbrepair)).into()
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetFileRestoreStatus(this, core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::AddDifferencedFilesByLastModifyTime(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&ftlastmodifytime)).into()
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::AddDifferencedFilesByLastModifyLSN(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&bstrlsnstring)).into()
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdifferencedfiles: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetDifferencedFilesCount(this, core::mem::transmute_copy(&pcdifferencedfiles)).into()
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrfilespec: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut core::mem::MaybeUninit<windows_core::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IVssComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponent_Impl::GetDifferencedFile(this, core::mem::transmute_copy(&idifferencedfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilespec), core::mem::transmute_copy(&pbrecursive), core::mem::transmute_copy(&pbstrlsnstring), core::mem::transmute_copy(&pftlastmodifytime)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentType: GetComponentType::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssComponentEx_Vtbl
    where
        Identity: IVssComponentEx_Impl,
    {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponentEx_Impl::SetPrepareForBackupFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponentEx_Impl::SetPostSnapshotFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponentEx_Impl::GetPrepareForBackupFailureMsg(this) {
                Ok(ok__) => {
                    pbstrfailuremsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponentEx_Impl::GetPostSnapshotFailureMsg(this) {
                Ok(ok__) => {
                    pbstrfailuremsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbauth: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponentEx_Impl::GetAuthoritativeRestore(this) {
                Ok(ok__) => {
                    pbauth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRollForward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponentEx_Impl::GetRollForward(this, core::mem::transmute_copy(&prolltype), core::mem::transmute_copy(&pbstrpoint)).into()
        }
        unsafe extern "system" fn GetRestoreName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssComponentEx_Impl::GetRestoreName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IVssComponent_Vtbl::new::<Identity, OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, OFFSET>,
            GetRollForward: GetRollForward::<Identity, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssComponentEx2_Vtbl
    where
        Identity: IVssComponentEx2_Impl,
    {
        unsafe extern "system" fn SetFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponentEx2_Impl::SetFailure(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&hrapplication), core::mem::transmute(&wszapplicationmessage), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssComponentEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssComponentEx2_Impl::GetFailure(this, core::mem::transmute_copy(&phr), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IVssComponentEx_Vtbl::new::<Identity, OFFSET>(), SetFailure: SetFailure::<Identity, OFFSET>, GetFailure: GetFailure::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssCreateExpressWriterMetadata_Vtbl
    where
        Identity: IVssCreateExpressWriterMetadata_Impl,
    {
        unsafe extern "system" fn AddExcludeFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: windows_core::GUID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssCreateExpressWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn SaveAsXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssCreateExpressWriterMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssCreateExpressWriterMetadata_Impl::SaveAsXML(this) {
                Ok(ok__) => {
                    pbstrxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Identity, OFFSET>,
            AddComponent: AddComponent::<Identity, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
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
                    pdoc.write(core::mem::transmute(ok__));
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt_Vtbl
    where
        Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
    {
        unsafe extern "system" fn AddDiffArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt_Impl::AddDiffArea(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt_Impl::ChangeDiffAreaMaximumSize(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryVolumesSupportedForDiffAreas(this, core::mem::transmute_copy(&pwszoriginalvolumename)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasOnVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForSnapshot(this, core::mem::transmute(&snapshotid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddDiffArea: AddDiffArea::<Identity, OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Identity, OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Identity, OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Identity, OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Identity, OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt2_Vtbl
    where
        Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl,
    {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::ChangeDiffAreaMaximumSizeEx(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace), core::mem::transmute_copy(&bvolatile)).into()
        }
        unsafe extern "system" fn MigrateDiffAreas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::MigrateDiffAreas(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&pwsznewdiffareavolumename)).into()
        }
        unsafe extern "system" fn QueryMigrationStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssDifferentialSoftwareSnapshotMgmt2_Impl::QueryMigrationStatus(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshot: windows_core::GUID, priority: u8) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt2_Impl::SetSnapshotPriority(this, core::mem::transmute(&idsnapshot), core::mem::transmute_copy(&priority)).into()
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl::new::<Identity, OFFSET>(),
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Identity, OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Identity, OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Identity, OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssDifferentialSoftwareSnapshotMgmt3_Vtbl
    where
        Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
    {
        unsafe extern "system" fn SetVolumeProtectLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::SetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::GetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::ClearVolumeProtectFault(this, core::mem::transmute_copy(&pwszvolumename)).into()
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdiffareavolumename: *const u16) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::DeleteUnusedDiffAreas(this, core::mem::transmute_copy(&pwszdiffareavolumename)).into()
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshotolder: windows_core::GUID, idsnapshotyounger: windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssDifferentialSoftwareSnapshotMgmt3_Impl::QuerySnapshotDeltaBitmap(this, core::mem::transmute(&idsnapshotolder), core::mem::transmute(&idsnapshotyounger), core::mem::transmute_copy(&pcblocksizeperbit), core::mem::transmute_copy(&pcbitmaplength), core::mem::transmute_copy(&ppbbitmap)).into()
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl::new::<Identity, OFFSET>(),
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Identity, OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Identity, OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Identity, OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Identity, OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssEnumMgmtObject_Vtbl
    where
        Identity: IVssEnumMgmtObject_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssEnumMgmtObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumMgmtObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IVssEnumMgmtObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumMgmtObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssEnumMgmtObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumMgmtObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssEnumMgmtObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumMgmtObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssEnumObject_Vtbl
    where
        Identity: IVssEnumObject_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssEnumObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IVssEnumObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssEnumObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssEnumObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssEnumObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssExpressWriter_Vtbl
    where
        Identity: IVssExpressWriter_Impl,
    {
        unsafe extern "system" fn CreateMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID, writername: windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssExpressWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssExpressWriter_Impl::CreateMetadata(this, core::mem::transmute(&writerid), core::mem::transmute(&writername), core::mem::transmute_copy(&usagetype), core::mem::transmute_copy(&versionmajor), core::mem::transmute_copy(&versionminor), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: windows_core::PCWSTR, reserved: u32) -> windows_core::HRESULT
        where
            Identity: IVssExpressWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssExpressWriter_Impl::LoadMetadata(this, core::mem::transmute(&metadata), core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssExpressWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssExpressWriter_Impl::Register(this).into()
        }
        unsafe extern "system" fn Unregister<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssExpressWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssExpressWriter_Impl::Unregister(this, core::mem::transmute(&writerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadata: CreateMetadata::<Identity, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssFileShareSnapshotProvider_Vtbl
    where
        Identity: IVssFileShareSnapshotProvider_Impl,
    {
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssFileShareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&lnewcontext), core::mem::transmute(&providerid)).into()
        }
        unsafe extern "system" fn IsPathSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssFileShareSnapshotProvider_Impl::IsPathSupported(this, core::mem::transmute_copy(&pwszsharepath)) {
                Ok(ok__) => {
                    pbsupportedbythisprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::IsPathSnapshotted(this, core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IVssFileShareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssFileShareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            IsPathSupported: IsPathSupported::<Identity, OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Identity, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssHardwareSnapshotProvider_Vtbl
    where
        Identity: IVssHardwareSnapshotProvider_Impl,
    {
        unsafe extern "system" fn AreLunsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::AreLunsSupported(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&rgwszdevices), core::mem::transmute_copy(&pluninformation), core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn FillInLunInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::FillInLunInfo(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pluninfo), core::mem::transmute_copy(&pbissupported)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgluninformation)).into()
        }
        unsafe extern "system" fn GetTargetLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::GetTargetLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgsourceluns), core::mem::transmute_copy(&rgdestinationluns)).into()
        }
        unsafe extern "system" fn LocateLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::LocateLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgsourceluns)).into()
        }
        unsafe extern "system" fn OnLunEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProvider_Impl::OnLunEmpty(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pinformation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AreLunsSupported: AreLunsSupported::<Identity, OFFSET>,
            FillInLunInfo: FillInLunInfo::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            GetTargetLuns: GetTargetLuns::<Identity, OFFSET>,
            LocateLuns: LocateLuns::<Identity, OFFSET>,
            OnLunEmpty: OnLunEmpty::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssHardwareSnapshotProviderEx_Vtbl
    where
        Identity: IVssHardwareSnapshotProviderEx_Impl,
    {
        unsafe extern "system" fn GetProviderCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProviderEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssHardwareSnapshotProviderEx_Impl::GetProviderCapabilities(this) {
                Ok(ok__) => {
                    plloriginalcapabilitymask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLunStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProviderEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProviderEx_Impl::OnLunStateChange(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ResyncLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProviderEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssHardwareSnapshotProviderEx_Impl::ResyncLuns(this, core::mem::transmute_copy(&psourceluns), core::mem::transmute_copy(&ptargetluns), core::mem::transmute_copy(&dwcount)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReuseLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::HRESULT
        where
            Identity: IVssHardwareSnapshotProviderEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssHardwareSnapshotProviderEx_Impl::OnReuseLuns(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount)).into()
        }
        Self {
            base__: IVssHardwareSnapshotProvider_Vtbl::new::<Identity, OFFSET>(),
            GetProviderCapabilities: GetProviderCapabilities::<Identity, OFFSET>,
            OnLunStateChange: OnLunStateChange::<Identity, OFFSET>,
            ResyncLuns: ResyncLuns::<Identity, OFFSET>,
            OnReuseLuns: OnReuseLuns::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssProviderCreateSnapshotSet_Vtbl
    where
        Identity: IVssProviderCreateSnapshotSet_Impl,
    {
        unsafe extern "system" fn EndPrepareSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::EndPrepareSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PreCommitSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::PreCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn CommitSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::CommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostCommitSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, lsnapshotscount: i32) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::PostCommitSnapshots(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&lsnapshotscount)).into()
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::PreFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::PostFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        unsafe extern "system" fn AbortSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssProviderCreateSnapshotSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderCreateSnapshotSet_Impl::AbortSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EndPrepareSnapshots: EndPrepareSnapshots::<Identity, OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Identity, OFFSET>,
            CommitSnapshots: CommitSnapshots::<Identity, OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Identity, OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Identity, OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Identity, OFFSET>,
            AbortSnapshots: AbortSnapshots::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssProviderNotifications_Vtbl
    where
        Identity: IVssProviderNotifications_Impl,
    {
        unsafe extern "system" fn OnLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssProviderNotifications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderNotifications_Impl::OnLoad(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssProviderNotifications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssProviderNotifications_Impl::OnUnload(this, core::mem::transmute_copy(&bforceunload)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLoad: OnLoad::<Identity, OFFSET>, OnUnload: OnUnload::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssSnapshotMgmt_Vtbl
    where
        Identity: IVssSnapshotMgmt_Impl,
    {
        unsafe extern "system" fn GetProviderMgmtInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, interfaceid: *const windows_core::GUID, ppitf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSnapshotMgmt_Impl::GetProviderMgmtInterface(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&interfaceid)) {
                Ok(ok__) => {
                    ppitf.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSnapshotMgmt_Impl::QueryVolumesSupportedForSnapshots(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, providerid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssSnapshotMgmt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSnapshotMgmt_Impl::QuerySnapshotsByVolume(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute(&providerid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Identity, OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Identity, OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssSnapshotMgmt2_Vtbl
    where
        Identity: IVssSnapshotMgmt2_Impl,
    {
        unsafe extern "system" fn GetMinDiffAreaSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllmindiffareasize: *mut i64) -> windows_core::HRESULT
        where
            Identity: IVssSnapshotMgmt2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSnapshotMgmt2_Impl::GetMinDiffAreaSize(this) {
                Ok(ok__) => {
                    pllmindiffareasize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMinDiffAreaSize: GetMinDiffAreaSize::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssSoftwareSnapshotProvider_Vtbl
    where
        Identity: IVssSoftwareSnapshotProvider_Impl,
    {
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSoftwareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&lnewcontext)).into()
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSoftwareSnapshotProvider_Impl::IsVolumeSupported(this, core::mem::transmute_copy(&pwszvolumename)) {
                Ok(ok__) => {
                    pbsupportedbythisprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::IsVolumeSnapshotted(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssSoftwareSnapshotProvider_Impl::RevertToSnapshot(this, core::mem::transmute(&snapshotid)).into()
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVssSoftwareSnapshotProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssSoftwareSnapshotProvider_Impl::QueryRevertStatus(this, core::mem::transmute_copy(&pwszvolume)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Identity, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssWMDependency_Vtbl
    where
        Identity: IVssWMDependency_Impl,
    {
        unsafe extern "system" fn GetWriterId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwriterid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVssWMDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssWMDependency_Impl::GetWriterId(this, core::mem::transmute_copy(&pwriterid)).into()
        }
        unsafe extern "system" fn GetLogicalPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlogicalpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssWMDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssWMDependency_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrlogicalpath)).into()
        }
        unsafe extern "system" fn GetComponentName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomponentname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssWMDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVssWMDependency_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrcomponentname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterId: GetWriterId::<Identity, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVssWMFiledesc_Vtbl
    where
        Identity: IVssWMFiledesc_Impl,
    {
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssWMFiledesc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssWMFiledesc_Impl::GetPath(this) {
                Ok(ok__) => {
                    pbstrpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilespec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilespec: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssWMFiledesc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssWMFiledesc_Impl::GetFilespec(this) {
                Ok(ok__) => {
                    pbstrfilespec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecursive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrecursive: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVssWMFiledesc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssWMFiledesc_Impl::GetRecursive(this) {
                Ok(ok__) => {
                    pbrecursive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstralternatelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IVssWMFiledesc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssWMFiledesc_Impl::GetAlternateLocation(this) {
                Ok(ok__) => {
                    pbstralternatelocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtypemask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVssWMFiledesc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVssWMFiledesc_Impl::GetBackupTypeMask(this) {
                Ok(ok__) => {
                    pdwtypemask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPath: GetPath::<Identity, OFFSET>,
            GetFilespec: GetFilespec::<Identity, OFFSET>,
            GetRecursive: GetRecursive::<Identity, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, OFFSET>,
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
                    ppcomponent.write(core::mem::transmute(ok__));
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
