pub trait IGameExplorer_Impl: Sized {
    fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IGameExplorer {}
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameExplorer_Vtbl
    where
        Identity: IGameExplorer_Impl,
    {
        unsafe extern "system" fn AddGame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: core::mem::MaybeUninit<windows_core::BSTR>, bstrgameinstalldirectory: core::mem::MaybeUninit<windows_core::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IGameExplorer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameExplorer_Impl::AddGame(this, core::mem::transmute(&bstrgdfbinarypath), core::mem::transmute(&bstrgameinstalldirectory), core::mem::transmute_copy(&installscope), core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn RemoveGame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IGameExplorer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameExplorer_Impl::RemoveGame(this, core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn UpdateGame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IGameExplorer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameExplorer_Impl::UpdateGame(this, core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn VerifyAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: core::mem::MaybeUninit<windows_core::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGameExplorer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameExplorer_Impl::VerifyAccess(this, core::mem::transmute(&bstrgdfbinarypath)) {
                Ok(ok__) => {
                    pfhasaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, OFFSET>,
            RemoveGame: RemoveGame::<Identity, OFFSET>,
            UpdateGame: UpdateGame::<Identity, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer as windows_core::Interface>::IID
    }
}
pub trait IGameExplorer2_Impl: Sized {
    fn InstallGame(&self, binarygdfpath: &windows_core::PCWSTR, installdirectory: &windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::Result<()>;
    fn UninstallGame(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CheckAccess(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IGameExplorer2 {}
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameExplorer2_Vtbl
    where
        Identity: IGameExplorer2_Impl,
    {
        unsafe extern "system" fn InstallGame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, installdirectory: windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::HRESULT
        where
            Identity: IGameExplorer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameExplorer2_Impl::InstallGame(this, core::mem::transmute(&binarygdfpath), core::mem::transmute(&installdirectory), core::mem::transmute_copy(&installscope)).into()
        }
        unsafe extern "system" fn UninstallGame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IGameExplorer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameExplorer2_Impl::UninstallGame(this, core::mem::transmute(&binarygdfpath)).into()
        }
        unsafe extern "system" fn CheckAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGameExplorer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameExplorer2_Impl::CheckAccess(this, core::mem::transmute(&binarygdfpath)) {
                Ok(ok__) => {
                    phasaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, OFFSET>,
            UninstallGame: UninstallGame::<Identity, OFFSET>,
            CheckAccess: CheckAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer2 as windows_core::Interface>::IID
    }
}
pub trait IGameStatistics_Impl: Sized {
    fn GetMaxCategoryLength(&self) -> windows_core::Result<u32>;
    fn GetMaxNameLength(&self) -> windows_core::Result<u32>;
    fn GetMaxValueLength(&self) -> windows_core::Result<u32>;
    fn GetMaxCategories(&self) -> windows_core::Result<u16>;
    fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16>;
    fn SetCategoryTitle(&self, categoryindex: u16, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR>;
    fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetStatistic(&self, categoryindex: u16, statindex: u16, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Save(&self, trackchanges: super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::Result<()>;
    fn GetLastPlayedCategory(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IGameStatistics {}
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameStatistics_Vtbl
    where
        Identity: IGameStatistics_Impl,
    {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetMaxCategoryLength(this) {
                Ok(ok__) => {
                    cch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetMaxNameLength(this) {
                Ok(ok__) => {
                    cch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetMaxValueLength(this) {
                Ok(ok__) => {
                    cch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetMaxCategories(this) {
                Ok(ok__) => {
                    pmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetMaxStatsPerCategory(this) {
                Ok(ok__) => {
                    pmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, title: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatistics_Impl::SetCategoryTitle(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, ptitle: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetCategoryTitle(this, core::mem::transmute_copy(&categoryindex)) {
                Ok(ok__) => {
                    ptitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatistics_Impl::GetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetStatistic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatistics_Impl::SetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatistics_Impl::Save(this, core::mem::transmute_copy(&trackchanges)).into()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u32) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatistics_Impl::SetLastPlayedCategory(this, core::mem::transmute_copy(&categoryindex)).into()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGameStatistics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameStatistics_Impl::GetLastPlayedCategory(this) {
                Ok(ok__) => {
                    pcategoryindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, OFFSET>,
            GetStatistic: GetStatistic::<Identity, OFFSET>,
            SetStatistic: SetStatistic::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatistics as windows_core::Interface>::IID
    }
}
pub trait IGameStatisticsMgr_Impl: Sized {
    fn GetGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut Option<IGameStatistics>) -> windows_core::Result<()>;
    fn RemoveGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGameStatisticsMgr {}
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameStatisticsMgr_Vtbl
    where
        Identity: IGameStatisticsMgr_Impl,
    {
        unsafe extern "system" fn GetGameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameStatisticsMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatisticsMgr_Impl::GetGameStatistics(this, core::mem::transmute(&gdfbinarypath), core::mem::transmute_copy(&opentype), core::mem::transmute_copy(&popenresult), core::mem::transmute_copy(&ppistats)).into()
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IGameStatisticsMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameStatisticsMgr_Impl::RemoveGameStatistics(this, core::mem::transmute(&gdfbinarypath)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as windows_core::Interface>::IID
    }
}
pub trait IXblIdpAuthManager_Impl: Sized {
    fn SetGamerAccount(&self, msaaccountid: &windows_core::PCWSTR, xuid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGamerAccount(&self, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetAppViewInitialized(&self, appsid: &windows_core::PCWSTR, msaaccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(&self, msaaccountid: &windows_core::PCWSTR, appsid: &windows_core::PCWSTR, msatarget: &windows_core::PCWSTR, msapolicy: &windows_core::PCWSTR, httpmethod: &windows_core::PCWSTR, uri: &windows_core::PCWSTR, headers: &windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> windows_core::Result<IXblIdpAuthTokenResult>;
}
impl windows_core::RuntimeName for IXblIdpAuthManager {}
impl IXblIdpAuthManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXblIdpAuthManager_Vtbl
    where
        Identity: IXblIdpAuthManager_Impl,
    {
        unsafe extern "system" fn SetGamerAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, xuid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXblIdpAuthManager_Impl::SetGamerAccount(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&xuid)).into()
        }
        unsafe extern "system" fn GetGamerAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXblIdpAuthManager_Impl::GetGamerAccount(this, core::mem::transmute_copy(&msaaccountid), core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msaaccountid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXblIdpAuthManager_Impl::SetAppViewInitialized(this, core::mem::transmute(&appsid), core::mem::transmute(&msaaccountid)).into()
        }
        unsafe extern "system" fn GetEnvironment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthManager_Impl::GetEnvironment(this) {
                Ok(ok__) => {
                    environment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthManager_Impl::GetSandbox(this) {
                Ok(ok__) => {
                    sandbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthManager_Impl::GetTokenAndSignatureWithTokenResult(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Identity, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, OFFSET>,
            GetSandbox: GetSandbox::<Identity, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as windows_core::Interface>::IID
    }
}
pub trait IXblIdpAuthManager2_Impl: Sized {
    fn GetUserlessTokenAndSignatureWithTokenResult(&self, appsid: &windows_core::PCWSTR, msatarget: &windows_core::PCWSTR, msapolicy: &windows_core::PCWSTR, httpmethod: &windows_core::PCWSTR, uri: &windows_core::PCWSTR, headers: &windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> windows_core::Result<IXblIdpAuthTokenResult>;
}
impl windows_core::RuntimeName for IXblIdpAuthManager2 {}
impl IXblIdpAuthManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXblIdpAuthManager2_Vtbl
    where
        Identity: IXblIdpAuthManager2_Impl,
    {
        unsafe extern "system" fn GetUserlessTokenAndSignatureWithTokenResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthManager2_Impl::GetUserlessTokenAndSignatureWithTokenResult(this, core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserlessTokenAndSignatureWithTokenResult: GetUserlessTokenAndSignatureWithTokenResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthManager2 as windows_core::Interface>::IID
    }
}
pub trait IXblIdpAuthTokenResult_Impl: Sized {
    fn GetStatus(&self) -> windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetToken(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignature(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaAccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetXuid(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAgeGroup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPrivileges(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaTarget(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaPolicy(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaAppId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRedirect(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMessage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHelpId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEnforcementBans(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTitleRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IXblIdpAuthTokenResult {}
impl IXblIdpAuthTokenResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXblIdpAuthTokenResult_Vtbl
    where
        Identity: IXblIdpAuthTokenResult_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetErrorCode(this) {
                Ok(ok__) => {
                    errorcode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetToken(this) {
                Ok(ok__) => {
                    token.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetSignature(this) {
                Ok(ok__) => {
                    signature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetSandbox(this) {
                Ok(ok__) => {
                    sandbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetEnvironment(this) {
                Ok(ok__) => {
                    environment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetMsaAccountId(this) {
                Ok(ok__) => {
                    msaaccountid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetXuid(this) {
                Ok(ok__) => {
                    xuid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamertag: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetGamertag(this) {
                Ok(ok__) => {
                    gamertag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, agegroup: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetAgeGroup(this) {
                Ok(ok__) => {
                    agegroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, privileges: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetPrivileges(this) {
                Ok(ok__) => {
                    privileges.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msatarget: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetMsaTarget(this) {
                Ok(ok__) => {
                    msatarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msapolicy: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetMsaPolicy(this) {
                Ok(ok__) => {
                    msapolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaappid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetMsaAppId(this) {
                Ok(ok__) => {
                    msaappid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, redirect: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetRedirect(this) {
                Ok(ok__) => {
                    redirect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetMessage(this) {
                Ok(ok__) => {
                    message.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, helpid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetHelpId(this) {
                Ok(ok__) => {
                    helpid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enforcementbans: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetEnforcementBans(this) {
                Ok(ok__) => {
                    enforcementbans.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetRestrictions(this) {
                Ok(ok__) => {
                    restrictions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, titlerestrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult_Impl::GetTitleRestrictions(this) {
                Ok(ok__) => {
                    titlerestrictions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, OFFSET>,
            GetToken: GetToken::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            GetSandbox: GetSandbox::<Identity, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Identity, OFFSET>,
            GetXuid: GetXuid::<Identity, OFFSET>,
            GetGamertag: GetGamertag::<Identity, OFFSET>,
            GetAgeGroup: GetAgeGroup::<Identity, OFFSET>,
            GetPrivileges: GetPrivileges::<Identity, OFFSET>,
            GetMsaTarget: GetMsaTarget::<Identity, OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Identity, OFFSET>,
            GetMsaAppId: GetMsaAppId::<Identity, OFFSET>,
            GetRedirect: GetRedirect::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetHelpId: GetHelpId::<Identity, OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Identity, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult as windows_core::Interface>::IID
    }
}
pub trait IXblIdpAuthTokenResult2_Impl: Sized {
    fn GetModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetModernGamertagSuffix(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUniqueModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IXblIdpAuthTokenResult2 {}
impl IXblIdpAuthTokenResult2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXblIdpAuthTokenResult2_Vtbl
    where
        Identity: IXblIdpAuthTokenResult2_Impl,
    {
        unsafe extern "system" fn GetModernGamertag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult2_Impl::GetModernGamertag(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult2_Impl::GetModernGamertagSuffix(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXblIdpAuthTokenResult2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXblIdpAuthTokenResult2_Impl::GetUniqueModernGamertag(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Identity, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as windows_core::Interface>::IID
    }
}
