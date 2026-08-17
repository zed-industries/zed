pub trait IGameExplorer_Impl: Sized {
    fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IGameExplorer {}
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>() -> IGameExplorer_Vtbl {
        unsafe extern "system" fn AddGame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: core::mem::MaybeUninit<windows_core::BSTR>, bstrgameinstalldirectory: core::mem::MaybeUninit<windows_core::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameExplorer_Impl::AddGame(this, core::mem::transmute(&bstrgdfbinarypath), core::mem::transmute(&bstrgameinstalldirectory), core::mem::transmute_copy(&installscope), core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn RemoveGame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameExplorer_Impl::RemoveGame(this, core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn UpdateGame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameExplorer_Impl::UpdateGame(this, core::mem::transmute(&guidinstanceid)).into()
        }
        unsafe extern "system" fn VerifyAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: core::mem::MaybeUninit<windows_core::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameExplorer_Impl::VerifyAccess(this, core::mem::transmute(&bstrgdfbinarypath)) {
                Ok(ok__) => {
                    core::ptr::write(pfhasaccess, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, Impl, OFFSET>,
            RemoveGame: RemoveGame::<Identity, Impl, OFFSET>,
            UpdateGame: UpdateGame::<Identity, Impl, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>() -> IGameExplorer2_Vtbl {
        unsafe extern "system" fn InstallGame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, installdirectory: windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameExplorer2_Impl::InstallGame(this, core::mem::transmute(&binarygdfpath), core::mem::transmute(&installdirectory), core::mem::transmute_copy(&installscope)).into()
        }
        unsafe extern "system" fn UninstallGame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameExplorer2_Impl::UninstallGame(this, core::mem::transmute(&binarygdfpath)).into()
        }
        unsafe extern "system" fn CheckAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameExplorer2_Impl::CheckAccess(this, core::mem::transmute(&binarygdfpath)) {
                Ok(ok__) => {
                    core::ptr::write(phasaccess, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, Impl, OFFSET>,
            UninstallGame: UninstallGame::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>() -> IGameStatistics_Vtbl {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetMaxCategoryLength(this) {
                Ok(ok__) => {
                    core::ptr::write(cch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetMaxNameLength(this) {
                Ok(ok__) => {
                    core::ptr::write(cch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetMaxValueLength(this) {
                Ok(ok__) => {
                    core::ptr::write(cch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetMaxCategories(this) {
                Ok(ok__) => {
                    core::ptr::write(pmax, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetMaxStatsPerCategory(this) {
                Ok(ok__) => {
                    core::ptr::write(pmax, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatistics_Impl::SetCategoryTitle(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, ptitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetCategoryTitle(this, core::mem::transmute_copy(&categoryindex)) {
                Ok(ok__) => {
                    core::ptr::write(ptitle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatistics_Impl::GetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetStatistic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatistics_Impl::SetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatistics_Impl::Save(this, core::mem::transmute_copy(&trackchanges)).into()
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatistics_Impl::SetLastPlayedCategory(this, core::mem::transmute_copy(&categoryindex)).into()
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGameStatistics_Impl::GetLastPlayedCategory(this) {
                Ok(ok__) => {
                    core::ptr::write(pcategoryindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, Impl, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, Impl, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, Impl, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, Impl, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, Impl, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, Impl, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, Impl, OFFSET>,
            GetStatistic: GetStatistic::<Identity, Impl, OFFSET>,
            SetStatistic: SetStatistic::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, Impl, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>() -> IGameStatisticsMgr_Vtbl {
        unsafe extern "system" fn GetGameStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatisticsMgr_Impl::GetGameStatistics(this, core::mem::transmute(&gdfbinarypath), core::mem::transmute_copy(&opentype), core::mem::transmute_copy(&popenresult), core::mem::transmute_copy(&ppistats)).into()
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGameStatisticsMgr_Impl::RemoveGameStatistics(this, core::mem::transmute(&gdfbinarypath)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, Impl, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>() -> IXblIdpAuthManager_Vtbl {
        unsafe extern "system" fn SetGamerAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, xuid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXblIdpAuthManager_Impl::SetGamerAccount(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&xuid)).into()
        }
        unsafe extern "system" fn GetGamerAccount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXblIdpAuthManager_Impl::GetGamerAccount(this, core::mem::transmute_copy(&msaaccountid), core::mem::transmute_copy(&xuid)).into()
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msaaccountid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXblIdpAuthManager_Impl::SetAppViewInitialized(this, core::mem::transmute(&appsid), core::mem::transmute(&msaaccountid)).into()
        }
        unsafe extern "system" fn GetEnvironment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthManager_Impl::GetEnvironment(this) {
                Ok(ok__) => {
                    core::ptr::write(environment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthManager_Impl::GetSandbox(this) {
                Ok(ok__) => {
                    core::ptr::write(sandbox, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthManager_Impl::GetTokenAndSignatureWithTokenResult(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Identity, Impl, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, Impl, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager2_Impl, const OFFSET: isize>() -> IXblIdpAuthManager2_Vtbl {
        unsafe extern "system" fn GetUserlessTokenAndSignatureWithTokenResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthManager2_Impl::GetUserlessTokenAndSignatureWithTokenResult(this, core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserlessTokenAndSignatureWithTokenResult: GetUserlessTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetErrorCode(this) {
                Ok(ok__) => {
                    core::ptr::write(errorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToken<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetToken(this) {
                Ok(ok__) => {
                    core::ptr::write(token, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetSignature(this) {
                Ok(ok__) => {
                    core::ptr::write(signature, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetSandbox(this) {
                Ok(ok__) => {
                    core::ptr::write(sandbox, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetEnvironment(this) {
                Ok(ok__) => {
                    core::ptr::write(environment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetMsaAccountId(this) {
                Ok(ok__) => {
                    core::ptr::write(msaaccountid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetXuid(this) {
                Ok(ok__) => {
                    core::ptr::write(xuid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamertag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamertag: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetGamertag(this) {
                Ok(ok__) => {
                    core::ptr::write(gamertag, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAgeGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, agegroup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetAgeGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(agegroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivileges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privileges: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetPrivileges(this) {
                Ok(ok__) => {
                    core::ptr::write(privileges, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msatarget: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetMsaTarget(this) {
                Ok(ok__) => {
                    core::ptr::write(msatarget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msapolicy: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetMsaPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(msapolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMsaAppId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaappid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetMsaAppId(this) {
                Ok(ok__) => {
                    core::ptr::write(msaappid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRedirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, redirect: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetRedirect(this) {
                Ok(ok__) => {
                    core::ptr::write(redirect, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetMessage(this) {
                Ok(ok__) => {
                    core::ptr::write(message, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, helpid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetHelpId(this) {
                Ok(ok__) => {
                    core::ptr::write(helpid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enforcementbans: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetEnforcementBans(this) {
                Ok(ok__) => {
                    core::ptr::write(enforcementbans, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetRestrictions(this) {
                Ok(ok__) => {
                    core::ptr::write(restrictions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, titlerestrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult_Impl::GetTitleRestrictions(this) {
                Ok(ok__) => {
                    core::ptr::write(titlerestrictions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Identity, Impl, OFFSET>,
            GetXuid: GetXuid::<Identity, Impl, OFFSET>,
            GetGamertag: GetGamertag::<Identity, Impl, OFFSET>,
            GetAgeGroup: GetAgeGroup::<Identity, Impl, OFFSET>,
            GetPrivileges: GetPrivileges::<Identity, Impl, OFFSET>,
            GetMsaTarget: GetMsaTarget::<Identity, Impl, OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Identity, Impl, OFFSET>,
            GetMsaAppId: GetMsaAppId::<Identity, Impl, OFFSET>,
            GetRedirect: GetRedirect::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetHelpId: GetHelpId::<Identity, Impl, OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Identity, Impl, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, Impl, OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>() -> IXblIdpAuthTokenResult2_Vtbl {
        unsafe extern "system" fn GetModernGamertag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult2_Impl::GetModernGamertag(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult2_Impl::GetModernGamertagSuffix(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXblIdpAuthTokenResult2_Impl::GetUniqueModernGamertag(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Identity, Impl, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, Impl, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as windows_core::Interface>::IID
    }
}
