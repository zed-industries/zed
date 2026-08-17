pub trait IEnumOfflineFilesItems_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesItems>;
}
impl windows_core::RuntimeName for IEnumOfflineFilesItems {}
impl IEnumOfflineFilesItems_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>() -> IEnumOfflineFilesItems_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumOfflineFilesItems_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOfflineFilesItems as windows_core::Interface>::IID
    }
}
pub trait IEnumOfflineFilesSettings_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesSettings>;
}
impl windows_core::RuntimeName for IEnumOfflineFilesSettings {}
impl IEnumOfflineFilesSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>() -> IEnumOfflineFilesSettings_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesSettings_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesSettings_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumOfflineFilesSettings_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumOfflineFilesSettings_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOfflineFilesSettings as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesCache_Impl: Sized {
    fn Synchronize(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: Option<&IOfflineFilesSyncConflictHandler>, piprogress: Option<&IOfflineFilesSyncProgress>, psyncid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteItems(&self, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: Option<&IOfflineFilesSimpleProgress>) -> windows_core::Result<()>;
    fn DeleteItemsForUser(&self, pszuser: &windows_core::PCWSTR, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: Option<&IOfflineFilesSimpleProgress>) -> windows_core::Result<()>;
    fn Pin(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: Option<&IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn Unpin(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: Option<&IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Encrypt(&self, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: Option<&IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn FindItem(&self, pszpath: &windows_core::PCWSTR, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>;
    fn FindItemEx(&self, pszpath: &windows_core::PCWSTR, pincludefilefilter: Option<&IOfflineFilesItemFilter>, pincludedirfilter: Option<&IOfflineFilesItemFilter>, pexcludefilefilter: Option<&IOfflineFilesItemFilter>, pexcludedirfilter: Option<&IOfflineFilesItemFilter>, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>;
    fn RenameItem(&self, pszpathoriginal: &windows_core::PCWSTR, pszpathnew: &windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLocation(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> windows_core::Result<()>;
    fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> windows_core::Result<()>;
    fn ProcessAdminPinPolicy(&self, ppinprogress: Option<&IOfflineFilesSyncProgress>, punpinprogress: Option<&IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn GetSettingObject(&self, pszsettingname: &windows_core::PCWSTR) -> windows_core::Result<IOfflineFilesSetting>;
    fn EnumSettingObjects(&self) -> windows_core::Result<IEnumOfflineFilesSettings>;
    fn IsPathCacheable(&self, pszpath: &windows_core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesCache {}
impl IOfflineFilesCache_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>() -> IOfflineFilesCache_Vtbl {
        unsafe extern "system" fn Synchronize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: *mut core::ffi::c_void, piprogress: *mut core::ffi::c_void, psyncid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::Synchronize(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwsynccontrol), windows_core::from_raw_borrowed(&pisyncconflicthandler), windows_core::from_raw_borrowed(&piprogress), core::mem::transmute_copy(&psyncid)).into()
        }
        unsafe extern "system" fn DeleteItems<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::DeleteItems(this, core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&basync), windows_core::from_raw_borrowed(&piprogress)).into()
        }
        unsafe extern "system" fn DeleteItemsForUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuser: windows_core::PCWSTR, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::DeleteItemsForUser(this, core::mem::transmute(&pszuser), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&basync), windows_core::from_raw_borrowed(&piprogress)).into()
        }
        unsafe extern "system" fn Pin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::Pin(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&bdeep), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwpincontrolflags), windows_core::from_raw_borrowed(&piprogress)).into()
        }
        unsafe extern "system" fn Unpin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::Unpin(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&bdeep), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwpincontrolflags), windows_core::from_raw_borrowed(&piprogress)).into()
        }
        unsafe extern "system" fn GetEncryptionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::GetEncryptionStatus(this, core::mem::transmute_copy(&pbencrypted), core::mem::transmute_copy(&pbpartial)).into()
        }
        unsafe extern "system" fn Encrypt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::Encrypt(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bencrypt), core::mem::transmute_copy(&dwencryptioncontrolflags), core::mem::transmute_copy(&basync), windows_core::from_raw_borrowed(&piprogress)).into()
        }
        unsafe extern "system" fn FindItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwqueryflags: u32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesCache_Impl::FindItem(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwqueryflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pincludefilefilter: *mut core::ffi::c_void, pincludedirfilter: *mut core::ffi::c_void, pexcludefilefilter: *mut core::ffi::c_void, pexcludedirfilter: *mut core::ffi::c_void, dwqueryflags: u32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesCache_Impl::FindItemEx(this, core::mem::transmute(&pszpath), windows_core::from_raw_borrowed(&pincludefilefilter), windows_core::from_raw_borrowed(&pincludedirfilter), windows_core::from_raw_borrowed(&pexcludefilefilter), windows_core::from_raw_borrowed(&pexcludedirfilter), core::mem::transmute_copy(&dwqueryflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpathoriginal: windows_core::PCWSTR, pszpathnew: windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::RenameItem(this, core::mem::transmute(&pszpathoriginal), core::mem::transmute(&pszpathnew), core::mem::transmute_copy(&breplaceifexists)).into()
        }
        unsafe extern "system" fn GetLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesCache_Impl::GetLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::GetDiskSpaceInformation(this, core::mem::transmute_copy(&pcbvolumetotal), core::mem::transmute_copy(&pcblimit), core::mem::transmute_copy(&pcbused), core::mem::transmute_copy(&pcbunpinnedlimit), core::mem::transmute_copy(&pcbunpinnedused)).into()
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::SetDiskSpaceLimits(this, core::mem::transmute_copy(&cblimit), core::mem::transmute_copy(&cbunpinnedlimit)).into()
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinprogress: *mut core::ffi::c_void, punpinprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::ProcessAdminPinPolicy(this, windows_core::from_raw_borrowed(&ppinprogress), windows_core::from_raw_borrowed(&punpinprogress)).into()
        }
        unsafe extern "system" fn GetSettingObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsettingname: windows_core::PCWSTR, ppsetting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesCache_Impl::GetSettingObject(this, core::mem::transmute(&pszsettingname)) {
                Ok(ok__) => {
                    core::ptr::write(ppsetting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSettingObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesCache_Impl::EnumSettingObjects(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathCacheable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache_Impl::IsPathCacheable(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&pbcacheable), core::mem::transmute_copy(&psharecachingmode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Synchronize: Synchronize::<Identity, Impl, OFFSET>,
            DeleteItems: DeleteItems::<Identity, Impl, OFFSET>,
            DeleteItemsForUser: DeleteItemsForUser::<Identity, Impl, OFFSET>,
            Pin: Pin::<Identity, Impl, OFFSET>,
            Unpin: Unpin::<Identity, Impl, OFFSET>,
            GetEncryptionStatus: GetEncryptionStatus::<Identity, Impl, OFFSET>,
            Encrypt: Encrypt::<Identity, Impl, OFFSET>,
            FindItem: FindItem::<Identity, Impl, OFFSET>,
            FindItemEx: FindItemEx::<Identity, Impl, OFFSET>,
            RenameItem: RenameItem::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetDiskSpaceInformation: GetDiskSpaceInformation::<Identity, Impl, OFFSET>,
            SetDiskSpaceLimits: SetDiskSpaceLimits::<Identity, Impl, OFFSET>,
            ProcessAdminPinPolicy: ProcessAdminPinPolicy::<Identity, Impl, OFFSET>,
            GetSettingObject: GetSettingObject::<Identity, Impl, OFFSET>,
            EnumSettingObjects: EnumSettingObjects::<Identity, Impl, OFFSET>,
            IsPathCacheable: IsPathCacheable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesCache as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesCache2_Impl: Sized + IOfflineFilesCache_Impl {
    fn RenameItemEx(&self, pszpathoriginal: &windows_core::PCWSTR, pszpathnew: &windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesCache2 {}
impl IOfflineFilesCache2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache2_Impl, const OFFSET: isize>() -> IOfflineFilesCache2_Vtbl {
        unsafe extern "system" fn RenameItemEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesCache2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpathoriginal: windows_core::PCWSTR, pszpathnew: windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesCache2_Impl::RenameItemEx(this, core::mem::transmute(&pszpathoriginal), core::mem::transmute(&pszpathnew), core::mem::transmute_copy(&breplaceifexists)).into()
        }
        Self { base__: IOfflineFilesCache_Vtbl::new::<Identity, Impl, OFFSET>(), RenameItemEx: RenameItemEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesCache2 as windows_core::Interface>::IID || iid == &<IOfflineFilesCache as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesChangeInfo_Impl: Sized {
    fn IsDirty(&self, pbdirty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT;
    fn IsDeletedOffline(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsCreatedOffline(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedData(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedAttributes(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedTime(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesChangeInfo {}
impl IOfflineFilesChangeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>() -> IOfflineFilesChangeInfo_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesChangeInfo_Impl::IsDirty(this, core::mem::transmute_copy(&pbdirty))
        }
        unsafe extern "system" fn IsDeletedOffline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesChangeInfo_Impl::IsDeletedOffline(this) {
                Ok(ok__) => {
                    core::ptr::write(pbdeletedoffline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCreatedOffline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesChangeInfo_Impl::IsCreatedOffline(this) {
                Ok(ok__) => {
                    core::ptr::write(pbcreatedoffline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedData(this) {
                Ok(ok__) => {
                    core::ptr::write(pblocallymodifieddata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedAttributes(this) {
                Ok(ok__) => {
                    core::ptr::write(pblocallymodifiedattributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pblocallymodifiedtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsDeletedOffline: IsDeletedOffline::<Identity, Impl, OFFSET>,
            IsCreatedOffline: IsCreatedOffline::<Identity, Impl, OFFSET>,
            IsLocallyModifiedData: IsLocallyModifiedData::<Identity, Impl, OFFSET>,
            IsLocallyModifiedAttributes: IsLocallyModifiedAttributes::<Identity, Impl, OFFSET>,
            IsLocallyModifiedTime: IsLocallyModifiedTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesChangeInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesConnectionInfo_Impl: Sized {
    fn GetConnectState(&self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::Result<()>;
    fn SetConnectState(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> windows_core::Result<()>;
    fn TransitionOnline(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
    fn TransitionOffline(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesConnectionInfo {}
impl IOfflineFilesConnectionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>() -> IOfflineFilesConnectionInfo_Vtbl {
        unsafe extern "system" fn GetConnectState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesConnectionInfo_Impl::GetConnectState(this, core::mem::transmute_copy(&pconnectstate), core::mem::transmute_copy(&pofflinereason)).into()
        }
        unsafe extern "system" fn SetConnectState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesConnectionInfo_Impl::SetConnectState(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&connectstate)).into()
        }
        unsafe extern "system" fn TransitionOnline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesConnectionInfo_Impl::TransitionOnline(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn TransitionOffline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesConnectionInfo_Impl::TransitionOffline(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&bforceopenfilesclosed)) {
                Ok(ok__) => {
                    core::ptr::write(pbopenfilespreventedtransition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectState: GetConnectState::<Identity, Impl, OFFSET>,
            SetConnectState: SetConnectState::<Identity, Impl, OFFSET>,
            TransitionOnline: TransitionOnline::<Identity, Impl, OFFSET>,
            TransitionOffline: TransitionOffline::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesConnectionInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesDirectoryItem_Impl: Sized + IOfflineFilesItem_Impl {}
impl windows_core::RuntimeName for IOfflineFilesDirectoryItem {}
impl IOfflineFilesDirectoryItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesDirectoryItem_Impl, const OFFSET: isize>() -> IOfflineFilesDirectoryItem_Vtbl {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesDirectoryItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesDirtyInfo_Impl: Sized {
    fn LocalDirtyByteCount(&self) -> windows_core::Result<i64>;
    fn RemoteDirtyByteCount(&self) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IOfflineFilesDirtyInfo {}
impl IOfflineFilesDirtyInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>() -> IOfflineFilesDirtyInfo_Vtbl {
        unsafe extern "system" fn LocalDirtyByteCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybytecount: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesDirtyInfo_Impl::LocalDirtyByteCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pdirtybytecount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybytecount: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesDirtyInfo_Impl::RemoteDirtyByteCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pdirtybytecount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LocalDirtyByteCount: LocalDirtyByteCount::<Identity, Impl, OFFSET>,
            RemoteDirtyByteCount: RemoteDirtyByteCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesDirtyInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOfflineFilesErrorInfo_Impl: Sized {
    fn GetRawData(&self) -> windows_core::Result<*mut super::super::System::Com::BYTE_BLOB>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOfflineFilesErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IOfflineFilesErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>() -> IOfflineFilesErrorInfo_Vtbl {
        unsafe extern "system" fn GetRawData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesErrorInfo_Impl::GetRawData(this) {
                Ok(ok__) => {
                    core::ptr::write(ppblob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesErrorInfo_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRawData: GetRawData::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesErrorInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesEvents_Impl: Sized {
    fn CacheMoved(&self, pszoldpath: &windows_core::PCWSTR, psznewpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CacheIsFull(&self) -> windows_core::Result<()>;
    fn CacheIsCorrupted(&self) -> windows_core::Result<()>;
    fn Enabled(&self, benabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EncryptionChanged(&self, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SyncBegin(&self, rsyncid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SyncFileResult(&self, rsyncid: *const windows_core::GUID, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn SyncConflictRecAdded(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncConflictRecUpdated(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncConflictRecRemoved(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncEnd(&self, rsyncid: *const windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn NetTransportArrived(&self) -> windows_core::Result<()>;
    fn NoNetTransports(&self) -> windows_core::Result<()>;
    fn ItemDisconnected(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemReconnected(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemAvailableOffline(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemNotAvailableOffline(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemPinned(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemNotPinned(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemModified(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ItemAddedToCache(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemDeletedFromCache(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemRenamed(&self, pszoldpath: &windows_core::PCWSTR, psznewpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn DataLost(&self) -> windows_core::Result<()>;
    fn Ping(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesEvents {}
impl IOfflineFilesEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>() -> IOfflineFilesEvents_Vtbl {
        unsafe extern "system" fn CacheMoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszoldpath: windows_core::PCWSTR, psznewpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::CacheMoved(this, core::mem::transmute(&pszoldpath), core::mem::transmute(&psznewpath)).into()
        }
        unsafe extern "system" fn CacheIsFull<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::CacheIsFull(this).into()
        }
        unsafe extern "system" fn CacheIsCorrupted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::CacheIsCorrupted(this).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::Enabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn EncryptionChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::EncryptionChanged(this, core::mem::transmute_copy(&bwasencrypted), core::mem::transmute_copy(&bwaspartial), core::mem::transmute_copy(&bisencrypted), core::mem::transmute_copy(&bispartial)).into()
        }
        unsafe extern "system" fn SyncBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncBegin(this, core::mem::transmute_copy(&rsyncid)).into()
        }
        unsafe extern "system" fn SyncFileResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncFileResult(this, core::mem::transmute_copy(&rsyncid), core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn SyncConflictRecAdded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncConflictRecAdded(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncConflictRecUpdated(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncConflictRecRemoved(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::SyncEnd(this, core::mem::transmute_copy(&rsyncid), core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn NetTransportArrived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::NetTransportArrived(this).into()
        }
        unsafe extern "system" fn NoNetTransports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::NoNetTransports(this).into()
        }
        unsafe extern "system" fn ItemDisconnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemDisconnected(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemReconnected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemReconnected(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemAvailableOffline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemAvailableOffline(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemNotAvailableOffline(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemPinned(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemNotPinned(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemModified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemModified(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype), core::mem::transmute_copy(&bmodifieddata), core::mem::transmute_copy(&bmodifiedattributes)).into()
        }
        unsafe extern "system" fn ItemAddedToCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemAddedToCache(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemDeletedFromCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemDeletedFromCache(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemRenamed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszoldpath: windows_core::PCWSTR, psznewpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::ItemRenamed(this, core::mem::transmute(&pszoldpath), core::mem::transmute(&psznewpath), core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn DataLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::DataLost(this).into()
        }
        unsafe extern "system" fn Ping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents_Impl::Ping(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CacheMoved: CacheMoved::<Identity, Impl, OFFSET>,
            CacheIsFull: CacheIsFull::<Identity, Impl, OFFSET>,
            CacheIsCorrupted: CacheIsCorrupted::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            EncryptionChanged: EncryptionChanged::<Identity, Impl, OFFSET>,
            SyncBegin: SyncBegin::<Identity, Impl, OFFSET>,
            SyncFileResult: SyncFileResult::<Identity, Impl, OFFSET>,
            SyncConflictRecAdded: SyncConflictRecAdded::<Identity, Impl, OFFSET>,
            SyncConflictRecUpdated: SyncConflictRecUpdated::<Identity, Impl, OFFSET>,
            SyncConflictRecRemoved: SyncConflictRecRemoved::<Identity, Impl, OFFSET>,
            SyncEnd: SyncEnd::<Identity, Impl, OFFSET>,
            NetTransportArrived: NetTransportArrived::<Identity, Impl, OFFSET>,
            NoNetTransports: NoNetTransports::<Identity, Impl, OFFSET>,
            ItemDisconnected: ItemDisconnected::<Identity, Impl, OFFSET>,
            ItemReconnected: ItemReconnected::<Identity, Impl, OFFSET>,
            ItemAvailableOffline: ItemAvailableOffline::<Identity, Impl, OFFSET>,
            ItemNotAvailableOffline: ItemNotAvailableOffline::<Identity, Impl, OFFSET>,
            ItemPinned: ItemPinned::<Identity, Impl, OFFSET>,
            ItemNotPinned: ItemNotPinned::<Identity, Impl, OFFSET>,
            ItemModified: ItemModified::<Identity, Impl, OFFSET>,
            ItemAddedToCache: ItemAddedToCache::<Identity, Impl, OFFSET>,
            ItemDeletedFromCache: ItemDeletedFromCache::<Identity, Impl, OFFSET>,
            ItemRenamed: ItemRenamed::<Identity, Impl, OFFSET>,
            DataLost: DataLost::<Identity, Impl, OFFSET>,
            Ping: Ping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesEvents2_Impl: Sized + IOfflineFilesEvents_Impl {
    fn ItemReconnectBegin(&self) -> windows_core::Result<()>;
    fn ItemReconnectEnd(&self) -> windows_core::Result<()>;
    fn CacheEvictBegin(&self) -> windows_core::Result<()>;
    fn CacheEvictEnd(&self) -> windows_core::Result<()>;
    fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> windows_core::Result<()>;
    fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> windows_core::Result<()>;
    fn PolicyChangeDetected(&self) -> windows_core::Result<()>;
    fn PreferenceChangeDetected(&self) -> windows_core::Result<()>;
    fn SettingsChangesApplied(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesEvents2 {}
impl IOfflineFilesEvents2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>() -> IOfflineFilesEvents2_Vtbl {
        unsafe extern "system" fn ItemReconnectBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::ItemReconnectBegin(this).into()
        }
        unsafe extern "system" fn ItemReconnectEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::ItemReconnectEnd(this).into()
        }
        unsafe extern "system" fn CacheEvictBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::CacheEvictBegin(this).into()
        }
        unsafe extern "system" fn CacheEvictEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::CacheEvictEnd(this).into()
        }
        unsafe extern "system" fn BackgroundSyncBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsynccontrolflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::BackgroundSyncBegin(this, core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn BackgroundSyncEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsynccontrolflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::BackgroundSyncEnd(this, core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn PolicyChangeDetected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::PolicyChangeDetected(this).into()
        }
        unsafe extern "system" fn PreferenceChangeDetected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::PreferenceChangeDetected(this).into()
        }
        unsafe extern "system" fn SettingsChangesApplied<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents2_Impl::SettingsChangesApplied(this).into()
        }
        Self {
            base__: IOfflineFilesEvents_Vtbl::new::<Identity, Impl, OFFSET>(),
            ItemReconnectBegin: ItemReconnectBegin::<Identity, Impl, OFFSET>,
            ItemReconnectEnd: ItemReconnectEnd::<Identity, Impl, OFFSET>,
            CacheEvictBegin: CacheEvictBegin::<Identity, Impl, OFFSET>,
            CacheEvictEnd: CacheEvictEnd::<Identity, Impl, OFFSET>,
            BackgroundSyncBegin: BackgroundSyncBegin::<Identity, Impl, OFFSET>,
            BackgroundSyncEnd: BackgroundSyncEnd::<Identity, Impl, OFFSET>,
            PolicyChangeDetected: PolicyChangeDetected::<Identity, Impl, OFFSET>,
            PreferenceChangeDetected: PreferenceChangeDetected::<Identity, Impl, OFFSET>,
            SettingsChangesApplied: SettingsChangesApplied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesEvents3_Impl: Sized + IOfflineFilesEvents2_Impl {
    fn TransparentCacheItemNotify(&self, pszpath: &windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrefetchFileBegin(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrefetchFileEnd(&self, pszpath: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesEvents3 {}
impl IOfflineFilesEvents3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>() -> IOfflineFilesEvents3_Vtbl {
        unsafe extern "system" fn TransparentCacheItemNotify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents3_Impl::TransparentCacheItemNotify(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&eventtype), core::mem::transmute_copy(&itemtype), core::mem::transmute_copy(&bmodifieddata), core::mem::transmute_copy(&bmodifiedattributes), core::mem::transmute(&pzsoldpath)).into()
        }
        unsafe extern "system" fn PrefetchFileBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents3_Impl::PrefetchFileBegin(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn PrefetchFileEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents3_Impl::PrefetchFileEnd(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base__: IOfflineFilesEvents2_Vtbl::new::<Identity, Impl, OFFSET>(),
            TransparentCacheItemNotify: TransparentCacheItemNotify::<Identity, Impl, OFFSET>,
            PrefetchFileBegin: PrefetchFileBegin::<Identity, Impl, OFFSET>,
            PrefetchFileEnd: PrefetchFileEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents3 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesEvents4_Impl: Sized + IOfflineFilesEvents3_Impl {
    fn PrefetchCloseHandleBegin(&self) -> windows_core::Result<()>;
    fn PrefetchCloseHandleEnd(&self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesEvents4 {}
impl IOfflineFilesEvents4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>() -> IOfflineFilesEvents4_Vtbl {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents4_Impl::PrefetchCloseHandleBegin(this).into()
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEvents4_Impl::PrefetchCloseHandleEnd(this, core::mem::transmute_copy(&dwclosedhandlecount), core::mem::transmute_copy(&dwopenhandlecount), core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base__: IOfflineFilesEvents3_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrefetchCloseHandleBegin: PrefetchCloseHandleBegin::<Identity, Impl, OFFSET>,
            PrefetchCloseHandleEnd: PrefetchCloseHandleEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents4 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents3 as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesEventsFilter_Impl: Sized {
    fn GetPathFilter(&self, ppszfilter: *mut windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::Result<()>;
    fn GetIncludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::Result<()>;
    fn GetExcludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesEventsFilter {}
impl IOfflineFilesEventsFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>() -> IOfflineFilesEventsFilter_Vtbl {
        unsafe extern "system" fn GetPathFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszfilter: *mut windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEventsFilter_Impl::GetPathFilter(this, core::mem::transmute_copy(&ppszfilter), core::mem::transmute_copy(&pmatch)).into()
        }
        unsafe extern "system" fn GetIncludedEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEventsFilter_Impl::GetIncludedEvents(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&prgevents), core::mem::transmute_copy(&pcevents)).into()
        }
        unsafe extern "system" fn GetExcludedEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesEventsFilter_Impl::GetExcludedEvents(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&prgevents), core::mem::transmute_copy(&pcevents)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPathFilter: GetPathFilter::<Identity, Impl, OFFSET>,
            GetIncludedEvents: GetIncludedEvents::<Identity, Impl, OFFSET>,
            GetExcludedEvents: GetExcludedEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEventsFilter as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesFileItem_Impl: Sized + IOfflineFilesItem_Impl {
    fn IsSparse(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEncrypted(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesFileItem {}
impl IOfflineFilesFileItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>() -> IOfflineFilesFileItem_Vtbl {
        unsafe extern "system" fn IsSparse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesFileItem_Impl::IsSparse(this) {
                Ok(ok__) => {
                    core::ptr::write(pbissparse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEncrypted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesFileItem_Impl::IsEncrypted(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisencrypted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsSparse: IsSparse::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesFileItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesFileSysInfo_Impl: Sized {
    fn GetAttributes(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<u32>;
    fn GetTimes(&self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFileSize(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IOfflineFilesFileSysInfo {}
impl IOfflineFilesFileSysInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>() -> IOfflineFilesFileSysInfo_Vtbl {
        unsafe extern "system" fn GetAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesFileSysInfo_Impl::GetAttributes(this, core::mem::transmute_copy(&copy)) {
                Ok(ok__) => {
                    core::ptr::write(pdwattributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesFileSysInfo_Impl::GetTimes(this, core::mem::transmute_copy(&copy), core::mem::transmute_copy(&pftcreationtime), core::mem::transmute_copy(&pftlastwritetime), core::mem::transmute_copy(&pftchangetime), core::mem::transmute_copy(&pftlastaccesstime)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesFileSysInfo_Impl::GetFileSize(this, core::mem::transmute_copy(&copy)) {
                Ok(ok__) => {
                    core::ptr::write(psize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesFileSysInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesGhostInfo_Impl: Sized {
    fn IsGhosted(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesGhostInfo {}
impl IOfflineFilesGhostInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>() -> IOfflineFilesGhostInfo_Vtbl {
        unsafe extern "system" fn IsGhosted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesGhostInfo_Impl::IsGhosted(this) {
                Ok(ok__) => {
                    core::ptr::write(pbghosted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsGhosted: IsGhosted::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesGhostInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesItem_Impl: Sized {
    fn GetItemType(&self) -> windows_core::Result<OFFLINEFILES_ITEM_TYPE>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentItem(&self) -> windows_core::Result<IOfflineFilesItem>;
    fn Refresh(&self, dwqueryflags: u32) -> windows_core::Result<()>;
    fn IsMarkedForDeletion(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesItem {}
impl IOfflineFilesItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>() -> IOfflineFilesItem_Vtbl {
        unsafe extern "system" fn GetItemType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItem_Impl::GetItemType(this) {
                Ok(ok__) => {
                    core::ptr::write(pitemtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItem_Impl::GetPath(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszpath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItem_Impl::GetParentItem(this) {
                Ok(ok__) => {
                    core::ptr::write(ppitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqueryflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesItem_Impl::Refresh(this, core::mem::transmute_copy(&dwqueryflags)).into()
        }
        unsafe extern "system" fn IsMarkedForDeletion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItem_Impl::IsMarkedForDeletion(this) {
                Ok(ok__) => {
                    core::ptr::write(pbmarkedfordeletion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            IsMarkedForDeletion: IsMarkedForDeletion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesItemContainer_Impl: Sized {
    fn EnumItems(&self, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems>;
    fn EnumItemsEx(&self, pincludefilefilter: Option<&IOfflineFilesItemFilter>, pincludedirfilter: Option<&IOfflineFilesItemFilter>, pexcludefilefilter: Option<&IOfflineFilesItemFilter>, pexcludedirfilter: Option<&IOfflineFilesItemFilter>, dwenumflags: u32, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems>;
}
impl windows_core::RuntimeName for IOfflineFilesItemContainer {}
impl IOfflineFilesItemContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>() -> IOfflineFilesItemContainer_Vtbl {
        unsafe extern "system" fn EnumItems<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqueryflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItemContainer_Impl::EnumItems(this, core::mem::transmute_copy(&dwqueryflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pincludefilefilter: *mut core::ffi::c_void, pincludedirfilter: *mut core::ffi::c_void, pexcludefilefilter: *mut core::ffi::c_void, pexcludedirfilter: *mut core::ffi::c_void, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesItemContainer_Impl::EnumItemsEx(this, windows_core::from_raw_borrowed(&pincludefilefilter), windows_core::from_raw_borrowed(&pincludedirfilter), windows_core::from_raw_borrowed(&pexcludefilefilter), windows_core::from_raw_borrowed(&pexcludedirfilter), core::mem::transmute_copy(&dwenumflags), core::mem::transmute_copy(&dwqueryflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, Impl, OFFSET>,
            EnumItemsEx: EnumItemsEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItemContainer as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesItemFilter_Impl: Sized {
    fn GetFilterFlags(&self, pullflags: *mut u64, pullmask: *mut u64) -> windows_core::Result<()>;
    fn GetTimeFilter(&self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> windows_core::Result<()>;
    fn GetPatternFilter(&self, pszpattern: windows_core::PWSTR, cchpattern: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesItemFilter {}
impl IOfflineFilesItemFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>() -> IOfflineFilesItemFilter_Vtbl {
        unsafe extern "system" fn GetFilterFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesItemFilter_Impl::GetFilterFlags(this, core::mem::transmute_copy(&pullflags), core::mem::transmute_copy(&pullmask)).into()
        }
        unsafe extern "system" fn GetTimeFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesItemFilter_Impl::GetTimeFilter(this, core::mem::transmute_copy(&pfttime), core::mem::transmute_copy(&pbevaltimeofday), core::mem::transmute_copy(&ptimetype), core::mem::transmute_copy(&pcompare)).into()
        }
        unsafe extern "system" fn GetPatternFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PWSTR, cchpattern: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesItemFilter_Impl::GetPatternFilter(this, core::mem::transmute_copy(&pszpattern), core::mem::transmute_copy(&cchpattern)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterFlags: GetFilterFlags::<Identity, Impl, OFFSET>,
            GetTimeFilter: GetTimeFilter::<Identity, Impl, OFFSET>,
            GetPatternFilter: GetPatternFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItemFilter as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesPinInfo_Impl: Sized {
    fn IsPinned(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesPinInfo {}
impl IOfflineFilesPinInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>() -> IOfflineFilesPinInfo_Vtbl {
        unsafe extern "system" fn IsPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesPinInfo_Impl::IsPinned(this) {
                Ok(ok__) => {
                    core::ptr::write(pbpinned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesPinInfo_Impl::IsPinnedForUser(this, core::mem::transmute_copy(&pbpinnedforuser), core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesPinInfo_Impl::IsPinnedForUserByPolicy(this, core::mem::transmute_copy(&pbpinnedforuser), core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesPinInfo_Impl::IsPinnedForComputer(this, core::mem::transmute_copy(&pbpinnedforcomputer), core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesPinInfo_Impl::IsPinnedForFolderRedirection(this, core::mem::transmute_copy(&pbpinnedforfolderredirection), core::mem::transmute_copy(&pbinherit)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPinned: IsPinned::<Identity, Impl, OFFSET>,
            IsPinnedForUser: IsPinnedForUser::<Identity, Impl, OFFSET>,
            IsPinnedForUserByPolicy: IsPinnedForUserByPolicy::<Identity, Impl, OFFSET>,
            IsPinnedForComputer: IsPinnedForComputer::<Identity, Impl, OFFSET>,
            IsPinnedForFolderRedirection: IsPinnedForFolderRedirection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesPinInfo2_Impl: Sized + IOfflineFilesPinInfo_Impl {
    fn IsPartlyPinned(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesPinInfo2 {}
impl IOfflineFilesPinInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>() -> IOfflineFilesPinInfo2_Vtbl {
        unsafe extern "system" fn IsPartlyPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesPinInfo2_Impl::IsPartlyPinned(this) {
                Ok(ok__) => {
                    core::ptr::write(pbpartlypinned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IOfflineFilesPinInfo_Vtbl::new::<Identity, Impl, OFFSET>(), IsPartlyPinned: IsPartlyPinned::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo2 as windows_core::Interface>::IID || iid == &<IOfflineFilesPinInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesProgress_Impl: Sized {
    fn Begin(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn QueryAbort(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn End(&self, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesProgress {}
impl IOfflineFilesProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>() -> IOfflineFilesProgress_Vtbl {
        unsafe extern "system" fn Begin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesProgress_Impl::Begin(this) {
                Ok(ok__) => {
                    core::ptr::write(pbabort, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesProgress_Impl::QueryAbort(this) {
                Ok(ok__) => {
                    core::ptr::write(pbabort, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesProgress_Impl::End(this, core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, Impl, OFFSET>,
            QueryAbort: QueryAbort::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesServerItem_Impl: Sized + IOfflineFilesItem_Impl {}
impl windows_core::RuntimeName for IOfflineFilesServerItem {}
impl IOfflineFilesServerItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesServerItem_Impl, const OFFSET: isize>() -> IOfflineFilesServerItem_Vtbl {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesServerItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSetting_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetValueType(&self) -> windows_core::Result<OFFLINEFILES_SETTING_VALUE_TYPE>;
    fn GetPreference(&self, pvarvalue: *mut windows_core::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn GetPreferenceScope(&self) -> windows_core::Result<u32>;
    fn SetPreference(&self, pvarvalue: *const windows_core::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn DeletePreference(&self, dwscope: u32) -> windows_core::Result<()>;
    fn GetPolicy(&self, pvarvalue: *mut windows_core::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn GetPolicyScope(&self) -> windows_core::Result<u32>;
    fn GetValue(&self, pvarvalue: *mut windows_core::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesSetting {}
impl IOfflineFilesSetting_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>() -> IOfflineFilesSetting_Vtbl {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSetting_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSetting_Impl::GetValueType(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>, dwscope: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSetting_Impl::GetPreference(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPreferenceScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscope: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSetting_Impl::GetPreferenceScope(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwscope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>, dwscope: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSetting_Impl::SetPreference(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn DeletePreference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwscope: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSetting_Impl::DeletePreference(this, core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>, dwscope: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSetting_Impl::GetPolicy(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicyScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscope: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSetting_Impl::GetPolicyScope(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwscope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSetting_Impl::GetValue(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&pbsetbypolicy)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValueType: GetValueType::<Identity, Impl, OFFSET>,
            GetPreference: GetPreference::<Identity, Impl, OFFSET>,
            GetPreferenceScope: GetPreferenceScope::<Identity, Impl, OFFSET>,
            SetPreference: SetPreference::<Identity, Impl, OFFSET>,
            DeletePreference: DeletePreference::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetPolicyScope: GetPolicyScope::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSetting as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesShareInfo_Impl: Sized {
    fn GetShareItem(&self) -> windows_core::Result<IOfflineFilesShareItem>;
    fn GetShareCachingMode(&self) -> windows_core::Result<OFFLINEFILES_CACHING_MODE>;
    fn IsShareDfsJunction(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesShareInfo {}
impl IOfflineFilesShareInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>() -> IOfflineFilesShareInfo_Vtbl {
        unsafe extern "system" fn GetShareItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshareitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesShareInfo_Impl::GetShareItem(this) {
                Ok(ok__) => {
                    core::ptr::write(ppshareitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShareCachingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesShareInfo_Impl::GetShareCachingMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pcachingmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShareDfsJunction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesShareInfo_Impl::IsShareDfsJunction(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisdfsjunction, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetShareItem: GetShareItem::<Identity, Impl, OFFSET>,
            GetShareCachingMode: GetShareCachingMode::<Identity, Impl, OFFSET>,
            IsShareDfsJunction: IsShareDfsJunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesShareInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesShareItem_Impl: Sized + IOfflineFilesItem_Impl {}
impl windows_core::RuntimeName for IOfflineFilesShareItem {}
impl IOfflineFilesShareItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesShareItem_Impl, const OFFSET: isize>() -> IOfflineFilesShareItem_Vtbl {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesShareItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSimpleProgress_Impl: Sized + IOfflineFilesProgress_Impl {
    fn ItemBegin(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn ItemResult(&self, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
impl windows_core::RuntimeName for IOfflineFilesSimpleProgress {}
impl IOfflineFilesSimpleProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>() -> IOfflineFilesSimpleProgress_Vtbl {
        unsafe extern "system" fn ItemBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSimpleProgress_Impl::ItemBegin(this, core::mem::transmute(&pszfile)) {
                Ok(ok__) => {
                    core::ptr::write(presponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSimpleProgress_Impl::ItemResult(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult)) {
                Ok(ok__) => {
                    core::ptr::write(presponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOfflineFilesProgress_Vtbl::new::<Identity, Impl, OFFSET>(),
            ItemBegin: ItemBegin::<Identity, Impl, OFFSET>,
            ItemResult: ItemResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSimpleProgress as windows_core::Interface>::IID || iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSuspend_Impl: Sized {
    fn SuspendRoot(&self, bsuspend: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesSuspend {}
impl IOfflineFilesSuspend_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSuspend_Impl, const OFFSET: isize>() -> IOfflineFilesSuspend_Vtbl {
        unsafe extern "system" fn SuspendRoot<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSuspend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSuspend_Impl::SuspendRoot(this, core::mem::transmute_copy(&bsuspend)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SuspendRoot: SuspendRoot::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSuspend as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSuspendInfo_Impl: Sized {
    fn IsSuspended(&self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesSuspendInfo {}
impl IOfflineFilesSuspendInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSuspendInfo_Vtbl {
        unsafe extern "system" fn IsSuspended<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSuspendInfo_Impl::IsSuspended(this, core::mem::transmute_copy(&pbsuspended), core::mem::transmute_copy(&pbsuspendedroot)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSuspended: IsSuspended::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSuspendInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSyncConflictHandler_Impl: Sized {
    fn ResolveConflict(&self, pszpath: &windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOfflineFilesSyncConflictHandler {}
impl IOfflineFilesSyncConflictHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>() -> IOfflineFilesSyncConflictHandler_Vtbl {
        unsafe extern "system" fn ResolveConflict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSyncConflictHandler_Impl::ResolveConflict(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&fstateknown), core::mem::transmute_copy(&state), core::mem::transmute_copy(&fchangedetails), core::mem::transmute_copy(&pconflictresolution), core::mem::transmute_copy(&ppsznewname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResolveConflict: ResolveConflict::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncConflictHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOfflineFilesSyncErrorInfo_Impl: Sized + IOfflineFilesErrorInfo_Impl {
    fn GetSyncOperation(&self) -> windows_core::Result<OFFLINEFILES_SYNC_OPERATION>;
    fn GetItemChangeFlags(&self) -> windows_core::Result<u32>;
    fn InfoEnumerated(&self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InfoAvailable(&self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLocalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetRemoteInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetOriginalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOfflineFilesSyncErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IOfflineFilesSyncErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSyncErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncOperation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorInfo_Impl::GetSyncOperation(this) {
                Ok(ok__) => {
                    core::ptr::write(psyncop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemChangeFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitemchangeflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorInfo_Impl::GetItemChangeFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwitemchangeflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoEnumerated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSyncErrorInfo_Impl::InfoEnumerated(this, core::mem::transmute_copy(&pblocalenumerated), core::mem::transmute_copy(&pbremoteenumerated), core::mem::transmute_copy(&pboriginalenumerated)).into()
        }
        unsafe extern "system" fn InfoAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSyncErrorInfo_Impl::InfoAvailable(this, core::mem::transmute_copy(&pblocalinfo), core::mem::transmute_copy(&pbremoteinfo), core::mem::transmute_copy(&pboriginalinfo)).into()
        }
        unsafe extern "system" fn GetLocalInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorInfo_Impl::GetLocalInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorInfo_Impl::GetRemoteInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorInfo_Impl::GetOriginalInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOfflineFilesErrorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncOperation: GetSyncOperation::<Identity, Impl, OFFSET>,
            GetItemChangeFlags: GetItemChangeFlags::<Identity, Impl, OFFSET>,
            InfoEnumerated: InfoEnumerated::<Identity, Impl, OFFSET>,
            InfoAvailable: InfoAvailable::<Identity, Impl, OFFSET>,
            GetLocalInfo: GetLocalInfo::<Identity, Impl, OFFSET>,
            GetRemoteInfo: GetRemoteInfo::<Identity, Impl, OFFSET>,
            GetOriginalInfo: GetOriginalInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorInfo as windows_core::Interface>::IID || iid == &<IOfflineFilesErrorInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSyncErrorItemInfo_Impl: Sized {
    fn GetFileAttributes(&self) -> windows_core::Result<u32>;
    fn GetFileTimes(&self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFileSize(&self) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IOfflineFilesSyncErrorItemInfo {}
impl IOfflineFilesSyncErrorItemInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSyncErrorItemInfo_Vtbl {
        unsafe extern "system" fn GetFileAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorItemInfo_Impl::GetFileAttributes(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwattributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTimes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IOfflineFilesSyncErrorItemInfo_Impl::GetFileTimes(this, core::mem::transmute_copy(&pftlastwrite), core::mem::transmute_copy(&pftchange)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncErrorItemInfo_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(psize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileAttributes: GetFileAttributes::<Identity, Impl, OFFSET>,
            GetFileTimes: GetFileTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorItemInfo as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesSyncProgress_Impl: Sized + IOfflineFilesProgress_Impl {
    fn SyncItemBegin(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn SyncItemResult(&self, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT, perrorinfo: Option<&IOfflineFilesSyncErrorInfo>) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
impl windows_core::RuntimeName for IOfflineFilesSyncProgress {}
impl IOfflineFilesSyncProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>() -> IOfflineFilesSyncProgress_Vtbl {
        unsafe extern "system" fn SyncItemBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncProgress_Impl::SyncItemBegin(this, core::mem::transmute(&pszfile)) {
                Ok(ok__) => {
                    core::ptr::write(presponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncItemResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT, perrorinfo: *mut core::ffi::c_void, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesSyncProgress_Impl::SyncItemResult(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult), windows_core::from_raw_borrowed(&perrorinfo)) {
                Ok(ok__) => {
                    core::ptr::write(presponse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOfflineFilesProgress_Vtbl::new::<Identity, Impl, OFFSET>(),
            SyncItemBegin: SyncItemBegin::<Identity, Impl, OFFSET>,
            SyncItemResult: SyncItemResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncProgress as windows_core::Interface>::IID || iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
pub trait IOfflineFilesTransparentCacheInfo_Impl: Sized {
    fn IsTransparentlyCached(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IOfflineFilesTransparentCacheInfo {}
impl IOfflineFilesTransparentCacheInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>() -> IOfflineFilesTransparentCacheInfo_Vtbl {
        unsafe extern "system" fn IsTransparentlyCached<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOfflineFilesTransparentCacheInfo_Impl::IsTransparentlyCached(this) {
                Ok(ok__) => {
                    core::ptr::write(pbtransparentlycached, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsTransparentlyCached: IsTransparentlyCached::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesTransparentCacheInfo as windows_core::Interface>::IID
    }
}
