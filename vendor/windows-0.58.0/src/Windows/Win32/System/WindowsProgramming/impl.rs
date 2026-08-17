#[cfg(feature = "Win32_System_Com")]
pub trait ICameraUIControl_Impl: Sized {
    fn Show(&self, pwindow: Option<&windows_core::IUnknown>, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: Option<&ICameraUIControlEventCallback>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn GetCurrentViewType(&self) -> windows_core::Result<CameraUIControlViewType>;
    fn GetActiveItem(&self, pbstractiveitempath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSelectedItems(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn RemoveCapturedItem(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICameraUIControl {}
#[cfg(feature = "Win32_System_Com")]
impl ICameraUIControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICameraUIControl_Vtbl
    where
        Identity: ICameraUIControl_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControl_Impl::Show(this, windows_core::from_raw_borrowed(&pwindow), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&selectionmode), core::mem::transmute_copy(&capturemode), core::mem::transmute_copy(&photoformat), core::mem::transmute_copy(&videoformat), core::mem::transmute_copy(&bhasclosebutton), windows_core::from_raw_borrowed(&peventcallback)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControl_Impl::Close(this).into()
        }
        unsafe extern "system" fn Suspend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICameraUIControl_Impl::Suspend(this) {
                Ok(ok__) => {
                    pbdeferralrequired.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControl_Impl::Resume(this).into()
        }
        unsafe extern "system" fn GetCurrentViewType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICameraUIControl_Impl::GetCurrentViewType(this) {
                Ok(ok__) => {
                    pviewtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstractiveitempath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControl_Impl::GetActiveItem(this, core::mem::transmute_copy(&pbstractiveitempath)).into()
        }
        unsafe extern "system" fn GetSelectedItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICameraUIControl_Impl::GetSelectedItems(this) {
                Ok(ok__) => {
                    ppselecteditempaths.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCapturedItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICameraUIControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControl_Impl::RemoveCapturedItem(this, core::mem::transmute(&pszpath)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            GetCurrentViewType: GetCurrentViewType::<Identity, OFFSET>,
            GetActiveItem: GetActiveItem::<Identity, OFFSET>,
            GetSelectedItems: GetSelectedItems::<Identity, OFFSET>,
            RemoveCapturedItem: RemoveCapturedItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraUIControl as windows_core::Interface>::IID
    }
}
pub trait ICameraUIControlEventCallback_Impl: Sized {
    fn OnStartupComplete(&self);
    fn OnSuspendComplete(&self);
    fn OnItemCaptured(&self, pszpath: &windows_core::PCWSTR);
    fn OnItemDeleted(&self, pszpath: &windows_core::PCWSTR);
    fn OnClosed(&self);
}
impl windows_core::RuntimeName for ICameraUIControlEventCallback {}
impl ICameraUIControlEventCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICameraUIControlEventCallback_Vtbl
    where
        Identity: ICameraUIControlEventCallback_Impl,
    {
        unsafe extern "system" fn OnStartupComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ICameraUIControlEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControlEventCallback_Impl::OnStartupComplete(this)
        }
        unsafe extern "system" fn OnSuspendComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ICameraUIControlEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControlEventCallback_Impl::OnSuspendComplete(this)
        }
        unsafe extern "system" fn OnItemCaptured<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR)
        where
            Identity: ICameraUIControlEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControlEventCallback_Impl::OnItemCaptured(this, core::mem::transmute(&pszpath))
        }
        unsafe extern "system" fn OnItemDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR)
        where
            Identity: ICameraUIControlEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControlEventCallback_Impl::OnItemDeleted(this, core::mem::transmute(&pszpath))
        }
        unsafe extern "system" fn OnClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ICameraUIControlEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICameraUIControlEventCallback_Impl::OnClosed(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartupComplete: OnStartupComplete::<Identity, OFFSET>,
            OnSuspendComplete: OnSuspendComplete::<Identity, OFFSET>,
            OnItemCaptured: OnItemCaptured::<Identity, OFFSET>,
            OnItemDeleted: OnItemDeleted::<Identity, OFFSET>,
            OnClosed: OnClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraUIControlEventCallback as windows_core::Interface>::IID
    }
}
pub trait IClipServiceNotificationHelper_Impl: Sized {
    fn ShowToast(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR, packagename: &windows_core::BSTR, appid: &windows_core::BSTR, launchcommand: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClipServiceNotificationHelper {}
impl IClipServiceNotificationHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClipServiceNotificationHelper_Vtbl
    where
        Identity: IClipServiceNotificationHelper_Impl,
    {
        unsafe extern "system" fn ShowToast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, titletext: core::mem::MaybeUninit<windows_core::BSTR>, bodytext: core::mem::MaybeUninit<windows_core::BSTR>, packagename: core::mem::MaybeUninit<windows_core::BSTR>, appid: core::mem::MaybeUninit<windows_core::BSTR>, launchcommand: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IClipServiceNotificationHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClipServiceNotificationHelper_Impl::ShowToast(this, core::mem::transmute(&titletext), core::mem::transmute(&bodytext), core::mem::transmute(&packagename), core::mem::transmute(&appid), core::mem::transmute(&launchcommand)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowToast: ShowToast::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClipServiceNotificationHelper as windows_core::Interface>::IID
    }
}
pub trait IContainerActivationHelper_Impl: Sized {
    fn CanActivateClientVM(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl windows_core::RuntimeName for IContainerActivationHelper {}
impl IContainerActivationHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContainerActivationHelper_Vtbl
    where
        Identity: IContainerActivationHelper_Impl,
    {
        unsafe extern "system" fn CanActivateClientVM<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IContainerActivationHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContainerActivationHelper_Impl::CanActivateClientVM(this) {
                Ok(ok__) => {
                    isallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanActivateClientVM: CanActivateClientVM::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerActivationHelper as windows_core::Interface>::IID
    }
}
pub trait IDefaultBrowserSyncSettings_Impl: Sized {
    fn IsEnabled(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IDefaultBrowserSyncSettings {}
impl IDefaultBrowserSyncSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDefaultBrowserSyncSettings_Vtbl
    where
        Identity: IDefaultBrowserSyncSettings_Impl,
    {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDefaultBrowserSyncSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDefaultBrowserSyncSettings_Impl::IsEnabled(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsEnabled: IsEnabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDefaultBrowserSyncSettings as windows_core::Interface>::IID
    }
}
pub trait IDeleteBrowsingHistory_Impl: Sized {
    fn DeleteBrowsingHistory(&self, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeleteBrowsingHistory {}
impl IDeleteBrowsingHistory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeleteBrowsingHistory_Vtbl
    where
        Identity: IDeleteBrowsingHistory_Impl,
    {
        unsafe extern "system" fn DeleteBrowsingHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDeleteBrowsingHistory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeleteBrowsingHistory_Impl::DeleteBrowsingHistory(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeleteBrowsingHistory: DeleteBrowsingHistory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeleteBrowsingHistory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IEditionUpgradeBroker_Impl: Sized {
    fn InitializeParentWindow(&self, parenthandle: super::Ole::OLE_HANDLE) -> windows_core::Result<()>;
    fn UpdateOperatingSystem(&self, parameter: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShowProductKeyUI(&self) -> windows_core::Result<()>;
    fn CanUpgrade(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IEditionUpgradeBroker {}
#[cfg(feature = "Win32_System_Ole")]
impl IEditionUpgradeBroker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEditionUpgradeBroker_Vtbl
    where
        Identity: IEditionUpgradeBroker_Impl,
    {
        unsafe extern "system" fn InitializeParentWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parenthandle: super::Ole::OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeBroker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeBroker_Impl::InitializeParentWindow(this, core::mem::transmute_copy(&parenthandle)).into()
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameter: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeBroker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeBroker_Impl::UpdateOperatingSystem(this, core::mem::transmute(&parameter)).into()
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeBroker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeBroker_Impl::ShowProductKeyUI(this).into()
        }
        unsafe extern "system" fn CanUpgrade<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeBroker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeBroker_Impl::CanUpgrade(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeParentWindow: InitializeParentWindow::<Identity, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, OFFSET>,
            CanUpgrade: CanUpgrade::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEditionUpgradeBroker as windows_core::Interface>::IID
    }
}
pub trait IEditionUpgradeHelper_Impl: Sized {
    fn CanUpgrade(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn UpdateOperatingSystem(&self, contentid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShowProductKeyUI(&self) -> windows_core::Result<()>;
    fn GetOsProductContentId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGenuineLocalStatus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IEditionUpgradeHelper {}
impl IEditionUpgradeHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEditionUpgradeHelper_Vtbl
    where
        Identity: IEditionUpgradeHelper_Impl,
    {
        unsafe extern "system" fn CanUpgrade<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEditionUpgradeHelper_Impl::CanUpgrade(this) {
                Ok(ok__) => {
                    isallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeHelper_Impl::UpdateOperatingSystem(this, core::mem::transmute(&contentid)).into()
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEditionUpgradeHelper_Impl::ShowProductKeyUI(this).into()
        }
        unsafe extern "system" fn GetOsProductContentId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEditionUpgradeHelper_Impl::GetOsProductContentId(this) {
                Ok(ok__) => {
                    contentid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEditionUpgradeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEditionUpgradeHelper_Impl::GetGenuineLocalStatus(this) {
                Ok(ok__) => {
                    isgenuine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CanUpgrade: CanUpgrade::<Identity, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, OFFSET>,
            GetOsProductContentId: GetOsProductContentId::<Identity, OFFSET>,
            GetGenuineLocalStatus: GetGenuineLocalStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEditionUpgradeHelper as windows_core::Interface>::IID
    }
}
pub trait IFClipNotificationHelper_Impl: Sized {
    fn ShowSystemDialog(&self, titletext: &windows_core::BSTR, bodytext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFClipNotificationHelper {}
impl IFClipNotificationHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFClipNotificationHelper_Vtbl
    where
        Identity: IFClipNotificationHelper_Impl,
    {
        unsafe extern "system" fn ShowSystemDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, titletext: core::mem::MaybeUninit<windows_core::BSTR>, bodytext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFClipNotificationHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFClipNotificationHelper_Impl::ShowSystemDialog(this, core::mem::transmute(&titletext), core::mem::transmute(&bodytext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowSystemDialog: ShowSystemDialog::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFClipNotificationHelper as windows_core::Interface>::IID
    }
}
pub trait IWindowsLockModeHelper_Impl: Sized {
    fn GetSMode(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWindowsLockModeHelper {}
impl IWindowsLockModeHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsLockModeHelper_Vtbl
    where
        Identity: IWindowsLockModeHelper_Impl,
    {
        unsafe extern "system" fn GetSMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsLockModeHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsLockModeHelper_Impl::GetSMode(this) {
                Ok(ok__) => {
                    issmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSMode: GetSMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsLockModeHelper as windows_core::Interface>::IID
    }
}
