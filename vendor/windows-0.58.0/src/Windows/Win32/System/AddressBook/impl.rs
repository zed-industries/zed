#[cfg(feature = "Win32_System_Com")]
pub trait IABContainer_Impl: Sized + IMAPIContainer_Impl {
    fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp>;
    fn CopyEntries(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()>;
    fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IABContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IABContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IABContainer_Vtbl
    where
        Identity: IABContainer_Impl,
    {
        unsafe extern "system" fn CreateEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IABContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IABContainer_Impl::CreateEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulcreateflags)) {
                Ok(ok__) => {
                    lppmapipropentry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IABContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IABContainer_Impl::CopyEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IABContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IABContainer_Impl::DeleteEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ResolveNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> windows_core::HRESULT
        where
            Identity: IABContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IABContainer_Impl::ResolveNames(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpadrlist)) {
                Ok(ok__) => {
                    lpflaglist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, OFFSET>,
            CopyEntries: CopyEntries::<Identity, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, OFFSET>,
            ResolveNames: ResolveNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IABContainer as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAddrBook_Impl: Sized + IMAPIProp_Impl {
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::Result<()>;
    fn Advise(&self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: Option<&IMAPIAdviseSink>, lpulconnection: *mut u32) -> windows_core::Result<()>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn CreateOneOff(&self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn NewEntry(&self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn ResolveName(&self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::Result<()>;
    fn Address(&self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::Result<()>;
    fn Details(&self, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::Result<()>;
    fn RecipOptions(&self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::Result<()>;
    fn QueryDefaultRecipOpt(&self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::Result<()>;
    fn GetPAB(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn SetPAB(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()>;
    fn GetDefaultDir(&self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::Result<()>;
    fn SetDefaultDir(&self, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::Result<()>;
    fn GetSearchPath(&self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::Result<()>;
    fn SetSearchPath(&self, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::Result<()>;
    fn PrepareRecips(&self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAddrBook {}
#[cfg(feature = "Win32_System_Com")]
impl IAddrBook_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAddrBook_Vtbl
    where
        Identity: IAddrBook_Impl,
    {
        unsafe extern "system" fn OpenEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::CompareEntryIDs(this, core::mem::transmute_copy(&cbentryid1), core::mem::transmute_copy(&lpentryid1), core::mem::transmute_copy(&cbentryid2), core::mem::transmute_copy(&lpentryid2), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulresult)).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::Advise(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uleventmask), windows_core::from_raw_borrowed(&lpadvisesink), core::mem::transmute_copy(&lpulconnection)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn CreateOneOff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::CreateOneOff(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&lpszadrtype), core::mem::transmute_copy(&lpszaddress), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn NewEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::NewEntry(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbeidcontainer), core::mem::transmute_copy(&lpeidcontainer), core::mem::transmute_copy(&cbeidnewentrytpl), core::mem::transmute_copy(&lpeidnewentrytpl), core::mem::transmute_copy(&lpcbeidnewentry), core::mem::transmute_copy(&lppeidnewentry)).into()
        }
        unsafe extern "system" fn ResolveName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::ResolveName(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsznewentrytitle), core::mem::transmute_copy(&lpadrlist)).into()
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::Address(this, core::mem::transmute_copy(&lpuluiparam), core::mem::transmute_copy(&lpadrparms), core::mem::transmute_copy(&lppadrlist)).into()
        }
        unsafe extern "system" fn Details<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::Details(this, core::mem::transmute_copy(&lpuluiparam), core::mem::transmute_copy(&lpfndismiss), core::mem::transmute_copy(&lpvdismisscontext), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpfbuttoncallback), core::mem::transmute_copy(&lpvbuttoncontext), core::mem::transmute_copy(&lpszbuttontext), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn RecipOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::RecipOptions(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lprecip)).into()
        }
        unsafe extern "system" fn QueryDefaultRecipOpt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::QueryDefaultRecipOpt(this, core::mem::transmute_copy(&lpszadrtype), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcvalues), core::mem::transmute_copy(&lppoptions)).into()
        }
        unsafe extern "system" fn GetPAB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::GetPAB(this, core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn SetPAB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::SetPAB(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetDefaultDir<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::GetDefaultDir(this, core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn SetDefaultDir<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::SetDefaultDir(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetSearchPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::GetSearchPath(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppsearchpath)).into()
        }
        unsafe extern "system" fn SetSearchPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::SetSearchPath(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsearchpath)).into()
        }
        unsafe extern "system" fn PrepareRecips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> windows_core::HRESULT
        where
            Identity: IAddrBook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAddrBook_Impl::PrepareRecips(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&lpreciplist)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            CreateOneOff: CreateOneOff::<Identity, OFFSET>,
            NewEntry: NewEntry::<Identity, OFFSET>,
            ResolveName: ResolveName::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            Details: Details::<Identity, OFFSET>,
            RecipOptions: RecipOptions::<Identity, OFFSET>,
            QueryDefaultRecipOpt: QueryDefaultRecipOpt::<Identity, OFFSET>,
            GetPAB: GetPAB::<Identity, OFFSET>,
            SetPAB: SetPAB::<Identity, OFFSET>,
            GetDefaultDir: GetDefaultDir::<Identity, OFFSET>,
            SetDefaultDir: SetDefaultDir::<Identity, OFFSET>,
            GetSearchPath: GetSearchPath::<Identity, OFFSET>,
            SetSearchPath: SetSearchPath::<Identity, OFFSET>,
            PrepareRecips: PrepareRecips::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrBook as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAttach_Impl: Sized + IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAttach {}
#[cfg(feature = "Win32_System_Com")]
impl IAttach_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAttach_Vtbl
    where
        Identity: IAttach_Impl,
    {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAttach as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDistList_Impl: Sized + IMAPIContainer_Impl {
    fn CreateEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> windows_core::Result<IMAPIProp>;
    fn CopyEntries(&self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteEntries(&self, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::Result<()>;
    fn ResolveNames(&self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> windows_core::Result<FlagList>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDistList {}
#[cfg(feature = "Win32_System_Com")]
impl IDistList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDistList_Vtbl
    where
        Identity: IDistList_Impl,
    {
        unsafe extern "system" fn CreateEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDistList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDistList_Impl::CreateEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulcreateflags)) {
                Ok(ok__) => {
                    lppmapipropentry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IDistList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDistList_Impl::CopyEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IDistList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDistList_Impl::DeleteEntries(this, core::mem::transmute_copy(&lpentries), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ResolveNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> windows_core::HRESULT
        where
            Identity: IDistList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDistList_Impl::ResolveNames(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpadrlist)) {
                Ok(ok__) => {
                    lpflaglist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, OFFSET>,
            CopyEntries: CopyEntries::<Identity, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, OFFSET>,
            ResolveNames: ResolveNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDistList as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIAdviseSink_Impl: Sized {
    fn OnNotify(&self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIAdviseSink {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIAdviseSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIAdviseSink_Vtbl
    where
        Identity: IMAPIAdviseSink_Impl,
    {
        unsafe extern "system" fn OnNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32
        where
            Identity: IMAPIAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIAdviseSink_Impl::OnNotify(this, core::mem::transmute_copy(&cnotif), core::mem::transmute_copy(&lpnotifications))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIContainer_Impl: Sized + IMAPIProp_Impl {
    fn GetContentsTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn GetHierarchyTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetSearchCriteria(&self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> windows_core::Result<()>;
    fn GetSearchCriteria(&self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIContainer_Vtbl
    where
        Identity: IMAPIContainer_Impl,
    {
        unsafe extern "system" fn GetContentsTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIContainer_Impl::GetContentsTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHierarchyTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIContainer_Impl::GetHierarchyTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIContainer_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn SetSearchCriteria<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIContainer_Impl::SetSearchCriteria(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&lpcontainerlist), core::mem::transmute_copy(&ulsearchflags)).into()
        }
        unsafe extern "system" fn GetSearchCriteria<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIContainer_Impl::GetSearchCriteria(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprestriction), core::mem::transmute_copy(&lppcontainerlist), core::mem::transmute_copy(&lpulsearchstate)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            GetContentsTable: GetContentsTable::<Identity, OFFSET>,
            GetHierarchyTable: GetHierarchyTable::<Identity, OFFSET>,
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            SetSearchCriteria: SetSearchCriteria::<Identity, OFFSET>,
            GetSearchCriteria: GetSearchCriteria::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIContainer as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
pub trait IMAPIControl_Impl: Sized {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR>;
    fn Activate(&self, ulflags: u32, uluiparam: usize) -> windows_core::Result<()>;
    fn GetState(&self, ulflags: u32, lpulstate: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMAPIControl {}
impl IMAPIControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIControl_Vtbl
    where
        Identity: IMAPIControl_Impl,
    {
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT
        where
            Identity: IMAPIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIControl_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppmapierror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, uluiparam: usize) -> windows_core::HRESULT
        where
            Identity: IMAPIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIControl_Impl::Activate(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&uluiparam)).into()
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIControl_Impl::GetState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIFolder_Impl: Sized + IMAPIContainer_Impl {
    fn CreateMessage(&self, lpinterface: *mut windows_core::GUID, ulflags: u32, lppmessage: *mut Option<IMessage>) -> windows_core::Result<()>;
    fn CopyMessages(&self, lpmsglist: *const SBinaryArray, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteMessages(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn CreateFolder(&self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IMAPIFolder>;
    fn CopyFolder(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn DeleteFolder(&self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn SetReadFlags(&self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn GetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32>;
    fn SetMessageStatus(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> windows_core::Result<u32>;
    fn SaveContentsSort(&self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::Result<()>;
    fn EmptyFolder(&self, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIFolder {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIFolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIFolder_Vtbl
    where
        Identity: IMAPIFolder_Impl,
    {
        unsafe extern "system" fn CreateMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, ulflags: u32, lppmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::CreateMessage(this, core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmessage)).into()
        }
        unsafe extern "system" fn CopyMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::CopyMessages(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestfolder), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::DeleteMessages(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn CreateFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const windows_core::GUID, ulflags: u32, lppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIFolder_Impl::CreateFolder(this, core::mem::transmute_copy(&ulfoldertype), core::mem::transmute_copy(&lpszfoldername), core::mem::transmute_copy(&lpszfoldercomment), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, lpdestfolder: *const core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::CopyFolder(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestfolder), core::mem::transmute_copy(&lpsznewfoldername), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::DeleteFolder(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetReadFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::SetReadFlags(this, core::mem::transmute_copy(&lpmsglist), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetMessageStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIFolder_Impl::GetMessageStatus(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpulmessagestatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMAPIFolder_Impl::SetMessageStatus(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulnewstatus), core::mem::transmute_copy(&ulnewstatusmask)) {
                Ok(ok__) => {
                    lpuloldstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveContentsSort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::SaveContentsSort(this, core::mem::transmute_copy(&lpsortcriteria), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn EmptyFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIFolder_Impl::EmptyFolder(this, core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IMAPIContainer_Vtbl::new::<Identity, OFFSET>(),
            CreateMessage: CreateMessage::<Identity, OFFSET>,
            CopyMessages: CopyMessages::<Identity, OFFSET>,
            DeleteMessages: DeleteMessages::<Identity, OFFSET>,
            CreateFolder: CreateFolder::<Identity, OFFSET>,
            CopyFolder: CopyFolder::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            SetReadFlags: SetReadFlags::<Identity, OFFSET>,
            GetMessageStatus: GetMessageStatus::<Identity, OFFSET>,
            SetMessageStatus: SetMessageStatus::<Identity, OFFSET>,
            SaveContentsSort: SaveContentsSort::<Identity, OFFSET>,
            EmptyFolder: EmptyFolder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIFolder as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID || iid == &<IMAPIContainer as windows_core::Interface>::IID
    }
}
pub trait IMAPIProgress_Impl: Sized {
    fn Progress(&self, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::Result<()>;
    fn GetFlags(&self, lpulflags: *mut u32) -> windows_core::Result<()>;
    fn GetMax(&self, lpulmax: *mut u32) -> windows_core::Result<()>;
    fn GetMin(&self, lpulmin: *mut u32) -> windows_core::Result<()>;
    fn SetLimits(&self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMAPIProgress {}
impl IMAPIProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIProgress_Vtbl
    where
        Identity: IMAPIProgress_Impl,
    {
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProgress_Impl::Progress(this, core::mem::transmute_copy(&ulvalue), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ultotal)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProgress_Impl::GetFlags(this, core::mem::transmute_copy(&lpulflags)).into()
        }
        unsafe extern "system" fn GetMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProgress_Impl::GetMax(this, core::mem::transmute_copy(&lpulmax)).into()
        }
        unsafe extern "system" fn GetMin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmin: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProgress_Impl::GetMin(this, core::mem::transmute_copy(&lpulmin)).into()
        }
        unsafe extern "system" fn SetLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProgress_Impl::SetLimits(this, core::mem::transmute_copy(&lpulmin), core::mem::transmute_copy(&lpulmax), core::mem::transmute_copy(&lpulflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Progress: Progress::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetMax: GetMax::<Identity, OFFSET>,
            GetMin: GetMin::<Identity, OFFSET>,
            SetLimits: SetLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIProgress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIProp_Impl: Sized {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn SaveChanges(&self, ulflags: u32) -> windows_core::Result<()>;
    fn GetProps(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::Result<()>;
    fn GetPropList(&self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()>;
    fn OpenProperty(&self, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProps(&self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn DeleteProps(&self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn CopyProps(&self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
    fn GetNamesFromIDs(&self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::Result<()>;
    fn GetIDsFromNames(&self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIProp {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIProp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIProp_Vtbl
    where
        Identity: IMAPIProp_Impl,
    {
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::SaveChanges(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::GetProps(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcvalues), core::mem::transmute_copy(&lppproparray)).into()
        }
        unsafe extern "system" fn GetPropList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::GetPropList(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproptagarray)).into()
        }
        unsafe extern "system" fn OpenProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulproptag: u32, lpiid: *mut windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::OpenProperty(this, core::mem::transmute_copy(&ulproptag), core::mem::transmute_copy(&lpiid), core::mem::transmute_copy(&ulinterfaceoptions), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn SetProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::SetProps(this, core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpproparray), core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn DeleteProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::DeleteProps(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::CopyTo(this, core::mem::transmute_copy(&ciidexclude), core::mem::transmute_copy(&rgiidexclude), core::mem::transmute_copy(&lpexcludeprops), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestobj), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn CopyProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut core::ffi::c_void, lpinterface: *mut windows_core::GUID, lpdestobj: *mut core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::CopyProps(this, core::mem::transmute_copy(&lpincludeprops), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&lpdestobj), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn GetNamesFromIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::GetNamesFromIDs(this, core::mem::transmute_copy(&lppproptags), core::mem::transmute_copy(&lppropsetguid), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcpropnames), core::mem::transmute_copy(&lppppropnames)).into()
        }
        unsafe extern "system" fn GetIDsFromNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> windows_core::HRESULT
        where
            Identity: IMAPIProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIProp_Impl::GetIDsFromNames(this, core::mem::transmute_copy(&cpropnames), core::mem::transmute_copy(&lpppropnames), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppproptags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            GetProps: GetProps::<Identity, OFFSET>,
            GetPropList: GetPropList::<Identity, OFFSET>,
            OpenProperty: OpenProperty::<Identity, OFFSET>,
            SetProps: SetProps::<Identity, OFFSET>,
            DeleteProps: DeleteProps::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            CopyProps: CopyProps::<Identity, OFFSET>,
            GetNamesFromIDs: GetNamesFromIDs::<Identity, OFFSET>,
            GetIDsFromNames: GetIDsFromNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPIStatus_Impl: Sized + IMAPIProp_Impl {
    fn ValidateState(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()>;
    fn SettingsDialog(&self, uluiparam: usize, ulflags: u32) -> windows_core::Result<()>;
    fn ChangePassword(&self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::Result<()>;
    fn FlushQueues(&self, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPIStatus {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPIStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPIStatus_Vtbl
    where
        Identity: IMAPIStatus_Impl,
    {
        unsafe extern "system" fn ValidateState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIStatus_Impl::ValidateState(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SettingsDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIStatus_Impl::SettingsDialog(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ChangePassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIStatus_Impl::ChangePassword(this, core::mem::transmute_copy(&lpoldpass), core::mem::transmute_copy(&lpnewpass), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn FlushQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPIStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPIStatus_Impl::FlushQueues(this, core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&cbtargettransport), core::mem::transmute_copy(&lptargettransport), core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            ValidateState: ValidateState::<Identity, OFFSET>,
            SettingsDialog: SettingsDialog::<Identity, OFFSET>,
            ChangePassword: ChangePassword::<Identity, OFFSET>,
            FlushQueues: FlushQueues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPIStatus as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMAPITable_Impl: Sized {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn Advise(&self, uleventmask: u32, lpadvisesink: Option<&IMAPIAdviseSink>, lpulconnection: *mut u32) -> windows_core::Result<()>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn GetStatus(&self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::Result<()>;
    fn SetColumns(&self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::Result<()>;
    fn QueryColumns(&self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::Result<()>;
    fn GetRowCount(&self, ulflags: u32, lpulcount: *mut u32) -> windows_core::Result<()>;
    fn SeekRow(&self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::Result<()>;
    fn SeekRowApprox(&self, ulnumerator: u32, uldenominator: u32) -> windows_core::Result<()>;
    fn QueryPosition(&self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::Result<()>;
    fn FindRow(&self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::Result<()>;
    fn Restrict(&self, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::Result<()>;
    fn CreateBookmark(&self, lpbkposition: *mut u32) -> windows_core::Result<()>;
    fn FreeBookmark(&self, bkposition: u32) -> windows_core::Result<()>;
    fn SortTable(&self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::Result<()>;
    fn QuerySortOrder(&self, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::Result<()>;
    fn QueryRows(&self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn ExpandRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::Result<()>;
    fn CollapseRow(&self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::Result<()>;
    fn GetCollapseState(&self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::Result<()>;
    fn SetCollapseState(&self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMAPITable {}
#[cfg(feature = "Win32_System_Com")]
impl IMAPITable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMAPITable_Vtbl
    where
        Identity: IMAPITable_Impl,
    {
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::Advise(this, core::mem::transmute_copy(&uleventmask), windows_core::from_raw_borrowed(&lpadvisesink), core::mem::transmute_copy(&lpulconnection)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::GetStatus(this, core::mem::transmute_copy(&lpultablestatus), core::mem::transmute_copy(&lpultabletype)).into()
        }
        unsafe extern "system" fn SetColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::SetColumns(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn QueryColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::QueryColumns(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpproptagarray)).into()
        }
        unsafe extern "system" fn GetRowCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::GetRowCount(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulcount)).into()
        }
        unsafe extern "system" fn SeekRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::SeekRow(this, core::mem::transmute_copy(&bkorigin), core::mem::transmute_copy(&lrowcount), core::mem::transmute_copy(&lplrowssought)).into()
        }
        unsafe extern "system" fn SeekRowApprox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::SeekRowApprox(this, core::mem::transmute_copy(&ulnumerator), core::mem::transmute_copy(&uldenominator)).into()
        }
        unsafe extern "system" fn QueryPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::QueryPosition(this, core::mem::transmute_copy(&lpulrow), core::mem::transmute_copy(&lpulnumerator), core::mem::transmute_copy(&lpuldenominator)).into()
        }
        unsafe extern "system" fn FindRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::FindRow(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&bkorigin), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn Restrict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::Restrict(this, core::mem::transmute_copy(&lprestriction), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn CreateBookmark<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbkposition: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::CreateBookmark(this, core::mem::transmute_copy(&lpbkposition)).into()
        }
        unsafe extern "system" fn FreeBookmark<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bkposition: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::FreeBookmark(this, core::mem::transmute_copy(&bkposition)).into()
        }
        unsafe extern "system" fn SortTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::SortTable(this, core::mem::transmute_copy(&lpsortcriteria), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn QuerySortOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::QuerySortOrder(this, core::mem::transmute_copy(&lppsortcriteria)).into()
        }
        unsafe extern "system" fn QueryRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::QueryRows(this, core::mem::transmute_copy(&lrowcount), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprows)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::Abort(this).into()
        }
        unsafe extern "system" fn ExpandRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::ExpandRow(this, core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&pbinstancekey), core::mem::transmute_copy(&ulrowcount), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpprows), core::mem::transmute_copy(&lpulmorerows)).into()
        }
        unsafe extern "system" fn CollapseRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::CollapseRow(this, core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&pbinstancekey), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulrowcount)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::WaitForCompletion(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ultimeout), core::mem::transmute_copy(&lpultablestatus)).into()
        }
        unsafe extern "system" fn GetCollapseState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::GetCollapseState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbinstancekey), core::mem::transmute_copy(&lpbinstancekey), core::mem::transmute_copy(&lpcbcollapsestate), core::mem::transmute_copy(&lppbcollapsestate)).into()
        }
        unsafe extern "system" fn SetCollapseState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMAPITable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMAPITable_Impl::SetCollapseState(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbcollapsestate), core::mem::transmute_copy(&pbcollapsestate), core::mem::transmute_copy(&lpbklocation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetColumns: SetColumns::<Identity, OFFSET>,
            QueryColumns: QueryColumns::<Identity, OFFSET>,
            GetRowCount: GetRowCount::<Identity, OFFSET>,
            SeekRow: SeekRow::<Identity, OFFSET>,
            SeekRowApprox: SeekRowApprox::<Identity, OFFSET>,
            QueryPosition: QueryPosition::<Identity, OFFSET>,
            FindRow: FindRow::<Identity, OFFSET>,
            Restrict: Restrict::<Identity, OFFSET>,
            CreateBookmark: CreateBookmark::<Identity, OFFSET>,
            FreeBookmark: FreeBookmark::<Identity, OFFSET>,
            SortTable: SortTable::<Identity, OFFSET>,
            QuerySortOrder: QuerySortOrder::<Identity, OFFSET>,
            QueryRows: QueryRows::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            ExpandRow: ExpandRow::<Identity, OFFSET>,
            CollapseRow: CollapseRow::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            GetCollapseState: GetCollapseState::<Identity, OFFSET>,
            SetCollapseState: SetCollapseState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMAPITable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMailUser_Impl: Sized + IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMailUser {}
#[cfg(feature = "Win32_System_Com")]
impl IMailUser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMailUser_Vtbl
    where
        Identity: IMailUser_Impl,
    {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMailUser as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessage_Impl: Sized + IMAPIProp_Impl {
    fn GetAttachmentTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn OpenAttach(&self, ulattachmentnum: u32, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IAttach>;
    fn CreateAttach(&self, lpinterface: *const windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut Option<IAttach>) -> windows_core::Result<()>;
    fn DeleteAttach(&self, ulattachmentnum: u32, uluiparam: usize, lpprogress: Option<&IMAPIProgress>, ulflags: u32) -> windows_core::Result<()>;
    fn GetRecipientTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn ModifyRecipients(&self, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::Result<()>;
    fn SubmitMessage(&self, ulflags: u32) -> windows_core::Result<()>;
    fn SetReadFlag(&self, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMessage {}
#[cfg(feature = "Win32_System_Com")]
impl IMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMessage_Vtbl
    where
        Identity: IMessage_Impl,
    {
        unsafe extern "system" fn GetAttachmentTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessage_Impl::GetAttachmentTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAttach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattachmentnum: u32, lpinterface: *const windows_core::GUID, ulflags: u32, lppattach: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessage_Impl::OpenAttach(this, core::mem::transmute_copy(&ulattachmentnum), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppattach.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinterface: *const windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessage_Impl::CreateAttach(this, core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulattachmentnum), core::mem::transmute_copy(&lppattach)).into()
        }
        unsafe extern "system" fn DeleteAttach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessage_Impl::DeleteAttach(this, core::mem::transmute_copy(&ulattachmentnum), core::mem::transmute_copy(&uluiparam), windows_core::from_raw_borrowed(&lpprogress), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetRecipientTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessage_Impl::GetRecipientTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyRecipients<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessage_Impl::ModifyRecipients(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpmods)).into()
        }
        unsafe extern "system" fn SubmitMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessage_Impl::SubmitMessage(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetReadFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessage_Impl::SetReadFlag(this, core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            GetAttachmentTable: GetAttachmentTable::<Identity, OFFSET>,
            OpenAttach: OpenAttach::<Identity, OFFSET>,
            CreateAttach: CreateAttach::<Identity, OFFSET>,
            DeleteAttach: DeleteAttach::<Identity, OFFSET>,
            GetRecipientTable: GetRecipientTable::<Identity, OFFSET>,
            ModifyRecipients: ModifyRecipients::<Identity, OFFSET>,
            SubmitMessage: SubmitMessage::<Identity, OFFSET>,
            SetReadFlag: SetReadFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessage as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsgStore_Impl: Sized + IMAPIProp_Impl {
    fn Advise(&self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: Option<&IMAPIAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, ulconnection: u32) -> windows_core::Result<()>;
    fn CompareEntryIDs(&self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> windows_core::Result<u32>;
    fn OpenEntry(&self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()>;
    fn GetReceiveFolder(&self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::Result<()>;
    fn GetReceiveFolderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn StoreLogoff(&self, lpulflags: *mut u32) -> windows_core::Result<()>;
    fn AbortSubmit(&self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::Result<()>;
    fn GetOutgoingQueue(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn SetLockState(&self, lpmessage: Option<&IMessage>, ullockstate: u32) -> windows_core::Result<()>;
    fn FinishedMsg(&self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::Result<()>;
    fn NotifyNewMail(&self, lpnotification: *const NOTIFICATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsgStore {}
#[cfg(feature = "Win32_System_Com")]
impl IMsgStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsgStore_Vtbl
    where
        Identity: IMsgStore_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: *mut core::ffi::c_void, lpulconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsgStore_Impl::Advise(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&uleventmask), windows_core::from_raw_borrowed(&lpadvisesink)) {
                Ok(ok__) => {
                    lpulconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulconnection: u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::Unadvise(this, core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsgStore_Impl::CompareEntryIDs(this, core::mem::transmute_copy(&cbentryid1), core::mem::transmute_copy(&lpentryid1), core::mem::transmute_copy(&cbentryid2), core::mem::transmute_copy(&lpentryid2), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpulresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::OpenEntry(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpulobjtype), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn SetReceiveFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::SetReceiveFolder(this, core::mem::transmute_copy(&lpszmessageclass), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetReceiveFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::GetReceiveFolder(this, core::mem::transmute_copy(&lpszmessageclass), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpcbentryid), core::mem::transmute_copy(&lppentryid), core::mem::transmute_copy(&lppszexplicitclass)).into()
        }
        unsafe extern "system" fn GetReceiveFolderTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsgStore_Impl::GetReceiveFolderTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreLogoff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpulflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::StoreLogoff(this, core::mem::transmute_copy(&lpulflags)).into()
        }
        unsafe extern "system" fn AbortSubmit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::AbortSubmit(this, core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid), core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetOutgoingQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsgStore_Impl::GetOutgoingQueue(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmessage: *mut core::ffi::c_void, ullockstate: u32) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::SetLockState(this, windows_core::from_raw_borrowed(&lpmessage), core::mem::transmute_copy(&ullockstate)).into()
        }
        unsafe extern "system" fn FinishedMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::FinishedMsg(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cbentryid), core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn NotifyNewMail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnotification: *const NOTIFICATION) -> windows_core::HRESULT
        where
            Identity: IMsgStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsgStore_Impl::NotifyNewMail(this, core::mem::transmute_copy(&lpnotification)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, OFFSET>,
            OpenEntry: OpenEntry::<Identity, OFFSET>,
            SetReceiveFolder: SetReceiveFolder::<Identity, OFFSET>,
            GetReceiveFolder: GetReceiveFolder::<Identity, OFFSET>,
            GetReceiveFolderTable: GetReceiveFolderTable::<Identity, OFFSET>,
            StoreLogoff: StoreLogoff::<Identity, OFFSET>,
            AbortSubmit: AbortSubmit::<Identity, OFFSET>,
            GetOutgoingQueue: GetOutgoingQueue::<Identity, OFFSET>,
            SetLockState: SetLockState::<Identity, OFFSET>,
            FinishedMsg: FinishedMsg::<Identity, OFFSET>,
            NotifyNewMail: NotifyNewMail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsgStore as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProfSect_Impl: Sized + IMAPIProp_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProfSect {}
#[cfg(feature = "Win32_System_Com")]
impl IProfSect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProfSect_Vtbl
    where
        Identity: IProfSect_Impl,
    {
        Self { base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProfSect as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPropData_Impl: Sized + IMAPIProp_Impl {
    fn HrSetObjAccess(&self, ulaccess: u32) -> windows_core::Result<()>;
    fn HrSetPropAccess(&self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::Result<()>;
    fn HrGetPropAccess(&self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::Result<()>;
    fn HrAddObjProps(&self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPropData {}
#[cfg(feature = "Win32_System_Com")]
impl IPropData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropData_Vtbl
    where
        Identity: IPropData_Impl,
    {
        unsafe extern "system" fn HrSetObjAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulaccess: u32) -> windows_core::HRESULT
        where
            Identity: IPropData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropData_Impl::HrSetObjAccess(this, core::mem::transmute_copy(&ulaccess)).into()
        }
        unsafe extern "system" fn HrSetPropAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPropData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropData_Impl::HrSetPropAccess(this, core::mem::transmute_copy(&lpproptagarray), core::mem::transmute_copy(&rgulaccess)).into()
        }
        unsafe extern "system" fn HrGetPropAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IPropData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropData_Impl::HrGetPropAccess(this, core::mem::transmute_copy(&lppproptagarray), core::mem::transmute_copy(&lprgulaccess)).into()
        }
        unsafe extern "system" fn HrAddObjProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> windows_core::HRESULT
        where
            Identity: IPropData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropData_Impl::HrAddObjProps(this, core::mem::transmute_copy(&lppproptagarray), core::mem::transmute_copy(&lprgulaccess)).into()
        }
        Self {
            base__: IMAPIProp_Vtbl::new::<Identity, OFFSET>(),
            HrSetObjAccess: HrSetObjAccess::<Identity, OFFSET>,
            HrSetPropAccess: HrSetPropAccess::<Identity, OFFSET>,
            HrGetPropAccess: HrGetPropAccess::<Identity, OFFSET>,
            HrAddObjProps: HrAddObjProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropData as windows_core::Interface>::IID || iid == &<IMAPIProp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderAdmin_Impl: Sized {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32) -> windows_core::Result<*mut MAPIERROR>;
    fn GetProviderTable(&self, ulflags: u32) -> windows_core::Result<IMAPITable>;
    fn CreateProvider(&self, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32) -> windows_core::Result<MAPIUID>;
    fn DeleteProvider(&self, lpuid: *const MAPIUID) -> windows_core::Result<()>;
    fn OpenProfileSection(&self, lpuid: *const MAPIUID, lpinterface: *const windows_core::GUID, ulflags: u32) -> windows_core::Result<IProfSect>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProviderAdmin {}
#[cfg(feature = "Win32_System_Com")]
impl IProviderAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderAdmin_Vtbl
    where
        Identity: IProviderAdmin_Impl,
    {
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT
        where
            Identity: IProviderAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderAdmin_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppmapierror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderAdmin_Impl::GetProviderTable(this, core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> windows_core::HRESULT
        where
            Identity: IProviderAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderAdmin_Impl::CreateProvider(this, core::mem::transmute_copy(&lpszprovider), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpprops), core::mem::transmute_copy(&uluiparam), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lpuid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuid: *const MAPIUID) -> windows_core::HRESULT
        where
            Identity: IProviderAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderAdmin_Impl::DeleteProvider(this, core::mem::transmute_copy(&lpuid)).into()
        }
        unsafe extern "system" fn OpenProfileSection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: *const windows_core::GUID, ulflags: u32, lppprofsect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProviderAdmin_Impl::OpenProfileSection(this, core::mem::transmute_copy(&lpuid), core::mem::transmute_copy(&lpinterface), core::mem::transmute_copy(&ulflags)) {
                Ok(ok__) => {
                    lppprofsect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            GetProviderTable: GetProviderTable::<Identity, OFFSET>,
            CreateProvider: CreateProvider::<Identity, OFFSET>,
            DeleteProvider: DeleteProvider::<Identity, OFFSET>,
            OpenProfileSection: OpenProfileSection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderAdmin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITableData_Impl: Sized {
    fn HrGetView(&self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut Option<IMAPITable>) -> windows_core::Result<()>;
    fn HrModifyRow(&self, param0: *mut SRow) -> windows_core::Result<()>;
    fn HrDeleteRow(&self, lpspropvalue: *mut SPropValue) -> windows_core::Result<()>;
    fn HrQueryRow(&self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::Result<()>;
    fn HrEnumRow(&self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::Result<()>;
    fn HrNotify(&self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::Result<()>;
    fn HrInsertRow(&self, ulirow: u32, lpsrow: *mut SRow) -> windows_core::Result<()>;
    fn HrModifyRows(&self, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::Result<()>;
    fn HrDeleteRows(&self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITableData {}
#[cfg(feature = "Win32_System_Com")]
impl ITableData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITableData_Vtbl
    where
        Identity: ITableData_Impl,
    {
        unsafe extern "system" fn HrGetView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrGetView(this, core::mem::transmute_copy(&lpssortorderset), core::mem::transmute_copy(&lpfcallerrelease), core::mem::transmute_copy(&ulcallerdata), core::mem::transmute_copy(&lppmapitable)).into()
        }
        unsafe extern "system" fn HrModifyRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut SRow) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrModifyRow(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn HrDeleteRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpspropvalue: *mut SPropValue) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrDeleteRow(this, core::mem::transmute_copy(&lpspropvalue)).into()
        }
        unsafe extern "system" fn HrQueryRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrQueryRow(this, core::mem::transmute_copy(&lpspropvalue), core::mem::transmute_copy(&lppsrow), core::mem::transmute_copy(&lpulirow)).into()
        }
        unsafe extern "system" fn HrEnumRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrEnumRow(this, core::mem::transmute_copy(&ulrownumber), core::mem::transmute_copy(&lppsrow)).into()
        }
        unsafe extern "system" fn HrNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrNotify(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&lpspropvalue)).into()
        }
        unsafe extern "system" fn HrInsertRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrInsertRow(this, core::mem::transmute_copy(&ulirow), core::mem::transmute_copy(&lpsrow)).into()
        }
        unsafe extern "system" fn HrModifyRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrModifyRows(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpsrowset)).into()
        }
        unsafe extern "system" fn HrDeleteRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITableData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableData_Impl::HrDeleteRows(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lprowsettodelete), core::mem::transmute_copy(&crowsdeleted)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HrGetView: HrGetView::<Identity, OFFSET>,
            HrModifyRow: HrModifyRow::<Identity, OFFSET>,
            HrDeleteRow: HrDeleteRow::<Identity, OFFSET>,
            HrQueryRow: HrQueryRow::<Identity, OFFSET>,
            HrEnumRow: HrEnumRow::<Identity, OFFSET>,
            HrNotify: HrNotify::<Identity, OFFSET>,
            HrInsertRow: HrInsertRow::<Identity, OFFSET>,
            HrModifyRows: HrModifyRows::<Identity, OFFSET>,
            HrDeleteRows: HrDeleteRows::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableData as windows_core::Interface>::IID
    }
}
pub trait IWABExtInit_Impl: Sized {
    fn Initialize(&self, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWABExtInit {}
impl IWABExtInit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWABExtInit_Vtbl
    where
        Identity: IWABExtInit_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> windows_core::HRESULT
        where
            Identity: IWABExtInit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABExtInit_Impl::Initialize(this, core::mem::transmute_copy(&lpwabextdisplay)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWABExtInit as windows_core::Interface>::IID
    }
}
pub trait IWABObject_Impl: Sized {
    fn GetLastError(&self, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::Result<()>;
    fn AllocateBuffer(&self, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateMore(&self, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeBuffer(&self, lpbuffer: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Backup(&self, lpfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn Import(&self, lpwip: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn Find(&self, lpiab: Option<&IAddrBook>, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn VCardDisplay(&self, lpiab: Option<&IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LDAPUrl(&self, lpiab: Option<&IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: &windows_core::PCSTR) -> windows_core::Result<IMailUser>;
    fn VCardCreate(&self, lpiab: Option<&IAddrBook>, ulflags: u32, lpszvcard: &windows_core::PCSTR, lpmailuser: Option<&IMailUser>) -> windows_core::Result<()>;
    fn VCardRetrieve(&self, lpiab: Option<&IAddrBook>, ulflags: u32, lpszvcard: &windows_core::PCSTR) -> windows_core::Result<IMailUser>;
    fn GetMe(&self, lpiab: Option<&IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn SetMe(&self, lpiab: Option<&IAddrBook>, ulflags: u32, sbeid: &SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWABObject {}
impl IWABObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWABObject_Vtbl
    where
        Identity: IWABObject_Impl,
    {
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::GetLastError(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn AllocateBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::AllocateBuffer(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn AllocateMore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, lpobject: *const core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::AllocateMore(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&lpobject), core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::FreeBuffer(this, core::mem::transmute_copy(&lpbuffer)).into()
        }
        unsafe extern "system" fn Backup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpfilename: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::Backup(this, core::mem::transmute(&lpfilename)).into()
        }
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpwip: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::Import(this, core::mem::transmute(&lpwip)).into()
        }
        unsafe extern "system" fn Find<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::Find(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn VCardDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::VCardDisplay(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&hwnd), core::mem::transmute(&lpszfilename)).into()
        }
        unsafe extern "system" fn LDAPUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: windows_core::PCSTR, lppmailuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWABObject_Impl::LDAPUrl(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszurl)) {
                Ok(ok__) => {
                    lppmailuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpszvcard: windows_core::PCSTR, lpmailuser: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::VCardCreate(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszvcard), windows_core::from_raw_borrowed(&lpmailuser)).into()
        }
        unsafe extern "system" fn VCardRetrieve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpszvcard: windows_core::PCSTR, lppmailuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWABObject_Impl::VCardRetrieve(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&lpszvcard)) {
                Ok(ok__) => {
                    lppmailuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::GetMe(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&lpdwaction), core::mem::transmute_copy(&lpsbeid), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn SetMe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiab: *mut core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWABObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWABObject_Impl::SetMe(this, windows_core::from_raw_borrowed(&lpiab), core::mem::transmute_copy(&ulflags), core::mem::transmute(&sbeid), core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            AllocateMore: AllocateMore::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            Backup: Backup::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            Find: Find::<Identity, OFFSET>,
            VCardDisplay: VCardDisplay::<Identity, OFFSET>,
            LDAPUrl: LDAPUrl::<Identity, OFFSET>,
            VCardCreate: VCardCreate::<Identity, OFFSET>,
            VCardRetrieve: VCardRetrieve::<Identity, OFFSET>,
            GetMe: GetMe::<Identity, OFFSET>,
            SetMe: SetMe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWABObject as windows_core::Interface>::IID
    }
}
